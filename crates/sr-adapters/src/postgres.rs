//! PostgreSQL adapter implementations
//!
//! D-10: EventStore adapter with append-only streams and optimistic concurrency
//! Per SR-SPEC §1.6.2

use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, PgPool, Row};
use sr_domain::{ActorKind, EventEnvelope, StreamKind, TypedRef};
use sr_ports::{EventStore, EventStoreError};
use tracing::{debug, error, instrument};

/// PostgreSQL-backed event store
///
/// Implements append-only event streams per SR-SPEC §1.6.2
/// with optimistic concurrency control using stream versions.
pub struct PostgresEventStore {
    pool: PgPool,
}

impl PostgresEventStore {
    /// Create a new PostgreSQL event store with an existing pool
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Connect to PostgreSQL and create a new event store
    pub async fn connect(database_url: &str) -> Result<Self, EventStoreError> {
        let pool =
            PgPool::connect(database_url)
                .await
                .map_err(|e| EventStoreError::ConnectionError {
                    message: e.to_string(),
                })?;
        Ok(Self { pool })
    }

    /// Get a reference to the underlying pool (for testing/migrations)
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Ensure stream exists, creating if necessary, and return current version
    async fn ensure_stream(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        stream_id: &str,
        stream_kind: &StreamKind,
    ) -> Result<u64, EventStoreError> {
        let kind_str = stream_kind_to_str(stream_kind);

        // Try to get existing stream with row lock
        let result =
            sqlx::query("SELECT stream_version FROM es.streams WHERE stream_id = $1 FOR UPDATE")
                .bind(stream_id)
                .fetch_optional(&mut **tx)
                .await
                .map_err(|e| EventStoreError::ConnectionError {
                    message: e.to_string(),
                })?;

        match result {
            Some(row) => {
                let version: i64 = row.get("stream_version");
                Ok(version as u64)
            }
            None => {
                // Create new stream
                sqlx::query(
                    "INSERT INTO es.streams (stream_id, stream_kind, stream_version) VALUES ($1, $2, 0)",
                )
                .bind(stream_id)
                .bind(kind_str)
                .execute(&mut **tx)
                .await
                .map_err(|e| EventStoreError::ConnectionError {
                    message: e.to_string(),
                })?;
                Ok(0)
            }
        }
    }

    /// Update stream version after appending events
    async fn update_stream_version(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        stream_id: &str,
        new_version: u64,
    ) -> Result<(), EventStoreError> {
        sqlx::query("UPDATE es.streams SET stream_version = $1 WHERE stream_id = $2")
            .bind(new_version as i64)
            .bind(stream_id)
            .execute(&mut **tx)
            .await
            .map_err(|e| EventStoreError::ConnectionError {
                message: e.to_string(),
            })?;
        Ok(())
    }
}

impl EventStore for PostgresEventStore {
    /// Append events to a stream with optimistic concurrency control
    ///
    /// Per SR-SPEC §1.6.2: Events are appended atomically within a transaction.
    /// If `expected_version` doesn't match the current stream version, a
    /// ConcurrencyConflict error is returned.
    ///
    /// Returns the new stream version after appending.
    #[instrument(skip(self, events), fields(stream_id = %stream_id, event_count = events.len()))]
    async fn append(
        &self,
        stream_id: &str,
        expected_version: u64,
        events: Vec<EventEnvelope>,
    ) -> Result<u64, EventStoreError> {
        if events.is_empty() {
            return Ok(expected_version);
        }

        let stream_kind = &events[0].stream_kind;

        let mut tx = self.pool.begin().await.map_err(|e| {
            error!(error = %e, "Failed to begin transaction");
            EventStoreError::ConnectionError {
                message: e.to_string(),
            }
        })?;

        // Get or create stream and check version
        let current_version = self.ensure_stream(&mut tx, stream_id, stream_kind).await?;

        if current_version != expected_version {
            debug!(
                expected = expected_version,
                actual = current_version,
                "Concurrency conflict detected"
            );
            return Err(EventStoreError::ConcurrencyConflict {
                expected: expected_version,
                actual: current_version,
            });
        }

        // Insert events
        let mut new_version = current_version;
        for event in &events {
            new_version += 1;

            let actor_kind_str = actor_kind_to_str(&event.actor_kind);
            let refs_json = serde_json::to_value(&event.refs).map_err(|e| {
                EventStoreError::SerializationError {
                    message: e.to_string(),
                }
            })?;
            let supersedes: Vec<String> = event.supersedes.clone();

            sqlx::query(
                r#"
                INSERT INTO es.events (
                    event_id, stream_id, stream_seq, occurred_at,
                    actor_kind, actor_id, event_type, correlation_id,
                    causation_id, supersedes, refs, payload, envelope_hash
                ) VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13
                )
                "#,
            )
            .bind(event.event_id.as_str())
            .bind(stream_id)
            .bind(new_version as i64)
            .bind(event.occurred_at)
            .bind(actor_kind_str)
            .bind(&event.actor_id)
            .bind(&event.event_type)
            .bind(&event.correlation_id)
            .bind(&event.causation_id)
            .bind(&supersedes)
            .bind(refs_json)
            .bind(&event.payload)
            .bind(&event.envelope_hash)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                error!(error = %e, event_id = %event.event_id.as_str(), "Failed to insert event");
                EventStoreError::ConnectionError {
                    message: e.to_string(),
                }
            })?;
        }

        // Update stream version
        self.update_stream_version(&mut tx, stream_id, new_version)
            .await?;

        tx.commit().await.map_err(|e| {
            error!(error = %e, "Failed to commit transaction");
            EventStoreError::ConnectionError {
                message: e.to_string(),
            }
        })?;

        debug!(new_version = new_version, "Events appended successfully");
        Ok(new_version)
    }

    /// Read events from a stream starting at a given sequence
    ///
    /// Per SR-SPEC §1.6.2: Events are returned in stream sequence order.
    #[instrument(skip(self), fields(stream_id = %stream_id, from_seq = from_seq, limit = limit))]
    async fn read_stream(
        &self,
        stream_id: &str,
        from_seq: u64,
        limit: usize,
    ) -> Result<Vec<EventEnvelope>, EventStoreError> {
        // Check if stream exists
        let stream_exists = sqlx::query("SELECT 1 FROM es.streams WHERE stream_id = $1")
            .bind(stream_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| EventStoreError::ConnectionError {
                message: e.to_string(),
            })?;

        if stream_exists.is_none() {
            return Err(EventStoreError::StreamNotFound {
                stream_id: stream_id.to_string(),
            });
        }

        let rows = sqlx::query(
            r#"
            SELECT
                global_seq, event_id, stream_id, stream_seq, occurred_at,
                actor_kind, actor_id, event_type, correlation_id,
                causation_id, supersedes, refs, payload, envelope_hash
            FROM es.events
            WHERE stream_id = $1 AND stream_seq > $2
            ORDER BY stream_seq ASC
            LIMIT $3
            "#,
        )
        .bind(stream_id)
        .bind(from_seq as i64)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| EventStoreError::ConnectionError {
            message: e.to_string(),
        })?;

        rows.iter().map(row_to_event_envelope).collect()
    }

    /// Replay all events in global order
    ///
    /// Per SR-SPEC §1.6.2: Events are returned in global sequence order for
    /// deterministic replay across all streams.
    #[instrument(skip(self), fields(from_global_seq = from_global_seq, limit = limit))]
    async fn replay_all(
        &self,
        from_global_seq: u64,
        limit: usize,
    ) -> Result<Vec<EventEnvelope>, EventStoreError> {
        let rows = sqlx::query(
            r#"
            SELECT
                global_seq, event_id, stream_id, stream_seq, occurred_at,
                actor_kind, actor_id, event_type, correlation_id,
                causation_id, supersedes, refs, payload, envelope_hash
            FROM es.events
            WHERE global_seq > $1
            ORDER BY global_seq ASC
            LIMIT $2
            "#,
        )
        .bind(from_global_seq as i64)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| EventStoreError::ConnectionError {
            message: e.to_string(),
        })?;

        rows.iter().map(row_to_event_envelope).collect()
    }
}

/// Convert a database row to an EventEnvelope
fn row_to_event_envelope(row: &PgRow) -> Result<EventEnvelope, EventStoreError> {
    let global_seq: i64 = row.get("global_seq");
    let event_id: String = row.get("event_id");
    let stream_id: String = row.get("stream_id");
    let stream_seq: i64 = row.get("stream_seq");
    let occurred_at: DateTime<Utc> = row.get("occurred_at");
    let actor_kind_str: String = row.get("actor_kind");
    let actor_id: String = row.get("actor_id");
    let event_type: String = row.get("event_type");
    let correlation_id: Option<String> = row.get("correlation_id");
    let causation_id: Option<String> = row.get("causation_id");
    let supersedes: Vec<String> = row.get("supersedes");
    let refs_json: serde_json::Value = row.get("refs");
    let payload: serde_json::Value = row.get("payload");
    let envelope_hash: String = row.get("envelope_hash");

    let actor_kind = str_to_actor_kind(&actor_kind_str)?;
    let stream_kind = infer_stream_kind(&stream_id);
    let refs: Vec<TypedRef> =
        serde_json::from_value(refs_json).map_err(|e| EventStoreError::SerializationError {
            message: format!("Failed to deserialize refs: {e}"),
        })?;

    Ok(EventEnvelope {
        event_id: sr_domain::EventId::from_string(event_id),
        stream_id,
        stream_kind,
        stream_seq: stream_seq as u64,
        global_seq: Some(global_seq as u64),
        event_type,
        occurred_at,
        actor_kind,
        actor_id,
        correlation_id,
        causation_id,
        supersedes,
        refs,
        payload,
        envelope_hash,
    })
}

/// Convert StreamKind to database string
fn stream_kind_to_str(kind: &StreamKind) -> &'static str {
    match kind {
        StreamKind::Loop => "LOOP",
        StreamKind::Iteration => "ITERATION",
        StreamKind::Candidate => "CANDIDATE",
        StreamKind::Run => "RUN",
        StreamKind::Approval => "APPROVAL",
        StreamKind::Decision => "DECISION",
        StreamKind::Governance => "GOVERNANCE",
        StreamKind::Exception => "EXCEPTION",
        StreamKind::OracleSuite => "ORACLE_SUITE",
        StreamKind::Freeze => "FREEZE",
        StreamKind::Intake => "INTAKE",
        StreamKind::WorkSurface => "WORK_SURFACE",
    }
}

/// Infer StreamKind from stream_id prefix
fn infer_stream_kind(stream_id: &str) -> StreamKind {
    if stream_id.starts_with("loop_") {
        StreamKind::Loop
    } else if stream_id.starts_with("iter_") {
        StreamKind::Iteration
    } else if stream_id.contains("cand_") {
        StreamKind::Candidate
    } else if stream_id.starts_with("run_") {
        StreamKind::Run
    } else if stream_id.starts_with("appr_") {
        StreamKind::Approval
    } else if stream_id.starts_with("dec_") {
        StreamKind::Decision
    } else if stream_id.starts_with("exc_") {
        StreamKind::Exception
    } else if stream_id.starts_with("suite_") {
        StreamKind::OracleSuite
    } else if stream_id.starts_with("freeze_") {
        StreamKind::Freeze
    } else if stream_id.starts_with("intake:") {
        StreamKind::Intake
    } else if stream_id.starts_with("work_surface:") {
        StreamKind::WorkSurface
    } else {
        StreamKind::Governance
    }
}

/// Convert ActorKind to database string
fn actor_kind_to_str(kind: &ActorKind) -> &'static str {
    match kind {
        ActorKind::Human => "HUMAN",
        ActorKind::Agent => "AGENT",
        ActorKind::System => "SYSTEM",
    }
}

/// Convert database string to ActorKind
fn str_to_actor_kind(s: &str) -> Result<ActorKind, EventStoreError> {
    match s {
        "HUMAN" => Ok(ActorKind::Human),
        "AGENT" => Ok(ActorKind::Agent),
        "SYSTEM" => Ok(ActorKind::System),
        _ => Err(EventStoreError::SerializationError {
            message: format!("Unknown actor kind: {s}"),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sr_domain::EventId;

    fn create_test_event(stream_id: &str, stream_seq: u64) -> EventEnvelope {
        EventEnvelope {
            event_id: EventId::new(),
            stream_id: stream_id.to_string(),
            stream_kind: StreamKind::Loop,
            stream_seq,
            global_seq: None,
            event_type: "LoopCreated".to_string(),
            occurred_at: Utc::now(),
            actor_kind: ActorKind::System,
            actor_id: "system".to_string(),
            correlation_id: Some("corr_123".to_string()),
            causation_id: None,
            supersedes: vec![],
            refs: vec![TypedRef {
                kind: "Loop".to_string(),
                id: stream_id.to_string(),
                rel: "self".to_string(),
                meta: serde_json::Value::Null,
            }],
            payload: serde_json::json!({
                "goal": "Test loop",
                "budgets": {
                    "max_iterations": 5
                }
            }),
            envelope_hash: "sha256:test_hash".to_string(),
        }
    }

    #[test]
    fn test_stream_kind_conversion() {
        assert_eq!(stream_kind_to_str(&StreamKind::Loop), "LOOP");
        assert_eq!(stream_kind_to_str(&StreamKind::Iteration), "ITERATION");
        assert_eq!(stream_kind_to_str(&StreamKind::Candidate), "CANDIDATE");
    }

    #[test]
    fn test_actor_kind_conversion() {
        assert_eq!(actor_kind_to_str(&ActorKind::Human), "HUMAN");
        assert_eq!(actor_kind_to_str(&ActorKind::Agent), "AGENT");
        assert_eq!(actor_kind_to_str(&ActorKind::System), "SYSTEM");

        assert!(matches!(str_to_actor_kind("HUMAN"), Ok(ActorKind::Human)));
        assert!(matches!(str_to_actor_kind("AGENT"), Ok(ActorKind::Agent)));
        assert!(matches!(str_to_actor_kind("SYSTEM"), Ok(ActorKind::System)));
        assert!(str_to_actor_kind("UNKNOWN").is_err());
    }

    #[test]
    fn test_infer_stream_kind() {
        assert!(matches!(infer_stream_kind("loop_abc"), StreamKind::Loop));
        assert!(matches!(
            infer_stream_kind("iter_abc"),
            StreamKind::Iteration
        ));
        assert!(matches!(
            infer_stream_kind("sha256:abc|cand_xyz"),
            StreamKind::Candidate
        ));
        assert!(matches!(infer_stream_kind("run_abc"), StreamKind::Run));
        assert!(matches!(
            infer_stream_kind("appr_abc"),
            StreamKind::Approval
        ));
        assert!(matches!(infer_stream_kind("dec_abc"), StreamKind::Decision));
        assert!(matches!(
            infer_stream_kind("exc_abc"),
            StreamKind::Exception
        ));
        assert!(matches!(
            infer_stream_kind("suite_abc"),
            StreamKind::OracleSuite
        ));
        assert!(matches!(
            infer_stream_kind("freeze_abc"),
            StreamKind::Freeze
        ));
        assert!(matches!(
            infer_stream_kind("other_abc"),
            StreamKind::Governance
        ));
    }

    #[test]
    fn test_create_test_event() {
        let event = create_test_event("loop_test123", 1);

        assert_eq!(event.stream_id, "loop_test123");
        assert_eq!(event.stream_seq, 1);
        assert_eq!(event.event_type, "LoopCreated");
        assert!(matches!(event.actor_kind, ActorKind::System));
    }
}

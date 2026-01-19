//! Outbox Publisher per SR-SPEC §1.6.2
//!
//! D-13: Implements reliable event publication to NATS/JetStream via the
//! transactional outbox pattern.
//!
//! Key properties:
//! - Events are written to outbox in the same transaction as event store
//! - Publisher reads from outbox and publishes to NATS
//! - Events are marked published after successful delivery
//! - Idempotent via message hashes (no duplicate logical events)

use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use sr_domain::EventEnvelope;
use tracing::{debug, error, info, instrument};

/// Outbox publisher error types
#[derive(Debug, thiserror::Error)]
pub enum OutboxError {
    #[error("Database error: {message}")]
    DatabaseError { message: String },

    #[error("Serialization error: {message}")]
    SerializationError { message: String },

    #[error("Publish error: {message}")]
    PublishError { message: String },
}

impl From<sqlx::Error> for OutboxError {
    fn from(e: sqlx::Error) -> Self {
        OutboxError::DatabaseError {
            message: e.to_string(),
        }
    }
}

/// Outbox entry
#[derive(Debug, Clone)]
pub struct OutboxEntry {
    pub outbox_id: i64,
    pub global_seq: i64,
    pub published_at: Option<DateTime<Utc>>,
    pub topic: String,
    pub message: serde_json::Value,
    pub message_hash: String,
}

/// Topic mapping for event types per SR-SPEC
pub fn topic_for_event(event_type: &str) -> String {
    match event_type {
        // Loop events
        "LoopCreated" | "LoopActivated" | "LoopPaused" | "LoopResumed" | "LoopClosed" => {
            "sr.events.loop".to_string()
        }

        // Iteration events
        "IterationStarted"
        | "IterationCompleted"
        | "IterationSummaryRecorded"
        | "StopTriggered" => "sr.events.iteration".to_string(),

        // Candidate events
        "CandidateMaterialized" | "CandidateVerificationComputed" => {
            "sr.events.candidate".to_string()
        }

        // Run/Evidence events
        "RunStarted" | "RunCompleted" | "EvidenceBundleRecorded" => "sr.events.run".to_string(),

        // Oracle events
        "OracleSuiteRegistered"
        | "OracleSuiteUpdated"
        | "OracleSuitePinned"
        | "OracleSuiteRebased" => "sr.events.oracle".to_string(),

        // Governance events
        "GovernedArtifactVersionRecorded" => "sr.events.governance".to_string(),

        // Freeze/Release events
        "FreezeRecordCreated" => "sr.events.freeze".to_string(),

        // Staleness events
        "NodeMarkedStale" | "ReEvaluationTriggered" | "StalenessResolved" => {
            "sr.events.staleness".to_string()
        }

        // Portal/Approval events
        "ApprovalRecorded" => "sr.events.approval".to_string(),

        // Exception events
        "DeviationCreated" | "DeferralCreated" | "WaiverCreated" | "ExceptionActivated"
        | "ExceptionResolved" | "ExceptionExpired" => "sr.events.exception".to_string(),

        // Decision events
        "DecisionRecorded" => "sr.events.decision".to_string(),

        // Work surface events
        "WorkSurfaceRecorded"
        | "TemplateSelected"
        | "StageEntered"
        | "StageCompleted"
        | "SemanticOracleEvaluated" => "sr.events.worksurface".to_string(),

        // Default topic for unknown events
        _ => "sr.events.other".to_string(),
    }
}

/// Compute message hash for idempotency
pub fn compute_message_hash(event: &EventEnvelope) -> String {
    let mut hasher = Sha256::new();
    hasher.update(event.event_id.as_str().as_bytes());
    hasher.update(event.stream_id.as_bytes());
    hasher.update(&event.stream_seq.to_le_bytes());
    let result = hasher.finalize();
    format!("sha256:{}", hex::encode(result))
}

/// Outbox writer - writes events to the outbox table
///
/// This is called within the same transaction as event store append
/// to ensure atomicity.
pub struct OutboxWriter;

impl OutboxWriter {
    /// Write an event to the outbox within an existing transaction
    #[instrument(skip(tx, event), fields(event_id = %event.event_id.as_str()))]
    pub async fn write(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        global_seq: i64,
        event: &EventEnvelope,
    ) -> Result<i64, OutboxError> {
        let topic = topic_for_event(&event.event_type);
        let message = serde_json::to_value(event).map_err(|e| OutboxError::SerializationError {
            message: e.to_string(),
        })?;
        let message_hash = compute_message_hash(event);

        let outbox_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO es.outbox (global_seq, topic, message, message_hash)
            VALUES ($1, $2, $3, $4)
            RETURNING outbox_id
            "#,
        )
        .bind(global_seq)
        .bind(&topic)
        .bind(&message)
        .bind(&message_hash)
        .fetch_one(&mut **tx)
        .await?;

        debug!(
            outbox_id = outbox_id,
            topic = topic,
            "Event written to outbox"
        );
        Ok(outbox_id)
    }

    /// Write multiple events to the outbox
    pub async fn write_batch(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        events: &[(i64, &EventEnvelope)],
    ) -> Result<Vec<i64>, OutboxError> {
        let mut outbox_ids = Vec::with_capacity(events.len());

        for (global_seq, event) in events {
            let outbox_id = Self::write(tx, *global_seq, event).await?;
            outbox_ids.push(outbox_id);
        }

        Ok(outbox_ids)
    }
}

/// Outbox publisher configuration
#[derive(Debug, Clone)]
pub struct OutboxPublisherConfig {
    /// Maximum number of messages to process per batch
    pub batch_size: usize,
    /// Whether to use JetStream (true) or Core NATS (false)
    pub use_jetstream: bool,
}

impl Default for OutboxPublisherConfig {
    fn default() -> Self {
        Self {
            batch_size: 100,
            use_jetstream: true,
        }
    }
}

/// Outbox publisher - reads from outbox and publishes to NATS
pub struct OutboxPublisher {
    pool: PgPool,
    config: OutboxPublisherConfig,
}

impl OutboxPublisher {
    /// Create a new outbox publisher
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            config: OutboxPublisherConfig::default(),
        }
    }

    /// Create with custom config
    pub fn with_config(pool: PgPool, config: OutboxPublisherConfig) -> Self {
        Self { pool, config }
    }

    /// Get unpublished messages from the outbox
    #[instrument(skip(self))]
    pub async fn get_unpublished(&self, limit: usize) -> Result<Vec<OutboxEntry>, OutboxError> {
        let rows = sqlx::query_as::<
            _,
            (
                i64,
                i64,
                Option<DateTime<Utc>>,
                String,
                serde_json::Value,
                String,
            ),
        >(
            r#"
            SELECT outbox_id, global_seq, published_at, topic, message, message_hash
            FROM es.outbox
            WHERE published_at IS NULL
            ORDER BY outbox_id ASC
            LIMIT $1
            "#,
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(
                |(outbox_id, global_seq, published_at, topic, message, message_hash)| OutboxEntry {
                    outbox_id,
                    global_seq,
                    published_at,
                    topic,
                    message,
                    message_hash,
                },
            )
            .collect())
    }

    /// Mark a message as published
    #[instrument(skip(self))]
    pub async fn mark_published(&self, outbox_id: i64) -> Result<(), OutboxError> {
        sqlx::query(
            r#"
            UPDATE es.outbox
            SET published_at = NOW()
            WHERE outbox_id = $1 AND published_at IS NULL
            "#,
        )
        .bind(outbox_id)
        .execute(&self.pool)
        .await?;

        debug!(outbox_id = outbox_id, "Message marked as published");
        Ok(())
    }

    /// Mark multiple messages as published
    pub async fn mark_published_batch(&self, outbox_ids: &[i64]) -> Result<(), OutboxError> {
        if outbox_ids.is_empty() {
            return Ok(());
        }

        sqlx::query(
            r#"
            UPDATE es.outbox
            SET published_at = NOW()
            WHERE outbox_id = ANY($1) AND published_at IS NULL
            "#,
        )
        .bind(outbox_ids)
        .execute(&self.pool)
        .await?;

        debug!(count = outbox_ids.len(), "Messages marked as published");
        Ok(())
    }

    /// Process unpublished messages using a callback
    ///
    /// The callback receives the topic and message payload.
    /// Returns the number of successfully published messages.
    #[instrument(skip(self, publish_fn))]
    pub async fn process_batch<F, Fut>(&self, publish_fn: F) -> Result<usize, OutboxError>
    where
        F: Fn(String, Vec<u8>) -> Fut,
        Fut: std::future::Future<Output = Result<(), OutboxError>>,
    {
        let entries = self.get_unpublished(self.config.batch_size).await?;

        if entries.is_empty() {
            return Ok(0);
        }

        info!(count = entries.len(), "Processing outbox batch");

        let mut published_ids = Vec::with_capacity(entries.len());

        for entry in entries {
            let payload = serde_json::to_vec(&entry.message).map_err(|e| {
                OutboxError::SerializationError {
                    message: e.to_string(),
                }
            })?;

            match publish_fn(entry.topic.clone(), payload).await {
                Ok(()) => {
                    published_ids.push(entry.outbox_id);
                    debug!(
                        outbox_id = entry.outbox_id,
                        topic = entry.topic,
                        "Message published"
                    );
                }
                Err(e) => {
                    error!(
                        outbox_id = entry.outbox_id,
                        error = %e,
                        "Failed to publish message"
                    );
                    // Continue with other messages, don't fail the whole batch
                }
            }
        }

        if !published_ids.is_empty() {
            self.mark_published_batch(&published_ids).await?;
        }

        Ok(published_ids.len())
    }

    /// Get the number of unpublished messages
    pub async fn unpublished_count(&self) -> Result<i64, OutboxError> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"SELECT COUNT(*) FROM es.outbox WHERE published_at IS NULL"#,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }

    /// Purge old published messages (cleanup)
    #[instrument(skip(self))]
    pub async fn purge_published(&self, older_than_hours: i32) -> Result<i64, OutboxError> {
        let result = sqlx::query(
            r#"
            DELETE FROM es.outbox
            WHERE published_at IS NOT NULL
              AND published_at < NOW() - INTERVAL '1 hour' * $1
            "#,
        )
        .bind(older_than_hours)
        .execute(&self.pool)
        .await?;

        let deleted = result.rows_affected() as i64;
        if deleted > 0 {
            info!(deleted = deleted, "Purged old published messages");
        }

        Ok(deleted)
    }
}

/// Extended EventStore that writes to outbox
///
/// This trait extension adds outbox writing to the event store append operation.
pub trait EventStoreWithOutbox {
    /// Append events and write to outbox in a single transaction
    fn append_with_outbox(
        &self,
        stream_id: &str,
        expected_version: u64,
        events: Vec<EventEnvelope>,
    ) -> impl std::future::Future<Output = Result<u64, OutboxError>> + Send;
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topic_mapping() {
        assert_eq!(topic_for_event("LoopCreated"), "sr.events.loop");
        assert_eq!(topic_for_event("IterationStarted"), "sr.events.iteration");
        assert_eq!(
            topic_for_event("CandidateMaterialized"),
            "sr.events.candidate"
        );
        assert_eq!(topic_for_event("RunStarted"), "sr.events.run");
        assert_eq!(topic_for_event("ApprovalRecorded"), "sr.events.approval");
        assert_eq!(topic_for_event("DecisionRecorded"), "sr.events.decision");
        assert_eq!(topic_for_event("FreezeRecordCreated"), "sr.events.freeze");
        assert_eq!(topic_for_event("Unknown"), "sr.events.other");
    }

    #[test]
    fn test_config_default() {
        let config = OutboxPublisherConfig::default();
        assert_eq!(config.batch_size, 100);
        assert!(config.use_jetstream);
    }

    #[test]
    fn test_message_hash_determinism() {
        use chrono::Utc;
        use sr_domain::{ActorKind, EventId, StreamKind};

        let event = EventEnvelope {
            event_id: EventId::from_string("evt_test123".to_string()),
            stream_id: "loop_abc".to_string(),
            stream_kind: StreamKind::Loop,
            stream_seq: 1,
            global_seq: Some(1),
            event_type: "LoopCreated".to_string(),
            occurred_at: Utc::now(),
            actor_kind: ActorKind::System,
            actor_id: "system".to_string(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload: serde_json::json!({}),
            envelope_hash: "sha256:test".to_string(),
        };

        let hash1 = compute_message_hash(&event);
        let hash2 = compute_message_hash(&event);

        assert_eq!(hash1, hash2);
        assert!(hash1.starts_with("sha256:"));
    }
}

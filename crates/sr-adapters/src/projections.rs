//! Projection Builder per SR-SPEC §1.7
//!
//! D-11: Materializes loop/iteration/candidate/run read models from event streams.
//!
//! Key properties:
//! - Deterministic: rebuilding from scratch matches incremental results
//! - Tracks last processed event (global_seq) for incremental updates
//! - All projections rebuildable from es.events alone

use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, PgPool, Row};
use sr_domain::{EventEnvelope, EventId};
use sr_ports::{EventStore, EventStoreError};
use tracing::{debug, error, info, instrument, warn};

/// Projection builder error types
#[derive(Debug, thiserror::Error)]
pub enum ProjectionError {
    #[error("Event store error: {0}")]
    EventStoreError(#[from] EventStoreError),

    #[error("Database error: {message}")]
    DatabaseError { message: String },

    #[error("Deserialization error: {message}")]
    DeserializationError { message: String },

    #[error("Unknown event type: {event_type}")]
    UnknownEventType { event_type: String },
}

impl From<sqlx::Error> for ProjectionError {
    fn from(e: sqlx::Error) -> Self {
        ProjectionError::DatabaseError {
            message: e.to_string(),
        }
    }
}

/// Projection builder configuration
#[derive(Debug, Clone)]
pub struct ProjectionConfig {
    /// Batch size for event processing
    pub batch_size: usize,
    /// Whether to rebuild projections on startup
    pub rebuild_on_startup: bool,
}

impl Default for ProjectionConfig {
    fn default() -> Self {
        Self {
            batch_size: 100,
            rebuild_on_startup: false,
        }
    }
}

/// Projection checkpoint - tracks the last processed event
#[derive(Debug, Clone)]
pub struct ProjectionCheckpoint {
    pub projection_name: String,
    pub last_global_seq: u64,
    pub last_event_id: String,
    pub updated_at: DateTime<Utc>,
}

/// Projection builder - materializes read models from event streams
///
/// Per SR-SPEC §1.7: All projections MUST be rebuildable from es.events alone.
/// No projection may introduce state that cannot be reconstructed from events.
pub struct ProjectionBuilder {
    pool: PgPool,
    config: ProjectionConfig,
}

impl ProjectionBuilder {
    /// Create a new projection builder
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            config: ProjectionConfig::default(),
        }
    }

    /// Create a projection builder with custom config
    pub fn with_config(pool: PgPool, config: ProjectionConfig) -> Self {
        Self { pool, config }
    }

    /// Get a reference to the underlying pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Get the current checkpoint for a projection
    #[instrument(skip(self))]
    pub async fn get_checkpoint(
        &self,
        projection_name: &str,
    ) -> Result<Option<ProjectionCheckpoint>, ProjectionError> {
        let result = sqlx::query(
            r#"
            SELECT projection_name, last_global_seq, last_event_id, updated_at
            FROM proj.checkpoints
            WHERE projection_name = $1
            "#,
        )
        .bind(projection_name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|row| ProjectionCheckpoint {
            projection_name: row.get("projection_name"),
            last_global_seq: row.get::<i64, _>("last_global_seq") as u64,
            last_event_id: row.get("last_event_id"),
            updated_at: row.get("updated_at"),
        }))
    }

    /// Update the checkpoint for a projection
    #[instrument(skip(self))]
    async fn update_checkpoint(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        projection_name: &str,
        global_seq: u64,
        event_id: &str,
    ) -> Result<(), ProjectionError> {
        sqlx::query(
            r#"
            INSERT INTO proj.checkpoints (projection_name, last_global_seq, last_event_id, updated_at)
            VALUES ($1, $2, $3, NOW())
            ON CONFLICT (projection_name) DO UPDATE
            SET last_global_seq = $2, last_event_id = $3, updated_at = NOW()
            "#,
        )
        .bind(projection_name)
        .bind(global_seq as i64)
        .bind(event_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// Process events and update projections
    ///
    /// Returns the number of events processed
    #[instrument(skip(self, event_store))]
    pub async fn process_events<E: EventStore>(
        &self,
        event_store: &E,
    ) -> Result<usize, ProjectionError> {
        // Get current checkpoint for the main projection
        let checkpoint = self.get_checkpoint("main").await?;
        let from_seq = checkpoint.map(|c| c.last_global_seq).unwrap_or(0);

        info!(from_seq = from_seq, "Starting event processing");

        let mut total_processed = 0;
        let mut current_seq = from_seq;

        loop {
            let events = event_store
                .replay_all(current_seq, self.config.batch_size)
                .await?;

            if events.is_empty() {
                break;
            }

            let batch_size = events.len();
            debug!(batch_size = batch_size, "Processing event batch");

            for event in events {
                self.apply_event(&event).await?;

                if let Some(seq) = event.global_seq {
                    current_seq = seq;
                }
                total_processed += 1;
            }
        }

        info!(total_processed = total_processed, "Event processing complete");
        Ok(total_processed)
    }

    /// Apply a single event to projections
    #[instrument(skip(self, event), fields(event_id = %event.event_id.as_str(), event_type = %event.event_type))]
    pub async fn apply_event(&self, event: &EventEnvelope) -> Result<(), ProjectionError> {
        let mut tx = self.pool.begin().await?;

        let result = match event.event_type.as_str() {
            // Loop events
            "LoopCreated" => self.apply_loop_created(&mut tx, event).await,
            "LoopActivated" => self.apply_loop_activated(&mut tx, event).await,
            "LoopPaused" => self.apply_loop_paused(&mut tx, event).await,
            "LoopResumed" => self.apply_loop_resumed(&mut tx, event).await,
            "LoopClosed" => self.apply_loop_closed(&mut tx, event).await,

            // Iteration events
            "IterationStarted" => self.apply_iteration_started(&mut tx, event).await,
            "IterationCompleted" => self.apply_iteration_completed(&mut tx, event).await,
            "IterationSummaryRecorded" => {
                self.apply_iteration_summary_recorded(&mut tx, event).await
            }

            // Candidate events
            "CandidateMaterialized" => self.apply_candidate_materialized(&mut tx, event).await,
            "CandidateVerificationComputed" => {
                self.apply_candidate_verification_computed(&mut tx, event).await
            }

            // Run events
            "RunStarted" => self.apply_run_started(&mut tx, event).await,
            "RunCompleted" => self.apply_run_completed(&mut tx, event).await,

            // Approval events
            "ApprovalRecorded" => self.apply_approval_recorded(&mut tx, event).await,

            // Exception events
            "DeviationCreated" | "DeferralCreated" | "WaiverCreated" => {
                self.apply_exception_created(&mut tx, event).await
            }
            "ExceptionActivated" => self.apply_exception_activated(&mut tx, event).await,
            "ExceptionResolved" => self.apply_exception_resolved(&mut tx, event).await,
            "ExceptionExpired" => self.apply_exception_expired(&mut tx, event).await,

            // Freeze events
            "FreezeRecordCreated" => self.apply_freeze_record_created(&mut tx, event).await,

            // Decision events
            "DecisionRecorded" => self.apply_decision_recorded(&mut tx, event).await,

            // Governed artifact events
            "GovernedArtifactVersionRecorded" => {
                self.apply_governed_artifact_recorded(&mut tx, event).await
            }

            // Events we acknowledge but don't project
            "EvidenceBundleRecorded"
            | "StopTriggered"
            | "OracleSuiteRegistered"
            | "OracleSuiteUpdated"
            | "OracleSuitePinned"
            | "OracleSuiteRebased"
            | "NodeMarkedStale"
            | "ReEvaluationTriggered"
            | "StalenessResolved"
            | "EvidenceMissingDetected"
            | "RecordCreated"
            | "RecordSuperseded"
            | "WorkSurfaceRecorded"
            | "ProcedureTemplateSelected"
            | "StageEntered"
            | "StageCompleted"
            | "SemanticOracleEvaluated" => {
                debug!(event_type = %event.event_type, "Event acknowledged, no projection update needed");
                Ok(())
            }

            _ => {
                warn!(event_type = %event.event_type, "Unknown event type");
                Ok(()) // Don't fail on unknown events, just skip
            }
        };

        if let Err(e) = result {
            error!(error = %e, "Failed to apply event");
            return Err(e);
        }

        // Update checkpoint
        if let Some(global_seq) = event.global_seq {
            self.update_checkpoint(&mut tx, "main", global_seq, event.event_id.as_str())
                .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    /// Rebuild all projections from scratch
    #[instrument(skip(self, event_store))]
    pub async fn rebuild_all<E: EventStore>(
        &self,
        event_store: &E,
    ) -> Result<usize, ProjectionError> {
        info!("Starting full projection rebuild");

        // Truncate all projection tables
        self.truncate_projections().await?;

        // Process all events from the beginning
        let processed = self.process_events(event_store).await?;

        info!(
            processed = processed,
            "Full projection rebuild complete"
        );
        Ok(processed)
    }

    /// Truncate all projection tables for rebuild
    async fn truncate_projections(&self) -> Result<(), ProjectionError> {
        let tables = [
            "proj.loops",
            "proj.iterations",
            "proj.candidates",
            "proj.runs",
            "proj.approvals",
            "proj.exceptions",
            "proj.freeze_records",
            "proj.decisions",
            "proj.governed_artifacts",
            "proj.shippable_status",
            "proj.human_judgment_records",
            "proj.checkpoints",
        ];

        for table in tables {
            sqlx::query(&format!("TRUNCATE TABLE {table} CASCADE"))
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }

    // ========================================================================
    // Loop Event Handlers
    // ========================================================================

    async fn apply_loop_created(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        event: &EventEnvelope,
    ) -> Result<(), ProjectionError> {
        let payload = &event.payload;
        let loop_id = &event.stream_id;

        let goal = payload["goal"].as_str().unwrap_or("");
        let work_unit = payload["work_unit"].as_str().unwrap_or(loop_id);
        let budgets = &payload["budgets"];
        let directive_ref = &payload["directive_ref"];

        sqlx::query(
            r#"
            INSERT INTO proj.loops (
                loop_id, goal, work_unit, state, budgets, directive_ref,
                created_by_kind, created_by_id, created_at, iteration_count,
                last_event_id, last_global_seq
            ) VALUES ($1, $2, $3, 'CREATED', $4, $5, $6, $7, $8, 0, $9, $10)
            "#,
        )
        .bind(loop_id)
        .bind(goal)
        .bind(work_unit)
        .bind(budgets)
        .bind(directive_ref)
        .bind(actor_kind_str(&event.actor_kind))
        .bind(&event.actor_id)
        .bind(event.occurred_at)
        .bind(event.event_id.as_str())
        .bind(event.global_seq.unwrap_or(0) as i64)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    async fn apply_loop_activated(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        event: &EventEnvelope,
    ) -> Result<(), ProjectionError> {
        sqlx::query(
            r#"
            UPDATE proj.loops
            SET state = 'ACTIVE', activated_at = $1, last_event_id = $2, last_global_seq = $3
            WHERE loop_id = $4
            "#,
        )
        .bind(event.occurred_at)
        .bind(event.event_id.as_str())
        .bind(event.global_seq.unwrap_or(0) as i64)
        .bind(&event.stream_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    async fn apply_loop_paused(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        event: &EventEnvelope,
    ) -> Result<(), ProjectionError> {
        sqlx::query(
            r#"
            UPDATE proj.loops
            SET state = 'PAUSED', last_event_id = $1, last_global_seq = $2
            WHERE loop_id = $3
            "#,
        )
        .bind(event.event_id.as_str())
        .bind(event.global_seq.unwrap_or(0) as i64)
        .bind(&event.stream_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    async fn apply_loop_resumed(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        event: &EventEnvelope,
    ) -> Result<(), ProjectionError> {
        sqlx::query(
            r#"
            UPDATE proj.loops
            SET state = 'ACTIVE', last_event_id = $1, last_global_seq = $2
            WHERE loop_id = $3
            "#,
        )
        .bind(event.event_id.as_str())
        .bind(event.global_seq.unwrap_or(0) as i64)
        .bind(&event.stream_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    async fn apply_loop_closed(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        event: &EventEnvelope,
    ) -> Result<(), ProjectionError> {
        sqlx::query(
            r#"
            UPDATE proj.loops
            SET state = 'CLOSED', closed_at = $1, last_event_id = $2, last_global_seq = $3
            WHERE loop_id = $4
            "#,
        )
        .bind(event.occurred_at)
        .bind(event.event_id.as_str())
        .bind(event.global_seq.unwrap_or(0) as i64)
        .bind(&event.stream_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    // ========================================================================
    // Iteration Event Handlers
    // ========================================================================

    async fn apply_iteration_started(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        event: &EventEnvelope,
    ) -> Result<(), ProjectionError> {
        let payload = &event.payload;
        let iteration_id = &event.stream_id;

        let loop_id = payload["loop_id"].as_str().unwrap_or("");
        let sequence = payload["sequence"].as_u64().unwrap_or(0) as i32;
        let refs = serde_json::to_value(&event.refs).unwrap_or_default();

        sqlx::query(
            r#"
            INSERT INTO proj.iterations (
                iteration_id, loop_id, sequence, state, started_at,
                refs, last_event_id, last_global_seq
            ) VALUES ($1, $2, $3, 'STARTED', $4, $5, $6, $7)
            "#,
        )
        .bind(iteration_id)
        .bind(loop_id)
        .bind(sequence)
        .bind(event.occurred_at)
        .bind(refs)
        .bind(event.event_id.as_str())
        .bind(event.global_seq.unwrap_or(0) as i64)
        .execute(&mut **tx)
        .await?;

        // Update loop iteration count
        sqlx::query(
            r#"
            UPDATE proj.loops
            SET iteration_count = iteration_count + 1, last_event_id = $1, last_global_seq = $2
            WHERE loop_id = $3
            "#,
        )
        .bind(event.event_id.as_str())
        .bind(event.global_seq.unwrap_or(0) as i64)
        .bind(loop_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    async fn apply_iteration_completed(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        event: &EventEnvelope,
    ) -> Result<(), ProjectionError> {
        let payload = &event.payload;
        let state = payload["outcome"]
            .as_str()
            .map(|s| if s == "SUCCESS" { "COMPLETED" } else { "FAILED" })
            .unwrap_or("COMPLETED");

        sqlx::query(
            r#"
            UPDATE proj.iterations
            SET state = $1, completed_at = $2, last_event_id = $3, last_global_seq = $4
            WHERE iteration_id = $5
            "#,
        )
        .bind(state)
        .bind(event.occurred_at)
        .bind(event.event_id.as_str())
        .bind(event.global_seq.unwrap_or(0) as i64)
        .bind(&event.stream_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    async fn apply_iteration_summary_recorded(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        event: &EventEnvelope,
    ) -> Result<(), ProjectionError> {
        let payload = &event.payload;
        let summary = &payload["summary"];

        sqlx::query(
            r#"
            UPDATE proj.iterations
            SET summary = $1, last_event_id = $2, last_global_seq = $3
            WHERE iteration_id = $4
            "#,
        )
        .bind(summary)
        .bind(event.event_id.as_str())
        .bind(event.global_seq.unwrap_or(0) as i64)
        .bind(&event.stream_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    // ========================================================================
    // Candidate Event Handlers
    // ========================================================================

    async fn apply_candidate_materialized(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        event: &EventEnvelope,
    ) -> Result<(), ProjectionError> {
        let payload = &event.payload;
        let candidate_id = &event.stream_id;

        let content_hash = payload["content_hash"].as_str().unwrap_or("");
        let produced_by = payload["produced_by_iteration_id"].as_str();
        let refs = serde_json::to_value(&event.refs).unwrap_or_default();

        sqlx::query(
            r#"
            INSERT INTO proj.candidates (
                candidate_id, content_hash, produced_by_iteration_id,
                verification_status, created_at, refs, last_event_id, last_global_seq
            ) VALUES ($1, $2, $3, 'UNVERIFIED', $4, $5, $6, $7)
            ON CONFLICT (candidate_id) DO UPDATE
            SET content_hash = $2, last_event_id = $6, last_global_seq = $7
            "#,
        )
        .bind(candidate_id)
        .bind(content_hash)
        .bind(produced_by)
        .bind(event.occurred_at)
        .bind(refs)
        .bind(event.event_id.as_str())
        .bind(event.global_seq.unwrap_or(0) as i64)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    async fn apply_candidate_verification_computed(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        event: &EventEnvelope,
    ) -> Result<(), ProjectionError> {
        let payload = &event.payload;
        let status = payload["verification_status"]
            .as_str()
            .unwrap_or("UNVERIFIED");

        sqlx::query(
            r#"
            UPDATE proj.candidates
            SET verification_status = $1, last_event_id = $2, last_global_seq = $3
            WHERE candidate_id = $4
            "#,
        )
        .bind(status)
        .bind(event.event_id.as_str())
        .bind(event.global_seq.unwrap_or(0) as i64)
        .bind(&event.stream_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    // ========================================================================
    // Run Event Handlers
    // ========================================================================

    async fn apply_run_started(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        event: &EventEnvelope,
    ) -> Result<(), ProjectionError> {
        let payload = &event.payload;
        let run_id = &event.stream_id;

        let candidate_id = payload["candidate_id"].as_str().unwrap_or("");
        let oracle_suite_id = payload["oracle_suite_id"].as_str().unwrap_or("");
        let oracle_suite_hash = payload["oracle_suite_hash"].as_str().unwrap_or("");

        sqlx::query(
            r#"
            INSERT INTO proj.runs (
                run_id, candidate_id, oracle_suite_id, oracle_suite_hash,
                state, started_at, actor_kind, actor_id, last_event_id, last_global_seq
            ) VALUES ($1, $2, $3, $4, 'STARTED', $5, $6, $7, $8, $9)
            "#,
        )
        .bind(run_id)
        .bind(candidate_id)
        .bind(oracle_suite_id)
        .bind(oracle_suite_hash)
        .bind(event.occurred_at)
        .bind(actor_kind_str(&event.actor_kind))
        .bind(&event.actor_id)
        .bind(event.event_id.as_str())
        .bind(event.global_seq.unwrap_or(0) as i64)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    async fn apply_run_completed(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        event: &EventEnvelope,
    ) -> Result<(), ProjectionError> {
        let payload = &event.payload;
        let state = payload["outcome"]
            .as_str()
            .map(|s| if s == "SUCCESS" { "COMPLETED" } else { "FAILED" })
            .unwrap_or("COMPLETED");
        let evidence_hash = payload["evidence_bundle_hash"].as_str();

        sqlx::query(
            r#"
            UPDATE proj.runs
            SET state = $1, completed_at = $2, evidence_bundle_hash = $3,
                last_event_id = $4, last_global_seq = $5
            WHERE run_id = $6
            "#,
        )
        .bind(state)
        .bind(event.occurred_at)
        .bind(evidence_hash)
        .bind(event.event_id.as_str())
        .bind(event.global_seq.unwrap_or(0) as i64)
        .bind(&event.stream_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    // ========================================================================
    // Approval Event Handlers
    // ========================================================================

    async fn apply_approval_recorded(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        event: &EventEnvelope,
    ) -> Result<(), ProjectionError> {
        let payload = &event.payload;
        let approval_id = &event.stream_id;

        let portal_id = payload["portal_id"].as_str().unwrap_or("");
        let decision = payload["decision"].as_str().unwrap_or("APPROVED");
        let subject_refs = &payload["subject_refs"];
        let evidence_refs: Vec<&str> = payload["evidence_refs"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();
        let exceptions_ack: Vec<&str> = payload["exceptions_acknowledged"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();
        let rationale = payload["rationale"].as_str();

        sqlx::query(
            r#"
            INSERT INTO proj.approvals (
                approval_id, portal_id, decision, subject_refs, evidence_refs,
                exceptions_acknowledged, rationale, approved_by_kind, approved_by_id,
                approved_at, last_event_id, last_global_seq
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
        )
        .bind(approval_id)
        .bind(portal_id)
        .bind(decision)
        .bind(subject_refs)
        .bind(&evidence_refs)
        .bind(&exceptions_ack)
        .bind(rationale)
        .bind(actor_kind_str(&event.actor_kind))
        .bind(&event.actor_id)
        .bind(event.occurred_at)
        .bind(event.event_id.as_str())
        .bind(event.global_seq.unwrap_or(0) as i64)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    // ========================================================================
    // Exception Event Handlers
    // ========================================================================

    async fn apply_exception_created(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        event: &EventEnvelope,
    ) -> Result<(), ProjectionError> {
        let payload = &event.payload;
        let exception_id = &event.stream_id;

        let kind = match event.event_type.as_str() {
            "DeviationCreated" => "DEVIATION",
            "DeferralCreated" => "DEFERRAL",
            "WaiverCreated" => "WAIVER",
            _ => "DEVIATION",
        };

        let scope = &payload["scope"];
        let rationale = payload["rationale"].as_str().unwrap_or("");
        let target_description = payload["target_description"].as_str().unwrap_or("");
        let expires_at = payload["expires_at"]
            .as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        sqlx::query(
            r#"
            INSERT INTO proj.exceptions (
                exception_id, kind, status, scope, rationale, target_description,
                created_by_kind, created_by_id, created_at, expires_at,
                last_event_id, last_global_seq
            ) VALUES ($1, $2, 'CREATED', $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(exception_id)
        .bind(kind)
        .bind(scope)
        .bind(rationale)
        .bind(target_description)
        .bind(actor_kind_str(&event.actor_kind))
        .bind(&event.actor_id)
        .bind(event.occurred_at)
        .bind(expires_at)
        .bind(event.event_id.as_str())
        .bind(event.global_seq.unwrap_or(0) as i64)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    async fn apply_exception_activated(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        event: &EventEnvelope,
    ) -> Result<(), ProjectionError> {
        sqlx::query(
            r#"
            UPDATE proj.exceptions
            SET status = 'ACTIVE', last_event_id = $1, last_global_seq = $2
            WHERE exception_id = $3
            "#,
        )
        .bind(event.event_id.as_str())
        .bind(event.global_seq.unwrap_or(0) as i64)
        .bind(&event.stream_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    async fn apply_exception_resolved(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        event: &EventEnvelope,
    ) -> Result<(), ProjectionError> {
        sqlx::query(
            r#"
            UPDATE proj.exceptions
            SET status = 'RESOLVED', resolved_at = $1, resolved_by_kind = $2, resolved_by_id = $3,
                last_event_id = $4, last_global_seq = $5
            WHERE exception_id = $6
            "#,
        )
        .bind(event.occurred_at)
        .bind(actor_kind_str(&event.actor_kind))
        .bind(&event.actor_id)
        .bind(event.event_id.as_str())
        .bind(event.global_seq.unwrap_or(0) as i64)
        .bind(&event.stream_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    async fn apply_exception_expired(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        event: &EventEnvelope,
    ) -> Result<(), ProjectionError> {
        sqlx::query(
            r#"
            UPDATE proj.exceptions
            SET status = 'EXPIRED', last_event_id = $1, last_global_seq = $2
            WHERE exception_id = $3
            "#,
        )
        .bind(event.event_id.as_str())
        .bind(event.global_seq.unwrap_or(0) as i64)
        .bind(&event.stream_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    // ========================================================================
    // Freeze Event Handlers
    // ========================================================================

    async fn apply_freeze_record_created(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        event: &EventEnvelope,
    ) -> Result<(), ProjectionError> {
        let payload = &event.payload;
        let freeze_id = &event.stream_id;

        let baseline_id = payload["baseline_id"].as_str().unwrap_or("");
        let candidate_id = payload["candidate_id"].as_str().unwrap_or("");
        let verification_mode = payload["verification_mode"].as_str().unwrap_or("STRICT");
        let oracle_suite_id = payload["oracle_suite_id"].as_str().unwrap_or("");
        let oracle_suite_hash = payload["oracle_suite_hash"].as_str().unwrap_or("");
        let evidence_refs: Vec<&str> = payload["evidence_bundle_refs"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();
        let waiver_refs: Vec<&str> = payload["waiver_refs"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();
        let release_approval_id = payload["release_approval_id"].as_str().unwrap_or("");
        let artifact_manifest = &payload["artifact_manifest"];
        let active_exceptions = &payload["active_exceptions"];

        sqlx::query(
            r#"
            INSERT INTO proj.freeze_records (
                freeze_id, baseline_id, candidate_id, verification_mode,
                oracle_suite_id, oracle_suite_hash, evidence_bundle_refs, waiver_refs,
                release_approval_id, artifact_manifest, active_exceptions,
                frozen_by_kind, frozen_by_id, frozen_at, last_event_id, last_global_seq
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            "#,
        )
        .bind(freeze_id)
        .bind(baseline_id)
        .bind(candidate_id)
        .bind(verification_mode)
        .bind(oracle_suite_id)
        .bind(oracle_suite_hash)
        .bind(&evidence_refs)
        .bind(&waiver_refs)
        .bind(release_approval_id)
        .bind(artifact_manifest)
        .bind(active_exceptions)
        .bind(actor_kind_str(&event.actor_kind))
        .bind(&event.actor_id)
        .bind(event.occurred_at)
        .bind(event.event_id.as_str())
        .bind(event.global_seq.unwrap_or(0) as i64)
        .execute(&mut **tx)
        .await?;

        // Update shippable status for the candidate
        sqlx::query(
            r#"
            INSERT INTO proj.shippable_status (
                candidate_id, is_verified, verification_mode, latest_evidence_hash,
                release_approval_id, freeze_id, has_unresolved_staleness,
                computed_at, last_event_id, last_global_seq
            ) VALUES ($1, TRUE, $2, $3, $4, $5, FALSE, $6, $7, $8)
            ON CONFLICT (candidate_id) DO UPDATE
            SET is_verified = TRUE, verification_mode = $2, freeze_id = $5,
                release_approval_id = $4, computed_at = $6, last_event_id = $7, last_global_seq = $8
            "#,
        )
        .bind(candidate_id)
        .bind(verification_mode)
        .bind(evidence_refs.first().copied())
        .bind(release_approval_id)
        .bind(freeze_id)
        .bind(event.occurred_at)
        .bind(event.event_id.as_str())
        .bind(event.global_seq.unwrap_or(0) as i64)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    // ========================================================================
    // Decision Event Handlers
    // ========================================================================

    async fn apply_decision_recorded(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        event: &EventEnvelope,
    ) -> Result<(), ProjectionError> {
        let payload = &event.payload;
        let decision_id = &event.stream_id;

        let trigger = payload["trigger"].as_str().unwrap_or("");
        let scope = &payload["scope"];
        let decision = payload["decision"].as_str().unwrap_or("");
        let rationale = payload["rationale"].as_str().unwrap_or("");
        let is_precedent = payload["is_precedent"].as_bool().unwrap_or(false);
        let applicability = payload["applicability"].as_str();
        let evidence_refs: Vec<&str> = payload["evidence_refs"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();
        let exceptions_ack = &payload["exceptions_acknowledged"];

        sqlx::query(
            r#"
            INSERT INTO proj.decisions (
                decision_id, trigger, scope, decision, rationale, is_precedent,
                applicability, evidence_refs, exceptions_acknowledged,
                decided_by_kind, decided_by_id, decided_at, last_event_id, last_global_seq
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#,
        )
        .bind(decision_id)
        .bind(trigger)
        .bind(scope)
        .bind(decision)
        .bind(rationale)
        .bind(is_precedent)
        .bind(applicability)
        .bind(&evidence_refs)
        .bind(exceptions_ack)
        .bind(actor_kind_str(&event.actor_kind))
        .bind(&event.actor_id)
        .bind(event.occurred_at)
        .bind(event.event_id.as_str())
        .bind(event.global_seq.unwrap_or(0) as i64)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    // ========================================================================
    // Governed Artifact Event Handlers
    // ========================================================================

    async fn apply_governed_artifact_recorded(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        event: &EventEnvelope,
    ) -> Result<(), ProjectionError> {
        let payload = &event.payload;

        let artifact_id = payload["artifact_id"].as_str().unwrap_or("");
        let artifact_type = payload["artifact_type"].as_str().unwrap_or("");
        let version = payload["version"].as_str().unwrap_or("");
        let content_hash = payload["content_hash"].as_str().unwrap_or("");
        let status = payload["status"].as_str().unwrap_or("draft");
        let normative_status = payload["normative_status"].as_str().unwrap_or("directional");
        let authority_kind = payload["authority_kind"].as_str().unwrap_or("content");
        let governed_by: Vec<&str> = payload["governed_by"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();
        let tags: Vec<&str> = payload["tags"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();
        let supersedes: Vec<&str> = payload["supersedes"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();
        let is_current = payload["is_current"].as_bool().unwrap_or(true);

        // If this is the current version, mark previous versions as not current
        if is_current {
            sqlx::query(
                r#"
                UPDATE proj.governed_artifacts
                SET is_current = FALSE, last_event_id = $1, last_global_seq = $2
                WHERE artifact_id = $3 AND is_current = TRUE
                "#,
            )
            .bind(event.event_id.as_str())
            .bind(event.global_seq.unwrap_or(0) as i64)
            .bind(artifact_id)
            .execute(&mut **tx)
            .await?;
        }

        sqlx::query(
            r#"
            INSERT INTO proj.governed_artifacts (
                artifact_id, artifact_type, version, content_hash, status,
                normative_status, authority_kind, governed_by, tags, supersedes,
                is_current, recorded_at, recorded_by_kind, recorded_by_id,
                last_event_id, last_global_seq
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            ON CONFLICT (artifact_id, version) DO UPDATE
            SET content_hash = $4, status = $5, is_current = $11,
                last_event_id = $15, last_global_seq = $16
            "#,
        )
        .bind(artifact_id)
        .bind(artifact_type)
        .bind(version)
        .bind(content_hash)
        .bind(status)
        .bind(normative_status)
        .bind(authority_kind)
        .bind(&governed_by)
        .bind(&tags)
        .bind(&supersedes)
        .bind(is_current)
        .bind(event.occurred_at)
        .bind(actor_kind_str(&event.actor_kind))
        .bind(&event.actor_id)
        .bind(event.event_id.as_str())
        .bind(event.global_seq.unwrap_or(0) as i64)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}

/// Convert ActorKind to database string
fn actor_kind_str(kind: &sr_domain::ActorKind) -> &'static str {
    match kind {
        sr_domain::ActorKind::Human => "HUMAN",
        sr_domain::ActorKind::Agent => "AGENT",
        sr_domain::ActorKind::System => "SYSTEM",
    }
}

// ============================================================================
// Query helpers for reading projections
// ============================================================================

/// Loop projection read model
#[derive(Debug, Clone)]
pub struct LoopProjection {
    pub loop_id: String,
    pub goal: String,
    pub work_unit: String,
    pub state: String,
    pub budgets: serde_json::Value,
    pub directive_ref: serde_json::Value,
    pub created_by_kind: String,
    pub created_by_id: String,
    pub created_at: DateTime<Utc>,
    pub activated_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub iteration_count: i32,
}

/// Iteration projection read model
#[derive(Debug, Clone)]
pub struct IterationProjection {
    pub iteration_id: String,
    pub loop_id: String,
    pub sequence: i32,
    pub state: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub refs: serde_json::Value,
    pub summary: Option<serde_json::Value>,
}

/// Candidate projection read model
#[derive(Debug, Clone)]
pub struct CandidateProjection {
    pub candidate_id: String,
    pub content_hash: String,
    pub produced_by_iteration_id: Option<String>,
    pub verification_status: String,
    pub created_at: DateTime<Utc>,
    pub refs: serde_json::Value,
}

/// Run projection read model
#[derive(Debug, Clone)]
pub struct RunProjection {
    pub run_id: String,
    pub candidate_id: String,
    pub oracle_suite_id: String,
    pub oracle_suite_hash: String,
    pub state: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub actor_kind: String,
    pub actor_id: String,
    pub evidence_bundle_hash: Option<String>,
}

/// Approval projection read model
#[derive(Debug, Clone)]
pub struct ApprovalProjection {
    pub approval_id: String,
    pub portal_id: String,
    pub decision: String,
    pub subject_refs: serde_json::Value,
    pub evidence_refs: Vec<String>,
    pub exceptions_acknowledged: Vec<String>,
    pub rationale: Option<String>,
    pub approved_by_kind: String,
    pub approved_by_id: String,
    pub approved_at: DateTime<Utc>,
}

/// Exception projection read model
#[derive(Debug, Clone)]
pub struct ExceptionProjection {
    pub exception_id: String,
    pub kind: String,
    pub status: String,
    pub scope: serde_json::Value,
    pub rationale: String,
    pub target_description: String,
    pub created_by_kind: String,
    pub created_by_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by_kind: Option<String>,
    pub resolved_by_id: Option<String>,
}

/// Decision projection read model
#[derive(Debug, Clone)]
pub struct DecisionProjection {
    pub decision_id: String,
    pub trigger: String,
    pub scope: serde_json::Value,
    pub decision: String,
    pub rationale: String,
    pub is_precedent: bool,
    pub applicability: Option<String>,
    pub evidence_refs: Vec<String>,
    pub exceptions_acknowledged: serde_json::Value,
    pub decided_by_kind: String,
    pub decided_by_id: String,
    pub decided_at: DateTime<Utc>,
}

/// Freeze record projection read model
#[derive(Debug, Clone)]
pub struct FreezeRecordProjection {
    pub freeze_id: String,
    pub baseline_id: String,
    pub candidate_id: String,
    pub verification_mode: String,
    pub oracle_suite_id: String,
    pub oracle_suite_hash: String,
    pub evidence_bundle_refs: Vec<String>,
    pub waiver_refs: Vec<String>,
    pub release_approval_id: String,
    pub artifact_manifest: serde_json::Value,
    pub active_exceptions: serde_json::Value,
    pub frozen_by_kind: String,
    pub frozen_by_id: String,
    pub frozen_at: DateTime<Utc>,
}

impl ProjectionBuilder {
    /// Get a loop by ID
    pub async fn get_loop(&self, loop_id: &str) -> Result<Option<LoopProjection>, ProjectionError> {
        let result = sqlx::query(
            r#"
            SELECT loop_id, goal, work_unit, state, budgets, directive_ref,
                   created_by_kind, created_by_id, created_at, activated_at,
                   closed_at, iteration_count
            FROM proj.loops WHERE loop_id = $1
            "#,
        )
        .bind(loop_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|row| LoopProjection {
            loop_id: row.get("loop_id"),
            goal: row.get("goal"),
            work_unit: row.get("work_unit"),
            state: row.get("state"),
            budgets: row.get("budgets"),
            directive_ref: row.get("directive_ref"),
            created_by_kind: row.get("created_by_kind"),
            created_by_id: row.get("created_by_id"),
            created_at: row.get("created_at"),
            activated_at: row.get("activated_at"),
            closed_at: row.get("closed_at"),
            iteration_count: row.get("iteration_count"),
        }))
    }

    /// Get iterations for a loop
    pub async fn get_iterations(
        &self,
        loop_id: &str,
    ) -> Result<Vec<IterationProjection>, ProjectionError> {
        let rows = sqlx::query(
            r#"
            SELECT iteration_id, loop_id, sequence, state, started_at,
                   completed_at, refs, summary
            FROM proj.iterations WHERE loop_id = $1
            ORDER BY sequence ASC
            "#,
        )
        .bind(loop_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| IterationProjection {
                iteration_id: row.get("iteration_id"),
                loop_id: row.get("loop_id"),
                sequence: row.get("sequence"),
                state: row.get("state"),
                started_at: row.get("started_at"),
                completed_at: row.get("completed_at"),
                refs: row.get("refs"),
                summary: row.get("summary"),
            })
            .collect())
    }

    /// Get a candidate by ID
    pub async fn get_candidate(
        &self,
        candidate_id: &str,
    ) -> Result<Option<CandidateProjection>, ProjectionError> {
        let result = sqlx::query(
            r#"
            SELECT candidate_id, content_hash, produced_by_iteration_id,
                   verification_status, created_at, refs
            FROM proj.candidates WHERE candidate_id = $1
            "#,
        )
        .bind(candidate_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|row| CandidateProjection {
            candidate_id: row.get("candidate_id"),
            content_hash: row.get("content_hash"),
            produced_by_iteration_id: row.get("produced_by_iteration_id"),
            verification_status: row.get("verification_status"),
            created_at: row.get("created_at"),
            refs: row.get("refs"),
        }))
    }

    /// Get runs for a candidate
    pub async fn get_runs_for_candidate(
        &self,
        candidate_id: &str,
    ) -> Result<Vec<RunProjection>, ProjectionError> {
        let rows = sqlx::query(
            r#"
            SELECT run_id, candidate_id, oracle_suite_id, oracle_suite_hash,
                   state, started_at, completed_at, actor_kind, actor_id,
                   evidence_bundle_hash
            FROM proj.runs WHERE candidate_id = $1
            ORDER BY started_at ASC
            "#,
        )
        .bind(candidate_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| RunProjection {
                run_id: row.get("run_id"),
                candidate_id: row.get("candidate_id"),
                oracle_suite_id: row.get("oracle_suite_id"),
                oracle_suite_hash: row.get("oracle_suite_hash"),
                state: row.get("state"),
                started_at: row.get("started_at"),
                completed_at: row.get("completed_at"),
                actor_kind: row.get("actor_kind"),
                actor_id: row.get("actor_id"),
                evidence_bundle_hash: row.get("evidence_bundle_hash"),
            })
            .collect())
    }

    /// List loops with optional state filter
    pub async fn list_loops(
        &self,
        state: &Option<String>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<LoopProjection>, ProjectionError> {
        let rows = if let Some(s) = state {
            sqlx::query(
                r#"
                SELECT loop_id, goal, work_unit, state, budgets, directive_ref,
                       created_by_kind, created_by_id, created_at, activated_at,
                       closed_at, iteration_count
                FROM proj.loops WHERE state = $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(s)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query(
                r#"
                SELECT loop_id, goal, work_unit, state, budgets, directive_ref,
                       created_by_kind, created_by_id, created_at, activated_at,
                       closed_at, iteration_count
                FROM proj.loops
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
                "#,
            )
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await?
        };

        Ok(rows
            .into_iter()
            .map(|row| LoopProjection {
                loop_id: row.get("loop_id"),
                goal: row.get("goal"),
                work_unit: row.get("work_unit"),
                state: row.get("state"),
                budgets: row.get("budgets"),
                directive_ref: row.get("directive_ref"),
                created_by_kind: row.get("created_by_kind"),
                created_by_id: row.get("created_by_id"),
                created_at: row.get("created_at"),
                activated_at: row.get("activated_at"),
                closed_at: row.get("closed_at"),
                iteration_count: row.get("iteration_count"),
            })
            .collect())
    }

    /// Get an iteration by ID
    pub async fn get_iteration(
        &self,
        iteration_id: &str,
    ) -> Result<Option<IterationProjection>, ProjectionError> {
        let result = sqlx::query(
            r#"
            SELECT iteration_id, loop_id, sequence, state, started_at,
                   completed_at, refs, summary
            FROM proj.iterations WHERE iteration_id = $1
            "#,
        )
        .bind(iteration_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|row| IterationProjection {
            iteration_id: row.get("iteration_id"),
            loop_id: row.get("loop_id"),
            sequence: row.get("sequence"),
            state: row.get("state"),
            started_at: row.get("started_at"),
            completed_at: row.get("completed_at"),
            refs: row.get("refs"),
            summary: row.get("summary"),
        }))
    }

    /// Get candidates produced by an iteration
    pub async fn get_candidates_for_iteration(
        &self,
        iteration_id: &str,
    ) -> Result<Vec<CandidateProjection>, ProjectionError> {
        let rows = sqlx::query(
            r#"
            SELECT candidate_id, content_hash, produced_by_iteration_id,
                   verification_status, created_at, refs
            FROM proj.candidates WHERE produced_by_iteration_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(iteration_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| CandidateProjection {
                candidate_id: row.get("candidate_id"),
                content_hash: row.get("content_hash"),
                produced_by_iteration_id: row.get("produced_by_iteration_id"),
                verification_status: row.get("verification_status"),
                created_at: row.get("created_at"),
                refs: row.get("refs"),
            })
            .collect())
    }

    /// List candidates with optional verification status filter
    pub async fn list_candidates(
        &self,
        verification_status: Option<&str>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<CandidateProjection>, ProjectionError> {
        let rows = if let Some(status) = verification_status {
            sqlx::query(
                r#"
                SELECT candidate_id, content_hash, produced_by_iteration_id,
                       verification_status, created_at, refs
                FROM proj.candidates WHERE verification_status = $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(status)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query(
                r#"
                SELECT candidate_id, content_hash, produced_by_iteration_id,
                       verification_status, created_at, refs
                FROM proj.candidates
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
                "#,
            )
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await?
        };

        Ok(rows
            .into_iter()
            .map(|row| CandidateProjection {
                candidate_id: row.get("candidate_id"),
                content_hash: row.get("content_hash"),
                produced_by_iteration_id: row.get("produced_by_iteration_id"),
                verification_status: row.get("verification_status"),
                created_at: row.get("created_at"),
                refs: row.get("refs"),
            })
            .collect())
    }

    /// Get a run by ID
    pub async fn get_run(&self, run_id: &str) -> Result<Option<RunProjection>, ProjectionError> {
        let result = sqlx::query(
            r#"
            SELECT run_id, candidate_id, oracle_suite_id, oracle_suite_hash,
                   state, started_at, completed_at, actor_kind, actor_id,
                   evidence_bundle_hash
            FROM proj.runs WHERE run_id = $1
            "#,
        )
        .bind(run_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|row| RunProjection {
            run_id: row.get("run_id"),
            candidate_id: row.get("candidate_id"),
            oracle_suite_id: row.get("oracle_suite_id"),
            oracle_suite_hash: row.get("oracle_suite_hash"),
            state: row.get("state"),
            started_at: row.get("started_at"),
            completed_at: row.get("completed_at"),
            actor_kind: row.get("actor_kind"),
            actor_id: row.get("actor_id"),
            evidence_bundle_hash: row.get("evidence_bundle_hash"),
        }))
    }

    /// List runs with optional state filter
    pub async fn list_runs(
        &self,
        state: Option<&str>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<RunProjection>, ProjectionError> {
        let rows = if let Some(s) = state {
            sqlx::query(
                r#"
                SELECT run_id, candidate_id, oracle_suite_id, oracle_suite_hash,
                       state, started_at, completed_at, actor_kind, actor_id,
                       evidence_bundle_hash
                FROM proj.runs WHERE state = $1
                ORDER BY started_at DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(s)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query(
                r#"
                SELECT run_id, candidate_id, oracle_suite_id, oracle_suite_hash,
                       state, started_at, completed_at, actor_kind, actor_id,
                       evidence_bundle_hash
                FROM proj.runs
                ORDER BY started_at DESC
                LIMIT $1 OFFSET $2
                "#,
            )
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await?
        };

        Ok(rows
            .into_iter()
            .map(|row| RunProjection {
                run_id: row.get("run_id"),
                candidate_id: row.get("candidate_id"),
                oracle_suite_id: row.get("oracle_suite_id"),
                oracle_suite_hash: row.get("oracle_suite_hash"),
                state: row.get("state"),
                started_at: row.get("started_at"),
                completed_at: row.get("completed_at"),
                actor_kind: row.get("actor_kind"),
                actor_id: row.get("actor_id"),
                evidence_bundle_hash: row.get("evidence_bundle_hash"),
            })
            .collect())
    }

    // ========================================================================
    // Approval Query Methods
    // ========================================================================

    /// Get an approval by ID
    pub async fn get_approval(
        &self,
        approval_id: &str,
    ) -> Result<Option<ApprovalProjection>, ProjectionError> {
        let result = sqlx::query(
            r#"
            SELECT approval_id, portal_id, decision, subject_refs, evidence_refs,
                   exceptions_acknowledged, rationale, approved_by_kind, approved_by_id,
                   approved_at
            FROM proj.approvals WHERE approval_id = $1
            "#,
        )
        .bind(approval_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|row| ApprovalProjection {
            approval_id: row.get("approval_id"),
            portal_id: row.get("portal_id"),
            decision: row.get("decision"),
            subject_refs: row.get("subject_refs"),
            evidence_refs: row.get("evidence_refs"),
            exceptions_acknowledged: row.get("exceptions_acknowledged"),
            rationale: row.get("rationale"),
            approved_by_kind: row.get("approved_by_kind"),
            approved_by_id: row.get("approved_by_id"),
            approved_at: row.get("approved_at"),
        }))
    }

    /// Get approvals for a portal
    pub async fn get_approvals_for_portal(
        &self,
        portal_id: &str,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<ApprovalProjection>, ProjectionError> {
        let rows = sqlx::query(
            r#"
            SELECT approval_id, portal_id, decision, subject_refs, evidence_refs,
                   exceptions_acknowledged, rationale, approved_by_kind, approved_by_id,
                   approved_at
            FROM proj.approvals WHERE portal_id = $1
            ORDER BY approved_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(portal_id)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| ApprovalProjection {
                approval_id: row.get("approval_id"),
                portal_id: row.get("portal_id"),
                decision: row.get("decision"),
                subject_refs: row.get("subject_refs"),
                evidence_refs: row.get("evidence_refs"),
                exceptions_acknowledged: row.get("exceptions_acknowledged"),
                rationale: row.get("rationale"),
                approved_by_kind: row.get("approved_by_kind"),
                approved_by_id: row.get("approved_by_id"),
                approved_at: row.get("approved_at"),
            })
            .collect())
    }

    /// List approvals
    pub async fn list_approvals(
        &self,
        decision: Option<&str>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<ApprovalProjection>, ProjectionError> {
        let rows = if let Some(d) = decision {
            sqlx::query(
                r#"
                SELECT approval_id, portal_id, decision, subject_refs, evidence_refs,
                       exceptions_acknowledged, rationale, approved_by_kind, approved_by_id,
                       approved_at
                FROM proj.approvals WHERE decision = $1
                ORDER BY approved_at DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(d)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query(
                r#"
                SELECT approval_id, portal_id, decision, subject_refs, evidence_refs,
                       exceptions_acknowledged, rationale, approved_by_kind, approved_by_id,
                       approved_at
                FROM proj.approvals
                ORDER BY approved_at DESC
                LIMIT $1 OFFSET $2
                "#,
            )
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await?
        };

        Ok(rows
            .into_iter()
            .map(|row| ApprovalProjection {
                approval_id: row.get("approval_id"),
                portal_id: row.get("portal_id"),
                decision: row.get("decision"),
                subject_refs: row.get("subject_refs"),
                evidence_refs: row.get("evidence_refs"),
                exceptions_acknowledged: row.get("exceptions_acknowledged"),
                rationale: row.get("rationale"),
                approved_by_kind: row.get("approved_by_kind"),
                approved_by_id: row.get("approved_by_id"),
                approved_at: row.get("approved_at"),
            })
            .collect())
    }

    // ========================================================================
    // Exception Query Methods
    // ========================================================================

    /// Get an exception by ID
    pub async fn get_exception(
        &self,
        exception_id: &str,
    ) -> Result<Option<ExceptionProjection>, ProjectionError> {
        let result = sqlx::query(
            r#"
            SELECT exception_id, kind, status, scope, rationale, target_description,
                   created_by_kind, created_by_id, created_at, expires_at,
                   resolved_at, resolved_by_kind, resolved_by_id
            FROM proj.exceptions WHERE exception_id = $1
            "#,
        )
        .bind(exception_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|row| ExceptionProjection {
            exception_id: row.get("exception_id"),
            kind: row.get("kind"),
            status: row.get("status"),
            scope: row.get("scope"),
            rationale: row.get("rationale"),
            target_description: row.get("target_description"),
            created_by_kind: row.get("created_by_kind"),
            created_by_id: row.get("created_by_id"),
            created_at: row.get("created_at"),
            expires_at: row.get("expires_at"),
            resolved_at: row.get("resolved_at"),
            resolved_by_kind: row.get("resolved_by_kind"),
            resolved_by_id: row.get("resolved_by_id"),
        }))
    }

    /// List exceptions
    pub async fn list_exceptions(
        &self,
        kind: Option<&str>,
        status: Option<&str>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<ExceptionProjection>, ProjectionError> {
        let rows = match (kind, status) {
            (Some(k), Some(s)) => {
                sqlx::query(
                    r#"
                    SELECT exception_id, kind, status, scope, rationale, target_description,
                           created_by_kind, created_by_id, created_at, expires_at,
                           resolved_at, resolved_by_kind, resolved_by_id
                    FROM proj.exceptions WHERE kind = $1 AND status = $2
                    ORDER BY created_at DESC
                    LIMIT $3 OFFSET $4
                    "#,
                )
                .bind(k)
                .bind(s)
                .bind(limit as i64)
                .bind(offset as i64)
                .fetch_all(&self.pool)
                .await?
            }
            (Some(k), None) => {
                sqlx::query(
                    r#"
                    SELECT exception_id, kind, status, scope, rationale, target_description,
                           created_by_kind, created_by_id, created_at, expires_at,
                           resolved_at, resolved_by_kind, resolved_by_id
                    FROM proj.exceptions WHERE kind = $1
                    ORDER BY created_at DESC
                    LIMIT $2 OFFSET $3
                    "#,
                )
                .bind(k)
                .bind(limit as i64)
                .bind(offset as i64)
                .fetch_all(&self.pool)
                .await?
            }
            (None, Some(s)) => {
                sqlx::query(
                    r#"
                    SELECT exception_id, kind, status, scope, rationale, target_description,
                           created_by_kind, created_by_id, created_at, expires_at,
                           resolved_at, resolved_by_kind, resolved_by_id
                    FROM proj.exceptions WHERE status = $1
                    ORDER BY created_at DESC
                    LIMIT $2 OFFSET $3
                    "#,
                )
                .bind(s)
                .bind(limit as i64)
                .bind(offset as i64)
                .fetch_all(&self.pool)
                .await?
            }
            (None, None) => {
                sqlx::query(
                    r#"
                    SELECT exception_id, kind, status, scope, rationale, target_description,
                           created_by_kind, created_by_id, created_at, expires_at,
                           resolved_at, resolved_by_kind, resolved_by_id
                    FROM proj.exceptions
                    ORDER BY created_at DESC
                    LIMIT $1 OFFSET $2
                    "#,
                )
                .bind(limit as i64)
                .bind(offset as i64)
                .fetch_all(&self.pool)
                .await?
            }
        };

        Ok(rows
            .into_iter()
            .map(|row| ExceptionProjection {
                exception_id: row.get("exception_id"),
                kind: row.get("kind"),
                status: row.get("status"),
                scope: row.get("scope"),
                rationale: row.get("rationale"),
                target_description: row.get("target_description"),
                created_by_kind: row.get("created_by_kind"),
                created_by_id: row.get("created_by_id"),
                created_at: row.get("created_at"),
                expires_at: row.get("expires_at"),
                resolved_at: row.get("resolved_at"),
                resolved_by_kind: row.get("resolved_by_kind"),
                resolved_by_id: row.get("resolved_by_id"),
            })
            .collect())
    }

    // ========================================================================
    // Decision Query Methods
    // ========================================================================

    /// Get a decision by ID
    pub async fn get_decision(
        &self,
        decision_id: &str,
    ) -> Result<Option<DecisionProjection>, ProjectionError> {
        let result = sqlx::query(
            r#"
            SELECT decision_id, trigger, scope, decision, rationale, is_precedent,
                   applicability, evidence_refs, exceptions_acknowledged,
                   decided_by_kind, decided_by_id, decided_at
            FROM proj.decisions WHERE decision_id = $1
            "#,
        )
        .bind(decision_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|row| DecisionProjection {
            decision_id: row.get("decision_id"),
            trigger: row.get("trigger"),
            scope: row.get("scope"),
            decision: row.get("decision"),
            rationale: row.get("rationale"),
            is_precedent: row.get("is_precedent"),
            applicability: row.get("applicability"),
            evidence_refs: row.get("evidence_refs"),
            exceptions_acknowledged: row.get("exceptions_acknowledged"),
            decided_by_kind: row.get("decided_by_kind"),
            decided_by_id: row.get("decided_by_id"),
            decided_at: row.get("decided_at"),
        }))
    }

    /// List decisions
    pub async fn list_decisions(
        &self,
        is_precedent: Option<bool>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<DecisionProjection>, ProjectionError> {
        let rows = if let Some(precedent) = is_precedent {
            sqlx::query(
                r#"
                SELECT decision_id, trigger, scope, decision, rationale, is_precedent,
                       applicability, evidence_refs, exceptions_acknowledged,
                       decided_by_kind, decided_by_id, decided_at
                FROM proj.decisions WHERE is_precedent = $1
                ORDER BY decided_at DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(precedent)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query(
                r#"
                SELECT decision_id, trigger, scope, decision, rationale, is_precedent,
                       applicability, evidence_refs, exceptions_acknowledged,
                       decided_by_kind, decided_by_id, decided_at
                FROM proj.decisions
                ORDER BY decided_at DESC
                LIMIT $1 OFFSET $2
                "#,
            )
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await?
        };

        Ok(rows
            .into_iter()
            .map(|row| DecisionProjection {
                decision_id: row.get("decision_id"),
                trigger: row.get("trigger"),
                scope: row.get("scope"),
                decision: row.get("decision"),
                rationale: row.get("rationale"),
                is_precedent: row.get("is_precedent"),
                applicability: row.get("applicability"),
                evidence_refs: row.get("evidence_refs"),
                exceptions_acknowledged: row.get("exceptions_acknowledged"),
                decided_by_kind: row.get("decided_by_kind"),
                decided_by_id: row.get("decided_by_id"),
                decided_at: row.get("decided_at"),
            })
            .collect())
    }

    // ========================================================================
    // Freeze Record Query Methods
    // ========================================================================

    /// Get a freeze record by ID
    pub async fn get_freeze_record(
        &self,
        freeze_id: &str,
    ) -> Result<Option<FreezeRecordProjection>, ProjectionError> {
        let result = sqlx::query(
            r#"
            SELECT freeze_id, baseline_id, candidate_id, verification_mode,
                   oracle_suite_id, oracle_suite_hash, evidence_bundle_refs, waiver_refs,
                   release_approval_id, artifact_manifest, active_exceptions,
                   frozen_by_kind, frozen_by_id, frozen_at
            FROM proj.freeze_records WHERE freeze_id = $1
            "#,
        )
        .bind(freeze_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|row| FreezeRecordProjection {
            freeze_id: row.get("freeze_id"),
            baseline_id: row.get("baseline_id"),
            candidate_id: row.get("candidate_id"),
            verification_mode: row.get("verification_mode"),
            oracle_suite_id: row.get("oracle_suite_id"),
            oracle_suite_hash: row.get("oracle_suite_hash"),
            evidence_bundle_refs: row.get("evidence_bundle_refs"),
            waiver_refs: row.get("waiver_refs"),
            release_approval_id: row.get("release_approval_id"),
            artifact_manifest: row.get("artifact_manifest"),
            active_exceptions: row.get("active_exceptions"),
            frozen_by_kind: row.get("frozen_by_kind"),
            frozen_by_id: row.get("frozen_by_id"),
            frozen_at: row.get("frozen_at"),
        }))
    }

    /// Get freeze records for a candidate
    pub async fn get_freeze_records_for_candidate(
        &self,
        candidate_id: &str,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<FreezeRecordProjection>, ProjectionError> {
        let rows = sqlx::query(
            r#"
            SELECT freeze_id, baseline_id, candidate_id, verification_mode,
                   oracle_suite_id, oracle_suite_hash, evidence_bundle_refs, waiver_refs,
                   release_approval_id, artifact_manifest, active_exceptions,
                   frozen_by_kind, frozen_by_id, frozen_at
            FROM proj.freeze_records WHERE candidate_id = $1
            ORDER BY frozen_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(candidate_id)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| FreezeRecordProjection {
                freeze_id: row.get("freeze_id"),
                baseline_id: row.get("baseline_id"),
                candidate_id: row.get("candidate_id"),
                verification_mode: row.get("verification_mode"),
                oracle_suite_id: row.get("oracle_suite_id"),
                oracle_suite_hash: row.get("oracle_suite_hash"),
                evidence_bundle_refs: row.get("evidence_bundle_refs"),
                waiver_refs: row.get("waiver_refs"),
                release_approval_id: row.get("release_approval_id"),
                artifact_manifest: row.get("artifact_manifest"),
                active_exceptions: row.get("active_exceptions"),
                frozen_by_kind: row.get("frozen_by_kind"),
                frozen_by_id: row.get("frozen_by_id"),
                frozen_at: row.get("frozen_at"),
            })
            .collect())
    }

    /// List freeze records
    pub async fn list_freeze_records(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<FreezeRecordProjection>, ProjectionError> {
        let rows = sqlx::query(
            r#"
            SELECT freeze_id, baseline_id, candidate_id, verification_mode,
                   oracle_suite_id, oracle_suite_hash, evidence_bundle_refs, waiver_refs,
                   release_approval_id, artifact_manifest, active_exceptions,
                   frozen_by_kind, frozen_by_id, frozen_at
            FROM proj.freeze_records
            ORDER BY frozen_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| FreezeRecordProjection {
                freeze_id: row.get("freeze_id"),
                baseline_id: row.get("baseline_id"),
                candidate_id: row.get("candidate_id"),
                verification_mode: row.get("verification_mode"),
                oracle_suite_id: row.get("oracle_suite_id"),
                oracle_suite_hash: row.get("oracle_suite_hash"),
                evidence_bundle_refs: row.get("evidence_bundle_refs"),
                waiver_refs: row.get("waiver_refs"),
                release_approval_id: row.get("release_approval_id"),
                artifact_manifest: row.get("artifact_manifest"),
                active_exceptions: row.get("active_exceptions"),
                frozen_by_kind: row.get("frozen_by_kind"),
                frozen_by_id: row.get("frozen_by_id"),
                frozen_at: row.get("frozen_at"),
            })
            .collect())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actor_kind_str() {
        assert_eq!(actor_kind_str(&sr_domain::ActorKind::Human), "HUMAN");
        assert_eq!(actor_kind_str(&sr_domain::ActorKind::Agent), "AGENT");
        assert_eq!(actor_kind_str(&sr_domain::ActorKind::System), "SYSTEM");
    }

    #[test]
    fn test_projection_config_default() {
        let config = ProjectionConfig::default();
        assert_eq!(config.batch_size, 100);
        assert!(!config.rebuild_on_startup);
    }
}

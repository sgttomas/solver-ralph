//! Oracle Execution Worker (SR-PLAN-V8 §V8-2)
//!
//! Implements the event-driven Oracle Execution Worker that:
//! - Subscribes to RunStarted events from NATS
//! - Validates suite hash against registry (TAMPER detection per C-OR-2)
//! - Materializes candidate workspace
//! - Executes oracle suites via PodmanOracleRunner
//! - Emits OracleExecutionStarted and OracleExecutionCompleted events
//!
//! Per SR-PLAN-V8 Amendment A-1: The worker uses event-driven architecture,
//! NOT direct API calls. It subscribes to events and emits events.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sr_domain::{
    entities::ActorKind,
    events::{
        EventEnvelope, EventId, IntegrityViolationDetected, OracleExecutionCompleted,
        OracleExecutionStarted, OracleExecutionStatus, StreamKind,
    },
    integrity::IntegrityCondition as DomainIntegrityCondition,
};
use sr_ports::{
    EventStore, EvidenceStore, MessageBusError, OracleRunResult, OracleRunnerError, OracleStatus,
    OracleSuiteRegistryPort,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, instrument, warn};

use crate::candidate_store::{CandidateWorkspace, WorkspaceError};
use crate::evidence::EvidenceManifest;
use crate::integrity::{IntegrityChecker, IntegrityViolation};
use crate::nats::{streams, subjects, MessageEnvelope, NatsConsumer, NatsMessageBus};
use crate::oracle_runner::PodmanOracleRunner;

// ============================================================================
// Worker Configuration
// ============================================================================

/// Oracle execution worker configuration
#[derive(Debug, Clone)]
pub struct OracleWorkerConfig {
    /// Consumer name for NATS subscription
    pub consumer_name: String,
    /// Batch size for message fetching
    pub batch_size: usize,
    /// Whether to enable test mode (skip actual execution)
    pub test_mode: bool,
    /// Worker ID for attribution
    pub worker_id: String,
}

impl Default for OracleWorkerConfig {
    fn default() -> Self {
        Self {
            consumer_name: "oracle-execution-worker".to_string(),
            batch_size: 10,
            test_mode: false,
            worker_id: format!("oracle_worker_{}", ulid::Ulid::new()),
        }
    }
}

impl OracleWorkerConfig {
    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            consumer_name: std::env::var("ORACLE_WORKER_CONSUMER_NAME")
                .unwrap_or_else(|_| "oracle-execution-worker".to_string()),
            batch_size: std::env::var("ORACLE_WORKER_BATCH_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            test_mode: std::env::var("ORACLE_WORKER_TEST_MODE")
                .map(|s| s == "true" || s == "1")
                .unwrap_or(false),
            worker_id: std::env::var("ORACLE_WORKER_ID")
                .unwrap_or_else(|_| format!("oracle_worker_{}", ulid::Ulid::new())),
        }
    }
}

// ============================================================================
// Worker Error Types
// ============================================================================

/// Oracle execution worker error types
#[derive(Debug, thiserror::Error)]
pub enum OracleWorkerError {
    #[error("Message bus error: {message}")]
    MessageBusError { message: String },

    #[error("Suite not found: {suite_id}")]
    SuiteNotFound { suite_id: String },

    #[error("Suite hash mismatch (TAMPER detected): expected {expected}, got {actual}")]
    SuiteHashMismatch { expected: String, actual: String },

    #[error("Workspace materialization failed: {reason}")]
    WorkspaceFailed { reason: String },

    #[error("Oracle execution failed: {reason}")]
    ExecutionFailed { reason: String },

    #[error("Event store error: {message}")]
    EventStoreError { message: String },

    #[error("Invalid event: {reason}")]
    InvalidEvent { reason: String },

    #[error("Registry error: {message}")]
    RegistryError { message: String },

    #[error("Serialization error: {message}")]
    SerializationError { message: String },
}

impl From<MessageBusError> for OracleWorkerError {
    fn from(e: MessageBusError) -> Self {
        OracleWorkerError::MessageBusError {
            message: e.to_string(),
        }
    }
}

impl From<WorkspaceError> for OracleWorkerError {
    fn from(e: WorkspaceError) -> Self {
        OracleWorkerError::WorkspaceFailed {
            reason: e.to_string(),
        }
    }
}

// ============================================================================
// RunStarted Payload
// ============================================================================

/// Payload from RunStarted event (from runs.rs handler)
#[derive(Debug, Clone, Deserialize)]
struct RunStartedPayload {
    candidate_id: String,
    oracle_suite_id: String,
    oracle_suite_hash: String,
    #[serde(default)]
    loop_id: Option<String>,
    #[serde(default)]
    iteration_id: Option<String>,
}

// ============================================================================
// Oracle Execution Worker
// ============================================================================

/// Oracle execution worker that subscribes to RunStarted events
///
/// Per SR-PLAN-V8 §V8-2: This worker implements the event-driven pattern
/// for oracle execution, subscribing to RunStarted events and emitting
/// OracleExecutionStarted and OracleExecutionCompleted events.
pub struct OracleExecutionWorker<S, R, Ev, C>
where
    S: EventStore + 'static,
    R: OracleSuiteRegistryPort + 'static,
    Ev: EvidenceStore + 'static,
    C: CandidateWorkspace + 'static,
{
    config: OracleWorkerConfig,
    message_bus: Arc<NatsMessageBus>,
    event_store: Arc<S>,
    oracle_registry: Arc<R>,
    oracle_runner: Arc<PodmanOracleRunner<Ev>>,
    candidate_workspace: Arc<C>,
    /// Processed run IDs (for idempotency)
    processed_runs: Arc<RwLock<HashMap<String, DateTime<Utc>>>>,
    /// Integrity checks for oracle outputs
    integrity_checker: IntegrityChecker,
}

impl<S, R, Ev, C> OracleExecutionWorker<S, R, Ev, C>
where
    S: EventStore + 'static,
    R: OracleSuiteRegistryPort + 'static,
    Ev: EvidenceStore + 'static,
    C: CandidateWorkspace + 'static,
{
    /// Create a new oracle execution worker
    pub fn new(
        config: OracleWorkerConfig,
        message_bus: Arc<NatsMessageBus>,
        event_store: Arc<S>,
        oracle_registry: Arc<R>,
        oracle_runner: Arc<PodmanOracleRunner<Ev>>,
        candidate_workspace: Arc<C>,
    ) -> Self {
        Self {
            config,
            message_bus,
            event_store,
            oracle_registry,
            oracle_runner,
            candidate_workspace,
            processed_runs: Arc::new(RwLock::new(HashMap::new())),
            integrity_checker: IntegrityChecker::new(Default::default()),
        }
    }

    /// Resolve loop_id from payload or persisted RunStarted event
    async fn resolve_loop_id(&self, run_id: &str, payload: &RunStartedPayload) -> Option<String> {
        if let Some(loop_id) = &payload.loop_id {
            return Some(loop_id.clone());
        }

        if let Ok(events) = self.event_store.read_stream(run_id, 0, 1).await {
            if let Some(event) = events.first() {
                if let Some(loop_id) = event.payload.get("loop_id").and_then(Value::as_str) {
                    return Some(loop_id.to_string());
                }
            }
        }

        None
    }

    /// Emit StopTriggered for integrity/stop conditions
    async fn emit_stop_triggered(
        &self,
        loop_id: &str,
        trigger: &str,
        recommended_portal: &str,
    ) -> Result<(), OracleWorkerError> {
        let events = self
            .event_store
            .read_stream(loop_id, 0, 1000)
            .await
            .map_err(|e| OracleWorkerError::EventStoreError {
                message: e.to_string(),
            })?;
        let current_version = events.len() as u64;

        let event_id = EventId::new();
        let payload = serde_json::json!({
            "trigger": trigger,
            "condition": trigger,
            "requires_decision": true,
            "recommended_portal": recommended_portal,
        });

        let event = EventEnvelope {
            event_id: event_id.clone(),
            stream_id: loop_id.to_string(),
            stream_kind: StreamKind::Loop,
            stream_seq: current_version + 1,
            global_seq: None,
            event_type: "StopTriggered".to_string(),
            occurred_at: Utc::now(),
            actor_kind: ActorKind::System,
            actor_id: self.config.worker_id.clone(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload,
            envelope_hash: compute_envelope_hash(&event_id),
        };

        self.event_store
            .append(loop_id, current_version, vec![event])
            .await
            .map_err(|e| OracleWorkerError::EventStoreError {
                message: e.to_string(),
            })?;

        info!(
            loop_id = %loop_id,
            trigger = %trigger,
            portal = %recommended_portal,
            "StopTriggered emitted by oracle worker"
        );

        Ok(())
    }

    fn violation_code(condition: &crate::oracle_suite::IntegrityCondition) -> &'static str {
        match condition {
            crate::oracle_suite::IntegrityCondition::OracleTamper => "ORACLE_TAMPER",
            crate::oracle_suite::IntegrityCondition::OracleGap => "ORACLE_GAP",
            crate::oracle_suite::IntegrityCondition::OracleEnvMismatch => "ORACLE_ENV_MISMATCH",
            crate::oracle_suite::IntegrityCondition::OracleFlake => "ORACLE_FLAKE",
            crate::oracle_suite::IntegrityCondition::EvidenceMissing => "EVIDENCE_MISSING",
            crate::oracle_suite::IntegrityCondition::ManifestInvalid => "MANIFEST_INVALID",
        }
    }

    fn map_violation_to_domain_condition(
        &self,
        violation: &IntegrityViolation,
        suite_id: &str,
    ) -> DomainIntegrityCondition {
        match violation.condition {
            crate::oracle_suite::IntegrityCondition::OracleTamper => {
                DomainIntegrityCondition::OracleTamper {
                    expected_hash: violation
                        .context
                        .get("expected_manifest_hash")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string(),
                    actual_hash: violation
                        .context
                        .get("actual_manifest_hash")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string(),
                    suite_id: suite_id.to_string(),
                }
            }
            crate::oracle_suite::IntegrityCondition::OracleGap => {
                DomainIntegrityCondition::OracleGap {
                    missing_oracles: violation
                        .context
                        .get("missing_oracles")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(Value::as_str)
                                .map(ToString::to_string)
                                .collect()
                        })
                        .unwrap_or_default(),
                    suite_id: suite_id.to_string(),
                }
            }
            crate::oracle_suite::IntegrityCondition::OracleEnvMismatch => {
                let violations = violation
                    .context
                    .get("violations")
                    .and_then(|v| v.as_array())
                    .cloned()
                    .unwrap_or_default();
                let first = violations
                    .get(0)
                    .and_then(|v| v.get("constraint").and_then(Value::as_str))
                    .unwrap_or("unknown");

                DomainIntegrityCondition::OracleEnvMismatch {
                    constraint: first.to_string(),
                    expected: violations
                        .get(0)
                        .and_then(|v| v.get("expected").and_then(Value::as_str))
                        .unwrap_or_default()
                        .to_string(),
                    actual: violations
                        .get(0)
                        .and_then(|v| v.get("actual").and_then(Value::as_str))
                        .unwrap_or_default()
                        .to_string(),
                }
            }
            crate::oracle_suite::IntegrityCondition::OracleFlake => {
                DomainIntegrityCondition::OracleFlake {
                    oracle_id: violation
                        .context
                        .get("flaky_oracles")
                        .and_then(|v| v.as_array())
                        .and_then(|arr| arr.first())
                        .and_then(Value::as_str)
                        .unwrap_or("unknown")
                        .to_string(),
                    run_1_hash: "".to_string(),
                    run_2_hash: "".to_string(),
                    description: violation.message.clone(),
                }
            }
            crate::oracle_suite::IntegrityCondition::EvidenceMissing => {
                DomainIntegrityCondition::EvidenceMissing {
                    reason: violation.message.clone(),
                }
            }
            crate::oracle_suite::IntegrityCondition::ManifestInvalid => {
                DomainIntegrityCondition::ManifestInvalid {
                    reason: violation.message.clone(),
                }
            }
        }
    }

    /// Run post-execution integrity checks using recorded evidence
    async fn run_integrity_checks(
        &self,
        run_id: &str,
        payload: &RunStartedPayload,
        result: &OracleRunResult,
        duration_ms: u64,
    ) -> Result<bool, OracleWorkerError> {
        let suite_def = self
            .oracle_runner
            .get_suite(&payload.oracle_suite_id)
            .await
            .ok_or_else(|| OracleWorkerError::SuiteNotFound {
                suite_id: payload.oracle_suite_id.clone(),
            })?;

        let manifest_bytes = match self
            .oracle_runner
            .evidence_store()
            .retrieve(&result.evidence_bundle_hash)
            .await
        {
            Ok(bytes) => bytes,
            Err(e) => {
                let condition = DomainIntegrityCondition::EvidenceMissing {
                    reason: e.to_string(),
                };
                self.handle_integrity_violation_condition(run_id, payload, &condition, duration_ms)
                    .await?;
                return Ok(false);
            }
        };

        let manifest: EvidenceManifest = match serde_json::from_slice(&manifest_bytes) {
            Ok(m) => m,
            Err(e) => {
                let condition = DomainIntegrityCondition::ManifestInvalid {
                    reason: e.to_string(),
                };
                self.handle_integrity_violation_condition(run_id, payload, &condition, duration_ms)
                    .await?;
                return Ok(false);
            }
        };

        let check = self
            .integrity_checker
            .check_integrity(
                &payload.candidate_id,
                run_id,
                &manifest,
                &suite_def,
                None,
                None,
                Some(&result.evidence_bundle_hash),
                Some(&payload.candidate_id),
            )
            .await;

        if check.passed {
            return Ok(true);
        }

        let loop_id = self.resolve_loop_id(run_id, payload).await;

        if let Some(violation) = check.violations.first() {
            let domain_condition =
                self.map_violation_to_domain_condition(violation, &payload.oracle_suite_id);

            self.emit_integrity_violation(
                run_id,
                &payload.candidate_id,
                &payload.oracle_suite_id,
                domain_condition,
            )
            .await?;

            if let Some(loop_id) = loop_id {
                let trigger = Self::violation_code(&violation.condition);
                self.emit_stop_triggered(&loop_id, trigger, "GovernanceChangePortal")
                    .await?;
            }
        }

        Ok(false)
    }

    async fn handle_integrity_violation_condition(
        &self,
        run_id: &str,
        payload: &RunStartedPayload,
        condition: &DomainIntegrityCondition,
        duration_ms: u64,
    ) -> Result<(), OracleWorkerError> {
        let loop_id = self.resolve_loop_id(run_id, payload).await;

        if !self.config.test_mode {
            self.emit_integrity_violation(
                run_id,
                &payload.candidate_id,
                &payload.oracle_suite_id,
                condition.clone(),
            )
            .await?;

            if let Some(loop_id) = loop_id {
                self.emit_stop_triggered(
                    &loop_id,
                    condition.condition_code(),
                    "GovernanceChangePortal",
                )
                .await?;
            }

            // Also record run completion as error for projections
            self.emit_execution_completed_error(
                run_id,
                &payload.candidate_id,
                &payload.oracle_suite_id,
                duration_ms,
                &condition.message(),
            )
            .await?;
        }

        Ok(())
    }

    /// Start the oracle execution worker
    ///
    /// This creates a consumer and begins processing RunStarted events.
    #[instrument(skip(self))]
    pub async fn start(&self) -> Result<(), OracleWorkerError> {
        info!(
            worker_id = %self.config.worker_id,
            "Starting oracle execution worker"
        );

        // Create consumer for run events
        let consumer = self
            .message_bus
            .create_consumer(
                streams::EVENTS,
                &self.config.consumer_name,
                Some(subjects::RUN_EVENTS),
            )
            .await?;

        info!("Oracle worker subscribed to run events");

        // Process messages in a loop
        loop {
            match self.process_batch(&consumer).await {
                Ok(count) => {
                    if count > 0 {
                        debug!(processed = count, "Processed run event batch");
                    }
                }
                Err(e) => {
                    error!(error = %e, "Error processing batch");
                    // Continue processing after errors
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }

            // Small delay between batches to avoid busy-waiting
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }

    /// Process a batch of messages
    async fn process_batch(&self, consumer: &NatsConsumer) -> Result<usize, OracleWorkerError> {
        let count = consumer
            .process(self.config.batch_size, |envelope| {
                let this = self;
                async move { this.handle_message(envelope).await }
            })
            .await?;

        Ok(count)
    }

    /// Handle a single message
    #[instrument(skip(self, envelope), fields(message_type = %envelope.message_type, message_id = %envelope.message_id))]
    async fn handle_message(&self, envelope: MessageEnvelope) -> Result<(), MessageBusError> {
        // Only process RunStarted events
        if envelope.message_type != "RunStarted" {
            debug!("Skipping non-RunStarted event");
            return Ok(());
        }

        // Extract run_id from the envelope
        // The stream_id in the outbox message contains the run_id
        let run_id = envelope
            .correlation_id
            .clone()
            .unwrap_or_else(|| envelope.message_id.replace("evt_", "run:"));

        // Check idempotency
        {
            let processed = self.processed_runs.read().await;
            if processed.contains_key(&run_id) {
                debug!(run_id = %run_id, "Run already processed");
                return Ok(());
            }
        }

        // Parse payload
        let payload: RunStartedPayload =
            serde_json::from_value(envelope.payload.clone()).map_err(|e| {
                MessageBusError::ConnectionError {
                    message: format!("Failed to parse RunStarted payload: {}", e),
                }
            })?;

        info!(
            run_id = %run_id,
            candidate_id = %payload.candidate_id,
            suite_id = %payload.oracle_suite_id,
            "Processing RunStarted event"
        );

        // Execute the oracle pipeline
        match self.execute_oracle_pipeline(&run_id, &payload).await {
            Ok(_) => {
                // Mark as processed
                let mut processed = self.processed_runs.write().await;
                processed.insert(run_id.clone(), Utc::now());

                info!(run_id = %run_id, "Oracle execution completed successfully");
                Ok(())
            }
            Err(e) => {
                error!(
                    run_id = %run_id,
                    error = %e,
                    "Oracle execution failed"
                );
                // Don't mark as processed - allow retry
                Err(MessageBusError::ConnectionError {
                    message: format!("Oracle execution failed: {}", e),
                })
            }
        }
    }

    /// Execute the oracle pipeline
    ///
    /// 1. Validate suite hash (TAMPER detection per C-OR-2)
    /// 2. Materialize candidate workspace
    /// 3. Emit OracleExecutionStarted event
    /// 4. Execute oracle suite
    /// 5. Emit OracleExecutionCompleted event
    #[instrument(skip(self, payload), fields(run_id = %run_id, suite_id = %payload.oracle_suite_id))]
    async fn execute_oracle_pipeline(
        &self,
        run_id: &str,
        payload: &RunStartedPayload,
    ) -> Result<(), OracleWorkerError> {
        let execution_start = Utc::now();

        // Step 1: Validate suite hash against registry (TAMPER detection)
        info!("Step 1: Validating suite hash");
        let suite_record = self
            .oracle_registry
            .get(&payload.oracle_suite_id)
            .await
            .map_err(|e| OracleWorkerError::RegistryError {
                message: e.to_string(),
            })?
            .ok_or_else(|| OracleWorkerError::SuiteNotFound {
                suite_id: payload.oracle_suite_id.clone(),
            })?;

        if suite_record.suite_hash != payload.oracle_suite_hash {
            warn!(
                expected = %payload.oracle_suite_hash,
                actual = %suite_record.suite_hash,
                "TAMPER DETECTED: Suite hash mismatch"
            );

            let condition = DomainIntegrityCondition::OracleTamper {
                expected_hash: payload.oracle_suite_hash.clone(),
                actual_hash: suite_record.suite_hash.clone(),
                suite_id: payload.oracle_suite_id.clone(),
            };

            self.handle_integrity_violation_condition(run_id, payload, &condition, 0)
                .await?;

            return Ok(());
        }
        debug!("Suite hash validated");

        // Step 2: Materialize candidate workspace
        info!("Step 2: Materializing candidate workspace");
        let workspace = self
            .candidate_workspace
            .materialize(&payload.candidate_id)
            .await?;
        debug!(workspace_path = %workspace.path.display(), "Workspace materialized");

        // Step 3: Emit OracleExecutionStarted event
        info!("Step 3: Emitting OracleExecutionStarted event");
        if !self.config.test_mode {
            self.emit_execution_started(
                run_id,
                &payload.candidate_id,
                &payload.oracle_suite_id,
                &payload.oracle_suite_hash,
                &workspace.path.to_string_lossy(),
            )
            .await?;
        }

        // Step 4: Execute oracle suite
        info!("Step 4: Executing oracle suite");
        let execution_result = self
            .oracle_runner
            .execute_suite(
                &payload.candidate_id,
                &payload.oracle_suite_id,
                &payload.oracle_suite_hash,
                &workspace.path,
            )
            .await;

        let execution_end = Utc::now();
        let duration_ms = (execution_end - execution_start).num_milliseconds() as u64;

        // Step 5: Emit OracleExecutionCompleted event
        info!("Step 5: Emitting OracleExecutionCompleted event");
        match execution_result {
            Ok(result) => {
                let mut emit_completion = true;
                if !self.config.test_mode {
                    emit_completion = self
                        .run_integrity_checks(run_id, payload, &result, duration_ms)
                        .await?;
                }

                if !self.config.test_mode && emit_completion {
                    self.emit_execution_completed(
                        run_id,
                        &payload.candidate_id,
                        &payload.oracle_suite_id,
                        &result,
                        duration_ms,
                        None,
                    )
                    .await?;
                }
                info!(
                    status = ?result.status,
                    evidence_hash = %result.evidence_bundle_hash,
                    "Oracle execution successful"
                );
                Ok(())
            }
            Err(OracleRunnerError::IntegrityViolation { condition }) => {
                self.handle_integrity_violation_condition(run_id, payload, &condition, duration_ms)
                    .await?;
                Ok(())
            }
            Err(e) => {
                let error_msg = e.to_string();
                if !self.config.test_mode {
                    self.emit_execution_completed_error(
                        run_id,
                        &payload.candidate_id,
                        &payload.oracle_suite_id,
                        duration_ms,
                        &error_msg,
                    )
                    .await?;
                }
                Err(OracleWorkerError::ExecutionFailed { reason: error_msg })
            }
        }
    }

    /// Emit OracleExecutionStarted event
    async fn emit_execution_started(
        &self,
        run_id: &str,
        candidate_id: &str,
        suite_id: &str,
        suite_hash: &str,
        workspace_path: &str,
    ) -> Result<(), OracleWorkerError> {
        let event_id = EventId::new();
        let now = Utc::now();

        let started_payload = OracleExecutionStarted {
            run_id: run_id.to_string(),
            candidate_id: candidate_id.to_string(),
            suite_id: suite_id.to_string(),
            suite_hash: suite_hash.to_string(),
            workspace_path: workspace_path.to_string(),
            started_at: now,
        };

        let payload = serde_json::to_value(&started_payload).map_err(|e| {
            OracleWorkerError::SerializationError {
                message: e.to_string(),
            }
        })?;

        // Get current stream version
        let events = self
            .event_store
            .read_stream(run_id, 0, 1000)
            .await
            .map_err(|e| OracleWorkerError::EventStoreError {
                message: e.to_string(),
            })?;
        let current_version = events.len() as u64;

        let event = EventEnvelope {
            event_id: event_id.clone(),
            stream_id: run_id.to_string(),
            stream_kind: StreamKind::Run,
            stream_seq: current_version + 1,
            global_seq: None,
            event_type: "OracleExecutionStarted".to_string(),
            occurred_at: now,
            actor_kind: ActorKind::System,
            actor_id: self.config.worker_id.clone(),
            correlation_id: Some(run_id.to_string()),
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload,
            envelope_hash: compute_envelope_hash(&event_id),
        };

        self.event_store
            .append(run_id, current_version, vec![event])
            .await
            .map_err(|e| OracleWorkerError::EventStoreError {
                message: e.to_string(),
            })?;

        debug!(event_id = %event_id.as_str(), "OracleExecutionStarted event emitted");
        Ok(())
    }

    /// Emit OracleExecutionCompleted event (success)
    async fn emit_execution_completed(
        &self,
        run_id: &str,
        candidate_id: &str,
        suite_id: &str,
        result: &OracleRunResult,
        duration_ms: u64,
        error: Option<&str>,
    ) -> Result<(), OracleWorkerError> {
        let event_id = EventId::new();
        let now = Utc::now();

        let status = match result.status {
            OracleStatus::Pass => OracleExecutionStatus::Pass,
            OracleStatus::Fail => OracleExecutionStatus::Fail,
            OracleStatus::Error => OracleExecutionStatus::Error,
        };

        let completed_payload = OracleExecutionCompleted {
            run_id: run_id.to_string(),
            candidate_id: candidate_id.to_string(),
            suite_id: suite_id.to_string(),
            status,
            evidence_bundle_hash: Some(result.evidence_bundle_hash.clone()),
            environment_fingerprint: result.environment_fingerprint.clone(),
            duration_ms,
            completed_at: now,
            error: error.map(|s| s.to_string()),
        };

        let payload = serde_json::to_value(&completed_payload).map_err(|e| {
            OracleWorkerError::SerializationError {
                message: e.to_string(),
            }
        })?;

        // Get current stream version
        let events = self
            .event_store
            .read_stream(run_id, 0, 1000)
            .await
            .map_err(|e| OracleWorkerError::EventStoreError {
                message: e.to_string(),
            })?;
        let current_version = events.len() as u64;

        let event = EventEnvelope {
            event_id: event_id.clone(),
            stream_id: run_id.to_string(),
            stream_kind: StreamKind::Run,
            stream_seq: current_version + 1,
            global_seq: None,
            event_type: "OracleExecutionCompleted".to_string(),
            occurred_at: now,
            actor_kind: ActorKind::System,
            actor_id: self.config.worker_id.clone(),
            correlation_id: Some(run_id.to_string()),
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload,
            envelope_hash: compute_envelope_hash(&event_id),
        };

        self.event_store
            .append(run_id, current_version, vec![event])
            .await
            .map_err(|e| OracleWorkerError::EventStoreError {
                message: e.to_string(),
            })?;

        debug!(event_id = %event_id.as_str(), "OracleExecutionCompleted event emitted");
        Ok(())
    }

    /// Emit OracleExecutionCompleted event (error case)
    async fn emit_execution_completed_error(
        &self,
        run_id: &str,
        candidate_id: &str,
        suite_id: &str,
        duration_ms: u64,
        error_message: &str,
    ) -> Result<(), OracleWorkerError> {
        let event_id = EventId::new();
        let now = Utc::now();

        let completed_payload = OracleExecutionCompleted {
            run_id: run_id.to_string(),
            candidate_id: candidate_id.to_string(),
            suite_id: suite_id.to_string(),
            status: OracleExecutionStatus::Error,
            evidence_bundle_hash: None,
            environment_fingerprint: serde_json::json!({}),
            duration_ms,
            completed_at: now,
            error: Some(error_message.to_string()),
        };

        let payload = serde_json::to_value(&completed_payload).map_err(|e| {
            OracleWorkerError::SerializationError {
                message: e.to_string(),
            }
        })?;

        // Get current stream version
        let events = self
            .event_store
            .read_stream(run_id, 0, 1000)
            .await
            .map_err(|e| OracleWorkerError::EventStoreError {
                message: e.to_string(),
            })?;
        let current_version = events.len() as u64;

        let event = EventEnvelope {
            event_id: event_id.clone(),
            stream_id: run_id.to_string(),
            stream_kind: StreamKind::Run,
            stream_seq: current_version + 1,
            global_seq: None,
            event_type: "OracleExecutionCompleted".to_string(),
            occurred_at: now,
            actor_kind: ActorKind::System,
            actor_id: self.config.worker_id.clone(),
            correlation_id: Some(run_id.to_string()),
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload,
            envelope_hash: compute_envelope_hash(&event_id),
        };

        self.event_store
            .append(run_id, current_version, vec![event])
            .await
            .map_err(|e| OracleWorkerError::EventStoreError {
                message: e.to_string(),
            })?;

        debug!(event_id = %event_id.as_str(), "OracleExecutionCompleted (error) event emitted");
        Ok(())
    }

    /// Emit IntegrityViolationDetected event (V8-3)
    ///
    /// Per C-OR-7: All integrity conditions MUST halt progression, record context,
    /// and route escalation.
    async fn emit_integrity_violation(
        &self,
        run_id: &str,
        candidate_id: &str,
        suite_id: &str,
        condition: DomainIntegrityCondition,
    ) -> Result<(), OracleWorkerError> {
        let event_id = EventId::new();
        let now = Utc::now();

        let violation_payload = IntegrityViolationDetected::new(
            run_id.to_string(),
            candidate_id.to_string(),
            suite_id.to_string(),
            condition,
        );

        let payload = serde_json::to_value(&violation_payload).map_err(|e| {
            OracleWorkerError::SerializationError {
                message: e.to_string(),
            }
        })?;

        // Get current stream version
        let events = self
            .event_store
            .read_stream(run_id, 0, 1000)
            .await
            .map_err(|e| OracleWorkerError::EventStoreError {
                message: e.to_string(),
            })?;
        let current_version = events.len() as u64;

        let event = EventEnvelope {
            event_id: event_id.clone(),
            stream_id: run_id.to_string(),
            stream_kind: StreamKind::Run,
            stream_seq: current_version + 1,
            global_seq: None,
            event_type: "IntegrityViolationDetected".to_string(),
            occurred_at: now,
            actor_kind: ActorKind::System,
            actor_id: self.config.worker_id.clone(),
            correlation_id: Some(run_id.to_string()),
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload,
            envelope_hash: compute_envelope_hash(&event_id),
        };

        self.event_store
            .append(run_id, current_version, vec![event])
            .await
            .map_err(|e| OracleWorkerError::EventStoreError {
                message: e.to_string(),
            })?;

        info!(
            event_id = %event_id.as_str(),
            condition = %violation_payload.condition.condition_code(),
            "IntegrityViolationDetected event emitted (C-OR-7)"
        );
        Ok(())
    }
}

/// Compute envelope hash from event ID
fn compute_envelope_hash(event_id: &EventId) -> String {
    format!("sha256:{}", event_id.as_str().replace("evt_", ""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = OracleWorkerConfig::default();
        assert_eq!(config.consumer_name, "oracle-execution-worker");
        assert_eq!(config.batch_size, 10);
        assert!(!config.test_mode);
        assert!(config.worker_id.starts_with("oracle_worker_"));
    }

    #[test]
    fn test_run_started_payload_parsing() {
        let json = serde_json::json!({
            "candidate_id": "cand:abc123",
            "oracle_suite_id": "suite:core-v1",
            "oracle_suite_hash": "sha256:deadbeef"
        });

        let payload: RunStartedPayload = serde_json::from_value(json).unwrap();
        assert_eq!(payload.candidate_id, "cand:abc123");
        assert_eq!(payload.oracle_suite_id, "suite:core-v1");
        assert_eq!(payload.oracle_suite_hash, "sha256:deadbeef");
    }

    #[test]
    fn test_error_types() {
        let err = OracleWorkerError::SuiteNotFound {
            suite_id: "suite:test".to_string(),
        };
        assert!(err.to_string().contains("Suite not found"));

        let err = OracleWorkerError::SuiteHashMismatch {
            expected: "sha256:aaa".to_string(),
            actual: "sha256:bbb".to_string(),
        };
        assert!(err.to_string().contains("TAMPER"));
    }
}

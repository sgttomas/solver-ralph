//! Reference Semantic Worker (D-41)
//!
//! Per SR-AGENT-WORKER-CONTRACT, implements the semantic worker that:
//! - Consumes IterationStarted events from NATS
//! - Obtains eligible set from Event Manager (D-40)
//! - Chooses one eligible work unit per iteration
//! - Executes the declared procedure stage
//! - Runs semantic oracle suites (D-39)
//! - Emits EvidenceBundleRecorded + iteration summaries
//!
//! Per SR-PLAN D-41:
//! - Worker can deterministically compile the context bundle from refs
//! - Worker executes procedure stages per Work Surface definition
//! - Worker invokes semantic oracle suites and captures results
//! - Worker emits stop triggers when appropriate

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sr_domain::{
    context::{CompilerConfig, ContextBundle, ContextCompiler},
    TypedRef,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, instrument, warn};

use crate::candidate_store::{CandidateWorkspace, TempWorkspace, WorkspaceError};
use crate::event_manager::{EligibleSet, EventManager};
use crate::nats::{streams, subjects, MessageEnvelope, NatsConsumer, NatsMessageBus};
use crate::oracle_runner::{OracleSuiteDefinition, PodmanOracleRunner};
use crate::oracle_suite::OracleSuiteRegistry;
use crate::worker::{ContentResolver, WorkerConfig, WorkerError};
use sr_ports::{EvidenceStore, MessageBusError, OracleRunResult, OracleRunnerError, OracleStatus};

// ============================================================================
// Semantic Worker Configuration
// ============================================================================

/// Semantic worker configuration
#[derive(Debug, Clone)]
pub struct SemanticWorkerConfig {
    /// Base worker configuration
    pub base: WorkerConfig,
    /// Maximum iterations per work unit before stopping
    pub max_iterations_per_work_unit: u32,
    /// Maximum oracle runs per iteration
    pub max_oracle_runs_per_iteration: u32,
    /// Whether to enable dry-run mode (no actual execution)
    pub dry_run: bool,
}

impl Default for SemanticWorkerConfig {
    fn default() -> Self {
        Self {
            base: WorkerConfig::default(),
            max_iterations_per_work_unit: 5,
            max_oracle_runs_per_iteration: 25,
            dry_run: false,
        }
    }
}

impl SemanticWorkerConfig {
    /// Create configuration from environment
    pub fn from_env() -> Self {
        Self {
            base: WorkerConfig::from_env(),
            max_iterations_per_work_unit: std::env::var("MAX_ITERATIONS_PER_WORK_UNIT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
            max_oracle_runs_per_iteration: std::env::var("MAX_ORACLE_RUNS_PER_ITERATION")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(25),
            dry_run: std::env::var("SEMANTIC_WORKER_DRY_RUN")
                .map(|s| s == "true" || s == "1")
                .unwrap_or(false),
        }
    }
}

// ============================================================================
// Selection Rationale per SR-AGENT-WORKER-CONTRACT §2.1
// ============================================================================

/// Work unit selection rationale per SR-AGENT-WORKER-CONTRACT §2.1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionRationale {
    /// ID of the selected work unit
    pub selected_work_unit_id: String,
    /// Stage being executed
    pub target_stage_id: String,
    /// Why this work unit was chosen
    pub reason: String,
    /// Eligible set snapshot hash for traceability
    pub eligible_set_snapshot_hash: String,
    /// Timestamp of selection
    pub selected_at: DateTime<Utc>,
    /// Number of eligible candidates considered
    pub candidates_considered: usize,
}

// ============================================================================
// Stage Execution Result
// ============================================================================

/// Result of executing a procedure stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageExecutionResult {
    /// Work unit being processed
    pub work_unit_id: String,
    /// Stage executed
    pub stage_id: String,
    /// Whether the stage completed successfully
    pub success: bool,
    /// Whether this was the terminal stage
    pub is_terminal: bool,
    /// Artifacts produced during stage execution
    pub artifacts: Vec<StageArtifact>,
    /// Oracle results from semantic suite
    pub oracle_results: Vec<SemanticOracleResult>,
    /// Stop trigger if one was raised
    pub stop_trigger: Option<StopTriggerInfo>,
    /// Summary of work done
    pub summary: String,
}

/// An artifact produced during stage execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageArtifact {
    pub artifact_id: String,
    pub content_hash: String,
    pub artifact_type: String,
    pub size_bytes: u64,
}

/// Result of a semantic oracle evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticOracleResult {
    pub oracle_id: String,
    pub passed: bool,
    pub score: Option<f64>,
    pub details: serde_json::Value,
}

/// Stop trigger information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopTriggerInfo {
    pub reason: StopTriggerReason,
    pub description: String,
    pub requires_portal: bool,
    pub portal_id: Option<String>,
}

/// Stop trigger reasons per SR-AGENT-WORKER-CONTRACT §3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopTriggerReason {
    OracleSuiteUnavailable,
    MissingStageInputs,
    ThrashingDetected,
    BudgetExhausted,
    NoEligibleWorkUnits,
    IntegrityViolation,
}

// ============================================================================
// Evidence Bundle per SR-AGENT-WORKER-CONTRACT §2.4
// ============================================================================

/// Evidence bundle payload for EvidenceBundleRecorded event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceBundlePayload {
    /// Bundle identifier
    pub bundle_id: String,
    /// Work unit this evidence is for
    pub work_unit_id: String,
    /// Candidate being evaluated
    pub candidate_id: String,
    /// Procedure template used
    pub procedure_template_id: String,
    /// Stage this evidence is for
    pub stage_id: String,
    /// Oracle suite results
    pub oracle_results: Vec<SemanticOracleResult>,
    /// Overall gate verdict
    pub gate_verdict: GateVerdict,
    /// Content hash of the evidence package
    pub content_hash: String,
    /// Timestamp
    pub recorded_at: DateTime<Utc>,
}

/// Gate verdict
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GateVerdict {
    Pass,
    Fail,
    Inconclusive,
    Waived,
}

// ============================================================================
// Iteration Summary per SR-AGENT-WORKER-CONTRACT §4
// ============================================================================

/// Iteration summary per SR-AGENT-WORKER-CONTRACT §4
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationSummary {
    /// Iteration identifier
    pub iteration_id: String,
    /// Work unit processed
    pub work_unit_id: String,
    /// Stage executed
    pub stage_id: String,
    /// Selection rationale
    pub selection_rationale: SelectionRationale,
    /// Stage execution result
    pub execution_result: StageExecutionResult,
    /// Evidence bundle reference
    pub evidence_bundle_ref: Option<String>,
    /// Recommended next step
    pub next_step_recommendation: NextStepRecommendation,
    /// Context artifact hash
    pub context_hash: String,
    /// Timestamp
    pub completed_at: DateTime<Utc>,
}

/// Next step recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NextStepRecommendation {
    /// Continue to next stage
    ContinueToStage { stage_id: String },
    /// Retry current stage
    RetryStage { reason: String },
    /// Work unit complete
    WorkUnitComplete,
    /// Requires portal action
    RequiresPortal { portal_id: String, reason: String },
    /// Stop due to failure
    StopFailed { reason: String },
}

// ============================================================================
// Reference Semantic Worker Bridge
// ============================================================================

/// Reference semantic worker bridge per SR-AGENT-WORKER-CONTRACT
///
/// This worker implements the semantic loop behavior:
/// 1. Subscribe to iteration events
/// 2. Obtain eligible set from Event Manager
/// 3. Choose one eligible work unit (with documented rationale)
/// 4. Execute the procedure stage
/// 5. Run semantic oracle suites
/// 6. Emit evidence bundle and iteration summary
pub struct SemanticWorkerBridge<E: EvidenceStore, W: CandidateWorkspace> {
    /// Worker configuration
    config: SemanticWorkerConfig,
    /// NATS message bus
    message_bus: Arc<NatsMessageBus>,
    /// HTTP client
    http_client: reqwest::Client,
    /// Context compiler
    context_compiler: ContextCompiler,
    /// Content resolver
    content_resolver: ContentResolver,
    /// Event manager for eligible set
    event_manager: Arc<RwLock<EventManager>>,
    /// Processed iterations (idempotency)
    processed_iterations: Arc<RwLock<HashMap<String, DateTime<Utc>>>>,
    /// Iteration count per work unit (thrashing detection)
    iteration_counts: Arc<RwLock<HashMap<String, u32>>>,
    /// Oracle runner for executing semantic oracle suites (V9-1)
    oracle_runner: Arc<PodmanOracleRunner<E>>,
    /// Evidence store for persisting evidence bundles (V9-1)
    evidence_store: Arc<E>,
    /// Oracle suite registry for suite definitions (V9-1)
    oracle_registry: Arc<OracleSuiteRegistry>,
    /// Candidate workspace materializer (V9-1)
    candidate_workspace: Arc<W>,
}

impl<E: EvidenceStore + 'static, W: CandidateWorkspace + 'static> SemanticWorkerBridge<E, W> {
    /// Create a new semantic worker bridge
    pub fn new(
        config: SemanticWorkerConfig,
        message_bus: Arc<NatsMessageBus>,
        event_manager: Arc<RwLock<EventManager>>,
        oracle_runner: Arc<PodmanOracleRunner<E>>,
        evidence_store: Arc<E>,
        oracle_registry: Arc<OracleSuiteRegistry>,
        candidate_workspace: Arc<W>,
    ) -> Self {
        Self {
            config,
            message_bus,
            http_client: reqwest::Client::new(),
            context_compiler: ContextCompiler::with_config(CompilerConfig::default()),
            content_resolver: ContentResolver::new(),
            event_manager,
            processed_iterations: Arc::new(RwLock::new(HashMap::new())),
            iteration_counts: Arc::new(RwLock::new(HashMap::new())),
            oracle_runner,
            evidence_store,
            oracle_registry,
            candidate_workspace,
        }
    }

    /// Start the semantic worker
    #[instrument(skip(self))]
    pub async fn start(&self) -> Result<(), WorkerError> {
        info!(
            worker_id = %self.config.base.worker_id,
            "Starting semantic worker bridge"
        );

        // Create consumer for iteration events
        let consumer = self
            .message_bus
            .create_consumer(
                streams::EVENTS,
                &format!("{}-semantic", self.config.base.consumer_name),
                Some(subjects::ITERATION_EVENTS),
            )
            .await?;

        info!("Semantic worker subscribed to iteration events");

        // Process messages
        loop {
            match self.process_batch(&consumer).await {
                Ok(count) => {
                    if count > 0 {
                        debug!(processed = count, "Processed iteration batch");
                    }
                }
                Err(e) => {
                    error!(error = %e, "Error processing batch");
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }

    async fn process_batch(&self, consumer: &NatsConsumer) -> Result<usize, WorkerError> {
        let count = consumer
            .process(self.config.base.batch_size, |envelope| {
                let this = self;
                async move { this.handle_iteration_event(envelope).await }
            })
            .await?;
        Ok(count)
    }

    /// Handle an iteration event per SR-AGENT-WORKER-CONTRACT
    #[instrument(skip(self, envelope), fields(event_type = %envelope.message_type))]
    async fn handle_iteration_event(
        &self,
        envelope: MessageEnvelope,
    ) -> Result<(), MessageBusError> {
        if envelope.message_type != "IterationStarted" {
            debug!("Skipping non-IterationStarted event");
            return Ok(());
        }

        let iteration_id = envelope
            .payload
            .get("iteration_id")
            .and_then(|v| v.as_str())
            .map(String::from)
            .unwrap_or_else(|| format!("iter_{}", envelope.message_id.replace("evt_", "")));

        let loop_id = envelope
            .payload
            .get("loop_id")
            .and_then(|v| v.as_str())
            .map(String::from)
            .ok_or_else(|| MessageBusError::ConnectionError {
                message: "Missing loop_id".to_string(),
            })?;

        // Idempotency check
        {
            let processed = self.processed_iterations.read().await;
            if processed.contains_key(&iteration_id) {
                debug!(iteration_id = %iteration_id, "Already processed");
                return Ok(());
            }
        }

        info!(
            iteration_id = %iteration_id,
            loop_id = %loop_id,
            "Processing semantic iteration"
        );

        // Execute semantic work pipeline
        match self
            .execute_semantic_pipeline(&iteration_id, &loop_id)
            .await
        {
            Ok(summary) => {
                let mut processed = self.processed_iterations.write().await;
                processed.insert(iteration_id.clone(), Utc::now());
                info!(
                    iteration_id = %iteration_id,
                    work_unit_id = %summary.work_unit_id,
                    "Semantic iteration completed"
                );
                Ok(())
            }
            Err(e) => {
                error!(iteration_id = %iteration_id, error = %e, "Semantic pipeline failed");
                Err(MessageBusError::ConnectionError {
                    message: format!("Semantic pipeline failed: {}", e),
                })
            }
        }
    }

    /// Execute the semantic work pipeline per SR-AGENT-WORKER-CONTRACT §2
    #[instrument(skip(self))]
    async fn execute_semantic_pipeline(
        &self,
        iteration_id: &str,
        loop_id: &str,
    ) -> Result<IterationSummary, WorkerError> {
        // Step 1: Get eligible set from Event Manager
        let eligible_set = {
            let em = self.event_manager.read().await;
            em.compute_eligible_set()
        };

        // Step 2: Check for no eligible work units (stop condition)
        if eligible_set.is_empty() {
            return Err(WorkerError::ContextCompilationError {
                reason: "No eligible work units - emitting stop trigger".to_string(),
            });
        }

        // Step 3: Choose one eligible work unit per §2.1
        let selection = self.select_work_unit(&eligible_set).await?;
        info!(
            work_unit_id = %selection.selected_work_unit_id,
            stage_id = %selection.target_stage_id,
            reason = %selection.reason,
            "Selected work unit"
        );

        // Step 4: Check for thrashing per §3
        let iteration_count = {
            let mut counts = self.iteration_counts.write().await;
            let count = counts
                .entry(selection.selected_work_unit_id.clone())
                .or_insert(0);
            *count += 1;
            *count
        };

        if iteration_count > self.config.max_iterations_per_work_unit {
            return Err(WorkerError::ContextCompilationError {
                reason: format!(
                    "Thrashing detected: {} iterations for work unit {}",
                    iteration_count, selection.selected_work_unit_id
                ),
            });
        }

        // Step 5: Compile context
        let context_bundle = self.compile_context_for_work_unit(&selection)?;

        // Step 6: Execute procedure stage per §2.2
        let execution_result = self
            .execute_stage(&selection, &context_bundle, iteration_id)
            .await?;

        // Step 7: Emit evidence bundle per §2.4 (if not dry run)
        let evidence_bundle_ref = if !self.config.dry_run && !self.config.base.test_mode {
            Some(
                self.emit_evidence_bundle(&selection, &execution_result, iteration_id)
                    .await?,
            )
        } else {
            None
        };

        // Step 8: Determine next step recommendation
        let next_step = self.determine_next_step(&execution_result);

        // Step 9: Build iteration summary per §4
        let summary = IterationSummary {
            iteration_id: iteration_id.to_string(),
            work_unit_id: selection.selected_work_unit_id.clone(),
            stage_id: selection.target_stage_id.clone(),
            selection_rationale: selection,
            execution_result,
            evidence_bundle_ref,
            next_step_recommendation: next_step,
            context_hash: context_bundle.content_hash.as_str().to_string(),
            completed_at: Utc::now(),
        };

        // Step 10: Emit iteration summary (if not dry run)
        if !self.config.dry_run && !self.config.base.test_mode {
            self.emit_iteration_summary(&summary).await?;
        }

        Ok(summary)
    }

    /// Select one eligible work unit per SR-AGENT-WORKER-CONTRACT §2.1
    async fn select_work_unit(
        &self,
        eligible_set: &EligibleSet,
    ) -> Result<SelectionRationale, WorkerError> {
        // Compute eligible set hash for traceability
        let mut hasher = Sha256::new();
        for entry in &eligible_set.entries {
            hasher.update(entry.work_unit_id.as_str().as_bytes());
            hasher.update(b":");
        }
        let snapshot_hash = hex::encode(hasher.finalize());

        // Per contract: choose first eligible (or apply priority)
        // In a real implementation, this might use more sophisticated selection
        let selected =
            eligible_set
                .entries
                .first()
                .ok_or_else(|| WorkerError::ContextCompilationError {
                    reason: "No eligible work units".to_string(),
                })?;

        let stage_id = selected
            .current_stage_id
            .as_ref()
            .map(|s| s.as_str().to_string())
            .unwrap_or_else(|| "stage:FRAME".to_string());

        Ok(SelectionRationale {
            selected_work_unit_id: selected.work_unit_id.as_str().to_string(),
            target_stage_id: stage_id,
            reason: format!(
                "Selected first eligible from {} candidates (priority-ordered)",
                eligible_set.len()
            ),
            eligible_set_snapshot_hash: snapshot_hash,
            selected_at: Utc::now(),
            candidates_considered: eligible_set.len(),
        })
    }

    /// Compile context for the selected work unit
    fn compile_context_for_work_unit(
        &self,
        selection: &SelectionRationale,
    ) -> Result<ContextBundle, WorkerError> {
        // Build refs for the work unit context
        let refs = vec![
            TypedRef {
                kind: "WorkUnit".to_string(),
                id: selection.selected_work_unit_id.clone(),
                rel: "target".to_string(),
                meta: serde_json::Value::Null,
            },
            TypedRef {
                kind: "Stage".to_string(),
                id: selection.target_stage_id.clone(),
                rel: "executing".to_string(),
                meta: serde_json::Value::Null,
            },
            TypedRef {
                kind: "EligibleSetSnapshot".to_string(),
                id: selection.eligible_set_snapshot_hash.clone(),
                rel: "supported_by".to_string(),
                meta: serde_json::Value::Null,
            },
        ];

        let timestamp = Utc::now();
        let resolver = &self.content_resolver;

        self.context_compiler
            .compile(&refs, timestamp, |r| {
                resolver.resolve(r).map_err(|e| {
                    sr_domain::errors::DomainError::InvariantViolation {
                        invariant: format!("Failed to resolve ref: {}", e),
                    }
                })
            })
            .map_err(|e| WorkerError::ContextCompilationError {
                reason: e.to_string(),
            })
    }

    /// Execute the procedure stage per SR-AGENT-WORKER-CONTRACT §2.2
    async fn execute_stage(
        &self,
        selection: &SelectionRationale,
        context: &ContextBundle,
        iteration_id: &str,
    ) -> Result<StageExecutionResult, WorkerError> {
        info!(
            work_unit_id = %selection.selected_work_unit_id,
            stage_id = %selection.target_stage_id,
            "Executing procedure stage"
        );

        // In a real implementation, this would:
        // 1. Load the ProcedureTemplate for the work unit
        // 2. Execute the stage steps
        // 3. Run semantic oracle suites
        // 4. Collect artifacts

        // For the reference implementation, we simulate execution
        let oracle_results = self.run_semantic_oracles(selection, context).await?;

        // Check if all oracles passed
        let all_passed = oracle_results.iter().all(|r| r.passed);

        // Compute artifact hash
        let mut hasher = Sha256::new();
        hasher.update(b"stage_output:");
        hasher.update(context.content_hash.as_str().as_bytes());
        hasher.update(b":");
        hasher.update(selection.selected_work_unit_id.as_bytes());
        hasher.update(b":");
        hasher.update(selection.target_stage_id.as_bytes());
        hasher.update(b":");
        hasher.update(iteration_id.as_bytes());
        let artifact_hash = hex::encode(hasher.finalize());

        let artifact = StageArtifact {
            artifact_id: format!("artifact_{}", ulid::Ulid::new()),
            content_hash: artifact_hash,
            artifact_type: "stage_output".to_string(),
            size_bytes: 0,
        };

        // Check for stop conditions
        let stop_trigger = if !all_passed {
            Some(StopTriggerInfo {
                reason: StopTriggerReason::IntegrityViolation,
                description: "Semantic oracle suite did not pass".to_string(),
                requires_portal: true,
                portal_id: Some("portal_exception".to_string()),
            })
        } else {
            None
        };

        // For reference implementation, assume non-terminal unless specified
        let is_terminal = selection.target_stage_id.contains("FINAL")
            || selection.target_stage_id.contains("COMPLETE");

        let oracle_count = oracle_results.len();

        Ok(StageExecutionResult {
            work_unit_id: selection.selected_work_unit_id.clone(),
            stage_id: selection.target_stage_id.clone(),
            success: all_passed && stop_trigger.is_none(),
            is_terminal: is_terminal && all_passed,
            artifacts: vec![artifact],
            oracle_results,
            stop_trigger,
            summary: format!(
                "Executed stage {} for work unit {} with {} oracle checks",
                selection.target_stage_id, selection.selected_work_unit_id, oracle_count
            ),
        })
    }

    /// Run semantic oracle suites per SR-AGENT-WORKER-CONTRACT §2.3
    ///
    /// V9-1: Now invokes real oracle runner instead of returning simulated data.
    async fn run_semantic_oracles(
        &self,
        selection: &SelectionRationale,
        _context: &ContextBundle,
    ) -> Result<Vec<SemanticOracleResult>, WorkerError> {
        use crate::semantic_suite::SUITE_INTAKE_ADMISSIBILITY_ID;

        info!(
            work_unit_id = %selection.selected_work_unit_id,
            stage_id = %selection.target_stage_id,
            "Running semantic oracle suites"
        );

        // 1. Get semantic oracle suite from registry
        let suite = self
            .oracle_registry
            .get_suite(SUITE_INTAKE_ADMISSIBILITY_ID)
            .await
            .ok_or_else(|| WorkerError::OracleError {
                message: format!(
                    "Semantic oracle suite not found: {}",
                    SUITE_INTAKE_ADMISSIBILITY_ID
                ),
            })?;

        info!(
            suite_id = %suite.suite_id,
            suite_hash = %suite.suite_hash,
            "Retrieved semantic oracle suite"
        );

        // 2. Materialize candidate workspace
        let workspace = self
            .candidate_workspace
            .materialize(&selection.selected_work_unit_id)
            .await
            .map_err(|e| WorkerError::WorkspaceError {
                message: format!("Failed to materialize workspace: {}", e),
            })?;

        info!(
            workspace_path = %workspace.path.display(),
            candidate_id = %workspace.candidate_id,
            "Materialized candidate workspace"
        );

        // 3. Execute oracle suite
        let result = self
            .oracle_runner
            .execute_suite(
                &selection.selected_work_unit_id,
                &suite.suite_id,
                &suite.suite_hash,
                &workspace.path,
            )
            .await
            .map_err(|e| WorkerError::OracleError {
                message: format!("Oracle execution failed: {}", e),
            })?;

        info!(
            run_id = %result.run_id,
            status = ?result.status,
            evidence_hash = %result.evidence_bundle_hash,
            "Oracle suite execution complete"
        );

        // 4. Map OracleRunResult to Vec<SemanticOracleResult>
        let oracle_results = self.map_oracle_results(&result, &suite);

        debug!(
            oracle_count = oracle_results.len(),
            "Semantic oracle evaluation complete"
        );

        Ok(oracle_results)
    }

    /// Map OracleRunResult to Vec<SemanticOracleResult>
    fn map_oracle_results(
        &self,
        result: &OracleRunResult,
        suite: &OracleSuiteDefinition,
    ) -> Vec<SemanticOracleResult> {
        // Map the overall suite result to SemanticOracleResult format
        vec![SemanticOracleResult {
            oracle_id: format!("SEMANTIC:{}", suite.suite_id),
            passed: result.status == OracleStatus::Pass,
            score: None, // OracleRunResult doesn't provide a score
            details: serde_json::json!({
                "run_id": result.run_id,
                "evidence_hash": result.evidence_bundle_hash,
                "status": format!("{:?}", result.status),
                "environment": result.environment_fingerprint,
            }),
        }]
    }

    /// Emit evidence bundle per SR-AGENT-WORKER-CONTRACT §2.4
    ///
    /// V9-1: Now persists to MinIO and emits EvidenceBundleRecorded event.
    async fn emit_evidence_bundle(
        &self,
        selection: &SelectionRationale,
        result: &StageExecutionResult,
        iteration_id: &str,
    ) -> Result<String, WorkerError> {
        let bundle_id = format!("bundle:{}", ulid::Ulid::new());

        // Compute gate verdict
        let gate_verdict = if result.success {
            GateVerdict::Pass
        } else if result.stop_trigger.is_some() {
            GateVerdict::Fail
        } else {
            GateVerdict::Inconclusive
        };

        // Build evidence bundle payload
        let payload = EvidenceBundlePayload {
            bundle_id: bundle_id.clone(),
            work_unit_id: selection.selected_work_unit_id.clone(),
            candidate_id: result
                .artifacts
                .first()
                .map(|a| a.artifact_id.clone())
                .unwrap_or_default(),
            procedure_template_id: "proc:GENERIC-KNOWLEDGE-WORK".to_string(),
            stage_id: selection.target_stage_id.clone(),
            oracle_results: result.oracle_results.clone(),
            gate_verdict: gate_verdict.clone(),
            content_hash: String::new(), // Will be set from store result
            recorded_at: Utc::now(),
        };

        // 1. Serialize and store in MinIO
        let manifest_bytes = serde_json::to_vec(&payload).map_err(|e| {
            WorkerError::SerializationError {
                message: format!("Failed to serialize evidence bundle: {}", e),
            }
        })?;

        let content_hash = self
            .evidence_store
            .store(&manifest_bytes, vec![])
            .await
            .map_err(|e| WorkerError::StorageError {
                message: format!("Failed to store evidence bundle: {}", e),
            })?;

        info!(
            bundle_id = %bundle_id,
            content_hash = %content_hash,
            "Evidence bundle stored in MinIO"
        );

        // 2. Emit EvidenceBundleRecorded event via NATS
        let event_payload = serde_json::json!({
            "event_type": "EvidenceBundleRecorded",
            "bundle_id": bundle_id,
            "content_hash": content_hash,
            "work_unit_id": selection.selected_work_unit_id,
            "stage_id": selection.target_stage_id,
            "iteration_id": iteration_id,
            "gate_verdict": format!("{:?}", gate_verdict),
            "recorded_at": Utc::now(),
        });

        let event_bytes = serde_json::to_vec(&event_payload).map_err(|e| {
            WorkerError::SerializationError {
                message: format!("Failed to serialize event: {}", e),
            }
        })?;

        self.message_bus
            .publish_with_id(subjects::ORACLE_EVENTS, &event_bytes, &bundle_id)
            .await
            .map_err(|e| WorkerError::MessageBusError {
                message: format!("Failed to emit EvidenceBundleRecorded event: {}", e),
            })?;

        info!(
            bundle_id = %bundle_id,
            content_hash = %content_hash,
            verdict = ?gate_verdict,
            "Evidence bundle recorded and event emitted"
        );

        Ok(bundle_id)
    }

    /// Determine next step recommendation
    fn determine_next_step(&self, result: &StageExecutionResult) -> NextStepRecommendation {
        if let Some(stop_trigger) = &result.stop_trigger {
            if stop_trigger.requires_portal {
                return NextStepRecommendation::RequiresPortal {
                    portal_id: stop_trigger.portal_id.clone().unwrap_or_default(),
                    reason: stop_trigger.description.clone(),
                };
            }
            return NextStepRecommendation::StopFailed {
                reason: stop_trigger.description.clone(),
            };
        }

        if result.is_terminal && result.success {
            return NextStepRecommendation::WorkUnitComplete;
        }

        if result.success {
            // In a real implementation, determine next stage from procedure template
            NextStepRecommendation::ContinueToStage {
                stage_id: "stage:NEXT".to_string(),
            }
        } else {
            NextStepRecommendation::RetryStage {
                reason: "Stage did not complete successfully".to_string(),
            }
        }
    }

    /// Emit iteration summary per SR-AGENT-WORKER-CONTRACT §4
    async fn emit_iteration_summary(&self, summary: &IterationSummary) -> Result<(), WorkerError> {
        info!(
            iteration_id = %summary.iteration_id,
            work_unit_id = %summary.work_unit_id,
            next_step = ?summary.next_step_recommendation,
            "Emitting iteration summary"
        );

        // In a real implementation, this would:
        // 1. POST to /api/v1/iterations/{id}/complete
        // 2. Include the full summary as structured payload

        Ok(())
    }

    /// Get count of processed iterations
    pub async fn processed_count(&self) -> usize {
        self.processed_iterations.read().await.len()
    }

    /// Get iteration count for a work unit
    pub async fn work_unit_iteration_count(&self, work_unit_id: &str) -> u32 {
        self.iteration_counts
            .read()
            .await
            .get(work_unit_id)
            .copied()
            .unwrap_or(0)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_worker_config_default() {
        let config = SemanticWorkerConfig::default();
        assert_eq!(config.max_iterations_per_work_unit, 5);
        assert_eq!(config.max_oracle_runs_per_iteration, 25);
        assert!(!config.dry_run);
    }

    #[test]
    fn test_selection_rationale_serialization() {
        let rationale = SelectionRationale {
            selected_work_unit_id: "WU-001".to_string(),
            target_stage_id: "stage:FRAME".to_string(),
            reason: "First eligible".to_string(),
            eligible_set_snapshot_hash: "abc123".repeat(11)[..64].to_string(),
            selected_at: Utc::now(),
            candidates_considered: 3,
        };

        let json = serde_json::to_string(&rationale).unwrap();
        let parsed: SelectionRationale = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.selected_work_unit_id, "WU-001");
    }

    #[test]
    fn test_stage_execution_result_success() {
        let result = StageExecutionResult {
            work_unit_id: "WU-001".to_string(),
            stage_id: "stage:FRAME".to_string(),
            success: true,
            is_terminal: false,
            artifacts: vec![],
            oracle_results: vec![SemanticOracleResult {
                oracle_id: "test_oracle".to_string(),
                passed: true,
                score: Some(1.0),
                details: serde_json::json!({}),
            }],
            stop_trigger: None,
            summary: "Test execution".to_string(),
        };

        assert!(result.success);
        assert!(result.stop_trigger.is_none());
    }

    #[test]
    fn test_evidence_bundle_payload_serialization() {
        let payload = EvidenceBundlePayload {
            bundle_id: "bundle_123".to_string(),
            work_unit_id: "WU-001".to_string(),
            candidate_id: "cand_123".to_string(),
            procedure_template_id: "GENERIC".to_string(),
            stage_id: "stage:FRAME".to_string(),
            oracle_results: vec![],
            gate_verdict: GateVerdict::Pass,
            content_hash: "hash1234".repeat(8).to_string(),
            recorded_at: Utc::now(),
        };

        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("gate_verdict"));
        assert!(json.contains("PASS"));
    }

    #[test]
    fn test_gate_verdict_variants() {
        assert_eq!(
            serde_json::to_string(&GateVerdict::Pass).unwrap(),
            "\"PASS\""
        );
        assert_eq!(
            serde_json::to_string(&GateVerdict::Fail).unwrap(),
            "\"FAIL\""
        );
        assert_eq!(
            serde_json::to_string(&GateVerdict::Inconclusive).unwrap(),
            "\"INCONCLUSIVE\""
        );
        assert_eq!(
            serde_json::to_string(&GateVerdict::Waived).unwrap(),
            "\"WAIVED\""
        );
    }

    #[test]
    fn test_stop_trigger_reasons() {
        let trigger = StopTriggerInfo {
            reason: StopTriggerReason::ThrashingDetected,
            description: "Too many iterations".to_string(),
            requires_portal: true,
            portal_id: Some("portal_exception".to_string()),
        };

        let json = serde_json::to_string(&trigger).unwrap();
        assert!(json.contains("thrashing_detected"));
    }

    #[test]
    fn test_iteration_summary_serialization() {
        let summary = IterationSummary {
            iteration_id: "iter_123".to_string(),
            work_unit_id: "WU-001".to_string(),
            stage_id: "stage:FRAME".to_string(),
            selection_rationale: SelectionRationale {
                selected_work_unit_id: "WU-001".to_string(),
                target_stage_id: "stage:FRAME".to_string(),
                reason: "Test".to_string(),
                eligible_set_snapshot_hash: "hash".to_string(),
                selected_at: Utc::now(),
                candidates_considered: 1,
            },
            execution_result: StageExecutionResult {
                work_unit_id: "WU-001".to_string(),
                stage_id: "stage:FRAME".to_string(),
                success: true,
                is_terminal: false,
                artifacts: vec![],
                oracle_results: vec![],
                stop_trigger: None,
                summary: "Done".to_string(),
            },
            evidence_bundle_ref: Some("bundle_123".to_string()),
            next_step_recommendation: NextStepRecommendation::WorkUnitComplete,
            context_hash: "ctx_hash".to_string(),
            completed_at: Utc::now(),
        };

        let json = serde_json::to_string(&summary).unwrap();
        assert!(json.contains("iteration_id"));
        assert!(json.contains("selection_rationale"));
    }

    #[test]
    fn test_next_step_recommendation_variants() {
        let continue_stage = NextStepRecommendation::ContinueToStage {
            stage_id: "stage:VERIFY".to_string(),
        };
        let json = serde_json::to_string(&continue_stage).unwrap();
        assert!(json.contains("continue_to_stage"));

        let retry = NextStepRecommendation::RetryStage {
            reason: "Failed".to_string(),
        };
        let json = serde_json::to_string(&retry).unwrap();
        assert!(json.contains("retry_stage"));

        let complete = NextStepRecommendation::WorkUnitComplete;
        let json = serde_json::to_string(&complete).unwrap();
        assert!(json.contains("work_unit_complete"));
    }
}

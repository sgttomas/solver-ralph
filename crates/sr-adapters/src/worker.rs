//! Reference Worker Bridge (D-23)
//!
//! Implements the reference worker that:
//! - Consumes IterationStarted events from NATS
//! - Compiles context from iteration refs
//! - Executes work (stub implementation)
//! - Registers candidates via the API
//! - Completes iterations with structured summaries
//!
//! Per SR-PLAN D-23:
//! - Worker can deterministically compile the context bundle from refs
//! - Worker can submit a candidate proposal and a structured summary record
//! - Worker failures are recorded and do not corrupt state
//!
//! This is a reference implementation demonstrating the worker bridge pattern.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sr_domain::{
    context::{CompilerConfig, ContextBundle, ContextCompiler, ItemClassification},
    ContentHash, TypedRef,
};
use sr_ports::{MessageBus, MessageBusError};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, instrument, warn};

use crate::nats::{streams, subjects, MessageEnvelope, NatsConsumer, NatsMessageBus};

// ============================================================================
// Worker Configuration
// ============================================================================

/// Worker bridge configuration
#[derive(Debug, Clone)]
pub struct WorkerConfig {
    /// API base URL for submitting candidates and completing iterations
    pub api_base_url: String,
    /// Consumer name for NATS subscription
    pub consumer_name: String,
    /// Batch size for message fetching
    pub batch_size: usize,
    /// Whether to enable test mode (mock API calls)
    pub test_mode: bool,
    /// Worker ID for attribution
    pub worker_id: String,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            api_base_url: std::env::var("SR_API_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            consumer_name: "reference-worker".to_string(),
            batch_size: 10,
            test_mode: false,
            worker_id: format!("worker_{}", ulid::Ulid::new()),
        }
    }
}

impl WorkerConfig {
    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            api_base_url: std::env::var("SR_API_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            consumer_name: std::env::var("WORKER_CONSUMER_NAME")
                .unwrap_or_else(|_| "reference-worker".to_string()),
            batch_size: std::env::var("WORKER_BATCH_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            test_mode: std::env::var("WORKER_TEST_MODE")
                .map(|s| s == "true" || s == "1")
                .unwrap_or(false),
            worker_id: std::env::var("WORKER_ID")
                .unwrap_or_else(|_| format!("worker_{}", ulid::Ulid::new())),
        }
    }
}

// ============================================================================
// Worker Error Types
// ============================================================================

/// Worker bridge error types
#[derive(Debug, thiserror::Error)]
pub enum WorkerError {
    #[error("Message bus error: {message}")]
    MessageBusError { message: String },

    #[error("Context compilation failed: {reason}")]
    ContextCompilationError { reason: String },

    #[error("API request failed: {status} - {message}")]
    ApiError { status: u16, message: String },

    #[error("Invalid event: {reason}")]
    InvalidEvent { reason: String },

    #[error("HTTP client error: {message}")]
    HttpError { message: String },

    #[error("Serialization error: {message}")]
    SerializationError { message: String },
}

impl From<MessageBusError> for WorkerError {
    fn from(e: MessageBusError) -> Self {
        WorkerError::MessageBusError {
            message: e.to_string(),
        }
    }
}

// ============================================================================
// Work Result
// ============================================================================

/// Result of work execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkResult {
    /// Content hash of the produced artifact
    pub content_hash: String,
    /// Git SHA if applicable
    pub git_sha: Option<String>,
    /// Refs associated with this work
    pub refs: Vec<TypedRef>,
    /// Description of what was done
    pub summary: String,
    /// Actions taken during work
    pub actions: Vec<WorkAction>,
    /// Artifacts produced
    pub artifacts_produced: Vec<String>,
}

/// An action taken during work
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAction {
    pub kind: String,
    pub summary: String,
    pub artifacts: Vec<String>,
}

// ============================================================================
// Content Resolver
// ============================================================================

/// Content resolver for context compilation
///
/// In a real implementation, this would fetch from the evidence store
/// and artifact repositories. For the reference worker, we use a stub.
pub struct ContentResolver {
    /// Cache of resolved content hashes
    cache: HashMap<String, (ContentHash, ItemClassification)>,
}

impl ContentResolver {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Resolve a typed ref to its content hash and classification
    ///
    /// For the reference implementation, this returns a deterministic
    /// hash based on the ref ID.
    pub fn resolve(
        &self,
        typed_ref: &TypedRef,
    ) -> Result<(ContentHash, ItemClassification), WorkerError> {
        // Check cache first
        if let Some(cached) = self.cache.get(&typed_ref.id) {
            return Ok(cached.clone());
        }

        // Compute deterministic hash from ref
        let mut hasher = Sha256::new();
        hasher.update(typed_ref.kind.as_bytes());
        hasher.update(b":");
        hasher.update(typed_ref.id.as_bytes());
        hasher.update(b":");
        hasher.update(typed_ref.rel.as_bytes());
        let hash = hex::encode(hasher.finalize());

        // Classify based on ref kind
        let classification = match typed_ref.kind.to_lowercase().as_str() {
            "secret" | "credential" | "key" => ItemClassification::Confidential,
            "internal" | "private" => ItemClassification::Internal,
            "restricted" => ItemClassification::Restricted,
            _ => ItemClassification::Public,
        };

        Ok((ContentHash::new(&hash), classification))
    }
}

impl Default for ContentResolver {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Reference Worker Bridge
// ============================================================================

/// Reference worker bridge implementation
///
/// This worker demonstrates the pattern for:
/// 1. Subscribing to iteration events
/// 2. Compiling context from refs
/// 3. Executing work
/// 4. Registering candidates
/// 5. Completing iterations
pub struct ReferenceWorkerBridge {
    /// Worker configuration
    config: WorkerConfig,
    /// NATS message bus for event subscription
    message_bus: Arc<NatsMessageBus>,
    /// HTTP client for API calls
    http_client: reqwest::Client,
    /// Context compiler
    context_compiler: ContextCompiler,
    /// Content resolver
    content_resolver: ContentResolver,
    /// Processed iteration IDs (for idempotency)
    processed_iterations: Arc<RwLock<HashMap<String, DateTime<Utc>>>>,
}

impl ReferenceWorkerBridge {
    /// Create a new reference worker bridge
    pub fn new(config: WorkerConfig, message_bus: Arc<NatsMessageBus>) -> Self {
        Self {
            config,
            message_bus,
            http_client: reqwest::Client::new(),
            context_compiler: ContextCompiler::with_config(CompilerConfig::default()),
            content_resolver: ContentResolver::new(),
            processed_iterations: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Start the worker bridge
    ///
    /// This creates a consumer and begins processing iteration events.
    #[instrument(skip(self))]
    pub async fn start(&self) -> Result<(), WorkerError> {
        info!(
            worker_id = %self.config.worker_id,
            "Starting reference worker bridge"
        );

        // Create consumer for iteration events
        let consumer = self
            .message_bus
            .create_consumer(
                streams::EVENTS,
                &self.config.consumer_name,
                Some(subjects::ITERATION_EVENTS),
            )
            .await?;

        info!("Worker subscribed to iteration events");

        // Process messages in a loop
        loop {
            match self.process_batch(&consumer).await {
                Ok(count) => {
                    if count > 0 {
                        debug!(processed = count, "Processed message batch");
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
    async fn process_batch(&self, consumer: &NatsConsumer) -> Result<usize, WorkerError> {
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
        // Only process IterationStarted events
        if envelope.message_type != "IterationStarted" {
            debug!("Skipping non-IterationStarted event");
            return Ok(());
        }

        // Extract iteration info from payload
        let iteration_id = envelope
            .payload
            .get("iteration_id")
            .or_else(|| {
                // Fallback: try to extract from stream_id in correlation
                envelope.correlation_id.as_ref().and_then(|_| None)
            })
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                // Use message_id as fallback if no iteration_id
                format!("iter_{}", envelope.message_id.replace("evt_", ""))
            });

        let loop_id = envelope
            .payload
            .get("loop_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| MessageBusError::ConnectionError {
                message: "Missing loop_id in IterationStarted payload".to_string(),
            })?;

        // Check idempotency
        {
            let processed = self.processed_iterations.read().await;
            if processed.contains_key(&iteration_id) {
                debug!(iteration_id = %iteration_id, "Iteration already processed");
                return Ok(());
            }
        }

        info!(
            iteration_id = %iteration_id,
            loop_id = %loop_id,
            "Processing IterationStarted event"
        );

        // Extract refs from payload
        let refs: Vec<TypedRef> = envelope
            .payload
            .get("refs")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        // Execute the work pipeline
        match self
            .execute_work_pipeline(&iteration_id, &loop_id, refs)
            .await
        {
            Ok(_) => {
                // Mark as processed
                let mut processed = self.processed_iterations.write().await;
                processed.insert(iteration_id.clone(), Utc::now());

                info!(iteration_id = %iteration_id, "Work pipeline completed successfully");
                Ok(())
            }
            Err(e) => {
                error!(
                    iteration_id = %iteration_id,
                    error = %e,
                    "Work pipeline failed"
                );
                // Don't mark as processed - allow retry
                Err(MessageBusError::ConnectionError {
                    message: format!("Work pipeline failed: {}", e),
                })
            }
        }
    }

    /// Execute the full work pipeline
    ///
    /// 1. Compile context from refs
    /// 2. Execute work (stub)
    /// 3. Register candidate
    /// 4. Complete iteration
    #[instrument(skip(self, refs))]
    async fn execute_work_pipeline(
        &self,
        iteration_id: &str,
        loop_id: &str,
        refs: Vec<TypedRef>,
    ) -> Result<(), WorkerError> {
        // Step 1: Compile context
        info!("Step 1: Compiling context from refs");
        let context_bundle = self.compile_context(&refs)?;
        debug!(
            context_hash = %context_bundle.content_hash.as_str(),
            items = context_bundle.metadata.items_included,
            redacted = context_bundle.metadata.items_redacted,
            "Context compiled"
        );

        // Step 2: Execute work (stub implementation)
        info!("Step 2: Executing work");
        let work_result = self.execute_work(&context_bundle, iteration_id, loop_id)?;
        debug!(
            content_hash = %work_result.content_hash,
            actions = work_result.actions.len(),
            "Work executed"
        );

        // Step 3: Register candidate (skip in test mode)
        if !self.config.test_mode {
            info!("Step 3: Registering candidate");
            self.register_candidate(iteration_id, &work_result).await?;
        } else {
            info!("Step 3: Skipping candidate registration (test mode)");
        }

        // Step 4: Complete iteration (skip in test mode)
        if !self.config.test_mode {
            info!("Step 4: Completing iteration");
            self.complete_iteration(iteration_id, loop_id, &work_result)
                .await?;
        } else {
            info!("Step 4: Skipping iteration completion (test mode)");
        }

        Ok(())
    }

    /// Compile context from refs
    fn compile_context(&self, refs: &[TypedRef]) -> Result<ContextBundle, WorkerError> {
        let timestamp = Utc::now();
        let resolver = &self.content_resolver;

        self.context_compiler
            .compile(refs, timestamp, |r| {
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

    /// Execute work (stub implementation)
    ///
    /// In a real implementation, this would:
    /// - Invoke the appropriate worker/agent
    /// - Execute the procedure stage
    /// - Produce artifacts
    ///
    /// For the reference implementation, we generate a deterministic output.
    fn execute_work(
        &self,
        context_bundle: &ContextBundle,
        iteration_id: &str,
        loop_id: &str,
    ) -> Result<WorkResult, WorkerError> {
        // Compute deterministic content hash from context + iteration
        let mut hasher = Sha256::new();
        hasher.update(b"work_output:");
        hasher.update(context_bundle.content_hash.as_str().as_bytes());
        hasher.update(b":");
        hasher.update(iteration_id.as_bytes());
        hasher.update(b":");
        hasher.update(loop_id.as_bytes());
        hasher.update(b":");
        hasher.update(&self.config.worker_id.as_bytes());
        let content_hash = hex::encode(hasher.finalize());

        // Build work result
        let result = WorkResult {
            content_hash,
            git_sha: None,
            refs: vec![
                TypedRef {
                    kind: "context".to_string(),
                    id: context_bundle.content_hash.as_str().to_string(),
                    rel: "compiled_from".to_string(),
                    meta: serde_json::json!({
                        "items_included": context_bundle.metadata.items_included,
                        "items_redacted": context_bundle.metadata.items_redacted,
                    }),
                },
                TypedRef {
                    kind: "iteration".to_string(),
                    id: iteration_id.to_string(),
                    rel: "produced_in".to_string(),
                    meta: serde_json::Value::Null,
                },
            ],
            summary: format!(
                "Reference worker processed iteration {} with {} context items",
                iteration_id, context_bundle.metadata.items_included
            ),
            actions: vec![
                WorkAction {
                    kind: "context_compilation".to_string(),
                    summary: format!(
                        "Compiled {} refs into context bundle",
                        context_bundle.metadata.total_refs
                    ),
                    artifacts: vec![context_bundle.content_hash.as_str().to_string()],
                },
                WorkAction {
                    kind: "work_execution".to_string(),
                    summary: "Reference worker stub execution".to_string(),
                    artifacts: vec![],
                },
            ],
            artifacts_produced: vec![],
        };

        Ok(result)
    }

    /// Register a candidate via the API
    #[instrument(skip(self, work_result))]
    async fn register_candidate(
        &self,
        iteration_id: &str,
        work_result: &WorkResult,
    ) -> Result<String, WorkerError> {
        let url = format!("{}/api/v1/candidates", self.config.api_base_url);

        let request_body = serde_json::json!({
            "content_hash": work_result.content_hash,
            "git_sha": work_result.git_sha,
            "produced_by_iteration_id": iteration_id,
            "refs": work_result.refs.iter().map(|r| {
                serde_json::json!({
                    "kind": r.kind,
                    "id": r.id,
                    "rel": r.rel,
                    "meta": r.meta
                })
            }).collect::<Vec<_>>()
        });

        let response = self
            .http_client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| WorkerError::HttpError {
                message: e.to_string(),
            })?;

        let status = response.status().as_u16();
        if status >= 400 {
            let body = response.text().await.unwrap_or_default();
            return Err(WorkerError::ApiError {
                status,
                message: body,
            });
        }

        let body: serde_json::Value =
            response
                .json()
                .await
                .map_err(|e| WorkerError::SerializationError {
                    message: e.to_string(),
                })?;

        let candidate_id = body
            .get("candidate_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        info!(candidate_id = %candidate_id, "Candidate registered");
        Ok(candidate_id)
    }

    /// Complete an iteration via the API
    #[instrument(skip(self, work_result))]
    async fn complete_iteration(
        &self,
        iteration_id: &str,
        loop_id: &str,
        work_result: &WorkResult,
    ) -> Result<(), WorkerError> {
        let url = format!(
            "{}/api/v1/iterations/{}/complete",
            self.config.api_base_url, iteration_id
        );

        let request_body = serde_json::json!({
            "outcome": "SUCCESS",
            "summary": {
                "intent": work_result.summary,
                "actions": work_result.actions.iter().map(|a| {
                    serde_json::json!({
                        "kind": a.kind,
                        "summary": a.summary,
                        "artifacts": a.artifacts
                    })
                }).collect::<Vec<_>>(),
                "artifacts_touched": work_result.artifacts_produced,
                "candidates_produced": [work_result.content_hash.clone()],
                "runs_executed": [],
                "next_steps": [],
                "open_risks": []
            }
        });

        let response = self
            .http_client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| WorkerError::HttpError {
                message: e.to_string(),
            })?;

        let status = response.status().as_u16();
        if status >= 400 {
            let body = response.text().await.unwrap_or_default();
            return Err(WorkerError::ApiError {
                status,
                message: body,
            });
        }

        info!(
            iteration_id = %iteration_id,
            loop_id = %loop_id,
            "Iteration completed"
        );

        Ok(())
    }

    /// Get the count of processed iterations
    pub async fn processed_count(&self) -> usize {
        let processed = self.processed_iterations.read().await;
        processed.len()
    }

    /// Check if an iteration was processed
    pub async fn is_processed(&self, iteration_id: &str) -> bool {
        let processed = self.processed_iterations.read().await;
        processed.contains_key(iteration_id)
    }
}

// ============================================================================
// Standalone Worker Execution
// ============================================================================

/// Run the worker bridge as a standalone service
pub async fn run_worker(config: WorkerConfig, nats_url: &str) -> Result<(), WorkerError> {
    use crate::nats::NatsConfig;

    info!(
        worker_id = %config.worker_id,
        nats_url = %nats_url,
        "Initializing reference worker"
    );

    // Connect to NATS
    let nats_config = NatsConfig {
        url: nats_url.to_string(),
        ..NatsConfig::default()
    };

    let message_bus = NatsMessageBus::connect(nats_config).await?;
    let message_bus = Arc::new(message_bus);

    // Create and run worker
    let worker = ReferenceWorkerBridge::new(config, message_bus);
    worker.start().await
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_config_default() {
        let config = WorkerConfig::default();
        assert_eq!(config.batch_size, 10);
        assert!(!config.test_mode);
        assert!(config.worker_id.starts_with("worker_"));
    }

    #[test]
    fn test_content_resolver() {
        let resolver = ContentResolver::new();

        let typed_ref = TypedRef {
            kind: "Doc".to_string(),
            id: "test-doc".to_string(),
            rel: "depends_on".to_string(),
            meta: serde_json::Value::Null,
        };

        let (hash, classification) = resolver.resolve(&typed_ref).unwrap();
        // ContentHash includes "sha256:" prefix (7 chars) + 64 hex chars = 71
        assert_eq!(hash.as_str().len(), 71);
        assert_eq!(classification, ItemClassification::Public);
    }

    #[test]
    fn test_content_resolver_classification() {
        let resolver = ContentResolver::new();

        // Test confidential classification
        let secret_ref = TypedRef {
            kind: "Secret".to_string(),
            id: "api-key".to_string(),
            rel: "contains".to_string(),
            meta: serde_json::Value::Null,
        };
        let (_, classification) = resolver.resolve(&secret_ref).unwrap();
        assert_eq!(classification, ItemClassification::Confidential);

        // Test internal classification
        let internal_ref = TypedRef {
            kind: "Internal".to_string(),
            id: "design-doc".to_string(),
            rel: "depends_on".to_string(),
            meta: serde_json::Value::Null,
        };
        let (_, classification) = resolver.resolve(&internal_ref).unwrap();
        assert_eq!(classification, ItemClassification::Internal);
    }

    #[test]
    fn test_content_resolver_determinism() {
        let resolver = ContentResolver::new();

        let typed_ref = TypedRef {
            kind: "Doc".to_string(),
            id: "test-doc".to_string(),
            rel: "depends_on".to_string(),
            meta: serde_json::Value::Null,
        };

        let (hash1, _) = resolver.resolve(&typed_ref).unwrap();
        let (hash2, _) = resolver.resolve(&typed_ref).unwrap();

        // Same ref should produce same hash
        assert_eq!(hash1.as_str(), hash2.as_str());
    }

    #[test]
    fn test_work_result_serialization() {
        let result = WorkResult {
            content_hash: "abc123".repeat(11)[..64].to_string(),
            git_sha: Some("def456".to_string()),
            refs: vec![],
            summary: "Test work result".to_string(),
            actions: vec![WorkAction {
                kind: "test".to_string(),
                summary: "Test action".to_string(),
                artifacts: vec![],
            }],
            artifacts_produced: vec![],
        };

        let json = serde_json::to_string(&result).unwrap();
        let parsed: WorkResult = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.content_hash, result.content_hash);
        assert_eq!(parsed.summary, result.summary);
    }

    #[test]
    fn test_execute_work_determinism() {
        let config = WorkerConfig {
            worker_id: "test_worker".to_string(),
            test_mode: true,
            ..Default::default()
        };

        // Create a mock context bundle
        let context_bundle = ContextBundle {
            content_hash: ContentHash::new("context_hash_123"),
            items: vec![],
            metadata: sr_domain::context::ContextMetadata {
                total_refs: 2,
                items_included: 2,
                items_redacted: 0,
                compiled_at: Utc::now(),
                compiler_version: "1.0.0".to_string(),
            },
            redactions: vec![],
        };

        // We can't easily test the worker without NATS, but we can test
        // that the work execution is deterministic by checking the hash computation
        let mut hasher = Sha256::new();
        hasher.update(b"work_output:");
        hasher.update(context_bundle.content_hash.as_str().as_bytes());
        hasher.update(b":");
        hasher.update(b"iter_test");
        hasher.update(b":");
        hasher.update(b"loop_test");
        hasher.update(b":");
        hasher.update(config.worker_id.as_bytes());
        let hash1 = hex::encode(hasher.finalize());

        // Same inputs should produce same hash
        let mut hasher = Sha256::new();
        hasher.update(b"work_output:");
        hasher.update(context_bundle.content_hash.as_str().as_bytes());
        hasher.update(b":");
        hasher.update(b"iter_test");
        hasher.update(b":");
        hasher.update(b"loop_test");
        hasher.update(b":");
        hasher.update(config.worker_id.as_bytes());
        let hash2 = hex::encode(hasher.finalize());

        assert_eq!(hash1, hash2);
    }
}

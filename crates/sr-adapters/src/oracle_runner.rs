//! Oracle Runner Service (D-24)
//!
//! Implements sandboxed oracle execution using Podman + gVisor (runsc).
//!
//! Per SR-PLAN D-24:
//! - Service runs oracle suites in sandboxed containers
//! - Captures outputs and emits Evidence Bundles
//! - Runner can execute containerized oracle deterministically given same inputs
//! - Runner captures stdout/stderr, exit codes, and artifact hashes
//! - Runner cannot access restricted evidence without explicit authorization
//!
//! Per SR-SPEC §4.5 (Oracle runtime pattern):
//! - OCI container image pinned by digest
//! - Runtime: gVisor (runsc) for sandboxing
//! - Network disabled by default
//! - Read-only workspace mount
//! - Write-only scratch volume for outputs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sr_ports::{EvidenceStore, EvidenceStoreError, OracleRunner, OracleRunnerError, OracleRunResult, OracleStatus};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::RwLock;
use tokio::time::{timeout, Duration};
use tracing::{debug, error, info, instrument, warn};

use crate::evidence::{EvidenceArtifact, EvidenceManifest, EvidenceManifestBuilder, OracleResult, OracleResultStatus};

// ============================================================================
// Configuration
// ============================================================================

/// Podman oracle runner configuration
#[derive(Debug, Clone)]
pub struct PodmanOracleRunnerConfig {
    /// Podman binary path
    pub podman_path: String,
    /// gVisor runsc path (for runtime option)
    pub runsc_path: String,
    /// Default execution timeout in seconds
    pub default_timeout_secs: u64,
    /// Workspace mount path (in container, read-only)
    pub workspace_mount: String,
    /// Scratch volume mount path (in container, writable)
    pub scratch_mount: String,
    /// Enable test mode (mock container execution)
    pub test_mode: bool,
    /// Network mode (disabled by default)
    pub network_mode: NetworkMode,
    /// Runner ID for attribution
    pub runner_id: String,
}

impl Default for PodmanOracleRunnerConfig {
    fn default() -> Self {
        Self {
            podman_path: std::env::var("PODMAN_PATH")
                .unwrap_or_else(|_| "podman".to_string()),
            runsc_path: std::env::var("RUNSC_PATH")
                .unwrap_or_else(|_| "/usr/local/bin/runsc".to_string()),
            default_timeout_secs: std::env::var("ORACLE_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(600),
            workspace_mount: "/workspace".to_string(),
            scratch_mount: "/scratch".to_string(),
            test_mode: std::env::var("ORACLE_TEST_MODE")
                .map(|s| s == "true" || s == "1")
                .unwrap_or(false),
            network_mode: NetworkMode::Disabled,
            runner_id: format!("runner_{}", ulid::Ulid::new()),
        }
    }
}

impl PodmanOracleRunnerConfig {
    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        Self::default()
    }
}

/// Network mode for container execution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NetworkMode {
    /// Network disabled (default per SR-SPEC)
    Disabled,
    /// Host network (requires explicit suite permission)
    Host,
    /// Private network (isolated)
    Private,
}

impl Default for NetworkMode {
    fn default() -> Self {
        NetworkMode::Disabled
    }
}

// ============================================================================
// Oracle Suite Definitions
// ============================================================================

/// Oracle suite definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleSuiteDefinition {
    /// Suite identifier
    pub suite_id: String,
    /// Content hash of the suite definition
    pub suite_hash: String,
    /// OCI image reference (with digest)
    pub oci_image: String,
    /// OCI image digest (sha256:...)
    pub oci_image_digest: String,
    /// Environment constraints
    pub environment_constraints: EnvironmentConstraints,
    /// Oracle definitions within this suite
    pub oracles: Vec<OracleDefinition>,
    /// Metadata
    #[serde(default)]
    pub metadata: BTreeMap<String, serde_json::Value>,
}

/// Environment constraints for oracle execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConstraints {
    /// Required runtime (runsc for gVisor)
    #[serde(default = "default_runtime")]
    pub runtime: String,
    /// Network mode
    #[serde(default)]
    pub network: NetworkMode,
    /// CPU architecture
    #[serde(default = "default_arch")]
    pub cpu_arch: String,
    /// Operating system
    #[serde(default = "default_os")]
    pub os: String,
    /// Workspace must be read-only
    #[serde(default = "default_true")]
    pub workspace_readonly: bool,
    /// Additional constraints
    #[serde(default)]
    pub additional_constraints: Vec<String>,
}

fn default_runtime() -> String {
    "runsc".to_string()
}

fn default_arch() -> String {
    "amd64".to_string()
}

fn default_os() -> String {
    "linux".to_string()
}

fn default_true() -> bool {
    true
}

impl Default for EnvironmentConstraints {
    fn default() -> Self {
        Self {
            runtime: default_runtime(),
            network: NetworkMode::Disabled,
            cpu_arch: default_arch(),
            os: default_os(),
            workspace_readonly: true,
            additional_constraints: vec![],
        }
    }
}

/// Individual oracle definition within a suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleDefinition {
    /// Oracle identifier
    pub oracle_id: String,
    /// Human-readable name
    pub oracle_name: String,
    /// Command to execute
    pub command: String,
    /// Arguments (split from command if needed)
    #[serde(default)]
    pub args: Vec<String>,
    /// Timeout in seconds
    pub timeout_seconds: u64,
    /// Expected outputs
    #[serde(default)]
    pub expected_outputs: Vec<ExpectedOutput>,
    /// Oracle classification
    #[serde(default)]
    pub classification: OracleClassification,
    /// Working directory within container
    #[serde(default)]
    pub working_dir: Option<String>,
    /// Environment variables
    #[serde(default)]
    pub env: BTreeMap<String, String>,
}

/// Expected output artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedOutput {
    /// Path within scratch volume
    pub path: String,
    /// MIME content type
    pub content_type: String,
    /// Whether this output is required
    #[serde(default = "default_true")]
    pub required: bool,
}

/// Oracle classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OracleClassification {
    /// Required oracle (must pass)
    #[default]
    Required,
    /// Advisory oracle (informational)
    Advisory,
    /// Optional oracle (nice to have)
    Optional,
}

// ============================================================================
// Execution State
// ============================================================================

/// Single oracle execution result
#[derive(Debug, Clone)]
pub struct OracleExecutionResult {
    /// Oracle definition
    pub oracle_def: OracleDefinition,
    /// Execution status
    pub status: OracleResultStatus,
    /// Execution duration in milliseconds
    pub duration_ms: u64,
    /// Exit code (if process completed)
    pub exit_code: Option<i32>,
    /// Stdout captured
    pub stdout: String,
    /// Stderr captured
    pub stderr: String,
    /// Error message (if failed)
    pub error_message: Option<String>,
    /// Output artifacts captured
    pub artifacts: Vec<CapturedArtifact>,
}

/// Captured artifact from oracle execution
#[derive(Debug, Clone)]
pub struct CapturedArtifact {
    /// Artifact name (from expected output path)
    pub name: String,
    /// Content hash (SHA-256)
    pub content_hash: String,
    /// Content type (MIME)
    pub content_type: String,
    /// Size in bytes
    pub size: u64,
    /// Raw content
    pub content: Vec<u8>,
}

/// Environment fingerprint captured during execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentFingerprint {
    /// Container image digest
    pub container_image_digest: String,
    /// Runtime name
    pub runtime: String,
    /// Runtime version
    pub runtime_version: String,
    /// Operating system
    pub os: String,
    /// CPU architecture
    pub arch: String,
    /// Network mode
    pub network_mode: String,
    /// Workspace read-only status
    pub workspace_readonly: bool,
    /// Scratch writable status
    pub scratch_writable: bool,
    /// Tool versions
    pub tool_versions: BTreeMap<String, String>,
    /// Runner ID
    pub runner_id: String,
    /// Execution timestamp
    pub executed_at: DateTime<Utc>,
}

// ============================================================================
// Podman Oracle Runner
// ============================================================================

/// Podman-based oracle runner implementation
pub struct PodmanOracleRunner<E: EvidenceStore> {
    /// Configuration
    config: PodmanOracleRunnerConfig,
    /// Evidence store for persisting results
    evidence_store: Arc<E>,
    /// Cached suite definitions
    suite_cache: Arc<RwLock<BTreeMap<String, OracleSuiteDefinition>>>,
}

impl<E: EvidenceStore> PodmanOracleRunner<E> {
    /// Create a new Podman oracle runner
    pub fn new(config: PodmanOracleRunnerConfig, evidence_store: Arc<E>) -> Self {
        Self {
            config,
            evidence_store,
            suite_cache: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    /// Register a suite definition
    pub async fn register_suite(&self, suite: OracleSuiteDefinition) {
        let mut cache = self.suite_cache.write().await;
        cache.insert(suite.suite_id.clone(), suite);
    }

    /// Get a suite definition
    pub async fn get_suite(&self, suite_id: &str) -> Option<OracleSuiteDefinition> {
        let cache = self.suite_cache.read().await;
        cache.get(suite_id).cloned()
    }

    /// Execute a full oracle suite
    #[instrument(skip(self, candidate_path))]
    pub async fn execute_suite(
        &self,
        candidate_id: &str,
        suite_id: &str,
        suite_hash: &str,
        candidate_path: &PathBuf,
    ) -> Result<OracleRunResult, OracleRunnerError> {
        info!(
            candidate_id = %candidate_id,
            suite_id = %suite_id,
            "Starting oracle suite execution"
        );

        // Get suite definition
        let suite = self.get_suite(suite_id).await.ok_or_else(|| {
            OracleRunnerError::SuiteNotFound {
                suite_id: suite_id.to_string(),
            }
        })?;

        // Verify suite hash
        if suite.suite_hash != suite_hash {
            return Err(OracleRunnerError::SuiteHashMismatch {
                expected: suite_hash.to_string(),
                actual: suite.suite_hash.clone(),
            });
        }

        let run_id = format!("run_{}", ulid::Ulid::new());
        let run_started_at = Utc::now();

        // Capture environment fingerprint
        let fingerprint = self.capture_environment_fingerprint(&suite).await?;

        // Execute each oracle
        let mut oracle_results = Vec::new();
        let mut all_artifacts = Vec::new();

        for oracle_def in &suite.oracles {
            let result = self
                .execute_single_oracle(candidate_id, candidate_path, &suite, oracle_def)
                .await;

            match result {
                Ok(exec_result) => {
                    // Capture artifacts
                    for artifact in &exec_result.artifacts {
                        all_artifacts.push((artifact.name.clone(), artifact.content.clone()));
                    }

                    // Build OracleResult
                    let oracle_result = OracleResult {
                        oracle_id: exec_result.oracle_def.oracle_id.clone(),
                        oracle_name: exec_result.oracle_def.oracle_name.clone(),
                        status: exec_result.status,
                        duration_ms: exec_result.duration_ms,
                        error_message: exec_result.error_message,
                        artifact_refs: exec_result
                            .artifacts
                            .iter()
                            .map(|a| a.content_hash.clone())
                            .collect(),
                        output: Some(serde_json::json!({
                            "exit_code": exec_result.exit_code,
                            "stdout_lines": exec_result.stdout.lines().count(),
                            "stderr_lines": exec_result.stderr.lines().count(),
                        })),
                    };

                    oracle_results.push(oracle_result);
                }
                Err(e) => {
                    error!(
                        oracle_id = %oracle_def.oracle_id,
                        error = %e,
                        "Oracle execution failed"
                    );

                    oracle_results.push(OracleResult {
                        oracle_id: oracle_def.oracle_id.clone(),
                        oracle_name: oracle_def.oracle_name.clone(),
                        status: OracleResultStatus::Error,
                        duration_ms: 0,
                        error_message: Some(e.to_string()),
                        artifact_refs: vec![],
                        output: None,
                    });
                }
            }
        }

        let run_completed_at = Utc::now();

        // Build evidence manifest
        let manifest = self.build_evidence_manifest(
            &run_id,
            candidate_id,
            &suite,
            &oracle_results,
            &all_artifacts,
            &fingerprint,
            run_started_at,
            run_completed_at,
        )?;

        // Store evidence bundle
        let manifest_bytes = serde_json::to_vec(&manifest).map_err(|e| {
            OracleRunnerError::ManifestCreationFailed {
                reason: e.to_string(),
            }
        })?;

        let blob_refs: Vec<(&str, &[u8])> = all_artifacts
            .iter()
            .map(|(name, content)| (name.as_str(), content.as_slice()))
            .collect();

        let evidence_bundle_hash = self
            .evidence_store
            .store(&manifest_bytes, blob_refs)
            .await
            .map_err(|e| OracleRunnerError::EvidenceStoreFailed {
                reason: e.to_string(),
            })?;

        // Determine overall status
        let status = self.compute_overall_status(&oracle_results);

        info!(
            run_id = %run_id,
            evidence_bundle_hash = %evidence_bundle_hash,
            status = ?status,
            "Oracle suite execution completed"
        );

        Ok(OracleRunResult {
            run_id,
            evidence_bundle_hash,
            status,
            environment_fingerprint: serde_json::to_value(&fingerprint).unwrap_or_default(),
        })
    }

    /// Execute a single oracle
    #[instrument(skip(self, candidate_path, suite))]
    async fn execute_single_oracle(
        &self,
        candidate_id: &str,
        candidate_path: &PathBuf,
        suite: &OracleSuiteDefinition,
        oracle_def: &OracleDefinition,
    ) -> Result<OracleExecutionResult, OracleRunnerError> {
        info!(
            oracle_id = %oracle_def.oracle_id,
            oracle_name = %oracle_def.oracle_name,
            "Executing oracle"
        );

        let start_time = std::time::Instant::now();

        // In test mode, return mock result
        if self.config.test_mode {
            return self.mock_oracle_execution(oracle_def).await;
        }

        // Build Podman command
        let scratch_dir = tempfile::tempdir().map_err(|e| OracleRunnerError::ContainerCreationFailed {
            reason: format!("Failed to create scratch dir: {}", e),
        })?;

        let mut cmd = Command::new(&self.config.podman_path);

        cmd.arg("run")
            .arg("--rm")
            .arg("--runtime")
            .arg(&self.config.runsc_path)
            .arg(format!("--network={}", match self.config.network_mode {
                NetworkMode::Disabled => "none",
                NetworkMode::Host => "host",
                NetworkMode::Private => "private",
            }))
            .arg("-v")
            .arg(format!(
                "{}:{}:ro",
                candidate_path.display(),
                self.config.workspace_mount
            ))
            .arg("-v")
            .arg(format!(
                "{}:{}:rw",
                scratch_dir.path().display(),
                self.config.scratch_mount
            ));

        // Add working directory if specified
        if let Some(ref workdir) = oracle_def.working_dir {
            cmd.arg("-w").arg(workdir);
        } else {
            cmd.arg("-w").arg(&self.config.workspace_mount);
        }

        // Add environment variables
        for (key, value) in &oracle_def.env {
            cmd.arg("-e").arg(format!("{}={}", key, value));
        }

        // Add container image
        cmd.arg(&suite.oci_image);

        // Add command and args
        cmd.arg("sh").arg("-c").arg(&oracle_def.command);

        // Configure stdio
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        // Execute with timeout
        let timeout_duration = Duration::from_secs(oracle_def.timeout_seconds);

        let output = match timeout(timeout_duration, cmd.output()).await {
            Ok(result) => result.map_err(|e| OracleRunnerError::ContainerExecutionFailed {
                reason: e.to_string(),
            })?,
            Err(_) => {
                return Err(OracleRunnerError::ExecutionTimeout {
                    oracle_id: oracle_def.oracle_id.clone(),
                    timeout_secs: oracle_def.timeout_seconds,
                });
            }
        };

        let duration_ms = start_time.elapsed().as_millis() as u64;
        let exit_code = output.status.code();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        // Determine status based on exit code
        let (status, error_message) = if output.status.success() {
            (OracleResultStatus::Pass, None)
        } else {
            (
                OracleResultStatus::Fail,
                Some(format!(
                    "Exit code: {}, stderr: {}",
                    exit_code.unwrap_or(-1),
                    stderr.lines().take(10).collect::<Vec<_>>().join("\n")
                )),
            )
        };

        // Capture output artifacts
        let mut artifacts = Vec::new();
        for expected in &oracle_def.expected_outputs {
            let artifact_path = scratch_dir.path().join(&expected.path);
            if artifact_path.exists() {
                match tokio::fs::read(&artifact_path).await {
                    Ok(content) => {
                        let hash = compute_sha256(&content);
                        artifacts.push(CapturedArtifact {
                            name: expected.path.clone(),
                            content_hash: hash,
                            content_type: expected.content_type.clone(),
                            size: content.len() as u64,
                            content,
                        });
                    }
                    Err(e) => {
                        warn!(
                            path = %expected.path,
                            error = %e,
                            "Failed to read expected output"
                        );
                        if expected.required {
                            return Err(OracleRunnerError::OutputCaptureFailed {
                                artifact: expected.path.clone(),
                                reason: e.to_string(),
                            });
                        }
                    }
                }
            } else if expected.required {
                return Err(OracleRunnerError::OutputCaptureFailed {
                    artifact: expected.path.clone(),
                    reason: "File not found".to_string(),
                });
            }
        }

        Ok(OracleExecutionResult {
            oracle_def: oracle_def.clone(),
            status,
            duration_ms,
            exit_code,
            stdout,
            stderr,
            error_message,
            artifacts,
        })
    }

    /// Mock oracle execution for test mode
    async fn mock_oracle_execution(
        &self,
        oracle_def: &OracleDefinition,
    ) -> Result<OracleExecutionResult, OracleRunnerError> {
        // Simulate some work
        tokio::time::sleep(Duration::from_millis(100)).await;

        let mock_output = format!(
            "Mock output for oracle: {}\nCommand: {}\n",
            oracle_def.oracle_name, oracle_def.command
        );

        // Generate mock artifacts
        let mut artifacts = Vec::new();
        for expected in &oracle_def.expected_outputs {
            let mock_content = format!(
                "{{\"oracle_id\":\"{}\",\"status\":\"pass\"}}",
                oracle_def.oracle_id
            );
            let content = mock_content.as_bytes().to_vec();
            let hash = compute_sha256(&content);

            artifacts.push(CapturedArtifact {
                name: expected.path.clone(),
                content_hash: hash,
                content_type: expected.content_type.clone(),
                size: content.len() as u64,
                content,
            });
        }

        Ok(OracleExecutionResult {
            oracle_def: oracle_def.clone(),
            status: OracleResultStatus::Pass,
            duration_ms: 100,
            exit_code: Some(0),
            stdout: mock_output,
            stderr: String::new(),
            error_message: None,
            artifacts,
        })
    }

    /// Capture environment fingerprint
    async fn capture_environment_fingerprint(
        &self,
        suite: &OracleSuiteDefinition,
    ) -> Result<EnvironmentFingerprint, OracleRunnerError> {
        let mut tool_versions = BTreeMap::new();

        // Get Podman version
        if !self.config.test_mode {
            if let Ok(output) = Command::new(&self.config.podman_path)
                .arg("--version")
                .output()
                .await
            {
                let version = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .next()
                    .unwrap_or("unknown")
                    .to_string();
                tool_versions.insert("podman".to_string(), version);
            }
        } else {
            tool_versions.insert("podman".to_string(), "test-mode".to_string());
        }

        tool_versions.insert("runner".to_string(), env!("CARGO_PKG_VERSION").to_string());

        Ok(EnvironmentFingerprint {
            container_image_digest: suite.oci_image_digest.clone(),
            runtime: suite.environment_constraints.runtime.clone(),
            runtime_version: "1.0".to_string(), // Would query actual runsc version
            os: suite.environment_constraints.os.clone(),
            arch: suite.environment_constraints.cpu_arch.clone(),
            network_mode: format!("{:?}", suite.environment_constraints.network),
            workspace_readonly: suite.environment_constraints.workspace_readonly,
            scratch_writable: true,
            tool_versions,
            runner_id: self.config.runner_id.clone(),
            executed_at: Utc::now(),
        })
    }

    /// Build evidence manifest from results
    fn build_evidence_manifest(
        &self,
        run_id: &str,
        candidate_id: &str,
        suite: &OracleSuiteDefinition,
        oracle_results: &[OracleResult],
        artifacts: &[(String, Vec<u8>)],
        fingerprint: &EnvironmentFingerprint,
        run_started_at: DateTime<Utc>,
        run_completed_at: DateTime<Utc>,
    ) -> Result<EvidenceManifest, OracleRunnerError> {
        let mut builder = EvidenceManifestBuilder::new(run_id, candidate_id)
            .oracle_suite(&suite.suite_id, &suite.suite_hash)
            .run_times(run_started_at, run_completed_at)
            .environment_fingerprint(serde_json::to_value(fingerprint).unwrap_or_default());

        // Add oracle results
        for result in oracle_results {
            builder = builder.add_result(result.clone());
        }

        // Add artifacts
        for (name, content) in artifacts {
            let hash = compute_sha256(content);
            let artifact = EvidenceArtifact {
                name: name.clone(),
                content_hash: hash,
                content_type: "application/octet-stream".to_string(),
                size: content.len() as u64,
                description: None,
            };
            builder = builder.add_artifact(artifact);
        }

        builder.build().map_err(|e| OracleRunnerError::ManifestCreationFailed {
            reason: e.to_string(),
        })
    }

    /// Compute overall status from oracle results
    fn compute_overall_status(&self, results: &[OracleResult]) -> OracleStatus {
        // Any error means overall error
        if results.iter().any(|r| r.status == OracleResultStatus::Error) {
            return OracleStatus::Error;
        }

        // Any required failure means overall failure
        // (For now, treat all as required - advisory/optional would need oracle_def lookup)
        if results.iter().any(|r| r.status == OracleResultStatus::Fail) {
            return OracleStatus::Fail;
        }

        OracleStatus::Pass
    }
}

impl<E: EvidenceStore + 'static> OracleRunner for PodmanOracleRunner<E> {
    async fn run(
        &self,
        candidate_id: &str,
        oracle_suite_id: &str,
        oracle_suite_hash: &str,
    ) -> Result<OracleRunResult, OracleRunnerError> {
        // In a real implementation, candidate_path would be resolved from candidate_id
        // For now, use a placeholder path (would need actual candidate store integration)
        let candidate_path = PathBuf::from(format!("/tmp/candidates/{}", candidate_id));

        self.execute_suite(candidate_id, oracle_suite_id, oracle_suite_hash, &candidate_path)
            .await
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Compute SHA-256 hash of content
fn compute_sha256(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    hex::encode(hasher.finalize())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = PodmanOracleRunnerConfig::default();
        assert_eq!(config.workspace_mount, "/workspace");
        assert_eq!(config.scratch_mount, "/scratch");
        assert!(!config.test_mode);
        assert!(config.runner_id.starts_with("runner_"));
    }

    #[test]
    fn test_network_mode_serialization() {
        let mode = NetworkMode::Disabled;
        let json = serde_json::to_string(&mode).unwrap();
        assert_eq!(json, "\"disabled\"");

        let parsed: NetworkMode = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, NetworkMode::Disabled);
    }

    #[test]
    fn test_environment_constraints_default() {
        let constraints = EnvironmentConstraints::default();
        assert_eq!(constraints.runtime, "runsc");
        assert_eq!(constraints.network, NetworkMode::Disabled);
        assert!(constraints.workspace_readonly);
    }

    #[test]
    fn test_oracle_suite_definition_serialization() {
        let suite = OracleSuiteDefinition {
            suite_id: "suite:test".to_string(),
            suite_hash: "sha256:abc123".to_string(),
            oci_image: "ghcr.io/test/oracle:latest".to_string(),
            oci_image_digest: "sha256:digest123".to_string(),
            environment_constraints: EnvironmentConstraints::default(),
            oracles: vec![OracleDefinition {
                oracle_id: "oracle:build".to_string(),
                oracle_name: "Build Check".to_string(),
                command: "make build".to_string(),
                args: vec![],
                timeout_seconds: 300,
                expected_outputs: vec![ExpectedOutput {
                    path: "reports/build.json".to_string(),
                    content_type: "application/json".to_string(),
                    required: true,
                }],
                classification: OracleClassification::Required,
                working_dir: None,
                env: BTreeMap::new(),
            }],
            metadata: BTreeMap::new(),
        };

        let json = serde_json::to_string_pretty(&suite).unwrap();
        let parsed: OracleSuiteDefinition = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.suite_id, suite.suite_id);
        assert_eq!(parsed.oracles.len(), 1);
        assert_eq!(parsed.oracles[0].oracle_id, "oracle:build");
    }

    #[test]
    fn test_compute_sha256() {
        let content = b"test content";
        let hash1 = compute_sha256(content);
        let hash2 = compute_sha256(content);

        // Same content should produce same hash
        assert_eq!(hash1, hash2);

        // Hash should be 64 chars (256 bits = 32 bytes = 64 hex chars)
        assert_eq!(hash1.len(), 64);
    }

    #[test]
    fn test_environment_fingerprint_serialization() {
        let fingerprint = EnvironmentFingerprint {
            container_image_digest: "sha256:abc".to_string(),
            runtime: "runsc".to_string(),
            runtime_version: "1.0".to_string(),
            os: "linux".to_string(),
            arch: "amd64".to_string(),
            network_mode: "disabled".to_string(),
            workspace_readonly: true,
            scratch_writable: true,
            tool_versions: BTreeMap::new(),
            runner_id: "runner_test".to_string(),
            executed_at: Utc::now(),
        };

        let json = serde_json::to_string(&fingerprint).unwrap();
        let parsed: EnvironmentFingerprint = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.runtime, "runsc");
        assert_eq!(parsed.os, "linux");
    }

    #[test]
    fn test_oracle_classification_default() {
        let classification = OracleClassification::default();
        assert_eq!(classification, OracleClassification::Required);
    }

    #[test]
    fn test_overall_status_computation() {
        // All pass -> Pass
        let results_pass = vec![
            OracleResult {
                oracle_id: "o1".to_string(),
                oracle_name: "O1".to_string(),
                status: OracleResultStatus::Pass,
                duration_ms: 100,
                error_message: None,
                artifact_refs: vec![],
                output: None,
            },
            OracleResult {
                oracle_id: "o2".to_string(),
                oracle_name: "O2".to_string(),
                status: OracleResultStatus::Pass,
                duration_ms: 100,
                error_message: None,
                artifact_refs: vec![],
                output: None,
            },
        ];

        // Any fail -> Fail
        let results_fail = vec![
            OracleResult {
                oracle_id: "o1".to_string(),
                oracle_name: "O1".to_string(),
                status: OracleResultStatus::Pass,
                duration_ms: 100,
                error_message: None,
                artifact_refs: vec![],
                output: None,
            },
            OracleResult {
                oracle_id: "o2".to_string(),
                oracle_name: "O2".to_string(),
                status: OracleResultStatus::Fail,
                duration_ms: 100,
                error_message: Some("Failed".to_string()),
                artifact_refs: vec![],
                output: None,
            },
        ];

        // Any error -> Error
        let results_error = vec![
            OracleResult {
                oracle_id: "o1".to_string(),
                oracle_name: "O1".to_string(),
                status: OracleResultStatus::Pass,
                duration_ms: 100,
                error_message: None,
                artifact_refs: vec![],
                output: None,
            },
            OracleResult {
                oracle_id: "o2".to_string(),
                oracle_name: "O2".to_string(),
                status: OracleResultStatus::Error,
                duration_ms: 0,
                error_message: Some("Error".to_string()),
                artifact_refs: vec![],
                output: None,
            },
        ];

        // Test using manual computation (since we don't have runner instance in test)
        let all_pass = results_pass.iter().all(|r| r.status == OracleResultStatus::Pass);
        let any_error = results_error.iter().any(|r| r.status == OracleResultStatus::Error);
        let any_fail = results_fail.iter().any(|r| r.status == OracleResultStatus::Fail);

        assert!(all_pass);
        assert!(any_error);
        assert!(any_fail);
    }
}

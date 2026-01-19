//! Evidence Manifest Library (D-15)
//!
//! Provides the evidence manifest v1 schema, validation, and deterministic serialization.
//! Per SR-CONTRACT §7 and SR-SPEC §1.9, evidence bundles must be:
//! - Content-addressed (SHA-256 hash)
//! - Deterministically serializable
//! - Machine-readable with required fields
//!
//! The manifest follows the `evidence.gate_packet` artifact type from SR-TYPES.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use thiserror::Error;

/// Evidence manifest version
pub const MANIFEST_VERSION: &str = "v1";

/// Evidence manifest artifact type
pub const ARTIFACT_TYPE: &str = "evidence.gate_packet";

/// Evidence manifest v1 schema
///
/// The manifest describes an evidence bundle from an oracle run.
/// All fields are required unless marked optional.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceManifest {
    /// Manifest version (always "v1" for this implementation)
    pub version: String,

    /// Artifact type (always "evidence.gate_packet")
    pub artifact_type: String,

    /// Unique identifier for this evidence bundle
    pub bundle_id: String,

    /// Run identifier this evidence is associated with
    pub run_id: String,

    /// Candidate identifier being evaluated
    pub candidate_id: String,

    /// Oracle suite identifier
    pub oracle_suite_id: String,

    /// Content hash of the oracle suite (for version binding)
    pub oracle_suite_hash: String,

    /// Timestamp when the oracle run started
    pub run_started_at: DateTime<Utc>,

    /// Timestamp when the oracle run completed
    pub run_completed_at: DateTime<Utc>,

    /// Environment fingerprint (deterministic environment state)
    pub environment_fingerprint: serde_json::Value,

    /// Oracle results (per-oracle outcomes)
    pub results: Vec<OracleResult>,

    /// Overall verdict (derived from results)
    pub verdict: OracleResultStatus,

    /// List of artifacts in this bundle
    pub artifacts: Vec<EvidenceArtifact>,

    /// Additional metadata (extensible, sorted by key)
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub metadata: BTreeMap<String, serde_json::Value>,

    // ---- Work Surface Context (SR-PLAN-V4 Phase 4c) ----
    /// Template ID this evidence is associated with
    /// Per SR-SPEC §1.9.1: Evidence bundles should include stage context
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,

    /// Stage ID within the template
    /// Per SR-SPEC §1.9.1: Evidence bundles should include stage context
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_id: Option<String>,

    /// Work Surface ID this evidence was recorded under
    /// Per SR-PLAN-V4 Phase 4c: Links evidence to the Work Surface binding
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_surface_id: Option<String>,
}

/// Individual oracle result within a suite
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OracleResult {
    /// Oracle identifier within the suite
    pub oracle_id: String,

    /// Human-readable oracle name
    pub oracle_name: String,

    /// Oracle result status
    pub status: OracleResultStatus,

    /// Duration in milliseconds
    pub duration_ms: u64,

    /// Error message if status is Error
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,

    /// Output artifact references (by name)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifact_refs: Vec<String>,

    /// Oracle-specific output data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
}

/// Oracle result status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OracleResultStatus {
    /// Oracle passed
    Pass,
    /// Oracle failed (deterministic failure)
    Fail,
    /// Oracle errored (non-deterministic or infrastructure failure)
    Error,
    /// Oracle was skipped (dependency not met)
    Skipped,
}

/// Evidence artifact descriptor
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceArtifact {
    /// Artifact name (unique within bundle)
    pub name: String,

    /// Content hash of the artifact (SHA-256)
    pub content_hash: String,

    /// Content type (MIME type)
    pub content_type: String,

    /// Size in bytes
    pub size: u64,

    /// Optional description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Manifest validation errors
#[derive(Debug, Error)]
pub enum ManifestValidationError {
    #[error("Invalid manifest version: expected '{expected}', got '{actual}'")]
    InvalidVersion { expected: String, actual: String },

    #[error("Invalid artifact type: expected '{expected}', got '{actual}'")]
    InvalidArtifactType { expected: String, actual: String },

    #[error("Missing required field: {field}")]
    MissingField { field: String },

    #[error("Invalid field value: {field}: {reason}")]
    InvalidFieldValue { field: String, reason: String },

    #[error("Timestamp ordering error: run_started_at must be before run_completed_at")]
    InvalidTimestampOrder,

    #[error("Duplicate artifact name: {name}")]
    DuplicateArtifact { name: String },

    #[error("Verdict mismatch: computed '{computed:?}', declared '{declared:?}'")]
    VerdictMismatch {
        computed: OracleResultStatus,
        declared: OracleResultStatus,
    },

    #[error("Serialization error: {message}")]
    SerializationError { message: String },

    #[error("Hash mismatch: expected '{expected}', computed '{computed}'")]
    HashMismatch { expected: String, computed: String },
}

impl EvidenceManifest {
    /// Validate the manifest structure and semantics
    pub fn validate(&self) -> Result<(), ManifestValidationError> {
        // Check version
        if self.version != MANIFEST_VERSION {
            return Err(ManifestValidationError::InvalidVersion {
                expected: MANIFEST_VERSION.to_string(),
                actual: self.version.clone(),
            });
        }

        // Check artifact type
        if self.artifact_type != ARTIFACT_TYPE {
            return Err(ManifestValidationError::InvalidArtifactType {
                expected: ARTIFACT_TYPE.to_string(),
                actual: self.artifact_type.clone(),
            });
        }

        // Check required IDs are non-empty
        if self.bundle_id.is_empty() {
            return Err(ManifestValidationError::MissingField {
                field: "bundle_id".to_string(),
            });
        }
        if self.run_id.is_empty() {
            return Err(ManifestValidationError::MissingField {
                field: "run_id".to_string(),
            });
        }
        if self.candidate_id.is_empty() {
            return Err(ManifestValidationError::MissingField {
                field: "candidate_id".to_string(),
            });
        }
        if self.oracle_suite_id.is_empty() {
            return Err(ManifestValidationError::MissingField {
                field: "oracle_suite_id".to_string(),
            });
        }
        if self.oracle_suite_hash.is_empty() {
            return Err(ManifestValidationError::MissingField {
                field: "oracle_suite_hash".to_string(),
            });
        }

        // Check timestamp ordering
        if self.run_started_at > self.run_completed_at {
            return Err(ManifestValidationError::InvalidTimestampOrder);
        }

        // Check for duplicate artifact names
        let mut seen_names = std::collections::HashSet::new();
        for artifact in &self.artifacts {
            if !seen_names.insert(&artifact.name) {
                return Err(ManifestValidationError::DuplicateArtifact {
                    name: artifact.name.clone(),
                });
            }
        }

        // Verify verdict matches computed verdict
        let computed_verdict = self.compute_verdict();
        if computed_verdict != self.verdict {
            return Err(ManifestValidationError::VerdictMismatch {
                computed: computed_verdict,
                declared: self.verdict,
            });
        }

        Ok(())
    }

    /// Compute the overall verdict from individual oracle results
    pub fn compute_verdict(&self) -> OracleResultStatus {
        if self.results.is_empty() {
            return OracleResultStatus::Error;
        }

        let mut has_fail = false;
        let mut has_error = false;

        for result in &self.results {
            match result.status {
                OracleResultStatus::Error => has_error = true,
                OracleResultStatus::Fail => has_fail = true,
                OracleResultStatus::Pass | OracleResultStatus::Skipped => {}
            }
        }

        if has_error {
            OracleResultStatus::Error
        } else if has_fail {
            OracleResultStatus::Fail
        } else {
            OracleResultStatus::Pass
        }
    }

    /// Serialize to deterministic JSON (sorted keys)
    pub fn to_deterministic_json(&self) -> Result<Vec<u8>, ManifestValidationError> {
        // serde_json with BTreeMap already produces sorted keys for metadata
        // We need to ensure the overall output is deterministic
        serde_json::to_vec_pretty(self).map_err(|e| ManifestValidationError::SerializationError {
            message: e.to_string(),
        })
    }

    /// Compute the content hash of the manifest
    pub fn compute_hash(&self) -> Result<String, ManifestValidationError> {
        let json = self.to_deterministic_json()?;
        let mut hasher = Sha256::new();
        hasher.update(&json);
        Ok(hex::encode(hasher.finalize()))
    }

    /// Parse from JSON bytes
    pub fn from_json(data: &[u8]) -> Result<Self, ManifestValidationError> {
        serde_json::from_slice(data).map_err(|e| ManifestValidationError::SerializationError {
            message: e.to_string(),
        })
    }
}

/// Compute the overall verdict from a list of oracle results (standalone helper)
///
/// This is a standalone version of EvidenceManifest::compute_verdict for use
/// when you have results but not a full manifest.
pub fn compute_verdict(results: &[OracleResult]) -> OracleResultStatus {
    if results.is_empty() {
        return OracleResultStatus::Error;
    }

    let mut has_fail = false;
    let mut has_error = false;

    for result in results {
        match result.status {
            OracleResultStatus::Error => has_error = true,
            OracleResultStatus::Fail => has_fail = true,
            OracleResultStatus::Pass | OracleResultStatus::Skipped => {}
        }
    }

    if has_error {
        OracleResultStatus::Error
    } else if has_fail {
        OracleResultStatus::Fail
    } else {
        OracleResultStatus::Pass
    }
}

/// Builder for constructing evidence manifests
pub struct EvidenceManifestBuilder {
    bundle_id: Option<String>,
    run_id: Option<String>,
    candidate_id: Option<String>,
    oracle_suite_id: Option<String>,
    oracle_suite_hash: Option<String>,
    run_started_at: Option<DateTime<Utc>>,
    run_completed_at: Option<DateTime<Utc>>,
    environment_fingerprint: Option<serde_json::Value>,
    results: Vec<OracleResult>,
    artifacts: Vec<EvidenceArtifact>,
    metadata: BTreeMap<String, serde_json::Value>,
    // Work Surface context (SR-PLAN-V4 Phase 4c)
    template_id: Option<String>,
    stage_id: Option<String>,
    work_surface_id: Option<String>,
}

impl EvidenceManifestBuilder {
    pub fn new() -> Self {
        Self {
            bundle_id: None,
            run_id: None,
            candidate_id: None,
            oracle_suite_id: None,
            oracle_suite_hash: None,
            run_started_at: None,
            run_completed_at: None,
            environment_fingerprint: None,
            results: Vec::new(),
            artifacts: Vec::new(),
            metadata: BTreeMap::new(),
            template_id: None,
            stage_id: None,
            work_surface_id: None,
        }
    }

    pub fn bundle_id(mut self, id: impl Into<String>) -> Self {
        self.bundle_id = Some(id.into());
        self
    }

    pub fn run_id(mut self, id: impl Into<String>) -> Self {
        self.run_id = Some(id.into());
        self
    }

    pub fn candidate_id(mut self, id: impl Into<String>) -> Self {
        self.candidate_id = Some(id.into());
        self
    }

    pub fn oracle_suite(mut self, id: impl Into<String>, hash: impl Into<String>) -> Self {
        self.oracle_suite_id = Some(id.into());
        self.oracle_suite_hash = Some(hash.into());
        self
    }

    pub fn run_times(mut self, started: DateTime<Utc>, completed: DateTime<Utc>) -> Self {
        self.run_started_at = Some(started);
        self.run_completed_at = Some(completed);
        self
    }

    pub fn environment_fingerprint(mut self, fingerprint: serde_json::Value) -> Self {
        self.environment_fingerprint = Some(fingerprint);
        self
    }

    pub fn add_result(mut self, result: OracleResult) -> Self {
        self.results.push(result);
        self
    }

    pub fn add_artifact(mut self, artifact: EvidenceArtifact) -> Self {
        self.artifacts.push(artifact);
        self
    }

    pub fn add_metadata(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }

    /// Set Work Surface context per SR-PLAN-V4 Phase 4c
    ///
    /// This binds the evidence to a specific (template_id, stage_id, work_surface_id) triple.
    pub fn work_surface_context(
        mut self,
        template_id: impl Into<String>,
        stage_id: impl Into<String>,
        work_surface_id: impl Into<String>,
    ) -> Self {
        self.template_id = Some(template_id.into());
        self.stage_id = Some(stage_id.into());
        self.work_surface_id = Some(work_surface_id.into());
        self
    }

    /// Set just template and stage (without full work surface binding)
    pub fn template_context(
        mut self,
        template_id: impl Into<String>,
        stage_id: impl Into<String>,
    ) -> Self {
        self.template_id = Some(template_id.into());
        self.stage_id = Some(stage_id.into());
        self
    }

    pub fn build(self) -> Result<EvidenceManifest, ManifestValidationError> {
        let bundle_id = self
            .bundle_id
            .ok_or(ManifestValidationError::MissingField {
                field: "bundle_id".to_string(),
            })?;
        let run_id = self.run_id.ok_or(ManifestValidationError::MissingField {
            field: "run_id".to_string(),
        })?;
        let candidate_id = self
            .candidate_id
            .ok_or(ManifestValidationError::MissingField {
                field: "candidate_id".to_string(),
            })?;
        let oracle_suite_id =
            self.oracle_suite_id
                .ok_or(ManifestValidationError::MissingField {
                    field: "oracle_suite_id".to_string(),
                })?;
        let oracle_suite_hash =
            self.oracle_suite_hash
                .ok_or(ManifestValidationError::MissingField {
                    field: "oracle_suite_hash".to_string(),
                })?;
        let run_started_at = self
            .run_started_at
            .ok_or(ManifestValidationError::MissingField {
                field: "run_started_at".to_string(),
            })?;
        let run_completed_at =
            self.run_completed_at
                .ok_or(ManifestValidationError::MissingField {
                    field: "run_completed_at".to_string(),
                })?;
        let environment_fingerprint =
            self.environment_fingerprint
                .ok_or(ManifestValidationError::MissingField {
                    field: "environment_fingerprint".to_string(),
                })?;

        // Compute verdict from results
        let verdict = if self.results.is_empty() {
            OracleResultStatus::Error
        } else {
            let mut has_fail = false;
            let mut has_error = false;
            for result in &self.results {
                match result.status {
                    OracleResultStatus::Error => has_error = true,
                    OracleResultStatus::Fail => has_fail = true,
                    _ => {}
                }
            }
            if has_error {
                OracleResultStatus::Error
            } else if has_fail {
                OracleResultStatus::Fail
            } else {
                OracleResultStatus::Pass
            }
        };

        let manifest = EvidenceManifest {
            version: MANIFEST_VERSION.to_string(),
            artifact_type: ARTIFACT_TYPE.to_string(),
            bundle_id,
            run_id,
            candidate_id,
            oracle_suite_id,
            oracle_suite_hash,
            run_started_at,
            run_completed_at,
            environment_fingerprint,
            results: self.results,
            verdict,
            artifacts: self.artifacts,
            metadata: self.metadata,
            // Work Surface context (SR-PLAN-V4 Phase 4c)
            template_id: self.template_id,
            stage_id: self.stage_id,
            work_surface_id: self.work_surface_id,
        };

        // Validate before returning
        manifest.validate()?;
        Ok(manifest)
    }
}

impl Default for EvidenceManifestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to create an artifact descriptor from data
pub fn create_artifact(
    name: impl Into<String>,
    content_type: impl Into<String>,
    data: &[u8],
) -> EvidenceArtifact {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let content_hash = hex::encode(hasher.finalize());

    EvidenceArtifact {
        name: name.into(),
        content_hash,
        content_type: content_type.into(),
        size: data.len() as u64,
        description: None,
    }
}

/// Validation oracle for evidence manifests
///
/// This can be invoked as part of the core oracle suite to validate
/// evidence manifests stored in the system.
pub struct ManifestValidationOracle;

impl ManifestValidationOracle {
    /// Validate a manifest against the v1 schema
    pub fn validate(manifest: &EvidenceManifest) -> Result<(), ManifestValidationError> {
        manifest.validate()
    }

    /// Validate manifest JSON bytes
    pub fn validate_json(data: &[u8]) -> Result<EvidenceManifest, ManifestValidationError> {
        let manifest = EvidenceManifest::from_json(data)?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// Verify manifest hash matches expected value
    pub fn verify_hash(
        manifest: &EvidenceManifest,
        expected_hash: &str,
    ) -> Result<(), ManifestValidationError> {
        let computed = manifest.compute_hash()?;
        if computed != expected_hash {
            return Err(ManifestValidationError::HashMismatch {
                expected: expected_hash.to_string(),
                computed,
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn sample_manifest() -> EvidenceManifest {
        // Use fixed timestamp for deterministic tests
        let now = DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        EvidenceManifestBuilder::new()
            .bundle_id("bundle-001")
            .run_id("run-001")
            .candidate_id("candidate-001")
            .oracle_suite("suite.core.v1", "abc123")
            .run_times(now, now + Duration::seconds(5))
            .environment_fingerprint(serde_json::json!({
                "rust_version": "1.75.0",
                "os": "linux"
            }))
            .add_result(OracleResult {
                oracle_id: "build".to_string(),
                oracle_name: "Build Check".to_string(),
                status: OracleResultStatus::Pass,
                duration_ms: 1000,
                error_message: None,
                artifact_refs: vec!["build.log".to_string()],
                output: None,
            })
            .add_artifact(EvidenceArtifact {
                name: "build.log".to_string(),
                content_hash: "deadbeef".to_string(),
                content_type: "text/plain".to_string(),
                size: 1024,
                description: Some("Build output log".to_string()),
            })
            .build()
            .unwrap()
    }

    #[test]
    fn test_manifest_validation_success() {
        let manifest = sample_manifest();
        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn test_manifest_invalid_version() {
        let mut manifest = sample_manifest();
        manifest.version = "v2".to_string();
        let err = manifest.validate().unwrap_err();
        assert!(matches!(
            err,
            ManifestValidationError::InvalidVersion { .. }
        ));
    }

    #[test]
    fn test_manifest_invalid_artifact_type() {
        let mut manifest = sample_manifest();
        manifest.artifact_type = "wrong.type".to_string();
        let err = manifest.validate().unwrap_err();
        assert!(matches!(
            err,
            ManifestValidationError::InvalidArtifactType { .. }
        ));
    }

    #[test]
    fn test_manifest_timestamp_ordering() {
        let now = Utc::now();
        let result = EvidenceManifestBuilder::new()
            .bundle_id("bundle-001")
            .run_id("run-001")
            .candidate_id("candidate-001")
            .oracle_suite("suite.core.v1", "abc123")
            .run_times(now + Duration::seconds(10), now) // Invalid: started after completed
            .environment_fingerprint(serde_json::json!({}))
            .add_result(OracleResult {
                oracle_id: "test".to_string(),
                oracle_name: "Test".to_string(),
                status: OracleResultStatus::Pass,
                duration_ms: 100,
                error_message: None,
                artifact_refs: vec![],
                output: None,
            })
            .build();

        assert!(matches!(
            result,
            Err(ManifestValidationError::InvalidTimestampOrder)
        ));
    }

    #[test]
    fn test_manifest_duplicate_artifacts() {
        let now = Utc::now();
        let mut manifest = EvidenceManifestBuilder::new()
            .bundle_id("bundle-001")
            .run_id("run-001")
            .candidate_id("candidate-001")
            .oracle_suite("suite.core.v1", "abc123")
            .run_times(now, now + Duration::seconds(5))
            .environment_fingerprint(serde_json::json!({}))
            .add_result(OracleResult {
                oracle_id: "test".to_string(),
                oracle_name: "Test".to_string(),
                status: OracleResultStatus::Pass,
                duration_ms: 100,
                error_message: None,
                artifact_refs: vec![],
                output: None,
            })
            .add_artifact(EvidenceArtifact {
                name: "output.txt".to_string(),
                content_hash: "hash1".to_string(),
                content_type: "text/plain".to_string(),
                size: 100,
                description: None,
            })
            .build()
            .unwrap();

        // Add duplicate artifact
        manifest.artifacts.push(EvidenceArtifact {
            name: "output.txt".to_string(),
            content_hash: "hash2".to_string(),
            content_type: "text/plain".to_string(),
            size: 200,
            description: None,
        });

        let err = manifest.validate().unwrap_err();
        assert!(matches!(
            err,
            ManifestValidationError::DuplicateArtifact { .. }
        ));
    }

    #[test]
    fn test_compute_verdict() {
        let manifest = sample_manifest();
        assert_eq!(manifest.compute_verdict(), OracleResultStatus::Pass);

        // Test with failure
        let now = Utc::now();
        let manifest_with_fail = EvidenceManifestBuilder::new()
            .bundle_id("bundle-001")
            .run_id("run-001")
            .candidate_id("candidate-001")
            .oracle_suite("suite.core.v1", "abc123")
            .run_times(now, now + Duration::seconds(5))
            .environment_fingerprint(serde_json::json!({}))
            .add_result(OracleResult {
                oracle_id: "build".to_string(),
                oracle_name: "Build".to_string(),
                status: OracleResultStatus::Pass,
                duration_ms: 100,
                error_message: None,
                artifact_refs: vec![],
                output: None,
            })
            .add_result(OracleResult {
                oracle_id: "test".to_string(),
                oracle_name: "Test".to_string(),
                status: OracleResultStatus::Fail,
                duration_ms: 200,
                error_message: Some("Test failed".to_string()),
                artifact_refs: vec![],
                output: None,
            })
            .build()
            .unwrap();

        assert_eq!(manifest_with_fail.verdict, OracleResultStatus::Fail);
    }

    #[test]
    fn test_deterministic_serialization() {
        let manifest1 = sample_manifest();
        let manifest2 = sample_manifest();

        let json1 = manifest1.to_deterministic_json().unwrap();
        let json2 = manifest2.to_deterministic_json().unwrap();

        assert_eq!(json1, json2, "Serialization should be deterministic");
    }

    #[test]
    fn test_hash_computation() {
        let manifest = sample_manifest();
        let hash1 = manifest.compute_hash().unwrap();
        let hash2 = manifest.compute_hash().unwrap();

        assert_eq!(hash1, hash2, "Hash should be reproducible");
        assert_eq!(hash1.len(), 64, "SHA-256 hash should be 64 hex chars");
    }

    #[test]
    fn test_roundtrip_serialization() {
        let manifest = sample_manifest();
        let json = manifest.to_deterministic_json().unwrap();
        let parsed = EvidenceManifest::from_json(&json).unwrap();

        assert_eq!(manifest, parsed, "Roundtrip should preserve data");
    }

    #[test]
    fn test_create_artifact_helper() {
        let data = b"hello world";
        let artifact = create_artifact("test.txt", "text/plain", data);

        assert_eq!(artifact.name, "test.txt");
        assert_eq!(artifact.content_type, "text/plain");
        assert_eq!(artifact.size, 11);
        assert!(!artifact.content_hash.is_empty());
    }

    #[test]
    fn test_validation_oracle() {
        let manifest = sample_manifest();
        assert!(ManifestValidationOracle::validate(&manifest).is_ok());

        let json = manifest.to_deterministic_json().unwrap();
        let validated = ManifestValidationOracle::validate_json(&json).unwrap();
        assert_eq!(validated, manifest);
    }

    #[test]
    fn test_work_surface_context() {
        let now = DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);

        let manifest = EvidenceManifestBuilder::new()
            .bundle_id("bundle-ws-001")
            .run_id("run-001")
            .candidate_id("candidate-001")
            .oracle_suite("suite.core.v1", "abc123")
            .run_times(now, now + Duration::seconds(5))
            .environment_fingerprint(serde_json::json!({}))
            .work_surface_context("proc:template-001", "stage:draft", "ws:001")
            .add_result(OracleResult {
                oracle_id: "test".to_string(),
                oracle_name: "Test".to_string(),
                status: OracleResultStatus::Pass,
                duration_ms: 100,
                error_message: None,
                artifact_refs: vec![],
                output: None,
            })
            .build()
            .unwrap();

        assert_eq!(
            manifest.template_id,
            Some("proc:template-001".to_string())
        );
        assert_eq!(manifest.stage_id, Some("stage:draft".to_string()));
        assert_eq!(manifest.work_surface_id, Some("ws:001".to_string()));

        // Verify it serializes correctly
        let json = manifest.to_deterministic_json().unwrap();
        let json_str = String::from_utf8_lossy(&json);
        assert!(json_str.contains("template_id"));
        assert!(json_str.contains("stage_id"));
        assert!(json_str.contains("work_surface_id"));

        // Verify roundtrip
        let parsed = EvidenceManifest::from_json(&json).unwrap();
        assert_eq!(parsed.template_id, manifest.template_id);
        assert_eq!(parsed.stage_id, manifest.stage_id);
        assert_eq!(parsed.work_surface_id, manifest.work_surface_id);
    }

    #[test]
    fn test_work_surface_context_optional() {
        // Verify that manifests without Work Surface context still work
        let manifest = sample_manifest();
        assert!(manifest.template_id.is_none());
        assert!(manifest.stage_id.is_none());
        assert!(manifest.work_surface_id.is_none());

        // Verify serialization doesn't include empty optional fields
        let json = manifest.to_deterministic_json().unwrap();
        let json_str = String::from_utf8_lossy(&json);
        assert!(!json_str.contains("template_id"));
        assert!(!json_str.contains("stage_id"));
        assert!(!json_str.contains("work_surface_id"));
    }
}

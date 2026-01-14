//! Oracle Integrity Checks (D-27)
//!
//! Implements oracle-integrity checks and stop triggers consistent with SR-SPEC/SR-DIRECTIVE.
//!
//! Per SR-PLAN D-27:
//! - Tamper detection for Evidence Bundles and candidate hashes
//! - Known integrity failure modes produce explicit failure records/flags
//! - Stop triggers route to correct portal touchpoints (no silent overrides)
//!
//! Integrity conditions per SR-SPEC §1.14 (non-waivable):
//! - ORACLE_TAMPER: Evidence bundle was tampered with
//! - ORACLE_GAP: Gap in oracle coverage
//! - ORACLE_ENV_MISMATCH: Environment constraint violation
//! - ORACLE_FLAKE: Flaky oracle behavior detected
//! - EVIDENCE_MISSING: Evidence missing or incomplete
//! - MANIFEST_INVALID: Manifest validation failed

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::{debug, error, info, instrument, warn};

use crate::evidence::{EvidenceManifest, OracleResultStatus};
use crate::oracle_runner::{EnvironmentConstraints, EnvironmentFingerprint, NetworkMode, OracleSuiteDefinition};
use crate::oracle_suite::{IntegrityCondition, VerificationProfile};

// ============================================================================
// Integrity Check Results
// ============================================================================

/// Result of a comprehensive integrity check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityCheckResult {
    /// Check identifier
    pub check_id: String,
    /// Candidate ID that was checked
    pub candidate_id: String,
    /// Run ID that was checked
    pub run_id: String,
    /// Suite ID that was executed
    pub suite_id: String,
    /// Whether all integrity checks passed
    pub passed: bool,
    /// Detected integrity violations
    pub violations: Vec<IntegrityViolation>,
    /// Individual check results
    pub check_results: IntegrityCheckDetails,
    /// Timestamp of the check
    pub checked_at: DateTime<Utc>,
    /// Stop trigger recommendation (if any)
    pub stop_trigger: Option<StopTrigger>,
}

/// Detailed results of each integrity check type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IntegrityCheckDetails {
    /// Tamper detection result
    pub tamper_check: Option<TamperCheckResult>,
    /// Gap detection result
    pub gap_check: Option<GapCheckResult>,
    /// Environment mismatch check result
    pub env_check: Option<EnvironmentCheckResult>,
    /// Flake detection result
    pub flake_check: Option<FlakeCheckResult>,
}

/// An integrity violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityViolation {
    /// The condition that was violated
    pub condition: IntegrityCondition,
    /// Severity level
    pub severity: ViolationSeverity,
    /// Human-readable message
    pub message: String,
    /// Additional context
    pub context: BTreeMap<String, serde_json::Value>,
    /// Timestamp of detection
    pub detected_at: DateTime<Utc>,
}

/// Violation severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ViolationSeverity {
    /// Low - informational
    Low,
    /// Medium - notable but may have workarounds
    Medium,
    /// High - requires immediate attention
    High,
    /// Critical - blocks all progress
    Critical,
}

/// Stop trigger recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopTrigger {
    /// Trigger type
    pub trigger_type: StopTriggerType,
    /// Reason for the stop
    pub reason: String,
    /// Recommended portal for resolution
    pub recommended_portal: String,
    /// Required actions before proceeding
    pub required_actions: Vec<String>,
    /// Whether automatic retry is allowed
    pub allow_retry: bool,
}

/// Types of stop triggers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StopTriggerType {
    /// Stop due to tamper detection
    TamperDetected,
    /// Stop due to coverage gap
    CoverageGap,
    /// Stop due to environment mismatch
    EnvironmentMismatch,
    /// Stop due to flaky behavior
    FlakeDetected,
    /// Stop due to missing evidence
    EvidenceMissing,
    /// Stop due to invalid manifest
    ManifestInvalid,
}

// ============================================================================
// Individual Check Results
// ============================================================================

/// Result of tamper detection check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TamperCheckResult {
    /// Whether the check passed
    pub passed: bool,
    /// Expected manifest hash
    pub expected_manifest_hash: Option<String>,
    /// Actual manifest hash
    pub actual_manifest_hash: String,
    /// Expected candidate hash
    pub expected_candidate_hash: Option<String>,
    /// Actual candidate hash (from manifest)
    pub actual_candidate_hash: String,
    /// Individual artifact hash checks
    pub artifact_checks: Vec<ArtifactHashCheck>,
    /// Detailed findings
    pub findings: Vec<String>,
}

/// Result of an artifact hash verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactHashCheck {
    /// Artifact name
    pub name: String,
    /// Expected hash
    pub expected_hash: String,
    /// Actual hash (recomputed)
    pub actual_hash: Option<String>,
    /// Whether hashes match
    pub matched: bool,
    /// Reason for mismatch (if any)
    pub mismatch_reason: Option<String>,
}

/// Result of oracle coverage gap check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapCheckResult {
    /// Whether the check passed
    pub passed: bool,
    /// Required oracles from suite
    pub required_oracles: Vec<String>,
    /// Oracles that were executed
    pub executed_oracles: Vec<String>,
    /// Missing oracles
    pub missing_oracles: Vec<String>,
    /// Extra oracles (not in suite definition)
    pub extra_oracles: Vec<String>,
    /// Coverage percentage
    pub coverage_percentage: f64,
}

/// Result of environment constraint check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentCheckResult {
    /// Whether the check passed
    pub passed: bool,
    /// Expected constraints
    pub expected_constraints: EnvironmentConstraints,
    /// Actual environment fingerprint
    pub actual_fingerprint: EnvironmentFingerprint,
    /// Constraint violations found
    pub violations: Vec<ConstraintViolation>,
}

/// A specific constraint violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintViolation {
    /// Constraint name
    pub constraint: String,
    /// Expected value
    pub expected: String,
    /// Actual value
    pub actual: String,
    /// Whether this is a critical violation
    pub critical: bool,
}

/// Result of flake detection check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlakeCheckResult {
    /// Whether the check passed (no flakes detected)
    pub passed: bool,
    /// Oracle IDs with detected flakes
    pub flaky_oracles: Vec<String>,
    /// Flake evidence details
    pub flake_evidence: Vec<FlakeEvidence>,
    /// Historical comparison data
    pub history_window: Option<FlakeHistoryWindow>,
}

/// Evidence of flaky behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlakeEvidence {
    /// Oracle ID
    pub oracle_id: String,
    /// Run IDs where inconsistency was detected
    pub inconsistent_runs: Vec<String>,
    /// Description of the inconsistency
    pub description: String,
    /// First detection timestamp
    pub first_detected: DateTime<Utc>,
    /// Number of occurrences
    pub occurrence_count: u32,
}

/// Historical window for flake analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlakeHistoryWindow {
    /// Start of window
    pub window_start: DateTime<Utc>,
    /// End of window
    pub window_end: DateTime<Utc>,
    /// Number of runs analyzed
    pub runs_analyzed: u32,
    /// Success rate per oracle
    pub oracle_success_rates: BTreeMap<String, f64>,
}

// ============================================================================
// Integrity Checker
// ============================================================================

/// Oracle integrity checker service
pub struct IntegrityChecker {
    /// Configuration
    config: IntegrityCheckerConfig,
    /// Flake history tracker
    flake_history: Arc<RwLock<FlakeHistoryTracker>>,
}

/// Integrity checker configuration
#[derive(Debug, Clone)]
pub struct IntegrityCheckerConfig {
    /// Enable tamper detection
    pub enable_tamper_check: bool,
    /// Enable gap detection
    pub enable_gap_check: bool,
    /// Enable environment check
    pub enable_env_check: bool,
    /// Enable flake detection
    pub enable_flake_check: bool,
    /// Flake threshold (percentage of failures to consider flaky)
    pub flake_threshold: f64,
    /// Flake history window in hours
    pub flake_history_hours: u32,
    /// Minimum runs for flake detection
    pub min_runs_for_flake: u32,
}

impl Default for IntegrityCheckerConfig {
    fn default() -> Self {
        Self {
            enable_tamper_check: true,
            enable_gap_check: true,
            enable_env_check: true,
            enable_flake_check: true,
            flake_threshold: 0.2, // 20% inconsistency triggers flake
            flake_history_hours: 24,
            min_runs_for_flake: 3,
        }
    }
}

impl IntegrityChecker {
    /// Create a new integrity checker
    pub fn new(config: IntegrityCheckerConfig) -> Self {
        Self {
            config,
            flake_history: Arc::new(RwLock::new(FlakeHistoryTracker::new())),
        }
    }

    /// Run all enabled integrity checks
    #[instrument(skip(self, manifest, suite_def, artifacts))]
    pub async fn check_integrity(
        &self,
        candidate_id: &str,
        run_id: &str,
        manifest: &EvidenceManifest,
        suite_def: &OracleSuiteDefinition,
        profile: Option<&VerificationProfile>,
        artifacts: Option<&HashMap<String, Vec<u8>>>,
        expected_manifest_hash: Option<&str>,
        expected_candidate_hash: Option<&str>,
    ) -> IntegrityCheckResult {
        let check_id = format!("integrity_{}", ulid::Ulid::new());
        let mut violations = Vec::new();
        let mut check_results = IntegrityCheckDetails::default();

        info!(
            check_id = %check_id,
            candidate_id = %candidate_id,
            run_id = %run_id,
            "Starting integrity checks"
        );

        // Run tamper check
        if self.config.enable_tamper_check {
            let tamper_result = self.check_tamper(
                manifest,
                artifacts,
                expected_manifest_hash,
                expected_candidate_hash,
            );

            if !tamper_result.passed {
                violations.push(IntegrityViolation {
                    condition: IntegrityCondition::OracleTamper,
                    severity: ViolationSeverity::Critical,
                    message: format!(
                        "Tamper detected: {}",
                        tamper_result.findings.join("; ")
                    ),
                    context: BTreeMap::from_iter([
                        ("expected_manifest_hash".to_string(),
                         serde_json::json!(tamper_result.expected_manifest_hash)),
                        ("actual_manifest_hash".to_string(),
                         serde_json::json!(tamper_result.actual_manifest_hash)),
                    ]),
                    detected_at: Utc::now(),
                });
            }
            check_results.tamper_check = Some(tamper_result);
        }

        // Run gap check
        if self.config.enable_gap_check {
            let gap_result = self.check_coverage_gap(manifest, suite_def);

            if !gap_result.passed {
                violations.push(IntegrityViolation {
                    condition: IntegrityCondition::OracleGap,
                    severity: ViolationSeverity::Critical,
                    message: format!(
                        "Coverage gap detected: missing oracles {:?}",
                        gap_result.missing_oracles
                    ),
                    context: BTreeMap::from_iter([
                        ("missing_oracles".to_string(),
                         serde_json::json!(gap_result.missing_oracles)),
                        ("coverage_percentage".to_string(),
                         serde_json::json!(gap_result.coverage_percentage)),
                    ]),
                    detected_at: Utc::now(),
                });
            }
            check_results.gap_check = Some(gap_result);
        }

        // Run environment check
        if self.config.enable_env_check {
            let env_result = self.check_environment(manifest, suite_def);

            if !env_result.passed {
                let critical_violations: Vec<_> = env_result.violations.iter()
                    .filter(|v| v.critical)
                    .map(|v| format!("{}: expected {}, got {}", v.constraint, v.expected, v.actual))
                    .collect();

                violations.push(IntegrityViolation {
                    condition: IntegrityCondition::OracleEnvMismatch,
                    severity: if critical_violations.is_empty() { ViolationSeverity::High } else { ViolationSeverity::Critical },
                    message: format!(
                        "Environment mismatch: {}",
                        if critical_violations.is_empty() {
                            "non-critical violations detected".to_string()
                        } else {
                            critical_violations.join("; ")
                        }
                    ),
                    context: BTreeMap::from_iter([
                        ("violations".to_string(),
                         serde_json::to_value(&env_result.violations).unwrap_or_default()),
                    ]),
                    detected_at: Utc::now(),
                });
            }
            check_results.env_check = Some(env_result);
        }

        // Run flake check
        if self.config.enable_flake_check {
            let flake_result = self.check_flakes(manifest, run_id).await;

            if !flake_result.passed {
                violations.push(IntegrityViolation {
                    condition: IntegrityCondition::OracleFlake,
                    severity: ViolationSeverity::High,
                    message: format!(
                        "Flaky behavior detected in oracles: {:?}",
                        flake_result.flaky_oracles
                    ),
                    context: BTreeMap::from_iter([
                        ("flaky_oracles".to_string(),
                         serde_json::json!(flake_result.flaky_oracles)),
                    ]),
                    detected_at: Utc::now(),
                });
            }
            check_results.flake_check = Some(flake_result);
        }

        // Check for missing evidence
        if manifest.artifacts.is_empty() {
            violations.push(IntegrityViolation {
                condition: IntegrityCondition::EvidenceMissing,
                severity: ViolationSeverity::Critical,
                message: "No evidence artifacts present in manifest".to_string(),
                context: BTreeMap::new(),
                detected_at: Utc::now(),
            });
        }

        // Check for verdict mismatch
        let expected_verdict = crate::evidence::compute_verdict(&manifest.results);
        if manifest.verdict != expected_verdict {
            violations.push(IntegrityViolation {
                condition: IntegrityCondition::ManifestInvalid,
                severity: ViolationSeverity::Critical,
                message: format!(
                    "Manifest verdict mismatch: declared {:?}, computed {:?}",
                    manifest.verdict, expected_verdict
                ),
                context: BTreeMap::from_iter([
                    ("declared_verdict".to_string(), serde_json::json!(manifest.verdict)),
                    ("computed_verdict".to_string(), serde_json::json!(expected_verdict)),
                ]),
                detected_at: Utc::now(),
            });
        }

        let passed = violations.is_empty();
        let stop_trigger = if !passed {
            Some(self.compute_stop_trigger(&violations, profile))
        } else {
            None
        };

        info!(
            check_id = %check_id,
            passed = passed,
            violation_count = violations.len(),
            "Integrity checks completed"
        );

        IntegrityCheckResult {
            check_id,
            candidate_id: candidate_id.to_string(),
            run_id: run_id.to_string(),
            suite_id: suite_def.suite_id.clone(),
            passed,
            violations,
            check_results,
            checked_at: Utc::now(),
            stop_trigger,
        }
    }

    /// Check for evidence tampering
    fn check_tamper(
        &self,
        manifest: &EvidenceManifest,
        artifacts: Option<&HashMap<String, Vec<u8>>>,
        expected_manifest_hash: Option<&str>,
        expected_candidate_hash: Option<&str>,
    ) -> TamperCheckResult {
        let mut findings = Vec::new();
        let mut artifact_checks = Vec::new();

        // Compute actual manifest hash
        let manifest_json = serde_json::to_string(manifest).unwrap_or_default();
        let actual_manifest_hash = compute_sha256(manifest_json.as_bytes());

        // Check manifest hash if expected is provided
        let manifest_hash_valid = if let Some(expected) = expected_manifest_hash {
            if expected != actual_manifest_hash {
                findings.push(format!(
                    "Manifest hash mismatch: expected {}, got {}",
                    expected, actual_manifest_hash
                ));
                false
            } else {
                true
            }
        } else {
            true // No expected hash provided, skip check
        };

        // Check candidate hash
        let candidate_hash_valid = if let Some(expected) = expected_candidate_hash {
            if expected != manifest.candidate_id {
                findings.push(format!(
                    "Candidate ID mismatch in manifest: expected {}, got {}",
                    expected, manifest.candidate_id
                ));
                false
            } else {
                true
            }
        } else {
            true
        };

        // Verify artifact hashes if artifacts provided
        if let Some(artifact_data) = artifacts {
            for artifact in &manifest.artifacts {
                let check = if let Some(content) = artifact_data.get(&artifact.name) {
                    let actual_hash = compute_sha256(content);
                    let matched = actual_hash == artifact.content_hash;

                    if !matched {
                        findings.push(format!(
                            "Artifact '{}' hash mismatch: expected {}, got {}",
                            artifact.name, artifact.content_hash, actual_hash
                        ));
                    }

                    ArtifactHashCheck {
                        name: artifact.name.clone(),
                        expected_hash: artifact.content_hash.clone(),
                        actual_hash: Some(actual_hash),
                        matched,
                        mismatch_reason: if !matched {
                            Some("Hash does not match declared value".to_string())
                        } else {
                            None
                        },
                    }
                } else {
                    findings.push(format!("Artifact '{}' not found for verification", artifact.name));

                    ArtifactHashCheck {
                        name: artifact.name.clone(),
                        expected_hash: artifact.content_hash.clone(),
                        actual_hash: None,
                        matched: false,
                        mismatch_reason: Some("Artifact content not provided for verification".to_string()),
                    }
                };

                artifact_checks.push(check);
            }
        }

        let artifacts_valid = artifact_checks.iter().all(|c| c.matched);
        let passed = manifest_hash_valid && candidate_hash_valid && artifacts_valid;

        TamperCheckResult {
            passed,
            expected_manifest_hash: expected_manifest_hash.map(String::from),
            actual_manifest_hash,
            expected_candidate_hash: expected_candidate_hash.map(String::from),
            actual_candidate_hash: manifest.candidate_id.clone(),
            artifact_checks,
            findings,
        }
    }

    /// Check for oracle coverage gaps
    fn check_coverage_gap(
        &self,
        manifest: &EvidenceManifest,
        suite_def: &OracleSuiteDefinition,
    ) -> GapCheckResult {
        // Get required oracle IDs from suite definition
        let required_oracles: BTreeSet<String> = suite_def.oracles.iter()
            .filter(|o| o.classification == crate::oracle_runner::OracleClassification::Required)
            .map(|o| o.oracle_id.clone())
            .collect();

        // Get executed oracle IDs from manifest
        let executed_oracles: BTreeSet<String> = manifest.results.iter()
            .map(|r| r.oracle_id.clone())
            .collect();

        // Find missing and extra oracles
        let missing_oracles: Vec<String> = required_oracles.difference(&executed_oracles)
            .cloned()
            .collect();

        let all_suite_oracles: BTreeSet<String> = suite_def.oracles.iter()
            .map(|o| o.oracle_id.clone())
            .collect();

        let extra_oracles: Vec<String> = executed_oracles.difference(&all_suite_oracles)
            .cloned()
            .collect();

        // Calculate coverage
        let coverage_percentage = if required_oracles.is_empty() {
            100.0
        } else {
            let covered = required_oracles.intersection(&executed_oracles).count();
            (covered as f64 / required_oracles.len() as f64) * 100.0
        };

        let passed = missing_oracles.is_empty();

        GapCheckResult {
            passed,
            required_oracles: required_oracles.into_iter().collect(),
            executed_oracles: executed_oracles.into_iter().collect(),
            missing_oracles,
            extra_oracles,
            coverage_percentage,
        }
    }

    /// Check environment constraints
    fn check_environment(
        &self,
        manifest: &EvidenceManifest,
        suite_def: &OracleSuiteDefinition,
    ) -> EnvironmentCheckResult {
        let constraints = &suite_def.environment_constraints;
        let mut violations = Vec::new();

        // Parse fingerprint from manifest
        let fingerprint: EnvironmentFingerprint = match serde_json::from_value(
            manifest.environment_fingerprint.clone()
        ) {
            Ok(fp) => fp,
            Err(e) => {
                // Cannot verify - return failed with parse error
                return EnvironmentCheckResult {
                    passed: false,
                    expected_constraints: constraints.clone(),
                    actual_fingerprint: EnvironmentFingerprint {
                        container_image_digest: String::new(),
                        runtime: "unknown".to_string(),
                        runtime_version: String::new(),
                        os: String::new(),
                        arch: String::new(),
                        network_mode: String::new(),
                        workspace_readonly: false,
                        scratch_writable: false,
                        tool_versions: BTreeMap::new(),
                        runner_id: String::new(),
                        executed_at: Utc::now(),
                    },
                    violations: vec![ConstraintViolation {
                        constraint: "fingerprint_parse".to_string(),
                        expected: "valid fingerprint".to_string(),
                        actual: format!("parse error: {}", e),
                        critical: true,
                    }],
                };
            }
        };

        // Check runtime
        if constraints.runtime != fingerprint.runtime {
            violations.push(ConstraintViolation {
                constraint: "runtime".to_string(),
                expected: constraints.runtime.clone(),
                actual: fingerprint.runtime.clone(),
                critical: true, // Runtime mismatch is critical (security)
            });
        }

        // Check network mode
        let expected_network = format!("{:?}", constraints.network);
        if expected_network.to_lowercase() != fingerprint.network_mode.to_lowercase() {
            violations.push(ConstraintViolation {
                constraint: "network_mode".to_string(),
                expected: expected_network,
                actual: fingerprint.network_mode.clone(),
                critical: constraints.network == NetworkMode::Disabled, // Critical if expecting disabled
            });
        }

        // Check workspace read-only
        if constraints.workspace_readonly && !fingerprint.workspace_readonly {
            violations.push(ConstraintViolation {
                constraint: "workspace_readonly".to_string(),
                expected: "true".to_string(),
                actual: "false".to_string(),
                critical: true, // Read-only workspace is critical for determinism
            });
        }

        // Check architecture
        if constraints.cpu_arch != fingerprint.arch {
            violations.push(ConstraintViolation {
                constraint: "cpu_arch".to_string(),
                expected: constraints.cpu_arch.clone(),
                actual: fingerprint.arch.clone(),
                critical: false, // Usually not critical but notable
            });
        }

        // Check OS
        if constraints.os != fingerprint.os {
            violations.push(ConstraintViolation {
                constraint: "os".to_string(),
                expected: constraints.os.clone(),
                actual: fingerprint.os.clone(),
                critical: false,
            });
        }

        let has_critical = violations.iter().any(|v| v.critical);
        let passed = !has_critical;

        EnvironmentCheckResult {
            passed,
            expected_constraints: constraints.clone(),
            actual_fingerprint: fingerprint,
            violations,
        }
    }

    /// Check for flaky oracle behavior
    async fn check_flakes(
        &self,
        manifest: &EvidenceManifest,
        run_id: &str,
    ) -> FlakeCheckResult {
        // Record current run results
        let mut history = self.flake_history.write().await;

        for result in &manifest.results {
            history.record_result(
                &result.oracle_id,
                run_id,
                result.status == OracleResultStatus::Pass,
            );
        }

        // Analyze for flakes
        let mut flaky_oracles = Vec::new();
        let mut flake_evidence = Vec::new();

        let cutoff = Utc::now() - Duration::hours(self.config.flake_history_hours as i64);

        for result in &manifest.results {
            if let Some(stats) = history.get_oracle_stats(&result.oracle_id, cutoff) {
                if stats.total_runs >= self.config.min_runs_for_flake {
                    let success_rate = stats.successes as f64 / stats.total_runs as f64;

                    // Flake = not consistently passing or failing
                    let is_flaky = success_rate > self.config.flake_threshold
                        && success_rate < (1.0 - self.config.flake_threshold);

                    if is_flaky {
                        flaky_oracles.push(result.oracle_id.clone());
                        flake_evidence.push(FlakeEvidence {
                            oracle_id: result.oracle_id.clone(),
                            inconsistent_runs: stats.run_ids.clone(),
                            description: format!(
                                "Success rate {}% over {} runs (threshold: {}%)",
                                (success_rate * 100.0) as u32,
                                stats.total_runs,
                                (self.config.flake_threshold * 100.0) as u32
                            ),
                            first_detected: stats.first_run,
                            occurrence_count: stats.total_runs,
                        });
                    }
                }
            }
        }

        let passed = flaky_oracles.is_empty();

        // Build history window if we have enough data
        let history_window = if history.total_runs() >= self.config.min_runs_for_flake {
            let mut oracle_success_rates = BTreeMap::new();
            for result in &manifest.results {
                if let Some(stats) = history.get_oracle_stats(&result.oracle_id, cutoff) {
                    oracle_success_rates.insert(
                        result.oracle_id.clone(),
                        stats.successes as f64 / stats.total_runs as f64,
                    );
                }
            }

            Some(FlakeHistoryWindow {
                window_start: cutoff,
                window_end: Utc::now(),
                runs_analyzed: history.total_runs(),
                oracle_success_rates,
            })
        } else {
            None
        };

        FlakeCheckResult {
            passed,
            flaky_oracles,
            flake_evidence,
            history_window,
        }
    }

    /// Compute stop trigger from violations
    fn compute_stop_trigger(
        &self,
        violations: &[IntegrityViolation],
        _profile: Option<&VerificationProfile>,
    ) -> StopTrigger {
        // Find most severe violation to determine trigger type
        let most_severe = violations.iter()
            .max_by_key(|v| match v.severity {
                ViolationSeverity::Critical => 4,
                ViolationSeverity::High => 3,
                ViolationSeverity::Medium => 2,
                ViolationSeverity::Low => 1,
            });

        let (trigger_type, recommended_portal) = if let Some(violation) = most_severe {
            match violation.condition {
                IntegrityCondition::OracleTamper => (
                    StopTriggerType::TamperDetected,
                    "SecurityReviewPortal".to_string(),
                ),
                IntegrityCondition::OracleGap => (
                    StopTriggerType::CoverageGap,
                    "OracleSuiteChangePortal".to_string(),
                ),
                IntegrityCondition::OracleEnvMismatch => (
                    StopTriggerType::EnvironmentMismatch,
                    "InfrastructureReviewPortal".to_string(),
                ),
                IntegrityCondition::OracleFlake => (
                    StopTriggerType::FlakeDetected,
                    "TestStabilityPortal".to_string(),
                ),
                IntegrityCondition::EvidenceMissing => (
                    StopTriggerType::EvidenceMissing,
                    "EvidenceReviewPortal".to_string(),
                ),
                IntegrityCondition::ManifestInvalid => (
                    StopTriggerType::ManifestInvalid,
                    "GovernanceChangePortal".to_string(),
                ),
            }
        } else {
            (StopTriggerType::EvidenceMissing, "GovernanceChangePortal".to_string())
        };

        let reason = violations.iter()
            .map(|v| v.message.clone())
            .collect::<Vec<_>>()
            .join("; ");

        // Determine required actions based on violation types
        let mut required_actions = Vec::new();
        for violation in violations {
            match violation.condition {
                IntegrityCondition::OracleTamper => {
                    required_actions.push("Investigate evidence chain for tampering".to_string());
                    required_actions.push("Verify source artifacts".to_string());
                }
                IntegrityCondition::OracleGap => {
                    required_actions.push("Execute missing oracles".to_string());
                    required_actions.push("Verify suite definition matches profile".to_string());
                }
                IntegrityCondition::OracleEnvMismatch => {
                    required_actions.push("Verify execution environment configuration".to_string());
                    required_actions.push("Check runner sandbox settings".to_string());
                }
                IntegrityCondition::OracleFlake => {
                    required_actions.push("Investigate flaky oracle behavior".to_string());
                    required_actions.push("Consider increasing retry count or timeout".to_string());
                }
                IntegrityCondition::EvidenceMissing => {
                    required_actions.push("Re-run oracle suite to generate evidence".to_string());
                }
                IntegrityCondition::ManifestInvalid => {
                    required_actions.push("Regenerate evidence manifest".to_string());
                    required_actions.push("Verify manifest schema compliance".to_string());
                }
            }
        }

        // Remove duplicates
        required_actions.sort();
        required_actions.dedup();

        // Flakes may allow retry, others generally don't
        let allow_retry = trigger_type == StopTriggerType::FlakeDetected;

        StopTrigger {
            trigger_type,
            reason,
            recommended_portal,
            required_actions,
            allow_retry,
        }
    }

    /// Verify profile integrity conditions are satisfied
    pub fn verify_profile_conditions(
        &self,
        result: &IntegrityCheckResult,
        profile: &VerificationProfile,
    ) -> ProfileVerificationResult {
        let mut blocking_violations = Vec::new();

        for violation in &result.violations {
            if profile.integrity_conditions.contains(&violation.condition) {
                blocking_violations.push(violation.clone());
            }
        }

        let passed = blocking_violations.is_empty();

        ProfileVerificationResult {
            profile_id: profile.profile_id.clone(),
            passed,
            blocking_violations,
            checked_at: Utc::now(),
        }
    }
}

/// Result of profile verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileVerificationResult {
    /// Profile that was verified against
    pub profile_id: String,
    /// Whether all profile integrity conditions are satisfied
    pub passed: bool,
    /// Violations that block progress per profile
    pub blocking_violations: Vec<IntegrityViolation>,
    /// Timestamp
    pub checked_at: DateTime<Utc>,
}

// ============================================================================
// Flake History Tracking
// ============================================================================

/// Tracks oracle results for flake detection
struct FlakeHistoryTracker {
    /// Results per oracle ID
    results: HashMap<String, Vec<OracleRunRecord>>,
}

struct OracleRunRecord {
    run_id: String,
    passed: bool,
    recorded_at: DateTime<Utc>,
}

impl FlakeHistoryTracker {
    fn new() -> Self {
        Self {
            results: HashMap::new(),
        }
    }

    fn record_result(&mut self, oracle_id: &str, run_id: &str, passed: bool) {
        let record = OracleRunRecord {
            run_id: run_id.to_string(),
            passed,
            recorded_at: Utc::now(),
        };

        self.results
            .entry(oracle_id.to_string())
            .or_insert_with(Vec::new)
            .push(record);
    }

    fn get_oracle_stats(&self, oracle_id: &str, since: DateTime<Utc>) -> Option<OracleStats> {
        let records = self.results.get(oracle_id)?;

        let filtered: Vec<_> = records.iter()
            .filter(|r| r.recorded_at >= since)
            .collect();

        if filtered.is_empty() {
            return None;
        }

        let successes = filtered.iter().filter(|r| r.passed).count() as u32;
        let total_runs = filtered.len() as u32;
        let run_ids: Vec<String> = filtered.iter().map(|r| r.run_id.clone()).collect();
        let first_run = filtered.iter().map(|r| r.recorded_at).min().unwrap();

        Some(OracleStats {
            successes,
            total_runs,
            run_ids,
            first_run,
        })
    }

    fn total_runs(&self) -> u32 {
        self.results.values().map(|v| v.len() as u32).sum()
    }
}

struct OracleStats {
    successes: u32,
    total_runs: u32,
    run_ids: Vec<String>,
    first_run: DateTime<Utc>,
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Compute SHA-256 hash
fn compute_sha256(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    hex::encode(hasher.finalize())
}

// ============================================================================
// Errors
// ============================================================================

/// Integrity check errors
#[derive(Debug, Error)]
pub enum IntegrityError {
    #[error("Tamper detected: {0}")]
    TamperDetected(String),

    #[error("Coverage gap detected: missing oracles {0:?}")]
    CoverageGap(Vec<String>),

    #[error("Environment mismatch: {0}")]
    EnvironmentMismatch(String),

    #[error("Flake detected in oracles: {0:?}")]
    FlakeDetected(Vec<String>),

    #[error("Evidence missing: {0}")]
    EvidenceMissing(String),

    #[error("Manifest invalid: {0}")]
    ManifestInvalid(String),
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence::{EvidenceArtifact, OracleResult};
    use crate::oracle_runner::{OracleClassification, OracleDefinition, ExpectedOutput};

    fn create_test_manifest() -> EvidenceManifest {
        EvidenceManifest {
            version: "v1".to_string(),
            artifact_type: "evidence.gate_packet".to_string(),
            bundle_id: "bundle_123".to_string(),
            run_id: "run_123".to_string(),
            candidate_id: "candidate_123".to_string(),
            oracle_suite_id: "suite:test".to_string(),
            oracle_suite_hash: "sha256:abc".to_string(),
            run_started_at: Utc::now(),
            run_completed_at: Utc::now(),
            environment_fingerprint: serde_json::json!({
                "runtime": "runsc",
                "runtime_version": "1.0",
                "os": "linux",
                "arch": "amd64",
                "network_mode": "Disabled",
                "workspace_readonly": true,
                "scratch_writable": true,
                "container_image_digest": "sha256:test",
                "tool_versions": {},
                "runner_id": "test_runner",
                "executed_at": Utc::now().to_rfc3339()
            }),
            results: vec![
                OracleResult {
                    oracle_id: "oracle:build".to_string(),
                    oracle_name: "Build".to_string(),
                    status: OracleResultStatus::Pass,
                    duration_ms: 100,
                    error_message: None,
                    artifact_refs: vec!["abc123".to_string()],
                    output: None,
                },
            ],
            verdict: OracleResultStatus::Pass,
            artifacts: vec![
                EvidenceArtifact {
                    name: "build.json".to_string(),
                    content_hash: "abc123".to_string(),
                    content_type: "application/json".to_string(),
                    size: 100,
                    description: None,
                },
            ],
            metadata: BTreeMap::new(),
        }
    }

    fn create_test_suite() -> OracleSuiteDefinition {
        OracleSuiteDefinition {
            suite_id: "suite:test".to_string(),
            suite_hash: "sha256:abc".to_string(),
            oci_image: "test:latest".to_string(),
            oci_image_digest: "sha256:test".to_string(),
            environment_constraints: EnvironmentConstraints {
                runtime: "runsc".to_string(),
                network: NetworkMode::Disabled,
                cpu_arch: "amd64".to_string(),
                os: "linux".to_string(),
                workspace_readonly: true,
                additional_constraints: vec![],
            },
            oracles: vec![
                OracleDefinition {
                    oracle_id: "oracle:build".to_string(),
                    oracle_name: "Build".to_string(),
                    command: "make build".to_string(),
                    args: vec![],
                    timeout_seconds: 300,
                    expected_outputs: vec![ExpectedOutput {
                        path: "build.json".to_string(),
                        content_type: "application/json".to_string(),
                        required: true,
                    }],
                    classification: OracleClassification::Required,
                    working_dir: None,
                    env: BTreeMap::new(),
                },
            ],
            metadata: BTreeMap::new(),
        }
    }

    #[tokio::test]
    async fn test_integrity_check_passes() {
        let checker = IntegrityChecker::new(IntegrityCheckerConfig::default());
        let manifest = create_test_manifest();
        let suite = create_test_suite();

        let result = checker.check_integrity(
            "candidate_123",
            "run_123",
            &manifest,
            &suite,
            None,
            None,
            None,
            None,
        ).await;

        assert!(result.passed, "Expected integrity check to pass: {:?}", result.violations);
        assert!(result.violations.is_empty());
        assert!(result.stop_trigger.is_none());
    }

    #[tokio::test]
    async fn test_tamper_detection() {
        let checker = IntegrityChecker::new(IntegrityCheckerConfig::default());
        let manifest = create_test_manifest();
        let suite = create_test_suite();

        // Provide wrong expected hash
        let result = checker.check_integrity(
            "candidate_123",
            "run_123",
            &manifest,
            &suite,
            None,
            None,
            Some("wrong_hash"),
            None,
        ).await;

        assert!(!result.passed);
        assert!(result.violations.iter().any(|v| v.condition == IntegrityCondition::OracleTamper));
    }

    #[tokio::test]
    async fn test_coverage_gap_detection() {
        let checker = IntegrityChecker::new(IntegrityCheckerConfig::default());
        let manifest = create_test_manifest();

        // Suite with more required oracles than manifest has
        let mut suite = create_test_suite();
        suite.oracles.push(OracleDefinition {
            oracle_id: "oracle:test".to_string(),
            oracle_name: "Test".to_string(),
            command: "make test".to_string(),
            args: vec![],
            timeout_seconds: 300,
            expected_outputs: vec![],
            classification: OracleClassification::Required,
            working_dir: None,
            env: BTreeMap::new(),
        });

        let result = checker.check_integrity(
            "candidate_123",
            "run_123",
            &manifest,
            &suite,
            None,
            None,
            None,
            None,
        ).await;

        assert!(!result.passed);
        assert!(result.violations.iter().any(|v| v.condition == IntegrityCondition::OracleGap));

        let gap_check = result.check_results.gap_check.unwrap();
        assert!(gap_check.missing_oracles.contains(&"oracle:test".to_string()));
    }

    #[tokio::test]
    async fn test_environment_mismatch_detection() {
        let checker = IntegrityChecker::new(IntegrityCheckerConfig::default());

        // Manifest with wrong runtime
        let mut manifest = create_test_manifest();
        manifest.environment_fingerprint = serde_json::json!({
            "runtime": "runc", // Wrong runtime!
            "runtime_version": "1.0",
            "os": "linux",
            "arch": "amd64",
            "network_mode": "Disabled",
            "workspace_readonly": true,
            "scratch_writable": true,
            "container_image_digest": "sha256:test",
            "tool_versions": {},
            "runner_id": "test_runner",
            "executed_at": Utc::now().to_rfc3339()
        });

        let suite = create_test_suite();

        let result = checker.check_integrity(
            "candidate_123",
            "run_123",
            &manifest,
            &suite,
            None,
            None,
            None,
            None,
        ).await;

        assert!(!result.passed);
        assert!(result.violations.iter().any(|v| v.condition == IntegrityCondition::OracleEnvMismatch));

        let env_check = result.check_results.env_check.unwrap();
        assert!(env_check.violations.iter().any(|v| v.constraint == "runtime"));
    }

    #[test]
    fn test_stop_trigger_generation() {
        let checker = IntegrityChecker::new(IntegrityCheckerConfig::default());

        let violations = vec![
            IntegrityViolation {
                condition: IntegrityCondition::OracleTamper,
                severity: ViolationSeverity::Critical,
                message: "Tamper detected".to_string(),
                context: BTreeMap::new(),
                detected_at: Utc::now(),
            },
        ];

        let trigger = checker.compute_stop_trigger(&violations, None);

        assert_eq!(trigger.trigger_type, StopTriggerType::TamperDetected);
        assert_eq!(trigger.recommended_portal, "SecurityReviewPortal");
        assert!(!trigger.allow_retry);
    }

    #[test]
    fn test_violation_severity_ordering() {
        assert!(ViolationSeverity::Critical as u8 > ViolationSeverity::High as u8);
        assert!(ViolationSeverity::High as u8 > ViolationSeverity::Medium as u8);
        assert!(ViolationSeverity::Medium as u8 > ViolationSeverity::Low as u8);
    }

    #[test]
    fn test_flake_history_tracker() {
        let mut tracker = FlakeHistoryTracker::new();

        tracker.record_result("oracle:build", "run_1", true);
        tracker.record_result("oracle:build", "run_2", true);
        tracker.record_result("oracle:build", "run_3", false);

        let cutoff = Utc::now() - Duration::hours(1);
        let stats = tracker.get_oracle_stats("oracle:build", cutoff).unwrap();

        assert_eq!(stats.total_runs, 3);
        assert_eq!(stats.successes, 2);
    }

    #[test]
    fn test_artifact_hash_verification() {
        let checker = IntegrityChecker::new(IntegrityCheckerConfig::default());
        let manifest = create_test_manifest();

        // Correct artifact content that matches hash
        let mut artifacts = HashMap::new();
        // We need to compute what content produces "abc123" hash
        // For test purposes, let's modify the manifest to use a real hash

        // Create content and compute its real hash
        let content = b"test content";
        let hash = compute_sha256(content);

        // Create manifest with real hash
        let mut test_manifest = manifest.clone();
        test_manifest.artifacts[0].content_hash = hash.clone();

        artifacts.insert("build.json".to_string(), content.to_vec());

        let result = checker.check_tamper(
            &test_manifest,
            Some(&artifacts),
            None,
            None,
        );

        assert!(result.passed);
        assert!(result.artifact_checks[0].matched);
    }

    #[test]
    fn test_profile_verification() {
        let checker = IntegrityChecker::new(IntegrityCheckerConfig::default());

        let result = IntegrityCheckResult {
            check_id: "check_123".to_string(),
            candidate_id: "candidate_123".to_string(),
            run_id: "run_123".to_string(),
            suite_id: "suite:test".to_string(),
            passed: false,
            violations: vec![
                IntegrityViolation {
                    condition: IntegrityCondition::OracleTamper,
                    severity: ViolationSeverity::Critical,
                    message: "Tamper detected".to_string(),
                    context: BTreeMap::new(),
                    detected_at: Utc::now(),
                },
            ],
            check_results: IntegrityCheckDetails::default(),
            checked_at: Utc::now(),
            stop_trigger: None,
        };

        let profile = VerificationProfile {
            profile_id: "profile:test".to_string(),
            name: "Test Profile".to_string(),
            description: "Test".to_string(),
            required_suites: vec![],
            optional_suites: vec![],
            waivable_failures: vec![],
            integrity_conditions: vec![IntegrityCondition::OracleTamper],
            applicable_deliverables: vec![],
            metadata: BTreeMap::new(),
        };

        let verification = checker.verify_profile_conditions(&result, &profile);

        assert!(!verification.passed);
        assert_eq!(verification.blocking_violations.len(), 1);
    }
}

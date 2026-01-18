//! Oracle Integrity Conditions per SR-CONTRACT §6 and SR-PLAN-V8 §V8-3
//!
//! Implements detection types for integrity conditions that MUST halt progression
//! and escalate per C-OR-7:
//!
//! - ORACLE_TAMPER (C-OR-2): Suite hash mismatch at run start
//! - ORACLE_GAP (C-OR-4): Missing required oracle result
//! - ORACLE_FLAKE (C-OR-5): Non-deterministic required oracle
//! - ORACLE_ENV_MISMATCH (C-OR-3): Environment constraint violation
//!
//! All integrity conditions block progression and require escalation.

use serde::{Deserialize, Serialize};

// ============================================================================
// Integrity Condition Types
// ============================================================================

/// Integrity condition detected during oracle execution
///
/// Per SR-CONTRACT §6, these conditions MUST halt progression and escalate (C-OR-7).
/// Each variant captures the detection metadata needed for audit and resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "condition_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IntegrityCondition {
    /// C-OR-2: Suite definition changed during run (hash mismatch)
    ///
    /// Detected when the suite hash at run time doesn't match the pinned hash
    /// from run start. This indicates the suite was modified after pinning.
    #[serde(rename = "ORACLE_TAMPER")]
    OracleTamper {
        /// Expected suite hash (pinned at run start)
        expected_hash: String,
        /// Actual suite hash (retrieved from registry)
        actual_hash: String,
        /// Suite identifier
        suite_id: String,
    },

    /// C-OR-4: Required oracle has no recorded result
    ///
    /// Detected after suite execution when one or more required oracles
    /// did not produce a result. No Verified claim until resolved.
    #[serde(rename = "ORACLE_GAP")]
    OracleGap {
        /// Oracle IDs that are missing results
        missing_oracles: Vec<String>,
        /// Suite identifier
        suite_id: String,
    },

    /// C-OR-5: Non-deterministic required oracle
    ///
    /// Detected when a required oracle produces different results for
    /// identical inputs. No Verified claim until resolved.
    #[serde(rename = "ORACLE_FLAKE")]
    OracleFlake {
        /// Oracle that exhibited non-determinism
        oracle_id: String,
        /// Result hash from first run
        run_1_hash: String,
        /// Result hash from second run
        run_2_hash: String,
        /// Human-readable description of the difference
        description: String,
    },

    /// C-OR-3: Run violated declared environment constraints
    ///
    /// Detected when the runtime environment doesn't match the suite's
    /// declared constraints (runtime, arch, OS, network mode, etc.).
    #[serde(rename = "ORACLE_ENV_MISMATCH")]
    OracleEnvMismatch {
        /// The constraint that was violated
        constraint: String,
        /// Expected value from suite constraints
        expected: String,
        /// Actual value from environment fingerprint
        actual: String,
    },

    /// C-EVID-6: Referenced evidence missing or unavailable
    #[serde(rename = "EVIDENCE_MISSING")]
    EvidenceMissing {
        /// Human-readable reason or missing content hash
        reason: String,
    },

    /// Manifest validation failed (schema/verdict mismatch)
    #[serde(rename = "MANIFEST_INVALID")]
    ManifestInvalid {
        /// Human-readable reason
        reason: String,
    },
}

impl IntegrityCondition {
    /// Get the condition code (matches SR-CONTRACT §2.5 naming)
    pub fn condition_code(&self) -> &'static str {
        match self {
            IntegrityCondition::OracleTamper { .. } => "ORACLE_TAMPER",
            IntegrityCondition::OracleGap { .. } => "ORACLE_GAP",
            IntegrityCondition::OracleFlake { .. } => "ORACLE_FLAKE",
            IntegrityCondition::OracleEnvMismatch { .. } => "ORACLE_ENV_MISMATCH",
            IntegrityCondition::EvidenceMissing { .. } => "EVIDENCE_MISSING",
            IntegrityCondition::ManifestInvalid { .. } => "MANIFEST_INVALID",
        }
    }

    /// Get the severity of this condition
    ///
    /// Per C-OR-7, all integrity conditions are blocking.
    pub fn severity(&self) -> Severity {
        Severity::Blocking
    }

    /// Check if this condition requires escalation
    ///
    /// Per C-OR-7, all integrity conditions MUST escalate.
    pub fn requires_escalation(&self) -> bool {
        true
    }

    /// Get the contract reference for this condition
    pub fn contract_ref(&self) -> &'static str {
        match self {
            IntegrityCondition::OracleTamper { .. } => "C-OR-2",
            IntegrityCondition::OracleGap { .. } => "C-OR-4",
            IntegrityCondition::OracleFlake { .. } => "C-OR-5",
            IntegrityCondition::OracleEnvMismatch { .. } => "C-OR-3",
            IntegrityCondition::EvidenceMissing { .. } => "C-EVID-6",
            IntegrityCondition::ManifestInvalid { .. } => "C-EVID-1",
        }
    }

    /// Get the suite ID associated with this condition (if applicable)
    pub fn suite_id(&self) -> Option<&str> {
        match self {
            IntegrityCondition::OracleTamper { suite_id, .. } => Some(suite_id),
            IntegrityCondition::OracleGap { suite_id, .. } => Some(suite_id),
            IntegrityCondition::OracleFlake { .. } => None,
            IntegrityCondition::OracleEnvMismatch { .. } => None,
            IntegrityCondition::EvidenceMissing { .. } => None,
            IntegrityCondition::ManifestInvalid { .. } => None,
        }
    }

    /// Generate a human-readable message for this condition
    pub fn message(&self) -> String {
        match self {
            IntegrityCondition::OracleTamper {
                expected_hash,
                actual_hash,
                suite_id,
            } => {
                format!(
                    "Oracle suite hash mismatch for '{}'. Expected '{}', got '{}'. \
                     Suite may have been modified since registration.",
                    suite_id, expected_hash, actual_hash
                )
            }
            IntegrityCondition::OracleGap {
                missing_oracles,
                suite_id,
            } => {
                format!(
                    "Missing required oracle results for suite '{}': {}. \
                     Cannot establish Verified claim.",
                    suite_id,
                    missing_oracles.join(", ")
                )
            }
            IntegrityCondition::OracleFlake {
                oracle_id,
                run_1_hash,
                run_2_hash,
                description,
            } => {
                format!(
                    "Oracle '{}' produced non-deterministic results. \
                     Run 1: '{}', Run 2: '{}'. {}",
                    oracle_id, run_1_hash, run_2_hash, description
                )
            }
            IntegrityCondition::OracleEnvMismatch {
                constraint,
                expected,
                actual,
            } => {
                format!(
                    "Environment constraint '{}' violated. Expected '{}', got '{}'.",
                    constraint, expected, actual
                )
            }
            IntegrityCondition::EvidenceMissing { reason } => {
                format!("Evidence missing or unavailable: {reason}")
            }
            IntegrityCondition::ManifestInvalid { reason } => {
                format!("Evidence manifest invalid: {reason}")
            }
        }
    }
}

// ============================================================================
// Severity
// ============================================================================

/// Severity level for integrity conditions
///
/// Per C-OR-7, all integrity conditions are blocking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Severity {
    /// Condition blocks progression (all integrity conditions)
    Blocking,
}

// ============================================================================
// Integrity Check Result
// ============================================================================

/// Result of an integrity check that may contain multiple conditions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IntegrityCheckResult {
    /// All detected integrity conditions
    pub conditions: Vec<IntegrityCondition>,
}

impl IntegrityCheckResult {
    /// Create a new empty result
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
        }
    }

    /// Create a result with a single condition
    pub fn with_condition(condition: IntegrityCondition) -> Self {
        Self {
            conditions: vec![condition],
        }
    }

    /// Add a condition to the result
    pub fn add(&mut self, condition: IntegrityCondition) {
        self.conditions.push(condition);
    }

    /// Check if any integrity conditions were detected
    pub fn has_violations(&self) -> bool {
        !self.conditions.is_empty()
    }

    /// Check if the result is clean (no violations)
    pub fn is_clean(&self) -> bool {
        self.conditions.is_empty()
    }

    /// Get the number of violations
    pub fn count(&self) -> usize {
        self.conditions.len()
    }

    /// Convert to Result, returning Err if any conditions detected
    pub fn into_result(self) -> Result<(), IntegrityError> {
        if self.conditions.is_empty() {
            Ok(())
        } else {
            Err(IntegrityError {
                conditions: self.conditions,
            })
        }
    }
}

// ============================================================================
// Integrity Error
// ============================================================================

/// Error type for integrity violations
///
/// Used when an integrity check fails with one or more conditions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityError {
    /// The integrity conditions that were detected
    pub conditions: Vec<IntegrityCondition>,
}

impl IntegrityError {
    /// Create an error with a single condition
    pub fn single(condition: IntegrityCondition) -> Self {
        Self {
            conditions: vec![condition],
        }
    }

    /// Get the first (or primary) condition
    pub fn primary_condition(&self) -> Option<&IntegrityCondition> {
        self.conditions.first()
    }

    /// Get all condition codes
    pub fn condition_codes(&self) -> Vec<&'static str> {
        self.conditions.iter().map(|c| c.condition_code()).collect()
    }
}

impl std::fmt::Display for IntegrityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.conditions.len() == 1 {
            write!(f, "Integrity violation: {}", self.conditions[0].message())
        } else {
            write!(
                f,
                "Multiple integrity violations ({}):\n{}",
                self.conditions.len(),
                self.conditions
                    .iter()
                    .map(|c| format!("  - {}", c.message()))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        }
    }
}

impl std::error::Error for IntegrityError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tamper_condition() {
        let condition = IntegrityCondition::OracleTamper {
            expected_hash: "sha256:abc123".to_string(),
            actual_hash: "sha256:def456".to_string(),
            suite_id: "suite:core-v1".to_string(),
        };

        assert_eq!(condition.condition_code(), "ORACLE_TAMPER");
        assert_eq!(condition.contract_ref(), "C-OR-2");
        assert_eq!(condition.severity(), Severity::Blocking);
        assert!(condition.requires_escalation());
        assert_eq!(condition.suite_id(), Some("suite:core-v1"));
        assert!(condition.message().contains("hash mismatch"));
    }

    #[test]
    fn test_gap_condition() {
        let condition = IntegrityCondition::OracleGap {
            missing_oracles: vec!["oracle:build".to_string(), "oracle:test".to_string()],
            suite_id: "suite:core-v1".to_string(),
        };

        assert_eq!(condition.condition_code(), "ORACLE_GAP");
        assert_eq!(condition.contract_ref(), "C-OR-4");
        assert!(condition.message().contains("oracle:build"));
        assert!(condition.message().contains("oracle:test"));
    }

    #[test]
    fn test_flake_condition() {
        let condition = IntegrityCondition::OracleFlake {
            oracle_id: "oracle:unit-tests".to_string(),
            run_1_hash: "sha256:aaa".to_string(),
            run_2_hash: "sha256:bbb".to_string(),
            description: "Timing-dependent test failure".to_string(),
        };

        assert_eq!(condition.condition_code(), "ORACLE_FLAKE");
        assert_eq!(condition.contract_ref(), "C-OR-5");
        assert!(condition.message().contains("non-deterministic"));
    }

    #[test]
    fn test_env_mismatch_condition() {
        let condition = IntegrityCondition::OracleEnvMismatch {
            constraint: "runtime".to_string(),
            expected: "runsc".to_string(),
            actual: "runc".to_string(),
        };

        assert_eq!(condition.condition_code(), "ORACLE_ENV_MISMATCH");
        assert_eq!(condition.contract_ref(), "C-OR-3");
        assert!(condition.message().contains("runtime"));
    }

    #[test]
    fn test_evidence_missing_condition() {
        let condition = IntegrityCondition::EvidenceMissing {
            reason: "hash:missing".to_string(),
        };

        assert_eq!(condition.condition_code(), "EVIDENCE_MISSING");
        assert_eq!(condition.contract_ref(), "C-EVID-6");
        assert!(condition.message().contains("Evidence missing"));

        let json = serde_json::to_string(&condition).unwrap();
        assert!(json.contains("EVIDENCE_MISSING"));

        let parsed: IntegrityCondition = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, condition);
    }

    #[test]
    fn test_condition_serialization() {
        let condition = IntegrityCondition::OracleTamper {
            expected_hash: "sha256:abc".to_string(),
            actual_hash: "sha256:def".to_string(),
            suite_id: "suite:test".to_string(),
        };

        let json = serde_json::to_string(&condition).unwrap();
        assert!(json.contains("ORACLE_TAMPER"));

        let parsed: IntegrityCondition = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, condition);
    }

    #[test]
    fn test_integrity_check_result() {
        let mut result = IntegrityCheckResult::new();
        assert!(result.is_clean());
        assert!(!result.has_violations());
        assert_eq!(result.count(), 0);

        result.add(IntegrityCondition::OracleGap {
            missing_oracles: vec!["oracle:build".to_string()],
            suite_id: "suite:core-v1".to_string(),
        });

        assert!(!result.is_clean());
        assert!(result.has_violations());
        assert_eq!(result.count(), 1);
    }

    #[test]
    fn test_integrity_check_result_into_result() {
        let clean = IntegrityCheckResult::new();
        assert!(clean.into_result().is_ok());

        let violated = IntegrityCheckResult::with_condition(IntegrityCondition::OracleTamper {
            expected_hash: "sha256:abc".to_string(),
            actual_hash: "sha256:def".to_string(),
            suite_id: "suite:test".to_string(),
        });
        assert!(violated.into_result().is_err());
    }

    #[test]
    fn test_integrity_error_display() {
        let error = IntegrityError::single(IntegrityCondition::OracleTamper {
            expected_hash: "sha256:abc".to_string(),
            actual_hash: "sha256:def".to_string(),
            suite_id: "suite:test".to_string(),
        });

        let message = error.to_string();
        assert!(message.contains("Integrity violation"));
        assert!(message.contains("hash mismatch"));
    }

    #[test]
    fn test_integrity_error_multiple() {
        let error = IntegrityError {
            conditions: vec![
                IntegrityCondition::OracleTamper {
                    expected_hash: "sha256:abc".to_string(),
                    actual_hash: "sha256:def".to_string(),
                    suite_id: "suite:test".to_string(),
                },
                IntegrityCondition::OracleGap {
                    missing_oracles: vec!["oracle:build".to_string()],
                    suite_id: "suite:test".to_string(),
                },
            ],
        };

        assert_eq!(error.condition_codes(), vec!["ORACLE_TAMPER", "ORACLE_GAP"]);
        let message = error.to_string();
        assert!(message.contains("Multiple integrity violations"));
    }

    #[test]
    fn test_severity_serialization() {
        let severity = Severity::Blocking;
        let json = serde_json::to_string(&severity).unwrap();
        assert_eq!(json, "\"BLOCKING\"");

        let parsed: Severity = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, Severity::Blocking);
    }
}

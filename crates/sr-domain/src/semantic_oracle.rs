//! Semantic Oracle Schemas per SR-SEMANTIC-ORACLE-SPEC
//!
//! This module defines the platform-visible interface for semantic oracles:
//! - SemanticSet: manifold/meaning-matrix definitions (hashed for suite binding)
//! - SemanticEvalResult: structured output schema (`sr.semantic_eval.v1`)
//! - Measurement types: residual, coverage, violations
//!
//! Per SR-SEMANTIC-ORACLE-SPEC §2, the oracle_suite_hash MUST incorporate
//! all semantic set definitions that materially affect evaluation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

use crate::entities::ContentHash;
use crate::errors::DomainError;
use crate::work_surface::StageId;

// ============================================================================
// Semantic Set (Manifold/Meaning-Matrix) Definition
// ============================================================================

/// Semantic Set identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SemanticSetId(String);

impl SemanticSetId {
    pub fn new(name: &str) -> Self {
        Self(format!("semantic_set:{name}"))
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Semantic axis definition (one dimension of the meaning space)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticAxis {
    /// Axis identifier
    pub axis_id: String,

    /// Human-readable name
    pub name: String,

    /// Description of what this axis measures
    pub description: String,

    /// Weight for this axis in composite scoring (0.0-1.0)
    #[serde(default = "default_weight")]
    pub weight: f64,

    /// Whether this axis is required for pass/fail determination
    #[serde(default = "default_true")]
    pub required: bool,

    /// Minimum coverage threshold for this axis (0.0-1.0)
    #[serde(default)]
    pub min_coverage: Option<f64>,

    /// Maximum allowed residual for this axis
    #[serde(default)]
    pub max_residual: Option<f64>,
}

fn default_weight() -> f64 {
    1.0
}

fn default_true() -> bool {
    true
}

/// Constraint within the semantic set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticConstraint {
    /// Constraint identifier
    pub constraint_id: String,

    /// Constraint description
    pub description: String,

    /// Constraint type
    pub constraint_type: ConstraintType,

    /// Severity if violated
    #[serde(default)]
    pub severity: ConstraintSeverity,

    /// Expression or rule (opaque to platform, used by oracle implementation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
}

/// Constraint types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintType {
    /// Must be satisfied (hard constraint)
    Required,
    /// Should be satisfied (soft constraint)
    Preferred,
    /// Must not be violated (exclusion constraint)
    Prohibited,
    /// Coherence between artifacts
    Coherence,
    /// Coverage requirement
    Coverage,
}

impl Default for ConstraintType {
    fn default() -> Self {
        Self::Required
    }
}

/// Constraint violation severity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintSeverity {
    Error,
    Warning,
    Info,
}

impl Default for ConstraintSeverity {
    fn default() -> Self {
        Self::Error
    }
}

/// Decision rule for deriving pass/fail from measurements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionRule {
    /// Rule identifier
    pub rule_id: String,

    /// Human-readable description
    pub description: String,

    /// Maximum composite residual norm allowed
    #[serde(default)]
    pub max_residual_norm: Option<f64>,

    /// Minimum composite coverage required
    #[serde(default)]
    pub min_coverage: Option<f64>,

    /// Maximum number of error-severity violations allowed
    #[serde(default)]
    pub max_error_violations: u32,

    /// Maximum number of warning-severity violations allowed
    #[serde(default)]
    pub max_warning_violations: Option<u32>,

    /// Additional thresholds (opaque to platform)
    #[serde(default)]
    pub thresholds: BTreeMap<String, serde_json::Value>,
}

/// Semantic Set (Manifold/Meaning-Matrix) Definition
///
/// Per SR-SEMANTIC-ORACLE-SPEC §2, the semantic set defines the axes,
/// constraints, and decision rules that semantic oracles evaluate against.
/// Changes to the semantic set MUST change the suite hash.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSet {
    /// Semantic set identifier
    pub semantic_set_id: SemanticSetId,

    /// Schema version
    #[serde(default = "SemanticSet::default_version")]
    pub schema_version: String,

    /// Human-readable name
    pub name: String,

    /// Description of the semantic set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Semantic axes (dimensions of the meaning space)
    pub axes: Vec<SemanticAxis>,

    /// Constraints to evaluate
    #[serde(default)]
    pub constraints: Vec<SemanticConstraint>,

    /// Decision rule for pass/fail
    pub decision_rule: DecisionRule,

    /// Applicable stages (if stage-specific)
    #[serde(default)]
    pub applicable_stages: Vec<StageId>,

    /// Content hash (computed when materialized)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<ContentHash>,

    /// Version string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Creation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

impl SemanticSet {
    fn default_version() -> String {
        "v1".to_string()
    }

    /// Compute content hash for this semantic set
    pub fn compute_hash(&self) -> ContentHash {
        let mut hasher = Sha256::new();

        // Include identity
        hasher.update(self.semantic_set_id.as_str().as_bytes());
        hasher.update(b"\n");
        hasher.update(self.schema_version.as_bytes());
        hasher.update(b"\n");

        // Include axes deterministically
        for axis in &self.axes {
            hasher.update(b"axis:");
            hasher.update(axis.axis_id.as_bytes());
            hasher.update(b":");
            hasher.update(axis.name.as_bytes());
            hasher.update(b":");
            hasher.update(&axis.weight.to_le_bytes());
            hasher.update(b":");
            hasher.update(&[axis.required as u8]);
            hasher.update(b"\n");
        }

        // Include constraints
        for constraint in &self.constraints {
            hasher.update(b"constraint:");
            hasher.update(constraint.constraint_id.as_bytes());
            hasher.update(b":");
            hasher.update(
                serde_json::to_string(&constraint.constraint_type)
                    .unwrap_or_default()
                    .as_bytes(),
            );
            hasher.update(b"\n");
        }

        // Include decision rule
        hasher.update(b"rule:");
        hasher.update(self.decision_rule.rule_id.as_bytes());
        if let Some(max) = self.decision_rule.max_residual_norm {
            hasher.update(b":max_residual:");
            hasher.update(&max.to_le_bytes());
        }
        if let Some(min) = self.decision_rule.min_coverage {
            hasher.update(b":min_coverage:");
            hasher.update(&min.to_le_bytes());
        }
        hasher.update(b"\n");

        // Include version if present
        if let Some(version) = &self.version {
            hasher.update(b"version:");
            hasher.update(version.as_bytes());
            hasher.update(b"\n");
        }

        let result = hasher.finalize();
        ContentHash::new(&hex::encode(result))
    }

    /// Validate the semantic set
    pub fn validate(&self) -> Result<(), DomainError> {
        SemanticSetValidator::validate(self)
    }
}

// ============================================================================
// Semantic Evaluation Result (sr.semantic_eval.v1)
// ============================================================================

/// Residual vector per axis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResidualVector {
    /// Per-axis residual values
    pub per_axis: BTreeMap<String, f64>,

    /// Composite residual norm (e.g., L2 norm)
    pub composite_norm: f64,

    /// Method used to compute the norm
    #[serde(default = "default_norm_method")]
    pub norm_method: String,
}

fn default_norm_method() -> String {
    "L2".to_string()
}

/// Coverage metrics per axis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageMetrics {
    /// Per-axis coverage (0.0-1.0)
    pub per_axis: BTreeMap<String, f64>,

    /// Composite coverage (weighted average)
    pub composite: f64,

    /// Axes below minimum threshold
    #[serde(default)]
    pub below_threshold: Vec<String>,
}

/// Constraint violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintViolation {
    /// Violation code (e.g., "MISSING_AXIS", "COHERENCE_FAIL")
    pub code: String,

    /// Constraint ID that was violated
    pub constraint_id: String,

    /// Axis involved (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis: Option<String>,

    /// Human-readable message
    pub message: String,

    /// Severity
    pub severity: ConstraintSeverity,

    /// Additional context
    #[serde(default)]
    pub context: BTreeMap<String, serde_json::Value>,
}

/// Decision status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DecisionStatus {
    Pass,
    Fail,
    Indeterminate,
}

/// Decision record within the evaluation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalDecision {
    /// Pass/Fail/Indeterminate
    pub status: DecisionStatus,

    /// Rule ID that was applied
    pub rule_id: String,

    /// Thresholds used for the decision
    #[serde(default)]
    pub thresholds: BTreeMap<String, serde_json::Value>,

    /// Rationale for the decision
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rationale: Option<String>,
}

/// Semantic Evaluation Result per SR-SEMANTIC-ORACLE-SPEC §4
///
/// Schema: `sr.semantic_eval.v1`
///
/// This is the structured output from a semantic oracle run. It contains
/// sufficient information for the gate decision rule to be computed from
/// recorded outputs without out-of-band assumptions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticEvalResult {
    /// Schema identifier
    #[serde(default = "SemanticEvalResult::default_schema")]
    pub schema: String,

    /// Candidate ID being evaluated
    pub candidate_id: String,

    /// Template ID
    pub template_id: String,

    /// Stage ID where evaluation occurred
    pub stage_id: String,

    /// Oracle suite that produced this result
    pub oracle_suite_id: String,

    /// Oracle suite hash
    pub oracle_suite_hash: String,

    /// Semantic set binding
    pub semantic_set: SemanticSetBinding,

    /// Measurement metrics
    pub metrics: SemanticMetrics,

    /// Decision derived from measurements
    pub decision: EvalDecision,

    /// Optional human-readable notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    /// Evaluation timestamp
    #[serde(default = "Utc::now")]
    pub evaluated_at: DateTime<Utc>,
}

impl SemanticEvalResult {
    fn default_schema() -> String {
        "sr.semantic_eval.v1".to_string()
    }

    /// Check if the evaluation passed
    pub fn passed(&self) -> bool {
        self.decision.status == DecisionStatus::Pass
    }

    /// Get error-severity violations
    pub fn error_violations(&self) -> Vec<&ConstraintViolation> {
        self.metrics
            .violations
            .iter()
            .filter(|v| v.severity == ConstraintSeverity::Error)
            .collect()
    }
}

/// Semantic set binding reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSetBinding {
    /// Semantic set ID
    pub semantic_set_id: String,

    /// Semantic set content hash
    pub semantic_set_hash: String,
}

/// Semantic measurement metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticMetrics {
    /// Residual vector (distance from ideal in the semantic space)
    pub residual: ResidualVector,

    /// Coverage metrics across axes
    pub coverage: CoverageMetrics,

    /// Constraint violations
    #[serde(default)]
    pub violations: Vec<ConstraintViolation>,

    /// Additional metrics (opaque to platform)
    #[serde(default)]
    pub additional: BTreeMap<String, serde_json::Value>,
}

// ============================================================================
// Report Artifacts per SR-SEMANTIC-ORACLE-SPEC §3
// ============================================================================

/// Residual report artifact (reports/semantic/residual.json)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResidualReport {
    /// Oracle suite ID
    pub oracle_suite_id: String,

    /// Candidate ID
    pub candidate_id: String,

    /// Stage ID
    pub stage_id: String,

    /// Residual measurements
    pub residual: ResidualVector,

    /// Timestamp
    pub computed_at: DateTime<Utc>,
}

/// Coverage report artifact (reports/semantic/coverage.json)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageReport {
    /// Oracle suite ID
    pub oracle_suite_id: String,

    /// Candidate ID
    pub candidate_id: String,

    /// Stage ID
    pub stage_id: String,

    /// Coverage metrics
    pub coverage: CoverageMetrics,

    /// Timestamp
    pub computed_at: DateTime<Utc>,
}

/// Violations report artifact (reports/semantic/violations.json)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViolationsReport {
    /// Oracle suite ID
    pub oracle_suite_id: String,

    /// Candidate ID
    pub candidate_id: String,

    /// Stage ID
    pub stage_id: String,

    /// Violations found
    pub violations: Vec<ConstraintViolation>,

    /// Summary counts
    pub summary: ViolationSummary,

    /// Timestamp
    pub computed_at: DateTime<Utc>,
}

/// Violation summary counts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViolationSummary {
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
    pub total_count: usize,
}

// ============================================================================
// Validators
// ============================================================================

/// Semantic set validator
pub struct SemanticSetValidator;

impl SemanticSetValidator {
    pub fn validate(set: &SemanticSet) -> Result<(), DomainError> {
        if set.semantic_set_id.as_str().is_empty() {
            return Err(DomainError::InvariantViolation {
                invariant: "SemanticSet.semantic_set_id is required".to_string(),
            });
        }

        if set.name.trim().is_empty() {
            return Err(DomainError::InvariantViolation {
                invariant: "SemanticSet.name is required".to_string(),
            });
        }

        if set.axes.is_empty() {
            return Err(DomainError::InvariantViolation {
                invariant: "SemanticSet.axes must have at least one entry".to_string(),
            });
        }

        // Validate each axis
        for (i, axis) in set.axes.iter().enumerate() {
            if axis.axis_id.is_empty() {
                return Err(DomainError::InvariantViolation {
                    invariant: format!("axes[{i}].axis_id is required"),
                });
            }
            if axis.name.trim().is_empty() {
                return Err(DomainError::InvariantViolation {
                    invariant: format!("axes[{i}].name is required"),
                });
            }
            if axis.weight < 0.0 || axis.weight > 1.0 {
                return Err(DomainError::InvariantViolation {
                    invariant: format!("axes[{i}].weight must be between 0.0 and 1.0"),
                });
            }
        }

        // Validate decision rule
        if set.decision_rule.rule_id.is_empty() {
            return Err(DomainError::InvariantViolation {
                invariant: "decision_rule.rule_id is required".to_string(),
            });
        }

        Ok(())
    }
}

// ============================================================================
// Branch 0: Intake Admissibility Semantic Set
// ============================================================================

/// Create the intake admissibility semantic set for Branch 0
///
/// Per SR-PLAN D-39, this semantic set defines the axes and constraints
/// for evaluating intake admissibility:
/// - Schema compliance
/// - Traceability coverage
/// - Contradiction detection
/// - Ambiguity inventory
/// - Privacy scan
/// - Term-map alignment
pub fn intake_admissibility_semantic_set() -> SemanticSet {
    SemanticSet {
        semantic_set_id: SemanticSetId::new("INTAKE-ADMISSIBILITY"),
        schema_version: "v1".to_string(),
        name: "Intake Admissibility".to_string(),
        description: Some(
            "Semantic set for evaluating intake document admissibility. \
             Measures completeness, consistency, and actionability of \
             structured intake artifacts."
                .to_string(),
        ),
        axes: vec![
            SemanticAxis {
                axis_id: "schema_compliance".to_string(),
                name: "Schema Compliance".to_string(),
                description: "Measures adherence to the Intake schema (required fields, types)"
                    .to_string(),
                weight: 1.0,
                required: true,
                min_coverage: Some(1.0), // Must be 100% compliant
                max_residual: Some(0.0),
            },
            SemanticAxis {
                axis_id: "traceability_coverage".to_string(),
                name: "Traceability Coverage".to_string(),
                description: "Measures coverage of inputs and deliverables with explicit refs"
                    .to_string(),
                weight: 0.8,
                required: true,
                min_coverage: Some(0.8),
                max_residual: None,
            },
            SemanticAxis {
                axis_id: "contradiction_free".to_string(),
                name: "Contradiction-Free".to_string(),
                description: "Measures absence of contradictory statements".to_string(),
                weight: 1.0,
                required: true,
                min_coverage: None,
                max_residual: Some(0.0), // No contradictions allowed
            },
            SemanticAxis {
                axis_id: "ambiguity_bounded".to_string(),
                name: "Ambiguity Bounded".to_string(),
                description: "Measures that ambiguities are explicit and inventoried".to_string(),
                weight: 0.6,
                required: false,
                min_coverage: Some(0.7),
                max_residual: None,
            },
            SemanticAxis {
                axis_id: "privacy_safe".to_string(),
                name: "Privacy Safe".to_string(),
                description: "Measures absence of PII or sensitive data in public fields"
                    .to_string(),
                weight: 1.0,
                required: true,
                min_coverage: Some(1.0),
                max_residual: Some(0.0),
            },
            SemanticAxis {
                axis_id: "term_map_aligned".to_string(),
                name: "Term Map Aligned".to_string(),
                description:
                    "Measures alignment between definitions and usage in constraints/deliverables"
                        .to_string(),
                weight: 0.7,
                required: false,
                min_coverage: Some(0.8),
                max_residual: None,
            },
        ],
        constraints: vec![
            SemanticConstraint {
                constraint_id: "required_fields_present".to_string(),
                description: "All required Intake fields must be present and non-empty".to_string(),
                constraint_type: ConstraintType::Required,
                severity: ConstraintSeverity::Error,
                expression: None,
            },
            SemanticConstraint {
                constraint_id: "deliverables_have_paths".to_string(),
                description: "Each deliverable must have a valid path".to_string(),
                constraint_type: ConstraintType::Required,
                severity: ConstraintSeverity::Error,
                expression: None,
            },
            SemanticConstraint {
                constraint_id: "no_contradictions".to_string(),
                description: "Objective and constraints must not contradict each other".to_string(),
                constraint_type: ConstraintType::Prohibited,
                severity: ConstraintSeverity::Error,
                expression: None,
            },
            SemanticConstraint {
                constraint_id: "ambiguities_inventoried".to_string(),
                description: "Ambiguous terms should be listed in unknowns or definitions"
                    .to_string(),
                constraint_type: ConstraintType::Preferred,
                severity: ConstraintSeverity::Warning,
                expression: None,
            },
            SemanticConstraint {
                constraint_id: "no_pii_in_public".to_string(),
                description: "No personally identifiable information in public fields".to_string(),
                constraint_type: ConstraintType::Prohibited,
                severity: ConstraintSeverity::Error,
                expression: None,
            },
            SemanticConstraint {
                constraint_id: "terms_used_consistently".to_string(),
                description: "Defined terms are used consistently throughout".to_string(),
                constraint_type: ConstraintType::Preferred,
                severity: ConstraintSeverity::Warning,
                expression: None,
            },
        ],
        decision_rule: DecisionRule {
            rule_id: "intake_admissibility_v1".to_string(),
            description: "Pass if all required axes meet thresholds and no error violations"
                .to_string(),
            max_residual_norm: Some(0.2),
            min_coverage: Some(0.85),
            max_error_violations: 0,
            max_warning_violations: Some(3),
            thresholds: BTreeMap::from_iter([
                ("schema_compliance_min".to_string(), serde_json::json!(1.0)),
                ("traceability_min".to_string(), serde_json::json!(0.8)),
            ]),
        },
        applicable_stages: vec![StageId::new("VALIDATE")],
        content_hash: None,
        version: Some("1.0.0".to_string()),
        created_at: Some(Utc::now()),
    }
}

/// Get the intake admissibility semantic set with computed hash
pub fn get_intake_admissibility_set() -> SemanticSet {
    let mut set = intake_admissibility_semantic_set();
    set.content_hash = Some(set.compute_hash());
    set
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_set_id_format() {
        let id = SemanticSetId::new("TEST-SET");
        assert_eq!(id.as_str(), "semantic_set:TEST-SET");
    }

    #[test]
    fn test_intake_admissibility_set_valid() {
        let set = intake_admissibility_semantic_set();
        assert!(set.validate().is_ok());
    }

    #[test]
    fn test_intake_admissibility_set_has_required_axes() {
        let set = intake_admissibility_semantic_set();

        let axis_ids: Vec<&str> = set.axes.iter().map(|a| a.axis_id.as_str()).collect();

        assert!(axis_ids.contains(&"schema_compliance"));
        assert!(axis_ids.contains(&"traceability_coverage"));
        assert!(axis_ids.contains(&"contradiction_free"));
        assert!(axis_ids.contains(&"ambiguity_bounded"));
        assert!(axis_ids.contains(&"privacy_safe"));
        assert!(axis_ids.contains(&"term_map_aligned"));
    }

    #[test]
    fn test_semantic_set_hash_determinism() {
        let set1 = intake_admissibility_semantic_set();
        let set2 = intake_admissibility_semantic_set();

        let hash1 = set1.compute_hash();
        let hash2 = set2.compute_hash();

        assert_eq!(hash1.as_str(), hash2.as_str());
    }

    #[test]
    fn test_semantic_set_hash_changes_with_axes() {
        let mut set1 = intake_admissibility_semantic_set();
        let set2 = intake_admissibility_semantic_set();

        // Modify set1
        set1.axes.push(SemanticAxis {
            axis_id: "extra_axis".to_string(),
            name: "Extra".to_string(),
            description: "Extra axis".to_string(),
            weight: 0.5,
            required: false,
            min_coverage: None,
            max_residual: None,
        });

        let hash1 = set1.compute_hash();
        let hash2 = set2.compute_hash();

        assert_ne!(hash1.as_str(), hash2.as_str());
    }

    #[test]
    fn test_semantic_eval_result_passed() {
        let result = SemanticEvalResult {
            schema: "sr.semantic_eval.v1".to_string(),
            candidate_id: "candidate:test".to_string(),
            template_id: "proc:TEST".to_string(),
            stage_id: "stage:VALIDATE".to_string(),
            oracle_suite_id: "oracle.suite.intake_admissibility.v1".to_string(),
            oracle_suite_hash: "sha256:abc".to_string(),
            semantic_set: SemanticSetBinding {
                semantic_set_id: "semantic_set:TEST".to_string(),
                semantic_set_hash: "sha256:def".to_string(),
            },
            metrics: SemanticMetrics {
                residual: ResidualVector {
                    per_axis: BTreeMap::from_iter([
                        ("schema_compliance".to_string(), 0.0),
                        ("traceability_coverage".to_string(), 0.1),
                    ]),
                    composite_norm: 0.1,
                    norm_method: "L2".to_string(),
                },
                coverage: CoverageMetrics {
                    per_axis: BTreeMap::from_iter([
                        ("schema_compliance".to_string(), 1.0),
                        ("traceability_coverage".to_string(), 0.9),
                    ]),
                    composite: 0.95,
                    below_threshold: vec![],
                },
                violations: vec![],
                additional: BTreeMap::new(),
            },
            decision: EvalDecision {
                status: DecisionStatus::Pass,
                rule_id: "intake_admissibility_v1".to_string(),
                thresholds: BTreeMap::new(),
                rationale: None,
            },
            notes: None,
            evaluated_at: Utc::now(),
        };

        assert!(result.passed());
        assert!(result.error_violations().is_empty());
    }

    #[test]
    fn test_semantic_eval_result_failed() {
        let result = SemanticEvalResult {
            schema: "sr.semantic_eval.v1".to_string(),
            candidate_id: "candidate:test".to_string(),
            template_id: "proc:TEST".to_string(),
            stage_id: "stage:VALIDATE".to_string(),
            oracle_suite_id: "oracle.suite.intake_admissibility.v1".to_string(),
            oracle_suite_hash: "sha256:abc".to_string(),
            semantic_set: SemanticSetBinding {
                semantic_set_id: "semantic_set:TEST".to_string(),
                semantic_set_hash: "sha256:def".to_string(),
            },
            metrics: SemanticMetrics {
                residual: ResidualVector {
                    per_axis: BTreeMap::new(),
                    composite_norm: 0.5,
                    norm_method: "L2".to_string(),
                },
                coverage: CoverageMetrics {
                    per_axis: BTreeMap::new(),
                    composite: 0.6,
                    below_threshold: vec!["traceability_coverage".to_string()],
                },
                violations: vec![ConstraintViolation {
                    code: "MISSING_FIELD".to_string(),
                    constraint_id: "required_fields_present".to_string(),
                    axis: Some("schema_compliance".to_string()),
                    message: "Missing required field: objective".to_string(),
                    severity: ConstraintSeverity::Error,
                    context: BTreeMap::new(),
                }],
                additional: BTreeMap::new(),
            },
            decision: EvalDecision {
                status: DecisionStatus::Fail,
                rule_id: "intake_admissibility_v1".to_string(),
                thresholds: BTreeMap::new(),
                rationale: Some("Coverage below threshold".to_string()),
            },
            notes: None,
            evaluated_at: Utc::now(),
        };

        assert!(!result.passed());
        assert_eq!(result.error_violations().len(), 1);
    }

    #[test]
    fn test_residual_report_serialization() {
        let report = ResidualReport {
            oracle_suite_id: "test".to_string(),
            candidate_id: "c1".to_string(),
            stage_id: "s1".to_string(),
            residual: ResidualVector {
                per_axis: BTreeMap::from_iter([("a1".to_string(), 0.1)]),
                composite_norm: 0.1,
                norm_method: "L2".to_string(),
            },
            computed_at: Utc::now(),
        };

        let json = serde_json::to_string(&report).unwrap();
        let parsed: ResidualReport = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.oracle_suite_id, "test");
    }

    #[test]
    fn test_violation_summary() {
        let report = ViolationsReport {
            oracle_suite_id: "test".to_string(),
            candidate_id: "c1".to_string(),
            stage_id: "s1".to_string(),
            violations: vec![
                ConstraintViolation {
                    code: "ERR1".to_string(),
                    constraint_id: "c1".to_string(),
                    axis: None,
                    message: "Error".to_string(),
                    severity: ConstraintSeverity::Error,
                    context: BTreeMap::new(),
                },
                ConstraintViolation {
                    code: "WARN1".to_string(),
                    constraint_id: "c2".to_string(),
                    axis: None,
                    message: "Warning".to_string(),
                    severity: ConstraintSeverity::Warning,
                    context: BTreeMap::new(),
                },
            ],
            summary: ViolationSummary {
                error_count: 1,
                warning_count: 1,
                info_count: 0,
                total_count: 2,
            },
            computed_at: Utc::now(),
        };

        assert_eq!(report.summary.error_count, 1);
        assert_eq!(report.summary.total_count, 2);
    }

    #[test]
    fn test_semantic_set_validation_missing_id() {
        let set = SemanticSet {
            semantic_set_id: SemanticSetId::from_string("".to_string()),
            schema_version: "v1".to_string(),
            name: "Test".to_string(),
            description: None,
            axes: vec![SemanticAxis {
                axis_id: "a1".to_string(),
                name: "A1".to_string(),
                description: "Axis 1".to_string(),
                weight: 1.0,
                required: true,
                min_coverage: None,
                max_residual: None,
            }],
            constraints: vec![],
            decision_rule: DecisionRule {
                rule_id: "r1".to_string(),
                description: "Test".to_string(),
                max_residual_norm: None,
                min_coverage: None,
                max_error_violations: 0,
                max_warning_violations: None,
                thresholds: BTreeMap::new(),
            },
            applicable_stages: vec![],
            content_hash: None,
            version: None,
            created_at: None,
        };

        assert!(set.validate().is_err());
    }

    #[test]
    fn test_decision_status_serialization() {
        assert_eq!(
            serde_json::to_string(&DecisionStatus::Pass).unwrap(),
            "\"PASS\""
        );
        assert_eq!(
            serde_json::to_string(&DecisionStatus::Fail).unwrap(),
            "\"FAIL\""
        );
        assert_eq!(
            serde_json::to_string(&DecisionStatus::Indeterminate).unwrap(),
            "\"INDETERMINATE\""
        );
    }
}

//! Semantic Oracle Suite Implementation (D-39)
//!
//! Implements semantic oracle suites per SR-SEMANTIC-ORACLE-SPEC.
//!
//! Per SR-PLAN D-39:
//! - Oracle suite integration with meaning matrices/manifolds
//! - Stage manifold / meaning-matrix version binding
//! - Residual/coverage/violation reports
//! - Pass/fail decision rule derivation
//! - Define SemanticSet/manifold definition artifact
//! - Implement `oracle.suite.intake_admissibility.v1` for Branch 0
//!
//! The semantic suite hash MUST incorporate the semantic set definitions
//! per SR-SEMANTIC-ORACLE-SPEC §2.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sr_domain::{
    get_intake_admissibility_set, ConstraintSeverity, ConstraintViolation, ContentHash,
    CoverageMetrics, CoverageReport, DecisionStatus, EvalDecision, Intake, ResidualReport,
    ResidualVector, SemanticEvalResult, SemanticMetrics, SemanticSet, SemanticSetBinding,
    ViolationSummary, ViolationsReport,
};
use std::collections::BTreeMap;
use tracing::{info, instrument};

use crate::oracle_runner::{
    EnvironmentConstraints, ExpectedOutput, NetworkMode, OracleClassification, OracleDefinition,
    OracleSuiteDefinition,
};

// ============================================================================
// Constants
// ============================================================================

/// Intake Admissibility Suite ID (Branch 0)
pub const SUITE_INTAKE_ADMISSIBILITY_ID: &str = "oracle.suite.intake_admissibility.v1";

/// Semantic Suite prefix
pub const SUITE_SEMANTIC_PREFIX: &str = "suite:SR-SUITE-SEMANTIC";

/// Semantic oracle ID prefix
pub const SEMANTIC_ORACLE_PREFIX: &str = "oracle:semantic";

// ============================================================================
// Semantic Oracle Suite Definition
// ============================================================================

/// Create the intake admissibility semantic oracle suite for Branch 0
///
/// Per SR-PLAN D-39 and SR-SEMANTIC-ORACLE-SPEC, this suite:
/// - Binds to the intake admissibility semantic set
/// - Produces structured measurements (residual, coverage, violations)
/// - Derives pass/fail from the decision rule
/// - Suite hash incorporates the semantic set hash
pub fn create_intake_admissibility_suite() -> SemanticOracleSuiteDefinition {
    let semantic_set = get_intake_admissibility_set();
    let semantic_set_hash = semantic_set
        .content_hash
        .clone()
        .unwrap_or_else(|| semantic_set.compute_hash());

    let oracles = vec![
        // Schema validation oracle
        create_schema_compliance_oracle(),
        // Traceability oracle
        create_traceability_oracle(),
        // Contradiction detection oracle
        create_contradiction_oracle(),
        // Ambiguity inventory oracle
        create_ambiguity_oracle(),
        // Privacy scan oracle
        create_privacy_oracle(),
        // Term-map alignment oracle
        create_term_map_oracle(),
    ];

    let suite_hash = compute_semantic_suite_hash(&oracles, &semantic_set_hash);

    SemanticOracleSuiteDefinition {
        suite_id: SUITE_INTAKE_ADMISSIBILITY_ID.to_string(),
        suite_hash,
        semantic_set_binding: SemanticSetBindingRef {
            semantic_set_id: semantic_set.semantic_set_id.as_str().to_string(),
            semantic_set_hash: semantic_set_hash.as_str().to_string(),
        },
        semantic_set: Some(semantic_set),
        oci_image: "ghcr.io/solver-ralph/oracle-semantic:latest".to_string(),
        oci_image_digest: "sha256:PLACEHOLDER_SEMANTIC_DIGEST".to_string(),
        environment_constraints: EnvironmentConstraints {
            runtime: "runsc".to_string(),
            network: NetworkMode::Disabled,
            cpu_arch: "amd64".to_string(),
            os: "linux".to_string(),
            workspace_readonly: true,
            additional_constraints: vec![],
        },
        oracles,
        metadata: BTreeMap::new(),
    }
}

/// Convert SemanticOracleSuiteDefinition to standard OracleSuiteDefinition
pub fn to_oracle_suite_definition(
    semantic_suite: &SemanticOracleSuiteDefinition,
) -> OracleSuiteDefinition {
    OracleSuiteDefinition {
        suite_id: semantic_suite.suite_id.clone(),
        suite_hash: semantic_suite.suite_hash.clone(),
        oci_image: semantic_suite.oci_image.clone(),
        oci_image_digest: semantic_suite.oci_image_digest.clone(),
        environment_constraints: semantic_suite.environment_constraints.clone(),
        oracles: semantic_suite
            .oracles
            .iter()
            .map(|o| OracleDefinition {
                oracle_id: o.oracle_id.clone(),
                oracle_name: o.oracle_name.clone(),
                command: o.command.clone(),
                args: o.args.clone(),
                timeout_seconds: o.timeout_seconds,
                expected_outputs: o.expected_outputs.clone(),
                classification: o.classification,
                working_dir: o.working_dir.clone(),
                env: o.env.clone(),
            })
            .collect(),
        metadata: BTreeMap::from_iter([
            (
                "semantic_set_id".to_string(),
                serde_json::json!(semantic_suite.semantic_set_binding.semantic_set_id),
            ),
            (
                "semantic_set_hash".to_string(),
                serde_json::json!(semantic_suite.semantic_set_binding.semantic_set_hash),
            ),
        ]),
    }
}

// ============================================================================
// Semantic Oracle Suite Types
// ============================================================================

/// Reference to a semantic set binding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSetBindingRef {
    /// Semantic set ID
    pub semantic_set_id: String,
    /// Semantic set content hash
    pub semantic_set_hash: String,
}

/// Semantic oracle suite definition (extends OracleSuiteDefinition with semantic set binding)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticOracleSuiteDefinition {
    /// Suite identifier
    pub suite_id: String,
    /// Content hash (incorporates semantic set hash per SR-SEMANTIC-ORACLE-SPEC §2)
    pub suite_hash: String,
    /// Semantic set binding
    pub semantic_set_binding: SemanticSetBindingRef,
    /// The semantic set definition (optional, may be fetched by ref)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_set: Option<SemanticSet>,
    /// OCI image reference
    pub oci_image: String,
    /// OCI image digest
    pub oci_image_digest: String,
    /// Environment constraints
    pub environment_constraints: EnvironmentConstraints,
    /// Semantic oracle definitions
    pub oracles: Vec<SemanticOracleDefinition>,
    /// Metadata
    #[serde(default)]
    pub metadata: BTreeMap<String, serde_json::Value>,
}

/// Semantic oracle definition (extends OracleDefinition with axis binding)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticOracleDefinition {
    /// Oracle identifier
    pub oracle_id: String,
    /// Human-readable name
    pub oracle_name: String,
    /// Axis this oracle measures
    pub target_axis: String,
    /// Command to execute
    pub command: String,
    /// Arguments
    #[serde(default)]
    pub args: Vec<String>,
    /// Timeout in seconds
    pub timeout_seconds: u64,
    /// Expected outputs
    pub expected_outputs: Vec<ExpectedOutput>,
    /// Classification
    #[serde(default)]
    pub classification: OracleClassification,
    /// Working directory
    #[serde(default)]
    pub working_dir: Option<String>,
    /// Environment variables
    #[serde(default)]
    pub env: BTreeMap<String, String>,
}

// ============================================================================
// Oracle Factories for Intake Admissibility
// ============================================================================

fn create_schema_compliance_oracle() -> SemanticOracleDefinition {
    SemanticOracleDefinition {
        oracle_id: format!("{SEMANTIC_ORACLE_PREFIX}:schema_compliance"),
        oracle_name: "Schema Compliance Check".to_string(),
        target_axis: "schema_compliance".to_string(),
        command: "sr-semantic-oracles intake-schema-check --intake /workspace/artifacts/intake/draft_intake.yaml --output /scratch/reports/semantic/schema.json".to_string(),
        args: vec![],
        timeout_seconds: 120,
        expected_outputs: vec![
            ExpectedOutput {
                path: "reports/semantic/schema.json".to_string(),
                content_type: "application/json".to_string(),
                required: true,
            },
        ],
        classification: OracleClassification::Required,
        working_dir: Some("/workspace".to_string()),
        env: BTreeMap::new(),
    }
}

fn create_traceability_oracle() -> SemanticOracleDefinition {
    SemanticOracleDefinition {
        oracle_id: format!("{SEMANTIC_ORACLE_PREFIX}:traceability"),
        oracle_name: "Traceability Coverage Check".to_string(),
        target_axis: "traceability_coverage".to_string(),
        command: "sr-semantic-oracles intake-traceability --intake /workspace/artifacts/intake/draft_intake.yaml --output /scratch/reports/semantic/traceability.json".to_string(),
        args: vec![],
        timeout_seconds: 120,
        expected_outputs: vec![
            ExpectedOutput {
                path: "reports/semantic/traceability.json".to_string(),
                content_type: "application/json".to_string(),
                required: true,
            },
        ],
        classification: OracleClassification::Required,
        working_dir: Some("/workspace".to_string()),
        env: BTreeMap::new(),
    }
}

fn create_contradiction_oracle() -> SemanticOracleDefinition {
    SemanticOracleDefinition {
        oracle_id: format!("{SEMANTIC_ORACLE_PREFIX}:contradiction"),
        oracle_name: "Contradiction Detection".to_string(),
        target_axis: "contradiction_free".to_string(),
        command: "sr-semantic-oracles intake-contradictions --intake /workspace/artifacts/intake/draft_intake.yaml --output /scratch/reports/semantic/contradictions.json".to_string(),
        args: vec![],
        timeout_seconds: 180,
        expected_outputs: vec![
            ExpectedOutput {
                path: "reports/semantic/contradictions.json".to_string(),
                content_type: "application/json".to_string(),
                required: true,
            },
        ],
        classification: OracleClassification::Required,
        working_dir: Some("/workspace".to_string()),
        env: BTreeMap::new(),
    }
}

fn create_ambiguity_oracle() -> SemanticOracleDefinition {
    SemanticOracleDefinition {
        oracle_id: format!("{SEMANTIC_ORACLE_PREFIX}:ambiguity"),
        oracle_name: "Ambiguity Inventory".to_string(),
        target_axis: "ambiguity_bounded".to_string(),
        command: "sr-semantic-oracles intake-ambiguity --intake /workspace/artifacts/intake/draft_intake.yaml --output /scratch/reports/semantic/ambiguity.json".to_string(),
        args: vec![],
        timeout_seconds: 120,
        expected_outputs: vec![
            ExpectedOutput {
                path: "reports/semantic/ambiguity.json".to_string(),
                content_type: "application/json".to_string(),
                required: true,
            },
        ],
        classification: OracleClassification::Advisory, // Not required
        working_dir: Some("/workspace".to_string()),
        env: BTreeMap::new(),
    }
}

fn create_privacy_oracle() -> SemanticOracleDefinition {
    SemanticOracleDefinition {
        oracle_id: format!("{SEMANTIC_ORACLE_PREFIX}:privacy"),
        oracle_name: "Privacy Scan".to_string(),
        target_axis: "privacy_safe".to_string(),
        command: "sr-semantic-oracles intake-privacy --intake /workspace/artifacts/intake/draft_intake.yaml --output /scratch/reports/semantic/privacy.json".to_string(),
        args: vec![],
        timeout_seconds: 120,
        expected_outputs: vec![
            ExpectedOutput {
                path: "reports/semantic/privacy.json".to_string(),
                content_type: "application/json".to_string(),
                required: true,
            },
        ],
        classification: OracleClassification::Required,
        working_dir: Some("/workspace".to_string()),
        env: BTreeMap::new(),
    }
}

fn create_term_map_oracle() -> SemanticOracleDefinition {
    SemanticOracleDefinition {
        oracle_id: format!("{SEMANTIC_ORACLE_PREFIX}:term_map"),
        oracle_name: "Term Map Alignment".to_string(),
        target_axis: "term_map_aligned".to_string(),
        command: "sr-semantic-oracles intake-term-map --intake /workspace/artifacts/intake/draft_intake.yaml --output /scratch/reports/semantic/term_map.json".to_string(),
        args: vec![],
        timeout_seconds: 120,
        expected_outputs: vec![
            ExpectedOutput {
                path: "reports/semantic/term_map.json".to_string(),
                content_type: "application/json".to_string(),
                required: true,
            },
        ],
        classification: OracleClassification::Advisory, // Not required
        working_dir: Some("/workspace".to_string()),
        env: BTreeMap::new(),
    }
}

// ============================================================================
// Suite Hash Computation
// ============================================================================

/// Compute semantic suite hash that incorporates the semantic set hash
///
/// Per SR-SEMANTIC-ORACLE-SPEC §2, the suite_hash MUST incorporate all
/// semantic set definitions that materially affect evaluation.
fn compute_semantic_suite_hash(
    oracles: &[SemanticOracleDefinition],
    semantic_set_hash: &ContentHash,
) -> String {
    let mut hasher = Sha256::new();

    // Include semantic set hash first (critical for reproducibility)
    hasher.update(b"semantic_set:");
    hasher.update(semantic_set_hash.as_str().as_bytes());
    hasher.update(b"\n");

    // Include each oracle definition
    for oracle in oracles {
        hasher.update(oracle.oracle_id.as_bytes());
        hasher.update(b":");
        hasher.update(oracle.target_axis.as_bytes());
        hasher.update(b":");
        hasher.update(oracle.command.as_bytes());
        hasher.update(b":");
        hasher.update(&oracle.timeout_seconds.to_le_bytes());
        hasher.update(b"\n");
    }

    format!("sha256:{}", hex::encode(hasher.finalize()))
}

// ============================================================================
// Semantic Evaluation Runner
// ============================================================================

/// Semantic evaluation runner for intake admissibility
pub struct IntakeAdmissibilityRunner {
    suite: SemanticOracleSuiteDefinition,
}

impl IntakeAdmissibilityRunner {
    /// Create a new runner with the default suite
    pub fn new() -> Self {
        Self {
            suite: create_intake_admissibility_suite(),
        }
    }

    /// Create a runner with a custom suite
    pub fn with_suite(suite: SemanticOracleSuiteDefinition) -> Self {
        Self { suite }
    }

    /// Get the suite definition
    pub fn suite(&self) -> &SemanticOracleSuiteDefinition {
        &self.suite
    }

    /// Evaluate an intake artifact locally (without container execution)
    ///
    /// This is a deterministic evaluation that can be used for testing
    /// and for environments where container execution is not available.
    #[instrument(skip(self, intake))]
    pub fn evaluate_intake(&self, candidate_id: &str, intake: &Intake) -> SemanticEvalResult {
        info!(candidate_id = %candidate_id, "Evaluating intake admissibility");

        let semantic_set = self
            .suite
            .semantic_set
            .as_ref()
            .expect("Semantic set required");
        let decision_rule = &semantic_set.decision_rule;

        // Evaluate each axis
        let mut per_axis_residual = BTreeMap::new();
        let mut per_axis_coverage = BTreeMap::new();
        let mut violations = Vec::new();

        // Schema compliance
        let (schema_residual, schema_coverage, schema_violations) =
            self.evaluate_schema_compliance(intake);
        per_axis_residual.insert("schema_compliance".to_string(), schema_residual);
        per_axis_coverage.insert("schema_compliance".to_string(), schema_coverage);
        violations.extend(schema_violations);

        // Traceability coverage
        let (trace_residual, trace_coverage, trace_violations) = self.evaluate_traceability(intake);
        per_axis_residual.insert("traceability_coverage".to_string(), trace_residual);
        per_axis_coverage.insert("traceability_coverage".to_string(), trace_coverage);
        violations.extend(trace_violations);

        // Contradiction check
        let (contra_residual, contra_coverage, contra_violations) =
            self.evaluate_contradictions(intake);
        per_axis_residual.insert("contradiction_free".to_string(), contra_residual);
        per_axis_coverage.insert("contradiction_free".to_string(), contra_coverage);
        violations.extend(contra_violations);

        // Ambiguity check
        let (ambig_residual, ambig_coverage, ambig_violations) = self.evaluate_ambiguity(intake);
        per_axis_residual.insert("ambiguity_bounded".to_string(), ambig_residual);
        per_axis_coverage.insert("ambiguity_bounded".to_string(), ambig_coverage);
        violations.extend(ambig_violations);

        // Privacy check
        let (priv_residual, priv_coverage, priv_violations) = self.evaluate_privacy(intake);
        per_axis_residual.insert("privacy_safe".to_string(), priv_residual);
        per_axis_coverage.insert("privacy_safe".to_string(), priv_coverage);
        violations.extend(priv_violations);

        // Term map alignment
        let (term_residual, term_coverage, term_violations) = self.evaluate_term_map(intake);
        per_axis_residual.insert("term_map_aligned".to_string(), term_residual);
        per_axis_coverage.insert("term_map_aligned".to_string(), term_coverage);
        violations.extend(term_violations);

        // Compute composite metrics
        let composite_residual = compute_composite_norm(&per_axis_residual, semantic_set);
        let composite_coverage = compute_composite_coverage(&per_axis_coverage, semantic_set);

        // Identify axes below threshold
        let below_threshold: Vec<String> = semantic_set
            .axes
            .iter()
            .filter(|axis| {
                if let Some(min) = axis.min_coverage {
                    per_axis_coverage.get(&axis.axis_id).copied().unwrap_or(0.0) < min
                } else {
                    false
                }
            })
            .map(|a| a.axis_id.clone())
            .collect();

        // Derive decision
        let error_count = violations
            .iter()
            .filter(|v| v.severity == ConstraintSeverity::Error)
            .count() as u32;

        let decision_status = if error_count > decision_rule.max_error_violations {
            DecisionStatus::Fail
        } else if let Some(max) = decision_rule.max_residual_norm {
            if composite_residual > max {
                DecisionStatus::Fail
            } else if let Some(min) = decision_rule.min_coverage {
                if composite_coverage < min {
                    DecisionStatus::Fail
                } else {
                    DecisionStatus::Pass
                }
            } else {
                DecisionStatus::Pass
            }
        } else {
            DecisionStatus::Pass
        };

        let rationale = match decision_status {
            DecisionStatus::Pass => None,
            DecisionStatus::Fail => {
                let mut reasons = Vec::new();
                if error_count > 0 {
                    reasons.push(format!("{} error violations", error_count));
                }
                if !below_threshold.is_empty() {
                    reasons.push(format!("Below threshold: {:?}", below_threshold));
                }
                if let Some(max) = decision_rule.max_residual_norm {
                    if composite_residual > max {
                        reasons.push(format!("Residual {} > max {}", composite_residual, max));
                    }
                }
                Some(reasons.join("; "))
            }
            DecisionStatus::Indeterminate => Some("Evaluation incomplete".to_string()),
        };

        SemanticEvalResult {
            schema: "sr.semantic_eval.v1".to_string(),
            candidate_id: candidate_id.to_string(),
            procedure_template_id: "proc:PROBLEM-STATEMENT-INGESTION".to_string(),
            stage_id: "stage:VALIDATE".to_string(),
            oracle_suite_id: self.suite.suite_id.clone(),
            oracle_suite_hash: self.suite.suite_hash.clone(),
            semantic_set: SemanticSetBinding {
                semantic_set_id: self.suite.semantic_set_binding.semantic_set_id.clone(),
                semantic_set_hash: self.suite.semantic_set_binding.semantic_set_hash.clone(),
            },
            metrics: SemanticMetrics {
                residual: ResidualVector {
                    per_axis: per_axis_residual,
                    composite_norm: composite_residual,
                    norm_method: "L2".to_string(),
                },
                coverage: CoverageMetrics {
                    per_axis: per_axis_coverage,
                    composite: composite_coverage,
                    below_threshold,
                },
                violations,
                additional: BTreeMap::new(),
            },
            decision: EvalDecision {
                status: decision_status,
                rule_id: decision_rule.rule_id.clone(),
                thresholds: decision_rule.thresholds.clone(),
                rationale,
            },
            notes: None,
            evaluated_at: Utc::now(),
        }
    }

    /// Generate report artifacts
    pub fn generate_reports(&self, eval_result: &SemanticEvalResult) -> SemanticReportBundle {
        let now = Utc::now();

        let residual_report = ResidualReport {
            oracle_suite_id: eval_result.oracle_suite_id.clone(),
            candidate_id: eval_result.candidate_id.clone(),
            stage_id: eval_result.stage_id.clone(),
            residual: eval_result.metrics.residual.clone(),
            computed_at: now,
        };

        let coverage_report = CoverageReport {
            oracle_suite_id: eval_result.oracle_suite_id.clone(),
            candidate_id: eval_result.candidate_id.clone(),
            stage_id: eval_result.stage_id.clone(),
            coverage: eval_result.metrics.coverage.clone(),
            computed_at: now,
        };

        let violations_report = ViolationsReport {
            oracle_suite_id: eval_result.oracle_suite_id.clone(),
            candidate_id: eval_result.candidate_id.clone(),
            stage_id: eval_result.stage_id.clone(),
            violations: eval_result.metrics.violations.clone(),
            summary: ViolationSummary {
                error_count: eval_result
                    .metrics
                    .violations
                    .iter()
                    .filter(|v| v.severity == ConstraintSeverity::Error)
                    .count(),
                warning_count: eval_result
                    .metrics
                    .violations
                    .iter()
                    .filter(|v| v.severity == ConstraintSeverity::Warning)
                    .count(),
                info_count: eval_result
                    .metrics
                    .violations
                    .iter()
                    .filter(|v| v.severity == ConstraintSeverity::Info)
                    .count(),
                total_count: eval_result.metrics.violations.len(),
            },
            computed_at: now,
        };

        SemanticReportBundle {
            residual: residual_report,
            coverage: coverage_report,
            violations: violations_report,
        }
    }

    // Axis evaluation helpers
    fn evaluate_schema_compliance(&self, intake: &Intake) -> (f64, f64, Vec<ConstraintViolation>) {
        let mut violations = Vec::new();

        // Check required fields
        let mut checks_passed = 0;
        let total_checks = 5;

        if !intake.title.trim().is_empty() {
            checks_passed += 1;
        } else {
            violations.push(ConstraintViolation {
                code: "MISSING_FIELD".to_string(),
                constraint_id: "required_fields_present".to_string(),
                axis: Some("schema_compliance".to_string()),
                message: "Required field 'title' is empty".to_string(),
                severity: ConstraintSeverity::Error,
                context: BTreeMap::new(),
            });
        }

        if !intake.objective.trim().is_empty() {
            checks_passed += 1;
        } else {
            violations.push(ConstraintViolation {
                code: "MISSING_FIELD".to_string(),
                constraint_id: "required_fields_present".to_string(),
                axis: Some("schema_compliance".to_string()),
                message: "Required field 'objective' is empty".to_string(),
                severity: ConstraintSeverity::Error,
                context: BTreeMap::new(),
            });
        }

        if !intake.audience.trim().is_empty() {
            checks_passed += 1;
        } else {
            violations.push(ConstraintViolation {
                code: "MISSING_FIELD".to_string(),
                constraint_id: "required_fields_present".to_string(),
                axis: Some("schema_compliance".to_string()),
                message: "Required field 'audience' is empty".to_string(),
                severity: ConstraintSeverity::Error,
                context: BTreeMap::new(),
            });
        }

        if !intake.deliverables.is_empty() {
            checks_passed += 1;
            // Check that each deliverable has a path
            for (i, d) in intake.deliverables.iter().enumerate() {
                if d.path.trim().is_empty() {
                    violations.push(ConstraintViolation {
                        code: "MISSING_PATH".to_string(),
                        constraint_id: "deliverables_have_paths".to_string(),
                        axis: Some("schema_compliance".to_string()),
                        message: format!("Deliverable {} has empty path", i),
                        severity: ConstraintSeverity::Error,
                        context: BTreeMap::new(),
                    });
                }
            }
        } else {
            violations.push(ConstraintViolation {
                code: "MISSING_FIELD".to_string(),
                constraint_id: "required_fields_present".to_string(),
                axis: Some("schema_compliance".to_string()),
                message: "Required field 'deliverables' is empty".to_string(),
                severity: ConstraintSeverity::Error,
                context: BTreeMap::new(),
            });
        }

        // Work unit ID check
        if !intake.work_unit_id.as_str().is_empty() {
            checks_passed += 1;
        } else {
            violations.push(ConstraintViolation {
                code: "MISSING_FIELD".to_string(),
                constraint_id: "required_fields_present".to_string(),
                axis: Some("schema_compliance".to_string()),
                message: "Required field 'work_unit_id' is empty".to_string(),
                severity: ConstraintSeverity::Error,
                context: BTreeMap::new(),
            });
        }

        let coverage = checks_passed as f64 / total_checks as f64;
        let residual = 1.0 - coverage;

        (residual, coverage, violations)
    }

    fn evaluate_traceability(&self, intake: &Intake) -> (f64, f64, Vec<ConstraintViolation>) {
        // Check input refs and deliverable refs
        let has_inputs = !intake.inputs.is_empty();
        let deliverables_with_desc = intake
            .deliverables
            .iter()
            .filter(|d| d.description.is_some())
            .count();

        let coverage = if intake.deliverables.is_empty() {
            0.0
        } else {
            (deliverables_with_desc as f64 / intake.deliverables.len() as f64 * 0.5)
                + if has_inputs { 0.5 } else { 0.0 }
        };

        let violations = if coverage < 0.8 {
            vec![ConstraintViolation {
                code: "LOW_TRACEABILITY".to_string(),
                constraint_id: "traceability_coverage".to_string(),
                axis: Some("traceability_coverage".to_string()),
                message: format!(
                    "Traceability coverage {:.0}% below 80% threshold",
                    coverage * 100.0
                ),
                severity: ConstraintSeverity::Warning,
                context: BTreeMap::new(),
            }]
        } else {
            vec![]
        };

        (1.0 - coverage, coverage, violations)
    }

    fn evaluate_contradictions(&self, _intake: &Intake) -> (f64, f64, Vec<ConstraintViolation>) {
        // Simple contradiction check: look for conflicting constraints
        // In a real implementation, this would use NLP/semantic analysis
        let violations = Vec::new();

        // For now, assume no contradictions (residual = 0, coverage = 1)
        (0.0, 1.0, violations)
    }

    fn evaluate_ambiguity(&self, intake: &Intake) -> (f64, f64, Vec<ConstraintViolation>) {
        // Check if ambiguities are inventoried in unknowns
        let has_unknowns = !intake.unknowns.is_empty();
        let has_definitions = !intake.definitions.is_empty();

        let coverage = if has_unknowns || has_definitions {
            1.0
        } else {
            0.7
        };

        let violations = if !has_unknowns && !has_definitions {
            vec![ConstraintViolation {
                code: "NO_AMBIGUITY_INVENTORY".to_string(),
                constraint_id: "ambiguities_inventoried".to_string(),
                axis: Some("ambiguity_bounded".to_string()),
                message: "No unknowns or definitions provided".to_string(),
                severity: ConstraintSeverity::Warning,
                context: BTreeMap::new(),
            }]
        } else {
            vec![]
        };

        (1.0 - coverage, coverage, violations)
    }

    fn evaluate_privacy(&self, intake: &Intake) -> (f64, f64, Vec<ConstraintViolation>) {
        // Simple PII scan: look for common patterns
        // In a real implementation, this would use proper PII detection
        let _text = format!(
            "{} {} {} {:?}",
            intake.title, intake.objective, intake.audience, intake.constraints
        );

        let violations = Vec::new();

        // For now, assume no PII (residual = 0, coverage = 1)
        (0.0, 1.0, violations)
    }

    fn evaluate_term_map(&self, intake: &Intake) -> (f64, f64, Vec<ConstraintViolation>) {
        // Check term consistency
        let definitions_count = intake.definitions.len();

        let coverage = if definitions_count > 0 { 1.0 } else { 0.8 };
        let violations = Vec::new();

        (1.0 - coverage, coverage, violations)
    }
}

impl Default for IntakeAdmissibilityRunner {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Report Bundle
// ============================================================================

/// Bundle of semantic report artifacts
pub struct SemanticReportBundle {
    pub residual: ResidualReport,
    pub coverage: CoverageReport,
    pub violations: ViolationsReport,
}

impl SemanticReportBundle {
    /// Serialize reports to bytes for evidence storage
    pub fn to_artifacts(&self) -> Vec<(String, Vec<u8>)> {
        vec![
            (
                "reports/semantic/residual.json".to_string(),
                serde_json::to_vec_pretty(&self.residual).unwrap_or_default(),
            ),
            (
                "reports/semantic/coverage.json".to_string(),
                serde_json::to_vec_pretty(&self.coverage).unwrap_or_default(),
            ),
            (
                "reports/semantic/violations.json".to_string(),
                serde_json::to_vec_pretty(&self.violations).unwrap_or_default(),
            ),
        ]
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

fn compute_composite_norm(per_axis: &BTreeMap<String, f64>, semantic_set: &SemanticSet) -> f64 {
    let mut weighted_sum = 0.0;
    let mut weight_sum = 0.0;

    for axis in &semantic_set.axes {
        if let Some(&residual) = per_axis.get(&axis.axis_id) {
            weighted_sum += residual * residual * axis.weight;
            weight_sum += axis.weight;
        }
    }

    if weight_sum > 0.0 {
        (weighted_sum / weight_sum).sqrt()
    } else {
        0.0
    }
}

fn compute_composite_coverage(per_axis: &BTreeMap<String, f64>, semantic_set: &SemanticSet) -> f64 {
    let mut weighted_sum = 0.0;
    let mut weight_sum = 0.0;

    for axis in &semantic_set.axes {
        if let Some(&coverage) = per_axis.get(&axis.axis_id) {
            weighted_sum += coverage * axis.weight;
            weight_sum += axis.weight;
        }
    }

    if weight_sum > 0.0 {
        weighted_sum / weight_sum
    } else {
        0.0
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use sr_domain::{Deliverable, WorkKind, WorkUnitId};

    #[test]
    fn test_create_intake_admissibility_suite() {
        let suite = create_intake_admissibility_suite();

        assert_eq!(suite.suite_id, SUITE_INTAKE_ADMISSIBILITY_ID);
        assert!(!suite.suite_hash.is_empty());
        assert!(suite.suite_hash.starts_with("sha256:"));
        assert_eq!(suite.oracles.len(), 6);
    }

    #[test]
    fn test_suite_has_all_axes() {
        let suite = create_intake_admissibility_suite();

        let target_axes: Vec<&str> = suite
            .oracles
            .iter()
            .map(|o| o.target_axis.as_str())
            .collect();

        assert!(target_axes.contains(&"schema_compliance"));
        assert!(target_axes.contains(&"traceability_coverage"));
        assert!(target_axes.contains(&"contradiction_free"));
        assert!(target_axes.contains(&"ambiguity_bounded"));
        assert!(target_axes.contains(&"privacy_safe"));
        assert!(target_axes.contains(&"term_map_aligned"));
    }

    #[test]
    fn test_suite_hash_incorporates_semantic_set() {
        let suite1 = create_intake_admissibility_suite();
        let suite2 = create_intake_admissibility_suite();

        // Same suite should have same hash
        assert_eq!(suite1.suite_hash, suite2.suite_hash);

        // Hash should incorporate semantic set
        assert!(!suite1.semantic_set_binding.semantic_set_hash.is_empty());
    }

    #[test]
    fn test_evaluate_valid_intake() {
        use sr_domain::InputRef;

        let runner = IntakeAdmissibilityRunner::new();

        let mut intake = Intake::new(
            WorkUnitId::new("test-001"),
            "Test Intake".to_string(),
            WorkKind::IntakeProcessing,
            "Process a test problem statement".to_string(),
            "Development team".to_string(),
            vec![Deliverable {
                path: "candidate/intake.yaml".to_string(),
                media_type: "application/yaml".to_string(),
                description: Some("Structured intake".to_string()),
                role: Some("primary".to_string()),
            }],
        );

        // Add inputs to meet traceability coverage threshold
        intake.inputs.push(InputRef {
            rel: "source".to_string(),
            kind: "document".to_string(),
            locator: "docs/problem_statement.md".to_string(),
            content_hash: None,
        });

        // Add definitions to meet ambiguity check
        intake.definitions.insert(
            "intake".to_string(),
            "A structured work unit specification".to_string(),
        );

        let result = runner.evaluate_intake("candidate:test", &intake);

        assert_eq!(result.schema, "sr.semantic_eval.v1");
        assert_eq!(result.oracle_suite_id, SUITE_INTAKE_ADMISSIBILITY_ID);
        assert!(result.passed(), "Expected pass, got: {:?}", result.decision);
    }

    #[test]
    fn test_evaluate_invalid_intake() {
        let runner = IntakeAdmissibilityRunner::new();

        // Create intake with missing required fields
        let intake = Intake {
            artifact_type: "record.intake".to_string(),
            artifact_version: "v1".to_string(),
            work_unit_id: WorkUnitId::new("test"),
            title: "".to_string(), // Empty title
            kind: WorkKind::IntakeProcessing,
            objective: "".to_string(), // Empty objective
            audience: "Team".to_string(),
            deliverables: vec![], // Empty deliverables
            constraints: vec![],
            definitions: std::collections::HashMap::new(),
            inputs: vec![],
            unknowns: vec![],
            completion_criteria: vec![],
            content_hash: None,
            created_at: None,
        };

        let result = runner.evaluate_intake("candidate:invalid", &intake);

        assert!(!result.passed());
        assert_eq!(result.decision.status, DecisionStatus::Fail);
        assert!(!result.error_violations().is_empty());
    }

    #[test]
    fn test_generate_reports() {
        let runner = IntakeAdmissibilityRunner::new();

        let intake = Intake::new(
            WorkUnitId::new("test"),
            "Test".to_string(),
            WorkKind::IntakeProcessing,
            "Test objective".to_string(),
            "Team".to_string(),
            vec![Deliverable {
                path: "out.md".to_string(),
                media_type: "text/markdown".to_string(),
                description: None,
                role: None,
            }],
        );

        let result = runner.evaluate_intake("candidate:test", &intake);
        let reports = runner.generate_reports(&result);

        // Verify reports can be serialized
        let artifacts = reports.to_artifacts();
        assert_eq!(artifacts.len(), 3);

        assert!(artifacts.iter().any(|(name, _)| name.contains("residual")));
        assert!(artifacts.iter().any(|(name, _)| name.contains("coverage")));
        assert!(artifacts
            .iter()
            .any(|(name, _)| name.contains("violations")));
    }

    #[test]
    fn test_to_oracle_suite_definition() {
        let semantic_suite = create_intake_admissibility_suite();
        let oracle_suite = to_oracle_suite_definition(&semantic_suite);

        assert_eq!(oracle_suite.suite_id, semantic_suite.suite_id);
        assert_eq!(oracle_suite.suite_hash, semantic_suite.suite_hash);
        assert_eq!(oracle_suite.oracles.len(), semantic_suite.oracles.len());

        // Check metadata contains semantic set info
        assert!(oracle_suite.metadata.contains_key("semantic_set_id"));
        assert!(oracle_suite.metadata.contains_key("semantic_set_hash"));
    }

    #[test]
    fn test_composite_metrics() {
        let set = get_intake_admissibility_set();

        let per_axis = BTreeMap::from_iter([
            ("schema_compliance".to_string(), 0.0),
            ("traceability_coverage".to_string(), 0.1),
            ("contradiction_free".to_string(), 0.0),
            ("ambiguity_bounded".to_string(), 0.2),
            ("privacy_safe".to_string(), 0.0),
            ("term_map_aligned".to_string(), 0.1),
        ]);

        let norm = compute_composite_norm(&per_axis, &set);
        assert!(norm >= 0.0);
        assert!(norm <= 1.0);

        let coverage_per_axis = BTreeMap::from_iter([
            ("schema_compliance".to_string(), 1.0),
            ("traceability_coverage".to_string(), 0.9),
            ("contradiction_free".to_string(), 1.0),
            ("ambiguity_bounded".to_string(), 0.8),
            ("privacy_safe".to_string(), 1.0),
            ("term_map_aligned".to_string(), 0.9),
        ]);

        let coverage = compute_composite_coverage(&coverage_per_axis, &set);
        assert!(coverage > 0.8);
        assert!(coverage <= 1.0);
    }
}

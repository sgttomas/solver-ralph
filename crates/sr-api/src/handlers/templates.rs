//! Template Registry API Handlers
//!
//! Per SR-TEMPLATES: Endpoints for browsing, viewing, and instantiating
//! user-configurable templates across 11 categories. Templates define
//! schemas for governed artifacts like Intakes, Procedure Templates,
//! Oracle Suites, Verification Profiles, and Exceptions.
//!
//! Endpoints:
//! - GET  /api/v1/templates                - List templates by category
//! - GET  /api/v1/templates/:template_id   - Get template detail
//! - GET  /api/v1/templates/schemas        - Get all template schemas
//! - GET  /api/v1/templates/schemas/:type_key - Get schema for type
//! - POST /api/v1/templates                - Create new template instance

use axum::{
    extract::{Path, Query, State},
    Json,
};
use rand;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, instrument};

use crate::auth::AuthenticatedUser;
use crate::handlers::{ApiError, ApiResult};

// ============================================================================
// Template Category Definitions
// ============================================================================

/// Template categories per SR-TEMPLATES
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TemplateCategory {
    WorkSurface,
    Oracle,
    Verification,
    SemanticSets,
    Gates,
    Portals,
    Execution,
    Context,
    Evidence,
    Release,
    Exceptions,
}

impl TemplateCategory {
    pub fn all() -> Vec<TemplateCategory> {
        vec![
            TemplateCategory::WorkSurface,
            TemplateCategory::Oracle,
            TemplateCategory::Verification,
            TemplateCategory::SemanticSets,
            TemplateCategory::Gates,
            TemplateCategory::Portals,
            TemplateCategory::Execution,
            TemplateCategory::Context,
            TemplateCategory::Evidence,
            TemplateCategory::Release,
            TemplateCategory::Exceptions,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            TemplateCategory::WorkSurface => "Work Surface",
            TemplateCategory::Oracle => "Oracle Configuration",
            TemplateCategory::Verification => "Verification Profiles",
            TemplateCategory::SemanticSets => "Semantic Sets",
            TemplateCategory::Gates => "Gate Configuration",
            TemplateCategory::Portals => "Portal Configuration",
            TemplateCategory::Execution => "Execution Policy",
            TemplateCategory::Context => "Iteration Context",
            TemplateCategory::Evidence => "Evidence",
            TemplateCategory::Release => "Release",
            TemplateCategory::Exceptions => "Exceptions",
        }
    }

    pub fn type_keys(&self) -> Vec<&'static str> {
        match self {
            TemplateCategory::WorkSurface => {
                vec!["record.intake", "config.procedure_template", "domain.work_surface"]
            }
            TemplateCategory::Oracle => {
                vec!["oracle_suite", "oracle_definition", "semantic_oracle"]
            }
            TemplateCategory::Verification => {
                vec!["verification_profile", "profile_selection_matrix"]
            }
            TemplateCategory::SemanticSets => {
                vec!["config.semantic_set", "semantic_axis", "decision_rule"]
            }
            TemplateCategory::Gates => vec!["gate_definition"],
            TemplateCategory::Portals => vec!["portal_playbook"],
            TemplateCategory::Execution => {
                vec!["budget_config", "stop_trigger", "config.gating_policy"]
            }
            TemplateCategory::Context => vec!["required_refs", "iteration_summary"],
            TemplateCategory::Evidence => {
                vec!["domain.evidence_bundle", "sr.semantic_eval.v1"]
            }
            TemplateCategory::Release => vec!["record.freeze"],
            TemplateCategory::Exceptions => {
                vec!["record.waiver", "record.deviation", "record.deferral"]
            }
        }
    }

    pub fn requires_portal(&self) -> bool {
        matches!(
            self,
            TemplateCategory::Oracle
                | TemplateCategory::Verification
                | TemplateCategory::SemanticSets
                | TemplateCategory::Exceptions
                | TemplateCategory::Release
        )
    }

    pub fn is_user_instantiable(&self) -> bool {
        matches!(
            self,
            TemplateCategory::WorkSurface
                | TemplateCategory::Oracle
                | TemplateCategory::Verification
                | TemplateCategory::SemanticSets
                | TemplateCategory::Execution
                | TemplateCategory::Exceptions
        )
    }
}

// ============================================================================
// Template Registry State
// ============================================================================

/// State for template registry endpoints
#[derive(Clone)]
pub struct TemplateRegistryState {
    pub registry: Arc<TemplateRegistry>,
}

/// In-memory template registry
pub struct TemplateRegistry {
    templates: RwLock<Vec<TemplateInstance>>,
    schemas: Vec<TemplateSchema>,
}

impl TemplateRegistry {
    pub fn new() -> Self {
        Self {
            templates: RwLock::new(Self::build_starter_instances()),
            schemas: Self::build_schemas(),
        }
    }

    /// Build starter reference templates per SR-TEMPLATES Phase 2
    fn build_starter_instances() -> Vec<TemplateInstance> {
        let timestamp = "2026-01-15T00:00:00Z".to_string();
        let created_by = "system".to_string();

        vec![
            // 1. Intake - Standard Research Memo
            TemplateInstance {
                id: "tmpl_starter_intake".to_string(),
                type_key: "record.intake".to_string(),
                name: "Standard Research Memo Intake".to_string(),
                category: TemplateCategory::WorkSurface,
                status: "reference".to_string(),
                content_hash: compute_content_hash(&serde_json::json!({
                    "work_unit_id": "WU-TEMPLATE-001",
                    "title": "API Rate Limiting Analysis",
                    "kind": "research_memo",
                    "audience": "Engineering team",
                    "objective": "Evaluate rate limiting strategies for the public API",
                    "deliverables": [
                        "Analysis of current implementation",
                        "Comparison of 3+ strategies",
                        "Recommendation with migration path"
                    ],
                    "constraints": [
                        "Maximum 2000 words",
                        "Must include performance impact analysis"
                    ],
                    "definitions": {
                        "rate_limit": "Maximum requests per time window per client",
                        "burst": "Short-term allowance above sustained rate"
                    },
                    "inputs": [],
                    "unknowns": ["Acceptable latency for rate limit checks?"],
                    "completion_criteria": [
                        "All strategies evaluated",
                        "Recommendation includes migration path"
                    ]
                })),
                content: serde_json::json!({
                    "work_unit_id": "WU-TEMPLATE-001",
                    "title": "API Rate Limiting Analysis",
                    "kind": "research_memo",
                    "audience": "Engineering team",
                    "objective": "Evaluate rate limiting strategies for the public API",
                    "deliverables": [
                        "Analysis of current implementation",
                        "Comparison of 3+ strategies",
                        "Recommendation with migration path"
                    ],
                    "constraints": [
                        "Maximum 2000 words",
                        "Must include performance impact analysis"
                    ],
                    "definitions": {
                        "rate_limit": "Maximum requests per time window per client",
                        "burst": "Short-term allowance above sustained rate"
                    },
                    "inputs": [],
                    "unknowns": ["Acceptable latency for rate limit checks?"],
                    "completion_criteria": [
                        "All strategies evaluated",
                        "Recommendation includes migration path"
                    ]
                }),
                created_at: timestamp.clone(),
                updated_at: timestamp.clone(),
                created_by: created_by.clone(),
                requires_portal: false,
                portal_approval_id: None,
                refs: vec![],
            },
            // 2. Procedure Template - Research Memo Procedure
            // Note: This uses the same structure as sr-domain's generic_knowledge_work_template()
            // to ensure proper deserialization into ProcedureTemplate struct
            TemplateInstance {
                id: "tmpl_starter_procedure".to_string(),
                type_key: "config.procedure_template".to_string(),
                name: "Research Memo Procedure".to_string(),
                category: TemplateCategory::WorkSurface,
                status: "reference".to_string(),
                content_hash: compute_content_hash(&serde_json::json!({
                    "artifact_type": "config.procedure_template",
                    "artifact_version": "v1",
                    "procedure_template_id": "proc:RESEARCH-MEMO",
                    "kind": ["research_memo"],
                    "name": "Research Memo Procedure",
                    "description": "Stage-gated workflow for research memos",
                    "terminal_stage_id": "stage:FINAL",
                    "initial_stage_id": "stage:FRAME",
                    "stages": [
                        {
                            "stage_id": "stage:FRAME",
                            "stage_name": "Frame",
                            "purpose": "Restate objective, extract constraints",
                            "required_outputs": [],
                            "steps": [],
                            "required_oracle_suites": ["suite:SR-SUITE-GOV"],
                            "gate_rule": "all_required_oracles_pass",
                            "transition_on_pass": "stage:OPTIONS",
                            "requires_portal": false,
                            "requires_approval": false
                        },
                        {
                            "stage_id": "stage:OPTIONS",
                            "stage_name": "Options Analysis",
                            "purpose": "Generate multiple candidate approaches",
                            "required_outputs": [],
                            "steps": [],
                            "required_oracle_suites": ["suite:SR-SUITE-GOV", "suite:SR-SUITE-CORE"],
                            "gate_rule": "all_required_oracles_pass",
                            "transition_on_pass": "stage:DRAFT",
                            "requires_portal": false,
                            "requires_approval": false
                        },
                        {
                            "stage_id": "stage:DRAFT",
                            "stage_name": "Draft",
                            "purpose": "Produce candidate deliverable",
                            "required_outputs": [],
                            "steps": [],
                            "required_oracle_suites": ["suite:SR-SUITE-CORE"],
                            "gate_rule": "all_required_oracles_pass",
                            "transition_on_pass": "stage:FINAL",
                            "requires_portal": false,
                            "requires_approval": false
                        },
                        {
                            "stage_id": "stage:FINAL",
                            "stage_name": "Final",
                            "purpose": "Package final candidate and evidence",
                            "required_outputs": [],
                            "steps": [],
                            "required_oracle_suites": ["suite:SR-SUITE-CORE", "suite:SR-SUITE-FULL"],
                            "gate_rule": "all_required_oracles_pass",
                            "transition_on_pass": null,
                            "requires_portal": false,
                            "requires_approval": true
                        }
                    ]
                })),
                content: serde_json::json!({
                    "artifact_type": "config.procedure_template",
                    "artifact_version": "v1",
                    "procedure_template_id": "proc:RESEARCH-MEMO",
                    "kind": ["research_memo"],
                    "name": "Research Memo Procedure",
                    "description": "Stage-gated workflow for research memos",
                    "terminal_stage_id": "stage:FINAL",
                    "initial_stage_id": "stage:FRAME",
                    "stages": [
                        {
                            "stage_id": "stage:FRAME",
                            "stage_name": "Frame",
                            "purpose": "Restate objective, extract constraints",
                            "required_outputs": [],
                            "steps": [],
                            "required_oracle_suites": ["suite:SR-SUITE-GOV"],
                            "gate_rule": "all_required_oracles_pass",
                            "transition_on_pass": "stage:OPTIONS",
                            "requires_portal": false,
                            "requires_approval": false
                        },
                        {
                            "stage_id": "stage:OPTIONS",
                            "stage_name": "Options Analysis",
                            "purpose": "Generate multiple candidate approaches",
                            "required_outputs": [],
                            "steps": [],
                            "required_oracle_suites": ["suite:SR-SUITE-GOV", "suite:SR-SUITE-CORE"],
                            "gate_rule": "all_required_oracles_pass",
                            "transition_on_pass": "stage:DRAFT",
                            "requires_portal": false,
                            "requires_approval": false
                        },
                        {
                            "stage_id": "stage:DRAFT",
                            "stage_name": "Draft",
                            "purpose": "Produce candidate deliverable",
                            "required_outputs": [],
                            "steps": [],
                            "required_oracle_suites": ["suite:SR-SUITE-CORE"],
                            "gate_rule": "all_required_oracles_pass",
                            "transition_on_pass": "stage:FINAL",
                            "requires_portal": false,
                            "requires_approval": false
                        },
                        {
                            "stage_id": "stage:FINAL",
                            "stage_name": "Final",
                            "purpose": "Package final candidate and evidence",
                            "required_outputs": [],
                            "steps": [],
                            "required_oracle_suites": ["suite:SR-SUITE-CORE", "suite:SR-SUITE-FULL"],
                            "gate_rule": "all_required_oracles_pass",
                            "transition_on_pass": null,
                            "requires_portal": false,
                            "requires_approval": true
                        }
                    ]
                }),
                created_at: timestamp.clone(),
                updated_at: timestamp.clone(),
                created_by: created_by.clone(),
                requires_portal: false,
                portal_approval_id: None,
                refs: vec![],
            },
            // 3. Work Surface Instance - Example Binding
            TemplateInstance {
                id: "tmpl_starter_work_surface".to_string(),
                type_key: "domain.work_surface".to_string(),
                name: "Example Work Surface Binding".to_string(),
                category: TemplateCategory::WorkSurface,
                status: "reference".to_string(),
                content_hash: compute_content_hash(&serde_json::json!({
                    "work_unit_id": "WU-TEMPLATE-001",
                    "intake_ref": {
                        "id": "intake:WU-TEMPLATE-001",
                        "content_hash": "sha256:placeholder"
                    },
                    "procedure_template_ref": {
                        "id": "proc:RESEARCH-MEMO",
                        "content_hash": "sha256:placeholder"
                    },
                    "stage_id": "stage:FRAME",
                    "oracle_suites": [
                        {"suite_id": "suite:SR-SUITE-GOV", "suite_hash": "sha256:placeholder"}
                    ],
                    "params": {
                        "max_residual_norm": 0.2,
                        "min_coverage": 0.8
                    }
                })),
                content: serde_json::json!({
                    "work_unit_id": "WU-TEMPLATE-001",
                    "intake_ref": {
                        "id": "intake:WU-TEMPLATE-001",
                        "content_hash": "sha256:placeholder"
                    },
                    "procedure_template_ref": {
                        "id": "proc:RESEARCH-MEMO",
                        "content_hash": "sha256:placeholder"
                    },
                    "stage_id": "stage:FRAME",
                    "oracle_suites": [
                        {"suite_id": "suite:SR-SUITE-GOV", "suite_hash": "sha256:placeholder"}
                    ],
                    "params": {
                        "max_residual_norm": 0.2,
                        "min_coverage": 0.8
                    }
                }),
                created_at: timestamp.clone(),
                updated_at: timestamp.clone(),
                created_by: created_by.clone(),
                requires_portal: false,
                portal_approval_id: None,
                refs: vec![],
            },
            // 4. Budget Config - Standard Budget Policy
            TemplateInstance {
                id: "tmpl_starter_budget".to_string(),
                type_key: "budget_config".to_string(),
                name: "Standard Budget Policy".to_string(),
                category: TemplateCategory::Execution,
                status: "reference".to_string(),
                content_hash: compute_content_hash(&serde_json::json!({
                    "policy_id": "budget:STANDARD",
                    "max_iterations": 5,
                    "max_oracle_runs": 25,
                    "max_wallclock_hours": 16,
                    "on_exhaustion": {
                        "stop_trigger": "BUDGET_EXHAUSTED",
                        "routing_portal": "HumanAuthorityExceptionProcess"
                    }
                })),
                content: serde_json::json!({
                    "policy_id": "budget:STANDARD",
                    "max_iterations": 5,
                    "max_oracle_runs": 25,
                    "max_wallclock_hours": 16,
                    "on_exhaustion": {
                        "stop_trigger": "BUDGET_EXHAUSTED",
                        "routing_portal": "HumanAuthorityExceptionProcess"
                    }
                }),
                created_at: timestamp.clone(),
                updated_at: timestamp.clone(),
                created_by: created_by.clone(),
                requires_portal: false,
                portal_approval_id: None,
                refs: vec![],
            },
            // 5. Gating Policy - Hybrid Gating Policy
            TemplateInstance {
                id: "tmpl_starter_gating".to_string(),
                type_key: "config.gating_policy".to_string(),
                name: "Hybrid Gating Policy".to_string(),
                category: TemplateCategory::Execution,
                status: "reference".to_string(),
                content_hash: compute_content_hash(&serde_json::json!({
                    "policy_id": "gating:HYBRID",
                    "mode": "hybrid",
                    "hooks": {
                        "plan_review": "soft",
                        "evaluation_on_verification": "soft",
                        "assessment_on_validation": "soft",
                        "closeout": "hard"
                    },
                    "hard_triggers": [
                        "EXCEPTIONS_ACTIVE",
                        "OPEN_RISK_HIGH",
                        "REPEATED_FAILURE",
                        "BUDGET_NEAR_EXHAUSTED",
                        "GOVERNANCE_TOUCH",
                        "CLOSEOUT_PENDING"
                    ]
                })),
                content: serde_json::json!({
                    "policy_id": "gating:HYBRID",
                    "mode": "hybrid",
                    "hooks": {
                        "plan_review": "soft",
                        "evaluation_on_verification": "soft",
                        "assessment_on_validation": "soft",
                        "closeout": "hard"
                    },
                    "hard_triggers": [
                        "EXCEPTIONS_ACTIVE",
                        "OPEN_RISK_HIGH",
                        "REPEATED_FAILURE",
                        "BUDGET_NEAR_EXHAUSTED",
                        "GOVERNANCE_TOUCH",
                        "CLOSEOUT_PENDING"
                    ]
                }),
                created_at: timestamp.clone(),
                updated_at: timestamp.clone(),
                created_by: created_by.clone(),
                requires_portal: false,
                portal_approval_id: None,
                refs: vec![],
            },
            // 6. Verification Profile - Project Standard
            TemplateInstance {
                id: "tmpl_starter_verification".to_string(),
                type_key: "verification_profile".to_string(),
                name: "Project Standard Profile".to_string(),
                category: TemplateCategory::Verification,
                status: "reference".to_string(),
                content_hash: compute_content_hash(&serde_json::json!({
                    "profile_id": "PROJECT-STANDARD",
                    "profile_version": "1.0.0",
                    "description": "Standard verification profile for project deliverables",
                    "required_suites": ["suite:SR-SUITE-CORE"],
                    "advisory_suites": ["suite:SR-SUITE-FULL"],
                    "verification_mode_default": "STRICT",
                    "waiver_policy": {
                        "waiver_eligible_failures": ["BUILD_FAIL", "UNIT_FAIL", "LINT_FAIL"]
                    },
                    "non_waivable_integrity_conditions": [
                        "ORACLE_TAMPER",
                        "ORACLE_GAP",
                        "EVIDENCE_MISSING"
                    ]
                })),
                content: serde_json::json!({
                    "profile_id": "PROJECT-STANDARD",
                    "profile_version": "1.0.0",
                    "description": "Standard verification profile for project deliverables",
                    "required_suites": ["suite:SR-SUITE-CORE"],
                    "advisory_suites": ["suite:SR-SUITE-FULL"],
                    "verification_mode_default": "STRICT",
                    "waiver_policy": {
                        "waiver_eligible_failures": ["BUILD_FAIL", "UNIT_FAIL", "LINT_FAIL"]
                    },
                    "non_waivable_integrity_conditions": [
                        "ORACLE_TAMPER",
                        "ORACLE_GAP",
                        "EVIDENCE_MISSING"
                    ]
                }),
                created_at: timestamp.clone(),
                updated_at: timestamp.clone(),
                created_by: created_by.clone(),
                requires_portal: false,
                portal_approval_id: None,
                refs: vec![],
            },
            // 7. Semantic Set - Research Memo Quality
            TemplateInstance {
                id: "tmpl_starter_semantic_set".to_string(),
                type_key: "config.semantic_set".to_string(),
                name: "Research Memo Quality Set".to_string(),
                category: TemplateCategory::SemanticSets,
                status: "reference".to_string(),
                content_hash: compute_content_hash(&serde_json::json!({
                    "semantic_set_id": "semset:research-memo-quality",
                    "name": "Research Memo Quality Set",
                    "axes": [
                        {"axis_id": "clarity", "name": "Clarity", "weight": 0.3, "required": true, "min_coverage": 0.8, "max_residual": 0.15},
                        {"axis_id": "completeness", "name": "Completeness", "weight": 0.4, "required": true, "min_coverage": 0.9, "max_residual": 0.1},
                        {"axis_id": "actionability", "name": "Actionability", "weight": 0.3, "required": true, "min_coverage": 0.7, "max_residual": 0.2}
                    ],
                    "constraints": [
                        {"constraint_id": "has_recommendation", "constraint_type": "required", "severity": "error", "expression": "sections.recommendation.present"}
                    ],
                    "decision_rule": {
                        "max_residual_norm": 0.2,
                        "min_coverage": 0.8,
                        "max_error_violations": 0,
                        "max_warning_violations": 3
                    }
                })),
                content: serde_json::json!({
                    "semantic_set_id": "semset:research-memo-quality",
                    "name": "Research Memo Quality Set",
                    "axes": [
                        {"axis_id": "clarity", "name": "Clarity", "weight": 0.3, "required": true, "min_coverage": 0.8, "max_residual": 0.15},
                        {"axis_id": "completeness", "name": "Completeness", "weight": 0.4, "required": true, "min_coverage": 0.9, "max_residual": 0.1},
                        {"axis_id": "actionability", "name": "Actionability", "weight": 0.3, "required": true, "min_coverage": 0.7, "max_residual": 0.2}
                    ],
                    "constraints": [
                        {"constraint_id": "has_recommendation", "constraint_type": "required", "severity": "error", "expression": "sections.recommendation.present"}
                    ],
                    "decision_rule": {
                        "max_residual_norm": 0.2,
                        "min_coverage": 0.8,
                        "max_error_violations": 0,
                        "max_warning_violations": 3
                    }
                }),
                created_at: timestamp.clone(),
                updated_at: timestamp.clone(),
                created_by: created_by.clone(),
                requires_portal: false,
                portal_approval_id: None,
                refs: vec![],
            },
            // 8. Waiver - Example Waiver Template
            TemplateInstance {
                id: "tmpl_starter_waiver".to_string(),
                type_key: "record.waiver".to_string(),
                name: "Example Waiver Template".to_string(),
                category: TemplateCategory::Exceptions,
                status: "reference".to_string(),
                content_hash: compute_content_hash(&serde_json::json!({
                    "exception_id": "exc_TEMPLATE_WAIVER",
                    "kind": "WAIVER",
                    "oracle_failure_ref": "run:example/oracle:lint/LINT_FAIL",
                    "scope": "per-candidate",
                    "risk_mitigation": "Lint failure is cosmetic; does not affect functionality",
                    "resolution_criteria": "Address in next iteration",
                    "expiry_date": "2026-02-15T00:00:00Z"
                })),
                content: serde_json::json!({
                    "exception_id": "exc_TEMPLATE_WAIVER",
                    "kind": "WAIVER",
                    "oracle_failure_ref": "run:example/oracle:lint/LINT_FAIL",
                    "scope": "per-candidate",
                    "risk_mitigation": "Lint failure is cosmetic; does not affect functionality",
                    "resolution_criteria": "Address in next iteration",
                    "expiry_date": "2026-02-15T00:00:00Z"
                }),
                created_at: timestamp.clone(),
                updated_at: timestamp.clone(),
                created_by: created_by.clone(),
                requires_portal: false,
                portal_approval_id: None,
                refs: vec![],
            },
            // 9. Deviation - Example Deviation Template
            TemplateInstance {
                id: "tmpl_starter_deviation".to_string(),
                type_key: "record.deviation".to_string(),
                name: "Example Deviation Template".to_string(),
                category: TemplateCategory::Exceptions,
                status: "reference".to_string(),
                content_hash: compute_content_hash(&serde_json::json!({
                    "exception_id": "exc_TEMPLATE_DEVIATION",
                    "kind": "DEVIATION",
                    "requirement_ref": "SR-SPEC:3.2.1",
                    "scope": "per-work-unit",
                    "justification": "Legacy integration requires non-standard context format",
                    "risk_mitigation": "Additional manual review step added",
                    "resolution_criteria": "Migrate to standard format in next release"
                })),
                content: serde_json::json!({
                    "exception_id": "exc_TEMPLATE_DEVIATION",
                    "kind": "DEVIATION",
                    "requirement_ref": "SR-SPEC:3.2.1",
                    "scope": "per-work-unit",
                    "justification": "Legacy integration requires non-standard context format",
                    "risk_mitigation": "Additional manual review step added",
                    "resolution_criteria": "Migrate to standard format in next release"
                }),
                created_at: timestamp.clone(),
                updated_at: timestamp.clone(),
                created_by: created_by.clone(),
                requires_portal: false,
                portal_approval_id: None,
                refs: vec![],
            },
            // 10. Deferral - Example Deferral Template
            TemplateInstance {
                id: "tmpl_starter_deferral".to_string(),
                type_key: "record.deferral".to_string(),
                name: "Example Deferral Template".to_string(),
                category: TemplateCategory::Exceptions,
                status: "reference".to_string(),
                content_hash: compute_content_hash(&serde_json::json!({
                    "exception_id": "exc_TEMPLATE_DEFERRAL",
                    "kind": "DEFERRAL",
                    "subject_ref": "WU-2026-042",
                    "scope": "per-baseline",
                    "justification": "Dependency on external API not yet available",
                    "target_date": "2026-03-01T00:00:00Z"
                })),
                content: serde_json::json!({
                    "exception_id": "exc_TEMPLATE_DEFERRAL",
                    "kind": "DEFERRAL",
                    "subject_ref": "WU-2026-042",
                    "scope": "per-baseline",
                    "justification": "Dependency on external API not yet available",
                    "target_date": "2026-03-01T00:00:00Z"
                }),
                created_at: timestamp.clone(),
                updated_at: timestamp.clone(),
                created_by: created_by.clone(),
                requires_portal: false,
                portal_approval_id: None,
                refs: vec![],
            },
            // 11. Oracle Suite - Custom Verification Suite
            TemplateInstance {
                id: "tmpl_starter_oracle_suite".to_string(),
                type_key: "oracle_suite".to_string(),
                name: "Custom Verification Suite".to_string(),
                category: TemplateCategory::Oracle,
                status: "reference".to_string(),
                content_hash: compute_content_hash(&serde_json::json!({
                    "suite_id": "suite:CUSTOM-VERIFY",
                    "suite_version": "1.0.0",
                    "description": "Custom verification suite example",
                    "determinism_required": true,
                    "environment_constraints": {
                        "runner": "ci",
                        "network": "disabled",
                        "cpu_arch": "amd64",
                        "os": "linux"
                    },
                    "oracles": [
                        {
                            "oracle_id": "oracle:custom-check",
                            "classification": "required",
                            "purpose": "Custom verification check",
                            "command": "sr-oracles verify --custom",
                            "timeout_seconds": 300,
                            "retries": 0
                        }
                    ],
                    "flake_policy": {
                        "on_required_flake": "stop_the_line",
                        "on_advisory_flake": "warn_only"
                    }
                })),
                content: serde_json::json!({
                    "suite_id": "suite:CUSTOM-VERIFY",
                    "suite_version": "1.0.0",
                    "description": "Custom verification suite example",
                    "determinism_required": true,
                    "environment_constraints": {
                        "runner": "ci",
                        "network": "disabled",
                        "cpu_arch": "amd64",
                        "os": "linux"
                    },
                    "oracles": [
                        {
                            "oracle_id": "oracle:custom-check",
                            "classification": "required",
                            "purpose": "Custom verification check",
                            "command": "sr-oracles verify --custom",
                            "timeout_seconds": 300,
                            "retries": 0
                        }
                    ],
                    "flake_policy": {
                        "on_required_flake": "stop_the_line",
                        "on_advisory_flake": "warn_only"
                    }
                }),
                created_at: timestamp.clone(),
                updated_at: timestamp.clone(),
                created_by: created_by.clone(),
                requires_portal: false,
                portal_approval_id: None,
                refs: vec![],
            },
        ]
    }

    /// Initialize with built-in schemas from SR-TEMPLATES
    fn build_schemas() -> Vec<TemplateSchema> {
        vec![
            // Work Surface templates
            TemplateSchema {
                type_key: "record.intake".to_string(),
                category: TemplateCategory::WorkSurface,
                name: "Intake".to_string(),
                description: "Structured specification of a work unit's objective, scope, deliverables, and constraints.".to_string(),
                source_ref: "SR-WORK-SURFACE §3".to_string(),
                required_fields: vec![
                    FieldSchema { name: "work_unit_id".to_string(), field_type: "string".to_string(), description: "Stable identifier".to_string(), example: Some("WU-2026-001".to_string()) },
                    FieldSchema { name: "title".to_string(), field_type: "string".to_string(), description: "Human-readable name".to_string(), example: Some("API Rate Limiting Analysis".to_string()) },
                    FieldSchema { name: "kind".to_string(), field_type: "string".to_string(), description: "Work kind taxonomy".to_string(), example: Some("research_memo".to_string()) },
                    FieldSchema { name: "objective".to_string(), field_type: "string".to_string(), description: "One-sentence goal".to_string(), example: None },
                    FieldSchema { name: "audience".to_string(), field_type: "string".to_string(), description: "Target readers/users".to_string(), example: None },
                    FieldSchema { name: "deliverables".to_string(), field_type: "array".to_string(), description: "Exact required outputs".to_string(), example: None },
                    FieldSchema { name: "constraints".to_string(), field_type: "array".to_string(), description: "Length, tone, required sections".to_string(), example: None },
                    FieldSchema { name: "definitions".to_string(), field_type: "object".to_string(), description: "Term-to-definition mapping".to_string(), example: None },
                    FieldSchema { name: "inputs".to_string(), field_type: "array".to_string(), description: "Provided context references".to_string(), example: None },
                    FieldSchema { name: "unknowns".to_string(), field_type: "array".to_string(), description: "Questions to resolve".to_string(), example: None },
                    FieldSchema { name: "completion_criteria".to_string(), field_type: "array".to_string(), description: "Human-facing acceptance criteria".to_string(), example: None },
                ],
                optional_fields: vec![
                    FieldSchema { name: "non_goals".to_string(), field_type: "array".to_string(), description: "Explicitly excluded scope".to_string(), example: None },
                ],
                requires_portal: false,
            },
            TemplateSchema {
                type_key: "config.procedure_template".to_string(),
                category: TemplateCategory::WorkSurface,
                name: "Procedure Template".to_string(),
                description: "Stage-gated workflow definition for candidate generation and oracle verification.".to_string(),
                source_ref: "SR-WORK-SURFACE §4".to_string(),
                required_fields: vec![
                    FieldSchema { name: "procedure_template_id".to_string(), field_type: "string".to_string(), description: "Stable id (format: proc:<NAME>)".to_string(), example: Some("proc:RESEARCH-MEMO".to_string()) },
                    FieldSchema { name: "kind".to_string(), field_type: "array".to_string(), description: "Applicable work kinds".to_string(), example: None },
                    FieldSchema { name: "stages".to_string(), field_type: "array".to_string(), description: "Stage definitions".to_string(), example: None },
                    FieldSchema { name: "terminal_stage_id".to_string(), field_type: "string".to_string(), description: "Final stage identifier".to_string(), example: Some("stage:FINAL".to_string()) },
                ],
                optional_fields: vec![
                    FieldSchema { name: "description".to_string(), field_type: "string".to_string(), description: "Human-readable description".to_string(), example: None },
                ],
                requires_portal: false,
            },
            TemplateSchema {
                type_key: "domain.work_surface".to_string(),
                category: TemplateCategory::WorkSurface,
                name: "Work Surface Instance".to_string(),
                description: "Runtime binding of a work unit to its intake, procedure, and oracle configuration.".to_string(),
                source_ref: "SR-WORK-SURFACE §5".to_string(),
                required_fields: vec![
                    FieldSchema { name: "work_unit_id".to_string(), field_type: "string".to_string(), description: "Work unit identifier".to_string(), example: None },
                    FieldSchema { name: "intake_ref".to_string(), field_type: "object".to_string(), description: "Content-addressed reference to Intake".to_string(), example: None },
                    FieldSchema { name: "procedure_template_ref".to_string(), field_type: "object".to_string(), description: "Content-addressed reference to Procedure Template".to_string(), example: None },
                    FieldSchema { name: "stage_id".to_string(), field_type: "string".to_string(), description: "Current stage being targeted".to_string(), example: None },
                    FieldSchema { name: "oracle_suites".to_string(), field_type: "array".to_string(), description: "Suite IDs + hashes".to_string(), example: None },
                ],
                optional_fields: vec![
                    FieldSchema { name: "params".to_string(), field_type: "object".to_string(), description: "Optional stage parameters and thresholds".to_string(), example: None },
                ],
                requires_portal: false,
            },
            // Verification Profile
            TemplateSchema {
                type_key: "verification_profile".to_string(),
                category: TemplateCategory::Verification,
                name: "Verification Profile".to_string(),
                description: "Map oracle suites to deliverable types with waiver policies.".to_string(),
                source_ref: "SR-DIRECTIVE §5.2".to_string(),
                required_fields: vec![
                    FieldSchema { name: "profile_id".to_string(), field_type: "string".to_string(), description: "Identifier (e.g., GOV-CORE, STRICT-CORE)".to_string(), example: Some("GOV-CORE".to_string()) },
                    FieldSchema { name: "profile_version".to_string(), field_type: "string".to_string(), description: "Version string".to_string(), example: Some("1.0.0".to_string()) },
                    FieldSchema { name: "description".to_string(), field_type: "string".to_string(), description: "Purpose".to_string(), example: None },
                    FieldSchema { name: "required_suites".to_string(), field_type: "array".to_string(), description: "Mandatory oracle suite IDs".to_string(), example: None },
                    FieldSchema { name: "verification_mode_default".to_string(), field_type: "string".to_string(), description: "STRICT".to_string(), example: Some("STRICT".to_string()) },
                ],
                optional_fields: vec![
                    FieldSchema { name: "advisory_suites".to_string(), field_type: "array".to_string(), description: "Optional oracle suite IDs".to_string(), example: None },
                    FieldSchema { name: "waiver_policy".to_string(), field_type: "object".to_string(), description: "Exception handling".to_string(), example: None },
                    FieldSchema { name: "waiver_eligible_failures".to_string(), field_type: "array".to_string(), description: "Failures that can be waived".to_string(), example: None },
                    FieldSchema { name: "non_waivable_integrity_conditions".to_string(), field_type: "array".to_string(), description: "Non-waivable conditions".to_string(), example: None },
                ],
                requires_portal: true,
            },
            // Semantic Set
            TemplateSchema {
                type_key: "config.semantic_set".to_string(),
                category: TemplateCategory::SemanticSets,
                name: "Semantic Set".to_string(),
                description: "Define meaning-matrix / manifold for semantic oracle evaluation.".to_string(),
                source_ref: "SR-SEMANTIC-ORACLE-SPEC §4".to_string(),
                required_fields: vec![
                    FieldSchema { name: "semantic_set_id".to_string(), field_type: "string".to_string(), description: "Unique identifier".to_string(), example: Some("semset:research-memo-v1".to_string()) },
                    FieldSchema { name: "name".to_string(), field_type: "string".to_string(), description: "Human-readable name".to_string(), example: None },
                    FieldSchema { name: "axes".to_string(), field_type: "array".to_string(), description: "Semantic axis definitions".to_string(), example: None },
                    FieldSchema { name: "constraints".to_string(), field_type: "array".to_string(), description: "Semantic constraint specifications".to_string(), example: None },
                    FieldSchema { name: "decision_rule".to_string(), field_type: "object".to_string(), description: "Pass/fail derivation rule".to_string(), example: None },
                ],
                optional_fields: vec![],
                requires_portal: true,
            },
            // Gating Policy
            TemplateSchema {
                type_key: "config.gating_policy".to_string(),
                category: TemplateCategory::Execution,
                name: "Gating Policy".to_string(),
                description: "Configure human judgment hooks per work unit.".to_string(),
                source_ref: "SR-SPEC §3.2.1.3".to_string(),
                required_fields: vec![
                    FieldSchema { name: "gating_mode".to_string(), field_type: "string".to_string(), description: "soft | hard | hybrid".to_string(), example: Some("hybrid".to_string()) },
                    FieldSchema { name: "hooks".to_string(), field_type: "array".to_string(), description: "Hook class configurations".to_string(), example: None },
                ],
                optional_fields: vec![
                    FieldSchema { name: "deterministic_triggers".to_string(), field_type: "array".to_string(), description: "Triggers that force hard gating".to_string(), example: None },
                ],
                requires_portal: false,
            },
            // Budget Configuration
            TemplateSchema {
                type_key: "budget_config".to_string(),
                category: TemplateCategory::Execution,
                name: "Budget Configuration".to_string(),
                description: "Resource constraints per work unit.".to_string(),
                source_ref: "SR-DIRECTIVE §4.1".to_string(),
                required_fields: vec![
                    FieldSchema { name: "max_iterations".to_string(), field_type: "integer".to_string(), description: "Maximum iterations per work unit".to_string(), example: Some("5".to_string()) },
                    FieldSchema { name: "max_oracle_runs".to_string(), field_type: "integer".to_string(), description: "Maximum oracle runs total".to_string(), example: Some("25".to_string()) },
                    FieldSchema { name: "max_wallclock_hours".to_string(), field_type: "integer".to_string(), description: "Maximum execution time".to_string(), example: Some("16".to_string()) },
                ],
                optional_fields: vec![],
                requires_portal: false,
            },
            // Exception templates
            TemplateSchema {
                type_key: "record.waiver".to_string(),
                category: TemplateCategory::Exceptions,
                name: "Waiver".to_string(),
                description: "Permission to proceed despite oracle FAIL.".to_string(),
                source_ref: "SR-SPEC §1.14".to_string(),
                required_fields: vec![
                    FieldSchema { name: "exception_id".to_string(), field_type: "string".to_string(), description: "Identifier (format: exc_<ULID>)".to_string(), example: None },
                    FieldSchema { name: "kind".to_string(), field_type: "string".to_string(), description: "WAIVER".to_string(), example: Some("WAIVER".to_string()) },
                    FieldSchema { name: "oracle_failure_ref".to_string(), field_type: "string".to_string(), description: "Specific oracle FAIL being waived".to_string(), example: None },
                    FieldSchema { name: "scope".to_string(), field_type: "string".to_string(), description: "per-candidate, per-loop, per-baseline, or time-boxed".to_string(), example: None },
                    FieldSchema { name: "risk_mitigation".to_string(), field_type: "string".to_string(), description: "Documented risk and mitigation".to_string(), example: None },
                    FieldSchema { name: "resolution_criteria".to_string(), field_type: "string".to_string(), description: "How to resolve/expire".to_string(), example: None },
                    FieldSchema { name: "expiry_date".to_string(), field_type: "string".to_string(), description: "Review/expiry date".to_string(), example: None },
                ],
                optional_fields: vec![],
                requires_portal: true,
            },
            TemplateSchema {
                type_key: "record.deviation".to_string(),
                category: TemplateCategory::Exceptions,
                name: "Deviation".to_string(),
                description: "Exception from a governed requirement.".to_string(),
                source_ref: "SR-CONTRACT §2.7".to_string(),
                required_fields: vec![
                    FieldSchema { name: "exception_id".to_string(), field_type: "string".to_string(), description: "Identifier".to_string(), example: None },
                    FieldSchema { name: "kind".to_string(), field_type: "string".to_string(), description: "DEVIATION".to_string(), example: Some("DEVIATION".to_string()) },
                    FieldSchema { name: "requirement_ref".to_string(), field_type: "string".to_string(), description: "Governed requirement being deviated from".to_string(), example: None },
                    FieldSchema { name: "scope".to_string(), field_type: "string".to_string(), description: "Bounded scope".to_string(), example: None },
                    FieldSchema { name: "justification".to_string(), field_type: "string".to_string(), description: "Why deviation is necessary".to_string(), example: None },
                    FieldSchema { name: "risk_mitigation".to_string(), field_type: "string".to_string(), description: "Risk and mitigation".to_string(), example: None },
                    FieldSchema { name: "resolution_criteria".to_string(), field_type: "string".to_string(), description: "How to resolve".to_string(), example: None },
                ],
                optional_fields: vec![],
                requires_portal: true,
            },
            TemplateSchema {
                type_key: "record.deferral".to_string(),
                category: TemplateCategory::Exceptions,
                name: "Deferral".to_string(),
                description: "Binding postponement of work or requirement.".to_string(),
                source_ref: "SR-CONTRACT §2.7".to_string(),
                required_fields: vec![
                    FieldSchema { name: "exception_id".to_string(), field_type: "string".to_string(), description: "Identifier".to_string(), example: None },
                    FieldSchema { name: "kind".to_string(), field_type: "string".to_string(), description: "DEFERRAL".to_string(), example: Some("DEFERRAL".to_string()) },
                    FieldSchema { name: "subject_ref".to_string(), field_type: "string".to_string(), description: "What is being deferred".to_string(), example: None },
                    FieldSchema { name: "scope".to_string(), field_type: "string".to_string(), description: "Bounded scope".to_string(), example: None },
                    FieldSchema { name: "justification".to_string(), field_type: "string".to_string(), description: "Why deferral is necessary".to_string(), example: None },
                    FieldSchema { name: "target_date".to_string(), field_type: "string".to_string(), description: "When deferred work should be addressed".to_string(), example: None },
                ],
                optional_fields: vec![],
                requires_portal: true,
            },
            // Freeze Record
            TemplateSchema {
                type_key: "record.freeze".to_string(),
                category: TemplateCategory::Release,
                name: "Freeze Record".to_string(),
                description: "Baseline snapshot for declaring Shippable.".to_string(),
                source_ref: "SR-SPEC §1.12".to_string(),
                required_fields: vec![
                    FieldSchema { name: "freeze_id".to_string(), field_type: "string".to_string(), description: "Identifier (format: freeze_<ULID>)".to_string(), example: None },
                    FieldSchema { name: "baseline_id".to_string(), field_type: "string".to_string(), description: "Human-meaningful name".to_string(), example: None },
                    FieldSchema { name: "candidate_id".to_string(), field_type: "string".to_string(), description: "Released candidate".to_string(), example: None },
                    FieldSchema { name: "verification".to_string(), field_type: "object".to_string(), description: "Mode, suite id/hash, evidence refs, waivers".to_string(), example: None },
                    FieldSchema { name: "release_approval_id".to_string(), field_type: "string".to_string(), description: "Release Portal approval reference".to_string(), example: None },
                    FieldSchema { name: "artifact_manifest".to_string(), field_type: "array".to_string(), description: "Governed artifacts in force".to_string(), example: None },
                ],
                optional_fields: vec![
                    FieldSchema { name: "active_exceptions".to_string(), field_type: "array".to_string(), description: "In-scope exceptions".to_string(), example: None },
                ],
                requires_portal: true,
            },
            // Evidence Bundle
            TemplateSchema {
                type_key: "domain.evidence_bundle".to_string(),
                category: TemplateCategory::Evidence,
                name: "Evidence Bundle Manifest".to_string(),
                description: "Structured oracle output for verification.".to_string(),
                source_ref: "SR-SPEC §1.9.1".to_string(),
                required_fields: vec![
                    FieldSchema { name: "artifact_type".to_string(), field_type: "string".to_string(), description: "evidence.gate_packet".to_string(), example: Some("evidence.gate_packet".to_string()) },
                    FieldSchema { name: "candidate_id".to_string(), field_type: "string".to_string(), description: "Candidate being evaluated".to_string(), example: None },
                    FieldSchema { name: "run_id".to_string(), field_type: "string".to_string(), description: "Run identifier".to_string(), example: None },
                    FieldSchema { name: "oracle_suite_id".to_string(), field_type: "string".to_string(), description: "Suite used".to_string(), example: None },
                    FieldSchema { name: "oracle_suite_hash".to_string(), field_type: "string".to_string(), description: "Suite hash".to_string(), example: None },
                    FieldSchema { name: "results".to_string(), field_type: "array".to_string(), description: "Per-oracle results".to_string(), example: None },
                    FieldSchema { name: "produced_at".to_string(), field_type: "string".to_string(), description: "ISO 8601 timestamp".to_string(), example: None },
                ],
                optional_fields: vec![
                    FieldSchema { name: "procedure_template_id".to_string(), field_type: "string".to_string(), description: "Procedure (semantic work)".to_string(), example: None },
                    FieldSchema { name: "stage_id".to_string(), field_type: "string".to_string(), description: "Stage (semantic work)".to_string(), example: None },
                ],
                requires_portal: false,
            },
            // Gate Definition (reference only)
            TemplateSchema {
                type_key: "gate_definition".to_string(),
                category: TemplateCategory::Gates,
                name: "Gate Definition".to_string(),
                description: "Define enforcement checkpoints with evidence requirements.".to_string(),
                source_ref: "SR-DIRECTIVE.KIT-GATE-REGISTRY.md".to_string(),
                required_fields: vec![
                    FieldSchema { name: "gate_id".to_string(), field_type: "string".to_string(), description: "Identifier (format: G-<NAME>)".to_string(), example: Some("G-VERIFIED-STRICT".to_string()) },
                    FieldSchema { name: "decision_purpose".to_string(), field_type: "string".to_string(), description: "What this gate decides".to_string(), example: None },
                    FieldSchema { name: "membranes_enforced".to_string(), field_type: "array".to_string(), description: "Trust boundaries enforced".to_string(), example: None },
                    FieldSchema { name: "enforcement_mechanism".to_string(), field_type: "string".to_string(), description: "How gate is enforced".to_string(), example: None },
                    FieldSchema { name: "failure_conditions".to_string(), field_type: "array".to_string(), description: "Conditions that fail the gate".to_string(), example: None },
                ],
                optional_fields: vec![
                    FieldSchema { name: "stop_triggers".to_string(), field_type: "array".to_string(), description: "Stop triggers this gate can fire".to_string(), example: None },
                    FieldSchema { name: "relief_valves".to_string(), field_type: "array".to_string(), description: "How failures can be relieved".to_string(), example: None },
                    FieldSchema { name: "routing_portal".to_string(), field_type: "string".to_string(), description: "Portal for relief routing".to_string(), example: None },
                ],
                requires_portal: false,
            },
            // Portal Playbook (reference only)
            TemplateSchema {
                type_key: "portal_playbook".to_string(),
                category: TemplateCategory::Portals,
                name: "Portal Playbook".to_string(),
                description: "Define human judgment portals with request types and procedures.".to_string(),
                source_ref: "SR-DIRECTIVE.KIT-PORTAL-PLAYBOOKS.md".to_string(),
                required_fields: vec![
                    FieldSchema { name: "portal_id".to_string(), field_type: "string".to_string(), description: "Stable identifier".to_string(), example: Some("HumanAuthorityExceptionProcess".to_string()) },
                    FieldSchema { name: "portal_kind".to_string(), field_type: "string".to_string(), description: "Type (exception_approval, governance_approval, release_approval)".to_string(), example: None },
                    FieldSchema { name: "request_types".to_string(), field_type: "array".to_string(), description: "Allowed request types with descriptions".to_string(), example: None },
                    FieldSchema { name: "actor_rules".to_string(), field_type: "object".to_string(), description: "Who can submit/approve (HUMAN only)".to_string(), example: None },
                    FieldSchema { name: "preconditions".to_string(), field_type: "array".to_string(), description: "Required conditions before acceptance".to_string(), example: None },
                ],
                optional_fields: vec![
                    FieldSchema { name: "evidence_review_checklist".to_string(), field_type: "array".to_string(), description: "Mandatory reviewer confirmations".to_string(), example: None },
                    FieldSchema { name: "decision_procedure".to_string(), field_type: "object".to_string(), description: "How to make decisions per request type".to_string(), example: None },
                    FieldSchema { name: "outputs".to_string(), field_type: "array".to_string(), description: "Binding records emitted".to_string(), example: None },
                ],
                requires_portal: false,
            },
        ]
    }

    pub async fn list_templates(&self, category: Option<TemplateCategory>) -> Vec<TemplateInstance> {
        let templates = self.templates.read().await;
        match category {
            Some(cat) => templates
                .iter()
                .filter(|t| t.category == cat)
                .cloned()
                .collect(),
            None => templates.clone(),
        }
    }

    pub async fn get_template(&self, template_id: &str) -> Option<TemplateInstance> {
        let templates = self.templates.read().await;
        templates.iter().find(|t| t.id == template_id).cloned()
    }

    pub fn get_schemas(&self) -> &[TemplateSchema] {
        &self.schemas
    }

    pub fn get_schema(&self, type_key: &str) -> Option<&TemplateSchema> {
        self.schemas.iter().find(|s| s.type_key == type_key)
    }

    pub fn get_schemas_by_category(&self, category: TemplateCategory) -> Vec<&TemplateSchema> {
        self.schemas
            .iter()
            .filter(|s| s.category == category)
            .collect()
    }

    pub async fn create_template(&self, template: TemplateInstance) -> TemplateInstance {
        let mut templates = self.templates.write().await;
        templates.push(template.clone());
        template
    }

    pub async fn update_template(&self, template_id: &str, name: Option<String>, content: Option<serde_json::Value>) -> Option<TemplateInstance> {
        let mut templates = self.templates.write().await;
        let template = templates.iter_mut().find(|t| t.id == template_id)?;

        // Don't allow updating reference templates
        if template.status == "reference" {
            return None;
        }

        if let Some(new_name) = name {
            template.name = new_name;
        }

        if let Some(new_content) = content {
            template.content_hash = compute_content_hash(&new_content);
            template.content = new_content;
        }

        template.updated_at = chrono::Utc::now().to_rfc3339();

        Some(template.clone())
    }
}

impl Default for TemplateRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Request/Response Types
// ============================================================================

/// Query parameters for listing templates
#[derive(Debug, Deserialize)]
pub struct ListTemplatesParams {
    #[serde(default)]
    pub category: Option<TemplateCategory>,
    #[serde(default)]
    pub type_key: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

/// Response for a single template (summary)
#[derive(Debug, Clone, Serialize)]
pub struct TemplateSummary {
    pub id: String,
    pub type_key: String,
    pub name: String,
    pub category: TemplateCategory,
    pub status: String,
    pub content_hash: String,
    pub created_at: String,
    pub updated_at: String,
    pub requires_portal: bool,
}

/// Response for listing templates
#[derive(Debug, Serialize)]
pub struct ListTemplatesResponse {
    pub templates: Vec<TemplateSummary>,
    pub total: usize,
    pub category_counts: BTreeMap<String, usize>,
}

/// Detailed template response
#[derive(Debug, Serialize)]
pub struct TemplateDetailResponse {
    pub id: String,
    pub type_key: String,
    pub name: String,
    pub category: TemplateCategory,
    pub category_label: String,
    pub status: String,
    pub content_hash: String,
    pub content: serde_json::Value,
    pub schema: TemplateSchemaResponse,
    pub created_at: String,
    pub updated_at: String,
    pub created_by: String,
    pub requires_portal: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_approval_id: Option<String>,
    pub refs: Vec<TemplateRef>,
}

/// Template reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateRef {
    pub rel: String,
    pub to: String,
}

/// A template instance (user-created artifact)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateInstance {
    pub id: String,
    pub type_key: String,
    pub name: String,
    pub category: TemplateCategory,
    pub status: String,
    pub content_hash: String,
    pub content: serde_json::Value,
    pub created_at: String,
    pub updated_at: String,
    pub created_by: String,
    pub requires_portal: bool,
    #[serde(default)]
    pub portal_approval_id: Option<String>,
    #[serde(default)]
    pub refs: Vec<TemplateRef>,
}

/// Schema definition for a template type
#[derive(Debug, Clone, Serialize)]
pub struct TemplateSchema {
    pub type_key: String,
    pub category: TemplateCategory,
    pub name: String,
    pub description: String,
    pub source_ref: String,
    pub required_fields: Vec<FieldSchema>,
    pub optional_fields: Vec<FieldSchema>,
    pub requires_portal: bool,
}

/// Schema response (for API)
#[derive(Debug, Clone, Serialize)]
pub struct TemplateSchemaResponse {
    pub type_key: String,
    pub name: String,
    pub description: String,
    pub source_ref: String,
    pub required_fields: Vec<FieldSchema>,
    pub optional_fields: Vec<FieldSchema>,
    pub requires_portal: bool,
}

/// Field schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSchema {
    pub name: String,
    pub field_type: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
}

/// Response for listing schemas
#[derive(Debug, Serialize)]
pub struct ListSchemasResponse {
    pub schemas: Vec<TemplateSchemaResponse>,
    pub total: usize,
    pub categories: Vec<CategoryInfo>,
}

/// Category information
#[derive(Debug, Serialize)]
pub struct CategoryInfo {
    pub id: TemplateCategory,
    pub label: String,
    pub type_keys: Vec<String>,
    pub requires_portal: bool,
    pub is_user_instantiable: bool,
    pub schema_count: usize,
}

/// Request to create a new template instance
#[derive(Debug, Deserialize)]
pub struct CreateTemplateRequest {
    pub type_key: String,
    pub name: String,
    pub content: serde_json::Value,
    #[serde(default)]
    pub refs: Vec<TemplateRefRequest>,
}

/// Template reference in request
#[derive(Debug, Deserialize)]
pub struct TemplateRefRequest {
    pub rel: String,
    pub to: String,
}

/// Response for template creation
#[derive(Debug, Serialize)]
pub struct CreateTemplateResponse {
    pub id: String,
    pub type_key: String,
    pub content_hash: String,
    pub status: String,
    pub requires_portal: bool,
}

/// Request body for updating a template
#[derive(Debug, Deserialize)]
pub struct UpdateTemplateRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub content: Option<serde_json::Value>,
}

/// Response for template update
#[derive(Debug, Serialize)]
pub struct UpdateTemplateResponse {
    pub id: String,
    pub name: String,
    pub content_hash: String,
    pub updated_at: String,
}

// ============================================================================
// Handlers
// ============================================================================

/// List templates by category
///
/// GET /api/v1/templates
#[instrument(skip(state, _user), fields(user_id = %_user.actor_id))]
pub async fn list_templates(
    State(state): State<TemplateRegistryState>,
    Query(params): Query<ListTemplatesParams>,
    _user: AuthenticatedUser,
) -> ApiResult<Json<ListTemplatesResponse>> {
    let templates = state.registry.list_templates(params.category).await;

    // Filter by type_key if specified
    let filtered: Vec<_> = if let Some(type_key) = &params.type_key {
        templates
            .iter()
            .filter(|t| &t.type_key == type_key)
            .cloned()
            .collect()
    } else {
        templates
    };

    // Filter by status if specified
    let filtered: Vec<_> = if let Some(status) = &params.status {
        filtered
            .into_iter()
            .filter(|t| &t.status == status)
            .collect()
    } else {
        filtered
    };

    // Build category counts
    let mut category_counts: BTreeMap<String, usize> = BTreeMap::new();
    for cat in TemplateCategory::all() {
        let count = filtered.iter().filter(|t| t.category == cat).count();
        category_counts.insert(format!("{:?}", cat).to_lowercase().replace("_", "-"), count);
    }

    let summaries: Vec<TemplateSummary> = filtered
        .iter()
        .map(|t| TemplateSummary {
            id: t.id.clone(),
            type_key: t.type_key.clone(),
            name: t.name.clone(),
            category: t.category,
            status: t.status.clone(),
            content_hash: t.content_hash.clone(),
            created_at: t.created_at.clone(),
            updated_at: t.updated_at.clone(),
            requires_portal: t.requires_portal,
        })
        .collect();

    let total = summaries.len();

    Ok(Json(ListTemplatesResponse {
        templates: summaries,
        total,
        category_counts,
    }))
}

/// Get a single template by ID
///
/// GET /api/v1/templates/:template_id
#[instrument(skip(state, _user), fields(user_id = %_user.actor_id, template_id = %template_id))]
pub async fn get_template(
    State(state): State<TemplateRegistryState>,
    Path(template_id): Path<String>,
    _user: AuthenticatedUser,
) -> ApiResult<Json<TemplateDetailResponse>> {
    let template = state
        .registry
        .get_template(&template_id)
        .await
        .ok_or_else(|| ApiError::NotFound {
            resource: "Template".to_string(),
            id: template_id.clone(),
        })?;

    // Get schema for this type
    let schema = state
        .registry
        .get_schema(&template.type_key)
        .ok_or_else(|| ApiError::NotFound {
            resource: "TemplateSchema".to_string(),
            id: template.type_key.clone(),
        })?;

    Ok(Json(TemplateDetailResponse {
        id: template.id,
        type_key: template.type_key,
        name: template.name,
        category: template.category,
        category_label: template.category.label().to_string(),
        status: template.status,
        content_hash: template.content_hash,
        content: template.content,
        schema: TemplateSchemaResponse {
            type_key: schema.type_key.clone(),
            name: schema.name.clone(),
            description: schema.description.clone(),
            source_ref: schema.source_ref.clone(),
            required_fields: schema.required_fields.clone(),
            optional_fields: schema.optional_fields.clone(),
            requires_portal: schema.requires_portal,
        },
        created_at: template.created_at,
        updated_at: template.updated_at,
        created_by: template.created_by,
        requires_portal: template.requires_portal,
        portal_approval_id: template.portal_approval_id,
        refs: template.refs,
    }))
}

/// List all template schemas
///
/// GET /api/v1/templates/schemas
#[instrument(skip(state, _user), fields(user_id = %_user.actor_id))]
pub async fn list_schemas(
    State(state): State<TemplateRegistryState>,
    _user: AuthenticatedUser,
) -> ApiResult<Json<ListSchemasResponse>> {
    let schemas = state.registry.get_schemas();

    let schema_responses: Vec<TemplateSchemaResponse> = schemas
        .iter()
        .map(|s| TemplateSchemaResponse {
            type_key: s.type_key.clone(),
            name: s.name.clone(),
            description: s.description.clone(),
            source_ref: s.source_ref.clone(),
            required_fields: s.required_fields.clone(),
            optional_fields: s.optional_fields.clone(),
            requires_portal: s.requires_portal,
        })
        .collect();

    let categories: Vec<CategoryInfo> = TemplateCategory::all()
        .into_iter()
        .map(|cat| {
            let cat_schemas = state.registry.get_schemas_by_category(cat);
            CategoryInfo {
                id: cat,
                label: cat.label().to_string(),
                type_keys: cat.type_keys().iter().map(|s| s.to_string()).collect(),
                requires_portal: cat.requires_portal(),
                is_user_instantiable: cat.is_user_instantiable(),
                schema_count: cat_schemas.len(),
            }
        })
        .collect();

    Ok(Json(ListSchemasResponse {
        total: schema_responses.len(),
        schemas: schema_responses,
        categories,
    }))
}

/// Get a single template schema by type key
///
/// GET /api/v1/templates/schemas/:type_key
#[instrument(skip(state, _user), fields(user_id = %_user.actor_id, type_key = %type_key))]
pub async fn get_schema(
    State(state): State<TemplateRegistryState>,
    Path(type_key): Path<String>,
    _user: AuthenticatedUser,
) -> ApiResult<Json<TemplateSchemaResponse>> {
    let schema = state
        .registry
        .get_schema(&type_key)
        .ok_or_else(|| ApiError::NotFound {
            resource: "TemplateSchema".to_string(),
            id: type_key.clone(),
        })?;

    Ok(Json(TemplateSchemaResponse {
        type_key: schema.type_key.clone(),
        name: schema.name.clone(),
        description: schema.description.clone(),
        source_ref: schema.source_ref.clone(),
        required_fields: schema.required_fields.clone(),
        optional_fields: schema.optional_fields.clone(),
        requires_portal: schema.requires_portal,
    }))
}

/// Create a new template instance
///
/// POST /api/v1/templates
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn create_template(
    State(state): State<TemplateRegistryState>,
    user: AuthenticatedUser,
    Json(body): Json<CreateTemplateRequest>,
) -> ApiResult<Json<CreateTemplateResponse>> {
    // Validate type_key exists
    let schema = state
        .registry
        .get_schema(&body.type_key)
        .ok_or_else(|| ApiError::BadRequest {
            message: format!("Unknown template type_key: {}", body.type_key),
        })?;

    // Check if category allows user instantiation
    if !schema.category.is_user_instantiable() {
        return Err(ApiError::BadRequest {
            message: format!(
                "Template type '{}' is not user-instantiable (category: {})",
                body.type_key,
                schema.category.label()
            ),
        });
    }

    // Compute content hash
    let content_hash = compute_content_hash(&body.content);

    // Generate ID using random bytes
    let random_bytes: [u8; 16] = rand::random();
    let id = format!("tmpl_{}", hex::encode(random_bytes));

    let now = chrono::Utc::now().to_rfc3339();

    // Status is 'draft' unless requires portal approval
    let status = if schema.requires_portal {
        "pending_approval".to_string()
    } else {
        "draft".to_string()
    };

    let template = TemplateInstance {
        id: id.clone(),
        type_key: body.type_key.clone(),
        name: body.name,
        category: schema.category,
        status: status.clone(),
        content_hash: content_hash.clone(),
        content: body.content,
        created_at: now.clone(),
        updated_at: now,
        created_by: user.actor_id.clone(),
        requires_portal: schema.requires_portal,
        portal_approval_id: None,
        refs: body
            .refs
            .into_iter()
            .map(|r| TemplateRef {
                rel: r.rel,
                to: r.to,
            })
            .collect(),
    };

    info!(
        template_id = %id,
        type_key = %body.type_key,
        content_hash = %content_hash,
        requires_portal = schema.requires_portal,
        created_by = %user.actor_id,
        "Creating new template instance"
    );

    state.registry.create_template(template).await;

    Ok(Json(CreateTemplateResponse {
        id,
        type_key: body.type_key,
        content_hash,
        status,
        requires_portal: schema.requires_portal,
    }))
}

/// Update an existing template instance
///
/// PUT /api/v1/templates/:template_id
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id, template_id = %template_id))]
pub async fn update_template(
    State(state): State<TemplateRegistryState>,
    Path(template_id): Path<String>,
    user: AuthenticatedUser,
    Json(body): Json<UpdateTemplateRequest>,
) -> ApiResult<Json<UpdateTemplateResponse>> {
    // Check template exists first
    let existing = state
        .registry
        .get_template(&template_id)
        .await
        .ok_or_else(|| ApiError::NotFound {
            resource: "Template".to_string(),
            id: template_id.clone(),
        })?;

    // Don't allow updating reference templates
    if existing.status == "reference" {
        return Err(ApiError::BadRequest {
            message: "Cannot update reference templates. Clone the template first.".to_string(),
        });
    }

    info!(
        template_id = %template_id,
        has_name_update = body.name.is_some(),
        has_content_update = body.content.is_some(),
        updated_by = %user.actor_id,
        "Updating template instance"
    );

    let updated = state
        .registry
        .update_template(&template_id, body.name, body.content)
        .await
        .ok_or_else(|| ApiError::NotFound {
            resource: "Template".to_string(),
            id: template_id.clone(),
        })?;

    Ok(Json(UpdateTemplateResponse {
        id: updated.id,
        name: updated.name,
        content_hash: updated.content_hash,
        updated_at: updated.updated_at,
    }))
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Compute deterministic content hash
fn compute_content_hash(content: &serde_json::Value) -> String {
    use sha2::{Digest, Sha256};

    let content_str = serde_json::to_string(content).unwrap_or_default();
    let mut hasher = Sha256::new();
    hasher.update(content_str.as_bytes());
    format!("sha256:{}", hex::encode(hasher.finalize()))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_category_all() {
        let categories = TemplateCategory::all();
        assert_eq!(categories.len(), 11);
    }

    #[test]
    fn test_template_category_type_keys() {
        let work_surface_keys = TemplateCategory::WorkSurface.type_keys();
        assert!(work_surface_keys.contains(&"record.intake"));
        assert!(work_surface_keys.contains(&"config.procedure_template"));
    }

    #[test]
    fn test_content_hash_determinism() {
        let content = serde_json::json!({
            "name": "test",
            "value": 123
        });

        let hash1 = compute_content_hash(&content);
        let hash2 = compute_content_hash(&content);

        assert_eq!(hash1, hash2);
        assert!(hash1.starts_with("sha256:"));
    }

    #[test]
    fn test_registry_schemas_loaded() {
        let registry = TemplateRegistry::new();
        let schemas = registry.get_schemas();

        // Should have schemas for key template types
        assert!(schemas.iter().any(|s| s.type_key == "record.intake"));
        assert!(schemas.iter().any(|s| s.type_key == "config.procedure_template"));
        assert!(schemas.iter().any(|s| s.type_key == "record.waiver"));
    }

    #[test]
    fn test_portal_requirements() {
        assert!(!TemplateCategory::WorkSurface.requires_portal());
        assert!(TemplateCategory::Oracle.requires_portal());
        assert!(TemplateCategory::Exceptions.requires_portal());
        assert!(!TemplateCategory::Execution.requires_portal());
    }

    #[tokio::test]
    async fn test_starter_templates_loaded() {
        let registry = TemplateRegistry::new();
        let templates = registry.list_templates(None).await;

        // Should have 11 starter templates
        assert_eq!(templates.len(), 11);

        // All should have status "reference"
        assert!(templates.iter().all(|t| t.status == "reference"));

        // All should be created by "system"
        assert!(templates.iter().all(|t| t.created_by == "system"));

        // Check specific templates exist
        assert!(templates.iter().any(|t| t.id == "tmpl_starter_intake"));
        assert!(templates.iter().any(|t| t.id == "tmpl_starter_procedure"));
        assert!(templates.iter().any(|t| t.id == "tmpl_starter_waiver"));
        assert!(templates.iter().any(|t| t.id == "tmpl_starter_oracle_suite"));
    }

    #[tokio::test]
    async fn test_starter_templates_by_category() {
        let registry = TemplateRegistry::new();

        // Work Surface should have 3 starter templates
        let ws_templates = registry.list_templates(Some(TemplateCategory::WorkSurface)).await;
        assert_eq!(ws_templates.len(), 3);

        // Execution should have 2 starter templates
        let exec_templates = registry.list_templates(Some(TemplateCategory::Execution)).await;
        assert_eq!(exec_templates.len(), 2);

        // Exceptions should have 3 starter templates
        let exc_templates = registry.list_templates(Some(TemplateCategory::Exceptions)).await;
        assert_eq!(exc_templates.len(), 3);
    }

    #[test]
    fn test_category_serialization() {
        // Verify that TemplateCategory serializes to kebab-case
        let work_surface = serde_json::to_string(&TemplateCategory::WorkSurface).unwrap();
        assert_eq!(work_surface, "\"work-surface\"");

        let semantic_sets = serde_json::to_string(&TemplateCategory::SemanticSets).unwrap();
        assert_eq!(semantic_sets, "\"semantic-sets\"");

        let oracle = serde_json::to_string(&TemplateCategory::Oracle).unwrap();
        assert_eq!(oracle, "\"oracle\"");
    }

    #[test]
    fn test_template_summary_serialization() {
        let summary = TemplateSummary {
            id: "test".to_string(),
            type_key: "record.intake".to_string(),
            name: "Test".to_string(),
            category: TemplateCategory::WorkSurface,
            status: "reference".to_string(),
            content_hash: "sha256:test".to_string(),
            created_at: "2026-01-15T00:00:00Z".to_string(),
            updated_at: "2026-01-15T00:00:00Z".to_string(),
            requires_portal: false,
        };

        let json = serde_json::to_value(&summary).unwrap();
        assert_eq!(json["category"], "work-surface");
        assert_eq!(json["status"], "reference");
    }
}

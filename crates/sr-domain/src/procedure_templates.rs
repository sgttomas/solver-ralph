//! Procedure Template Definitions per SR-PROCEDURE-KIT
//!
//! This module provides canonical procedure template instances for semantic work.
//! Templates defined here are governed configuration artifacts that SHOULD be
//! reusable across work units of the same kind.
//!
//! Branch 0 MVP requires at minimum the Problem Statement Ingestion template.

use crate::entities::ContentHash;
use crate::work_surface::{
    GateRule, OracleSuiteBinding, ProcedureTemplate, ProcedureTemplateId, RequiredOutput, Stage,
    StageId, TransitionTarget, WorkKind,
};
use sha2::{Digest, Sha256};

// ============================================================================
// Branch 0: Problem Statement Ingestion Template
// ============================================================================

/// Problem Statement Ingestion Procedure Template
///
/// Per SR-PLAN §4.1 Branch 0, this template provides the MVP spine for
/// semantic work: Problem Statement → Typed Intake Baseline.
///
/// Stages:
/// - INGEST: Parse and normalize the problem statement
/// - VALIDATE: Run admissibility oracle suite
/// - ACCEPT: Portal approval for intake baseline (human authority boundary)
///
/// This template is intentionally minimal for Branch 0 demonstration.
pub fn problem_statement_ingestion_template() -> ProcedureTemplate {
    ProcedureTemplate {
        artifact_type: "config.procedure_template".to_string(),
        artifact_version: "v1".to_string(),
        procedure_template_id: ProcedureTemplateId::new("PROBLEM-STATEMENT-INGESTION"),
        kind: vec![WorkKind::IntakeProcessing],
        name: Some("Problem Statement Ingestion".to_string()),
        description: Some(
            "Stage-gated procedure for ingesting a problem statement and producing a \
             validated, typed intake baseline. This is the Branch 0 MVP template."
                .to_string(),
        ),
        stages: vec![
            // Stage 0: INGEST
            Stage {
                stage_id: StageId::new("INGEST"),
                stage_name: "Ingest Problem Statement".to_string(),
                purpose: "Parse the raw problem statement, extract key elements, and produce \
                          a structured intake draft with objective, scope, constraints, \
                          definitions, and required deliverables."
                    .to_string(),
                required_outputs: vec![
                    RequiredOutput {
                        path: "artifacts/intake/draft_intake.yaml".to_string(),
                        role: Some("candidate".to_string()),
                        media_type: Some("application/yaml".to_string()),
                    },
                    RequiredOutput {
                        path: "artifacts/intake/extraction_log.json".to_string(),
                        role: Some("evidence".to_string()),
                        media_type: Some("application/json".to_string()),
                    },
                ],
                steps: vec![],
                required_oracle_suites: vec!["suite:SR-SUITE-GOV".to_string()],
                gate_rule: GateRule::AllRequiredOraclesPass,
                transition_on_pass: TransitionTarget::Stage(StageId::new("VALIDATE")),
                requires_portal: false,
                portal_id: None,
                requires_approval: false,
            },
            // Stage 1: VALIDATE
            Stage {
                stage_id: StageId::new("VALIDATE"),
                stage_name: "Validate Intake Admissibility".to_string(),
                purpose: "Run the intake admissibility oracle suite to verify schema \
                          compliance, traceability coverage, contradiction detection, \
                          ambiguity inventory, privacy scan, and term-map alignment."
                    .to_string(),
                required_outputs: vec![
                    RequiredOutput {
                        path: "artifacts/validation/admissibility_report.json".to_string(),
                        role: Some("evidence".to_string()),
                        media_type: Some("application/json".to_string()),
                    },
                    RequiredOutput {
                        path: "artifacts/validation/term_map.json".to_string(),
                        role: Some("context".to_string()),
                        media_type: Some("application/json".to_string()),
                    },
                ],
                steps: vec![],
                required_oracle_suites: vec![
                    "suite:SR-SUITE-GOV".to_string(),
                    "oracle.suite.intake_admissibility.v1".to_string(),
                ],
                gate_rule: GateRule::AllRequiredOraclesPass,
                transition_on_pass: TransitionTarget::Stage(StageId::new("ACCEPT")),
                requires_portal: false,
                portal_id: None,
                requires_approval: false,
            },
            // Stage 2: ACCEPT (Portal boundary)
            Stage {
                stage_id: StageId::new("ACCEPT"),
                stage_name: "Intake Acceptance Portal".to_string(),
                purpose: "Human authority boundary for accepting the validated intake. \
                          Portal approval produces a freeze baseline for the accepted \
                          intake bundle. This is a trust boundary per SR-CONTRACT."
                    .to_string(),
                required_outputs: vec![
                    RequiredOutput {
                        path: "candidate/intake.yaml".to_string(),
                        role: Some("primary".to_string()),
                        media_type: Some("application/yaml".to_string()),
                    },
                    RequiredOutput {
                        path: "evidence/gate_packet.json".to_string(),
                        role: Some("evidence".to_string()),
                        media_type: Some("application/json".to_string()),
                    },
                ],
                steps: vec![],
                required_oracle_suites: vec!["suite:SR-SUITE-REFS".to_string()],
                gate_rule: GateRule::PortalApprovalRequired,
                transition_on_pass: TransitionTarget::Terminal,
                requires_portal: true,
                portal_id: Some("IntakeAcceptancePortal".to_string()),
                requires_approval: true, // Trust boundary per SR-CONTRACT C-TB-3
            },
        ],
        terminal_stage_id: StageId::new("ACCEPT"),
        initial_stage_id: Some(StageId::new("INGEST")),
        content_hash: None,
        version: Some("1.0.0".to_string()),
    }
}

/// Generic Knowledge Work Procedure Template
///
/// Per SR-PROCEDURE-KIT §2, this baseline template provides a consistent stage
/// structure across many kinds of knowledge work. It allows different semantic
/// manifolds per stage.
///
/// Stages:
/// - FRAME: Restate objective, extract constraints/definitions
/// - OPTIONS: Generate multiple candidate approaches
/// - DRAFT: Produce candidate deliverables with traceability
/// - SEMANTIC_EVAL: Evaluate against stage manifold/meaning-matrix
/// - FINAL: Package final candidate with evidence bundle
pub fn generic_knowledge_work_template() -> ProcedureTemplate {
    ProcedureTemplate {
        artifact_type: "config.procedure_template".to_string(),
        artifact_version: "v1".to_string(),
        procedure_template_id: ProcedureTemplateId::new("GENERIC-KNOWLEDGE-WORK"),
        kind: vec![
            WorkKind::ResearchMemo,
            WorkKind::DecisionRecord,
            WorkKind::AnalysisReport,
            WorkKind::TechnicalSpec,
        ],
        name: Some("Generic Knowledge Work".to_string()),
        description: Some(
            "Baseline template for general semantic knowledge work. Suitable for \
             research memos, decision records, analysis reports, and technical specs."
                .to_string(),
        ),
        stages: vec![
            // stage:FRAME
            Stage {
                stage_id: StageId::new("FRAME"),
                stage_name: "Frame the problem".to_string(),
                purpose: "Restate objective, audience, non-goals; extract constraints \
                          and definitions into machine-checkable artifacts."
                    .to_string(),
                required_outputs: vec![
                    RequiredOutput {
                        path: "artifacts/context/frame.md".to_string(),
                        role: Some("context".to_string()),
                        media_type: Some("text/markdown".to_string()),
                    },
                    RequiredOutput {
                        path: "artifacts/context/constraints.json".to_string(),
                        role: Some("context".to_string()),
                        media_type: Some("application/json".to_string()),
                    },
                    RequiredOutput {
                        path: "artifacts/context/definitions.json".to_string(),
                        role: Some("context".to_string()),
                        media_type: Some("application/json".to_string()),
                    },
                ],
                steps: vec![],
                required_oracle_suites: vec!["suite:SR-SUITE-STRUCTURE".to_string()],
                gate_rule: GateRule::AllRequiredOraclesPass,
                transition_on_pass: TransitionTarget::Stage(StageId::new("OPTIONS")),
                requires_portal: false,
                portal_id: None,
                requires_approval: false,
            },
            // stage:OPTIONS
            Stage {
                stage_id: StageId::new("OPTIONS"),
                stage_name: "Generate options".to_string(),
                purpose: "Generate multiple candidate approaches/outlines before drafting."
                    .to_string(),
                required_outputs: vec![
                    RequiredOutput {
                        path: "artifacts/candidates/option_A.md".to_string(),
                        role: Some("candidate".to_string()),
                        media_type: Some("text/markdown".to_string()),
                    },
                    RequiredOutput {
                        path: "artifacts/candidates/option_B.md".to_string(),
                        role: Some("candidate".to_string()),
                        media_type: Some("text/markdown".to_string()),
                    },
                    RequiredOutput {
                        path: "artifacts/candidates/selection.md".to_string(),
                        role: Some("context".to_string()),
                        media_type: Some("text/markdown".to_string()),
                    },
                ],
                steps: vec![],
                required_oracle_suites: vec!["suite:SR-SUITE-NONTRIVIALITY".to_string()],
                gate_rule: GateRule::AllRequiredOraclesPass,
                transition_on_pass: TransitionTarget::Stage(StageId::new("DRAFT")),
                requires_portal: false,
                portal_id: None,
                requires_approval: false,
            },
            // stage:DRAFT
            Stage {
                stage_id: StageId::new("DRAFT"),
                stage_name: "Produce draft".to_string(),
                purpose: "Produce the candidate deliverable(s) in requested structure; \
                          produce traceability artifacts."
                    .to_string(),
                required_outputs: vec![
                    RequiredOutput {
                        path: "candidate/main.md".to_string(),
                        role: Some("primary".to_string()),
                        media_type: Some("text/markdown".to_string()),
                    },
                    RequiredOutput {
                        path: "evidence/traceability.json".to_string(),
                        role: Some("evidence".to_string()),
                        media_type: Some("application/json".to_string()),
                    },
                ],
                steps: vec![],
                required_oracle_suites: vec![
                    "suite:SR-SUITE-STRUCTURE".to_string(),
                    "suite:SR-SUITE-TRACEABILITY".to_string(),
                ],
                gate_rule: GateRule::AllRequiredOraclesPass,
                transition_on_pass: TransitionTarget::Stage(StageId::new("SEMANTIC_EVAL")),
                requires_portal: false,
                portal_id: None,
                requires_approval: false,
            },
            // stage:SEMANTIC_EVAL - Trust boundary per SR-PROCEDURE-KIT §1
            Stage {
                stage_id: StageId::new("SEMANTIC_EVAL"),
                stage_name: "Semantic evaluation".to_string(),
                purpose: "Evaluate candidate against the stage manifold/meaning-matrix \
                          suite(s). Capture structured semantic measurements and derived \
                          pass/fail."
                    .to_string(),
                required_outputs: vec![
                    RequiredOutput {
                        path: "reports/semantic/residual.json".to_string(),
                        role: Some("evidence".to_string()),
                        media_type: Some("application/json".to_string()),
                    },
                    RequiredOutput {
                        path: "reports/semantic/coverage.json".to_string(),
                        role: Some("evidence".to_string()),
                        media_type: Some("application/json".to_string()),
                    },
                    RequiredOutput {
                        path: "reports/semantic/violations.json".to_string(),
                        role: Some("evidence".to_string()),
                        media_type: Some("application/json".to_string()),
                    },
                ],
                steps: vec![],
                required_oracle_suites: vec!["suite:SR-SUITE-SEMANTIC".to_string()],
                gate_rule: GateRule::AllRequiredOraclesPass,
                transition_on_pass: TransitionTarget::Stage(StageId::new("FINAL")),
                requires_portal: false,
                portal_id: None,
                requires_approval: true, // Trust boundary per SR-CONTRACT C-TB-3
            },
            // stage:FINAL - Trust boundary per SR-PROCEDURE-KIT §1
            Stage {
                stage_id: StageId::new("FINAL"),
                stage_name: "Package final output".to_string(),
                purpose: "Package final candidate + summary; ensure evidence bundle \
                          references everything required to reconstruct context and \
                          evaluation."
                    .to_string(),
                required_outputs: vec![
                    RequiredOutput {
                        path: "candidate/final.md".to_string(),
                        role: Some("primary".to_string()),
                        media_type: Some("text/markdown".to_string()),
                    },
                    RequiredOutput {
                        path: "evidence/gate_packet.json".to_string(),
                        role: Some("evidence".to_string()),
                        media_type: Some("application/json".to_string()),
                    },
                ],
                steps: vec![],
                required_oracle_suites: vec![
                    "suite:SR-SUITE-REFS".to_string(),
                    "suite:SR-SUITE-SEMANTIC".to_string(),
                ],
                gate_rule: GateRule::AllRequiredOraclesPass,
                transition_on_pass: TransitionTarget::Terminal,
                requires_portal: false,
                portal_id: None,
                requires_approval: true, // Trust boundary per SR-CONTRACT C-TB-3
            },
        ],
        terminal_stage_id: StageId::new("FINAL"),
        initial_stage_id: Some(StageId::new("FRAME")),
        content_hash: None,
        version: Some("1.0.0".to_string()),
    }
}

// ============================================================================
// Template Registry
// ============================================================================

/// Procedure template registry entry
pub struct TemplateRegistryEntry {
    pub template: ProcedureTemplate,
    pub content_hash: ContentHash,
}

/// Get all registered procedure templates
pub fn get_registered_templates() -> Vec<TemplateRegistryEntry> {
    vec![
        create_entry(problem_statement_ingestion_template()),
        create_entry(generic_knowledge_work_template()),
    ]
}

/// Get a template by ID
pub fn get_template_by_id(id: &ProcedureTemplateId) -> Option<TemplateRegistryEntry> {
    get_registered_templates()
        .into_iter()
        .find(|e| &e.template.procedure_template_id == id)
}

/// Create a registry entry with computed content hash
fn create_entry(mut template: ProcedureTemplate) -> TemplateRegistryEntry {
    let hash = compute_template_hash(&template);
    template.content_hash = Some(hash.clone());
    TemplateRegistryEntry {
        template,
        content_hash: hash,
    }
}

/// Compute deterministic content hash for a procedure template
pub fn compute_template_hash(template: &ProcedureTemplate) -> ContentHash {
    let mut hasher = Sha256::new();

    // Include template identity
    hasher.update(template.procedure_template_id.as_str().as_bytes());
    hasher.update(b"\n");

    // Include version if present
    if let Some(version) = &template.version {
        hasher.update(b"version:");
        hasher.update(version.as_bytes());
        hasher.update(b"\n");
    }

    // Include each stage deterministically
    for stage in &template.stages {
        hasher.update(b"stage:");
        hasher.update(stage.stage_id.as_str().as_bytes());
        hasher.update(b":");
        hasher.update(stage.stage_name.as_bytes());
        hasher.update(b"\n");

        // Include required oracle suites
        for suite in &stage.required_oracle_suites {
            hasher.update(b"suite:");
            hasher.update(suite.as_bytes());
            hasher.update(b"\n");
        }

        // Include gate rule
        let gate_str = match &stage.gate_rule {
            GateRule::AllRequiredOraclesPass => "all_required_oracles_pass",
            GateRule::AllOraclesPass => "all_oracles_pass",
            GateRule::PortalApprovalRequired => "portal_approval_required",
            GateRule::Custom(s) => s.as_str(),
        };
        hasher.update(b"gate:");
        hasher.update(gate_str.as_bytes());
        hasher.update(b"\n");

        // Include portal info
        if stage.requires_portal {
            hasher.update(b"portal:");
            if let Some(portal_id) = &stage.portal_id {
                hasher.update(portal_id.as_bytes());
            }
            hasher.update(b"\n");
        }
    }

    // Include terminal stage
    hasher.update(b"terminal:");
    hasher.update(template.terminal_stage_id.as_str().as_bytes());
    hasher.update(b"\n");

    let result = hasher.finalize();
    ContentHash::new(&hex::encode(result))
}

// ============================================================================
// Oracle Suite Bindings for Branch 0
// ============================================================================

/// Get oracle suite bindings for the Problem Statement Ingestion template
pub fn intake_admissibility_oracle_suites() -> Vec<OracleSuiteBinding> {
    vec![
        OracleSuiteBinding {
            suite_id: "suite:SR-SUITE-GOV".to_string(),
            suite_hash: ContentHash::new(
                "placeholder_gov_suite_hash_to_be_computed_from_actual_suite_definition",
            ),
        },
        OracleSuiteBinding {
            suite_id: "oracle.suite.intake_admissibility.v1".to_string(),
            suite_hash: ContentHash::new(
                "placeholder_admissibility_suite_hash_to_be_computed_from_actual_suite_definition",
            ),
        },
    ]
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_statement_ingestion_template_valid() {
        let template = problem_statement_ingestion_template();
        assert!(template.validate().is_ok());
        assert_eq!(
            template.procedure_template_id.as_str(),
            "proc:PROBLEM-STATEMENT-INGESTION"
        );
        assert_eq!(template.stages.len(), 3);
    }

    #[test]
    fn test_problem_statement_ingestion_stages() {
        let template = problem_statement_ingestion_template();

        // Verify stage order and IDs
        assert_eq!(template.stages[0].stage_id.as_str(), "stage:INGEST");
        assert_eq!(template.stages[1].stage_id.as_str(), "stage:VALIDATE");
        assert_eq!(template.stages[2].stage_id.as_str(), "stage:ACCEPT");

        // Verify transitions
        assert_eq!(
            template.stages[0].transition_on_pass,
            TransitionTarget::Stage(StageId::new("VALIDATE"))
        );
        assert_eq!(
            template.stages[1].transition_on_pass,
            TransitionTarget::Stage(StageId::new("ACCEPT"))
        );
        assert_eq!(
            template.stages[2].transition_on_pass,
            TransitionTarget::Terminal
        );

        // Verify portal boundary on ACCEPT stage
        assert!(template.stages[2].requires_portal);
        assert_eq!(
            template.stages[2].portal_id,
            Some("IntakeAcceptancePortal".to_string())
        );
        assert_eq!(
            template.stages[2].gate_rule,
            GateRule::PortalApprovalRequired
        );
    }

    #[test]
    fn test_problem_statement_ingestion_terminal() {
        let template = problem_statement_ingestion_template();
        assert!(template.is_terminal(&StageId::new("ACCEPT")));
        assert!(!template.is_terminal(&StageId::new("INGEST")));
    }

    #[test]
    fn test_generic_knowledge_work_template_valid() {
        let template = generic_knowledge_work_template();
        assert!(template.validate().is_ok());
        assert_eq!(
            template.procedure_template_id.as_str(),
            "proc:GENERIC-KNOWLEDGE-WORK"
        );
        assert_eq!(template.stages.len(), 5);
    }

    #[test]
    fn test_generic_knowledge_work_stages() {
        let template = generic_knowledge_work_template();

        // Verify all expected stages exist
        let stage_ids: Vec<&str> = template
            .stages
            .iter()
            .map(|s| s.stage_id.as_str())
            .collect();

        assert!(stage_ids.contains(&"stage:FRAME"));
        assert!(stage_ids.contains(&"stage:OPTIONS"));
        assert!(stage_ids.contains(&"stage:DRAFT"));
        assert!(stage_ids.contains(&"stage:SEMANTIC_EVAL"));
        assert!(stage_ids.contains(&"stage:FINAL"));

        // Verify initial stage
        assert_eq!(template.get_initial_stage().as_str(), "stage:FRAME");
    }

    #[test]
    fn test_template_hash_deterministic() {
        let template1 = problem_statement_ingestion_template();
        let template2 = problem_statement_ingestion_template();

        let hash1 = compute_template_hash(&template1);
        let hash2 = compute_template_hash(&template2);

        assert_eq!(hash1.as_str(), hash2.as_str());
    }

    #[test]
    fn test_template_hash_differs_for_different_templates() {
        let template1 = problem_statement_ingestion_template();
        let template2 = generic_knowledge_work_template();

        let hash1 = compute_template_hash(&template1);
        let hash2 = compute_template_hash(&template2);

        assert_ne!(hash1.as_str(), hash2.as_str());
    }

    #[test]
    fn test_template_registry() {
        let templates = get_registered_templates();
        assert_eq!(templates.len(), 2);

        // Verify all templates have content hashes
        for entry in &templates {
            assert!(entry.template.content_hash.is_some());
            assert_eq!(
                entry.template.content_hash.as_ref().unwrap().as_str(),
                entry.content_hash.as_str()
            );
        }
    }

    #[test]
    fn test_get_template_by_id() {
        let id = ProcedureTemplateId::new("PROBLEM-STATEMENT-INGESTION");
        let entry = get_template_by_id(&id);
        assert!(entry.is_some());

        let template = entry.unwrap().template;
        assert_eq!(template.procedure_template_id, id);
    }

    #[test]
    fn test_get_template_by_id_not_found() {
        let id = ProcedureTemplateId::new("NONEXISTENT");
        let entry = get_template_by_id(&id);
        assert!(entry.is_none());
    }

    #[test]
    fn test_intake_admissibility_oracle_suites() {
        let suites = intake_admissibility_oracle_suites();
        assert_eq!(suites.len(), 2);
        assert!(suites.iter().any(|s| s.suite_id == "suite:SR-SUITE-GOV"));
        assert!(suites
            .iter()
            .any(|s| s.suite_id == "oracle.suite.intake_admissibility.v1"));
    }

    #[test]
    fn test_work_kinds_for_templates() {
        let psi = problem_statement_ingestion_template();
        assert!(psi.kind.contains(&WorkKind::IntakeProcessing));

        let gkw = generic_knowledge_work_template();
        assert!(gkw.kind.contains(&WorkKind::ResearchMemo));
        assert!(gkw.kind.contains(&WorkKind::DecisionRecord));
        assert!(gkw.kind.contains(&WorkKind::AnalysisReport));
        assert!(gkw.kind.contains(&WorkKind::TechnicalSpec));
    }
}

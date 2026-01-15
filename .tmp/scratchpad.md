     Templates UI Implementation Plan - Phase 2: Starter Templates

     Overview

     Seed the Templates registry with comprehensive starter template instances for each user-instantiable type. These
      starter templates serve as reference implementations demonstrating correct schema usage, field population, and
     governance patterns. Users can view, clone, and modify these templates to create their own governed artifacts.

     ---
     Template Type Summary & Starter Instances

     Category 1: Work Surface (Self-Service)

     1.1 Intake (record.intake)

     Purpose: Structured specification defining a work unit's objective, scope, deliverables, and constraints.

     Starter Instance: Standard Research Memo Intake
     {
       "work_unit_id": "WU-TEMPLATE-001",
       "title": "API Rate Limiting Analysis",
       "kind": "research_memo",
       "objective": "Evaluate rate limiting strategies for the public API to prevent abuse while maintaining 
     developer experience.",
       "audience": "Engineering team and API stakeholders",
       "deliverables": [
         "Analysis of current rate limiting implementation",
         "Comparison of 3+ alternative rate limiting strategies",
         "Recommendation with implementation approach",
         "Test plan for validating the chosen strategy"
       ],
       "constraints": [
         "Maximum 2000 words",
         "Must include performance impact analysis",
         "Must address backward compatibility"
       ],
       "definitions": {
         "rate limiting": "Controlling the number of requests a client can make within a time window",
         "burst allowance": "Short-term allowance for exceeding the base rate limit"
       },
       "inputs": [
         "ref:doc:current-api-metrics",
         "ref:doc:competitor-analysis"
       ],
       "unknowns": [
         "Current p99 latency under load",
         "Expected traffic growth over 12 months"
       ],
       "completion_criteria": [
         "All three strategies evaluated with pros/cons",
         "Recommendation includes migration path",
         "Performance projections provided"
       ],
       "non_goals": [
         "Implementation of the solution",
         "Changes to authentication system"
       ]
     }

     ---
     1.2 Procedure Template (config.procedure_template)

     Purpose: Stage-gated workflow definition for candidate generation and oracle verification.

     Starter Instance: Research Memo Procedure
     {
       "procedure_template_id": "proc:RESEARCH-MEMO",
       "kind": ["research_memo", "analysis_doc"],
       "description": "Four-stage procedure for producing research memos: Frame the problem, explore options, draft 
     the memo, and finalize.",
       "stages": [
         {
           "stage_id": "stage:FRAME",
           "stage_name": "Frame",
           "purpose": "Establish problem definition, scope boundaries, and success criteria",
           "required_outputs": ["framing_doc.md"],
           "steps": [
             "Review intake objectives and constraints",
             "Identify key questions to answer",
             "Define scope boundaries explicitly",
             "List assumptions and unknowns"
           ],
           "required_oracle_suites": ["suite:SR-SUITE-GOV"],
           "gate_rule": "all_required_oracles_pass",
           "transition_on_pass": "stage:OPTIONS"
         },
         {
           "stage_id": "stage:OPTIONS",
           "stage_name": "Options Analysis",
           "purpose": "Identify and evaluate alternative approaches",
           "required_outputs": ["options_analysis.md"],
           "steps": [
             "Identify at least 3 viable options",
             "Define evaluation criteria from intake",
             "Analyze each option against criteria",
             "Document trade-offs and risks"
           ],
           "required_oracle_suites": ["suite:SR-SUITE-GOV", "suite:SR-SUITE-CORE"],
           "gate_rule": "all_required_oracles_pass",
           "transition_on_pass": "stage:DRAFT"
         },
         {
           "stage_id": "stage:DRAFT",
           "stage_name": "Draft",
           "purpose": "Produce complete draft meeting intake requirements",
           "required_outputs": ["draft.md"],
           "steps": [
             "Synthesize framing and options into coherent narrative",
             "Include all required sections from constraints",
             "Ensure recommendation is actionable",
             "Self-review against completion criteria"
           ],
           "required_oracle_suites": ["suite:SR-SUITE-CORE"],
           "gate_rule": "all_required_oracles_pass",
           "transition_on_pass": "stage:FINAL"
         },
         {
           "stage_id": "stage:FINAL",
           "stage_name": "Final",
           "purpose": "Polish and prepare for delivery",
           "required_outputs": ["final.md"],
           "steps": [
             "Address any oracle feedback from draft stage",
             "Final prose and formatting review",
             "Verify all deliverables present",
             "Prepare for closeout review"
           ],
           "required_oracle_suites": ["suite:SR-SUITE-CORE", "suite:SR-SUITE-FULL"],
           "gate_rule": "all_required_oracles_pass",
           "transition_on_pass": "terminal"
         }
       ],
       "terminal_stage_id": "stage:FINAL"
     }

     ---
     1.3 Work Surface Instance (domain.work_surface)

     Purpose: Runtime binding of a work unit to its intake, procedure, and oracle configuration.

     Starter Instance: Example Work Surface Binding
     {
       "artifact_type": "domain.work_surface",
       "work_unit_id": "WU-TEMPLATE-001",
       "intake_ref": {
         "id": "intake:WU-TEMPLATE-001",
         "content_hash": "sha256:example_intake_hash_placeholder"
       },
       "procedure_template_ref": {
         "id": "proc:RESEARCH-MEMO",
         "content_hash": "sha256:example_procedure_hash_placeholder"
       },
       "stage_id": "stage:FRAME",
       "oracle_suites": [
         {"suite_id": "suite:SR-SUITE-GOV", "suite_hash": "sha256:gov_suite_hash"},
         {"suite_id": "suite:SR-SUITE-CORE", "suite_hash": "sha256:core_suite_hash"}
       ],
       "params": {
         "max_iterations_per_stage": 3,
         "semantic_threshold": 0.8
       }
     }

     ---
     Category 2: Execution Policy (Self-Service)

     2.1 Budget Configuration (budget_config)

     Purpose: Resource constraints per work unit preventing runaway execution.

     Starter Instance: Standard Budget Policy
     {
       "policy_id": "budget:STANDARD",
       "name": "Standard Work Unit Budget",
       "description": "Default resource constraints for typical work units. Allows 5 iterations with 25 oracle runs 
     over 16 hours.",
       "max_iterations": 5,
       "max_oracle_runs": 25,
       "max_wallclock_hours": 16,
       "warning_thresholds": {
         "iterations_warning_at": 4,
         "oracle_runs_warning_at": 20,
         "wallclock_warning_at": 12
       },
       "on_exhaustion": {
         "stop_trigger": "BUDGET_EXHAUSTED",
         "routing_portal": "HumanAuthorityExceptionProcess"
       }
     }

     ---
     2.2 Gating Policy (config.gating_policy)

     Purpose: Configure human judgment hooks per work unit.

     Starter Instance: Hybrid Gating Policy
     {
       "policy_id": "gating:HYBRID-STANDARD",
       "name": "Standard Hybrid Gating",
       "description": "Soft gating by default with automatic escalation to hard gating on specific triggers.",
       "gating_mode": "hybrid",
       "hooks": [
         {
           "hook_id": "plan_review",
           "description": "Non-binding plan review before iteration starts",
           "trigger": "iteration_start",
           "required": false
         },
         {
           "hook_id": "evaluation_on_verification",
           "description": "Human review of oracle evidence",
           "trigger": "oracle_suite_pass",
           "required": false
         },
         {
           "hook_id": "assessment_on_validation",
           "description": "Human assessment before stage advancement",
           "trigger": "stage_gate_pass",
           "required": true
         },
         {
           "hook_id": "closeout",
           "description": "Final approval before freezing candidate",
           "trigger": "terminal_stage_complete",
           "required": true,
           "portal": "ReleaseApprovalPortal"
         }
       ],
       "deterministic_triggers": [
         {
           "condition": "EXCEPTIONS_ACTIVE",
           "escalates_to": "hard",
           "description": "Active waivers, deviations, or deferrals require hard gating"
         },
         {
           "condition": "REPEATED_FAILURE",
           "threshold": 3,
           "escalates_to": "hard",
           "description": "3+ consecutive failures require human review"
         },
         {
           "condition": "BUDGET_NEAR_EXHAUSTED",
           "threshold_percent": 80,
           "escalates_to": "hard",
           "description": "Budget consumption >= 80% triggers human review"
         }
       ]
     }

     ---
     Category 3: Oracle Configuration (Portal Required: GovernanceChangePortal)

     3.1 Oracle Suite Definition (oracle_suite)

     Purpose: Define a set of oracles that run together in a sandboxed environment.

     Starter Instance: Custom Verification Suite
     {
       "suite_id": "suite:CUSTOM-VERIFY",
       "suite_version": "1.0.0",
       "description": "Example custom oracle suite for project-specific verification",
       "determinism_required": true,
       "environment_constraints": {
         "runner": "ci",
         "network": "disabled",
         "oci_image": "ghcr.io/org/oracle-runner:v1.0.0",
         "oci_image_digest": "sha256:example_digest_placeholder",
         "cpu_arch": "amd64",
         "os": "linux",
         "workspace_readonly": true,
         "additional_constraints": ["runtime=runsc"]
       },
       "oracles": [
         {
           "oracle_id": "oracle:custom_lint",
           "classification": "required",
           "purpose": "Project-specific linting rules",
           "command": "./scripts/lint.sh",
           "timeout_seconds": 120,
           "retries": 1,
           "expected_outputs": [
             {"path": "reports/lint.json", "media_type": "application/json", "role": "report"}
           ]
         },
         {
           "oracle_id": "oracle:custom_test",
           "classification": "required",
           "purpose": "Project-specific test execution",
           "command": "./scripts/test.sh",
           "timeout_seconds": 300,
           "retries": 2,
           "expected_outputs": [
             {"path": "reports/test.json", "media_type": "application/json", "role": "report"},
             {"path": "coverage/lcov.info", "media_type": "text/plain", "role": "artifact"}
           ]
         }
       ],
       "flake_policy": {
         "required_oracles": "stop_the_line",
         "advisory_oracles": "warn_only"
       },
       "evidence_capture_policy": {
         "capture_stdout": true,
         "capture_stderr": true,
         "capture_env_fingerprint": true,
         "capture_artifact_hashes": true
       }
     }

     ---
     Category 4: Verification Profiles (Portal Required: GovernanceChangePortal)

     4.1 Verification Profile (verification_profile)

     Purpose: Map oracle suites to deliverable types with waiver policies.

     Starter Instance: Project Standard Profile
     {
       "profile_id": "profile:PROJECT-STANDARD",
       "profile_version": "1.0.0",
       "name": "Project Standard Verification",
       "description": "Standard verification profile for project deliverables requiring code and governance checks",
       "required_suites": [
         "suite:SR-SUITE-GOV",
         "suite:SR-SUITE-CORE"
       ],
       "advisory_suites": [
         "suite:SR-SUITE-FULL"
       ],
       "verification_mode_default": "STRICT",
       "waiver_policy": {
         "enabled": true,
         "portal": "HumanAuthorityExceptionProcess",
         "requires_justification": true,
         "requires_risk_mitigation": true,
         "requires_expiry_date": true
       },
       "waiver_eligible_failures": [
         "BUILD_FAIL",
         "UNIT_FAIL",
         "LINT_FAIL",
         "SCHEMA_FAIL"
       ],
       "non_waivable_integrity_conditions": [
         "ORACLE_TAMPER",
         "ORACLE_GAP",
         "ORACLE_ENV_MISMATCH",
         "ORACLE_FLAKE",
         "EVIDENCE_MISSING",
         "MANIFEST_INVALID"
       ],
       "applicable_deliverables": [
         "research_memo",
         "decision_record",
         "code_change"
       ]
     }

     ---
     Category 5: Semantic Sets (Portal Required: GovernanceChangePortal)

     5.1 Semantic Set (config.semantic_set)

     Purpose: Define meaning-matrix/manifold for semantic oracle evaluation.

     Starter Instance: Research Memo Quality Set
     {
       "semantic_set_id": "semset:research-memo-quality-v1",
       "name": "Research Memo Quality Metrics",
       "description": "Semantic quality dimensions for evaluating research memos against intake requirements",
       "axes": [
         {
           "axis_id": "clarity",
           "name": "Clarity",
           "description": "How clearly the content communicates its ideas",
           "weight": 0.25,
           "required": true,
           "min_coverage": 0.8,
           "max_residual": 0.15,
           "evaluation_criteria": [
             "Sentences are concise and unambiguous",
             "Technical terms are defined or well-known",
             "Structure aids comprehension"
           ]
         },
         {
           "axis_id": "completeness",
           "name": "Completeness",
           "description": "Whether all required elements from intake are addressed",
           "weight": 0.30,
           "required": true,
           "min_coverage": 0.9,
           "max_residual": 0.1,
           "evaluation_criteria": [
             "All deliverables from intake are present",
             "All unknowns are addressed or explained",
             "Completion criteria are demonstrably met"
           ]
         },
         {
           "axis_id": "actionability",
           "name": "Actionability",
           "description": "Whether recommendations can be acted upon",
           "weight": 0.25,
           "required": true,
           "min_coverage": 0.75,
           "max_residual": 0.2,
           "evaluation_criteria": [
             "Recommendations include concrete next steps",
             "Trade-offs are clearly stated",
             "Decision criteria are explicit"
           ]
         },
         {
           "axis_id": "accuracy",
           "name": "Accuracy",
           "description": "Factual correctness and logical soundness",
           "weight": 0.20,
           "required": true,
           "min_coverage": 0.95,
           "max_residual": 0.05,
           "evaluation_criteria": [
             "Claims are supported by evidence or citations",
             "Logical arguments are valid",
             "No internal contradictions"
           ]
         }
       ],
       "constraints": [
         {
           "constraint_id": "length_compliance",
           "description": "Content respects length constraints from intake",
           "severity": "error",
           "check": "word_count <= intake.constraints.max_words"
         },
         {
           "constraint_id": "required_sections",
           "description": "All required sections from intake are present",
           "severity": "error",
           "check": "all(intake.constraints.required_sections in sections)"
         }
       ],
       "decision_rule": {
         "rule_id": "composite_quality",
         "max_residual_norm": 0.2,
         "min_coverage": 0.8,
         "max_error_violations": 0,
         "max_warning_violations": 3,
         "aggregation": "weighted_average"
       }
     }

     ---
     Category 6: Exceptions (Portal Required: HumanAuthorityExceptionProcess)

     6.1 Waiver (record.waiver)

     Purpose: Permission to proceed despite an oracle FAIL.

     Starter Instance: Example Waiver Template
     {
       "exception_id": "exc_TEMPLATE_WAIVER_001",
       "kind": "WAIVER",
       "name": "Unit Test Waiver Template",
       "description": "Template for waiving unit test failures in controlled circumstances",
       "oracle_failure_ref": "oracle:unit_test",
       "scope": "per-candidate",
       "scope_description": "This waiver applies only to the specific candidate identified",
       "risk_mitigation": "Integration tests provide coverage for the waived unit tests. Manual testing confirms 
     functionality. Monitoring will be enhanced post-deployment.",
       "resolution_criteria": "Waiver expires when unit tests are fixed or candidate is superseded",
       "expiry_date": "2026-02-15T00:00:00Z",
       "justification_template": "Unit tests fail due to [REASON]. The risk is mitigated by [MITIGATION]. This waiver
      is time-bounded to [DATE].",
       "required_evidence": [
         "Link to failing test results",
         "Integration test results showing coverage",
         "Manual test results"
       ],
       "non_waivable_notice": "This template cannot be used for integrity conditions: ORACLE_TAMPER, ORACLE_GAP, 
     ORACLE_ENV_MISMATCH, ORACLE_FLAKE, EVIDENCE_MISSING"
     }

     ---
     6.2 Deviation (record.deviation)

     Purpose: Exception from a governed requirement.

     Starter Instance: Example Deviation Template
     {
       "exception_id": "exc_TEMPLATE_DEVIATION_001",
       "kind": "DEVIATION",
       "name": "Architectural Deviation Template",
       "description": "Template for deviating from architectural requirements in bounded scope",
       "requirement_ref": "SR-CONTRACT §X.Y",
       "requirement_description": "The specific architectural requirement being deviated from",
       "scope": "per-module",
       "scope_description": "Deviation is bounded to the specific module/component identified",
       "justification_template": "Deviation from [REQUIREMENT] is necessary because [REASON]. The alternative would 
     require [COST]. This deviation is bounded to [SCOPE].",
       "risk_mitigation": "Risk is contained by [BOUNDARY]. Impact is limited to [SCOPE]. Fallback approach is 
     [FALLBACK].",
       "resolution_criteria": "Deviation is resolved when the requirement can be met without [CONSTRAINT]",
       "required_evidence": [
         "Analysis showing why requirement cannot be met",
         "Impact assessment",
         "Fallback plan documentation"
       ]
     }

     ---
     6.3 Deferral (record.deferral)

     Purpose: Binding postponement of work or requirement.

     Starter Instance: Example Deferral Template
     {
       "exception_id": "exc_TEMPLATE_DEFERRAL_001",
       "kind": "DEFERRAL",
       "name": "Work Deferral Template",
       "description": "Template for deferring work to a future iteration or sprint",
       "subject_ref": "work_item:XYZ or requirement:ABC",
       "subject_description": "The work item or requirement being deferred",
       "scope": "time-boxed",
       "scope_description": "Deferral is bounded to the specified time period",
       "justification_template": "Deferring [SUBJECT] because [REASON]. Current priority is [PRIORITY]. The work will
      be addressed by [TARGET_DATE].",
       "target_date": "2026-03-01T00:00:00Z",
       "target_description": "Next sprint planning cycle",
       "tracking": {
         "create_follow_up": true,
         "follow_up_type": "work_item",
         "notify_on_expiry": true
       },
       "required_evidence": [
         "Current priority justification",
         "Impact assessment of deferral",
         "Target timeline feasibility"
       ]
     }

     ---
     Implementation Approach

     Backend Changes (crates/sr-api/src/handlers/templates.rs)

     1. Add build_starter_instances() method to TemplateRegistry that creates starter instances for each
     user-instantiable type
     2. Initialize registry with starters in TemplateRegistry::new():
     impl TemplateRegistry {
         pub fn new() -> Self {
             let mut registry = Self {
                 templates: RwLock::new(Vec::new()),
                 schemas: Self::build_schemas(),
             };
             // Seed starter templates synchronously during initialization
             registry.seed_starter_templates();
             registry
         }

         fn seed_starter_templates(&self) {
             let starters = Self::build_starter_instances();
             let mut templates = self.templates.blocking_write();
             templates.extend(starters);
         }
     }

     3. Starter instances use special naming convention:
       - ID prefix: tmpl_starter_ to distinguish from user-created
       - Status: "reference" (new status) to indicate read-only templates
       - Created_by: "system" to indicate platform-provided
     4. Add "reference" status to template status enum for platform-provided templates

     Frontend Changes (ui/src/pages/Templates.tsx)

     1. Show starter templates prominently at top of each category tab
     2. Add visual indicator (badge) for "Reference" templates
     3. Add "Clone" button to create editable copy from reference template
     4. Reference templates are read-only in detail view

     ---
     Files to Modify





     ┌─────────────────────────────────────────┬─────────────────────────────────────────────────────────────────────
     ──┐
     │                  File                   │                                Changes
       │
     ├─────────────────────────────────────────┼─────────────────────────────────────────────────────────────────────
     ──┤
     │ crates/sr-api/src/handlers/templates.rs │ Add build_starter_instances(), seed_starter_templates(), update
     new() │
     ├─────────────────────────────────────────┼─────────────────────────────────────────────────────────────────────
     ──┤
     │ ui/src/pages/Templates.tsx              │ Add reference template section, clone functionality
       │
     ├─────────────────────────────────────────┼─────────────────────────────────────────────────────────────────────
     ──┤
     │ ui/src/pages/TemplateDetail.tsx         │ Add read-only mode for reference templates
       │
     └─────────────────────────────────────────┴─────────────────────────────────────────────────────────────────────
     ──┘



     ---
     Verification

     1. Backend: cargo test --package sr-api - verify starter templates load
     2. Frontend: npm run type-check && npm run build - verify UI compiles
     3. Manual: Navigate to /templates, verify each category shows starter template
     4. Manual: Click starter template, verify detail view shows full content
     5. Manual: Verify reference templates show "Reference" badge and are read-only
  ⎿  Interrupted · What should Claude do instead?
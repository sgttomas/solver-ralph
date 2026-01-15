---
doc_id: SR-TEMPLATES
doc_kind: governance.templates_registry
layer: platform
status: draft
refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: depends_on
    to: SR-TYPES
  - rel: depends_on
    to: SR-WORK-SURFACE
  - rel: depends_on
    to: SR-SEMANTIC-ORACLE-SPEC
  - rel: depends_on
    to: SR-PROCEDURE-KIT
  - rel: depends_on
    to: SR-DIRECTIVE
  - rel: depends_on
    to: SR-SPEC
  - rel: depends_on
    to: SR-CONTRACT
---

# SR-TEMPLATES — User Configuration Registry

**Purpose:** Comprehensive registry of all user-configurable templates, schemas, and configuration artifacts in SOLVER-Ralph. This document catalogs what users must define to configure semantic work units, oracle verification, execution policies, and governance workflows.

**Normative Status:** Index (navigation). Authoritative schemas are defined in the referenced source documents.

---

## 1. Overview

SOLVER-Ralph requires configuration across **11 categories** to enable governed semantic work:

| # | Category | Key Templates | Primary Source |
|---|----------|---------------|----------------|
| 1 | Work Surface | Intake, Procedure Template, Work Surface Instance | SR-WORK-SURFACE |
| 2 | Oracle Configuration | Oracle Suite, Oracle Definition, Semantic Oracle | SR-DIRECTIVE, SR-SEMANTIC-ORACLE-SPEC |
| 3 | Verification Profiles | Verification Profile, Profile Selection Matrix | SR-DIRECTIVE §5 |
| 4 | Semantic Sets | Semantic Set, Semantic Axis, Decision Rule | SR-SEMANTIC-ORACLE-SPEC §4 |
| 5 | Gate Configuration | Gate Definition | SR-DIRECTIVE.KIT-GATE-REGISTRY |
| 6 | Portal Configuration | Portal Playbook | SR-DIRECTIVE.KIT-PORTAL-PLAYBOOKS |
| 7 | Execution Policy | Budgets, Stop Triggers, Gating Policy | SR-DIRECTIVE §4, SR-SPEC §3.2.1.3 |
| 8 | Iteration Context | Required Refs, Iteration Summary | SR-SPEC §3.2.1, §3.2.2 |
| 9 | Evidence | Evidence Bundle Manifest, Semantic Eval Result | SR-SPEC §1.9, SR-SEMANTIC-ORACLE-SPEC |
| 10 | Release | Freeze Record | SR-SPEC §1.12 |
| 11 | Exceptions | Waiver, Deviation, Deferral | SR-SPEC §1.14, SR-CONTRACT §2.7 |

---

## 2. Work Surface Templates

### 2.1 Intake

**Type Key:** `record.intake`
**Purpose:** Structured specification of a work unit's objective, scope, deliverables, and constraints.
**Source:** SR-WORK-SURFACE §3

#### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `work_unit_id` | string | Stable identifier |
| `title` | string | Human-readable name |
| `kind` | string | Work kind taxonomy (e.g., `research_memo`, `decision_record`, `ontology_build`) |
| `objective` | string | One-sentence goal |
| `audience` | string | Target readers/users |
| `deliverables[]` | array | Exact required outputs |
| `constraints[]` | array | Length, tone, required sections, prohibited content |
| `definitions{}` | object | Term-to-definition mapping |
| `inputs[]` | array | Provided context references |
| `unknowns[]` | array | Questions to resolve |
| `completion_criteria[]` | array | Human-facing acceptance criteria |

#### Example

```yaml
---
artifact_type: record.intake
artifact_version: v1
work_unit_id: WU-2026-001
title: API Rate Limiting Analysis
kind: research_memo
audience: Engineering team
objective: Evaluate rate limiting strategies for the public API
deliverables:
  - path: candidate/main.md
    media_type: text/markdown
    description: Research memo with recommendation
constraints:
  - "Must include sections: Background, Options, Recommendation"
  - "Maximum 2000 words"
  - "Must cite at least 3 industry references"
definitions:
  rate_limit: "Maximum requests per time window per client"
  burst: "Short-term allowance above sustained rate"
inputs:
  - rel: depends_on
    kind: context
    locator: refs/current_api_spec.yaml
unknowns:
  - "What is acceptable latency for rate limit checks?"
completion_criteria:
  - "Reviewer can make implementation decision from memo"
---
```

---

### 2.2 Procedure Template

**Type Key:** `config.procedure_template`
**Purpose:** Stage-gated workflow definition for candidate generation and oracle verification.
**Source:** SR-WORK-SURFACE §4, SR-PROCEDURE-KIT

#### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `procedure_template_id` | string | Stable id (format: `proc:<NAME>`) |
| `kind` | array | Applicable work kinds |
| `stages[]` | array | Stage definitions (see below) |
| `terminal_stage_id` | string | Final stage identifier |

#### Stage Fields

| Field | Type | Description |
|-------|------|-------------|
| `stage_id` | string | Stage identifier (format: `stage:<NAME>`) |
| `stage_name` | string | Human-readable name |
| `purpose` | string | What this stage accomplishes |
| `required_outputs[]` | array | Artifacts worker must produce |
| `steps[]` | array | Proceduralized instructions |
| `required_oracle_suites[]` | array | Oracle suite IDs for gating |
| `gate_rule` | string | Decision logic (e.g., `all_required_oracles_pass`) |
| `transition_on_pass` | string | Next stage_id or `terminal` |

#### Example

```yaml
---
artifact_type: config.procedure_template
artifact_version: v1
procedure_template_id: proc:RESEARCH-MEMO
kind: [research_memo, decision_record]
terminal_stage_id: stage:FINAL
stages:
  - stage_id: stage:FRAME
    stage_name: Frame the problem
    purpose: Restate objective, extract constraints and definitions
    required_outputs:
      - path: artifacts/context/frame.md
        role: context
    required_oracle_suites: ["suite:SR-SUITE-STRUCTURE"]
    gate_rule: all_required_oracles_pass
    transition_on_pass: stage:OPTIONS

  - stage_id: stage:OPTIONS
    stage_name: Generate options
    purpose: Generate multiple candidate approaches before drafting
    required_outputs:
      - path: artifacts/candidates/option_A.md
        role: candidate
      - path: artifacts/candidates/selection.md
        role: decision
    required_oracle_suites: ["suite:SR-SUITE-NONTRIVIALITY"]
    gate_rule: all_required_oracles_pass
    transition_on_pass: stage:DRAFT

  - stage_id: stage:DRAFT
    stage_name: Produce draft
    purpose: Produce candidate deliverable with traceability
    required_outputs:
      - path: candidate/main.md
        role: deliverable
      - path: evidence/traceability.json
        role: evidence
    required_oracle_suites: ["suite:SR-SUITE-STRUCTURE", "suite:SR-SUITE-TRACEABILITY"]
    gate_rule: all_required_oracles_pass
    transition_on_pass: stage:FINAL

  - stage_id: stage:FINAL
    stage_name: Package final
    purpose: Package final candidate and evidence bundle
    required_outputs:
      - path: candidate/final.md
        role: deliverable
    required_oracle_suites: ["suite:SR-SUITE-REFS"]
    gate_rule: all_required_oracles_pass
    transition_on_pass: terminal
---
```

---

### 2.3 Work Surface Instance

**Type Key:** `domain.work_surface`
**Purpose:** Runtime binding of a work unit to its intake, procedure, and oracle configuration.
**Source:** SR-WORK-SURFACE §5

#### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `artifact_type` | string | Must be `domain.work_surface` |
| `work_unit_id` | string | Work unit identifier |
| `intake_ref` | object | Content-addressed reference to Intake |
| `procedure_template_ref` | object | Content-addressed reference to Procedure Template |
| `stage_id` | string | Current stage being targeted |
| `oracle_suites[]` | array | Suite IDs + hashes |
| `params{}` | object | Optional stage parameters and thresholds |

#### Example

```yaml
---
artifact_type: domain.work_surface
artifact_version: v1
work_unit_id: WU-2026-001
intake_ref:
  id: intake:WU-2026-001
  content_hash: sha256:abc123...
procedure_template_ref:
  id: proc:RESEARCH-MEMO
  content_hash: sha256:def456...
stage_id: stage:DRAFT
oracle_suites:
  - suite_id: suite:SR-SUITE-STRUCTURE
    suite_hash: sha256:789abc...
  - suite_id: suite:SR-SUITE-TRACEABILITY
    suite_hash: sha256:012def...
params:
  max_residual_norm: 0.2
  min_coverage: 0.8
---
```

---

## 3. Oracle Configuration

### 3.1 Oracle Suite Definition

**Purpose:** Define a set of oracles that run together in a sandboxed environment.
**Source:** SR-DIRECTIVE.KIT-ORACLES_PROFILES_MATRIX.yaml

#### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `suite_id` | string | Identifier (format: `suite:<NAME>`) |
| `suite_version` | string | Version string |
| `description` | string | Purpose of this suite |
| `determinism_required` | boolean | Whether determinism is enforced |
| `environment_constraints` | object | Execution environment requirements |
| `oracles[]` | array | Oracle definitions |
| `flake_policy` | object | Handling of non-deterministic results |
| `evidence_capture_policy` | object | What to capture in evidence |

#### Environment Constraints

| Field | Type | Description |
|-------|------|-------------|
| `runner` | string | `ci` or `local` |
| `network` | string | `disabled`, `restricted`, `allowed` |
| `oci_image_digest` | string | Pinned container image digest |
| `cpu_arch` | string | `amd64`, `arm64` |
| `os` | string | `linux`, `darwin` |
| `additional_constraints[]` | array | Extra constraints (e.g., `runtime=runsc`) |

#### Example

```yaml
- suite_id: suite:SR-SUITE-CORE
  suite_version: "1.0.0"
  description: Core deterministic checks (meta/schema/build/unit/lint)
  determinism_required: true
  environment_constraints:
    runner: ci
    network: disabled
    oci_image_digest: sha256:abc123...
    cpu_arch: amd64
    os: linux
    additional_constraints:
      - runtime=runsc
      - workspace_readonly=true
  oracles:
    - oracle_id: oracle:build
      classification: required
      purpose: Build all services and libraries
      command: sr-oracles build --all
      timeout_seconds: 600
      retries: 0
      expected_outputs:
        - path: reports/build.json
          media_type: application/json
          role: report
  flake_policy:
    on_required_flake: stop_the_line
    on_advisory_flake: warn_only
  evidence_capture_policy:
    include_stdout: true
    include_stderr: true
    include_environment_fingerprint: true
    include_artifact_hashes: true
```

---

### 3.2 Oracle Definition

**Purpose:** Specify an individual oracle's execution parameters.
**Source:** SR-DIRECTIVE.KIT-ORACLES_PROFILES_MATRIX.yaml

#### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `oracle_id` | string | Identifier (format: `oracle:<name>`) |
| `classification` | string | `required` or `advisory` |
| `purpose` | string | What this oracle validates |
| `command` | string | Execution command |
| `timeout_seconds` | integer | Execution timeout |
| `retries` | integer | Retry count on failure |
| `expected_outputs[]` | array | Output artifact specifications |

#### Expected Output Fields

| Field | Type | Description |
|-------|------|-------------|
| `path` | string | Output file path |
| `media_type` | string | MIME type |
| `role` | string | `report`, `log`, or `artifact` |

---

### 3.3 Semantic Oracle Definition

**Purpose:** Oracle that measures candidate quality against a semantic set.
**Source:** SR-SEMANTIC-ORACLE-SPEC

#### Additional Fields (beyond standard Oracle Definition)

| Field | Type | Description |
|-------|------|-------------|
| `semantic_set_id` | string | Reference to meaning-matrix |
| `semantic_set_hash` | string | Content hash of semantic set |
| `target_axis` | string | Which semantic axis this oracle measures |

#### Required Outputs

Semantic oracles MUST produce:
- `reports/semantic/residual.json`
- `reports/semantic/coverage.json`
- `reports/semantic/violations.json`

---

## 4. Verification Profiles

### 4.1 Verification Profile

**Purpose:** Map oracle suites to deliverable types with waiver policies.
**Source:** SR-DIRECTIVE §5.2

#### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `profile_id` | string | Identifier (e.g., `GOV-CORE`, `STRICT-CORE`) |
| `profile_version` | string | Version string |
| `description` | string | Purpose |
| `required_suites[]` | array | Mandatory oracle suite IDs |
| `advisory_suites[]` | array | Optional oracle suite IDs |
| `verification_mode_default` | string | `STRICT` |
| `waiver_policy` | object | Exception handling |
| `waiver_eligible_failures[]` | array | Failures that can be waived |
| `non_waivable_integrity_conditions[]` | array | Non-waivable conditions |

#### Pre-defined Profiles

| Profile | Required Suites | Waivable |
|---------|-----------------|----------|
| `GOV-CORE` | `suite:SR-SUITE-GOV` | No |
| `STRICT-CORE` | `suite:SR-SUITE-CORE` | Yes (BUILD_FAIL, UNIT_FAIL, LINT_FAIL, SCHEMA_FAIL) |
| `STRICT-FULL` | `suite:SR-SUITE-FULL` | Yes (+ INTEGRATION_FAIL, E2E_FAIL) |

#### Non-waivable Integrity Conditions (all profiles)

- `ORACLE_TAMPER`
- `ORACLE_GAP`
- `ORACLE_ENV_MISMATCH`
- `ORACLE_FLAKE`
- `EVIDENCE_MISSING`

---

### 4.2 Profile Selection Matrix

**Purpose:** Map work unit types to default verification profiles.
**Source:** SR-DIRECTIVE.KIT-ORACLES_PROFILES_MATRIX.yaml

#### Fields

| Field | Type | Description |
|-------|------|-------------|
| `work_unit_type` | string | Type of work unit |
| `deliverable_ids[]` | array | Associated deliverable IDs |
| `default_profile_id` | string | Default verification profile |
| `override_rules[]` | array | Conditions for overriding default |

---

## 5. Semantic Set Configuration

### 5.1 Semantic Set

**Type Key:** `config.semantic_set`
**Purpose:** Define meaning-matrix / manifold for semantic oracle evaluation.
**Source:** SR-SEMANTIC-ORACLE-SPEC §4

#### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `semantic_set_id` | string | Unique identifier |
| `name` | string | Human-readable name |
| `axes[]` | array | Semantic axis definitions |
| `constraints[]` | array | Semantic constraint specifications |
| `decision_rule` | object | Pass/fail derivation rule |

### 5.2 Semantic Axis

| Field | Type | Description |
|-------|------|-------------|
| `axis_id` | string | Identifier |
| `name` | string | Human name |
| `weight` | float | Weight in composite scoring (0.0-1.0) |
| `required` | boolean | Whether required for pass/fail |
| `min_coverage` | float | Minimum coverage threshold |
| `max_residual` | float | Maximum allowed residual |

### 5.3 Decision Rule

| Field | Type | Description |
|-------|------|-------------|
| `max_residual_norm` | float | Maximum residual norm for PASS |
| `min_coverage` | float | Minimum required coverage |
| `max_error_violations` | integer | Maximum error-level violations allowed |
| `max_warning_violations` | integer | Maximum warning-level violations |

#### Example

```yaml
semantic_set_id: semset:research-memo-v1
name: Research Memo Semantic Set
axes:
  - axis_id: clarity
    name: Clarity
    weight: 0.3
    required: true
    min_coverage: 0.8
    max_residual: 0.15
  - axis_id: completeness
    name: Completeness
    weight: 0.4
    required: true
    min_coverage: 0.9
    max_residual: 0.1
  - axis_id: actionability
    name: Actionability
    weight: 0.3
    required: true
    min_coverage: 0.7
    max_residual: 0.2
constraints:
  - constraint_id: has_recommendation
    constraint_type: required
    severity: error
    expression: "sections.recommendation.present"
decision_rule:
  max_residual_norm: 0.2
  min_coverage: 0.8
  max_error_violations: 0
  max_warning_violations: 3
```

---

## 6. Gate Configuration

### 6.1 Gate Definition

**Purpose:** Define enforcement checkpoints with evidence requirements.
**Source:** SR-DIRECTIVE.KIT-GATE-REGISTRY.md

#### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `gate_id` | string | Identifier (format: `G-<NAME>`) |
| `decision_purpose` | string | What this gate decides |
| `membranes_enforced[]` | array | Trust boundaries enforced |
| `enforcement_mechanism` | string | How gate is enforced |
| `failure_conditions[]` | array | Conditions that fail the gate |
| `stop_triggers[]` | array | Stop triggers this gate can fire |
| `relief_valves[]` | array | How failures can be relieved |
| `routing_portal` | string | Portal for relief routing |

#### Example Gate Categories

| Category | Example Gates |
|----------|---------------|
| Architecture | `G-DOMAIN-PURITY`, `G-EVT-APPENDONLY`, `G-PROJECTIONS-REBUILDABLE` |
| Verification | `G-VERIFIED-STRICT`, `G-CI-GREEN`, `G-BUILD-REPRO` |
| Integrity | `G-ORACLE-INTEGRITY`, `G-ORACLE-ENV-PINNED`, `G-ORACLE-SANDBOXED` |
| Portal | `G-AUTHZ-HUMAN-BINDING`, `G-ENG-APPROVED`, `G-RELEASE-APPROVED` |

---

## 7. Portal Configuration

### 7.1 Portal Playbook

**Purpose:** Define human judgment portals with request types and procedures.
**Source:** SR-DIRECTIVE.KIT-PORTAL-PLAYBOOKS.md

#### Seeded Portals

| Portal ID | Purpose |
|-----------|---------|
| `HumanAuthorityExceptionProcess` | Exceptions, deferrals, waivers, budget extensions |
| `GovernanceChangePortal` | Policy/spec/oracle suite/profile changes |
| `ReleaseApprovalPortal` | Release approval and Freeze Record creation |

#### Playbook Fields

| Field | Type | Description |
|-------|------|-------------|
| `portal_id` | string | Stable identifier |
| `portal_kind` | string | Type (exception_approval, governance_approval, release_approval) |
| `request_types[]` | array | Allowed request types with descriptions |
| `actor_rules` | object | Who can submit/approve (HUMAN only) |
| `preconditions[]` | array | Required conditions before acceptance |
| `evidence_review_checklist[]` | array | Mandatory reviewer confirmations |
| `decision_procedure` | object | How to make decisions per request type |
| `outputs[]` | array | Binding records emitted |

#### Request Types by Portal

**HumanAuthorityExceptionProcess:**
- `WAIVER_ORACLE_FAIL`
- `DEFERRAL`
- `DEVIATION`
- `ENG_ACCEPT`
- `BUDGET_ESCALATION`

**GovernanceChangePortal:**
- `ORACLE_SUITE_OR_PROFILE_CHANGE`
- `GOVERNANCE_CHANGE`
- `PROCEDURE_CHANGE`

**ReleaseApprovalPortal:**
- `RELEASE_APPROVAL`

---

## 8. Execution Policy Templates

### 8.1 Budget Configuration

**Purpose:** Resource constraints per work unit.
**Source:** SR-DIRECTIVE §4.1

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_iterations` | integer | 5 | Maximum iterations per work unit |
| `max_oracle_runs` | integer | 25 | Maximum oracle runs total |
| `max_wallclock_hours` | integer | 16 | Maximum execution time |

---

### 8.2 Stop Trigger Registry

**Purpose:** Conditions that halt execution and require human decision.
**Source:** SR-DIRECTIVE §4.2

| Trigger | Description | Routing |
|---------|-------------|---------|
| `BUDGET_EXHAUSTED` | Resource limits exceeded | HumanAuthorityExceptionProcess |
| `ORACLE_GAP` | Required oracle missing | GovernanceChangePortal |
| `ORACLE_FLAKE` | Non-deterministic required oracle | GovernanceChangePortal |
| `ORACLE_TAMPER` | Suite changed during run | GovernanceChangePortal |
| `ORACLE_ENV_MISMATCH` | Environment constraint violated | GovernanceChangePortal |
| `EVIDENCE_MISSING` | Referenced evidence unavailable | GovernanceChangePortal |
| `REPEATED_FAILURE` | N ≥ 3 consecutive failures | HumanAuthorityExceptionProcess |
| `INTEGRITY_VIOLATION` | Contract invariant violated | GovernanceChangePortal |
| `NO_ELIGIBLE_WORK` | No work units ready | HumanAuthorityExceptionProcess |
| `WORK_SURFACE_MISSING` | Intake/procedure/stage absent | GovernanceChangePortal |
| `STAGE_UNKNOWN` | Stage not in procedure template | GovernanceChangePortal |
| `SEMANTIC_PROFILE_MISSING` | Required semantic profile undefined | GovernanceChangePortal |

---

### 8.3 Gating Policy

**Type Key:** `config.gating_policy`
**Purpose:** Configure human judgment hooks per work unit.
**Source:** SR-SPEC §3.2.1.3

#### Hook Classes

| Hook | Purpose |
|------|---------|
| `plan_review` | Non-binding plan review |
| `evaluation_on_verification` | Human review of oracle evidence |
| `assessment_on_validation` | Human assessment of fitness |
| `closeout` | Final approval via Portal |

#### Gating Modes

| Mode | Behavior |
|------|----------|
| `soft` | Iterations proceed but deficit surfaced |
| `hard` | Iterations blocked without required records |
| `hybrid` | Soft by default, hard on triggers |

#### Deterministic Triggers

- `EXCEPTIONS_ACTIVE`
- `OPEN_RISK_HIGH`
- `REPEATED_FAILURE`
- `BUDGET_NEAR_EXHAUSTED`
- `GOVERNANCE_TOUCH`
- `CLOSEOUT_PENDING`

---

## 9. Iteration Context Templates

### 9.1 Required Refs Schema

**Purpose:** Define what must be referenced in `IterationStarted.refs[]`.
**Source:** SR-SPEC §3.2.1.1

#### Required Reference Categories

| # | Category | Kind | Rel | Required |
|---|----------|------|-----|----------|
| 1 | Loop | `Loop` | `in_scope_of` | Yes |
| 2 | Governing artifacts | `GovernedArtifact` | `depends_on` | Yes |
| 3 | Prior iteration summaries | `Iteration` | `depends_on` | If not first |
| 4 | Base candidate | `Candidate` | `depends_on` | If iterating |
| 5 | Oracle suite | `OracleSuite` | `depends_on` | Yes |
| 6 | Active exceptions | `Deviation\|Deferral\|Waiver` | `depends_on` | If applicable |
| 7 | Intervention notes | `Record` | `depends_on` | If applicable |
| 8 | Agent definition | `GovernedArtifact` | `supported_by` | Yes |
| 9 | Gating policy | `GovernedArtifact` | `supported_by` | Yes |
| 10 | Intake | `Intake` | `depends_on` | Yes (semantic) |
| 11 | Procedure Template | `ProcedureTemplate` | `depends_on` | Yes (semantic) |

---

### 9.2 Iteration Summary Schema

**Purpose:** Structured output of each iteration.
**Source:** SR-SPEC §3.2.2

#### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `iteration_id` | string | Iteration identifier |
| `loop_id` | string | Parent loop identifier |
| `intent` | string | What was attempted |
| `actions[]` | array | Structured action records |
| `artifacts_touched[]` | array | File paths or artifact refs |
| `candidates_produced[]` | array | Candidate references |
| `runs_executed[]` | array | Run references |
| `outcomes` | object | Oracle results summary |
| `next_steps[]` | array | Machine-schedulable next steps |
| `open_risks[]` | array | Identified risks |

#### Action Shape

```json
{
  "kind": "code_change|doc_change|config_change|analysis|run_oracles",
  "summary": "string",
  "artifacts": ["path1", "path2"]
}
```

#### Outcomes Shape

```json
{
  "oracle_results": [
    { "run_id": "...", "oracle_suite_id": "...", "status": "PASS|FAIL", "evidence_refs": [] }
  ],
  "stop_triggers_fired": []
}
```

---

## 10. Evidence Templates

### 10.1 Evidence Bundle Manifest

**Type Key:** `domain.evidence_bundle` (manifest `artifact_type`: `evidence.gate_packet`)
**Purpose:** Structured oracle output for verification.
**Source:** SR-SPEC §1.9.1

#### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `artifact_type` | string | `evidence.gate_packet` |
| `artifact_version` | string | `v1` |
| `candidate_id` | string | Candidate being evaluated |
| `run_id` | string | Run identifier |
| `oracle_suite_id` | string | Suite used |
| `oracle_suite_hash` | string | Suite hash |
| `procedure_template_id` | string | Procedure (semantic work) |
| `stage_id` | string | Stage (semantic work) |
| `results[]` | array | Per-oracle results |
| `context` | object | Environment and governed refs |
| `produced_at` | string | ISO 8601 timestamp |

---

### 10.2 Semantic Evaluation Result

**Schema:** `sr.semantic_eval.v1`
**Purpose:** Structured semantic oracle output.
**Source:** SR-SEMANTIC-ORACLE-SPEC §4

#### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `schema` | string | `sr.semantic_eval.v1` |
| `candidate_id` | string | Candidate evaluated |
| `procedure_template_id` | string | Procedure template |
| `stage_id` | string | Stage evaluated |
| `oracle_suite_id` | string | Suite used |
| `oracle_suite_hash` | string | Suite hash |
| `semantic_set` | object | `{semantic_set_id, semantic_set_hash}` |
| `metrics` | object | `{residual_norm, coverage, violations}` |
| `decision` | object | `{status, rule_id, thresholds}` |

---

## 11. Release Templates

### 11.1 Freeze Record

**Type Key:** `record.freeze`
**Purpose:** Baseline snapshot for declaring Shippable.
**Source:** SR-SPEC §1.12

#### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `freeze_id` | string | Identifier (format: `freeze_<ULID>`) |
| `baseline_id` | string | Human-meaningful name |
| `candidate_id` | string | Released candidate |
| `verification` | object | Mode, suite id/hash, evidence refs, waivers |
| `release_approval_id` | string | Release Portal approval reference |
| `artifact_manifest[]` | array | Governed artifacts in force |
| `active_exceptions[]` | array | In-scope exceptions |
| `frozen_by` | object | Actor identity |
| `frozen_at` | string | ISO 8601 timestamp |

---

## 12. Exception Templates

### 12.1 Waiver

**Type Key:** `record.waiver`
**Purpose:** Permission to proceed despite oracle FAIL.
**Source:** SR-SPEC §1.14

#### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `exception_id` | string | Identifier (format: `exc_<ULID>`) |
| `kind` | string | `WAIVER` |
| `oracle_failure_ref` | string | Specific oracle FAIL being waived |
| `scope` | string | `per-candidate`, `per-loop`, `per-baseline`, or time-boxed |
| `risk_mitigation` | string | Documented risk and mitigation |
| `resolution_criteria` | string | How to resolve/expire |
| `expiry_date` | string | Review/expiry date |
| `approved_by` | object | Human actor identity |
| `approved_at` | string | ISO 8601 timestamp |

**Constraint:** Waivers MUST NOT target integrity conditions (`ORACLE_TAMPER`, `ORACLE_GAP`, `ORACLE_ENV_MISMATCH`, `ORACLE_FLAKE`, `EVIDENCE_MISSING`).

---

### 12.2 Deviation

**Type Key:** `record.deviation`
**Purpose:** Exception from a governed requirement.
**Source:** SR-CONTRACT §2.7

#### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `exception_id` | string | Identifier |
| `kind` | string | `DEVIATION` |
| `requirement_ref` | string | Governed requirement being deviated from |
| `scope` | string | Bounded scope |
| `justification` | string | Why deviation is necessary |
| `risk_mitigation` | string | Risk and mitigation |
| `resolution_criteria` | string | How to resolve |
| `approved_by` | object | Human actor identity |
| `approved_at` | string | ISO 8601 timestamp |

---

### 12.3 Deferral

**Type Key:** `record.deferral`
**Purpose:** Binding postponement of work or requirement.
**Source:** SR-CONTRACT §2.7

#### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `exception_id` | string | Identifier |
| `kind` | string | `DEFERRAL` |
| `subject_ref` | string | What is being deferred |
| `scope` | string | Bounded scope |
| `justification` | string | Why deferral is necessary |
| `target_date` | string | When deferred work should be addressed |
| `approved_by` | object | Human actor identity |
| `approved_at` | string | ISO 8601 timestamp |

---

## 13. Quick Reference Matrix

| Category | Template | Type Key | Authoritative Source |
|----------|----------|----------|----------------------|
| Work Surface | Intake | `record.intake` | SR-WORK-SURFACE §3 |
| Work Surface | Procedure Template | `config.procedure_template` | SR-WORK-SURFACE §4 |
| Work Surface | Work Surface Instance | `domain.work_surface` | SR-WORK-SURFACE §5 |
| Oracle | Oracle Suite Definition | — | SR-DIRECTIVE.KIT-ORACLES |
| Oracle | Oracle Definition | — | SR-DIRECTIVE.KIT-ORACLES |
| Oracle | Semantic Oracle | — | SR-SEMANTIC-ORACLE-SPEC |
| Verification | Verification Profile | — | SR-DIRECTIVE §5.2 |
| Semantic | Semantic Set | `config.semantic_set` | SR-SEMANTIC-ORACLE-SPEC §4 |
| Gate | Gate Definition | — | SR-DIRECTIVE.KIT-GATE-REGISTRY |
| Portal | Portal Playbook | — | SR-DIRECTIVE.KIT-PORTAL-PLAYBOOKS |
| Execution | Budget Configuration | — | SR-DIRECTIVE §4.1 |
| Execution | Gating Policy | `config.gating_policy` | SR-SPEC §3.2.1.3 |
| Context | Required Refs | — | SR-SPEC §3.2.1.1 |
| Context | Iteration Summary | — | SR-SPEC §3.2.2 |
| Evidence | Evidence Bundle | `domain.evidence_bundle` | SR-SPEC §1.9.1 |
| Evidence | Semantic Eval Result | `sr.semantic_eval.v1` | SR-SEMANTIC-ORACLE-SPEC §4 |
| Release | Freeze Record | `record.freeze` | SR-SPEC §1.12 |
| Exception | Waiver | `record.waiver` | SR-SPEC §1.14 |
| Exception | Deviation | `record.deviation` | SR-CONTRACT §2.7 |
| Exception | Deferral | `record.deferral` | SR-CONTRACT §2.7 |

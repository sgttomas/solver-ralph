---
doc_id: SR-DIRECTIVE
doc_kind: governance.dev_directive
layer: build
status: draft
refs:
- rel: governed_by
  to: SR-CHANGE
- rel: depends_on
  to: SR-CONTRACT
- rel: depends_on
  to: SR-SPEC
- rel: depends_on
  to: SR-TYPES
- rel: depends_on
  to: SR-WORK-SURFACE
- rel: depends_on
  to: SR-TEMPLATES
- rel: depends_on
  to: SR-EVENT-MANAGER
- rel: depends_on
  to: SR-SEMANTIC-ORACLE-SPEC
- rel: depends_on
  to: SR-AGENT-WORKER-CONTRACT
- rel: supported_by
  to: SR-DIRECTIVE.KIT-ORACLES_PROFILES_MATRIX.yaml
- rel: supported_by
  to: SR-DIRECTIVE.KIT-PORTAL-PLAYBOOKS.md
- rel: supported_by
  to: SR-DIRECTIVE.KIT-GATE-REGISTRY.md
- rel: supported_by
  to: SR-DIRECTIVE.KIT-PLAN-TO-WORKFLOW-MAPPING.md
- rel: informs
  to: SR-PLAN
- rel: informs
  to: SR-EXCEPTIONS
---

# SR-DIRECTIVE (Semantic Ralph Loop Execution)

> This directive is assembled by filling the kit tables and keeping prose thin.

## 0. Change log

- 2026-01-11: initial assembly from filled kit artifacts (Steps 1–4).
- 2026-01-12: extended directive to support **Semantic Ralph Loops** (knowledge work via Work Surfaces + stage-gated procedures + semantic oracle suites) while preserving the existing build-plan guidance tables.
- 2026-01-13: extracted oracle suites/profiles/matrix kit YAML to `SR-DIRECTIVE.KIT-ORACLES_PROFILES_MATRIX.yaml` to reduce SR-DIRECTIVE context footprint (no semantic change).
- 2026-01-13: extracted portal playbooks, Gate Registry, and Plan-to-Workflow mapping tables to kit artifacts to reduce SR-DIRECTIVE context footprint (no semantic change).


## 1. Scope and authority

### Terminology and routing normalization

**Terminology guardrail:** The non-canonical phrase “evidence packet(s)” is deprecated. Use **Evidence Bundle** for the domain object, and use `artifact_type = evidence.gate_packet` only for the manifest carrier.

- **Evidence Bundle** is the domain object/type (`domain.evidence_bundle`). In the Evidence Bundle manifest schema, the JSON field `artifact_type` uses `evidence.gate_packet`.
- Only the seeded **Portal** identities are used as routing targets. Additional needs are expressed as **request types** within those portals.
  - Seeded portals: `HumanAuthorityExceptionProcess`, `GovernanceChangePortal`, `ReleaseApprovalPortal`.


### 1.1 Purpose

Define *how* a governed SR-PLAN instance is executed using SR-SPEC mechanics and SR-CONTRACT invariants, with an emphasis on producing trustworthy outputs.

This directive governs two execution modes:

1) **Build-plan execution** (the original intent): software engineering work where code/tests are primary oracles.
2) **Semantic Ralph Loops** (new): knowledge work executed via a **Work Surface** (Intake + Procedure Template) and **stage-gated semantic oracle suites** (meaning-matrix/semantic-set evaluations) rather than assuming compilation/tests.

This document constrains *process and enforcement surfaces* only. It does not redefine platform semantics (SR-CONTRACT/SR-TYPES/SR-SPEC).

### 1.2 Non-goals

- SR-DIRECTIVE does not redefine binding semantics.
- SR-DIRECTIVE does not invent new portal kinds. (Shorthand identifiers that appear in gate tables are **request_type** values within seeded portals, not new portals.)
- SR-DIRECTIVE does not modify the SR-PLAN deliverable inventory.

### 1.3 Precedence and conflict resolution

- If SR-DIRECTIVE conflicts with SR-CONTRACT or SR-SPEC, SR-CONTRACT/SR-SPEC control.
- If a new binding semantic is needed, route through SR-CHANGE.

## 2. Execution model

### 2.1 Canonical loop

A work unit proceeds through the following skeleton (expressed using SR-SPEC objects/events):

1. **IterationStarted** with required refs (governed artifacts + selected oracle suite(s) + any base candidate).
2. **CandidateSubmitted** (or candidate materialized) for the work unit scope.
3. **OracleRunRequested/OracleRunCompleted** for each oracle in the required suite.
4. **EvidenceBundleRecorded**: a single `evidence.gate_packet` manifest aggregating runs, environment fingerprint, suite hash.
5. **Gate evaluation** (Start → Accept → Release as applicable) using the Gate Registry.
6. **Portal crossing** (human-only) when a gate is blocked and relief is permitted:
 - HumanAuthorityExceptionProcess (exceptions/deferrals/ENG_ACCEPT/budget escalation as request types)
 - GovernanceChangePortal (governance/policy/suite/profile changes)
 - ReleaseApprovalPortal (release approvals tied to freeze)
7. **FreezeRecordCreated** (human-only) when release criteria are satisfied and approval acknowledges exceptions.

### 2.2 Concurrency policy

- Multiple work units may run concurrently if they do not violate dependency ordering.
- Each work unit must carry its own Evidence Bundles; cross-talk is forbidden.
- Budgets apply per work unit unless the Plan-to-Workflow mapping overrides them.

### 2.3 Dependency-first scheduling and deterministic eligibility

- The authoritative dependency graph for execution is derived from the active SR-PLAN instance’s `depends_on` relationships, plus any binding deferrals/waivers recorded via Portals.
- Work units may only enter Accept/Release gates when all upstream dependencies are satisfied (or have explicit deferrals recorded).

**Deterministic eligibility (normative):**
- The system MUST compute the **eligible set** as a deterministic function of:
  - the SR-PLAN dependency graph, and
  - recorded completion / blocking events (EvidenceBundleRecorded + gate outcomes + stop triggers + portal decisions).
- This eligibility computation MUST be performed by a deterministic system component (the **Event Manager / Projection Builder**), not by the agent.
- The agent MAY choose any *one* eligible work unit to execute per iteration, but MUST record the selection rationale in iteration records.



### 2.4 Semantic Ralph Loop specialization (Work Surface + stage gates)

A Semantic Ralph Loop executes knowledge work through a **stage-gated procedure** rather than assuming the existence of code-level tests.

Normative requirements:

- Each semantic work unit MUST have a **Work Surface** consisting of:
  - an **Intake** (objective, scope, constraints, definitions, deliverables),
  - a **Procedure Template** (stages, required intermediate artifacts, and gate criteria),
  - the current `stage_id` targeted by the iteration,
  - the selected **oracle profile/suites** for that stage (including semantic set definitions when applicable).
- Each Iteration SHOULD target a single procedure stage (or a single stage transition) and MUST produce:
  - updated candidate artifacts for that stage, and
  - an Evidence Bundle (`evidence.gate_packet`) that binds results to (`candidate_id`, `procedure_template_id`, `stage_id`).
- A stage gate is considered passed only when the required oracle suites for that stage have recorded results and the gate decision rules evaluate to PASS (or are explicitly waived by a binding portal decision where permitted).

See SR-WORK-SURFACE and SR-TEMPLATES for the governed "work surface" schemas and template format.

## 3. Inputs and refs discipline

### 3.1 Required IterationStarted refs

Each `IterationStarted` MUST include (as `depends_on` unless noted):

- **Governed platform docs**: SR-TYPES, SR-CONTRACT, SR-SPEC, SR-DIRECTIVE (with content hashes)
- **Active plan instance**: SR-PLAN-INSTANCE-* (content hash)
- **Work Surface refs (semantic work):**
  - Intake for the selected work unit
  - Procedure Template (and the current `stage_id`)
  - Selected oracle profile/suite(s) for the stage (suite id + hash; semantic suites MUST bind semantic set definitions via suite hash)
- **Candidate lineage**: optional base candidate ref (if iterating on an earlier candidate)
- Supporting (`supported_by`): current work-unit projection snapshot / eligibility computation output (the agent may read, but it is not authoritative)

Rationale: Iteration context and eligibility MUST be reconstructible from recorded artifacts; no ghost inputs.

### 3.2 depends_on vs supported_by policy

- `depends_on`: blocking dependencies; changes may cause staleness propagation and must block shippable status until resolved.
- `supported_by`: provenance/audit; does not cause staleness blocking.

## 4. Budgets and stop triggers

### 4.1 Budget policy

Default per-work-unit budget (unless overridden in the Plan-to-Workflow mapping `budgets_default`):

```json
{
 "max_iterations": 5,
 "max_oracle_runs": 25,
 "max_wallclock_hours": 16
}
```

Budget extensions (and any re-scope) are processed as request types in **HumanAuthorityExceptionProcess** and must be recorded as binding decisions.

### 4.2 Stop-the-line trigger registry

Stop triggers observed in the Gate Registry (build) and applicable to semantic work:

- BUDGET_EXHAUSTED
- EVIDENCE_MISSING (or ref cannot be fetched)
- INTEGRITY_VIOLATION
- ORACLE_ENV_MISMATCH
- ORACLE_FLAKE
- ORACLE_GAP (required oracle missing / suite missing)
- ORACLE_TAMPER (suite mismatch)
- REPEATED_FAILURE
- NO_ELIGIBLE_WORK (nothing eligible under dependency + blocking rules)
- WORK_SURFACE_MISSING (intake/procedure/stage context absent for a semantic work unit)
- STAGE_UNKNOWN (stage_id not defined in the bound procedure template)
- SEMANTIC_PROFILE_MISSING (required stage profile/suite not declared)

Stop triggers route to:
- **GovernanceChangePortal** when policy is implicated (integrity/authority boundaries, suite/profile changes, procedure template changes).
- **HumanAuthorityExceptionProcess** when the stop can be relieved by an explicit exception that does not waive integrity.

## 5. Verification profiles and oracle suites

The directive’s oracle suite definitions, verification profile definitions, and the profile selection matrix are maintained as a **filled kit artifact** to keep SR-DIRECTIVE thin.

Authoritative kit file:
- `SR-DIRECTIVE.KIT-ORACLES_PROFILES_MATRIX.yaml` (verbatim extract of the previously inlined kit YAML)

SR-DIRECTIVE only treats the following identifiers as normative stable names:

### 5.1 Oracle suite IDs
- `suite:SR-SUITE-GOV` — governance/metadata checks (schemas, IDs, lineage, refs).
- `suite:SR-SUITE-CORE` — core deterministic checks (meta/schema/build/unit/lint + integrity smoke).
- `suite:SR-SUITE-FULL` — core + integration/e2e + replay/rebuild proofs (SBOM advisory).
- `suite:SR-SUITE-SEMANTIC` — semantic evaluation oracles (semantic set binding, meaning-matrix checks).
- `suite:SR-SUITE-INTEGRATION` — integration tests (network-enabled; PostgreSQL, MinIO, NATS dependencies).

### 5.2 Verification profile IDs
- `GOV-CORE` → required_suites: [`suite:SR-SUITE-GOV`]
- `STRICT-CORE` → required_suites: [`suite:SR-SUITE-CORE`]
- `STRICT-FULL` → required_suites: [`suite:SR-SUITE-FULL`]

All profiles share these **non-waivable integrity conditions**: `ORACLE_TAMPER`, `ORACLE_GAP`, `ORACLE_ENV_MISMATCH`, `ORACLE_FLAKE`, `EVIDENCE_MISSING`.

### 5.3 Selection and change control
- Default profile selection follows the kit’s `profile_selection_matrix` unless the Plan-to-Workflow mapping in §8 explicitly overrides.
- Any change to suite/profile definitions (including suite hash changes) MUST be routed to `GovernanceChangePortal` with `request_type: ORACLE_SUITE_OR_PROFILE_CHANGE` and applied via SR-CHANGE.

## 6. Portals and human judgment hooks

Portal playbooks are maintained as filled kit artifacts to keep SR-DIRECTIVE thin.

Authoritative kit file:
- `SR-DIRECTIVE.KIT-PORTAL-PLAYBOOKS.md` (verbatim extract of the previously inlined portal playbooks)

Normative stable portal IDs (routing targets):
- `HumanAuthorityExceptionProcess`
- `GovernanceChangePortal`
- `ReleaseApprovalPortal`

Rule: Additional operational needs are expressed as **request types** within these seeded portals; SR-DIRECTIVE does not define new portal identities.

## 7. Gate registry

The Gate Registry (gate_id → enforcement surface, evidence requirements, stop triggers, and relief routing) is maintained as a filled kit artifact.

Authoritative kit file:
- `SR-DIRECTIVE.KIT-GATE-REGISTRY.md` (verbatim extract of the previously inlined Gate Registry table)

Gate evaluation MUST consult the Gate Registry. Any change to gate definitions, enforcement surfaces, or routing MUST be routed through `GovernanceChangePortal` with `request_type: GOVERNANCE_CHANGE` (via SR-CHANGE).

## 8. Plan-to-workflow mapping

The Plan-to-Workflow mapping (deliverable_id/work_unit_type → profiles, gates, required portals, budgets, overrides) is maintained as a filled kit artifact.

Authoritative kit file:
- `SR-DIRECTIVE.KIT-PLAN-TO-WORKFLOW-MAPPING.md` (verbatim extract of the previously inlined mapping table)

Budgets default to §4.1 unless overridden by this mapping.

## 9. Exceptions, deviations, deferrals, waivers

- Exceptions are always explicit records emitted via **HumanAuthorityExceptionProcess**.
- Waivers may only waive *oracle FAIL outcomes* and are never permitted to waive integrity conditions.
- Every exception MUST be scoped and SHOULD be time-boxed; release approval requires explicit acknowledgement of active exceptions.

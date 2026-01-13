---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-DIRECTIVE"
  type: "governance.dev_directive"
  title: "Development Directive (instance-1)"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "normative"
  authority_kind: "process"
  governed_by: ["SR-CHANGE"]
  created: "2026-01-11"
  updated: "2026-01-11"
  tags: ["directive", "execution", "governance", "instance-1"]
  refs:
    # REQUIRED (depends_on)
    - kind: "GovernedArtifact"
      id: "SR-TYPES@3.3.0-draft.8"
      rel: "depends_on"
      meta: { content_hash: "sha256:1d4fe783e4bf943913911d864ebe296246c87d154de6f44fb00ee6cfa2225147" }
    - kind: "GovernedArtifact"
      id: "SR-CONTRACT@1.1.0-draft.1"
      rel: "depends_on"
      meta: { content_hash: "sha256:0d72191fb36d375062eea7358b043dc9aa76ff93affc122803757775404aa20c" }
    - kind: "GovernedArtifact"
      id: "SR-SPEC@1.3.0-draft.1"
      rel: "depends_on"
      meta: { content_hash: "sha256:c14a45be626d931bb90bd0e8081a5d35a832e8ef7a52f361480d18b96c6b13df" }
    - kind: "GovernedArtifact"
      id: "SR-PLAN-INSTANCE-1@1.0.0-draft.3"
      rel: "depends_on"
      meta: { content_hash: "sha256:a88abdfb86192a42e76826ee076300ca211bbc82a8aa822fc286ae53651b5438" }
    # SUPPORTING (supported_by)
    - kind: "GovernedArtifact"
      id: "SR-ETT@1.1.0-draft.1"
      rel: "supported_by"
      meta: { content_hash: "sha256:17acacb67fa3186ad99f3e6d9db783f5e9af6a6795d3b5e9617db7e906b3f2b7" }
    - kind: "ProcessState"
      id: "PS-SOLVER-RALPH-INSTANCE-1-SAPS"
      rel: "supported_by"
      meta: { content_hash: "<sha256:process-state-hash>" }
---

# SR-DIRECTIVE (Instance 1)

> This directive is assembled by filling the kit tables and keeping prose thin.

## 0. Change log

- 2026-01-11: initial assembly from filled kit artifacts (Steps 1–4).

## 1. Scope and authority

### 1.1 Purpose

Define *how* SR‑PLAN instance‑1 is executed using SR‑SPEC mechanics while satisfying SR‑CONTRACT invariants; SR‑ETT is used to ensure each constraint is placed on an enforceable surface.

### 1.2 Non-goals

- SR‑DIRECTIVE does not redefine binding semantics.
- SR‑DIRECTIVE does not invent new portal kinds.
- SR‑DIRECTIVE does not modify the SR‑PLAN deliverable inventory.

### 1.3 Precedence and conflict resolution

- If SR‑DIRECTIVE conflicts with SR‑CONTRACT or SR‑SPEC, SR‑CONTRACT/SR‑SPEC control.
- If a new binding semantic is needed, route through SR‑CHANGE.

## 2. Execution model

### 2.1 Canonical loop

A work unit proceeds through the following skeleton (expressed using SR‑SPEC objects/events):

1. **IterationStarted** with required refs (governed artifacts + selected oracle suite(s) + any base candidate).
2. **CandidateSubmitted** (or candidate materialized) for the work unit scope.
3. **OracleRunRequested/OracleRunCompleted** for each oracle in the required suite.
4. **EvidenceBundleRecorded**: a single `evidence.gate_packet` manifest aggregating runs, environment fingerprint, suite hash.
5. **Gate evaluation** (Start → Accept → Release as applicable) using the Gate Registry.
6. **Portal crossing** (human-only) when a gate is blocked and relief is permitted:
   - ExceptionApprovalPortal (exceptions/deferrals/ENG_ACCEPT/budget escalation as request types)
   - GovernanceChangePortal (governance/policy/suite/profile changes)
   - ReleaseApprovalPortal (release approvals tied to freeze)
7. **FreezeRecordCreated** (human-only) when release criteria are satisfied and approval acknowledges exceptions.

### 2.2 Concurrency policy

- Multiple work units may run concurrently if they do not violate dependency ordering.
- Each work unit must carry its own evidence packets; cross-talk is forbidden.
- Budgets apply per work unit unless the Plan‑to‑Workflow mapping overrides them.

### 2.3 Dependency-first scheduling

- The Plan‑to‑Workflow mapping is the authoritative dependency graph for instance-1.
- Work units may only enter Accept gates when all upstream dependencies are satisfied (or have explicit deferrals recorded).

## 3. Inputs and refs discipline

### 3.1 Required IterationStarted refs

Each IterationStarted MUST include (as `depends_on` unless noted):
- SR‑TYPES, SR‑CONTRACT, SR‑SPEC, SR‑PLAN‑INSTANCE‑1 (pinned versions + hashes)
- Selected oracle suite(s) (suite id + hash)
- Optional: base candidate ref (if iterating on an earlier candidate)
- Supporting (`supported_by`): SR‑ETT, ProcessState

### 3.2 depends_on vs supported_by policy

- `depends_on`: blocking dependencies; changes may cause staleness propagation and must block shippable status until resolved.
- `supported_by`: provenance/audit; does not cause staleness blocking.

## 4. Budgets and stop triggers

### 4.1 Budget policy

Default per-work-unit budget (unless overridden in the Plan‑to‑Workflow mapping `budgets_default`):

```json
{
  "max_iterations": 5,
  "max_oracle_runs": 25,
  "max_wallclock_hours": 16
}
```

Budget extensions (and any re-scope) are processed as request types in **ExceptionApprovalPortal** and must be recorded as binding decisions.

### 4.2 Stop-the-line trigger registry

Stop triggers observed in the Gate Registry (instance-1):

- BUDGET_EXHAUSTED
- BUDGET_EXHAUSTED (if repeated)
- EVIDENCE_MISSING
- EVIDENCE_MISSING (if ref cannot be fetched)
- INTEGRITY_VIOLATION
- ORACLE_*
- ORACLE_* (as applicable)
- ORACLE_ENV_MISMATCH
- ORACLE_FLAKE
- ORACLE_GAP
- ORACLE_GAP (if suite missing)
- ORACLE_TAMPER
- ORACLE_TAMPER (if suite mismatch)
- REPEATED_FAILURE

Stop triggers route to:
- **GovernanceChangePortal** when policy is implicated (integrity/authority boundaries, suite/profile changes).
- **ExceptionApprovalPortal** when the stop can be relieved by an explicit exception that does not waive integrity.

## 5. Verification profiles and oracle suites

The authoritative profile and suite definitions are below (copied verbatim from the filled kit YAML):

```yaml
version: 1.0.0-draft.3
created: '2026-01-11'
updated: '2026-01-11'
oracle_suites:
- suite_id: suite:SR-SUITE-GOV
  suite_version: 0.1.0-draft.1
  description: Governance/metadata checks only (schemas, IDs, lineage).
  determinism_required: true
  environment_constraints:
    runner: ci
    network: disabled
    oci_image_digest: sha256:REPLACE_WITH_PINNED_DIGEST
    cpu_arch: amd64
    os: linux
    additional_constraints:
    - runtime=runsc
    - workspace_readonly=true
  oracles:
  - oracle_id: oracle:meta_validate
    classification: required
    purpose: Validate governed document metadata (frontmatter schema, stable IDs/lineage).
    command: sr-oracles meta validate --all
    timeout_seconds: 600
    retries: 0
    expected_outputs:
    - path: reports/meta_validate.json
      media_type: application/json
      role: report
    - path: logs/meta_validate.log
      media_type: text/plain
      role: log
  - oracle_id: oracle:refs_validate
    classification: required
    purpose: Ensure declared refs are well-formed and dereferenceable where required.
    command: sr-oracles refs validate --all
    timeout_seconds: 600
    retries: 0
    expected_outputs:
    - path: reports/refs_validate.json
      media_type: application/json
      role: report
    - path: logs/refs_validate.log
      media_type: text/plain
      role: log
  flake_policy:
    on_required_flake: stop_the_line
    on_advisory_flake: warn_only
  evidence_capture_policy:
    include_stdout: true
    include_stderr: true
    include_environment_fingerprint: true
    include_artifact_hashes: true
- suite_id: suite:SR-SUITE-CORE
  suite_version: 0.1.0-draft.1
  description: Core deterministic checks (meta/schema/build/unit/lint/format).
  determinism_required: true
  environment_constraints:
    runner: ci
    network: disabled
    oci_image_digest: sha256:REPLACE_WITH_PINNED_DIGEST
    cpu_arch: amd64
    os: linux
    additional_constraints:
    - runtime=runsc
    - workspace_readonly=true
  oracles:
  - oracle_id: oracle:meta_validate
    classification: required
    purpose: Validate governed document metadata (frontmatter schema, stable IDs/lineage).
    command: sr-oracles meta validate --all
    timeout_seconds: 600
    retries: 0
    expected_outputs:
    - path: reports/meta_validate.json
      media_type: application/json
      role: report
    - path: logs/meta_validate.log
      media_type: text/plain
      role: log
  - oracle_id: oracle:build
    classification: required
    purpose: Build all services and libraries in pinned CI environment.
    command: sr-oracles build --all
    timeout_seconds: 600
    retries: 0
    expected_outputs:
    - path: reports/build.json
      media_type: application/json
      role: report
    - path: logs/build.log
      media_type: text/plain
      role: log
  - oracle_id: oracle:unit_tests
    classification: required
    purpose: Run unit tests (fast, deterministic).
    command: sr-oracles test unit
    timeout_seconds: 600
    retries: 0
    expected_outputs:
    - path: reports/unit.json
      media_type: application/json
      role: report
    - path: logs/unit.log
      media_type: text/plain
      role: log
  - oracle_id: oracle:lint
    classification: required
    purpose: Run lint/format/type checks (no diffs allowed).
    command: sr-oracles lint --check
    timeout_seconds: 600
    retries: 0
    expected_outputs:
    - path: reports/lint.json
      media_type: application/json
      role: report
    - path: logs/lint.log
      media_type: text/plain
      role: log
  - oracle_id: oracle:schema_validate
    classification: required
    purpose: Validate schemas/migrations/types are consistent (no drift).
    command: sr-oracles schema validate
    timeout_seconds: 600
    retries: 0
    expected_outputs:
    - path: reports/schema.json
      media_type: application/json
      role: report
    - path: logs/schema.log
      media_type: text/plain
      role: log
  - oracle_id: oracle:integrity_smoke
    classification: required
    purpose: Smoke-test oracle integrity enforcement (gap/env mismatch/tamper/flake pathways).
    command: sr-oracles integrity smoke
    timeout_seconds: 600
    retries: 0
    expected_outputs:
    - path: reports/integrity_smoke.json
      media_type: application/json
      role: report
    - path: logs/integrity_smoke.log
      media_type: text/plain
      role: log
  flake_policy:
    on_required_flake: stop_the_line
    on_advisory_flake: warn_only
  evidence_capture_policy:
    include_stdout: true
    include_stderr: true
    include_environment_fingerprint: true
    include_artifact_hashes: true
- suite_id: suite:SR-SUITE-FULL
  suite_version: 0.1.0-draft.1
  description: Core + integration/e2e + replay/determinism checks for self-hosted stack.
  determinism_required: true
  environment_constraints:
    runner: ci
    network: restricted
    oci_image_digest: sha256:REPLACE_WITH_PINNED_DIGEST
    cpu_arch: amd64
    os: linux
    additional_constraints:
    - runtime=runsc
    - workspace_readonly=true
    - network_egress=blocked
    - allowed_hosts=localhost,127.0.0.1,::1
  oracles:
  - oracle_id: oracle:meta_validate
    classification: required
    purpose: Validate governed document metadata (frontmatter schema, stable IDs/lineage).
    command: sr-oracles meta validate --all
    timeout_seconds: 600
    retries: 0
    expected_outputs:
    - path: reports/meta_validate.json
      media_type: application/json
      role: report
    - path: logs/meta_validate.log
      media_type: text/plain
      role: log
  - oracle_id: oracle:build
    classification: required
    purpose: Build all services and libraries in pinned CI environment.
    command: sr-oracles build --all
    timeout_seconds: 600
    retries: 0
    expected_outputs:
    - path: reports/build.json
      media_type: application/json
      role: report
    - path: logs/build.log
      media_type: text/plain
      role: log
  - oracle_id: oracle:unit_tests
    classification: required
    purpose: Run unit tests (fast, deterministic).
    command: sr-oracles test unit
    timeout_seconds: 600
    retries: 0
    expected_outputs:
    - path: reports/unit.json
      media_type: application/json
      role: report
    - path: logs/unit.log
      media_type: text/plain
      role: log
  - oracle_id: oracle:lint
    classification: required
    purpose: Run lint/format/type checks (no diffs allowed).
    command: sr-oracles lint --check
    timeout_seconds: 600
    retries: 0
    expected_outputs:
    - path: reports/lint.json
      media_type: application/json
      role: report
    - path: logs/lint.log
      media_type: text/plain
      role: log
  - oracle_id: oracle:schema_validate
    classification: required
    purpose: Validate schemas/migrations/types are consistent (no drift).
    command: sr-oracles schema validate
    timeout_seconds: 600
    retries: 0
    expected_outputs:
    - path: reports/schema.json
      media_type: application/json
      role: report
    - path: logs/schema.log
      media_type: text/plain
      role: log
  - oracle_id: oracle:integration
    classification: required
    purpose: Stand up the self-host stack and run integration tests (API + stores).
    command: sr-oracles test integration --self-host
    timeout_seconds: 1800
    retries: 0
    expected_outputs:
    - path: reports/integration.json
      media_type: application/json
      role: report
    - path: logs/integration.log
      media_type: text/plain
      role: log
    - path: artifacts/stack_health.json
      media_type: application/json
      role: artifact
  - oracle_id: oracle:e2e
    classification: required
    purpose: Run deterministic end-to-end flows including portal submissions (non-binding).
    command: sr-oracles test e2e --self-host
    timeout_seconds: 3600
    retries: 0
    expected_outputs:
    - path: reports/e2e.json
      media_type: application/json
      role: report
    - path: logs/e2e.log
      media_type: text/plain
      role: log
    - path: artifacts/e2e_transcript.ndjson
      media_type: application/x-ndjson
      role: artifact
  - oracle_id: oracle:replay_verify
    classification: required
    purpose: "Replay event stream \u2192 reconstruct state \u2192 compare deterministic checksum."
    command: sr-oracles replay verify
    timeout_seconds: 1800
    retries: 0
    expected_outputs:
    - path: reports/replay.json
      media_type: application/json
      role: report
    - path: logs/replay.log
      media_type: text/plain
      role: log
    - path: artifacts/state_checksum.txt
      media_type: text/plain
      role: artifact
  - oracle_id: oracle:integrity_smoke
    classification: required
    purpose: Smoke-test oracle integrity enforcement (gap/env mismatch/tamper/flake pathways).
    command: sr-oracles integrity smoke
    timeout_seconds: 600
    retries: 0
    expected_outputs:
    - path: reports/integrity_smoke.json
      media_type: application/json
      role: report
    - path: logs/integrity_smoke.log
      media_type: text/plain
      role: log
  - oracle_id: oracle:sbom
    classification: advisory
    purpose: Produce an SBOM for the oracle image + workspace (non-blocking).
    command: sr-oracles sbom generate
    timeout_seconds: 1200
    retries: 0
    expected_outputs:
    - path: reports/sbom.json
      media_type: application/json
      role: report
    - path: logs/sbom.log
      media_type: text/plain
      role: log
  flake_policy:
    on_required_flake: stop_the_line
    on_advisory_flake: warn_only
  evidence_capture_policy:
    include_stdout: true
    include_stderr: true
    include_environment_fingerprint: true
    include_artifact_hashes: true
verification_profiles:
- profile_id: GOV-CORE
  profile_version: 0.1.0-draft.1
  description: Governance-only profile (metadata/refs coherence).
  required_suites:
  - suite:SR-SUITE-GOV
  advisory_suites: []
  verification_mode_default: STRICT
  waiver_policy:
    allow_with_exceptions: false
    waiver_eligible_failures: []
    non_waivable_integrity_conditions:
    - ORACLE_TAMPER
    - ORACLE_GAP
    - ORACLE_ENV_MISMATCH
    - ORACLE_FLAKE
    - EVIDENCE_MISSING
- profile_id: STRICT-CORE
  profile_version: 0.1.0-draft.1
  description: Default profile for most implementation deliverables.
  required_suites:
  - suite:SR-SUITE-CORE
  advisory_suites: []
  verification_mode_default: STRICT
  waiver_policy:
    allow_with_exceptions: true
    waiver_eligible_failures:
    - BUILD_FAIL
    - UNIT_FAIL
    - LINT_FAIL
    - SCHEMA_FAIL
    non_waivable_integrity_conditions:
    - ORACLE_TAMPER
    - ORACLE_GAP
    - ORACLE_ENV_MISMATCH
    - ORACLE_FLAKE
    - EVIDENCE_MISSING
- profile_id: STRICT-FULL
  profile_version: 0.1.0-draft.1
  description: High-stakes profile (integration/e2e + replayability).
  required_suites:
  - suite:SR-SUITE-FULL
  advisory_suites: []
  verification_mode_default: STRICT
  waiver_policy:
    allow_with_exceptions: true
    waiver_eligible_failures:
    - BUILD_FAIL
    - UNIT_FAIL
    - LINT_FAIL
    - SCHEMA_FAIL
    - INTEGRATION_FAIL
    - E2E_FAIL
    non_waivable_integrity_conditions:
    - ORACLE_TAMPER
    - ORACLE_GAP
    - ORACLE_ENV_MISMATCH
    - ORACLE_FLAKE
    - EVIDENCE_MISSING
profile_selection_matrix:
- work_unit_type: adapter_eventstore
  deliverable_ids:
  - D-10
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: adapter_evidence_store
  deliverable_ids:
  - D-14
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: api_core_endpoints
  deliverable_ids:
  - D-18
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: api_evidence_endpoints
  deliverable_ids:
  - D-20
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: api_portals_and_governance_endpoints
  deliverable_ids:
  - D-19
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: api_scaffold_and_auth
  deliverable_ids:
  - D-17
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: bootstrap_init_scripts
  deliverable_ids:
  - D-32
  default_profile_id: STRICT-FULL
  override_rules: []
- work_unit_type: ci_substrate
  deliverable_ids:
  - D-03
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: context_compilation
  deliverable_ids:
  - D-08
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: dependency_graph_and_staleness
  deliverable_ids:
  - D-12
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: dev_tooling
  deliverable_ids:
  - D-04
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: domain_core_primitives
  deliverable_ids:
  - D-05
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: domain_core_state_machines
  deliverable_ids:
  - D-06
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: domain_ports_and_boundaries
  deliverable_ids:
  - D-07
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: e2e_harness_failure_modes
  deliverable_ids:
  - D-35
  default_profile_id: STRICT-FULL
  override_rules: []
- work_unit_type: e2e_harness_happy_path
  deliverable_ids:
  - D-34
  default_profile_id: STRICT-FULL
  override_rules: []
- work_unit_type: evidence_manifest_lib_and_oracle
  deliverable_ids:
  - D-15
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: governance_intervention_record
  deliverable_ids:
  - D-01
  default_profile_id: GOV-CORE
  override_rules: []
- work_unit_type: loop_governor_service
  deliverable_ids:
  - D-22
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: messaging_integration
  deliverable_ids:
  - D-21
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: ops_logging_observability
  deliverable_ids:
  - D-33
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: oracle_integrity_checks
  deliverable_ids:
  - D-27
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: oracle_runner_service
  deliverable_ids:
  - D-24
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: oracle_suite_core_implementation
  deliverable_ids:
  - D-25
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: oracle_suite_integration_e2e
  deliverable_ids:
  - D-26
  default_profile_id: STRICT-FULL
  override_rules: []
- work_unit_type: outbox_publisher_and_event_pub
  deliverable_ids:
  - D-13
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: persistence_schema
  deliverable_ids:
  - D-09
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: projection_builder
  deliverable_ids:
  - D-11
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: reference_worker_bridge
  deliverable_ids:
  - D-23
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: replayability_demonstration
  deliverable_ids:
  - D-36
  default_profile_id: STRICT-FULL
  override_rules: []
- work_unit_type: repo_scaffold
  deliverable_ids:
  - D-02
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: restricted_evidence_handling
  deliverable_ids:
  - D-16
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: self_host_deployment_stack
  deliverable_ids:
  - D-31
  default_profile_id: STRICT-FULL
  override_rules: []
- work_unit_type: ui_portal_workflows
  deliverable_ids:
  - D-30
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: ui_scaffold_and_login
  deliverable_ids:
  - D-28
  default_profile_id: STRICT-CORE
  override_rules: []
- work_unit_type: ui_views_and_evidence_viewer
  deliverable_ids:
  - D-29
  default_profile_id: STRICT-CORE
  override_rules: []

```

## 6. Portals and human judgment hooks

Only the portals with provided seed documents are defined for this instance; additional operational needs are expressed as **request types** within these portals (see playbooks).

### 6.1 ExceptionApprovalPortal

---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "ExceptionApprovalPortal"
  type: "config.portal_playbook"
  title: "ExceptionApprovalPortal — Playbook"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "binding"
  authority_kind: "config"
  created: "2026-01-11"
  updated: "2026-01-11"
  tags: ["sr-directive", "portal", "playbook"]
---

# ExceptionApprovalPortal — Playbook

## 1) Portal identification

- **portal_id:** `ExceptionApprovalPortal`
- **portal_kind:** `exception_approval`
- **scope:** `instance:PS-SOLVER-RALPH-INSTANCE-1-SAPS (all phases)`

## 2) Purpose and boundary

- **Purpose (one sentence):**  
  Provide a human-only, fully-audited relief valve for *explicit* oracle FAILs and time-boxed deviations/deferrals, without ever bypassing integrity conditions.

- **Trust boundary being crossed (SR-ETT membranes):**  
  Authority Boundary; Evidence Integrity; Change; Accountability

- **What this portal MUST NOT do:**  
  - Mint or modify verification evidence.
- Waive integrity conditions (EVIDENCE_MISSING, ORACLE_TAMPER, ORACLE_GAP, ORACLE_FLAKE, ORACLE_ENV_MISMATCH).
- Create implicit approvals (all outcomes must be emitted as binding records + events).

## 3) Allowed request types

The portal accepts the following request types (each produces a binding record):

- **WAIVER_ORACLE_FAIL** — waive a *required oracle FAIL outcome* for a specific scope (deliverable/work-unit/candidate), with explicit rationale, conditions, and expiry.
- **DEFERRAL** — defer a binding requirement to a later deliverable/phase with a time-box and compensating controls.
- **DEVIATION** — approve a deviation from a workflow requirement (e.g., CI shape) with compensating controls.
- **ENG_ACCEPT** — record engineering acceptance approval for plan deliverables that require human sign-off (treated as a request type here to stay within the 3 seeded portals).
- **BUDGET_ESCALATION** — approve a budget increase / scope change with explicit accounting and stop-trigger reassessment.


## 4) Actor rules

- **Binding actor kind:** `HUMAN` only (portal refuses non-human submissions for binding outcomes).
- **Minimum roles (suggested):**
  - `ExceptionReviewer` for WAIVER/DEFERRAL/DEVIATION
  - `EngineeringApprover` for ENG_ACCEPT
  - `BudgetApprover` for BUDGET_ESCALATION
- **Separation of duties (recommended):**
  - Requester SHOULD NOT be the sole approver for WAIVER/DEVIATION.
- **Attribution requirements:**
  - portal must record: human identity, timestamp, rationale, and linked evidence bundle IDs.


## 5) Preconditions

- **Integrity conditions are non-waivable.** If any integrity condition is active for the scope, the portal MUST refuse WAIVER/DEFERRAL/ENG_ACCEPT until the integrity condition is resolved (or escalated to GovernanceChangePortal if policy must change).
- Request must reference a concrete scope:
  - deliverable_id (D-##) and/or work_unit_id and/or candidate_id
  - gate_id(s) impacted
- Request must include links to the relevant evidence bundles (gate packets) and the current verification profile selection for that scope.
- For **BUDGET_ESCALATION**, request must include current budget burn and proposed new ceilings.


## 6) Evidence review checklist (Accountability harness)

Reviewers MUST confirm (and the portal SHOULD checklist):

1. **Scope clarity**
   - Which deliverable/work-unit/candidate is affected?
   - Which gate(s) are being relieved?
2. **Evidence completeness**
   - Evidence bundle manifest(s) present and validated (no EVIDENCE_MISSING).
   - Runs referenced include environment fingerprint + suite hash.
3. **Failure semantics**
   - Confirm the failure is an *oracle FAIL outcome* (waivable) vs an *integrity condition* (non-waivable).
4. **Compensating controls**
   - Additional tests / additional reviewers / narrower rollout / monitoring requirements.
5. **Expiry & revisit**
   - Explicit expiry date/time or “next deliverable by-id” for deferrals.
6. **Staleness impact**
   - Whether the exception creates downstream staleness and how it will be routed.


## 7) Decision procedure (what to do)

**WAIVER_ORACLE_FAIL**
- Verify: requested relief is ONLY for explicit oracle FAIL(s).
- Require: rationale, conditions, expiry, and scope narrowing (prefer smallest scope).
- Emit: `GateWaiverRecorded` with:
  - gate_id, oracle_id(s), suite_id+version, run_id(s), scope refs, rationale, conditions, expiry, approver identity.

**DEFERRAL**
- Require: target deliverable/phase, time-box, and compensating controls.
- Emit: `DeferralRecorded` with scope + destination + controls.

**DEVIATION**
- Require: exact workflow delta and compensating controls.
- Emit: `DeviationRecorded`.

**ENG_ACCEPT**
- Require: acceptance checklist completed for the deliverable(s), including links to required evidence bundles.
- Emit: `ApprovalRecorded(kind=ENG_ACCEPT)`.

**BUDGET_ESCALATION**
- Require: updated budget ceilings, justification, and stop-trigger policy re-check.
- Emit: `DecisionRecorded(kind=BUDGET_ESCALATION)` + `BudgetPolicyUpdated` (if your event taxonomy includes it) OR a `DecisionRecorded` that carries the new ceilings.


## 8) Outputs (binding records emitted)

Portal MUST emit **binding records** (and corresponding events) — no implicit outcomes:

- `record.gate_waiver` + `event.GateWaiverRecorded`
- `record.deferral` + `event.DeferralRecorded`
- `record.deviation` + `event.DeviationRecorded`
- `record.approval(kind=ENG_ACCEPT)` + `event.ApprovalRecorded`
- `record.decision(kind=BUDGET_ESCALATION)` + `event.DecisionRecorded`

Each record MUST include:
- actor_kind = HUMAN
- stable identity reference for the approver
- linked evidence bundle IDs / run IDs
- scope refs (deliverable/work-unit/candidate)
- rationale + conditions + expiry (where applicable)


## 9) Failure handling and routing

- If the request attempts to waive an integrity condition → **REJECT** and instruct remediation; optionally escalate to GovernanceChangePortal if policy itself is inconsistent.
- If evidence bundles are missing/invalid → **REJECT** and route to remediation (integrity conditions).
- If scope is ambiguous → **REJECT**; request must be precise.
- If the request is actually a governance change (new gate, new profile, new portal policy) → **ROUTE** to **GovernanceChangePortal**.
- All rejections must be recorded as `DecisionRecorded(kind=REJECTED)` with rationale.


## 10) Auditability

- All requests, comments, and final outcomes are written to the event log.
- Portal must provide a stable audit view keyed by:
  - request_id, approver identity, gate_id(s), run_id(s), evidence_bundle_id(s), deliverable_id(s).
- Portal must not store “final authority” in UI state; authority lives in records/events.


## 11) Cross-references

- **SR-CONTRACT:** C-EXC-4, C-EXC-5 (waivers), C-EXC-2 (freeze must surface exceptions), C-TB-5 (human-only binding), C-DEC-1 (decisions).
- **SR-SPEC:** §1.9 (waiver scope + integrity conditions), §2.3.4 (Approvals/Decisions), Appendix C (integrity conditions).
- **SR-ETT:** Authority Boundary; Evidence Integrity harness membranes.



### 6.2 GovernanceChangePortal

---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "GovernanceChangePortal"
  type: "config.portal_playbook"
  title: "GovernanceChangePortal — Playbook"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "binding"
  authority_kind: "config"
  created: "2026-01-11"
  updated: "2026-01-11"
  tags: ["sr-directive", "portal", "playbook"]
---

# GovernanceChangePortal — Playbook

## 1) Portal identification

- **portal_id:** `GovernanceChangePortal`
- **portal_kind:** `governance_change`
- **scope:** `instance:PS-SOLVER-RALPH-INSTANCE-1-SAPS (governance artifacts + policy)`

## 2) Purpose and boundary

- **Purpose (one sentence):**  
  Provide an audited, human-only workflow to propose and ratify changes to governed policy (gates, profiles, portal policies, stop-trigger thresholds, and directive structure).

- **Trust boundary being crossed (SR-ETT membranes):**  
  Change; Authority Boundary; Evidence Integrity; Accountability

- **What this portal MUST NOT do:**  
  - Grant waivers for integrity conditions.
- Edit evidence bundles or alter run outcomes.
- Allow uncontrolled changes without versioning + lineage.

## 3) Allowed request types

Allowed request types (each produces a binding record):

- **GOVERNANCE_CHANGE** — change SR-DIRECTIVE policy prose, gate registry entries, or plan-to-workflow mapping.
- **ORACLE_SUITE_OR_PROFILE_CHANGE** — change oracle suite definitions or verification profile selection matrix.
- **PORTAL_POLICY_CHANGE** — adjust allowed request types, role rules, or checklists for portals.
- **STOPTRIGGER_POLICY_CHANGE** — change thresholds (e.g., REPEATED_FAILURE N), budgets policy, or escalation routing.


## 4) Actor rules

- **Binding actor kind:** `HUMAN` only.
- **Minimum roles (suggested):**
  - `GovernanceSteward` (required approver)
  - `EngineeringLead` (recommended co-approver for changes affecting enforcement)
- **Change control:**
  - All approved changes MUST be applied via new governed artifact versions (pinned IDs, supersedes lineage).


## 5) Preconditions

- Request must include:
  - target artifact(s) + version(s)
  - proposed change set (diff or structured patch)
  - impact analysis: which gates/workflows/deliverables are affected
  - migration plan (if any)
- If the change affects verification (suites/profiles), request must include:
  - new suite/profile definitions
  - determinism + environment pinning rationale
  - updated waiver policy implications
- If the change affects stop triggers/budgets, request must include:
  - updated thresholds/ceilings
  - failure-mode routing expectations


## 6) Evidence review checklist (Accountability harness)

Reviewers MUST confirm:

1. **Versioning + lineage**
   - new versions have stable IDs and a clear supersedes chain.
2. **Coherence**
   - proposed changes do not contradict SR-CONTRACT or SR-SPEC.
   - plan-to-workflow, gate registry, and profile definitions remain consistent.
3. **Enforceability**
   - each referenced gate has a realizable enforcement mechanism and evidence plan.
4. **Blast radius**
   - list deliverables/work-units affected and any migration steps.
5. **Integrity preserved**
   - no change permits bypassing integrity conditions.


## 7) Decision procedure (what to do)

- Triage:
  - If request is actually an exception/waiver for a single scope → route to ExceptionApprovalPortal.
  - If request is a release action → route to ReleaseApprovalPortal.
- For governance changes:
  1) Require a structured change proposal (diff/patch).
  2) Require at least one enforcement-owner sign-off when enforcement changes.
  3) Approve or reject.
- On approval, require:
  - new versions of affected governed artifacts
  - updated pinned references in SR-DIRECTIVE assembly


## 8) Outputs (binding records emitted)

Portal MUST emit binding records/events:

- `record.governance_change_request` + `event.GovernanceChangeRequested`
- `record.decision(kind=GOVERNANCE_CHANGE_APPROVAL|REJECTION)` + `event.DecisionRecorded`
- (When applicable) `record.governed_artifact_version` updates (new versions with supersedes lineage)

Each decision record MUST include:
- approver identity (HUMAN)
- rationale + change summary
- affected gate_ids / profile_ids / portal_ids
- migration notes (if any)


## 9) Failure handling and routing

- If proposal is incoherent with SR-CONTRACT/SR-SPEC → **REJECT** and record rationale.
- If proposal introduces unenforceable gates/profiles → **REJECT**.
- If proposal attempts to waive integrity conditions → **REJECT** and route to remediation.
- Rejections must be recorded; “silent decline” is forbidden.


## 10) Auditability

- Maintain an immutable change log: every request, comment, diff, and final decision is an event-linked artifact.
- Provide audit views:
  - by artifact id/version chain
  - by gate_id/profile_id touched
  - by approver identity


## 11) Cross-references

- **SR-CONTRACT:** C-META-* (versioning/lineage), C-EXC-* (exception policy boundaries), C-DEC-1 (decision recording), C-TB-5 (human binding).
- **SR-SPEC:** §3.3 (metadata/lineage), §2.3.5 (governed artifacts), §1.11 (verification profiles), Appendix A/C.
- **SR-ETT:** Change + Authority Boundary membranes.



### 6.3 ReleaseApprovalPortal

---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "ReleaseApprovalPortal"
  type: "config.portal_playbook"
  title: "ReleaseApprovalPortal — Playbook"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "binding"
  authority_kind: "config"
  created: "2026-01-11"
  updated: "2026-01-11"
  tags: ["sr-directive", "portal", "playbook"]
---

# ReleaseApprovalPortal — Playbook

## 1) Portal identification

- **portal_id:** `ReleaseApprovalPortal`
- **portal_kind:** `release_approval`
- **scope:** `instance:PS-SOLVER-RALPH-INSTANCE-1-SAPS (release baselines only)`

## 2) Purpose and boundary

- **Purpose (one sentence):**  
  Provide a human-only, audited release decision that references a FreezeRecord and explicitly acknowledges the verification posture and active exceptions.

- **Trust boundary being crossed (SR-ETT membranes):**  
  Authority Boundary; Change; Accountability; Event Integrity

- **What this portal MUST NOT do:**  
  - Change verification results.
- Release without a FreezeRecord.
- Hide or omit active exceptions from the release decision.

## 3) Allowed request types

Allowed request types:

- **RELEASE_APPROVAL** — approve publishing/shipping a baseline snapshot identified by `freeze_id`.
- **RELEASE_HOLD** — place a hold on a freeze_id with rationale (optional but useful).


## 4) Actor rules

- **Binding actor kind:** `HUMAN` only.
- **Minimum roles (suggested):**
  - `ReleaseApprover` (required)
  - `EngineeringApprover` (recommended co-approver for high-risk releases)
- Release approval MUST reference the approver’s identity and the exact freeze_id.


## 5) Preconditions

- A `FreezeRecord` exists for the proposed release baseline:
  - lists included candidates/artifacts by content hash
  - lists active exceptions/waivers/deferrals affecting included items
  - is itself content-addressed and recorded in the event log
- For every included candidate:
  - Verified(STRICT) computed and recorded OR explicitly listed as exception with scope/expiry
  - no active integrity conditions


## 6) Evidence review checklist (Accountability harness)

Reviewers MUST confirm:

1. **Freeze completeness**
   - freeze_id exists; artifact list complete; hashes present.
2. **Verification posture**
   - required suites PASS per profile OR covered by recorded waivers (FAIL only).
3. **Integrity conditions**
   - confirm none of the non-waivable integrity conditions are active for included items.
4. **Exceptions surfaced**
   - every waiver/deferral/deviation is listed and acknowledged.
5. **Rollback / recovery**
   - rebuild/replay evidence available (where applicable).


## 7) Decision procedure (what to do)

- Validate freeze_id and fetch its contents.
- Validate verification summary (computed) and integrity-condition summary.
- Validate exception acknowledgements:
  - approver must explicitly check “I acknowledge active exceptions listed in FreezeRecord”.
- Approve or hold:
  - On approval: emit `ApprovalRecorded(kind=RELEASE_APPROVAL)` referencing freeze_id.
  - On hold: emit `DecisionRecorded(kind=RELEASE_HOLD)` with rationale and scope.


## 8) Outputs (binding records emitted)

Portal MUST emit binding records/events:

- `record.approval(kind=RELEASE_APPROVAL)` + `event.ApprovalRecorded`
  - refs: freeze_id, included candidates, verification summary artifact, exception list
- `record.decision(kind=RELEASE_HOLD)` + `event.DecisionRecorded` (optional)

All outputs MUST be event-linked and attributable to a HUMAN approver.


## 9) Failure handling and routing

- If FreezeRecord missing/incomplete → **REJECT** and route to remediation (no release).
- If any non-waivable integrity condition is active → **REJECT** (stop-the-line).
- If verification evidence missing → **REJECT** (integrity).
- If governance changes are required to proceed → **ROUTE** to GovernanceChangePortal.


## 10) Auditability

- Release approvals are immutable records; UI state is non-binding.
- Provide audit views:
  - by freeze_id
  - by included candidates and evidence bundle IDs
  - by approver identity and timestamp


## 11) Cross-references

- **SR-CONTRACT:** C-SHIP-1 (Shippable), C-EXC-2 (freeze surfaces exceptions), C-TB-5 (human binding).
- **SR-SPEC:** §2.3.6 (Freeze), §1.12 (Shippable), Appendix C (integrity conditions).
- **SR-ETT:** Authority Boundary + Event Integrity membranes.



## 7. Gate registry

(Authoritative table for instance-1)

| gate_id                       | gate_name                                    | gate_kind        | applies_to     | purpose_decision                                                                                                             | membranes_enforced                                                     | enforced_surface_hex                        | enforcement_mechanism              | allowed_actor_kinds   | required_refs_depends_on                                                                                                 | required_commitment_objects                                                  | required_evidence                                              | verification_profile_or_suite                            | failure_conditions                                                                                                | stop_triggers_on_failure                                                                         | relief_valve                                                                 | routing_portal_on_block                                                                                             | notes                                                             | contract_refs                                  | spec_refs                                                  | plan_refs                                                                                                                                                                                               |
|:------------------------------|:---------------------------------------------|:-----------------|:---------------|:-----------------------------------------------------------------------------------------------------------------------------|:-----------------------------------------------------------------------|:--------------------------------------------|:-----------------------------------|:----------------------|:-------------------------------------------------------------------------------------------------------------------------|:-----------------------------------------------------------------------------|:---------------------------------------------------------------|:---------------------------------------------------------|:------------------------------------------------------------------------------------------------------------------|:-------------------------------------------------------------------------------------------------|:-----------------------------------------------------------------------------|:--------------------------------------------------------------------------------------------------------------------|:------------------------------------------------------------------|:-----------------------------------------------|:-----------------------------------------------------------|:--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| G-API-CONTRACTS               | API contracts satisfied                      | verification     | api            | Ensure governed API/record schemas match SR-TYPES/SR-SPEC expectations; prevent breaking changes without governance.         | Ontological; Change; Accountability                                    | ports; adapters; ci                         | schema_diff_check; ci_check        | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | candidate api schemas; SR-TYPES refs                                         | evidence.gate_packet (schema checks)                           | profile:STRICT-CORE                                      | Schema mismatch; breaking change; missing migration notes.                                                        | INTEGRITY_VIOLATION                                                                              | governance change                                                            | GovernanceChangePortal                                                                                              | nan                                                               | C-ARCH-1; C-CTX-2                              | SR-SPEC:record schemas; SR-TYPES                           | SR-PLAN(instance-1):D-17;D-18;D-19;D-20                                                                                                                                                                 |
| G-ASYNC-RELIABILITY           | Async reliability                            | verification     | runtime        | Ensure asynchronous processing delivers at-least-once and idempotent handling where required; no lost events.                | Accountability; Operational                                            | runtime; event_store                        | property_tests; chaos              | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | nan                                                                          | evidence.gate_packet (reliability report)                      | profile:STRICT-FULL                                      | Lost events; non-idempotent handling; ordering violations that change projections.                                | INTEGRITY_VIOLATION                                                                              | decision                                                                     | GovernanceChangePortal                                                                                              | nan                                                               | C-EVT-1; C-EVT-3; C-VER-3                      | SR-SPEC:projection rebuild + event handling                | SR-PLAN(instance-1):D-21;D-22;D-23                                                                                                                                                                      |
| G-AUTHZ-HUMAN-BINDING         | AuthZ: human-only binding                    | trust_boundary   | portal         | Ensure only humans can emit binding approvals/decisions/freezes; system/agents cannot.                                       | Authority & Integrity                                                  | portal; event_store                         | authz_check; record_validation     | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | record.ApprovalRecorded; record.DecisionRecorded; record.FreezeRecordCreated | identity attestation                                           | nan                                                      | Non-human actor emits binding record; missing attribution.                                                        | INTEGRITY_VIOLATION                                                                              | decision                                                                     | GovernanceChangePortal                                                                                              | nan                                                               | C-TB-1; C-TB-5; C-TB-6                         | SR-SPEC:human actor constraints                            | SR-PLAN(instance-1):D-17;D-18;D-19;D-20                                                                                                                                                                 |
| G-BUDGETS-STOPTRIGGERS        | Budgets + stop triggers configured           | operational      | loop           | Ensure loop budgets and stop triggers are configured per directive and enforced (hard stop on budget exhaustion).            | Operational; Accountability                                            | runtime                                     | config_check; runtime_guard        | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | config.budgets; config.stop_triggers                                         | evidence.gate_packet (budget config)                           | nan                                                      | Budgets missing; stop triggers missing; hard stop not enforced.                                                   | BUDGET_EXHAUSTED; INTEGRITY_VIOLATION                                                            | budget escalation                                                            | BudgetEscalationPortal                                                                                              | nan                                                               | C-LOOP-2; C-LOOP-3; C-LOOP-4                   | SR-SPEC:bounded iteration + stop triggers                  | SR-PLAN(instance-1):D-21;D-22;D-23                                                                                                                                                                      |
| G-BUILD-REPRO                 | Build reproducible                           | verification     | build          | Ensure builds are reproducible under pinned toolchain/env.                                                                   | Accountability; Operational                                            | ci; runtime                                 | ci_check; build_repro_test         | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | nan                                                                          | evidence.gate_packet (build report)                            | profile:STRICT-CORE                                      | Non-reproducible build outputs; toolchain mismatch; missing lockfiles.                                            | REPEATED_FAILURE                                                                                 | decision                                                                     | GovernanceChangePortal                                                                                              | nan                                                               | C-VER-3; C-CTX-1                               | SR-SPEC:deterministic context + env fingerprint            | SR-PLAN(instance-1):D-02;D-03;D-04                                                                                                                                                                      |
| G-CI-GREEN                    | CI green                                     | verification     | ci             | Ensure required CI jobs pass (unit/lint/typecheck/etc per profile).                                                          | Accountability                                                         | ci                                          | ci_check                           | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | nan                                                                          | evidence.gate_packet (ci run)                                  | profile:STRICT-CORE                                      | Any required CI job fails or missing.                                                                             | REPEATED_FAILURE                                                                                 | decision                                                                     | ExceptionApprovalPortal (only if allowed) or GovernanceChangePortal                                                 | nan                                                               | C-VER-1; C-EVID-4                              | SR-SPEC:oracle results recorded into evidence              | SR-PLAN(instance-1):D-02;D-03;D-04                                                                                                                                                                      |
| G-CTX-ADMISSIBLE              | Context admissible (no ghost inputs)         | work_start       | all            | Ensure IterationStarted context refs are sufficient, dereferenceable, and pinned; block on ambiguity.                        | Intent & Objective; Ontological; Accountability; Authority & Integrity | domain_core; driving_port                   | schema_validation; runtime_guard   | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | event.IterationStarted                                                       | none (this gate validates refs/structure prior to oracle runs) | nan                                                      | Missing required refs; refs not dereferenceable; ref hashes missing; wrong rel types; unknown governed artifacts. | EVIDENCE_MISSING (if ref cannot be fetched); ORACLE_TAMPER (if suite mismatch); REPEATED_FAILURE | decision                                                                     | GovernanceChangePortal (if governance refs mismatch); ExceptionApprovalPortal (if a bounded exception is requested) | nan                                                               | C-CTX-1; C-CTX-2; C-TB-2                       | SR-SPEC:IterationStarted refs + replayability constraints  | SR-PLAN(instance-1):D-01;D-02;D-03;D-04;D-05;D-06;D-07;D-08;D-09;D-10;D-11;D-12;D-13;D-14;D-15;D-16;D-17;D-18;D-19;D-20;D-21;D-22;D-23;D-24;D-25;D-26;D-27;D-28;D-29;D-30;D-31;D-32;D-33;D-34;D-35;D-36 |
| G-CTX-COMPILE-DETERMINISTIC   | Compile deterministic under context          | verification     | build          | Ensure compilation is deterministic given IterationStarted context and pinned environment.                                   | Accountability; Intent & Objective                                     | ci; runtime                                 | determinism_check                  | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | nan                                                                          | evidence.gate_packet (determinism report)                      | profile:STRICT-CORE                                      | Non-deterministic compile outputs; missing env fingerprint.                                                       | REPEATED_FAILURE                                                                                 | decision                                                                     | GovernanceChangePortal                                                                                              | nan                                                               | C-CTX-2; C-VER-3                               | SR-SPEC:environment_fingerprint + replay                   | SR-PLAN(instance-1):D-05;D-06;D-07;D-08                                                                                                                                                                 |
| G-DETERMINISM                 | Determinism upheld                           | verification     | runtime        | Ensure required deterministic behaviors hold (ordering, hash stability, replay).                                             | Accountability; Authority & Integrity                                  | domain_core; runtime; oracle_engine         | property_tests; replay_check       | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | nan                                                                          | evidence.gate_packet (determinism proof)                       | profile:STRICT-FULL                                      | Replay diverges; nondeterministic ordering; hash instability.                                                     | INTEGRITY_VIOLATION; REPEATED_FAILURE                                                            | decision                                                                     | GovernanceChangePortal                                                                                              | nan                                                               | C-VER-3; C-EVT-3; C-CTX-2                      | SR-SPEC:replayability + projections                        | SR-PLAN(instance-1):D-05;D-06;D-07;D-08                                                                                                                                                                 |
| G-DOMAIN-PURITY               | Domain purity                                | architecture     | domain_core    | Ensure domain core is infrastructure-free; boundary crossings occur via ports/adapters.                                      | Ontological; Authority & Integrity                                     | domain_core                                 | lint; static_analysis; arch_test   | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | candidate source tree                                                        | evidence.gate_packet (lint/arch results)                       | profile:STRICT-CORE                                      | Forbidden imports; infra dependencies in domain; port bypass.                                                     | REPEATED_FAILURE                                                                                 | governance change (if architecture rules must change)                        | GovernanceChangePortal                                                                                              | nan                                                               | C-ARCH-1; C-ARCH-2                             | SR-SPEC:hex separation principles                          | SR-PLAN(instance-1):D-05;D-06;D-07;D-08                                                                                                                                                                 |
| G-ENG-APPROVED                | Engineering acceptance approved              | portal           | work_unit      | Bind a human acceptance record to work-unit completion when required (non-release milestone).                                | Authority & Integrity; Accountability                                  | driving_port; portal                        | portal_review; record_validation   | HUMAN                 | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | record.DecisionRecorded or record.ApprovalRecorded                           | evidence.gate_packet                                           | nan                                                      | Missing acceptance record; record not bound to evidence; reviewer attribution missing.                            | EVIDENCE_MISSING                                                                                 | decision                                                                     | EngAcceptPortal                                                                                                     | nan                                                               | C-TB-3; C-TB-6                                 | SR-SPEC:DecisionRecorded / ApprovalRecorded                | SR-PLAN(instance-1):D-02;D-03;D-04;D-05;D-06;D-07;D-08;D-09;D-10;D-11;D-12;D-13;D-14;D-15;D-16;D-17;D-18;D-19;D-20;D-21;D-22;D-23;D-24;D-25;D-26;D-27;D-28;D-29;D-30;D-31;D-32;D-33;D-34;D-35;D-36      |
| G-EVID-CONTENT-ADDRESSED      | Evidence content addressed                   | evidence         | all            | Ensure evidence bundle is complete for the declared suite and mode (no missing required oracle results).                     | Accountability; Evidence; Authority & Integrity                        | oracle_engine; driving_port                 | oracle_suite; schema_validation    | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | event.EvidenceBundleRecorded                                                 | evidence.gate_packet                                           | nan                                                      | Required oracle has no recorded result; missing suite hash; missing environment fingerprint.                      | ORACLE_GAP; EVIDENCE_MISSING; REPEATED_FAILURE                                                   | decision                                                                     | GovernanceChangePortal                                                                                              | nan                                                               | C-EVID-4; C-OR-4                               | SR-SPEC:EvidenceBundleRecorded; required_oracles           | SR-PLAN(instance-1):D-14;D-15;D-16                                                                                                                                                                      |
| G-EVID-MANIFEST-V1            | Evidence manifest conforms to v1             | evidence         | all            | Validate evidence.gate_packet manifest schema and required fields for downstream audit/replay.                               | Accountability; Evidence                                               | driving_port; event_store                   | schema_validation                  | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | event.EvidenceBundleRecorded                                                 | evidence.gate_packet (manifest)                                | nan                                                      | Missing required fields; bad hashes; oracle_suite_hash mismatch; malformed run records.                           | ORACLE_GAP; EVIDENCE_MISSING                                                                     | decision                                                                     | GovernanceChangePortal (if schema mismatch implies governed meaning change)                                         | nan                                                               | C-EVID-1; C-EVID-2; C-EVID-3                   | SR-SPEC:evidence.gate_packet                               | SR-PLAN(instance-1):D-14;D-15;D-16                                                                                                                                                                      |
| G-EVID-SECRET-HANDLING        | Evidence secret handling (no leaks)          | integrity        | all            | Ensure evidence artifacts redact/avoid secrets and comply with governed secret-handling rules.                               | Leakage; Accountability                                                | oracle_engine; event_store; driving_port    | runtime_guard; scanner             | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | event.EvidenceBundleRecorded                                                 | evidence.gate_packet; redaction report                         | nan                                                      | Secrets detected in evidence; unredacted logs; prohibited content captured.                                       | INTEGRITY_VIOLATION; REPEATED_FAILURE                                                            | decision                                                                     | GovernanceChangePortal                                                                                              | nan                                                               | C-EVID-5; C-TB-2                               | SR-SPEC:evidence bundle storage + retention                | SR-PLAN(instance-1):D-14;D-15;D-16                                                                                                                                                                      |
| G-EVIDENCE-SCHEMA-VALIDATOR   | Evidence schema validator present + enforced | architecture     | all            | Ensure evidence is validated by the canonical schema validator before acceptance/release.                                    | Accountability; Evidence                                               | driving_port; ci                            | ci_check; schema_validation        | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | event.EvidenceBundleRecorded                                                 | evidence.gate_packet                                           | nan                                                      | Validator missing/disabled; evidence accepted without validation.                                                 | INTEGRITY_VIOLATION                                                                              | decision                                                                     | GovernanceChangePortal                                                                                              | nan                                                               | C-EVID-1; C-EVID-2                             | SR-SPEC:evidence validation requirements                   | SR-PLAN(instance-1):D-15                                                                                                                                                                                |
| G-EVT-APPENDONLY              | Event log append-only                        | architecture     | event_store    | Ensure the event store is append-only and immutable with content-addressed refs.                                             | Accountability; Change; Authority & Integrity                          | event_store; projection_engine              | ci_check; invariants_test          | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | event.* (event log); projection rebuild artifacts                            | evidence.gate_packet (replay test report)                      | profile:STRICT-CORE                                      | Event mutation detected; missing hashes; non-monotonic sequence; non-durable storage.                             | INTEGRITY_VIOLATION; REPEATED_FAILURE                                                            | decision                                                                     | GovernanceChangePortal                                                                                              | nan                                                               | C-ARCH-3; C-EVT-1; C-EVT-2                     | SR-SPEC:event store invariants                             | SR-PLAN(instance-1):D-09;D-10;D-11;D-12;D-13                                                                                                                                                            |
| G-FREEZE-READY                | Freeze ready                                 | release          | candidate      | Ensure all prerequisites for FreezeRecordCreated are satisfied (verified + approved + no staleness).                         | Authority & Integrity; Accountability; Change                          | driving_port; event_store                   | record_validation; policy_check    | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | record.ApprovalRecorded; event.EvidenceBundleRecorded                        | evidence.gate_packet                                           | nan                                                      | Approval missing; evidence missing; active exceptions not acknowledged; unresolved staleness; integrity issues.   | ORACLE_*; EVIDENCE_MISSING                                                                       | exception/governance change                                                  | ReleaseApprovalPortal; ExceptionApprovalPortal; GovernanceChangePortal                                              | nan                                                               | C-SHIP-1; C-TB-6; C-VER-1                      | SR-SPEC:FreezeRecordCreated preconditions; staleness rules | SR-PLAN(instance-1):D-34;D-35;D-36                                                                                                                                                                      |
| G-GOV-COHERENCE               | Governance coherence (no contradictions)     | governance       | governed_stack | Ensure SR-PARADIGM/SR-TYPES/SR-INTENT/SR-PLAN/SR-CONTRACT/SR-SPEC/SR-ETT are mutually coherent for this instance.            | Ontological; Change; Accountability                                    | driving_port                                | review_check; invariants_test      | HUMAN                 | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | record.DecisionRecorded (coherence attestation) or evidence bundle           | coherence note + refs                                          | nan                                                      | Detected contradiction; missing governed artifact; inconsistent definitions.                                      | INTEGRITY_VIOLATION                                                                              | governance change                                                            | GovernanceChangePortal                                                                                              | nan                                                               | C-ARCH-1; C-ARCH-2; C-ARCH-3                   | SR-SPEC:governed artifacts referenced in IterationStarted  | SR-PLAN(instance-1):D-01                                                                                                                                                                                |
| G-GRAPH-SEMANTICS             | Reference graph semantics correct            | architecture     | graph          | Ensure depends_on edges are used for staleness propagation and supported_by for provenance only.                             | Ontological; Accountability                                            | event_store; projection_engine              | schema_validation; invariants_test | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | event.* (refs); projection graph view                                        | evidence.gate_packet (graph checks)                            | nan                                                      | Wrong rel usage; missing content_hash; cyclic depends_on without justification; orphan refs.                      | INTEGRITY_VIOLATION                                                                              | governance change (if semantics need change)                                 | GovernanceChangePortal                                                                                              | nan                                                               | C-CTX-2; C-ARCH-3                              | SR-SPEC:refs rel semantics; staleness propagation          | SR-PLAN(instance-1):D-09;D-10;D-11;D-12;D-13                                                                                                                                                            |
| G-HARNESS-FAILURE-MODES       | Harness failure modes validated              | verification     | harness        | Ensure harness correctly handles failure modes (missing evidence, integrity violation, budget exhaustion, staleness).        | Operational; Accountability                                            | harness; runtime                            | e2e_test; fault_injection          | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | nan                                                                          | evidence.gate_packet (fault injection report)                  | profile:STRICT-FULL                                      | Failure modes not detected; unsafe continuation; missing stop triggers.                                           | INTEGRITY_VIOLATION                                                                              | decision                                                                     | GovernanceChangePortal                                                                                              | nan                                                               | C-LOOP-2; C-OR-2; C-EVID-4                     | SR-SPEC:stop triggers; staleness; evidence rules           | SR-PLAN(instance-1):D-34;D-35;D-36                                                                                                                                                                      |
| G-HARNESS-HAPPY               | Harness happy-path validated                 | verification     | harness        | Ensure the end-to-end harness executes the canonical happy path (loop→iteration→candidate→oracles→evidence→approval→freeze). | Accountability; Operational                                            | harness; oracle_engine; portal              | e2e_test                           | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | nan                                                                          | evidence.gate_packet (e2e run)                                 | profile:STRICT-FULL                                      | Any step in happy path fails; missing records; incorrect sequencing.                                              | REPEATED_FAILURE                                                                                 | decision                                                                     | GovernanceChangePortal                                                                                              | nan                                                               | C-EVT-1; C-TB-3; C-SHIP-1                      | SR-SPEC:canonical event sequences                          | SR-PLAN(instance-1):D-34;D-35;D-36                                                                                                                                                                      |
| G-OBSERVABILITY-MIN           | Observability minimums                       | operational      | runtime        | Ensure minimal logs/metrics are emitted for oracle runs, portal actions, and freeze operations.                              | Accountability; Operational                                            | runtime; oracle_engine; portal              | smoke_check                        | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | nan                                                                          | evidence.gate_packet (observability check)                     | nan                                                      | Missing logs/metrics; missing traceability fields (candidate_id/run_id).                                          | REPEATED_FAILURE                                                                                 | decision                                                                     | GovernanceChangePortal                                                                                              | nan                                                               | C-EVT-1; C-TB-6                                | SR-SPEC:event attribution + evidence fields                | SR-PLAN(instance-1):D-31;D-32;D-33                                                                                                                                                                      |
| G-ORACLE-ENV-PINNED           | Oracle environment pinned + matched          | oracle_integrity | oracle_run     | Ensure run environment matches declared suite constraints (OCI digest, sandbox mode, network, FS).                           | Authority & Integrity; Accountability; Operational                     | oracle_engine; runtime                      | oracle_suite; runtime_guard        | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | event.OracleRunRecorded; event.EvidenceBundleRecorded                        | evidence.gate_packet                                           | suite:* (as pinned in IterationStarted)                  | Environment fingerprint missing; OCI digest mismatch; network not disabled when required; sandbox mismatch.       | ORACLE_ENV_MISMATCH                                                                              | decision (via GovernanceChange if suite constraints need change)             | GovernanceChangePortal; OracleSuiteChangePortal                                                                     | nan                                                               | C-OR-5; C-EVID-3                               | SR-SPEC:environment_fingerprint                            | SR-PLAN(instance-1):D-24;D-25;D-26;D-27                                                                                                                                                                 |
| G-ORACLE-INTEGRITY            | Oracle integrity conditions satisfied        | oracle_integrity | oracle_run     | Detect tamper/gap/flake/env mismatch for required oracles; block or halt on integrity violation.                             | Authority & Integrity; Accountability; Operational                     | oracle_engine; runtime                      | oracle_suite; runtime_guard        | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | event.OracleRunRecorded; event.EvidenceBundleRecorded                        | evidence.gate_packet                                           | suite:* (as pinned in IterationStarted)                  | ORACLE_TAMPER; ORACLE_GAP; ORACLE_FLAKE; ORACLE_ENV_MISMATCH; suite hash mismatch.                                | ORACLE_TAMPER; ORACLE_GAP; ORACLE_FLAKE; ORACLE_ENV_MISMATCH                                     | none (non-waivable integrity)                                                | GovernanceChangePortal (tamper/mismatch); ExceptionApprovalPortal (flake mitigation only if allowed)                | nan                                                               | C-OR-2; C-OR-3; C-OR-4; C-OR-5; C-OR-6; C-OR-7 | SR-SPEC:oracle suite pinning + integrity                   | SR-PLAN(instance-1):D-24;D-25;D-26;D-27                                                                                                                                                                 |
| G-ORACLE-SANDBOXED            | Oracle sandboxing enforced                   | oracle_integrity | oracle_run     | Ensure oracle runs are isolated (no network by default, restricted FS, deterministic mounts).                                | Authority & Integrity; Accountability; Operational                     | oracle_engine; runtime                      | oracle_suite; runtime_guard        | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | event.OracleRunRecorded; event.EvidenceBundleRecorded                        | evidence.gate_packet                                           | suite:* (as pinned in IterationStarted)                  | Sandbox disabled; network access observed; write access outside allowed paths.                                    | INTEGRITY_VIOLATION                                                                              | decision                                                                     | GovernanceChangePortal                                                                                              | nan                                                               | C-OR-5; C-TB-2                                 | SR-SPEC:oracle runtime constraints                         | SR-PLAN(instance-1):D-24;D-25;D-26;D-27                                                                                                                                                                 |
| G-PORTAL-SUBMISSIONS-RECORDED | Portal submissions recorded                  | operational      | portal         | Ensure all portal submissions and decisions are durably recorded (auditability).                                             | Accountability; Authority & Integrity                                  | portal; event_store                         | record_validation                  | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | record.ApprovalRecorded; record.DecisionRecorded                             | submission payload hash; refs                                  | nan                                                      | Missing submission hash; missing reviewer attribution; undurable record.                                          | INTEGRITY_VIOLATION                                                                              | decision                                                                     | GovernanceChangePortal                                                                                              | nan                                                               | C-TB-3; C-TB-5; C-TB-6                         | SR-SPEC:DecisionRecorded / ApprovalRecorded                | SR-PLAN(instance-1):D-28;D-29;D-30                                                                                                                                                                      |
| G-PROJECTIONS-REBUILDABLE     | Projections rebuildable                      | architecture     | event_store    | Ensure projection rebuild pipeline exists and produces the same results across runs.                                         | Accountability; Change; Authority & Integrity                          | event_store; projection_engine              | ci_check; invariants_test          | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | event.* (event log); projection rebuild artifacts                            | evidence.gate_packet (replay test report)                      | profile:STRICT-CORE                                      | Projection rebuild fails; non-deterministic ordering; missing migration path.                                     | INTEGRITY_VIOLATION; REPEATED_FAILURE                                                            | decision                                                                     | GovernanceChangePortal                                                                                              | nan                                                               | C-EVT-3; C-VER-3                               | SR-SPEC:projection rebuild                                 | SR-PLAN(instance-1):D-09;D-10;D-11;D-12;D-13                                                                                                                                                            |
| G-REBUILD-FROM-EVENTS-ONLY    | Rebuild state from events only               | architecture     | event_store    | Ensure all derived state/projections can be rebuilt deterministically from the event log.                                    | Accountability; Change; Authority & Integrity                          | event_store; projection_engine              | ci_check; invariants_test          | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | event.* (event log); projection rebuild artifacts                            | evidence.gate_packet (replay test report)                      | profile:STRICT-CORE                                      | Projection depends on external DB; replay differs; missing events; hidden inputs.                                 | INTEGRITY_VIOLATION; REPEATED_FAILURE                                                            | decision                                                                     | GovernanceChangePortal                                                                                              | nan                                                               | C-ARCH-3; C-EVT-3; C-CTX-2                     | SR-SPEC:projections + replay                               | SR-PLAN(instance-1):D-36                                                                                                                                                                                |
| G-RELEASE-APPROVED            | Release approved (human binding)             | portal           | candidate      | Ensure a human ApprovalRecorded exists for this candidate with explicit evidence + exception acknowledgement.                | Authority & Integrity; Accountability                                  | driving_port; portal                        | portal_review; record_validation   | HUMAN                 | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | record.ApprovalRecorded                                                      | evidence.gate_packet referenced by approval                    | nan                                                      | Missing approval; approval not bound to candidate; evidence refs missing; exceptions not acknowledged explicitly. | EVIDENCE_MISSING                                                                                 | resubmit approval; or route to exception/governance change                   | ReleaseApprovalPortal; ExceptionApprovalPortal                                                                      | nan                                                               | C-TB-1; C-TB-3; C-TB-6; C-SHIP-1               | SR-SPEC:ApprovalRecorded; exception acknowledgement rule   | SR-PLAN(instance-1):D-31;D-32;D-33;D-34;D-35;D-36                                                                                                                                                       |
| G-REPLAYABILITY-PROOF         | Replayability proof                          | verification     | event_log      | Prove state can be replayed from event log and matches expected projections.                                                 | Accountability                                                         | event_store; projection_engine              | replay_check                       | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | nan                                                                          | evidence.gate_packet (replay proof)                            | profile:STRICT-FULL                                      | Replay fails; projection mismatch; missing events.                                                                | INTEGRITY_VIOLATION                                                                              | decision                                                                     | GovernanceChangePortal                                                                                              | nan                                                               | C-EVT-3; C-ARCH-3; C-VER-3                     | SR-SPEC:replay + projections                               | SR-PLAN(instance-1):D-34;D-35;D-36                                                                                                                                                                      |
| G-STACK-HEALTH                | Stack health checks                          | operational      | runtime        | Ensure runtime components required for harness execution are healthy (event store, oracle runner, portals).                  | Operational                                                            | runtime; event_store; oracle_engine; portal | health_check                       | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | nan                                                                          | evidence.gate_packet (health report)                           | nan                                                      | Component unhealthy; connectivity errors; storage non-durable.                                                    | BUDGET_EXHAUSTED (if repeated); REPEATED_FAILURE                                                 | decision                                                                     | BudgetEscalationPortal                                                                                              | nan                                                               | C-ARCH-3; C-EVT-1                              | SR-SPEC:runtime assumptions                                | SR-PLAN(instance-1):D-31;D-32;D-33                                                                                                                                                                      |
| G-STOPTRIGGER-N-DEFINED       | Stop triggers defined                        | operational      | loop           | Ensure stop triggers are configured (e.g., repeated failure threshold) and enforced by runtime.                              | Operational                                                            | runtime                                     | config_check; runtime_guard        | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | config.stop_triggers                                                         | evidence.gate_packet (config report)                           | nan                                                      | Stop triggers missing/disabled; threshold unset.                                                                  | INTEGRITY_VIOLATION                                                                              | decision                                                                     | BudgetEscalationPortal                                                                                              | nan                                                               | C-LOOP-2; C-LOOP-4                             | SR-SPEC:stop trigger behavior                              | SR-PLAN(instance-1):D-22                                                                                                                                                                                |
| G-UI-NONBINDING               | UI is non-binding                            | trust_boundary   | ui             | Ensure UI does not create binding state; only portals + records do.                                                          | Authority & Integrity                                                  | ui; driving_port                            | design_review; invariants_test     | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | nan                                                                          | evidence.gate_packet (UI non-binding checks)                   | nan                                                      | UI writes binding records; UI bypasses portals.                                                                   | INTEGRITY_VIOLATION                                                                              | decision                                                                     | GovernanceChangePortal                                                                                              | nan                                                               | C-TB-2; C-TB-7                                 | SR-SPEC:portals as trust boundaries                        | SR-PLAN(instance-1):D-28;D-29;D-30                                                                                                                                                                      |
| G-VERIFIED-STRICT             | Verified (STRICT) for required profile       | verification     | candidate      | Require STRICT verification outcome for the mapped profile (core/full) for this work unit.                                   | Accountability; Authority & Integrity                                  | oracle_engine; driving_port                 | oracle_suite; gate_policy          | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | event.CandidateSubmitted; event.EvidenceBundleRecorded                       | evidence.gate_packet (STRICT outcome)                          | profile:STRICT-CORE or profile:STRICT-FULL (per mapping) | Any required oracle fails; integrity condition fails; evidence missing; suite mismatch.                           | REPEATED_FAILURE; ORACLE_* (as applicable)                                                       | exception (WITH_EXCEPTIONS) only if allowed by profile + non-waivable checks | ExceptionApprovalPortal                                                                                             | nan                                                               | C-VER-1; C-VER-2; C-EVID-1; C-OR-2             | SR-SPEC:verification mode + evidence bundles               | SR-PLAN(instance-1):D-01;D-02;D-03;D-04;D-05;D-06;D-07;D-08;D-09;D-10;D-11;D-12;D-13;D-14;D-15;D-16;D-17;D-18;D-19;D-20;D-21;D-22;D-23;D-24;D-25;D-26;D-27;D-28;D-29;D-30;D-31;D-32;D-33;D-34;D-35;D-36 |
| G-VERSIONS-PINNED             | Governed versions pinned                     | integrity        | all            | Ensure all governed artifacts (SR-* docs and oracle suite hashes) are version+hash pinned for this work unit.                | Accountability; Authority & Integrity                                  | driving_port; event_store                   | schema_validation; runtime_guard   | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; SR-PLAN(instance-1)                                                        | event.IterationStarted                                                       | nan                                                            | nan                                                      | Unpinned SR-* versions; missing content_hash; suite hash absent; inconsistent versions across refs.               | EVIDENCE_MISSING; REPEATED_FAILURE                                                               | decision                                                                     | GovernanceChangePortal                                                                                              | nan                                                               | C-CTX-1; C-CTX-2; C-OR-1                       | SR-SPEC:refs (depends_on vs supported_by) + suite pinning  | SR-PLAN(instance-1):D-31;D-32;D-33                                                                                                                                                                      |
| G-00                          | Context admissible (no ghost inputs)         | work_start       | all            | Ensure IterationStarted context refs are sufficient and derivation is deterministic.                                         | Intent & Objective; Ontological; Accountability; Authority & Integrity | domain_core; driving_port                   | schema_validation; runtime_guard   | SYSTEM                | SR-TYPES; SR-CONTRACT; SR-SPEC; SR-DIRECTIVE; OracleSuite; Loop; (Base Candidate optional); (Active exceptions optional) | IterationStarted event                                                       | N/A (schema validation evidence optional)                      | nan                                                      | Missing required refs; refs not dereferenceable; unknown governed artifact versions                               | EVIDENCE_MISSING (if ref cannot be fetched); ORACLE_GAP (if suite missing)                       | decision                                                                     | GovernanceChangePortal (if governance refs missing) / BudgetExtensionPortal (if blocked by budget)                  | Keep conservative: block if unsure; escalate via portal decision. | C-CTX-1; C-CTX-2; C-LOOP-2                     | SR-SPEC §3.2.1.1                                           | nan                                                                                                                                                                                                     |

## 8. Plan-to-workflow mapping

(Authoritative table for instance-1)

| deliverable_id   | recommended_profile   | workflow_phase   | work_unit_type                       | start_gate_ids   | accept_gate_ids                                                                                                  | release_gate_ids                                 | required_portals                             | required_oracle_suite_id   | required_evidence_bundle_kinds   | budgets_default                                                    |   stop_triggers_overrides | notes                                                                                                                                                                 |
|:-----------------|:----------------------|:-----------------|:-------------------------------------|:-----------------|:-----------------------------------------------------------------------------------------------------------------|:-------------------------------------------------|:---------------------------------------------|:---------------------------|:---------------------------------|:-------------------------------------------------------------------|--------------------------:|:----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| D-01             | nan                   | PKG-01           | governance_intervention_record       | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-GOV-COHERENCE                                                                                | nan                                              | PORTAL-GOVERNANCE-CHANGE                     | suite:SR-SUITE-GOV         | record.intervention_note         | {"max_iterations":1,"max_oracle_runs":5,"max_wallclock_hours":4}   |                       nan | Conditional: only if governance/type mismatches block implementation; route changes via GovernanceChangePortal.                                                       |
| D-02             | STRICT-CORE           | PKG-02           | repo_scaffold                        | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-BUILD-REPRO;G-CI-GREEN                                                                       | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16} |                       nan | nan                                                                                                                                                                   |
| D-03             | STRICT-CORE           | PKG-02           | ci_substrate                         | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-BUILD-REPRO;G-CI-GREEN                                                                       | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16} |                       nan | nan                                                                                                                                                                   |
| D-04             | STRICT-CORE           | PKG-02           | dev_tooling                          | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-BUILD-REPRO;G-CI-GREEN                                                                       | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16} |                       nan | nan                                                                                                                                                                   |
| D-05             | STRICT-CORE           | PKG-03           | domain_core_primitives               | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-DOMAIN-PURITY;G-DETERMINISM;G-CTX-COMPILE-DETERMINISTIC                                      | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16} |                       nan | nan                                                                                                                                                                   |
| D-06             | STRICT-CORE           | PKG-03           | domain_core_state_machines           | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-DOMAIN-PURITY;G-DETERMINISM;G-CTX-COMPILE-DETERMINISTIC                                      | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16} |                       nan | nan                                                                                                                                                                   |
| D-07             | STRICT-CORE           | PKG-03           | domain_ports_and_boundaries          | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-DOMAIN-PURITY;G-DETERMINISM;G-CTX-COMPILE-DETERMINISTIC                                      | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16} |                       nan | nan                                                                                                                                                                   |
| D-08             | STRICT-CORE           | PKG-03           | context_compilation                  | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-DOMAIN-PURITY;G-DETERMINISM;G-CTX-COMPILE-DETERMINISTIC                                      | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16} |                       nan | nan                                                                                                                                                                   |
| D-09             | STRICT-CORE           | PKG-04           | persistence_schema                   | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-EVT-APPENDONLY;G-PROJECTIONS-REBUILDABLE;G-GRAPH-SEMANTICS                                   | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16} |                       nan | nan                                                                                                                                                                   |
| D-10             | STRICT-CORE           | PKG-04           | adapter_eventstore                   | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-EVT-APPENDONLY;G-PROJECTIONS-REBUILDABLE;G-GRAPH-SEMANTICS                                   | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16} |                       nan | nan                                                                                                                                                                   |
| D-11             | STRICT-CORE           | PKG-04           | projection_builder                   | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-EVT-APPENDONLY;G-PROJECTIONS-REBUILDABLE;G-GRAPH-SEMANTICS                                   | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16} |                       nan | nan                                                                                                                                                                   |
| D-12             | STRICT-CORE           | PKG-04           | dependency_graph_and_staleness       | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-EVT-APPENDONLY;G-PROJECTIONS-REBUILDABLE;G-GRAPH-SEMANTICS                                   | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16} |                       nan | nan                                                                                                                                                                   |
| D-13             | STRICT-CORE           | PKG-04           | outbox_publisher_and_event_pub       | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-EVT-APPENDONLY;G-PROJECTIONS-REBUILDABLE;G-GRAPH-SEMANTICS                                   | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16} |                       nan | nan                                                                                                                                                                   |
| D-14             | STRICT-CORE           | PKG-05           | adapter_evidence_store               | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-EVID-MANIFEST-V1;G-EVID-CONTENT-ADDRESSED;G-EVID-SECRET-HANDLING                             | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16} |                       nan | nan                                                                                                                                                                   |
| D-15             | STRICT-CORE           | PKG-05           | evidence_manifest_lib_and_oracle     | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-EVID-MANIFEST-V1;G-EVID-CONTENT-ADDRESSED;G-EVID-SECRET-HANDLING;G-EVIDENCE-SCHEMA-VALIDATOR | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16} |                       nan | nan                                                                                                                                                                   |
| D-16             | STRICT-CORE           | PKG-05           | restricted_evidence_handling         | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-EVID-MANIFEST-V1;G-EVID-CONTENT-ADDRESSED;G-EVID-SECRET-HANDLING                             | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16} |                       nan | Evidence containing secrets must follow restricted-vault + redacted-copy policy.                                                                                      |
| D-17             | STRICT-CORE           | PKG-06           | api_scaffold_and_auth                | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-AUTHZ-HUMAN-BINDING;G-API-CONTRACTS                                                          | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16} |                       nan | nan                                                                                                                                                                   |
| D-18             | STRICT-CORE           | PKG-06           | api_core_endpoints                   | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-AUTHZ-HUMAN-BINDING;G-API-CONTRACTS                                                          | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16} |                       nan | nan                                                                                                                                                                   |
| D-19             | STRICT-CORE           | PKG-06           | api_portals_and_governance_endpoints | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-AUTHZ-HUMAN-BINDING;G-API-CONTRACTS                                                          | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT;PORTAL-PORTAL-POLICY       | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16} |                       nan | Implements portal APIs; ensure UI/API remain non-binding—only record submissions/approvals.                                                                           |
| D-20             | STRICT-CORE           | PKG-06           | api_evidence_endpoints               | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-AUTHZ-HUMAN-BINDING;G-API-CONTRACTS                                                          | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16} |                       nan | nan                                                                                                                                                                   |
| D-21             | STRICT-CORE           | PKG-07           | messaging_integration                | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-BUDGETS-STOPTRIGGERS;G-ASYNC-RELIABILITY                                                     | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16} |                       nan | nan                                                                                                                                                                   |
| D-22             | STRICT-CORE           | PKG-07           | loop_governor_service                | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-BUDGETS-STOPTRIGGERS;G-ASYNC-RELIABILITY;G-STOPTRIGGER-N-DEFINED                             | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16} |                       nan | nan                                                                                                                                                                   |
| D-23             | STRICT-CORE           | PKG-07           | reference_worker_bridge              | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-BUDGETS-STOPTRIGGERS;G-ASYNC-RELIABILITY                                                     | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16} |                       nan | nan                                                                                                                                                                   |
| D-24             | STRICT-CORE           | PKG-08           | oracle_runner_service                | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-ORACLE-ENV-PINNED;G-ORACLE-SANDBOXED;G-ORACLE-INTEGRITY                                      | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT;PORTAL-ORACLE-SUITE-CHANGE | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":6,"max_oracle_runs":40,"max_wallclock_hours":24} |                       nan | Oracle integrity stop triggers (ORACLE_TAMPER/GAP/ENV_MISMATCH/FLAKE) are non-waivable; suite changes require portal approval.                                        |
| D-25             | STRICT-CORE           | PKG-08           | oracle_suite_core_implementation     | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-ORACLE-ENV-PINNED;G-ORACLE-SANDBOXED;G-ORACLE-INTEGRITY                                      | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT;PORTAL-ORACLE-SUITE-CHANGE | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":6,"max_oracle_runs":40,"max_wallclock_hours":24} |                       nan | Oracle integrity stop triggers (ORACLE_TAMPER/GAP/ENV_MISMATCH/FLAKE) are non-waivable; suite changes require portal approval.                                        |
| D-26             | STRICT-FULL           | PKG-08           | oracle_suite_integration_e2e         | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-ORACLE-ENV-PINNED;G-ORACLE-SANDBOXED;G-ORACLE-INTEGRITY                                      | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT;PORTAL-ORACLE-SUITE-CHANGE | suite:SR-SUITE-FULL        | evidence.gate_packet             | {"max_iterations":6,"max_oracle_runs":40,"max_wallclock_hours":24} |                       nan | Oracle integrity stop triggers (ORACLE_TAMPER/GAP/ENV_MISMATCH/FLAKE) are non-waivable; suite changes require portal approval.                                        |
| D-27             | STRICT-CORE           | PKG-08           | oracle_integrity_checks              | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-ORACLE-ENV-PINNED;G-ORACLE-SANDBOXED;G-ORACLE-INTEGRITY                                      | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT;PORTAL-ORACLE-SUITE-CHANGE | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":6,"max_oracle_runs":40,"max_wallclock_hours":24} |                       nan | Oracle integrity stop triggers (ORACLE_TAMPER/GAP/ENV_MISMATCH/FLAKE) are non-waivable; suite changes require portal approval.                                        |
| D-28             | STRICT-CORE           | PKG-09           | ui_scaffold_and_login                | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-UI-NONBINDING;G-PORTAL-SUBMISSIONS-RECORDED                                                  | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":6,"max_oracle_runs":40,"max_wallclock_hours":24} |                       nan | nan                                                                                                                                                                   |
| D-29             | STRICT-CORE           | PKG-09           | ui_views_and_evidence_viewer         | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-UI-NONBINDING;G-PORTAL-SUBMISSIONS-RECORDED                                                  | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT                            | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":6,"max_oracle_runs":40,"max_wallclock_hours":24} |                       nan | nan                                                                                                                                                                   |
| D-30             | STRICT-CORE           | PKG-09           | ui_portal_workflows                  | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-UI-NONBINDING;G-PORTAL-SUBMISSIONS-RECORDED                                                  | G-ENG-APPROVED                                   | PORTAL-ENG-ACCEPT;PORTAL-PORTAL-POLICY       | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":6,"max_oracle_runs":40,"max_wallclock_hours":24} |                       nan | UI must not imply approval; must record portal submissions with stable identity.                                                                                      |
| D-31             | STRICT-FULL           | PKG-10           | self_host_deployment_stack           | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-STACK-HEALTH;G-VERSIONS-PINNED;G-OBSERVABILITY-MIN                                           | G-ENG-APPROVED;G-RELEASE-APPROVED                | PORTAL-ENG-ACCEPT;PORTAL-RELEASE             | suite:SR-SUITE-FULL        | evidence.gate_packet             | {"max_iterations":6,"max_oracle_runs":40,"max_wallclock_hours":32} |                       nan | Release portal required for artifacts promoted to self-host baseline / demo freeze.                                                                                   |
| D-32             | STRICT-FULL           | PKG-10           | bootstrap_init_scripts               | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-STACK-HEALTH;G-VERSIONS-PINNED;G-OBSERVABILITY-MIN                                           | G-ENG-APPROVED;G-RELEASE-APPROVED                | PORTAL-ENG-ACCEPT;PORTAL-RELEASE             | suite:SR-SUITE-FULL        | evidence.gate_packet             | {"max_iterations":6,"max_oracle_runs":40,"max_wallclock_hours":32} |                       nan | Release portal required for artifacts promoted to self-host baseline / demo freeze.                                                                                   |
| D-33             | STRICT-CORE           | PKG-10           | ops_logging_observability            | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-STACK-HEALTH;G-VERSIONS-PINNED;G-OBSERVABILITY-MIN                                           | G-ENG-APPROVED;G-RELEASE-APPROVED                | PORTAL-ENG-ACCEPT;PORTAL-RELEASE             | suite:SR-SUITE-CORE        | evidence.gate_packet             | {"max_iterations":6,"max_oracle_runs":40,"max_wallclock_hours":32} |                       nan | Release portal required for artifacts promoted to self-host baseline / demo freeze.                                                                                   |
| D-34             | STRICT-FULL           | PKG-11           | e2e_harness_happy_path               | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-HARNESS-HAPPY;G-HARNESS-FAILURE-MODES;G-REPLAYABILITY-PROOF                                  | G-ENG-APPROVED;G-RELEASE-APPROVED;G-FREEZE-READY | PORTAL-ENG-ACCEPT;PORTAL-RELEASE             | suite:SR-SUITE-FULL        | evidence.gate_packet             | {"max_iterations":8,"max_oracle_runs":80,"max_wallclock_hours":48} |                       nan | Release portal required for artifacts promoted to self-host baseline / demo freeze. E2E harness outputs should be eligible for FreezeRecord creation once gates pass. |
| D-35             | STRICT-FULL           | PKG-11           | e2e_harness_failure_modes            | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-HARNESS-HAPPY;G-HARNESS-FAILURE-MODES;G-REPLAYABILITY-PROOF                                  | G-ENG-APPROVED;G-RELEASE-APPROVED;G-FREEZE-READY | PORTAL-ENG-ACCEPT;PORTAL-RELEASE             | suite:SR-SUITE-FULL        | evidence.gate_packet             | {"max_iterations":8,"max_oracle_runs":80,"max_wallclock_hours":48} |                       nan | Release portal required for artifacts promoted to self-host baseline / demo freeze. E2E harness outputs should be eligible for FreezeRecord creation once gates pass. |
| D-36             | STRICT-FULL           | PKG-11           | replayability_demonstration          | G-CTX-ADMISSIBLE | G-VERIFIED-STRICT;G-HARNESS-HAPPY;G-HARNESS-FAILURE-MODES;G-REPLAYABILITY-PROOF;G-REBUILD-FROM-EVENTS-ONLY       | G-ENG-APPROVED;G-RELEASE-APPROVED;G-FREEZE-READY | PORTAL-ENG-ACCEPT;PORTAL-RELEASE             | suite:SR-SUITE-FULL        | evidence.gate_packet             | {"max_iterations":8,"max_oracle_runs":80,"max_wallclock_hours":48} |                       nan | Release portal required for artifacts promoted to self-host baseline / demo freeze. E2E harness outputs should be eligible for FreezeRecord creation once gates pass. |

## 9. Exceptions, deviations, deferrals, waivers

- Exceptions are always explicit records emitted via **ExceptionApprovalPortal**.
- Waivers may only waive *oracle FAIL outcomes* and are never permitted to waive integrity conditions.
- Every exception MUST be scoped and SHOULD be time-boxed; release approval requires explicit acknowledgement of active exceptions.

## 10. SR-ETT membrane coverage matrix

| harness_or_membrane   | enforcing_gates                                                                                                                                                                                                                 | enforcing_portals                                                      | profiles_or_suites_supporting             | relief_valves                                                               |
|:----------------------|:--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|:-----------------------------------------------------------------------|:------------------------------------------|:----------------------------------------------------------------------------|
| Authority Boundary    |                                                                                                                                                                                                                                 | ExceptionApprovalPortal; GovernanceChangePortal; ReleaseApprovalPortal |                                           | waiver/deferral/deviation via ExceptionApprovalPortal (never for integrity) |
| Evidence Integrity    |                                                                                                                                                                                                                                 | ExceptionApprovalPortal; GovernanceChangePortal; ReleaseApprovalPortal | suite:SR-SUITE-CORE / suite:SR-SUITE-FULL | waiver/deferral/deviation via ExceptionApprovalPortal (never for integrity) |
| Change                | G-API-CONTRACTS, G-EVT-APPENDONLY, G-FREEZE-READY, G-GOV-COHERENCE, G-PROJECTIONS-REBUILDABLE, G-REBUILD-FROM-EVENTS-ONLY                                                                                                       | ExceptionApprovalPortal; GovernanceChangePortal; ReleaseApprovalPortal |                                           | waiver/deferral/deviation via ExceptionApprovalPortal (never for integrity) |
| Accountability        | G-00, G-API-CONTRACTS, G-ASYNC-RELIABILITY, G-BUDGETS-STOPTRIGGERS, G-BUILD-REPRO, G-CI-GREEN, G-CTX-ADMISSIBLE, G-CTX-COMPILE-DETERMINISTIC, G-DETERMINISM, G-ENG-APPROVED, G-EVID-CONTENT-ADDRESSED, G-EVID-MANIFEST-V1 …     | ExceptionApprovalPortal; GovernanceChangePortal; ReleaseApprovalPortal |                                           | waiver/deferral/deviation via ExceptionApprovalPortal (never for integrity) |
| Operational           | G-ASYNC-RELIABILITY, G-BUDGETS-STOPTRIGGERS, G-BUILD-REPRO, G-HARNESS-FAILURE-MODES, G-HARNESS-HAPPY, G-OBSERVABILITY-MIN, G-ORACLE-ENV-PINNED, G-ORACLE-INTEGRITY, G-ORACLE-SANDBOXED, G-STACK-HEALTH, G-STOPTRIGGER-N-DEFINED | ExceptionApprovalPortal; GovernanceChangePortal; ReleaseApprovalPortal |                                           | waiver/deferral/deviation via ExceptionApprovalPortal (never for integrity) |
| Port Separation       |                                                                                                                                                                                                                                 | GovernanceChangePortal                                                 |                                           | waiver/deferral/deviation via ExceptionApprovalPortal (never for integrity) |
| Event Integrity       |                                                                                                                                                                                                                                 | ExceptionApprovalPortal; GovernanceChangePortal; ReleaseApprovalPortal | suite:SR-SUITE-CORE / suite:SR-SUITE-FULL | waiver/deferral/deviation via ExceptionApprovalPortal (never for integrity) |
| Runtime Integrity     |                                                                                                                                                                                                                                 | GovernanceChangePortal                                                 | suite:SR-SUITE-CORE / suite:SR-SUITE-FULL | waiver/deferral/deviation via ExceptionApprovalPortal (never for integrity) |
| Leakage Control       |                                                                                                                                                                                                                                 | GovernanceChangePortal                                                 |                                           | waiver/deferral/deviation via ExceptionApprovalPortal (never for integrity) |

## 11. Directive self-verification

Before marking this directive as “current”:
- Run `suite:SR-SUITE-CORE` against the directive artifacts (schema checks for all tables, playbook linting).
- Human review: GovernanceChangePortal reviewers confirm coherence with SR‑CONTRACT/SR‑SPEC and confirm no new binding semantics were introduced.
- Dry-run: execute one canonical loop with a trivial candidate and confirm evidence packets, gates, portal routing, and freeze eligibility behave as intended.

## Appendices

- A. Gate Registry (CSV): `gate_registry_instance1_filled.csv`
- B. Portal Playbooks (filled): `portal_playbook__*__filled.md`
- C. Profile Definitions (YAML): `profile_definitions_instance1_filled.yaml`
- D. Plan‑to‑Workflow mapping (CSV): `plan_to_workflow_mapping_instance1_filled.csv`

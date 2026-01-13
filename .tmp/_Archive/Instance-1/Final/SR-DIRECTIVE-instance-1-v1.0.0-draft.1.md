---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-DIRECTIVE"
  type: "governance.dev_directive"
  title: "SOLVER-Ralph Development Directive — instance-1"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "normative"
  authority_kind: "process"
  governed_by:
    - "SR-CHANGE"
  created: "2026-01-11"
  updated: "2026-01-11"
  tags:
    - "sr-directive"
    - "instance-1"
    - "execution"
    - "gates"
    - "profiles"
    - "portals"
    - "stop-triggers"
    - "budgets"
  refs:
    - kind: "Record"
      id: "PS-SOLVER-RALPH-INSTANCE-1-SAPS"
      rel: "depends_on"
      meta:
        type_key: "record.problem_statement"
        stage: "SAPS"
        version: "1.0.0-draft.1"
        content_hash: "sha256:5c27764e1747010d4e9dc4b24c25dffa69b0d3b6f8baed8fc2c3d78901043fcb"
    - kind: "GovernedArtifact"
      id: "SR-PLAN-instance-1"
      rel: "depends_on"
      meta:
        type_key: "governance.plan"
        version: "1.0.0-draft.3"
        content_hash: "sha256:a88abdfb86192a42e76826ee076300ca211bbc82a8aa822fc286ae53651b5438"
    - kind: "GovernedArtifact"
      id: "SR-CONTRACT"
      rel: "depends_on"
      meta:
        type_key: "governance.arch_contract"
        version: "1.1.0-draft.1"
        content_hash: "sha256:0d72191fb36d375062eea7358b043dc9aa76ff93affc122803757775404aa20c"
    - kind: "GovernedArtifact"
      id: "SR-SPEC"
      rel: "depends_on"
      meta:
        type_key: "governance.technical_spec"
        version: "1.3.0-draft.1"
        content_hash: "sha256:c14a45be626d931bb90bd0e8081a5d35a832e8ef7a52f361480d18b96c6b13df"
    - kind: "GovernedArtifact"
      id: "SR-TYPES"
      rel: "depends_on"
      meta:
        type_key: "governance.types"
        version: "3.3.0-draft.8"
        content_hash: "sha256:1d4fe783e4bf943913911d864ebe296246c87d154de6f44fb00ee6cfa2225147"
    - kind: "GovernedArtifact"
      id: "SR-ETT"
      rel: "supported_by"
      meta:
        type_key: "governance.epistemic_trust_topology"
        version: "1.1.0-draft.1"
        content_hash: "sha256:17acacb67fa3186ad99f3e6d9db783f5e9af6a6795d3b5e9617db7e906b3f2b7"
    - kind: "GovernedArtifact"
      id: "SR-INTENT"
      rel: "supported_by"
      meta:
        type_key: "governance.intent"
        version: "1.1.0-draft.6"
    - kind: "GovernedArtifact"
      id: "SR-PARADIGM"
      rel: "supported_by"
      meta:
        type_key: "governance.paradigm"
        version: "1.0.0-draft.10"
  ext:
    dev_directive:
      instance_id: "SOLVER-RALPH-INSTANCE-1"
      phases:
        - id: "PH-0"
          name: "Bootstrap"
        - id: "PH-1"
          name: "Domain Core"
        - id: "PH-2"
          name: "Persistence+Evidence"
        - id: "PH-3"
          name: "Oracles"
        - id: "PH-4"
          name: "API+Identity"
        - id: "PH-5"
          name: "Orchestration+UI+Ops"
        - id: "PH-6"
          name: "E2E+Proof"
      portals:
        - "GovernanceChangePortal"
        - "ReleaseApprovalPortal"
        - "ExceptionApprovalPortal"
      stop_triggers:
        repeated_failure_n_default: 3
        non_waivable:
          - "ORACLE_TAMPER"
          - "ORACLE_GAP"
          - "ORACLE_ENV_MISMATCH"
          - "ORACLE_FLAKE"
          - "EVIDENCE_MISSING"
        waivable_via_human_decision:
          - "REPEATED_FAILURE"
          - "BUDGET_EXHAUSTED"
      verification_profiles:
        - "profile:STRICT-CORE@v1"
        - "profile:STRICT-FULL@v1"
        - "suite:CTX-REFSET-VALIDATION@v1"
        - "suite:RUNTIME-SUBSTRATE@v1"
        - "suite:EVIDENCE-INTEGRITY@v1"
        - "suite:STALENESS-GRAPH@v1"
        - "suite:STOP-TRIGGERS@v1"
        - "suite:PORTAL-WORKFLOWS@v1"
        - "suite:SELFHOST-SMOKE@v1"
        - "suite:E2E-HAPPY@v1"
        - "suite:E2E-FAILURE@v1"
        - "suite:REPLAYABILITY@v1"
        - "suite:FREEZE-VALIDATION@v1"
---

# SOLVER-Ralph Development Directive — instance-1

> **Status:** 1.0.0-draft.1 (draft)  
> **Scope:** Execution governance for implementing **SR-PLAN-instance-1@1.0.0-draft.3** under **SR-CONTRACT@1.1.0-draft.1** and **SR-SPEC@1.3.0-draft.1**, using **SR-ETT@1.1.0-draft.1** as the membrane/topology lens.

---

## 0. Version changes

- **Initial assembled directive** for SOLVER‑Ralph instance‑1.
- Incorporates the following instance‑1 control surfaces:
  - **Gate Registry:** `gate_registry_instance1_v1.0.0-draft.2.md` (sha256:d64dba8f986390af18860c45bec53590b6a9322067889542c5fe09cce0d65344)
  - **Verification Profiles:** `profile_definitions_instance1_v1.0.0-draft.1.yaml` (sha256:2a56f930357e6740824ebd81338ce0b677e61b78884ac31ba470d3a3b3dfa9f2)
  - **Plan→Workflow mapping:** `plan_to_workflow_mapping_instance1_firstpass_v1.0.0-draft.2.csv` (sha256:31f27217055a07264d78cb69d19eb53044d89bf7f9cd00f7bf9a90ad5ebf17e1)
  - **Portal Playbooks:** v1.0.0-draft.1 (see §8)

> Note: the plan→workflow mapping is included in this directive as a binding default mapping. If the mapping needs change, route via **GovernanceChangePortal** (see §8.1).

---

## 1. Scope, authority, and precedence

### 1.1 What this directive governs

This directive is the **binding execution discipline** for SR‑PLAN instance‑1:

- Workflow phases and sequencing constraints (derived from the SR‑PLAN dependency graph).
- Gate IDs, meanings, and acceptance requirements (Gate Registry).
- Verification profile selection and oracle suite composition (Profile Definitions).
- Stop‑the‑line behavior and escalation to human authority (stop triggers + Decision routing).
- Portal workflows for binding actions (Approvals, Waivers/Exceptions, Freeze, Decisions).

### 1.2 What this directive does not govern

- It **does not** redefine binding semantics (e.g., what “Verified”, “Approved”, or “Shippable” mean). Those are defined by SR‑CONTRACT and SR‑SPEC.
- It **does not** introduce new portal kinds or new binding record types. If that is required, it is a governance change via SR‑CHANGE.

### 1.3 Precedence rule

If any part of this directive conflicts with:

- **SR‑CONTRACT** (normative invariants), or
- **SR‑SPEC** (normative mechanics),

…then SR‑CONTRACT/SR‑SPEC take precedence, and the conflict MUST be routed as a governance change (GovernanceChangePortal).

SR‑ETT is a **design lens**; it guides placement of coercion and trust boundaries. It does not override contract/spec semantics.

---

## 2. Execution model

### 2.1 Unit of execution

- The unit of planned work is **deliverable** `D-xx` from SR‑PLAN instance‑1.
- The unit of runtime work is a **Loop** (SR‑SPEC), typically one Loop per deliverable (or a small cluster of tightly coupled deliverables where justified and explicitly recorded).

### 2.2 Canonical Loop pattern (agents + HITL)

This directive assumes the canonical pattern below (event names per SR‑SPEC):

1) **LoopCreated** (goal, work_unit, budgets, directive_ref)  
2) **IterationStarted** *(SYSTEM‑only)* with the Iteration Context Ref Set (see Gate **G‑00**)  
3) **Exploration (agent work)** produces proposals; proposals are non‑binding  
4) **CandidateMaterialized** registers a candidate snapshot  
5) **RunStarted → RunCompleted → EvidenceBundleRecorded** (oracle suite run against candidate)  
6) **Human evaluation/assessment** may be recorded as non‑binding notes; binding actions happen only via:
   - **ApprovalRecorded** (portal approval)  
   - **WaiverCreated / WaiverActivated** (exception record)  
   - **DecisionRecorded** (binding decision to proceed/terminate/adjust)  
7) **FreezeRecordCreated** *(human‑only)* establishes a baseline snapshot for release and shippable computation  
8) **StopTriggered** events pause execution and require explicit decision routing before resumption

### 2.3 Workflow phases

This directive uses the following phases as a scheduling scaffold (non‑binding labels; dependencies + gates are binding):

- **PH‑0 Bootstrap**
- **PH‑1 Domain Core**
- **PH‑2 Persistence+Evidence**
- **PH‑3 Oracles**
- **PH‑4 API+Identity**
- **PH‑5 Orchestration+UI+Ops**
- **PH‑6 E2E+Proof**

### 2.4 Concurrency policy (default)

- Deliverables MAY be executed concurrently when their `depends_on` constraints are satisfied.
- Default maximum parallel Loops: **3**.  
  Exceeding this requires a human DecisionRecorded with rationale (resource + accountability membranes).

---

## 3. Work unit taxonomy

Work units are classified in the Plan→Workflow mapping (see §7) using `work_unit_type`, which controls:

- default verification profile selection (see §5),
- default gate expectations (Gate Registry),
- default escalation sensitivity (e.g., governance-touch routing).

The initial taxonomy used by instance‑1 is:

- `deliverable.record.governance`
- `deliverable.bootstrap.repo`
- `deliverable.domain_core`
- `deliverable.persistence_evidence`
- `deliverable.oracles`
- `deliverable.api_identity`
- `deliverable.orchestration_ui_ops`
- `deliverable.e2e_proof`

---

## 4. Budgets and stop triggers

### 4.1 Default budgets (first pass)

Budgets are attached at **LoopCreated**. Unless overridden in `IterationStarted.refs[]` by an explicit directive addendum, the defaults are:

- **Per Loop**
  - iterations_max: **10**
  - wall_clock_hours_max: **8**
  - compute_cost_usd_equivalent_max: **50**
- **Per iteration**
  - max oracle re-runs per suite: **1** *(re‑run more requires DecisionRecorded)*

Budget extension is permitted only via binding **DecisionRecorded** and/or portal action as routed by the Gate Registry.

### 4.2 Stop triggers (binding)

The instance‑1 stop triggers are defined in the Gate Registry and Profile Definitions and MUST include contract minimums:

**Non‑waivable (block until resolved):**
- `ORACLE_TAMPER`
- `ORACLE_GAP`
- `ORACLE_ENV_MISMATCH`
- `ORACLE_FLAKE`
- `EVIDENCE_MISSING`

**Waivable only via explicit human decision (and only where contract permits):**
- `REPEATED_FAILURE` *(default N = 3; see §4.3)*
- `BUDGET_EXHAUSTED`

### 4.3 REPEATED_FAILURE threshold

- Default threshold: **N = 3** consecutive iterations without reaching the deliverable’s accept gates (per Plan→Workflow mapping).
- When `REPEATED_FAILURE` fires, the Loop MUST transition to `PAUSED` via `StopTriggered` and require `DecisionRecorded` to proceed.

---

## 5. Verification profiles and oracle suites

### 5.1 Role of profiles

Verification profiles define:
- which oracle suites MUST run for a given deliverable/work unit,
- which oracles are REQUIRED vs ADVISORY,
- which environments are pinned,
- what artifacts are captured as evidence bundles.

### 5.2 Profile selection

Profile selection follows the **profile_selection_matrix** in the Profile Definitions (§5.3). Overrides MUST be explicit and attributable.

### 5.3 Instance‑1 Profile Definitions (binding)

The following YAML is the binding first‑pass profile definition set for instance‑1:

```yaml
document:
  title: SR-DIRECTIVE Profile Definitions (Instance-1)
  version: 1.0.0-draft.1
  status: draft
  scope: SOLVER-Ralph instance-1
  created: '2026-01-11'
  updated: '2026-01-11'
  notes:
  - This file defines the verification profiles and oracle suites referenced by the SR-DIRECTIVE Gate Registry for
    instance-1.
  - Suite/profile IDs are referenced as <kind>:<name>@<version>, matching SR-SPEC oracle suite pinning requirements.
  - Replace sha256:__PIN_ME__ with actual OCI image digests before treating these as operationally binding.
oracle_suites:
- suite_id: suite:STRICT-CORE
  suite_version: v1
  description: 'Core deterministic checks intended to be REQUIRED for most candidate deliverables: build, unit tests,
    formatting/lint, and schema validation.'
  determinism_required: true
  environment_constraints: &id001
    runner: oracle_runner:podman+gvisor
    network: disabled
    oci_image_digest: sha256:__PIN_ME__
    cpu_arch: amd64
    os: linux
    additional_constraints:
    - Toolchains/dependencies must be pre-fetched or vendored; no network fetch during required oracles.
    - Workspace inputs must be fully specified via content-addressed refs; no ambient state.
    - Clock/timezone must be pinned; nondeterministic timestamps must be stripped from summaries.
  oracles:
  - oracle_id: oracle:rust-build
    classification: required
    purpose: Build all Rust workspace targets under a pinned toolchain and lockfile.
    command: cargo build --workspace --locked --all-targets
    timeout_seconds: 900
    expected_outputs:
    - exit_code=0
    - build_log.txt
    flake_handling: STOP
    evidence_artifacts:
    - stdout.log
    - stderr.log
    - build_log.txt
  - oracle_id: oracle:rust-unit-tests
    classification: required
    purpose: Run Rust unit tests deterministically for the workspace.
    command: cargo test --workspace --locked --lib
    timeout_seconds: 1200
    expected_outputs:
    - junit.xml
    - test_summary.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - stdout.log
    - stderr.log
    - junit.xml
    - test_summary.json
  - oracle_id: oracle:rust-fmt
    classification: required
    purpose: Enforce Rust formatting (no diffs).
    command: cargo fmt --all --check
    timeout_seconds: 300
    expected_outputs:
    - exit_code=0
    - fmt_report.txt
    flake_handling: STOP
    evidence_artifacts:
    - stdout.log
    - stderr.log
    - fmt_report.txt
  - oracle_id: oracle:rust-clippy
    classification: required
    purpose: Enforce Rust linting (deny warnings).
    command: cargo clippy --workspace --all-targets --locked -- -D warnings
    timeout_seconds: 900
    expected_outputs:
    - exit_code=0
    - clippy_report.txt
    flake_handling: STOP
    evidence_artifacts:
    - stdout.log
    - stderr.log
    - clippy_report.txt
  - oracle_id: oracle:schema-validate
    classification: required
    purpose: Validate governed schemas (events/records) and local JSON schemas against SR-TYPES/SR-SPEC expectations.
    command: make schema-validate
    timeout_seconds: 600
    expected_outputs:
    - schema_validation_report.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - stdout.log
    - stderr.log
    - schema_validation_report.json
  - oracle_id: oracle:ui-build
    classification: required
    purpose: Build the Portal/UI workspace (React) with pinned lockfile to ensure UI compile integrity.
    command: cd ui && pnpm install --frozen-lockfile --offline && pnpm build
    timeout_seconds: 1200
    expected_outputs:
    - ui_build_log.txt
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - stdout.log
    - stderr.log
    - ui_build_log.txt
  - oracle_id: oracle:ui-unit-tests
    classification: advisory
    purpose: Run UI unit tests (advisory by default; reclassify only via governance).
    command: cd ui && pnpm test -- --ci
    timeout_seconds: 1200
    expected_outputs:
    - ui_test_report.xml
    - exit_code=0
    flake_handling: ADVISORY_ONLY
    evidence_artifacts:
    - stdout.log
    - stderr.log
    - ui_test_report.xml
  evidence_bundle_kinds:
  - evidence.gate_packet
  waiver_policy:
    allow_with_exceptions: false
    waiver_eligible_failures: []
    non_waivable_conditions:
    - ORACLE_TAMPER
    - ORACLE_GAP
    - ORACLE_ENV_MISMATCH
    - ORACLE_FLAKE
    - EVIDENCE_MISSING
  notes:
  - If pnpm offline install is not feasible, replace with an OCI image that already contains the pnpm store populated
    from the lockfile; keep network disabled for required oracles.
  - Any change that weakens required oracles or changes required/advisory classification is a governance change
    (C-OR-6).
- suite_id: suite:STRICT-FULL
  suite_version: v1
  description: Extended verification for high-stakes deliverables (integration, e2e, self-host boot, and replayability).
  determinism_required: true
  environment_constraints: &id002
    runner: oracle_runner:podman+gvisor
    network: restricted
    oci_image_digest: sha256:__PIN_ME__
    cpu_arch: amd64
    os: linux
    additional_constraints:
    - Network egress to the public Internet must be blocked; only loopback/internal compose network allowed.
    - Service versions must be pinned (image digests or exact versions) and recorded in evidence.
    - DB/object-store/message-bus state must be initialized deterministically (idempotent bootstrap).
  oracles:
  - oracle_id: oracle:strict-core-suite
    classification: required
    purpose: Run the STRICT-CORE suite as a prerequisite.
    command: make oracle-strict-core
    timeout_seconds: 1800
    expected_outputs:
    - strict_core_summary.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - stdout.log
    - stderr.log
    - strict_core_summary.json
  - oracle_id: oracle:selfhost-smoke
    classification: required
    purpose: Bring up the full stack and verify health checks and pinned versions.
    command: make selfhost-smoke
    timeout_seconds: 1800
    expected_outputs:
    - selfhost_health_report.json
    - versions_pinned.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - stdout.log
    - stderr.log
    - selfhost_health_report.json
    - versions_pinned.json
  - oracle_id: oracle:integration-suite
    classification: required
    purpose: Run integration suite across DB/MinIO/NATS/API (and UI if applicable).
    command: make integration-test
    timeout_seconds: 2400
    expected_outputs:
    - integration_junit.xml
    - integration_summary.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - stdout.log
    - stderr.log
    - integration_junit.xml
    - integration_summary.json
  - oracle_id: oracle:e2e-suite
    classification: required
    purpose: "Run end-to-end suite exercising loop\u2192iteration\u2192candidate\u2192oracles\u2192approval\u2192\
      freeze (or representative subset)."
    command: make e2e-test
    timeout_seconds: 3600
    expected_outputs:
    - e2e_transcript.jsonl
    - e2e_summary.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - stdout.log
    - stderr.log
    - e2e_transcript.jsonl
    - e2e_summary.json
  - oracle_id: oracle:replayability-check
    classification: required
    purpose: Replay a recorded event stream and assert deterministic reconstructed state checksum.
    command: make replayability-check
    timeout_seconds: 1800
    expected_outputs:
    - replay_checksum.txt
    - replay_report.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - stdout.log
    - stderr.log
    - replay_checksum.txt
    - replay_report.json
  - oracle_id: oracle:security-scan
    classification: advisory
    purpose: Run security scanning (advisory by default; treat HIGH/CRITICAL findings as stop triggers via SR-DIRECTIVE).
    command: make security-scan
    timeout_seconds: 1800
    expected_outputs:
    - security_report.sarif
    flake_handling: ADVISORY_ONLY
    evidence_artifacts:
    - stdout.log
    - stderr.log
    - security_report.sarif
  evidence_bundle_kinds:
  - evidence.gate_packet
  waiver_policy:
    allow_with_exceptions: true
    waiver_eligible_failures:
    - oracle:integration-suite
    - oracle:e2e-suite
    non_waivable_conditions:
    - ORACLE_TAMPER
    - ORACLE_GAP
    - ORACLE_ENV_MISMATCH
    - ORACLE_FLAKE
    - EVIDENCE_MISSING
  notes:
  - "Waivers apply only to explicit required-oracle FAIL outcomes and require human approval; they MUST NOT be used\
    \ for integrity conditions (SR-SPEC \xA71.14)."
  - If integration/e2e tests are not deterministic, they MUST NOT remain required (C-OR-1); reclassify only via
    governance (C-OR-6).
- suite_id: suite:CTX-REFSET-VALIDATION
  suite_version: v1
  description: Context admissibility and reference-set validation for IterationStarted refs and deterministic context
    compilation.
  determinism_required: true
  environment_constraints: *id001
  oracles:
  - oracle_id: oracle:refs-required-present
    classification: required
    purpose: Verify required refs are present (SAPS, SR-CONTRACT, SR-SPEC, SR-PLAN instance, SR-DIRECTIVE, SR-ETT)
      and properly typed.
    command: srctl context validate-refs --require standard
    timeout_seconds: 120
    expected_outputs:
    - refs_validation.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - refs_validation.json
  - oracle_id: oracle:refs-dereferenceable
    classification: required
    purpose: Dereference each depends_on ref and verify meta.content_hash matches fetched content.
    command: srctl context deref --verify-hash
    timeout_seconds: 300
    expected_outputs:
    - deref_report.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - deref_report.json
  - oracle_id: oracle:context-compiler-determinism
    classification: required
    purpose: Run ContextCompiler twice from the same refs/payload and ensure identical ContextBundle content hash.
    command: srctl context compile --twice --compare-hash
    timeout_seconds: 600
    expected_outputs:
    - context_bundle_hashes.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - context_bundle_hashes.json
    - context_bundle.manifest.json
  evidence_bundle_kinds:
  - evidence.gate_packet
  waiver_policy:
    allow_with_exceptions: false
    waiver_eligible_failures: []
    non_waivable_conditions:
    - EVIDENCE_MISSING
  notes:
  - Operationalizes SR-ETT accountability/ontological membranes at the iteration boundary (no ghost inputs).
- suite_id: suite:RUNTIME-SUBSTRATE
  suite_version: v1
  description: 'Runtime substrate verification for persistence primitives: event store invariants, projection rebuild
    determinism, and outbox publish semantics.'
  determinism_required: true
  environment_constraints: *id002
  oracles:
  - oracle_id: oracle:strict-core-suite
    classification: required
    purpose: Baseline STRICT-CORE checks prior to runtime substrate tests.
    command: make oracle-strict-core
    timeout_seconds: 1800
    expected_outputs:
    - strict_core_summary.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - strict_core_summary.json
  - oracle_id: oracle:eventstore-invariants
    classification: required
    purpose: Verify event store is append-only, enforces optimistic concurrency, and preserves ordering per stream.
    command: make test-eventstore-invariants
    timeout_seconds: 1200
    expected_outputs:
    - eventstore_invariants_report.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - eventstore_invariants_report.json
    - stdout.log
    - stderr.log
  - oracle_id: oracle:projection-rebuild-determinism
    classification: required
    purpose: Rebuild projections from the same event stream and assert identical projection checksums.
    command: make test-projection-rebuild
    timeout_seconds: 1200
    expected_outputs:
    - projection_rebuild_report.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - projection_rebuild_report.json
  - oracle_id: oracle:outbox-publish-sim
    classification: advisory
    purpose: Simulate outbox publication to NATS/JetStream and verify at-least-once semantics and idempotent consumer
      behavior.
    command: make test-outbox-publish
    timeout_seconds: 1200
    expected_outputs:
    - outbox_publish_report.json
    flake_handling: ADVISORY_ONLY
    evidence_artifacts:
    - outbox_publish_report.json
  evidence_bundle_kinds:
  - evidence.gate_packet
  waiver_policy:
    allow_with_exceptions: false
    waiver_eligible_failures: []
    non_waivable_conditions:
    - ORACLE_TAMPER
    - ORACLE_GAP
    - ORACLE_ENV_MISMATCH
    - ORACLE_FLAKE
    - EVIDENCE_MISSING
  notes:
  - Typically used to ACCEPT deliverables that affect event store/projections (D-10, D-11, D-13).
- suite_id: suite:STALENESS-GRAPH
  suite_version: v1
  description: Dependency graph + staleness traversal verification. Ensures staleness propagation is correct and
    Shippable blocks on unresolved staleness.
  determinism_required: true
  environment_constraints: *id002
  oracles:
  - oracle_id: oracle:dependency-graph-tests
    classification: required
    purpose: Build dependency graph projection and validate edge semantics for depends_on vs supported_by.
    command: make test-dependency-graph
    timeout_seconds: 900
    expected_outputs:
    - dependency_graph_report.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - dependency_graph_report.json
  - oracle_id: oracle:staleness-propagation-tests
    classification: required
    purpose: Validate staleness propagation rules on governed artifact changes and dependency updates.
    command: make test-staleness-propagation
    timeout_seconds: 900
    expected_outputs:
    - staleness_propagation_report.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - staleness_propagation_report.json
  - oracle_id: oracle:shippable-blocks-on-stale
    classification: required
    purpose: Assert candidates with unresolved staleness cannot be Shippable=true.
    command: make test-shippable-staleness
    timeout_seconds: 600
    expected_outputs:
    - shippable_staleness_report.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - shippable_staleness_report.json
  evidence_bundle_kinds:
  - evidence.gate_packet
  waiver_policy:
    allow_with_exceptions: false
    waiver_eligible_failures: []
    non_waivable_conditions:
    - EVIDENCE_MISSING
  notes:
  - Used to ACCEPT D-12 and to support freeze/shippable gating later.
- suite_id: suite:EVIDENCE-INTEGRITY
  suite_version: v1
  description: Evidence ingestion, manifest validation, content-address verification, and retrievability checks
    for evidence bundles.
  determinism_required: true
  environment_constraints: *id002
  oracles:
  - oracle_id: oracle:evidence-manifest-validate
    classification: required
    purpose: Validate evidence bundle manifest schema and required fields.
    command: make oracle-evidence-manifest-validate
    timeout_seconds: 600
    expected_outputs:
    - evidence_manifest_validation.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - evidence_manifest_validation.json
  - oracle_id: oracle:evidence-content-hash-verify
    classification: required
    purpose: Verify each artifact listed is retrievable and matches its content hash.
    command: make oracle-evidence-hash-verify
    timeout_seconds: 900
    expected_outputs:
    - evidence_hash_verify.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - evidence_hash_verify.json
  - oracle_id: oracle:evidence-retrievability-walk
    classification: required
    purpose: Walk a referenced evidence bundle and ensure all artifacts can be fetched under declared access policy.
    command: make oracle-evidence-retrievability
    timeout_seconds: 900
    expected_outputs:
    - evidence_retrievability.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - evidence_retrievability.json
  - oracle_id: oracle:evidence-missing-negative-test
    classification: advisory
    purpose: 'Negative-path: simulate missing evidence and confirm EVIDENCE_MISSING stop trigger.'
    command: make test-evidence-missing-negative
    timeout_seconds: 600
    expected_outputs:
    - evidence_missing_negative_report.json
    flake_handling: ADVISORY_ONLY
    evidence_artifacts:
    - evidence_missing_negative_report.json
  evidence_bundle_kinds:
  - evidence.gate_packet
  waiver_policy:
    allow_with_exceptions: false
    waiver_eligible_failures: []
    non_waivable_conditions:
    - EVIDENCE_MISSING
    - ORACLE_TAMPER
  notes:
  - Used to ACCEPT D-14..D-16 and D-20 and as a prerequisite for portal approvals/freeze.
- suite_id: suite:STOP-TRIGGERS
  suite_version: v1
  description: Oracle integrity and stop-trigger verification (TAMPER/GAP/FLAKE/ENV_MISMATCH) plus correct routing
    to portals without silent override.
  determinism_required: true
  environment_constraints: *id002
  oracles:
  - oracle_id: oracle:tamper-detection-test
    classification: required
    purpose: Simulate evidence hash mismatch and confirm ORACLE_TAMPER is emitted and blocks progression.
    command: make test-oracle-tamper
    timeout_seconds: 600
    expected_outputs:
    - tamper_test_report.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - tamper_test_report.json
  - oracle_id: oracle:gap-detection-test
    classification: required
    purpose: Simulate missing required oracle result and confirm ORACLE_GAP is emitted and blocks progression.
    command: make test-oracle-gap
    timeout_seconds: 600
    expected_outputs:
    - gap_test_report.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - gap_test_report.json
  - oracle_id: oracle:flake-detection-test
    classification: required
    purpose: Run a purposely non-deterministic oracle twice and confirm ORACLE_FLAKE is emitted.
    command: make test-oracle-flake
    timeout_seconds: 600
    expected_outputs:
    - flake_test_report.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - flake_test_report.json
  - oracle_id: oracle:env-mismatch-detection-test
    classification: required
    purpose: Simulate environment constraint mismatch and confirm ORACLE_ENV_MISMATCH is emitted.
    command: make test-oracle-env-mismatch
    timeout_seconds: 600
    expected_outputs:
    - env_mismatch_test_report.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - env_mismatch_test_report.json
  - oracle_id: oracle:trigger-routing-test
    classification: required
    purpose: Verify stop triggers route to the correct portal touchpoints and that no silent override occurs.
    command: make test-stop-trigger-routing
    timeout_seconds: 900
    expected_outputs:
    - trigger_routing_report.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - trigger_routing_report.json
  evidence_bundle_kinds:
  - evidence.gate_packet
  waiver_policy:
    allow_with_exceptions: false
    waiver_eligible_failures: []
    non_waivable_conditions:
    - ORACLE_TAMPER
    - ORACLE_GAP
    - ORACLE_ENV_MISMATCH
    - ORACLE_FLAKE
  notes:
  - Used to ACCEPT D-22 and D-27.
- suite_id: suite:PORTAL-WORKFLOWS
  suite_version: v1
  description: Portal/gov API and UI workflow verification for approvals, exceptions, oracle suite changes, and
    freeze submission semantics.
  determinism_required: true
  environment_constraints: *id002
  oracles:
  - oracle_id: oracle:portal-api-contract-tests
    classification: required
    purpose: Verify portal/gov API endpoints implement SR-SPEC semantics for approvals/exceptions/freezes/shippable
      computation.
    command: make test-portal-api
    timeout_seconds: 1800
    expected_outputs:
    - portal_api_contract_report.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - portal_api_contract_report.json
  - oracle_id: oracle:exception-acknowledgement-check
    classification: required
    purpose: Ensure approvals and freezes explicitly acknowledge active exceptions (even if empty).
    command: make test-portal-exception-ack
    timeout_seconds: 900
    expected_outputs:
    - exception_ack_report.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - exception_ack_report.json
  - oracle_id: oracle:portal-ui-workflow-tests
    classification: advisory
    purpose: UI workflow tests for portal submissions (advisory if UI flakiness is observed).
    command: make test-portal-ui
    timeout_seconds: 1800
    expected_outputs:
    - portal_ui_report.json
    flake_handling: ADVISORY_ONLY
    evidence_artifacts:
    - portal_ui_report.json
  evidence_bundle_kinds:
  - evidence.gate_packet
  waiver_policy:
    allow_with_exceptions: false
    waiver_eligible_failures: []
    non_waivable_conditions:
    - EVIDENCE_MISSING
  notes:
  - Used to ACCEPT D-19 and D-30.
- suite_id: suite:SELFHOST-SMOKE
  suite_version: v1
  description: 'Self-host smoke suite: boot stack, health checks, idempotent init, and version pinning evidence.'
  determinism_required: true
  environment_constraints: *id002
  oracles:
  - oracle_id: oracle:selfhost-boot
    classification: required
    purpose: Start self-host stack and wait for readiness.
    command: make selfhost-up-and-wait
    timeout_seconds: 1800
    expected_outputs:
    - startup_logs.txt
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - startup_logs.txt
  - oracle_id: oracle:selfhost-health
    classification: required
    purpose: Run health checks for core services (API, DB, MinIO, NATS, Zitadel).
    command: make selfhost-health
    timeout_seconds: 600
    expected_outputs:
    - health_report.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - health_report.json
  - oracle_id: oracle:versions-pinned-report
    classification: required
    purpose: Emit a machine-readable report of pinned service versions/digests.
    command: make selfhost-versions-report
    timeout_seconds: 300
    expected_outputs:
    - versions_pinned.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - versions_pinned.json
  evidence_bundle_kinds:
  - evidence.gate_packet
  waiver_policy:
    allow_with_exceptions: false
    waiver_eligible_failures: []
    non_waivable_conditions:
    - ORACLE_ENV_MISMATCH
    - ORACLE_TAMPER
    - EVIDENCE_MISSING
  notes:
  - Used to ACCEPT D-31..D-33 and as a prerequisite for integration/e2e suites.
- suite_id: suite:E2E-HAPPY
  suite_version: v1
  description: "End-to-end 'happy path' harness execution: loop\u2192iteration\u2192candidate\u2192oracles\u2192\
    approval\u2192freeze."
  determinism_required: true
  environment_constraints: *id002
  oracles:
  - oracle_id: oracle:e2e-happy-harness
    classification: required
    purpose: Run the happy-path harness and record a full transcript with object IDs.
    command: make harness-e2e-happy
    timeout_seconds: 3600
    expected_outputs:
    - harness_transcript.jsonl
    - harness_outputs.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - harness_transcript.jsonl
    - harness_outputs.json
  - oracle_id: oracle:freeze-validated-post-harness
    classification: required
    purpose: Validate the produced freeze record and approval linkage from the harness run.
    command: make harness-validate-freeze
    timeout_seconds: 900
    expected_outputs:
    - freeze_validation_report.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - freeze_validation_report.json
  evidence_bundle_kinds:
  - evidence.gate_packet
  waiver_policy:
    allow_with_exceptions: false
    waiver_eligible_failures: []
    non_waivable_conditions:
    - EVIDENCE_MISSING
  notes:
  - Used to ACCEPT D-34 (happy-path proof).
- suite_id: suite:E2E-FAILURE
  suite_version: v1
  description: 'End-to-end ''failure path'' harness execution: inject integrity failures, exercise waiver/exception
    flows, and prove stop-the-line behavior.'
  determinism_required: true
  environment_constraints: *id002
  oracles:
  - oracle_id: oracle:e2e-failure-harness
    classification: required
    purpose: Run failure-mode harness with deterministic fault injection and record transcripts + portal submissions.
    command: make harness-e2e-failure
    timeout_seconds: 3600
    expected_outputs:
    - failure_transcript.jsonl
    - failure_outputs.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - failure_transcript.jsonl
    - failure_outputs.json
  - oracle_id: oracle:nonwaivable-integrity-enforced
    classification: required
    purpose: Prove integrity conditions cannot be waived and always stop the line.
    command: make harness-verify-nonwaivable
    timeout_seconds: 900
    expected_outputs:
    - nonwaivable_report.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - nonwaivable_report.json
  evidence_bundle_kinds:
  - evidence.gate_packet
  waiver_policy:
    allow_with_exceptions: false
    waiver_eligible_failures: []
    non_waivable_conditions:
    - ORACLE_TAMPER
    - ORACLE_GAP
    - ORACLE_ENV_MISMATCH
    - ORACLE_FLAKE
  notes:
  - Used to ACCEPT D-35 (failure-mode proof).
- suite_id: suite:REPLAYABILITY
  suite_version: v1
  description: 'Replayability and determinism proof: replay event stream and verify reconstructed state/projections
    match checksum and invariants.'
  determinism_required: true
  environment_constraints: *id001
  oracles:
  - oracle_id: oracle:replay-event-stream
    classification: required
    purpose: Replay recorded event stream into empty store and rebuild all projections.
    command: make replay-run
    timeout_seconds: 1800
    expected_outputs:
    - replay_logs.txt
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - replay_logs.txt
  - oracle_id: oracle:replay-checksum-compare
    classification: required
    purpose: Compare reconstructed projection checksums against baseline and assert equality.
    command: make replay-compare-checksums
    timeout_seconds: 600
    expected_outputs:
    - replay_checksum.txt
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - replay_checksum.txt
  evidence_bundle_kinds:
  - evidence.gate_packet
  waiver_policy:
    allow_with_exceptions: false
    waiver_eligible_failures: []
    non_waivable_conditions:
    - EVIDENCE_MISSING
  notes:
  - Used to ACCEPT D-36.
- suite_id: suite:FREEZE-VALIDATION
  suite_version: v1
  description: 'Freeze gating validation: freeze record completeness, explicit exception acknowledgement, no unresolved
    staleness, and Shippable computation correctness.'
  determinism_required: true
  environment_constraints: *id001
  oracles:
  - oracle_id: oracle:freeze-record-completeness
    classification: required
    purpose: Validate FreezeRecordCreated contains required fields and enumerates governed artifacts + evidence
      pointers.
    command: srctl freeze validate --completeness
    timeout_seconds: 600
    expected_outputs:
    - freeze_completeness.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - freeze_completeness.json
  - oracle_id: oracle:approval-acknowledges-exceptions
    classification: required
    purpose: Ensure ReleaseApprovalPortal approval explicitly acknowledges active exceptions (including empty list).
    command: srctl approval validate --exceptions-ack
    timeout_seconds: 300
    expected_outputs:
    - exceptions_ack_check.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - exceptions_ack_check.json
  - oracle_id: oracle:staleness-none-check
    classification: required
    purpose: Ensure no unresolved staleness markers affect the candidate, its dependencies, the oracle suite, or
      governed artifacts.
    command: srctl staleness assert-none --candidate
    timeout_seconds: 300
    expected_outputs:
    - staleness_none_check.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - staleness_none_check.json
  - oracle_id: oracle:shippable-computation-check
    classification: required
    purpose: Compute Shippable per SR-SPEC and assert Shippable=true only when all preconditions hold.
    command: srctl candidate compute-shippable --assert
    timeout_seconds: 300
    expected_outputs:
    - shippable_check.json
    - exit_code=0
    flake_handling: STOP
    evidence_artifacts:
    - shippable_check.json
  evidence_bundle_kinds:
  - evidence.gate_packet
  waiver_policy:
    allow_with_exceptions: false
    waiver_eligible_failures: []
    non_waivable_conditions:
    - EVIDENCE_MISSING
    - ORACLE_GAP
  notes:
  - Used at the release gate (G-90) before treating any baseline as Shippable.
verification_profiles:
- profile_id: STRICT-CORE
  profile_version: v1
  description: Default verification profile for most candidate deliverables. Requires deterministic core checks
    (build/tests/lint/schema).
  required_suites:
  - suite:STRICT-CORE@v1
  advisory_suites: []
  verification_mode_default: STRICT
  waiver_policy:
    allow_with_exceptions: false
    waiver_eligible_failures: []
    non_waivable_conditions:
    - ORACLE_TAMPER
    - ORACLE_GAP
    - ORACLE_ENV_MISMATCH
    - ORACLE_FLAKE
    - EVIDENCE_MISSING
  notes:
  - This profile is intended to remain small and deterministic; add oracles conservatively.
  - Advisory oracles may be added without changing the Verified meaning, but changing required/advisory classification
    is governance-controlled (C-OR-6).
- profile_id: STRICT-FULL
  profile_version: v1
  description: High-assurance verification profile for deliverables that are on the critical path to end-to-end
    proofs or release readiness.
  required_suites:
  - suite:STRICT-FULL@v1
  advisory_suites:
  - suite:STOP-TRIGGERS@v1
  verification_mode_default: STRICT
  waiver_policy:
    allow_with_exceptions: true
    waiver_eligible_failures:
    - oracle:integration-suite
    - oracle:e2e-suite
    non_waivable_conditions:
    - ORACLE_TAMPER
    - ORACLE_GAP
    - ORACLE_ENV_MISMATCH
    - ORACLE_FLAKE
    - EVIDENCE_MISSING
  notes:
  - If WITH_EXCEPTIONS is used, a Waiver record and portal decision are required; ReleaseApproval must explicitly
    acknowledge active exceptions (SR-SPEC freeze/approval requirements).
profile_selection_matrix:
- work_unit_type: deliverable.record.governance
  deliverable_ids:
  - D-01
  default_profile_id: null
  override_rules: []
  notes:
  - D-01 is a governance record (SAPS) and is accepted via context admissibility + governance portal, not by oracle
    suites.
- work_unit_type: deliverable.candidate.default
  deliverable_ids:
  - D-02
  - D-03
  - D-04
  - D-05
  - D-06
  - D-07
  - D-08
  - D-09
  - D-10
  - D-11
  - D-12
  - D-13
  - D-14
  - D-15
  - D-16
  - D-17
  - D-18
  - D-19
  - D-20
  - D-21
  - D-22
  - D-23
  - D-24
  - D-25
  - D-27
  - D-28
  - D-29
  - D-30
  - D-33
  default_profile_id: profile:STRICT-CORE@v1
  override_rules:
  - if: deliverable_modifies_governed_artifact == true
    then:
      require_portal: GovernanceChangePortal
      verification_profile: profile:STRICT-CORE@v1
  - if: risk_level == HIGH
    then:
      verification_profile: profile:STRICT-FULL@v1
  notes:
  - Applies to the majority of code deliverables. Upgrade to STRICT-FULL via risk escalation or when the deliverable
    becomes release-critical.
- work_unit_type: deliverable.candidate.release_critical
  deliverable_ids:
  - D-26
  - D-31
  - D-32
  - D-34
  - D-35
  - D-36
  default_profile_id: profile:STRICT-FULL@v1
  override_rules:
  - if: integrity_conditions_present == true
    then:
      block: true
      route_to_portal: GovernanceChangePortal
  notes:
  - Deliverables on the path to self-host readiness and end-to-end proofs. These default to STRICT-FULL as recommended
    by SR-PLAN.
```

---

## 6. Gate Registry (binding control surface)

The Gate Registry is the binding control surface that turns plan mapping into enforceable workflow rules.

> **Key principle (SR‑ETT):** Coercion belongs at membranes—where outputs become inputs, especially across actor boundaries.  
> Gates are those membranes made explicit, named, and testable.

# SR-DIRECTIVE Gate Registry — Instance-1 (v1.0.0-draft.2)

*Generated:* 2026-01-11

This registry defines the binding gate control surface for executing SR-PLAN instance-1.
Each gate specifies membranes (SR-ETT), enforcement surfaces, required refs/evidence, and routing/relief valves.

## G-00: Context admissible (IterationStarted ref-set; no ghost inputs)

- **Gate kind:** work_start

- **Applies to:** D-01..D-36

- **Purpose/decision:** Establish an admissible, deterministic Iteration context by enforcing the normative IterationStarted.refs[] checklist and prohibiting any unreferenced inputs.

- **Membranes enforced (SR-ETT):** Intent & Objective; Ontological; Accountability; Authority & Integrity; Change

- **Enforcement mechanism:** IterationStarted schema validation; ref dereference + content_hash validation; ContextCompiler deterministic compilation; staleness preflight on depends_on refs

- **Allowed actor kinds:** SYSTEM

- **Required refs (depends_on):** Loop (rel=in_scope_of); GovernedArtifact SR-TYPES/SR-CONTRACT/SR-SPEC/SR-DIRECTIVE (rel=depends_on, version+content_hash); SR-PLAN instance (rel=depends_on); SAPS / problem_statement record (rel=depends_on); OracleSuite intended for verification (rel=depends_on); Prior Iteration summaries carried forward (rel=depends_on, optional); Base Candidate (rel=depends_on, optional but required for incremental work); Active exceptions (Deviation/Deferral/Waiver, rel=depends_on, optional/empty allowed); Human notes (intervention/evaluation/assessment, rel=depends_on when present/required by gating policy)

- **Required commitment objects:** LoopCreated; IterationStarted (actor_kind=SYSTEM); referenced governed artifacts are registered/available with matching content_hash

- **Required evidence:** N/A (capture preflight logs as evidence.gate_packet when blocked; optional for audit)

- **Verification profile / suite:** suite:CTX-REFSET-VALIDATION@v1

- **Success outputs:** IterationStarted accepted; ContextBundle compiled deterministically; iteration may proceed

- **Success state effect:** Iteration may start; agent work is permitted only within compiled ContextBundle

- **Failure conditions:** Missing required refs or missing meta.content_hash; refs not dereferenceable; governed artifact versions/hashes unknown; oracle suite ref missing; active exceptions referenced but unavailable; required human note missing per gating policy; depends_on refs stale with unresolved staleness markers

- **Stop triggers on failure:** EVIDENCE_MISSING (if any required ref cannot be fetched); ORACLE_GAP (if required OracleSuite ref missing); REPEATED_FAILURE (if repeated inability to compile context)

- **Relief valve:** DecisionRecorded (human) to amend ref set / resolve staleness / correct scope; GovernanceChangePortal for systemic ref-set/schema issues

- **Routing authority boundary on block:** GovernanceChangePortal (systemic); otherwise DecisionRecorded (human) for this loop/iteration

- **Contract refs:** C-CTX-1; C-CTX-2; C-EVID-6; C-EVT-1

- **Spec refs:** SR-SPEC §3.2.1.1; SR-SPEC ContextCompiler semantics; SR-SPEC §1.13 (staleness)

- **Plan refs:** D-01..D-36

- **Notes:** Default conservative: if context cannot be proven admissible, block iteration start rather than proceed on implicit/ambient inputs.


## G-10: Runtime substrate verified (event log + projections + outbox invariants)

- **Gate kind:** foundation_accept

- **Applies to:** D-09..D-13; D-18; D-21..D-23; D-32; D-34; D-36

- **Purpose/decision:** Accept foundational runtime/persistence components only when the event log is append-only, projection rebuild is deterministic, and outbox publication invariants hold under the declared environment constraints.

- **Membranes enforced (SR-ETT):** Architectural; Operational; Accountability; Authority & Integrity; Isomorphic

- **Enforcement mechanism:** oracle_run (integration suite); projection rebuild checks; event-envelope conformance tests; outbox/at-least-once tests

- **Allowed actor kinds:** SYSTEM

- **Required refs (depends_on):** Candidate (rel=depends_on); OracleSuite suite:RUNTIME-SUBSTRATE (rel=depends_on); governing artifacts in force (rel=depends_on)

- **Required commitment objects:** RunStarted + RunCompleted; EvidenceBundleRecorded; (if applicable) projection tables created/migrated

- **Required evidence:** Evidence bundle containing: event-store invariant tests; projection rebuild determinism logs; outbox publish simulation logs

- **Verification profile / suite:** suite:RUNTIME-SUBSTRATE@v1

- **Success outputs:** EvidenceBundleRecorded with PASS for required runtime invariants

- **Success state effect:** Candidate verification status may be computed as Verified (Strict) for the runtime substrate scope; downstream deliverables may depend_on this substrate

- **Failure conditions:** Any required runtime invariant oracle FAIL; inability to rebuild projections deterministically; non-monotonic global_seq; event-envelope schema violations; outbox duplication/ordering violations outside declared guarantees; oracle integrity faults (tamper/gap/flake/env mismatch); evidence not retrievable

- **Stop triggers on failure:** ORACLE_TAMPER; ORACLE_GAP; ORACLE_ENV_MISMATCH; ORACLE_FLAKE; EVIDENCE_MISSING

- **Relief valve:** Fix implementation; if a requirement is infeasible, route Deviation/Deferral via GovernanceChangePortal (do not silently weaken invariants).

- **Routing authority boundary on block:** GovernanceChangePortal (for requirement conflicts) / DecisionRecorded (human) for stop-trigger arbitration

- **Contract refs:** C-ARCH-1; C-ARCH-3; C-EVT-3; C-OR-1; C-OR-2; C-EVID-6

- **Spec refs:** SR-SPEC §1.2 (event envelope); SR-SPEC §1.5 (event log); SR-SPEC §1.6–§1.8 (projections/outbox); SR-SPEC §3.3 (Verified rules)

- **Plan refs:** D-09..D-13; D-18; D-21..D-23; D-32; D-34; D-36

- **Notes:** This gate is meant for *substrate* components whose failures poison downstream work; treat failures as stop-the-line until resolved. | [vpos-note] includes STRICT-CORE baseline + runtime invariants


## G-15: Dependency graph + staleness routing verified

- **Gate kind:** invariant_accept

- **Applies to:** D-12; D-19

- **Purpose/decision:** Accept dependency-graph and staleness traversal behavior only when staleness propagation, re-evaluation triggering, and unresolved-staleness blocking semantics match SR-SPEC.

- **Membranes enforced (SR-ETT):** Isomorphic; Ontological; Operational; Accountability; Change

- **Enforcement mechanism:** oracle_run (staleness traversal suite); projection conformance checks; reference-relationship validation (depends_on vs supported_by)

- **Allowed actor kinds:** SYSTEM

- **Required refs (depends_on):** Candidate (rel=depends_on); OracleSuite suite:STALENESS-GRAPH (rel=depends_on); governing artifacts in force (rel=depends_on)

- **Required commitment objects:** RunStarted + RunCompleted; EvidenceBundleRecorded; dependency graph projection populated

- **Required evidence:** Evidence bundle containing: dependency graph build tests; staleness traversal tests; re-evaluation routing tests

- **Verification profile / suite:** suite:STALENESS-GRAPH@v1

- **Success outputs:** EvidenceBundleRecorded with PASS for staleness/graph semantics

- **Success state effect:** Staleness markers can be relied upon to block Shippable and trigger re-evaluation deterministically

- **Failure conditions:** Incorrect traversal/propagation; failure to block when upstream dependency changes; incorrect handling of rel=depends_on vs rel=supported_by; non-deterministic projection rebuild; evidence not retrievable

- **Stop triggers on failure:** EVIDENCE_MISSING; ORACLE_GAP; ORACLE_TAMPER; ORACLE_FLAKE

- **Relief valve:** Fix implementation; if semantics need change, route governance change (SR-SPEC/SR-CONTRACT) via GovernanceChangePortal.

- **Routing authority boundary on block:** GovernanceChangePortal (semantic conflicts) / DecisionRecorded (human) for arbitration

- **Contract refs:** C-EVT-6; C-SHIP-1; C-CTX-1

- **Spec refs:** SR-SPEC §1.13 (staleness); SR-SPEC §1.12.4 (Shippable requires no unresolved staleness); SR-SPEC §3.2.1.1 (rel semantics)

- **Plan refs:** D-12; D-19

- **Notes:** This gate is load-bearing for staleness-aware Shippable gating; keep it strict and avoid waiver by default.


## G-20: Evidence store integrity + retrievability verified

- **Gate kind:** foundation_accept

- **Applies to:** D-14..D-16; D-19..D-20; D-23..D-27; D-29; D-32..D-34

- **Purpose/decision:** Accept evidence subsystem components only when evidence bundles are content-addressed, validated on ingest, remain retrievable, and missing evidence is detected and blocks progression.

- **Membranes enforced (SR-ETT):** Accountability; Authority & Integrity; Operational

- **Enforcement mechanism:** oracle_run (evidence integrity suite); evidence manifest schema validation; retrievability reference-walk tests; negative tests for missing evidence

- **Allowed actor kinds:** SYSTEM

- **Required refs (depends_on):** Candidate (rel=depends_on); OracleSuite suite:EVIDENCE-INTEGRITY (rel=depends_on); governing artifacts in force (rel=depends_on)

- **Required commitment objects:** RunStarted + RunCompleted; EvidenceBundleRecorded; evidence objects stored content-addressed

- **Required evidence:** Evidence bundle containing: ingest validation tests; content-hash verification logs; retrieval checks; EVIDENCE_MISSING negative-path proof

- **Verification profile / suite:** suite:EVIDENCE-INTEGRITY@v1

- **Success outputs:** EvidenceBundleRecorded with PASS for evidence ingest/validation/retrievability

- **Success state effect:** Evidence may be used as basis for Verified claims; missing evidence will be caught and block binding claims

- **Failure conditions:** Evidence manifest schema invalid; content hash mismatch; evidence objects not retrievable; presigned-url or storage layer breaks auditability; missing evidence not detected or does not block; oracle integrity faults

- **Stop triggers on failure:** EVIDENCE_MISSING; ORACLE_GAP; ORACLE_TAMPER; ORACLE_ENV_MISMATCH; ORACLE_FLAKE

- **Relief valve:** Fix storage/ingest; if retention constraints conflict, route Deviation/Deferral via GovernanceChangePortal (do not allow silent loss of evidence).

- **Routing authority boundary on block:** GovernanceChangePortal (retention/policy conflicts) / DecisionRecorded (human) for incident handling

- **Contract refs:** C-EVID-1; C-EVID-2; C-EVID-6; C-OR-2; C-EVT-1

- **Spec refs:** SR-SPEC §1.9 (EvidenceBundle manifest); SR-SPEC §2.3.3 (Runs and EvidenceBundleRecorded); SR-SPEC §3.4 (EVIDENCE_MISSING handling)

- **Plan refs:** D-14..D-16; D-19..D-20; D-23..D-27; D-29; D-32..D-34

- **Notes:** Evidence availability is non-negotiable for binding claims; treat storage incidents as stop-the-line for affected claims.


## G-30: Core verification PASS (STRICT-CORE) for deliverable acceptance

- **Gate kind:** verification_accept

- **Applies to:** D-02..D-12; D-14..D-15; D-17..D-20; D-22..D-31; D-33..D-34

- **Purpose/decision:** Accept a candidate deliverable only when the assigned core verification profile passes with deterministic required oracles, suite pinning is intact, and evidence is recorded and retrievable.

- **Membranes enforced (SR-ETT):** Architectural; Ontological; Accountability; Authority & Integrity; Operational

- **Enforcement mechanism:** RunStarted/RunCompleted; required oracle suite execution; EvidenceBundleRecorded; candidate status computation

- **Allowed actor kinds:** SYSTEM

- **Required refs (depends_on):** Candidate (rel=depends_on); OracleSuite profile/suite for STRICT-CORE (rel=depends_on); governing artifacts in force (rel=depends_on); active exceptions in scope (rel=depends_on, optional)

- **Required commitment objects:** RunStarted + RunCompleted; EvidenceBundleRecorded; (if WITH_EXCEPTIONS) WaiverCreated + WaiverActivated

- **Required evidence:** Evidence bundle manifest + logs for all required oracles in STRICT-CORE (build/test/lint/schema etc, as defined in the profile)

- **Verification profile / suite:** profile:STRICT-CORE@v1

- **Success outputs:** EvidenceBundleRecorded; Candidate computed as Verified (Strict) OR Verified-with-Exceptions (only if waiver(s) exist and are in-scope)

- **Success state effect:** Deliverable may be marked Accepted for its workflow phase; downstream depends_on edges may proceed

- **Failure conditions:** Any required oracle FAIL without an in-scope active waiver; any oracle integrity fault (tamper/gap/flake/env mismatch); evidence missing/unretrievable; suite hash mismatch; attempt to treat advisory or human notes as replacing evidence

- **Stop triggers on failure:** ORACLE_TAMPER; ORACLE_GAP; ORACLE_ENV_MISMATCH; ORACLE_FLAKE; EVIDENCE_MISSING; REPEATED_FAILURE (if N consecutive no-progress iterations)

- **Relief valve:** Fix failing checks; if acceptability requires exceptions, route WaiverCreated (human) for specific FAIL outcomes only; for systemic changes, route GovernanceChangePortal.

- **Routing authority boundary on block:** DecisionRecorded (human) for stop-trigger arbitration; GovernanceChangePortal for systemic requirement changes; /exceptions/waivers (human) for waiver requests

- **Contract refs:** C-VER-1; C-VER-2; C-OR-1; C-OR-2; C-EVID-6; C-TB-7

- **Spec refs:** SR-SPEC §3.3 (Candidate verification status); SR-SPEC §3.5 (suite pinning); SR-SPEC §2.3.3 (EvidenceBundleRecorded)

- **Plan refs:** D-02..D-12; D-14..D-15; D-17..D-20; D-22..D-31; D-33..D-34

- **Notes:** Do not downgrade required oracles to advisory without governance change routing; keep STRICT as default unless explicitly allowed for a work-unit. | [vpos-note] required oracles deterministic; advisory allowed


## G-31: Full verification PASS (STRICT-FULL) for high-stakes acceptance

- **Gate kind:** verification_accept

- **Applies to:** D-26

- **Purpose/decision:** Accept a candidate only when extended integration/e2e verification passes under deterministic required oracles, with evidence recorded and suite pinned.

- **Membranes enforced (SR-ETT):** Architectural; Isomorphic; Accountability; Authority & Integrity; Operational

- **Enforcement mechanism:** RunStarted/RunCompleted; extended oracle suite execution; EvidenceBundleRecorded; candidate status computation

- **Allowed actor kinds:** SYSTEM

- **Required refs (depends_on):** Candidate (rel=depends_on); OracleSuite profile/suite for STRICT-FULL (rel=depends_on); governing artifacts in force (rel=depends_on); active exceptions (rel=depends_on, optional)

- **Required commitment objects:** RunStarted + RunCompleted; EvidenceBundleRecorded; (if WITH_EXCEPTIONS) WaiverCreated + WaiverActivated

- **Required evidence:** Evidence bundle manifest + logs for all required oracles in STRICT-FULL (integration/e2e/security etc as defined in the profile)

- **Verification profile / suite:** profile:STRICT-FULL@v1

- **Success outputs:** EvidenceBundleRecorded; Candidate computed as Verified (Strict) OR Verified-with-Exceptions (waiver-scoped)

- **Success state effect:** High-stakes deliverable may be accepted and used as basis for end-to-end gates

- **Failure conditions:** Required oracle FAIL without in-scope waiver; integrity faults; evidence missing; suite hash mismatch; non-deterministic required oracle

- **Stop triggers on failure:** ORACLE_TAMPER; ORACLE_GAP; ORACLE_ENV_MISMATCH; ORACLE_FLAKE; EVIDENCE_MISSING

- **Relief valve:** Fix; or request tightly-scoped waiver (human) for specific FAIL results; otherwise route governance change for profile/suite modification.

- **Routing authority boundary on block:** DecisionRecorded (human) / GovernanceChangePortal / /exceptions/waivers (human)

- **Contract refs:** C-VER-1; C-OR-1; C-OR-2; C-EVID-6

- **Spec refs:** SR-SPEC §3.3 (Verified rules); SR-SPEC §3.5 (suite pinning); SR-SPEC §2.3.3

- **Plan refs:** D-26

- **Notes:** Use sparingly for gates/deliverables that would otherwise create high-cost false positives (e.g., integration suite itself, release candidates).


## G-40: Stop-the-line triggers + integrity conditions enforced

- **Gate kind:** integrity_enforcement

- **Applies to:** D-22..D-23; D-27; D-33..D-35

- **Purpose/decision:** Confirm that mandatory stop triggers and integrity conditions are implemented end-to-end: integrity faults halt progression, StopTriggered is emitted, loops pause, and human DecisionRecorded is required to proceed.

- **Membranes enforced (SR-ETT):** Operational; Authority & Integrity; Accountability; Resource; Change

- **Enforcement mechanism:** oracle_run (stop-trigger suite); simulated fault injection; loop lifecycle assertions (PAUSED gating); decision/arbitration workflow checks

- **Allowed actor kinds:** SYSTEM (trigger detection) + HUMAN (DecisionRecorded to resume)

- **Required refs (depends_on):** Candidate (rel=depends_on); OracleSuite suite:STOP-TRIGGERS (rel=depends_on); governing artifacts in force (rel=depends_on)

- **Required commitment objects:** StopTriggered; Loop state transitions to PAUSED; DecisionRecorded (human) required for resume; EvidenceMissingDetected (when applicable)

- **Required evidence:** Evidence bundle containing: fault-injection transcripts for each mandatory trigger; lifecycle projection assertions; decision workflow proofs

- **Verification profile / suite:** suite:STOP-TRIGGERS@v1

- **Success outputs:** EvidenceBundleRecorded (from stop-trigger suite); demonstrated StopTriggered→PAUSED→DecisionRecorded path

- **Success state effect:** Stop-the-line discipline may be relied upon during plan execution; integrity faults are non-waivable

- **Failure conditions:** Any mandatory trigger not emitted when condition occurs; loop does not pause; system proceeds without DecisionRecorded; attempted waiver of non-waivable integrity; missing evidence does not block; REPEATED_FAILURE threshold undefined in directive

- **Stop triggers on failure:** ORACLE_TAMPER; ORACLE_GAP; ORACLE_ENV_MISMATCH; ORACLE_FLAKE; EVIDENCE_MISSING; REPEATED_FAILURE; BUDGET_EXHAUSTED

- **Relief valve:** Fix; if arbitration semantics need change, route governance change via GovernanceChangePortal (do not bypass stop-the-line).

- **Routing authority boundary on block:** DecisionRecorded (human) for arbitration; GovernanceChangePortal for systemic trigger/semantics changes

- **Contract refs:** C-LOOP-3; C-DEC-1; C-OR-2; C-EVID-6

- **Spec refs:** SR-SPEC §3.4 (mandatory stop triggers; PAUSED semantics); SR-SPEC Loop lifecycle (§3.1); SR-SPEC DecisionRecorded model (§1.11)

- **Plan refs:** D-22..D-23; D-27; D-33..D-35

- **Notes:** This gate is itself integrity-critical; treat failures as blockers for any downstream work that depends on budgets/stop triggers.


## G-50: Portal workflows functional (approvals/exceptions/freeze submission paths)

- **Gate kind:** integration_accept

- **Applies to:** D-19; D-30; D-34

- **Purpose/decision:** Accept portal-facing surfaces only when portal workflows can create the required binding records (ApprovalRecorded, WaiverCreated, FreezeRecordCreated) with correct field constraints and actor-kind enforcement.

- **Membranes enforced (SR-ETT):** Authority & Integrity; Accountability; Ontological; Change

- **Enforcement mechanism:** oracle_run (portal workflow integration suite); API contract tests; UI-to-API trace tests; actor-kind enforcement checks

- **Allowed actor kinds:** SYSTEM (tests) + HUMAN (binding actions at runtime)

- **Required refs (depends_on):** Candidate (rel=depends_on); governing artifacts in force (rel=depends_on); OracleSuite suite:PORTAL-WORKFLOWS (rel=depends_on)

- **Required commitment objects:** ApprovalRecorded (human) created via portal API; WaiverCreated (human) where applicable; FreezeRecordCreated (human) where applicable

- **Required evidence:** Evidence bundle containing: API integration traces for approvals/waivers/freeze; schema validation logs; negative tests for actor_kind!=HUMAN

- **Verification profile / suite:** suite:PORTAL-WORKFLOWS@v1

- **Success outputs:** EvidenceBundleRecorded; verified ability to produce required binding records through portal workflows

- **Success state effect:** Portal boundary crossings are operationally usable for later gates (release/freeze/exception workflows)

- **Failure conditions:** Cannot create ApprovalRecorded; approval missing required fields; portal allows agent/system to spoof human actor_kind; FreezeRecordCreated rejected due to missing exception acknowledgement but UI/API fails to capture; waiver scope constraints not enforced

- **Stop triggers on failure:** ORACLE_GAP; ORACLE_TAMPER; ORACLE_FLAKE; EVIDENCE_MISSING

- **Relief valve:** Fix implementation; if portal field requirements change, route SR-SPEC/SR-CONTRACT update via GovernanceChangePortal.

- **Routing authority boundary on block:** GovernanceChangePortal (schema/semantics conflicts) / DecisionRecorded (human) for operational arbitration

- **Contract refs:** C-TB-1; C-TB-4; C-TB-6; C-TB-7; C-SHIP-1

- **Spec refs:** SR-SPEC §2.3.4 (Approvals); SR-SPEC §2.3.5 (Exceptions); SR-SPEC §1.12 (Freeze record fields and exception acknowledgement)

- **Plan refs:** D-19; D-30; D-34

- **Notes:** This gate is about *functional correctness* of portal workflows, not about granting approvals for a real release.


## G-60: Self-host deploy smoke verified (single-command up + health checks)

- **Gate kind:** ops_accept

- **Applies to:** D-26; D-31..D-34

- **Purpose/decision:** Accept self-host/ops deliverables only when the full stack boots deterministically with pinned versions and passes health checks in the declared environment.

- **Membranes enforced (SR-ETT):** Resource; Operational; Accountability; Authority & Integrity

- **Enforcement mechanism:** oracle_run (selfhost smoke suite); deployment harness; health-check probes; version pinning validation

- **Allowed actor kinds:** SYSTEM

- **Required refs (depends_on):** Candidate (rel=depends_on); OracleSuite suite:SELFHOST-SMOKE (rel=depends_on); governing artifacts in force (rel=depends_on)

- **Required commitment objects:** RunStarted + RunCompleted; EvidenceBundleRecorded; deployment logs captured

- **Required evidence:** Evidence bundle containing: bootstrap logs; service health checks; API ping tests; version/digest pinning report

- **Verification profile / suite:** suite:SELFHOST-SMOKE@v1

- **Success outputs:** EvidenceBundleRecorded with PASS for self-host boot + health checks

- **Success state effect:** Self-host environment may be used as a substrate for e2e and replayability harnesses

- **Failure conditions:** Stack does not boot; health checks fail; non-deterministic boot; required secrets not handled per policy; evidence missing

- **Stop triggers on failure:** BUDGET_EXHAUSTED; REPEATED_FAILURE; EVIDENCE_MISSING; ORACLE_ENV_MISMATCH

- **Relief valve:** Fix; if environment constraints need adjustment, route suite/environment changes via GovernanceChangePortal.

- **Routing authority boundary on block:** DecisionRecorded (human) for budget/stop arbitration; GovernanceChangePortal for environment constraint changes

- **Contract refs:** C-LOOP-1; C-OR-1; C-EVID-6

- **Spec refs:** SR-SPEC §3.5 (environment constraints); SR-SPEC §2.3 (API surface)

- **Plan refs:** D-26; D-31..D-34

- **Notes:** Keep this gate fast (smoke) but strict on determinism and pinning to prevent 'works on my machine' drift.


## G-70: End-to-end happy path transcript verified

- **Gate kind:** e2e_accept

- **Applies to:** D-34..D-36

- **Purpose/decision:** Accept end-to-end harness only when the canonical happy path executes: Loop→IterationStarted→Candidate submission→Run(oracles)→EvidenceBundleRecorded→ApprovalRecorded→FreezeRecordCreated, producing a replayable transcript and IDs.

- **Membranes enforced (SR-ETT):** Intent & Objective; Operational; Architectural; Ontological; Isomorphic; Change; Authority & Integrity; Resource; Accountability

- **Enforcement mechanism:** oracle_run (e2e harness); transcript capture; invariant assertions across projections; simulated human actor for portal steps in test env

- **Allowed actor kinds:** SYSTEM (harness) + HUMAN (simulated test identity for portal steps) + ORACLE (run execution)

- **Required refs (depends_on):** Candidate (rel=depends_on); OracleSuite suite:E2E-HAPPY (rel=depends_on); governing artifacts in force (rel=depends_on); self-host environment ref (rel=depends_on)

- **Required commitment objects:** IterationStarted; RunStarted/RunCompleted; EvidenceBundleRecorded; ApprovalRecorded; FreezeRecordCreated

- **Required evidence:** Evidence bundle containing: full harness transcript/logs; produced object IDs; assertions for each membrane boundary; retrievability check results

- **Verification profile / suite:** suite:E2E-HAPPY@v1

- **Success outputs:** EvidenceBundleRecorded + harness transcript; produced ApprovalRecorded and FreezeRecordCreated in test

- **Success state effect:** Demonstrated canonical workflow is executable; unlocks failure-path and replayability proofs

- **Failure conditions:** Any step in canonical workflow fails; approvals/freeze cannot be recorded; harness cannot demonstrate replayable transcript; integrity faults unhandled; staleness incorrectly ignored

- **Stop triggers on failure:** ORACLE_GAP; ORACLE_TAMPER; ORACLE_ENV_MISMATCH; ORACLE_FLAKE; EVIDENCE_MISSING; REPEATED_FAILURE

- **Relief valve:** Fix; if workflow requires governance semantic changes, route via GovernanceChangePortal (do not weaken harness expectations silently).

- **Routing authority boundary on block:** GovernanceChangePortal (semantic conflicts) / DecisionRecorded (human) for stop-trigger arbitration

- **Contract refs:** C-ARCH-1; C-CTX-1; C-VER-1; C-TB-4; C-SHIP-1; C-EVID-6

- **Spec refs:** SR-SPEC §3.1–§3.3 (loop/iteration/candidate lifecycle); SR-SPEC §2.3.3–§2.3.5 (runs/evidence/approvals/exceptions); SR-SPEC §1.12 (freeze/shippable)

- **Plan refs:** D-34..D-36

- **Notes:** Test harness may use a fixed test human identity; production releases still require real human actors with stable identities. | [vpos-note] uses STRICT-CORE + portal/freeze validation


## G-71: End-to-end failure/exception path transcript verified

- **Gate kind:** e2e_accept

- **Applies to:** D-35

- **Purpose/decision:** Accept failure-path harness only when integrity faults and waiver-eligible FAIL outcomes follow the required routes: StopTriggered→PAUSED, EvidenceMissingDetected blocks claims, WaiverCreated enables Verified-with-Exceptions only when allowed, and human DecisionRecorded/exception resolution is required to proceed.

- **Membranes enforced (SR-ETT):** Operational; Authority & Integrity; Accountability; Resource; Change; Isomorphic

- **Enforcement mechanism:** oracle_run (e2e failure harness); fault injection; transcript capture; projection assertions

- **Allowed actor kinds:** SYSTEM (harness) + HUMAN (simulated test identity) + ORACLE

- **Required refs (depends_on):** Candidate (rel=depends_on); OracleSuite suite:E2E-FAILURE (rel=depends_on); governing artifacts in force (rel=depends_on)

- **Required commitment objects:** StopTriggered; DecisionRecorded; (when applicable) WaiverCreated/WaiverActivated; EvidenceMissingDetected

- **Required evidence:** Evidence bundle containing: failure-case transcripts; stop-trigger assertions; waiver scope assertions; portal/decision traces

- **Verification profile / suite:** suite:E2E-FAILURE@v1

- **Success outputs:** EvidenceBundleRecorded + transcripts demonstrating correct failure routing and relief valves

- **Success state effect:** Confirms system cannot silently progress under integrity faults; validates exception/decision mechanisms

- **Failure conditions:** Integrity faults fail to halt; system progresses without decision; waiver used to bypass non-waivable integrity; missing evidence does not block

- **Stop triggers on failure:** ORACLE_GAP; ORACLE_TAMPER; ORACLE_ENV_MISMATCH; ORACLE_FLAKE; EVIDENCE_MISSING

- **Relief valve:** Fix; if exception semantics change is required, route via GovernanceChangePortal.

- **Routing authority boundary on block:** GovernanceChangePortal / DecisionRecorded (human)

- **Contract refs:** C-LOOP-3; C-DEC-1; C-EXC-1..C-EXC-5; C-EVID-6

- **Spec refs:** SR-SPEC §3.4 (stop triggers); SR-SPEC §2.3.5 (waivers); SR-SPEC §1.9 (evidence); SR-SPEC §1.11 (decisions)

- **Plan refs:** D-35

- **Notes:** This gate should include explicit negative tests proving that non-waivable conditions cannot be waived.


## G-80: Replayability proof verified (event stream → identical reconstruction)

- **Gate kind:** proof_accept

- **Applies to:** D-36

- **Purpose/decision:** Accept replayability deliverable only when replaying the same event stream reconstructs identical projections/checksums under declared environment constraints.

- **Membranes enforced (SR-ETT):** Accountability; Isomorphic; Operational; Architectural

- **Enforcement mechanism:** oracle_run (replayability suite); snapshot+replay checksum comparison; projection rebuild assertions

- **Allowed actor kinds:** SYSTEM

- **Required refs (depends_on):** Candidate (rel=depends_on); OracleSuite suite:REPLAYABILITY (rel=depends_on); governing artifacts in force (rel=depends_on)

- **Required commitment objects:** RunStarted + RunCompleted; EvidenceBundleRecorded; archived event stream snapshot used for replay proof

- **Required evidence:** Evidence bundle containing: event stream capture hash; replay logs; projection checksum comparisons; determinism assertions

- **Verification profile / suite:** suite:REPLAYABILITY@v1

- **Success outputs:** EvidenceBundleRecorded with PASS for replay proof; archived replay artifacts

- **Success state effect:** System may claim audit replayability for covered flows; supports release readiness

- **Failure conditions:** Replay produces different projection/checksum; missing events; non-deterministic ordering; evidence missing; staleness not handled

- **Stop triggers on failure:** EVIDENCE_MISSING; ORACLE_ENV_MISMATCH; ORACLE_FLAKE

- **Relief valve:** Fix determinism; if non-determinism is unavoidable, route governance change (do not relax replayability silently).

- **Routing authority boundary on block:** GovernanceChangePortal / DecisionRecorded (human)

- **Contract refs:** C-ARCH-3; C-EVT-1; C-EVT-3; C-EVID-6

- **Spec refs:** SR-SPEC §1.5 (event log source of truth); SR-SPEC §1.6–§1.8 (projections); SR-SPEC §3.3 (status computed from events)

- **Plan refs:** D-36

- **Notes:** Replay proof should be run on the same pinned oracle suite/environment to avoid false drift signals.


## G-90: Freeze baseline created (Release Approval + FreezeRecordCreated complete)

- **Gate kind:** release_freeze

- **Applies to:** D-34..D-36

- **Purpose/decision:** Permit baseline freezing / release readiness only when a Verified candidate has an explicit human Release Approval and a complete Freeze Record enumerating governed artifacts, evidence, and active exceptions, with no unresolved staleness.

- **Membranes enforced (SR-ETT):** Authority & Integrity; Accountability; Change; Operational; Isomorphic; Resource

- **Enforcement mechanism:** portal_action (ReleaseApprovalPortal); freeze submission; oracle validation of freeze/approval linkage; staleness check

- **Allowed actor kinds:** HUMAN (approval + freeze) + SYSTEM (validation/projection)

- **Required refs (depends_on):** Candidate (rel=releases/depends_on); EvidenceBundle(s) supporting verification (rel=supported_by or approved_by as appropriate); OracleSuite used for verification (rel=depends_on); governing artifacts enumerated in artifact_manifest (rel=depends_on); Active exceptions (Deviation/Deferral/Waiver) in scope (rel=depends_on)

- **Required commitment objects:** ApprovalRecorded (portal_id=ReleaseApprovalPortal, actor_kind=HUMAN, exceptions_acknowledged[] explicit); FreezeRecordCreated (actor_kind=HUMAN); Candidate Verified (Strict or With-Exceptions); no unresolved staleness markers affecting candidate/suite/governed artifacts

- **Required evidence:** Evidence bundle refs recorded in approval/freeze; oracle validation output for freeze record completeness + exception acknowledgement; retrievability check

- **Verification profile / suite:** suite:FREEZE-VALIDATION@v1

- **Success outputs:** ApprovalRecorded; FreezeRecordCreated; Candidate computed Shippable=true (subject to no unresolved staleness)

- **Success state effect:** Candidate may be treated as Shippable for the declared baseline_id; release baseline becomes binding snapshot

- **Failure conditions:** Candidate not Verified; approval missing or not human; approval missing required fields or exceptions_acknowledged[]; freeze record incomplete; freeze references evidence that is missing; approval does not acknowledge active exceptions; unresolved staleness present

- **Stop triggers on failure:** EVIDENCE_MISSING; ORACLE_GAP; ORACLE_TAMPER; ORACLE_ENV_MISMATCH; ORACLE_FLAKE

- **Relief valve:** Resolve staleness; fix missing evidence; create/resolve exceptions; defer release; do not ship without a valid freeze baseline.

- **Routing authority boundary on block:** ReleaseApprovalPortal (for approval) + GovernanceChangePortal (if rule/field conflicts) / DecisionRecorded (human) for arbitration

- **Contract refs:** C-SHIP-1; C-TB-1; C-TB-4; C-TB-6; C-VER-1; C-EVID-6; C-EVT-6

- **Spec refs:** SR-SPEC §1.12 (Freeze record + Shippable rules); SR-SPEC §1.13 (staleness); SR-SPEC §2.3.4 (approvals); SR-SPEC §2.3.6? (freeze API); SR-SPEC §3.3.1 (Shippable)

- **Plan refs:** D-34..D-36

- **Notes:** Freeze baseline is binding; require explicit exceptions acknowledgement even when empty. Never allow agents to perform approval/freeze actions. | [vpos-note] portal:ReleaseApprovalPortal

---

## 7. Plan→Workflow mapping (binding default)

This mapping assigns each SR‑PLAN deliverable `D-xx`:
- `workflow_phase`
- `start_gate_ids`
- `accept_gate_ids`
- required oracle suite / profile
- required portals (if any)

> This mapping is binding as the default execution plan. Deviations require either:
> - a recorded exception (waiver/deferral/deviation), or
> - an explicit governance change approval (depending on scope).

| deliverable_id   | title                                                                                   | workflow_phase            | work_unit_type                   | depends_on                               | start_gate_ids                     | accept_gate_ids   | required_oracle_suite_id                              | required_portals   |
|:-----------------|:----------------------------------------------------------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------------|:-----------------------------------|:------------------|:------------------------------------------------------|:-------------------|
| D-01             | Governance hygiene patchset for instance-1 build                                        | PH-0 Bootstrap            | deliverable.record.governance    |                                          | G-00                               | G-00              | suite:CTX-REFSET-VALIDATION@v1                        |                    |
| D-02             | Repository scaffold and workspace layout                                                | PH-0 Bootstrap            | deliverable.bootstrap.repo       |                                          | G-00                               | G-30              | profile:STRICT-CORE@v1                                |                    |
| D-03             | Continuous integration baseline (build/test/lint)                                       | PH-0 Bootstrap            | deliverable.bootstrap.repo       | D-02                                     | G-00;G-30                          | G-30              | profile:STRICT-CORE@v1                                |                    |
| D-04             | Local developer tooling (scripts, dev env bootstrap)                                    | PH-0 Bootstrap            | deliverable.bootstrap.repo       | D-02                                     | G-00;G-30                          | G-30              | profile:STRICT-CORE@v1                                |                    |
| D-05             | Domain model primitives and invariants                                                  | PH-1 Domain Core          | deliverable.domain_core          | D-02                                     | G-00;G-30                          | G-30              | profile:STRICT-CORE@v1                                |                    |
| D-06             | Deterministic state machines and transition validation                                  | PH-1 Domain Core          | deliverable.domain_core          | D-05                                     | G-00;G-30                          | G-30              | profile:STRICT-CORE@v1                                |                    |
| D-07             | Ports and boundary interfaces (domain → adapters)                                       | PH-1 Domain Core          | deliverable.domain_core          | D-05                                     | G-00;G-30                          | G-30              | profile:STRICT-CORE@v1                                |                    |
| D-08             | Context compilation rules (refs → deterministic context bundle)                         | PH-1 Domain Core          | deliverable.domain_core          | D-06, D-07                               | G-00;G-30                          | G-00;G-30         | suite:CTX-REFSET-VALIDATION@v1;profile:STRICT-CORE@v1 |                    |
| D-09             | Postgres schemas and migrations (event store, projections, graph)                       | PH-0 Bootstrap            | deliverable.domain_core          | D-02                                     | G-00;G-30                          | G-10              | suite:RUNTIME-SUBSTRATE@v1                            |                    |
| D-10             | EventStore adapter (append-only streams + concurrency control)                          | PH-2 Persistence+Evidence | deliverable.persistence_eventing | D-07, D-09                               | G-00;G-10;G-30                     | G-10              | suite:RUNTIME-SUBSTRATE@v1                            |                    |
| D-11             | Projection builder (read models from event streams)                                     | PH-2 Persistence+Evidence | deliverable.persistence_eventing | D-10, D-06                               | G-00;G-10;G-30                     | G-10              | suite:RUNTIME-SUBSTRATE@v1                            |                    |
| D-12             | Dependency graph projection + staleness traversal                                       | PH-2 Persistence+Evidence | deliverable.persistence_eventing | D-10, D-09, D-06                         | G-00;G-10;G-30                     | G-15              | suite:STALENESS-GRAPH@v1                              |                    |
| D-13             | Outbox publisher and NATS event publication                                             | PH-2 Persistence+Evidence | deliverable.persistence_eventing | D-10                                     | G-00;G-10                          | G-10              | suite:RUNTIME-SUBSTRATE@v1                            |                    |
| D-14             | Evidence store adapter (MinIO, content-addressed bundles)                               | PH-2 Persistence+Evidence | deliverable.evidence_subsystem   | D-07, D-02                               | G-00;G-30                          | G-20              | suite:EVIDENCE-INTEGRITY@v1                           |                    |
| D-15             | Evidence manifest v1 library + validation oracle                                        | PH-0 Bootstrap            | deliverable.evidence_subsystem   | D-02                                     | G-00;G-30                          | G-20              | suite:EVIDENCE-INTEGRITY@v1                           |                    |
| D-16             | Restricted evidence handling (Infisical + envelope keys)                                | PH-2 Persistence+Evidence | deliverable.evidence_subsystem   | D-14                                     | G-00;G-20                          | G-20              | suite:EVIDENCE-INTEGRITY@v1                           |                    |
| D-17             | API service scaffold (Axum) + OIDC auth (Zitadel)                                       | PH-0 Bootstrap            | deliverable.api_identity         | D-02                                     | G-00;G-30                          | G-30              | profile:STRICT-CORE@v1                                |                    |
| D-18             | Core API endpoints (loops, iterations, candidates, runs)                                | PH-4 API+Identity         | deliverable.api_identity         | D-17, D-10, D-06, D-11                   | G-00;G-10;G-30                     | G-30              | profile:STRICT-CORE@v1                                |                    |
| D-19             | Governance and portal-related API endpoints (approvals, freezes, exceptions, artifacts) | PH-4 API+Identity         | deliverable.api_identity         | D-18, D-12, D-14                         | G-00;G-15;G-20;G-30                | G-50              | suite:PORTAL-WORKFLOWS@v1                             |                    |
| D-20             | Evidence API (upload, retrieve, associate with runs/candidates)                         | PH-4 API+Identity         | deliverable.api_identity         | D-14, D-18                               | G-00;G-20;G-30                     | G-20              | suite:EVIDENCE-INTEGRITY@v1                           |                    |
| D-21             | NATS/JetStream messaging integration (contracts + reliability)                          | PH-5 Orchestration+UI+Ops | deliverable.orchestration        | D-13                                     | G-00;G-10                          | G-10              | suite:RUNTIME-SUBSTRATE@v1                            |                    |
| D-22             | Loop governor service (SYSTEM event emission + budgets)                                 | PH-5 Orchestration+UI+Ops | deliverable.orchestration        | D-21, D-18                               | G-00;G-10;G-30                     | G-40              | suite:STOP-TRIGGERS@v1                                |                    |
| D-23             | Reference worker bridge (IterationStarted → context → candidate proposal)               | PH-5 Orchestration+UI+Ops | deliverable.orchestration        | D-08, D-22, D-20                         | G-00;G-20;G-30;G-40                | G-10              | suite:RUNTIME-SUBSTRATE@v1                            |                    |
| D-24             | Oracle runner service (Podman + gVisor)                                                 | PH-3 Oracles              | deliverable.oracles_runtime      | D-02, D-14                               | G-00;G-20;G-30                     | G-20              | suite:EVIDENCE-INTEGRITY@v1                           |                    |
| D-25             | Core oracle suite implementation (build/unit/schema/lint)                               | PH-3 Oracles              | deliverable.oracles_runtime      | D-03, D-15                               | G-00;G-20;G-30                     | G-30              | profile:STRICT-CORE@v1                                |                    |
| D-26             | Integration/e2e oracle suite (DB/MinIO/NATS/API/UI)                                     | PH-5 Orchestration+UI+Ops | deliverable.oracles_runtime      | D-24, D-25, D-31, D-18, D-28             | G-00;G-20;G-30;G-60                | G-31              | profile:STRICT-FULL@v1                                |                    |
| D-27             | Oracle integrity checks (TAMPER/GAP/FLAKE/ENV_MISMATCH)                                 | PH-3 Oracles              | deliverable.oracles_runtime      | D-24, D-25                               | G-00;G-20;G-30                     | G-40              | suite:STOP-TRIGGERS@v1                                |                    |
| D-28             | UI scaffold (React) + OIDC login                                                        | PH-5 Orchestration+UI+Ops | deliverable.ui_portals           | D-02, D-17                               | G-00;G-30                          | G-30              | profile:STRICT-CORE@v1                                |                    |
| D-29             | Loop/iteration/candidate views + evidence viewer                                        | PH-5 Orchestration+UI+Ops | deliverable.ui_portals           | D-28, D-20, D-18                         | G-00;G-20;G-30                     | G-30              | profile:STRICT-CORE@v1                                |                    |
| D-30             | Portal workflows UI (approvals, exceptions, oracle suite changes)                       | PH-5 Orchestration+UI+Ops | deliverable.ui_portals           | D-29, D-19                               | G-00;G-30;G-50                     | G-50              | suite:PORTAL-WORKFLOWS@v1                             |                    |
| D-31             | Self-host deployment stack (compose/podman)                                             | PH-0 Bootstrap            | deliverable.ops_selfhost         | D-02                                     | G-00;G-30                          | G-60              | suite:SELFHOST-SMOKE@v1                               |                    |
| D-32             | Bootstrap/init scripts (DB, buckets, identity realm, secrets)                           | PH-5 Orchestration+UI+Ops | deliverable.ops_selfhost         | D-31, D-09, D-16                         | G-00;G-10;G-20;G-60                | G-60              | suite:SELFHOST-SMOKE@v1                               |                    |
| D-33             | Operational logging + minimal observability                                             | PH-5 Orchestration+UI+Ops | deliverable.ops_selfhost         | D-17, D-22, D-24                         | G-00;G-20;G-30;G-40                | G-60              | suite:SELFHOST-SMOKE@v1                               |                    |
| D-34             | End-to-end harness (happy path)                                                         | PH-6 E2E+Proof            | deliverable.proof_e2e            | D-22, D-23, D-24, D-25, D-19, D-30, D-31 | G-00;G-10;G-20;G-30;G-40;G-50;G-60 | G-70;G-90         | suite:E2E-HAPPY@v1;suite:FREEZE-VALIDATION@v1         |                    |
| D-35             | End-to-end harness (failure modes: integrity + exception)                               | PH-6 E2E+Proof            | deliverable.proof_e2e            | D-34, D-27                               | G-00;G-40;G-70;G-90                | G-71              | suite:E2E-FAILURE@v1                                  |                    |
| D-36             | Replayability demonstration (event stream → reconstructed state)                        | PH-6 E2E+Proof            | deliverable.proof_e2e            | D-10, D-11, D-34                         | G-00;G-10;G-70;G-90                | G-80              | suite:REPLAYABILITY@v1                                |                    |

---

## 8. Portal playbooks (first pass)

Portal playbooks describe the default human procedures for binding actions.  
They MUST remain consistent with SR‑CONTRACT/SR‑SPEC and MUST NOT introduce new binding semantics.

### 8.1 GovernanceChangePortal (playbook)

# GovernanceChangePortal — Portal Playbook (first pass)

> **Purpose:** Human-only binding boundary for **normative governance changes** (governed artifacts, binding meanings, and systemic policy changes).  
> **Derived from:** SR-CONTRACT portal minimums + SR-SPEC approval/registry mechanics + gate-routing in the instance‑1 Gate Registry.

## 1) Portal identification

- **portal_id:** `GovernanceChangePortal`
- **portal_kind:** `approval_portal` *(records `ApprovalRecorded` with `portal_id=GovernanceChangePortal`)*  
- **scope:** `instance` *(SOLVER‑Ralph instance‑1)*

## 2) Purpose and boundary

- **Purpose (one sentence):**  
  Approve or reject a **governance change packet** so that changes to governed artifacts and binding meanings are **durable, attributable, evidence‑bound, and replayable**.

- **Trust boundary being crossed (SR‑ETT membranes):**  
  **Change**; **Authority & Integrity**; **Accountability** *(secondary: Ontological; Isomorphic)*

- **What this portal MUST NOT do:**  
  - MUST NOT treat an agent narrative as sufficient basis for changing governed artifacts (“no approval by narrative”).  
  - MUST NOT mint “Verified/Approved/Shippable” state by itself (it only records approvals; status comes from the governed mechanics).  
  - MUST NOT introduce new binding semantics; if the requested change alters meaning beyond current SR‑CONTRACT/SR‑SPEC, route via **SR‑CHANGE**.

## 3) Allowed request types

This portal accepts **only** the following request types:

- [x] `approval_request` *(portal approval for a governance change packet)*
- [x] `governed_artifact_current_pointer_change` *(approve setting `is_current=true` for a governed artifact version)*
- [x] `governed_artifact_semantics_change` *(approve a new SR‑* artifact version that changes normative meaning)*
- [x] other: `governance_change_arbitration` *(approve a decision to route/sequence governance work without redefining semantics; still recorded as approval)*

Explicitly rejected at this portal:

- [ ] `freeze_request` *(belongs to ReleaseApprovalPortal / Freeze workflow)*
- [ ] `waiver_request` *(belongs to ExceptionApprovalPortal)*
- [ ] `decision_record` *(belongs to Decision recording flow)*

## 4) Actor rules

- **Allowed actor kinds:** `HUMAN only`
- **Identity requirements:** authenticated + authorized per SR‑SPEC authn/authz (OIDC); stable actor identity required.
- **Attribution policy:** portal approval MUST record (directly or by derivation): `actor_kind=HUMAN`, stable `actor_id`, timestamp.

## 5) Preconditions

The portal MUST reject the request unless all preconditions hold.

### 5.1 Required commitment objects present

- A **governance change packet** MUST be present and referenceable, consisting of at least:
  - the proposed governed artifact version(s) (content-hash pinned), and
  - a change rationale and impact summary (can be a document/candidate ref).
- If the request is “set is_current=true”:
  - the target governed artifact version MUST already be registered (or included with registration evidence).

### 5.2 Required refs present

The approval request MUST reference (directly or indirectly) the active:

- `SR-CONTRACT@…`
- `SR-SPEC@…`
- `SR-TYPES@…`
- `SR-ETT@…`
- `SR-PLAN instance-1@…` *(when change affects plan or plan execution)*

### 5.3 Evidence retrievability

- Evidence referenced by the request MUST be retrievable; **EVIDENCE_MISSING is non‑waivable**.

### 5.4 Staleness / coherence

- If the requested change depends on superseded governed artifacts, the request MUST either:
  - update to current governed artifacts, or
  - explicitly request to change the “current” pointer as part of the packet.
- Cross‑reference integrity MUST pass (no broken refs in the new artifact versions).

## 6) Inputs required at submission time

### 6.1 Request payload (minimum)

For an `approval_request` to GovernanceChangePortal:

- `portal_id = "GovernanceChangePortal"`
- `decision ∈ {approve, reject}`
- `subject_refs[]` including:
  - governed artifact ids + versions being approved (or current-pointer change target)
  - any affected directive/spec/contract artifacts
- `evidence_refs[]` including:
  - coherence / cross‑reference integrity evidence (or manual review packet pointers)
  - any required oracle outputs validating schema/frontmatter
- `exceptions_acknowledged[]` MUST be present *(explicit empty list allowed)*

### 6.2 Recommended supporting evidence

- Schema/frontmatter validation results for each artifact version
- Cross-reference integrity report
- Coherence/S11-style audit report (even if lightweight)

## 7) Procedure

1) **Intake + validation (SYSTEM):**  
   Validate payload shape, actor identity, required refs, and evidence retrievability.

2) **Change packet review (HUMAN):**  
   - Review what artifacts change, what semantics change, and what downstream impacts occur.
   - Confirm the change does not silently redefine binding meanings outside SR‑CHANGE.

3) **Decision (HUMAN):**  
   - Approve only if the packet is coherent, attributable, and properly evidenced.  
   - Otherwise reject with rationale (non‑binding “request changes” may be recorded separately as notes).

4) **Record binding outcome (SYSTEM emits from HUMAN action):**  
   - Emit `ApprovalRecorded` with `portal_id=GovernanceChangePortal`.

5) **Follow‑on actions (SYSTEM, if approved):**  
   - If the packet includes governed artifact registration: register versions (if not already).  
   - If approval includes `is_current=true` changes: update registry current pointer per SR‑SPEC constraints.  
   - Trigger staleness propagation if changing current governed artifacts affects downstream dependents.

## 8) Outputs (binding records emitted)

- **Primary record type:** `ApprovalRecorded`
- **Required fields:**  
  - `portal_id="GovernanceChangePortal"`  
  - `decision`  
  - `subject_refs[]`  
  - `evidence_refs[]`  
  - `exceptions_acknowledged[]` (explicit; may be empty)

- **Follow‑on events (as applicable):**  
  - `GovernedArtifactVersionRecorded` *(registry ingestion)*  
  - staleness events (`NodeMarkedStale`, etc.) when current pointers change

## 9) Failure handling and routing

- **If preconditions fail:** reject; requester must supply missing refs/evidence or correct schema.  
- **If evidence is missing/unfetchable:** treat as **non‑waivable**; block until resolved; incident handling may be required.  
- **If the request implies new binding semantics:** reject and require a governance change routed via SR‑CHANGE (contract/spec amendments).

## 10) Auditability

Store (as evidence/provenance):

- portal submission payload hash
- reviewer identity + timestamp
- links to reviewed evidence bundle(s) and change packet refs
- approval record id

Retention expectation: same as governed artifact + baseline audit retention (treat as long‑lived).

## 11) Cross-references

- **Gate routing (from Gate Registry):** routed-to by gates: `G-00, G-10, G-15, G-20, G-30, G-31, G-40, G-50, G-60, G-70, G-71, G-80, G-90`  
- **SR‑CONTRACT clauses:** `C-TB-4; C-TB-6; C-META-1..3; C-EXC-3; C-DEC-1 (when approval accompanies systemic stop arbitration)`  
- **SR‑SPEC sections:** `§2.3.4 (Approvals); §2.3.6 (Governed artifacts); §1.10 (Registry semantics); §1.13 (Staleness)`  
- **SR‑ETT membranes:** Change; Authority & Integrity; Accountability  
- **SR‑PLAN items:** governance-touch deliverables + portal/API/UI deliverables (notably D‑19, D‑30)

### 8.2 ReleaseApprovalPortal (playbook)

# ReleaseApprovalPortal — Portal Playbook (first pass)

> **Purpose:** Human-only binding boundary for release / baseline freeze progression.  
> Approves a **specific Candidate** for release, explicitly acknowledging active exceptions, enabling FreezeRecord creation.

## 1) Portal identification

- **portal_id:** `ReleaseApprovalPortal`
- **portal_kind:** `approval_portal` *(records `ApprovalRecorded` with `portal_id=ReleaseApprovalPortal`)*  
- **scope:** `instance` *(SOLVER‑Ralph instance‑1)*

## 2) Purpose and boundary

- **Purpose (one sentence):**  
  Approve (or reject) a Candidate for **baseline freezing / shippable progression** using evidence-bound verification results and explicit exception acknowledgement.

- **Trust boundary being crossed (SR‑ETT membranes):**  
  **Authority & Integrity**; **Accountability**; **Operational** *(secondary: Change; Isomorphic)*

- **What this portal MUST NOT do:**  
  - MUST NOT approve without evidence references (“no approval by narrative”).  
  - MUST NOT bypass integrity conditions (e.g., ORACLE_TAMPER, EVIDENCE_MISSING).  
  - MUST NOT approve a stale Candidate as shippable unless staleness is explicitly resolved per mechanics.

## 3) Allowed request types

- [x] `approval_request` *(release approval for candidate)*  
- [x] `freeze_request` *(create FreezeRecordCreated for an approved candidate; human-only)*  
- [ ] `waiver_request` *(ExceptionApprovalPortal)*  
- [ ] `governance_change_request` *(GovernanceChangePortal)*  
- [ ] `decision_record` *(DecisionRecorded flow)*

## 4) Actor rules

- **Allowed actor kinds:** `HUMAN only` *(for the portal action(s): Approval and Freeze)*  
- **Identity requirements:** stable, verifiable identity; authorized to approve releases.  
- **Attribution policy:** the approval/freeze MUST be attributable to the human actor id.

## 5) Preconditions

The portal MUST reject the request unless all preconditions hold.

### 5.1 Required commitment objects present

- `CandidateMaterialized` exists and Candidate identity is stable and referenceable.
- Candidate is **Verified (Strict)** or **Verified-with-Exceptions** (per Contract/SPEC).
- For Verified-with-Exceptions:
  - all required waivers are present and active and correctly scoped.

### 5.2 Required refs present

Approval request MUST include refs to:

- `Candidate` (subject)
- `EvidenceBundleRecorded` for the relevant verification Run(s) (STRICT‑CORE / STRICT‑FULL as applicable)
- Active governed artifacts in force (SR‑CONTRACT, SR‑SPEC, SR‑TYPES, SR‑DIRECTIVE, and the pinned oracle suite/profile identifiers)

### 5.3 Evidence integrity + retrievability

- Evidence must be retrievable; **EVIDENCE_MISSING is non‑waivable**.
- Required oracle suite hash must match the declared suite/profile.

### 5.4 Staleness constraints

- The Candidate MUST NOT have unresolved staleness on:
  - any `rel=depends_on` dependency, including governed artifacts, and
  - the oracle suite/profile used for verification.

If staleness exists: require resolution/re-verification before approval and freeze.

### 5.5 Exception acknowledgement constraint

- The Approval request MUST contain `exceptions_acknowledged[]` explicitly (including empty list).  
- If a subsequent `FreezeRecordCreated.active_exceptions[]` is non-empty, the approval’s acknowledgements MUST cover them.

## 6) Inputs required at submission time

### 6.1 Release approval request payload (minimum)

- `portal_id = "ReleaseApprovalPortal"`
- `decision ∈ {approve, reject}`
- `subject_refs[]` MUST include:
  - `candidate_id`
  - `run_id`(s) / `evidence_bundle_id`(s) used to compute verification
- `evidence_refs[]` MUST include:
  - evidence bundle(s) from the required verification suite(s)
  - any integrity-condition records (if present)
- `exceptions_acknowledged[]` MUST be present (explicit empty list allowed)

### 6.2 Freeze request payload (minimum)

- `baseline_id` (freeze id / baseline label)
- `candidate_id`
- `verification` (STRICT or WITH_EXCEPTIONS) + suite/profile reference
- `release_approval_id` (the approval just created)
- `artifact_manifest[]` (the governed artifact versions in force)
- `active_exceptions[]` (in scope)

## 7) Procedure

1) **Intake + validation (SYSTEM):**  
   Validate required refs, evidence retrievability, suite hash match, and staleness.

2) **Review (HUMAN):**  
   - Review verification evidence bundle(s).  
   - Review active exceptions and associated waivers/deviations/deferrals.  
   - Confirm `exceptions_acknowledged[]` matches what is active/in-scope.

3) **Decision (HUMAN):**  
   - Approve only if:
     - candidate verification is acceptable (strict or with-exceptions),
     - no non-waivable integrity faults exist,
     - no unresolved staleness exists,
     - exceptions are explicitly acknowledged.

4) **Record approval (SYSTEM emits from HUMAN action):**  
   Emit `ApprovalRecorded` with `portal_id=ReleaseApprovalPortal`.

5) **Freeze baseline (HUMAN → SYSTEM):**  
   If approval is `approve`, submit `FreezeRecordCreated` referencing the approval, pinned artifact manifest, and active exceptions.  
   System MUST enforce the exception‑acknowledgement constraint.

## 8) Outputs (binding records emitted)

- **Primary record types:**  
  - `ApprovalRecorded` *(release approval)*  
  - `FreezeRecordCreated` *(baseline snapshot)*

- **Required fields (ApprovalRecorded):**  
  - `portal_id="ReleaseApprovalPortal"`  
  - `decision`  
  - `subject_refs[]`  
  - `evidence_refs[]`  
  - `exceptions_acknowledged[]` (explicit)

- **Required fields (FreezeRecordCreated):**  
  - `baseline_id`, `candidate_id`, `verification`, `release_approval_id`  
  - `artifact_manifest[]`, `active_exceptions[]`

- **Follow-on events:** shippable computation is a projection; baseline becomes binding snapshot for replay/audit.

## 9) Failure handling and routing

- **If preconditions fail:** reject; do not freeze.  
- **If evidence is missing/unfetchable:** halt; treat as non-waivable; route to **GovernanceChangePortal** for policy/incident handling if needed.  
- **If integrity conditions detected:** halt; re-run verification or remediate; do not approve/freeze.  
- **If staleness present:** resolve staleness (re-verify/re-evaluate) before approval.

## 10) Auditability

Store:

- approval payload hash + identity + timestamp
- freeze payload hash + identity + timestamp
- evidence bundle ids reviewed
- exception set acknowledged (explicit list)
- resulting approval_id and freeze_id

Retention expectation: baseline-grade retention (treat as long‑lived).

## 11) Cross-references

- **Gate routing (from Gate Registry):** routed-to by gates: `G-90` (and indirectly by failures in upstream gates requiring arbitration)  
- **SR‑CONTRACT clauses:** `C-TB-4; C-TB-6; C-VER-1..3; C-SHIP-1; C-EXC-2; C-EVT-6; C-EVID-6`  
- **SR‑SPEC sections:** `§2.3.4 (Approvals); §2.3.8 (Freeze records + shippable endpoint); §1.12.3 (exception acknowledgement constraint); §1.12.4 (shippable rule); §1.13.5 (staleness + shippable gating)`  
- **SR‑ETT membranes:** Authority & Integrity; Accountability; Operational  
- **SR‑PLAN items:** D‑34..D‑36 (E2E + replay + freeze path); D‑19 (portal API); D‑30 (portal UI)

### 8.3 ExceptionApprovalPortal (playbook)

# ExceptionApprovalPortal — Portal Playbook (first pass)

> **Purpose:** Human-only boundary for creating and managing **exceptions as records** (deviations/deferrals/waivers).  
> This portal is the only allowed place to grant a waiver that enables **Verified-with-Exceptions**.

## 1) Portal identification

- **portal_id:** `ExceptionApprovalPortal`
- **portal_kind:** `exception_portal` *(records `WaiverCreated` / `DeviationCreated` / `DeferralCreated` etc.)*  
- **scope:** `instance` *(SOLVER‑Ralph instance‑1)*

## 2) Purpose and boundary

- **Purpose (one sentence):**  
  Create and approve **scoped, time-boxed exception records** so the system can proceed without silently rewriting governance.

- **Trust boundary being crossed (SR‑ETT membranes):**  
  **Authority & Integrity**; **Change**; **Accountability** *(secondary: Resource; Operational)*

- **What this portal MUST NOT do:**  
  - MUST NOT waive integrity conditions (ORACLE_TAMPER, EVIDENCE_MISSING, ORACLE_ENV_MISMATCH, etc.).  
  - MUST NOT grant unbounded scope waivers (“class-wide indefinite waivers”).  
  - MUST NOT implicitly change SR‑CONTRACT/SR‑SPEC meaning; if that’s required, route to GovernanceChangePortal.

## 3) Allowed request types

- [x] `waiver_request` *(gate/oracle waiver; FAIL outcomes only; scoped)*  
- [x] `deviation_request` *(exception from a requirement)*  
- [x] `deferral_request` *(postponement of a requirement/deliverable)*  
- [x] `exception_resolution_request` *(resolve/expire exception)*  
- [ ] `approval_request` *(ReleaseApprovalPortal / GovernanceChangePortal)*  
- [ ] `freeze_request` *(ReleaseApprovalPortal)*  
- [ ] `decision_record` *(DecisionRecorded flow)*

## 4) Actor rules

- **Allowed actor kinds:** `HUMAN only`
- **Identity requirements:** stable, verifiable identity; authorized to approve exceptions.
- **Attribution policy:** exception create/activate/resolve MUST be attributable to a human actor.

## 5) Preconditions

### 5.1 Global preconditions (all exception types)

- Request MUST reference:
  - the exact governed requirement/gate/oracle being excepted,
  - the scope (candidate/loop/baseline/time-boxed class),
  - risk + mitigation,
  - expiry/sunset or review date,
  - resolution criteria + owner.

- Evidence referenced MUST be retrievable (EVIDENCE_MISSING blocks).

### 5.2 Waiver-specific preconditions

- Waiver MUST apply only to an explicit **FAIL outcome** (not “missing evidence” or “integrity faults”).  
- Waiver MUST NOT be used to bypass any integrity condition.
- Scope MUST satisfy Contract waiver scope constraints (default per-candidate).

### 5.3 Deviation/deferral preconditions

- Must reference the specific requirement (Contract/SPEC/Directive/Plan item).  
- Must define scope and expiry/review and resolution.

## 6) Inputs required at submission time

### 6.1 Waiver request payload (minimum)

- references the specific gate/oracle and failure(s) being waived
- lists failure(s) waived (if multiple)
- includes scope + env constraints
- states risk/mitigation
- includes resolution criteria + owner
- includes expiry/review date
- evidence references supporting the FAIL outcome (run/evidence bundle ids)

### 6.2 Deviation/deferral payload (minimum)

- requirement reference(s)
- scope (deliverable/phase/loop/baseline)
- rationale + impact
- expiry/review date
- resolution plan

## 7) Procedure

1) **Intake + validation (SYSTEM):**
   - validate required fields
   - validate scope constraints
   - validate evidence retrievability

2) **Review (HUMAN):**
   - ensure the exception is necessary and appropriately scoped
   - ensure it does not waive integrity conditions
   - ensure expiry/review and resolution are credible

3) **Decision (HUMAN):**
   - approve or reject

4) **Record exception (SYSTEM emits from HUMAN action):**
   - Emit `WaiverCreated` / `DeviationCreated` / `DeferralCreated`
   - If immediately in force: emit `ExceptionActivated`

5) **Resolution (when applicable):**
   - Use resolve/expire paths to emit `ExceptionResolved` / `ExceptionExpired`

## 8) Outputs (binding records emitted)

- **Primary record types:** `WaiverCreated`, `DeviationCreated`, `DeferralCreated`, `ExceptionActivated`, `ExceptionResolved`, `ExceptionExpired`
- **Required fields (waiver):**
  - gate/oracle reference + waived failures
  - scope + env constraints
  - risk/mitigation + resolution criteria + owner
  - expiry or review date
  - approver identity + timestamp
- **Follow-on effects:**
  - Enables `Verified-with-Exceptions` only when waiver is active and in-scope.

## 9) Failure handling and routing

- **If preconditions fail:** reject; requester must correct fields/scope/evidence.
- **If evidence is missing/unfetchable:** block; treat as non-waivable; route to GovernanceChangePortal for incident/policy handling if needed.
- **If the request attempts to waive integrity conditions:** reject; record as a policy violation; consider GovernanceChangePortal if systemic.
- **If the exception implies governance semantics change:** route to GovernanceChangePortal.

## 10) Auditability

Store:

- exception request payload hash
- reviewer identity + timestamp
- evidence bundle ids reviewed
- resulting exception ids and their scope/expiry

Retention expectation: baseline-grade (exceptions must remain visible at approval/freeze).

## 11) Cross-references

- **Gate routing (from Gate Registry):** waiver routing referenced by gates: `G-30, G-31` (as `/exceptions/waivers`)  
- **SR‑CONTRACT clauses:** `C-EXC-1..5; C-EVID-6; C-TB-1; C-TB-6`  
- **SR‑SPEC sections:** `§2.3.5 (Exceptions); §1.14 (Gate Waiver scope/constraints); Appendix C (integrity conditions)`  
- **SR‑ETT membranes:** Authority & Integrity; Change; Accountability  
- **SR‑PLAN items:** deliverables that define exception + portal workflows (notably D‑19, D‑30; and any deliverable using WITH_EXCEPTIONS flows)

### 8.4 DecisionRecorded (playbook)

# DecisionRecorded — Decision Recording Playbook (first pass)

> **Purpose:** Provide a human-only, auditable mechanism to resolve **stop-the-line escalations** and other binding arbitration outcomes by emitting `DecisionRecorded`.

## 1) Portal identification

- **portal_id:** `DecisionRecorded` *(semantic label: “decision recording”; the binding artifact is the `DecisionRecorded` event)*  
- **portal_kind:** `decision_portal` *(records `DecisionRecorded`)*  
- **scope:** `instance` *(SOLVER‑Ralph instance‑1)*

## 2) Purpose and boundary

- **Purpose (one sentence):**  
  Record binding arbitration decisions that unblock (or terminate) work when automation cannot safely decide.

- **Trust boundary being crossed (SR‑ETT membranes):**  
  **Authority & Integrity**; **Operational**; **Accountability** *(secondary: Change; Resource)*

- **What this portal MUST NOT do:**  
  - MUST NOT “approve release” (ReleaseApprovalPortal does that).  
  - MUST NOT grant waivers (ExceptionApprovalPortal does that).  
  - MUST NOT silently weaken oracle integrity rules; systemic changes route to GovernanceChangePortal.

## 3) Allowed request types

- [x] `decision_record` *(resolve stop triggers / arbitration)*  
- [ ] `approval_request`  
- [ ] `waiver_request`  
- [ ] `freeze_request`  
- [ ] `governance_change_request`

## 4) Actor rules

- **Allowed actor kinds:** `HUMAN only`
- **Identity requirements:** stable, verifiable identity; authorized to arbitrate stop triggers for this instance.
- **Attribution policy:** decision must be attributable to the human actor id.

## 5) Preconditions

- A triggering condition exists (e.g., `StopTriggered` event, blocked gate, budget exhaustion, repeated failure threshold, integrity fault, staleness gating block).
- The decision request includes:
  - trigger code (e.g., `STOP_TRIGGER:REPEATED_FAILURE`, `STOP_TRIGGER:ORACLE_FLAKE`, etc.)
  - scope (loop_id, iteration_id; candidate_id if relevant)
  - subject_refs[] and evidence_refs[] adequate to justify the decision
- Evidence referenced must be retrievable (EVIDENCE_MISSING blocks).
- Decision MUST NOT claim to waive a non-waivable integrity condition.

## 6) Inputs required at submission time

Minimum payload for `POST /decisions`:

- `trigger` (string; include stop trigger id)
- `scope` (loop_id, iteration_id, candidate_id as applicable)
- `decision` (e.g., continue | pause | terminate | rerun_oracles | require_human_review | route_to_portal:<id>)
- `rationale` (plain text)
- `subject_refs[]` (what the decision is about)
- `evidence_refs[]` (evidence considered)
- `exceptions_acknowledged[]` (explicit; may be empty)
- optional: `is_precedent`, `applicability` (if the decision sets a reusable policy)

## 7) Procedure

1) **Intake + validation (SYSTEM):**
   - validate actor identity and payload shape
   - validate evidence retrievability

2) **Review (HUMAN):**
   - review evidence, history of attempts, stop trigger conditions
   - determine safest next action under a conservative posture

3) **Decision (HUMAN):**
   - choose an action; document rationale and scope
   - if decision implies governance change or semantics change → route to GovernanceChangePortal instead

4) **Record decision (SYSTEM emits from HUMAN action):**
   - emit `DecisionRecorded`
   - follow-on events may occur (LoopResumed, LoopClosed, etc.) according to the governor and SR‑SPEC state machine rules.

## 8) Outputs (binding records emitted)

- **Primary record type:** `DecisionRecorded`
- **Required fields (per SR‑SPEC):**
  - stable decision_id
  - trigger + scope
  - decision + rationale
  - subject_refs[] + evidence_refs[]
  - exceptions_acknowledged[] (explicit)

- **Follow-on events (examples):**  
  - `LoopResumed` / `LoopClosed`  
  - `ReEvaluationTriggered` (if staleness or re-verify chosen)

## 9) Failure handling and routing

- **If preconditions fail:** reject; require missing scope/evidence.
- **If evidence missing/unfetchable:** treat as non-waivable; halt and resolve evidence availability first.
- **If integrity conditions detected:** do not proceed via waiver; require remediation or governance-level incident handling.
- **If systemic change needed (stop trigger definitions, oracle policy weakening, semantics changes):** route to GovernanceChangePortal.

## 10) Auditability

Store:

- decision payload hash
- reviewer identity + timestamp
- evidence bundle ids reviewed
- resulting decision_id + scope

Retention expectation: baseline-grade (decisions justify continuation/termination and must be replayable).

## 11) Cross-references

- **Gate routing (from Gate Registry):** arbitration referenced by gates: `G-00, G-10, G-15, G-20, G-30, G-31, G-40, G-50, G-60, G-70, G-71, G-80, G-90`  
- **SR‑CONTRACT clauses:** `C-DEC-1; C-LOOP-1; C-LOOP-3; C-EVID-6`  
- **SR‑SPEC sections:** `§1.11.2 (DecisionRecorded event); §2.3.7 (Decisions API); stop trigger + loop lifecycle sections`  
- **SR‑ETT membranes:** Authority & Integrity; Operational; Accountability  
- **SR‑PLAN items:** gates and deliverables that exercise stop triggers and arbitration (notably D‑22, D‑27, D‑35)

---

## 9. Evidence and refs discipline

### 9.1 No ghost inputs

Iteration context MUST be derivable only from:
- `IterationStarted` payload, and
- `IterationStarted.refs[]` (typed refs)

Agent outputs that are not captured as proposal artifacts referenced from the event graph MUST NOT be treated as valid inputs.

### 9.2 depends_on vs supported_by

- Use `rel=depends_on` for inputs that create blocking requirements and staleness propagation.
- Use `rel=supported_by` for provenance that does not affect validity (audit/supporting evidence).

### 9.3 Evidence retrievability

If any referenced evidence is not retrievable (`EVIDENCE_MISSING`), the system MUST stop‑the‑line and route to the appropriate portal; this condition is non‑waivable.

---

## 10. Exceptions and waivers

- Exceptions are durable records (waiver/deviation/deferral) governed by SR‑SPEC and SR‑CONTRACT.
- Exceptions MUST be:
  - scoped,
  - time‑bounded or review‑bounded,
  - explicitly acknowledged at release approval, and
  - included in freeze records when active.

WITH_EXCEPTIONS verification mode is permitted only when:
- the relevant failures are waiver‑eligible, and
- the ExceptionApprovalPortal approves and records the waiver(s).

---

## 11. Conformance and self-verification

This directive is considered “ready for use” (still draft) when:

1) **Schema/frontmatter validation** passes for this document and all referenced governed artifacts.  
2) **Internal consistency checks** pass:
   - every deliverable in Plan→Workflow mapping references existing gate IDs
   - every gate references defined oracle suites/profiles
   - every routed portal exists in Portal Playbooks and/or SR‑SPEC portal mechanics
3) **Contract compliance checks** pass:
   - required portals exist (C‑TB‑4)
   - stop triggers include the contract minimums (C‑LOOP‑3)
   - human‑only binding actions are preserved (C‑TB‑1, C‑TB‑7)
4) **Governance-touch approval:** Setting `is_current=true` for this SR‑DIRECTIVE version MUST be approved via GovernanceChangePortal.

---

## Appendix A. Content hashes (informational)

- SR‑TYPES 3.3.0‑draft.8: `sha256:1d4fe783e4bf943913911d864ebe296246c87d154de6f44fb00ee6cfa2225147`
- SR‑CONTRACT 1.1.0‑draft.1: `sha256:0d72191fb36d375062eea7358b043dc9aa76ff93affc122803757775404aa20c`
- SR‑SPEC 1.3.0‑draft.1: `sha256:c14a45be626d931bb90bd0e8081a5d35a832e8ef7a52f361480d18b96c6b13df`
- SR‑PLAN‑instance‑1 1.0.0‑draft.3: `sha256:a88abdfb86192a42e76826ee076300ca211bbc82a8aa822fc286ae53651b5438`
- SR‑ETT 1.1.0‑draft.1: `sha256:17acacb67fa3186ad99f3e6d9db783f5e9af6a6795d3b5e9617db7e906b3f2b7`
- Plan→Workflow mapping CSV: `sha256:31f27217055a07264d78cb69d19eb53044d89bf7f9cd00f7bf9a90ad5ebf17e1`
- Gate Registry MD: `sha256:d64dba8f986390af18860c45bec53590b6a9322067889542c5fe09cce0d65344`
- Profile Definitions YAML: `sha256:2a56f930357e6740824ebd81338ce0b677e61b78884ac31ba470d3a3b3dfa9f2`


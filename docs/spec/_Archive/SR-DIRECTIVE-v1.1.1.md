---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-DIRECTIVE"
  type: "governance.dev_directive"
  title: "SOLVER-Ralph Development Directive"
  version: 1.1.1
  status: "draft"
  normative_status: "normative"
  authority_kind: "process"
  governed_by: ["SR-CHANGE"]
  supersedes: ["SR-DIRECTIVE@1.1.0"]
  created: "2026-01-09"
  updated: '2026-01-10'
  tags:
    - "solver-ralph"
    - "development-directive"
    - "governance"
    - "gates"
    - "phases"
    - "execution-plan"
    - "ralph-loop"
    - "verification-profiles"
    - "oracle-suites"
    - "trust-boundary"
    - "staleness"
    - "freeze"
  ext:
    dev_directive:
      phases: ["P0-Bootstrap", "P1-Core", "P2-Orchestration", "P3-Integration", "P4-SelfHost"]
      portals:
        - id: "GovernanceChangePortal"
          description: "Normative governance artifact changes (canonical set + governed config)"
        - id: "ReleaseApprovalPortal"
          description: "Candidate release approval (trust boundary for shipping)"
        - id: "OracleSuiteApprovalPortal"
          description: "Oracle suite registration / rebase / environment-policy changes"
        - id: "ExceptionApprovalPortal"
          description: "Deviation / Deferral / Waiver approval"
      stop_triggers:
        repeated_failure_n: 3
        mandatory:
          - "ORACLE_TAMPER"
          - "ORACLE_GAP"
          - "ORACLE_ENV_MISMATCH"
          - "ORACLE_FLAKE"
          - "REPEATED_FAILURE"
          - "BUDGET_EXHAUSTED"
        extended:
          - "SECURITY_CRITICAL"
          - "CONTRACT_VIOLATION"
      default_budgets:
        max_iterations: 10
        max_time_minutes: 120
        max_cost_usd: 50
      verification_profiles:
        - id: "STRICT-CORE"
          oracle_suite_id: "SR-SUITE-CORE"
          mode: "STRICT"
        - id: "STRICT-FULL"
          oracle_suite_id: "SR-SUITE-FULL"
          mode: "STRICT"
        - id: "WITH-EXCEPTIONS"
          oracle_suite_id: "SR-SUITE-FULL"
          mode: "WITH_EXCEPTIONS"
---

# SOLVER-Ralph Development Directive v1.1.1

**Purpose:** This directive defines **how SOLVER-Ralph is built and verified**: the implementation phases, the gates that control phase exit and release readiness, the verification profiles (oracle suites + modes), the stop-the-line triggers (including `REPEATED_FAILURE`), and the trust-boundary portals that require human approval.

**DO NOT CHANGE (guardrails):** No new Portals for plan approval; do not redefine **Verified**/**Approved** semantics; do not move agent orchestration into the hexagonal domain core.


## How to interpret this document

### Binding vs guidance

This directive is **mixed**:

- **Binding (MUST/SHALL):**
  - Gate definitions and gate-exit criteria
  - Verification profile semantics (STRICT vs WITH-EXCEPTIONS)
  - Stop-the-line trigger set and `REPEATED_FAILURE` threshold (N)
  - Portal identities, scopes, and minimum approval requirements
  - Default budget bounds (and how they may be changed)

- **Strong guidance (SHOULD):**
  - Phase boundaries and package sequencing
  - Suggested oracle groupings per profile
  - Recommended checklists for consistent execution

### Precedence and conflict handling

If this directive conflicts with the governed set, the binding precedence is:

1. **SR-CONTRACT** (Architectural Contract)
2. **SR-SPEC** (Technical Specification)
3. **SR-DIRECTIVE** (this document)
4. **SR-INTENT** (Design Intent)
5. **SR-README** (Project index)

If a conflict is discovered, **do not “override” by interpretation**; route it through **SR-CHANGE** (Change Management) as a governance change request.

### Canonical governed set

The canonical governed set (baseline documents) includes:

- `SR-TYPES` — artifact types and metadata rules
- `SR-README` — navigation/orientation (index)
- `SR-INTENT` — design intent (directional)
- `SR-CONTRACT` — architectural contract (normative)
- `SR-SPEC` — technical spec (normative)
- `SR-DIRECTIVE` — development directive (mixed)
- `SR-CHANGE` — change management (normative, process)

---

## 0. Version Changes


### Documentation is infrastructure (not an afterthought)

SOLVER-Ralph is designed for long-running work with stochastic agent workers. In this setting, documentation is not tertiary; it is **the substrate that stabilizes meaning**.

Accordingly:

- Any constraint, decision, exception, or interpretation that will be relied upon downstream **MUST** be captured as a governed artifact or a binding record and referenced explicitly (rather than living only in chat/session context).
- Iteration context and evaluation expectations **MUST** be reconstructable from referenced artifacts and recorded payloads; unreferenced transcript memory is non-authoritative.
- Teams **SHOULD** treat document updates (intent/type/contract/spec/directive/change/plan) as normal loop outputs when meaning would otherwise drift.

This rule exists to prevent false foundations: narrative plausibility must not become institutional fact without an attributable record.

### 1.1.1 (2026-01-10)

- Clarifies the central role of governed documentation as the meaning substrate for long-running, agentic development (constraints and interpretations must become attributable records, not chat memory).

### 1.0.2 (2026-01-10)

- Updates the Ralph Loop execution protocol to make `IterationStarted.refs[]` the authoritative iteration input set (no ghost inputs).
- Clarifies that only SYSTEM may emit `IterationStarted` and that iteration start is SYSTEM-mediated (per SR-SPEC).
- Adds explicit non-binding internal plan/semantic review discipline (recorded in summaries, not portals).
- Defines human intervention as an input record/config referenced in the next iteration (not a portal approval unless governance changes).

### 1.0.1 (2026-01-09)

- Aligns `governance.dev_directive` with the canonical presence of `governance.change_mgmt` (SR-CHANGE) and treats change control as first-class.
- Removes procedural “freeze” and “exception management” playbooks from this directive (those belong in SR-CHANGE and project initialization), while preserving **clear definitions** and **cross-artifact semantics** for future agent generation.
- Tightens portal usage: all trust-boundary approvals are mapped to the four contract-compatible portals; review topics are handled as **checklists**, not additional portals.
- Strengthens consistency with SR-SPEC by explicitly requiring:
  - governed artifact registry ingestion,
  - staleness marking and shippable gating,
  - freeze record creation semantics and exception acknowledgement coupling.

### 1.0.0 (2026-01-09)

- Initial directive defining phases P0–P4, stop trigger threshold `REPEATED_FAILURE` with N=3, and baseline verification profiles.

---

## 1. Phases Overview

SOLVER-Ralph is implemented in **five phases**. Each phase has:

- an **exit gate** (binding),
- a default **verification profile** for phase work (binding defaults; may be overridden by an explicit recorded decision),
- expected **deliverables** (strong guidance).

### 1.1 Phase summary

| Phase | Name | Primary goal | Default verification profile | Exit gate |
|:---:|---|---|---|---|
| P0 | Bootstrap | Establish governed set + minimal runnable harness | STRICT-CORE (bootstrap subset) | `G-P0-EXIT` |
| P1 | Core | Implement pure domain core (events, state machines, trigger eval, ports) | STRICT-CORE | `G-P1-EXIT` |
| P2 | Orchestration | Implement adapters + oracle runner + loop governor + evidence store | STRICT-CORE | `G-P2-EXIT` |
| P3 | Integration | Implement API/auth boundary + UI + operational integration | STRICT-FULL | `G-P3-EXIT` |
| P4 | Self-Host | Use SOLVER-Ralph to govern SOLVER-Ralph (dogfooding) | STRICT-FULL (or WITH-EXCEPTIONS for controlled acceptance) | `G-P4-EXIT` |

### 1.2 Phase P0: Bootstrap

**Goal:** Stand up the minimum governed set + bootstrap tooling required to begin governed implementation work.

**Deliverables (guidance):**
- Canonical governed artifacts exist with valid YAML frontmatter metadata.
- Repository structure exists for domain core + adapters per SR-SPEC.
- Initial oracle suites exist (at minimum: metadata validation + build/test for the chosen stack).

**Exit gate:** `G-P0-EXIT`.

### 1.3 Phase P1: Core

**Goal:** Implement the **pure domain core**: event types, state machines, verification computation, stop trigger evaluation, and port definitions.

**Deliverables (guidance):**
- Domain core crate compiles and is infrastructure-free.
- State machines and invariants have unit and property tests.
- Stop-the-line trigger evaluation is deterministic and fully covered by tests.
- Domain events cover governance records required by SR-SPEC (approvals, decisions, exceptions, governed artifacts, freeze records, staleness).

**Exit gate:** `G-P1-EXIT`.

### 1.4 Phase P2: Orchestration

**Goal:** Implement the backing runtime: event store adapter, evidence store adapter, oracle runner adapter, projection rebuild tooling, dependency graph/staleness plumbing, and loop governor.

**Definition (binding-in-context):** In P2, “correctness” is not “it ran once.” Correctness is “it ran, produced immutable evidence, and is replayable from events.”

**Exit gate:** `G-P2-EXIT`.

### 1.5 Phase P3: Integration

**Goal:** Implement end-to-end surface area:
- API boundary with authorization and trust-boundary enforcement
- UI shell sufficient for review/approval workflows
- Optional operator CLI and async execution integration

**Exit gate:** `G-P3-EXIT`.

### 1.6 Phase P4: Self-Host

**Goal:** Demonstrate self-hosting: SOLVER-Ralph can run a governed loop against itself, produce evidence, stop and escalate correctly, and ship a release baseline with freeze + shippable evaluation.

**Exit gate:** `G-P4-EXIT`.

---

## 2. Gates and Criteria

### 2.1 Gate model

A **gate** is a binding checkpoint that determines whether a phase is complete or whether a candidate may cross a trust boundary.

Each gate MUST be supported by a **gate packet**, which minimally includes:

- one or more `evidence.gate_packet` manifests (SR-SPEC evidence manifest v1),
- the corresponding content-addressed evidence blobs (logs/reports),
- references to the governed artifacts and exceptions that were in force for the run,
- any required portal approval records.

> Note: The canonical evidence manifest format and immutability rules are defined in SR-SPEC.

### 2.2 Gate packets: minimum contents

For each gate evaluation, the gate packet MUST contain:

1. **Candidate reference** (commit hash / content hash) and toolchain information.
2. **Oracle suite identity**: suite id + suite hash, plus environment fingerprint.
3. **All oracle results** required by the selected profile (PASS/FAIL recorded; no suppression).
4. **Integrity conditions list** (may be empty); if non-empty, triggers apply.
5. **Dependency refs**:
   - governed artifacts (id, version, content hash)
   - active exceptions (ids and statuses)
   - loop and iteration identifiers
6. **Reproducibility note**: how to re-run the suite (command or orchestrator action).

### 2.3 Stop-the-line triggers (binding)

Stop-the-line triggers halt autonomous progression. When a trigger fires:

- the system MUST record the trigger in the event stream,
- the loop MUST enter a paused/escalated state,
- resolution MUST require a human decision and (where applicable) portal approval.

#### 2.3.1 Mandatory triggers

The following triggers are mandatory:

- `ORACLE_TAMPER`
- `ORACLE_GAP`
- `ORACLE_ENV_MISMATCH`
- `ORACLE_FLAKE`
- `REPEATED_FAILURE` (N consecutive non-advancing iterations; N ≥ 3)
- `BUDGET_EXHAUSTED`

This directive sets: **`REPEATED_FAILURE` threshold N = 3**.

#### 2.3.2 Extended triggers

The following extended triggers are enabled:

- `SECURITY_CRITICAL`
- `CONTRACT_VIOLATION`

These are conservative escalations: they MAY be implemented as computed triggers over oracle results and/or domain validation outcomes.

#### 2.3.3 Trigger evaluation order

Triggers SHOULD be evaluated after each oracle run completes, in this order:

1. **Integrity conditions** (`ORACLE_TAMPER`, `ORACLE_GAP`, `ORACLE_ENV_MISMATCH`, `ORACLE_FLAKE`)
2. **Security conditions** (`SECURITY_CRITICAL`)
3. **Contract conditions** (`CONTRACT_VIOLATION`)
4. **Progress conditions** (`REPEATED_FAILURE`)
5. **Budget conditions** (`BUDGET_EXHAUSTED`)

If multiple triggers fire, all MUST be recorded; the highest-severity trigger determines the default escalation path.

#### 2.3.4 Definition: “advancing” vs “non-advancing” (for REPEATED_FAILURE)

An iteration is **advancing** if it improves the best-known candidate status within the loop by at least one of:

- resolving any previously-present integrity condition, OR
- reducing the number of failing REQUIRED oracles under the active suite/profile, OR
- converting at least one REQUIRED oracle from FAIL → PASS (even if others still fail).

If none of the above happens, the iteration is **non-advancing**.

`REPEATED_FAILURE` fires when there are **3 consecutive non-advancing iterations** under the same active suite/profile **without an explicit recorded decision to re-baseline expectations** (e.g., suite rebase via OracleSuiteApprovalPortal).

#### 2.3.5 Non-waivable integrity conditions

The following MUST NOT be treated as “acceptable via waiver”:

- `ORACLE_TAMPER`
- `ORACLE_ENV_MISMATCH`
- `ORACLE_GAP`

Resolution requires remediation and re-run, and/or (for env mismatch) an explicit suite rebase through `OracleSuiteApprovalPortal`.

`ORACLE_FLAKE` is not “acceptable”; it requires stabilization or reclassification (with explicit approval) because it undermines epistemic confidence.

### 2.4 Budgets (binding defaults)

Each Ralph Loop MUST run with explicit budgets. Defaults (may be overridden only by explicit human decision):

- `max_iterations`: **10**
- `max_time_minutes`: **120**
- `max_cost_usd`: **50**

`BUDGET_EXHAUSTED` fires when any configured budget is exceeded.

### 2.5 Verification profiles (binding semantics)

A verification profile defines:

- the **oracle suite id** to use, and
- the **verification mode**.

#### 2.5.1 Mode: STRICT

A candidate is **Verified** only if all REQUIRED oracles in the suite **PASS**, and no integrity conditions are present.

#### 2.5.2 Mode: WITH-EXCEPTIONS

A candidate may be treated as Verified-with-Exceptions only if:

- every failing REQUIRED oracle is covered by an **approved waiver** (exception record), and
- evidence includes all failing outputs (no suppression),
- integrity conditions are absent (see non-waivable integrity conditions above).

> Exception/waiver lifecycle and templates are governed by SR-CHANGE. This directive defines only the semantics needed to run gates consistently.

#### 2.5.3 Profile: STRICT-CORE (default for P1–P2)

- **Suite:** `SR-SUITE-CORE`
- **Mode:** STRICT
- **Minimum intended required oracle groups (policy):**
  - build + unit tests (Rust)
  - static analysis/lint (e.g., clippy)
  - architecture boundary lint (domain core must not import infrastructure)
  - metadata validation for governed artifacts

#### 2.5.4 Profile: STRICT-FULL (default for P3–P4)

- **Suite:** `SR-SUITE-FULL`
- **Mode:** STRICT
- **Minimum intended required oracle groups (policy):**
  - everything in STRICT-CORE, plus:
  - API contract tests
  - integration tests
  - end-to-end smoke tests
  - dependency vulnerability audit (critical/high = FAIL)
  - secrets scan (any leak = FAIL)

#### 2.5.5 Profile: WITH-EXCEPTIONS (release pressure / controlled risk)

- **Suite:** `SR-SUITE-FULL`
- **Mode:** WITH-EXCEPTIONS
- **Policy constraints:**
  - waivers SHOULD be per-candidate; broader scopes require explicit constraint and expiry
  - a release approval MUST explicitly acknowledge active exceptions
  - shippable status MUST still be blocked by unresolved staleness (SR-SPEC)

### 2.6 Trust-boundary portals (binding)

Portals are the **only** mechanism for creating binding approval state at trust boundaries. Portals are identified by `portal_id` strings and MUST be stable.

This directive defines four portals (minimum set + operational extensions) and intentionally avoids proliferating portal types. Additional review concerns (security, architecture, compliance) are handled as **review checklists** required by these portals.

#### 2.6.1 Recommended authorization roles/claims (policy)

To implement portal enforcement consistently, the API SHOULD require distinct role claims per portal:

- GovernanceChangePortal → `sr.portal.governance_change.approve`
- ReleaseApprovalPortal → `sr.portal.release.approve`
- OracleSuiteApprovalPortal → `sr.portal.oracle_suite.approve`
- ExceptionApprovalPortal → `sr.portal.exception.approve`

These claims MAY map to organizational roles; early-stage implementations may assign multiple claims to a single person, but approvals MUST always record stable human identity (SR-SPEC actor model).

#### 2.6.2 GovernanceChangePortal

**Purpose:** Approve changes to the governed set and governed configuration artifacts (including oracle suite definitions and this directive).

**When required (minimum):**
- Any change to canonical governed artifacts (SR-TYPES, SR-CONTRACT, SR-SPEC, SR-DIRECTIVE, SR-CHANGE, SR-INTENT, SR-README)
- Any change that alters gate semantics, trigger thresholds, portal IDs, or suite definitions

**Approval record MUST include:**
- portal_id = `GovernanceChangePortal`
- approver stable identity (human)
- artifact refs changed (ids + versions + hashes)
- rationale and risk summary
- references to evidence and/or decision records supporting the change

**Registry coupling (normative via SR-SPEC):**
- When a governed artifact version is registered as `is_current=true` and `normative_status=normative`, the registration MUST reference an approval from this portal.

#### 2.6.3 ReleaseApprovalPortal

**Purpose:** Approve a specific candidate release for shipping (trust boundary).

**When required:**
- Promoting a candidate to “Approved for release” in the release readiness state machine.

**Approval record MUST include:**
- portal_id = `ReleaseApprovalPortal`
- candidate_id and evidence bundle refs
- explicit acknowledgement of active exceptions (explicit empty list if none)
- any release-scoping constraints (deployment environment, time window, etc.)

#### 2.6.4 OracleSuiteApprovalPortal

**Purpose:** Approve oracle suite registration and “rebase” (suite hash/environment policy changes).

**When required:**
- Introducing a new suite id
- Changing required/advisory classification of an oracle
- Changing pinned environment (image digest, toolchain versions, network policy) such that env mismatch would otherwise occur

**Approval record MUST include:**
- portal_id = `OracleSuiteApprovalPortal`
- suite id + suite hash (before/after)
- rationale for the rebase and expected impact on verification comparability
- references to evidence demonstrating the suite is runnable and deterministic

#### 2.6.5 ExceptionApprovalPortal

**Purpose:** Approve exception records (deviations, deferrals, waivers).

**When required:**
- Creating or renewing a deviation/deferral/waiver that affects gate outcomes or release decisions

**Approval record MUST include:**
- portal_id = `ExceptionApprovalPortal`
- exception record id(s)
- scope and expiry/review constraints
- risk and mitigation summary

### 2.7 Phase exit gates (binding)

The phase exit gates below are binding. Each requires:

- an appropriate verification profile run,
- a complete gate packet,
- any required portal approvals.

#### 2.7.1 `G-P0-EXIT` (Bootstrap complete)

**Required:**
- Governance artifacts exist with valid metadata (SR-TYPES rules).
- `SR-SPEC` configuration artifact paths are established (directive location, suite location).
- `SR-SUITE-CORE` exists and is runnable (may be minimal, but must be pinned and hashable).
- Evidence: at least one successful STRICT-CORE run against a minimal build.

**Portal requirement:** none by default (initialization), unless a governance change is being made relative to a prior baseline.

#### 2.7.2 `G-P1-EXIT` (Domain core complete)

**Required:**
- Domain core compiles and is infrastructure-free.
- Domain state machines exist for: loop, iteration, candidate, run, verification status, stop triggers.
- Domain events and command validation include governance record semantics required by SR-SPEC:
  - approvals (portals),
  - decisions,
  - exceptions (waivers),
  - governed artifact versions,
  - freeze records,
  - staleness markers.
- Trigger evaluation includes all mandatory triggers and threshold N=3.
- Tests exist and pass for invariants and trigger behavior.

**Verification:** STRICT-CORE

**Portal requirement:** none for phase exit, but any changes to governed artifacts during P1 must be approved via GovernanceChangePortal.

#### 2.7.3 `G-P2-EXIT` (Orchestration runtime complete)

**Required:**
- Event store adapter persists and replays events as source-of-truth.
- Evidence store adapter writes immutable, content-addressed evidence bundles.
- Oracle runner adapter executes suites in pinned environments and records environment fingerprints.
- Loop governor enforces budgets and stop triggers, and records events consistently.
- Projection rebuild tooling exists and is replay-driven (no hidden source-of-truth).
- Dependency graph and staleness core is present (at least enough to block shippable when staleness is unresolved).

**Verification:** STRICT-CORE (plus required adapter/integration oracles included in SR-SUITE-CORE as it evolves)

**Portal requirement:** suite changes required to reach this gate must be approved via OracleSuiteApprovalPortal.

#### 2.7.4 `G-P3-EXIT` (Integration complete)

**Required:**
- API endpoints exist per SR-SPEC for:
  - runs and evidence retrieval,
  - approvals (portals),
  - exceptions (waivers),
  - governed artifact registration,
  - decisions,
  - freeze records and shippable computation,
  - staleness marking and dependents query.
- Trust boundary enforcement:
  - human-only endpoints reject non-human actors,
  - portal approvals require corresponding role/claim policy.
- Identity integration exists (OIDC) with stable actor identity.
- UI exists to view evidence, runs, candidates, approvals, exceptions, staleness, and freeze records.
- Security posture baseline: secrets scanning and dependency audit oracles are functional.

**Verification:** STRICT-FULL

**Portal requirement:** none for phase exit, but any release candidate requires ReleaseApprovalPortal (and, for Shippable, freeze record semantics).

#### 2.7.5 `G-P4-EXIT` (Self-host complete)

**Required demonstration scenario:**
- SOLVER-Ralph uses its own loop runner to produce a candidate that changes its own code or governed artifacts.
- The run produces evidence and satisfies STRICT-FULL (or WITH-EXCEPTIONS with explicit approved waivers).
- A stop-the-line event is handled at least once (simulated or real), and resolved via a recorded decision.
- A governed change is approved through GovernanceChangePortal and applied.
- A release approval is issued via ReleaseApprovalPortal for a candidate.
- A freeze record is created for that approved candidate, and **shippable computation returns true** with **no unresolved staleness markers** (per SR-SPEC).

**Verification:** STRICT-FULL (preferred)

**Portal requirement:** GovernanceChangePortal + ReleaseApprovalPortal must both be exercised (and ExceptionApprovalPortal if WITH-EXCEPTIONS is used).

---

## 3. Package Breakdown

Packages are suggested units of work. The sequencing is guidance, but package outputs MUST satisfy the binding gates.

### 3.1 Phase P0 packages (Bootstrap)

| Package ID | Name | Key inputs | Outputs | Acceptance (minimum) |
|---|---|---|---|---|
| P0-PKG-1 | Canonical governed set ready | SR-INTENT, SR-CONTRACT, SR-SPEC | Canonical docs with metadata | Metadata validator passes; canonical set is navigable |
| P0-PKG-2 | Oracle suites bootstrap | SR-SPEC oracle requirements | `SR-SUITE-CORE` (and optionally `SR-SUITE-BOOTSTRAP`) | Suite is pinned, hashable, runnable; produces evidence.gate_packet |
| P0-PKG-3 | Repo + CI scaffolding | SR-SPEC stack constraints | workspace layout, CI jobs | Build/test/lint in CI yields evidence artifacts |

### 3.2 Phase P1 packages (Core)

| Package ID | Name | Key inputs | Outputs | Acceptance (minimum) |
|---|---|---|---|---|
| P1-PKG-1 | Event model (full) | SR-SPEC events | domain event types + envelope | Compiles; serialization tests pass |
| P1-PKG-2 | State machines | SR-SPEC state machines | loop/iteration/candidate/run state machines | Unit + property tests pass |
| P1-PKG-3 | Verification computation | SR-CONTRACT verification rules; SR-SPEC | Verified/Approved/Shippable computation logic | Tests cover strict + with-exception semantics + staleness block |
| P1-PKG-4 | Stop triggers + budgets | SR-CONTRACT + this directive | trigger evaluation + budget enforcement | All mandatory triggers fire correctly in tests; N=3 |
| P1-PKG-5 | Ports (traits) | SR-SPEC ports | EventStore/EvidenceStore/OracleRunner/etc traits | Domain crate remains infra-free |
| P1-PKG-6 | Domain conformance harness | SR-CONTRACT conformance checklist | domain test harness + fixtures | `SR-SUITE-CORE` includes domain-level oracles |

### 3.3 Phase P2 packages (Orchestration)

| Package ID | Name | Key inputs | Outputs | Acceptance (minimum) |
|---|---|---|---|---|
| P2-PKG-1 | Event store adapter | SR-SPEC storage | Postgres adapter + migrations | Replay produces identical projections; append-only enforced |
| P2-PKG-2 | Evidence store adapter | SR-SPEC evidence | MinIO adapter (content-addressed) | Immutable write-once semantics verified |
| P2-PKG-3 | Oracle runner adapter | SR-SPEC oracle runtime | Podman+gVisor runner | Env fingerprint recorded; network policy enforced |
| P2-PKG-4 | Projections + rebuild tooling | SR-SPEC projections | rebuild procedure + graph tables | Rebuild works solely from events |
| P2-PKG-5 | Loop governor | SR-SPEC orchestration | loop service (sync first) | budgets + stop triggers enforced; evidence produced |
| P2-PKG-6 | Suite ingestion + tamper checks | SR-SPEC config | suite loader + suite hash pinning | ORACLE_ENV_MISMATCH + tamper detection works |
| P2-PKG-7 | Staleness core + dependency graph | SR-SPEC staleness | graph traversal + stale marker events | Can mark a root stale and list unresolved dependents |

### 3.4 Phase P3 packages (Integration)

| Package ID | Name | Key inputs | Outputs | Acceptance (minimum) |
|---|---|---|---|---|
| P3-PKG-1 | API boundary (full) | SR-SPEC API | Axum service + auth middleware | All SR-SPEC endpoints implemented; human-only enforced |
| P3-PKG-2 | UI shell | SR-SPEC UI goals | React UI for evidence + approvals | Review flows usable |
| P3-PKG-3 | Secrets + restricted evidence | SR-SPEC evidence secrets | Infisical integration + restricted bucket | Redaction path tested |
| P3-PKG-4 | Async execution (optional) | SR-SPEC NATS | NATS outbox + worker | Correctness without NATS preserved |
| P3-PKG-5 | Operator tooling (optional) | SR-SPEC ops | CLI/ops scripts | Can replay, rebuild, and inspect |

### 3.5 Phase P4 packages (Self-host)

| Package ID | Name | Key inputs | Outputs | Acceptance (minimum) |
|---|---|---|---|---|
| P4-PKG-1 | Dogfooding loop + freeze | P0–P3 outputs | self-governed run + approval + freeze | Shippable is true; portals exercised |
| P4-PKG-2 | Deployment baseline | SR-SPEC deployment | compose/helm + docs | deploy + run locally |
| P4-PKG-3 | Hardening suite (optional) | operational needs | additional suite(s) | performance/backup tests defined and runnable |

---

## 4. Dependency Graph

### 4.1 Phase dependencies (high level)

```
P0 (Bootstrap)
  └──► P1 (Core)
        └──► P2 (Orchestration)
              └──► P3 (Integration)
                    └──► P4 (Self-Host)
```

### 4.2 Package dependencies (illustrative, not exhaustive)

```
P0-PKG-1 ──► P0-PKG-2 ──► P0-PKG-3
                │
                ▼
           P1-PKG-1 ──► P1-PKG-2 ──► P1-PKG-3 ──► P1-PKG-4 ──► P1-PKG-6
                │                      │
                └────────► P1-PKG-5 ◄──┘
                                  │
                                  ▼
           P2-PKG-1 ──► P2-PKG-4 ──► P2-PKG-5
                │          │          │
                ▼          ▼          ▼
           P2-PKG-2     P2-PKG-3 ──► P2-PKG-6
                                  │
                                  ▼
                                P2-PKG-7 ──► P3-PKG-1 ──► P3-PKG-2
                                                  ├──► P3-PKG-3
                                                  ├──► P3-PKG-4
                                                  └──► P3-PKG-5
                                                          │
                                                          ▼
                                                      P4-PKG-1
```

### 4.3 Governed artifact dependencies (binding reference)

```
SR-TYPES (taxonomy + metadata rules)
    │
    ├──► SR-INTENT (rationale, directional)
    │         │
    │         ▼
    ├──► SR-CONTRACT (binding invariants)
    │         │
    │         ▼
    ├──► SR-SPEC (implementation requirements)
    │         │
    │         ▼
    ├──► SR-DIRECTIVE (how we build + verify)
    │
    └──► SR-CHANGE (how we change + freeze)
```

---

## 5. Execution Checklist

### 5.1 Ralph Loop execution protocol (recommended)

**Default scoping (high concurrency):** prefer **one Ralph Loop per work unit** (task/package/module) rather than one loop per entire project. This enables many independent, concurrent loops.

Each Iteration has a control-plane start (SYSTEM) and a work-plane execution (Agent Worker). The authoritative input set is the `IterationStarted.refs[]` list.

#### A) SYSTEM (Loop Governor) — start an iteration (control-plane)

1. **Assemble the Iteration Context Ref Set**
   - [ ] Determine the work unit / loop scope and confirm the target `loop_id`
   - [ ] Assemble `IterationStarted.refs[]` per SR-SPEC §3.2.1.1 (minimum required ref categories)
   - [ ] Determine the gating policy in force for this work unit (default per `work_unit`, with overrides if specified)
   - [ ] Include `config.gating_policy` in `IterationStarted.refs[]` as `rel=supported_by` (audit-only by default)
   - [ ] Evaluate required hooks for progression (plan review, evaluation on verification, assessment on validation, closeout) based on the gating policy
   - [ ] If hard-gated and the required human judgment record is missing, **do not start** a progressing iteration; request the appropriate record as an input to the next iteration (`record.evaluation_note` / `record.assessment_note`)
   - [ ] If soft-gated and missing, start the iteration but ensure the deficit is surfaced as an open risk and/or pending hook until satisfied
   - [ ] Ensure dereferenceable refs include `meta.content_hash` (and `meta.selector` when slicing is intended)
   - [ ] Ensure audit-only inputs (e.g., agent definitions) use `rel=supported_by`, not `rel=depends_on`

2. **Emit `IterationStarted` (SYSTEM-only)**
   - [ ] Emit `IterationStarted` with `actor_kind=SYSTEM`
   - [ ] Treat `IterationStarted.refs[]` as the authoritative “actual inputs” record for the Iteration
   - [ ] Publish the event for worker consumption

#### B) Agent Worker — execute the iteration (work-plane)

3. **Compile context deterministically (no ghost inputs)**
   - [ ] Dereference `IterationStarted.refs[]`
   - [ ] Compile the iteration `ContextBundle` deterministically from `IterationStarted` payload + dereferenced refs
   - [ ] **MUST NOT** use raw transcript memory or out-of-band documents as semantic inputs

4. **Optional internal plan discipline (non-binding)**
   - [ ] Worker MAY draft an internal plan
   - [ ] A reviewer (human or another agent) MAY provide critique as a **non-binding semantic review**
   - [ ] Record the exchange outcome in `IterationSummary.ext.non_binding_reviews[]`
   - [ ] This is NOT a portal approval and MUST NOT be treated as “Verified”

5. **Work execution**
   - [ ] Execute work within declared budgets
   - [ ] Record artifact changes (code + governance + config), maintaining metadata hygiene
   - [ ] Produce Candidate snapshot(s) with traceable identity (Git commit + sha256 digest)
   - [ ] Register Candidate(s) and request Runs (oracle suite execution) as required

6. **Verification + trigger evaluation**
   - [ ] Evaluate integrity conditions first
   - [ ] Evaluate stop-the-line triggers (security/contract)
   - [ ] Evaluate progress (repeated failure) and budgets
   - [ ] If any stop trigger fires, stop and escalate per policy

#### C) Human intervention and decisions

Human evaluation/assessment notes are **auditable inputs** but MUST NOT be carried forward as iteration memory by default; they should be referenced only when needed (typically the next iteration) via `IterationStarted.refs[]`.

7. **Escalation + decision (when required)**
   - [ ] Human reviews evidence
   - [ ] Record a Decision (with scope, rationale, and references)
   - [ ] If a portal action is required, submit to the correct portal

8. **Human intervention (any time, non-binding unless governance changes)**
   - [ ] If a human wants to intervene mid-stream (guidance, constraints, corrections), create a typed input record/config (e.g., `record.intervention_note` or a governed config artifact).
   - [ ] Include that input as a typed ref in the **next** `IterationStarted.refs[]`.
   - [ ] If the intervention changes governed requirements, route it through SR-CHANGE and the appropriate portal (governance change).

#### D) Iteration record

9. **Record completion**
   - [ ] Record a structured IterationSummary (actions/outcomes/next_steps/open_risks) per SR-SPEC §3.2.2
   - [ ] Do not embed raw transcripts; prefer typed summaries + referenced artifacts/evidence only

### 5.2 Portal submission checklist (recommended)

#### GovernanceChangePortal submission

- [ ] Summary of changes (what, why)
- [ ] References to affected governed artifacts (id/version/hash)
- [ ] Gate packet(s) demonstrating consistency (metadata validation, regression results)
- [ ] Risk analysis and rollback plan
- [ ] Decision record if change resolves an escalation

#### OracleSuiteApprovalPortal submission

- [ ] Suite id + suite hash (before/after)
- [ ] Environment policy changes (digests, tool versions, network policy)
- [ ] Evidence the suite runs deterministically and produces manifests
- [ ] Rationale explaining comparability impact

#### ExceptionApprovalPortal submission

- [ ] Exception record(s) (deviation/deferral/waiver)
- [ ] Scope and expiry/review constraints
- [ ] Evidence and risk mitigation notes

#### ReleaseApprovalPortal submission

- [ ] Candidate reference and the final evidence bundle refs
- [ ] Verification mode (STRICT or WITH-EXCEPTIONS)
- [ ] Explicit list of active exceptions acknowledged (explicit empty list if none)
- [ ] Intended release scope constraints (if any)
- [ ] Validation Evidence Bundle refs (the evidence being relied upon for closeout)
- [ ] Approval attestation that validation was assessed (e.g., `attestations.validation_assessed=true` in the approval payload)
- [ ] If required by gating policy, include (or separately reference) the human `record.assessment_note` as provenance (non-binding; not a portal)

### 5.3 Definitions for freeze, deviations, and waivers (intentionally minimal here)

This directive intentionally omits procedural guidance for:

- freeze creation and freeze policy
- deviation/deferral lifecycle management
- waiver renewal cadence and escalation details

Those belong in **SR-CHANGE** and (for project instances) are often generated at project initialization by coding agents.

However, to keep ontology and semantics consistent, see **Appendix A** for the definitional semantics used by this directive.

---

## Appendix A: Vocabulary and record semantics (for future agent generation)

This appendix is definitional (not a full playbook). It exists so future agents can generate consistent SR-CHANGE guidance and consistent record templates.

### A.1 Freeze record (record.freeze)

A **Freeze Record** is a binding snapshot that identifies:

- which governed artifact versions were in force,
- which exceptions (deviations/deferrals/waivers) were active and acknowledged,
- which evidence bundles support the baseline/release,
- the release approval that authorized the ship decision.

**Coupling constraint (normative via SR-SPEC):** A freeze record MUST NOT be accepted unless the referenced **Release Approval** explicitly acknowledges the active exceptions listed in the freeze record (explicit empty lists are required when none apply).

### A.2 Shippable determination (summary)

A candidate is **Shippable** if and only if:

- it is Verified (strict) or Verified-with-Exceptions (with approved waivers), AND
- it has a release approval, AND
- it has a complete freeze record, AND
- there are **no unresolved staleness markers** affecting the candidate, suite, or governed artifacts in the freeze manifest.

(Full normative definition is in SR-SPEC.)

### A.3 Deviation (record.deviation)

A **Deviation** is a binding operational exception that explicitly states:

- what governed requirement is not being complied with,
- why, the risk, and mitigation,
- what “resolution” means and who owns it.

Deviations are preferred over repeatedly rewriting governance for every mismatch (see SR-INTENT).

### A.4 Deferral (record.deferral)

A **Deferral** is a binding postponement of a requirement or deliverable. It MUST:

- reference the deferred requirement,
- specify the new target (date, milestone, or condition),
- specify review/expiry conditions.

### A.5 Waiver (record.waiver / exception record used for WITH-EXCEPTIONS)

A **Waiver** is an exception used specifically to treat a candidate as Verified-with-Exceptions when a REQUIRED oracle fails.

Key semantics:

- waivers MUST be explicit records approved via ExceptionApprovalPortal,
- waivers SHOULD be per-candidate; broader scope requires explicit constraint and expiry,
- waivers MUST NOT be used to bypass integrity conditions (`ORACLE_TAMPER`, `ORACLE_ENV_MISMATCH`, `ORACLE_GAP`).

### A.6 Decision record (record.decision)

A **Decision Record** is required whenever a human resolves a stop-the-line escalation. It MUST include:

- stable id,
- trigger context and scope,
- decision and rationale,
- explicit acknowledgement of active exceptions (explicit empty list if none),
- references to affected governed artifacts and any related exception records.

---

## Appendix B: Practical note on “reviews” vs “portals”

Security review, architecture review, and operational review are **not separate portals** in this directive.

Instead, they are review topics that MUST be addressed as part of:

- GovernanceChangePortal approvals (for changes to governed meaning), and/or
- ReleaseApprovalPortal approvals (for shipping decisions), and/or
- OracleSuiteApprovalPortal approvals (for changes to verification environments).

**Plan review is different:** internal plan/semantic reviews are a useful discipline, but they are **non-binding** and MUST NOT be modeled as “approvals” or as new portals. Record them (if desired) as `IterationSummary.ext.non_binding_reviews[]` or as referenced artifacts.

This preserves a small, auditable set of trust-boundary mechanisms while still enforcing scrutiny where it matters.


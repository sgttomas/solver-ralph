---
doc_id: SR-CHANGE
doc_kind: governance.change_mgmt
layer: build
status: draft
refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: depends_on
    to: SR-TYPES
  - rel: depends_on
    to: SR-CONTRACT
---

# SR-CHANGE

Normative change-control policy for SOLVER-Ralph.

This document defines:

- **What requires formal change control**
- **How a change is proposed, verified, approved, and made "current"**
- **How portal routing works** (including how an instance may *alias portal functions* into a smaller set of portal IDs)
- **How exceptions, oracle-suite changes, and freeze baselines are governed**

SR-CHANGE is itself governed. Changes to SR-CHANGE MUST follow SR-CHANGE.

---

## 0. Version changes

### 0.1  (this draft)

- **SR-ETT eliminated:** Removed SR-ETT from refs and canonical governed set. SR-ETT content has been absorbed into SR-INTENT (rationale), SR-CONTRACT (invariants), and SR-SPEC (mechanics).
- **SR-PARADIGM → SR-GUIDE:** Renamed in governed set. SR-PARADIGM is now SR-GUIDE (usage guidance document).
- **SR-README removed:** SR-README was a narrative overview; its role is subsumed by SR-GUIDE.

- **Compatibility note:** SR-TYPES may still include a `governance.readme` *template/type* for `README.md` as an **index** artifact. This does **not** imply an SR-README governed artifact exists in the canonical set unless explicitly reintroduced via SR-CHANGE. For this corpus: treat `README.md` as `normative_status: index` and treat SR-GUIDE as the directional usage guide.
- **SR-AGENTS added:** Added to canonical governed set (agent semantics are now explicitly governed).
- Updated routing table to reflect current canonical set.
- Updated type key from `governance.change_policy` to `governance.change_mgmt` per SR-TYPES.

### 0.2  (2026-01-11)

- Reframed the required portal list as **portal functions** + **instance portal IDs**, enabling **portal-function aliasing**.
- Expanded the canonical governed set to include SR-PARADIGM and SR-ETT.
- Completed the non-waivable integrity condition set to include `ORACLE_FLAKE` and `EVIDENCE_MISSING`.
- Added explicit routing guidance for phase gates and budget extensions.

### 0.3  (2026-01-15)

- **SR-SPEC §1.5.2:** Added `WORK_SURFACE` and `INTAKE` to StreamKind enum to reflect Phase 4a implementation.
- **SR-SPEC §1.3.3:** Fixed malformed section (removed orphaned Work Surface events that corrupted Candidate identity text).
- **SR-SPEC Appendix A:** Added Work Surface events (`WorkSurfaceBound`, `StageEntered`, `StageCompleted`, `WorkSurfaceCompleted`, `WorkSurfaceArchived`) and Intake events (`IntakeCreated`, `IntakeUpdated`, `IntakeActivated`, `IntakeArchived`, `IntakeForked`) to canonical event registry.
- **SR-TYPES §7.7:** Expanded Work Surface schema with full field definitions including `WorkSurfaceId` (format: `ws:<ULID>`), `WorkSurfaceStatus` enum, `StageCompletionStatus` enum, and `StageStatusRecord` structure.
- **Classification:** G:MINOR (additive; backward-compatible; existing baselines remain interpretable).

### 0.4  (2026-01-15)

- **SR-PROCEDURE-KIT §1:** Added `requires_approval` field to stage schema alignment table. This boolean field indicates whether a stage represents a trust boundary requiring HUMAN approval via portal before transition. Default is `false`.
- **SR-PROCEDURE-KIT §2:** Updated baseline template GENERIC-KNOWLEDGE-WORK to include `requires_approval` for all stages:
  - `stage:FRAME` — `false`
  - `stage:OPTIONS` — `false`
  - `stage:DRAFT` — `false`
  - `stage:SEMANTIC_EVAL` — `true` (trust boundary: human must verify oracle results)
  - `stage:FINAL` — `true` (trust boundary: human must approve final candidate)
- **Rationale:** Enables enforcement of SR-CONTRACT C-TB-3 (portal crossings produce approvals) at stage gates. Stage completion for approval-required stages MUST be preceded by a recorded approval at the appropriate portal (e.g., `portal:stage-gate:<stage_id>`).
- **Classification:** G:MINOR (additive; backward-compatible; existing procedure templates without `requires_approval` default to `false`).

### 1.3  (2026-01-17)

- **SR-SPEC §2.3.1:** Added `PATCH /loops/{loop_id}` endpoint documentation:
  - Budget monotonicity constraint (normative): values can only increase
  - State constraint: Loop MUST NOT be CLOSED
- **SR-SPEC Appendix A:** Added `LoopUpdated` to canonical event registry with payload schema
- **Classification:** G:MINOR (documentation only; implementation already in 1.2).

### 1.2  (2026-01-17)

- **Loop PATCH Endpoint (V10-5):** Added `PATCH /api/v1/loops/:loop_id` for updating Loop goal and budgets:
  - Budget monotonicity enforced: new values must be >= current values
  - `LoopUpdated` event emitted on successful update
  - Projection handler updates `budgets` and/or `goal` in `proj.loops`
- **OracleSuite Hash Prefix Fix (V10-6):** Fixed doubled `sha256:sha256:` prefix in OracleSuite content_hash:
  - Root cause: `ContentHash::new()` was called with already-prefixed values
  - Fixed in `crates/sr-api/src/handlers/work_surfaces.rs` and test fixtures
- **Files modified:**
  - `crates/sr-api/src/handlers/loops.rs` — Added `PatchLoopRequest`, `LoopBudgetsPatch`, `patch_loop` handler
  - `crates/sr-api/src/main.rs` — Added PATCH route for loops
  - `crates/sr-adapters/src/projections.rs` — Added `apply_loop_updated` handler
  - `crates/sr-api/src/handlers/work_surfaces.rs` — Fixed hash prefix
  - `crates/sr-domain/src/work_surface.rs` — Fixed test fixtures
- **Verification:** Tests 8, 10 from SR-PLAN-LOOPS now passing. V10 complete.
- **Classification:** G:MINOR (additive; backward-compatible).

### 1.1  (2026-01-17)

- **Loop Governor Stop Triggers (V10-1 through V10-4):** Implemented SR-SPEC §3.4 stop trigger evaluation:
  - `StopTriggered` event emission for `BUDGET_EXHAUSTED` and `REPEATED_FAILURE`
  - Decision-required resume: `resume_loop()` validates `decision_id` when `requires_decision=true`
  - Loop state transitions to `PAUSED` with `last_stop_trigger` tracking
  - `consecutive_failures` counter in projection (resets on SUCCESS, increments on FAILURE)
- **Migration 009:** Added `proj.loops` columns:
  - `consecutive_failures` (INTEGER DEFAULT 0)
  - `last_stop_trigger` (TEXT)
  - `paused_at` (TIMESTAMPTZ)
  - `requires_decision` (BOOLEAN DEFAULT false)
  - Added index `idx_candidates_produced_by_iteration` on `proj.candidates`
- **Iteration Context (V10-4):** Loop ref added to `IterationStarted.refs[]` with `rel="in_scope_of"`
- **Verification:** Tests 9, 12-15 from SR-PLAN-LOOPS re-run and verified passing
- **Files modified:**
  - `crates/sr-adapters/src/projections.rs` — StopTriggered projection, consecutive_failures tracking
  - `crates/sr-api/src/handlers/loops.rs` — Decision validation in resume_loop()
  - `crates/sr-api/src/handlers/work_surfaces.rs` — Budget enforcement, stop trigger emission, Loop ref in refs[]
  - `migrations/009_loop_stop_triggers.sql` — Schema additions
- **Rationale:** Implements SR-PLAN-V10 Phases V10-1 through V10-4. Completes D-22 (Loop Governor) stop trigger enforcement per SR-SPEC §3.4 and C-LOOP-1/C-LOOP-3.
- **Classification:** G:MINOR (additive; backward-compatible; new projection columns default to safe values; existing loops continue to function).

### 1.0  (2026-01-16)

- **SR-REPLAY-PROOF:** Created new canonical artifact `docs/platform/SR-REPLAY-PROOF.md`:
  - Formal proof of deterministic replay per SR-CONTRACT C-EVT-7
  - Documents `compute_state_hash()`, `verify_replay()`, `find_discrepancies()` verification methods
  - Proves EventManager rebuild produces identical state from same event sequence
  - Covers state hash equality, eligible set equality, status projection equality, no ghost inputs (C-CTX-2)
- **SR-EVENT-MANAGER §3.1:** Added "Verification methods" section documenting:
  - `compute_state_hash()` — deterministic SHA-256 hash of projection state
  - `verify_replay(events)` — replays events on fresh instance and compares state hashes
  - `find_discrepancies(other)` — field-level comparison between EventManager instances
  - Added `verified_by: SR-REPLAY-PROOF` to document refs
- **sr-adapters replay module:** Added `crates/sr-adapters/src/replay.rs` with:
  - `ReplayProof` — proof artifact capturing hashes, discrepancies, event count
  - `ReplayDiscrepancy` — field-level comparison result
  - `EligibleSetComparison` — eligible set comparison between original and replayed state
- **Integration tests:** Added `crates/sr-api/tests/integration/replay_determinism_test.rs` with 7 tests validating determinism
- **Rationale:** Documents SR-PLAN-V9 Phase V9-3 implementation of Replayability Demonstration (D-36). Proves compliance with C-EVT-7 ("Projections MUST be rebuildable from the event log") and C-CTX-2 (no ghost inputs).
- **Classification:** G:MINOR (additive; backward-compatible; adds new canonical artifact SR-REPLAY-PROOF; documents verification methods that strengthen existing replay guarantees).

### 0.9  (2026-01-16)

- **oracle-suites/semantic-v1:** Created Semantic Oracle Suite container implementation:
  - `Dockerfile` — Multi-stage build: compile sr-oracles, copy to slim Debian runtime
  - `suite.json` — Suite definition with `semantic_set_binding` per SR-SEMANTIC-ORACLE-SPEC §2
  - `oracles/semantic-eval.sh` — Semantic evaluation oracle script
  - `semantic-sets/intake-admissibility.json` — Bundled semantic set definition (6 axes)
  - `README.md` — Suite documentation
- **sr-oracles CLI:** Added `semantic-eval` command:
  - Evaluates intake YAML files against intake admissibility semantic set
  - Produces 4 output artifacts: eval.json, residual.json, coverage.json, violations.json
  - Reuses existing `IntakeAdmissibilityRunner` from sr-adapters
- **SR-SPEC §1.9.1.2:** Added `sr.semantic_eval.v1` schema documentation:
  - Semantic evaluation result schema (distinct from `sr.oracle_result.v1`)
  - Required fields: candidate_id, semantic_set, metrics (residual/coverage/violations), decision
  - Supporting artifact schemas: ResidualReport, CoverageReport, ViolationsReport
- **SR-PLAN-V8:** Marked complete with completion summary (Appendix E):
  - All 5 phases (V8-1 through V8-5) complete
  - Actual effort: ~10 sessions (estimated 7-10)
  - Deliverables D-24, D-25, D-27, D-39 complete
- **SR-PLAN-GAP-ANALYSIS:** Updated for V8 completion:
  - V8 phases marked complete
  - D-24, D-25, D-27, D-39 status updated to complete
  - Branch 0 acceptance criteria updated (Evidence Bundle now complete)
  - Critical path updated (V8 complete, V9 next)
  - Milestone 1 completion updated to ~95%
- **sr-adapters tests:** Added 7 V8-5 integration tests validating semantic suite.json parsing
- **Environment constraints:** Suite declares `runsc` runtime, `network:disabled`, `workspace_readonly:true`
- **Contract compliance:** C-OR-1, C-OR-2, C-OR-3, C-OR-4, C-EVID-1 satisfied
- **Rationale:** Completes SR-PLAN-V8 Phase V8-5 per Amendment A-3 (uses existing type definitions). SR-PLAN-V8 is now **COMPLETE** — all 5 phases implemented, tested, and documented.
- **Classification:** G:MINOR (additive; backward-compatible; creates new directory `oracle-suites/semantic-v1/`; updates plan and spec documentation).

### 0.8  (2026-01-16)

- **oracle-suites/core-v1:** Created Core Oracle Suite container implementation:
  - `Dockerfile` — Container image with Rust 1.83 toolchain, Node.js, jq
  - `suite.json` — Suite definition manifest with environment constraints
  - `oracles/build.sh` — Build verification oracle (Rust/Node.js/Make detection)
  - `oracles/unit-tests.sh` — Unit test oracle with pass/fail parsing
  - `oracles/schema-validation.sh` — JSON schema validation oracle
  - `oracles/lint.sh` — Code linting oracle (advisory classification)
  - `README.md` — Suite documentation
- **SR-SPEC §1.9.1.1:** Added `sr.oracle_result.v1` schema documentation:
  - Individual oracle output format (distinct from `evidence.gate_packet` manifest)
  - Required fields: `schema`, `oracle_id`, `status`, `started_at`, `completed_at`, `duration_ms`, `exit_code`, `summary`
  - Optional fields: `details`, `artifacts`
  - Relationship to Evidence Bundle aggregation
- **Environment constraints:** Suite declares `runsc` runtime, `network:disabled`, `workspace_readonly:true`
- **Classification matrix:** 3 required oracles (build, unit-tests, schema-validation) + 1 advisory (lint)
- **sr-adapters tests:** Added 6 integration tests validating suite.json parsing and GAP detection
- **Rationale:** Documents SR-PLAN-V8 Phase V8-4 implementation of Core Oracle Suite Container. Oracles execute inside Podman containers with gVisor sandboxing, producing structured evidence per C-EVID-1.
- **Classification:** G:MINOR (additive; backward-compatible; creates new directory `oracle-suites/`; no changes to existing contracts).

### 0.7  (2026-01-16)

- **SR-SPEC Appendix A:** Added `IntegrityViolationDetected` event to canonical event registry with:
  - Event type, stream kind (`RUN`), stream id format (`run:{run_id}`)
  - Payload schema (v1) including `condition` object with `condition_type` discriminator
  - Documentation of four condition types: `ORACLE_TAMPER` (C-OR-2), `ORACLE_ENV_MISMATCH` (C-OR-3), `ORACLE_GAP` (C-OR-4), `ORACLE_FLAKE` (C-OR-5)
  - Note clarifying C-OR-7 escalation requirements
- **Rationale:** Documents SR-PLAN-V8 Phase V8-3 implementation of Oracle Integrity Condition Detection. The `IntegrityViolationDetected` event is emitted per C-OR-7 when any integrity condition is detected during oracle execution. All integrity conditions are blocking and require escalation.
- **Classification:** G:MINOR (additive; backward-compatible; existing baselines remain interpretable; strengthens integrity enforcement without changing verification semantics).

### 0.6  (2026-01-16)

- **SR-SPEC §1.5.2:** Added `ATTACHMENT` to StreamKind enum for human-uploaded supporting files.
- **SR-SPEC Appendix A:** Added `AttachmentRecorded` event to canonical event registry with:
  - Event type, stream kind (`ATTACHMENT`), stream id format (`attach:{attachment_id}`)
  - Payload schema (v1) including `attachment_id`, `content_hash`, `media_type`, `size_bytes`, `filename`
  - Clarification note distinguishing attachments from evidence bundles
- **SR-TYPES §4.4:** Added `record.attachment` to Operational Record Types table:
  - Authority Kind: `record`
  - Normative Status: `record`
  - Purpose: "Human-uploaded supporting file (does NOT satisfy C-VER-1)"
- **SR-CONTRACT §7 (C-EVID-2):** Added clarification subsection "C-EVID-2 Clarification: Attachments vs Evidence Bundles" documenting:
  - Attachments share C-EVID-2 immutable, content-addressed storage semantics
  - Attachments do NOT satisfy C-VER-1 verification requirements
  - Attachments use separate `attachments` bucket and `artifact_type: record.attachment`
  - Attachments serve as supporting context only, not verification authority
- **Rationale:** Documents SR-PLAN-V7 Phase V7-3 implementation of attachment upload endpoint. Attachments enable humans to upload supporting files (PDFs, images, documents) to Work Surfaces while maintaining clear ontological separation from oracle-produced Evidence Bundles.
- **Classification:** G:MINOR (additive; backward-compatible; existing baselines remain interpretable; no changes to verification semantics).

### 0.5  (2026-01-16)

- **SR-SPEC §2.3.1:** Added `work_surface_id` to Loop creation endpoint documentation:
  - When `work_unit` is explicitly provided, the system validates an active Work Surface exists and returns HTTP 412 `WORK_SURFACE_MISSING` if not found.
  - `LoopCreated` event payload and API response include `work_surface_id` when bound.
  - Documented iteration context inheritance from Loop's `work_unit` when Loop has bound Work Surface.
- **SR-SPEC Appendix A:** Added `LoopCreated` payload schema (v1) with `work_surface_id` field.
- **SR-TYPES §7.1:** Added `work_unit` and `work_surface_id` fields to Work Unit (Loop) key fields:
  - `work_unit` — work unit identifier (defaults to `id` if not explicitly provided)
  - `work_surface_id` — identifier of the bound Work Surface, if any (enables iteration context inheritance)
- **SR-WORK-SURFACE §5.4:** Added new section "Loop ↔ Work Surface binding (normative)" documenting:
  - Binding semantics (validation, population, error handling)
  - Iteration context inheritance behavior
  - Projection model (unidirectional Loop → Work Surface relationship)
- **Rationale:** Documents Phase 5b implementation of Loop ↔ Work Surface binding per SR-PLAN-V5 §4. Enables semantic loops to bind to Work Surfaces at creation time and auto-inherit context for iterations.
- **Classification:** G:MINOR (additive; backward-compatible; existing Loops without `work_surface_id` continue to function as unbound loops).

---

## 1. What requires change control

### 1.1 Governance-impacting artifacts

The following artifacts are considered **governed** and MUST be changed only via the SR-CHANGE workflow:

- Any artifact listed in the **canonical governed set** (frontmatter `ext.change_mgmt.governed_set.canonical_ids`)
- Any artifact referenced as **normative** by SR-CONTRACT or SR-SPEC
- Any instance's **control surface artifacts** that affect enforcement (e.g., a Gate Registry, verification profile definitions, portal playbooks, or other directive-owned operational policies)

### 1.2 Canonical governed set

The canonical governed set for SOLVER-Ralph is:

| Artifact | Purpose | Layer |
|----------|---------|-------|
| SR-CHARTER | Human-provided scope, boundaries, and build priorities | Build |
| SR-TYPES | Type registry and metadata schema | Platform |
| SR-CONTRACT | Binding invariants | Platform |
| SR-SPEC | Technical mechanics | Platform |
| SR-WORK-SURFACE | Work surface schemas (intake + procedure + context bundle) | Platform |
| SR-PROCEDURE-KIT | Procedure template registry | Platform |
| SR-SEMANTIC-ORACLE-SPEC | Semantic oracle interface and required evidence outputs | Platform |
| SR-EVENT-MANAGER | Deterministic state / eligibility / dependency-graph computation spec | Platform |
| SR-REPLAY-PROOF | Determinism proof per C-EVT-7 (verified_by SR-EVENT-MANAGER) | Platform |
| SR-AGENT-WORKER-CONTRACT | Minimum worker behavior contract | Platform |
| SR-AGENTS | Agent actor semantics | Platform |
| SR-MODEL | Layer model and document-role map (orientation; non-binding) | Platform |
| SR-INTENT | Design rationale (non-binding) | Platform |
| SR-PLAN | Instance scope + deliverable inventory + dependency graph | Instance |
| SR-DIRECTIVE | Instance operational policy | Instance |
| SR-EXCEPTIONS | Exception ledger (build-time authority port) | Build |
| SR-CHANGE | This document | Build |
| SR-TASKS | Task assignment from event manager | Build |

### 1.3 Change classification

Change classification is used for routing, required evidence strength, and review expectations.

- **G:PATCH** — Clarify text, fix typos, tighten wording, add examples, correct references. No semantic changes.
- **G:MINOR** — Additive / backward-compatible semantic extensions (new gate IDs, new stop triggers, new portal functions, new default profiles, additional canonical artifacts) **provided that older baselines remain interpretable**.
- **G:MAJOR** — Breaking changes: rename/remove portals or binding terms, redefine what "Verified/Approved/Shippable" mean, change core event semantics, remove required invariants, or any change that makes old baselines ambiguous.

**Note on canonical-set expansion:** Adding a new canonical artifact MAY be treated as **G:MINOR** if:
1) existing canonical IDs are not renamed/removed, and  
2) the new requirement applies **prospectively** (new freezes and new "current" selections), and  
3) the change includes an explicit "compatibility note" stating how older freezes remain interpretable.

---

## 2. Standard change workflow

All governed changes MUST follow this sequence:

1) **Proposal** — Draft the change as a proposal artifact (e.g., draft document, patch, diff).
2) **Verification** — Run the applicable oracle suite(s) to produce evidence bundles (lint, schema validation, coherence checks, cross-reference checks, etc.).
3) **Approval** — Obtain a HUMAN approval via the appropriate portal function (see §3).
4) **Registration / Selection** — Make the result "current" (where applicable) via the governed-artifact registry mechanism defined by SR-SPEC.

**Binding rule:** A change is not authoritative merely because it exists. Authority flows only through durable objects: evidence bundles + explicit human approvals + explicit "current" selection where needed.

---

## 3. Portal routing and approval semantics

### 3.1 Portal functions vs portal IDs (binding)

SR-CHANGE defines **portal functions** (semantic trust-boundary roles). An instance implements these functions using one or more **portal IDs**.

- A **portal function** answers: *what kind of binding decision is this?*
- A **portal ID** answers: *where (which portal surface) was the decision executed?*

**Minimum instantiated portal IDs for an instance (build phase):**
- `GovernanceChangePortal`
- `ReleaseApprovalPortal`
- `HumanAuthority`

### 3.2 Portal-function aliasing (binding)

An instance MAY implement a portal function as a typed request within another portal ID (e.g., implementing OracleSuiteChange inside GovernanceChangePortal), provided:

- The record is **explicit** about what is being approved/decided (no ambiguity).
- The record is **attributable** to a HUMAN identity.
- The record references **all required evidence** for that decision type.
- The portal playbook for the hosting portal includes the alias request type and required checks.

Aliasing does **not** reduce requirements; it only changes which UI/API endpoint is used to execute the function.

### 3.3 Portal routing matrix (binding)

If a change spans categories, **each applicable portal function MUST be satisfied**.


**Build-phase exceptions:** Exception approvals are executed by **Human Authority** and recorded in **SR-EXCEPTIONS**. Agents must cite an `EX-####` entry when proceeding under a deviation.

| Change kind | Required portal function(s) | Default portal ID(s) | Notes |
|---|---|---|---|
| Change to a normative artifact (SR-CONTRACT, SR-SPEC, SR-TYPES, SR-CHANGE) | GovernanceChange | GovernanceChangePortal | Requires evidence + explicit approval |
| Change to a directional artifact (SR-INTENT, SR-GUIDE, SR-AGENTS) | GovernanceChange | GovernanceChangePortal | Canonical status means auditability matters |
| Change to instance policy (SR-DIRECTIVE) | GovernanceChange | GovernanceChangePortal | Instance policy is governed |
| Change to "current selection" of any canonical artifact | GovernanceChange | GovernanceChangePortal | Selection changes are binding decisions |
| Oracle suite registration / required↔advisory changes / environment-constraint changes | OracleSuiteChange (+ GovernanceChange if it affects a canonical rule) | GovernanceChangePortal (alias) or OracleSuiteChangePortal (dedicated) | No silent oracle weakening (§7.2) |
| Deviation / Deferral / Gate Waiver creation or modification | ExceptionApproval | HumanAuthority | Exceptions create binding state without editing canonical docs |
| Phase completion ("phase gate") approval | PhaseGate | GovernanceChangePortal (alias) or PhaseGatePortal (dedicated) | Phase gates are binding coordination events |
| Budget extension approval (after BUDGET_EXHAUSTED or similar) | BudgetExtension | GovernanceChangePortal (alias) or BudgetExtensionPortal (dedicated) | Must reference budget status + rationale |
| Candidate release (Shippable declaration / release approval) | ReleaseApproval | ReleaseApprovalPortal | Freeze creation depends on this approval (§5) |

### 3.4 Approval/decision record requirements (binding via SR-CONTRACT)

For any portal function execution, the binding record MUST, at minimum:

- identify **which portal ID** was crossed
- identify **what** was approved/decided (subject refs, request refs, or both)
- include **evidence refs** that the approver reviewed (explicit empty list is allowed only if the relevant playbook permits it)
- include an explicit list of **exceptions acknowledged** (explicit empty list is allowed)

**No hardcoded portal_id:** `portal_id` MUST reflect the actual portal surface used.

---

## 4. Drafts, versions, and "current" selection

- Draft artifacts MAY exist without being "current."
- The system MUST be able to dereference historical versions for audit/replay.
- Changing "current" selection of canonical artifacts is a **binding change** and MUST be approved (GovernanceChange portal function).

---

## 5. Freeze policy (Shippable baselines)

Freeze records are the durable boundary objects for public "baseline" claims. A freeze bundles:

- the candidate and its verification status
- the evidence bundles and oracle suite identity
- any active exceptions (explicitly acknowledged)
- the manifest of governed artifacts in force

### 5.1 Freeze manifest completeness (binding)

A Freeze record's `artifact_manifest[]` MUST include, at minimum, all **canonical governed artifacts** (the canonical governed set), at the versions in force at the time of the freeze.

### 5.2 Instance inputs (binding)

If an instance uses directive-owned control surfaces, the Freeze record MUST also include:

- the **instance SR-DIRECTIVE** and its control surface artifacts (Gate Registry / Profile Definitions / Portal Playbooks) that governed execution.

This ensures that a baseline is replayable with the *same rules* that produced it.

### 5.3 Freeze requires release approval (binding)

A freeze that is intended to serve as a basis for "Shippable" MUST be preceded by a **ReleaseApproval** portal function execution that:

- references the candidate and its evidence
- explicitly acknowledges active exceptions (even if none)

### 5.4 Freeze and staleness (binding)

If any upstream dependency in a Freeze manifest changes (including "current" selection changes), impacted dependents MUST be marked **stale** and MUST NOT be treated as a valid basis for "Shippable" without re-evaluation and (typically) a new freeze.

---

## 6. Exceptions: deviations, deferrals, gate waivers

Exceptions are explicit, durable records that allow controlled progress without silently rewriting canonical meaning.

### 6.1 Exception kinds (binding)

- **Deviation** — "We are intentionally differing from a stated requirement," with scope + rationale + mitigation + expiry/review.
- **Deferral** — "We will satisfy this requirement later," with scope + conditions + expiry/review.
- **Gate Waiver** — "We acknowledge a specific verification failure and proceed anyway," with scope + evidence + mitigation.

### 6.2 Non-waivable failures (binding)

Gate Waivers MUST NOT be used to bypass integrity conditions (see §7.1). If an integrity condition is present, resolution requires remediation or an explicitly approved suite change (OracleSuiteChange portal function) plus any required governance change.

---

## 7. Oracle suite change constraints

### 7.1 Integrity conditions are not waivable (binding)

The following are **integrity conditions** and MUST NOT be bypassed via Gate Waiver:

- `ORACLE_TAMPER`
- `ORACLE_ENV_MISMATCH`
- `ORACLE_GAP`
- `ORACLE_FLAKE`
- `EVIDENCE_MISSING`

Resolution requires rerun/remediation in compliance, or an explicit, human-approved oracle-suite change (OracleSuiteChange portal function) plus any required governance change.

### 7.2 No silent oracle weakening (binding)

The system MUST NOT silently:

- disable required oracles,
- downgrade FAIL to PASS,
- suppress failing categories,
- reduce oracle scope/coverage, or
- change required/advisory classification

without explicit governed routing and explicit human approval recorded as a binding record.

A Gate Waiver is not "silent weakening" because it preserves the failure evidence and attaches explicit human acknowledgment.

---

## 8. Decision records

Decision records capture binding, attributable human arbitration when an operational decision changes execution course (e.g., repeated failure, budget exhaustion, phase gate rejection).

### 8.1 When a Decision record is required (binding)

A Decision record MUST be created for at least:

- resolution of stop-the-line events that require human judgment (e.g., REPEATED_FAILURE)
- budget extensions (BudgetExtension portal function)
- phase gate approvals/rejections (PhaseGate portal function)
- any "proceed anyway" action that is not already captured as an Exception approval

### 8.2 Decision routing (binding)

Decision records are executed through the **portal function** that matches the decision type (GovernanceChange / PhaseGate / BudgetExtension / ReleaseApproval), implemented via the instance's portal IDs (including aliasing where permitted).

---

## 9. Emergency change procedure

In emergencies (e.g., integrity faults preventing progress), an instance MAY invoke an emergency change procedure that:

- records the emergency rationale as a Decision record
- limits scope/time explicitly
- schedules follow-up to restore normal governance

Emergency procedures MUST NOT be used to bypass integrity conditions.

---

## 10. Build and self-amendment (SR-CHANGE)

Changes to SR-CHANGE itself:

- MUST follow the same workflow (proposal → evidence → approval → current selection)
- MUST include evidence demonstrating coherence with SR-CONTRACT and SR-SPEC
- SHOULD include a compatibility note if semantics change
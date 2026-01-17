---
doc_id: SR-README
doc_kind: governance.readme
layer: build
status: draft
normative_status: index

refs:
  - rel: governed_by
    to: SR-CHANGE
---

# SR-README

Tasks are no longer assigned by SR-PLAN because the build out phase is complete.  See below for the details of your current assignment.

Start by reviewing docs/charter/SR-CHARTER.md

The project documentation constitutes a total development plan and specification with detailed instructions on types and contracts.  Always read the SR-* files that appear related to the task before going to read the code files.  Documentation leads development for this project.  Documentation is how you know your ontology, epistemology, and semantics.

Once you have read the appropriate project docs, then do whatever deliverables and tasks you think should be done next.

Your task is done when there are no more deliverables to be assigned, which means all tests have passed and gates cleared.

You should push on to resolve any findings and consult the docs/ frequently for guidance and direction from the SR-* documents, several of which are normative.  They are typed documents and structured to aid in navigation.

You should git add && commit && push after completing each deliverable.  You can remain on the same branch throughout this development session.

ALWAYS refer to the project docs/*/SR-* for the authoritative coding architecture, plan, and semantics.  Understand the full set of docs/ and refer to the applicable SR-* document instead of making assumptions.

When troubleshooting, refer to the appropriate SR-* documents.

---

## Canonical document paths

Canonical index for the SR-* document set.

| doc_id | Folder | Purpose |
|--------|--------|---------|
| SR-CHARTER | `charter/` | Project scope and priorities |
| SR-CONTRACT | `platform/` | Binding invariants |
| SR-SPEC | `platform/` | Platform mechanics |
| SR-TYPES | `platform/` | Type registry and schemas |
| SR-WORK-SURFACE | `platform/` | Work surface definitions |
| SR-PROCEDURE-KIT | `platform/` | Procedure templates |
| SR-SEMANTIC-ORACLE-SPEC | `platform/` | Semantic oracle interface |
| SR-EVENT-MANAGER | `platform/` | Event projections spec |
| SR-AGENT-WORKER-CONTRACT | `platform/` | Agent/worker contract |
| SR-INTENT | `platform/` | Design rationale |
| SR-CHANGE | `build-governance/` | Change control process |
| SR-AGENTS | `build-governance/` | Agent actor model |
| SR-TASKS | `build-governance/` | Task assignment |
| SR-EXCEPTIONS | `build-governance/` | Exception ledger |
| SR-PLAN | `program/` | Build plan instance |
| SR-DIRECTIVE | `program/` | Execution policy |
| SR-TEMPLATES | `platform/` | User configuration registry |
| SR-README | `charter/` | This index |

### Feature Implementation Plans

The `docs/planning/` folder contains feature-specific implementation plans that are subordinate to SR-PLAN. These plans detail specific feature implementations and are not permanent governance documents — they become historical artifacts once implementation is complete.

Per `SR-PLAN-GAP-ANALYSIS.md`, the path to Milestone 1 completion:

| Plan | Scope | Key Deliverables | Status |
|------|-------|------------------|--------|
| SR-PLAN-V7 | Stabilization & Attachments | Tests, UX, `record.attachment` | **Complete** |
| SR-PLAN-V8 | Oracle Runner & Semantic Suites | D-24, D-25, D-27, D-39 | **Complete** |
| SR-PLAN-V9 | Semantic Worker & Branch 0 | D-23, D-41, D-36 | **Reviews Complete — Implementation Active** |

**Milestone 1 (MVP) projected completion:** After V9 implementation (~5-8 sessions)

---

## SR-PLAN-V9 Status (APPROVED — IMPLEMENTATION ACTIVE)

| Phase | Status | Description |
|-------|--------|-------------|
| V9-1: Semantic Worker Integration | ✅ **COMPLETE** | Wire semantic worker to oracle runner, persist evidence |
| V9-2: E2E Flow Integration Test | ✅ **COMPLETE** | Branch 0 flow test with 5 stages, portal approvals, freeze |
| V9-3: Replayability Demonstration | 🔄 **ACTIVE** | Prove EventManager.rebuild() determinism (D-36) |
| V9-4: Branch 0 Acceptance | 📝 Planned | Document criteria satisfaction, human approval |

**Reviews Complete:**
- Coherence review: APPROVED (`docs/reviews/SR-PLAN-V9-COHERENCE-REVIEW.md`)
- Consistency evaluation: APPROVED WITH NOTES (`docs/reviews/SR-PLAN-V9-CONSISTENCY-EVALUATION.md`)

---

## V9-1 Session Summary (2026-01-16)

V9-1 wired the semantic worker to real oracle execution. Key implementation details:

**Files Modified:**
- `sr-adapters/src/semantic_worker.rs` — `SemanticWorkerBridge` is now generic over `E: EvidenceStore` and `W: CandidateWorkspace`; `run_semantic_oracles()` calls `PodmanOracleRunner.execute_suite()`; `emit_evidence_bundle()` persists to MinIO and emits `EvidenceBundleRecorded` via NATS
- `sr-adapters/src/worker.rs` — Added `OracleError`, `WorkspaceError`, `StorageError` variants to `WorkerError`
- `sr-api/src/config.rs` — Added `enable_semantic_worker: bool` (env: `SR_ENABLE_SEMANTIC_WORKER`)
- `sr-api/src/main.rs` — Spawns `SemanticWorkerBridge` when enabled, initializes NATS, EventManager, PodmanOracleRunner, SimpleCandidateWorkspace

**Key Integration Points:**
- Semantic worker subscribes to `IterationStarted` events via NATS (`sr.events.iteration`)
- Oracle execution uses `OracleSuiteRegistry.get_suite(SUITE_INTAKE_ADMISSIBILITY_ID)`
- Evidence persisted via `EvidenceStore.store()` trait method
- Events emitted to `subjects::ORACLE_EVENTS` ("sr.events.oracle")

**Commit:** `19217ae feat(V9-1): Wire semantic worker to real oracle execution`

---

## V9-2 Session Summary (2026-01-16)

V9-2 implemented Branch 0 E2E integration tests. Key implementation details:

**Files Created:**
- `sr-api/tests/integration/branch_0_e2e_test.rs` — Complete E2E test suite with 4 tests

**Test Cases:**
- `test_branch_0_complete_flow` — Full 5-stage workflow: FRAME → OPTIONS → DRAFT → SEMANTIC_EVAL → FINAL with portal approvals and freeze baseline
- `test_branch_0_portal_approvals_required` — Verifies 412 rejection and approval recording at trust boundaries
- `test_branch_0_evidence_capture` — Polls for evidence bundles when semantic worker is running
- `test_branch_0_freeze_baseline` — Creates and verifies freeze record

**Key Features:**
- Follows existing TestClient pattern from `semantic_ralph_loop_e2e.rs`
- Helper functions: `create_work_surface_generic()`, `create_and_activate_loop()`, `complete_stage()`, `record_approval()`, `poll_for_evidence()`
- Tests GENERIC-KNOWLEDGE-WORK template (5 stages with portal approvals at SEMANTIC_EVAL and FINAL)
- Graceful skip when semantic worker not running (evidence capture test)

---

## Next Instance Prompt: V9-3 Replayability Demonstration

### Assignment

**Implement V9-3: Replayability Demonstration (D-36)** — prove deterministic replay per SR-CONTRACT C-EVT-7. This is a CODE IMPLEMENTATION task.

### Context from V9-2

The E2E test suite proves the Branch 0 flow works:
- Work surfaces with GENERIC-KNOWLEDGE-WORK template (5 stages)
- Portal approvals at SEMANTIC_EVAL and FINAL
- Freeze baseline creation

V9-3 must now **prove** replay determinism — that `EventManager.rebuild()` produces identical state from the same event sequence.

### Quick Orientation

1. **Read the plan:** `docs/planning/SR-PLAN-V9.md` §3.3 (Phase V9-3)
2. **Review EventManager:** `sr-adapters/src/event_manager.rs` for `rebuild()` and `apply_event()`
3. **Understand C-EVT-7:** SR-CONTRACT — projections MUST be rebuildable from event log

### What V9-3 Must Deliver

| File | Action | Description |
|------|--------|-------------|
| `sr-adapters/src/event_manager.rs` | MODIFY | Add `compute_state_hash()` and `verify_replay()` methods |
| `sr-adapters/src/replay.rs` | CREATE | Replay proof types (`ReplayProof`, `ReplayDiscrepancy`) |
| `sr-api/tests/integration/replay_determinism_test.rs` | CREATE | Integration test proving determinism |
| `docs/platform/SR-REPLAY-PROOF.md` | CREATE | Formal proof documentation |

### Replay Proof Requirements

The replay proof must demonstrate:

1. **State Hash Equality** — `compute_state_hash()` produces deterministic hashes
2. **Eligible Set Equality** — `compute_eligible_set()` returns identical results after replay
3. **Status Projection Equality** — All work unit statuses identical after replay
4. **No Ghost Inputs** — `apply_event()` uses only event data, no external state

### Canonical References

| Document | Relevant Sections |
|----------|-------------------|
| SR-PLAN-V9 | §3.3 (V9-3 deliverables and replay proof architecture) |
| SR-CONTRACT | C-EVT-7 (Projections rebuildable from event log) |
| SR-EVENT-MANAGER | §2 (Projections), §3 (Determinism) |

### Acceptance Criteria

- [ ] `EventManager.compute_state_hash()` implemented
- [ ] `EventManager.verify_replay()` implemented
- [ ] Replay proof test passes
- [ ] SR-REPLAY-PROOF.md documents the proof
- [ ] Proof covers: state hash, eligible set, status projections
- [ ] `cargo test --package sr-adapters replay` passes

### On Completion

1. Commit changes with descriptive message
2. Update SR-PLAN-V9 to mark V9-3 acceptance criteria complete
3. Update SR-README to mark V9-3 complete and set V9-4 as active
4. Push to branch `solver-ralph-9`



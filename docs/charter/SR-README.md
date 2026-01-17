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
| V9-2: E2E Flow Integration Test | 🔄 **ACTIVE** | Complete Branch 0 flow test (5 stages) |
| V9-3: Replayability Demonstration | 📝 Planned | Prove EventManager.rebuild() determinism (D-36) |
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

## Next Instance Prompt: V9-2 E2E Flow Integration Test

### Assignment

**Implement V9-2: End-to-End Flow Integration Test** — create an integration test demonstrating complete Branch 0 procedure flow. This is a CODE IMPLEMENTATION task.

### Context from V9-1

The semantic worker is now fully wired:
- `SemanticWorkerBridge<E, W>` requires `oracle_runner`, `evidence_store`, `oracle_registry`, `candidate_workspace`
- Worker can be enabled via `SR_ENABLE_SEMANTIC_WORKER=true`
- Real oracle execution happens via `PodmanOracleRunner`
- Evidence bundles are persisted to MinIO and events emitted to NATS

V9-2 must now **prove** this integration works end-to-end.

### Quick Orientation

1. **Read the plan:** `docs/planning/SR-PLAN-V9.md` §3.2 (Phase V9-2)
2. **Review existing integration tests:** `sr-api/tests/integration/` for patterns
3. **Understand Branch 0 criteria:** SR-PLAN §4.1

### What V9-2 Must Deliver

| File | Action | Description |
|------|--------|-------------|
| `sr-api/tests/integration/branch_0_e2e_test.rs` | CREATE | Full E2E integration test |
| `sr-api/tests/fixtures/branch_0_intake.json` | CREATE | Test intake fixture |
| `sr-api/tests/fixtures/branch_0_work_surface.json` | CREATE | Test work surface fixture |

### Test Flow Requirements

The E2E test must demonstrate the complete Branch 0 flow:

1. **Work Surface Creation** — Create work surface with intake, procedure template, oracle suites
2. **Loop Creation** — Create Ralph loop bound to work surface
3. **Iteration Cycling** — Start iterations, verify semantic worker processes them
4. **Stage Progression** — Traverse FRAME → OPTIONS → DRAFT → SEMANTIC_EVAL → FINAL
5. **Evidence Capture** — Verify evidence bundles created for each stage
6. **Portal Approvals** — Record human approvals at gate stages
7. **Freeze Baseline** — Create freeze record on completion

### Infrastructure Notes

The test will need:
- Running PostgreSQL, MinIO, NATS (or mocks)
- Test mode flags to skip actual Podman execution
- Existing `TestApp` pattern from other integration tests

### Canonical References

| Document | Relevant Sections |
|----------|-------------------|
| SR-PLAN-V9 | §3.2 (V9-2 deliverables and test flow sketch) |
| SR-PLAN | §4.1 (Branch 0 acceptance criteria) |
| SR-PROCEDURE-KIT | §2 (GENERIC-KNOWLEDGE-WORK stages) |
| SR-CONTRACT | C-LOOP-*, C-CTX-*, C-VER-* |

### Acceptance Criteria

- [ ] E2E test creates work surface and loop
- [ ] E2E test progresses through all 5 stages
- [ ] E2E test records portal approvals where required
- [ ] E2E test creates freeze baseline
- [ ] Evidence bundles exist for each stage transition
- [ ] Test passes in CI environment
- [ ] Test documented in project README or test file docstring

### On Completion

1. Commit changes with descriptive message
2. Update SR-PLAN-V9 to mark V9-2 acceptance criteria complete
3. Update SR-README to mark V9-2 complete and set V9-3 as active
4. Push to branch `solver-ralph-9`



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

| doc_id | Status | Purpose |
|--------|--------|---------|
| SR-PLAN-V6 | **ready** | UI Integration for MVP Workflow (coherence verified, ready for implementation) |
| SR-PLAN-V5 | **complete** | Semantic Ralph Loop End-to-End Integration (Phases 5a-5d) |
| SR-PLAN-V4 | **complete** | Work Surface Composition (Phase 4) — All phases complete |
| SR-PLAN-V3 | **complete** | Intakes & References implementation (Phases 0-3) |
| SR-PLAN-V2 | superseded | Intakes & References draft (10 unresolved issues) |

---

## SR-PLAN-V6 Implementation Status

| Phase | Status | Description |
|-------|--------|-------------|
| Coherence Review | ✅ Complete | `docs/reviews/SR-PLAN-V6-COHERENCE-REVIEW.md` — PASS_WITH_NOTES |
| V6-1: Backend | ✅ Complete | `POST /work-surfaces/{id}/start` endpoint implemented |
| V6-2: Frontend | ✅ Complete | Wizard calls `/start` after Work Surface creation |
| V6-3: E2E Verification | ⏳ Pending | Document and verify complete human workflow |

---

## Previous Session Summary (V6-1 Backend)

### What Was Done

1. **Implemented `start_work_surface` handler** in `crates/sr-api/src/handlers/work_surfaces.rs`:
   - `StartWorkResponse` struct for API response
   - `start_work_surface` handler with idempotency logic
   - `create_loop_internal` — creates Loop with HUMAN actor and default directive_ref
   - `activate_loop_internal` — activates CREATED loops
   - `start_iteration_as_system` — starts iteration with SYSTEM actor (`system:work-surface-start`)
   - `fetch_work_surface_refs` — populates iteration refs from Work Surface context

2. **Added `PreconditionFailed` error variant** to `crates/sr-api/src/handlers/error.rs`

3. **Registered route** in `crates/sr-api/src/main.rs`:
   - `POST /api/v1/work-surfaces/:work_surface_id/start`

4. **Updated canonical documentation**:
   - `docs/platform/SR-SPEC.md` — Added §2.3.12 Work Surfaces (all Work Surface API endpoints)
   - `docs/platform/SR-WORK-SURFACE.md` — Added §5.5 Starting work via /start endpoint

### Verification

- `cargo build --package sr-api` ✅ (warnings only)
- `cargo test --package sr-api` ✅ (36 passed)

### Files Modified

| File | Lines Added |
|------|-------------|
| `crates/sr-api/src/handlers/work_surfaces.rs` | ~250 |
| `crates/sr-api/src/handlers/error.rs` | ~10 |
| `crates/sr-api/src/main.rs` | ~4 |
| `docs/platform/SR-SPEC.md` | ~45 |
| `docs/platform/SR-WORK-SURFACE.md` | ~42 |

### Notes for Next Instance

- The `get_loop_by_work_unit` method was already present in `projections.rs` (from an earlier partial implementation attempt)
- All acceptance criteria from SR-PLAN-V6 §4 (Phase V6-1) are satisfied
- Documentation was updated to close a gap: Work Surface API endpoints were not previously documented in SR-SPEC

---

## Previous Session Summary (V6-2 Frontend)

### What Was Done

1. **Modified `handleSubmit` in `WorkSurfaceCompose.tsx`** (lines 170-231):
   - Added Step 2 after Work Surface creation to call `POST /work-surfaces/{id}/start`
   - Graceful error handling: if `/start` fails, still navigate to detail page
   - Handles `already_started: true` response by logging info message
   - Uses inline `fetch()` pattern per SR-PLAN-V6 §5.1 (no centralized API client)

### Verification

- `npm run type-check` ✅ (no errors)
- `npm run build` ✅ (built successfully)

### Files Modified

| File | Lines Changed |
|------|---------------|
| `ui/src/pages/WorkSurfaceCompose.tsx` | ~25 (modified `handleSubmit`) |

### Notes for Next Instance

- The codebase uses inline `fetch()` calls, not a centralized API client
- SR-README mentioned `ui/src/lib/api.ts` but that file doesn't exist
- Implementation follows SR-PLAN-V6 §5.1 pattern guidance

---

## Next Instance Prompt: Execute SR-PLAN-V6 Phase V6-3 (E2E Verification)

### Context

Phases V6-1 (Backend) and V6-2 (Frontend) are complete. The full workflow is now wired:
- Wizard creates Work Surface
- Wizard calls `/start` to create Loop, activate it, and start Iteration

### Current State

- Branch: `solver-ralph-7`
- Backend: `/work-surfaces/{id}/start` endpoint implemented
- Frontend: Wizard calls `/start` after Work Surface creation
- Gap: Full human workflow not yet verified end-to-end

### Assignment

Execute **Phase V6-3: E2E Verification** as specified in `docs/planning/SR-PLAN-V6.md` §4.

This phase documents and verifies the complete human workflow through the UI.

### Verification Steps

Per SR-PLAN-V6 §7.3:

1. Start API: `SR_AUTH_TEST_MODE=true cargo run --package sr-api`
2. Start UI: `cd ui && npm run dev`
3. Open browser to `http://localhost:5173`
4. Navigate to `/intakes/new`, create and activate an intake
5. Navigate to `/work-surfaces/new`
6. Select intake, select template, click "Create Work Surface"
7. Verify redirect to `/work-surfaces/{id}`
8. Complete stages: FRAME → OPTIONS → DRAFT → FINAL
9. For FINAL: click "Record Approval", approve, then complete
10. Verify Work Surface status = "completed"

### Acceptance Criteria

From SR-PLAN-V6 §4 (Phase V6-3):

- [ ] Full workflow completes without curl commands
- [ ] All stages can be completed via UI
- [ ] Approval flow works for FINAL stage
- [ ] No console errors during workflow

### Reference Documents

- `docs/planning/SR-PLAN-V6.md` — Implementation plan
- `docs/platform/SR-SPEC.md` §2.3.12 — Work Surface API documentation

### Phase Complete When

- [ ] Manual test of complete workflow succeeds
- [ ] All acceptance criteria documented as passing
- [ ] SR-README.md updated with V6-3 completion status
- [ ] Committed and pushed


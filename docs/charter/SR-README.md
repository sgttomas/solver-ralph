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
| SR-PLAN-V6 | **in progress** | UI Integration for MVP Workflow (V6-1, V6-2 complete; V6-3 pending) |
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

1. **Modified `handleSubmit` in `ui/src/pages/WorkSurfaceCompose.tsx`** (lines 170-231):
   - Added Step 2 after Work Surface creation to call `POST /work-surfaces/{id}/start`
   - Graceful error handling: if `/start` fails, still navigate to detail page
   - Handles `already_started: true` response by logging info message
   - Uses inline `fetch()` pattern per SR-PLAN-V6 §5.1 (no centralized API client)

2. **Updated SR-README.md** with V6-2 completion status and V6-3 prompt

### Verification

- `npm run type-check` ✅ (no errors)
- `npm run build` ✅ (built successfully)
- Commit: `e4ff9c7` pushed to `solver-ralph-7`

### Files Modified

| File | Lines Changed |
|------|---------------|
| `ui/src/pages/WorkSurfaceCompose.tsx` | ~25 (modified `handleSubmit`) |
| `docs/charter/SR-README.md` | Updated status and next prompt |

### Notes for Next Instance

- The UI codebase uses **inline `fetch()` calls**, not a centralized API client — this is intentional per SR-PLAN-V6 §5.1
- The wizard file is `ui/src/pages/WorkSurfaceCompose.tsx` (not `components/`)
- All acceptance criteria from SR-PLAN-V6 §4 (Phase V6-2) are satisfied

---

## Next Instance Prompt: Execute SR-PLAN-V6 Phase V6-3 (E2E Verification)

### Context

Phases V6-1 (Backend) and V6-2 (Frontend) are complete. The one-click workflow is now fully wired:

1. User completes wizard → `POST /work-surfaces` creates Work Surface
2. Wizard automatically calls `POST /work-surfaces/{id}/start`
3. Backend creates Loop, activates it, starts Iteration as SYSTEM actor
4. User arrives at Work Surface detail page ready to complete stages

### Current State

- Branch: `solver-ralph-7`
- Backend: `/work-surfaces/{id}/start` endpoint implemented and documented
- Frontend: Wizard calls `/start` after Work Surface creation
- **Gap**: Full human workflow not yet verified end-to-end via manual testing

### Assignment

Execute **Phase V6-3: E2E Verification** as specified in `docs/planning/SR-PLAN-V6.md` §4.

This phase verifies the complete human workflow through the UI — no curl commands should be needed.

### Verification Steps

Per SR-PLAN-V6 §7:

```bash
# Terminal 1: Start API in test mode
SR_AUTH_TEST_MODE=true cargo run --package sr-api

# Terminal 2: Start UI dev server
cd ui && npm run dev
```

Then in browser at `http://localhost:5173`:

1. **Create Intake**: Navigate to `/intakes/new`, fill form, click "Create Intake"
2. **Activate Intake**: On detail page, click "Activate"
3. **Start Wizard**: Navigate to `/work-surfaces/new`
4. **Select Intake**: Choose the activated intake
5. **Select Template**: Choose a compatible procedure template
6. **Create Work Surface**: Click "Create Work Surface"
7. **Verify Redirect**: Should arrive at `/work-surfaces/{id}` with Loop/Iteration active
8. **Complete FRAME**: Click "Complete Stage", fill form, submit
9. **Complete OPTIONS**: Repeat
10. **Complete DRAFT**: Repeat
11. **Complete FINAL**: First click "Record Approval" link, approve, then complete stage
12. **Verify Completion**: Work Surface status should be "completed"

### Acceptance Criteria

From SR-PLAN-V6 §4 (Phase V6-3):

- [ ] Full workflow completes without curl commands
- [ ] All stages can be completed via UI
- [ ] Approval flow works for FINAL stage
- [ ] No console errors during workflow

### Reference Documents

- `docs/planning/SR-PLAN-V6.md` — Implementation plan (§4 for phases, §7 for verification)
- `docs/platform/SR-SPEC.md` §2.3.12 — Work Surface API documentation
- `docs/platform/SR-WORK-SURFACE.md` §5.5 — /start endpoint semantics

### Phase Complete When

- [ ] Manual test of complete workflow succeeds
- [ ] All acceptance criteria documented as passing
- [ ] SR-README.md updated with V6-3 completion status
- [ ] SR-PLAN-V6 marked as **complete** in Feature Implementation Plans table
- [ ] Committed and pushed


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
| SR-PLAN-V7 | **pending** | MVP Stabilization & Evidence Foundation |
| SR-PLAN-V6 | **complete** | UI Integration for MVP Workflow (V6-1, V6-2, V6-3 complete) |
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
| V6-3: E2E Verification | ✅ Complete | Complete human workflow verified end-to-end |

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

## Previous Session Summary (V6-3 E2E Verification)

**Session Goal:** Execute SR-PLAN-V6 Phase V6-3 — verify that the complete MVP workflow functions end-to-end through the UI.

### What Was Accomplished

1. **Environment Setup**
   - Started API server: `SR_AUTH_TEST_MODE=true cargo run --package sr-api` (port 3000)
   - Started UI dev server: `cd ui && npm run dev` (port 3002)

2. **Complete Workflow Verification**
   - Created Intake (`intake:01KF3ZQB1ADK3X8AK31KZGQECB`)
   - Activated Intake
   - Created Work Surface (`ws:01KF3ZS01HS3AYV7VJWDZQT4NR`)
   - Called `/start` endpoint — verified Loop/Iteration creation (V6-1 deliverable)
   - Completed all 4 stages: FRAME → OPTIONS → DRAFT → FINAL
   - Recorded approval for FINAL stage (approval gating works)
   - Work Surface reached **completed** status

3. **Documentation Updates**
   - Updated SR-README.md with V6-3 completion summary
   - Updated SR-PLAN-V6.md status to **Complete**
   - Marked all acceptance criteria as satisfied
   - Added SR-PLAN-V7 to Feature Implementation Plans table
   - Added next instance prompt for SR-PLAN-V7

### Commits

| Hash | Description |
|------|-------------|
| `22bc8a2` | docs: complete SR-PLAN-V6 Phase V6-3 E2E verification |
| `8813f9d` | docs: add SR-PLAN-V7 to implementation plans and next instance prompt |

### Acceptance Criteria (All Satisfied)

- [x] Full workflow completes without curl commands
- [x] All stages can be completed via UI endpoints
- [x] Approval flow works for FINAL stage
- [x] No console errors during workflow

### Work Surface Final State

```json
{
  "status": "completed",
  "current_stage_id": "stage:FINAL",
  "completed_at": "2026-01-16T18:07:16.785895+00:00",
  "stage_status": {
    "stage:FRAME": { "status": "completed" },
    "stage:OPTIONS": { "status": "completed" },
    "stage:DRAFT": { "status": "completed" },
    "stage:FINAL": { "status": "completed" }
  }
}
```

### Milestone Reached

**SR-PLAN-V6 is now complete.** The MVP UI integration is fully functional:
- V6-1 (Backend): `/start` endpoint implemented
- V6-2 (Frontend): Wizard wired to call `/start`
- V6-3 (E2E Verification): Complete workflow verified

---

## SR-PLAN-V6 Complete

All phases of SR-PLAN-V6 have been successfully completed:

| Phase | Status | Summary |
|-------|--------|---------|
| Coherence Review | ✅ | Plan validated for coherence |
| V6-1: Backend | ✅ | `/start` endpoint implemented |
| V6-2: Frontend | ✅ | Wizard wired to call `/start` |
| V6-3: E2E Verification | ✅ | Complete workflow verified |

The MVP UI integration is now complete. Users can:
1. Create and activate an Intake
2. Use the wizard to create a Work Surface (auto-starts work)
3. Complete all stages via the UI
4. Record approvals for approval-gated stages
5. See the Work Surface reach "completed" status

---

## Next Instance Prompt: Plan SR-PLAN-V7 (MVP Stabilization & Evidence Foundation)

### Context

SR-PLAN-V6 is complete. The MVP workflow is now fully functional end-to-end through the UI:
- Intakes can be created and activated
- Work Surfaces can be created via wizard (auto-starts Loop/Iteration)
- All stages can be completed with approval gating
- Work Surfaces reach "completed" status

### Current State

- Branch: `solver-ralph-7`
- MVP UI Integration: **Complete** (SR-PLAN-V6)
- All prior plans (V3, V4, V5, V6): **Complete**

### Assignment

Plan and execute **SR-PLAN-V7: MVP Stabilization & Evidence Foundation**.

**Goal:** Validate the completed MVP, then add the most architecturally significant extension (Evidence Upload) which is core to the semantic oracle model.

### First Action

Read `docs/planning/SR-PLAN-V7.md` for the detailed implementation plan and requirements.

### Reference Documents

- `docs/planning/SR-PLAN-V7.md` — Implementation plan (to be created by human authority)
- `docs/platform/SR-SPEC.md` — Platform mechanics
- `docs/platform/SR-CONTRACT.md` — Binding invariants (especially C-EV-* for evidence)
- `docs/platform/SR-SEMANTIC-ORACLE-SPEC.md` — Semantic oracle interface


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
| V6-2: Frontend | ⏳ Pending | Wire wizard to call `/start` after creation |
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

## Next Instance Prompt: Implement SR-PLAN-V6 Phase V6-2 (Frontend)

### Context

Phase V6-1 (Backend) is complete. The `POST /work-surfaces/{id}/start` endpoint is implemented and documented. Now the frontend wizard needs to call this endpoint after Work Surface creation.

### Current State

- Branch: `solver-ralph-7`
- Backend: `/work-surfaces/{id}/start` endpoint fully implemented
- Frontend: Wizard creates Work Surface but does not start work automatically
- Gap: User must manually start work after wizard completes

### Assignment

Implement **Phase V6-2: Frontend — Wire Wizard to Start Endpoint** as specified in `docs/planning/SR-PLAN-V6.md` §4.

This phase updates the React wizard to call `POST /work-surfaces/{id}/start` after successful Work Surface creation, completing the one-click workflow.

### Files to Modify

Per SR-PLAN-V6 §4 (Phase V6-2):

| File | Action | Description |
|------|--------|-------------|
| `ui/src/lib/api.ts` | EDIT | Add `startWorkSurface(id)` API function |
| `ui/src/components/WorkSurfaceWizard.tsx` | EDIT | Call start after successful bind |

### Implementation Requirements

Per SR-PLAN-V6 §4:

1. **API Client Function**
   ```typescript
   export async function startWorkSurface(workSurfaceId: string): Promise<StartWorkResponse> {
     const response = await fetch(`${API_BASE}/work-surfaces/${workSurfaceId}/start`, {
       method: 'POST',
       headers: getAuthHeaders(),
     });
     if (!response.ok) throw new ApiError(response);
     return response.json();
   }
   ```

2. **Wizard Integration**
   - After successful `bindWorkSurface()` call, immediately call `startWorkSurface()`
   - Handle the response to show `loop_id` and `iteration_id` to user
   - Handle `already_started: true` case gracefully (not an error)
   - Handle 412 error if Work Surface is not active

3. **UX Considerations**
   - Show loading state during start
   - On success: display confirmation with loop/iteration IDs
   - On `already_started`: show "Work already in progress" message
   - On error: show actionable error message

### Acceptance Criteria

From SR-PLAN-V6 §4 (Phase V6-2):

- [ ] `startWorkSurface` API function added to `api.ts`
- [ ] Wizard calls `/start` after successful Work Surface creation
- [ ] Success shows loop_id and iteration_id to user
- [ ] `already_started: true` handled gracefully
- [ ] Error states displayed appropriately
- [ ] No breaking changes to existing wizard flow

### Reference Documents

- `docs/planning/SR-PLAN-V6.md` — Implementation plan
- `docs/platform/SR-SPEC.md` §2.3.12 — Work Surface API documentation
- `docs/platform/SR-WORK-SURFACE.md` §5.5 — /start endpoint semantics

### Reference Code

- `ui/src/lib/api.ts` — Existing API client patterns
- `ui/src/components/WorkSurfaceWizard.tsx` — Current wizard implementation
- `crates/sr-api/src/handlers/work_surfaces.rs` — Backend response structure

### Guidelines

- Follow existing React/TypeScript patterns in the UI codebase
- Use existing error handling and loading state patterns
- Test the full wizard flow end-to-end after changes
- Run `npm run build` and `npm run lint` to verify

### Phase Complete When

- [ ] `startWorkSurface` API function implemented
- [ ] Wizard integration complete
- [ ] All acceptance criteria met
- [ ] `npm run build` passes
- [ ] `npm run lint` passes
- [ ] Manual test of wizard flow succeeds
- [ ] Committed and pushed


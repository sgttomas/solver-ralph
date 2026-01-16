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
| SR-PLAN-V5 | **in progress** | Semantic Ralph Loop End-to-End Integration (Phases 5a-5d) |
| SR-PLAN-V4 | **complete** | Work Surface Composition (Phase 4) — All phases complete |
| SR-PLAN-V3 | **complete** | Intakes & References implementation (Phases 0-3) |
| SR-PLAN-V2 | superseded | Intakes & References draft (10 unresolved issues) |

---

## SR-PLAN-V5 Implementation Status

**Status: IN PROGRESS**

SR-PLAN-V5 (Semantic Ralph Loop End-to-End Integration) connects the infrastructure from V3+V4 into a functioning end-to-end workflow, completing the MVP per SR-CHARTER §Immediate Objective.

### Phase Overview

| Phase | Status | Description |
|-------|--------|-------------|
| Phase 5a | **Complete** | Stage Advancement UI — "Complete Stage" button in WorkSurfaceDetail |
| Phase 5b | **Complete** | Loop ↔ Work Surface Binding — Loops inherit context automatically |
| Phase 5c | Pending | Approval-Gated Stages — Trust boundaries enforced via portal approvals |
| Phase 5d | Pending | End-to-End Integration Test — Prove the complete workflow |

### Key Design Decisions (Resolved in SR-PLAN-V5)

| Question | Resolution |
|----------|------------|
| Approval requirement location | In Procedure Template stage definitions via `requires_approval` field |
| Approval enforcement | Stage completion checks for recorded approval at `portal:stage-gate:<stage_id>` |
| Loop-Work Surface binding | Loop creation validates Work Surface exists; iterations auto-inherit context |
| Trust boundary stages | SEMANTIC_EVAL and FINAL stages require approval in baseline template |

### Planned Deliverables

```
ui/src/components/StageCompletionForm.tsx  — Stage completion form
ui/src/components/EvidenceBundleSelector.tsx — Evidence picker
ui/src/pages/WorkSurfaceDetail.tsx — Add stage completion UI
crates/sr-api/src/handlers/loops.rs — Work Surface validation
crates/sr-api/src/handlers/work_surfaces.rs — Approval check
crates/sr-api/tests/integration/semantic_ralph_loop_e2e.rs — E2E test
```

---

## Summary of Previous Development Iterations

### Session: 2026-01-15 — SR-PLAN-V5 Planning & Canonical Doc Updates

**Objective:** Plan the Semantic Ralph Loop end-to-end integration (MVP completion) and update canonical specifications.

**Work Performed:**

1. **Gap Analysis**
   - Reviewed SR-CHARTER §Immediate Objective (MVP requirements)
   - Identified three integration gaps: Stage Advancement UI, Loop-Work Surface Binding, Approval-Gated Stages
   - Explored codebase: loops.rs, iterations.rs, work_surfaces.rs, approvals.rs, governor.rs

2. **SR-PLAN-V5 Created** (`docs/planning/SR-PLAN-V5.md`)
   - Phase 5a: Stage Advancement UI — "Complete Stage" button in WorkSurfaceDetail
   - Phase 5b: Loop ↔ Work Surface Binding — Loops inherit context automatically
   - Phase 5c: Approval-Gated Stages — Trust boundaries enforced via portal approvals
   - Phase 5d: End-to-End Integration Test — Prove complete workflow

3. **Canonical Specification Updates**
   - **SR-PROCEDURE-KIT §1:** Added `requires_approval` field to stage schema
   - **SR-PROCEDURE-KIT §2:** Updated baseline template with approval requirements:
     - FRAME, OPTIONS, DRAFT: `requires_approval: false`
     - SEMANTIC_EVAL, FINAL: `requires_approval: true` (trust boundaries)
   - **SR-CHANGE §0.4:** Logged specification change (G:MINOR classification)

**Files Created/Modified:**

| File | Action | Description |
|------|--------|-------------|
| `docs/planning/SR-PLAN-V5.md` | CREATE | Comprehensive E2E integration plan |
| `docs/platform/SR-PROCEDURE-KIT.md` | EDIT | Added `requires_approval` field |
| `docs/build-governance/SR-CHANGE.md` | EDIT | Added version 0.4 entry |
| `docs/charter/SR-README.md` | EDIT | Added SR-PLAN-V5 status section |

**No code was modified.** This was a planning and specification session.

**Next Step:** Implement Phase 5a (Stage Advancement UI) per SR-PLAN-V5.

---

### Session: 2026-01-16 (Interrupted) — Phase 5a Implementation

**Objective:** Implement Phase 5a (Stage Advancement UI) per SR-PLAN-V5.

**What Occurred (Reconstructed):**

The previous instance successfully implemented all Phase 5a deliverables but was interrupted before committing the work. Based on examination of the uncommitted files:

1. **Created `EvidenceBundleSelector.tsx`** (179 lines)
   - Fetches evidence bundles from `GET /api/v1/evidence?limit=20`
   - Dropdown showing truncated hash, verdict, and relative time
   - Manual hash entry fallback when API fails or user prefers
   - Proper loading and error states

2. **Created `StageCompletionForm.tsx`** (374 lines)
   - Complete form per SR-PLAN-V5 §3.2-3.7
   - Evidence bundle selector integration
   - Gate result radio buttons (PASS, PASS_WITH_WAIVERS, FAIL)
   - Conditional waiver refs field (required for PASS_WITH_WAIVERS)
   - Oracle results pre-populated from `current_oracle_suites`
   - Client-side validation per §3.6
   - API call to `POST /api/v1/work-surfaces/:id/stages/:stage_id/complete`
   - Error handling for 400, 409, 412 per §3.7
   - Success message with next stage info

3. **Modified `WorkSurfaceDetail.tsx`**
   - Added `showCompletionForm` state
   - "Complete Stage" button visible when `status === 'active' && stage.status === 'entered'`
   - StageCompletionForm rendered conditionally
   - `onComplete` callback refreshes data and hides form

**Session ended before:** `git add && commit && push`

**Files left uncommitted:**
- `ui/src/components/EvidenceBundleSelector.tsx` (new)
- `ui/src/components/StageCompletionForm.tsx` (new)
- `ui/src/pages/WorkSurfaceDetail.tsx` (modified)

---

### Session: 2026-01-16 — Phase 5a Verification & Commit

**Objective:** Verify and commit the Phase 5a implementation left by the interrupted session.

**Work Performed:**

1. **Code Review**
   - Examined all three files against SR-PLAN-V5 §3.9 acceptance criteria
   - Confirmed all criteria satisfied

2. **Verification**
   - `npm run type-check` — Passed
   - `npm run build` — Passed (warning about chunk size, acceptable)

3. **Commit & Push**
   - `1ba5275` — "Implement Phase 5a: Stage Advancement UI"
   - `b41d643` — "Mark Phase 5a as complete in SR-README"

**Files Committed:**

| File | Action | Lines |
|------|--------|-------|
| `ui/src/components/EvidenceBundleSelector.tsx` | CREATE | 179 |
| `ui/src/components/StageCompletionForm.tsx` | CREATE | 374 |
| `ui/src/pages/WorkSurfaceDetail.tsx` | EDIT | +49 |

**Next Step:** Implement Phase 5c (Approval-Gated Stages) per SR-PLAN-V5.

---

### Session: 2026-01-16 — Phase 5b Implementation & Canonical Doc Updates

**Objective:** Implement Phase 5b (Loop ↔ Work Surface Binding) and update canonical specifications.

**Work Performed:**

1. **Database Migration**
   - Created `migrations/007_loops_work_surface_id.sql` — Adds `work_surface_id` column to `proj.loops` with FK constraint

2. **Backend Implementation**
   - **`crates/sr-adapters/src/projections.rs`** — Added `work_surface_id` to `LoopProjection` struct, `apply_loop_created`, `get_loop`, and `list_loops`
   - **`crates/sr-api/src/handlers/loops.rs`**:
     - Added Work Surface validation when `work_unit` is explicitly provided
     - Returns 412 `WORK_SURFACE_MISSING` if no active Work Surface found
     - Added `work_surface_id` to `LoopCreated` event payload
     - Added `work_surface_id` to `LoopResponse` struct
   - **`crates/sr-api/src/handlers/iterations.rs`** — Auto-populates `work_unit_id` from Loop's `work_unit` when Loop has bound Work Surface

3. **Frontend Implementation**
   - **`ui/src/hooks/useLoops.ts`** — Added `work_surface_id` to `LoopListItem` interface and data transformation
   - **`ui/src/pages/Loops.tsx`** — Added "Work Surface" column with link to Work Surface detail page
   - **`ui/src/pages/LoopDetail.tsx`** — Enhanced Work Surface card to show `work_surface_id` prominently with link

4. **Canonical Specification Updates**
   - **SR-SPEC §2.3.1:** Documented `work_unit` validation and `work_surface_id` binding semantics
   - **SR-SPEC Appendix A:** Added `LoopCreated` payload schema (v1) with `work_surface_id`
   - **SR-TYPES §7.1:** Added `work_unit` and `work_surface_id` to Work Unit key fields
   - **SR-WORK-SURFACE §5.4:** Added new section "Loop ↔ Work Surface binding (normative)"
   - **SR-CHANGE §0.5:** Logged all specification changes (G:MINOR classification)

5. **Verification**
   - `cargo build` — Passed
   - `cargo test --workspace` — All 27 tests passed
   - `npm run type-check` — Passed
   - `npm run build` — Passed

**Files Created/Modified:**

| File | Action | Description |
|------|--------|-------------|
| `migrations/007_loops_work_surface_id.sql` | CREATE | Database migration |
| `crates/sr-adapters/src/projections.rs` | EDIT | Add work_surface_id projection |
| `crates/sr-api/src/handlers/loops.rs` | EDIT | Work Surface validation + response |
| `crates/sr-api/src/handlers/iterations.rs` | EDIT | Auto-inherit work_unit_id |
| `ui/src/hooks/useLoops.ts` | EDIT | Add work_surface_id to types |
| `ui/src/pages/Loops.tsx` | EDIT | Add Work Surface column |
| `ui/src/pages/LoopDetail.tsx` | EDIT | Enhance Work Surface display |
| `docs/platform/SR-SPEC.md` | EDIT | Document Loop-Work Surface binding |
| `docs/platform/SR-TYPES.md` | EDIT | Add work_surface_id field |
| `docs/platform/SR-WORK-SURFACE.md` | EDIT | Add §5.4 binding semantics |
| `docs/build-governance/SR-CHANGE.md` | EDIT | Add version 0.5 entry |

**Acceptance Criteria (all met):**
- [x] Loop creation with work_unit fails if no active Work Surface exists
- [x] LoopCreated event includes work_surface_id when bound
- [x] Iterations automatically receive Work Surface context from Loop
- [x] Loops list shows bound Work Surface
- [x] LoopDetail shows Work Surface binding prominently

---


# Next Instance Prompt: SR-PLAN-V5 Phase 5c Implementation

## Your Assignment

You are continuing work on the **Semantic Ralph Loop MVP** project. Phases 5a (Stage Advancement UI) and 5b (Loop ↔ Work Surface Binding) are complete. Your task is to **implement Phase 5c: Approval-Gated Stages** as specified in `docs/planning/SR-PLAN-V5.md` §5.

### Key Documents to Read
1. **`docs/planning/SR-PLAN-V5.md`** — The implementation plan (especially §5 Phase 5c)
2. **`docs/charter/SR-README.md`** — Assignment orientation and canonical document index
3. **`docs/platform/SR-CONTRACT.md`** — Binding invariants (especially C-TB-3: portal crossings produce approvals)
4. **`docs/platform/SR-PROCEDURE-KIT.md`** — Procedure template definitions with `requires_approval` field

### What You Need to Implement (Phase 5c)

**Goal:** Enforce approval requirements at trust-boundary stages. Stages with `requires_approval: true` (SEMANTIC_EVAL and FINAL in the baseline template) must have a recorded approval before completion is allowed.

**Key Implementation Details:**
- Stage completion endpoint must check if the stage has `requires_approval: true`
- If approval required, verify an approval exists at `portal:stage-gate:<stage_id>` for this work surface
- Return 412 `APPROVAL_REQUIRED` if no approval found
- The approval must reference the Work Surface and stage being completed

**Deliverables (from SR-PLAN-V5 §5):**

| File | Action | Description |
|------|--------|-------------|
| `crates/sr-api/src/handlers/work_surfaces.rs` | EDIT | Check for approval before stage completion |
| `crates/sr-api/src/handlers/approvals.rs` | EDIT | Support stage-gate portal approvals |
| `ui/src/pages/WorkSurfaceDetail.tsx` | EDIT | Show approval status for current stage |
| `ui/src/components/StageApprovalForm.tsx` | CREATE | Form to create stage-gate approval |

**Acceptance Criteria (from SR-PLAN-V5 §5.6):**
- [ ] Stage completion fails with 412 if requires_approval=true and no approval exists
- [ ] Approval can be created via `POST /approvals` with `portal_id: portal:stage-gate:<stage_id>`
- [ ] UI shows whether current stage requires approval
- [ ] UI provides form to create stage-gate approval when needed
- [ ] Complete workflow: create approval → complete stage succeeds

### Existing Code to Reference
- **Work Surface Handler:** `crates/sr-api/src/handlers/work_surfaces.rs` (see `complete_stage` function)
- **Approvals Handler:** `crates/sr-api/src/handlers/approvals.rs`
- **Procedure Kit:** `docs/platform/SR-PROCEDURE-KIT.md` (see `requires_approval` field)
- **Stage Completion Form:** `ui/src/components/StageCompletionForm.tsx`

### Database/API Notes
- Approvals are stored via `ApprovalRecorded` events
- Query approvals by portal_id and subject_refs to check if approval exists
- The approval's `subject_refs` should include the Work Surface ID and stage ID

## Begin Implementation
Start by reading `docs/planning/SR-PLAN-V5.md` §5 (Phase 5c) in full, then examine `crates/sr-api/src/handlers/work_surfaces.rs` to understand the current `complete_stage` flow and where approval checking should be added.


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
| Phase 5b | Pending | Loop ↔ Work Surface Binding — Loops inherit context automatically |
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

**Next Step:** Implement Phase 5b (Loop ↔ Work Surface Binding) per SR-PLAN-V5.

---


# Next Instance Prompt: SR-PLAN-V5 Phase 5b Implementation

## Your Assignment

You are continuing work on the **Semantic Ralph Loop MVP** project. Phase 5a (Stage Advancement UI) is complete. Your task is to **implement Phase 5b: Loop ↔ Work Surface Binding** as specified in `docs/planning/SR-PLAN-V5.md` §4.

### Key Documents to Read
1. **`docs/planning/SR-PLAN-V5.md`** — The implementation plan (especially §4 Phase 5b)
2. **`docs/charter/SR-README.md`** — Assignment orientation and canonical document index
3. **`docs/platform/SR-CONTRACT.md`** — Binding invariants

### What You Need to Implement (Phase 5b)

**Goal:** Ensure Loops are created with explicit Work Surface binding and inherit context automatically.

**Deliverables (from SR-PLAN-V5 §4.5):**

| File | Action | Description |
|------|--------|-------------|
| `crates/sr-api/src/handlers/loops.rs` | EDIT | Add Work Surface validation on loop creation |
| `crates/sr-api/src/handlers/iterations.rs` | EDIT | Auto-populate work_unit_id from Loop |
| `crates/sr-adapters/src/projections.rs` | EDIT | Project work_surface_id to loops table |
| `ui/src/pages/Loops.tsx` | EDIT | Add Work Surface column |
| `ui/src/pages/LoopDetail.tsx` | EDIT | Enhance Work Surface display |

**Key Implementation Details:**
- When `work_unit` is provided to loop creation, validate active Work Surface exists
- Return 412 PRECONDITION_FAILED with code `WORK_SURFACE_REQUIRED` if not found
- Add `work_surface_id` to LoopCreated event payload
- Auto-populate `work_unit_id` in iterations from Loop's stored work_unit
- UI: Show Work Surface column in Loops list, enhance LoopDetail display

**Acceptance Criteria (from SR-PLAN-V5 §4.6):**
- [ ] Loop creation with work_unit fails if no active Work Surface exists
- [ ] LoopCreated event includes work_surface_id when bound
- [ ] Iterations automatically receive Work Surface context from Loop
- [ ] Loops list shows bound Work Surface
- [ ] LoopDetail shows Work Surface binding prominently

### Existing Code to Reference
- **Loop Handler:** `crates/sr-api/src/handlers/loops.rs`
- **Iteration Handler:** `crates/sr-api/src/handlers/iterations.rs`
- **Projections:** `crates/sr-adapters/src/projections.rs`
- **Loop UI:** `ui/src/pages/Loops.tsx`, `ui/src/pages/LoopDetail.tsx`

### Database Schema Note
You may need to add a migration for `work_surface_id` column on `proj.loops` table:
```sql
ALTER TABLE proj.loops ADD COLUMN work_surface_id TEXT REFERENCES proj.work_surfaces(work_surface_id);
```

## Begin Implementation
Start by reading `docs/planning/SR-PLAN-V5.md` §4 (Phase 5b) in full, then examine `crates/sr-api/src/handlers/loops.rs` to understand the current loop creation flow.


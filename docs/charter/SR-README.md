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


# Next Instance Prompt: SR-PLAN-V5 Phase 5a Implementation

## Your Assignment

You are continuing work on the **Semantic Ralph Loop MVP** project. The planning and review phase is complete. Your task is to **implement Phase 5a: Stage Advancement UI** as specified in `docs/planning/SR-PLAN-V5.md`.

### Key Documents to Read
1. **`docs/planning/SR-PLAN-V5.md`** — The implementation plan (especially §3 Phase 5a)
2. **`docs/charter/SR-README.md`** — Assignment orientation and canonical document index
3. **`docs/platform/SR-CONTRACT.md`** — Binding invariants (C-EVID-*, C-TB-*, C-VER-*)

### What You Need to Implement (Phase 5a)

**Goal:** Add UI to complete stages with evidence from the WorkSurfaceDetail page.

**Deliverables (from SR-PLAN-V5 §3.8):**

**Key Implementation Details:**
- Form visible only when: Work Surface status is "active" AND current stage status is "entered"
- Pre-populate oracle IDs from `workSurface.current_oracle_suites`
- Gate result status options: PASS, PASS_WITH_WAIVERS, FAIL
- If PASS_WITH_WAIVERS selected, waiver refs field is required
- API endpoint: `POST /api/v1/work-surfaces/:work_surface_id/stages/:stage_id/complete`

**Acceptance Criteria (from SR-PLAN-V5 §3.9):**

### Existing Code to Reference
- **Backend API:** `crates/sr-api/src/handlers/work_surfaces.rs` — `complete_stage` endpoint exists
- **Evidence API:** `crates/sr-api/src/handlers/evidence.rs` — `GET /api/v1/evidence` for bundle list
- **Current UI:** `ui/src/pages/WorkSurfaceDetail.tsx` — where form will be integrated

### Implementation Order (from SR-PLAN-V5 §7)

### MVP Limitations (Accepted)
- Evidence bundle existence not validated by backend
- Waiver refs not validated to exist
- Oracle results are user-entered, not from actual oracle runs
- FAIL records evidence but does NOT advance the stage

## Begin Implementation
Start by reading `docs/planning/SR-PLAN-V5.md` §3 (Phase 5a) in full, then examine `ui/src/pages/WorkSurfaceDetail.tsx` to understand the current UI structure before creating the new components.


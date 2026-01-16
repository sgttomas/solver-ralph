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

Tasks are no longer assigned by SR-PLAN because the build out phase is complete.  See below for the Comprehensive Implementation Plan for the next phase of build out and implementation.

Begin your task assignment by reading SR-CHARTER.  The project documentation constitutes a total development plan and specification with detailed instructions on types and contracts.  Always read the SR-* files that appear related to the task before going to read the code files.  Documentation leads development for this project.  Documentation is how you know your ontology, epistemology, and semantics.

Once you have read the appropriate project docs, then do whatever deliverables and tasks you think should be done next.

Your task is done when there are no more deliverables to be assigned, which means all tests have passed and gates cleared.

You should push on to resolve any findings and consult the docs/ frequently for guidance and direction from the SR-* documents, several of which are normative.  They are typed documents and structured to aid in navigation.

You should git add && commit && push after completing each deliverable.  You can remain on the same branch throughout this development session.

If you cannot pass the tests for that deliverable then you must summarize what you did during that development session, delete the previous message where it says "Development History Summary for this Deliveralbe" and then append your new message including how to identify the task that was being worked on when the next instance of yourself begins the next iteration.

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

## SR-PLAN-V3 Implementation Status

**Status: COMPLETE**

All phases of the Intakes & References implementation (SR-PLAN-V3) are now complete.

| Phase | Status | Description |
|-------|--------|-------------|
| Phase 0a | **Complete** | Core Infrastructure — TypedRef module, Intake domain, database migrations, event definitions |
| Phase 0b | **Complete** | Intake API — Intake handler with CRUD + lifecycle operations |
| Phase 0c | **Complete** | References API — References browser backend (15 endpoints) |
| Phase 1 | **Complete** | UI Structure — Sidebar and route reorganization |
| Phase 2 | **Complete** | Intakes UI — Full Intake CRUD UI |
| Phase 3 | **Complete** | References UI — References browser UI |

### Current Routes

```
/intakes                              → Intakes.tsx (list with filters)
/intakes/new                          → IntakeCreate.tsx (create form)
/intakes/:intakeId                    → IntakeDetail.tsx (detail view)
/intakes/:intakeId/edit               → IntakeEdit.tsx (edit form)
/references                           → References.tsx (category sidebar)
/references/documents/:documentId     → ReferenceDocumentDetail.tsx
/references/bundles/:bundleId         → ReferenceBundleDetail.tsx
/references/governed-artifacts/:id    → GovernedArtifactDetail.tsx
```

---

## SR-PLAN-V4 Implementation Status

**Status: COMPLETE**

SR-PLAN-V4 (Work Surface Composition) is now fully implemented. All four phases are complete.

### Phase Overview

| Phase | Status | Description |
|-------|--------|-------------|
| Phase 4a | **Complete** | Core Infrastructure — WorkSurfaceId, events, database migrations |
| Phase 4b | **Complete** | Work Surface API — 9 endpoints for CRUD, stage transitions, compatibility |
| Phase 4c | **Complete** | Event Integration — IterationStarted refs, EvidenceBundleRecorded binding, Governor stop trigger |
| Phase 4d | **Complete** | Work Surface UI — List, composition wizard, detail with stage progress |

### Key Design Decisions (Resolved in SR-PLAN-V4)

| Question | Resolution |
|----------|------------|
| Compatibility checking | Intake `kind` must match Procedure Template's `kind[]`; binding fails otherwise |
| Stage initialization | New Work Surface starts at template's initial stage; `StageEntered` emitted immediately |
| Oracle suite binding | Per-stage binding (not once for whole Work Surface); resolved dynamically on stage entry |
| Immutability | Binding refs immutable; only stage progression mutable via events |
| Relationship to Loops | Work Surface 1:1 with Work Unit (not Loop); multiple Loops share same Work Surface |
| UI workflow | Step-by-step wizard (Select Intake → Select Template → Review & Confirm) |

### Implemented Routes

```
/work-surfaces                        → WorkSurfaces.tsx (list with filters)
/work-surfaces/new                    → WorkSurfaceCompose.tsx (composition wizard)
/work-surfaces/:workSurfaceId         → WorkSurfaceDetail.tsx (detail with stage progress)
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

## SR-PLAN-V5 Implementation Status

**Status: IN PROGRESS**

SR-PLAN-V5 (Semantic Ralph Loop End-to-End Integration) connects the infrastructure from V3+V4 into a functioning end-to-end workflow, completing the MVP per SR-CHARTER §Immediate Objective.

### Phase Overview

| Phase | Status | Description |
|-------|--------|-------------|
| Phase 5a | Pending | Stage Advancement UI — "Complete Stage" button in WorkSurfaceDetail |
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

## Prompt for Next Instance

Evaluate the proposed implementation plan SR-PLAN-V5 Phase 5a (Stage Advancement UI). See `docs/planning/SR-PLAN-V5.md` for full specification.  Is this plan complete for the intended purpose?  Do you agree with the plan? Explain your rationale.  To do this properly you need to understand the canonical documents in the context of this SR-PLAN-V5.  Review the list of canonical documents and read thoroughly any applicable documents.  Review the codebase and read any pertinent files.  Then produce your report of findings and any recommendations to mitigate errors and omissions.

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
| SR-SEMANTIC-ORACLE-SPEC | `platform/` | Semantic oracle interface |
| SR-EVENT-MANAGER | `platform/` | Event projections spec |
| SR-REPLAY-PROOF | `platform/` | Determinism proof (C-EVT-7) |
| SR-DEPLOYMENT | `platform/` | Deployment guide |
| SR-OBSERVABILITY | `platform/` | Observability reference |
| SR-AGENT-WORKER-CONTRACT | `platform/` | Agent/worker contract |
| SR-INTENT | `platform/` | Design rationale |
| SR-CHANGE | `build-governance/` | Change control process |
| SR-AGENTS | `build-governance/` | Agent actor model |
| SR-TASKS | `build-governance/` | Task assignment |
| SR-EXCEPTIONS | `build-governance/` | Exception ledger |
| SR-PLAN | `program/` | Build plan instance |
| SR-DIRECTIVE | `program/` | Execution policy |
| SR-TEMPLATES | `platform/` | Template definitions (merged with former SR-PROCEDURE-KIT) |
| SR-README | `charter/` | This index |


## Current Assignment: UI Simplification — Workflow-Centric Single Screen

### Constraint: Frontend Only

**This is entirely frontend development.** Do NOT modify:
- Any Rust code (`crates/*`)
- Database migrations
- API endpoints

If you believe backend changes are needed, **stop and explain why** — do not proceed without explicit approval.

### The Problem

The current UI was built from a **data-centric perspective** — it exposes the underlying data model (Loops, Iterations, Candidates, Evidence Bundles, Approvals) as separate browsable screens. Users must mentally stitch together the workflow by jumping across 17 sidebar items and 36 pages.

MVP1 added `WorkScreen` at `/work/:workSurfaceId` as a **workflow-centric** unified view, but:
1. **It's not in the sidebar** — users can't even navigate to it
2. **17 nav items remain** — exposing the data model, not the workflow
3. **Evidence bundles are still browsable by hash** — but users should never need to "find" evidence; it appears automatically in context when they make a judgment

### Data-Centric vs. Workflow-Centric

| Data-Centric (Current) | Workflow-Centric (Target) |
|------------------------|---------------------------|
| "Here are your Loops" | "Here is your work" |
| "Here are your Evidence Bundles" | Evidence appears when you need to decide |
| Navigate between entity types | Follow the work |
| 17 sidebar items = 17 piles of data | 3-5 sidebar items = stages/actions |
| User stitches workflow together mentally | UI guides user through workflow |

### The Core Workflow

```
1. Define Work    → Create Intake, bind to Template = Work Surface
2. Run Work       → Loop executes iterations (automatic, no UI needed)
3. Review & Decide → Human sees candidate + evidence, judges
4. Done           → Work surface complete
```

**Key insights:**
- **Loops are invisible** — Users manage Work Surfaces, not Loops. Loop state appears *within* WorkScreen.
- **Evidence is contextual** — It's produced by the system and consumed where decisions are made. Users never browse evidence by hash.
- **One primary screen** — WorkScreen is where work happens. Everything else feeds into it or is removed.

### Target UI Structure

```
Work Surfaces (list) → Click one → WorkScreen (everything happens here)
                                        ↓
                              - Intake context
                              - Template expectations
                              - Loop status (running/stopped/budget)
                              - Candidate output
                              - Evidence (auto-loaded, not selected)
                              - Judgment actions (Approve/Reject)
```

### Target Sidebar (~3-5 items)

```
Work Surfaces    ← List all work; click opens WorkScreen
New Work         ← Create intake + work surface (wizard or modal)
Settings         ← System config
```

Everything else should either:
- Be accessible *within* WorkScreen as contextual detail
- Be removed entirely

### Your Task

1. **Audit** `ui/src/pages/` (36 files) — for each page determine:
   - Is it data-centric (exposes entity type) or workflow-centric (guides user action)?
   - Can its function be accessed through WorkScreen?
   - Is it actually used, or leftover from development?
   - Does the user need direct nav, or is it a detail view reached via link?

2. **Create** `docs/planning/SR-PLAN-MVP2.md` with:
   - Pages to DELETE (with rationale)
   - Pages to KEEP (with rationale)
   - New simplified sidebar
   - Changes needed to WorkScreen (if any) to absorb functionality
   - How WorkScreen becomes accessible (it's not in sidebar currently!)

3. **Do NOT execute** — create the plan for review

### Key Files

| File | Purpose |
|------|---------|
| `ui/src/routes.tsx` | All 30+ routes |
| `ui/src/layout/Sidebar.tsx` | 17-item navigation |
| `ui/src/pages/WorkScreen.tsx` | Unified view (NOT in sidebar!) |
| `ui/src/pages/*.tsx` | 36 page components to audit |
| `docs/planning/SR-PLAN-MVP1.md` | Context on WorkScreen design |
| `docs/charter/SR-CHARTER.md` | Core workflow definition |

### Guiding Questions

For each page, ask:
1. Does this expose data or guide workflow?
2. Can WorkScreen provide this functionality in context?
3. Would a user ever navigate here directly, or only via a link from elsewhere?
4. If removed, what breaks? What improves?

---

## Previous Assignment: MVP1 (Complete)

**Status:** MVP1 implementation and cleanup complete. See `docs/planning/SR-PLAN-MVP1.md` for full progress log.

| Task | Description | Status |
|------|-------------|--------|
| **A1-A8** | Nomenclature refactor (`ProcedureTemplate` → `Template`) | ✅ Complete |
| **B1-B6** | Fresh UI build (WorkScreen) | ✅ Complete |
| **A7** | TypeScript interface renames | ✅ Complete |
| **B7** | Legacy component removal | ✅ Complete |

---

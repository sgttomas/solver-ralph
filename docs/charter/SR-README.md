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
| SR-TEMPLATES | `platform/` | User configuration registry |
| SR-README | `charter/` | This index |


## Current Assignment: MVP Simplification — Planning Phase

> **Assignment:** Generate detailed implementation plans for simplifying the MVP to provide a user-friendly workflow where humans review agent work at checkpoints and decide whether to proceed.

### Core Intent

The target user experience:
1. **Create** — Define what work to do (Work Surface)
2. **Work** — Agent runs iterations autonomously
3. **Checkpoint** — When stopped or stage complete, user sees summary and decides
4. **Done** — Release approval when complete

The user should **never** have to manually select evidence bundles from a list of hashes.

---

### Task: Generate Implementation Plans

Create detailed implementation plans in `docs/planning/` for each of the following deliverables. Each plan should specify:

- **Files to modify** (with specific functions/components)
- **New files to create** (migrations, endpoints, components)
- **Data flow** (how information moves through the system)
- **API contracts** (request/response shapes)
- **UI wireframes** (text-based description of the interface)
- **Test cases** (what to verify)
- **Migration/backfill strategy** (for existing data)

---

### Deliverables to Plan

#### MVP-1: Auto-Link Evidence to Work Surface/Stage

**Problem:** Evidence bundles are tracked by `run_id`/`candidate_id` but not by `work_surface_id`/`stage_id`, forcing users to manually select from cryptic hashes.

**Plan should cover:**
- Database migration for new columns
- Projection handler changes to populate them
- API endpoint to query evidence by work surface + stage
- Backfill strategy for existing evidence bundles
- How evidence gets associated during iteration execution

---

#### MVP-2: Unified Work Surface View

**Problem:** Users bounce between Work Surfaces page (context) and Loops page (state).

**Plan should cover:**
- What loop data to embed in Work Surface detail view
- Decision UI component design
- API changes needed (or existing endpoints to use)
- Which Loops page features to keep vs. remove
- Navigation/routing changes

---

#### MVP-3: Simplified Stage Completion

**Problem:** Stage completion form requires manual evidence selection, gate result radio buttons, oracle result entry.

**Plan should cover:**
- New stage completion component design
- Auto-fetch evidence logic
- Oracle result summary display format
- Button actions and their API calls
- Waiver flow (when failures exist)
- What to do when no evidence exists for the stage

---

### Output

Create the following files:

1. `docs/planning/MVP-1-EVIDENCE-LINKING.md` — Plan for auto-linking evidence
2. `docs/planning/MVP-2-UNIFIED-VIEW.md` — Plan for unified Work Surface view
3. `docs/planning/MVP-3-SIMPLE-COMPLETION.md` — Plan for simplified stage completion

Each plan should be detailed enough that implementation can proceed without ambiguity.

### Success Criteria

1. Plans are complete and unambiguous
2. Plans identify all files that need modification
3. Plans include test cases
4. Plans address backward compatibility with existing data
5. Plans are reviewed and approved before implementation begins

---

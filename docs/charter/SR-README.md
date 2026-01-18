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


## Current Assignment: Execute SR-PLAN-MVP1

**Assignment:** Execute the implementation plan at `docs/planning/SR-PLAN-MVP1.md`.

### Quick Start

1. **Read the plan:** `docs/planning/SR-PLAN-MVP1.md`
2. **Start at:** Task A1 (Documentation)
3. **Follow:** The execution order diagram in the plan
4. **Verify:** Each task has verification commands — run them before moving on
5. **Gate:** Complete the Part A Completion Gate before starting Part B

### What You're Implementing

| Part | Description | Key Outcome |
|------|-------------|-------------|
| **Part A** | Nomenclature Refactor | `ProcedureTemplate` → `Template`, remove `GateRule` |
| **Part B** | Fresh UI Build | New WorkScreen with auto-linked evidence |

### Critical Path

```
A1 → A2 → A3 → A4/A5 → A5.5 → A6 → A7 → A8 → [Part A Gate] → B1 → B2 → B3 → B4 → B5 → B6 → B7
```

### Key Documents to Reference

| Document | When to Reference |
|----------|-------------------|
| `docs/planning/SR-PLAN-MVP1.md` | Primary task list and verification steps |
| `docs/platform/SR-TEMPLATES.md` | Template schema (update in A1) |
| `docs/platform/SR-WORK-SURFACE.md` | Work surface definitions |
| `docs/platform/SR-CONTRACT.md` | Binding invariants |

### Success Criteria

**Part A Complete When:**
- All `grep` searches for old terminology return 0 results
- `SR-PROCEDURE-KIT.md` deleted
- All tests pass (`cargo test --workspace`)
- UI compiles and typechecks
- API responds with new field names

**Part B Complete When:**
- WorkScreen renders at `/work/{id}`
- Evidence loads automatically (no hash selection)
- Approve/Reject/Waive actions functional
- End-to-end test passes

---

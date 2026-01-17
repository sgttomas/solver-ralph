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


---


## Current Status

**V10-1 through V10-4:** ✅ VERIFIED (2026-01-17)
**V10-5, V10-6:** Pending
**Branch:** `solver-ralph-10`

Tests 9, 12-15 re-run and verified passing. See `docs/planning/SR-PLAN-LOOPS.md` for detailed verification results.

See `docs/planning/SR-PLAN-GAP-ANALYSIS.md §4` for V10 implementation details and status.

---

## Next Instance Prompt

> **Assignment:** Verify V10 implementation, then complete V10-5/V10-6 if time permits.

### Orientation

Read these documents in order:

1. `docs/planning/SR-PLAN-GAP-ANALYSIS.md §4` — V10 scope, what's done (V10-1 through V10-4), what remains (V10-5, V10-6)
2. `docs/planning/SR-PLAN-LOOPS.md` — Implementation Status Update section shows which tests should now pass; re-run Tests 9, 12-15 to verify

### Deliverables

1. **Verify:** Re-run SR-PLAN-LOOPS Tests 9, 12-15 against the implementation
2. **Implement (if time):** V10-5 (Loop PATCH endpoint) per SR-PLAN-GAP-ANALYSIS
3. **Update:** SR-PLAN-LOOPS verification results after testing

### Constraints

- Migration 009 is already in place — do not duplicate schema changes
- V10-1 through V10-4 are complete — do not re-implement

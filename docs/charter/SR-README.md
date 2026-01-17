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


## Current Status: Loop Validation Plan Ready for Execution

**Branch 0 Acceptance:** COMPLETE (V9-4)
**Consistency Evaluation:** APPROVED WITH NOTES (see `docs/reviews/SR-PLAN-LOOPS-CONSISTENCY-EVALUATION.md`)
**Current Focus:** Execute SR-PLAN-LOOPS validation

### Recent Development

The Loop Functionality Validation Plan (`docs/planning/SR-PLAN-LOOPS.md`) has been:
1. Revised to address coherence gaps
2. Evaluated for consistency against canonical SR-* documents
3. **Approved for execution** with minor notes documented

---

## Next Instance Prompt: Execute SR-PLAN-LOOPS Validation

> **Session Type:** Manual validation (UI + API + database verification)
> **Estimated Effort:** 1-2 sessions
> **Prerequisite:** Infrastructure running (`make deploy`), API running, UI running

### Assignment

**Execute the Loop Functionality Validation Plan** — Run all 19 tests in `docs/planning/SR-PLAN-LOOPS.md` and document results.

### Orientation

1. **Read first:**
   - `docs/charter/SR-CHARTER.md` — Project scope and authority model
   - `docs/planning/SR-PLAN-LOOPS.md` — The validation plan to execute (contains all test steps)
   - `docs/reviews/SR-PLAN-LOOPS-CONSISTENCY-EVALUATION.md` — Evaluation notes for execution awareness

2. **Execution notes from evaluation:**
   - ORACLE_TAMPER is not tested (same category as ORACLE_GAP/EVIDENCE_MISSING)
   - Test 9 refs[] categories are the correct minimum per SR-DIRECTIVE §3.1
   - Verify `suite_hash` presence in Work Surface binding during Test 11

3. **Infrastructure prerequisites** (per SR-PLAN-LOOPS §Prerequisites):
   - `make deploy` for infrastructure
   - `SR_OIDC_SKIP_VALIDATION=true cargo run --bin sr-api` for API
   - `make dev-ui` for UI on port 3001
   - Database access for event verification

### Deliverable

Update the verification checklists in SR-PLAN-LOOPS with Pass/Fail results, and document any gaps discovered in the Gap Tracking table.

### Do NOT

- Re-evaluate the plan (already done)
- Skip tests without documenting why
- Assume features work without verification

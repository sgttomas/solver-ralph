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


## Current Status: V10 Implementation Ready

**Branch 0 Acceptance:** COMPLETE (V9-4)
**Loop Validation:** COMPLETE (2026-01-17) — 11 PASS, 6 PARTIAL/GAP, 2 DEFERRED
**Current Focus:** Implement V10 gaps (Loop Governor Completion)

### Recent Development

SR-PLAN-LOOPS validation completed with results documented:
- **Critical gaps identified:** Stop triggers (BUDGET_EXHAUSTED, REPEATED_FAILURE) not implemented
- **V10 scope refined:** 6 phases addressing validation findings
- **V11 scope updated:** Deferred items (integrity E2E, GovernedArtifact refs)

See `docs/planning/SR-PLAN-GAP-ANALYSIS.md §4` for detailed V10 scope.

---

## Next Instance Prompt: Implement V10 (Loop Governor Completion)

> **Session Type:** Implementation
> **Scope:** D-22, D-12, D-18, D-24 (per SR-PLAN-GAP-ANALYSIS §4)
> **Branch:** `solver-ralph-loops` (continue on current branch)

### Assignment

**Implement V10 phases** — Address critical and high-priority gaps from SR-PLAN-LOOPS validation, starting with stop triggers.

### Orientation

1. **Read first:**
   - `docs/planning/SR-PLAN-GAP-ANALYSIS.md §4` — V10 scope, phases, and gap descriptions (V10-G1 through V10-G7)
   - `docs/planning/SR-PLAN-LOOPS.md` — Verification checklists show what failed and why
   - `docs/platform/SR-CONTRACT.md` — C-LOOP-1, C-LOOP-3 (budget enforcement), C-LOOP-4 (traceability)

2. **Implementation priority (from SR-PLAN-GAP-ANALYSIS):**
   - V10-1: Stop triggers — `crates/sr-api/src/handlers/work_surfaces.rs` (start_iteration_as_system)
   - V10-2: Decision-required resume — Loop projection + `/resume` endpoint
   - V10-3: Candidate traceability — `crates/sr-api/src/handlers/candidates.rs`
   - V10-4 through V10-6: Refs completeness, Loop PATCH, hash fix

3. **Key code locations:**
   - `crates/sr-api/src/handlers/work_surfaces.rs:1830` — iteration start logic
   - `crates/sr-api/src/handlers/loops.rs` — Loop state transitions
   - `crates/sr-adapters/src/projections.rs` — Loop projection (add consecutive_failures counter)

### Deliverable

- Implement V10-1 and V10-2 (stop triggers + decision gating) as minimum
- Re-run SR-PLAN-LOOPS Tests 13-15 to verify fix
- Update SR-PLAN-GAP-ANALYSIS with completion status

### Do NOT

- Re-run full validation (only re-test fixed items)
- Change V10 scope without documenting rationale
- Skip StopTriggered event emission (required for audit trail)

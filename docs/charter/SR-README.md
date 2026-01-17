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

**V10:** ✅ COMPLETE (2026-01-17)
**V11:** Planning phase
**Branch:** `solver-ralph-11`

All V10 phases verified and complete:
- V10-1 through V10-4: Loop Governor stop triggers, decision gating, traceability
- V10-5: Loop PATCH endpoint with budget monotonicity
- V10-6: OracleSuite hash prefix fix

See `docs/planning/SR-PLAN-LOOPS.md` for detailed verification results.
See `docs/build-governance/SR-CHANGE.md` v1.2 (implementation) and v1.3 (SR-SPEC updates).

---

## Next Instance Prompt

> **Assignment:** Review SR-PLAN-V11 for coherence with the codebase. Do NOT begin implementation until this review is complete.

### Context

SR-PLAN-V11 has been drafted but requires validation before implementation begins. Your task is to evaluate whether the plan is coherent with the actual codebase state.

### Orientation

1. Read `docs/planning/SR-PLAN-V11.md` — The plan to be reviewed
2. Read `docs/planning/SR-PLAN-GAP-ANALYSIS.md §4` — V11 scope source
3. Explore relevant codebase directories referenced in the plan

### Task: Review SR-PLAN-V11 for Coherence

Evaluate the plan against the actual codebase and report:

1. **File/Module Accuracy** — Do the files and modules referenced in the plan actually exist? Are the descriptions accurate?
2. **Dependency Feasibility** — Are the stated dependencies correct? Are there hidden dependencies not mentioned?
3. **Phase Ordering** — Is the proposed phase order logical given the codebase structure?
4. **Gap Coverage** — Does the plan adequately address all deferred V10 items and proposed V11 deliverables?
5. **Risk Assessment** — Are there any risks or blockers not identified in the plan?

### Deliverable

Provide a coherence report with:
- **Findings** — Issues discovered, if any
- **Recommendations** — Suggested changes to the plan
- **Verdict** — APPROVE (proceed to implementation) or REVISE (update plan first)

### Constraints

- **Review only** — Do not implement until review is complete and approved
- V10 is complete — do not modify V10 code
- Be thorough — explore the codebase to validate plan assumptions
- Focus on feasibility, not stylistic preferences

### Key Codebase Areas to Validate

- `crates/sr-e2e-harness/` — E2E testing infrastructure
- `crates/sr-oracles/` — Oracle implementations
- `crates/sr-adapters/src/` — Infisical, observability, integrity modules
- `scripts/` — Build and init scripts
- `docs/platform/` — Canonical SR-* documents

---

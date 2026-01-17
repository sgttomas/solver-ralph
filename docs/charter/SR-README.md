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

> **Assignment:** Plan V11 scope. Do NOT begin implementation until the plan is reviewed.

### Orientation

1. Read `docs/planning/SR-PLAN-GAP-ANALYSIS.md §4` — V11 proposed scope
2. Read `docs/planning/SR-PLAN-LOOPS.md` — Deferred items (Tests 17-18)
3. V11 focuses on **Production Hardening & E2E Testing**

### Task: Author SR-PLAN-V11

Create `docs/planning/SR-PLAN-V11.md` with:

1. **Scope Definition** — Which deliverables and gaps to address
2. **Phase Breakdown** — Ordered phases (V11-1, V11-2, etc.)
3. **Verification Plan** — How each phase will be tested
4. **Dependencies** — What infrastructure/tooling is needed

### V11 Proposed Scope (from SR-PLAN-GAP-ANALYSIS)

| Deliverable | Description | Priority |
|-------------|-------------|----------|
| D-16 | Restricted evidence handling (Infisical envelope keys) | High |
| D-26 | Integration/E2E oracle suite | High |
| D-32 | Build/init scripts completion | Medium |
| D-33 | Operational observability | Medium |
| D-35 | E2E failure mode harness | High |
| D-08 | GovernedArtifact refs in iteration context | Low |

### Deferred from V10

- V10-G5: Active exceptions not included in IterationStarted.refs[]
- Tests 17-18: Integrity condition E2E testing (ORACLE_GAP, EVIDENCE_MISSING)

### Constraints

- **Planning only** — Do not implement until plan is approved
- V10 is complete — do not re-implement or modify V10 code
- Consider what E2E infrastructure exists vs needs to be built
- Identify any blocking dependencies between phases

### Suggested Approach

1. Read SR-PLAN-GAP-ANALYSIS §4 (V11 Proposed section)
2. Inventory existing E2E infrastructure in `crates/sr-e2e-harness/` and `crates/sr-oracles/`
3. Draft SR-PLAN-V11.md with phased approach
4. Present plan for review before implementation

---

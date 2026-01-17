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


---


## Current Status

**V10:** ✅ COMPLETE (2026-01-17)
**V11:** ✅ COMPLETE (2026-01-17)
**V12:** 🔄 READY FOR IMPLEMENTATION
**Branch:** `solver-ralph-12`

### V12 Reviews (2026-01-17)

**V12 Coherence Review** ✅ APPROVED
- Validated all file references and line counts in SR-PLAN-V12
- Confirmed `evidence.rs` (890 lines), `nats.rs` (677 lines), `governor.rs` (1059 lines) — all exact matches
- See `docs/reviews/SR-PLAN-V12-COHERENCE-REVIEW.md`

**V12 Consistency Review** ✅ APPROVED
- Analyzed SR-PLAN-V12 against canonical SR-* documents (ontology, epistemology, semantics)
- No high-severity inconsistencies found
- One low-severity recommendation: explicitly state `classification: required` for manifest-validation oracle
- See `docs/reviews/SR-PLAN-V12-CONSISTENCY-REVIEW.md`

---

## Next Instance Prompt

> **Assignment:** Implement SR-PLAN-V12 (Operational Refinement)

### Orientation

1. Read `docs/planning/SR-PLAN-V12.md` — the implementation plan (3 phases)
2. Read `docs/reviews/SR-PLAN-V12-CONSISTENCY-REVIEW.md` — approved with one recommendation (S-2)
3. Reference existing implementations:
   - `oracle-suites/core-v1/oracles/schema-validation.sh` — oracle implementation pattern
   - `oracle-suites/core-v1/suite.json` — suite registration pattern
   - `crates/sr-adapters/src/nats.rs` — MessageEnvelope and contracts to document
   - `crates/sr-adapters/src/governor.rs` — governor logic to extract

### Phase Order

V12-1 and V12-2 can run in parallel (no dependencies).
V12-3 depends on V12-2 (needs formalized message contracts).

### Deliverables

| Phase | Deliverable | Key Files |
|-------|-------------|-----------|
| V12-1 | Evidence manifest validation oracle | `oracle-suites/core-v1/oracles/manifest-validation.sh`, update `suite.json` |
| V12-2 | Message contract documentation + tests | `schemas/messaging/SR-MESSAGE-CONTRACTS.md`, `schemas/messaging/message-envelope.schema.json` |
| V12-3 | Standalone governor binary | `crates/sr-governor/`, update `deploy/docker-compose.yml` |

### Verification

- V12-1: Oracle passes/fails correctly on valid/invalid manifests
- V12-2: JSON Schema validates example messages; contract tests pass
- V12-3: Governor service starts, emits iterations, respects budgets

Git commit after each phase completion.

---

## Previous Session Summary (2026-01-17)

### Completed: SR-PLAN-V12 Consistency Review

- Analyzed V12-1 (manifest validation oracle) — aligned with SR-SPEC §1.9.1.1, C-EVID-1
- Analyzed V12-2 (message contracts) — MessageEnvelope is adapter-layer, not domain type
- Analyzed V12-3 (governor service) — satisfies C-LOOP-1, C-LOOP-2, C-CTX-1
- Verdict: APPROVE — plan can proceed to implementation
- Produced `docs/reviews/SR-PLAN-V12-CONSISTENCY-REVIEW.md`

---

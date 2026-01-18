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


## Current Assignment: Implement Audit Remediation Plan

> **Assignment:** Implement the remediation plan for findings from the consolidated codebase audit, following the phased approach in `docs/planning/SR-CODEBASE-AUDIT-PLAN.md`.

1) Use `docs/planning/SR-CODEBASE-AUDIT-PLAN.md` Phase 1 as the checklist; P1-STOPS-COMPLETE and P1-INTEGRITY-WIRE are now done.
2) Finish **P1-BUDGET-GOV**: add end-to-end proof that requested budgets propagate (API → projections → governor) and enforce max_oracle_runs exhaustion routing; document any gaps.
3) Finish **P1-SHIP-GATE**: add coverage that freeze/release gate rejects unverified, stale, or approval-missing states and passes the happy path; ensure stop/staleness flags are honored.
4) When Phase 1 is clear, proceed to staleness/evidence-status/note APIs (P1-STALENESS-API, P1-EVID-STATUS, P1-NOTES-API) per plan; rerun `cargo test --workspace` after each deliverable and update plan/blockers.

### Input Documents

1. **`docs/planning/SR-CODEBASE-AUDIT-PLAN.md`** — The remediation plan (your execution guide)
2. **`docs/platform/SR-CONTRACT.md`** — The contract invariants being enforced
3. **`docs/program/SR-DIRECTIVE.md`** — The execution policies being aligned

Reference `docs/platform/SR-SPEC.md` for API signatures, event schemas, and state machine behavior.


### Execution Protocol

**For each phase:**

1. **Read** the relevant SR-* documents for the invariants/specs being addressed
2. **Implement** the changes specified in the plan
3. **Test** — Run the existing test suite plus any new tests required by the plan
4. **Verify** — Confirm the deliverable satisfies its acceptance criteria
5. **Commit** — `git add && git commit` with message referencing the deliverable ID
6. **Proceed** — Move to the next deliverable

**Phase sequencing:**
- Complete all Phase 1 deliverables before moving to Phase 2
- Within a phase, deliverables may be done in any order unless dependencies exist
- Do not skip phases; the plan is designed for incremental risk reduction

### Risk & Rollback

Each deliverable should be:
- **Atomic** — One logical change per commit
- **Reversible** — Changes can be rolled back independently
- **Tested** — Existing tests must pass; new tests required for new invariants

If a deliverable causes test failures:
1. Fix forward if the fix is obvious
2. Otherwise, revert and document the blocker
3. Do not proceed to the next phase with failing tests

### Success Criteria

1. All Phase 1 deliverables implemented and tested (P1-* complete)
2. All Phase 2-6 deliverables implemented and tested
3. Full test suite passes (`cargo test --workspace`)
4. No regressions in existing functionality
5. Commit history shows one commit per deliverable ID
6. Any blockers documented in `docs/reviews/SR-CODEBASE-AUDIT-BLOCKERS.md` (create if needed)

### Completion

When all phases are complete:
1. Update `docs/reviews/SR-CODEBASE-AUDIT-CONSOLIDATED.md` to mark findings as resolved
2. Update `docs/planning/SR-CODEBASE-AUDIT-PLAN.md` to mark deliverables as complete
3. Push all commits to the remote branch

---

## Dev Session Log (latest)

- Semantic worker now emits `NO_ELIGIBLE_WORK` stops and runs IntegrityChecker post-oracle to emit `IntegrityViolationDetected` + `StopTriggered` (GovernanceChangePortal) for tamper/gap/env/flake/evidence-missing; evidence emission skips when a stop fires.
- Governor enforces `max_oracle_runs` budgets end-to-end and emits `StopTriggered` with HumanAuthority routing when exhausted; budget patch helper reused across API + tests.
- Portal recommendation alignment fixed for repeated failures; budget/test suite still passing (`cargo test --workspace`, integration tests gated by env flags).

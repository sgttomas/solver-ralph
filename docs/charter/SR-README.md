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

### Context

A multi-agent codebase audit identified gaps between the SR-* specifications and the implementation. The findings have been consolidated and a remediation plan has been prepared. Your task is to implement the remediations.

### Input Documents

1. **`docs/planning/SR-CODEBASE-AUDIT-PLAN.md`** — The remediation plan (your execution guide)
2. **`docs/platform/SR-CONTRACT.md`** — The contract invariants being enforced
3. **`docs/program/SR-DIRECTIVE.md`** — The execution policies being aligned

Reference `docs/platform/SR-SPEC.md` for API signatures, event schemas, and state machine behavior.

### Remediation Plan Overview

The plan is organized into 6 phases:

| Phase | Focus | Priority | Deliverable IDs |
|-------|-------|----------|-----------------|
| **Phase 1** | Trust Boundary & Verification | P1 (Critical) | P1-TB-PORTALS, P1-VER-COMPUTE, P1-DIR-BUDGET, P1-DIR-STOP |
| **Phase 2** | Ontological Alignment | P2 (High) | P2-ONT-STALENESS, P2-ONT-PROJECTION |
| **Phase 3** | Epistemological Completeness | P2 (High) | P3-EPI-EVIDENCE, P3-EPI-ORACLE |
| **Phase 4** | Semantic Accuracy | P3 (Medium) | P4-SEM-ERROR, P4-SEM-METRIC |
| **Phase 5** | UI Parity | P3 (Medium) | P5-UI-ADMIN |
| **Phase 6** | Documentation & Governance | P3 (Medium) | P6-DOC-SYNC |

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

### Key Implementation Targets

**Phase 1 (Critical — Do First):**

| ID | Target | What to Implement |
|----|--------|-------------------|
| P1-TB-PORTALS | `sr-api/src/handlers/` | Add `portal_id` whitelist validation per C-TB-4 |
| P1-VER-COMPUTE | `sr-core/src/` | Implement `VerificationComputer` with evidence aggregation per C-VER-* |
| P1-DIR-BUDGET | `sr-api/src/governor.rs` | Align defaults to SR-DIRECTIVE (5/25/16), not (100/3600) |
| P1-DIR-STOP | `sr-api/src/governor.rs` | Wire `ORACLE_FLAKE` detection to stop trigger emission |

**Phase 2-6:** See `SR-CODEBASE-AUDIT-PLAN.md` for full details.

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

- Stop triggers now include `condition` aliases; governor consumes trigger strings and tracks recommended portals. Work-surface iteration start emits StopTriggered for `STAGE_UNKNOWN` and `SEMANTIC_PROFILE_MISSING` with GovernanceChangePortal routing.
- OracleExecutionWorker runs IntegrityChecker post-run (tamper/gap/env/flake/evidence_missing/manifest_invalid) and emits IntegrityViolationDetected plus StopTriggered to GovernanceChangePortal; integrity domain enums expanded for evidence missing/manifest invalid.
- Evidence-stop conditions flow via integrity violations; stage-semantic gaps now stop before iteration. `cargo test --workspace` passes (Infisical integration tests remain gated behind RUN_INFISICAL_TESTS).

## Handoff Prompt (next instance)

1) Re-read SR-DIRECTIVE §4 and SR-CONTRACT integrity sections; keep `docs/planning/SR-CODEBASE-AUDIT-PLAN.md` Phase 1 as the checklist.
2) Finish Phase 1 stops/integrity:
   - Add remaining stop triggers and tests: NO_ELIGIBLE_WORK (semantic worker), ORACLE_FLAKE propagation, REPEATED_FAILURE coverage, max_oracle_runs exhaustion routing, and portal recommendations per directive.
   - Extend integrity wiring/tests: simulate flake/gap/env/tamper/evidence-missing paths to assert IntegrityViolationDetected + StopTriggered; ensure verification/shippable honor integrity conditions.
3) Close budget alignment gaps: honor requested budgets end-to-end, monotonic PATCH tests, projections/governor consistency.
4) Update plan progress and blockers as needed; rerun `cargo test --workspace` before moving to next deliverables.

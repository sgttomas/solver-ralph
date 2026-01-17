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
**V11:** ✅ Reviews complete — ready for implementation
**Branch:** `solver-ralph-11`

All V10 phases verified and complete:
- V10-1 through V10-4: Loop Governor stop triggers, decision gating, traceability
- V10-5: Loop PATCH endpoint with budget monotonicity
- V10-6: OracleSuite hash prefix fix

V11 Coherence Review (2026-01-17):
- SR-PLAN-V11 reviewed against codebase — APPROVED
- Revisions incorporated: V10-G5 addressed, existing infrastructure acknowledged, effort estimates reduced
- See `docs/planning/SR-PLAN-V11-COHERENCE-REVIEW.md` for detailed findings

V11 Consistency Review (2026-01-17):
- SR-PLAN-V11 reviewed for consistency with canonical SR-* documents — REVISED
- Corrected V11-6 schemas to align with SR-SPEC §1.5.2 and §1.5.3
- Fixed: `rel: "governed_by"` → `rel: "depends_on"`, added required `id` field
- Fixed: `kind: "Exception"` → `kind: "Waiver|Deviation|Deferral"`, moved `exception_id` to `id` field
- See `docs/planning/SR-PLAN-V11-CONSISTENCY-REVIEW.md` for detailed findings

See `docs/planning/SR-PLAN-LOOPS.md` for V10 verification results.
See `docs/build-governance/SR-CHANGE.md` v1.2 (implementation) and v1.3 (SR-SPEC updates).

---

## Previous Session Summary (2026-01-17)

### Completed: SR-PLAN-V11 Consistency Review (across three dimensions)

**Objective:** Analyze SR-PLAN-V11 for consistency with canonical SR-* documents across ontology, epistemology, and semantics.

**High-Severity Findings (corrected):**
1. **O-2:** `rel: "governed_by"` explicitly deprecated per SR-SPEC §1.5.3 — changed to `rel: "depends_on"`
2. **S-1:** GovernedArtifact ref missing required `id` field per SR-SPEC §1.5.2 — added `id: "<doc_id>"`
3. **S-2:** Exception ref schema mismatch — changed `kind: "Exception"` to `kind: "Waiver|Deviation|Deferral"`, moved `exception_id` to `id` field

**Deliverables Produced:**
- `docs/planning/SR-PLAN-V11-CONSISTENCY-REVIEW.md` — Detailed consistency report with 15 findings
- `docs/planning/SR-PLAN-V11.md` — Corrected V11-6 schemas to align with SR-SPEC

**Verdict:** REVISED — Plan corrected and now consistent with canonical SR-* documents.

### Earlier: SR-PLAN-V11 Codebase Coherence Review

**Key Findings:**
1. **V11-1 (Infisical):** Implementation more complete than stated (431 lines with full `SecretProvider` trait)
2. **V11-2 (Build Scripts):** Infrastructure already exists (`deploy/init.sh` 21KB, `docker-compose.yml` 6.5KB)
3. **V11-3 (Observability):** Foundation exists (`/health`, `/metrics` endpoints); needs `/ready` and domain metrics
4. **V11-4 (E2E Harness):** 45K+ lines of failure mode code already implemented
5. **V11-5 (Integration Suite):** `IntegrationRunner` exists (38KB); needs suite registration only
6. **V11-6 (GovernedArtifact Refs):** Content hashing approach was undefined — now specified

**Effort Reduction:** Total estimated effort reduced from 8-12 sessions to 6.5-9.5 sessions due to existing infrastructure.

---

## Next Instance Prompt

> **Assignment:** Implement SR-PLAN-V11. Both codebase coherence and canonical consistency reviews are complete.

### Context

SR-PLAN-V11 has passed both gates:
1. **Codebase Coherence Review** — files exist, dependencies correct, phase ordering valid
2. **Consistency Review** — terminology, schemas, and structures align with canonical SR-* documents

The plan is ready for implementation. See `docs/planning/SR-PLAN-V11.md` for the full plan with corrected schemas.

### Orientation

1. Read `docs/planning/SR-PLAN-V11.md` — the implementation plan
2. Review the reviews if needed:
   - `docs/planning/SR-PLAN-V11-COHERENCE-REVIEW.md` — what infrastructure already exists
   - `docs/planning/SR-PLAN-V11-CONSISTENCY-REVIEW.md` — schema corrections applied
3. Navigate the canonical SR-* documents as needed (see canonical index above)

### Task: V11 Implementation

Implement the V11 phases as specified in SR-PLAN-V11. The phases can be executed according to the dependency graph in §3 of the plan:

- **V11-1, V11-2, V11-3** can run in parallel (no dependencies)
- **V11-4** can start once V11-2 verifies docker-compose
- **V11-5, V11-6** can run in parallel after V11-4

Much of the infrastructure already exists (see coherence review). Focus on:
- Verification and documentation of existing code
- Filling gaps identified in the coherence review
- Implementing the corrected V11-6 schemas per the consistency review

### Verification

Per SR-PLAN-V11 §4, each phase has specific verification criteria. Run verification after completing each phase before proceeding.

### Constraints

- Commit after completing each phase
- Update SR-README with progress after each phase
- Consult SR-* documents when implementation decisions arise

---

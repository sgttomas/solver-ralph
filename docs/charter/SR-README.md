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
**V12:** ✅ COMPLETE (2026-01-17)
**Branch:** `solver-ralph-12`

### V12 Summary (2026-01-17)

**V12-1: Evidence Manifest Validation Oracle (D-15)** ✅ COMPLETE
- Oracle registered in `oracle-suites/core-v1/suite.json` with `classification: required`
- Implementation: `oracle-suites/core-v1/oracles/manifest-validation.sh`

**V12-2: NATS Message Contract Documentation (D-21)** ✅ COMPLETE
- Documentation: `schemas/messaging/SR-MESSAGE-CONTRACTS.md`
- JSON Schema: `schemas/messaging/message-envelope.schema.json`

**V12-3: Standalone Governor Service Binary (D-22)** ✅ COMPLETE
- Binary: `crates/sr-governor/`
- Docker: `deploy/docker-compose.yml`

### V12 Reviews (2026-01-17)

- Coherence Review: `docs/reviews/SR-PLAN-V12-COHERENCE-REVIEW.md`
- Consistency Review: `docs/reviews/SR-PLAN-V12-CONSISTENCY-REVIEW.md`

---

## Next Instance Prompt

> **Assignment:** Audit SOLVER-Ralph codebase for coherence and consistency against SR-CONTRACT, SR-SPEC, and SR-TYPES.

### Orientation

1. Read `docs/reviews/SR-CODEBASE-AUDIT-METHODOLOGY.md` — the audit methodology and checklists
2. Read `docs/platform/SR-CONTRACT.md` §1.1 — the invariants index (C-* identifiers)
3. Read `docs/platform/SR-SPEC.md` §1.5-1.12 — schema definitions
4. Read `docs/platform/SR-TYPES.md` §4 — type registry

### Execution

**Layer 1 (Contract Compliance):** For each C-* invariant, search codebase for enforcement and test coverage.
**Layer 2 (Schema Alignment):** Compare SR-SPEC schemas to Rust structs in sr-domain/sr-adapters.
**Layer 3 (Type Conformance):** Verify SR-TYPES §4.3/§4.4 entries have implementations.
**Layer 4 (API Coverage):** Verify SR-SPEC §2.3 endpoints exist with correct behavior.

### Deliverable

Produce `docs/reviews/SR-CODEBASE-AUDIT-FINDINGS.md` with:
- Contract compliance matrix (invariant → enforcement → test → status)
- Schema alignment report (spec schema → code struct → issues)
- Type registry coverage (type_key → Rust type → status)
- Prioritized remediation list (P0/P1/P2 with specific actions)

### Success Criteria

- Every C-* invariant has a status: ✅ Enforced | ⚠️ Partial | ❌ Missing
- Every SR-SPEC schema compared to code
- Remediation list is actionable (file + function + change)

---

---

## Previous Session Summary (2026-01-17)

### Completed: SR-PLAN-V12 Implementation

- V12-1: Evidence manifest validation oracle (`oracle-suites/core-v1/oracles/manifest-validation.sh`)
- V12-2: NATS message contract documentation (`schemas/messaging/`)
- V12-3: Standalone governor service binary (`crates/sr-governor/`)

### Completed: SR-PLAN-V12 Reviews

- Coherence Review: APPROVED — `docs/reviews/SR-PLAN-V12-COHERENCE-REVIEW.md`
- Consistency Review: APPROVED — `docs/reviews/SR-PLAN-V12-CONSISTENCY-REVIEW.md`

---

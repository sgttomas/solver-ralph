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
**Branch:** `solver-ralph-11`

### V11 Progress (2026-01-17)

**V11-1: Infisical Integration (D-16)** ✅ COMPLETE
- Added 15 integration tests with wiremock for mock Infisical API
- Tests cover: get/store/delete secrets, envelope key retrieval, caching, error handling
- Created `.env.example` with Infisical configuration variables
- Documentation in `docs/platform/SR-DEPLOYMENT.md`

**V11-2: Build/Init Scripts (D-32)** ✅ COMPLETE
- Audited `deploy/init.sh` — confirmed comprehensive initialization
- Created `scripts/init-all.sh` wrapper with pre-flight checks
- Documentation in `docs/platform/SR-DEPLOYMENT.md`

**V11-3: Operational Observability (D-33)** ✅ COMPLETE
- Added `/ready` endpoint with PostgreSQL, MinIO, NATS connectivity checks
- Added domain-specific metrics (loops, iterations, candidates, oracle runs, events)
- Created `docs/platform/SR-OBSERVABILITY.md` documentation

**V11-4: E2E Failure Mode Harness (D-35)** ✅ COMPLETE
- Added `EvidenceMissing` scenario to E2E harness (Test 18)
- Implemented `run_evidence_missing()` with proper integrity condition detection
- Added `--evidence-missing` CLI flag
- Verifies EVIDENCE_MISSING is non-waivable per SR-CONTRACT C-OR-7

**V11-5: Integration Oracle Suite (D-26)** ✅ COMPLETE
- Created `create_integration_suite()` factory function
- Added `SUITE_INTEGRATION_ID` constant (`suite:SR-SUITE-INTEGRATION`)
- Registered in `OracleSuiteRegistry::with_core_suites()` (now 5 suites total)
- Network mode: Private (integration tests need network access)

**V11-6: GovernedArtifact & Exception Refs (D-08)** ✅ COMPLETE
- Created `crates/sr-api/src/governed.rs` — GovernedManifest with content-addressed artifacts
- Computed manifest at API startup with SHA-256 content hashes
- Added `get_active_exceptions_for_loop()` to ProjectionBuilder
- Updated `start_iteration()` to add GovernedArtifact and active Exception refs
- Used corrected schemas per SR-PLAN-V11-CONSISTENCY-REVIEW:
  - GovernedArtifact: `rel: "depends_on"`, `id: doc_id`, `meta: {content_hash, version, type_key}`
  - Exceptions: `kind: "Waiver|Deviation|Deferral"`, `id: exception_id`, `rel: "depends_on"`

### V11 Reviews (2026-01-17)

V11 Coherence Review:
- SR-PLAN-V11 reviewed against codebase — APPROVED
- Revisions incorporated: V10-G5 addressed, existing infrastructure acknowledged, effort estimates reduced
- See `docs/planning/SR-PLAN-V11-COHERENCE-REVIEW.md` for detailed findings

V11 Consistency Review:
- SR-PLAN-V11 reviewed for consistency with canonical SR-* documents — REVISED
- Corrected V11-6 schemas to align with SR-SPEC §1.5.2 and §1.5.3
- Fixed: `rel: "governed_by"` → `rel: "depends_on"`, added required `id` field
- Fixed: `kind: "Exception"` → `kind: "Waiver|Deviation|Deferral"`, moved `exception_id` to `id` field
- See `docs/planning/SR-PLAN-V11-CONSISTENCY-REVIEW.md` for detailed findings

See `docs/planning/SR-PLAN-LOOPS.md` for V10 verification results.
See `docs/build-governance/SR-CHANGE.md` v1.2 (implementation) and v1.3 (SR-SPEC updates).

---

## Next Instance Prompt

**Status:** V11 is COMPLETE. All phases (V11-1 through V11-6) have been implemented and verified.

**Assignment:** Review V11 completion and identify any remaining work or next plan.

**Orientation:**
1. Read `docs/planning/SR-PLAN-V11.md` — the completed implementation plan
2. Check `docs/program/SR-PLAN.md` for any remaining deliverables or future plans
3. Navigate canonical SR-* documents as needed (see index above)

**V11 Summary:**
- **V11-1:** Infisical Integration with 15 wiremock-based integration tests
- **V11-2:** Build/Init Scripts with pre-flight checks
- **V11-3:** Operational Observability with `/ready` endpoint and domain metrics
- **V11-4:** E2E Failure Mode Harness with EVIDENCE_MISSING scenario (Test 18)
- **V11-5:** Integration Oracle Suite registered as `SR-SUITE-INTEGRATION`
- **V11-6:** GovernedArtifact & Exception refs in `IterationStarted.refs[]`

**Verification:** All tests pass. Build succeeds. See SR-PLAN-V11 §5 for verification criteria.

---

## Previous Session Summary (2026-01-17)

### Completed: V11-1, V11-2, V11-3 Implementation

**V11-1: Infisical Integration (D-16)**
- Added 15 wiremock-based integration tests to `crates/sr-adapters/src/infisical.rs`
- Tests cover: secret CRUD, envelope key retrieval/caching, error handling (auth, network, not found)
- Created `.env.example` with Infisical configuration

**V11-2: Build/Init Scripts (D-32)**
- Created `scripts/init-all.sh` — wrapper with Docker pre-flight, dependency checks, service startup
- Audited `deploy/init.sh` — confirmed comprehensive (PostgreSQL, MinIO, Zitadel, secrets)

**V11-3: Operational Observability (D-33)**
- Added `/ready` endpoint with PostgreSQL, MinIO, NATS health checks (HTTP 200/503)
- Added domain metrics: loops, iterations, candidates, oracle runs, events (with latencies)
- Created `docs/platform/SR-DEPLOYMENT.md` and `docs/platform/SR-OBSERVABILITY.md`

**Canonical updates:**
- SR-SPEC §2.3.13 — Added Operational endpoints (`/health`, `/ready`, `/api/v1/metrics`)
- SR-README — Added SR-DEPLOYMENT, SR-OBSERVABILITY to canonical paths
- SR-CHANGE v1.4 — Recorded V11-1/2/3 implementation

**Commits:**
- `ada5202` — feat(v11): Implement V11-1, V11-2, V11-3
- `8af3298` — docs: Update canonical SR-* documents for V11-1/2/3

**Effort Reduction:** Total estimated effort reduced from 8-12 sessions to 6.5-9.5 sessions due to existing infrastructure.

---

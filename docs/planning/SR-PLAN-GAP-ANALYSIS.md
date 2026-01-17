# SR-PLAN Gap Analysis & Roadmap

**Purpose:** Track completion status of SR-PLAN deliverables and inform future implementation plans.

**Last Updated:** 2026-01-17
**Updated By:** Agent (SR-PLAN-LOOPS validation session)

---

## 1. Charter Milestone Mapping

### Milestone 1: MVP = Semantic Work Unit Runtime

Per SR-CHARTER §Immediate Objective:

| Requirement | Status | Implementing Deliverables | Notes |
|-------------|--------|---------------------------|-------|
| Bounded agentic work | ✅ Complete | D-05, D-06, D-22 | Loops with budgets, state machines |
| Iteration cycling with controlled memory | ✅ Complete | D-08, D-18, V6-1 | `/start` endpoint, SYSTEM-mediated |
| Declared work surface (intake + procedure stages) | ✅ Complete | D-37, V3, V4, V5 | Intakes, Procedure Templates, Stages |
| Deterministic event recording | ✅ Complete | D-09, D-10 | Event store, append-only |
| Governance state projection | ✅ Complete | D-11, D-12 | Projections rebuildable |
| Semantic oracle evidence capture and gating | ✅ Complete | D-24, D-25, D-39 | V8 complete; oracle runner + semantic suite |
| Explicit human authority points | ✅ Complete | D-19, D-30 | Approval recording, stage gating |

**Milestone 1 Completion:** 100% — COMPLETE. V9 delivered semantic worker and Branch 0 acceptance.

### Milestone 2: External API Integration

Per SR-CHARTER, follows Milestone 1. **Not yet scoped.**

---

## 2. SR-PLAN Deliverable Status

### PKG-01 — Governance hygiene and unblockers

| Deliverable | Title | Status | Completed In |
|-------------|-------|--------|--------------|
| D-01 | Governance hygiene patchset | ✅ Complete | Initial setup |

### PKG-02 — Repo and CI substrate

| Deliverable | Title | Status | Completed In |
|-------------|-------|--------|--------------|
| D-02 | Repository scaffold | ✅ Complete | Initial setup |
| D-03 | CI baseline | ✅ Complete | Initial setup |
| D-04 | Local developer tooling | ✅ Complete | Initial setup |

### PKG-03 — Domain core

| Deliverable | Title | Status | Completed In |
|-------------|-------|--------|--------------|
| D-05 | Domain model primitives | ✅ Complete | sr-core |
| D-06 | Deterministic state machines | ✅ Complete | sr-core |
| D-07 | Ports and boundary interfaces | ✅ Complete | sr-core |
| D-08 | Context compilation rules | ✅ Complete | sr-core |

### PKG-04 — Persistence, projections, and graph

| Deliverable | Title | Status | Completed In |
|-------------|-------|--------|--------------|
| D-09 | Postgres schemas and migrations | ✅ Complete | sr-adapters |
| D-10 | EventStore adapter | ✅ Complete | sr-adapters |
| D-11 | Projection builder | ✅ Complete | sr-adapters |
| D-12 | Dependency graph projection | ✅ Complete | sr-adapters |
| D-13 | Outbox publisher (NATS) | ✅ Complete | sr-adapters |

### PKG-05 — Evidence storage and integrity

| Deliverable | Title | Status | Completed In | Notes |
|-------------|-------|--------|--------------|-------|
| D-14 | Evidence store adapter (MinIO) | ✅ Complete | sr-adapters | minio.rs (440 lines) |
| D-15 | Evidence manifest v1 library | ⚠️ Partial | sr-adapters | evidence.rs exists; validation oracle not implemented |
| D-16 | Restricted evidence handling | ❌ Not Started | — | Infisical envelope keys not implemented |

### PKG-06 — API and identity boundary

| Deliverable | Title | Status | Completed In |
|-------------|-------|--------|--------------|
| D-17 | API scaffold + OIDC | ✅ Complete | sr-api |
| D-18 | Core API endpoints | ✅ Complete | sr-api |
| D-19 | Governance/portal endpoints | ✅ Complete | sr-api |
| D-20 | Evidence API | ✅ Complete | sr-api |

### PKG-07 — Orchestration runtime

| Deliverable | Title | Status | Completed In | Notes |
|-------------|-------|--------|--------------|-------|
| D-21 | NATS/JetStream integration | ⚠️ Partial | sr-adapters | Outbox exists; full messaging contracts not formalized |
| D-22 | Loop governor service | ⚠️ Partial | sr-api | `/start` endpoint serves as minimal governor; full service not built |
| D-23 | Reference worker bridge | ✅ Complete | V9-1 | `ReferenceWorkerBridge` in sr-adapters |

### PKG-08 — Oracles and verification substrate

| Deliverable | Title | Status | Completed In | Notes |
|-------------|-------|--------|--------------|-------|
| D-24 | Oracle runner service | ✅ Complete | V8-1, V8-2 | Registry port + event-driven worker |
| D-25 | Core oracle suite | ✅ Complete | V8-4 | oracle-suites/core-v1 container |
| D-26 | Integration/E2E oracle suite | ❌ Not Started | — | Depends on D-24, D-25 |
| D-27 | Oracle integrity checks | ✅ Complete | V8-3 | TAMPER/GAP/FLAKE/ENV_MISMATCH detection |

### PKG-09 — UI portals and human review surface

| Deliverable | Title | Status | Completed In |
|-------------|-------|--------|--------------|
| D-28 | UI scaffold + OIDC | ✅ Complete | ui/ |
| D-29 | Loop/iteration/candidate views | ✅ Complete | ui/ |
| D-30 | Portal workflows UI | ✅ Complete | ui/, V6 |

### PKG-10 — Self-host and operations substrate

| Deliverable | Title | Status | Completed In | Notes |
|-------------|-------|--------|--------------|-------|
| D-31 | Self-host deployment stack | ✅ Complete | docker-compose | Postgres, MinIO, API, UI |
| D-32 | Build/init scripts | ⚠️ Partial | scripts/ | DB init exists; Infisical setup manual |
| D-33 | Operational logging | ⚠️ Partial | sr-api | Basic logging; no structured observability |

### PKG-11 — End-to-end demonstration and determinism proof

| Deliverable | Title | Status | Completed In | Notes |
|-------------|-------|--------|--------------|-------|
| D-34 | E2E harness (happy path) | ⚠️ Partial | V6-3 | Manual verification; no automated harness |
| D-35 | E2E harness (failure modes) | ❌ Not Started | — | Integrity/exception flows not tested |
| D-36 | Replayability demonstration | ✅ Complete | V9-3 | SR-REPLAY-PROOF.md, replay_determinism_test.rs |

### PKG-12 — Semantic work surface + prompt decomposition

| Deliverable | Title | Status | Completed In | Notes |
|-------------|-------|--------|--------------|-------|
| D-37 | Work surface schemas | ✅ Complete | V3, V4, V5 | Intake + Procedure Template + Work Surface |
| D-38 | Prompt → Plan Instance decomposition | ❌ Not Started | — | Automated work decomposition |
| D-39 | Semantic oracle integration | ✅ Complete | V8-5 | oracle-suites/semantic-v1 container |
| D-40 | Event Manager eligible-set computation | ✅ Complete | V9-1 | `EventManager.compute_eligible_set()` in sr-adapters |
| D-41 | Reference semantic worker | ✅ Complete | V9-1 | `SemanticWorkerBridge` in sr-adapters |

---

## 3. Branch 0 Acceptance Criteria

Per SR-PLAN §4.1, Branch 0 (Semantic Manifold MVP) requires:

| Criterion | Status | Evidence |
|-----------|--------|----------|
| Loop created for problem-statement work unit | ✅ Complete | V9-2 `branch_0_e2e_test.rs` |
| Iteration started with Work Surface ref set | ✅ Complete | V9-2 `branch_0_e2e_test.rs` |
| Candidate intake bundle produced | ✅ Complete | V9-1 `SemanticWorkerBridge` |
| Evidence Bundle from semantic oracle suite | ✅ Complete | V8 (D-24, D-39), V9-2 |
| Human portal approval recorded | ✅ Complete | V9-2 `test_branch_0_portal_approvals_required` |
| Freeze baseline created | ✅ Complete | V9-2 `test_branch_0_freeze_baseline` |
| Replay determinism proof | ✅ Complete | V9-3 SR-REPLAY-PROOF.md |

**Branch 0: COMPLETE.** All criteria satisfied. See `SR-BRANCH-0-ACCEPTANCE.md` for formal verification.

---

## 4. Roadmap: Remaining Plans

### SR-PLAN-V7 (Current)

**Status:** ✅ Complete (V7-1 through V7-5 complete)
**Scope:** MVP Stabilization & Attachment Foundation

| Phase | Deliverables Addressed | Status |
|-------|------------------------|--------|
| V7-1 | Integration tests (stabilization) | ✅ Complete |
| V7-2 | Error handling & UX | ✅ Complete |
| V7-3 | Attachment backend (`record.attachment`) | ✅ Complete |
| V7-4 | Attachment frontend | ✅ Complete |
| V7-5 | Multiple iterations | ✅ Complete |

### SR-PLAN-V8 (Complete)

**Status:** ✅ Complete (2026-01-16)
**Scope:** Oracle Runner & Semantic Suite Foundation
**Target Deliverables:** D-24, D-25, D-27, D-39
**Actual Effort:** ~10 sessions

| Phase | Focus | Deliverables | Status |
|-------|-------|--------------|--------|
| V8-1 | Oracle Suite Registry | D-24 (partial) | ✅ Complete |
| V8-2 | Event-Driven Worker | D-24 (partial) | ✅ Complete |
| V8-3 | Integrity Checks | D-27 | ✅ Complete |
| V8-4 | Core Oracle Suite | D-25 | ✅ Complete |
| V8-5 | Semantic Oracles | D-39 | ✅ Complete |

### SR-PLAN-V9 (Complete)

**Status:** ✅ Complete (2026-01-16)
**Scope:** Semantic Worker & Branch 0 Completion
**Target Deliverables:** D-23, D-40, D-41, D-36
**Actual Effort:** 4 sessions

| Phase | Focus | Deliverables | Status |
|-------|-------|--------------|--------|
| V9-1 | Semantic worker integration | D-23, D-40, D-41 | ✅ Complete |
| V9-2 | E2E flow integration test | — | ✅ Complete |
| V9-3 | Replayability demonstration | D-36 | ✅ Complete |
| V9-4 | Branch 0 acceptance verification | — | ✅ Complete |

### SR-PLAN-V10 (Proposed)

**Status:** Not yet authored
**Scope:** Loop Governor Completion & Traceability
**Target Deliverables:** D-22, D-12, D-18
**Validation Source:** SR-PLAN-LOOPS validation (2026-01-17)

| Phase | Focus | Deliverables | Gap Source |
|-------|-------|--------------|------------|
| V10-1 | Stop triggers (BUDGET_EXHAUSTED, REPEATED_FAILURE) | D-22 | Tests 13-14 (Critical) |
| V10-2 | Decision-required resume after stop trigger | D-22 | Test 15 |
| V10-3 | Candidate → Iteration traceability | D-12 | Test 12 |
| V10-4 | Iteration refs completeness (Loop, exceptions) | D-18 | Tests 9, 16 |
| V10-5 | Loop edit endpoint with budget monotonicity | D-18 | Test 8 |
| V10-6 | OracleSuite hash prefix fix | D-24 | Test 10 |

**Critical Path:** V10-1 → V10-2 (stop triggers enable decision gating)

**Detailed Gap Descriptions:**

| ID | Description | Contract | Severity |
|----|-------------|----------|----------|
| V10-G1 | Budget exhaustion not enforced; iterations exceed max_iterations | C-LOOP-1, C-LOOP-3 | Critical |
| V10-G2 | Repeated failure (3+ consecutive) doesn't pause Loop | C-LOOP-3 | Critical |
| V10-G3 | Candidate `produced_by_iteration_id` not recorded; graph edges missing | C-LOOP-4 | High |
| V10-G4 | Loop ref missing from IterationStarted.refs[] (only in correlation_id) | C-CTX-1 | Medium |
| V10-G5 | Active exceptions not included in IterationStarted.refs[] | C-CTX-1 | Medium |
| V10-G6 | No Loop PATCH endpoint for budget updates | — | Medium |
| V10-G7 | OracleSuite content_hash has doubled `sha256:sha256:` prefix | — | Low |

### SR-PLAN-V11 (Proposed)

**Status:** Not yet authored
**Scope:** Production Hardening & E2E Testing
**Target Deliverables:** D-16, D-26, D-32, D-33, D-35, D-08
**Validation Source:** SR-PLAN-LOOPS validation (2026-01-17) — deferred items

| Phase | Focus | Deliverables | Gap Source |
|-------|-------|--------------|------------|
| V11-1 | Restricted evidence handling | D-16 | — |
| V11-2 | Integration/E2E oracle suite | D-26 | — |
| V11-3 | Build/init scripts completion | D-32 | — |
| V11-4 | Operational observability | D-33 | — |
| V11-5 | E2E failure mode harness (integrity conditions) | D-35 | Tests 17-18 |
| V11-6 | GovernedArtifact refs in iteration context | D-08 | Test 9 |

**Deferred from V10:**

| ID | Description | Reason for Deferral |
|----|-------------|---------------------|
| V11-D1 | ORACLE_GAP/EVIDENCE_MISSING E2E testing | Requires automated harness (D-35) |
| V11-D2 | GovernedArtifact refs (SR-DIRECTIVE, etc.) | Requires content-hashing of governed docs |

---

## 5. Milestone Completion Projection

| Milestone | Target Plans | Estimated Sessions |
|-----------|--------------|-------------------|
| Milestone 1 (MVP) | V7, V8, V9 | ~20-25 sessions |
| Production Ready | V10, V11 | ~15-20 sessions |
| Milestone 2 (External API) | V12+ | TBD |

---

## 6. Critical Path

```
V7 (Complete) → V8 (Complete) → V9 (Complete) → Milestone 1 COMPLETE
                                                        ↓
                                              V10 (Automation) → V11 (Hardening) → Production Ready
```

**Milestone 1 deliverables — ALL COMPLETE:**
1. ~~D-24 (Oracle runner)~~ — ✅ Complete (V8-1, V8-2)
2. ~~D-39 (Semantic oracle integration)~~ — ✅ Complete (V8-5)
3. ~~D-41 (Semantic worker)~~ — ✅ Complete (V9-1)
4. ~~D-36 (Replay proof)~~ — ✅ Complete (V9-3)

---

## 7. Maintenance Notes

This document should be updated when:
- A plan phase completes (mark deliverables as complete)
- A new plan is authored (add to roadmap section)
- Scope changes require deliverable reassignment
- Charter milestones are revised

**Update protocol:**
1. Update deliverable status table
2. Update Branch 0 acceptance criteria
3. Update milestone completion projection
4. Commit with message: `docs: update SR-PLAN-GAP-ANALYSIS after [plan/phase]`

# SR-PLAN Gap Analysis & Roadmap

**Purpose:** Track completion status of SR-PLAN deliverables and inform future implementation plans.

**Last Updated:** 2026-01-16
**Updated By:** Agent (V7-3 Attachment Backend session)

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
| Semantic oracle evidence capture and gating | ⚠️ Partial | D-24, D-25, D-39 | Evidence API exists; oracle runner missing |
| Explicit human authority points | ✅ Complete | D-19, D-30 | Approval recording, stage gating |

**Milestone 1 Completion:** ~80% — Missing oracle execution infrastructure.

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
| D-23 | Reference worker bridge | ❌ Not Started | — | No automated worker exists |

### PKG-08 — Oracles and verification substrate

| Deliverable | Title | Status | Completed In | Notes |
|-------------|-------|--------|--------------|-------|
| D-24 | Oracle runner service | ❌ **Not Started** | — | **Critical gap** — Podman + gVisor sandbox |
| D-25 | Core oracle suite | ❌ **Not Started** | — | Build/unit/schema/lint oracles |
| D-26 | Integration/E2E oracle suite | ❌ Not Started | — | Depends on D-24, D-25 |
| D-27 | Oracle integrity checks | ❌ **Not Started** | — | TAMPER/GAP/FLAKE/ENV_MISMATCH |

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
| D-36 | Replayability demonstration | ❌ Not Started | — | No formal replay proof |

### PKG-12 — Semantic work surface + prompt decomposition

| Deliverable | Title | Status | Completed In | Notes |
|-------------|-------|--------|--------------|-------|
| D-37 | Work surface schemas | ✅ Complete | V3, V4, V5 | Intake + Procedure Template + Work Surface |
| D-38 | Prompt → Plan Instance decomposition | ❌ Not Started | — | Automated work decomposition |
| D-39 | Semantic oracle integration | ❌ **Not Started** | — | Meaning matrices, residual/coverage artifacts |
| D-40 | Event Manager eligible-set computation | ❌ Not Started | — | Dependency-based scheduling |
| D-41 | Reference semantic worker | ❌ **Not Started** | — | Work Surface executor |

---

## 3. Branch 0 Acceptance Criteria

Per SR-PLAN §4.1, Branch 0 (Semantic Manifold MVP) requires:

| Criterion | Status | Gap |
|-----------|--------|-----|
| Loop created for problem-statement work unit | ✅ Complete | — |
| Iteration started with Work Surface ref set | ✅ Complete | — |
| Candidate intake bundle produced | ✅ Complete | — |
| Evidence Bundle from semantic oracle suite | ❌ **Not Done** | D-24, D-39 required |
| Human portal approval recorded | ✅ Complete | — |
| Freeze baseline created | ❌ Not Done | Freeze API exists but not integrated |
| Replay determinism proof | ❌ Not Done | D-36 required |

**Branch 0 is NOT complete.** Blocking items: D-24, D-39, D-36.

---

## 4. Roadmap: Remaining Plans

### SR-PLAN-V7 (Current)

**Status:** In progress (V7-1, V7-2, V7-3 complete; V7-4 next)
**Scope:** MVP Stabilization & Attachment Foundation

| Phase | Deliverables Addressed | Status |
|-------|------------------------|--------|
| V7-1 | Integration tests (stabilization) | ✅ Complete |
| V7-2 | Error handling & UX | ✅ Complete |
| V7-3 | Attachment backend (`record.attachment`) | ✅ Complete |
| V7-4 | Attachment frontend | ⏳ Next |
| V7-5 | Multiple iterations | ⏳ Pending |

### SR-PLAN-V8 (Proposed)

**Status:** Not yet authored
**Scope:** Oracle Runner & Semantic Suite Foundation
**Target Deliverables:** D-24, D-25, D-27, D-39

| Phase | Focus | Deliverables |
|-------|-------|--------------|
| V8-1 | Oracle runner service | D-24 |
| V8-2 | Core oracle suite | D-25 |
| V8-3 | Oracle integrity checks | D-27 |
| V8-4 | Semantic oracle integration | D-39 |

### SR-PLAN-V9 (Proposed)

**Status:** Not yet authored
**Scope:** Semantic Worker & Branch 0 Completion
**Target Deliverables:** D-23, D-41, D-36

| Phase | Focus | Deliverables |
|-------|-------|--------------|
| V9-1 | Reference worker bridge | D-23 |
| V9-2 | Reference semantic worker | D-41 |
| V9-3 | Replayability demonstration | D-36 |
| V9-4 | Branch 0 acceptance verification | — |

### SR-PLAN-V10 (Proposed)

**Status:** Not yet authored
**Scope:** Automation & Scheduling Foundation
**Target Deliverables:** D-38, D-40, D-22

| Phase | Focus | Deliverables |
|-------|-------|--------------|
| V10-1 | Loop governor service (full) | D-22 |
| V10-2 | Event Manager eligible-set | D-40 |
| V10-3 | Prompt → Plan decomposition | D-38 |

### SR-PLAN-V11 (Proposed)

**Status:** Not yet authored
**Scope:** Production Hardening
**Target Deliverables:** D-16, D-26, D-32, D-33, D-35

| Phase | Focus | Deliverables |
|-------|-------|--------------|
| V11-1 | Restricted evidence handling | D-16 |
| V11-2 | Integration/E2E oracle suite | D-26 |
| V11-3 | Build/init scripts completion | D-32 |
| V11-4 | Operational observability | D-33 |
| V11-5 | E2E failure mode harness | D-35 |

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
V7 (Stabilization) → V8 (Oracle Runner) → V9 (Semantic Worker) → Milestone 1 Complete
                                                                        ↓
                                                              V10 (Automation) → V11 (Hardening) → Production Ready
```

**Blocking dependencies for Milestone 1:**
1. D-24 (Oracle runner) — blocks all oracle execution
2. D-39 (Semantic oracle integration) — blocks semantic evidence
3. D-41 (Semantic worker) — blocks automated execution
4. D-36 (Replay proof) — blocks acceptance verification

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

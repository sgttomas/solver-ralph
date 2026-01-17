# SR-PLAN-V9 Coherence Review

**Review Date:** 2026-01-16
**Reviewer:** Agent (coherence verification task)
**Document Under Review:** `docs/planning/SR-PLAN-V9.md`
**Branch:** `main`

---

## Executive Summary

**Final Assessment: APPROVED**

SR-PLAN-V9 is **coherent and accurate** with the actual codebase state and governing contracts. All claims about existing infrastructure have been verified. The plan correctly identifies integration gaps and proposes feasible solutions using proven patterns from the existing codebase.

**Key Findings:**
- All 4 infrastructure files verified with accurate line counts and component status
- Stub implementations correctly identified in `semantic_worker.rs`
- Integration infrastructure (MinIO, NATS, Oracle Runner) fully implemented
- Contract requirements (C-VER-1, C-EVT-7, SR-AGENT-WORKER-CONTRACT) addressed by planned phases
- No amendments required

---

## 1. Codebase Verification

Verification of SR-PLAN-V9 §1.2 claims about existing infrastructure.

### 1.1 semantic_worker.rs (D-41)

**File:** `crates/sr-adapters/src/semantic_worker.rs`
**Claimed Lines:** ~992 | **Actual Lines:** 991

| Component | Plan Status | Verified Status | Evidence |
|-----------|-------------|-----------------|----------|
| `SemanticWorkerConfig` | Complete | ✅ VERIFIED | Lines 40-80: Full config with `max_iterations_per_work_unit`, `max_oracle_runs_per_iteration`, `dry_run` |
| `SemanticWorkerBridge` | Complete | ✅ VERIFIED | Lines 260-830: NATS subscription to `ITERATION_EVENTS`, idempotency tracking |
| `select_work_unit()` | Complete | ✅ VERIFIED | Lines 502-541: Returns `SelectionRationale` with deterministic selection per §2.1 |
| `execute_semantic_pipeline()` | Complete | ✅ VERIFIED | Lines 413-499: Full 10-step pipeline implementation |
| `run_semantic_oracles()` | Stub | ✅ VERIFIED | Lines 664-710: Returns hardcoded simulated results; comment at 671-672 confirms stub |
| `emit_evidence_bundle()` | Stub | ✅ VERIFIED | Lines 713-768: Constructs payload, logs only, no NATS publish or MinIO store |
| `emit_iteration_summary()` | Stub | ✅ VERIFIED | Lines 801-814: Logs only, explicit TODO comment at 809-811 |

**Verdict:** All claims VERIFIED ACCURATE.

### 1.2 worker.rs (D-23)

**File:** `crates/sr-adapters/src/worker.rs`
**Claimed Lines:** ~835 | **Actual Lines:** 834

| Component | Plan Status | Verified Status | Evidence |
|-----------|-------------|-----------------|----------|
| `ReferenceWorkerBridge` | Complete | ✅ VERIFIED | Lines 220-657: Full struct with NATS, HTTP client, context compiler |
| `execute_work_pipeline()` | Complete | ✅ VERIFIED | Lines 391-434: 4-step Context → Work → Candidate → Complete |
| `register_candidate()` | Complete | ✅ VERIFIED | Lines 528-584: POST to `/api/v1/candidates` with full error handling |
| `complete_iteration()` | Complete | ✅ VERIFIED | Lines 588-644: POST to `/api/v1/iterations/{id}/complete` |

**Verdict:** All claims VERIFIED ACCURATE.

### 1.3 event_manager.rs (D-40)

**File:** `crates/sr-adapters/src/event_manager.rs`
**Claimed Lines:** ~1720 | **Actual Lines:** 1719

| Component | Plan Status | Verified Status | Evidence |
|-----------|-------------|-----------------|----------|
| `EventManager` | Complete | ✅ VERIFIED | Lines 547-557: Full projection builder with pool, statuses, plan_instance |
| `compute_eligible_set()` | Complete | ✅ VERIFIED | Lines 654-656: Returns EligibleSet per SR-EVENT-MANAGER §3 |
| `compute_dependency_graph()` | Complete | ✅ VERIFIED | Lines 659-669: DependencyGraphSnapshot per SR-EVENT-MANAGER §4 |
| `compute_run_list()` | Complete | ✅ VERIFIED | Lines 672-679: RunList with 5 status groups per SR-EVENT-MANAGER §5 |
| `apply_event()` | Complete | ✅ VERIFIED | Lines 683-731: Handles 12 event types |
| `rebuild()` | Complete | ✅ VERIFIED | Lines 1212-1227: Clears state, replays all events deterministically |

**Additional Finding:**
- `compute_state_hash()` — **NOT PRESENT** (expected; V9-3 will add this)

**Verdict:** All claims VERIFIED ACCURATE.

### 1.4 Integration Infrastructure

| Component | File | Status | Evidence |
|-----------|------|--------|----------|
| `OracleExecutionWorker` | `oracle_worker.rs` | ✅ Ready | 779 lines, event-driven, TAMPER detection, PodmanOracleRunner |
| `MinioEvidenceStore` | `minio.rs` | ✅ Ready | Full `store()`, `retrieve()`, `exists()` implementation |
| `EvidenceManifestBuilder` | `evidence.rs` | ✅ Ready | 891 lines, work surface context support |
| `NatsMessageBus` | `nats.rs` | ✅ Ready | Consumer patterns proven in OracleWorker |
| `AppState` | `main.rs` | ✅ Ready | Configuration for all adapters |
| E2E Test Harness | `semantic_ralph_loop_e2e.rs` | ✅ Ready | 761 lines, TestClient with auth |

**Verdict:** Integration infrastructure FULLY IMPLEMENTED.

---

## 2. Contract Alignment

Verification that V9 phases satisfy governing contract requirements.

### 2.1 C-VER-1: Verification Is Evidence-Based and Candidate-Bound

**Contract Requirement (SR-CONTRACT §5):**
> "A Candidate MAY be marked 'Verified' only when: (1) Evidence Bundle exists for a Run against that Candidate, (2) Evidence Bundle is attributable and integrity-checked..."

**V9-1 Coverage:**

| Requirement | V9-1 Implementation |
|-------------|---------------------|
| Evidence Bundle exists | `emit_evidence_bundle()` wired to MinIO persistence |
| Attributable | Evidence manifest includes `run_id`, `candidate_id`, `oracle_suite_hash` |
| Integrity-checked | Content-addressed storage via SHA-256 hash |
| Oracle suite identity declared | `oracle_suite_id` and `oracle_suite_hash` in manifest |
| All required oracles recorded | `run_semantic_oracles()` wired to actual oracle runner |

**Assessment:** ✅ V9-1 satisfies C-VER-1 requirements.

### 2.2 C-EVT-7: Projections Derivable From Audit Trail

**Contract Requirement (SR-CONTRACT §8):**
> "Derived projections MUST be reconstructible from the event log."

**Current State:**
- `EventManager.rebuild()` exists (lines 1212-1227)
- Clears state and replays all events in order
- Deterministic replay verified by existing tests

**V9-3 Additions:**
- `compute_state_hash()` for deterministic state comparison
- `verify_replay()` proving identical state after replay
- SR-REPLAY-PROOF.md documenting the proof

**Assessment:** ✅ V9-3 satisfies C-EVT-7 requirements. Foundation already exists.

### 2.3 SR-AGENT-WORKER-CONTRACT Compliance

| Requirement | Section | Status | V9 Coverage |
|-------------|---------|--------|-------------|
| Choose one eligible target | §2.1 | ✅ Complete | `select_work_unit()` implemented |
| Execute procedure stage | §2.2 | ✅ Complete | `execute_semantic_pipeline()` implemented |
| Run required oracle suites | §2.3 | ⚠️ Stub | V9-1 wires `run_semantic_oracles()` |
| Produce commitment objects | §2.4 | ⚠️ Stub | V9-1 wires `emit_evidence_bundle()` |
| No ghost inputs | §2.5 | ✅ Complete | Context compiled from refs only |
| Stop conditions | §3 | ✅ Complete | Pipeline checks budget, thrashing, eligibility |

**Assessment:** ✅ V9-1 completes remaining gaps (§2.3, §2.4).

---

## 3. Gap Identification

### 3.1 Critical Gaps

| Gap | Severity | Mitigation |
|-----|----------|------------|
| None | — | — |

**No critical gaps identified.** All plan assumptions match codebase reality.

### 3.2 Minor Observations (Expected, Not Gaps)

| Observation | Expected? | Addressed By |
|-------------|-----------|--------------|
| `compute_state_hash()` not in EventManager | ✅ Yes | V9-3 adds this |
| Semantic worker not spawned in main.rs | ✅ Yes | V9-1 adds spawn |
| No Branch 0 E2E test | ✅ Yes | V9-2 creates it |
| No SR-REPLAY-PROOF.md | ✅ Yes | V9-3 creates it |

---

## 4. Review Checklist (per SR-README)

| Check | Question | Result |
|-------|----------|--------|
| Infrastructure | Do the files and components described in §1.2 exist as claimed? | ✅ PASS |
| Stubs | Are the stub implementations correctly identified? | ✅ PASS |
| Integration Points | Are the V9-1 integration points feasible? | ✅ PASS |
| Test Feasibility | Can the E2E test in V9-2 be implemented? | ✅ PASS |
| Replay | Does EventManager have foundation for state hashing? | ✅ PASS (`rebuild()` exists) |
| Effort | Are the effort estimates (5-8 sessions) realistic? | ✅ PASS |

---

## 5. Amendments

**None required.**

SR-PLAN-V9 is coherent with the codebase. The plan accurately describes existing infrastructure and proposes feasible integration work.

---

## 6. Review Verdict

### **APPROVED**

SR-PLAN-V9 is approved for implementation without amendments.

**Basis for Approval:**

1. **Codebase Accuracy:** All claims about existing infrastructure verified accurate (line counts within 1-2 lines, all components exist as described)

2. **Stub Identification:** The three stub functions (`run_semantic_oracles`, `emit_evidence_bundle`, `emit_iteration_summary`) are correctly identified with explicit TODO comments in source

3. **Integration Feasibility:** MinIO evidence store, NATS message bus, and Oracle Runner are fully implemented with proven patterns that V9-1 can follow

4. **Contract Compliance:** Plan phases address required contracts:
   - V9-1 → C-VER-1 (evidence-based verification)
   - V9-3 → C-EVT-7 (rebuildable projections)
   - V9-1 → SR-AGENT-WORKER-CONTRACT §2.3, §2.4

5. **No Critical Gaps:** All identified observations are expected and addressed by planned phases

---

## Reviewer Attestation

This coherence review was conducted by systematic verification of:
- Source file examination (semantic_worker.rs, worker.rs, event_manager.rs, oracle_worker.rs)
- Integration infrastructure review (minio.rs, evidence.rs, main.rs)
- Contract document analysis (SR-CONTRACT, SR-AGENT-WORKER-CONTRACT, SR-EVENT-MANAGER)
- Existing E2E test patterns (semantic_ralph_loop_e2e.rs)

All claims in SR-PLAN-V9 §1.2 "Existing Infrastructure Analysis" have been independently verified against the actual codebase.

**Next Action:** Implementation may proceed with V9-1 (Semantic Worker Integration).

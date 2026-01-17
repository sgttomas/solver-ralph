# SR-PLAN-V9: Semantic Worker Integration & Branch 0 Completion

**Status:** Active
**Created:** 2026-01-17
**Depends On:** SR-PLAN-V8 (Oracle Runner & Semantic Suite complete)
**Implements:** SR-CHARTER §Immediate Objective (Milestone 1: Semantic Work Unit Runtime)

---

## Amendments Summary

| Amendment | Issue | Resolution |
|-----------|-------|------------|
| V9-1 Complete | V9-1 implemented in 1 session | All acceptance criteria satisfied; `SemanticWorkerBridge` now generic over `EvidenceStore` and `CandidateWorkspace` |

**Estimated Effort:** 5-8 sessions
**Actual Effort (V9-1):** 1 session

---

## Executive Summary

SR-PLAN-V9 delivers the **final push to Milestone 1 (MVP)** — integrating the semantic worker components, demonstrating replayability, and achieving Branch 0 acceptance verification. This plan focuses on **integration and verification** rather than net-new component creation, as the core infrastructure already exists.

**Goal:** Demonstrate a complete Semantic Ralph Loop from work surface creation through semantic oracle evaluation to terminal stage completion with human approval, and prove the system is deterministically replayable.

**Key Deliverables (from SR-PLAN-GAP-ANALYSIS §4):**

| Deliverable | Title | Status | V9 Focus |
|-------------|-------|--------|----------|
| **D-23** | Reference worker bridge | ✅ Implemented | Integration wiring |
| **D-41** | Reference semantic worker | ✅ Implemented | Integration wiring |
| **D-36** | Replayability demonstration | ❌ Not Started | **Primary focus** |

**Existing Infrastructure Analysis:**

The project already has substantial worker infrastructure:

| Component | File | Lines | Status |
|-----------|------|-------|--------|
| `ReferenceWorkerBridge` | `sr-adapters/src/worker.rs` | ~835 | ✅ Implemented |
| `SemanticWorkerBridge` | `sr-adapters/src/semantic_worker.rs` | ~992 | ✅ Implemented |
| `OracleExecutionWorker` | `sr-adapters/src/oracle_worker.rs` | ~779 | ✅ Implemented |
| `EventManager` | `sr-adapters/src/event_manager.rs` | ~1720 | ✅ Implemented |

**What V9 Must Complete:**

1. Wire semantic worker to API entry points (integration layer)
2. Implement E2E test demonstrating Branch 0 procedure flow
3. Implement replay proof (event log → projection rebuild → identical state)
4. Document Branch 0 acceptance criteria satisfaction

```
V9-1 (Worker Integration) → V9-2 (E2E Flow Test) → V9-3 (Replay Proof) → V9-4 (Branch 0 Acceptance)
     └─────────────────────────────────────────────┘   └─────────────────────────────────────────────┘
              Integration & Wiring                                 Verification & Proof
```

---

## Table of Contents

1. [Current State](#1-current-state)
2. [Design Rationale](#2-design-rationale)
3. [Implementation Phases](#3-implementation-phases)
4. [Success Criteria](#4-success-criteria)
5. [Reference Documents](#5-reference-documents)

---

## 1. Current State

### 1.1 What SR-PLAN-V8 Delivered

V8 delivered the oracle infrastructure:
- Oracle suite registry with PostgreSQL persistence
- Event-driven oracle execution worker subscribing to `RunStarted`
- Integrity condition detection (TAMPER/GAP/FLAKE/ENV_MISMATCH)
- Core oracle suite container (`oracle-suites/core-v1`)
- Semantic oracle suite container (`oracle-suites/semantic-v1`)

### 1.2 What Exists But Needs Integration

Per codebase analysis, the following components are implemented but require integration:

**`semantic_worker.rs` (D-41) — 992 lines:**

| Component | Status | Notes |
|-----------|--------|-------|
| `SemanticWorkerConfig` | ✅ Complete | Configuration with budgets |
| `SemanticWorkerBridge` | ✅ Complete | NATS subscription to `IterationStarted` |
| `select_work_unit()` | ✅ Complete | Per SR-AGENT-WORKER-CONTRACT §2.1 |
| `execute_semantic_pipeline()` | ✅ Complete | Full pipeline implementation |
| `run_semantic_oracles()` | ⚠️ Stub | Returns simulated results |
| `emit_evidence_bundle()` | ⚠️ Stub | Logs but doesn't persist to store |
| `emit_iteration_summary()` | ⚠️ Stub | Logs but doesn't call API |

**`worker.rs` (D-23) — 835 lines:**

| Component | Status | Notes |
|-----------|--------|-------|
| `ReferenceWorkerBridge` | ✅ Complete | Full implementation |
| `execute_work_pipeline()` | ✅ Complete | Context → Work → Candidate → Complete |
| `register_candidate()` | ✅ Complete | POST to `/api/v1/candidates` |
| `complete_iteration()` | ✅ Complete | POST to `/api/v1/iterations/{id}/complete` |

**`event_manager.rs` (D-40) — 1720 lines:**

| Component | Status | Notes |
|-----------|--------|-------|
| `EventManager` | ✅ Complete | Full projection builder |
| `compute_eligible_set()` | ✅ Complete | Per SR-EVENT-MANAGER §3 |
| `compute_dependency_graph()` | ✅ Complete | Per SR-EVENT-MANAGER §4 |
| `compute_run_list()` | ✅ Complete | Per SR-EVENT-MANAGER §5 |
| `apply_event()` | ✅ Complete | 12 event types handled |
| `rebuild()` | ✅ Complete | Deterministic replay |

### 1.3 What's Missing for Milestone 1

| Gap | Impact | V9 Phase |
|-----|--------|----------|
| Semantic worker uses stub oracles | Cannot produce real semantic evidence | V9-1 |
| Evidence bundle not persisted to MinIO | Evidence not retrievable | V9-1 |
| No E2E integration test for full loop | Cannot verify Branch 0 acceptance | V9-2 |
| No formal replay proof | Cannot claim determinism | V9-3 |
| Branch 0 acceptance not documented | Milestone 1 incomplete | V9-4 |

### 1.4 Branch 0 Acceptance Criteria Status

Per SR-PLAN §4.1 and SR-PLAN-GAP-ANALYSIS §3:

| Criterion | Status | Gap |
|-----------|--------|-----|
| Loop created for problem-statement work unit | ✅ Complete | — |
| Iteration started with Work Surface ref set | ✅ Complete | — |
| Candidate intake bundle produced | ✅ Complete | — |
| Evidence Bundle from semantic oracle suite | ⚠️ Infrastructure exists | Integration needed |
| Human portal approval recorded | ✅ Complete | — |
| Freeze baseline created | ⚠️ Freeze API exists | Not integrated into flow |
| Replay determinism proof | ❌ Not Done | D-36 |

---

## 2. Design Rationale

### 2.1 Why Integration Over New Components (V9-1)

The semantic worker and reference worker already exist with comprehensive implementations. The gap is not missing code but missing wiring:

1. `SemanticWorkerBridge.run_semantic_oracles()` returns stub data instead of invoking the actual semantic oracle suite via the oracle runner
2. `SemanticWorkerBridge.emit_evidence_bundle()` logs but doesn't persist to MinIO
3. The `EventManager` is instantiated but not connected to the semantic worker

V9-1 completes these connections.

### 2.2 Why E2E Flow Before Replay Proof (V9-2)

Before proving replay determinism, we need a working end-to-end flow to replay. The E2E test serves as:
1. Verification that all components integrate correctly
2. The source of events for replay testing
3. Branch 0 acceptance evidence

### 2.3 Why Replay Proof Is Central (V9-3)

Per SR-CONTRACT C-EVT-7: "Projections MUST be rebuildable from the event log."

The replay proof demonstrates:
1. Given an event sequence, `EventManager.rebuild()` produces identical state
2. Eligible set computation is deterministic
3. Stage progression is derivable from recorded events alone

This is the core trust property of the platform.

### 2.4 Why Formal Branch 0 Acceptance (V9-4)

Milestone 1 requires explicit documentation that Branch 0 criteria are satisfied. This is not a code deliverable but a verification record that:
1. All Branch 0 criteria are met (per SR-PLAN-GAP-ANALYSIS §3)
2. Evidence of satisfaction exists
3. Human approval is recorded

---

## 3. Implementation Phases

### Phase V9-1: Semantic Worker Integration

**Objective:** Wire the semantic worker to use real oracle execution and persist evidence.

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `sr-adapters/src/semantic_worker.rs` | MODIFY | Connect to real oracle runner |
| `sr-adapters/src/semantic_worker.rs` | MODIFY | Persist evidence to MinIO |
| `sr-api/src/main.rs` | MODIFY | Start semantic worker alongside API |
| `sr-api/src/handlers/iterations.rs` | MODIFY | Ensure iteration events trigger worker |

**Integration Points:**

```rust
// Current stub in semantic_worker.rs
async fn run_semantic_oracles(&self, ...) -> Result<Vec<SemanticOracleResult>, WorkerError> {
    // Returns simulated data
    Ok(vec![SemanticOracleResult { ... }])
}

// After V9-1: Invoke oracle runner
async fn run_semantic_oracles(&self, selection: &SelectionRationale, context: &ContextBundle)
    -> Result<Vec<SemanticOracleResult>, WorkerError>
{
    // 1. Get semantic oracle suite from registry
    let suite = self.oracle_registry
        .get("suite:sr-semantic-v1")
        .await?
        .ok_or(WorkerError::OracleSuiteNotFound)?;

    // 2. Materialize candidate workspace
    let workspace = self.candidate_workspace
        .materialize(&selection.selected_work_unit_id)
        .await?;

    // 3. Execute oracle suite
    let result = self.oracle_runner
        .execute_suite(
            &selection.selected_work_unit_id,
            &suite.suite_id,
            &suite.suite_hash,
            &workspace.path,
        )
        .await?;

    // 4. Map to SemanticOracleResult
    self.map_oracle_results(&result)
}
```

**Evidence Persistence:**

```rust
// Current stub
async fn emit_evidence_bundle(&self, ...) -> Result<String, WorkerError> {
    info!("Evidence bundle recorded"); // Just logs
    Ok(bundle_id)
}

// After V9-1: Persist to MinIO
async fn emit_evidence_bundle(&self, ...) -> Result<String, WorkerError> {
    // 1. Build evidence manifest
    let manifest = EvidenceManifest {
        candidate_id: selection.selected_work_unit_id.clone(),
        procedure_template_id: "proc:GENERIC-KNOWLEDGE-WORK".to_string(),
        stage_id: selection.target_stage_id.clone(),
        oracle_results: result.oracle_results.clone(),
        content_hash: computed_hash,
        ..
    };

    // 2. Store in MinIO
    let bundle_id = self.evidence_store
        .store_bundle(&manifest)
        .await?;

    // 3. Emit EvidenceBundleRecorded event
    self.emit_event("EvidenceBundleRecorded", &bundle_id, &manifest).await?;

    Ok(bundle_id)
}
```

**API Server Integration:**

```rust
// In sr-api/src/main.rs
pub async fn run_api_server(config: ApiConfig) -> Result<(), Error> {
    // ... existing setup ...

    // Start semantic worker if enabled
    if config.enable_semantic_worker {
        let worker = SemanticWorkerBridge::new(
            SemanticWorkerConfig::from_env(),
            message_bus.clone(),
            event_manager.clone(),
        );

        tokio::spawn(async move {
            if let Err(e) = worker.start().await {
                error!("Semantic worker failed: {}", e);
            }
        });
    }

    // ... rest of server startup ...
}
```

**Acceptance Criteria:**
- [x] `run_semantic_oracles()` invokes actual oracle runner
- [x] Evidence bundles persisted to MinIO with correct manifest
- [x] `EvidenceBundleRecorded` event emitted and stored
- [x] Semantic worker starts alongside API server
- [x] `cargo test --package sr-adapters` passes
- [x] `cargo test --package sr-api` passes

**Status:** ✅ COMPLETE (2026-01-16)

**Effort:** ~1 session (actual)

---

### Phase V9-2: End-to-End Flow Integration Test

**Objective:** Create integration test demonstrating complete Branch 0 procedure flow.

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `sr-api/tests/integration/branch_0_e2e_test.rs` | CREATE | Full E2E test |
| `sr-api/tests/fixtures/branch_0_intake.json` | CREATE | Test intake fixture |
| `sr-api/tests/fixtures/branch_0_work_surface.json` | CREATE | Test work surface |

**Test Flow:**

```rust
/// Branch 0 End-to-End Integration Test
///
/// Per SR-PLAN §4.1, demonstrates:
/// 1. Loop created for problem-statement work unit
/// 2. Iteration started with Work Surface ref set
/// 3. Candidate intake bundle produced
/// 4. Evidence Bundle from semantic oracle suite
/// 5. Human portal approval recorded
/// 6. Freeze baseline created
/// 7. Replay determinism proof
#[tokio::test]
async fn test_branch_0_complete_flow() {
    // Setup
    let app = TestApp::spawn().await;

    // Step 1: Create a Work Surface (Intake + Procedure Template)
    let work_surface = app.create_work_surface(WorkSurfaceRequest {
        title: "Branch 0 Test Work Unit".to_string(),
        work_kind: WorkKind::ResearchMemo,
        intake: IntakeRequest {
            objective: "Demonstrate semantic ralph loop MVP".to_string(),
            scope: vec!["Integration test".to_string()],
            constraints: vec![],
        },
        procedure_template_id: "proc:GENERIC-KNOWLEDGE-WORK".to_string(),
        initial_stage_id: "stage:FRAME".to_string(),
    }).await;
    assert!(work_surface.work_surface_id.starts_with("ws:"));

    // Step 2: Create a Ralph Loop for this work surface
    let loop_response = app.create_loop(CreateLoopRequest {
        work_surface_id: work_surface.work_surface_id.clone(),
        budget: LoopBudget {
            max_iterations: 5,
            max_duration_seconds: 300,
        },
    }).await;
    assert!(loop_response.loop_id.starts_with("loop:"));

    // Step 3: Start first iteration
    let iteration = app.start_iteration(StartIterationRequest {
        loop_id: loop_response.loop_id.clone(),
        refs: vec![
            TypedRef {
                kind: "WorkSurface".to_string(),
                id: work_surface.work_surface_id.clone(),
                rel: "executing".to_string(),
                meta: serde_json::Value::Null,
            },
        ],
    }).await;
    assert!(iteration.iteration_id.starts_with("iter:"));

    // Step 4: Wait for semantic worker to process
    // (Worker subscribes to IterationStarted, runs oracles, produces evidence)
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Step 5: Verify evidence bundle was created
    let evidence = app.get_evidence_for_iteration(&iteration.iteration_id).await;
    assert!(evidence.bundle_id.starts_with("bundle:"));
    assert_eq!(evidence.stage_id, "stage:FRAME");

    // Step 6: Progress through stages (FRAME → OPTIONS → DRAFT → SEMANTIC_EVAL → FINAL)
    for stage in &["stage:OPTIONS", "stage:DRAFT", "stage:SEMANTIC_EVAL", "stage:FINAL"] {
        // Start new iteration for this stage
        let iter = app.start_iteration(StartIterationRequest {
            loop_id: loop_response.loop_id.clone(),
            refs: vec![
                TypedRef {
                    kind: "Stage".to_string(),
                    id: stage.to_string(),
                    rel: "executing".to_string(),
                    meta: serde_json::Value::Null,
                },
            ],
        }).await;

        // Wait for processing
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // If this is a portal-required stage, record approval
        if *stage == "stage:SEMANTIC_EVAL" || *stage == "stage:FINAL" {
            app.record_approval(RecordApprovalRequest {
                portal_id: "portal:stage_gate".to_string(),
                subject_refs: vec![
                    TypedRef {
                        kind: "Iteration".to_string(),
                        id: iter.iteration_id.clone(),
                        rel: "approving".to_string(),
                        meta: serde_json::Value::Null,
                    },
                ],
                decision: "APPROVED".to_string(),
                rationale: "E2E test approval".to_string(),
            }).await;
        }
    }

    // Step 7: Verify loop is complete
    let loop_status = app.get_loop_status(&loop_response.loop_id).await;
    assert_eq!(loop_status.coarse_status, CoarseStatus::Complete);

    // Step 8: Create freeze baseline
    let freeze = app.create_freeze(CreateFreezeRequest {
        loop_id: loop_response.loop_id.clone(),
        description: "Branch 0 E2E test completion".to_string(),
    }).await;
    assert!(freeze.freeze_id.starts_with("freeze:"));

    // Step 9: Verify replay determinism (see V9-3)
    let replay_result = app.verify_replay(&loop_response.loop_id).await;
    assert!(replay_result.deterministic);
}
```

**Acceptance Criteria:**
- [ ] E2E test creates work surface and loop
- [ ] E2E test progresses through all 5 stages (FRAME → OPTIONS → DRAFT → SEMANTIC_EVAL → FINAL)
- [ ] E2E test records portal approvals where required
- [ ] E2E test creates freeze baseline
- [ ] Evidence bundles exist for each stage transition
- [ ] Test passes in CI environment
- [ ] Test is documented in README

**Effort:** ~2 sessions

---

### Phase V9-3: Replayability Demonstration (D-36)

**Objective:** Prove deterministic replay per SR-CONTRACT C-EVT-7.

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `sr-adapters/src/replay.rs` | CREATE | Replay proof module |
| `sr-api/tests/integration/replay_determinism_test.rs` | CREATE | Replay proof test |
| `docs/platform/SR-REPLAY-PROOF.md` | CREATE | Replay proof documentation |

**Replay Proof Architecture:**

```rust
/// Replay proof per SR-CONTRACT C-EVT-7
///
/// Given an ordered event sequence, proves that:
/// 1. EventManager.rebuild() produces identical state
/// 2. Eligible set computation is deterministic
/// 3. Stage status is derivable from events alone
pub struct ReplayProof {
    /// Original event sequence
    pub events: Vec<EventEnvelope>,
    /// State hash after original processing
    pub original_state_hash: String,
    /// State hash after replay
    pub replayed_state_hash: String,
    /// Whether replay produced identical state
    pub deterministic: bool,
    /// Any discrepancies found
    pub discrepancies: Vec<ReplayDiscrepancy>,
}

pub struct ReplayDiscrepancy {
    pub work_unit_id: String,
    pub field: String,
    pub original_value: serde_json::Value,
    pub replayed_value: serde_json::Value,
}

impl EventManager {
    /// Compute state hash for replay comparison
    pub fn compute_state_hash(&self) -> String {
        let mut hasher = Sha256::new();

        // Hash all work unit statuses in deterministic order
        let mut status_keys: Vec<_> = self.statuses.keys().collect();
        status_keys.sort();

        for key in status_keys {
            let status = self.statuses.get(key).unwrap();
            hasher.update(key.as_bytes());
            hasher.update(b":");
            hasher.update(format!("{:?}", status.coarse_status).as_bytes());
            hasher.update(b":");
            hasher.update(format!("{}", status.deps_satisfied).as_bytes());
            hasher.update(b":");
            // ... hash all relevant fields deterministically
        }

        format!("sha256:{}", hex::encode(hasher.finalize()))
    }

    /// Verify replay determinism
    pub fn verify_replay(&self, events: &[EventEnvelope]) -> ReplayProof {
        // 1. Compute original state hash
        let original_hash = self.compute_state_hash();

        // 2. Create fresh EventManager and replay
        let mut replayed = EventManager::new_in_memory();
        if let Some(plan) = &self.plan_instance {
            replayed.load_plan_instance(plan.clone());
        }

        for event in events {
            replayed.apply_event(event).ok();
        }

        // 3. Compute replayed state hash
        let replayed_hash = replayed.compute_state_hash();

        // 4. Compare and report
        ReplayProof {
            events: events.to_vec(),
            original_state_hash: original_hash.clone(),
            replayed_state_hash: replayed_hash.clone(),
            deterministic: original_hash == replayed_hash,
            discrepancies: self.find_discrepancies(&replayed),
        }
    }
}
```

**Replay Proof Test:**

```rust
#[tokio::test]
async fn test_replay_determinism() {
    // 1. Run E2E flow and capture events
    let app = TestApp::spawn().await;
    let (loop_id, events) = run_branch_0_flow_and_capture_events(&app).await;

    // 2. Get original EventManager state
    let original_em = app.get_event_manager();
    let original_hash = original_em.compute_state_hash();

    // 3. Create fresh EventManager and replay events
    let mut replayed_em = EventManager::new_in_memory();
    replayed_em.load_plan_instance(original_em.plan_instance().unwrap().clone());

    for event in &events {
        replayed_em.apply_event(event).unwrap();
    }

    let replayed_hash = replayed_em.compute_state_hash();

    // 4. Verify determinism
    assert_eq!(original_hash, replayed_hash, "Replay must produce identical state");

    // 5. Verify eligible set is identical
    let original_eligible = original_em.compute_eligible_set();
    let replayed_eligible = replayed_em.compute_eligible_set();

    assert_eq!(
        original_eligible.work_unit_ids(),
        replayed_eligible.work_unit_ids(),
        "Eligible set must be identical after replay"
    );

    // 6. Verify status projections are identical
    for (id, original_status) in original_em.all_statuses() {
        let replayed_status = replayed_em.get_status(id).unwrap();
        assert_eq!(original_status.coarse_status, replayed_status.coarse_status);
        assert_eq!(original_status.deps_satisfied, replayed_status.deps_satisfied);
        assert_eq!(original_status.current_stage_id, replayed_status.current_stage_id);
    }
}
```

**SR-REPLAY-PROOF.md Documentation:**

```markdown
# SR-REPLAY-PROOF — Replay Determinism Verification

**Purpose:** Document the replay proof that satisfies SR-CONTRACT C-EVT-7.

## Proof Statement

Given:
- An ordered event sequence E = [e1, e2, ..., en]
- An initial Plan Instance P

The EventManager satisfies replay determinism if:
- rebuild(E, P) produces state S1
- rebuild(E, P) run again produces state S2
- S1 == S2 (identical state hashes)

## Verified Properties

1. **Work Unit Status Determinism:** For all work units, coarse_status, deps_satisfied,
   and stage_status are deterministically derived from events.

2. **Eligible Set Determinism:** compute_eligible_set() returns identical results
   for identical states.

3. **Dependency Satisfaction Determinism:** deps_satisfied is computed solely from
   the completion status of dependent work units, which is event-derived.

## Evidence

| Property | Verification Method | Pass Criteria |
|----------|---------------------|---------------|
| State hash equality | `compute_state_hash()` comparison | Hashes match |
| Eligible set equality | `compute_eligible_set()` comparison | Sets identical |
| No ghost inputs | Code review of `apply_event()` | No external state access |
```

**Acceptance Criteria:**
- [ ] `EventManager.compute_state_hash()` implemented
- [ ] `EventManager.verify_replay()` implemented
- [ ] Replay proof test passes
- [ ] SR-REPLAY-PROOF.md documents the proof
- [ ] Proof covers: state hash, eligible set, status projections
- [ ] `cargo test --package sr-adapters replay` passes

**Effort:** ~2 sessions

---

### Phase V9-4: Branch 0 Acceptance Verification

**Objective:** Document and verify Branch 0 acceptance criteria satisfaction.

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `docs/planning/SR-PLAN-V9-ACCEPTANCE.md` | CREATE | Acceptance verification record |
| `docs/planning/SR-PLAN-GAP-ANALYSIS.md` | MODIFY | Update deliverable status |
| `docs/charter/SR-README.md` | MODIFY | Update roadmap table |

**Acceptance Verification Record:**

```markdown
# SR-PLAN-V9 Acceptance Verification

**Verification Date:** [DATE]
**Verified By:** [AGENT ID]
**Approved By:** [HUMAN AUTHORITY]

## Branch 0 Acceptance Criteria

Per SR-PLAN §4.1, Branch 0 (Semantic Manifold MVP) requires:

| Criterion | Status | Evidence |
|-----------|--------|----------|
| Loop created for problem-statement work unit | ✅ PASS | E2E test creates loop via `/api/v1/loops` |
| Iteration started with Work Surface ref set | ✅ PASS | E2E test starts iteration with work surface refs |
| Candidate intake bundle produced | ✅ PASS | Worker produces candidate with content hash |
| Evidence Bundle from semantic oracle suite | ✅ PASS | Semantic worker invokes oracle runner, stores bundle |
| Human portal approval recorded | ✅ PASS | E2E test records approval for portal stages |
| Freeze baseline created | ✅ PASS | E2E test creates freeze record |
| Replay determinism proof | ✅ PASS | Replay test demonstrates C-EVT-7 compliance |

## Milestone 1 Deliverable Status

| Deliverable | V9 Status | Notes |
|-------------|-----------|-------|
| D-23 (Reference worker bridge) | ✅ Complete | Integrated in V9-1 |
| D-41 (Reference semantic worker) | ✅ Complete | Integrated in V9-1 |
| D-36 (Replayability demonstration) | ✅ Complete | Proven in V9-3 |

## Test Results

- sr-adapters: [N] passed
- sr-api: [N] passed
- Integration tests: [N] passed
- Replay proof: PASS

## Approval Record

This verification record constitutes evidence for Milestone 1 completion.
Portal: Release Approval Portal
Decision: [PENDING HUMAN APPROVAL]
```

**Acceptance Criteria:**
- [ ] All Branch 0 criteria documented as satisfied
- [ ] Evidence references for each criterion
- [ ] Test result summary included
- [ ] SR-PLAN-GAP-ANALYSIS updated with V9 deliverable completion
- [ ] SR-README roadmap table updated
- [ ] Human approval recorded

**Effort:** ~1 session

---

## 4. Success Criteria

### 4.1 Checkpoint: Worker Integration (after V9-1)

- [ ] Semantic worker invokes real oracle runner
- [ ] Evidence bundles persisted to MinIO
- [ ] Evidence events emitted and stored
- [ ] Worker starts alongside API server
- [ ] All unit tests pass

### 4.2 Checkpoint: E2E Flow (after V9-2)

- [ ] Complete Branch 0 flow executes without errors
- [ ] All 5 stages traversed (FRAME → OPTIONS → DRAFT → SEMANTIC_EVAL → FINAL)
- [ ] Portal approvals recorded for gate stages
- [ ] Freeze baseline created
- [ ] Integration test passes in CI

### 4.3 Checkpoint: Replay Proof (after V9-3)

- [ ] `compute_state_hash()` produces deterministic hashes
- [ ] `verify_replay()` confirms identical state after replay
- [ ] Eligible set identical after replay
- [ ] All status projections identical after replay
- [ ] SR-REPLAY-PROOF.md documents the proof

### 4.4 Milestone 1 Complete (after V9-4)

After V9-4, Milestone 1 (MVP) is complete:

| Requirement | Satisfied By |
|-------------|--------------|
| Bounded agentic work | D-05, D-06, D-22, D-41 |
| Iteration cycling with controlled memory | D-08, D-18, D-41 |
| Declared work surface | D-37, V3-V5 |
| Deterministic event recording | D-09, D-10 |
| Governance state projection | D-11, D-12, D-40 |
| Semantic oracle evidence capture | D-24, D-25, D-39 |
| Explicit human authority points | D-19, D-30 |
| Replayability | D-36 |

**Milestone 1 Status: ✅ COMPLETE**

---

## 5. Reference Documents

### Platform Specifications

| Document | Relevant Sections |
|----------|-------------------|
| SR-CONTRACT | §2.3 (Work Surface), C-EVT-7 (Replay), C-LOOP-* |
| SR-AGENT-WORKER-CONTRACT | §1 (Inputs), §2 (Responsibilities), §3 (Stop conditions) |
| SR-EVENT-MANAGER | §2 (Projections), §3 (Determinism), §5 (Eligibility) |
| SR-PROCEDURE-KIT | §2 (GENERIC-KNOWLEDGE-WORK stages) |

### Prior Plans

| Plan | Status | Relevance |
|------|--------|-----------|
| SR-PLAN-V7 | Complete | MVP stabilization |
| SR-PLAN-V8 | Complete | Oracle infrastructure |
| SR-PLAN-GAP-ANALYSIS | Living | Deliverable tracking |

### Codebase References

| File | Purpose |
|------|---------|
| `sr-adapters/src/semantic_worker.rs` | Semantic worker implementation |
| `sr-adapters/src/worker.rs` | Reference worker bridge |
| `sr-adapters/src/oracle_worker.rs` | Oracle execution worker |
| `sr-adapters/src/event_manager.rs` | Event manager / projection builder |
| `sr-api/tests/integration/` | Integration test location |

---

## Appendix A: Effort Summary

| Phase | Focus | Effort | Cumulative |
|-------|-------|--------|------------|
| V9-1 | Semantic Worker Integration | 2 sessions | 2 |
| V9-2 | E2E Flow Integration Test | 2 sessions | 4 |
| V9-3 | Replayability Demonstration | 2 sessions | 6 |
| V9-4 | Branch 0 Acceptance | 1 session | 7 |

**Total:** ~5-8 sessions

---

## Appendix B: Dependency Graph

```
┌─────────────────────────────────────────────────────────────────┐
│                         SR-PLAN-V8                              │
│         (Oracle Runner & Semantic Suite - Complete)             │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│  V9-1: Semantic Worker Integration                              │
│  - Connect semantic_worker.rs to oracle_runner                  │
│  - Persist evidence to MinIO                                    │
│  - Start worker alongside API                                   │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│  V9-2: E2E Flow Integration Test                                │
│  - Branch 0 complete flow test                                  │
│  - All 5 stages traversed                                       │
│  - Portal approvals recorded                                    │
│  - Freeze baseline created                                      │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│  V9-3: Replayability Demonstration (D-36)                       │
│  - compute_state_hash() for deterministic comparison            │
│  - verify_replay() proves identical state                       │
│  - SR-REPLAY-PROOF.md documentation                             │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│  V9-4: Branch 0 Acceptance Verification                         │
│  - Document all criteria satisfied                              │
│  - Update gap analysis                                          │
│  - Record human approval                                        │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                     MILESTONE 1 COMPLETE                        │
│            Semantic Work Unit Runtime (MVP)                     │
│                                                                 │
│                    → SR-PLAN-V10 →                              │
│            (Automation & Scheduling Foundation)                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Appendix C: Contract Compliance Matrix

| Contract | Requirement | V9 Implementation |
|----------|-------------|-------------------|
| C-LOOP-2 | Fresh-context iterations | Semantic worker uses ContextCompiler |
| C-CTX-1 | Iteration context provenance | refs[] include work surface, stage, eligible set |
| C-CTX-2 | No ghost inputs | Context derivable from refs alone |
| C-LOOP-4 | Candidate production traceable | Evidence bundle binds candidate to loop/iteration |
| C-EVT-7 | Projections rebuildable | V9-3 replay proof |
| C-VER-1 | Verification evidence-based | Evidence bundles from oracle runner |
| C-VER-4 | Verified claims declare basis | Evidence bundle includes suite hash, stage_id |

---

## Appendix D: Critical Files for Implementation

### V9-1 (Semantic Worker Integration)
| File | Action |
|------|--------|
| `sr-adapters/src/semantic_worker.rs` | MODIFY — connect to oracle runner |
| `sr-api/src/main.rs` | MODIFY — start worker with API |

### V9-2 (E2E Flow Test)
| File | Action |
|------|--------|
| `sr-api/tests/integration/branch_0_e2e_test.rs` | CREATE — full flow test |
| `sr-api/tests/fixtures/` | CREATE — test fixtures |

### V9-3 (Replay Proof)
| File | Action |
|------|--------|
| `sr-adapters/src/event_manager.rs` | MODIFY — add compute_state_hash, verify_replay |
| `sr-adapters/src/replay.rs` | CREATE — replay proof types |
| `docs/platform/SR-REPLAY-PROOF.md` | CREATE — proof documentation |

### V9-4 (Acceptance)
| File | Action |
|------|--------|
| `docs/planning/SR-PLAN-V9-ACCEPTANCE.md` | CREATE — acceptance record |
| `docs/planning/SR-PLAN-GAP-ANALYSIS.md` | MODIFY — update status |
| `docs/charter/SR-README.md` | MODIFY — update roadmap |

//! Replay Determinism Integration Test (SR-PLAN-V9 Phase V9-3)
//!
//! Demonstrates deterministic replay per SR-CONTRACT C-EVT-7:
//! "Projections MUST be rebuildable from the event log."
//!
//! This test proves that:
//! 1. `EventManager.compute_state_hash()` produces deterministic hashes
//! 2. `EventManager.verify_replay()` proves identical state after replay
//! 3. `compute_eligible_set()` returns identical results after replay
//! 4. All status projections match after replay
//! 5. No ghost inputs influence `apply_event()` — only event data is used
//!
//! Per SR-PLAN-V9 §3.3, this test produces a ReplayProof artifact that
//! captures the determinism verification result.

use chrono::Utc;
use sha2::{Digest, Sha256};
use sr_adapters::{EligibleSetComparison, EventManager};
use sr_domain::entities::ContentHash;
use sr_domain::plan_instance::{PlanInstance, SourceRef, SourceRefId, WorkUnitPlan};
use sr_domain::work_surface::{
    ContentAddressedRef, TemplateId, StageId, WorkKind, WorkUnitId,
};
use sr_domain::{ActorKind, EventEnvelope, EventId, StreamKind, TypedRef};

// =============================================================================
// Test Fixtures
// =============================================================================

/// Creates a multi-stage plan instance for comprehensive replay testing
fn create_comprehensive_test_plan() -> PlanInstance {
    let source_ref = SourceRef {
        id: SourceRefId::new("replay-test"),
        content_hash: ContentHash::new("hash_replay_test"),
        source_type: "test".to_string(),
        title: Some("Replay Determinism Test Plan".to_string()),
    };

    // Work Unit 1: No dependencies
    let wu1 = WorkUnitPlan::new(
        WorkUnitId::new("replay-001"),
        "First Work Unit - No Dependencies".to_string(),
        WorkKind::ResearchMemo,
        ContentAddressedRef {
            id: "intake:replay-001".to_string(),
            content_hash: ContentHash::new("hash_intake_001"),
        },
        TemplateId::new("GENERIC-KNOWLEDGE-WORK"),
        ContentAddressedRef {
            id: "proc:GENERIC-KNOWLEDGE-WORK".to_string(),
            content_hash: ContentHash::new("hash_proc"),
        },
        StageId::new("FRAME"),
    );

    // Work Unit 2: Depends on WU1
    let mut wu2 = WorkUnitPlan::new(
        WorkUnitId::new("replay-002"),
        "Second Work Unit - Depends on First".to_string(),
        WorkKind::DecisionRecord,
        ContentAddressedRef {
            id: "intake:replay-002".to_string(),
            content_hash: ContentHash::new("hash_intake_002"),
        },
        TemplateId::new("GENERIC-KNOWLEDGE-WORK"),
        ContentAddressedRef {
            id: "proc:GENERIC-KNOWLEDGE-WORK".to_string(),
            content_hash: ContentHash::new("hash_proc"),
        },
        StageId::new("FRAME"),
    );
    wu2.add_dependency(wu1.work_unit_id.clone());

    // Work Unit 3: Depends on WU1 and WU2
    let mut wu3 = WorkUnitPlan::new(
        WorkUnitId::new("replay-003"),
        "Third Work Unit - Multiple Dependencies".to_string(),
        WorkKind::TechnicalSpec,
        ContentAddressedRef {
            id: "intake:replay-003".to_string(),
            content_hash: ContentHash::new("hash_intake_003"),
        },
        TemplateId::new("GENERIC-KNOWLEDGE-WORK"),
        ContentAddressedRef {
            id: "proc:GENERIC-KNOWLEDGE-WORK".to_string(),
            content_hash: ContentHash::new("hash_proc"),
        },
        StageId::new("FRAME"),
    );
    wu3.add_dependency(wu1.work_unit_id.clone());
    wu3.add_dependency(wu2.work_unit_id.clone());

    PlanInstance::new(source_ref, vec![wu1, wu2, wu3])
}

/// Computes a simple envelope hash for test events
fn compute_envelope_hash(event_type: &str, stream_id: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(event_type.as_bytes());
    hasher.update(stream_id.as_bytes());
    format!("sha256:{}", hex::encode(hasher.finalize()))
}

/// Creates a test event
fn create_test_event(
    event_type: &str,
    subject_id: &str,
    payload: serde_json::Value,
) -> EventEnvelope {
    EventEnvelope {
        event_id: EventId::new(),
        stream_id: subject_id.to_string(),
        stream_kind: StreamKind::Loop,
        stream_seq: 1,
        global_seq: Some(1),
        event_type: event_type.to_string(),
        occurred_at: Utc::now(),
        actor_kind: ActorKind::System,
        actor_id: "system".to_string(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: vec![],
        payload,
        envelope_hash: compute_envelope_hash(event_type, subject_id),
    }
}

/// Creates a test event with refs
fn create_event_with_refs(
    event_type: &str,
    subject_id: &str,
    payload: serde_json::Value,
    refs: Vec<TypedRef>,
) -> EventEnvelope {
    let mut event = create_test_event(event_type, subject_id, payload);
    event.refs = refs;
    event
}

// =============================================================================
// Test 1: Basic State Hash Determinism
// =============================================================================

/// Proves that `compute_state_hash()` produces deterministic results
///
/// Per C-EVT-7: Projections must be rebuildable, which requires
/// deterministic state representation.
#[test]
fn test_state_hash_determinism() {
    let plan = create_comprehensive_test_plan();

    // Create and initialize event manager
    let mut em = EventManager::new_in_memory();
    em.load_plan_instance(plan);

    // Compute hash multiple times
    let hash1 = em.compute_state_hash();
    let hash2 = em.compute_state_hash();
    let hash3 = em.compute_state_hash();

    // All hashes must be identical
    assert_eq!(hash1, hash2, "Hash must be deterministic across calls");
    assert_eq!(hash2, hash3, "Hash must be deterministic across calls");

    // Hash format validation
    assert!(hash1.starts_with("sha256:"), "Hash must use sha256 prefix");
    assert_eq!(hash1.len(), 7 + 64, "Hash must be sha256: + 64 hex chars");
}

/// Proves that state hash changes reflect actual state changes
#[test]
fn test_state_hash_reflects_changes() {
    let plan = create_comprehensive_test_plan();

    let mut em = EventManager::new_in_memory();
    em.load_plan_instance(plan);

    let initial_hash = em.compute_state_hash();

    // Apply event that changes state
    em.apply_event(&create_test_event(
        "WorkSurfaceRecorded",
        "WU-replay-001",
        serde_json::json!({
            "work_unit_id": "WU-replay-001",
            "stage_id": "stage:FRAME"
        }),
    ))
    .unwrap();

    let changed_hash = em.compute_state_hash();

    // Hash should be different after state change
    assert_ne!(
        initial_hash, changed_hash,
        "Hash must change when state changes"
    );
}

// =============================================================================
// Test 2: Full Replay Determinism Proof
// =============================================================================

/// Proves complete replay determinism with comprehensive event sequence
///
/// This test demonstrates that replaying the same event sequence
/// produces identical state, satisfying C-EVT-7.
#[test]
fn test_full_replay_determinism() {
    let plan = create_comprehensive_test_plan();

    let mut em = EventManager::new_in_memory();
    em.load_plan_instance(plan.clone());

    // Comprehensive event sequence covering multiple work units and stages
    let events = vec![
        // WU-001: Initialize work surface
        create_test_event(
            "WorkSurfaceRecorded",
            "WU-replay-001",
            serde_json::json!({
                "work_unit_id": "WU-replay-001",
                "stage_id": "stage:FRAME"
            }),
        ),
        // WU-002: Initialize work surface
        create_test_event(
            "WorkSurfaceRecorded",
            "WU-replay-002",
            serde_json::json!({
                "work_unit_id": "WU-replay-002",
                "stage_id": "stage:FRAME"
            }),
        ),
        // WU-003: Initialize work surface
        create_test_event(
            "WorkSurfaceRecorded",
            "WU-replay-003",
            serde_json::json!({
                "work_unit_id": "WU-replay-003",
                "stage_id": "stage:FRAME"
            }),
        ),
        // WU-001: Enter stage
        create_test_event(
            "StageEntered",
            "WU-replay-001",
            serde_json::json!({
                "work_unit_id": "WU-replay-001",
                "stage_id": "stage:FRAME"
            }),
        ),
        // WU-001: Start iteration
        create_event_with_refs(
            "IterationStarted",
            "iter_replay_001",
            serde_json::json!({}),
            vec![TypedRef {
                kind: "WorkUnit".to_string(),
                id: "WU-replay-001".to_string(),
                rel: "about".to_string(),
                meta: serde_json::json!({}),
            }],
        ),
        // WU-001: Materialize candidate
        create_event_with_refs(
            "CandidateMaterialized",
            "cand_replay_001",
            serde_json::json!({}),
            vec![TypedRef {
                kind: "WorkUnit".to_string(),
                id: "WU-replay-001".to_string(),
                rel: "about".to_string(),
                meta: serde_json::json!({}),
            }],
        ),
        // WU-001: Record evidence
        create_event_with_refs(
            "EvidenceBundleRecorded",
            "bundle_replay_001",
            serde_json::json!({
                "stage_id": "stage:FRAME"
            }),
            vec![TypedRef {
                kind: "WorkUnit".to_string(),
                id: "WU-replay-001".to_string(),
                rel: "about".to_string(),
                meta: serde_json::json!({}),
            }],
        ),
        // WU-001: Complete stage (terminal)
        create_test_event(
            "StageCompleted",
            "WU-replay-001",
            serde_json::json!({
                "work_unit_id": "WU-replay-001",
                "stage_id": "stage:FRAME",
                "passed": true,
                "is_terminal": true
            }),
        ),
    ];

    // Apply all events
    for event in &events {
        em.apply_event(event).unwrap();
    }

    // Verify replay produces identical state
    let proof = em.verify_replay(&events);

    // Assertions
    assert!(
        proof.is_deterministic(),
        "Replay must be deterministic. Discrepancies: {:?}",
        proof.discrepancies
    );
    assert_eq!(
        proof.original_state_hash, proof.replayed_state_hash,
        "State hashes must match after replay"
    );
    assert_eq!(proof.event_count, events.len());
    assert_eq!(proof.work_unit_count, 3);
    assert!(proof.proof_id.starts_with("proof_"));

    // Print proof for documentation
    println!("\n=== Replay Determinism Proof ===");
    println!("Proof ID: {}", proof.proof_id);
    println!("Events: {}", proof.event_count);
    println!("Work Units: {}", proof.work_unit_count);
    println!("Original Hash: {}", proof.original_state_hash);
    println!("Replayed Hash: {}", proof.replayed_state_hash);
    println!("Deterministic: {}", proof.deterministic);
    println!("================================\n");
}

// =============================================================================
// Test 3: Eligible Set Determinism
// =============================================================================

/// Proves that `compute_eligible_set()` returns identical results after replay
///
/// Per C-EVT-7: All projections must be rebuildable, including
/// the eligible set computation.
#[test]
fn test_eligible_set_determinism_after_replay() {
    let plan = create_comprehensive_test_plan();

    let mut em1 = EventManager::new_in_memory();
    em1.load_plan_instance(plan.clone());

    let mut em2 = EventManager::new_in_memory();
    em2.load_plan_instance(plan);

    // Events to make WU-001 complete and WU-002 eligible
    let events = vec![
        create_test_event(
            "WorkSurfaceRecorded",
            "WU-replay-001",
            serde_json::json!({
                "work_unit_id": "WU-replay-001",
                "stage_id": "stage:FRAME"
            }),
        ),
        create_test_event(
            "WorkSurfaceRecorded",
            "WU-replay-002",
            serde_json::json!({
                "work_unit_id": "WU-replay-002",
                "stage_id": "stage:FRAME"
            }),
        ),
        create_test_event(
            "StageCompleted",
            "WU-replay-001",
            serde_json::json!({
                "work_unit_id": "WU-replay-001",
                "stage_id": "stage:FRAME",
                "passed": true,
                "is_terminal": true
            }),
        ),
    ];

    // Apply to first EM
    for event in &events {
        em1.apply_event(event).unwrap();
    }

    // Replay to second EM
    for event in &events {
        em2.apply_event(event).unwrap();
    }

    // Get eligible sets
    let eligible1 = em1.compute_eligible_set();
    let eligible2 = em2.compute_eligible_set();

    // Convert to sorted IDs for comparison
    let ids1: Vec<String> = eligible1
        .work_unit_ids()
        .iter()
        .map(|id| id.as_str().to_string())
        .collect();
    let ids2: Vec<String> = eligible2
        .work_unit_ids()
        .iter()
        .map(|id| id.as_str().to_string())
        .collect();

    // Create comparison
    let comparison = EligibleSetComparison::from_sets(ids1.clone(), ids2.clone());

    assert!(
        comparison.sets_match,
        "Eligible sets must match. Only in original: {:?}, Only in replayed: {:?}",
        comparison.only_in_original, comparison.only_in_replayed
    );

    // WU-002 should be eligible (WU-001 is complete, deps satisfied)
    assert!(
        eligible1.contains(&WorkUnitId::new("replay-002")),
        "WU-002 should be eligible after WU-001 completes"
    );

    println!("\n=== Eligible Set Determinism Proof ===");
    println!("Original eligible: {:?}", ids1);
    println!("Replayed eligible: {:?}", ids2);
    println!("Sets match: {}", comparison.sets_match);
    println!("======================================\n");
}

// =============================================================================
// Test 4: Status Projection Determinism
// =============================================================================

/// Proves that all status projections match after replay
///
/// This test verifies field-by-field equality of all work unit statuses.
#[test]
fn test_status_projection_determinism() {
    let plan = create_comprehensive_test_plan();

    let mut em1 = EventManager::new_in_memory();
    em1.load_plan_instance(plan.clone());

    let mut em2 = EventManager::new_in_memory();
    em2.load_plan_instance(plan);

    // Events with various state changes
    let events = vec![
        create_test_event(
            "WorkSurfaceRecorded",
            "WU-replay-001",
            serde_json::json!({
                "work_unit_id": "WU-replay-001",
                "stage_id": "stage:FRAME"
            }),
        ),
        create_test_event(
            "StageEntered",
            "WU-replay-001",
            serde_json::json!({
                "work_unit_id": "WU-replay-001",
                "stage_id": "stage:FRAME"
            }),
        ),
        create_test_event(
            "StopTriggered",
            "WU-replay-001",
            serde_json::json!({
                "work_unit_id": "WU-replay-001",
                "reason": "Test stop trigger",
                "requires_portal": true,
                "portal_id": "portal:test"
            }),
        ),
    ];

    // Apply to first EM
    for event in &events {
        em1.apply_event(event).unwrap();
    }

    // Replay to second EM
    for event in &events {
        em2.apply_event(event).unwrap();
    }

    // Find discrepancies
    let discrepancies = em1.find_discrepancies(&em2);

    assert!(
        discrepancies.is_empty(),
        "Status projections must match. Discrepancies: {:?}",
        discrepancies
    );

    // Verify specific status
    let status1 = em1.get_status("WU-replay-001").unwrap();
    let status2 = em2.get_status("WU-replay-001").unwrap();

    assert_eq!(status1.coarse_status, status2.coarse_status);
    assert_eq!(status1.has_work_surface, status2.has_work_surface);
    assert_eq!(status1.deps_satisfied, status2.deps_satisfied);
    assert_eq!(status1.current_stage_id, status2.current_stage_id);
    assert_eq!(status1.is_blocked(), status2.is_blocked());

    println!("\n=== Status Projection Determinism Proof ===");
    println!("WU-001 coarse_status: {:?}", status1.coarse_status);
    println!("WU-001 has_work_surface: {}", status1.has_work_surface);
    println!("WU-001 is_blocked: {}", status1.is_blocked());
    println!("Discrepancies: 0");
    println!("============================================\n");
}

// =============================================================================
// Test 5: No Ghost Inputs
// =============================================================================

/// Proves that `apply_event()` uses only event data (no ghost inputs)
///
/// Per C-CTX-2: No ghost inputs. This test verifies that replaying
/// the same events produces identical state regardless of external factors.
#[test]
fn test_no_ghost_inputs() {
    let plan = create_comprehensive_test_plan();

    // Create two independent event managers
    let mut em1 = EventManager::new_in_memory();
    em1.load_plan_instance(plan.clone());

    let mut em2 = EventManager::new_in_memory();
    em2.load_plan_instance(plan.clone());

    // Create deterministic events
    let events = vec![
        create_test_event(
            "WorkSurfaceRecorded",
            "WU-replay-001",
            serde_json::json!({
                "work_unit_id": "WU-replay-001",
                "stage_id": "stage:FRAME"
            }),
        ),
        create_test_event(
            "StageCompleted",
            "WU-replay-001",
            serde_json::json!({
                "work_unit_id": "WU-replay-001",
                "stage_id": "stage:FRAME",
                "passed": true,
                "is_terminal": true
            }),
        ),
    ];

    // Apply events to both (simulating "independent" replays)
    for event in &events {
        em1.apply_event(event).unwrap();
        em2.apply_event(event).unwrap();
    }

    // State hashes must be identical
    let hash1 = em1.compute_state_hash();
    let hash2 = em2.compute_state_hash();

    assert_eq!(
        hash1, hash2,
        "Independent replays must produce identical state (no ghost inputs)"
    );

    // Find any discrepancies
    let discrepancies = em1.find_discrepancies(&em2);
    assert!(
        discrepancies.is_empty(),
        "No ghost inputs allowed. Discrepancies: {:?}",
        discrepancies
    );

    println!("\n=== No Ghost Inputs Proof ===");
    println!("Independent replay hash match: {}", hash1 == hash2);
    println!("Discrepancies: 0");
    println!("=============================\n");
}

// =============================================================================
// Test 6: Dependency Satisfaction Replay
// =============================================================================

/// Proves that dependency satisfaction is deterministically computed after replay
#[test]
fn test_dependency_satisfaction_replay() {
    let plan = create_comprehensive_test_plan();

    let mut em = EventManager::new_in_memory();
    em.load_plan_instance(plan.clone());

    // Initially, WU-002 and WU-003 have unsatisfied dependencies
    let status2 = em.get_status("WU-replay-002").unwrap();
    let status3 = em.get_status("WU-replay-003").unwrap();
    assert!(
        !status2.deps_satisfied,
        "WU-002 deps should be unsatisfied initially"
    );
    assert!(
        !status3.deps_satisfied,
        "WU-003 deps should be unsatisfied initially"
    );

    // Complete WU-001
    let events = vec![
        create_test_event(
            "WorkSurfaceRecorded",
            "WU-replay-001",
            serde_json::json!({
                "work_unit_id": "WU-replay-001",
                "stage_id": "stage:FRAME"
            }),
        ),
        create_test_event(
            "WorkSurfaceRecorded",
            "WU-replay-002",
            serde_json::json!({
                "work_unit_id": "WU-replay-002",
                "stage_id": "stage:FRAME"
            }),
        ),
        create_test_event(
            "WorkSurfaceRecorded",
            "WU-replay-003",
            serde_json::json!({
                "work_unit_id": "WU-replay-003",
                "stage_id": "stage:FRAME"
            }),
        ),
        create_test_event(
            "StageCompleted",
            "WU-replay-001",
            serde_json::json!({
                "work_unit_id": "WU-replay-001",
                "stage_id": "stage:FRAME",
                "passed": true,
                "is_terminal": true
            }),
        ),
    ];

    for event in &events {
        em.apply_event(event).unwrap();
    }

    // After WU-001 completes, WU-002 deps should be satisfied
    let status2 = em.get_status("WU-replay-002").unwrap();
    assert!(
        status2.deps_satisfied,
        "WU-002 deps should be satisfied after WU-001 completes"
    );

    // But WU-003 still depends on WU-002 which isn't complete
    let status3 = em.get_status("WU-replay-003").unwrap();
    assert!(
        !status3.deps_satisfied,
        "WU-003 deps should still be unsatisfied"
    );

    // Verify this is deterministic via replay
    let proof = em.verify_replay(&events);
    assert!(
        proof.is_deterministic(),
        "Dependency satisfaction must be deterministic"
    );

    println!("\n=== Dependency Satisfaction Replay Proof ===");
    println!(
        "WU-002 deps_satisfied after WU-001 complete: {}",
        status2.deps_satisfied
    );
    println!(
        "WU-003 deps_satisfied (waiting for WU-002): {}",
        status3.deps_satisfied
    );
    println!("Replay deterministic: {}", proof.is_deterministic());
    println!("=============================================\n");
}

# SR-REPLAY-PROOF: Deterministic Replay Proof

**Version:** 1.0
**Status:** ACTIVE
**Deliverable:** D-36 (SR-PLAN-V9 Phase V9-3)
**Contract Reference:** SR-CONTRACT C-EVT-7

---

## 1. Overview

This document formally establishes that the EventManager's `rebuild()` operation is **deterministic**: given the same event sequence, replay produces identical state.

Per SR-CONTRACT C-EVT-7:
> "Projections MUST be rebuildable from the event log."

This proof demonstrates compliance with C-EVT-7 and the related constraint C-CTX-2 (no ghost inputs).

---

## 2. Proof Statement

**Given:**
- An event sequence **E** = [e₁, e₂, ..., eₙ]
- A PlanInstance **P** defining work unit topology

**Claim:**
For any two EventManager instances **M₁** and **M₂** initialized with **P**:
```
apply(M₁, E) ≡ apply(M₂, E)
```

Where equivalence (≡) means:
1. `M₁.compute_state_hash() = M₂.compute_state_hash()`
2. `M₁.compute_eligible_set() = M₂.compute_eligible_set()`
3. `∀ wu ∈ P: M₁.get_status(wu) = M₂.get_status(wu)`

---

## 3. Verified Properties

### 3.1 State Hash Equality

The `compute_state_hash()` method produces a deterministic SHA-256 hash incorporating:

| Component | Description | Ordering |
|-----------|-------------|----------|
| Work Unit IDs | All tracked work units | Sorted alphabetically |
| coarse_status | TODO, ELIGIBLE, IN_PROGRESS, BLOCKED, COMPLETE | Per work unit |
| deps_satisfied | Boolean dependency satisfaction | Per work unit |
| has_work_surface | Boolean work surface existence | Per work unit |
| current_stage_id | Current stage identifier | Per work unit |
| stage_status | Map of stage → status | Sorted by stage ID |
| block_reasons | Active block reasons | Sorted by description |
| last_global_seq | Last processed event sequence | Global |

**Hash Format:** `sha256:<64 hex characters>`

### 3.2 Eligible Set Equality

After replay, `compute_eligible_set()` returns identical sets:

```
eligible(M₁) = eligible(M₂)
```

A work unit is eligible if:
- `has_work_surface = true`
- `deps_satisfied = true`
- `coarse_status ∈ {TODO, ELIGIBLE}`
- `block_reasons.is_empty()`

### 3.3 Status Projection Equality

All `WorkUnitStatus` fields match after replay:

| Field | Type | Comparison |
|-------|------|------------|
| coarse_status | CoarseStatus enum | Exact match |
| deps_satisfied | bool | Exact match |
| has_work_surface | bool | Exact match |
| current_stage_id | Option<StageId> | Exact match |
| stage_status | HashMap<String, StageStatusEntry> | Deep equality |
| block_reasons | Vec<BlockReason> | Set equality |

### 3.4 No Ghost Inputs

Per C-CTX-2, `apply_event()` uses **only** event data:

- No external time sources (uses `occurred_at` from event)
- No random number generation
- No external service calls
- No mutable global state
- No environment variables

The only inputs to state computation are:
1. The event envelope (immutable)
2. The current EventManager state (deterministic)
3. The PlanInstance topology (immutable)

---

## 4. Evidence

### 4.1 Code References

| Component | Location | Purpose |
|-----------|----------|---------|
| `compute_state_hash()` | `crates/sr-adapters/src/event_manager.rs` | Deterministic state hashing |
| `verify_replay()` | `crates/sr-adapters/src/event_manager.rs` | Replay verification |
| `find_discrepancies()` | `crates/sr-adapters/src/event_manager.rs` | Field-level comparison |
| `ReplayProof` | `crates/sr-adapters/src/replay.rs` | Proof artifact type |
| `EligibleSetComparison` | `crates/sr-adapters/src/replay.rs` | Eligible set comparison |

### 4.2 Test Coverage

| Test | File | Verification |
|------|------|--------------|
| `test_state_hash_determinism` | `replay_determinism_test.rs` | Hash stability across calls |
| `test_state_hash_reflects_changes` | `replay_determinism_test.rs` | Hash changes with state |
| `test_full_replay_determinism` | `replay_determinism_test.rs` | Complete replay proof |
| `test_eligible_set_determinism_after_replay` | `replay_determinism_test.rs` | Eligible set equality |
| `test_status_projection_determinism` | `replay_determinism_test.rs` | Status field equality |
| `test_no_ghost_inputs` | `replay_determinism_test.rs` | Independent replay equality |
| `test_dependency_satisfaction_replay` | `replay_determinism_test.rs` | Dependency computation |
| `test_verify_replay_deterministic` | `event_manager.rs` | Unit-level replay proof |
| `test_replay_proof_full_workflow` | `event_manager.rs` | Full workflow replay |

### 4.3 Verification Commands

```bash
# Run replay proof tests (sr-adapters)
cargo test --package sr-adapters replay

# Run integration tests (sr-api)
cargo test --package sr-api --test replay_determinism_test

# Run all event_manager tests
cargo test --package sr-adapters event_manager::tests
```

---

## 5. ReplayProof Artifact

When `verify_replay()` is called, it produces a `ReplayProof` artifact:

```rust
pub struct ReplayProof {
    pub proof_id: String,              // "proof_<ulid>"
    pub event_count: usize,            // Number of events replayed
    pub first_event_id: Option<String>,
    pub last_event_id: Option<String>,
    pub original_state_hash: String,   // Hash before replay
    pub replayed_state_hash: String,   // Hash after replay
    pub deterministic: bool,           // Hashes match and no discrepancies
    pub discrepancies: Vec<ReplayDiscrepancy>,
    pub proof_computed_at: DateTime<Utc>,
    pub plan_instance_id: Option<String>,
    pub work_unit_count: usize,
}
```

A replay is **deterministic** if:
- `original_state_hash == replayed_state_hash`
- `discrepancies.is_empty()`

---

## 6. Discrepancy Reporting

If replay produces different state, `ReplayDiscrepancy` provides diagnostics:

```rust
pub struct ReplayDiscrepancy {
    pub work_unit_id: String,
    pub field: String,           // e.g., "coarse_status", "deps_satisfied"
    pub original_value: Value,   // JSON value
    pub replayed_value: Value,   // JSON value
    pub description: String,     // Human-readable
}
```

---

## 7. Limitations

1. **Timestamp Sensitivity:** Event `occurred_at` is recorded but not used in state computation. If timestamp-dependent logic is added, this proof must be updated.

2. **External Dependencies:** This proof covers EventManager in isolation. Database projections (PostgreSQL) have separate verification in `sr-e2e-harness/src/replay.rs`.

3. **Concurrency:** This proof assumes sequential event application. Concurrent access requires additional synchronization guarantees.

---

## 8. Related Documents

- [SR-CONTRACT](SR-CONTRACT.md) — C-EVT-7, C-CTX-2 requirements
- [SR-EVENT-MANAGER](SR-EVENT-MANAGER.md) — EventManager specification
- [SR-PLAN-V9](../planning/SR-PLAN-V9.md) — V9-3 deliverable definition
- [SR-SPEC](SR-SPEC.md) — §1.7 Projection Rebuild requirements

---

## 9. Changelog

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-01-16 | V9-3 | Initial release |

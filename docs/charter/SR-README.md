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

**V10-1 through V10-4:** ✅ VERIFIED (2026-01-17)
**V10-5, V10-6:** Pending (optional)
**Branch:** `solver-ralph-10`

Loop Governor stop triggers (D-22) are now fully operational:
- BUDGET_EXHAUSTED and REPEATED_FAILURE triggers fire correctly
- Decision-required resume gating works
- Tests 9, 12-15 verified passing

See `docs/planning/SR-PLAN-LOOPS.md` for detailed verification results.
See `docs/build-governance/SR-CHANGE.md` v1.1 for implementation summary.

---

## Previous Session Summary (2026-01-17)

**Completed:**
1. Applied migration 009 (V10 schema columns + candidate traceability index)
2. Verified all V10-1 through V10-4 implementations via API testing:
   - Test 9: Loop ref in IterationStarted.refs[] with `rel="in_scope_of"` ✅
   - Test 12: Index `idx_candidates_produced_by_iteration` exists ✅
   - Test 13: BUDGET_EXHAUSTED fires at max_iterations, returns 412 ✅
   - Test 14: REPEATED_FAILURE fires after 3 consecutive failures ✅
   - Test 15: Resume requires Decision when `requires_decision=true` ✅
3. Updated SR-PLAN-LOOPS, SR-PLAN-GAP-ANALYSIS, SR-CHANGE with verification results
4. Committed: `feat(V10): Implement and verify Loop Governor stop triggers`

**Remaining V10 work (low priority):**
- V10-5: Loop PATCH endpoint for budget monotonicity (Test 8 gap)
- V10-6: OracleSuite hash prefix fix — `sha256:sha256:` doubled prefix (Test 10 gap)

---

## Next Instance Prompt

> **Assignment:** Complete V10-5 and V10-6 to finish V10 scope, then proceed to V11.

### Orientation

1. `docs/planning/SR-PLAN-GAP-ANALYSIS.md §4` — V10-5/V10-6 gap descriptions
2. `docs/planning/SR-PLAN-LOOPS.md` — Test 8 (edit endpoint), Test 10 (hash prefix)

---

### V10-5: Loop PATCH Endpoint

**Gap:** No endpoint to update Loop budgets after creation (Test 8).

**Implementation:**

```rust
// Add to crates/sr-api/src/handlers/loops.rs

#[derive(Debug, Deserialize)]
pub struct UpdateLoopRequest {
    pub budgets: Option<LoopBudgets>,  // Partial update
}

// PATCH /api/v1/loops/{loop_id}
pub async fn update_loop(...) -> ApiResult<Json<LoopActionResponse>> {
    // 1. Load current Loop from projection
    // 2. Validate budget monotonicity:
    //    - new.max_iterations >= current.max_iterations
    //    - new.max_oracle_runs >= current.max_oracle_runs
    //    - new.max_wallclock_hours >= current.max_wallclock_hours
    // 3. Emit LoopUpdated event with new budgets
    // 4. Return updated Loop state
}
```

**Add route in `crates/sr-api/src/routes.rs`:**
```rust
.route("/loops/:loop_id", patch(handlers::loops::update_loop))
```

**Add event handling in `crates/sr-adapters/src/projections.rs`:**
```rust
"LoopUpdated" => self.apply_loop_updated(&mut tx, event).await,
```

**Verification:** After implementation, update SR-PLAN-LOOPS Test 8 to PASS.

---

### V10-6: OracleSuite Hash Prefix Fix

**Gap:** OracleSuite `content_hash` shows `sha256:sha256:...` (doubled prefix).

**Root cause location:** `crates/sr-adapters/src/oracle_suite.rs`

**Investigation:**
1. Search for where `content_hash` is computed/assigned
2. Look for double application of `sha256:` prefix
3. Fix to ensure single `sha256:` prefix

**Verification:** After fix, query IterationStarted event refs and confirm OracleSuite meta shows single prefix.

---

### Constraints

- V10-1 through V10-4 are verified complete — do not re-implement or re-test
- Migration 009 is applied — do not duplicate schema changes
- Commit each phase separately: V10-5 then V10-6
- Update SR-PLAN-LOOPS and SR-PLAN-GAP-ANALYSIS after each phase
- Add SR-CHANGE entry for V10-5/V10-6 completion

### After V10 Complete

Proceed to V11 scoping (see SR-PLAN-GAP-ANALYSIS §4 SR-PLAN-V11 Proposed).

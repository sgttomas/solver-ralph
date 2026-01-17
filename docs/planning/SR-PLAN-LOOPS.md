# Branch 0 Loop Functionality Validation Plan

## Objective

Explicitly test Loop functionality through the UI and API to validate acceptance criteria for Loop creation, lifecycle management, iteration cycling, Work Surface binding, and contract compliance.

## Scope Context (per SR-PLAN-GAP-ANALYSIS)

This validation plan addresses:
- **D-29** (Loop/iteration/candidate views) — ✅ Complete, validating UI works
- **D-34** (E2E harness - happy path) — ⚠️ Partial, this IS the manual verification
- **D-35** (E2E harness - failure modes) — ❌ Not Started, **Part C begins this work**
- **D-22** (Loop governor service) — ⚠️ Partial, testing minimal governor via `/start`

**Note:** Full Loop governor (D-22) and automated E2E harness (D-34, D-35) are scoped for V10/V11.

---

## Prerequisites

- Infrastructure running (`make deploy`)
- API running with auth bypass (`SR_OIDC_SKIP_VALIDATION=true cargo run --bin sr-api`)
- UI running (`make dev-ui` on port 3001)
- Database access for event verification (`psql` or pgAdmin)

---

## Plan Structure

The plan is organized into four phases, each building on the previous:

| Phase | Focus | Tests | Contracts Validated |
|-------|-------|-------|---------------------|
| **Part A** | Happy Path - Loop Basics | 1-8 | C-LOOP-1, C-LOOP-2, State Machine |
| **Part B** | Context & Work Surface Binding | 9-12 | C-CTX-1, C-CTX-2, C-LOOP-4 |
| **Part C** | Failure Modes (Waivable) | 13-16 | C-LOOP-3, C-DEC-1 |
| **Part D** | Integrity Conditions (Non-Waivable) | 17-19 | C-OR-7, C-LOOP-3 |

---

# Part A: Happy Path — Loop Basics

These tests validate core Loop functionality works before testing advanced scenarios.

---

## Test 1: View Existing Loop

**Purpose:** Verify Loops created via Work Surface "Start Work" are visible and correctly linked.

**Steps:**
1. Navigate to **Loops** in the sidebar
2. Find a Loop created earlier (e.g., associated with a known Work Surface)
3. Verify Loop details display:
   - Loop ID (format: `loop_<ULID>`)
   - State: `ACTIVE` or `CLOSED`
   - Work Unit ID matches the Work Surface's `work_unit_id`
   - Iteration count shows at least 1

**Expected:** Loop is visible with correct bindings.

**Validates:** C-LOOP-1 (Loop bound to work unit)

---

## Test 2: Create New Loop Manually

**Purpose:** Test Loop creation independent of Work Surface flow.

**Steps:**
1. Navigate to **Loops** → Click **"Create Loop"**
2. Fill in the form:
   - **Goal:** "Manual Loop Test - Part A"
   - **Work Unit:** Leave empty (system generates) or enter custom ID
   - **Procedure Template:** Select "Research Memo Procedure"
   - **Initial Stage:** FRAME
   - **Oracle Suite:** Select available suite
   - **Budgets:**
     - Max Iterations: 5
     - Max Oracle Runs: 20
     - Max Wallclock Hours: 8
   - **Activate immediately:** Leave unchecked
3. Click **Create**
4. Verify:
   - Loop appears in list with state `CREATED`
   - Loop ID follows format `loop_<ULID>`

**Expected:** Loop created in `CREATED` state with specified budgets.

**Validates:** C-LOOP-1 (bounded iteration with hard stop budgets)

---

## Test 3: Loop State Transitions (Lifecycle)

**Purpose:** Test all valid state transitions per SR-SPEC §3.1.2.

### Test 3a: Activate Loop (CREATED → ACTIVE)

**Steps:**
1. Select the Loop from Test 2
2. Click **"Activate"** button
3. Verify:
   - State changes to `ACTIVE`
   - Activate button disappears or disables
4. Check API terminal for `LoopActivated` event

**Expected:** `LoopActivated` event emitted, state = `ACTIVE`.

### Test 3b: Pause Loop (ACTIVE → PAUSED)

**Steps:**
1. Click **"Pause"** button on the active Loop
2. Verify state changes to `PAUSED`
3. Check API terminal for pause-related event

**Expected:** Loop paused, UI reflects state change.

### Test 3c: Resume Loop (PAUSED → ACTIVE) — Manual Pause Only

**Steps:**
1. Click **"Resume"** button
2. Verify state changes back to `ACTIVE`

**Note:** This tests manual pause/resume. Stop-trigger-induced pause requires Decision (see Test 15).

**Expected:** Loop resumes without requiring Decision (manual pause).

### Test 3d: Close Loop (Any → CLOSED)

**Steps:**
1. Click **"Close"** button
2. Confirm warning dialog
3. Verify:
   - State changes to `CLOSED` (terminal)
   - No state transition buttons available
   - Attempting any action shows "Loop is closed" error

**Expected:** `LoopClosed` event emitted, no further transitions possible.

**Validates:** Loop state machine per SR-SPEC §3.1.2

---

## Test 4: Loop Detail View — UI Completeness

**Purpose:** Verify all required Loop information is displayed.

**Steps:**
1. Click on any Loop to open detail view
2. Verify these sections are present and populated:

| Section | Required Fields |
|---------|-----------------|
| **Overview** | Loop ID, goal, work unit, state, created_at |
| **Work Surface Binding** | Intake name, procedure template, current stage |
| **Budgets & Usage** | max_iterations, used_iterations, progress bar |
| | max_oracle_runs, used_oracle_runs |
| | max_wallclock_hours, elapsed_hours |
| **Iterations Table** | List with: sequence, iteration_id, state, started_at, duration |
| **Stop Triggers** | Table (may be empty if none fired) |
| **Active Exceptions** | Table (may be empty if none active) |

**Expected:** All sections render without errors; data matches API responses.

**Validates:** UI completeness for Loop governance

---

## Test 5: Start First Iteration

**Purpose:** Test starting an iteration within a Loop.

**Steps:**
1. Create and activate a new Loop (or use existing active Loop)
2. Click **"Start Iteration"** (in Loop detail or via Work Surface)
3. Verify:
   - `IterationStarted` event emitted (check API terminal)
   - Iteration appears in Iterations table with:
     - `sequence: 1`
     - `state: STARTED` or `RUNNING`
     - `iteration_id` format: `iter_<ULID>`
   - Loop's `iteration_count` increments

**Expected:** First iteration created and visible.

**Validates:** C-LOOP-2 (iteration cycling)

---

## Test 6: Complete Iteration and Start Second

**Purpose:** Test iteration completion and cycling to next iteration.

**Steps:**
1. Using the iteration from Test 5, complete it:
   - Via UI "Complete" action, OR
   - Via API: `POST /api/v1/iterations/{id}/complete`
2. Verify:
   - Iteration state changes to `COMPLETED`
   - `IterationCompleted` event emitted
3. Start a second iteration
4. Verify:
   - New iteration has `sequence: 2`
   - New unique `iteration_id`
   - Budget usage updates (2 of N iterations used)

**Expected:** Multiple iterations cycle correctly with proper sequencing.

**Validates:** C-LOOP-2 (fresh-context iterations)

---

## Test 7: Budget Display and Tracking

**Purpose:** Verify budget tracking updates correctly.

**Steps:**
1. Create a Loop with specific budgets:
   - `max_iterations: 3`
   - `max_oracle_runs: 5`
2. Start and complete 2 iterations
3. Verify budget display shows:
   - Iterations: "2 / 3" or equivalent progress
   - Progress bar at ~66%
4. If oracle runs occurred, verify oracle run count incremented

**Expected:** Budget usage accurately reflected in UI.

**Validates:** C-LOOP-1 (budget tracking)

---

## Test 8: Loop Edit Restrictions

**Purpose:** Test editing constraints based on Loop state.

**Steps:**

### 8a: Edit in CREATED state
1. Create a new Loop (don't activate)
2. Click **"Edit"**
3. Modify goal text and increase `max_iterations`
4. Save changes
5. Verify changes persisted

### 8b: Edit restrictions in ACTIVE state
1. Activate the Loop
2. Attempt to edit
3. Verify:
   - Goal MAY be editable (informational)
   - Budgets can only be **increased**, not decreased
   - Attempting to decrease budget shows error

**Expected:** Edit restrictions enforced based on state.

**Validates:** Budget constraints (monotonic increase only)

---

# Part B: Context & Work Surface Binding

These tests validate iteration context provenance and Work Surface integration per C-CTX-1, C-CTX-2, and C-LOOP-4.

---

## Test 9: Iteration Context Refs — Minimum Required Categories

**Purpose:** Verify `IterationStarted.refs[]` contains required reference categories per SR-SPEC §3.2.1.1.

**Steps:**
1. Start a new iteration on a Loop bound to a Work Surface
2. Query the `IterationStarted` event:
   ```sql
   SELECT payload, refs
   FROM es.events
   WHERE event_type = 'IterationStarted'
   ORDER BY global_seq DESC LIMIT 1;
   ```
3. Verify `refs[]` contains these minimum categories:

| # | Kind | rel | Required? | Notes |
|---|------|-----|-----------|-------|
| 1 | `Loop` | `in_scope_of` | ✅ Yes | Loop this iteration belongs to |
| 2 | `GovernedArtifact` | `depends_on` | ✅ Yes | SR-TYPES, SR-CONTRACT, SR-SPEC, SR-DIRECTIVE |
| 3 | `Iteration` | `depends_on` | Conditional | Prior summaries (empty for first iteration) |
| 4 | `Candidate` | `depends_on` | Conditional | Base candidate if incremental work |
| 5 | `OracleSuite` | `depends_on` | ✅ Yes | Must have `suite_hash` in meta |
| 6 | `Deviation/Deferral/Waiver` | `depends_on` | Conditional | Active exceptions (empty if none) |
| 7 | `Intake` | `depends_on` | ✅ Yes | Work Surface component |
| 8 | `ProcedureTemplate` | `depends_on` | ✅ Yes | Must have `current_stage_id` in meta |

**Expected:** All required refs present with correct `rel` and `meta` fields.

**Validates:** C-CTX-1 (iteration context provenance), "No ghost inputs"

---

## Test 10: Iteration Context Refs — Meta Requirements

**Purpose:** Verify `meta` fields on refs meet SR-SPEC §1.5.3.1 requirements.

**Steps:**
1. Using the `refs[]` from Test 9, verify meta requirements:

| Ref Kind | Required Meta Fields |
|----------|---------------------|
| `GovernedArtifact` | `content_hash` (sha256), `version` (SemVer) |
| `OracleSuite` | `content_hash`, `suite_hash` |
| `Intake` | `content_hash` |
| `ProcedureTemplate` | `content_hash`, `current_stage_id` |
| `Candidate` | `content_hash` |
| `EvidenceBundle` | `content_hash` |

2. Verify `content_hash` format: `sha256:<64-hex-chars>`

**Expected:** All dereferenceable refs have valid `meta.content_hash`.

**Validates:** C-CTX-1 (content-addressed provenance)

---

## Test 11: Work Surface Binding Verification

**Purpose:** Verify Work Surface correctly binds Intake + Procedure Template + Oracle Suite.

**Steps:**
1. Navigate to **Work Surfaces**
2. Select a Work Surface bound to a Loop
3. Verify Work Surface displays:
   - **Intake:** Name, objective summary, content hash
   - **Procedure Template:** Name, current stage, total stages
   - **Oracle Suite:** Suite name, suite hash
4. Navigate to the bound Loop
5. Verify Loop's Work Surface section shows same bindings
6. Start an iteration and verify `IterationStarted.refs[]` includes all three

**Expected:** Consistent binding across Work Surface, Loop, and Iteration refs.

**Validates:** C-CTX-2 (Work Surface binding), SR-SPEC §1.2.4

---

## Test 12: Candidate Traceability (C-LOOP-4)

**Purpose:** Verify candidates are traceable to Loop, Iteration, and Runs.

**Steps:**
1. In an active Loop, start an iteration
2. Register a candidate (via API or simulated agent work):
   ```bash
   curl -X POST "http://localhost:3000/api/v1/candidates" \
     -H "Authorization: Bearer test-token" \
     -H "Content-Type: application/json" \
     -d '{
       "iteration_id": "<iteration_id>",
       "manifest_hash": "sha256:abc123...",
       "description": "Test candidate for traceability"
     }'
   ```
3. Verify candidate:
   - Has stable identity including `sha256:<manifest_hash>`
   - Is linked to the iteration (`iteration_id` in response or refs)
4. Navigate to **Candidates** list
5. Click on the candidate and verify:
   - Link to parent Iteration
   - Link to parent Loop
   - Runs table (empty until verification runs)
6. Query graph to verify relationships:
   ```sql
   SELECT * FROM graph.edges
   WHERE source_id = '<candidate_id>' OR target_id = '<candidate_id>';
   ```

**Expected:** Candidate traceable via: Loop → Iteration → Candidate.

**Validates:** C-LOOP-4 (candidate traceability)

---

# Part C: Failure Modes — Waivable Stop Triggers

These tests validate waivable stop triggers and the Decision-required resume flow.

---

## Test 13: Stop Trigger — BUDGET_EXHAUSTED

**Purpose:** Verify budget exhaustion fires stop trigger and pauses Loop.

**Steps:**
1. Create a Loop with minimal budget:
   - `max_iterations: 2`
2. Activate the Loop
3. Start and complete iteration 1
4. Start and complete iteration 2 (budget now exhausted)
5. Attempt to start iteration 3
6. Verify:
   - API rejects with error OR Loop auto-transitions to `PAUSED`
   - `StopTriggered` event emitted with `trigger: BUDGET_EXHAUSTED`
   - UI shows Loop in `PAUSED` state
   - Stop Triggers section shows `BUDGET_EXHAUSTED`

**Verification Query:**
```sql
SELECT event_type, payload->>'trigger' as trigger
FROM es.events
WHERE stream_id LIKE 'loop:%' AND event_type = 'StopTriggered'
ORDER BY global_seq DESC LIMIT 1;
```

**Expected:** `BUDGET_EXHAUSTED` trigger fires, Loop pauses.

**Validates:** C-LOOP-1, C-LOOP-3 (waivable trigger)

---

## Test 14: Stop Trigger — REPEATED_FAILURE

**Purpose:** Test REPEATED_FAILURE stop trigger (N ≥ 3 consecutive failures).

**Steps:**
1. Create and activate a new Loop with `max_iterations: 5`
2. Start iteration 1
3. Fail the iteration (via API):
   ```bash
   curl -X POST "http://localhost:3000/api/v1/iterations/{id}/complete" \
     -H "Authorization: Bearer test-token" \
     -H "Content-Type: application/json" \
     -d '{"outcome": "FAILED", "reason": "Test failure 1"}'
   ```
4. Repeat: Start iteration 2, fail it
5. Repeat: Start iteration 3, fail it
6. Verify after 3rd failure:
   - Loop transitions to `PAUSED`
   - `StopTriggered` event with `trigger: REPEATED_FAILURE`
   - UI displays stop trigger

**Gap Identification:** If `/complete` with `outcome: FAILED` doesn't work, or if there's no failure endpoint, document this as a gap for V10.

**Expected:** `REPEATED_FAILURE` trigger fires after 3 consecutive failures.

**Validates:** C-LOOP-3 (waivable trigger)

---

## Test 15: Resume Requires Decision After Stop Trigger

**Purpose:** Verify PAUSED → ACTIVE requires human Decision when caused by stop trigger.

**Precondition:** Loop paused by BUDGET_EXHAUSTED (from Test 13).

**Steps:**

### 15a: Attempt Resume Without Decision
1. On the paused Loop, click **"Resume"**
2. Verify:
   - Resume is **blocked**
   - Error indicates Decision required (e.g., 412 Precondition Failed)
   - UI shows message: "Decision required to resume after stop trigger"

### 15b: Record Decision
1. Navigate to **Approvals** → **Decisions**
2. Click **"Record Decision"**
3. Fill in:
   - **Trigger:** BUDGET_EXHAUSTED
   - **Scope:** Loop ID from Test 13
   - **Decision:** "Approved budget extension for continued testing"
   - **Rationale:** "Validation testing requires additional iterations"
4. Submit (must be HUMAN actor)
5. Verify `DecisionRecorded` event emitted with `actor_kind: HUMAN`

### 15c: Resume After Decision
1. Return to the paused Loop
2. Click **"Resume"**
3. Verify:
   - Resume succeeds
   - Loop transitions to `ACTIVE`
   - `LoopResumed` event emitted
   - Decision is linked/visible in Loop detail

**Expected:** Resume blocked without Decision; succeeds after Decision recorded.

**Validates:** SR-SPEC §3.1.2, C-DEC-1 (Decision required for arbitration)

---

## Test 16: Exception Lifecycle — DEVIATION, DEFERRAL, WAIVER

**Purpose:** Test exception workflow integration with Loops.

### Test 16a: Create and Activate DEVIATION

**Steps:**
1. Navigate to **Approvals** → **Exceptions**
2. Click **"Create Exception"**
3. Fill in:
   - **Kind:** DEVIATION
   - **Scope:** Reference the Loop ID (explicit, bounded)
   - **Reason:** "Testing deviation workflow"
   - **Governed Artifact:** Reference a specific requirement being deviated from
   - **Expires At:** Set future date
4. Submit (must be HUMAN actor)
5. Click **"Activate"** on the created exception
6. Navigate to Loop detail
7. Verify: Exception appears in "Active Exceptions" table

### Test 16b: Resolve Exception

**Steps:**
1. Return to **Exceptions** tab
2. Click **"Resolve"** on the active exception
3. Provide resolution rationale
4. Navigate to Loop detail
5. Verify: Exception shows as resolved (or removed from active list)

### Test 16c: Verify Exception in Iteration Refs

**Steps:**
1. With an active exception on a Loop, start a new iteration
2. Query `IterationStarted.refs[]`
3. Verify: Active exception appears with:
   - `kind: Deviation`
   - `rel: depends_on`
   - Correct `id`

**Expected:** Exception lifecycle works; active exceptions appear in iteration context.

**Validates:** C-EXC-1 through C-EXC-5 (exception semantics)

---

# Part D: Integrity Conditions — Non-Waivable

These tests validate non-waivable integrity conditions per C-OR-7. These conditions **cannot** be bypassed even with a Decision.

---

## Test 17: Integrity Condition — ORACLE_GAP

**Purpose:** Verify missing required oracle result blocks progression (non-waivable).

**Steps:**
1. Create a Loop with an oracle suite that has multiple required oracles
2. Start an iteration and register a candidate
3. Start a Run but only produce results for **some** oracles (simulate gap)
4. Attempt to mark candidate as Verified
5. Verify:
   - Verification blocked with `ORACLE_GAP` integrity condition
   - Cannot proceed even if Decision is recorded
   - UI shows integrity violation indicator

**Gap Identification:** If oracle gap detection isn't implemented, document for V10.

**Expected:** `ORACLE_GAP` is non-waivable; no Decision can override it.

**Validates:** C-OR-7 (non-waivable integrity conditions)

---

## Test 18: Integrity Condition — EVIDENCE_MISSING

**Purpose:** Verify missing evidence blocks progression (non-waivable).

**Steps:**
1. Create a Run that references an evidence bundle
2. Delete or corrupt the evidence blob in MinIO (admin action)
3. Attempt to verify the candidate using that evidence
4. Verify:
   - System detects `EVIDENCE_MISSING`
   - Verification blocked
   - Cannot be waived by Decision or Exception

**Expected:** `EVIDENCE_MISSING` is non-waivable.

**Validates:** C-OR-7 (non-waivable integrity conditions)

---

## Test 19: Actor Kind Enforcement

**Purpose:** Verify actor kind constraints are enforced.

**Steps:**

### 19a: IterationStarted Must Be SYSTEM
1. Query recent `IterationStarted` events:
   ```sql
   SELECT event_id, actor_kind, actor_id
   FROM es.events
   WHERE event_type = 'IterationStarted'
   ORDER BY global_seq DESC LIMIT 5;
   ```
2. Verify all have `actor_kind = 'SYSTEM'`

### 19b: Approvals/Decisions Must Be HUMAN
1. Query recent `ApprovalRecorded` and `DecisionRecorded` events:
   ```sql
   SELECT event_type, actor_kind, actor_id
   FROM es.events
   WHERE event_type IN ('ApprovalRecorded', 'DecisionRecorded')
   ORDER BY global_seq DESC LIMIT 5;
   ```
2. Verify all have `actor_kind = 'HUMAN'`

### 19c: Attempt AGENT Approval (Negative Test)
1. If possible, attempt to record an Approval with agent credentials
2. Verify: Request rejected with 403 Forbidden

**Expected:** Actor kind constraints enforced per SR-CONTRACT.

**Validates:** C-TB-1 (HUMAN-only for binding authority)

---

# Verification Checklists

## Executive Summary

**Validation Executed:** 2026-01-17
**V10-1 through V10-4 Verified:** 2026-01-17
**Overall Result:** 15 PASS, 2 PARTIAL, 2 DEFERRED

| Part | Tests | Pass | Partial | Gap | Blocked | Deferred |
|------|-------|------|---------|-----|---------|----------|
| A | 1-8 | 7 | 0 | 1 | 0 | 0 |
| B | 9-12 | 3 | 1 | 0 | 0 | 0 |
| C | 13-16 | 3 | 1 | 0 | 0 | 0 |
| D | 17-19 | 1 | 0 | 0 | 0 | 2 |

**V10 Critical Gaps RESOLVED:**
- ✅ BUDGET_EXHAUSTED stop trigger enforced (V10-1)
- ✅ REPEATED_FAILURE stop trigger implemented (V10-1)
- ✅ Decision-required resume gating (V10-2)
- ✅ Loop ref in IterationStarted.refs[] (V10-4)
- ✅ Candidate traceability index (V10-3)

**Remaining V10 Gaps (V10-5, V10-6):**
- V10-5: Loop PATCH endpoint for budget updates
- V10-6: OracleSuite hash doubled prefix (sha256:sha256:)

---

### Implementation Status Update (2026-01-17)

**V10-1 through V10-4 implemented** in branch `solver-ralph-10`. Code-level gaps addressed:

| Test | Gap | Implementation Status |
|------|-----|----------------------|
| 9 | Loop ref missing in refs[] | ✅ Fixed — Loop ref added with `rel="in_scope_of"` |
| 12 | Candidate iteration linkage | ✅ Fixed — Index added on `produced_by_iteration_id` |
| 13 | BUDGET_EXHAUSTED not enforced | ✅ Fixed — Enforced in `start_work_surface_iteration()` |
| 14 | REPEATED_FAILURE not implemented | ✅ Fixed — Tracks `consecutive_failures` in projection |
| 15 | Resume Decision gating | ✅ Fixed — `resume_loop()` validates `decision_id` when `requires_decision=true` |

**Verification completed:** 2026-01-17 — Tests 9, 12-15 re-run and verified passing.

---

## Part A: Happy Path (D-34)

**Execution Date:** 2026-01-17

| Test | Contract | Expected Outcome | Pass/Fail |
|------|----------|------------------|-----------|
| 1 | C-LOOP-1 | Existing Loop visible and linked | ✅ PASS |
| 2 | C-LOOP-1 | New Loop created with budgets | ✅ PASS |
| 3a | State Machine | CREATED → ACTIVE works | ✅ PASS |
| 3b | State Machine | ACTIVE → PAUSED works | ✅ PASS |
| 3c | State Machine | PAUSED → ACTIVE (manual) works | ✅ PASS |
| 3d | State Machine | Any → CLOSED works (terminal) | ✅ PASS |
| 4 | UI | All Loop detail sections render | ✅ PASS |
| 5 | C-LOOP-2 | First iteration starts correctly | ✅ PASS |
| 6 | C-LOOP-2 | Multiple iterations cycle correctly | ✅ PASS |
| 7 | C-LOOP-1 | Budget tracking accurate | ✅ PASS |
| 8 | Constraints | Edit restrictions enforced | ⚠️ GAP (no edit endpoint) |

## Part B: Context & Work Surface (C-CTX-1, C-CTX-2, C-LOOP-4)

| Test | Contract | Expected Outcome | Pass/Fail |
|------|----------|------------------|-----------|
| 9 | C-CTX-1 | Iteration refs contain minimum categories | ✅ PASS (V10-4 verified 2026-01-17: Loop ref added with `rel="in_scope_of"`) |
| 10 | C-CTX-1 | Refs have required meta fields | ⚠️ PARTIAL (OracleSuite has doubled sha256: prefix — V10-6 pending) |
| 11 | C-CTX-2 | Work Surface binding consistent | ✅ PASS |
| 12 | C-LOOP-4 | Candidate traceable to Loop/Iteration | ✅ PASS (V10-3 verified 2026-01-17: Index on `produced_by_iteration_id`) |

## Part C: Failure Modes — Waivable (D-35 Foundation)

| Test | Contract | Expected Outcome | Pass/Fail |
|------|----------|------------------|-----------|
| 13 | C-LOOP-3 | BUDGET_EXHAUSTED trigger fires | ✅ PASS (V10-1 verified 2026-01-17: StopTriggered event emitted, Loop PAUSED) |
| 14 | C-LOOP-3 | REPEATED_FAILURE trigger fires | ✅ PASS (V10-1 verified 2026-01-17: 3 consecutive failures triggers stop) |
| 15 | SR-SPEC §3.1.2 | Resume requires Decision after trigger | ✅ PASS (V10-2 verified 2026-01-17: 412 without Decision, succeeds with Decision) |
| 16 | C-EXC-* | Exception lifecycle works | ⚠️ PARTIAL (lifecycle works, refs missing) |

## Part D: Integrity Conditions — Non-Waivable (C-OR-7)

| Test | Contract | Expected Outcome | Pass/Fail |
|------|----------|------------------|-----------|
| 17 | C-OR-7 | ORACLE_GAP blocks (non-waivable) | ⚠️ DEFERRED (infra exists, E2E needed) |
| 18 | C-OR-7 | EVIDENCE_MISSING blocks (non-waivable) | ⚠️ DEFERRED (infra exists, E2E needed) |
| 19 | C-TB-1 | Actor kind constraints enforced | ✅ PASS |

---

# Gap Tracking

Gaps discovered during this validation are tracked in **SR-PLAN-GAP-ANALYSIS.md §4** (V10/V11 Roadmap).

**Summary of findings (2026-01-17):**
- ~~2 Critical gaps: Stop triggers (BUDGET_EXHAUSTED, REPEATED_FAILURE) not implemented~~ → ✅ RESOLVED (V10-1)
- ~~Candidate traceability~~ → ✅ RESOLVED (V10-3)
- ~~Loop ref missing in iteration refs~~ → ✅ RESOLVED (V10-4)
- Loop edit endpoint (V10-5) — PENDING
- OracleSuite hash prefix (V10-6) — PENDING
- 2 Deferred: Integrity condition E2E testing (to V11 automated harness)

See SR-PLAN-GAP-ANALYSIS for detailed scope and deliverable assignments.

---

# Files to Monitor

| File | Purpose |
|------|---------|
| `ui/src/pages/Loops.tsx` | Loop list page |
| `ui/src/pages/LoopDetail.tsx` | Loop detail page |
| `ui/src/components/LoopCreateModal.tsx` | Loop creation form |
| `crates/sr-api/src/handlers/loops.rs` | Loop API endpoints |
| `crates/sr-api/src/handlers/iterations.rs` | Iteration API endpoints |
| `crates/sr-api/src/handlers/work_surfaces.rs` | Work Surface API endpoints |
| `crates/sr-domain/src/state_machines.rs` | State transition logic |
| `crates/sr-domain/src/events.rs` | Event definitions |

---

# Post-Test: Event Verification

After completing all tests, run comprehensive event verification:

```sql
-- Recent Loop/Iteration events
SELECT event_type, stream_id, actor_kind, occurred_at
FROM es.events
WHERE event_type LIKE 'Loop%' OR event_type LIKE 'Iteration%'
ORDER BY global_seq DESC LIMIT 30;

-- Stop triggers
SELECT event_type, payload->>'trigger' as trigger, stream_id
FROM es.events
WHERE event_type = 'StopTriggered'
ORDER BY global_seq DESC LIMIT 10;

-- Decisions
SELECT event_id, actor_kind, payload->>'trigger' as trigger, payload->>'scope' as scope
FROM es.events
WHERE event_type = 'DecisionRecorded'
ORDER BY global_seq DESC LIMIT 10;

-- Iteration context refs (sample)
SELECT event_id, jsonb_array_length(refs) as ref_count,
       refs->0->>'kind' as first_ref_kind
FROM es.events
WHERE event_type = 'IterationStarted'
ORDER BY global_seq DESC LIMIT 5;
```

**Expected Events by Test Phase:**

| Phase | Expected Events |
|-------|-----------------|
| Part A | `LoopCreated`, `LoopActivated`, `LoopPaused`, `LoopResumed`, `LoopClosed`, `IterationStarted`, `IterationCompleted` |
| Part B | `IterationStarted` (with refs), `CandidateMaterialized` |
| Part C | `StopTriggered`, `DecisionRecorded`, `LoopResumed`, `ExceptionCreated`, `ExceptionActivated`, `ExceptionResolved` |
| Part D | Integrity violation events (if implemented) |

---

# Relationship to V10 and V11 Roadmap

## How This Validation Fits

```
Branch 0 Acceptance (COMPLETE)
         ↓
    This Plan (Loop Validation)  ←── You are here
         │
         ├── Part A & B: Validate existing works
         ├── Part C: Foundation for D-35
         └── Part D: Identify integrity gaps
         ↓
    Gap Report → Informs V10/V11 scope
         ↓
    V10 (Full Loop Governor + Automation)
         ↓
    V11 (Automated E2E Harness including D-35)
```

## V10: Automation & Scheduling Foundation

| V10 Phase | Deliverable | This Plan's Contribution |
|-----------|-------------|--------------------------|
| **V10-1** | D-22 (Loop governor - full) | Tests 1-8 validate basics; Tests 13-15 identify governor gaps |
| **V10-2** | D-38 (Prompt → Plan) | Loop infrastructure must work first |

**Expected Gaps for V10:**
- Test 14 may reveal iteration failure endpoint is incomplete
- Test 15 may reveal Decision→Resume linkage needs work
- Test 17-18 may reveal integrity condition detection missing

## V11: Production Hardening

| V11 Phase | Deliverable | This Plan's Contribution |
|-----------|-------------|--------------------------|
| **V11-5** | D-35 (E2E failure harness) | Tests 13-19 are the **manual prototype** |

**Path to D-35 Automation:**
```
This Plan (Manual)     →    V11-5 (Automated)
─────────────────────────────────────────────
Test 13 (BUDGET)       →    Automated budget exhaustion test
Test 14 (REPEATED)     →    Automated failure injection test
Test 15 (Decision)     →    Automated decision flow test
Test 17-18 (Integrity) →    Automated integrity violation tests
```

---

# Summary

By executing this plan, we:

1. **Confirm** Loop UI/API works for happy path (D-29, D-34)
2. **Validate** context provenance and Work Surface binding (C-CTX-1, C-CTX-2)
3. **Validate** candidate traceability (C-LOOP-4)
4. **Begin** failure mode coverage (D-35 foundation)
5. **Test** both waivable and non-waivable conditions (C-LOOP-3, C-OR-7)
6. **Identify** gaps to inform V10/V11 scope
7. **Document** findings to update SR-PLAN-GAP-ANALYSIS

**Execution Order:** Complete Part A before Part B, Part B before Part C, Part C before Part D. Each part builds on verified functionality from the previous part.

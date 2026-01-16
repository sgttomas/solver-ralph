# SR-PLAN-V5: Semantic Ralph Loop End-to-End Integration

**Status:** Draft
**Created:** 2026-01-15
**Supersedes:** N/A (new plan)
**Implements:** SR-CHARTER §Immediate Objective (Milestone 1: MVP)

---

## Executive Summary

SR-PLAN-V5 completes the **Semantic Ralph Loop MVP** by connecting the infrastructure built in V3 (Intakes & References) and V4 (Work Surfaces) into a functioning end-to-end workflow. The goal is to demonstrate a complete semantic work unit lifecycle:

```
Create Intake → Bind Work Surface → Create Loop → Run Iterations → Complete Stages → Human Approval at Gates → Terminal Completion
```

This plan addresses three integration gaps:

1. **Stage Advancement UI** — Users can complete stages with evidence from WorkSurfaceDetail
2. **Loop ↔ Work Surface Binding** — Loops are created bound to Work Surfaces, inheriting context
3. **Approval-Gated Stage Progression** — Stages requiring human approval block until approved

---

## Table of Contents

1. [Rationale](#1-rationale)
2. [Current State Analysis](#2-current-state-analysis)
3. [Phase 5a: Stage Advancement UI](#3-phase-5a-stage-advancement-ui)
4. [Phase 5b: Loop-Work Surface Binding](#4-phase-5b-loop-work-surface-binding)
5. [Phase 5c: Approval-Gated Stages](#5-phase-5c-approval-gated-stages)
6. [Phase 5d: End-to-End Integration Test](#6-phase-5d-end-to-end-integration-test)
7. [Implementation Order](#7-implementation-order)
8. [Verification](#8-verification)

---

## 1. Rationale

### 1.1 The MVP Gap

SR-CHARTER §Immediate Objective defines the MVP as a "functioning semantic work-unit runtime" with:

| Requirement | Current State | Gap |
|-------------|---------------|-----|
| Bounded agentic work | Work Surfaces exist | No stage advancement from UI |
| Iteration cycling | Iteration API works | Loop doesn't auto-bind Work Surface |
| Declared work surface | V4 complete | Connected but not enforced |
| Semantic oracle evidence | Evidence API works | Not wired to stage gates |
| Human authority points | Approvals API exists | Not integrated with stage progression |

### 1.2 The Stop Rule

SR-CHARTER's Stop Rule states: *"If implementation work starts to drift into adapter/platform build-out before the semantic-ralph-loop MVP is demonstrated end-to-end, stop and escalate."*

We have built substantial infrastructure but have not yet proven the core loop works. This plan directly addresses this by:
1. Making the existing pieces work together
2. Enabling an end-to-end demonstration
3. Validating the architecture before building more features

### 1.3 Success Criteria

The MVP is complete when a user can:
1. Create an Intake defining a work unit
2. Bind a Work Surface (Intake + Procedure Template)
3. Create a Loop bound to that Work Surface
4. Run Iterations that automatically receive Work Surface context
5. Complete stages with evidence, advancing through the procedure
6. For approval-required stages, record human approval before progression
7. Reach terminal stage completion, marking the Work Surface complete

---

## 2. Current State Analysis

### 2.1 What Exists (Post V3+V4)

**Loop → Work Surface Integration (Phase 4c):**
- `CreateLoopRequest` accepts optional `work_unit` field
- `StartIterationRequest` accepts optional `work_unit_id`
- When `work_unit_id` provided, Work Surface refs (Intake, ProcedureTemplate, OracleSuites) are fetched and included in `IterationStarted.refs[]`
- `LoopGovernor` validates `work_surface_available` precondition
- `LoopDetail.tsx` displays Work Surface info when available

**Stage Advancement (Phase 4b):**
- `POST /api/v1/work-surfaces/:id/stages/:stage_id/complete` endpoint exists
- Request body: `{ evidence_bundle_ref, gate_result: { status, oracle_results, waiver_refs } }`
- Auto-advances to next stage via `StageEntered` event emission
- Terminal stage triggers `WorkSurfaceCompleted` event

**Approval Workflow:**
- `POST /api/v1/approvals` — Record approval (HUMAN-only enforced)
- `Approvals.tsx` — UI for manual approval recording
- Exceptions and Decisions APIs exist
- Freeze records require `release_approval_id`

### 2.2 What's Missing

| Component | Gap | Impact |
|-----------|-----|--------|
| Stage Advancement UI | WorkSurfaceDetail.tsx has no "Complete Stage" button | Users can't advance stages from UI |
| Loop Creation Binding | Loop creation doesn't validate/enforce Work Surface | Loops can start without context |
| Approval-Gate Integration | Stage completion doesn't check for required approvals | Trust boundaries not enforced |
| Procedure Template Gate Rules | No field specifying which stages require approval | Can't determine approval requirements |

### 2.3 Key Files

| Component | Location |
|-----------|----------|
| Loop Handler | `crates/sr-api/src/handlers/loops.rs` |
| Iteration Handler | `crates/sr-api/src/handlers/iterations.rs` |
| Work Surface Handler | `crates/sr-api/src/handlers/work_surfaces.rs` |
| Approval Handler | `crates/sr-api/src/handlers/approvals.rs` |
| Loop Governor | `crates/sr-adapters/src/governor.rs` |
| Projections | `crates/sr-adapters/src/projections.rs` |
| LoopDetail UI | `ui/src/pages/LoopDetail.tsx` |
| WorkSurfaceDetail UI | `ui/src/pages/WorkSurfaceDetail.tsx` |
| Approvals UI | `ui/src/pages/Approvals.tsx` |

---

## 3. Phase 5a: Stage Advancement UI

### 3.1 Objective

Add the ability to complete stages with evidence from the WorkSurfaceDetail page.

### 3.2 UI Design

**Location:** `ui/src/pages/WorkSurfaceDetail.tsx`

**New Component: StageCompletionForm**

When a stage is "entered" (current), display a "Complete Stage" button that expands to show:

```
┌─────────────────────────────────────────────────────────┐
│ Complete Stage: DRAFT                                   │
├─────────────────────────────────────────────────────────┤
│ Evidence Bundle *                                       │
│ ┌─────────────────────────────────────────────────────┐ │
│ │ sha256:abc123...                              [▼]  │ │
│ └─────────────────────────────────────────────────────┘ │
│ (Select from recent evidence bundles or enter hash)     │
│                                                         │
│ Gate Result *                                           │
│ ○ Pass   ○ Pass with Waivers   ○ Fail                  │
│                                                         │
│ Oracle Results                              [+ Add]     │
│ ┌─────────────────────────────────────────────────────┐ │
│ │ Oracle: semantic:coherence  Status: PASS  Ref: ...  │ │
│ └─────────────────────────────────────────────────────┘ │
│                                                         │
│ Waiver References (if Pass with Waivers)                │
│ ┌─────────────────────────────────────────────────────┐ │
│ │ ex:waiver-123, ex:waiver-456                        │ │
│ └─────────────────────────────────────────────────────┘ │
│                                                         │
│                          [Cancel]  [Complete Stage]     │
└─────────────────────────────────────────────────────────┘
```

### 3.3 API Integration

**Endpoint:** `POST /api/v1/work-surfaces/:work_surface_id/stages/:stage_id/complete`

**Request:**
```typescript
interface CompleteStageRequest {
  evidence_bundle_ref: string;
  gate_result: {
    status: 'PASS' | 'PASS_WITH_WAIVERS' | 'FAIL';
    oracle_results: Array<{
      oracle_id: string;
      status: string;
      evidence_ref?: string;
    }>;
    waiver_refs: string[];
  };
}
```

**Response:**
```typescript
interface StageCompletionResponse {
  work_surface_id: string;
  completed_stage_id: string;
  next_stage_id: string | null;
  is_terminal: boolean;
  work_surface_status: 'active' | 'completed';
}
```

### 3.4 Evidence Bundle Selector

To make evidence selection user-friendly, add a dropdown that fetches recent evidence bundles:

**Endpoint:** `GET /api/v1/evidence?limit=20` (existing)

Display as:
```
sha256:abc123... (uploaded 2 hours ago, verdict: PASS)
sha256:def456... (uploaded yesterday, verdict: PASS_WITH_FINDINGS)
[Enter hash manually...]
```

### 3.5 Deliverables

| File | Action | Description |
|------|--------|-------------|
| `ui/src/pages/WorkSurfaceDetail.tsx` | EDIT | Add StageCompletionForm component |
| `ui/src/components/StageCompletionForm.tsx` | CREATE | Reusable stage completion form |
| `ui/src/components/EvidenceBundleSelector.tsx` | CREATE | Evidence bundle picker |

### 3.6 Acceptance Criteria

- [ ] "Complete Stage" button visible for current (entered) stage only
- [ ] Form validates required fields before submission
- [ ] Successful completion refreshes page showing next stage as current
- [ ] Terminal stage completion shows Work Surface as "completed"
- [ ] Error states handled (invalid evidence, already completed, etc.)

---

## 4. Phase 5b: Loop-Work Surface Binding

### 4.1 Objective

Ensure Loops are created with explicit Work Surface binding and inherit context automatically.

### 4.2 Current Flow vs Target Flow

**Current Flow:**
```
1. User creates Loop with optional work_unit
2. User starts Iteration with optional work_unit_id
3. If work_unit_id provided, Work Surface refs are fetched
```

**Target Flow:**
```
1. User creates Loop WITH required work_unit_id
2. System validates active Work Surface exists for work_unit_id
3. Loop stores work_surface_id reference
4. All Iterations automatically inherit Work Surface context
5. Loop displays bound Work Surface prominently
```

### 4.3 Backend Changes

#### 4.3.1 Loop Creation Validation

**File:** `crates/sr-api/src/handlers/loops.rs`

**Change:** When `work_unit` is provided, validate Work Surface exists:

```rust
// In create_loop handler, after parsing request:
if let Some(ref work_unit_id) = request.work_unit {
    // Query for active Work Surface
    let work_surface = sqlx::query!(
        r#"SELECT work_surface_id FROM proj.work_surfaces
           WHERE work_unit_id = $1 AND status = 'active'"#,
        work_unit_id
    )
    .fetch_optional(&*pool)
    .await?;

    if work_surface.is_none() {
        return Err(AppError::PreconditionFailed {
            code: "WORK_SURFACE_REQUIRED".into(),
            message: format!(
                "No active Work Surface found for work unit '{}'. Bind a Work Surface first.",
                work_unit_id
            ),
        });
    }
}
```

#### 4.3.2 LoopCreated Event Enhancement

**File:** `crates/sr-domain/src/events.rs`

Add `work_surface_id` to LoopCreated payload when available:

```rust
// LoopCreated payload should include:
{
    "loop_id": "...",
    "goal": "...",
    "work_unit_id": "...",
    "work_surface_id": "ws:...",  // NEW: Include if bound
    "directive_ref": {...},
    "budgets": {...}
}
```

#### 4.3.3 Automatic Iteration Context

**File:** `crates/sr-api/src/handlers/iterations.rs`

When `StartIterationRequest.work_unit_id` is not provided but Loop has a `work_unit`, auto-populate:

```rust
// In start_iteration handler:
let effective_work_unit_id = request.work_unit_id.or_else(|| {
    // Fetch Loop's work_unit from projection
    // Return it as the effective work_unit_id
});
```

### 4.4 Frontend Changes

#### 4.4.1 Loop Creation Flow

**File:** `ui/src/pages/LoopCreate.tsx` (or wherever loops are created)

Add Work Unit selector that:
1. Shows available Work Units (from active Work Surfaces)
2. Validates selection before submission
3. Displays bound Work Surface summary

#### 4.4.2 Loop List Enhancement

**File:** `ui/src/pages/Loops.tsx`

Add column showing bound Work Surface (if any):

| Loop ID | Goal | Work Surface | State | Created |
|---------|------|--------------|-------|---------|
| loop:abc | Research API rate limiting | ws:xyz (DRAFT stage) | ACTIVE | 2h ago |

### 4.5 Deliverables

| File | Action | Description |
|------|--------|-------------|
| `crates/sr-api/src/handlers/loops.rs` | EDIT | Add Work Surface validation |
| `crates/sr-api/src/handlers/iterations.rs` | EDIT | Auto-populate work_unit_id from Loop |
| `crates/sr-adapters/src/projections.rs` | EDIT | Project work_surface_id to loops table |
| `ui/src/pages/Loops.tsx` | EDIT | Add Work Surface column |
| `ui/src/pages/LoopDetail.tsx` | EDIT | Enhance Work Surface display |

### 4.6 Acceptance Criteria

- [ ] Loop creation with work_unit fails if no active Work Surface exists
- [ ] LoopCreated event includes work_surface_id when bound
- [ ] Iterations automatically receive Work Surface context from Loop
- [ ] Loops list shows bound Work Surface
- [ ] LoopDetail shows Work Surface binding prominently

---

## 5. Phase 5c: Approval-Gated Stages

### 5.1 Objective

Integrate the approval workflow with stage progression so that stages requiring human approval cannot complete until approval is recorded.

### 5.2 Design Decision: Where to Define Approval Requirements

**Option A: In Procedure Template** (Recommended)
- Each stage definition includes `requires_approval: boolean`
- Template author defines which stages are trust boundaries
- Consistent with SR-PROCEDURE-KIT stage definitions

**Option B: In Oracle Suite Configuration**
- Oracle suites define if their gate requires approval
- More flexible but less explicit

**Recommendation:** Option A — add `requires_approval` flag to stage definitions in Procedure Templates.

### 5.3 Procedure Template Enhancement

**File:** `docs/platform/SR-PROCEDURE-KIT.md`

Add to stage schema:
```yaml
stages:
  - id: "stage:frame"
    name: "Frame"
    oracle_suite: "suite:frame-check"
    requires_approval: false

  - id: "stage:eval"
    name: "Evaluation"
    oracle_suite: "suite:eval-check"
    requires_approval: true   # <-- Trust boundary

  - id: "stage:final"
    name: "Final"
    oracle_suite: "suite:final-check"
    requires_approval: true   # <-- Trust boundary
```

### 5.4 Backend: Approval Check on Stage Completion

**File:** `crates/sr-api/src/handlers/work_surfaces.rs`

Modify `complete_stage` handler to check for required approval:

```rust
async fn complete_stage(
    // ... existing params
) -> Result<Json<StageCompletionResponse>, AppError> {
    // ... existing validation ...

    // NEW: Check if stage requires approval
    let stage_def = procedure_template.get_stage(&stage_id)?;
    if stage_def.requires_approval {
        // Query for approval on this stage completion
        let approval = sqlx::query!(
            r#"SELECT approval_id FROM proj.approvals
               WHERE portal_id = $1
               AND subject_refs @> $2
               AND decision = 'APPROVED'"#,
            format!("portal:stage-gate:{}", stage_id),
            json!([{"kind": "WorkSurface", "id": work_surface_id}])
        )
        .fetch_optional(&*pool)
        .await?;

        if approval.is_none() {
            return Err(AppError::PreconditionFailed {
                code: "APPROVAL_REQUIRED".into(),
                message: format!(
                    "Stage '{}' requires approval before completion. Record approval at portal 'portal:stage-gate:{}'.",
                    stage_id, stage_id
                ),
            });
        }
    }

    // ... proceed with stage completion ...
}
```

### 5.5 Approval Request Flow

When a user attempts to complete an approval-required stage without approval:

1. **API returns 412 PRECONDITION_FAILED** with `APPROVAL_REQUIRED` code
2. **UI displays:** "This stage requires human approval. Record approval to proceed."
3. **UI provides link:** "Record Approval" → navigates to Approvals page with pre-filled portal_id
4. **User records approval** via Approvals UI
5. **User returns to WorkSurfaceDetail** and clicks "Complete Stage" again
6. **Stage completes successfully**

### 5.6 UI: Approval Status Display

**File:** `ui/src/pages/WorkSurfaceDetail.tsx`

For stages that require approval, show approval status:

```
┌─────────────────────────────────────────────────────────┐
│ Stage: EVALUATION (requires approval)                   │
│ Status: Entered                                         │
│                                                         │
│ Approval Status: ⚠️ Not yet approved                    │
│ Portal: portal:stage-gate:stage:eval                    │
│                                                         │
│ [Record Approval]  [Complete Stage (requires approval)] │
└─────────────────────────────────────────────────────────┘
```

After approval:
```
│ Approval Status: ✓ Approved by human:ryan at 2:30 PM    │
│ Approval ID: appr:abc123                                │
│                                                         │
│ [Complete Stage]                                        │
```

### 5.7 New API Endpoint: Check Stage Approval Status

**Endpoint:** `GET /api/v1/work-surfaces/:id/stages/:stage_id/approval-status`

**Response:**
```typescript
interface StageApprovalStatus {
  stage_id: string;
  requires_approval: boolean;
  approval: {
    approval_id: string;
    decision: string;
    recorded_at: string;
    recorded_by: { kind: string; id: string };
  } | null;
}
```

### 5.8 Deliverables

| File | Action | Description |
|------|--------|-------------|
| `docs/platform/SR-PROCEDURE-KIT.md` | EDIT | Add requires_approval to stage schema |
| `crates/sr-domain/src/procedure.rs` | EDIT | Add requires_approval field to Stage struct |
| `crates/sr-api/src/handlers/work_surfaces.rs` | EDIT | Add approval check to complete_stage |
| `crates/sr-api/src/handlers/work_surfaces.rs` | EDIT | Add get_stage_approval_status endpoint |
| `ui/src/pages/WorkSurfaceDetail.tsx` | EDIT | Show approval status for gated stages |
| `ui/src/pages/Approvals.tsx` | EDIT | Accept pre-filled portal_id from URL params |

### 5.9 Acceptance Criteria

- [ ] Procedure Templates can define `requires_approval` per stage
- [ ] Completing approval-required stage without approval returns 412
- [ ] UI shows approval status for gated stages
- [ ] User can navigate to Approvals page with pre-filled portal
- [ ] After recording approval, stage completion succeeds
- [ ] Approval is linked to Work Surface as subject

---

## 6. Phase 5d: End-to-End Integration Test

### 6.1 Objective

Verify the complete Semantic Ralph Loop workflow functions end-to-end.

### 6.2 Test Scenario

**Scenario: Research Memo Work Unit**

1. **Create Intake**
   - Work Unit ID: `wu:test-rate-limiting`
   - Kind: `research_memo`
   - Title: "API Rate Limiting Research"
   - Activate intake

2. **Bind Work Surface**
   - Select Intake: `wu:test-rate-limiting`
   - Select Template: `proc:research-memo` (3 stages: FRAME, DRAFT, FINAL)
   - Template stages:
     - FRAME: `requires_approval: false`
     - DRAFT: `requires_approval: false`
     - FINAL: `requires_approval: true`

3. **Create Loop**
   - Work Unit: `wu:test-rate-limiting`
   - Goal: "Research API rate limiting approaches"
   - Verify Work Surface is bound

4. **Run Iterations**
   - Start iteration 1
   - Verify IterationStarted includes Work Surface refs
   - Complete iteration

5. **Advance Stages**
   - Complete FRAME stage with evidence
   - Verify auto-advance to DRAFT
   - Complete DRAFT stage with evidence
   - Verify auto-advance to FINAL

6. **Approval Gate**
   - Attempt to complete FINAL stage
   - Verify 412 APPROVAL_REQUIRED error
   - Record approval at `portal:stage-gate:stage:final`
   - Complete FINAL stage successfully

7. **Verify Completion**
   - Work Surface status = "completed"
   - Loop can continue or close

### 6.3 Automated Test

**File:** `crates/sr-api/tests/integration/semantic_ralph_loop_e2e.rs`

```rust
#[tokio::test]
async fn test_semantic_ralph_loop_end_to_end() {
    // Setup: Create test database, seed procedure template

    // 1. Create and activate intake
    let intake = create_intake(&client, "wu:test", "research_memo").await;
    activate_intake(&client, &intake.intake_id).await;

    // 2. Bind work surface
    let ws = create_work_surface(&client, &intake.intake_id, "proc:research-memo").await;
    assert_eq!(ws.status, "active");
    assert_eq!(ws.current_stage_id, "stage:frame");

    // 3. Create loop bound to work unit
    let loop_result = create_loop(&client, "wu:test", "Test goal").await;
    assert!(loop_result.work_surface_id.is_some());

    // 4. Start iteration, verify Work Surface refs
    let iteration = start_iteration(&client, &loop_result.loop_id).await;
    assert!(iteration.refs.iter().any(|r| r.kind == "Intake"));
    assert!(iteration.refs.iter().any(|r| r.kind == "ProcedureTemplate"));

    // 5. Complete non-approval stages
    complete_stage(&client, &ws.work_surface_id, "stage:frame", &evidence_ref).await?;
    complete_stage(&client, &ws.work_surface_id, "stage:draft", &evidence_ref).await?;

    // 6. Attempt completion of approval-required stage (should fail)
    let result = complete_stage(&client, &ws.work_surface_id, "stage:final", &evidence_ref).await;
    assert!(matches!(result, Err(e) if e.code == "APPROVAL_REQUIRED"));

    // 7. Record approval
    record_approval(&client, "portal:stage-gate:stage:final", &ws.work_surface_id).await;

    // 8. Complete final stage (should succeed)
    let completion = complete_stage(&client, &ws.work_surface_id, "stage:final", &evidence_ref).await?;
    assert!(completion.is_terminal);
    assert_eq!(completion.work_surface_status, "completed");

    // 9. Verify Work Surface is completed
    let final_ws = get_work_surface(&client, &ws.work_surface_id).await;
    assert_eq!(final_ws.status, "completed");
}
```

### 6.4 Manual Verification Checklist

- [ ] Create Intake via UI, activate it
- [ ] Create Work Surface via composition wizard
- [ ] Create Loop bound to work unit
- [ ] Verify LoopDetail shows Work Surface
- [ ] Start iteration from LoopDetail
- [ ] Complete first stage from WorkSurfaceDetail
- [ ] Verify stage auto-advances
- [ ] On approval-required stage, verify blocking behavior
- [ ] Record approval via Approvals page
- [ ] Complete final stage, verify Work Surface completed

### 6.5 Deliverables

| File | Action | Description |
|------|--------|-------------|
| `crates/sr-api/tests/integration/semantic_ralph_loop_e2e.rs` | CREATE | E2E integration test |
| `docs/charter/SR-README.md` | EDIT | Document MVP completion |

---

## 7. Implementation Order

### Phase 5a: Stage Advancement UI (Foundation)
**Priority:** P0 — Required for any stage advancement
**Effort:** Medium
**Dependencies:** None

1. Create `StageCompletionForm.tsx` component
2. Create `EvidenceBundleSelector.tsx` component
3. Integrate into `WorkSurfaceDetail.tsx`
4. Test stage completion flow

### Phase 5b: Loop-Work Surface Binding (Integration)
**Priority:** P0 — Required for iteration context
**Effort:** Medium
**Dependencies:** None (parallel with 5a)

1. Add Work Surface validation to loop creation
2. Enhance LoopCreated event with work_surface_id
3. Auto-populate work_unit_id in iterations
4. Update UI (Loops list, LoopDetail)

### Phase 5c: Approval-Gated Stages (Trust Boundaries)
**Priority:** P1 — Required for MVP trust model
**Effort:** High
**Dependencies:** 5a (stage completion UI)

1. Add `requires_approval` to procedure template schema
2. Add approval check to `complete_stage` handler
3. Add approval status endpoint
4. Update UI to show approval status and links
5. Update Approvals page to accept pre-filled portal

### Phase 5d: End-to-End Test (Validation)
**Priority:** P0 — Proves MVP
**Effort:** Medium
**Dependencies:** 5a, 5b, 5c complete

1. Write integration test
2. Execute manual verification
3. Document results
4. Update SR-README with MVP status

---

## 8. Verification

### 8.1 Unit Tests

| Component | Test |
|-----------|------|
| Stage completion handler | Approval check logic |
| Procedure template parser | `requires_approval` field parsing |
| Loop creation handler | Work Surface validation |

### 8.2 Integration Tests

| Test | Verifies |
|------|----------|
| `semantic_ralph_loop_e2e.rs` | Complete workflow |
| Stage advancement | Event emission, projection updates |
| Loop-Work Surface binding | Context inheritance |

### 8.3 Build Verification

```bash
# Backend
cargo build
cargo test --workspace
cargo clippy

# Frontend
cd ui && npm run type-check
cd ui && npm run build
```

### 8.4 Manual Verification

Execute the test scenario in §6.2 against a running instance.

---

## Appendix A: API Summary

### New Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/v1/work-surfaces/:id/stages/:stage_id/approval-status` | Get approval status for stage |

### Modified Endpoints

| Method | Path | Change |
|--------|------|--------|
| POST | `/api/v1/loops` | Validate Work Surface exists for work_unit |
| POST | `/api/v1/work-surfaces/:id/stages/:stage_id/complete` | Check approval requirement |

---

## Appendix B: Schema Changes

### Procedure Template Stage Schema

```yaml
# Before
stages:
  - id: "stage:eval"
    name: "Evaluation"
    oracle_suite: "suite:eval"

# After
stages:
  - id: "stage:eval"
    name: "Evaluation"
    oracle_suite: "suite:eval"
    requires_approval: true  # NEW
```

### proj.loops Table

```sql
-- Add work_surface_id column
ALTER TABLE proj.loops
ADD COLUMN work_surface_id TEXT REFERENCES proj.work_surfaces(work_surface_id);
```

---

## Appendix C: Event Changes

### LoopCreated Payload Enhancement

```json
{
  "loop_id": "loop:abc",
  "goal": "Research rate limiting",
  "work_unit_id": "wu:research-1",
  "work_surface_id": "ws:xyz",  // NEW
  "directive_ref": {...},
  "budgets": {...}
}
```

---

## Appendix D: References

| Document | Relevance |
|----------|-----------|
| SR-CHARTER §Immediate Objective | MVP definition |
| SR-CONTRACT C-TB-3 | Portal/approval requirements |
| SR-PROCEDURE-KIT | Stage definitions |
| SR-PLAN-V4 | Work Surface implementation |
| SR-SPEC §3.2.1.1 | Iteration context requirements |

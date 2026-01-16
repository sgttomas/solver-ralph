# SR-PLAN-V6: UI Integration for MVP Workflow

**Status:** Complete
**Created:** 2026-01-16
**Validated:** 2026-01-16
**Design Resolved:** 2026-01-16
**Completed:** 2026-01-16
**Supersedes:** N/A (new plan)
**Depends On:** SR-PLAN-V5 (MVP API complete)
**Implements:** SR-CHARTER §Immediate Objective (Milestone 2: Usable MVP)

---

## Executive Summary

SR-PLAN-V6 makes the Semantic Ralph Loop MVP **usable by humans** through UI integration. The MVP API workflow is proven (via E2E tests), but currently requires curl commands to operate. This plan completes the web UI to drive the full workflow:

```
Dashboard → New Work Wizard → Work Surface Detail → Stage Completion → Approval → Done
```

**Key Finding from Research:** The UI is significantly more complete than initially anticipated. Most components from Phases 5a-5c already exist. The primary gap is wizard orchestration to start work (Loop creation + activation + iteration start).

---

## Table of Contents

1. [Research Findings](#1-research-findings)
2. [Current State Analysis](#2-current-state-analysis)
3. [Design Decisions](#3-design-decisions)
4. [Implementation Phases](#4-implementation-phases)
5. [Technical Architecture](#5-technical-architecture)
6. [Success Criteria](#6-success-criteria)
7. [Verification Steps](#7-verification-steps)

---

## 1. Research Findings

### 1.1 UI Technology Stack (Validated)

| Aspect | Finding |
|--------|---------|
| Build Tool | **Vite 5.4.11** |
| Framework | **React 18.3.1** + TypeScript 5.7.2 |
| Routing | **React Router v6** (`createBrowserRouter` with nested routes) |
| Styling | **CSS Modules** with custom theme (NOT Tailwind) |
| State Management | **React hooks + inline fetch** (NO external library) |
| API Client | **None** — direct `fetch()` calls in components |
| Auth | OIDC via Zitadel (`react-oidc-context`) with dev bypass mode |

### 1.2 Existing Components (Confirmed Present)

All Phase 5a-5c deliverables **already exist**:

| Component | Location | Size | Status |
|-----------|----------|------|--------|
| `StageCompletionForm.tsx` | `ui/src/components/` | 12.6KB | ✅ Complete |
| `EvidenceBundleSelector.tsx` | `ui/src/components/` | 5.1KB | ✅ Complete |
| `StageApprovalForm.tsx` | `ui/src/components/` | 8.5KB | ✅ Complete |
| `StageProgress.tsx` | `ui/src/components/` | — | ✅ Complete |
| `WorkSurfaceDetail.tsx` | `ui/src/pages/` | 22KB | ✅ Complete with stage progress, completion, approval |
| `WorkSurfaceCompose.tsx` | `ui/src/pages/` | 20KB | ✅ 3-step wizard (Intake → Template → Create) |
| `WorkSurfaces.tsx` | `ui/src/pages/` | 11KB | ✅ List with filtering, pagination |

### 1.3 Existing Routes

| Route | Component | Purpose |
|-------|-----------|---------|
| `/work-surfaces` | WorkSurfaces | List all work surfaces |
| `/work-surfaces/new` | WorkSurfaceCompose | Create work surface wizard |
| `/work-surfaces/:id` | WorkSurfaceDetail | View/complete stages |
| `/intakes` | Intakes | List intakes |
| `/intakes/new` | IntakeCreate | Create intake |
| `/loops` | Loops | List loops |
| `/loops/:id` | LoopDetail | Loop detail |
| `/approvals` | Approvals | Approval portal |

### 1.4 The Gap: Wizard Orchestration

The current `WorkSurfaceCompose.tsx` wizard:
1. ✅ Step 1: Select active Intake
2. ✅ Step 2: Select compatible Procedure Template
3. ✅ Step 3: Create Work Surface (`POST /work-surfaces`)
4. ✅ Navigate to Work Surface detail

**What's missing** (per E2E test in `semantic_ralph_loop_e2e.rs`):
1. ❌ Create Loop bound to work unit (`POST /loops` with `work_unit`)
2. ❌ Activate Loop (`POST /loops/{id}/activate`)
3. ❌ Start Iteration (`POST /iterations` with SYSTEM actor)

Without these steps, the Work Surface exists but has no active iteration context for stage progression.

---

## 2. Current State Analysis

### 2.1 API Endpoints (from SR-PLAN-V5)

| Endpoint | Purpose | UI Implementation |
|----------|---------|-------------------|
| `POST /intakes` | Create intake | IntakeCreate.tsx ✅ |
| `POST /intakes/{id}/activate` | Activate intake | IntakeDetail.tsx ✅ |
| `POST /work-surfaces` | Create work surface | WorkSurfaceCompose.tsx ✅ |
| `GET /work-surfaces` | List work surfaces | WorkSurfaces.tsx ✅ |
| `GET /work-surfaces/{id}` | Get work surface details | WorkSurfaceDetail.tsx ✅ |
| `POST /work-surfaces/{id}/stages/{stage}/complete` | Complete stage | StageCompletionForm.tsx ✅ |
| `GET /work-surfaces/{id}/stages/{stage}/approval-status` | Check approval | WorkSurfaceDetail.tsx ✅ |
| `POST /loops` | Create loop | ❌ Not wired to wizard |
| `POST /loops/{id}/activate` | Activate loop | ❌ Not wired to wizard |
| `POST /iterations` | Start iteration (SYSTEM) | ❌ Requires SYSTEM token |
| `POST /approvals` | Record approval | StageApprovalForm.tsx ✅ |

### 2.2 SYSTEM Token Challenge

Per SR-SPEC §2.2, iteration start requires `actor_kind=SYSTEM`. The UI operates with a HUMAN token from OIDC.

**Resolution:** Add backend endpoint `POST /work-surfaces/{id}/start` that handles Loop creation, activation, and iteration start as SYSTEM actor. This aligns with SR-SPEC intent (SYSTEM mediates iteration start).

---

## 3. Design Decisions

### 3.1 Wizard Orchestration (Validated)

**Decision:** The Work Surface wizard should orchestrate the full workflow.

**Implementation:** Extend `WorkSurfaceCompose.tsx` to call a new backend endpoint after creating the Work Surface.

### 3.2 Backend Mediation for Iteration Start

**Decision:** Add `POST /work-surfaces/{id}/start` endpoint that:
1. Creates Loop bound to work unit (if not exists)
2. Activates Loop
3. Starts Iteration as SYSTEM actor
4. Returns Loop ID and Iteration ID

**Rationale:** Per SR-SPEC §2.2, iteration start must be SYSTEM-mediated. This keeps authorization correct while simplifying UX.

### 3.3 Stage-Centric Work Surface View (Validated)

The existing `WorkSurfaceDetail.tsx` already implements this with visual progress bar, stage completion, and approval status. **No changes needed.**

### 3.4 Approval as Distinct Action (Validated)

The existing implementation separates approval (via StageApprovalForm on `/approvals` page) from stage completion. **No changes needed.**

### 3.5 Authentication Mode

**Decision:** Use test mode (`SR_AUTH_TEST_MODE=true`) for V6 validation. Real auth integration deferred.

### 3.6 SYSTEM Actor Mediation (Resolved 2026-01-16)

**Question:** How will `/start` emit SYSTEM-actor iteration events when called by a HUMAN token?

**Resolution:** Follow the established pattern from `governor.rs` and `prompt_loop.rs`:
- Hard-code `actor_kind: ActorKind::System` when emitting `IterationStarted` event
- Use `actor_id: "system:work-surface-start"` as the system identity
- The calling HUMAN actor is recorded in the Loop creation event (audit trail preserved)

**Precedent:** `governor.rs:729` uses `actor_kind: ActorKind::System` with `actor_id: "governor"` for iteration events. `prompt_loop.rs:92` uses `system_actor_id = "system:prompt-loop"`.

### 3.7 Directive Ref Handling (Resolved 2026-01-16)

**Question:** What `directive_ref` should be used for auto-created Loops?

**Resolution:** Use the same default as `prompt_loop.rs`:
```json
{
    "kind": "doc",
    "id": "SR-DIRECTIVE",
    "rel": "governs",
    "meta": {}
}
```

**Rationale:** SR-DIRECTIVE is the canonical execution policy document per SR-README. This default is already established in the codebase and semantically correct.

### 3.8 Idempotency Design (Resolved 2026-01-16)

**Question:** Should `/start` be idempotent (safe to retry)?

**Resolution:** Yes. The endpoint should be idempotent:
1. Query for existing Loop bound to `work_unit_id`
2. If Loop exists and is ACTIVE with an iteration, return existing IDs (200 OK)
3. If Loop exists but not ACTIVE, activate it and start iteration
4. If no Loop exists, create → activate → start iteration

**Rationale:** Idempotency prevents inconsistent state if the UI retries after a network failure. The frontend can safely call `/start` without checking if work has already started.

**New helper method needed:** `get_loop_by_work_unit(work_unit_id) -> Option<LoopProjection>` in `projections.rs`

---

## 4. Implementation Phases

### Phase V6-1: Backend — Start Work Endpoint

**Objective:** Add SYSTEM-mediated endpoint to start work on a Work Surface.

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `crates/sr-adapters/src/projections.rs` | EDIT | Add `get_loop_by_work_unit` method |
| `crates/sr-api/src/handlers/work_surfaces.rs` | EDIT | Add `start_work_surface` handler |
| `crates/sr-api/src/main.rs` | EDIT | Register `/work-surfaces/{id}/start` route |

**New Projection Method (`projections.rs`):**
```rust
/// Get a loop by work_unit ID (for idempotency checks)
pub async fn get_loop_by_work_unit(&self, work_unit_id: &str) -> Result<Option<LoopProjection>, ProjectionError> {
    let result = sqlx::query(
        r#"
        SELECT loop_id, goal, work_unit, work_surface_id, state, budgets, directive_ref,
               created_by_kind, created_by_id, created_at, activated_at,
               closed_at, iteration_count
        FROM proj.loops WHERE work_unit = $1
        ORDER BY created_at DESC
        LIMIT 1
        "#,
    )
    .bind(work_unit_id)
    .fetch_optional(&self.pool)
    .await?;

    Ok(result.map(|row| /* ... same as get_loop ... */))
}
```

**Handler Logic (`work_surfaces.rs`):**
```rust
/// Response for starting work on a Work Surface
#[derive(Debug, Serialize)]
pub struct StartWorkResponse {
    pub work_surface_id: String,
    pub loop_id: String,
    pub iteration_id: String,
    pub already_started: bool, // true if idempotent return
}

/// Start work on a Work Surface (HUMAN-callable, SYSTEM-mediated iteration)
///
/// POST /api/v1/work-surfaces/{id}/start
///
/// Per SR-PLAN-V6 §3.6-3.8:
/// - Creates Loop bound to work_unit (or reuses existing)
/// - Activates Loop if needed
/// - Starts Iteration as SYSTEM actor
/// - Idempotent: safe to retry
#[instrument(skip(state, user), fields(user_id = %user.actor_id))]
pub async fn start_work_surface(
    State(state): State<WorkSurfaceState>,
    user: AuthenticatedUser,
    Path(work_surface_id): Path<String>,
) -> ApiResult<Json<StartWorkResponse>> {
    // 1. Get work surface and verify active
    let ws = get_work_surface_projection(&state.app_state, &work_surface_id).await?;
    if ws.status != "active" {
        return Err(ApiError::PreconditionFailed {
            code: "WORK_SURFACE_NOT_ACTIVE".to_string(),
            message: format!("Work Surface status is '{}', must be 'active'", ws.status),
        });
    }

    // 2. Check for existing Loop (idempotency per §3.8)
    let existing_loop = state.app_state.projections
        .get_loop_by_work_unit(&ws.work_unit_id)
        .await?;

    let (loop_id, already_started) = match existing_loop {
        Some(loop_proj) if loop_proj.state == "ACTIVE" && loop_proj.iteration_count > 0 => {
            // Already fully started - idempotent return
            let iterations = state.app_state.projections.get_iterations(&loop_proj.loop_id).await?;
            let latest_iter = iterations.last().ok_or_else(|| ApiError::Internal {
                message: "Loop has iteration_count > 0 but no iterations found".to_string(),
            })?;
            return Ok(Json(StartWorkResponse {
                work_surface_id,
                loop_id: loop_proj.loop_id,
                iteration_id: latest_iter.iteration_id.clone(),
                already_started: true,
            }));
        }
        Some(loop_proj) => {
            // Loop exists but needs activation or iteration
            (loop_proj.loop_id, false)
        }
        None => {
            // Create new Loop
            let new_loop_id = create_loop_internal(&state, &ws, &user).await?;
            (new_loop_id, false)
        }
    };

    // 3. Activate Loop if not ACTIVE
    let loop_proj = state.app_state.projections.get_loop(&loop_id).await?
        .ok_or_else(|| ApiError::Internal { message: "Loop just created/found is missing".to_string() })?;

    if loop_proj.state == "CREATED" {
        activate_loop_internal(&state.app_state, &loop_id, &user).await?;
    }

    // 4. Start Iteration as SYSTEM (per §3.6)
    let iteration_id = start_iteration_as_system(&state.app_state, &loop_id, &ws.work_unit_id).await?;

    info!(
        work_surface_id = %work_surface_id,
        loop_id = %loop_id,
        iteration_id = %iteration_id,
        "Work started on Work Surface"
    );

    Ok(Json(StartWorkResponse {
        work_surface_id,
        loop_id,
        iteration_id,
        already_started,
    }))
}

/// Create Loop with default directive_ref (per §3.7)
async fn create_loop_internal(
    state: &WorkSurfaceState,
    ws: &WorkSurfaceResponse,
    user: &AuthenticatedUser,
) -> Result<String, ApiError> {
    let loop_id = LoopId::new();
    let event_id = EventId::new();
    let now = Utc::now();

    // Default directive_ref per §3.7
    let directive_ref = serde_json::json!({
        "kind": "doc",
        "id": "SR-DIRECTIVE",
        "rel": "governs",
        "meta": {}
    });

    let payload = serde_json::json!({
        "goal": format!("Process work surface {}", ws.work_surface_id),
        "work_unit": ws.work_unit_id,
        "work_surface_id": ws.work_surface_id,
        "budgets": {
            "max_iterations": 5,
            "max_oracle_runs": 25,
            "max_wallclock_hours": 16
        },
        "directive_ref": directive_ref
    });

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: loop_id.as_str().to_string(),
        stream_kind: StreamKind::Loop,
        stream_seq: 1,
        global_seq: None,
        event_type: "LoopCreated".to_string(),
        occurred_at: now,
        actor_kind: user.actor_kind.clone(), // HUMAN creates the Loop
        actor_id: user.actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: vec![],
        payload,
        envelope_hash: compute_envelope_hash(&event_id),
    };

    state.app_state.event_store.append(loop_id.as_str(), 0, vec![event]).await?;
    state.app_state.projections.process_events(&*state.app_state.event_store).await?;

    Ok(loop_id.as_str().to_string())
}

/// Start iteration as SYSTEM actor (per §3.6)
async fn start_iteration_as_system(
    state: &AppState,
    loop_id: &str,
    work_unit_id: &str,
) -> Result<String, ApiError> {
    // Per §3.6: Use SYSTEM actor for iteration start
    let system_actor_kind = ActorKind::System;
    let system_actor_id = "system:work-surface-start".to_string();

    let iteration_id = IterationId::new();
    let event_id = EventId::new();
    let now = Utc::now();

    // Get iteration sequence
    let iterations = state.projections.get_iterations(loop_id).await?;
    let sequence = (iterations.len() + 1) as u32;

    // Fetch Work Surface refs for iteration context
    let refs = fetch_work_surface_refs(state, work_unit_id).await?;

    let payload = serde_json::json!({
        "loop_id": loop_id,
        "iteration_number": sequence,
    });

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: iteration_id.as_str().to_string(),
        stream_kind: StreamKind::Iteration,
        stream_seq: 1,
        global_seq: None,
        event_type: "IterationStarted".to_string(),
        occurred_at: now,
        actor_kind: system_actor_kind, // SYSTEM per §3.6
        actor_id: system_actor_id,
        correlation_id: Some(loop_id.to_string()),
        causation_id: None,
        supersedes: vec![],
        refs,
        payload,
        envelope_hash: compute_envelope_hash(&event_id),
    };

    state.event_store.append(iteration_id.as_str(), 0, vec![event]).await?;
    state.projections.process_events(&*state.event_store).await?;

    Ok(iteration_id.as_str().to_string())
}
```

**Acceptance Criteria:**
- [ ] `POST /work-surfaces/{id}/start` creates Loop, activates, starts Iteration
- [ ] Iteration is emitted with `actor_kind=SYSTEM`, `actor_id="system:work-surface-start"`
- [ ] Loop uses default `directive_ref` pointing to SR-DIRECTIVE
- [ ] Returns Loop ID and Iteration ID
- [ ] 412 if Work Surface not active
- [ ] Idempotent: returns existing IDs with `already_started: true` if called again
- [ ] HUMAN actor recorded on LoopCreated event (audit trail)

### Phase V6-2: Frontend — Wizard Completion

**Objective:** Wire wizard to call `/start` after creating Work Surface.

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `ui/src/pages/WorkSurfaceCompose.tsx` | EDIT | Call `/start` endpoint after creation |

**Changes to `handleSubmit` in `WorkSurfaceCompose.tsx`:**
```typescript
const handleSubmit = async () => {
  // ... existing validation ...

  setSubmitting(true);
  setError(null);

  try {
    // Step 1: Create Work Surface (existing)
    const wsRes = await fetch(`${config.apiUrl}/api/v1/work-surfaces`, {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${auth.user.access_token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        work_unit_id: selectedIntake.work_unit_id,
        intake_id: selectedIntake.intake_id,
        procedure_template_id: selectedTemplate.procedure_template_id,
        params: {},
      }),
    });

    if (!wsRes.ok) {
      const errorData = await wsRes.json().catch(() => ({}));
      throw new Error(errorData.message || `Failed to create work surface: HTTP ${wsRes.status}`);
    }

    const wsData = await wsRes.json();

    // Step 2: Start work (NEW)
    const startRes = await fetch(`${config.apiUrl}/api/v1/work-surfaces/${wsData.work_surface_id}/start`, {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${auth.user.access_token}`,
        'Content-Type': 'application/json',
      },
    });

    if (!startRes.ok) {
      // Work surface created but start failed - still navigate, show warning
      console.warn('Work surface created but start failed:', startRes.status);
    }

    navigate(`/work-surfaces/${wsData.work_surface_id}`);
  } catch (err) {
    setError(err instanceof Error ? err.message : 'Failed to create work surface');
  } finally {
    setSubmitting(false);
  }
};
```

**Acceptance Criteria:**
- [ ] Wizard creates Work Surface AND starts work
- [ ] User arrives at WorkSurfaceDetail with Loop/Iteration active
- [ ] "Complete Stage" button is functional
- [ ] Graceful handling if start fails (navigate anyway, log warning)

### Phase V6-3: End-to-End Verification

**Objective:** Document and verify the complete human workflow.

**Verification Script:**
1. Start API: `SR_AUTH_TEST_MODE=true cargo run --package sr-api`
2. Start UI: `cd ui && npm run dev`
3. Open browser to `http://localhost:5173`
4. Navigate to `/intakes/new`, create and activate an intake
5. Navigate to `/work-surfaces/new`
6. Select intake, select template, click "Create Work Surface"
7. Verify redirect to `/work-surfaces/{id}`
8. Complete stages: FRAME → OPTIONS → DRAFT → FINAL
9. For FINAL: click "Record Approval", approve, then complete
10. Verify Work Surface status = "completed"

**Acceptance Criteria:**
- [x] Full workflow completes without curl commands
- [x] All stages can be completed via UI
- [x] Approval flow works for FINAL stage
- [x] No console errors during workflow

**Verification Completed:** 2026-01-16

---

## 5. Technical Architecture

### 5.1 API Pattern (Existing)

The codebase uses **inline `fetch()` calls** with auth token from OIDC context. This pattern should be maintained for consistency.

```typescript
// Existing pattern (do NOT change to centralized client)
const res = await fetch(`${config.apiUrl}/api/v1/endpoint`, {
  method: 'POST',
  headers: {
    Authorization: `Bearer ${auth.user.access_token}`,
    'Content-Type': 'application/json',
  },
  body: JSON.stringify(payload),
});
```

### 5.2 Styling (Existing)

CSS Modules with custom theme variables. Use existing classes from `styles/pages.module.css`.

**Theme colors (from `styles/theme.css`):**
- Paper: `#ede4d4` / `#e5dac8`
- Ink: `#1a110d`
- Accent: `#ce8c55` (burnt caramel)
- Success: `#2f6b4f`
- Warning: `#9a6a16`

### 5.3 Component Hierarchy (Existing)

```
App (routes.tsx)
├── /work-surfaces → WorkSurfaces (list)
├── /work-surfaces/new → WorkSurfaceCompose (wizard)
├── /work-surfaces/:id → WorkSurfaceDetail
│   ├── StageProgress (visual)
│   ├── StageCompletionForm (when completing)
│   └── Approval status (when required)
├── /intakes/new → IntakeCreate
├── /approvals → Approvals
│   └── StageApprovalForm (when recording)
└── /loops/:id → LoopDetail
```

---

## 6. Success Criteria

### 6.1 Functional Criteria

A user can:
1. ✅ Open the UI in a browser
2. ✅ See existing work surfaces at `/work-surfaces`
3. ✅ Create an intake at `/intakes/new`
4. ✅ Activate the intake
5. ✅ Start wizard at `/work-surfaces/new`
6. ✅ Select intake, select template
7. **NEW** Click "Create" → Loop created, activated, iteration started
8. ✅ See stage progression at `/work-surfaces/{id}`
9. ✅ Complete stages via "Complete Stage" button
10. ✅ For FINAL: record approval first via "Record Approval" link
11. ✅ Complete FINAL stage
12. ✅ See Work Surface status = "completed"

### 6.2 Technical Criteria

- [ ] No console errors in browser during workflow
- [ ] API errors displayed meaningfully
- [ ] Navigation works (back button, direct URL)
- [ ] Builds pass: `npm run type-check && npm run build`
- [ ] Backend builds: `cargo build --package sr-api`

### 6.3 Non-Criteria (Out of Scope)

- Production-ready error handling
- Mobile-responsive design
- Real OIDC authentication
- File upload for evidence
- Multiple iteration management
- Real semantic oracle evaluation

---

## 7. Verification Steps

### 7.1 Build Verification

```bash
# Backend
cd /Users/ryan/ai-env/projects/solver-ralph
cargo build --package sr-api
cargo test --package sr-api

# Frontend
cd ui
npm run type-check
npm run build
```

### 7.2 Runtime Verification

```bash
# Terminal 1: Start API in test mode
SR_AUTH_TEST_MODE=true cargo run --package sr-api

# Terminal 2: Start UI dev server
cd ui && npm run dev

# Terminal 3: Watch for errors
# Open browser to http://localhost:5173
```

### 7.3 Manual Test Script

1. **Create Intake**
   - Navigate to `/intakes/new`
   - Fill out: title, kind (research_memo), objective, deliverables
   - Click "Create Intake"
   - Click "Activate" on detail page

2. **Start Work**
   - Navigate to `/work-surfaces/new`
   - Select the activated intake
   - Select a procedure template
   - Click "Create Work Surface"
   - Verify redirect to detail page

3. **Complete Stages**
   - Click "Complete Stage" for FRAME
   - Fill form, submit
   - Repeat for OPTIONS, DRAFT
   - For FINAL: click "Record Approval" first

4. **Verify Completion**
   - After FINAL completes, status should be "completed"
   - Navigate back to `/work-surfaces` to see completed item

---

## Appendix A: Files Reference

### Backend Files

| File | Purpose |
|------|---------|
| `crates/sr-adapters/src/projections.rs` | Add `get_loop_by_work_unit` method |
| `crates/sr-api/src/handlers/work_surfaces.rs` | Work Surface handlers (add `start`) |
| `crates/sr-api/src/handlers/loops.rs` | Loop creation/activation (reference) |
| `crates/sr-api/src/handlers/iterations.rs` | Iteration start (reference for SYSTEM pattern) |
| `crates/sr-api/src/main.rs` | Route registration |

### Frontend Files

| File | Purpose |
|------|---------|
| `ui/src/pages/WorkSurfaceCompose.tsx` | Wizard (add `/start` call) |
| `ui/src/pages/WorkSurfaceDetail.tsx` | Detail view (existing, complete) |
| `ui/src/pages/WorkSurfaces.tsx` | List view (existing, complete) |
| `ui/src/components/StageCompletionForm.tsx` | Stage completion (existing) |
| `ui/src/components/StageApprovalForm.tsx` | Approval form (existing) |

### Documentation Files

| File | Purpose |
|------|---------|
| `docs/platform/SR-SPEC.md` | Platform mechanics |
| `docs/platform/SR-WORK-SURFACE.md` | Work surface definitions |
| `docs/platform/SR-CONTRACT.md` | C-TB-3 for approvals |
| `crates/sr-api/tests/integration/semantic_ralph_loop_e2e.rs` | API workflow reference |

# SR-PLAN-V6: UI Integration for MVP Workflow

**Status:** Ready for Implementation
**Created:** 2026-01-16
**Validated:** 2026-01-16
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

---

## 4. Implementation Phases

### Phase V6-1: Backend — Start Work Endpoint

**Objective:** Add SYSTEM-mediated endpoint to start work on a Work Surface.

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `crates/sr-api/src/handlers/work_surfaces.rs` | EDIT | Add `start_work_surface` handler |
| `crates/sr-api/src/main.rs` | EDIT | Register `/work-surfaces/{id}/start` route |

**Handler Logic:**
```rust
// POST /work-surfaces/{id}/start
// Actor: HUMAN (but iteration emitted as SYSTEM)
async fn start_work_surface(
    Path(work_surface_id): Path<String>,
    State(state): State<AppState>,
    auth: AuthenticatedActor,
) -> Result<Json<StartWorkResponse>, ApiError> {
    // 1. Get work surface
    let ws = get_work_surface(&state.pool, &work_surface_id).await?;

    // 2. Check status is active
    if ws.status != "active" {
        return Err(ApiError::precondition_failed("WORK_SURFACE_NOT_ACTIVE"));
    }

    // 3. Create loop bound to work unit
    let loop_id = create_loop(&state, CreateLoopRequest {
        goal: format!("Process work surface {}", work_surface_id),
        work_unit: Some(ws.work_unit_id.clone()),
        budgets: default_budgets(),
    }, auth.clone()).await?;

    // 4. Activate loop
    activate_loop(&state, &loop_id).await?;

    // 5. Start iteration as SYSTEM
    let iteration_id = start_iteration_as_system(&state, &loop_id, &ws.work_unit_id).await?;

    Ok(Json(StartWorkResponse {
        loop_id,
        iteration_id,
        work_surface_id,
    }))
}
```

**Acceptance Criteria:**
- [ ] `POST /work-surfaces/{id}/start` creates Loop, activates, starts Iteration
- [ ] Iteration is emitted with `actor_kind=SYSTEM`
- [ ] Returns Loop ID and Iteration ID
- [ ] 412 if Work Surface not active
- [ ] 409 if Loop already exists for work unit

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
- [ ] Full workflow completes without curl commands
- [ ] All stages can be completed via UI
- [ ] Approval flow works for FINAL stage
- [ ] No console errors during workflow

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
| `crates/sr-api/src/handlers/work_surfaces.rs` | Work Surface handlers (add `start`) |
| `crates/sr-api/src/handlers/loops.rs` | Loop creation/activation |
| `crates/sr-api/src/handlers/iterations.rs` | Iteration start (SYSTEM) |
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

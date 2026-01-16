# SR-PLAN-V6: UI Integration for MVP Workflow

**Status:** Draft (Pending Research & Validation)
**Created:** 2026-01-16
**Supersedes:** N/A (new plan)
**Depends On:** SR-PLAN-V5 (MVP API complete)
**Implements:** SR-CHARTER §Immediate Objective (Milestone 2: Usable MVP)

---

## Executive Summary

SR-PLAN-V6 makes the Semantic Ralph Loop MVP **usable by humans** through UI integration. The MVP API workflow is proven (via E2E tests), but currently requires curl commands to operate. This plan delivers a web UI that drives the complete workflow:

```
Dashboard → New Work Wizard → Work Surface Detail → Stage Completion → Approval → Done
```

The goal is not to build a production-ready application, but to prove the MVP workflow is usable from a human interface.

---

## Table of Contents

1. [Rationale](#1-rationale)
2. [Current State Analysis](#2-current-state-analysis)
3. [Design Decisions & Assumptions](#3-design-decisions--assumptions)
4. [Proposed Phases](#4-proposed-phases)
5. [Technical Architecture](#5-technical-architecture)
6. [Open Questions](#6-open-questions)
7. [Research Tasks for Next Instance](#7-research-tasks-for-next-instance)
8. [Success Criteria](#8-success-criteria)

---

## 1. Rationale

### 1.1 The Usability Gap

SR-PLAN-V5 completed the MVP API workflow. The E2E test (`semantic_ralph_loop_e2e.rs`) proves:
- Intake creation and activation
- Work Surface binding with procedure template
- Loop creation with Work Surface validation
- Iteration start with context inheritance
- Stage progression through FRAME → OPTIONS → DRAFT → FINAL
- Approval-gated trust boundaries
- Work Surface completion

**However**, executing this workflow requires 10+ curl commands with correct sequencing. This is not how humans work.

### 1.2 Why UI Now

The MVP is architecturally complete but practically unusable. Before building more features (real oracles, LLM integration, evidence management), we should validate that the workflow makes sense from a user's perspective. A UI forces us to confront UX questions that curl commands hide.

### 1.3 Scope Boundary

This plan focuses on **workflow integration**, not polish:
- Functional forms, not beautiful designs
- Working flows, not edge case handling
- Proof of integration, not production readiness

---

## 2. Current State Analysis

### 2.1 API Endpoints (from SR-PLAN-V5)

| Endpoint | Purpose | Used In UI |
|----------|---------|------------|
| `POST /intakes` | Create intake | New Work Wizard |
| `POST /intakes/{id}/activate` | Activate intake | New Work Wizard |
| `GET /intakes/{id}` | Get intake details | Work Surface Detail |
| `POST /work-surfaces` | Create work surface | New Work Wizard |
| `GET /work-surfaces` | List work surfaces | Dashboard |
| `GET /work-surfaces/{id}` | Get work surface details | Work Surface Detail |
| `POST /work-surfaces/{id}/stages/{stage}/complete` | Complete stage | Stage Completion |
| `POST /loops` | Create loop | New Work Wizard |
| `POST /loops/{id}/activate` | Activate loop | New Work Wizard |
| `POST /iterations` | Start iteration (SYSTEM) | New Work Wizard |
| `POST /approvals` | Record approval | Approval Panel |

### 2.2 Existing UI Components (To Be Validated)

The `ui/` directory contains a React/TypeScript application. Components that may exist from earlier phases:

| Component | Expected Location | Status |
|-----------|------------------|--------|
| StageCompletionForm | `ui/src/components/` | Phase 5a deliverable - verify exists |
| WorkSurfaceDetail | `ui/src/pages/` or `ui/src/components/` | May exist from Phase 4 |
| Dashboard/Home | `ui/src/pages/` | Unknown |
| API Client | `ui/src/api/` or `ui/src/services/` | Unknown |

**Research Required:** Inventory existing UI components and patterns before finalizing plan.

### 2.3 Authentication State

The API supports `SR_AUTH_TEST_MODE=true` which accepts any Bearer token and determines actor type by token content:
- Tokens containing "system" → SYSTEM actor
- Tokens containing "agent" → AGENT actor
- Otherwise → HUMAN actor

**Decision Point:** Use test mode for V6 or implement real auth?

---

## 3. Design Decisions & Assumptions

### 3.1 Combine API Calls in Wizard

**Decision:** The "New Work Wizard" should orchestrate multiple API calls behind a simple UX.

**Rationale:** The raw API requires 6+ sequential calls:
1. Create intake
2. Activate intake
3. Create work surface
4. Create loop
5. Activate loop
6. Start iteration

This separation is correct for the domain model (auditability, flexibility) but poor UX. The wizard presents: "Define work → Choose template → Start" and handles orchestration.

**Assumption:** This aligns with SR-SPEC intent. Validate by reading SR-SPEC and SR-INTENT.

### 3.2 Stage-Centric Work Surface View

**Decision:** The Work Surface Detail view centers on the current stage with visual progress indicator.

**Rationale:** Users think "I'm on step 3 of 4" not "current_stage_id=stage:DRAFT". The procedure template defines the stage sequence; the UI should make this progression visible and intuitive.

### 3.3 Approval as Distinct Action

**Decision:** Approval and stage completion are separate UI actions for trust boundary stages.

**Rationale:** Per SR-CONTRACT C-TB-3, trust boundaries require HUMAN approval. Making this a distinct action (not just a checkbox) emphasizes the governance meaning. The user must consciously approve before they can complete.

### 3.4 Hide Iteration Complexity

**Decision:** Iterations are infrastructure, not user-facing concepts (for now).

**Rationale:** The MVP auto-creates one iteration per work session. Users don't need to manage iterations directly. The wizard starts an iteration automatically; the UI doesn't expose iteration management.

**Assumption:** This may change when we support multiple iterations per work surface. Revisit if needed.

### 3.5 Hardcode Single Template

**Decision:** Use `proc:RESEARCH-MEMO` template without template browsing UI.

**Rationale:** Only one template exists in the starter registry. Building template browsing UI is scope creep. Hardcode for V6; the architecture supports expansion later.

---

## 4. Proposed Phases

### Phase UI-1: Foundation & Dashboard

**Objective:** Establish UI infrastructure and show existing work surfaces.

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `ui/src/api/client.ts` | CREATE/EDIT | API client with typed endpoints |
| `ui/src/api/types.ts` | CREATE/EDIT | TypeScript types matching API |
| `ui/src/pages/Dashboard.tsx` | CREATE/EDIT | List work surfaces with status |
| `ui/src/components/WorkSurfaceCard.tsx` | CREATE | Summary card for dashboard list |

**Acceptance Criteria:**
- [ ] API client can call all required endpoints
- [ ] Dashboard displays list of work surfaces from API
- [ ] Each card shows: title, current stage, status
- [ ] Clicking card navigates to detail view (stub)

### Phase UI-2: New Work Wizard

**Objective:** Single flow to go from "idea" to "active work surface".

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `ui/src/components/NewWorkWizard/index.tsx` | CREATE | Multi-step wizard container |
| `ui/src/components/NewWorkWizard/IntakeForm.tsx` | CREATE | Step 1: Define intake |
| `ui/src/components/NewWorkWizard/TemplateSelect.tsx` | CREATE | Step 2: Choose template (hardcoded) |
| `ui/src/components/NewWorkWizard/Confirm.tsx` | CREATE | Step 3: Review and start |

**Acceptance Criteria:**
- [ ] User can fill out intake form (title, kind, objective, deliverables, constraints)
- [ ] Template selection shows RESEARCH-MEMO with stage preview
- [ ] "Start Work" button orchestrates: create intake → activate → create work surface → create loop → activate → start iteration
- [ ] On success, navigates to Work Surface Detail
- [ ] On error, shows meaningful message

### Phase UI-3: Work Surface Detail

**Objective:** View and interact with an active work surface.

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `ui/src/pages/WorkSurfaceDetail.tsx` | CREATE/EDIT | Main detail view |
| `ui/src/components/StageProgress.tsx` | CREATE | Visual stage progression (FRAME → OPTIONS → DRAFT → FINAL) |
| `ui/src/components/StagePanel.tsx` | CREATE | Current stage info and actions |
| `ui/src/components/StageCompletionForm.tsx` | EDIT | Integrate with complete endpoint (may exist from 5a) |

**Acceptance Criteria:**
- [ ] Shows work surface title, status, intake summary
- [ ] Visual progress bar/steps showing stage progression
- [ ] Current stage highlighted with "Complete Stage" action
- [ ] Completed stages show checkmark
- [ ] Future stages show as locked/pending

### Phase UI-4: Approval Flow

**Objective:** Enable approval for trust boundary stages (FINAL).

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `ui/src/components/ApprovalPanel.tsx` | CREATE | Approval UI for gated stages |
| `ui/src/pages/WorkSurfaceDetail.tsx` | EDIT | Integrate approval panel |

**Acceptance Criteria:**
- [ ] FINAL stage shows "Requires Approval" indicator
- [ ] Approval panel with rationale input and Approve/Reject buttons
- [ ] After approval, "Complete Stage" becomes enabled
- [ ] Completion without approval shows error (412)

### Phase UI-5: End-to-End Verification

**Objective:** Manual testing of complete flow.

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `docs/planning/SR-PLAN-V6-VERIFICATION.md` | CREATE | Test script and results |

**Acceptance Criteria:**
- [ ] Complete flow documented with screenshots
- [ ] All happy path scenarios work
- [ ] Error scenarios show appropriate messages

---

## 5. Technical Architecture

### 5.1 API Client Pattern

```typescript
// ui/src/api/client.ts
const API_BASE = import.meta.env.VITE_API_URL || 'http://localhost:3000/api/v1';

async function request<T>(method: string, path: string, body?: unknown): Promise<T> {
  const response = await fetch(`${API_BASE}${path}`, {
    method,
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${getToken()}`, // test-token or system-token
    },
    body: body ? JSON.stringify(body) : undefined,
  });

  if (!response.ok) {
    const error = await response.json();
    throw new ApiError(error.code, error.error);
  }

  return response.json();
}

export const api = {
  intakes: {
    create: (data: CreateIntakeRequest) => request<IntakeResponse>('POST', '/intakes', data),
    activate: (id: string) => request<IntakeResponse>('POST', `/intakes/${id}/activate`),
    get: (id: string) => request<IntakeResponse>('GET', `/intakes/${id}`),
  },
  workSurfaces: {
    list: () => request<ListWorkSurfacesResponse>('GET', '/work-surfaces'),
    get: (id: string) => request<WorkSurfaceResponse>('GET', `/work-surfaces/${id}`),
    create: (data: CreateWorkSurfaceRequest) => request<WorkSurfaceResponse>('POST', '/work-surfaces', data),
    completeStage: (wsId: string, stageId: string, data: CompleteStageRequest) =>
      request<StageCompletionResponse>('POST', `/work-surfaces/${wsId}/stages/${stageId}/complete`, data),
  },
  loops: {
    create: (data: CreateLoopRequest) => request<LoopResponse>('POST', '/loops', data),
    activate: (id: string) => request<LoopResponse>('POST', `/loops/${id}/activate`),
  },
  iterations: {
    start: (data: StartIterationRequest) => request<IterationResponse>('POST', '/iterations', data),
  },
  approvals: {
    record: (data: RecordApprovalRequest) => request<ApprovalResponse>('POST', '/approvals', data),
  },
};
```

### 5.2 Component Hierarchy

```
App
├── Dashboard
│   └── WorkSurfaceCard (×n)
├── NewWorkWizard
│   ├── IntakeForm
│   ├── TemplateSelect
│   └── Confirm
└── WorkSurfaceDetail
    ├── StageProgress
    ├── StagePanel
    │   ├── StageCompletionForm
    │   └── ApprovalPanel (if requires_approval)
    └── IntakeSummary
```

### 5.3 State Management

**Recommendation:** Start simple with React Query or SWR for API state. Avoid Redux/Zustand unless complexity demands it.

```typescript
// Example with React Query
const { data: workSurfaces } = useQuery('workSurfaces', api.workSurfaces.list);
const { data: workSurface } = useQuery(['workSurface', id], () => api.workSurfaces.get(id));
```

---

## 6. Open Questions

| Question | Options | Recommendation |
|----------|---------|----------------|
| Authentication | Test mode vs. real auth | Test mode for V6; defer real auth |
| Evidence bundles | Placeholder vs. file upload | Placeholder hashes; defer upload |
| Template selection | Hardcode vs. browse | Hardcode RESEARCH-MEMO |
| Routing library | React Router vs. other | Check existing UI patterns |
| Styling | Tailwind vs. CSS modules vs. other | Check existing UI patterns |
| Form handling | React Hook Form vs. native | Check existing UI patterns |

---

## 7. Research Tasks for Next Instance

Before finalizing this plan, the implementing instance should:

### 7.1 UI Codebase Inventory

```bash
# Run these to understand existing UI structure
ls -la ui/src/
ls -la ui/src/components/
ls -la ui/src/pages/
ls -la ui/src/api/ || ls -la ui/src/services/
```

Questions to answer:
- What routing library is used?
- What styling approach is used?
- What state management exists?
- Does StageCompletionForm exist from Phase 5a?
- Is there an existing API client?

### 7.2 Documentation Review

Read these files for context:
- `docs/platform/SR-SPEC.md` — Platform mechanics, any UI guidance
- `docs/platform/SR-WORK-SURFACE.md` — Work surface definitions
- `docs/platform/SR-INTENT.md` — Design rationale
- `crates/sr-api/tests/integration/semantic_ralph_loop_e2e.rs` — API workflow reference

### 7.3 API Response Validation

Verify API response shapes match assumptions:
```bash
# With API running (SR_AUTH_TEST_MODE=true cargo run --package sr-api)
curl -s http://localhost:3000/api/v1/work-surfaces -H "Authorization: Bearer test" | jq .
curl -s http://localhost:3000/api/v1/templates?category=work-surface -H "Authorization: Bearer test" | jq .
```

### 7.4 Validate Assumptions

- [ ] Confirm wizard orchestration aligns with SR-SPEC intent
- [ ] Confirm approval flow matches SR-CONTRACT C-TB-3
- [ ] Confirm portal ID convention: `portal:STAGE_COMPLETION:{stage_id}`
- [ ] Check if RESEARCH-MEMO template has `requires_approval: true` on FINAL stage

---

## 8. Success Criteria

### 8.1 Functional Criteria

A user can:
1. Open the UI in a browser
2. See a dashboard of existing work surfaces (empty initially)
3. Click "New Work" and fill out an intake form
4. Choose a procedure template (RESEARCH-MEMO)
5. Click "Start" and be taken to the new work surface
6. See stage progression: FRAME → OPTIONS → DRAFT → FINAL
7. Click "Complete Stage" for FRAME, OPTIONS, DRAFT
8. See FINAL requires approval
9. Approve with rationale
10. Complete FINAL
11. See work surface status = "completed"
12. Return to dashboard and see completed work surface

### 8.2 Technical Criteria

- [ ] No console errors in browser
- [ ] API errors displayed to user meaningfully
- [ ] Navigation works (back button, direct URL)
- [ ] Responsive enough to use (not mobile-optimized)

### 8.3 Non-Criteria (Out of Scope for V6)

- Production-ready error handling
- Mobile-responsive design
- Real authentication
- File upload for evidence
- Multiple templates
- Iteration management UI
- Real semantic oracle evaluation

---

## Appendix A: Workflow Diagrams

### User Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                        DASHBOARD                                 │
│  [+ New Work]                                                   │
│                                                                 │
│  Active Work Surfaces:                                          │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ "API Rate Limiting Research"    DRAFT (3/4)    [View]   │   │
│  │ "Auth System Design"            FRAME (1/4)    [View]   │   │
│  └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     NEW WORK WIZARD                              │
│                                                                 │
│  Step 1: Define Intake                                          │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ Title: [________________________]                        │   │
│  │ Kind:  [Research Memo ▼]                                │   │
│  │ Objective: [____________________________________]        │   │
│  │ Deliverables: [+Add]                                    │   │
│  │ Constraints: [+Add]                                     │   │
│  │ Completion Criteria: [+Add]                             │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                          [Cancel] [Next →]      │
│                                                                 │
│  Step 2: Choose Procedure Template                              │
│  Step 3: Confirm & Start                                        │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│              WORK SURFACE DETAIL VIEW                           │
│                                                                 │
│  "API Rate Limiting Research"                    Status: ACTIVE │
│                                                                 │
│  Progress: ████████░░░░░░░░ DRAFT (3/4)                        │
│                                                                 │
│  ┌──────────┬──────────┬──────────┬──────────┐                 │
│  │  FRAME   │ OPTIONS  │  DRAFT   │  FINAL   │                 │
│  │    ✓     │    ✓     │    ✓     │  🔒      │                 │
│  │ completed│ completed│ current  │ locked   │                 │
│  └──────────┴──────────┴──────────┴──────────┘                 │
│                                                                 │
│  Current Stage: DRAFT                                           │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ Purpose: Produce candidate deliverable                   │   │
│  │                                                          │   │
│  │ Evidence Bundle: [Upload/Reference]                      │   │
│  │                                                          │   │
│  │              [Complete Stage]                            │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│  FINAL Stage requires approval before completion                │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ [Approve] [Request Changes]                              │   │
│  │ Rationale: [________________________________]            │   │
│  └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

### API Call Sequence (New Work Wizard)

```
User clicks "Start Work"
         │
         ▼
    ┌─────────────────┐
    │ POST /intakes   │ ─── Create intake with form data
    └────────┬────────┘
             │ intake_id
             ▼
    ┌─────────────────────────────┐
    │ POST /intakes/{id}/activate │ ─── Activate intake
    └────────┬────────────────────┘
             │
             ▼
    ┌─────────────────────┐
    │ POST /work-surfaces │ ─── Bind intake + template
    └────────┬────────────┘
             │ work_surface_id
             ▼
    ┌─────────────────┐
    │ POST /loops     │ ─── Create loop for work unit
    └────────┬────────┘
             │ loop_id
             ▼
    ┌──────────────────────────┐
    │ POST /loops/{id}/activate│ ─── Activate loop
    └────────┬─────────────────┘
             │
             ▼
    ┌─────────────────────┐
    │ POST /iterations    │ ─── Start iteration (SYSTEM token)
    └────────┬────────────┘
             │
             ▼
    Navigate to WorkSurfaceDetail
```

---

## Appendix B: Files Reference

### API Implementation (for response shape reference)

| Handler File | Endpoints |
|-------------|-----------|
| `crates/sr-api/src/handlers/intakes.rs` | `/intakes/*` |
| `crates/sr-api/src/handlers/work_surfaces.rs` | `/work-surfaces/*` |
| `crates/sr-api/src/handlers/loops.rs` | `/loops/*` |
| `crates/sr-api/src/handlers/iterations.rs` | `/iterations/*` |
| `crates/sr-api/src/handlers/approvals.rs` | `/approvals/*` |

### E2E Test (workflow reference)

| File | Purpose |
|------|---------|
| `crates/sr-api/tests/integration/semantic_ralph_loop_e2e.rs` | Complete API workflow |

### Documentation

| File | Purpose |
|------|---------|
| `docs/platform/SR-SPEC.md` | Platform mechanics |
| `docs/platform/SR-WORK-SURFACE.md` | Work surface definitions |
| `docs/platform/SR-CONTRACT.md` | Binding invariants (C-TB-3 for approvals) |
| `docs/planning/SR-PLAN-V5.md` | Previous plan (format reference) |

# SR-PLAN-V7: MVP Stabilization & Evidence Foundation

**Status:** Ready for Implementation
**Created:** 2026-01-16
**Depends On:** SR-PLAN-V6 (UI Integration complete)
**Implements:** SR-CHARTER §Immediate Objective (Milestone 1: Semantic Work Unit Runtime)

---

## Executive Summary

SR-PLAN-V7 validates the completed MVP and adds the most architecturally significant extension: **Evidence Upload**. Evidence is foundational to SR-CONTRACT's verification model — currently stage completion accepts text only, but real semantic work requires file-based evidence.

**Goal:** Validate first, then extend thoughtfully.

```
V7-1 (Tests) → V7-2 (Error Handling) → V7-3 (Evidence Backend) → V7-4 (Evidence UI) → V7-5 (Iterations)
     └──────────────────────────────┘   └─────────────────────────────────────────┘   └─────────────────┘
              Stable MVP                        Evidence Foundation                    Full Workflow
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

### 1.1 What SR-PLAN-V6 Delivered

The MVP UI integration is complete:
- Intakes can be created and activated
- Work Surfaces can be created via wizard (auto-starts Loop/Iteration)
- All stages can be completed with approval gating
- Work Surfaces reach "completed" status

### 1.2 What's Missing

| Gap | Impact |
|-----|--------|
| No integration tests for `/start` endpoint | Regressions possible as we extend |
| Silent failures in UI | Users don't know when errors occur |
| Text-only evidence | Can't support real semantic oracle verification |
| Single iteration only | Can't retry or iterate on stuck work |

---

## 2. Design Rationale

### 2.1 Why Validate First (V7-1, V7-2)

The `/start` endpoint is a critical orchestration point. Before building more features on top of it, we need:
- **Integration tests** to catch regressions
- **Error handling** so users understand failures

### 2.2 Why Evidence Before Iterations (V7-3, V7-4 before V7-5)

Per SR-CONTRACT:
- **C-VER-1:** "A Candidate MAY be marked 'Verified' only when Evidence Bundle exists"
- **C-EVID-1:** Evidence Bundles have required manifest fields
- **C-EVID-2:** Evidence Bundles MUST be immutable, content-addressed

Evidence is foundational to the verification model. Iterations without proper evidence capture are less useful — you can retry, but you can't prove what happened.

### 2.3 Why Multiple Iterations Last (V7-5)

Multiple iterations are useful, but the workflow functions without them. Once evidence upload works, iteration history becomes more valuable because each iteration can be compared by its evidence.

---

## 3. Implementation Phases

### Phase V7-1: Integration Tests for `/start` Endpoint

**Objective:** Ensure the orchestration endpoint is regression-proof before extending.

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `crates/sr-api/tests/integration/work_surface_start_test.rs` | CREATE | Integration tests for `/start` |

**Test Cases:**

| Test | Description | Validates |
|------|-------------|-----------|
| `start_happy_path` | Active Work Surface → Loop created → activated → Iteration started | Core flow |
| `start_idempotent` | Call twice → second returns `already_started: true` | Idempotency (§3.8) |
| `start_rejects_inactive` | Non-active Work Surface → HTTP 412 | Precondition |
| `start_activates_created_loop` | Existing CREATED Loop → activates and starts | Edge case |
| `start_human_on_loop_created` | Verify `LoopCreated` has HUMAN actor | Audit trail |
| `start_system_on_iteration` | Verify `IterationStarted` has SYSTEM actor | C-CTX-1 compliance |

**Acceptance Criteria:**
- [ ] All 6 test cases pass
- [ ] `cargo test --package sr-api` passes
- [ ] Tests cover the acceptance criteria from SR-PLAN-V6 §4

**Effort:** ~1 session

---

### Phase V7-2: Error Handling & UX Feedback

**Objective:** Make failures visible to users.

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `ui/src/components/Toast.tsx` | CREATE | Toast notification component |
| `ui/src/components/ToastContext.tsx` | CREATE | Context provider for toast state |
| `ui/src/components/ApiErrorHandler.tsx` | CREATE | Translate API errors to user messages |
| `ui/src/pages/WorkSurfaceCompose.tsx` | MODIFY | Add loading states, error toasts, retry logic |
| `ui/src/App.tsx` | MODIFY | Wrap with ToastProvider |

**Toast Component:**
```typescript
interface Toast {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  message: string;
  duration?: number; // ms, default 5000
}
```

**API Error Mapping:**

| HTTP Status | Error Code | User Message |
|-------------|------------|--------------|
| 401 | — | "Session expired. Please sign in again." |
| 403 | — | "You don't have permission to perform this action." |
| 412 | `WORK_SURFACE_NOT_ACTIVE` | "Work Surface is not active. It may have been completed or archived." |
| 412 | `WORK_SURFACE_MISSING` | "Work Surface not found. It may have been deleted." |
| 500 | — | "Something went wrong. Please try again." |

**Loading States:**
- Spinner during "Create Work Surface" button click
- Disable button while request in flight
- Show progress: "Creating Work Surface..." → "Starting work..." → redirect

**Retry Logic:**
- On transient failure (5xx), retry up to 2 times with exponential backoff
- On permanent failure (4xx), show error toast, don't retry

**Acceptance Criteria:**
- [ ] Toast component renders success/error messages
- [ ] API errors show user-friendly messages
- [ ] Loading spinner shows during create + start sequence
- [ ] Retry logic handles transient failures
- [ ] `npm run type-check` passes
- [ ] `npm run build` passes

**Effort:** ~1 session

---

### Phase V7-3: Evidence Upload (Backend)

**Objective:** Enable file-based evidence upload to MinIO.

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `crates/sr-api/src/handlers/evidence.rs` | CREATE | Evidence upload handler |
| `crates/sr-api/src/main.rs` | MODIFY | Register evidence routes |

**New Endpoint:**

`POST /api/v1/evidence`
- **Content-Type:** `multipart/form-data`
- **Body:** `file` (the evidence file)
- **Response:**
```json
{
  "content_hash": "sha256:abc123...",
  "size_bytes": 12345,
  "media_type": "application/pdf",
  "stored_at": "2026-01-16T12:00:00Z"
}
```

**Implementation Requirements:**

Per SR-SPEC §1.9.2 (Evidence Immutability):
- Store in MinIO bucket `evidence-public`
- Object key: `sha256/{hash}` (content-addressed)
- Compute hash server-side (don't trust client)
- Prevent overwriting existing objects at same key

Per SR-CONTRACT C-EVID-2:
- Evidence Bundles MUST be immutable, content-addressed
- Protected against modification

**Handler Logic:**
```rust
pub async fn upload_evidence(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    mut multipart: Multipart,
) -> ApiResult<Json<EvidenceUploadResponse>> {
    // 1. Extract file from multipart
    // 2. Read into bytes, compute sha256
    // 3. Check if object already exists (idempotent)
    // 4. Store to MinIO with content-addressed key
    // 5. Return hash and metadata
}
```

**Acceptance Criteria:**
- [ ] `POST /evidence` accepts file upload
- [ ] Returns content hash (sha256)
- [ ] Stores in MinIO with content-addressed key
- [ ] Idempotent: re-upload same file returns same hash
- [ ] `cargo build --package sr-api` passes
- [ ] `cargo test --package sr-api` passes

**Effort:** ~1 session

---

### Phase V7-4: Evidence Upload (Frontend)

**Objective:** Wire the UI to use the evidence upload capability.

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `ui/src/components/EvidenceUploader.tsx` | CREATE | Drag-drop file upload component |
| `ui/src/components/EvidencePreview.tsx` | CREATE | Preview uploaded evidence |
| `ui/src/components/StageCompletionForm.tsx` | MODIFY | Add file upload option |

**EvidenceUploader Component:**
```typescript
interface EvidenceUploaderProps {
  onUploadComplete: (evidence: { content_hash: string; size_bytes: number; media_type: string }) => void;
  onError: (error: string) => void;
  accept?: string; // e.g., ".pdf,.md,.json"
  maxSizeMB?: number; // default 10
}
```

**Features:**
- Drag-drop zone with visual feedback
- Click to browse files
- Upload progress bar
- File type validation (client-side)
- Size validation (client-side)
- Show uploaded file info after success

**EvidencePreview Component:**
```typescript
interface EvidencePreviewProps {
  contentHash: string;
  mediaType: string;
  sizeBytes: number;
  onRemove?: () => void;
}
```

**Features:**
- Show file icon based on media type
- Display hash (truncated) and size
- "Remove" button to clear selection

**StageCompletionForm Changes:**

Current form has text input only. Add tabbed interface:
- **Tab 1: Notes** — existing text input
- **Tab 2: Upload Evidence** — new file upload

When submitting:
- If file uploaded: include `evidence_bundle_ref: content_hash` in request
- If text only: use existing behavior

**UX Flow:**
1. User clicks "Complete Stage"
2. Form shows tabs: "Notes" | "Upload Evidence"
3. User selects tab and fills in content
4. If Upload: file uploads to `/evidence`, progress shown
5. On success: preview shows, "Complete" button enabled
6. Submit stage completion with evidence ref

**Acceptance Criteria:**
- [ ] EvidenceUploader supports drag-drop and click-to-browse
- [ ] Upload progress bar shows during upload
- [ ] EvidencePreview shows uploaded file info
- [ ] StageCompletionForm has tabbed interface
- [ ] Stage completion can include evidence_bundle_ref
- [ ] `npm run type-check` passes
- [ ] `npm run build` passes

**Effort:** ~1-2 sessions

---

### Phase V7-5: Multiple Iteration Support

**Objective:** Enable viewing iteration history and starting new iterations.

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `crates/sr-api/src/handlers/work_surfaces.rs` | MODIFY | Add iteration endpoints |
| `crates/sr-api/src/main.rs` | MODIFY | Register new routes |
| `ui/src/components/IterationHistory.tsx` | CREATE | Iteration timeline component |
| `ui/src/pages/WorkSurfaceDetail.tsx` | MODIFY | Integrate iteration history |

**New Backend Endpoints:**

`GET /api/v1/work-surfaces/{id}/iterations`
- Returns iterations for the Work Surface's Loop
- Response:
```json
{
  "iterations": [
    {
      "iteration_id": "iter_...",
      "iteration_number": 1,
      "started_at": "2026-01-16T12:00:00Z",
      "completed_at": "2026-01-16T12:30:00Z",
      "status": "completed",
      "stage_id": "stage:FRAME"
    }
  ],
  "loop_id": "loop_...",
  "total": 1
}
```

`POST /api/v1/work-surfaces/{id}/iterations`
- Starts a new iteration (reuses SYSTEM mediation pattern from `/start`)
- Response: `{ iteration_id, iteration_number }`
- **Constraint:** Loop must be ACTIVE

**IterationHistory Component:**
```typescript
interface IterationHistoryProps {
  workSurfaceId: string;
  iterations: Iteration[];
  onStartNewIteration?: () => void;
  canStartNew: boolean; // false if current iteration still in progress
}
```

**Features:**
- Timeline view showing iterations
- Each iteration shows: number, stage, status, duration
- Expandable to show iteration details (artifacts, evidence)
- "New Iteration" button (when enabled)

**WorkSurfaceDetail Integration:**
- Add collapsible "Iteration History" section
- Show current iteration prominently
- "New Iteration" button available when:
  - Current iteration is completed, OR
  - Stop trigger fired (REPEATED_FAILURE, etc.)

**Acceptance Criteria:**
- [ ] `GET /work-surfaces/{id}/iterations` returns iteration list
- [ ] `POST /work-surfaces/{id}/iterations` starts new iteration as SYSTEM
- [ ] IterationHistory shows timeline of iterations
- [ ] "New Iteration" button starts new iteration
- [ ] `cargo test --package sr-api` passes
- [ ] `npm run type-check` passes
- [ ] `npm run build` passes

**Effort:** ~2 sessions

---

## 4. Success Criteria

### 4.1 Checkpoint: Stable MVP (after V7-2)

- [ ] Integration tests for `/start` endpoint pass
- [ ] Error handling shows user-friendly messages
- [ ] Loading states provide feedback during operations
- [ ] No regressions in existing MVP workflow

### 4.2 Checkpoint: Evidence Foundation (after V7-4)

- [ ] Evidence files can be uploaded via UI
- [ ] Uploaded evidence stored in MinIO (content-addressed)
- [ ] Stage completion can reference uploaded evidence
- [ ] Evidence is immutable and retrievable by hash

### 4.3 Checkpoint: Full Workflow (after V7-5)

- [ ] Iteration history visible on Work Surface detail
- [ ] New iterations can be started from UI
- [ ] Each iteration tracked with its evidence
- [ ] Complete workflow supports retry/iteration pattern

---

## 5. Reference Documents

### Platform Specifications

| Document | Relevant Sections |
|----------|-------------------|
| SR-CONTRACT | C-EVID-1..C-EVID-6 (Evidence integrity) |
| SR-SPEC | §1.9 (Evidence bundle model), §2.3.3 (Runs and evidence) |
| SR-WORK-SURFACE | §5.5 (Starting work via /start endpoint) |

### Prior Plans

| Plan | Status | Relevance |
|------|--------|-----------|
| SR-PLAN-V6 | Complete | `/start` endpoint implementation |
| SR-PLAN-V5 | Complete | E2E integration patterns |

### Codebase References

| File | Purpose |
|------|---------|
| `crates/sr-api/src/handlers/work_surfaces.rs` | Work Surface handlers (extend for iterations) |
| `crates/sr-api/src/handlers/evidence.rs` | Evidence handlers (to be created) |
| `crates/sr-adapters/src/evidence_store.rs` | MinIO evidence storage adapter |
| `ui/src/components/StageCompletionForm.tsx` | Stage completion (extend for evidence upload) |

---

## Appendix A: Effort Summary

| Phase | Focus | Effort | Cumulative |
|-------|-------|--------|------------|
| V7-1 | Integration tests | 1 session | 1 |
| V7-2 | Error handling & UX | 1 session | 2 |
| V7-3 | Evidence upload (backend) | 1 session | 3 |
| V7-4 | Evidence upload (frontend) | 1-2 sessions | 4-5 |
| V7-5 | Multiple iterations | 2 sessions | 6-7 |

**Total:** ~6-7 sessions for complete execution

---

## Appendix B: Dependency Graph

```
┌─────────────────────────────────────────────────────────────────┐
│                         SR-PLAN-V6                              │
│                    (UI Integration - Complete)                  │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│  V7-1: Integration Tests                                        │
│  - work_surface_start_test.rs                                   │
│  - 6 test cases covering /start endpoint                        │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│  V7-2: Error Handling & UX                                      │
│  - Toast.tsx, ToastContext.tsx                                  │
│  - ApiErrorHandler.tsx                                          │
│  - Loading states, retry logic                                  │
└─────────────────────────────────────────────────────────────────┘
                                │
                ┌───────────────┴───────────────┐
                │                               │
                ▼                               │
┌───────────────────────────────┐               │
│  V7-3: Evidence Backend       │               │
│  - POST /evidence endpoint    │               │
│  - MinIO storage              │               │
│  - Content-addressed keys     │               │
└───────────────────────────────┘               │
                │                               │
                ▼                               │
┌───────────────────────────────┐               │
│  V7-4: Evidence Frontend      │               │
│  - EvidenceUploader.tsx       │               │
│  - EvidencePreview.tsx        │               │
│  - StageCompletionForm mods   │               │
└───────────────────────────────┘               │
                │                               │
                └───────────────┬───────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│  V7-5: Multiple Iterations                                      │
│  - GET/POST /work-surfaces/{id}/iterations                      │
│  - IterationHistory.tsx                                         │
│  - WorkSurfaceDetail integration                                │
└─────────────────────────────────────────────────────────────────┘
```

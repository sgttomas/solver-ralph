# SR-PLAN-V7: MVP Stabilization & Attachment Foundation

**Status:** Ready for Implementation (Amended)
**Created:** 2026-01-16
**Amended:** 2026-01-16 (Ontological corrections per coherence review)
**Depends On:** SR-PLAN-V6 (UI Integration complete)
**Implements:** SR-CHARTER §Immediate Objective (Milestone 1: Semantic Work Unit Runtime)

---

## Executive Summary

SR-PLAN-V7 validates the completed MVP and adds the most architecturally significant extension: **Supporting Artifact Upload**. Human users need a simple way to upload supporting files (PDFs, documents) to attach to their work.

**Goal:** Validate first, then extend thoughtfully.

**Ontological Clarification (Amendment):** This plan distinguishes between:
- **Evidence Bundles** (`domain.evidence_bundle`): Oracle-generated semantic bundles with full manifests, run IDs, and verdicts. These satisfy C-EVID-1 requirements.
- **Attachments** (`record.attachment`): Human-uploaded supporting files that are content-addressed and immutable but are NOT oracle output and do NOT claim verification semantics.

This distinction preserves SR-CONTRACT's epistemological clarity: only oracle-produced Evidence Bundles can satisfy verification gates (C-VER-1); human attachments provide supporting context for audit and reference.

**Note:** The evidence backend infrastructure already exists (812 lines in `evidence.rs`, 440 lines in `minio.rs`). V7-3 adds a simple attachment upload endpoint for human users, reusing the content-addressed storage infrastructure.

```
V7-1 (Tests) → V7-2 (Error Handling) → V7-3 (Attachment Endpoint) → V7-4 (Attachment UI) → V7-5 (Iterations)
     └──────────────────────────────┘   └────────────────────────────────────────────────┘   └─────────────────┘
              Stable MVP                           Attachment Foundation                        Full Workflow
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
| No simple file upload for human attachments | Existing Evidence API requires full oracle manifests; users can't easily attach supporting files |
| Single iteration only | Can't retry or iterate on stuck work |

**Note:** The evidence backend exists (`POST /evidence` for oracle bundles, MinIO storage, content-addressing). What's missing is a simple attachment upload endpoint for human users to add supporting files.

---

## 2. Design Rationale

### 2.1 Why Validate First (V7-1, V7-2)

The `/start` endpoint is a critical orchestration point. Before building more features on top of it, we need:
- **Integration tests** to catch regressions
- **Error handling** so users understand failures

### 2.2 Why Attachments (not "Evidence Upload") (V7-3, V7-4)

**Ontological Distinction (Critical):**

Per SR-CONTRACT §2.3, an **Evidence Bundle** is defined as "Oracle output; non-authoritative verification artifacts (including semantic measurements)." Human-uploaded files are NOT oracle output — they are supporting documents.

Per SR-CONTRACT C-EVID-1, Evidence Bundles require:
- Candidate reference
- Oracle suite hash
- Governed artifact references
- Per-oracle results with PASS/FAIL verdicts

Human-uploaded files cannot satisfy these requirements because they are not produced by oracle runs.

**Solution:** Introduce `record.attachment` — a content-addressed, immutable artifact type for human-uploaded supporting files that:
- Shares storage infrastructure with Evidence Bundles (MinIO, content-addressing)
- Does NOT claim to be oracle output
- Does NOT satisfy verification gates (C-VER-1)
- CAN be referenced by Iterations and Candidates for audit/context
- Has a simpler manifest without oracle-specific fields

**Storage semantics preserved:**
- Per SR-SPEC §1.9.2: Content-addressed storage (`sha256/{hash}`)
- Per C-EVID-2: Immutable once stored (same guarantees apply to attachments)

### 2.3 Why Multiple Iterations Last (V7-5)

Multiple iterations are useful, but the workflow functions without them. Once attachment upload works, iteration history becomes more valuable because each iteration can reference its supporting artifacts.

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
- [x] All 6 test cases pass
- [x] `cargo test --package sr-api` passes
- [x] Tests cover the acceptance criteria from SR-PLAN-V6 §4

**Effort:** ~1 session (✅ Complete)

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
- [x] Toast component renders success/error messages
- [x] API errors show user-friendly messages
- [x] Loading spinner shows during create + start sequence
- [x] Retry logic handles transient failures
- [x] `npm run type-check` passes
- [x] `npm run build` passes

**Effort:** ~1 session (✅ Complete)

---

### Phase V7-3: Attachment Upload Endpoint (Backend)

**Objective:** Add a simple file upload endpoint for human-provided supporting files (attachments).

**Context:** The existing `POST /api/v1/evidence` endpoint (812 lines in `evidence.rs`) requires a full `EvidenceManifest` with oracle results, run IDs, and verdicts. This is designed for oracle-generated semantic bundles per C-EVID-1. For human users uploading supporting files (PDFs, documents), we need a separate endpoint that creates `record.attachment` artifacts.

**Ontological Note:** Attachments are NOT Evidence Bundles. They share storage infrastructure but have distinct semantics:
- Evidence Bundles → oracle output → can satisfy verification gates
- Attachments → human uploads → supporting context for audit/reference only

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `crates/sr-api/src/handlers/attachments.rs` | CREATE | Attachment upload handler |
| `crates/sr-api/src/main.rs` | MODIFY | Register new route |

**New Endpoint:**

`POST /api/v1/attachments`
- **Content-Type:** `multipart/form-data`
- **Body:** `file` field with the uploaded file
- **Response:**
```json
{
  "attachment_id": "attach_01J...",
  "content_hash": "sha256:abc123...",
  "size_bytes": 12345,
  "media_type": "application/pdf",
  "filename": "supporting-doc.pdf",
  "uploaded_by": "oidc_sub:...",
  "uploaded_at": "2026-01-16T12:00:00Z"
}
```

**Attachment Manifest Schema:**
```json
{
  "artifact_type": "record.attachment",
  "attachment_id": "attach_01J...",
  "content_hash": "sha256:...",
  "size_bytes": 12345,
  "media_type": "application/pdf",
  "filename": "original-filename.pdf",
  "uploaded_by": {
    "actor_kind": "HUMAN",
    "actor_id": "oidc_sub:..."
  },
  "uploaded_at": "2026-01-16T12:00:00Z"
}
```

**Implementation Requirements:**

Per SR-SPEC §1.9.2 (reusing Evidence storage infrastructure):
- Store in MinIO bucket `attachments` (new bucket, separate from `evidence-public`)
- Object key: `sha256/{hash}` (content-addressed)
- Compute hash server-side (don't trust client)
- Prevent overwriting existing objects at same key

Immutability guarantees (same as C-EVID-2 but for attachments):
- Attachments MUST be immutable once stored
- Protected against modification
- Content-addressed for integrity

**Handler Logic:**
```rust
pub async fn upload_attachment(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    mut multipart: Multipart,
) -> ApiResult<Json<AttachmentUploadResponse>> {
    // 1. Extract file from multipart
    // 2. Read into bytes, compute sha256
    // 3. Detect media type from file content/extension
    // 4. Generate attachment_id (attach_<ULID>)
    // 5. Create attachment manifest:
    //    - artifact_type: "record.attachment"
    //    - NO oracle fields (no run_id, no verdict, no results)
    // 6. Check if object already exists (idempotent)
    // 7. Store to MinIO with content-addressed key
    // 8. Emit AttachmentRecorded event
    // 9. Return attachment metadata
}
```

**Event Emission:**

For auditability per C-EVT-1, emit `AttachmentRecorded` event:
```json
{
  "event_type": "AttachmentRecorded",
  "stream_kind": "ATTACHMENT",
  "stream_id": "attach:{attachment_id}",
  "payload": {
    "attachment_id": "attach_01J...",
    "content_hash": "sha256:...",
    "media_type": "application/pdf",
    "size_bytes": 12345,
    "filename": "original-filename.pdf"
  },
  "actor_kind": "HUMAN",
  "actor_id": "oidc_sub:..."
}
```

**Why Separate from Evidence:**
- Existing `POST /evidence` is for oracle-generated semantic bundles (C-EVID-1 compliant)
- New `POST /attachments` is for human-uploaded supporting files
- Different `artifact_type` distinguishes them (`record.attachment` vs `evidence.gate_packet`)
- Preserves ontological clarity: only Evidence Bundles satisfy verification gates

**Acceptance Criteria:**
- [ ] `POST /attachments` accepts multipart file upload
- [ ] Returns attachment_id and content hash (sha256)
- [ ] Stores in MinIO with content-addressed key
- [ ] Idempotent: re-upload same file returns same hash
- [ ] Emits `AttachmentRecorded` event
- [ ] `cargo build --package sr-api` passes
- [ ] `cargo test --package sr-api` passes

**Effort:** ~1 session

---

### Phase V7-4: Attachment Upload (Frontend)

**Objective:** Wire the UI to use the attachment upload capability.

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `ui/src/components/AttachmentUploader.tsx` | CREATE | Drag-drop file upload component |
| `ui/src/components/AttachmentPreview.tsx` | CREATE | Preview uploaded attachment |
| `ui/src/components/StageCompletionForm.tsx` | MODIFY | Add attachment upload option |

**AttachmentUploader Component:**
```typescript
interface AttachmentUploaderProps {
  onUploadComplete: (attachment: {
    attachment_id: string;
    content_hash: string;
    size_bytes: number;
    media_type: string;
    filename: string;
  }) => void;
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

**AttachmentPreview Component:**
```typescript
interface AttachmentPreviewProps {
  attachmentId: string;
  contentHash: string;
  mediaType: string;
  sizeBytes: number;
  filename: string;
  onRemove?: () => void;
}
```

**Features:**
- Show file icon based on media type
- Display filename and size
- Show hash (truncated) for verification
- "Remove" button to clear selection

**StageCompletionForm Changes:**

Current form already has:
- Evidence bundle selector dropdown (`EvidenceBundleSelector.tsx`)
- Oracle result status selection
- Waiver references input

Add tabbed interface for artifact source:
- **Tab 1: Select Evidence Bundle** — current `EvidenceBundleSelector` dropdown (oracle-produced)
- **Tab 2: Upload Supporting File** — new `AttachmentUploader` component (human-uploaded)

**Semantic distinction in UI:**
- Tab 1 label: "Evidence Bundle (Oracle)" — for verification-grade artifacts
- Tab 2 label: "Supporting Attachment" — for human-uploaded context files

When submitting:
- If Evidence Bundle selected via Tab 1: use `evidence_bundle_ref` (existing behavior, satisfies C-VER-1)
- If Attachment uploaded via Tab 2: call `POST /attachments`, then include returned `attachment_id` as `attachment_refs[]` (supplementary, does NOT satisfy C-VER-1)

**UX Flow:**
1. User clicks "Complete Stage"
2. Form shows artifact source tabs: "Evidence Bundle (Oracle)" | "Supporting Attachment"
3. User selects artifact source:
   - Tab 1: Choose from dropdown of existing oracle-produced bundles
   - Tab 2: Drag-drop or browse to upload supporting file
4. If Upload: file uploads to `POST /attachments`, progress shown
5. On success: preview shows uploaded file info
6. User fills in gate result and optional waiver refs
7. Submit stage completion:
   - With `evidence_bundle_ref` if oracle evidence selected (for gate satisfaction)
   - With `attachment_refs[]` if supporting file uploaded (for audit context)

**Important UX Note:** The UI should make clear that:
- Evidence Bundles (Tab 1) are required for verification gates per C-VER-1
- Attachments (Tab 2) provide supporting context but do NOT satisfy verification requirements

**Acceptance Criteria:**
- [ ] AttachmentUploader supports drag-drop and click-to-browse
- [ ] Upload progress bar shows during upload
- [ ] AttachmentPreview shows uploaded file info
- [ ] StageCompletionForm has tabbed interface with clear semantic labels
- [ ] Stage completion can include `attachment_refs[]` for supporting files
- [ ] UI clearly distinguishes Evidence Bundles from Attachments
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
      "stage_id": "stage:FRAME",
      "attachment_refs": ["attach_01J..."]
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
- Expandable to show iteration details (evidence bundles, attachments)
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

### 4.2 Checkpoint: Attachment Foundation (after V7-4)

- [ ] Supporting files can be uploaded via UI as Attachments
- [ ] Uploaded attachments stored in MinIO (content-addressed)
- [ ] Stage completion can reference attachments for context
- [ ] Attachments are immutable and retrievable by hash
- [ ] UI clearly distinguishes Evidence Bundles (oracle) from Attachments (human)
- [ ] Ontological distinction preserved: only Evidence Bundles satisfy C-VER-1

### 4.3 Checkpoint: Full Workflow (after V7-5)

- [ ] Iteration history visible on Work Surface detail
- [ ] New iterations can be started from UI
- [ ] Each iteration tracked with its evidence and attachments
- [ ] Complete workflow supports retry/iteration pattern

---

## 5. Reference Documents

### Platform Specifications

| Document | Relevant Sections |
|----------|-------------------|
| SR-CONTRACT | C-EVID-1..C-EVID-6 (Evidence integrity), C-VER-1 (Verification requires Evidence) |
| SR-SPEC | §1.9 (Evidence bundle model), §2.3.3 (Runs and evidence) |
| SR-WORK-SURFACE | §5.5 (Starting work via /start endpoint) |
| SR-TYPES | §4 (Platform domain types — `record.attachment` to be added) |

### Prior Plans

| Plan | Status | Relevance |
|------|--------|-----------|
| SR-PLAN-V6 | Complete | `/start` endpoint implementation |
| SR-PLAN-V5 | Complete | E2E integration patterns |

### Codebase References

| File | Purpose |
|------|---------|
| `crates/sr-api/src/handlers/work_surfaces.rs` | Work Surface handlers (extend for iterations) |
| `crates/sr-api/src/handlers/evidence.rs` | Evidence handlers (reference for storage patterns) |
| `crates/sr-api/src/handlers/attachments.rs` | Attachment handlers (NEW in V7-3) |
| `crates/sr-adapters/src/minio.rs` | MinIO storage adapter (reuse for attachments) |
| `ui/src/components/StageCompletionForm.tsx` | Stage completion (extend for attachment upload) |
| `ui/src/components/EvidenceBundleSelector.tsx` | Existing bundle selector dropdown |

---

## Appendix A: Effort Summary

| Phase | Focus | Effort | Cumulative |
|-------|-------|--------|------------|
| V7-1 | Integration tests | 1 session | 1 |
| V7-2 | Error handling & UX | 1 session | 2 |
| V7-3 | Attachment upload (backend) | 1 session | 3 |
| V7-4 | Attachment upload (frontend) | 1-2 sessions | 4-5 |
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
│  V7-3: Attachment Endpoint    │               │
│  - POST /attachments          │               │
│  - record.attachment type     │               │
│  - Uses existing MinIO infra  │               │
│  - NOT Evidence Bundle        │               │
└───────────────────────────────┘               │
                │                               │
                ▼                               │
┌───────────────────────────────┐               │
│  V7-4: Attachment Frontend    │               │
│  - AttachmentUploader.tsx     │               │
│  - AttachmentPreview.tsx      │               │
│  - StageCompletionForm mods   │               │
│  - Clear Evidence/Attachment  │               │
│    distinction in UI          │               │
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

---

## Appendix C: Ontological Clarification (Amendment Record)

**Date:** 2026-01-16
**Reason:** Pre-implementation coherence review identified ontological gap

### Problem Statement

The original SR-PLAN-V7 proposed creating "Evidence Bundles" from human file uploads using `artifact_type: "evidence.human_upload"`. This conflated two ontologically distinct concepts:

1. **Evidence Bundles** (`domain.evidence_bundle`): Per SR-CONTRACT §2.3, these are "Oracle output; non-authoritative verification artifacts." They require full manifests per C-EVID-1 (candidate reference, oracle suite hash, governed artifact refs, per-oracle results).

2. **Human-uploaded files**: Supporting documents that are NOT oracle output and cannot satisfy C-EVID-1 requirements.

### Resolution

Introduce `record.attachment` as a distinct artifact type:
- Content-addressed and immutable (shares C-EVID-2 storage semantics)
- Does NOT claim to be Evidence Bundle
- Does NOT satisfy verification gates (C-VER-1)
- Simpler manifest without oracle-specific fields
- Can be referenced by Iterations and Candidates for audit context

### Contract Compliance

| Contract | Original Plan | Amended Plan |
|----------|---------------|--------------|
| C-EVID-1 | ❌ Not satisfied (no oracle fields) | ✅ N/A (attachments not Evidence Bundles) |
| C-EVID-2 | ✅ Satisfied | ✅ Satisfied (same storage semantics) |
| C-VER-1 | ⚠️ Confused (implied human files = verification) | ✅ Clear (only oracle Evidence satisfies verification) |

### Terminology Changes

| Original Term | Amended Term |
|---------------|--------------|
| `POST /evidence/files` | `POST /attachments` |
| `evidence.human_upload` | `record.attachment` |
| `evidence_bundle_ref` (for uploads) | `attachment_refs[]` |
| "Evidence Upload" | "Attachment Upload" |
| `EvidenceUploader.tsx` | `AttachmentUploader.tsx` |
| `EvidencePreview.tsx` | `AttachmentPreview.tsx` |

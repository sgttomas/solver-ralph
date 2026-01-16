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

### Feature Implementation Plans

The `docs/planning/` folder contains feature-specific implementation plans that are subordinate to SR-PLAN. These plans detail specific feature implementations and are not permanent governance documents — they become historical artifacts once implementation is complete.

| doc_id | Status | Purpose |
|--------|--------|---------|
| SR-PLAN-GAP-ANALYSIS | **living** | Deliverable status tracking & roadmap |
| SR-PLAN-V7 | **ready** | MVP Stabilization & Attachment Foundation |
| SR-PLAN-V6 | **complete** | UI Integration for MVP Workflow (V6-1, V6-2, V6-3 complete) |
| SR-PLAN-V5 | **complete** | Semantic Ralph Loop End-to-End Integration (Phases 5a-5d) |
| SR-PLAN-V4 | **complete** | Work Surface Composition (Phase 4) — All phases complete |
| SR-PLAN-V3 | **complete** | Intakes & References implementation (Phases 0-3) |
| SR-PLAN-V2 | superseded | Intakes & References draft (10 unresolved issues) |

### Roadmap (from Gap Analysis)

Per `SR-PLAN-GAP-ANALYSIS.md`, the path to Milestone 1 completion:

| Plan | Scope | Key Deliverables | Status |
|------|-------|------------------|--------|
| SR-PLAN-V7 | Stabilization & Attachments | Tests, UX, `record.attachment` | **Ready** |
| SR-PLAN-V8 | Oracle Runner & Semantic Suites | D-24, D-25, D-27, D-39 | Proposed |
| SR-PLAN-V9 | Semantic Worker & Branch 0 | D-23, D-41, D-36 | Proposed |

**Milestone 1 (MVP) projected completion:** After V9

---

## SR-PLAN-V7 Implementation Status

| Phase | Status | Description |
|-------|--------|-------------|
| Coherence Review | ✅ Complete | Ontological review completed, plan amended |
| V7-1: Integration Tests | ✅ Complete | Integration tests for `/start` endpoint |
| V7-2: Error Handling | ✅ Complete | Toast notifications, loading states, retry logic |
| V7-3: Attachment Backend | ✅ Complete | `POST /attachments` endpoint |
| V7-4: Attachment Frontend | ⏳ Pending | AttachmentUploader, AttachmentPreview components |
| V7-5: Multiple Iterations | ⏳ Pending | Iteration history and new iteration support |

---

## Previous Session Summary (V7 Coherence Review)

**Session Goal:** Evaluate SR-PLAN-V7 for ontological, epistemological, and semantic consistency with canonical SR-* documents before implementation.

### What Was Accomplished

1. **Read and analyzed canonical documents:**
   - SR-README (assignment and context)
   - SR-PLAN-V7 (original plan)
   - SR-SPEC (platform mechanics, especially §1.9 Evidence bundle model)
   - SR-SEMANTIC-ORACLE-SPEC (oracle interface requirements)
   - SR-CONTRACT (binding invariants, especially C-EVID-1, C-EVID-2, C-VER-1)

2. **Identified ontological gap in original SR-PLAN-V7:**
   - Original plan proposed `artifact_type: "evidence.human_upload"` for human file uploads
   - This conflated two ontologically distinct concepts:
     - **Evidence Bundles** (`domain.evidence_bundle`): Oracle output with full manifests per C-EVID-1
     - **Human uploads**: Supporting files that are NOT oracle output
   - Human-uploaded files cannot satisfy C-EVID-1 requirements (no candidate ref, no oracle suite hash, no per-oracle results)
   - Using "evidence" terminology for human uploads would violate C-VER-1 semantics

3. **Produced coherence review with verdict: COHERENT WITH NOTES**
   - Ontology: Gap identified (Evidence Bundle definition)
   - Epistemology: Consistent (no authority leakage)
   - Semantics: Minor naming clarification needed

4. **Amended SR-PLAN-V7 with ontological corrections:**
   - Introduced `record.attachment` as distinct artifact type
   - Changed endpoint from `POST /evidence/files` to `POST /attachments`
   - Changed `artifact_type` from `evidence.human_upload` to `record.attachment`
   - Renamed UI components: `EvidenceUploader.tsx` → `AttachmentUploader.tsx`
   - Added clear semantic distinction in UI between Evidence Bundles (oracle) and Attachments (human)
   - Added Appendix C documenting the amendment rationale

### Key Ontological Distinction (Amendment)

| Concept | Type Key | Source | Satisfies C-VER-1? |
|---------|----------|--------|-------------------|
| **Evidence Bundle** | `domain.evidence_bundle` | Oracle output | ✅ Yes |
| **Attachment** | `record.attachment` | Human upload | ❌ No |

This preserves SR-CONTRACT's epistemological clarity: only oracle-produced Evidence Bundles can satisfy verification gates.

### Files Modified

| File | Change |
|------|--------|
| `docs/planning/SR-PLAN-V7.md` | Comprehensive amendment with ontological corrections |

### Contract Compliance After Amendment

| Contract | Status |
|----------|--------|
| C-EVID-1 | ✅ N/A (attachments are not Evidence Bundles) |
| C-EVID-2 | ✅ Satisfied (same storage semantics for immutability) |
| C-VER-1 | ✅ Clear (only oracle Evidence satisfies verification) |
| C-CTX-1 | ✅ Satisfied (V7-5 iteration creation remains SYSTEM-mediated) |

---

## Previous Session Summary (V7-1 Integration Tests)

**Session Goal:** Implement SR-PLAN-V7 Phase V7-1 — Integration tests for `/start` endpoint.

### What Was Accomplished

1. **Explored codebase to understand test patterns:**
   - Examined existing `semantic_ralph_loop_e2e.rs` for test infrastructure patterns
   - Analyzed `/start` endpoint implementation in `work_surfaces.rs:1396-1484`
   - Verified actor mediation: LoopCreated uses HUMAN, IterationStarted uses SYSTEM

2. **Created integration test file:**
   - `crates/sr-api/tests/integration/work_surface_start_test.rs` (695 lines)
   - Registered in `crates/sr-api/Cargo.toml` as `[[test]]`

3. **Implemented all 6 test cases:**

   | Test | Description | Validates |
   |------|-------------|-----------|
   | `test_start_happy_path` | Active Work Surface → Loop → Iteration | Core flow |
   | `test_start_idempotent` | Second call returns `already_started: true` | Idempotency |
   | `test_start_rejects_inactive` | Archived Work Surface → HTTP 412 | Precondition |
   | `test_start_activates_created_loop` | Existing CREATED Loop → activates | Edge case |
   | `test_start_human_on_loop_created` | `created_by.kind == "HUMAN"` | Audit trail |
   | `test_start_system_on_iteration` | HUMAN `/start` succeeds, direct `POST /iterations` fails 403 | C-CTX-1 |

4. **All tests pass:** `cargo test --package sr-api` succeeds (36 unit + 6 integration tests)

### Files Modified

| File | Change |
|------|--------|
| `crates/sr-api/tests/integration/work_surface_start_test.rs` | Created — 6 test cases |
| `crates/sr-api/Cargo.toml` | Added `[[test]]` registration |
| `docs/planning/SR-PLAN-V7.md` | Checked V7-1 acceptance criteria |

### Commits

- (pending) V7-1 integration tests for `/start` endpoint

---

## Previous Session Summary (V7-2 Error Handling)

**Session Goal:** Implement SR-PLAN-V7 Phase V7-2 — Toast notifications, loading states, and retry logic.

### What Was Accomplished

1. **Created toast notification system:**
   - `Toast.tsx`: Component with auto-dismiss, slide animation, accessibility (role="alert" for errors)
   - `Toast.module.css`: Styles using design tokens from theme.css
   - `ToastContext.tsx`: Provider with `useToast` hook for app-wide access
   - `ApiErrorHandler.ts`: Error mapping + retry logic with exponential backoff

2. **Integrated into WorkSurfaceCompose page:**
   - Replaced console.warn/info with toast notifications
   - Added progress state: "Creating Work Surface..." → "Starting work..." → redirect
   - Used `fetchWithRetry` for transient failure handling (5xx retries)
   - Show success toast on completion, warning if start fails, error on create failure

3. **Wrapped app with ToastProvider:**
   - Updated `main.tsx` to wrap RouterProvider with ToastProvider
   - Exported new components from `components/index.ts`

4. **All checks pass:** `npm run type-check` and `npm run build` succeed

### Files Modified

| File | Change |
|------|--------|
| `ui/src/components/Toast.tsx` | Created — Toast notification component |
| `ui/src/components/Toast.module.css` | Created — Toast styles |
| `ui/src/components/ToastContext.tsx` | Created — Provider + useToast hook |
| `ui/src/components/ApiErrorHandler.ts` | Created — Error mapping + retry logic |
| `ui/src/pages/WorkSurfaceCompose.tsx` | Modified — Toast integration, progress states |
| `ui/src/main.tsx` | Modified — ToastProvider wrapper |
| `ui/src/components/index.ts` | Modified — New exports |

### Commits

- `bb9b910` feat(ui): add toast notifications and error handling (V7-2)
- `097324f` docs: mark V7-2 complete, update next instance prompt for V7-3

### Documents Updated

| Document | Update |
|----------|--------|
| `docs/planning/SR-PLAN-V7.md` | V7-2 acceptance criteria marked complete |
| `docs/planning/SR-PLAN-GAP-ANALYSIS.md` | V7 roadmap status updated |
| `docs/charter/SR-README.md` | V7-2 session summary, V7-3 next prompt |

---

## Previous Session Summary (V7-3 Attachment Backend)

**Session Goal:** Implement SR-PLAN-V7 Phase V7-3 — Attachment Upload Endpoint (Backend)

### What Was Accomplished

1. **Implemented attachment upload endpoint and supporting infrastructure:**

   | Component | File | Purpose |
   |-----------|------|---------|
   | `MinioAttachmentStore` | `crates/sr-adapters/src/attachment_store.rs` | Content-addressed storage in `attachments` bucket |
   | `upload_attachment` handler | `crates/sr-api/src/handlers/attachments.rs` | `POST /api/v1/attachments` multipart handler |
   | `StreamKind::Attachment` | `crates/sr-domain/src/events.rs` | New stream kind for attachment events |

2. **Key implementation details:**
   - Accepts `multipart/form-data` with `file` field
   - Computes SHA-256 hash server-side for content addressing
   - Stores in MinIO `attachments` bucket at `sha256/{hash}`
   - Idempotent: re-uploading same content returns same hash without re-writing
   - Emits `AttachmentRecorded` event per C-EVT-1 for audit trail
   - Auto-detects media type from filename extension

3. **Updated canonical documents:**
   - **SR-SPEC §1.5.2:** Added `ATTACHMENT` to StreamKind enum
   - **SR-SPEC Appendix A:** Added `AttachmentRecorded` event with payload schema
   - **SR-TYPES §4.4:** Added `record.attachment` type
   - **SR-CONTRACT §7 (C-EVID-2):** Added clarification distinguishing attachments from evidence bundles
   - **SR-CHANGE §0.6:** Documented all canonical changes (G:MINOR classification)

4. **All checks pass:**
   - `cargo build --package sr-api` ✅
   - `cargo test --package sr-api` ✅ (41 tests)
   - `cargo test --package sr-adapters attachment_store` ✅ (3 tests)

### Files Created

| File | Lines | Purpose |
|------|-------|---------|
| `crates/sr-adapters/src/attachment_store.rs` | ~270 | MinIO attachment store adapter |
| `crates/sr-api/src/handlers/attachments.rs` | ~250 | Upload handler with multipart processing |

### Files Modified

| File | Change |
|------|--------|
| `crates/sr-api/Cargo.toml` | Added multipart feature, ulid dependency |
| `crates/sr-domain/src/events.rs` | Added `StreamKind::Attachment` |
| `crates/sr-adapters/src/lib.rs` | Registered and exported attachment_store module |
| `crates/sr-adapters/src/postgres.rs` | Added Attachment handling in stream_kind functions |
| `crates/sr-api/src/handlers/mod.rs` | Registered attachments module |
| `crates/sr-api/src/main.rs` | Added route, state initialization, imports |
| `docs/platform/SR-SPEC.md` | StreamKind enum + AttachmentRecorded event |
| `docs/platform/SR-TYPES.md` | Added `record.attachment` type |
| `docs/platform/SR-CONTRACT.md` | Added C-EVID-2 clarification |
| `docs/build-governance/SR-CHANGE.md` | Version 0.6 change log entry |

### Ontological Clarity Preserved

| Concept | Type Key | Storage | Satisfies C-VER-1? |
|---------|----------|---------|-------------------|
| Evidence Bundle | `domain.evidence_bundle` | `evidence` bucket | ✅ Yes |
| Attachment | `record.attachment` | `attachments` bucket | ❌ No |

---

## Next Instance Prompt: Execute SR-PLAN-V7 Phase V7-4

### Context

V7-3 is complete. The backend now supports attachment uploads:
- `POST /api/v1/attachments` accepts multipart file uploads
- Files stored content-addressed in MinIO `attachments` bucket
- Returns `attachment_id`, `content_hash`, `size_bytes`, `media_type`, `filename`, `uploaded_by`, `uploaded_at`
- `AttachmentRecorded` event emitted for audit trail

The API is ready for UI integration. Next: add attachment upload UI components.

### Current State

- Branch: `solver-ralph-7`
- SR-PLAN-V7 Phase V7-1: **Complete** (Integration tests)
- SR-PLAN-V7 Phase V7-2: **Complete** (Error handling)
- SR-PLAN-V7 Phase V7-3: **Complete** (Attachment backend)
- SR-PLAN-V7 Phase V7-4: **Pending** (next)

### Assignment

**Execute SR-PLAN-V7 Phase V7-4: Attachment Frontend**

Add UI components for uploading and displaying attachments on Work Surfaces.

### Key Requirements (from SR-PLAN-V7 §V7-4)

**Ontological Note:** Attachments are NOT Evidence Bundles in the UI:
- Evidence = oracle-produced, verification authority
- Attachments = human-uploaded supporting files, no verification authority
- UI must maintain this distinction visually and semantically

**Components to create:**
1. `AttachmentUploader.tsx` — Drag-and-drop file upload component
2. `AttachmentPreview.tsx` — Display uploaded attachment metadata
3. `AttachmentList.tsx` — List attachments for a Work Surface (if needed)

**Integration points:**
- Add to `WorkSurfaceDetail.tsx` page
- Use existing toast system for success/error feedback
- Handle upload progress states

**API integration:**
```typescript
// Upload
POST /api/v1/attachments
Content-Type: multipart/form-data
Body: file field

// Response
{
  attachment_id: string,
  content_hash: string,
  size_bytes: number,
  media_type: string,
  filename: string,
  uploaded_by: string,
  uploaded_at: string
}
```

### Acceptance Criteria

- [ ] `AttachmentUploader` component with drag-and-drop support
- [ ] `AttachmentPreview` displays attachment metadata (filename, size, type)
- [ ] Upload progress indicator during file transfer
- [ ] Success/error toast notifications
- [ ] Attachments displayed on Work Surface detail page
- [ ] Visual distinction from Evidence Bundles (different styling/section)
- [ ] `npm run type-check` passes
- [ ] `npm run build` passes

### First Action

1. Read SR-PLAN-V7 §V7-4 for full requirements
2. Examine `ui/src/pages/WorkSurfaceDetail.tsx` for integration point
3. Review existing component patterns in `ui/src/components/`
4. Create `AttachmentUploader.tsx` with drag-and-drop


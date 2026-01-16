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
| V7-3: Attachment Backend | ⏳ Pending | `POST /attachments` endpoint |
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

---

## Next Instance Prompt: Execute SR-PLAN-V7 Phase V7-3

### Context

V7-2 is complete. The UI now has:
- Toast notification system for success/error/warning/info messages
- User-friendly error messages mapped from HTTP status codes
- Retry logic for transient failures (5xx with exponential backoff)
- Progress states during Work Surface creation

The UX is now resilient and informative. Next: add attachment upload capability.

### Current State

- Branch: `solver-ralph-7`
- SR-PLAN-V7 Phase V7-1: **Complete** (Integration tests)
- SR-PLAN-V7 Phase V7-2: **Complete** (Error handling)
- SR-PLAN-V7 Phase V7-3: **Pending** (next)

### Assignment

**Execute SR-PLAN-V7 Phase V7-3: Attachment Upload Endpoint (Backend)**

Add `POST /api/v1/attachments` endpoint for human-uploaded supporting files.

### Key Requirements (from SR-PLAN-V7 §V7-3)

**Ontological Note:** Attachments are NOT Evidence Bundles. They:
- Share storage infrastructure (MinIO, content-addressing)
- Do NOT claim to be oracle output
- Do NOT satisfy verification gates (C-VER-1)
- Have `artifact_type: "record.attachment"`

**Endpoint:** `POST /api/v1/attachments`
- Content-Type: `multipart/form-data`
- Body: `file` field with uploaded file
- Response: `{ attachment_id, content_hash, size_bytes, media_type, filename, uploaded_by, uploaded_at }`

**Storage:**
- MinIO bucket: `attachments` (new bucket, separate from `evidence-public`)
- Object key: `sha256/{hash}` (content-addressed)
- Compute hash server-side
- Prevent overwriting existing objects

**Event:** Emit `AttachmentRecorded` event for auditability per C-EVT-1

### Acceptance Criteria

- [ ] `POST /attachments` accepts multipart file upload
- [ ] Returns attachment_id and content hash (sha256)
- [ ] Stores in MinIO with content-addressed key
- [ ] Idempotent: re-upload same file returns same hash
- [ ] Emits `AttachmentRecorded` event
- [ ] `cargo build --package sr-api` passes
- [ ] `cargo test --package sr-api` passes

### First Action

1. Read SR-PLAN-V7 §V7-3 for full requirements
2. Examine `crates/sr-api/src/handlers/evidence.rs` for storage patterns
3. Examine `crates/sr-adapters/src/minio.rs` for MinIO adapter
4. Create `attachments.rs` handler


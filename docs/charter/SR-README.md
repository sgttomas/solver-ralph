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
| SR-PLAN-V7 | **complete** | MVP Stabilization & Attachment Foundation |
| SR-PLAN-V6 | **complete** | UI Integration for MVP Workflow (V6-1, V6-2, V6-3 complete) |
| SR-PLAN-V5 | **complete** | Semantic Ralph Loop End-to-End Integration (Phases 5a-5d) |
| SR-PLAN-V4 | **complete** | Work Surface Composition (Phase 4) — All phases complete |
| SR-PLAN-V3 | **complete** | Intakes & References implementation (Phases 0-3) |
| SR-PLAN-V2 | superseded | Intakes & References draft (10 unresolved issues) |

### Roadmap (from Gap Analysis)

Per `SR-PLAN-GAP-ANALYSIS.md`, the path to Milestone 1 completion:

| Plan | Scope | Key Deliverables | Status |
|------|-------|------------------|--------|
| SR-PLAN-V7 | Stabilization & Attachments | Tests, UX, `record.attachment` | **Complete** |
| SR-PLAN-V8 | Oracle Runner & Semantic Suites | D-24, D-25, D-27, D-39 | Not yet authored |
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
| V7-4: Attachment Frontend | ✅ Complete | AttachmentUploader, AttachmentPreview, tabbed StageCompletionForm |
| V7-5: Multiple Iterations | ✅ Complete | Iteration history and new iteration support |

**SR-PLAN-V7 is now complete.** All MVP stabilization and attachment foundation work is done.

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

## Previous Session Summary (V7-4 Attachment Frontend)

**Session Goal:** Implement SR-PLAN-V7 Phase V7-4 — Attachment upload UI components.

### What Was Accomplished

1. **Created attachment upload components:**

   | Component | File | Purpose |
   |-----------|------|---------|
   | `AttachmentUploader` | `ui/src/components/AttachmentUploader.tsx` | Drag-drop upload with progress tracking |
   | `AttachmentPreview` | `ui/src/components/AttachmentPreview.tsx` | Display uploaded file metadata |
   | Styles | `ui/src/components/AttachmentUploader.module.css` | Dropzone, progress bar, preview styles |

2. **Modified StageCompletionForm with tabbed interface:**
   - Tab 1: "Evidence Bundle (Oracle)" — existing `EvidenceBundleSelector`
   - Tab 2: "Supporting Attachment" — new `AttachmentUploader`
   - Updated validation to require either evidence OR attachment
   - Updated submit payload to include `attachment_refs[]` when attachment uploaded

3. **Key features implemented:**
   - Drag-and-drop with visual feedback (border highlight on dragover)
   - Click-to-browse fallback
   - Client-side validation (file size, accepted types)
   - Upload progress bar using XMLHttpRequest
   - AttachmentPreview shows file icon, name, size, truncated hash
   - Toast notifications for success/error feedback
   - Semantic note explaining attachments don't satisfy C-VER-1

4. **All checks pass:** `npm run type-check` and `npm run build` succeed

### Files Created

| File | Lines | Purpose |
|------|-------|---------|
| `ui/src/components/AttachmentUploader.tsx` | ~250 | Drag-drop upload component |
| `ui/src/components/AttachmentUploader.module.css` | ~150 | Uploader and preview styles |
| `ui/src/components/AttachmentPreview.tsx` | ~130 | Attachment metadata display |

### Files Modified

| File | Change |
|------|--------|
| `ui/src/components/StageCompletionForm.tsx` | Added tabbed artifact source interface |
| `ui/src/components/index.ts` | Exported new components |

### Ontological Distinction Preserved

| Concept | UI Label | Source | Satisfies C-VER-1? |
|---------|----------|--------|-------------------|
| Evidence Bundle | "Evidence Bundle (Oracle)" | Oracle output | ✅ Yes |
| Attachment | "Supporting Attachment" | Human upload | ❌ No |

---

## Previous Session Summary (V7-5 Multiple Iteration Support)

**Session Goal:** Implement SR-PLAN-V7 Phase V7-5 — Multiple iteration support.

### What Was Accomplished

1. **Implemented backend iteration endpoints:**

   | Endpoint | Purpose |
   |----------|---------|
   | `GET /api/v1/work-surfaces/{id}/iterations` | Returns iteration list for Work Surface's Loop |
   | `POST /api/v1/work-surfaces/{id}/iterations` | Starts new iteration (SYSTEM-mediated per C-CTX-1) |

2. **Backend implementation details:**
   - Reused existing `start_iteration_as_system()` helper from `/start` endpoint
   - Added `WorkSurfaceIterationsResponse` and `IterationSummary` types
   - Validation: Loop must be ACTIVE, current iteration must be COMPLETED or FAILED
   - Returns HTTP 412 Precondition Failed for invalid states

3. **Created frontend iteration history component:**

   | File | Purpose |
   |------|---------|
   | `ui/src/components/IterationHistory.tsx` | Timeline view with expandable iteration cards |
   | `ui/src/components/IterationHistory.module.css` | Timeline styling with animated active dot |

4. **Integrated into WorkSurfaceDetail.tsx:**
   - Added iteration fetching with `fetchIterations()` callback
   - Added "Iteration History" card with IterationHistory component
   - Wired "New Iteration" button with toast feedback on success/error
   - Button disabled when current iteration still in progress

5. **All checks pass:**
   - `cargo test --package sr-api` ✅ (41 tests)
   - `npm run type-check` ✅
   - `npm run build` ✅

### Files Created

| File | Lines | Purpose |
|------|-------|---------|
| `ui/src/components/IterationHistory.tsx` | ~230 | Timeline component with expandable details |
| `ui/src/components/IterationHistory.module.css` | ~180 | Timeline styling |

### Files Modified

| File | Change |
|------|--------|
| `crates/sr-api/src/handlers/work_surfaces.rs` | Added iteration list/create endpoints (+220 lines) |
| `crates/sr-api/src/main.rs` | Registered `/iterations` routes |
| `ui/src/pages/WorkSurfaceDetail.tsx` | Integrated iteration history (+104 lines) |
| `ui/src/components/index.ts` | Exported new component |

### Commits

- `01742c0` SR-PLAN-V7 Phase V7-5: Multiple iteration support
- `927a941` docs: mark V7-5 complete in SR-PLAN-V7 and SR-PLAN-GAP-ANALYSIS

### SR-PLAN-V7 Complete

With V7-5 done, SR-PLAN-V7 is fully complete:
- V7-1: Integration tests ✅
- V7-2: Error handling ✅
- V7-3: Attachment backend ✅
- V7-4: Attachment frontend ✅
- V7-5: Multiple iterations ✅

---

## SR-PLAN-V8 Implementation Status

| Phase | Status | Description |
|-------|--------|-------------|
| V8-1: Oracle Suite Registry | ⏳ Pending | PostgreSQL storage, suite management API |
| V8-2: API Integration | ⏳ Pending | Wire PodmanOracleRunner to `/runs` endpoint |
| V8-3: Integrity Checks | ⏳ Pending | TAMPER/GAP/FLAKE/ENV_MISMATCH detection |
| V8-4: Core Oracle Suite | ⏳ Pending | Build/unit/schema/lint oracles + container |
| V8-5: Semantic Oracles | ⏳ Pending | sr.semantic_eval.v1, residual/coverage artifacts |

**SR-PLAN-V8 is now authored and ready for implementation.**

---

## Previous Session Summary (V8 Authoring)

**Session Goal:** Author SR-PLAN-V8 — Oracle Runner & Semantic Suite Foundation

### What Was Accomplished

1. **Read and analyzed canonical documents:**
   - SR-README (assignment context)
   - SR-PLAN-GAP-ANALYSIS (deliverable status, D-24/D-25/D-27/D-39 gaps)
   - SR-SEMANTIC-ORACLE-SPEC (oracle interface requirements, sr.semantic_eval.v1 schema)
   - SR-CONTRACT §6 (C-OR-1..7 oracle integrity), §7 (C-EVID-1..6 evidence)
   - Existing `oracle_runner.rs` (~1027 lines of partial implementation)

2. **Analyzed existing infrastructure:**
   - `PodmanOracleRunner<E>` already implemented with Podman command building
   - Evidence manifest builder exists
   - Test mode for mock execution exists
   - Gaps identified: suite registry (in-memory only), candidate path (placeholder), integrity detection (missing)

3. **Authored SR-PLAN-V8 with 5 phases:**

   | Phase | Focus | Deliverables |
   |-------|-------|--------------|
   | V8-1 | Oracle Suite Registry | DB schema, API endpoints, port trait |
   | V8-2 | API Integration | Wire runner to `/runs`, candidate workspace |
   | V8-3 | Integrity Checks | TAMPER/GAP/FLAKE/ENV_MISMATCH detection |
   | V8-4 | Core Oracle Suite | Container image, 4 oracles, suite.json |
   | V8-5 | Semantic Oracles | sr.semantic_eval.v1, residual/coverage/violations |

4. **Included contract compliance matrix:**
   - C-OR-1 through C-OR-7 mapped to implementation phases
   - C-EVID-1, C-EVID-2 addressed via existing infrastructure
   - C-VER-1 satisfied via semantic oracle evidence

### Files Created

| File | Lines | Purpose |
|------|-------|---------|
| `docs/planning/SR-PLAN-V8.md` | ~700 | Oracle Runner & Semantic Suite Foundation plan |

### Key Architectural Decisions

| Decision | Rationale |
|----------|-----------|
| Registry before API | C-OR-2 requires suite pinning; need persistent storage first |
| Integrity as separate phase | C-OR-7 requires all conditions halt and escalate; deserves focused implementation |
| Core oracles before semantic | Validate packaging/execution before adding semantic complexity |
| 5 phases (~7-9 sessions total) | Each phase completable in 1-2 sessions; clear acceptance criteria |

### Estimated Effort

| Phase | Sessions |
|-------|----------|
| V8-1 | 1 |
| V8-2 | 1-2 |
| V8-3 | 1-2 |
| V8-4 | 2 |
| V8-5 | 2 |
| **Total** | **7-9** |

---

## Next Instance Prompt: Coherence Review of SR-PLAN-V8

### Context

SR-PLAN-V8 has been authored but not yet validated against the actual codebase. Before implementation begins, we need to verify that the plan's assumptions about existing infrastructure, type definitions, and integration points are accurate.

### Current State

- Branch: `solver-ralph-8` (V7 complete, V8 plan authored)
- SR-PLAN-V7: **Complete** (all phases)
- SR-PLAN-V8: **Authored** (pending coherence review)

### Assignment

**Evaluate SR-PLAN-V8 for coherence with the existing codebase**

The plan makes several assumptions about what exists and how components fit together. Your task is to verify these assumptions by examining the actual code and identifying any gaps, conflicts, or necessary adjustments.

### Key Questions to Answer

1. **Oracle Runner Integration:**
   - Does `oracle_runner.rs` actually implement the `OracleRunner` trait from `sr-ports`?
   - What is the current state of the `/runs` endpoint in `runs.rs`? Does it already call the oracle runner, or is it a stub?
   - How does the existing `EvidenceManifestBuilder` work, and does V8-2's proposed integration align with it?

2. **Type Consistency:**
   - Do the proposed domain types (`OracleSuite`, `OracleSuiteStatus`) conflict with or duplicate anything in `sr-domain`?
   - Are there existing types in `oracle_runner.rs` (like `OracleSuiteDefinition`) that should be reused vs. replaced?

3. **Database Patterns:**
   - What patterns do existing migrations follow? (naming, structure, conventions)
   - How do other PostgreSQL adapters implement their repository traits?

4. **API Patterns:**
   - How are other handlers structured in `sr-api`? What's the standard for request/response types?
   - What's the route registration pattern in `main.rs`?

5. **Evidence Store Integration:**
   - How does the existing `MinioEvidenceStore` work with `PodmanOracleRunner`?
   - Is there a mismatch between V8's proposed flow and what's already wired?

### Deliverable

Produce a coherence assessment with one of these verdicts:
- **COHERENT**: Plan aligns with codebase; proceed to implementation
- **COHERENT WITH AMENDMENTS**: Plan mostly aligns but needs specific corrections (list them)
- **REQUIRES REVISION**: Significant misalignment; plan needs rewriting before implementation

### First Actions

1. Read `crates/sr-ports/src/lib.rs` to understand the port trait definitions
2. Read `crates/sr-api/src/handlers/runs.rs` to understand current `/runs` implementation
3. Read `crates/sr-adapters/src/evidence.rs` to understand evidence manifest building
4. Examine existing migrations in `migrations/` for naming conventions
5. Compare V8's proposed types against existing types in `sr-domain` and `oracle_runner.rs`


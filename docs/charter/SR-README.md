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

Tasks are no longer assigned by SR-PLAN because the build out phase is complete.  See below for the Comprehensive Implementation Plan for the next phase of build out and implementation.

Begin your task assignment by reading SR-CHARTER.  The project documentation constitutes a total development plan and specification with detailed instructions on types and contracts.  Always read the SR-* files that appear related to the task before going to read the code files.  Documentation leads development for this project.  Documentation is how you know your ontology, epistemology, and semantics.

Once you have read the appropriate project docs, then do whatever deliverables and tasks you think should be done next.

Your task is done when there are no more deliverables to be assigned, which means all tests have passed and gates cleared.

You should push on to resolve any findings and consult the docs/ frequently for guidance and direction from the SR-* documents, several of which are normative.  They are typed documents and structured to aid in navigation.

You should git add && commit && push after completing each deliverable.  You can remain on the same branch throughout this development session.

If you cannot pass the tests for that deliverable then you must summarize what you did during that development session, delete the previous message where it says "Development History Summary for this Deliveralbe" and then append your new message including how to identify the task that was being worked on when the next instance of yourself begins the next iteration.

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
| SR-PLAN-V4 | **pending review** | Work Surface Composition (Phase 4) — awaiting coherence review |
| SR-PLAN-V3 | **complete** | Intakes & References implementation (Phases 0-3) |
| SR-PLAN-V2 | superseded | Intakes & References draft (10 unresolved issues) |

---

## SR-PLAN-V3 Implementation Status

**Status: COMPLETE**

All phases of the Intakes & References implementation (SR-PLAN-V3) are now complete.

| Phase | Status | Description |
|-------|--------|-------------|
| Phase 0a | **Complete** | Core Infrastructure — TypedRef module, Intake domain, database migrations, event definitions |
| Phase 0b | **Complete** | Intake API — Intake handler with CRUD + lifecycle operations |
| Phase 0c | **Complete** | References API — References browser backend (15 endpoints) |
| Phase 1 | **Complete** | UI Structure — Sidebar and route reorganization |
| Phase 2 | **Complete** | Intakes UI — Full Intake CRUD UI |
| Phase 3 | **Complete** | References UI — References browser UI |

### Current Routes

```
/intakes                              → Intakes.tsx (list with filters)
/intakes/new                          → IntakeCreate.tsx (create form)
/intakes/:intakeId                    → IntakeDetail.tsx (detail view)
/intakes/:intakeId/edit               → IntakeEdit.tsx (edit form)
/references                           → References.tsx (category sidebar)
/references/documents/:documentId     → ReferenceDocumentDetail.tsx
/references/bundles/:bundleId         → ReferenceBundleDetail.tsx
/references/governed-artifacts/:id    → GovernedArtifactDetail.tsx
```

---

## SR-PLAN-V4 Implementation Status

**Status: READY FOR IMPLEMENTATION**

SR-PLAN-V4 (Work Surface Composition) has passed coherence review. The review report is at `docs/reviews/SR-PLAN-V4-COHERENCE-REVIEW.md`. The plan is at `docs/planning/SR-PLAN-V4.md`.

**Coherence Review Summary:** The plan is coherent with canonical SR-* specifications. Three minor issues were identified (status enum naming, RefKind::Stage consideration, SQL type naming) — all can be addressed during implementation without plan revision.

### Phase Overview

| Phase | Status | Description |
|-------|--------|-------------|
| Phase 4a | **Complete** | Core Infrastructure — WorkSurfaceId, events, database migrations |
| Phase 4b | Pending | Work Surface API — 9 endpoints for CRUD, stage transitions, compatibility |
| Phase 4c | Pending | Event Integration — IterationStarted refs, EvidenceBundleRecorded binding |
| Phase 4d | Pending | Work Surface UI — List, composition wizard, detail with stage progress |

### Key Design Decisions (Resolved in SR-PLAN-V4)

| Question | Resolution |
|----------|------------|
| Compatibility checking | Intake `kind` must match Procedure Template's `kind[]`; binding fails otherwise |
| Stage initialization | New Work Surface starts at template's initial stage; `StageEntered` emitted immediately |
| Oracle suite binding | Per-stage binding (not once for whole Work Surface); resolved dynamically on stage entry |
| Immutability | Binding refs immutable; only stage progression mutable via events |
| Relationship to Loops | Work Surface 1:1 with Work Unit (not Loop); multiple Loops share same Work Surface |
| UI workflow | Step-by-step wizard (Select Intake → Select Template → Review & Confirm) |

### Planned Routes

```
/work-surfaces                        → WorkSurfaces.tsx (list with filters)
/work-surfaces/new                    → WorkSurfaceCompose.tsx (composition wizard)
/work-surfaces/:workSurfaceId         → WorkSurfaceDetail.tsx (detail with stage progress)
```

---

## Prompt for Next Instance: Implement SR-PLAN-V4 Phase 4b (Work Surface API)

**Phase 4a Status:** COMPLETE (2026-01-15) — Core infrastructure implemented

### Task

Implement **Phase 4b: Work Surface API** from SR-PLAN-V4.

### Required Reading

1. `docs/planning/SR-PLAN-V4.md` — Full implementation plan (especially §3 API Specification)
2. `docs/platform/SR-WORK-SURFACE.md` — Primary specification (§5 Work Surface Instance)
3. `crates/sr-domain/src/work_surface.rs` — Domain model with `ManagedWorkSurface`, events, lifecycle methods
4. `crates/sr-api/src/handlers/intakes.rs` — Reference pattern for handler implementation

### Deliverables

1. **Work Surface Handler** (`crates/sr-api/src/handlers/work_surfaces.rs`)
   - `POST /api/v1/work-surfaces` — Create/bind a new Work Surface
   - `GET /api/v1/work-surfaces` — List Work Surfaces (with filters: status, work_unit_id, intake_id)
   - `GET /api/v1/work-surfaces/:id` — Get Work Surface detail
   - `POST /api/v1/work-surfaces/:id/enter-stage` — Enter a new stage
   - `POST /api/v1/work-surfaces/:id/complete-stage` — Complete current stage (with evidence)
   - `POST /api/v1/work-surfaces/:id/complete` — Complete the Work Surface (terminal stage)
   - `POST /api/v1/work-surfaces/:id/archive` — Archive Work Surface
   - `GET /api/v1/work-surfaces/check-compatibility` — Check Intake/Template compatibility
   - `GET /api/v1/work-surfaces/by-work-unit/:work_unit_id` — Get active Work Surface for Work Unit

2. **Projections** (`crates/sr-adapters/src/projections.rs`)
   - Add projection handlers for Work Surface events
   - Handle `WorkSurfaceBound`, `StageEntered`, `StageCompleted`, `WorkSurfaceCompleted`, `WorkSurfaceArchived`

3. **Route Registration** (`crates/sr-api/src/main.rs`)
   - Add all 9 Work Surface routes
   - Wire up handler functions

### Verification Checklist

- [ ] `cargo build` passes
- [ ] `cargo test --workspace` passes
- [ ] All 9 endpoints return appropriate responses
- [ ] Compatibility check validates Intake kind vs Template kind[]
- [ ] Stage transitions emit correct events
- [ ] Only one active Work Surface per Work Unit enforced

### Constraints

- **Follow SR-PLAN-V4 §3** — Use the exact API schemas defined in the plan
- **Pattern match intakes.rs** — Follow the same handler patterns
- **Event sourcing** — All mutations via events, projections update state

---

## Summary of Previous Development Iterations

### Session: 2026-01-15 — Phase 4a Implementation + Canonical Doc Updates

**Objective:** Implement Phase 4a (Core Infrastructure) and update canonical specifications to reflect implementation.

**Work Performed:**

1. **Phase 4a Implementation** (from prior context — summarized)
   - Added `WorkSurfaceId` with ULID generation (format: `ws:<ULID>`)
   - Added `WorkSurfaceStatus` enum (Active, Completed, Archived)
   - Added `StageCompletionStatus` enum (Pending, Entered, Completed, Skipped)
   - Added `StageStatusRecord` struct with lifecycle tracking
   - Added `ManagedWorkSurface` struct with full lifecycle methods
   - Added `WorkSurfaceLifecycleError` error type
   - Added `validate_work_kind_compatibility()` function
   - Added Work Surface events to `events.rs` (`WorkSurfaceBound`, `StageEntered`, `StageCompleted`, `WorkSurfaceCompleted`, `WorkSurfaceArchived`)
   - Added `StreamKind::WorkSurface` and `StreamKind::Intake`
   - Created `migrations/006_work_surfaces.sql` with enums, tables, indexes, and constraints
   - Fixed `postgres.rs` exhaustive match for new StreamKind variants

2. **Canonical Specification Updates**
   - **SR-SPEC §1.5.2:** Added `WORK_SURFACE` and `INTAKE` to StreamKind enum
   - **SR-SPEC §1.3.3:** Fixed malformed section (removed orphaned Work Surface events corrupting Candidate identity text)
   - **SR-SPEC Appendix A:** Added Work Surface events and Intake events to canonical event registry
   - **SR-TYPES §7.7:** Expanded Work Surface schema with full field definitions (`WorkSurfaceId`, `WorkSurfaceStatus`, `StageCompletionStatus`, `StageStatusRecord`)
   - **SR-CHANGE §0.3:** Added version entry documenting all spec changes (classified as G:MINOR)

3. **Files Modified**
   | File | Changes |
   |------|---------|
   | `crates/sr-domain/src/work_surface.rs` | Added types, lifecycle methods, 15+ unit tests |
   | `crates/sr-domain/src/events.rs` | Added Work Surface events, StreamKind variants |
   | `crates/sr-adapters/src/postgres.rs` | Added StreamKind::WorkSurface handling |
   | `migrations/006_work_surfaces.sql` | New migration for Work Surfaces |
   | `docs/platform/SR-SPEC.md` | StreamKind, §1.3.3 fix, Appendix A events |
   | `docs/platform/SR-TYPES.md` | Expanded §7.7 Work Surface schema |
   | `docs/build-governance/SR-CHANGE.md` | Added version 0.3 change log |

**Verification:** `cargo build` and `cargo test --workspace` pass (118 tests in sr-domain).

**Next Step:** Implement Phase 4b (Work Surface API) per the prompt above.

---

### Session: 2026-01-16 — SR-PLAN-V4 Coherence Review

**Objective:** Perform coherence review of SR-PLAN-V4 against canonical SR-* specifications.

**Work Performed:**

1. **Document Review (Required Reading)**
   - Read SR-PLAN-V4.md (plan under review — full document)
   - Read SR-CONTRACT.md (§2.3, §2.8, C-CTX-1, C-CTX-2 — binding invariants)
   - Read SR-WORK-SURFACE.md (§2, §5 — primary specification)
   - Read SR-SPEC.md (§1.5.3, §1.9, §3.2.1.1, Appendix A — event patterns)
   - Read SR-TYPES.md (§3.1, §4.3 — type definitions)
   - Read SR-PROCEDURE-KIT.md (§1-2 — stage machine semantics)
   - Read SR-SEMANTIC-ORACLE-SPEC.md (§2-4 — oracle binding)

2. **Coherence Analysis**
   - **Ontology:** 9 items checked — all ✅ with 1 ⚠️ (missing RefKind::Stage consideration)
   - **Epistemology:** 8 items checked — all ✅
   - **Semantics:** 11 items checked — all ✅ with 2 ⚠️ (status enum naming, SQL type naming)

3. **Artifacts Created**
   - `docs/reviews/SR-PLAN-V4-COHERENCE-REVIEW.md` — Full coherence review report

4. **Documentation Updates**
   - Updated `docs/charter/SR-README.md`:
     - Changed SR-PLAN-V4 status from "PENDING COHERENCE REVIEW" to "READY FOR IMPLEMENTATION"
     - Added coherence review summary
     - Merged review prompt with implementation prompt

**Review Verdict:** Coherent with Minor Issues — ready for implementation.

**Minor Issues Identified (addressable during implementation):**
1. Status enum uses lifecycle values (Active/Completed/Archived) instead of artifact status values — acceptable deviation documented
2. Consider adding RefKind::Stage for stage-specific refs
3. Verify SQL type naming for stage_completion_status

**No code was modified.** This was a review-only session.

**Next Step:** Implement Phase 4a (Core Infrastructure) per the prompt.

---

### Session: 2026-01-16 — SR-PLAN-V4 Planning

**Objective:** Create comprehensive implementation plan for Phase 4: Work Surface Composition.

**Work Performed:**

1. **Document Review**
   - Read SR-README.md for assignment and orientation
   - Read SR-WORK-SURFACE (primary specification — §2-5)
   - Read SR-PROCEDURE-KIT (stage machine mechanics)
   - Read SR-SEMANTIC-ORACLE-SPEC (oracle suite binding)
   - Read SR-CONTRACT (C-CTX-1, C-CTX-2 — iteration context invariants)
   - Read SR-SPEC (TypedRef, IterationStarted, event patterns)
   - Read SR-PLAN-V3 (structural reference for plan quality)
   - Reviewed existing `crates/sr-domain/src/work_surface.rs` (foundational types already exist)

2. **Key Design Questions Resolved**
   - **Compatibility checking:** Intake `kind` must match template's supported `kind[]`
   - **Stage initialization:** Start at template's initial stage; emit `StageEntered` immediately
   - **Oracle suite binding:** Per-stage (not whole Work Surface); resolved dynamically on stage entry
   - **Immutability:** Binding refs immutable; only stage progression mutable via events
   - **Relationship to Loops:** Work Surface 1:1 with Work Unit (not Loop)
   - **UI workflow:** Step-by-step wizard (3 steps)

3. **Artifacts Created**
   - `docs/planning/SR-PLAN-V4.md` (~650 lines) — Complete implementation plan including:
     - Executive Summary and Architecture Overview
     - Domain Model (`ManagedWorkSurface`, `WorkSurfaceId`, status enums)
     - Events (`WorkSurfaceBound`, `StageEntered`, `StageCompleted`, etc.)
     - Database Schema (`proj.work_surfaces` with constraints)
     - API Specification (9 endpoints for CRUD, stage transitions, compatibility)
     - UI Specification (3 pages: list, wizard, detail with stage progress)
     - Event Integration (IterationStarted refs, EvidenceBundleRecorded binding)
     - 4-phase implementation breakdown (4a-4d)

4. **Documentation Updates**
   - Updated `docs/charter/SR-README.md`:
     - Changed SR-PLAN-V4 status from "pending" to "ready"
     - Added SR-PLAN-V4 Implementation Status section
     - Added Key Design Decisions table
     - Added Planned Routes
     - Replaced "Create SR-PLAN-V4" prompt with "Implement Phase 4a" prompt

**No code was modified.** This was a planning-only session.

**Next Step:** Implement Phase 4a (Core Infrastructure) per the new prompt.

---

### Session: 2026-01-15 — Phase 3 Implementation

**Objective:** Implement Phase 3 of SR-PLAN-V3 (References Browser UI).

**Work Performed:**

1. **CSS Styles Added** (`ui/src/styles/pages.module.css`)
   - `.referencesLayout` — Two-column grid layout (sidebar + content)
   - `.categorySidebar` — Left sidebar with category list
   - `.categoryItem` / `.categoryItemActive` — Category buttons with count badges
   - `.tableRowHover` — Hover state for clickable table rows

2. **References.tsx Refactored** (~575 lines)
   - Replaced tab-based UI with category sidebar layout
   - 10 categories: Governing Artifacts, Procedure Templates, Oracle Suites, Evidence Bundles, Iterations, Candidates, Exceptions, Agent Definitions, Gating Policies, Intakes
   - Each category fetches from corresponding `/api/v1/references/*` endpoint
   - Category counts fetched on mount
   - Client-side search filtering
   - Pagination with page size selector
   - Row click navigation to appropriate detail pages
   - Document upload preserved (shown with "not implemented" note for stub backend)

3. **GovernedArtifactDetail.tsx Implemented** (~296 lines)
   - Fetches from `/api/v1/references/governed-artifacts/:id`
   - Displays all fields: artifact_id, artifact_type, version, content_hash, status, normative_status, authority_kind
   - Tags, Governed By, and Supersedes cards with links
   - Recording info (recorded_at, recorded_by)

4. **ReferenceBundleDetail.tsx Updated** (~289 lines)
   - Changed endpoint: `/api/v1/context/bundles/:id` → `/api/v1/references/evidence-bundles/:hash`
   - Updated interface to match `EvidenceBundleDetailResponse`
   - Displays verdict, artifact_count, linked entities (run, candidate, iteration, oracle suite)

**Verification:** `npm run type-check` and `npm run build` pass.

---

### Session: 2026-01-15 — Phase 2 Implementation

**Objective:** Implement Phase 2 of SR-PLAN-V3 (Intakes UI — Full Intake CRUD UI).

**Work Performed:**

1. **Shared Editor Components Created**
   - `ArrayStringEditor.tsx` — Reusable array editor for constraints, unknowns, completion_criteria
   - `DeliverablesEditor.tsx` — Editor for deliverables array (name, format, path, description)
   - `DefinitionsEditor.tsx` — Key-value editor for term definitions
   - `InputsEditor.tsx` — TypedRef picker for input references
   - `IntakeLifecycleActions.tsx` — Status-based action buttons with confirmation dialogs

2. **IntakeDetail.tsx Updated**
   - Fixed endpoint: `/api/v1/context/intakes/` → `/api/v1/intakes/`
   - Updated TypeScript interfaces to match backend response
   - Added lifecycle action buttons (Edit, Activate, Fork, Archive)
   - Added sections: Definitions, Unknowns, Completion Criteria
   - Added display for activated_at/activated_by and archived_at/archived_by

3. **Intakes.tsx Implemented**
   - Full list page with status and kind filters
   - Client-side search by title, objective, or ID
   - Pagination with page size selector
   - Status badges: draft=warning, active=success, archived=neutral
   - Clickable rows navigate to detail page

4. **IntakeCreate.tsx Implemented**
   - Form with all SR-WORK-SURFACE §3.1 fields
   - Uses shared editor components
   - Validation and error handling
   - POSTs to `/api/v1/intakes`

5. **IntakeEdit.tsx Implemented**
   - Fetches existing intake data
   - Redirects if intake is not draft (immutable)
   - PUTs to `/api/v1/intakes/:intake_id`
   - Work Unit ID and Kind are read-only after creation

**Verification:** `npm run type-check` and `npm run build` pass.

---

### Session: 2026-01-15 — Phase 1 Implementation

**Objective:** Implement Phase 1 of SR-PLAN-V3 (UI Structure Reorganization).

**Work Performed:**

1. **Sidebar Navigation Update** (`ui/src/layout/Sidebar.tsx`)
   - Added "Intakes" navigation item after Loops
   - Renamed "Context" to "References"
   - Position: Loops → Intakes → References → Prompts

2. **Route Updates** (`ui/src/routes.tsx`)
   - Added 4 Intakes routes: `/intakes`, `/intakes/new`, `/intakes/:intakeId`, `/intakes/:intakeId/edit`
   - Added 4 References routes: `/references`, `/references/documents/:documentId`, `/references/bundles/:bundleId`, `/references/governed-artifacts/:artifactId`
   - Removed old Context routes

3. **File Renames**
   | Old | New |
   |-----|-----|
   | `Context.tsx` | `References.tsx` |
   | `ContextDocumentDetail.tsx` | `ReferenceDocumentDetail.tsx` |
   | `ContextBundleDetail.tsx` | `ReferenceBundleDetail.tsx` |

4. **New Stub Pages Created**
   - `Intakes.tsx` — List page stub
   - `IntakeCreate.tsx` — Create form stub
   - `IntakeEdit.tsx` — Edit form stub
   - `GovernedArtifactDetail.tsx` — Detail view stub

5. **Updated Files**
   - `IntakeDetail.tsx` — Updated breadcrumb to link to `/intakes`
   - `ui/src/pages/index.ts` — Updated exports

**Verification:** `npm run type-check` and `npm run build` pass.

---

### Session: 2026-01-15 — Phase 0c Implementation

**Objective:** Implement Phase 0c of SR-PLAN-V3 (References API — References browser backend).

**Work Performed:**

1. **References Handler** (Complete)
   - Created `crates/sr-api/src/handlers/references.rs` (~645 lines)
   - 15 endpoints for unified References browser API
   - `ReferencesState` combining AppState, OracleRegistryState, and TemplateRegistryState
   - Standardized `{ refs, total, page, page_size }` response format per SR-PLAN-V3 §2.2

2. **Endpoints Implemented:**
   | Endpoint | Source | Status |
   |----------|--------|--------|
   | `GET /api/v1/references` | Aggregated | ✅ |
   | `GET /api/v1/references/governed-artifacts` | `proj.governed_artifacts` | ✅ |
   | `GET /api/v1/references/governed-artifacts/:id` | `proj.governed_artifacts` | ✅ |
   | `GET /api/v1/references/candidates` | `proj.candidates` | ✅ |
   | `GET /api/v1/references/evidence-bundles` | `proj.evidence_bundles` | ✅ |
   | `GET /api/v1/references/evidence-bundles/:hash` | `proj.evidence_bundles` | ✅ |
   | `GET /api/v1/references/oracle-suites` | OracleSuiteRegistry | ✅ |
   | `GET /api/v1/references/procedure-templates` | TemplateRegistry | ✅ |
   | `GET /api/v1/references/exceptions` | `proj.exceptions` | ✅ |
   | `GET /api/v1/references/iteration-summaries` | `proj.iterations` | ✅ |
   | `GET /api/v1/references/agent-definitions` | Stub (empty) | ⚡ |
   | `GET /api/v1/references/gating-policies` | TemplateRegistry | ✅ |
   | `GET /api/v1/references/intakes` | `proj.intakes` | ✅ |
   | `POST /api/v1/references/documents` | Stub (501) | ⚡ |
   | `GET /api/v1/references/documents/:id` | Stub (404) | ⚡ |

3. **Supporting Changes:**
   - Updated `crates/sr-api/src/handlers/mod.rs` — Added references module
   - Updated `crates/sr-api/src/handlers/error.rs` — Added `NotImplemented` error variant
   - Updated `crates/sr-api/src/main.rs` — Added 15 routes and `ReferencesState` initialization

4. **Documentation Updates:**
   - Updated `docs/charter/SR-README.md` — Phase 0c marked complete, next step updated to Phase 1

**Verification:** `cargo build` and `cargo test --workspace` pass (170+ tests).

**Notes:**
- Agent Definitions stubbed (no current data source — deferred to future phase)
- Document upload/retrieval stubbed (storage infrastructure not yet implemented)

---

### Session: 2026-01-16 (Part 3) — Phase 0a & 0b Implementation

**Objective:** Implement Phases 0a and 0b of SR-PLAN-V3 (Core Infrastructure and Intake API).

**Work Performed:**

1. **Phase 0a: Core Infrastructure** (Complete)
   - Created `crates/sr-domain/src/refs.rs` — StrongTypedRef with RefKind, RefRelation enums
   - Created `crates/sr-domain/src/intake.rs` — IntakeUlidId, IntakeStatus enum, ManagedIntake, lifecycle events
   - Created `migrations/005_intakes.sql` — intake_status enum, work_kind enum, proj.intakes table
   - Updated `crates/sr-domain/src/events.rs` — Added StreamKind::Intake and new EventTypes

2. **Phase 0b: Intake API** (Complete)
   - Created `crates/sr-api/src/handlers/intakes.rs` (~940 lines) — Full CRUD + lifecycle handler
   - Updated `crates/sr-api/src/main.rs` — Added intake routes
   - Updated `crates/sr-api/src/handlers/mod.rs` — Registered intakes module
   - Updated `crates/sr-adapters/src/projections.rs` — Added intake event projection handlers
   - Updated `crates/sr-adapters/src/postgres.rs` — Added StreamKind::Intake handling

3. **Documentation Updates**
   - Updated `docs/charter/SR-README.md` — Added Feature Implementation Plans section
   - Updated `docs/program/SR-PLAN.md` — Added `implemented_by` ref to SR-PLAN-V3

**Verification:** `cargo build` and `cargo test --workspace` pass (30 tests in sr-api).

---

### Session: 2026-01-16 (Part 2) — SR-PLAN-V3 Finalization

**Objective:** Resolve all 10 issues from SR-PLAN-V2 and produce an implementation-ready V3 plan.

**Work Performed:**

1. **Document Review**
   - Read SR-PLAN-V2 and identified all 10 `[REVIEW NOTE]` issues
   - Read SR-CONTRACT for binding invariants (C-EVT-1, C-EVID-6, commitment objects)
   - Read SR-SPEC for TypedRef schema (§1.5.3), RefRelation enum, event patterns
   - Read SR-TYPES for status enums (§3.1) and type taxonomy
   - Read SR-WORK-SURFACE for Intake schema (§3.1)
   - Read SR-PROCEDURE-KIT for procedure template structure
   - Reviewed existing `handlers/templates.rs` (Issue #9)

2. **Issue Resolutions**
   - **Issue #1:** Unified InputRef and TypedRef into single schema
   - **Issue #2:** Defined complete event model (IntakeCreated, IntakeUpdated, IntakeActivated, IntakeArchived, IntakeForked)
   - **Issue #3:** Created full PostgreSQL schema for `proj.intakes`
   - **Issue #4:** Aligned status terminology with SR-TYPES §3.1 (draft/active/archived → draft/governed/archived)
   - **Issue #5:** Clarified by-hash retrieval returns all statuses per C-EVID-6
   - **Issue #6:** Standardized all References API responses to `{ refs, total, page, page_size }`
   - **Issue #7:** Verified RefRelation enum is complete per SR-SPEC §1.5.3
   - **Issue #8:** Split Phase 0 into 0a (Core Infrastructure), 0b (Intake API), 0c (References API)
   - **Issue #9:** Confirmed templates.rs handles schemas, not runtime; new handler needed
   - **Issue #10:** Added Agent Definitions and Gating Policies to ref categories

3. **Artifacts Created**
   - `docs/planning/SR-PLAN-V3.md` — Implementation-ready plan with all issues resolved
   - Updated `docs/charter/SR-README.md` — Updated prompt for implementation phase

**No code was modified.** This was a planning-only session.

---

### Session: 2026-01-16 (Part 1) — Initial Planning

**Objective:** Develop a comprehensive implementation plan for Intakes UI/API and References browser.

**Work Performed:**
- Context gathering from SR-* governance docs
- Codebase exploration (found missing backend endpoints)
- V1 and V2 plan development
- Identified 10 issues requiring resolution

**Artifacts Created:**
- `docs/planning/SR-PLAN-V2.md` — Plan with embedded `[REVIEW NOTE]` markers

**User Decisions Established:**
- Intakes are a top-level nav item (separate from References)
- Show all intakes regardless of status, with filter
- No backward compatibility needed — clean implementation
- Backend-first (Phase 0 before UI)
- "References" is acceptable user-facing term (renamed from "Context")
- "Prompts" stays as-is (lower priority)


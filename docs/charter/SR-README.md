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
| SR-PLAN-V4 | pending | Work Surface Composition (Phase 4) — to be created |
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

## Prompt for Next Instance: Create SR-PLAN-V4 (Work Surface Composition)

### Task

Create a comprehensive implementation plan for **Phase 4: Work Surface Composition**. This plan should be written to `docs/planning/SR-PLAN-V4.md` and follow the structure and quality of SR-PLAN-V3.

**Objective:** Design the architecture for binding Intake + Procedure Template + Oracle Suite into **Work Surface Instances** that drive Semantic Ralph Loop iterations.

### What Exists (Prerequisites)

| Component | Location | Status |
|-----------|----------|--------|
| **Intake domain** | `crates/sr-domain/src/intake.rs` | Complete — CRUD + lifecycle (draft/active/archived) |
| **Intake API** | `crates/sr-api/src/handlers/intakes.rs` | Complete — 8 endpoints |
| **Intake UI** | `ui/src/pages/Intakes*.tsx` | Complete — list, create, edit, detail, lifecycle actions |
| **Procedure Templates** | `crates/sr-api/src/handlers/templates.rs` | Exists — registry with list/get |
| **Oracle Suites** | `crates/sr-adapters/src/oracle_suite/` | Exists — registry with list/get |
| **References API** | `crates/sr-api/src/handlers/references.rs` | Complete — unified browser for all ref types |
| **References UI** | `ui/src/pages/References.tsx` | Complete — category sidebar browser |

### What Needs to Be Built

Per **SR-WORK-SURFACE §5**, a Work Surface Instance binds:

```yaml
artifact_type: domain.work_surface
work_unit_id: WU-...
intake_ref:
  id: intake:...
  content_hash: sha256:...
procedure_template_ref:
  id: proc:...
  content_hash: sha256:...
stage_id: stage:FRAME
oracle_suites:
  - suite_id: suite:SR-SUITE-STRUCTURE
    suite_hash: sha256:...
params:
  threshold: 0.9
```

The plan must address:

1. **Domain Model** — `WorkSurface` struct with refs to Intake, Procedure Template, current stage
2. **Database Schema** — `proj.work_surfaces` table with appropriate indexes
3. **Events** — `WorkSurfaceBound`, `StageTransitioned` (per SR-CONTRACT)
4. **API Endpoints** — CRUD + stage transitions + iteration context compilation
5. **UI** — Composition wizard, Work Surface detail view, stage progress visualization
6. **Integration** — How `IterationStarted` binds to Work Surface Instance (SR-SPEC §1.9)

### Required Reading

Before writing the plan, read these documents thoroughly:

| Document | Sections | Purpose |
|----------|----------|---------|
| **SR-WORK-SURFACE** | §2 (concepts), §4 (Procedure Template), §5 (Work Surface Instance) | Primary specification |
| **SR-PROCEDURE-KIT** | §1-2 (stage structure, gate rules) | Stage machine mechanics |
| **SR-SEMANTIC-ORACLE-SPEC** | §2-4 (suite identity, binding) | Oracle suite hash binding |
| **SR-CONTRACT** | §2.8 (Commitment Objects), C-CTX-1, C-CTX-2 | Binding invariants |
| **SR-SPEC** | §1.9 (IterationStarted), §1.5.3 (TypedRef) | Event integration |
| **SR-TYPES** | §4.3 (`domain.work_surface`) | Type registry |
| **SR-PLAN-V3** | Full document | Reference for plan structure and quality |

### Key Design Questions to Address

1. **Compatibility checking** — How to validate that an Intake's `kind` matches Procedure Template's supported kinds?
2. **Stage initialization** — Which stage does a new Work Surface start at? First stage in template?
3. **Oracle suite binding** — Are suites bound per-stage or once for the whole Work Surface?
4. **Immutability** — Once bound, can a Work Surface Instance be modified? Or is it a commitment object?
5. **Relationship to Loops** — Is Work Surface 1:1 with a Loop, or can multiple Loops share a Work Surface?
6. **UI workflow** — Step-by-step wizard vs. single form? How to select compatible templates?

### Deliverable

Create `docs/planning/SR-PLAN-V4.md` with:

1. **Executive Summary** — What this phase enables
2. **Architecture Overview** — Domain model, data flow, component relationships
3. **Phase Breakdown** — Sub-phases (like 0a/0b/0c in V3) if needed
4. **API Specification** — Endpoints, request/response formats
5. **Database Migrations** — Table schemas
6. **UI Specification** — Pages, components, user flows
7. **Event Integration** — How Work Surface connects to iteration events
8. **Verification Checklist** — How to validate the implementation
9. **Intentional Deviations** — Any deviations from SR-* specs (ideally none)

### Constraints

- **Do not implement** — This task is planning only
- **Follow SR-* specifications** — No deviations without documented justification
- **Backend-first phasing** — Domain/API before UI (like V3)
- **Ask questions** — Use AskUserQuestion for design decisions that need user input

### After Planning

Once SR-PLAN-V4 is complete and reviewed, update this SR-README to reference the new plan and provide an implementation prompt for Phase 4.

---

## Summary of Previous Development Iterations

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


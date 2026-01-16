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
| SR-PLAN-V3 | active | Intakes & References implementation (Phases 0-3) |
| SR-PLAN-V2 | superseded | Intakes & References draft (10 unresolved issues) |

---

## Prompt for Next Instance: Intakes & References Implementation

### Task

Continue implementing the Intakes & References infrastructure per **SR-PLAN-V3.md**.

### Current Progress

| Phase | Status | Description |
|-------|--------|-------------|
| Phase 0a | **Complete** | Core Infrastructure — TypedRef module, Intake domain, database migrations, event definitions |
| Phase 0b | **Complete** | Intake API — Intake handler with CRUD + lifecycle operations |
| Phase 0c | **Complete** | References API — References browser backend (15 endpoints) |
| Phase 1 | **Complete** | UI Structure — Sidebar and route reorganization |
| Phase 2 | Pending | Intakes UI — Full Intake CRUD UI |
| Phase 3 | Pending | References UI — References browser UI |

### Next Step

Begin **Phase 2: Intakes UI** — implement full Intake CRUD UI per SR-PLAN-V3 §3 (Phase 2).

### Current UI Structure (Phase 1 Output)

**Sidebar navigation** (`ui/src/layout/Sidebar.tsx`):
- Loops → **Intakes** → **References** → Prompts

**Routes** (`ui/src/routes.tsx`):
```
/intakes                  → Intakes.tsx (stub - implement in Phase 2)
/intakes/new              → IntakeCreate.tsx (stub - implement in Phase 2)
/intakes/:intakeId        → IntakeDetail.tsx (exists, needs lifecycle actions)
/intakes/:intakeId/edit   → IntakeEdit.tsx (stub - implement in Phase 2)
/references               → References.tsx (functional, renamed from Context)
/references/documents/:id → ReferenceDocumentDetail.tsx
/references/bundles/:id   → ReferenceBundleDetail.tsx
/references/governed-artifacts/:id → GovernedArtifactDetail.tsx (stub)
```

**Stub pages to implement** (currently show placeholder text):
- `ui/src/pages/Intakes.tsx` — List page
- `ui/src/pages/IntakeCreate.tsx` — Create form
- `ui/src/pages/IntakeEdit.tsx` — Edit form

**Existing page to enhance**:
- `ui/src/pages/IntakeDetail.tsx` — Needs lifecycle action buttons

### Backend API Available (Phase 0b)

The Intakes API is fully implemented in `crates/sr-api/src/handlers/intakes.rs`:

| Method | Endpoint | Purpose |
|--------|----------|---------|
| POST | `/api/v1/intakes` | Create draft intake |
| GET | `/api/v1/intakes` | List intakes (query: `status`, `kind`, `page`, `page_size`) |
| GET | `/api/v1/intakes/:intake_id` | Get intake by ID |
| GET | `/api/v1/intakes/by-hash/:content_hash` | Get intake(s) by content hash |
| PUT | `/api/v1/intakes/:intake_id` | Update draft intake |
| DELETE | `/api/v1/intakes/:intake_id` | Delete draft intake |
| POST | `/api/v1/intakes/:intake_id/activate` | Transition draft → active |
| POST | `/api/v1/intakes/:intake_id/archive` | Transition active → archived |
| POST | `/api/v1/intakes/:intake_id/fork` | Fork to new draft |

### Intake Schema (SR-WORK-SURFACE §3.1)

Required fields for intake creation/editing:

```typescript
interface IntakeForm {
  work_unit_id: string;        // e.g., "wu:research-rate-limiting"
  title: string;               // Human-readable title
  kind: WorkKind;              // research_memo | decision_record | ontology_build | analysis_report | design_document | review_response
  objective: string;           // ONE sentence
  audience: string;            // Target audience
  deliverables: Deliverable[]; // { name, format, path, description? }
  constraints: string[];       // Length, tone, required sections, etc.
  definitions: Record<string, string>; // Term definitions
  inputs: TypedRef[];          // Content-addressed references
  unknowns: string[];          // Questions to resolve
  completion_criteria: string[]; // Success criteria
}
```

### Deliverables (Phase 2)

1. **Intakes list page** (`ui/src/pages/Intakes.tsx`)
   - Fetch from `GET /api/v1/intakes`
   - Filter by status (draft/active/archived), kind
   - Search by title
   - Status badges: Draft=yellow (`warning`), Active=green (`success`), Archived=gray (`neutral`)
   - Pagination using `page` and `page_size` query params
   - "New Intake" button → `/intakes/new`
   - Click row → `/intakes/:intakeId`

2. **Intake create form** (`ui/src/pages/IntakeCreate.tsx`)
   - POST to `/api/v1/intakes`
   - All SR-WORK-SURFACE §3.1 fields
   - Array editors for: deliverables, constraints, unknowns, completion_criteria
   - Key-value editor for definitions
   - Input references selector (TypedRef picker - can be simplified for Phase 2)
   - On success → navigate to `/intakes/:intakeId`

3. **Intake detail page** (`ui/src/pages/IntakeDetail.tsx`) — **Enhance existing**
   - Add lifecycle action buttons based on status:
     - Draft: [Edit] → `/intakes/:id/edit`, [Activate] → POST activate, [Delete] → DELETE
     - Active: [Fork to New Draft] → POST fork, [Archive] → POST archive
     - Archived: [Fork to New Draft] → POST fork
   - Display content_hash for Active/Archived intakes
   - Show activated_at, activated_by for Active intakes

4. **Intake edit form** (`ui/src/pages/IntakeEdit.tsx`)
   - Fetch from `GET /api/v1/intakes/:intake_id`
   - PUT to `/api/v1/intakes/:intake_id`
   - Same form as create, pre-populated
   - Only accessible for Draft status (redirect if not draft)

### UI Patterns Reference

Existing pages to reference for patterns:
- `ui/src/pages/Loops.tsx` — List page with filters and pagination
- `ui/src/pages/LoopDetail.tsx` — Detail page with action buttons
- `ui/src/pages/Templates.tsx` — Complex forms
- `ui/src/ui/index.tsx` — Shared components (Card, Pill, Button, etc.)
- `ui/src/styles/pages.module.css` — Shared page styles

Auth pattern:
```typescript
const auth = useAuth();
fetch(`${config.apiUrl}/api/v1/intakes`, {
  headers: { Authorization: `Bearer ${auth.user.access_token}` }
});
```

### Required Reading

1. **`docs/planning/SR-PLAN-V3.md`** §3 (Phase 2) — Full Intakes UI specification
2. **`docs/platform/SR-WORK-SURFACE.md`** §3.1 — Intake schema definition
3. **`crates/sr-api/src/handlers/intakes.rs`** — Backend API implementation
4. **`ui/src/pages/Loops.tsx`** — Reference for list page pattern
5. **`ui/src/pages/LoopDetail.tsx`** — Reference for detail page with actions

### Verification

After Phase 2:
```bash
cd ui
npm run type-check    # No TypeScript errors
npm run build         # Build succeeds
npm run dev           # Manual verification:
                      #   - Navigate to /intakes
                      #   - Create new intake
                      #   - View intake detail
                      #   - Edit draft intake
                      #   - Activate intake
                      #   - Fork active intake
                      #   - Archive active intake
```

### Constraints

- **Phase 1 complete** — UI structure and routes are ready
- **Follow SR-PLAN-V3** — All components and interactions are specified
- **Use existing API** — Backend Intakes API is ready (Phase 0b complete)
- **Use existing patterns** — Follow Loops.tsx/LoopDetail.tsx patterns
- **Stop after phase** — Wait for instructions before proceeding to Phase 3

---

## Summary of Previous Development Iterations

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


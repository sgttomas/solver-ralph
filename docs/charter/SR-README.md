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

Tasks are no longer assigned by SR-PLAN because the build out phase is complete.  We are now in auditing, quality control, and implementation testing.

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




## Development Session Summary (2026-01-14)

**Branch:** `solver-ralph-2`

### Completed Work

#### 1. UI Redesign Integration
Integrated the Chirality AI governance console UI with existing functional pages:
- Restored `AuthProvider` wrapper in `main.tsx`
- Added auth check and loading state in `AppLayout.tsx`
- Connected all functional pages in `routes.tsx` (Loops, LoopDetail, IterationDetail, CandidateDetail, Evidence, EvidenceDetail, Approvals, PromptLoop)
- Added user info display and logout button to `Topbar.tsx`
- Fixed ESLint errors in `AuthProvider.tsx` and `PromptLoop.tsx`

#### 2. Custom Logo
- Added custom logo image (`ui/public/logo.png`) to replace the orange square placeholder in the sidebar

#### 3. UI Terminology Updates
Renamed user-facing labels throughout the UI to better reflect the platform's concepts:

| Old Term | New Term | Rationale |
|----------|----------|-----------|
| Loops | Workflows | Clearer terminology for workflow collections |
| Prompt Loop | Tasks | Simplified name for the task interface |
| Evidence | Artifacts | More general term for oracle outputs |
| Documents | Context | Better reflects the purpose |

#### 4. Sidebar Navigation Reordering
Final sidebar order (top to bottom):
1. Overview
2. Agents
3. Protocols
4. Workflows
5. Tasks
6. Context
7. Artifacts
8. Approvals
9. Audit Log
10. Settings

### Quality Status
- TypeScript type-check: PASS
- ESLint: PASS
- UI build: PASS
- Rust tests: 27 passed, 0 failed
- E2E harness tests: 16 passed, 0 failed

### Commits (chronological)
1. `1151fa1` - Integrate Chirality AI UI with functional pages and auth
2. `917fbc8` - Fix ESLint errors in AuthProvider and PromptLoop
3. `989362d` - Add custom logo to sidebar
4. `abfbb63` - Rename UI labels: Loops→Workflows, Prompt Loop→Tasks
5. `bedf193` - Rename loop references to task on Task page
6. `07a6ae7` - Update search placeholder: loops → workflows
7. `6668e22` - Rename Evidence to Artifacts throughout UI
8. `6627184` - Rename Documents to Context in UI
9. `3befa4e` - Reorder sidebar navigation items

### Notes
- Dev auth bypass: `VITE_DEV_AUTH_BYPASS=true`
- Backend auth bypass: `SR_AUTH_TEST_MODE=true`
- The `src/pages/PromptLoop.tsx` is the FUNCTIONAL one with SSE streaming; `src/screens/PromptLoopScreen.tsx` is a wireframe (can be removed)

---

## Development Session Summary (2026-01-15)

**Branch:** `solver-ralph-2`

### Completed Work: Templates UI (Phase 1)

Implemented the Templates management page per SR-TEMPLATES.md, enabling users to browse, view, and instantiate templates from the 11 template categories.

#### 1. Backend API (Rust)

Created new Template Registry API in `crates/sr-api/src/handlers/templates.rs`:

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/templates` | List all template instances (filterable by category) |
| GET | `/api/v1/templates/:id` | Get template instance detail with schema |
| POST | `/api/v1/templates` | Create new template instance |
| GET | `/api/v1/templates/schemas` | List all template schemas |
| GET | `/api/v1/templates/schemas/:type_key` | Get schema detail |

**Template Categories (7 user-facing tabs):**
1. **Work Surface** - Intakes, Procedure Templates, Work Surface Instances
2. **Execution** - Budget Configs, Gating Policies
3. **Oracle** - Oracle Suite Definitions
4. **Verification** - Verification Profiles
5. **Semantic** - Semantic Sets
6. **Context** - Iteration Context Refs
7. **Exceptions** - Waivers, Deviations, Deferrals

#### 2. Frontend UI (React/TypeScript)

| Page | Route | Features |
|------|-------|----------|
| `Templates.tsx` | `/templates` | Category tabs, schema browser with expandable fields, instance list |
| `TemplateDetail.tsx` | `/templates/:category/:templateId` | Full schema info, field tables, references, raw JSON viewer |

#### 3. Files Created/Modified

| File | Change |
|------|--------|
| `crates/sr-api/src/handlers/templates.rs` | NEW - Template registry (~600 lines) |
| `crates/sr-api/src/handlers/mod.rs` | Export templates module |
| `crates/sr-api/src/main.rs` | Add TemplateRegistryState, wire routes |
| `ui/src/pages/Templates.tsx` | NEW - Templates list page |
| `ui/src/pages/TemplateDetail.tsx` | NEW - Template detail page |
| `ui/src/pages/index.ts` | Export new pages |
| `ui/src/routes.tsx` | Add template routes |
| `ui/src/layout/Sidebar.tsx` | Add Templates nav item |

### Quality Status
- Backend: `cargo build` PASS, 22 tests pass
- Frontend: `npm run type-check && npm run build` PASS

---

## Completed Work: Templates UI (Phase 2) - Starter Templates

### Overview

Seeded the Templates registry with 11 starter reference templates demonstrating correct schema usage per SR-TEMPLATES. These are read-only examples users can clone.

### Starter Templates Created

| # | Type Key | Name | Category |
|---|----------|------|----------|
| 1 | `record.intake` | Standard Research Memo Intake | WorkSurface |
| 2 | `config.procedure_template` | Research Memo Procedure | WorkSurface |
| 3 | `domain.work_surface` | Example Work Surface Binding | WorkSurface |
| 4 | `budget_config` | Standard Budget Policy | Execution |
| 5 | `config.gating_policy` | Hybrid Gating Policy | Execution |
| 6 | `verification_profile` | Project Standard Profile | Verification |
| 7 | `config.semantic_set` | Research Memo Quality Set | SemanticSets |
| 8 | `record.waiver` | Example Waiver Template | Exceptions |
| 9 | `record.deviation` | Example Deviation Template | Exceptions |
| 10 | `record.deferral` | Example Deferral Template | Exceptions |
| 11 | `oracle_suite` | Custom Verification Suite | Oracle |

### Implementation Details

**Backend (`templates.rs`):**
- Added `build_starter_instances()` method creating 11 templates
- ID format: `tmpl_starter_{type_key_suffix}` (e.g., `tmpl_starter_intake`)
- Status: `"reference"` (read-only)
- Created by: `"system"`
- Seeded on `TemplateRegistry::new()`

**Frontend (Templates.tsx):**
- Reference templates sorted first in each category
- "Reference" badge (info tone - blue) on status pill
- Clone button for reference templates
- Clone creates editable copy with "(Copy)" suffix

**Frontend (TemplateDetail.tsx):**
- Read-only banner for reference templates
- Clone Template button in header
- Info banner explaining reference templates

**UI Components:**
- Added `--info` CSS variable (#2e5b8c) to theme.css
- Added `info` tone to Pill component

### Quality Status
- Backend: `cargo build` PASS, 24 tests pass
- Frontend: `npm run type-check && npm run build` PASS

---

## Completed Work: Templates UI (Phase 3) - Clone & Edit Workflow

### Overview

Enhanced the Templates UI with full clone and edit capabilities, allowing users to create customized instances from reference templates.

### New API Endpoint

| Method | Endpoint | Description |
|--------|----------|-------------|
| PUT | `/api/v1/templates/:id` | Update template name and/or content |

**Constraints:**
- Reference templates (`status: "reference"`) cannot be updated directly
- Must clone first to create editable copy

### Frontend Enhancements

**Templates.tsx - Starter Templates Section:**
- Dedicated card-based layout for reference templates
- Each card shows template name, type key, and "Reference" badge
- **View** button → opens detail page
- **Use Template** button → clones and opens edit mode

**Templates.tsx - Your Templates Section:**
- Table view for user-created template instances
- Shows ID, name, type, status, hash, and created date

**TemplateDetail.tsx - Clone & Edit:**
- **Clone Template** button in reference banner (prominent placement)
- **Use Template** button in header for reference templates
- **Edit** button in header for user templates
- Edit mode with:
  - Template name input field
  - JSON content textarea with syntax highlighting
  - Save/Cancel buttons
  - JSON validation before save
- Auto-opens edit mode after cloning (`?edit=true` query param)

### User Workflow

1. Navigate to **Templates** in sidebar
2. Select category tab (Work Surface, Oracle, etc.)
3. View **Starter Templates** cards
4. Click **Use Template** or **Clone Template**
5. Customize name and JSON content for specific use case
6. Click **Save Changes**
7. New template appears in **Your Templates** section

### Quality Status
- Backend: `cargo build` PASS, 26 tests pass
- Frontend: `npm run type-check && npm run build` PASS

### Commits (2026-01-15 continued)
- `75edc76` - Implement Templates UI with starter reference templates (Phase 1+2)
- `2d4bc59` - Update SR-README.md with Phase 2 completion summary
- `b1eb5aa` - Add starter templates visibility and user clone/edit workflow
- `1658b78` - Add debug info card to Templates page for troubleshooting
- `b286520` - Add Clone Template button to reference banner, remove debug code

---

## Implementation Status

### Templates UI - COMPLETE

| Feature | Status |
|---------|--------|
| Template schemas (14 types) | ✅ Implemented |
| Starter reference templates (11) | ✅ Implemented |
| List/browse by category | ✅ Implemented |
| View template details | ✅ Implemented |
| Clone reference templates | ✅ Implemented |
| Edit user templates | ✅ Implemented |
| Create from schema | ⏳ Future |
| Template versioning | ⏳ Future |
| Portal approval integration | ⏳ Future |

### Notes
- Backend auth bypass: `SR_AUTH_TEST_MODE=true`
- Frontend auth bypass: `VITE_DEV_AUTH_BYPASS=true`
- Templates nav item added to sidebar
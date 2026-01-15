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

## Development Session Summary (2026-01-14, Session 2)

**Branch:** `solver-ralph-2`

### Completed Work

#### UI Component Library Port
Ported all functional pages from inline styles to use the shared component library (`Card`, `Pill`, `Button`) and CSS module for consistent Chirality design aesthetic.

**New Files Created:**
| File | Purpose |
|------|---------|
| `ui/src/styles/pages.module.css` | Shared CSS module with common patterns (tables, forms, tabs, breadcrumbs, etc.) |
| `ui/src/ui/utils.ts` | Utility functions: `getStatusTone()`, `truncate()`, `truncateHash()` |
| `ui/src/ui/index.ts` | Export barrel for UI primitives |

**Pages Ported (8 total):**
| Page | Key Changes |
|------|-------------|
| `Loops.tsx` | Card wrapper, Pill for status badges, shared table styles |
| `Evidence.tsx` | Card wrapper, truncateHash for content hashes |
| `LoopDetail.tsx` | Breadcrumbs, info rows, stats grid, iteration table |
| `IterationDetail.tsx` | Context refs list, summary sections, candidate table |
| `CandidateDetail.tsx` | Tabbed content (Runs/Artifacts/Freeze), Button for forms |
| `EvidenceDetail.tsx` | Oracle results cards, artifact download buttons, raw manifest viewer |
| `Approvals.tsx` | Three-tab layout (Approvals/Exceptions/Decisions), all forms ported |
| `PromptLoop.tsx` | Task form, streaming output display, artifact stats grid |

**Design System Applied:**
- Replaced hardcoded colors (`#1a1a2e`, `#0066cc`, etc.) with CSS variables (`var(--ink)`, `var(--accent)`, etc.)
- Replaced inline style objects with CSS module classes
- Used `getStatusTone()` utility to map status strings to Pill tones (success/warning/danger/neutral)
- Consistent typography using `var(--font)` and `var(--mono)`

### Quality Status
- TypeScript type-check: PASS
- ESLint: PASS
- Vite build: PASS (642ms)

### Architecture Notes
The UI now follows a consistent pattern:
```
ui/src/
├── ui/           # Primitives (Button, Card, Pill) + utils
├── styles/       # pages.module.css (shared page patterns)
├── pages/        # Functional pages using primitives + shared styles
├── screens/      # Wireframe placeholders (can be deprecated)
└── layout/       # AppLayout, Sidebar, Topbar
```

### What's Next
- Wire up remaining placeholder screens (Agents, Protocols, Context, Audit, Settings)
- Add real data fetching to OverviewScreen dashboard
- Remove deprecated `src/screens/` wireframes once replaced

---

## Development Session Summary (2026-01-14, Session 3)

**Branch:** `solver-ralph-2`

### Completed Work

#### 1. Fixed Placeholder Page 404 Errors
Pages that were showing "Error: HTTP 404" (Agents, Protocols, Context, Audit, Settings) now gracefully handle missing backend endpoints by displaying empty state instead of errors.

#### 2. URL Path Updates
Updated internal URLs to match renamed page labels:
| Old Path | New Path | Page |
|----------|----------|------|
| `/loops` | `/workflows` | Workflows list |
| `/evidence` | `/artifacts` | Artifacts list |
| `/prompt-loop` | `/loops` | Loops (prompt interface) |
| `/documents` | `/context` | Context documents |

#### 3. Background Color Fix
Updated theme to match chirality.ai's creamy beige:
- `--paper`: `#ede4d4` (was `#fbf7f2`)
- `--paper2`: `#e5dac8`

#### 4. Detail Page Implementations
Replaced 5 `PlaceholderScreen` stubs with fully functional detail pages:

| Page | Route | Features |
|------|-------|----------|
| `AgentDetail.tsx` | `/agents/:agentId` | Overview, Capabilities, Work Envelope, Trust Constraints, Recent Proposals |
| `ProtocolDetail.tsx` | `/protocols/:templateId` | Stage Flow visualization, Expandable stages, Active Work Units |
| `ContextDocumentDetail.tsx` | `/context/:documentId` | Metadata, Tags, Content Preview, Download, References |
| `IntakeDetail.tsx` | `/context/intakes/:intakeId` | Objective, Deliverables, Constraints (grouped), Inputs, Non-Goals |
| `ContextBundleDetail.tsx` | `/context/bundles/:bundleId` | Linked Entities, Context References, Raw JSON |

#### 5. Terminology Revert
Reverted "Tasks" back to "Loops" throughout the UI (sidebar, routes, App.tsx).

#### 6. Comprehensive Loops Page Implementation
Major enhancement to the core Loops functionality per SR-SPEC domain model:

**New Files Created:**
| File | Purpose |
|------|---------|
| `hooks/useLoops.ts` | Data fetching with filtering, pagination, sorting, state transitions |
| `hooks/index.ts` | Hooks module exports |
| `components/BudgetProgress.tsx` | Progress bars with warning/danger thresholds |
| `components/BudgetProgress.module.css` | Budget component styles |
| `components/StageProgress.tsx` | Visual stage flow with expandable details |
| `components/StageProgress.module.css` | Stage component styles |
| `components/StateTransitionButton.tsx` | State actions with confirmation dialogs |
| `components/StateTransitionButton.module.css` | Button styles |
| `components/LoopCreateModal.tsx` | Loop creation with full Work Surface binding |
| `components/LoopEditModal.tsx` | Loop editing (when CREATED/PAUSED) |
| `components/LoopModal.module.css` | Shared modal styles |
| `styles/loops.module.css` | Loops page specific styles |

**Enhanced Pages:**

**Loops.tsx (List Page):**
- Search by goal, work unit, or ID
- State filter dropdown (All, CREATED, ACTIVE, PAUSED, CLOSED)
- Sortable columns (Progress, Created)
- Pagination with page size selector (10, 25, 50)
- Quick action buttons per row (Activate/Pause/Resume)
- "Create Loop" button with modal
- Progress bar visualization (iterations used / max)

**LoopDetail.tsx (Detail Page):**
- State transition buttons in header (Activate/Pause/Resume/Close)
- Edit button with modal
- **Work Surface Card**: Intake link, Procedure Template link, Current Stage, Oracle Suite
- **Stage Progress Card**: Visual flow with completion status, expandable stage details
- **Budgets Card**: Progress bars for Iterations, Oracle Runs, Wallclock Hours
- **Stop Triggers Card**: Table of fired triggers with resolution status
- **Active Exceptions Card**: Table of DEVIATION/DEFERRAL/WAIVER records
- **Enhanced Iterations Table**: Candidates count, Duration column
- **Governed Artifacts Card**: List of SR-CONTRACT, SR-SPEC, etc. with version/hash

**LoopCreateModal Features:**
- Goal (required), Work Unit
- Procedure Template dropdown with quick-select buttons
- Stage dropdown (populated from selected template)
- Oracle Suite dropdown
- Budget configuration (max iterations, oracle runs, wallclock hours)
- "Activate after create" checkbox

### Quality Status
- TypeScript type-check: PASS
- ESLint: PASS
- Vite build: PASS (92 modules, 465KB bundle)

### Architecture Notes
New component structure follows SR-SPEC domain model:
```
ui/src/
├── hooks/
│   ├── useLoops.ts      # Loop data + state transitions
│   └── index.ts
├── components/
│   ├── BudgetProgress.tsx
│   ├── StageProgress.tsx
│   ├── StateTransitionButton.tsx
│   ├── LoopCreateModal.tsx
│   ├── LoopEditModal.tsx
│   └── index.ts
├── pages/
│   ├── Loops.tsx        # Enhanced list page
│   └── LoopDetail.tsx   # Enhanced detail page
└── styles/
    └── loops.module.css
```

### Domain Model Alignment
The Loops implementation now reflects SR-SPEC concepts:
- **Loop States**: CREATED → ACTIVE ↔ PAUSED → CLOSED
- **Work Surface**: Intake + Procedure Template + Stage + Oracle Suite
- **Budgets**: max_iterations, max_oracle_runs, max_wallclock_hours
- **Stop Triggers**: ORACLE_GAP, BUDGET_EXHAUSTED, REPEATED_FAILURE, etc.
- **Exceptions**: DEVIATION, DEFERRAL, WAIVER with status tracking
- **Stage Progress**: Visual representation of procedure template flow

### What's Next
- Wire real-time updates (WebSocket/polling) for loop status
- Add audit log entries for state transitions
- Connect Overview dashboard to loop statistics
- Implement bulk operations on list page

---

## Development Session Summary (2026-01-14, Session 4)

**Branch:** `solver-ralph-2`

### Completed Work

#### 1. URL Corrections: Loops at `/loops`
Corrected the URL structure so that Loops (the core platform component) are at `/loops`:

| Route | Page | Change |
|-------|------|--------|
| `/loops` | Loops list | Was `/workflows` |
| `/loops/:loopId` | Loop detail | Was `/workflows/:loopId` |
| `/prompts` | Prompts | Was `/prompt`, then `/loops` |

**Files Updated (20+ files):**
- `routes.tsx`, `App.tsx` - Route definitions
- `Sidebar.tsx`, `Layout.tsx`, `Home.tsx` - Navigation links
- `Loops.tsx`, `LoopDetail.tsx` - Breadcrumbs, back links
- `IterationDetail.tsx`, `CandidateDetail.tsx` - Loop references
- `IntakeDetail.tsx`, `ProtocolDetail.tsx` - Work unit links
- `ContextBundleDetail.tsx`, `ContextDocumentDetail.tsx` - Loop links
- `Context.tsx`, `Audit.tsx` - Reference links
- `Agents.tsx`, `AgentDetail.tsx` - Work unit links
- `OverviewScreen.tsx` - "Active Loops" label

**Labels Updated:**
- "Workflows" → "Loops" (page title, breadcrumbs, error messages, placeholders)
- "Active Workflows" → "Active Loops" (OverviewScreen)

#### 2. New Workflows Page
Created a new basic Workflows page for higher-level orchestrations:

**New File:** `ui/src/pages/Workflows.tsx`
- Route: `/workflows`
- Purpose: Orchestrated sequences of loops and procedures
- Features: Stats overview, info note, workflows table
- Handles 404 gracefully with empty state

**Sidebar Position:** Between Protocols and Loops

#### 3. Prompts Page Updates
Renamed and relocated the prompt interface page:

| Aspect | Before | After |
|--------|--------|-------|
| URL | `/prompt` | `/prompts` |
| Page Title | "Task" | "Prompts" |
| Sidebar Label | "Prompt" | "Prompts" |
| Button | "Run Task" | "Run Prompt" |
| Loading Text | "Initializing task..." | "Initializing prompt..." |
| Artifact Label | "Task" | "Loop" |

### Current Sidebar Order
1. Overview
2. Agents
3. Protocols
4. **Oracles** (new - oracle suites and verification profiles)
5. Workflows
6. Loops
7. Prompts
8. Context
9. Artifacts
10. Approvals
11. Audit Log
12. Settings

### Quality Status
- TypeScript type-check: PASS
- ESLint: PASS
- Vite build: PASS (93 modules, 468KB bundle)

### Architecture Notes
The URL structure now correctly reflects the domain model:
- `/workflows` - Higher-level orchestrations (new)
- `/loops` - Core work units (the primary platform component)
- `/prompts` - Quick prompt-to-loop interface

### Terminology Clarification
| Term | Definition | URL |
|------|------------|-----|
| **Workflows** | Higher-level orchestrations containing multiple loops | `/workflows` |
| **Loops** | Bounded work units with iterations (core platform component) | `/loops` |
| **Prompts** | Interface to quickly create and run governed loops | `/prompts` |

---

## Development Session Summary (2026-01-15)

**Branch:** `solver-ralph-2`

### Completed Work: Oracles Management Page

Implemented a complete Oracles management page per SR-SEMANTIC-ORACLE-SPEC for viewing and registering oracle suites and verification profiles.

#### 1. Backend API (Rust)

Created new Oracle Registry API endpoints in `crates/sr-api/src/handlers/oracles.rs`:

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/oracles/suites` | List all registered oracle suites |
| GET | `/api/v1/oracles/suites/:suite_id` | Get suite detail with oracles |
| POST | `/api/v1/oracles/suites` | Register a new oracle suite |
| GET | `/api/v1/oracles/profiles` | List verification profiles |
| GET | `/api/v1/oracles/profiles/:profile_id` | Get profile detail |
| POST | `/api/v1/oracles/profiles` | Register a new verification profile |

**Key Implementation Details:**
- Added `OracleRegistryState` to main.rs with `OracleSuiteRegistry::with_core_suites()`
- Pre-registered 4 core suites: SR-SUITE-GOV, SR-SUITE-CORE, SR-SUITE-FULL, intake_admissibility
- Pre-registered 3 profiles: GOV-CORE, STRICT-CORE, STRICT-FULL
- Suite hash computed deterministically from oracle definitions

#### 2. Frontend UI (React/TypeScript)

Created three new pages in `ui/src/pages/`:

| Page | Description |
|------|-------------|
| `Oracles.tsx` | Main page with tabs for Suites and Profiles, stats cards, core suites reference table |
| `OracleSuiteDetail.tsx` | Detailed view showing environment constraints, OCI image, expandable oracles table |
| `VerificationProfileDetail.tsx` | Shows required/optional suites, waivable failures, integrity conditions, applicable deliverables |

**Navigation Updates:**
- Added "Oracles" to sidebar between "Protocols" and "Workflows"
- Added routes: `/oracles`, `/oracles/suites/:suiteId`, `/oracles/profiles/:profileId`

#### 3. Documentation Updates

- **SR-SPEC §2.3.11**: Added Oracle Registry API section documenting all 6 endpoints
- **SR-README**: Updated sidebar order to include Oracles

### Files Created
| File | Purpose |
|------|---------|
| `crates/sr-api/src/handlers/oracles.rs` | Backend API handlers (580 lines) |
| `ui/src/pages/Oracles.tsx` | Main oracles list page |
| `ui/src/pages/OracleSuiteDetail.tsx` | Suite detail page |
| `ui/src/pages/VerificationProfileDetail.tsx` | Profile detail page |

### Files Modified
| File | Change |
|------|--------|
| `crates/sr-api/src/handlers/mod.rs` | Export oracles module |
| `crates/sr-api/src/main.rs` | Add OracleRegistryState, wire oracle routes |
| `ui/src/layout/Sidebar.tsx` | Add Oracles nav item |
| `ui/src/routes.tsx` | Add oracle routes |
| `ui/src/pages/index.ts` | Export new pages |
| `docs/platform/SR-SPEC.md` | Add §2.3.11 Oracle Registry API |

### Quality Status
- Backend tests: 17 passed (including 2 new oracle tests)
- Frontend build: PASS
- TypeScript type-check: PASS

### Remaining Work (Optional)
- Registration modals for creating suites/profiles via UI forms (API already supports this)

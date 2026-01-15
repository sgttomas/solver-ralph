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

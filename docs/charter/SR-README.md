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

## Phase 2 Plan: Starter Templates

### Overview

Seed the Templates registry with comprehensive starter template instances for each user-instantiable type. These serve as reference implementations demonstrating correct schema usage.

### Starter Instances to Create

#### Category 1: Work Surface (Self-Service)

**1.1 Intake (`record.intake`)** - `Standard Research Memo Intake`
```json
{
  "work_unit_id": "WU-TEMPLATE-001",
  "title": "API Rate Limiting Analysis",
  "kind": "research_memo",
  "objective": "Evaluate rate limiting strategies for the public API...",
  "deliverables": ["Analysis of current implementation", "Comparison of 3+ strategies", "Recommendation with approach"],
  "constraints": ["Maximum 2000 words", "Must include performance impact"],
  "completion_criteria": ["All strategies evaluated", "Recommendation includes migration path"]
}
```

**1.2 Procedure Template (`config.procedure_template`)** - `Research Memo Procedure`
```json
{
  "procedure_template_id": "proc:RESEARCH-MEMO",
  "kind": ["research_memo"],
  "stages": [
    {"stage_id": "stage:FRAME", "stage_name": "Frame", "required_oracle_suites": ["suite:SR-SUITE-GOV"]},
    {"stage_id": "stage:OPTIONS", "stage_name": "Options Analysis", "required_oracle_suites": ["suite:SR-SUITE-GOV", "suite:SR-SUITE-CORE"]},
    {"stage_id": "stage:DRAFT", "stage_name": "Draft", "required_oracle_suites": ["suite:SR-SUITE-CORE"]},
    {"stage_id": "stage:FINAL", "stage_name": "Final", "required_oracle_suites": ["suite:SR-SUITE-CORE", "suite:SR-SUITE-FULL"]}
  ]
}
```

**1.3 Work Surface Instance (`domain.work_surface`)** - `Example Work Surface Binding`

#### Category 2: Execution Policy (Self-Service)

**2.1 Budget Configuration (`budget_config`)** - `Standard Budget Policy`
```json
{
  "policy_id": "budget:STANDARD",
  "max_iterations": 5,
  "max_oracle_runs": 25,
  "max_wallclock_hours": 16,
  "on_exhaustion": {"stop_trigger": "BUDGET_EXHAUSTED", "routing_portal": "HumanAuthorityExceptionProcess"}
}
```

**2.2 Gating Policy (`config.gating_policy`)** - `Hybrid Gating Policy`

#### Category 3: Oracle (Portal Required)

**3.1 Oracle Suite (`oracle_suite`)** - `Custom Verification Suite`

#### Category 4: Verification (Portal Required)

**4.1 Verification Profile (`verification_profile`)** - `Project Standard Profile`

#### Category 5: Semantic Sets (Portal Required)

**5.1 Semantic Set (`config.semantic_set`)** - `Research Memo Quality Set`

#### Category 6: Exceptions (Portal Required)

**6.1 Waiver (`record.waiver`)** - `Example Waiver Template`
**6.2 Deviation (`record.deviation`)** - `Example Deviation Template`
**6.3 Deferral (`record.deferral`)** - `Example Deferral Template`

### Implementation Approach

1. **Backend**: Add `build_starter_instances()` to `TemplateRegistry` in `templates.rs`
2. **Frontend**: Show starter templates with "Reference" badge, add "Clone" button
3. **Status**: Reference templates use `status: "reference"` and are read-only

### Files to Modify

| File | Changes |
|------|---------|
| `crates/sr-api/src/handlers/templates.rs` | Add `build_starter_instances()`, `seed_starter_templates()` |
| `ui/src/pages/Templates.tsx` | Add reference template section, clone functionality |
| `ui/src/pages/TemplateDetail.tsx` | Add read-only mode for reference templates |

---

## Next Instance Prompt

```
read docs/charter/SR-README.md and then docs/charter/SR-CHARTER.md and then docs/platform/SR-TEMPLATES.md. Your task is to implement Phase 2 of the Templates UI: Starter Templates.

The Phase 1 Templates UI is complete (see SR-README.md "Development Session Summary 2026-01-15"). Now implement Phase 2:

1. In `crates/sr-api/src/handlers/templates.rs`:
   - Add `build_starter_instances()` method that creates 11 starter template instances (see Phase 2 Plan in SR-README.md for the JSON structures)
   - Add `seed_starter_templates()` to call this during `TemplateRegistry::new()`
   - Use ID prefix `tmpl_starter_` and status `"reference"` for these templates

2. In `ui/src/pages/Templates.tsx`:
   - Show starter templates at top of each category tab with "Reference" badge
   - Add "Clone" button to create editable copy from reference

3. In `ui/src/pages/TemplateDetail.tsx`:
   - Add read-only mode when viewing reference templates (hide edit buttons)

Verify with: `cargo build` and `cd ui && npm run type-check && npm run build`

Full starter template JSON examples are in the plan file: /Users/ryan/.claude/plans/whimsical-drifting-island.md
```
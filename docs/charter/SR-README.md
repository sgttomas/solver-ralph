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


## Resolved: UI Redesign Integration Task

**Status: COMPLETED**

The Chirality AI governance console UI has been integrated with the existing functional pages. All changes have been made and type-check passes.

### What Was Done

1. **main.tsx** - Restored `AuthProvider` wrapper around `RouterProvider`
2. **AppLayout.tsx** - Added auth check with redirect to `/callback` for unauthenticated users, loading state
3. **routes.tsx** - Connected all functional pages (Loops, LoopDetail, IterationDetail, CandidateDetail, Evidence, EvidenceDetail, Approvals, PromptLoop) and moved Callback outside the layout
4. **Sidebar.tsx** - Added Prompt Loop to navigation, reordered items for better UX
5. **Topbar.tsx** - Added user info display and logout button

### Testing Checklist

- [x] `npm run type-check` passes
- [ ] Open http://localhost:3001/ - should redirect to /overview
- [ ] Navigate to /loops - should show loop list from API
- [ ] Navigate to /prompt-loop - should show streaming prompt interface
- [ ] Navigate to /approvals - should show approval workflows

### Notes

- The existing pages use inline styles; they'll work but won't match the new design
- Future work: Port existing pages to use Card/Pill/Button primitives
- The `src/pages/PromptLoop.tsx` is the FUNCTIONAL one with SSE streaming; `src/screens/PromptLoopScreen.tsx` is just a wireframe (can be removed)
- Dev auth bypass is controlled by `VITE_DEV_AUTH_BYPASS=true` in environment
- Backend auth bypass uses `SR_AUTH_TEST_MODE=true`

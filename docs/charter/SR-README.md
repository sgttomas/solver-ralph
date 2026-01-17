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

Per `SR-PLAN-GAP-ANALYSIS.md`, the path to Milestone 1 completion:

| Plan | Scope | Key Deliverables | Status |
|------|-------|------------------|--------|
| SR-PLAN-V7 | Stabilization & Attachments | Tests, UX, `record.attachment` | **Complete** |
| SR-PLAN-V8 | Oracle Runner & Semantic Suites | D-24, D-25, D-27, D-39 | **Complete** |
| SR-PLAN-V9 | Semantic Worker & Branch 0 | D-23, D-41, D-36 | **Next** |

**Milestone 1 (MVP) projected completion:** After V9

---

## SR-PLAN-V8 Implementation Status (COMPLETE)

| Phase | Status | Description | Amendment |
|-------|--------|-------------|-----------|
| V8-1: Oracle Suite Registry | ✅ Complete | Port trait extracted, PostgreSQL adapter ready | A-4 |
| V8-2: Event-Driven Worker | ✅ Complete | Worker subscribes to `RunStarted` events | A-1 |
| V8-3: Integrity Checks | ✅ Complete | TAMPER/GAP/ENV_MISMATCH detection + events | — |
| V8-4: Core Oracle Suite | ✅ Complete | Build/unit/schema/lint oracles + container | — |
| V8-5: Semantic Oracles | ✅ Complete | Semantic oracle suite container + CLI command | A-3 |

**SR-PLAN-V8 COMPLETE. All phases implemented, tested, and documented.**

---

## Next Instance Prompt: Author SR-PLAN-V9

### Assignment

**Plan and author SR-PLAN-V9** — create the implementation plan document for Semantic Worker & Branch 0 completion. Do NOT implement code; focus on creating the plan document.

### Context

Per `SR-PLAN-GAP-ANALYSIS.md §4`:
- V9 scope: Semantic Worker & Branch 0 Completion
- Target deliverables: D-23, D-41, D-36
- Blocking items for Milestone 1: D-41 (Semantic worker), D-36 (Replay proof)

### What You Must Produce

Create `docs/planning/SR-PLAN-V9.md` following the established pattern from SR-PLAN-V8:

1. **Preamble** — YAML frontmatter with doc_id, refs, status: active
2. **Overview** — Scope, target deliverables, dependencies
3. **Phase breakdown** — V9-1 through V9-N with clear deliverables per phase
4. **Acceptance criteria** — Testable conditions for each phase
5. **Contract compliance** — How deliverables satisfy C-* contracts
6. **Appendix: Type inventory** — What exists vs. what needs creation

### Proposed Phases (refine as needed)

From SR-PLAN-GAP-ANALYSIS.md §4:

| Phase | Focus | Deliverables |
|-------|-------|--------------|
| V9-1 | Reference worker bridge | D-23 |
| V9-2 | Reference semantic worker | D-41 |
| V9-3 | Replayability demonstration | D-36 |
| V9-4 | Branch 0 acceptance verification | — |

### Research Before Authoring

Read these to understand what V9 must accomplish:

| Document/File | What You'll Learn |
|---------------|-------------------|
| `docs/planning/SR-PLAN-GAP-ANALYSIS.md` | Full deliverable status and V9 scope |
| `docs/planning/SR-PLAN-V8.md` | Template for plan document structure |
| `docs/platform/SR-PROCEDURE-KIT.md` | Branch 0 procedure definition |
| `docs/platform/SR-AGENT-WORKER-CONTRACT.md` | Worker interface requirements |
| `crates/sr-adapters/src/semantic_worker.rs` | Existing worker (if any) |

### Current State

- Branch: `solver-ralph-8` (continue)
- V8: ✅ COMPLETE
- V9: 📝 Plan authoring needed
- Milestone 1 completion: ~95% — awaiting V9

### On Completion

1. Verify SR-PLAN-V9.md follows established plan document pattern
2. Add SR-PLAN-V9 to SR-README roadmap table
3. Git commit: `docs: Author SR-PLAN-V9 for Semantic Worker & Branch 0`
4. Do NOT begin implementation — await approval

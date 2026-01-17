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
| SR-PLAN-V9 | Semantic Worker & Branch 0 | D-23, D-41, D-36 | **Authored — Pending Review** |

**Milestone 1 (MVP) projected completion:** After V9 implementation (~5-8 sessions post-review)

---

## SR-PLAN-V9 Status (COHERENCE APPROVED — CONSISTENCY REVIEW PENDING)

| Phase | Status | Description |
|-------|--------|-------------|
| V9-1: Semantic Worker Integration | 📝 Planned | Wire semantic worker to oracle runner, persist evidence |
| V9-2: E2E Flow Integration Test | 📝 Planned | Complete Branch 0 flow test (5 stages) |
| V9-3: Replayability Demonstration | 📝 Planned | Prove EventManager.rebuild() determinism (D-36) |
| V9-4: Branch 0 Acceptance | 📝 Planned | Document criteria satisfaction, human approval |

**SR-PLAN-V9 coherence review APPROVED.** Awaiting consistency evaluation before implementation.

---

## Next Instance Prompt: Consistency Evaluation of SR-PLAN-V9

### Assignment

**Perform a Consistency Evaluation of SR-PLAN-V9** — evaluate the plan's consistency with the canonical SR-* documentation on the basis of **ontology, epistemology, and semantics**. Do NOT implement code; focus on evaluation and producing a consistency assessment document.

### Context

SR-PLAN-V9 has passed coherence review (codebase verification). The coherence review confirmed:
- All infrastructure claims accurate (line counts, component status)
- Stub implementations correctly identified
- Integration points feasible with existing patterns
- Contract alignment verified (C-VER-1, C-EVT-7, SR-AGENT-WORKER-CONTRACT)

**Coherence ≠ Consistency.** Coherence verified the plan matches the *codebase*. Consistency verifies the plan matches the *canonical documentation* — that it uses terms correctly, makes supported claims, and follows the project's semantic conventions.

### Required Reading Before Evaluation

| Document | What to Evaluate |
|----------|------------------|
| `docs/planning/SR-PLAN-V9.md` | The plan under evaluation |
| `docs/reviews/SR-PLAN-V9-COHERENCE-REVIEW.md` | Prior review findings |
| `docs/platform/SR-CONTRACT.md` | Canonical definitions (§2), invariants (C-*) |
| `docs/platform/SR-SPEC.md` | Platform mechanics, API semantics |
| `docs/platform/SR-TYPES.md` | Type registry, naming conventions |
| `docs/platform/SR-EVENT-MANAGER.md` | Projection semantics |
| `docs/platform/SR-AGENT-WORKER-CONTRACT.md` | Worker behavioral contract |
| `docs/platform/SR-WORK-SURFACE.md` | Work Surface definitions |
| `docs/program/SR-DIRECTIVE.md` | Execution policy, stop triggers, gate rules |
| `docs/program/SR-PLAN.md` | Build plan instance, deliverable definitions |

### What You Must Produce

Create `docs/reviews/SR-PLAN-V9-CONSISTENCY-EVALUATION.md` containing:

#### 1. Ontological Consistency


#### 2. Epistemological Consistency


#### 3. Semantic Consistency


#### 4. Findings Summary

| Category | Status | Notes |
|----------|--------|-------|
| Ontological | PASS/FAIL/NOTES | ... |
| Epistemological | PASS/FAIL/NOTES | ... |
| Semantic | PASS/FAIL/NOTES | ... |




### On Completion

Create `docs/reviews/SR-PLAN-V9-CONSISTENCY-EVALUATION.md` with findings



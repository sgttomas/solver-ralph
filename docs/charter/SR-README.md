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
| SR-PLAN-V9 | Semantic Worker & Branch 0 | D-23, D-41, D-36 | **Reviews Complete — Implementation Active** |

**Milestone 1 (MVP) projected completion:** After V9 implementation (~5-8 sessions)

---

## SR-PLAN-V9 Status (APPROVED — IMPLEMENTATION ACTIVE)

| Phase | Status | Description |
|-------|--------|-------------|
| V9-1: Semantic Worker Integration | ✅ **COMPLETE** | Wire semantic worker to oracle runner, persist evidence |
| V9-2: E2E Flow Integration Test | 🔄 **ACTIVE** | Complete Branch 0 flow test (5 stages) |
| V9-3: Replayability Demonstration | 📝 Planned | Prove EventManager.rebuild() determinism (D-36) |
| V9-4: Branch 0 Acceptance | 📝 Planned | Document criteria satisfaction, human approval |

**Reviews Complete:**
- Coherence review: APPROVED (`docs/reviews/SR-PLAN-V9-COHERENCE-REVIEW.md`)
- Consistency evaluation: APPROVED WITH NOTES (`docs/reviews/SR-PLAN-V9-CONSISTENCY-EVALUATION.md`)

---

## Next Instance Prompt: V9-2 E2E Flow Integration Test

### Assignment

**Implement V9-2: End-to-End Flow Integration Test** — create an integration test demonstrating complete Branch 0 procedure flow. This is a CODE IMPLEMENTATION task.

### Quick Orientation

1. **Read the plan first:** `docs/planning/SR-PLAN-V9.md` §3.2 (Phase V9-2)
2. **Review V9-1 completion:** V9-1 wired the semantic worker to real oracle execution
3. **Understand test requirements:** Branch 0 acceptance requires demonstrating all 5 stages traversed

### What V9-2 Must Deliver

Per SR-PLAN-V9 §3.2:

| File | Action | Description |
|------|--------|-------------|
| `sr-api/tests/integration/branch_0_e2e_test.rs` | CREATE | Full E2E integration test |
| `sr-api/tests/fixtures/branch_0_intake.json` | CREATE | Test intake fixture |
| `sr-api/tests/fixtures/branch_0_work_surface.json` | CREATE | Test work surface fixture |

### Test Flow Requirements

The E2E test must demonstrate:
1. Loop created for problem-statement work unit
2. Iteration started with Work Surface ref set
3. Candidate intake bundle produced
4. Evidence Bundle from semantic oracle suite
5. Human portal approval recorded
6. Freeze baseline created
7. Stage progression: FRAME → OPTIONS → DRAFT → SEMANTIC_EVAL → FINAL

### Canonical References

| Document | Relevant Sections |
|----------|-------------------|
| SR-PLAN-V9 | §3.2 (V9-2 deliverables and test flow) |
| SR-PLAN | §4.1 (Branch 0 acceptance criteria) |
| SR-PROCEDURE-KIT | §2 (GENERIC-KNOWLEDGE-WORK stages) |

### Acceptance Criteria (from V9-2)

- [ ] E2E test creates work surface and loop
- [ ] E2E test progresses through all 5 stages (FRAME → OPTIONS → DRAFT → SEMANTIC_EVAL → FINAL)
- [ ] E2E test records portal approvals where required
- [ ] E2E test creates freeze baseline
- [ ] Evidence bundles exist for each stage transition
- [ ] Test passes in CI environment
- [ ] Test is documented in README

### On Completion

1. Commit changes with descriptive message
2. Update SR-README to mark V9-2 complete and set V9-3 as active
3. Push to branch `solver-ralph-9`



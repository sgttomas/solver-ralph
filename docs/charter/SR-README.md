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

## SR-PLAN-V9 Status (AUTHORED — PENDING REVIEW)

| Phase | Status | Description |
|-------|--------|-------------|
| V9-1: Semantic Worker Integration | 📝 Planned | Wire semantic worker to oracle runner, persist evidence |
| V9-2: E2E Flow Integration Test | 📝 Planned | Complete Branch 0 flow test (5 stages) |
| V9-3: Replayability Demonstration | 📝 Planned | Prove EventManager.rebuild() determinism (D-36) |
| V9-4: Branch 0 Acceptance | 📝 Planned | Document criteria satisfaction, human approval |

**SR-PLAN-V9 authored. Awaiting coherence review before implementation.**

---

## Next Instance Prompt: Coherence Review of SR-PLAN-V9

### Assignment

**Perform a Coherence Review of SR-PLAN-V9** — validate the plan against the actual codebase state and governing documents. Do NOT implement code; focus on review and producing a coherence assessment document.

### Context

SR-PLAN-V9 was authored based on codebase analysis showing that D-23 (Reference Worker Bridge) and D-41 (Semantic Worker) are already substantially implemented. The plan focuses on **integration and verification** rather than net-new component creation.

Key findings from plan authoring:
- `semantic_worker.rs` (~992 lines) exists with stub oracle invocation
- `worker.rs` (~835 lines) exists with full reference worker implementation
- `event_manager.rs` (~1720 lines) exists with eligibility computation
- The gap is **wiring**, not new code

### What You Must Produce

Create `docs/reviews/SR-PLAN-V9-COHERENCE-REVIEW.md` containing:

1. **Codebase Verification** — Confirm the plan's claims about existing infrastructure
   - Does `semantic_worker.rs` have the components described?
   - Are the stub implementations correctly identified?
   - Does `event_manager.rs` have `compute_eligible_set()` as claimed?

2. **Contract Alignment** — Verify plan phases satisfy SR-CONTRACT requirements
   - Does V9-1 address C-VER-1 (evidence-based verification)?
   - Does V9-3 address C-EVT-7 (rebuildable projections)?
   - Are SR-AGENT-WORKER-CONTRACT responsibilities covered?

3. **Gap Identification** — Identify any gaps between plan and reality
   - Missing dependencies not accounted for
   - Incorrect assumptions about existing code
   - Underestimated complexity

4. **Amendments (if needed)** — Propose amendments to SR-PLAN-V9
   - Follow the amendment pattern from SR-PLAN-V8 (A-1, A-2, etc.)

5. **Review Verdict** — One of:
   - **APPROVED** — Plan is coherent, proceed to implementation
   - **APPROVED WITH AMENDMENTS** — Plan is sound but requires amendments
   - **REVISE** — Significant issues require plan rewrite

### Review Checklist

| Check | Question |
|-------|----------|
| Infrastructure | Do the files and components described in §1.2 exist as claimed? |
| Stubs | Are the stub implementations (run_semantic_oracles, emit_evidence_bundle) correctly identified? |
| Integration Points | Are the V9-1 integration points feasible given current code structure? |
| Test Feasibility | Can the E2E test in V9-2 be implemented with existing API endpoints? |
| Replay | Does EventManager have the foundation for compute_state_hash()? |
| Effort | Are the effort estimates (5-8 sessions) realistic? |

### Research Before Reviewing

| Document/File | What to Verify |
|---------------|----------------|
| `docs/planning/SR-PLAN-V9.md` | The plan being reviewed |
| `crates/sr-adapters/src/semantic_worker.rs` | Verify §1.2 claims |
| `crates/sr-adapters/src/worker.rs` | Verify D-23 status |
| `crates/sr-adapters/src/event_manager.rs` | Verify D-40 claims |
| `docs/platform/SR-CONTRACT.md` | Contract compliance |
| `docs/platform/SR-AGENT-WORKER-CONTRACT.md` | Worker responsibilities |

### Current State

- Branch: `solver-ralph-8` (continue)
- V8: ✅ COMPLETE
- V9: 📝 AUTHORED — pending coherence review
- Milestone 1 completion: ~95% — awaiting V9 review and implementation

### On Completion

1. Create `docs/reviews/SR-PLAN-V9-COHERENCE-REVIEW.md` with findings
2. If amendments needed, document them in the review
3. Git commit: `docs: Coherence review of SR-PLAN-V9`
4. If APPROVED, next instance can begin V9-1 implementation

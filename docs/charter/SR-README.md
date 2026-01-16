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

| doc_id | Status | Purpose |
|--------|--------|---------|
| SR-PLAN-GAP-ANALYSIS | **living** | Deliverable status tracking & roadmap |
| SR-PLAN-V7 | **complete** | MVP Stabilization & Attachment Foundation |
| SR-PLAN-V6 | **complete** | UI Integration for MVP Workflow (V6-1, V6-2, V6-3 complete) |
| SR-PLAN-V5 | **complete** | Semantic Ralph Loop End-to-End Integration (Phases 5a-5d) |
| SR-PLAN-V4 | **complete** | Work Surface Composition (Phase 4) — All phases complete |
| SR-PLAN-V3 | **complete** | Intakes & References implementation (Phases 0-3) |
| SR-PLAN-V2 | superseded | Intakes & References draft (10 unresolved issues) |

### Roadmap (from Gap Analysis)

Per `SR-PLAN-GAP-ANALYSIS.md`, the path to Milestone 1 completion:

| Plan | Scope | Key Deliverables | Status |
|------|-------|------------------|--------|
| SR-PLAN-V7 | Stabilization & Attachments | Tests, UX, `record.attachment` | **Complete** |
| SR-PLAN-V8 | Oracle Runner & Semantic Suites | D-24, D-25, D-27, D-39 | **Under Review** |
| SR-PLAN-V9 | Semantic Worker & Branch 0 | D-23, D-41, D-36 | Proposed |

**Milestone 1 (MVP) projected completion:** After V9

---

## SR-PLAN-V8 Implementation Status

| Phase | Status | Description |
|-------|--------|-------------|
| V8-1: Oracle Suite Registry | ⏳ Pending | PostgreSQL storage, suite management API |
| V8-2: API Integration | ⏳ Pending | Wire PodmanOracleRunner to `/runs` endpoint |
| V8-3: Integrity Checks | ⏳ Pending | TAMPER/GAP/FLAKE/ENV_MISMATCH detection |
| V8-4: Core Oracle Suite | ⏳ Pending | Build/unit/schema/lint oracles + container |
| V8-5: Semantic Oracles | ⏳ Pending | sr.semantic_eval.v1, residual/coverage artifacts |

**SR-PLAN-V8 is now under review prior to implementation.**

---

## Previous Session Summary (V8 Authoring)

**Session Goal:** Author SR-PLAN-V8 — Oracle Runner & Semantic Suite Foundation

### What Was Accomplished

1. **Read and analyzed canonical documents:**
   - SR-README (assignment context)
   - SR-PLAN-GAP-ANALYSIS (deliverable status, D-24/D-25/D-27/D-39 gaps)
   - SR-SEMANTIC-ORACLE-SPEC (oracle interface requirements, sr.semantic_eval.v1 schema)
   - SR-CONTRACT §6 (C-OR-1..7 oracle integrity), §7 (C-EVID-1..6 evidence)
   - Existing `oracle_runner.rs` (~1027 lines of partial implementation)

2. **Analyzed existing infrastructure:**
   - `PodmanOracleRunner<E>` already implemented with Podman command building
   - Evidence manifest builder exists
   - Test mode for mock execution exists
   - Gaps identified: suite registry (in-memory only), candidate path (placeholder), integrity detection (missing)

3. **Authored SR-PLAN-V8 with 5 phases:**

   | Phase | Focus | Deliverables |
   |-------|-------|--------------|
   | V8-1 | Oracle Suite Registry | DB schema, API endpoints, port trait |
   | V8-2 | API Integration | Wire runner to `/runs`, candidate workspace |
   | V8-3 | Integrity Checks | TAMPER/GAP/FLAKE/ENV_MISMATCH detection |
   | V8-4 | Core Oracle Suite | Container image, 4 oracles, suite.json |
   | V8-5 | Semantic Oracles | sr.semantic_eval.v1, residual/coverage/violations |

4. **Included contract compliance matrix:**
   - C-OR-1 through C-OR-7 mapped to implementation phases
   - C-EVID-1, C-EVID-2 addressed via existing infrastructure
   - C-VER-1 satisfied via semantic oracle evidence

### Files Created

| File | Lines | Purpose |
|------|-------|---------|
| `docs/planning/SR-PLAN-V8.md` | ~700 | Oracle Runner & Semantic Suite Foundation plan |

### Key Architectural Decisions

| Decision | Rationale |
|----------|-----------|
| Registry before API | C-OR-2 requires suite pinning; need persistent storage first |
| Integrity as separate phase | C-OR-7 requires all conditions halt and escalate; deserves focused implementation |
| Core oracles before semantic | Validate packaging/execution before adding semantic complexity |
| 5 phases (~7-9 sessions total) | Each phase completable in 1-2 sessions; clear acceptance criteria |

### Estimated Effort

| Phase | Sessions |
|-------|----------|
| V8-1 | 1 |
| V8-2 | 1-2 |
| V8-3 | 1-2 |
| V8-4 | 2 |
| V8-5 | 2 |
| **Total** | **7-9** |

---

## Next Instance Prompt: Coherence Review of SR-PLAN-V8

### Context

SR-PLAN-V8 has been authored but not yet validated against the actual codebase. Before implementation begins, we need to verify that the plan's assumptions about existing infrastructure, type definitions, and integration points are accurate.

### Current State

- Branch: `solver-ralph-8` (V7 complete, V8 plan authored)
- SR-PLAN-V7: **Complete** (all phases)
- SR-PLAN-V8: **Authored** (pending coherence review)

### Assignment

**Evaluate SR-PLAN-V8 for coherence with the existing codebase**

The plan makes several assumptions about what exists and how components fit together. Your task is to verify these assumptions by examining the actual code and identifying any gaps, conflicts, or necessary adjustments.

### Key Questions to Answer

1. **Oracle Runner Integration:**
   - Does `oracle_runner.rs` actually implement the `OracleRunner` trait from `sr-ports`?
   - What is the current state of the `/runs` endpoint in `runs.rs`? Does it already call the oracle runner, or is it a stub?
   - How does the existing `EvidenceManifestBuilder` work, and does V8-2's proposed integration align with it?

2. **Type Consistency:**
   - Do the proposed domain types (`OracleSuite`, `OracleSuiteStatus`) conflict with or duplicate anything in `sr-domain`?
   - Are there existing types in `oracle_runner.rs` (like `OracleSuiteDefinition`) that should be reused vs. replaced?

3. **Database Patterns:**
   - What patterns do existing migrations follow? (naming, structure, conventions)
   - How do other PostgreSQL adapters implement their repository traits?

4. **API Patterns:**
   - How are other handlers structured in `sr-api`? What's the standard for request/response types?
   - What's the route registration pattern in `main.rs`?

5. **Evidence Store Integration:**
   - How does the existing `MinioEvidenceStore` work with `PodmanOracleRunner`?
   - Is there a mismatch between V8's proposed flow and what's already wired?

### Deliverable

Produce a coherence assessment with one of these verdicts:
- **COHERENT**: Plan aligns with codebase; proceed to implementation
- **COHERENT WITH AMENDMENTS**: Plan mostly aligns but needs specific corrections (list them)
- **REQUIRES REVISION**: Significant misalignment; plan needs rewriting before implementation

### First Actions

1. Read `crates/sr-ports/src/lib.rs` to understand the port trait definitions
2. Read `crates/sr-api/src/handlers/runs.rs` to understand current `/runs` implementation
3. Read `crates/sr-adapters/src/evidence.rs` to understand evidence manifest building
4. Examine existing migrations in `migrations/` for naming conventions
5. Compare V8's proposed types against existing types in `sr-domain` and `oracle_runner.rs`


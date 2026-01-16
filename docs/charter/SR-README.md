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

## SR-PLAN-V8 Implementation Status (REVIEWED 2026-01-16)

| Phase | Status | Description | Amendment |
|-------|--------|-------------|-----------|
| V8-1: Oracle Suite Registry | 🎯 Ready | Extract port trait, add PostgreSQL persistence | A-4 |
| V8-2: Event-Driven Worker | ⏳ Pending | Worker subscribes to `RunStarted` events | A-1 |
| V8-3: Integrity Checks | ⏳ Pending | TAMPER/GAP/FLAKE/ENV_MISMATCH detection | — |
| V8-4: Core Oracle Suite | ⏳ Pending | Build/unit/schema/lint oracles + container | — |
| V8-5: Semantic Oracles | ⏳ Pending | Use existing types, focus on container packaging | A-3 |

**SR-PLAN-V8 has passed coherence assessment AND philosophical consistency review. Ready for implementation.**

### Amendments Applied

| ID | Issue | Resolution |
|----|-------|------------|
| A-1 | V8-2 assumed direct API call | Use Event-Driven Worker pattern |
| A-2 | Type relationship unclear | `OracleSuiteDefinition` = config, `OracleSuiteRecord` = stored entity |
| A-3 | Semantic types already exist | Use `sr-domain/src/semantic_oracle.rs` |
| A-4 | Registry partially implemented | Extract port from existing `OracleSuiteRegistry` |

**Revised Effort:** 7-10 sessions (was 7-9)

---

## Previous Session Summary (V8 Coherence Assessment)

**Session Goal:** Review SR-PLAN-V8 for coherence with existing codebase

### What Was Accomplished

1. **Performed codebase coherence review:**
   - Verified existing `OracleRunner` trait and `PodmanOracleRunner` implementation
   - Discovered `POST /runs` uses event sourcing (creates `RunStarted` event, does NOT call oracle runner)
   - Found existing `OracleSuiteRegistry` struct with 6 working API endpoints
   - Found comprehensive semantic types in `sr-domain/src/semantic_oracle.rs` (~1024 lines)

2. **Identified 4 required amendments:**

   | Amendment | Finding |
   |-----------|---------|
   | A-1 | V8-2 must use Event-Driven Worker pattern (not direct API call) |
   | A-2 | Clarify `OracleSuiteDefinition` vs `OracleSuiteRecord` relationship |
   | A-3 | V8-5 should use existing semantic types, not create new ones |
   | A-4 | V8-1 should extract port trait from existing registry implementation |

3. **Revised SR-PLAN-V8:**
   - Added Amendments Summary section
   - Rewrote V8-2 for event-driven architecture
   - Updated V8-5 to reference existing types
   - Revised effort estimates (7-10 sessions)
   - Added critical files appendix

### Verdict

**COHERENT WITH AMENDMENTS** — Plan is sound but required 4 specific corrections before implementation.

### Files Modified

| File | Changes |
|------|---------|
| `docs/planning/SR-PLAN-V8.md` | Added amendments, rewrote V8-2, updated V8-5, revised estimates |

### Estimated Effort (Revised)

| Phase | Sessions | Change |
|-------|----------|--------|
| V8-1 | 1 | — |
| V8-2 | 2-3 | +1 (event-driven complexity) |
| V8-3 | 1-2 | — |
| V8-4 | 2 | — |
| V8-5 | 1-2 | -1 (types exist) |
| **Total** | **7-10** | +1 |

---

## Next Instance Prompt: Implement SR-PLAN-V8 Phase V8-1

### Assignment

Implement **Phase V8-1: Oracle Suite Registry** — extract a port trait from the existing in-memory registry and add PostgreSQL persistence.

### Where to Look

| Document | What You'll Find |
|----------|------------------|
| `docs/planning/SR-PLAN-V8.md` §3 Phase V8-1 | Full specification: schema, trait signature, file list, acceptance criteria |
| `docs/reviews/SR-PLAN-V8-CONSISTENCY.md` | Philosophical consistency review (verdict: CONSISTENT) |
| `crates/sr-adapters/src/oracle_suite.rs` | Existing in-memory implementation to extract from |
| `crates/sr-ports/src/lib.rs` | Existing port trait patterns to follow |

### Current State

- Branch: `solver-ralph-8`
- SR-PLAN-V8: Ready for implementation (coherence ✅, consistency ✅)
- Phase V8-1: 🎯 Your assignment

### On Completion

1. Run tests: `cargo test --package sr-api && cargo test --package sr-adapters`
2. Git commit
3. Update V8-1 status in this file to ✅ Complete


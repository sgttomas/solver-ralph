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

### Amendments Applied

| ID | Issue | Resolution |
|----|-------|------------|
| A-1 | V8-2 assumed direct API call | Use Event-Driven Worker pattern |
| A-2 | Type relationship unclear | `OracleSuiteDefinition` = config, `OracleSuiteRecord` = stored entity |
| A-3 | Semantic types already exist | Use `sr-domain/src/semantic_oracle.rs` |
| A-4 | Registry partially implemented | Extract port from existing `OracleSuiteRegistry` |

**Revised Effort:** 7-10 sessions (was 7-9)

---

## Previous Session Summary (V8-5 Completion + Doc Updates)

**Session Goal:** Complete SR-PLAN-V8 Phase V8-5 and update canonical documents

### What Was Accomplished

1. **Implemented V8-5: Semantic Oracle Suite**
   - Added `semantic-eval` command to `sr-oracles` CLI
   - Created `oracle-suites/semantic-v1/` with Dockerfile, suite.json, oracle script
   - Bundled `intake-admissibility.json` semantic set definition
   - Added 7 integration tests to `sr-adapters/src/oracle_runner.rs`
   - All tests passed: sr-adapters (148), sr-api (41), sr-oracles (27)

2. **Updated Canonical Documents**
   - **SR-PLAN-GAP-ANALYSIS.md**: V8 complete, D-24/25/27/39 complete, Branch 0 criteria updated
   - **SR-PLAN-V8.md**: Status to Complete, added Appendix E completion summary
   - **SR-SPEC.md**: Added §1.9.1.2 documenting `sr.semantic_eval.v1` schema
   - **SR-CHANGE.md**: Added version 0.9 documenting V8-5 + doc updates

### Commits

| Hash | Message |
|------|---------|
| `f52857c` | V8-5: Complete Semantic Oracle Suite Container |
| `0e7681d` | docs: Update canonical SR-* documents for V8-5 completion |

### Key Implementation Decisions

- Reused existing `IntakeAdmissibilityRunner` instead of creating new crate
- Added `semantic-eval` command to existing `sr-oracles` crate (cleaner than new binary)
- No `Waived` variant added to `DecisionStatus` (waivers handled at gate level per C-INT-2)
- Amendment A-3 followed: used existing types from `sr-domain/src/semantic_oracle.rs`

### SR-PLAN-V8 Status

**COMPLETE** — All 5 phases implemented, tested, documented, and committed.

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

---

## V8-4 Session Archive

**Session Goal:** Implement SR-PLAN-V8 Phase V8-4 — Core Oracle Suite Container

### What Was Accomplished (V8-4)

1. **Created core oracle suite directory** (`oracle-suites/core-v1/`):
   - Complete directory structure with Dockerfile, suite.json, and oracle scripts
   - Suite ID: `suite:sr-core-v1`
   - OCI image: `ghcr.io/solver-ralph/oracle-suite-core:v1`

2. **Created Dockerfile** (`oracle-suites/core-v1/Dockerfile`):
   - Base image: `rust:1.83-bookworm`
   - Includes Rust toolchain with clippy/rustfmt
   - Node.js, npm, Python3, jq for multi-language support
   - Oracle scripts installed at `/oracles/`

3. **Created suite.json manifest**:
   - Environment constraints: runsc, network:disabled, workspace_readonly:true
   - Four oracles defined with timeouts, expected outputs, classifications
   - Contract refs: C-OR-1, C-OR-3, C-EVID-1

4. **Created four oracle scripts** (`oracle-suites/core-v1/oracles/`):
   - `build.sh` — Detects Rust/Node.js/Make, runs build, outputs `sr.oracle_result.v1`
   - `unit-tests.sh` — Runs cargo test or npm test, parses pass/fail counts
   - `schema-validation.sh` — Validates JSON files, JSON Schema files, tsconfig.json
   - `lint.sh` — Runs clippy (Rust) or eslint (Node.js), advisory classification

5. **Added integration tests** (`crates/sr-adapters/src/oracle_runner.rs`):
   - `test_core_suite_v1_definition_parsing` — Validates suite.json structure
   - `test_core_suite_v1_oracle_definitions` — Verifies all 4 oracles defined correctly
   - `test_core_suite_v1_metadata` — Checks metadata fields
   - `test_core_suite_v1_required_oracles_count` — 3 required + 1 advisory
   - `test_core_suite_v1_gap_detection_all_required` — GAP detection with all present
   - `test_core_suite_v1_gap_detection_missing_required` — GAP detection with missing oracle

6. **Created README documentation** (`oracle-suites/core-v1/README.md`):
   - Oracle descriptions and output schemas
   - Environment constraints documentation
   - Container build and registration instructions
   - Contract compliance matrix

7. **Updated canonical documents**:
   - SR-SPEC §1.9.1.1: Added `sr.oracle_result.v1` schema documentation
   - SR-CHANGE: Added version 0.8 documenting V8-4 implementation

### Test Results

| Package | Result |
|---------|--------|
| sr-adapters | ✅ 141 passed (1 ignored) |
| sr-api | ✅ 41 passed |

### Files Created/Modified

| File | Action |
|------|--------|
| `oracle-suites/core-v1/Dockerfile` | CREATED — Container image definition |
| `oracle-suites/core-v1/suite.json` | CREATED — Suite definition manifest |
| `oracle-suites/core-v1/oracles/build.sh` | CREATED — Build check oracle |
| `oracle-suites/core-v1/oracles/unit-tests.sh` | CREATED — Unit test oracle |
| `oracle-suites/core-v1/oracles/schema-validation.sh` | CREATED — Schema validation oracle |
| `oracle-suites/core-v1/oracles/lint.sh` | CREATED — Lint check oracle (advisory) |
| `oracle-suites/core-v1/README.md` | CREATED — Suite documentation |
| `crates/sr-adapters/src/oracle_runner.rs` | MODIFIED — Added 6 V8-4 integration tests |
| `docs/platform/SR-SPEC.md` | MODIFIED — Added §1.9.1.1 `sr.oracle_result.v1` schema |
| `docs/build-governance/SR-CHANGE.md` | MODIFIED — Added version 0.8 |

### Commits

| Hash | Message |
|------|---------|
| `46e657a` | V8-4: Complete Core Oracle Suite Container |
| `1bfb431` | docs: Add sr.oracle_result.v1 schema to SR-SPEC §1.9.1.1 |

### Contract Compliance

| Contract | Requirement | Implementation |
|----------|-------------|----------------|
| C-OR-1 | Required oracles deterministic | Build/unit/schema use deterministic tooling |
| C-OR-3 | Environment constraints declared | suite.json declares runsc, network:disabled |
| C-EVID-1 | Evidence bundle manifest | All outputs conform to sr.oracle_result.v1 |

### Implementation Notes

- Oracle scripts auto-detect project type (Rust vs Node.js vs Make)
- All outputs follow `sr.oracle_result.v1` schema with status, duration, summary, details
- `oracle:lint` is advisory classification — missing it does not trigger GAP
- Suite hash is a placeholder; computed at registration time
- Integration tests use `include_str!()` to load suite.json at compile time



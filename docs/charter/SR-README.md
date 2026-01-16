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
| SR-PLAN-V8 | Oracle Runner & Semantic Suites | D-24, D-25, D-27, D-39 | **In Progress** (V8-1 ✅, V8-2 ✅) |
| SR-PLAN-V9 | Semantic Worker & Branch 0 | D-23, D-41, D-36 | Proposed |

**Milestone 1 (MVP) projected completion:** After V9

---

## SR-PLAN-V8 Implementation Status (UPDATED 2026-01-16)

| Phase | Status | Description | Amendment |
|-------|--------|-------------|-----------|
| V8-1: Oracle Suite Registry | ✅ Complete | Port trait extracted, PostgreSQL adapter ready | A-4 |
| V8-2: Event-Driven Worker | ✅ Complete | Worker subscribes to `RunStarted` events | A-1 |
| V8-3: Integrity Checks | ✅ Complete | TAMPER/GAP/ENV_MISMATCH detection + events | — |
| V8-4: Core Oracle Suite | ✅ Complete | Build/unit/schema/lint oracles + container | — |
| V8-5: Semantic Oracles | 🎯 Next | Use existing types, focus on container packaging | A-3 |

**SR-PLAN-V8 has passed coherence assessment AND philosophical consistency review. V8-1 through V8-4 complete. V8-5 ready.**

### Amendments Applied

| ID | Issue | Resolution |
|----|-------|------------|
| A-1 | V8-2 assumed direct API call | Use Event-Driven Worker pattern |
| A-2 | Type relationship unclear | `OracleSuiteDefinition` = config, `OracleSuiteRecord` = stored entity |
| A-3 | Semantic types already exist | Use `sr-domain/src/semantic_oracle.rs` |
| A-4 | Registry partially implemented | Extract port from existing `OracleSuiteRegistry` |

**Revised Effort:** 7-10 sessions (was 7-9)

---

## Previous Session Summary (V8-4 Implementation)

**Session Goal:** Implement SR-PLAN-V8 Phase V8-4 — Core Oracle Suite Container

### What Was Accomplished

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

---

## Next Instance Prompt: Implement SR-PLAN-V8 Phase V8-5

### Assignment

Implement **Phase V8-5: Semantic Oracle Integration** — create the semantic oracle suite container using existing type definitions from `sr-domain/src/semantic_oracle.rs`.

### Key Concept

V8-1 through V8-4 built the complete core oracle infrastructure. V8-5 creates **semantic oracles** that evaluate candidates against meaning matrices, producing residual/coverage/violations artifacts.

**IMPORTANT (Amendment A-3):** The semantic oracle types already exist in `sr-domain/src/semantic_oracle.rs` (~1024 lines). Do NOT recreate them. Focus on:
1. Container packaging
2. Oracle suite definition
3. Integration with existing types

### Existing Types (DO NOT RECREATE)

| Type | Location | Purpose |
|------|----------|---------|
| `SemanticEvalResult` | `semantic_oracle.rs:178-192` | Main evaluation result |
| `SemanticMetrics` | `semantic_oracle.rs:195-201` | Residual, coverage, violations |
| `EvalDecision` | `semantic_oracle.rs:204-210` | Pass/Fail decision |
| `DecisionStatus` | `semantic_oracle.rs:156-160` | Pass/Fail/Indeterminate |
| `ResidualReport` | `semantic_oracle.rs:263-273` | Output artifact |
| `CoverageReport` | `semantic_oracle.rs:276-286` | Output artifact |
| `ViolationsReport` | `semantic_oracle.rs:289-298` | Output artifact |

### What V8-5 Must Implement

1. **Semantic oracle suite container** (`oracle-suites/semantic-v1/`):
   - Dockerfile for semantic evaluation
   - Suite definition with `semantic_set_binding`
   - Oracle script that produces residual/coverage/violations

2. **Suite definition** (`suite:sr-semantic-v1`):
   - Suite ID with `semantic_set_binding` incorporated into hash
   - Environment constraints (runsc, network:disabled)
   - Single oracle: `oracle:semantic-eval`

3. **Required output artifacts** (per SR-SEMANTIC-ORACLE-SPEC §3):
   - `reports/semantic/residual.json` — Residual vector and norm
   - `reports/semantic/coverage.json` — Axis coverage metrics
   - `reports/semantic/violations.json` — Constraint violations
   - `reports/semantic/eval.json` — Main evaluation result (`sr.semantic_eval.v1`)

4. **Optional type modification**:
   - Add `Waived` variant to `DecisionStatus` if needed

### Where to Look

| Document/File | What You'll Find |
|---------------|------------------|
| `docs/planning/SR-PLAN-V8.md` §V8-5 | Full specification |
| `docs/platform/SR-SEMANTIC-ORACLE-SPEC.md` | Semantic oracle interface |
| `crates/sr-domain/src/semantic_oracle.rs` | **Existing types** (use these!) |
| `crates/sr-adapters/src/semantic_suite.rs` | Semantic suite factory |
| `oracle-suites/core-v1/` | Reference implementation pattern |

### Current State

- Branch: `solver-ralph-8`
- V8-1: ✅ Complete (registry)
- V8-2: ✅ Complete (event-driven worker)
- V8-3: ✅ Complete (integrity detection)
- V8-4: ✅ Complete (core oracle suite)
- V8-5: 🎯 Your assignment
- Estimated effort: 1-2 sessions (reduced — types exist)

### Acceptance Criteria (from SR-PLAN-V8)

- [ ] Semantic oracle suite Dockerfile builds successfully
- [ ] Suite definition conforms to SR-SEMANTIC-ORACLE-SPEC
- [ ] `semantic_set_hash` incorporated into `oracle_suite_hash` (§2)
- [ ] All 3 required artifacts produced: residual, coverage, violations
- [ ] Outputs conform to existing `sr-domain` type definitions
- [ ] `Waived` variant added to `DecisionStatus` if needed
- [ ] Integration test validates semantic oracle execution
- [ ] Evidence bundle contains semantic artifacts
- [ ] Suite registers via existing API endpoint
- [ ] `cargo test --package sr-adapters` passes

### On Completion

1. Run tests: `cargo test --package sr-adapters && cargo test --package sr-api`
2. Git commit
3. Update SR-README: Mark V8-5 complete, summarize V8 completion
4. SR-PLAN-V8 will be COMPLETE after V8-5


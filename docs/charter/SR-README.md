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
| V8-4: Core Oracle Suite | 🎯 Next | Build/unit/schema/lint oracles + container | — |
| V8-5: Semantic Oracles | ⏳ Pending | Use existing types, focus on container packaging | A-3 |

**SR-PLAN-V8 has passed coherence assessment AND philosophical consistency review. V8-1, V8-2, and V8-3 complete. V8-4 ready.**

### Amendments Applied

| ID | Issue | Resolution |
|----|-------|------------|
| A-1 | V8-2 assumed direct API call | Use Event-Driven Worker pattern |
| A-2 | Type relationship unclear | `OracleSuiteDefinition` = config, `OracleSuiteRecord` = stored entity |
| A-3 | Semantic types already exist | Use `sr-domain/src/semantic_oracle.rs` |
| A-4 | Registry partially implemented | Extract port from existing `OracleSuiteRegistry` |

**Revised Effort:** 7-10 sessions (was 7-9)

---

## Previous Session Summary (V8-3 Implementation)

**Session Goal:** Implement SR-PLAN-V8 Phase V8-3 — Oracle Integrity Condition Detection

### What Was Accomplished

1. **Created integrity module** (`crates/sr-domain/src/integrity.rs`):
   - `IntegrityCondition` enum with four variants:
     - `OracleTamper { expected_hash, actual_hash, suite_id }` (C-OR-2)
     - `OracleGap { missing_oracles, suite_id }` (C-OR-4)
     - `OracleFlake { oracle_id, run_1_hash, run_2_hash, description }` (C-OR-5)
     - `OracleEnvMismatch { constraint, expected, actual }` (C-OR-3)
   - Helper methods: `condition_code()`, `severity()`, `requires_escalation()`, `contract_ref()`, `message()`
   - `Severity` enum (Blocking)
   - `IntegrityCheckResult` for aggregating conditions
   - `IntegrityError` wrapper with Display impl
   - Comprehensive unit tests for all condition types

2. **Added IntegrityViolationDetected event** (`crates/sr-domain/src/events.rs`):
   - Event struct with run_id, candidate_id, suite_id, condition, detected_at, requires_escalation
   - Added to `EventType` enum
   - `new()` constructor that sets requires_escalation=true per C-OR-7

3. **Added detection functions** (`crates/sr-adapters/src/oracle_runner.rs`):
   - `validate_environment()` — Detects ENV_MISMATCH before oracle execution
   - `check_for_gaps()` — Detects GAP after oracle execution (missing required results)
   - Integrated into `execute_suite()` flow
   - 6 new integration tests covering all detection scenarios

4. **Updated oracle worker** (`crates/sr-adapters/src/oracle_worker.rs`):
   - `emit_integrity_violation()` method for event emission
   - TAMPER detection now emits `IntegrityViolationDetected` event per C-OR-7

5. **Added IntegrityViolation error variant** (`crates/sr-ports/src/lib.rs`):
   - `OracleRunnerError::IntegrityViolation { condition }` for structured error handling

6. **Updated canonical documents**:
   - SR-SPEC Appendix A: Added `IntegrityViolationDetected` event with full payload schema
   - SR-CHANGE: Added version 0.7 documenting the changes

### Test Results

| Package | Result |
|---------|--------|
| sr-domain | ✅ 128 passed |
| sr-adapters | ✅ 135 passed (1 ignored) |
| sr-api | ✅ 41 passed |

### Files Created/Modified

| File | Action |
|------|--------|
| `crates/sr-domain/src/integrity.rs` | CREATED — Rich integrity condition types |
| `crates/sr-domain/src/lib.rs` | MODIFIED — Export integrity module |
| `crates/sr-domain/src/events.rs` | MODIFIED — Added IntegrityViolationDetected event |
| `crates/sr-ports/src/lib.rs` | MODIFIED — Added IntegrityViolation error variant |
| `crates/sr-adapters/src/oracle_runner.rs` | MODIFIED — Added detection functions + tests |
| `crates/sr-adapters/src/oracle_worker.rs` | MODIFIED — Emit integrity violation events |
| `docs/platform/SR-SPEC.md` | MODIFIED — Added IntegrityViolationDetected to Appendix A |
| `docs/build-governance/SR-CHANGE.md` | MODIFIED — Added version 0.7 |

### Contract Compliance

| Contract | Requirement | Implementation |
|----------|-------------|----------------|
| C-OR-2 | Suite pinning, TAMPER detection | Enhanced with event emission |
| C-OR-3 | Environment constraints | ENV_MISMATCH detection before execution |
| C-OR-4 | Oracle gaps blocking | GAP detection after execution |
| C-OR-5 | Flake is stop-the-line | Type defined; full detection deferred to V8-4 |
| C-OR-7 | Integrity conditions halt and escalate | All conditions halt, emit events |

### Implementation Notes

- Detection functions are module-level for testability (not struct methods)
- GAP detection considers Error status as "missing" (only Pass/Fail count as results)
- Advisory oracles are excluded from GAP checks (only Required oracles)
- FLAKE detection requires repeat-run infrastructure; type defined but detection deferred
- All integrity conditions set `requires_escalation: true` per C-OR-7

---

## Next Instance Prompt: Implement SR-PLAN-V8 Phase V8-4

### Assignment

Implement **Phase V8-4: Core Oracle Suite Container** — create the actual oracle container with build/unit/schema/lint oracles that produce evidence.

### Key Concept

V8-1 through V8-3 built the infrastructure (registry, worker, integrity checks). V8-4 creates the actual oracles that run inside containers:

| Oracle | Purpose | Expected Output |
|--------|---------|-----------------|
| `oracle:build` | Verify code compiles | Build report JSON |
| `oracle:unit` | Run unit tests | Test results JSON |
| `oracle:schema` | Validate schemas | Schema validation report |
| `oracle:lint` | Check code quality | Lint report JSON |

### Where to Look

| Document/File | What You'll Find |
|---------------|------------------|
| `docs/planning/SR-PLAN-V8.md` §4 Phase V8-4 | Full specification, container structure |
| `docs/platform/SR-CONTRACT.md` §6 | C-OR-1..7 oracle requirements |
| `crates/sr-adapters/src/oracle_suite.rs` | `OracleSuiteDefinition`, `OracleDefinition` types |
| `crates/sr-adapters/src/oracle_runner.rs` | `PodmanOracleRunner` that executes suites |
| `crates/sr-adapters/src/evidence.rs` | Evidence manifest and artifact types |

### What V8-4 Must Implement

1. **Oracle container** (Dockerfile + scripts):
   - Base image with build tools (Rust toolchain)
   - Entry point that runs oracles based on command
   - Output capture to `/scratch/reports/`
   - JSON-formatted evidence output

2. **Oracle suite definition** (`suite:SR-SUITE-CORE`):
   - Register in `OracleSuiteRegistry` with proper hash
   - Define environment constraints (runsc, network:disabled, etc.)
   - Specify expected outputs for each oracle

3. **Evidence artifact schemas**:
   - Build report schema
   - Test results schema
   - Lint report schema

4. **Integration test**:
   - End-to-end test that runs suite against a test candidate
   - Verifies evidence bundle is produced

### Current State

- Branch: `solver-ralph-8`
- V8-1: ✅ Complete (registry)
- V8-2: ✅ Complete (event-driven worker)
- V8-3: ✅ Complete (integrity detection)
- V8-4: 🎯 Your assignment
- Estimated effort: 2-3 sessions

### Acceptance Criteria (from SR-PLAN-V8)

- [ ] Container image builds successfully
- [ ] `oracle:build` produces build report evidence
- [ ] `oracle:unit` produces test results evidence
- [ ] `oracle:lint` produces lint report evidence
- [ ] Suite registered with correct hash
- [ ] Environment constraints enforced (runsc, network:disabled)
- [ ] Integration test passes end-to-end
- [ ] `cargo test --package sr-adapters` passes

### On Completion

1. Run tests: `cargo test --package sr-adapters && cargo test --package sr-api`
2. Git commit
3. Update SR-README: Mark V8-4 complete, write V8-5 prompt


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
| V8-3: Integrity Checks | 🎯 Next | TAMPER/GAP/FLAKE/ENV_MISMATCH detection | — |
| V8-4: Core Oracle Suite | ⏳ Pending | Build/unit/schema/lint oracles + container | — |
| V8-5: Semantic Oracles | ⏳ Pending | Use existing types, focus on container packaging | A-3 |

**SR-PLAN-V8 has passed coherence assessment AND philosophical consistency review. V8-1 and V8-2 complete, V8-3 ready.**

### Amendments Applied

| ID | Issue | Resolution |
|----|-------|------------|
| A-1 | V8-2 assumed direct API call | Use Event-Driven Worker pattern |
| A-2 | Type relationship unclear | `OracleSuiteDefinition` = config, `OracleSuiteRecord` = stored entity |
| A-3 | Semantic types already exist | Use `sr-domain/src/semantic_oracle.rs` |
| A-4 | Registry partially implemented | Extract port from existing `OracleSuiteRegistry` |

**Revised Effort:** 7-10 sessions (was 7-9)

---

## Previous Session Summary (V8-2 Implementation)

**Session Goal:** Implement SR-PLAN-V8 Phase V8-2 — Event-Driven Oracle Worker

### What Was Accomplished

1. **Added domain events to sr-domain** (`crates/sr-domain/src/events.rs`):
   - `OracleExecutionStarted` event payload struct
   - `OracleExecutionCompleted` event payload struct
   - `OracleExecutionStatus` enum (Pass, Fail, Error)
   - Added event types to `EventType` enum

2. **Created candidate workspace materializer** (`crates/sr-adapters/src/candidate_store.rs`):
   - `CandidateWorkspace` trait for materializing candidate content
   - `TempWorkspace` struct with Drop cleanup
   - `SimpleCandidateWorkspace` implementation (placeholder for V8-2)
   - `WorkspaceError` error enum

3. **Created Oracle Execution Worker** (`crates/sr-adapters/src/oracle_worker.rs`):
   - `OracleExecutionWorker<S, R, Ev, C>` generic struct
   - Subscribes to `sr.events.run` NATS subject for `RunStarted` events
   - Validates suite hash against registry (TAMPER detection per C-OR-2)
   - Materializes candidate workspace
   - Executes oracle suite via `PodmanOracleRunner`
   - Emits `OracleExecutionStarted` and `OracleExecutionCompleted` events
   - Idempotency tracking via processed_runs map
   - `OracleWorkerConfig` and `OracleWorkerError` types

4. **Updated exports** (`crates/sr-adapters/src/lib.rs`):
   - Exports `candidate_store` and `oracle_worker` modules
   - Exports key types: `OracleExecutionWorker`, `OracleWorkerConfig`, etc.

### Test Results

| Package | Result |
|---------|--------|
| sr-domain | ✅ 118 passed |
| sr-adapters | ✅ 128 passed |
| sr-api | ✅ 41 passed |

### Files Created/Modified

| File | Action |
|------|--------|
| `crates/sr-domain/src/events.rs` | MODIFIED — Added oracle execution events |
| `crates/sr-adapters/src/candidate_store.rs` | CREATED — Workspace materializer |
| `crates/sr-adapters/src/oracle_worker.rs` | CREATED — Event-driven worker |
| `crates/sr-adapters/src/lib.rs` | MODIFIED — Exports new modules |

### Implementation Notes

- Worker follows `ReferenceWorkerBridge` and `SemanticWorkerBridge` patterns
- Uses generic type parameters (not dyn traits) for async compatibility
- TAMPER detection validates suite hash before execution (C-OR-2)
- Test mode skips event emission for unit testing
- Workspace materialization is placeholder; full content fetching deferred

---

## Next Instance Prompt: Implement SR-PLAN-V8 Phase V8-3

### Assignment

Implement **Phase V8-3: Oracle Integrity Condition Detection** — add detection and escalation for TAMPER/GAP/FLAKE/ENV_MISMATCH integrity conditions.

### Key Concept

Per SR-CONTRACT §6, integrity conditions are fundamental to trust:

| Condition | Contract | Detection Point |
|-----------|----------|-----------------|
| `ORACLE_TAMPER` | C-OR-2 | Suite hash mismatch at run start |
| `ORACLE_GAP` | C-OR-4 | Missing required oracle result |
| `ORACLE_FLAKE` | C-OR-5 | Non-deterministic required oracle |
| `ORACLE_ENV_MISMATCH` | C-OR-3 | Environment constraint violation |

All integrity conditions MUST halt progression and escalate (C-OR-7).

### Where to Look

| Document/File | What You'll Find |
|---------------|------------------|
| `docs/planning/SR-PLAN-V8.md` §3 Phase V8-3 | Full specification: detection points, code examples |
| `docs/platform/SR-CONTRACT.md` §6 | C-OR-1..7 oracle contract requirements |
| `crates/sr-adapters/src/oracle_runner.rs` | Where to add integrity detection |
| `crates/sr-adapters/src/oracle_worker.rs` | TAMPER detection already implemented here |
| `crates/sr-adapters/src/oracle_suite.rs` | `IntegrityCondition` enum already exists |

### What V8-3 Must Implement

1. **Integrity condition types** (`sr-domain/src/integrity.rs`):
   - `IntegrityCondition` enum with TAMPER/GAP/FLAKE/ENV_MISMATCH variants
   - Detection metadata for each condition type
   - Severity and escalation requirements

2. **Detection in oracle runner**:
   - ORACLE_GAP: Check all required oracles have results after execution
   - ORACLE_FLAKE: Detect via determinism declaration or repeat runs
   - ORACLE_ENV_MISMATCH: Validate environment fingerprint vs suite constraints

3. **`IntegrityViolationDetected` event**:
   - Emitted when any integrity condition is detected
   - Includes condition details and escalation requirements

4. **API error response** for integrity violations

### Current State

- Branch: `solver-ralph-8`
- V8-1: ✅ Complete (registry)
- V8-2: ✅ Complete (event-driven worker with TAMPER detection)
- V8-3: 🎯 Your assignment
- Estimated effort: 1-2 sessions

### Acceptance Criteria (from SR-PLAN-V8)

- [ ] ORACLE_TAMPER detected when suite hash mismatches (already in V8-2)
- [ ] ORACLE_GAP detected when required oracle missing from results
- [ ] ORACLE_ENV_MISMATCH detected when environment constraints violated
- [ ] Integrity violations halt run and return structured error
- [ ] `IntegrityViolationDetected` event emitted
- [ ] Integration test validates each integrity condition
- [ ] `cargo test --package sr-api` passes

### On Completion

1. Run tests: `cargo test --package sr-adapters && cargo test --package sr-api`
2. Git commit
3. Update SR-README: Mark V8-3 complete, write V8-4 prompt


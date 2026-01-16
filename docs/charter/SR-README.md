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
| SR-PLAN-V8 | Oracle Runner & Semantic Suites | D-24, D-25, D-27, D-39 | **In Progress** (V8-1 ✅) |
| SR-PLAN-V9 | Semantic Worker & Branch 0 | D-23, D-41, D-36 | Proposed |

**Milestone 1 (MVP) projected completion:** After V9

---

## SR-PLAN-V8 Implementation Status (UPDATED 2026-01-16)

| Phase | Status | Description | Amendment |
|-------|--------|-------------|-----------|
| V8-1: Oracle Suite Registry | ✅ Complete | Port trait extracted, PostgreSQL adapter ready | A-4 |
| V8-2: Event-Driven Worker | 🎯 Next | Worker subscribes to `RunStarted` events | A-1 |
| V8-3: Integrity Checks | ⏳ Pending | TAMPER/GAP/FLAKE/ENV_MISMATCH detection | — |
| V8-4: Core Oracle Suite | ⏳ Pending | Build/unit/schema/lint oracles + container | — |
| V8-5: Semantic Oracles | ⏳ Pending | Use existing types, focus on container packaging | A-3 |

**SR-PLAN-V8 has passed coherence assessment AND philosophical consistency review. V8-1 complete, V8-2 ready.**

### Amendments Applied

| ID | Issue | Resolution |
|----|-------|------------|
| A-1 | V8-2 assumed direct API call | Use Event-Driven Worker pattern |
| A-2 | Type relationship unclear | `OracleSuiteDefinition` = config, `OracleSuiteRecord` = stored entity |
| A-3 | Semantic types already exist | Use `sr-domain/src/semantic_oracle.rs` |
| A-4 | Registry partially implemented | Extract port from existing `OracleSuiteRegistry` |

**Revised Effort:** 7-10 sessions (was 7-9)

---

## Previous Session Summary (V8-1 Implementation)

**Session Goal:** Implement SR-PLAN-V8 Phase V8-1 — Oracle Suite Registry

### What Was Accomplished

1. **Added port trait to sr-ports** (`crates/sr-ports/src/lib.rs`):
   - `OracleSuiteRegistryPort` trait with 5 methods: `register`, `get`, `get_by_hash`, `list`, `deprecate`
   - `OracleSuiteRecord` struct (stored entity with lifecycle metadata)
   - `OracleSuiteStatus` enum (Active, Deprecated, Archived)
   - `RegisterSuiteInput` struct (bundles registration parameters for clippy compliance)
   - `SuiteFilter` struct for list queries
   - `OracleSuiteRegistryError` enum

2. **Created PostgreSQL adapter** (`crates/sr-adapters/src/postgres_oracle_registry.rs`):
   - `PostgresOracleSuiteRegistry` struct implementing `OracleSuiteRegistryPort`
   - Full CRUD operations against `proj.oracle_suites` table

3. **Created database migration** (`migrations/008_oracle_suite_registry.sql`):
   - `proj.oracle_suites` table with indexes on `suite_hash` and `status`

4. **Updated existing in-memory registry** (`crates/sr-adapters/src/oracle_suite.rs`):
   - `OracleSuiteRegistry` now implements `OracleSuiteRegistryPort`
   - Added `record_to_definition()` conversion function
   - Backwards compatible with existing API endpoints

5. **Updated exports** (`crates/sr-adapters/src/lib.rs`):
   - Exports `PostgresOracleSuiteRegistry` and `record_to_definition`

### Test Results

| Package | Result |
|---------|--------|
| sr-ports | ✅ Compiles clean |
| sr-adapters | ✅ 123 passed |
| sr-api | ✅ 41 passed |

### Files Created/Modified

| File | Action |
|------|--------|
| `crates/sr-ports/src/lib.rs` | MODIFIED — Added port trait, types, error enum |
| `migrations/008_oracle_suite_registry.sql` | CREATED — Database schema |
| `crates/sr-adapters/src/postgres_oracle_registry.rs` | CREATED — PostgreSQL adapter |
| `crates/sr-adapters/src/oracle_suite.rs` | MODIFIED — Implements port trait |
| `crates/sr-adapters/src/lib.rs` | MODIFIED — Exports new module |
| `docs/planning/SR-PLAN-V8.md` | MODIFIED — Marked V8-1 acceptance criteria complete |

### Implementation Notes

- Port trait named `OracleSuiteRegistryPort` (not `OracleSuiteRegistry`) to avoid collision with existing concrete type during transition
- `RegisterSuiteInput` struct introduced to satisfy clippy's "too many arguments" warning
- API layer continues using in-memory registry; PostgreSQL adapter available for production wiring
- Per Amendment A-2: `OracleSuiteDefinition` = execution config, `OracleSuiteRecord` = stored entity

---

## Next Instance Prompt: Implement SR-PLAN-V8 Phase V8-2

### Assignment

Implement **Phase V8-2: Event-Driven Oracle Worker** — create a worker that subscribes to `RunStarted` events and executes oracle suites.

### Key Concept (Amendment A-1)

V8-2 uses the **Event-Driven Worker pattern**, NOT direct API calls:
```
RunStarted event → NATS → Oracle Worker → executes suite → OracleExecutionCompleted event
```

The worker subscribes to `sr.runs.started` subject and processes events asynchronously.

### Where to Look

| Document/File | What You'll Find |
|---------------|------------------|
| `docs/planning/SR-PLAN-V8.md` §4 Phase V8-2 | Full specification: event flow, worker structure, acceptance criteria |
| `crates/sr-adapters/src/worker.rs` | Existing worker pattern (Reference Worker) to follow |
| `crates/sr-adapters/src/semantic_worker.rs` | Semantic worker pattern |
| `crates/sr-adapters/src/nats.rs` | NATS message bus adapter and subject conventions |
| `crates/sr-adapters/src/oracle_runner.rs` | Existing `PodmanOracleRunner` implementation |
| `crates/sr-domain/src/events.rs` | Event types (may need `OracleExecutionStarted`, `OracleExecutionCompleted`) |

### What V8-2 Must Implement

1. **Oracle Worker** that:
   - Subscribes to `sr.runs.started` NATS subject
   - Validates suite hash against registry (detect TAMPER)
   - Captures environment fingerprint
   - Calls existing `PodmanOracleRunner` to execute oracles
   - Emits `OracleExecutionStarted` and `OracleExecutionCompleted` events

2. **New Domain Events** (if not already present):
   - `OracleExecutionStarted` — emitted when worker begins processing
   - `OracleExecutionCompleted` — emitted with results and evidence hash

### Current State

- Branch: `solver-ralph-8`
- V8-1: ✅ Complete (port trait + PostgreSQL adapter)
- V8-2: 🎯 Your assignment
- Estimated effort: 2-3 sessions

### Acceptance Criteria (from SR-PLAN-V8)

- [ ] Worker subscribes to `RunStarted` events
- [ ] Suite hash validated before execution (TAMPER detection)
- [ ] Environment fingerprint captured
- [ ] Oracle runner invoked for each oracle in suite
- [ ] Evidence bundle stored in MinIO
- [ ] `OracleExecutionCompleted` event emitted
- [ ] `cargo test --package sr-adapters` passes

### On Completion

1. Run tests: `cargo test --package sr-adapters`
2. Git commit
3. Update SR-README: Mark V8-2 complete, write V8-3 prompt


# SR-PLAN-V12 Coherence Review

**Reviewer:** Agent (solver-ralph-12 branch)
**Review Date:** 2026-01-17
**Document Under Review:** `docs/planning/SR-PLAN-V12.md`
**Status:** Complete

---

## Executive Summary

SR-PLAN-V12 is **coherent with the codebase**. All file references exist, line counts are accurate, and the gap analysis correctly identifies what remains to be built. The plan proposes feasible implementation work that builds directly on existing infrastructure.

**Verdict: APPROVE**

The plan can proceed to implementation without revisions. All claims about existing code are accurate, and the proposed tasks are well-scoped.

---

## 1. File/Module Accuracy

### V12-1: Evidence Manifest Validation Oracle (D-15)

| Plan Reference | Actual Location | Status |
|----------------|-----------------|--------|
| `sr-adapters/src/evidence.rs` (890 lines) | ✅ Exists (890 lines) | **Exact match** |
| `oracle-suites/core-v1/oracles/schema-validation.sh` | ✅ Exists (131 lines) | Accurate |
| `oracle-suites/core-v1/suite.json` | ✅ Exists | Accurate |

**Finding F-1.1: ACCURATE — Evidence manifest implementation verified**

The `evidence.rs` file contains exactly what the plan describes:
- `EvidenceManifest` struct with full schema (lines 27-87)
- `EvidenceManifest::validate()` method (lines 187-256)
- `EvidenceManifest::to_deterministic_json()` method (lines 285-291)
- `EvidenceManifest::compute_hash()` method (lines 294-299)
- `ManifestValidationOracle` struct with `validate()` and `validate_json()` (lines 568-596)
- Comprehensive test suite (lines 599-890)

**Finding F-1.2: ACCURATE — Oracle pattern verified**

The `schema-validation.sh` oracle demonstrates the pattern for shell-based oracles:
- Reads inputs from workspace
- Outputs `sr.oracle_result.v1` JSON format
- Reports pass/fail with structured details

**Gap Assessment:** The plan correctly identifies that `ManifestValidationOracle` exists in Rust but is not packaged as a containerized oracle. The validation logic is complete and can be wrapped.

**Feasibility:** HIGH — Existing validation logic can be exposed via shell script or direct Rust binary.

---

### V12-2: NATS Message Contract Documentation (D-21)

| Plan Reference | Actual Location | Status |
|----------------|-----------------|--------|
| `sr-adapters/src/nats.rs` (677 lines) | ✅ Exists (677 lines) | **Exact match** |
| `MessageEnvelope` struct | ✅ Exists (lines 91-113) | Accurate |
| `streams` module | ✅ Exists (lines 119-126) | Accurate |
| `subjects` module | ✅ Exists (lines 128-178) | Accurate |
| `schemas/` directory | ✅ Exists | Accurate |
| `schemas/messaging/` | ❌ Does not exist | Expected (is the gap) |

**Finding F-2.1: ACCURATE — NATS implementation verified**

The `nats.rs` file contains:
- `MessageEnvelope` struct with all fields documented (lines 91-113)
- `SCHEMA_VERSION` constant: "1.0" (line 116)
- `streams` module with `EVENTS`, `COMMANDS`, `QUERIES` constants (lines 119-126)
- `subjects` module with 13 event subjects and 4 command subjects (lines 128-178)
- `NatsMessageBus` implementing `MessageBus` port (lines 186-396)
- Basic unit tests for serialization (lines 615-677)

**Finding F-2.2: ACCURATE — Contract documentation gap verified**

The `schemas/README.md` exists and defines the directory structure, but:
- No `messaging/` subdirectory exists
- No `message-envelope.schema.json` exists
- No formal contract documentation exists

Current `schemas/` structure:
```
schemas/
├── README.md
├── api/.gitkeep
├── codegen/.gitkeep
├── domain/.gitkeep
├── events/.gitkeep
└── evidence/.gitkeep
```

**Gap Assessment:** The plan correctly identifies that message contracts are embedded in code but not externalized as documentation or JSON Schema.

**Feasibility:** HIGH — The `MessageEnvelope` struct and subject patterns are well-defined and can be directly documented.

---

### V12-3: Standalone Governor Service Binary (D-22)

| Plan Reference | Actual Location | Status |
|----------------|-----------------|--------|
| `sr-adapters/src/governor.rs` (1059 lines) | ✅ Exists (1059 lines) | **Exact match** |
| `sr-api/src/handlers/work_surfaces.rs` | ✅ Exists (2101 lines) | Accurate |
| `/start` endpoint | ✅ Exists (lines 1495-1584) | Accurate |
| `crates/sr-governor/` | ❌ Does not exist | Expected (is the gap) |
| `deploy/docker-compose.yml` sr-governor service | ❌ Does not exist | Expected (is the gap) |

**Finding F-3.1: ACCURATE — Governor implementation verified**

The `governor.rs` file contains full implementation:
- `LoopBudget` configuration struct (lines 31-52)
- `StopCondition` enum with 6 variants (lines 54-70)
- `IterationPreconditions` with 7 conditions including `work_surface_available` (lines 72-123)
- `LoopTrackingState` with comprehensive state fields (lines 130-206)
- `GovernorDecision` audit record (lines 212-266)
- `LoopGovernor<E: EventStore>` service with full logic (lines 301-885)
- `try_start_iteration()` method with precondition checking (lines 553-698)
- `create_iteration_started_event()` helper (lines 700-738)
- Tests (lines 892-1059)

**Finding F-3.2: ACCURATE — API endpoint invokes governor logic inline**

The `work_surfaces.rs` file shows:
- `start_work_surface()` handler at `/start` (lines 1495-1584)
- `start_iteration_as_system()` helper (lines 1932-2008)
- Budget checking inline (lines 1713-1731)
- Failure tracking inline (lines 1733-1751)

Governor logic is currently invoked synchronously within API request handling, not as a separate service.

**Finding F-3.3: ACCURATE — No standalone service exists**

Verified:
- `crates/sr-governor/` directory does not exist
- `deploy/docker-compose.yml` has no `sr-governor` service
- No `Dockerfile.governor` exists

**Gap Assessment:** The plan correctly identifies that the governor logic is complete but runs only via API endpoint, not as a standalone polling service.

**Feasibility:** MEDIUM — Requires:
1. New crate with main.rs
2. Polling loop connecting to Postgres/NATS
3. Docker service definition
4. Health check endpoint
5. Integration with existing `LoopGovernor` implementation

The existing `LoopGovernor` struct is generic over `EventStore` and can be reused directly.

---

## 2. Gap Analysis Validation

### Claimed Gaps vs. Actual State

| ID | Claimed Gap | Verified? | Notes |
|----|-------------|-----------|-------|
| D-15 | Validation oracle not containerized | ✅ Yes | `ManifestValidationOracle` is Rust-only |
| D-21 | Formal contract docs not externalized | ✅ Yes | No `schemas/messaging/` exists |
| D-21 | No contract tests | ✅ Yes | Only basic serialization tests |
| D-22 | Standalone service binary not built | ✅ Yes | No `sr-governor` crate exists |
| D-22 | Runs only via API endpoint | ✅ Yes | `work_surfaces.rs` invokes inline |

### Missing Gap Items

**Finding F-4.1: No missing gaps identified**

The plan's gap analysis is comprehensive. I did not identify any additional gaps that would affect V12 implementation.

---

## 3. Dependency Feasibility

### Stated Dependencies (from plan §3)

| Phase | Stated Dependencies | Assessment |
|-------|---------------------|------------|
| V12-1 | None | ✅ Correct |
| V12-2 | None | ✅ Correct |
| V12-3 | V12-2 (message contracts) | ✅ Correct — governor needs formalized message schema |

**Finding F-5.1: Dependencies are accurately stated**

V12-1 and V12-2 are independent. V12-3 depends on V12-2 because the standalone governor service will need to publish messages to NATS using the documented contract.

---

## 4. Phase Ordering Assessment

### Plan Proposes

```
V12-1 (Manifest Oracle) ─────┐
                              ├──→ V12-3 (Governor Service)
V12-2 (Message Contracts) ───┘
```

**Finding F-6.1: Phase ordering is correct**

- V12-1 and V12-2 can run in parallel (no dependencies)
- V12-3 should follow V12-2 to use formalized message contracts
- This ordering minimizes rework

---

## 5. Effort Estimate Assessment

| Phase | Plan Estimate | Assessment |
|-------|---------------|------------|
| V12-1 | 0.5-1 session | ✅ Reasonable — wrap existing validation logic |
| V12-2 | 0.5-1 session | ✅ Reasonable — documentation from existing code |
| V12-3 | 1.5-2 sessions | ✅ Reasonable — new crate + docker integration |
| **Total** | **3-5 sessions** | ✅ Accurate |

**Finding F-7.1: Effort estimates are realistic**

The estimates account for:
- V12-1: Shell script wrapper + test fixtures + suite.json update
- V12-2: Markdown documentation + JSON Schema + contract tests
- V12-3: New crate + main.rs + docker-compose + integration tests

---

## 6. Risk Assessment Validation

### Plan-Stated Risks (from §6)

| Risk | Plan Assessment | Validation |
|------|-----------------|------------|
| Governor race conditions | Medium/High | ✅ Valid — event store optimistic locking mitigates |
| Message contract changes break consumers | Low/Medium | ✅ Valid — versioning strategy needed |
| Governor adds operational complexity | Medium/Low | ✅ Valid — keeping optional is correct approach |

**Finding F-8.1: Risks are appropriately identified**

No additional risks identified. The mitigation strategies are sound:
- Race conditions: Use event store optimistic locking (already exists in `EventStore::append`)
- Contract changes: JSON Schema versioning + evolution tests
- Complexity: Governor is optional; API `/start` continues to work

---

## 7. Verification Plan Assessment

### Plan Proposes

| Phase | Verification Method | Assessment |
|-------|---------------------|------------|
| V12-1 | Oracle runner test | ✅ Appropriate |
| V12-2 | Schema review + tests | ✅ Appropriate |
| V12-3 | Docker-compose test | ✅ Appropriate |

**Finding F-9.1: Verification methods are appropriate**

Each phase has clear success criteria that can be objectively verified.

---

## 8. Out of Scope Validation

The plan explicitly excludes:
- D-38 (Prompt → Plan Instance decomposition) — separate sub-project
- New features
- Performance optimization

**Finding F-10.1: Out of scope items correctly identified**

D-38 is a substantial feature that would expand V12 beyond operational refinement. Deferring it is appropriate.

---

## 9. Summary of Findings

| ID | Severity | Summary |
|----|----------|---------|
| F-1.1 | Info | Evidence manifest implementation verified (890 lines exact) |
| F-1.2 | Info | Oracle pattern exists in schema-validation.sh |
| F-2.1 | Info | NATS implementation verified (677 lines exact) |
| F-2.2 | Info | Contract documentation gap verified |
| F-3.1 | Info | Governor implementation verified (1059 lines exact) |
| F-3.2 | Info | API endpoint invokes governor inline — verified |
| F-3.3 | Info | No standalone governor service — verified |
| F-4.1 | Info | No missing gaps identified |
| F-5.1 | Info | Dependencies accurately stated |
| F-6.1 | Info | Phase ordering is correct |
| F-7.1 | Info | Effort estimates are realistic |
| F-8.1 | Info | Risks appropriately identified |
| F-9.1 | Info | Verification methods appropriate |
| F-10.1 | Info | Out of scope items correctly identified |

---

## 10. Recommendations

**No revisions required.** The plan is coherent and accurate.

### Implementation Notes

1. **V12-1 (Manifest Oracle):** Consider using the existing `ManifestValidationOracle::validate_json()` method directly. A simple shell wrapper can call a Rust binary that invokes this method.

2. **V12-2 (Message Contracts):** The `MessageEnvelope` struct at `nats.rs:91-113` has clear field documentation. Extract this to `schemas/messaging/message-envelope.schema.json` with matching field descriptions.

3. **V12-3 (Governor Service):** The `LoopGovernor<E>` struct is generic over `EventStore`. Create `sr-governor` crate with:
   - `main.rs` that instantiates `LoopGovernor<PgEventStore>`
   - Polling loop that calls `check_preconditions()` and `try_start_iteration()`
   - NATS publisher for `IterationStarted` events
   - Health endpoint for docker-compose

---

## 11. Verdict

### **APPROVE**

SR-PLAN-V12 is coherent with the codebase. All claims about existing code are verified accurate:
- Line counts are exact matches
- Feature descriptions are correct
- Gap analysis is complete and accurate
- Proposed tasks are feasible

Implementation may proceed on `solver-ralph-12` branch.

---

## Appendix: Codebase Validation Evidence

### Files Verified to Exist (with line counts)

```
✅ crates/sr-adapters/src/evidence.rs (890 lines)
✅ crates/sr-adapters/src/nats.rs (677 lines)
✅ crates/sr-adapters/src/governor.rs (1059 lines)
✅ crates/sr-api/src/handlers/work_surfaces.rs (2101 lines)
✅ oracle-suites/core-v1/oracles/schema-validation.sh (131 lines)
✅ oracle-suites/core-v1/suite.json
✅ schemas/README.md
✅ deploy/docker-compose.yml
```

### Files/Directories Verified NOT to Exist (gaps confirmed)

```
❌ crates/sr-governor/ (no such directory)
❌ schemas/messaging/ (no such directory)
❌ schemas/messaging/message-envelope.schema.json (no such file)
❌ deploy/docker-compose.yml service: sr-governor (no such service)
```

### Key Code Locations Verified

| Feature | File | Line(s) |
|---------|------|---------|
| EvidenceManifest struct | evidence.rs | 27-87 |
| EvidenceManifest::validate() | evidence.rs | 187-256 |
| ManifestValidationOracle | evidence.rs | 568-596 |
| MessageEnvelope struct | nats.rs | 91-113 |
| streams module | nats.rs | 119-126 |
| subjects module | nats.rs | 128-178 |
| LoopGovernor struct | governor.rs | 301-312 |
| try_start_iteration() | governor.rs | 553-698 |
| /start endpoint | work_surfaces.rs | 1495-1584 |
| start_iteration_as_system() | work_surfaces.rs | 1932-2008 |

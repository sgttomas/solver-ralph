# SR-PLAN-V12: Operational Refinement

**Status:** Draft — Pending Review
**Scope:** Complete partial deliverables D-15, D-21, D-22
**Estimated Effort:** 3-5 sessions
**Branch:** `solver-ralph-12`

---

## 1. Overview

V12 addresses three partial deliverables that remain after V11:

| ID | Title | Current State | Gap |
|----|-------|---------------|-----|
| D-15 | Evidence manifest v1 library + validation oracle | `evidence.rs` (890 lines) implements manifest schema, validation, deterministic serialization | **Validation oracle** not implemented as containerized oracle |
| D-21 | NATS/JetStream messaging integration | `nats.rs` (677 lines) implements message bus, streams, contracts | **Formal contract documentation** not externalized; no **contract tests** |
| D-22 | Loop governor service | `governor.rs` (1059 lines) implements full governor logic | **Standalone service binary** not built; runs only via API endpoint |

These are operational refinements, not new capabilities. The functionality exists; V12 formalizes and hardens it.

---

## 2. Phase Definitions

### V12-1: Evidence Manifest Validation Oracle

**Objective:** Package the existing `EvidenceManifest::validate()` logic as a containerized oracle that can be run by the oracle runner.

**What Exists:**
- `sr-adapters/src/evidence.rs`: Full manifest schema, validation, deterministic JSON, hash computation
- `oracle-suites/core-v1/oracles/schema-validation.sh`: Pattern for shell-based oracle

**Tasks:**
1. Create `oracle-suites/core-v1/oracles/manifest-validation.sh`:
   - Reads evidence manifest JSON from stdin or file
   - Validates against `evidence.gate_packet` schema
   - Checks required fields, timestamp ordering, verdict consistency
   - Outputs `sr.oracle_result.v1` format
2. Add manifest-validation to `oracle-suites/core-v1/suite.json`
3. Create test fixtures: valid manifest, invalid manifests (missing fields, bad timestamps, mismatched verdict)
4. Integration test: run oracle via oracle runner

**Verification:**
- Oracle passes on valid manifests
- Oracle fails with specific error codes on invalid manifests
- Oracle produces deterministic output for same input

**Estimated Effort:** 0.5-1 session

---

### V12-2: NATS Message Contract Documentation & Tests

**Objective:** Externalize the message contracts defined in `nats.rs` into a governed schema document and add contract tests.

**What Exists:**
- `sr-adapters/src/nats.rs`: `MessageEnvelope` struct, `streams` module, `subjects` module
- Full JetStream integration with publish/subscribe

**Tasks:**
1. Create `schemas/messaging/SR-MESSAGE-CONTRACTS.md`:
   - Document `MessageEnvelope` schema with field descriptions
   - Document all stream names and their purpose
   - Document all subject patterns and routing rules
   - Include versioning strategy for schema evolution
2. Create `schemas/messaging/message-envelope.schema.json`:
   - JSON Schema for `MessageEnvelope`
   - Can be used for validation in polyglot consumers
3. Add contract tests in `sr-adapters/src/nats.rs`:
   - Test message serialization roundtrip
   - Test idempotency key handling
   - Test stream/subject routing
4. Update `schemas/README.md` to index the new contracts

**Verification:**
- Schema document passes review (accurate, complete)
- JSON Schema validates example messages
- Contract tests pass
- Schema version matches `SCHEMA_VERSION` constant

**Estimated Effort:** 0.5-1 session

---

### V12-3: Standalone Governor Service Binary

**Objective:** Extract the loop governor into a standalone service that can run independently of the API.

**What Exists:**
- `sr-adapters/src/governor.rs`: Full `LoopGovernor` implementation with state tracking, precondition checking, decision recording
- `sr-api/src/handlers/work_surfaces.rs`: `/start` endpoint that invokes governor logic inline

**Tasks:**
1. Create `crates/sr-governor/Cargo.toml` and `src/main.rs`:
   - Standalone binary that runs the governor loop
   - Connects to Postgres (event store) and NATS (message bus)
   - Polls for loops requiring iteration starts
   - Emits `IterationStarted` events via NATS
   - Records all decisions as events
2. Add governor configuration:
   - Poll interval (default: 1s)
   - Max concurrent loops to process
   - Enable/disable dry-run mode
3. Add to `deploy/docker-compose.yml`:
   - `sr-governor` service
   - Depends on postgres, nats
   - Health check endpoint
4. Update `deploy/init.sh` to handle governor initialization
5. Integration test: governor starts iterations when preconditions met

**Constraints:**
- Governor must be **optional** — API `/start` endpoint continues to work without governor service
- Governor decisions are **auditable** — all decisions recorded as events
- Governor is **idempotent** — safe to restart, no duplicate iterations

**Verification:**
- Governor service starts and connects to dependencies
- Governor emits IterationStarted when loop is eligible
- Governor respects budgets and stop conditions
- API `/start` continues to work independently
- E2E test with governor service in docker-compose

**Estimated Effort:** 1.5-2 sessions

---

## 3. Phase Ordering

```
V12-1 (Manifest Oracle) ─────┐
                              ├──→ V12-3 (Governor Service)
V12-2 (Message Contracts) ───┘
```

V12-1 and V12-2 have no dependencies and can run in parallel.
V12-3 depends on V12-2 (needs formalized message contracts for governor→NATS communication).

---

## 4. Verification Plan

### Per-Phase Verification

| Phase | Verification Method | Success Criteria |
|-------|---------------------|------------------|
| V12-1 | Oracle runner test | Manifest validation oracle passes/fails correctly |
| V12-2 | Schema review + tests | Contract doc complete; JSON Schema valid; tests pass |
| V12-3 | Docker-compose test | Governor service runs; emits iterations; respects budgets |

### Final V12 Acceptance

1. D-15 complete: Evidence manifest validation oracle in `oracle-suites/core-v1`
2. D-21 complete: Message contracts documented in `schemas/messaging/`; contract tests pass
3. D-22 complete: `sr-governor` binary runs as standalone service
4. All three deliverables marked complete in SR-PLAN-GAP-ANALYSIS.md
5. No regressions: existing E2E tests pass

---

## 5. Deliverable Mapping

| Phase | SR-PLAN Deliverable | Completion |
|-------|---------------------|------------|
| V12-1 | D-15 (Evidence manifest v1 library + validation oracle) | Partial → Complete |
| V12-2 | D-21 (NATS/JetStream messaging integration) | Partial → Complete |
| V12-3 | D-22 (Loop governor service) | Partial → Complete |

---

## 6. Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Governor race conditions | Medium | High | Use event store optimistic locking; test concurrent scenarios |
| Message contract changes break consumers | Low | Medium | Version contracts; add schema evolution tests |
| Governor service adds operational complexity | Medium | Low | Make governor optional; document when to use |

---

## 7. Out of Scope

- **D-38 (Prompt → Plan Instance decomposition):** Deferred to separate sub-project per user direction
- **New features:** V12 is operational refinement only
- **Performance optimization:** Address in future plan if needed

---

## 8. References

- `docs/planning/SR-PLAN-GAP-ANALYSIS.md` — Deliverable status tracking
- `crates/sr-adapters/src/evidence.rs` — Existing manifest implementation
- `crates/sr-adapters/src/nats.rs` — Existing NATS implementation
- `crates/sr-adapters/src/governor.rs` — Existing governor implementation
- `docs/program/SR-PLAN.md` — Original deliverable definitions

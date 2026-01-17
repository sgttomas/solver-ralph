# SR-PLAN-V11: Production Hardening & E2E Testing

**Status:** Draft — Pending Coherence Review
**Scope:** Production Hardening & E2E Testing
**Target Deliverables:** D-16, D-26, D-32, D-33, D-35, D-08
**Branch:** `solver-ralph-11`
**Predecessor:** SR-PLAN-V10 (Complete)

---

## 1. Scope Definition

### 1.1 Objectives

V11 focuses on hardening the system for production readiness:

1. **Restricted Evidence Handling (D-16):** Complete Infisical integration for envelope key management
2. **Integration/E2E Oracle Suite (D-26):** Oracle suite for validating system integration
3. **Build/Init Scripts (D-32):** Complete development and deployment tooling
4. **Operational Observability (D-33):** Metrics, logging, and monitoring infrastructure
5. **E2E Failure Mode Harness (D-35):** Automated testing of failure modes and integrity conditions
6. **GovernedArtifact Refs (D-08):** Include SR-DIRECTIVE and other governed docs in iteration context

### 1.2 Deferred from V10

| ID | Description | Reason for Deferral |
|----|-------------|---------------------|
| V10-G5 | Active exceptions not in IterationStarted.refs[] | Low priority; requires exception lifecycle completion |
| Tests 17-18 | ORACLE_GAP/EVIDENCE_MISSING E2E testing | Requires E2E harness completion (D-35) |

### 1.3 Success Criteria

- All deferred SR-PLAN-LOOPS tests (17-18) passing
- E2E harness can execute full Loop lifecycle scenarios
- Infisical envelope key retrieval operational
- Observability endpoints exposed and functional
- Build scripts support clean dev environment setup

---

## 2. Phase Breakdown

### V11-1: Restricted Evidence Handling (D-16)

**Focus:** Complete Infisical integration for secure envelope key management

**Existing Infrastructure:**
- `crates/sr-adapters/src/infisical.rs` — Infisical client (exists, needs completion)
- `crates/sr-adapters/src/restricted.rs` — Restricted evidence store wrapper

**Tasks:**
1. Audit current Infisical client implementation for completeness
2. Implement envelope key retrieval for evidence decryption
3. Add configuration for Infisical connection (env vars, secrets)
4. Write integration tests for key retrieval flow
5. Document configuration in SR-DEPLOYMENT (or create if missing)

**Verification:**
- Unit tests for Infisical client
- Integration test with mock Infisical server
- Manual verification of key retrieval in dev environment

**Dependencies:** None (can start immediately)

---

### V11-2: Build/Init Scripts Completion (D-32)

**Focus:** Complete development and deployment tooling

**Existing Infrastructure:**
- `scripts/check-deps.sh` — Dependency checker
- `scripts/dev-setup.sh` — Development setup
- `scripts/run-tests.sh` — Test runner

**Tasks:**
1. Audit existing scripts for completeness
2. Add database initialization script (PostgreSQL schema)
3. Add MinIO bucket initialization script
4. Add NATS stream initialization script
5. Create `scripts/init-all.sh` orchestration script
6. Add Docker Compose configuration for local dev stack
7. Document in README or SR-DEPLOYMENT

**Verification:**
- Fresh clone + `./scripts/init-all.sh` produces working environment
- CI pipeline uses same scripts

**Dependencies:** None (can run in parallel with V11-1)

---

### V11-3: Operational Observability (D-33)

**Focus:** Metrics, logging, and monitoring infrastructure

**Existing Infrastructure:**
- `crates/sr-api/src/observability.rs` — Metrics endpoint (exists)
- Tracing instrumentation throughout codebase

**Tasks:**
1. Audit current metrics exposure (Prometheus format)
2. Add key operational metrics:
   - Loop lifecycle durations
   - Iteration success/failure rates
   - Oracle run times and outcomes
   - Event store append latencies
   - Projection processing times
3. Add health check endpoint (`/health`)
4. Add readiness check endpoint (`/ready`)
5. Configure structured logging (JSON format option)
6. Document metrics in SR-OBSERVABILITY (or create)

**Verification:**
- `/metrics` endpoint returns Prometheus-format metrics
- `/health` and `/ready` endpoints functional
- Grafana dashboard (optional, if time permits)

**Dependencies:** None (can run in parallel with V11-1, V11-2)

---

### V11-4: E2E Failure Mode Harness (D-35)

**Focus:** Automated testing of failure modes and integrity conditions

**Existing Infrastructure:**
- `crates/sr-e2e-harness/src/harness.rs` — E2E test harness
- `crates/sr-e2e-harness/src/failure_modes.rs` — Failure mode definitions
- `crates/sr-e2e-harness/src/replay.rs` — Replay verification
- `crates/sr-e2e-harness/src/transcript.rs` — Transcript recording
- `crates/sr-oracles/src/e2e.rs` — E2E oracle runner
- `crates/sr-oracles/src/integration.rs` — Integration tests

**Tasks:**
1. Audit existing E2E harness capabilities
2. Implement scenario definitions for:
   - Happy path: Full Loop lifecycle
   - BUDGET_EXHAUSTED trigger and Decision flow
   - REPEATED_FAILURE trigger and Decision flow
   - ORACLE_GAP detection (Test 17)
   - EVIDENCE_MISSING detection (Test 18)
3. Add CLI interface for running E2E scenarios
4. Integrate with CI pipeline
5. Document scenario authoring in SR-E2E-HARNESS (or create)

**Verification:**
- `cargo run --bin sr-e2e-harness -- --scenario happy-path` passes
- `cargo run --bin sr-e2e-harness -- --scenario oracle-gap` detects gap correctly
- Tests 17-18 from SR-PLAN-LOOPS marked as PASS

**Dependencies:** V11-2 (needs init scripts for test environment), V11-3 (observability helps debugging)

---

### V11-5: Integration/E2E Oracle Suite (D-26)

**Focus:** Oracle suite for validating system integration end-to-end

**Existing Infrastructure:**
- `crates/sr-oracles/src/integration.rs` — Integration test framework
- `crates/sr-adapters/src/oracle_suite.rs` — Oracle suite registry

**Tasks:**
1. Define integration oracle suite covering:
   - API endpoint availability
   - Database connectivity
   - MinIO connectivity
   - NATS connectivity
   - Event store integrity
   - Projection consistency
2. Register as `suite:SR-SUITE-INTEGRATION`
3. Add to oracle suite registry
4. Document in SR-ORACLE-SUITE (or update)

**Verification:**
- Integration oracle suite runs against live system
- All checks pass in healthy environment
- Clear failure messages for unhealthy components

**Dependencies:** V11-3 (health endpoints), V11-4 (harness for running)

---

### V11-6: GovernedArtifact Refs in Iteration Context (D-08)

**Focus:** Include SR-DIRECTIVE and other governed docs in IterationStarted.refs[]

**Existing Infrastructure:**
- `crates/sr-api/src/handlers/iterations.rs` — Iteration start handler
- `crates/sr-domain/src/entities.rs` — ContentHash type

**Tasks:**
1. Define which governed artifacts should be included:
   - SR-DIRECTIVE (execution policy)
   - SR-CONTRACT (invariants)
   - SR-SPEC (system spec) — optional
2. Implement content hashing for governed documents
3. Add refs to `IterationStarted.refs[]` with:
   - `kind: "GovernedArtifact"`
   - `rel: "governed_by"`
   - `meta: { content_hash: "sha256:..." }`
4. Update SR-SPEC Appendix A with GovernedArtifact ref schema

**Verification:**
- `IterationStarted` events include GovernedArtifact refs
- Content hashes are stable and reproducible
- Test 9 (if applicable) updated

**Dependencies:** V11-4 (E2E harness for verification)

---

## 3. Phase Ordering and Dependencies

```
V11-1 (Infisical) ─────────────────────────────────────────────────────┐
                                                                        │
V11-2 (Scripts) ───────────────────────────────────────────────────────┤
                                                                        │
V11-3 (Observability) ─────────────────────────────────────────────────┤
                                                                        ▼
                                                               V11-4 (E2E Harness)
                                                                        │
                                                                        ▼
                                                               V11-5 (Integration Suite)
                                                                        │
                                                                        ▼
                                                               V11-6 (GovernedArtifact Refs)
```

**Parallel Execution:**
- V11-1, V11-2, V11-3 can run in parallel (no dependencies)
- V11-4 depends on V11-2 (init scripts for test environment)
- V11-5 depends on V11-3 (health endpoints) and V11-4 (harness)
- V11-6 depends on V11-4 (E2E verification)

---

## 4. Verification Plan

### Per-Phase Verification

| Phase | Verification Method | Success Criteria |
|-------|---------------------|------------------|
| V11-1 | Unit + integration tests | Infisical key retrieval works |
| V11-2 | Fresh clone test | `init-all.sh` produces working env |
| V11-3 | Manual + automated | `/metrics`, `/health`, `/ready` functional |
| V11-4 | E2E scenario runs | All scenarios pass, Tests 17-18 pass |
| V11-5 | Integration suite run | All integration checks pass |
| V11-6 | Event inspection | GovernedArtifact refs in events |

### Final V11 Acceptance

1. All deferred SR-PLAN-LOOPS tests passing (17-18)
2. E2E harness operational with documented scenarios
3. Integration oracle suite registered and functional
4. Observability endpoints exposed
5. Build scripts enable clean dev setup
6. Documentation updated

---

## 5. Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Infisical API changes | Low | Medium | Pin client version, add integration tests |
| E2E harness complexity | Medium | High | Start with simple scenarios, iterate |
| Test environment flakiness | Medium | Medium | Use Docker Compose for isolation |
| Scope creep | Medium | Medium | Strict phase boundaries, defer non-essential |

---

## 6. Documentation Updates Required

After V11 completion, update:
- SR-PLAN-LOOPS: Tests 17-18 → PASS
- SR-PLAN-GAP-ANALYSIS: V11 → Complete
- SR-CHANGE: V11 implementation entries
- SR-SPEC: GovernedArtifact ref schema (if added)
- Create or update: SR-DEPLOYMENT, SR-OBSERVABILITY, SR-E2E-HARNESS

---

## 7. Estimated Effort

| Phase | Estimated Effort |
|-------|------------------|
| V11-1 | 1-2 sessions |
| V11-2 | 1 session |
| V11-3 | 1-2 sessions |
| V11-4 | 3-4 sessions |
| V11-5 | 1-2 sessions |
| V11-6 | 1 session |
| **Total** | **8-12 sessions** |

---

## 8. Open Questions

1. **Infisical deployment:** Is there a dev Infisical instance, or should we mock it?
2. **GovernedArtifact selection:** Which documents are mandatory vs optional in refs?
3. **E2E environment:** Docker Compose vs Kubernetes for test isolation?
4. **Metrics backend:** Prometheus assumed — is this correct?

These questions should be resolved during V11 coherence review or early implementation.

# SR-PLAN-V11: Production Hardening & E2E Testing

**Status:** Approved — Ready for Implementation
**Scope:** Production Hardening & E2E Testing
**Target Deliverables:** D-16, D-26, D-32, D-33, D-35, D-08
**Branch:** `solver-ralph-11`
**Predecessor:** SR-PLAN-V10 (Complete)
**Coherence Review:** 2026-01-17 — APPROVED WITH REVISIONS (incorporated below)

---

## 1. Scope Definition

### 1.1 Objectives

V11 focuses on hardening the system for production readiness:

1. **Restricted Evidence Handling (D-16):** Verify and test Infisical integration for envelope key management
2. **Build/Init Scripts Audit (D-32):** Audit and document existing deployment tooling
3. **Operational Observability (D-33):** Add domain-specific metrics and `/ready` endpoint
4. **E2E Failure Mode Harness (D-35):** Complete and verify failure mode scenarios
5. **Integration/E2E Oracle Suite (D-26):** Register integration runner as oracle suite
6. **GovernedArtifact Refs (D-08):** Include SR-DIRECTIVE and other governed docs in iteration context
7. **Active Exception Refs (V10-G5):** Include active exceptions in IterationStarted.refs[]

### 1.2 Deferred from V10

| ID | Description | V11 Phase |
|----|-------------|-----------|
| V10-G5 | Active exceptions not in IterationStarted.refs[] | V11-6 |
| Tests 17-18 | ORACLE_GAP/EVIDENCE_MISSING E2E testing | V11-4 |

### 1.3 Success Criteria

- All deferred SR-PLAN-LOOPS tests (17-18) passing
- E2E harness scenarios execute successfully against live system
- Infisical envelope key retrieval verified end-to-end
- `/ready` endpoint checks all dependencies (DB, MinIO, NATS)
- Domain-specific metrics exposed (loop, iteration, oracle latencies)
- Existing deployment tooling documented
- GovernedArtifact and active exception refs in IterationStarted events

---

## 2. Phase Breakdown

### V11-1: Restricted Evidence Handling (D-16)

**Focus:** Verify and test existing Infisical integration for secure envelope key management

**Existing Infrastructure (per coherence review):**
- `crates/sr-adapters/src/infisical.rs` (431 lines) — **Complete implementation** including:
  - Full `SecretProvider` trait (get, store, delete, exists)
  - `get_envelope_key()` with AES-256-GCM support
  - Key caching with `RwLock<HashMap>`
  - Environment variable configuration
  - Unit tests for path parsing
- `crates/sr-adapters/src/restricted.rs` — Envelope encryption wrapper

**Tasks:**
1. ~~Implement envelope key retrieval~~ (Already implemented)
2. Write integration test with mock Infisical server (wiremock or similar)
3. Verify end-to-end flow: store restricted evidence → retrieve with envelope key
4. Document Infisical configuration requirements in SR-DEPLOYMENT
5. Add example `.env.example` with required Infisical variables

**Verification:**
- Integration test passes with mock Infisical
- Manual verification of key retrieval in dev environment (if Infisical available)
- Documentation complete

**Dependencies:** None (can start immediately)

**Estimated Effort:** 1 session (reduced from 1-2; implementation exists)

---

### V11-2: Build/Init Scripts Audit (D-32)

**Focus:** Audit and document existing deployment tooling

**Existing Infrastructure (per coherence review):**
- `deploy/init.sh` (21,387 bytes) — **Comprehensive initialization** including:
  - PostgreSQL schema creation and migrations
  - MinIO bucket initialization
  - Zitadel OIDC setup
  - Secrets initialization
- `deploy/docker-compose.yml` (6,562 bytes) — **Full stack** including:
  - PostgreSQL 16.4
  - MinIO with JetStream
  - NATS 2.10.22
  - Zitadel OIDC
  - Infisical (optional)
- `deploy/start.sh` — Service startup
- `deploy/init-scripts/postgres/` — SQL initialization
- `scripts/check-deps.sh`, `scripts/dev-setup.sh`, `scripts/run-tests.sh`
- `migrations/001-009` — Database migrations

**Tasks:**
1. ~~Create init scripts~~ (Already exist in `deploy/`)
2. ~~Create Docker Compose~~ (Already exists)
3. Audit `deploy/init.sh` for completeness:
   - Verify NATS stream initialization is included
   - Verify all migrations are applied
4. Create `scripts/init-all.sh` as convenience wrapper that calls `deploy/init.sh`
5. Document deployment tooling in SR-DEPLOYMENT:
   - Prerequisites
   - Environment variables
   - Step-by-step setup guide
6. Update README.md with quick start instructions

**Verification:**
- Fresh clone + `docker-compose up -d` + `./deploy/init.sh` produces working environment
- Documentation reviewed for completeness

**Dependencies:** None (can run in parallel with V11-1)

**Estimated Effort:** 0.5 sessions (reduced from 1; infrastructure exists)

---

### V11-3: Operational Observability (D-33)

**Focus:** Add domain-specific metrics and readiness endpoint

**Existing Infrastructure (per coherence review):**
- `crates/sr-api/src/observability.rs` (378 lines) — **Foundation exists** including:
  - `/health` endpoint (confirmed at `main.rs:163`)
  - `/api/v1/metrics` endpoint (confirmed at `main.rs:166`)
  - `Metrics` struct with request counters
  - `MetricsSnapshot` with `avg_latency_ms`
  - Request context middleware with correlation IDs
  - Tracing instrumentation
- JSON format metrics (not Prometheus)

**Tasks:**
1. ~~Add `/health` endpoint~~ (Already exists)
2. Add `/ready` endpoint that checks:
   - PostgreSQL connectivity
   - MinIO connectivity
   - NATS connectivity
3. Add domain-specific metrics:
   - `loop_lifecycle_duration_seconds` — histogram
   - `iteration_total` — counter with `outcome` label (success/failure)
   - `oracle_run_duration_seconds` — histogram with `suite_id` label
   - `event_store_append_duration_seconds` — histogram
   - `projection_rebuild_duration_seconds` — histogram
4. Optionally add Prometheus format endpoint (`/metrics/prometheus`)
5. Document metrics in SR-OBSERVABILITY

**Verification:**
- `/ready` returns 503 when any dependency is down
- `/api/v1/metrics` includes domain metrics
- Documentation complete

**Dependencies:** None (can run in parallel with V11-1, V11-2)

**Estimated Effort:** 1-2 sessions

---

### V11-4: E2E Failure Mode Harness (D-35)

**Focus:** Complete and verify existing failure mode scenarios

**Existing Infrastructure (per coherence review):**
- `crates/sr-e2e-harness/src/harness.rs` (24,914 bytes) — Core harness
- `crates/sr-e2e-harness/src/failure_modes.rs` (45,553 bytes) — **Scenarios implemented**:
  - `FailureMode::OracleFailure`
  - `FailureMode::IntegrityTamper`
  - `FailureMode::IntegrityGap`
  - `FailureMode::ExceptionWaiver`
- `crates/sr-e2e-harness/src/replay.rs` (20,030 bytes) — Replay verification
- `crates/sr-e2e-harness/src/transcript.rs` (11,437 bytes) — Transcript recording
- `crates/sr-e2e-harness/src/main.rs` (26,630 bytes) — CLI with flags:
  - `--replay`, `--verify-determinism`
  - `--oracle-failure`, `--integrity-tamper`, `--exception-waiver`

**Tasks:**
1. ~~Implement failure mode scenarios~~ (Already implemented)
2. Verify existing `IntegrityGap` scenario covers ORACLE_GAP (Test 17)
3. Add or verify EVIDENCE_MISSING scenario (Test 18):
   - Corrupt/delete evidence blob in MinIO
   - Verify system blocks verification
4. Run all scenarios against live system:
   - `cargo run --bin sr-e2e-harness -- --scenario happy-path`
   - `cargo run --bin sr-e2e-harness -- --oracle-failure`
   - `cargo run --bin sr-e2e-harness -- --integrity-tamper`
   - `cargo run --bin sr-e2e-harness -- --exception-waiver`
5. Fix any failures discovered during scenario runs
6. Add CI job to run E2E scenarios (can be nightly/manual trigger)
7. Document scenario authoring in SR-E2E-HARNESS

**Verification:**
- All existing scenarios pass against live system
- Tests 17-18 from SR-PLAN-LOOPS marked as PASS
- CI integration complete

**Dependencies:** Existing `deploy/docker-compose.yml` (V11-2 is audit only)

**Estimated Effort:** 2-3 sessions (reduced from 3-4; harness exists)

---

### V11-5: Integration/E2E Oracle Suite (D-26)

**Focus:** Register existing integration runner as formal oracle suite

**Existing Infrastructure (per coherence review):**
- `crates/sr-oracles/src/integration.rs` (38,047 bytes) — **Complete runner** including:
  - `IntegrationRunner` with PostgreSQL, MinIO, NATS, API tests
  - `IntegrationConfig` with env var configuration
  - `FlakeControl` for retry handling
  - `IntegrationReport` generation
- `crates/sr-adapters/src/oracle_suite.rs` — Suite registry with:
  - `OracleSuiteRegistry::register_suite()`
  - `with_core_suites()` initialization
  - Existing suites: `SR-SUITE-GOV`, `SR-SUITE-CORE`, `SR-SUITE-FULL`

**Tasks:**
1. ~~Implement integration tests~~ (Already implemented in `IntegrationRunner`)
2. Create adapter to expose `IntegrationRunner` as `OracleSuiteDefinition`
3. Register as `suite:SR-SUITE-INTEGRATION` in `with_core_suites()`
4. Handle cross-crate dependency:
   - Option A: Move registration logic to `sr-adapters` (imports `sr-oracles`)
   - Option B: Expose registration hook from `sr-api` startup
5. Document suite in SR-SPEC Appendix or SR-ORACLE-SUITE

**Verification:**
- `SR-SUITE-INTEGRATION` appears in suite registry
- Suite can be invoked via oracle runner
- Clear failure messages for unhealthy components

**Dependencies:** V11-3 (readiness endpoint useful for integration checks)

**Estimated Effort:** 1 session (reduced from 1-2; implementation exists)

---

### V11-6: GovernedArtifact & Exception Refs (D-08, V10-G5)

**Focus:** Include governed docs and active exceptions in IterationStarted.refs[]

**Existing Infrastructure:**
- `crates/sr-api/src/handlers/iterations.rs` — Iteration start handler
- `crates/sr-domain/src/entities.rs` — `ContentHash` type
- `crates/sr-domain/src/refs.rs` — `TypedRef` type

**Scope Clarification (from coherence review):**

This phase addresses TWO deferred items:
1. **D-08:** GovernedArtifact refs (SR-DIRECTIVE, SR-CONTRACT)
2. **V10-G5:** Active exception refs

### Part A: GovernedArtifact Refs (D-08)

**Design Decision — Content Hashing Approach:**

Governed documents (SR-DIRECTIVE, SR-CONTRACT, etc.) are markdown files that rarely change during runtime. To ensure reproducible hashes:

1. **Compute at startup:** API computes `GovernedManifest` at startup containing:
   - Document path
   - Content hash (SHA-256 of raw file bytes)
   - Document version (from YAML frontmatter if present)

2. **Canonicalization:** Use raw file bytes (no whitespace normalization) for hash stability

3. **Mandatory vs Optional:**
   - **Mandatory:** SR-DIRECTIVE (execution policy) — affects agent behavior
   - **Optional:** SR-CONTRACT, SR-SPEC — can be included for auditability

4. **Storage:** `GovernedManifest` stored in `AppState`, refs added at iteration start

**Tasks (Part A):**
1. Create `GovernedManifest` struct:
   ```rust
   struct GovernedArtifactRef {
       doc_id: String,           // e.g., "SR-DIRECTIVE"
       path: String,             // e.g., "docs/program/SR-DIRECTIVE.md"
       content_hash: ContentHash,
       version: Option<String>,  // from frontmatter
   }
   struct GovernedManifest {
       artifacts: Vec<GovernedArtifactRef>,
       computed_at: DateTime<Utc>,
   }
   ```
2. Implement manifest computation at API startup
3. Configure which docs to include (env var or config file)
4. Add refs to `IterationStarted.refs[]` per SR-SPEC §1.5.2:
   - `kind: "GovernedArtifact"`
   - `id: "<doc_id>"` (e.g., `"SR-DIRECTIVE"`)
   - `rel: "depends_on"` (per SR-SPEC §1.5.3 — `governed_by` is deprecated)
   - `meta: { content_hash: "sha256:...", version: "...", type_key: "governance.dev_directive" }`

### Part B: Active Exception Refs (V10-G5)

**Tasks (Part B):**
1. Query active exceptions for the work unit at iteration start
2. Add refs to `IterationStarted.refs[]` per SR-SPEC §1.5.2 and §3.2.1.1:
   - `kind: "Waiver"` (or `"Deviation"` or `"Deferral"` — use specific exception type)
   - `id: "<exception_id>"` (e.g., `"exc_01J..."`)
   - `rel: "depends_on"` (per SR-SPEC §3.2.1.1 item 6)
   - `meta: { scope: "per-candidate|per-loop|per-baseline|time-boxed", expires_at: "..." }`

**Verification:**
- `IterationStarted` events include GovernedArtifact refs with stable hashes
- `IterationStarted` events include active exception refs
- Replay produces identical refs (determinism)
- SR-SPEC Appendix A updated with ref schemas

**Dependencies:** V11-4 (E2E harness for verification)

**Estimated Effort:** 1-2 sessions

---

## 3. Phase Ordering and Dependencies

```
V11-1 (Infisical verify) ──────────────────────────────────┐
                                                            │
V11-2 (Scripts audit) ─────────────────────────────────────┤
                                                            │  Can start V11-4 once
V11-3 (Observability) ─────────────────────────────────────┤  docker-compose verified
                                                            │
                                                            ▼
                                                   V11-4 (E2E Harness verify)
                                                            │
                                              ┌─────────────┴─────────────┐
                                              ▼                           ▼
                                     V11-5 (Integration Suite)    V11-6 (Refs)
```

**Parallel Execution:**
- V11-1, V11-2, V11-3 can run in parallel (no dependencies)
- V11-4 can start once existing `deploy/docker-compose.yml` is verified working (V11-2 is audit-only)
- V11-5 and V11-6 can run in parallel after V11-4

**Critical Path:** V11-2 (verify infra) → V11-4 (run scenarios) → V11-6 (refs)

---

## 4. Verification Plan

### Per-Phase Verification

| Phase | Verification Method | Success Criteria |
|-------|---------------------|------------------|
| V11-1 | Integration test | Mock Infisical test passes; docs complete |
| V11-2 | Fresh clone test | `docker-compose up` + `init.sh` works; docs complete |
| V11-3 | Endpoint tests | `/ready` checks dependencies; domain metrics exposed |
| V11-4 | Scenario runs | All E2E scenarios pass; Tests 17-18 pass |
| V11-5 | Registry check | `SR-SUITE-INTEGRATION` registered and runnable |
| V11-6 | Event inspection | GovernedArtifact + Exception refs in IterationStarted |

### Final V11 Acceptance

1. All deferred SR-PLAN-LOOPS tests passing (17-18)
2. E2E harness scenarios pass against live system
3. Integration oracle suite registered and functional
4. `/ready` endpoint operational with dependency checks
5. Domain-specific metrics exposed
6. GovernedArtifact and active exception refs in iteration context
7. Documentation updated (SR-DEPLOYMENT, SR-OBSERVABILITY, SR-E2E-HARNESS)

---

## 5. Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Infisical API changes | Low | Medium | Pin client version, add integration tests |
| E2E scenario failures | Medium | Medium | Fix issues discovered; scenarios exist |
| Cross-crate suite registration | Low | Low | Choose registration approach early |
| GovernedArtifact hash instability | Low | Medium | Use raw bytes, test determinism |
| Scope creep | Medium | Medium | Strict phase boundaries, defer non-essential |

**Risk removed:** "E2E harness complexity" — harness already exists with 45K+ lines of failure mode code.

---

## 6. Documentation Updates Required

After V11 completion, update:
- SR-PLAN-LOOPS: Tests 17-18 → PASS
- SR-PLAN-GAP-ANALYSIS: V11 → Complete; update D-32/D-33/D-35 status
- SR-CHANGE: V11 implementation entries
- SR-SPEC Appendix A: GovernedArtifact and Exception ref schemas
- Create: SR-DEPLOYMENT (deployment guide)
- Create: SR-OBSERVABILITY (metrics reference)
- Create: SR-E2E-HARNESS (scenario authoring guide)

---

## 7. Estimated Effort

| Phase | Estimated Effort | Notes |
|-------|------------------|-------|
| V11-1 | 1 session | Reduced; implementation exists |
| V11-2 | 0.5 sessions | Reduced; infrastructure exists |
| V11-3 | 1-2 sessions | Foundation exists; add domain metrics |
| V11-4 | 2-3 sessions | Reduced; harness exists; verify + fix |
| V11-5 | 1 session | Reduced; runner exists; registration only |
| V11-6 | 1-2 sessions | New work; design decisions made above |
| **Total** | **6.5-9.5 sessions** | Reduced from 8-12 |

---

## 8. Resolved Questions

| Question | Resolution |
|----------|------------|
| Infisical deployment | Mock for tests; real instance optional for manual verification |
| GovernedArtifact selection | SR-DIRECTIVE mandatory; SR-CONTRACT/SR-SPEC optional |
| E2E environment | Docker Compose (exists at `deploy/docker-compose.yml`) |
| Metrics backend | JSON default; Prometheus format optional |
| Content hashing approach | Raw file bytes at API startup; stored in `GovernedManifest` |
| V10-G5 handling | Included in V11-6 scope |

---

## 9. Coherence Review Cross-Reference

This plan incorporates findings from `SR-PLAN-V11-COHERENCE-REVIEW.md` (2026-01-17):

| Finding | Recommendation | Incorporated |
|---------|----------------|--------------|
| F-1.1 | Update V11-1 scope | ✅ Reduced scope |
| F-1.2 | Revise V11-2 to audit existing infra | ✅ Changed to audit |
| F-1.3 | Acknowledge observability foundation | ✅ Listed existing |
| F-1.4 | Focus V11-4 on verification | ✅ Changed to verify |
| F-1.5 | Note suite registration approach | ✅ Added options |
| F-1.6 | Define hashing approach | ✅ Added design section |
| F-4.1 | Address V10-G5 | ✅ Added to V11-6 |
| R-3.1 | Update phase diagram | ✅ Updated |
| R-6.1 | Answer open question 3 | ✅ Resolved |

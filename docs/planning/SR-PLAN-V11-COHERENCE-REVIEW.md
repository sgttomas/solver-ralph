# SR-PLAN-V11 Coherence Review

**Reviewer:** Agent (solver-ralph-11 branch)
**Review Date:** 2026-01-17
**Document Under Review:** `docs/planning/SR-PLAN-V11.md`
**Status:** Complete

---

## Executive Summary

SR-PLAN-V11 is **substantially coherent** with the codebase. The plan accurately references existing infrastructure and proposes feasible implementation work. However, several corrections and clarifications are needed before implementation can proceed safely.

**Verdict: APPROVE WITH REVISIONS**

The plan can proceed to implementation after addressing the findings documented below. Most issues are minor (documentation accuracy) rather than structural (wrong architecture).

---

## 1. File/Module Accuracy

### V11-1: Restricted Evidence Handling (D-16)

| Plan Reference | Actual Location | Status |
|----------------|-----------------|--------|
| `crates/sr-adapters/src/infisical.rs` | ✅ Exists (431 lines) | Accurate |
| `crates/sr-adapters/src/restricted.rs` | ✅ Exists | Accurate |

**Finding F-1.1: ACCURATE — Infisical implementation more complete than stated**

The plan states Infisical "needs completion" but the implementation is more substantial than implied:
- Full `SecretProvider` trait implementation (get, store, delete, exists)
- Complete `get_envelope_key()` with AES-256-GCM support
- Key caching with `RwLock<HashMap>`
- Path parsing for folder/key conventions
- Configuration from environment variables
- Unit tests for path parsing and config

**Impact:** V11-1 scope may be smaller than estimated. The main gaps are:
1. Integration tests with real/mock Infisical
2. Documentation of configuration requirements
3. Verification that envelope key retrieval works end-to-end

**Recommendation R-1.1:** Update V11-1 task list to focus on:
- Integration test with mock Infisical server
- Documentation in SR-DEPLOYMENT
- End-to-end verification with restricted evidence flow

---

### V11-2: Build/Init Scripts (D-32)

| Plan Reference | Actual Location | Status |
|----------------|-----------------|--------|
| `scripts/check-deps.sh` | ✅ Exists (1758 bytes) | Accurate |
| `scripts/dev-setup.sh` | ✅ Exists (1463 bytes) | Accurate |
| `scripts/run-tests.sh` | ✅ Exists (1763 bytes) | Accurate |

**Finding F-1.2: PARTIAL — Plan misses existing infrastructure**

The plan proposes creating:
- Database initialization script (PostgreSQL schema)
- MinIO bucket initialization script
- NATS stream initialization script
- `scripts/init-all.sh` orchestration script
- Docker Compose configuration

**Actual codebase state:**
- `deploy/init.sh` (21,387 bytes) — **Already exists** with PostgreSQL, MinIO, Zitadel, and secrets initialization
- `deploy/docker-compose.yml` (6,562 bytes) — **Already exists** with full stack (Postgres, MinIO, NATS, Zitadel, Infisical)
- `deploy/start.sh` — Exists
- `migrations/` — 9 migrations exist (001-009)
- `deploy/init-scripts/postgres/01_init.sql` — Exists

**Impact:** V11-2 scope is significantly smaller than planned. Most infrastructure already exists.

**Recommendation R-1.2:** Revise V11-2 to:
1. **Audit** existing `deploy/init.sh` for completeness
2. **Add** NATS stream initialization if missing from `deploy/init.sh`
3. **Create** `scripts/init-all.sh` as a wrapper that calls `deploy/init.sh` (or document that `deploy/init.sh` IS the orchestration script)
4. **Document** the existing tooling in README or SR-DEPLOYMENT

---

### V11-3: Operational Observability (D-33)

| Plan Reference | Actual Location | Status |
|----------------|-----------------|--------|
| `crates/sr-api/src/observability.rs` | ✅ Exists (378 lines) | Accurate |

**Finding F-1.3: PARTIAL — Plan understates existing implementation**

The plan proposes adding `/health` and `/ready` endpoints. Current state:

**Already implemented:**
- `/health` endpoint (confirmed in `main.rs:163`)
- `/api/v1/metrics` endpoint (confirmed in `main.rs:166`)
- `Metrics` struct with request counters (total, success, client_error, server_error)
- `MetricsSnapshot` with `avg_latency_ms`
- Request context middleware with correlation IDs
- Tracing instrumentation

**Not implemented (per plan):**
- `/ready` endpoint (distinct from `/health`)
- Loop lifecycle duration metrics
- Iteration success/failure rate metrics
- Oracle run time metrics
- Event store append latency metrics
- Projection processing time metrics
- Prometheus-format output (current is JSON)

**Impact:** V11-3 scope is moderate. Foundation exists but domain-specific metrics need implementation.

**Recommendation R-1.3:** Update V11-3 to acknowledge existing infrastructure and focus on:
1. Add `/ready` endpoint (checks DB, MinIO, NATS connectivity)
2. Add domain-specific metrics (loop, iteration, oracle latencies)
3. Optionally add Prometheus format option alongside JSON

---

### V11-4: E2E Failure Mode Harness (D-35)

| Plan Reference | Actual Location | Status |
|----------------|-----------------|--------|
| `crates/sr-e2e-harness/src/harness.rs` | ✅ Exists (24,914 bytes) | Accurate |
| `crates/sr-e2e-harness/src/failure_modes.rs` | ✅ Exists (45,553 bytes) | Accurate |
| `crates/sr-e2e-harness/src/replay.rs` | ✅ Exists (20,030 bytes) | Accurate |
| `crates/sr-e2e-harness/src/transcript.rs` | ✅ Exists (11,437 bytes) | Accurate |
| `crates/sr-oracles/src/e2e.rs` | ✅ Exists (21,136 bytes) | Accurate |
| `crates/sr-oracles/src/integration.rs` | ✅ Exists (38,047 bytes) | Accurate |

**Finding F-1.4: ACCURATE — E2E harness infrastructure exists and is substantial**

The E2E harness already implements:
- CLI interface with `--oracle-failure`, `--integrity-tamper`, `--exception-waiver` flags
- `FailureMode` enum: `OracleFailure`, `IntegrityTamper`, `IntegrityGap`, `ExceptionWaiver`
- `HarnessTranscript` for recording test execution
- Happy path flow (`run_happy_path`)
- Oracle failure flow (`run_oracle_failure`)
- Integrity tamper flow (`run_integrity_tamper`)
- Exception/waiver flow (`run_exception_waiver`)

**Gap:** The harness exists but Tests 17-18 (ORACLE_GAP, EVIDENCE_MISSING) need scenarios added or verified working.

**Recommendation R-1.4:** V11-4 should focus on:
1. Verify existing `FailureMode::IntegrityGap` covers ORACLE_GAP scenario
2. Add EVIDENCE_MISSING scenario if not covered
3. Run existing scenarios against live system and fix any failures
4. Integrate with CI pipeline
5. Document scenario authoring

---

### V11-5: Integration/E2E Oracle Suite (D-26)

| Plan Reference | Actual Location | Status |
|----------------|-----------------|--------|
| `crates/sr-oracles/src/integration.rs` | ✅ Exists (38,047 bytes) | Accurate |
| `crates/sr-adapters/src/oracle_suite.rs` | ✅ Exists | Accurate |

**Finding F-1.5: ACCURATE — Integration runner exists**

The `IntegrationRunner` in `sr-oracles/src/integration.rs` already implements:
- PostgreSQL connectivity test
- MinIO connectivity test (with bucket checks)
- NATS connectivity test
- API health check
- `IntegrationConfig` with environment variable configuration
- `FlakeControl` for retry handling
- `IntegrationReport` generation

**Gap:** Not registered as a formal oracle suite in the registry.

**Recommendation R-1.5:** V11-5 should focus on:
1. Register `IntegrationRunner` as `suite:SR-SUITE-INTEGRATION` in `OracleSuiteRegistry`
2. Add to `with_core_suites()` initialization
3. Document in SR-ORACLE-SUITE or SR-SPEC Appendix

---

### V11-6: GovernedArtifact Refs (D-08)

| Plan Reference | Actual Location | Status |
|----------------|-----------------|--------|
| `crates/sr-api/src/handlers/iterations.rs` | ✅ Exists (15,583 bytes) | Accurate |
| `crates/sr-domain/src/entities.rs` | ✅ Contains `ContentHash` | Accurate |

**Finding F-1.6: PARTIAL — Plan underestimates implementation complexity**

The plan proposes adding GovernedArtifact refs with content hashes to `IterationStarted.refs[]`. However:

1. **Content hashing for docs:** Governed documents (SR-DIRECTIVE, SR-CONTRACT) are markdown files. Hashing them requires:
   - Reading files from disk at iteration start time
   - Stable canonicalization (whitespace normalization?)
   - Storing the hash in a reproducible way

2. **Which docs to include:** Plan lists SR-DIRECTIVE, SR-CONTRACT, SR-SPEC as candidates but doesn't define selection criteria.

3. **Runtime vs build-time:** Governed docs are static during a session. Should hashes be computed:
   - At API startup (once)?
   - At iteration start (every time)?
   - From a pre-computed manifest?

**Impact:** V11-6 scope is understated. Implementation requires architectural decisions.

**Recommendation R-1.6:** Before implementing V11-6:
1. Decide: compute hashes at startup or per-iteration?
2. Define canonical doc list (SR-DIRECTIVE mandatory; others optional)
3. Consider: create `GovernedManifest` at build/startup with pre-computed hashes
4. Update iterations handler to include refs from manifest

---

## 2. Dependency Feasibility

### Stated Dependencies (from plan §3)

| Phase | Stated Dependencies | Assessment |
|-------|---------------------|------------|
| V11-1 | None | ✅ Correct |
| V11-2 | None | ✅ Correct |
| V11-3 | None | ✅ Correct |
| V11-4 | V11-2 (init scripts) | ⚠️ See F-2.1 |
| V11-5 | V11-3 (health), V11-4 (harness) | ⚠️ See F-2.2 |
| V11-6 | V11-4 (E2E verification) | ✅ Correct |

**Finding F-2.1: V11-4 dependency on V11-2 is weaker than stated**

The plan says V11-4 depends on V11-2 "for test environment." However:
- `deploy/docker-compose.yml` already exists
- `deploy/init.sh` already exists
- V11-4 can run against existing infrastructure

**Impact:** V11-4 can start in parallel with V11-2 if using existing `deploy/` infrastructure.

**Recommendation R-2.1:** Update dependency diagram to show V11-4 can start once existing `docker-compose` environment is verified working.

---

**Finding F-2.2: Hidden dependency — V11-5 requires oracle suite registration mechanism**

V11-5 wants to register `suite:SR-SUITE-INTEGRATION` but:
- Registration requires `OracleSuiteRegistry::register_suite()` — exists
- But integration runner is in `sr-oracles`, not `sr-adapters`
- Cross-crate dependency: `sr-oracles` → `sr-adapters` for registration

**Impact:** Minor. Can be resolved by:
1. Moving registration logic to `sr-adapters`, OR
2. Exposing registration API from `sr-adapters` that `sr-oracles` calls

**Recommendation R-2.2:** Note that V11-5 may require cross-crate refactoring or API exposure.

---

## 3. Phase Ordering

**Finding F-3.1: Phase ordering is correct but parallelization opportunity is understated**

The plan shows:
```
V11-1, V11-2, V11-3 → V11-4 → V11-5 → V11-6
```

Given findings above, a more aggressive parallelization is possible:
```
V11-1 ─────────────────────────────────────────┐
V11-2 (audit existing infra) ──────────────────┤
V11-3 (add /ready + domain metrics) ───────────┤
                                               ↓
                                         V11-4 (E2E scenarios)
                                               ↓
                                         V11-5 (Integration suite registration)
                                               ↓
                                         V11-6 (GovernedArtifact refs)
```

**Since V11-2's actual scope is smaller (infrastructure exists), V11-4 can likely start earlier.**

**Recommendation R-3.1:** Update phase diagram to reflect that:
- V11-2 is primarily audit + documentation
- V11-4 can start once existing `docker-compose` is verified

---

## 4. Gap Coverage

### Deferred V10 Items

| Item | Plan Coverage | Assessment |
|------|---------------|------------|
| V10-G5 (Active exceptions in refs[]) | Not addressed | ⚠️ Missing |
| Tests 17-18 (ORACLE_GAP/EVIDENCE_MISSING) | V11-4 | ✅ Covered |

**Finding F-4.1: V10-G5 not addressed in V11**

SR-PLAN-GAP-ANALYSIS §4 lists:
> V10-G5: Active exceptions not included in IterationStarted.refs[] — Deferred to V11

But SR-PLAN-V11 does not include this item. V11-6 addresses GovernedArtifact refs but not active exceptions.

**Impact:** Gap remains unaddressed.

**Recommendation R-4.1:** Either:
1. Add V10-G5 to V11-6 scope (include active exceptions in refs[]), OR
2. Explicitly defer to V12 with rationale

---

### SR-PLAN-GAP-ANALYSIS Deliverables

| Deliverable | Plan Phase | Gap Analysis Status | Coherent? |
|-------------|------------|---------------------|-----------|
| D-16 | V11-1 | ❌ Not Started | ✅ Yes |
| D-26 | V11-5 | ❌ Not Started | ✅ Yes |
| D-32 | V11-2 | ⚠️ Partial | ⚠️ Plan overstates scope |
| D-33 | V11-3 | ⚠️ Partial | ⚠️ Plan understates existing |
| D-35 | V11-4 | ❌ Not Started | ⚠️ Plan understates existing harness |
| D-08 | V11-6 | ✅ Complete* | ⚠️ Refs exist but GovernedArtifact new |

**Finding F-4.2: Gap Analysis needs update after V11 review**

The Gap Analysis shows some deliverables as "Not Started" that actually have substantial infrastructure:
- D-32: `deploy/init.sh` is comprehensive
- D-33: `observability.rs` has metrics foundation
- D-35: `failure_modes.rs` has multiple scenarios implemented

**Recommendation R-4.2:** After V11 coherence review approval, update SR-PLAN-GAP-ANALYSIS to reflect actual codebase state:
- D-32: ⚠️ Partial → "init.sh exists; needs audit/documentation"
- D-33: ⚠️ Partial → "observability exists; needs domain metrics"
- D-35: ⚠️ Partial → "harness exists; needs scenario completion"

---

## 5. Risk Assessment

### Plan-Stated Risks (Validated)

| Risk | Plan Assessment | Review Assessment |
|------|-----------------|-------------------|
| Infisical API changes | Low/Medium | ✅ Accurate |
| E2E harness complexity | Medium/High | ⚠️ Lower than stated — harness exists |
| Test environment flakiness | Medium/Medium | ✅ Accurate |
| Scope creep | Medium/Medium | ✅ Accurate |

### Additional Risks Identified

**Risk R-5.1: GovernedArtifact hashing approach not defined**

V11-6 requires content hashing of governance documents. No approach is specified for:
- When to compute hashes (startup vs per-iteration)
- How to canonicalize markdown (affects hash stability)
- Which documents are mandatory vs optional

**Mitigation:** Define approach in V11-6 task description or open question section.

---

**Risk R-5.2: Integration suite registration across crates**

V11-5 requires registering `IntegrationRunner` (in `sr-oracles`) as an oracle suite (registry in `sr-adapters`). This cross-crate registration pattern isn't established.

**Mitigation:** Decide on registration approach (move code, expose API, or use startup hook).

---

## 6. Open Questions Validation

The plan lists 4 open questions. Assessment:

| Question | Assessment |
|----------|------------|
| 1. Infisical deployment (dev instance or mock?) | Valid — should be answered in V11-1 |
| 2. GovernedArtifact selection (mandatory vs optional) | Valid — critical for V11-6 |
| 3. E2E environment (Docker Compose vs K8s) | **Answered:** Docker Compose exists and is appropriate for dev |
| 4. Metrics backend (Prometheus assumed) | Valid — current is JSON; Prometheus optional |

**Finding F-6.1: Question 3 is already answered**

The plan asks "Docker Compose vs Kubernetes for test isolation?" — `deploy/docker-compose.yml` already exists and is appropriate for local dev/test. K8s is overkill for V11 scope.

**Recommendation R-6.1:** Remove question 3 or note that Docker Compose is the answer.

---

## 7. Summary of Findings

| ID | Severity | Summary |
|----|----------|---------|
| F-1.1 | Low | V11-1 scope smaller than estimated (Infisical more complete) |
| F-1.2 | Medium | V11-2 infrastructure mostly exists (`deploy/init.sh`, `docker-compose.yml`) |
| F-1.3 | Low | V11-3 foundation exists; domain metrics needed |
| F-1.4 | Low | V11-4 E2E harness exists; scenarios need completion |
| F-1.5 | Low | V11-5 integration runner exists; needs suite registration |
| F-1.6 | Medium | V11-6 complexity understated (content hashing approach undefined) |
| F-2.1 | Low | V11-4 can start earlier (dependency weaker) |
| F-2.2 | Low | V11-5 has hidden cross-crate dependency |
| F-3.1 | Low | Parallelization opportunity understated |
| F-4.1 | Medium | V10-G5 (active exceptions in refs) not addressed |
| F-4.2 | Low | Gap Analysis needs update after review |
| F-6.1 | Low | Open question 3 already answered |

---

## 8. Recommendations Summary

### Must Address Before Implementation

| ID | Recommendation |
|----|----------------|
| R-1.2 | Revise V11-2 to audit existing infra rather than create new |
| R-1.6 | Define GovernedArtifact hashing approach for V11-6 |
| R-4.1 | Address V10-G5 (active exceptions) or explicitly defer to V12 |

### Should Address (Improves Plan Quality)

| ID | Recommendation |
|----|----------------|
| R-1.1 | Update V11-1 scope to reflect existing Infisical implementation |
| R-1.3 | Acknowledge existing observability foundation in V11-3 |
| R-1.4 | Focus V11-4 on scenario completion, not infrastructure |
| R-1.5 | Note suite registration approach for V11-5 |
| R-2.1 | Update dependency diagram for V11-4 |
| R-2.2 | Note cross-crate dependency for V11-5 |
| R-3.1 | Update phase diagram for parallelization |
| R-4.2 | Plan Gap Analysis update after V11 |
| R-6.1 | Remove/answer open question 3 |

---

## 9. Verdict

### **APPROVE WITH REVISIONS** → **APPROVED**

SR-PLAN-V11 is coherent with the codebase and addresses the correct deliverables. The plan can proceed to implementation after:

1. ~~**Required:** Address V10-G5 (active exceptions in refs) — either add to V11-6 or explicitly defer~~ ✅ Incorporated
2. ~~**Required:** Define GovernedArtifact hashing approach for V11-6~~ ✅ Incorporated
3. ~~**Recommended:** Update V11-2 scope to reflect existing `deploy/` infrastructure~~ ✅ Incorporated
4. ~~**Recommended:** Update effort estimates (V11-1, V11-2, V11-4 likely smaller than stated)~~ ✅ Incorporated

**Status Update (2026-01-17):** All required revisions have been incorporated into SR-PLAN-V11. Implementation may proceed on `solver-ralph-11` branch.

---

## Appendix: Codebase Validation Evidence

### Files Verified to Exist

```
✅ crates/sr-adapters/src/infisical.rs (431 lines)
✅ crates/sr-adapters/src/restricted.rs
✅ crates/sr-adapters/src/oracle_suite.rs
✅ crates/sr-api/src/observability.rs (378 lines)
✅ crates/sr-api/src/handlers/iterations.rs (15,583 bytes)
✅ crates/sr-domain/src/entities.rs (ContentHash type)
✅ crates/sr-e2e-harness/src/harness.rs (24,914 bytes)
✅ crates/sr-e2e-harness/src/failure_modes.rs (45,553 bytes)
✅ crates/sr-e2e-harness/src/replay.rs (20,030 bytes)
✅ crates/sr-e2e-harness/src/transcript.rs (11,437 bytes)
✅ crates/sr-e2e-harness/src/main.rs (26,630 bytes)
✅ crates/sr-oracles/src/integration.rs (38,047 bytes)
✅ crates/sr-oracles/src/e2e.rs (21,136 bytes)
✅ scripts/check-deps.sh
✅ scripts/dev-setup.sh
✅ scripts/run-tests.sh
✅ deploy/docker-compose.yml (6,562 bytes)
✅ deploy/init.sh (21,387 bytes)
✅ deploy/init-scripts/postgres/01_init.sql
✅ migrations/001_event_store.sql through 009_loop_stop_triggers.sql
```

### API Endpoints Verified

```
✅ /health (main.rs:163)
✅ /api/v1/metrics (main.rs:166)
```

### E2E Harness CLI Flags Verified

```
✅ --replay
✅ --verify-determinism
✅ --oracle-failure
✅ --integrity-tamper
✅ --exception-waiver
```

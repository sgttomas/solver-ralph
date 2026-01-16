# SR-PLAN-V8: Oracle Runner & Semantic Suite Foundation

**Status:** Ready for Implementation (Amended)
**Created:** 2026-01-16
**Amended:** 2026-01-16 (Coherence Assessment)
**Depends On:** SR-PLAN-V7 (MVP Stabilization complete)
**Implements:** SR-CHARTER §Immediate Objective (Milestone 1: Semantic Work Unit Runtime)

---

## Amendments Summary (2026-01-16 Coherence Assessment)

| Amendment | Issue | Resolution |
|-----------|-------|------------|
| **A-1** | V8-2 assumed direct API call to oracle runner | Use **Event-Driven Worker** pattern (subscribes to `RunStarted`) |
| **A-2** | `OracleSuite` vs `OracleSuiteDefinition` unclear | Clarify: `OracleSuiteDefinition` = execution config, `OracleSuite` = domain entity with lifecycle |
| **A-3** | V8-5 proposed creating semantic types that already exist | Use existing types from `sr-domain/src/semantic_oracle.rs` |
| **A-4** | V8-1 didn't acknowledge existing registry implementation | Extract port trait from existing `OracleSuiteRegistry` in `sr-adapters` |

**Revised Effort:** 7-10 sessions (was 7-9)

---

## Executive Summary

SR-PLAN-V8 delivers the **Oracle Runner infrastructure** — the critical path blocker for Milestone 1 (MVP) completion. Without oracle execution, the system cannot produce Evidence Bundles, and without Evidence Bundles, verification gates cannot be satisfied (C-VER-1).

**Goal:** Enable end-to-end oracle execution that produces C-EVID-1 compliant Evidence Bundles.

**Key Deliverables (from SR-PLAN-GAP-ANALYSIS):**

| Deliverable | Title | Priority |
|-------------|-------|----------|
| **D-24** | Oracle runner service | Critical — Podman + gVisor sandbox |
| **D-25** | Core oracle suite | Critical — Build/unit/schema/lint oracles |
| **D-27** | Oracle integrity checks | Critical — TAMPER/GAP/FLAKE/ENV_MISMATCH |
| **D-39** | Semantic oracle integration | High — Meaning matrices, residual/coverage artifacts |

**Existing Infrastructure:**

The project already has substantial oracle runner infrastructure in `crates/sr-adapters/src/oracle_runner.rs` (~1000 lines):
- `PodmanOracleRunner<E>` with Podman command building
- `OracleSuiteDefinition`, `OracleDefinition`, `EnvironmentConstraints` types
- Evidence manifest building and storage integration
- Test mode for mock execution

**What V8 Must Complete:**
1. Wire oracle runner to API endpoints (`/runs`)
2. Implement oracle suite registry (DB storage/retrieval)
3. Implement integrity condition detection (C-OR-1 through C-OR-7)
4. Create core oracle suite package
5. Add semantic oracle output schemas (sr.semantic_eval.v1)

```
V8-1 (Registry) → V8-2 (API Integration) → V8-3 (Integrity Checks) → V8-4 (Core Suite) → V8-5 (Semantic Oracles)
     └────────────────────────────────────┘   └─────────────────────┘   └────────────────────────────────────────┘
              Oracle Infrastructure                 Contract Compliance            Oracle Implementation
```

---

## Table of Contents

1. [Current State](#1-current-state)
2. [Design Rationale](#2-design-rationale)
3. [Implementation Phases](#3-implementation-phases)
4. [Success Criteria](#4-success-criteria)
5. [Reference Documents](#5-reference-documents)

---

## 1. Current State

### 1.1 What SR-PLAN-V7 Delivered

The MVP is stabilized with:
- Integration tests for `/start` endpoint
- User-friendly error handling and toast notifications
- Attachment upload support (distinct from Evidence Bundles)
- Multiple iteration support for retry/iteration patterns

### 1.2 What's Missing for Milestone 1

| Gap | Contract | Impact |
|-----|----------|--------|
| Oracle runner not wired to API | — | Cannot trigger oracle runs |
| No oracle suite registry | C-OR-2 | Cannot pin suite identity |
| No integrity condition detection | C-OR-1..7 | Cannot detect TAMPER/GAP/FLAKE/ENV_MISMATCH |
| No core oracle suite | D-25 | No oracles to execute |
| No semantic oracle outputs | SR-SEMANTIC-ORACLE-SPEC | Cannot satisfy semantic gates |

### 1.3 Existing Infrastructure Analysis

**`crates/sr-adapters/src/oracle_runner.rs` (1027 lines):**

| Component | Status | Notes |
|-----------|--------|-------|
| `PodmanOracleRunner<E>` | ✅ Implemented | Full Podman command building |
| `OracleSuiteDefinition` | ✅ Implemented | Suite identity + oracles |
| `execute_suite()` | ✅ Implemented | Orchestrates oracle execution |
| `execute_single_oracle()` | ✅ Implemented | Runs individual oracle in container |
| `build_evidence_manifest()` | ✅ Implemented | Creates C-EVID-1 manifest |
| Environment fingerprint | ✅ Implemented | Captures execution environment |
| Test mode | ✅ Implemented | Mock execution for tests |
| Suite registry | ❌ In-memory only | Need DB persistence |
| Candidate path resolution | ❌ Placeholder | Uses `/tmp/candidates/{id}` |
| Integrity detection | ❌ Not implemented | TAMPER/GAP/FLAKE/ENV_MISMATCH |

**`crates/sr-ports/src/lib.rs`:**

| Component | Status |
|-----------|--------|
| `OracleRunner` trait | ✅ Defined |
| `OracleRunResult` | ✅ Defined |
| `OracleRunnerError` | ✅ Defined with variants |

**`crates/sr-api/src/handlers/runs.rs`:**

| Component | Status | Notes |
|-----------|--------|-------|
| `POST /runs` handler | ✅ Complete | Creates `RunStarted` event (event-sourced) |
| `POST /runs/:id/complete` | ✅ Complete | Creates `RunCompleted` event |

> **AMENDMENT A-1:** The runs handler uses **event sourcing** — it does NOT call the oracle runner directly. It creates a `RunStarted` event, and oracle execution must happen via a separate worker that subscribes to these events.

**`crates/sr-adapters/src/oracle_suite.rs`:**

| Component | Status | Notes |
|-----------|--------|-------|
| `OracleSuiteRegistry` struct | ✅ Implemented | In-memory registry with core suites |
| `VerificationProfile` | ✅ Implemented | Profile definitions |
| `IntegrityCondition` enum | ✅ Implemented | 6 variants defined |
| `WaivableCondition` enum | ✅ Implemented | 6 variants defined |
| Core suite factories | ✅ Implemented | GOV, CORE, FULL suites |

> **AMENDMENT A-4:** The in-memory registry already exists with working API endpoints. V8-1 should extract a port trait and add PostgreSQL persistence, not create from scratch.

**`crates/sr-domain/src/semantic_oracle.rs` (~1024 lines):**

| Component | Status | Notes |
|-----------|--------|-------|
| `SemanticEvalResult` | ✅ Implemented | Full `sr.semantic_eval.v1` schema |
| `SemanticMetrics` | ✅ Implemented | Residual, coverage, violations |
| `DecisionStatus` | ✅ Implemented | Pass/Fail/Indeterminate |
| `ResidualReport` | ✅ Implemented | Output artifact schema |
| `CoverageReport` | ✅ Implemented | Output artifact schema |
| `ViolationsReport` | ✅ Implemented | Output artifact schema |

> **AMENDMENT A-3:** Semantic oracle types already exist. V8-5 should focus on oracle suite packaging and container creation, not type definitions.

---

## 2. Design Rationale

### 2.1 Why Registry First (V8-1)

Per C-OR-2, runs MUST pin oracle suite identity at start. Before we can execute oracles through the API, we need:
- Persistent storage of suite definitions
- Ability to retrieve suite by ID and verify hash
- Registration API for new suites

The existing in-memory cache (`suite_cache`) is insufficient for production.

### 2.2 Why API Integration Before Integrity (V8-2)

We need a working end-to-end flow before adding integrity checks:
1. API receives run request
2. Retrieves suite from registry
3. Resolves candidate workspace
4. Executes oracles via `PodmanOracleRunner`
5. Stores evidence bundle
6. Returns run result

This validates the existing implementation works through the API layer.

### 2.3 Why Integrity Checks Are Central (V8-3)

Per SR-CONTRACT §6, integrity conditions are fundamental to trust:

| Condition | Contract | Detection Point |
|-----------|----------|-----------------|
| `ORACLE_TAMPER` | C-OR-2 | Suite hash mismatch at run start |
| `ORACLE_GAP` | C-OR-4 | Missing required oracle result |
| `ORACLE_FLAKE` | C-OR-5 | Non-deterministic required oracle |
| `ORACLE_ENV_MISMATCH` | C-OR-3 | Environment constraint violation |

These MUST halt progression and escalate (C-OR-7).

### 2.4 Why Core Suite Before Semantic (V8-4)

Before implementing semantic oracles (which emit meaning matrices), we need basic oracles that validate:
- Build success (compilation)
- Unit test pass
- Schema validation
- Lint checks

These provide the foundation for understanding oracle packaging and execution.

### 2.5 Why Semantic Last (V8-5)

Semantic oracles (per SR-SEMANTIC-ORACLE-SPEC) have additional requirements:
- `sr.semantic_eval.v1` output schema
- Residual/coverage/violations artifacts
- Semantic set binding in suite hash

Building on working core oracles, V8-5 adds semantic measurement capabilities.

---

## 3. Implementation Phases

### Phase V8-1: Oracle Suite Registry (AMENDED per A-4)

**Objective:** Add persistent storage to the existing oracle suite registry.

> **Amendment A-4:** The in-memory `OracleSuiteRegistry` already exists in `sr-adapters/src/oracle_suite.rs` with 6 working API endpoints in `sr-api/src/handlers/oracles.rs`. This phase extracts a port trait and adds PostgreSQL persistence.

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `crates/sr-ports/src/lib.rs` | MODIFY | Add `OracleSuiteRegistry` port trait |
| `crates/sr-adapters/src/postgres_oracle_registry.rs` | CREATE | PostgreSQL adapter implementing port |
| `crates/sr-adapters/src/oracle_suite.rs` | MODIFY | Implement port trait on existing struct |
| `crates/sr-api/src/main.rs` | MODIFY | Use DB-backed registry |
| `migrations/008_oracle_suite_registry.sql` | CREATE | Database schema |

**Database Schema:**

```sql
CREATE TABLE oracle_suites (
    suite_id VARCHAR(255) PRIMARY KEY,
    suite_hash VARCHAR(64) NOT NULL,
    oci_image VARCHAR(512) NOT NULL,
    oci_image_digest VARCHAR(128) NOT NULL,
    environment_constraints JSONB NOT NULL,
    oracles JSONB NOT NULL,
    metadata JSONB DEFAULT '{}',
    registered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    registered_by VARCHAR(255) NOT NULL,
    status VARCHAR(32) NOT NULL DEFAULT 'active'
);

CREATE INDEX idx_oracle_suites_hash ON oracle_suites(suite_hash);
CREATE INDEX idx_oracle_suites_status ON oracle_suites(status);
```

**Existing Endpoints (already implemented in `handlers/oracles.rs`):**

- `GET /api/v1/oracles/suites` — List all registered oracle suites
- `GET /api/v1/oracles/suites/:suite_id` — Get suite detail with oracles
- `POST /api/v1/oracles/suites` — Register a new oracle suite
- `GET /api/v1/oracles/profiles` — List verification profiles
- `GET /api/v1/oracles/profiles/:id` — Get profile detail
- `POST /api/v1/oracles/profiles` — Register a new verification profile

> These endpoints currently use the in-memory `OracleSuiteRegistry`. No endpoint changes needed.

**Type Relationship (Amendment A-2):**

- `OracleSuiteDefinition` (in `sr-adapters/src/oracle_runner.rs`) = **immutable execution configuration**
- `OracleSuiteRecord` (new in `sr-ports`) = **stored entity with registry metadata**

```rust
// Existing in sr-adapters - the execution config
pub struct OracleSuiteDefinition {
    pub suite_id: String,
    pub suite_hash: String,
    pub oci_image: String,
    pub oci_image_digest: String,
    pub environment_constraints: EnvironmentConstraints,
    pub oracles: Vec<OracleDefinition>,
    pub metadata: BTreeMap<String, serde_json::Value>,
}

// New in sr-ports - the stored record with lifecycle
pub struct OracleSuiteRecord {
    // All fields from OracleSuiteDefinition, plus:
    pub registered_at: DateTime<Utc>,
    pub registered_by_kind: String,
    pub registered_by_id: String,
    pub status: OracleSuiteStatus,
}

// Conversion for oracle runner usage
impl From<OracleSuiteRecord> for OracleSuiteDefinition { ... }
```

**Port Trait (new in `sr-ports/src/lib.rs`):**

```rust
#[async_trait]
pub trait OracleSuiteRegistry: Send + Sync {
    async fn register(&self, suite: OracleSuiteDefinition, actor: &ActorRef)
        -> Result<OracleSuite, RegistryError>;

    async fn get(&self, suite_id: &str)
        -> Result<Option<OracleSuite>, RegistryError>;

    async fn get_by_hash(&self, suite_hash: &str)
        -> Result<Option<OracleSuite>, RegistryError>;

    async fn list(&self, filter: SuiteFilter)
        -> Result<Vec<OracleSuite>, RegistryError>;

    async fn deprecate(&self, suite_id: &str, actor: &ActorRef)
        -> Result<(), RegistryError>;
}
```

**Acceptance Criteria:**
- [ ] `POST /oracle-suites` registers suite with computed hash
- [ ] `GET /oracle-suites/{id}` retrieves suite definition
- [ ] Suite hash computed from deterministic serialization of definition
- [ ] PostgreSQL adapter implements `OracleSuiteRegistry`
- [ ] `cargo test --package sr-api` passes
- [ ] `cargo test --package sr-adapters` passes

**Effort:** ~1 session

---

### Phase V8-2: Oracle Execution Worker (AMENDED per A-1)

**Objective:** Implement event-driven oracle execution that subscribes to `RunStarted` events.

> **Amendment A-1:** The `POST /runs` handler uses **event sourcing** — it creates a `RunStarted` event and returns immediately. It does NOT call the oracle runner directly. Oracle execution must happen via a separate worker process that subscribes to `RunStarted` events.

**Architecture:**

```
POST /runs → RunStarted event → Event Store
                                    │
                                    ▼
                          OracleExecutionWorker
                                    │
                    ┌───────────────┼───────────────┐
                    ▼               ▼               ▼
              Resolve Suite   Materialize WS   Execute Oracles
                    │               │               │
                    └───────────────┼───────────────┘
                                    ▼
                          OracleExecutionCompleted event
                                    │
                                    ▼
                          RunCompleted event (via existing handler)
```

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `crates/sr-adapters/src/oracle_worker.rs` | CREATE | Event-driven oracle execution worker |
| `crates/sr-adapters/src/candidate_store.rs` | CREATE | Candidate workspace materializer |
| `crates/sr-domain/src/events.rs` | MODIFY | Add `OracleExecutionStarted`, `OracleExecutionCompleted` |
| `crates/sr-api/src/main.rs` | MODIFY | Start worker alongside API server |

**New Event Types:**

```rust
/// Emitted when oracle execution begins (after worker picks up RunStarted)
pub struct OracleExecutionStarted {
    pub run_id: String,
    pub candidate_id: String,
    pub suite_id: String,
    pub suite_hash: String,
    pub workspace_path: String,
    pub started_at: DateTime<Utc>,
}

/// Emitted when oracle execution completes (success or failure)
pub struct OracleExecutionCompleted {
    pub run_id: String,
    pub candidate_id: String,
    pub suite_id: String,
    pub status: OracleStatus,  // Pass/Fail/Error
    pub evidence_bundle_hash: Option<String>,
    pub environment_fingerprint: serde_json::Value,
    pub duration_ms: u64,
    pub completed_at: DateTime<Utc>,
    pub error: Option<String>,
}
```

**Oracle Execution Worker:**

```rust
pub struct OracleExecutionWorker<R, W, E>
where
    R: OracleSuiteRegistry,
    W: CandidateWorkspace,
    E: EvidenceStore,
{
    event_store: Arc<dyn EventStore>,
    oracle_registry: Arc<R>,
    oracle_runner: Arc<PodmanOracleRunner<E>>,
    candidate_workspace: Arc<W>,
}

impl<R, W, E> OracleExecutionWorker<R, W, E> {
    /// Poll for RunStarted events and execute oracles
    pub async fn run(&self) -> Result<(), WorkerError> {
        loop {
            // 1. Poll for unprocessed RunStarted events
            let events = self.poll_run_started_events().await?;

            for event in events {
                // 2. Extract run details from event
                let run_id = &event.stream_id;
                let payload: RunStartedPayload = serde_json::from_value(event.payload)?;

                // 3. Retrieve and validate suite
                let suite = self.oracle_registry
                    .get(&payload.oracle_suite_id)
                    .await?
                    .ok_or(WorkerError::SuiteNotFound)?;

                if suite.suite_hash != payload.oracle_suite_hash {
                    // Emit integrity violation event
                    self.emit_integrity_violation(run_id, IntegrityCondition::OracleTamper { ... }).await?;
                    continue;
                }

                // 4. Materialize candidate workspace
                let workspace = self.candidate_workspace
                    .materialize(&payload.candidate_id)
                    .await?;

                // 5. Emit OracleExecutionStarted
                self.emit_execution_started(run_id, &payload, &workspace).await?;

                // 6. Execute oracle suite
                let result = self.oracle_runner
                    .execute_suite(
                        &payload.candidate_id,
                        &payload.oracle_suite_id,
                        &payload.oracle_suite_hash,
                        &workspace.path,
                    )
                    .await;

                // 7. Emit OracleExecutionCompleted
                self.emit_execution_completed(run_id, result).await?;
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}
```

**Candidate Workspace Resolution:**

The oracle runner currently uses a placeholder path (`/tmp/candidates/{id}`). We need to:
1. Resolve candidate from Candidate projection
2. Materialize workspace directory with candidate content
3. Pass workspace path to oracle runner

```rust
#[async_trait]
pub trait CandidateWorkspace: Send + Sync {
    /// Materialize candidate content to a temporary workspace directory
    async fn materialize(&self, candidate_id: &str)
        -> Result<TempWorkspace, WorkspaceError>;
}

pub struct TempWorkspace {
    pub path: PathBuf,
    pub candidate_id: String,
    pub candidate_hash: String,
    // Implements Drop for cleanup
}
```

**Acceptance Criteria:**
- [ ] Worker subscribes to `RunStarted` events
- [ ] Worker retrieves suite from registry and validates hash
- [ ] Worker materializes candidate workspace
- [ ] Worker executes oracles via `PodmanOracleRunner` (test mode)
- [ ] Worker emits `OracleExecutionStarted` and `OracleExecutionCompleted` events
- [ ] Evidence bundle stored in MinIO
- [ ] Existing `POST /runs/:id/complete` can consume worker results
- [ ] End-to-end integration test passes
- [ ] `cargo test --package sr-adapters` passes

**Effort:** ~2-3 sessions (increased from 1-2 due to event-driven architecture)

---

### Phase V8-3: Oracle Integrity Condition Detection

**Objective:** Implement detection and escalation for TAMPER/GAP/FLAKE/ENV_MISMATCH.

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `crates/sr-domain/src/integrity.rs` | CREATE | Integrity condition types |
| `crates/sr-adapters/src/oracle_runner.rs` | MODIFY | Add integrity detection |
| `crates/sr-api/src/handlers/runs.rs` | MODIFY | Handle integrity violations |
| `crates/sr-domain/src/events.rs` | MODIFY | Add integrity events |

**Integrity Conditions (per SR-CONTRACT §6):**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrityCondition {
    /// C-OR-2: Suite hash mismatch at run start
    OracleTamper {
        expected_hash: String,
        actual_hash: String,
        suite_id: String,
    },

    /// C-OR-4: Missing required oracle result
    OracleGap {
        missing_oracles: Vec<String>,
        suite_id: String,
    },

    /// C-OR-5: Non-deterministic required oracle
    OracleFlake {
        oracle_id: String,
        run_1_hash: String,
        run_2_hash: String,
        description: String,
    },

    /// C-OR-3: Environment constraint violation
    OracleEnvMismatch {
        constraint: String,
        expected: String,
        actual: String,
    },
}

impl IntegrityCondition {
    pub fn severity(&self) -> Severity {
        Severity::Blocking // All integrity conditions halt progression
    }

    pub fn requires_escalation(&self) -> bool {
        true // C-OR-7: Must escalate
    }
}
```

**Detection Points:**

| Condition | Detection Point | Implementation |
|-----------|-----------------|----------------|
| `ORACLE_TAMPER` | Before execution | Compare registered hash vs request hash |
| `ORACLE_GAP` | After execution | Check all required oracles have results |
| `ORACLE_FLAKE` | After execution | Detect via determinism declaration or repeat runs |
| `ORACLE_ENV_MISMATCH` | Before/during execution | Validate environment fingerprint vs constraints |

**Environment Fingerprint Fields (per SR-SPEC §4.5):**

The `environment_fingerprint` MUST include the following fields for ENV_MISMATCH detection:

| Field | Description | Example |
|-------|-------------|---------|
| `container_image_digest` | OCI image pinned by digest | `sha256:abc123...` |
| `runtime` | Runtime name/version | `runsc 2024.01.1` (gVisor) |
| `os_arch` | Operating system and architecture | `linux/amd64` |
| `tool_versions` | Critical tool versions as declared by suite | `{"rustc": "1.75.0", "node": "20.10.0"}` |
| `network_mode` | Network access mode | `disabled` (default for required oracles) |

`ORACLE_ENV_MISMATCH` MUST be raised when the run-time fingerprint does not match the suite's declared `environment_constraints`.

**ORACLE_TAMPER Detection:**

```rust
impl PodmanOracleRunner {
    pub async fn execute_suite_with_integrity(
        &self,
        candidate_id: &str,
        suite_id: &str,
        expected_suite_hash: &str,
        candidate_path: &PathBuf,
    ) -> Result<OracleRunResult, IntegrityOrRunError> {
        // 1. Retrieve suite
        let suite = self.get_suite(suite_id).await
            .ok_or(OracleRunnerError::SuiteNotFound { suite_id: suite_id.to_string() })?;

        // 2. Check for TAMPER (C-OR-2)
        if suite.suite_hash != expected_suite_hash {
            return Err(IntegrityOrRunError::Integrity(
                IntegrityCondition::OracleTamper {
                    expected_hash: expected_suite_hash.to_string(),
                    actual_hash: suite.suite_hash.clone(),
                    suite_id: suite_id.to_string(),
                }
            ));
        }

        // 3. Validate environment constraints (C-OR-3)
        self.validate_environment(&suite.environment_constraints)?;

        // 4. Execute oracles
        let result = self.execute_suite_internal(&suite, candidate_id, candidate_path).await?;

        // 5. Check for GAP (C-OR-4)
        self.check_for_gaps(&suite, &result)?;

        Ok(result)
    }
}
```

**ORACLE_GAP Detection:**

```rust
fn check_for_gaps(
    &self,
    suite: &OracleSuiteDefinition,
    result: &OracleRunResult,
) -> Result<(), IntegrityCondition> {
    let required_oracles: HashSet<_> = suite.oracles
        .iter()
        .filter(|o| o.classification == OracleClassification::Required)
        .map(|o| &o.oracle_id)
        .collect();

    let executed_oracles: HashSet<_> = result.oracle_results
        .iter()
        .map(|r| &r.oracle_id)
        .collect();

    let missing: Vec<_> = required_oracles
        .difference(&executed_oracles)
        .map(|s| s.to_string())
        .collect();

    if !missing.is_empty() {
        return Err(IntegrityCondition::OracleGap {
            missing_oracles: missing,
            suite_id: suite.suite_id.clone(),
        });
    }

    Ok(())
}
```

**Integrity Event:**

```rust
pub struct IntegrityViolationDetected {
    pub run_id: String,
    pub candidate_id: String,
    pub suite_id: String,
    pub condition: IntegrityCondition,
    pub detected_at: DateTime<Utc>,
    pub requires_escalation: bool,
}
```

**API Response for Integrity Violations:**

```json
{
  "error": "INTEGRITY_VIOLATION",
  "condition": "ORACLE_TAMPER",
  "details": {
    "expected_hash": "sha256:abc...",
    "actual_hash": "sha256:def...",
    "suite_id": "suite:core-v1"
  },
  "message": "Oracle suite hash mismatch. Suite may have been modified since registration.",
  "requires_escalation": true
}
```

**Acceptance Criteria:**
- [ ] ORACLE_TAMPER detected when suite hash mismatches
- [ ] ORACLE_GAP detected when required oracle missing from results
- [ ] ORACLE_ENV_MISMATCH detected when environment constraints violated
- [ ] Integrity violations halt run and return structured error
- [ ] `IntegrityViolationDetected` event emitted
- [ ] Integration test validates each integrity condition
- [ ] `cargo test --package sr-api` passes

**Effort:** ~1-2 sessions

---

### Phase V8-4: Core Oracle Suite

**Objective:** Create a minimal oracle suite for build/unit/schema/lint checks.

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `oracle-suites/core-v1/Dockerfile` | CREATE | Container image definition |
| `oracle-suites/core-v1/suite.json` | CREATE | Suite definition manifest |
| `oracle-suites/core-v1/oracles/` | CREATE | Oracle scripts |
| `oracle-suites/core-v1/README.md` | CREATE | Suite documentation |

**Suite Definition (`suite.json`):**

```json
{
  "suite_id": "suite:sr-core-v1",
  "suite_version": "1.0.0",
  "description": "Core verification oracles for Semantic Ralph projects",
  "oci_image": "ghcr.io/solver-ralph/oracle-suite-core:v1",
  "environment_constraints": {
    "runtime": "runsc",
    "network": "disabled",
    "cpu_arch": "amd64",
    "os": "linux",
    "workspace_readonly": true
  },
  "oracles": [
    {
      "oracle_id": "oracle:build",
      "oracle_name": "Build Check",
      "description": "Verifies project builds successfully",
      "command": "/oracles/build.sh",
      "timeout_seconds": 300,
      "classification": "required",
      "expected_outputs": [
        {
          "path": "reports/build.json",
          "content_type": "application/json",
          "required": true
        }
      ]
    },
    {
      "oracle_id": "oracle:unit-tests",
      "oracle_name": "Unit Tests",
      "description": "Runs unit test suite",
      "command": "/oracles/unit-tests.sh",
      "timeout_seconds": 600,
      "classification": "required",
      "expected_outputs": [
        {
          "path": "reports/unit-tests.json",
          "content_type": "application/json",
          "required": true
        }
      ]
    },
    {
      "oracle_id": "oracle:schema-validation",
      "oracle_name": "Schema Validation",
      "description": "Validates JSON schemas and types",
      "command": "/oracles/schema-validation.sh",
      "timeout_seconds": 120,
      "classification": "required",
      "expected_outputs": [
        {
          "path": "reports/schema.json",
          "content_type": "application/json",
          "required": true
        }
      ]
    },
    {
      "oracle_id": "oracle:lint",
      "oracle_name": "Lint Check",
      "description": "Runs code linting",
      "command": "/oracles/lint.sh",
      "timeout_seconds": 180,
      "classification": "advisory",
      "expected_outputs": [
        {
          "path": "reports/lint.json",
          "content_type": "application/json",
          "required": false
        }
      ]
    }
  ]
}
```

**Oracle Output Schema (for all oracles):**

```json
{
  "schema": "sr.oracle_result.v1",
  "oracle_id": "oracle:build",
  "status": "PASS",
  "started_at": "2026-01-16T12:00:00Z",
  "completed_at": "2026-01-16T12:01:30Z",
  "duration_ms": 90000,
  "summary": "Build completed successfully",
  "details": {
    "target": "release",
    "warnings": 3,
    "errors": 0
  },
  "artifacts": [
    {
      "name": "build-log.txt",
      "hash": "sha256:...",
      "size_bytes": 12345
    }
  ]
}
```

**Dockerfile:**

```dockerfile
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    nodejs npm \
    python3 python3-pip \
    jq \
    && rm -rf /var/lib/apt/lists/*

# Install oracle scripts
COPY oracles/ /oracles/
RUN chmod +x /oracles/*.sh

# Set working directory
WORKDIR /workspace

# Default command (overridden by oracle execution)
CMD ["/bin/sh"]
```

**Build Oracle Script (`oracles/build.sh`):**

```bash
#!/bin/bash
set -e

OUTPUT_DIR="${SCRATCH_MOUNT:-/scratch}/reports"
mkdir -p "$OUTPUT_DIR"

cd "${WORKSPACE_MOUNT:-/workspace}"

# Detect project type and build
if [ -f "Cargo.toml" ]; then
    cargo build --release 2>&1 | tee /tmp/build.log
    STATUS=$?
elif [ -f "package.json" ]; then
    npm ci && npm run build 2>&1 | tee /tmp/build.log
    STATUS=$?
else
    echo "Unknown project type" | tee /tmp/build.log
    STATUS=1
fi

# Generate report
cat > "$OUTPUT_DIR/build.json" << EOF
{
  "schema": "sr.oracle_result.v1",
  "oracle_id": "oracle:build",
  "status": $([ $STATUS -eq 0 ] && echo '"PASS"' || echo '"FAIL"'),
  "completed_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "exit_code": $STATUS,
  "summary": "Build $([ $STATUS -eq 0 ] && echo 'succeeded' || echo 'failed')"
}
EOF

exit $STATUS
```

**Suite Registration:**

After building the container image:

```bash
# Build and push image
docker build -t ghcr.io/solver-ralph/oracle-suite-core:v1 .
docker push ghcr.io/solver-ralph/oracle-suite-core:v1

# Get image digest
DIGEST=$(docker inspect --format='{{index .RepoDigests 0}}' ghcr.io/solver-ralph/oracle-suite-core:v1)

# Register suite via API
curl -X POST http://localhost:8080/api/v1/oracle-suites \
  -H "Content-Type: application/json" \
  -d @suite.json
```

**Acceptance Criteria:**
- [ ] Dockerfile builds successfully
- [ ] All 4 oracles execute in container
- [ ] Oracle output conforms to `sr.oracle_result.v1` schema
- [ ] Suite registers successfully via API
- [ ] End-to-end test: run suite against test candidate
- [ ] Evidence bundle produced with all oracle results
- [ ] Documentation complete

**Effort:** ~2 sessions

---

### Phase V8-5: Semantic Oracle Integration (AMENDED per A-3)

**Objective:** Create semantic oracle suite container using existing type definitions.

> **Amendment A-3:** The semantic oracle types already exist in `sr-domain/src/semantic_oracle.rs` (~1024 lines). This phase focuses on **container packaging** and **oracle suite creation**, not type definitions.

**Existing Types (DO NOT RECREATE):**

| Type | Location | Purpose |
|------|----------|---------|
| `SemanticEvalResult` | `semantic_oracle.rs:178-192` | Main evaluation result |
| `SemanticMetrics` | `semantic_oracle.rs:195-201` | Residual, coverage, violations |
| `EvalDecision` | `semantic_oracle.rs:204-210` | Pass/Fail decision |
| `DecisionStatus` | `semantic_oracle.rs:156-160` | Pass/Fail/Indeterminate |
| `ResidualReport` | `semantic_oracle.rs:263-273` | Output artifact |
| `CoverageReport` | `semantic_oracle.rs:276-286` | Output artifact |
| `ViolationsReport` | `semantic_oracle.rs:289-298` | Output artifact |

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `oracle-suites/semantic-v1/` | CREATE | Semantic oracle suite container |
| `oracle-suites/semantic-v1/Dockerfile` | CREATE | Container image definition |
| `oracle-suites/semantic-v1/suite.json` | CREATE | Suite manifest |
| `crates/sr-domain/src/semantic_oracle.rs` | MODIFY | Add `Waived` to `DecisionStatus` if needed |

**Semantic Oracle Suite:**

Per SR-SEMANTIC-ORACLE-SPEC §3, semantic oracles MUST produce:
- `reports/semantic/residual.json`
- `reports/semantic/coverage.json`
- `reports/semantic/violations.json`

**Suite Definition (`semantic-v1/suite.json`):**

```json
{
  "suite_id": "suite:sr-semantic-v1",
  "suite_version": "1.0.0",
  "description": "Semantic evaluation oracles for meaning matrix verification",
  "oci_image": "ghcr.io/solver-ralph/oracle-suite-semantic:v1",
  "environment_constraints": {
    "runtime": "runsc",
    "network": "disabled",
    "cpu_arch": "amd64",
    "os": "linux",
    "workspace_readonly": true
  },
  "semantic_set_binding": {
    "semantic_set_id": "semantic-set:sr-meaning-matrix-v1",
    "semantic_set_hash": "sha256:..."
  },
  "oracles": [
    {
      "oracle_id": "oracle:semantic-eval",
      "oracle_name": "Semantic Evaluation",
      "description": "Evaluates candidate against meaning matrix",
      "command": "/oracles/semantic-eval.sh",
      "timeout_seconds": 300,
      "classification": "required",
      "expected_outputs": [
        {
          "path": "reports/semantic/residual.json",
          "content_type": "application/json",
          "required": true
        },
        {
          "path": "reports/semantic/coverage.json",
          "content_type": "application/json",
          "required": true
        },
        {
          "path": "reports/semantic/violations.json",
          "content_type": "application/json",
          "required": true
        },
        {
          "path": "reports/semantic/eval.json",
          "content_type": "application/json",
          "required": true
        }
      ]
    }
  ]
}
```

**Semantic Evaluation Result Schema (`sr.semantic_eval.v1`):**

Per SR-SEMANTIC-ORACLE-SPEC §4:

```json
{
  "schema": "sr.semantic_eval.v1",
  "candidate_id": "sha256:...",
  "procedure_template_id": "proc:GENERIC-KNOWLEDGE-WORK",
  "stage_id": "stage:SEMANTIC_EVAL",
  "oracle_suite_id": "suite:sr-semantic-v1",
  "oracle_suite_hash": "sha256:...",
  "semantic_set": {
    "semantic_set_id": "semantic-set:sr-meaning-matrix-v1",
    "semantic_set_hash": "sha256:..."
  },
  "metrics": {
    "residual_norm": 0.12,
    "coverage": {
      "ontology": 0.95,
      "epistemology": 0.88,
      "semantics": 0.92
    },
    "violations": []
  },
  "decision": {
    "status": "PASS",
    "rule_id": "rule:semantic-threshold-v1",
    "thresholds": {
      "residual_norm_max": 0.2,
      "coverage_min": 0.8
    }
  },
  "notes": "Candidate satisfies semantic constraints for stage advancement"
}
```

**Optional Type Modification:**

If the `Waived` decision status is needed, add it to the existing enum:

```rust
// In sr-domain/src/semantic_oracle.rs
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DecisionStatus {
    Pass,
    Fail,
    Indeterminate,
    Waived,  // ADD THIS IF NEEDED
}
```

**Output Schemas (already defined in `semantic_oracle.rs`):**

The existing types produce these JSON schemas — no changes needed:

**Residual Output Schema (`reports/semantic/residual.json`):**

```json
{
  "schema": "sr.semantic_residual.v1",
  "candidate_id": "sha256:...",
  "semantic_set_id": "semantic-set:sr-meaning-matrix-v1",
  "residual_vector": [0.02, -0.05, 0.08, 0.01],
  "residual_norm": 0.12,
  "axis_residuals": {
    "ontology": 0.02,
    "epistemology": -0.05,
    "semantics": 0.08,
    "pragmatics": 0.01
  }
}
```

**Coverage Output Schema (`reports/semantic/coverage.json`):**

```json
{
  "schema": "sr.semantic_coverage.v1",
  "candidate_id": "sha256:...",
  "semantic_set_id": "semantic-set:sr-meaning-matrix-v1",
  "overall_coverage": 0.91,
  "axis_coverage": {
    "ontology": 0.95,
    "epistemology": 0.88,
    "semantics": 0.92,
    "pragmatics": 0.89
  },
  "uncovered_axes": []
}
```

**Violations Output Schema (`reports/semantic/violations.json`):**

```json
{
  "schema": "sr.semantic_violations.v1",
  "candidate_id": "sha256:...",
  "semantic_set_id": "semantic-set:sr-meaning-matrix-v1",
  "violations": [],
  "total_violations": 0,
  "blocking_violations": 0
}
```

**Acceptance Criteria:**
- [ ] Semantic oracle suite Dockerfile builds successfully
- [ ] Suite definition conforms to SR-SEMANTIC-ORACLE-SPEC
- [ ] `semantic_set_hash` incorporated into `oracle_suite_hash` (§2)
- [ ] All 3 required artifacts produced: residual, coverage, violations
- [ ] Outputs conform to existing `sr-domain` type definitions
- [ ] `Waived` variant added to `DecisionStatus` if needed
- [ ] Integration test validates semantic oracle execution
- [ ] Evidence bundle contains semantic artifacts
- [ ] Suite registers via existing API endpoint

**Effort:** ~1-2 sessions (reduced from 2 — types already exist)

---

## 4. Success Criteria

### 4.1 Checkpoint: Oracle Infrastructure (after V8-2)

- [ ] Oracle suites can be registered and retrieved
- [ ] `POST /runs` triggers oracle execution
- [ ] Evidence bundles stored with C-EVID-1 compliant manifests
- [ ] End-to-end test passes in test mode (mock containers)

### 4.2 Checkpoint: Contract Compliance (after V8-3)

- [ ] ORACLE_TAMPER detected and escalated (C-OR-2)
- [ ] ORACLE_GAP detected and escalated (C-OR-4)
- [ ] ORACLE_ENV_MISMATCH detected and escalated (C-OR-3)
- [ ] Integrity violations halt progression (C-OR-7)
- [ ] All integrity events recorded for audit

### 4.3 Checkpoint: Core Oracles (after V8-4)

- [ ] Core oracle suite builds and registers
- [ ] Build/unit/schema/lint oracles execute
- [ ] Oracle outputs conform to `sr.oracle_result.v1`
- [ ] Evidence bundle contains all oracle results

### 4.4 Checkpoint: Semantic Oracles (after V8-5)

- [ ] Semantic oracle suite builds and registers
- [ ] Semantic evaluation produces residual/coverage/violations
- [ ] Results conform to `sr.semantic_eval.v1`
- [ ] Semantic set binding verified in suite hash
- [ ] Stage gates can use semantic oracle results

### 4.5 Milestone 1 Readiness

After V8-5, the following Milestone 1 deliverables are complete:

| Deliverable | Status |
|-------------|--------|
| D-24: Oracle runner service | ✅ Complete |
| D-25: Core oracle suite | ✅ Complete |
| D-27: Oracle integrity checks | ✅ Complete |
| D-39: Semantic oracle integration | ✅ Complete |

Remaining Milestone 1 work (for SR-PLAN-V9):
- D-23: Semantic worker runtime
- D-41: Branch-0 bootstrap
- D-36: Semantic worker contract

---

## 5. Reference Documents

### Platform Specifications

| Document | Relevant Sections |
|----------|-------------------|
| SR-CONTRACT | §6 (C-OR-1..7), §7 (C-EVID-1..6), §5 (C-VER-1..4) |
| SR-SPEC | §4.5 (Oracle runtime pattern), §1.9 (Evidence bundle model) |
| SR-SEMANTIC-ORACLE-SPEC | §2 (Suite identity), §3 (Required outputs), §4 (Result schema) |
| SR-TYPES | §4 (Platform domain types) |

### Prior Plans

| Plan | Status | Relevance |
|------|--------|-----------|
| SR-PLAN-V7 | Complete | MVP stabilization foundation |
| SR-PLAN-GAP-ANALYSIS | Living | Deliverable tracking |

### Codebase References

| File | Purpose |
|------|---------|
| `crates/sr-adapters/src/oracle_runner.rs` | Existing oracle runner implementation |
| `crates/sr-ports/src/lib.rs` | Port traits for oracle runner |
| `crates/sr-api/src/handlers/runs.rs` | Run handler (to be enhanced) |
| `crates/sr-adapters/src/minio.rs` | Evidence storage adapter |
| `crates/sr-adapters/src/evidence.rs` | Evidence manifest builder |

---

## Appendix A: Effort Summary (AMENDED)

| Phase | Focus | Effort | Cumulative | Amendment |
|-------|-------|--------|------------|-----------|
| V8-1 | Oracle Suite Registry | 1 session | 1 | A-4: Extract port from existing |
| V8-2 | Event-Driven Worker | **2-3 sessions** | 3-4 | A-1: Event sourcing architecture |
| V8-3 | Integrity Checks | 1-2 sessions | 4-6 | — |
| V8-4 | Core Oracle Suite | 2 sessions | 6-8 | — |
| V8-5 | Semantic Oracles | **1-2 sessions** | 7-10 | A-3: Use existing types |

**Total:** ~7-10 sessions for complete execution (was 7-9)

---

## Appendix B: Dependency Graph (AMENDED)

```
┌─────────────────────────────────────────────────────────────────┐
│                         SR-PLAN-V7                              │
│                (MVP Stabilization - Complete)                   │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│  V8-1: Oracle Suite Registry (AMENDED A-4)                      │
│  - Extract port trait from existing OracleSuiteRegistry         │
│  - Add PostgreSQL persistence layer                             │
│  - Existing endpoints continue to work                          │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│  V8-2: Oracle Execution Worker (AMENDED A-1)                    │
│  - Event-driven worker subscribes to RunStarted                 │
│  - NOT direct API call (event sourcing architecture)            │
│  - Emits OracleExecutionStarted/Completed events                │
│  - Candidate workspace materialization                          │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│  V8-3: Integrity Condition Detection                            │
│  - ORACLE_TAMPER (suite hash mismatch)                          │
│  - ORACLE_GAP (missing required oracle)                         │
│  - ORACLE_ENV_MISMATCH (constraint violation)                   │
│  - IntegrityViolationDetected event                             │
└─────────────────────────────────────────────────────────────────┘
                                │
                ┌───────────────┴───────────────┐
                │                               │
                ▼                               ▼
┌───────────────────────────────┐ ┌───────────────────────────────┐
│  V8-4: Core Oracle Suite      │ │  V8-5: Semantic Oracles       │
│  - Build oracle               │ │  (AMENDED A-3)                │
│  - Unit test oracle           │ │  - Use EXISTING types from    │
│  - Schema validation oracle   │ │    sr-domain/semantic_oracle  │
│  - Lint oracle (advisory)     │ │  - Focus on container package │
│  - Container packaging        │ │  - Add Waived if needed       │
└───────────────────────────────┘ └───────────────────────────────┘
                │                               │
                └───────────────┬───────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Milestone 1 Ready                           │
│        D-24, D-25, D-27, D-39 Complete                          │
│                                                                 │
│                    → SR-PLAN-V9 →                               │
│         (D-23 Semantic Worker, D-41 Branch-0, D-36 Contract)    │
└─────────────────────────────────────────────────────────────────┘
```

---

## Appendix C: Contract Compliance Matrix

| Contract | Requirement | V8 Implementation |
|----------|-------------|-------------------|
| C-OR-1 | Required oracles deterministic | V8-3: FLAKE detection |
| C-OR-2 | Suite pinning and integrity | V8-1: Registry + V8-3: TAMPER detection |
| C-OR-3 | Environment constraints declared | V8-3: ENV_MISMATCH detection |
| C-OR-4 | Oracle gaps blocking | V8-3: GAP detection |
| C-OR-5 | Oracle flake stop-the-line | V8-3: FLAKE detection |
| C-OR-6 | No silent oracle weakening | V8-3: Integrity events + escalation |
| C-OR-7 | Integrity conditions halt and escalate | V8-3: All conditions halt + emit event |
| C-EVID-1 | Evidence bundle minimum manifest | V8-2: Existing manifest builder |
| C-EVID-2 | Evidence immutability | Existing: MinIO content-addressed storage |
| C-VER-1 | Verification evidence-based | V8-5: Semantic oracle produces evidence |

---

## Appendix D: Critical Files for Implementation

### V8-1 (Registry)
| File | Action |
|------|--------|
| `crates/sr-ports/src/lib.rs` | MODIFY — add `OracleSuiteRegistry` trait |
| `crates/sr-adapters/src/oracle_suite.rs` | MODIFY — implement port trait |
| `crates/sr-adapters/src/postgres_oracle_registry.rs` | CREATE — DB adapter |
| `migrations/008_oracle_suite_registry.sql` | CREATE — schema |

### V8-2 (Event-Driven Worker)
| File | Action |
|------|--------|
| `crates/sr-adapters/src/oracle_worker.rs` | CREATE — event-driven worker |
| `crates/sr-adapters/src/candidate_store.rs` | CREATE — workspace materializer |
| `crates/sr-api/src/handlers/runs.rs` | MINIMAL — event already created |
| `crates/sr-domain/src/events.rs` | MODIFY — add oracle execution events |

### V8-3 (Integrity)
| File | Action |
|------|--------|
| `crates/sr-domain/src/integrity.rs` | CREATE — integrity condition types |
| `crates/sr-adapters/src/oracle_runner.rs` | MODIFY — add integrity detection |
| `crates/sr-domain/src/events.rs` | MODIFY — add `IntegrityViolationDetected` |

### V8-4 (Core Suite)
| File | Action |
|------|--------|
| `oracle-suites/core-v1/` | CREATE — directory with Dockerfile, suite.json, scripts |

### V8-5 (Semantic Oracles)
| File | Action |
|------|--------|
| `oracle-suites/semantic-v1/` | CREATE — directory |
| `crates/sr-domain/src/semantic_oracle.rs` | MODIFY — add `Waived` variant if needed |

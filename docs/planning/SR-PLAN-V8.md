# SR-PLAN-V8: Oracle Runner & Semantic Suite Foundation

**Status:** Ready for Implementation
**Created:** 2026-01-16
**Depends On:** SR-PLAN-V7 (MVP Stabilization complete)
**Implements:** SR-CHARTER §Immediate Objective (Milestone 1: Semantic Work Unit Runtime)

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
| `POST /runs` handler | ⚠️ Partial | Exists but may need oracle runner integration |

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

### Phase V8-1: Oracle Suite Registry

**Objective:** Persistent storage and retrieval of oracle suite definitions.

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `crates/sr-domain/src/entities/oracle_suite.rs` | CREATE | Oracle suite domain entity |
| `crates/sr-adapters/src/postgres_oracle_registry.rs` | CREATE | PostgreSQL repository |
| `crates/sr-api/src/handlers/oracle_suites.rs` | CREATE | Suite management endpoints |
| `migrations/YYYYMMDD_oracle_suite_registry.sql` | CREATE | Database schema |

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

**New Endpoints:**

`POST /api/v1/oracle-suites`
- Registers a new oracle suite
- Computes suite_hash from definition content
- Validates OCI image reference format
- Response: `{ suite_id, suite_hash, registered_at }`

`GET /api/v1/oracle-suites/{suite_id}`
- Retrieves suite definition by ID
- Validates suite_hash matches stored (C-OR-2 foundation)
- Response: Full `OracleSuiteDefinition`

`GET /api/v1/oracle-suites`
- Lists registered suites
- Supports status filter (active, deprecated)
- Response: `{ suites: [...], total }`

**Domain Entity:**

```rust
pub struct OracleSuite {
    pub suite_id: String,
    pub suite_hash: String,
    pub oci_image: String,
    pub oci_image_digest: String,
    pub environment_constraints: EnvironmentConstraints,
    pub oracles: Vec<OracleDefinition>,
    pub metadata: serde_json::Value,
    pub registered_at: DateTime<Utc>,
    pub registered_by: ActorRef,
    pub status: OracleSuiteStatus,
}

pub enum OracleSuiteStatus {
    Active,
    Deprecated,
    Revoked,
}
```

**Port Trait:**

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

### Phase V8-2: Oracle Runner API Integration

**Objective:** Wire `PodmanOracleRunner` to API endpoints for end-to-end oracle execution.

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `crates/sr-api/src/handlers/runs.rs` | MODIFY | Integrate oracle runner |
| `crates/sr-api/src/main.rs` | MODIFY | Initialize runner in app state |
| `crates/sr-adapters/src/oracle_runner.rs` | MODIFY | Fix candidate path resolution |
| `crates/sr-adapters/src/candidate_store.rs` | CREATE | Candidate workspace materializer |

**Candidate Workspace Resolution:**

The oracle runner currently uses a placeholder path (`/tmp/candidates/{id}`). We need to:
1. Resolve candidate from Candidate store
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

**`POST /runs` Handler Enhancement:**

```rust
pub async fn start_run(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(request): Json<StartRunRequest>,
) -> ApiResult<Json<RunStartedResponse>> {
    // 1. Validate request
    let candidate_id = &request.candidate_id;
    let suite_id = &request.oracle_suite_id;
    let suite_hash = &request.oracle_suite_hash;

    // 2. Retrieve suite from registry (V8-1)
    let suite = state.oracle_registry
        .get(suite_id)
        .await?
        .ok_or(ApiError::not_found("Oracle suite not found"))?;

    // 3. Verify suite hash (C-OR-2 foundation)
    if suite.suite_hash != *suite_hash {
        return Err(ApiError::conflict("Suite hash mismatch"));
    }

    // 4. Materialize candidate workspace
    let workspace = state.candidate_workspace
        .materialize(candidate_id)
        .await?;

    // 5. Execute oracle suite
    let result = state.oracle_runner
        .execute_suite(
            candidate_id,
            suite_id,
            suite_hash,
            &workspace.path,
        )
        .await?;

    // 6. Emit RunCompleted event
    state.event_store
        .append(RunCompleted {
            run_id: result.run_id.clone(),
            candidate_id: candidate_id.clone(),
            evidence_bundle_hash: result.evidence_bundle_hash.clone(),
            status: result.status,
        })
        .await?;

    // 7. Return result
    Ok(Json(RunStartedResponse {
        run_id: result.run_id,
        evidence_bundle_hash: result.evidence_bundle_hash,
        status: result.status.to_string(),
    }))
}
```

**App State Enhancement:**

```rust
pub struct AppState {
    // ... existing fields ...
    pub oracle_registry: Arc<dyn OracleSuiteRegistry>,
    pub oracle_runner: Arc<PodmanOracleRunner<MinioEvidenceStore>>,
    pub candidate_workspace: Arc<dyn CandidateWorkspace>,
}
```

**Acceptance Criteria:**
- [ ] `POST /runs` triggers oracle suite execution
- [ ] Candidate workspace materialized correctly
- [ ] Oracle runner executes in test mode (mock containers)
- [ ] Evidence bundle stored in MinIO
- [ ] `RunCompleted` event emitted
- [ ] End-to-end integration test passes
- [ ] `cargo test --package sr-api` passes

**Effort:** ~1-2 sessions

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

### Phase V8-5: Semantic Oracle Integration

**Objective:** Add semantic oracle support with `sr.semantic_eval.v1` output schema.

**Deliverables:**

| File | Action | Description |
|------|--------|-------------|
| `oracle-suites/semantic-v1/` | CREATE | Semantic oracle suite |
| `crates/sr-domain/src/semantic.rs` | CREATE | Semantic evaluation types |
| `docs/platform/SR-TYPES.md` | MODIFY | Add semantic result types |

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

**Domain Types:**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticEvalResult {
    pub schema: String,  // "sr.semantic_eval.v1"
    pub candidate_id: String,
    pub procedure_template_id: String,
    pub stage_id: String,
    pub oracle_suite_id: String,
    pub oracle_suite_hash: String,
    pub semantic_set: SemanticSetRef,
    pub metrics: SemanticMetrics,
    pub decision: SemanticDecision,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSetRef {
    pub semantic_set_id: String,
    pub semantic_set_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticMetrics {
    pub residual_norm: f64,
    pub coverage: HashMap<String, f64>,
    pub violations: Vec<SemanticViolation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticViolation {
    pub code: String,
    pub axis: String,
    pub description: String,
    pub severity: ViolationSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticDecision {
    pub status: DecisionStatus,
    pub rule_id: String,
    pub thresholds: HashMap<String, f64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DecisionStatus {
    Pass,
    Fail,
    Waived,
}
```

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
- [ ] Semantic oracle suite definition conforms to SR-SEMANTIC-ORACLE-SPEC
- [ ] `semantic_set_hash` incorporated into `oracle_suite_hash` (§2)
- [ ] All 3 required artifacts produced: residual, coverage, violations
- [ ] Evaluation result conforms to `sr.semantic_eval.v1` schema
- [ ] Domain types defined in `sr-domain`
- [ ] SR-TYPES updated with semantic result types
- [ ] Integration test validates semantic oracle execution
- [ ] Evidence bundle contains semantic artifacts

**Effort:** ~2 sessions

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

## Appendix A: Effort Summary

| Phase | Focus | Effort | Cumulative |
|-------|-------|--------|------------|
| V8-1 | Oracle Suite Registry | 1 session | 1 |
| V8-2 | API Integration | 1-2 sessions | 2-3 |
| V8-3 | Integrity Checks | 1-2 sessions | 3-5 |
| V8-4 | Core Oracle Suite | 2 sessions | 5-7 |
| V8-5 | Semantic Oracles | 2 sessions | 7-9 |

**Total:** ~7-9 sessions for complete execution

---

## Appendix B: Dependency Graph

```
┌─────────────────────────────────────────────────────────────────┐
│                         SR-PLAN-V7                              │
│                (MVP Stabilization - Complete)                   │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│  V8-1: Oracle Suite Registry                                    │
│  - PostgreSQL storage for suite definitions                     │
│  - POST/GET /oracle-suites endpoints                            │
│  - Suite hash computation                                       │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│  V8-2: Oracle Runner API Integration                            │
│  - Wire PodmanOracleRunner to POST /runs                        │
│  - Candidate workspace materialization                          │
│  - Evidence bundle storage                                      │
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
│  - Build oracle               │ │  - sr.semantic_eval.v1 schema │
│  - Unit test oracle           │ │  - Residual/coverage/violation│
│  - Schema validation oracle   │ │  - Semantic set binding       │
│  - Lint oracle (advisory)     │ │  - Stage gate integration     │
│  - Container packaging        │ │                               │
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

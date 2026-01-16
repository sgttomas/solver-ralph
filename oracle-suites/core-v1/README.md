# SOLVER-Ralph Core Oracle Suite v1

Core verification oracles for SOLVER-Ralph projects. This suite provides foundational verification capabilities for code quality and correctness.

## Overview

| Oracle | Classification | Description |
|--------|----------------|-------------|
| `oracle:build` | Required | Verifies project builds successfully |
| `oracle:unit-tests` | Required | Runs unit test suite |
| `oracle:schema-validation` | Required | Validates JSON schemas and type definitions |
| `oracle:lint` | Advisory | Runs code linting (does not block verification) |

## Supported Project Types

- **Rust** - Detects via `Cargo.toml`
- **Node.js** - Detects via `package.json`
- **Make** - Detects via `Makefile` (build only)

## Output Schema

All oracles produce JSON conforming to `sr.oracle_result.v1`:

```json
{
  "schema": "sr.oracle_result.v1",
  "oracle_id": "oracle:build",
  "status": "PASS",
  "started_at": "2026-01-16T12:00:00Z",
  "completed_at": "2026-01-16T12:01:30Z",
  "duration_ms": 90000,
  "exit_code": 0,
  "summary": "Build completed successfully",
  "details": { ... },
  "artifacts": []
}
```

## Environment Constraints

Per SR-SPEC §4.5, this suite requires:

| Constraint | Value | Description |
|------------|-------|-------------|
| `runtime` | `runsc` | gVisor sandbox runtime |
| `network` | `disabled` | No network access |
| `workspace_readonly` | `true` | Candidate content is read-only |
| `cpu_arch` | `amd64` | x86-64 architecture |
| `os` | `linux` | Linux operating system |

## Container Mounts

| Mount Point | Mode | Purpose |
|-------------|------|---------|
| `/workspace` | Read-only | Candidate content |
| `/scratch` | Read-write | Oracle outputs |

## Oracle Details

### oracle:build

Verifies the project builds successfully.

- **Timeout:** 300 seconds
- **Output:** `reports/build.json`
- **Rust:** Runs `cargo build --release`
- **Node.js:** Runs `npm ci && npm run build`
- **Make:** Runs `make`

### oracle:unit-tests

Runs the project's unit test suite.

- **Timeout:** 600 seconds
- **Output:** `reports/unit-tests.json`
- **Rust:** Runs `cargo test`, parses pass/fail counts
- **Node.js:** Runs `npm test`

### oracle:schema-validation

Validates JSON files and schema definitions.

- **Timeout:** 120 seconds
- **Output:** `reports/schema.json`
- **Checks:**
  - All `.json` files are valid JSON
  - JSON Schema files (with `$schema` keyword) are valid
  - TypeScript config (`tsconfig.json`) is valid

### oracle:lint

Runs code linting (advisory - does not block verification).

- **Timeout:** 180 seconds
- **Output:** `reports/lint.json`
- **Classification:** Advisory
- **Rust:** Runs `cargo clippy --all-targets --all-features -- -D warnings`
- **Node.js:** Runs `npx eslint` if configured

## Building the Container

```bash
# Build the container image
docker build -t ghcr.io/solver-ralph/oracle-suite-core:v1 .

# Get the image digest
docker inspect --format='{{index .RepoDigests 0}}' ghcr.io/solver-ralph/oracle-suite-core:v1
```

## Registration

Register the suite via the API:

```bash
curl -X POST http://localhost:3000/api/v1/oracle-suites \
  -H "Content-Type: application/json" \
  -d @suite.json
```

## Contract Compliance

| Contract | Requirement | Implementation |
|----------|-------------|----------------|
| C-OR-1 | Required oracles deterministic | Build/unit/schema use deterministic tooling |
| C-OR-3 | Environment constraints declared | suite.json declares all constraints |
| C-EVID-1 | Evidence bundle manifest | All outputs conform to sr.oracle_result.v1 |

## Version History

- **v1.0.0** (2026-01-16): Initial release for SR-PLAN-V8 Phase V8-4

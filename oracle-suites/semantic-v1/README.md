# Semantic Oracle Suite v1

Semantic evaluation oracles for SOLVER-Ralph intake admissibility verification.

## Overview

This oracle suite implements **SR-PLAN-V8 Phase V8-5** — Semantic Oracle Integration. It evaluates candidate intake artifacts against the intake admissibility semantic set per **SR-SEMANTIC-ORACLE-SPEC**.

## Suite Identity

| Field | Value |
|-------|-------|
| Suite ID | `suite:sr-semantic-v1` |
| OCI Image | `ghcr.io/solver-ralph/oracle-suite-semantic:v1` |
| Schema | `sr.semantic_eval.v1` |

## Oracles

### `oracle:semantic-eval`

Evaluates an intake artifact against the intake admissibility semantic set, producing structured measurements and a pass/fail decision.

| Property | Value |
|----------|-------|
| Classification | **Required** |
| Timeout | 300 seconds |
| Command | `/oracles/semantic-eval.sh` |

#### Output Artifacts

All outputs conform to existing `sr-domain` type definitions:

| Artifact | Schema | Description |
|----------|--------|-------------|
| `reports/semantic/eval.json` | `sr.semantic_eval.v1` | Main evaluation result with decision |
| `reports/semantic/residual.json` | `ResidualReport` | Per-axis residual measurements |
| `reports/semantic/coverage.json` | `CoverageReport` | Per-axis coverage metrics |
| `reports/semantic/violations.json` | `ViolationsReport` | Constraint violations with severity |

## Semantic Set: Intake Admissibility

The suite binds to the `semantic_set:INTAKE-ADMISSIBILITY` semantic set with 6 evaluation axes:

| Axis | Weight | Required | Description |
|------|--------|----------|-------------|
| `schema_compliance` | 1.0 | Yes | Required fields present, correct types |
| `traceability_coverage` | 0.8 | Yes | Inputs/deliverables have refs |
| `contradiction_free` | 1.0 | Yes | No contradictory statements |
| `ambiguity_bounded` | 0.6 | No | Ambiguities inventoried |
| `privacy_safe` | 1.0 | Yes | No PII in public fields |
| `term_map_aligned` | 0.7 | No | Terms used consistently |

### Decision Rule: `intake_admissibility_v1`

| Threshold | Value |
|-----------|-------|
| `max_residual_norm` | 0.2 |
| `min_coverage` | 0.85 |
| `max_error_violations` | 0 |
| `max_warning_violations` | 3 |

## Environment Constraints

Per **C-OR-3**, the suite declares these constraints:

| Constraint | Value |
|------------|-------|
| `runtime` | `runsc` (gVisor) |
| `network` | `disabled` |
| `cpu_arch` | `amd64` |
| `os` | `linux` |
| `workspace_readonly` | `true` |

## Contract Compliance

| Contract | Requirement | Implementation |
|----------|-------------|----------------|
| **C-OR-1** | Required oracles deterministic | Evaluation uses deterministic logic |
| **C-OR-2** | Suite pinning | `suite_hash` incorporates `semantic_set_hash` |
| **C-OR-3** | Environment constraints | runsc, network disabled, workspace readonly |
| **C-OR-4** | Oracle gaps blocking | Single required oracle, detected by runner |
| **C-EVID-1** | Evidence minimum manifest | 4 output artifacts with schemas |

## Building

```bash
# From repository root
docker build -f oracle-suites/semantic-v1/Dockerfile \
  -t ghcr.io/solver-ralph/oracle-suite-semantic:v1 .
```

## Running Locally

```bash
# Using sr-oracles CLI
sr-oracles semantic-eval \
  --intake path/to/intake.yaml \
  --output-dir reports/semantic

# In container
docker run -v $(pwd)/workspace:/workspace:ro \
           -v /tmp/scratch:/scratch:rw \
           ghcr.io/solver-ralph/oracle-suite-semantic:v1 \
           /oracles/semantic-eval.sh
```

## Related Documents

- `docs/platform/SR-SEMANTIC-ORACLE-SPEC.md` — Semantic oracle interface specification
- `docs/planning/SR-PLAN-V8.md` — Oracle runner implementation plan
- `crates/sr-domain/src/semantic_oracle.rs` — Type definitions
- `crates/sr-adapters/src/semantic_suite.rs` — Evaluation implementation

#!/bin/bash
# oracle:semantic-eval - Semantic Evaluation Oracle
#
# Evaluates candidate intake artifacts against the intake admissibility
# semantic set. Produces structured reports per SR-SEMANTIC-ORACLE-SPEC Section 3.
#
# Required outputs:
# - /scratch/reports/semantic/eval.json (sr.semantic_eval.v1)
# - /scratch/reports/semantic/residual.json (ResidualReport)
# - /scratch/reports/semantic/coverage.json (CoverageReport)
# - /scratch/reports/semantic/violations.json (ViolationsReport)
#
# Exit: 0 = evaluation completed (check status in eval.json), non-zero = error
#
# Contract compliance:
# - C-OR-1: Deterministic evaluation (no external calls, fixed seed)
# - C-OR-3: Runs in gVisor sandbox with network disabled
# - C-EVID-1: All outputs conform to defined schemas

set -euo pipefail

# Configuration
WORKSPACE="${WORKSPACE_MOUNT:-/workspace}"
REPORTS_DIR="${REPORTS_DIR:-/scratch/reports}"
SEMANTIC_DIR="${REPORTS_DIR}/semantic"

# Initialize
mkdir -p "${SEMANTIC_DIR}"
STARTED_AT=$(date -u +%Y-%m-%dT%H:%M:%SZ)
START_SECONDS=$(date +%s)

# Helper: Generate error output
generate_error() {
    local message="$1"

    COMPLETED_AT=$(date -u +%Y-%m-%dT%H:%M:%SZ)
    END_SECONDS=$(date +%s)
    DURATION_MS=$(( (END_SECONDS - START_SECONDS) * 1000 ))

    cat > "${SEMANTIC_DIR}/eval.json" << EOF
{
  "schema": "sr.oracle_result.v1",
  "oracle_id": "oracle:semantic-eval",
  "status": "ERROR",
  "started_at": "${STARTED_AT}",
  "completed_at": "${COMPLETED_AT}",
  "duration_ms": ${DURATION_MS},
  "exit_code": 1,
  "summary": "${message}",
  "details": {
    "error": "${message}",
    "workspace": "${WORKSPACE}"
  },
  "artifacts": []
}
EOF
}

echo "Starting semantic evaluation..."
echo "Workspace: ${WORKSPACE}"

# Locate intake file (try multiple paths)
INTAKE_PATH=""
for candidate in \
    "${WORKSPACE}/artifacts/intake/draft_intake.yaml" \
    "${WORKSPACE}/intake.yaml" \
    "${WORKSPACE}/draft_intake.yaml" \
    "${WORKSPACE}/intake.yml"; do
    if [ -f "${candidate}" ]; then
        INTAKE_PATH="${candidate}"
        break
    fi
done

if [ -z "${INTAKE_PATH}" ]; then
    echo "ERROR: No intake file found in workspace"
    generate_error "No intake file found in workspace"
    exit 1
fi

echo "Found intake at: ${INTAKE_PATH}"

# Execute semantic evaluation using sr-oracles binary
# The binary handles:
# 1. Loading the intake YAML
# 2. Running semantic evaluation against the intake admissibility set
# 3. Computing residual, coverage, violations
# 4. Deriving pass/fail decision from decision rule
# 5. Writing all four required output files
if sr-oracles semantic-eval \
    --intake "${INTAKE_PATH}" \
    --output-dir "${SEMANTIC_DIR}" \
    --candidate-id "candidate:container-eval" \
    --suite-id "suite:sr-semantic-v1"; then

    echo "Semantic evaluation completed successfully"

    # Verify all required outputs were produced
    MISSING=""
    for report in eval.json residual.json coverage.json violations.json; do
        if [ ! -f "${SEMANTIC_DIR}/${report}" ]; then
            MISSING="${MISSING} ${report}"
        fi
    done

    if [ -n "${MISSING}" ]; then
        echo "ERROR: Expected outputs not produced:${MISSING}"
        exit 1
    fi

    # Extract status from eval.json for logging
    if command -v jq >/dev/null 2>&1; then
        STATUS=$(jq -r '.decision.status // "UNKNOWN"' "${SEMANTIC_DIR}/eval.json" 2>/dev/null || echo "UNKNOWN")
        echo "Evaluation decision: ${STATUS}"
    fi

    exit 0
else
    EXIT_CODE=$?
    echo "ERROR: Semantic evaluation failed with exit code ${EXIT_CODE}"

    # If eval.json wasn't created, generate an error report
    if [ ! -f "${SEMANTIC_DIR}/eval.json" ]; then
        generate_error "Semantic evaluation binary failed with exit code ${EXIT_CODE}"
    fi

    exit 1
fi

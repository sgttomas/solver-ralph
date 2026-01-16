#!/bin/bash
# oracle:lint - Lint Check Oracle (Advisory)
#
# Runs code linting on the project.
# Supports: Rust (clippy), Node.js (eslint)
#
# This is an ADVISORY oracle - it does not block verification.
#
# Output: /scratch/reports/lint.json (sr.oracle_result.v1 schema)
# Exit: 0 = PASS (no warnings), non-zero = warnings/errors present

set -uo pipefail

# Configuration
WORKSPACE="${WORKSPACE_MOUNT:-/workspace}"
REPORTS_DIR="${REPORTS_DIR:-/scratch/reports}"
OUTPUT_FILE="${REPORTS_DIR}/lint.json"
LOG_FILE="/tmp/lint.log"

# Initialize
mkdir -p "${REPORTS_DIR}"
STARTED_AT=$(date -u +%Y-%m-%dT%H:%M:%SZ)
START_SECONDS=$(date +%s)

# Helper: Generate output JSON
generate_output() {
    local status="$1"
    local exit_code="$2"
    local summary="$3"
    local project_type="$4"
    local warnings="${5:-0}"
    local errors="${6:-0}"

    COMPLETED_AT=$(date -u +%Y-%m-%dT%H:%M:%SZ)
    END_SECONDS=$(date +%s)
    DURATION_MS=$(( (END_SECONDS - START_SECONDS) * 1000 ))

    cat > "${OUTPUT_FILE}" << EOF
{
  "schema": "sr.oracle_result.v1",
  "oracle_id": "oracle:lint",
  "status": "${status}",
  "started_at": "${STARTED_AT}",
  "completed_at": "${COMPLETED_AT}",
  "duration_ms": ${DURATION_MS},
  "exit_code": ${exit_code},
  "summary": "${summary}",
  "details": {
    "project_type": "${project_type}",
    "warnings": ${warnings},
    "errors": ${errors},
    "classification": "advisory",
    "workspace": "${WORKSPACE}"
  },
  "artifacts": []
}
EOF
}

# Change to workspace
cd "${WORKSPACE}"

# Detect project type and run linter
if [ -f "Cargo.toml" ]; then
    echo "Detected Rust project (Cargo.toml)"
    PROJECT_TYPE="rust"

    # Run cargo clippy
    set +e
    cargo clippy --all-targets --all-features -- -D warnings 2>&1 | tee "${LOG_FILE}"
    EXIT_CODE=${PIPESTATUS[0]}
    set -e

    # Count warnings and errors from clippy output
    WARNINGS=$(grep -c "warning:" "${LOG_FILE}" 2>/dev/null || echo "0")
    ERRORS=$(grep -c "error\[" "${LOG_FILE}" 2>/dev/null || echo "0")

    if [ "${EXIT_CODE}" -eq 0 ]; then
        generate_output "PASS" 0 "No clippy warnings or errors" "${PROJECT_TYPE}" 0 0
        exit 0
    else
        generate_output "FAIL" "${EXIT_CODE}" "Clippy found ${WARNINGS} warnings and ${ERRORS} errors" "${PROJECT_TYPE}" "${WARNINGS}" "${ERRORS}"
        # Advisory oracle - we still exit non-zero to indicate issues were found
        exit 1
    fi

elif [ -f "package.json" ]; then
    echo "Detected Node.js project (package.json)"
    PROJECT_TYPE="nodejs"

    # Check if eslint is available
    if [ -f ".eslintrc.js" ] || [ -f ".eslintrc.json" ] || [ -f ".eslintrc" ] || grep -q '"eslint"' package.json 2>/dev/null; then
        # Try to run eslint
        set +e
        npx eslint . --ext .js,.ts,.tsx 2>&1 | tee "${LOG_FILE}"
        EXIT_CODE=${PIPESTATUS[0]}
        set -e

        WARNINGS=$(grep -c "warning" "${LOG_FILE}" 2>/dev/null || echo "0")
        ERRORS=$(grep -c "error" "${LOG_FILE}" 2>/dev/null || echo "0")

        if [ "${EXIT_CODE}" -eq 0 ]; then
            generate_output "PASS" 0 "No ESLint warnings or errors" "${PROJECT_TYPE}" 0 0
            exit 0
        else
            generate_output "FAIL" "${EXIT_CODE}" "ESLint found ${WARNINGS} warnings and ${ERRORS} errors" "${PROJECT_TYPE}" "${WARNINGS}" "${ERRORS}"
            exit 1
        fi
    else
        echo "No ESLint configuration found"
        generate_output "PASS" 0 "No linter configuration found - skipping" "${PROJECT_TYPE}" 0 0
        exit 0
    fi

else
    echo "No recognized project configuration found"
    PROJECT_TYPE="unknown"
    generate_output "PASS" 0 "No linter configuration found - skipping" "${PROJECT_TYPE}" 0 0
    exit 0
fi

#!/bin/bash
# oracle:unit-tests - Unit Tests Oracle
#
# Runs the project's unit test suite and captures results.
# Supports: Rust (cargo test), Node.js (npm test)
#
# Output: /scratch/reports/unit-tests.json (sr.oracle_result.v1 schema)
# Exit: 0 = PASS (all tests pass), non-zero = FAIL

set -uo pipefail

# Configuration
WORKSPACE="${WORKSPACE_MOUNT:-/workspace}"
REPORTS_DIR="${REPORTS_DIR:-/scratch/reports}"
OUTPUT_FILE="${REPORTS_DIR}/unit-tests.json"
LOG_FILE="/tmp/unit-tests.log"

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
    local passed="${5:-0}"
    local failed="${6:-0}"
    local ignored="${7:-0}"

    COMPLETED_AT=$(date -u +%Y-%m-%dT%H:%M:%SZ)
    END_SECONDS=$(date +%s)
    DURATION_MS=$(( (END_SECONDS - START_SECONDS) * 1000 ))
    TOTAL=$((passed + failed + ignored))

    cat > "${OUTPUT_FILE}" << EOF
{
  "schema": "sr.oracle_result.v1",
  "oracle_id": "oracle:unit-tests",
  "status": "${status}",
  "started_at": "${STARTED_AT}",
  "completed_at": "${COMPLETED_AT}",
  "duration_ms": ${DURATION_MS},
  "exit_code": ${exit_code},
  "summary": "${summary}",
  "details": {
    "project_type": "${project_type}",
    "tests_total": ${TOTAL},
    "tests_passed": ${passed},
    "tests_failed": ${failed},
    "tests_ignored": ${ignored},
    "workspace": "${WORKSPACE}"
  },
  "artifacts": []
}
EOF
}

# Change to workspace
cd "${WORKSPACE}"

# Detect project type and run tests
if [ -f "Cargo.toml" ]; then
    echo "Detected Rust project (Cargo.toml)"
    PROJECT_TYPE="rust"

    # Run cargo test and capture output
    set +e
    cargo test 2>&1 | tee "${LOG_FILE}"
    EXIT_CODE=${PIPESTATUS[0]}
    set -e

    # Parse test results from cargo test output
    # Format: "test result: ok. X passed; Y failed; Z ignored"
    if grep -q "test result:" "${LOG_FILE}"; then
        RESULT_LINE=$(grep "test result:" "${LOG_FILE}" | tail -1)
        PASSED=$(echo "${RESULT_LINE}" | grep -oP '\d+(?= passed)' || echo "0")
        FAILED=$(echo "${RESULT_LINE}" | grep -oP '\d+(?= failed)' || echo "0")
        IGNORED=$(echo "${RESULT_LINE}" | grep -oP '\d+(?= ignored)' || echo "0")
    else
        PASSED=0
        FAILED=0
        IGNORED=0
    fi

    if [ "${EXIT_CODE}" -eq 0 ]; then
        generate_output "PASS" 0 "All ${PASSED} tests passed" "${PROJECT_TYPE}" "${PASSED}" 0 "${IGNORED}"
        exit 0
    else
        generate_output "FAIL" "${EXIT_CODE}" "${FAILED} tests failed out of $((PASSED + FAILED))" "${PROJECT_TYPE}" "${PASSED}" "${FAILED}" "${IGNORED}"
        exit 1
    fi

elif [ -f "package.json" ]; then
    echo "Detected Node.js project (package.json)"
    PROJECT_TYPE="nodejs"

    # Check if test script exists
    if ! grep -q '"test"' package.json; then
        generate_output "PASS" 0 "No test script defined in package.json" "${PROJECT_TYPE}" 0 0 0
        exit 0
    fi

    # Run npm test and capture output
    set +e
    npm test 2>&1 | tee "${LOG_FILE}"
    EXIT_CODE=${PIPESTATUS[0]}
    set -e

    if [ "${EXIT_CODE}" -eq 0 ]; then
        # Try to parse test counts from common frameworks
        PASSED=$(grep -oP '\d+(?= passing)' "${LOG_FILE}" 2>/dev/null | head -1 || echo "0")
        [ -z "${PASSED}" ] && PASSED=0
        generate_output "PASS" 0 "Tests passed" "${PROJECT_TYPE}" "${PASSED}" 0 0
        exit 0
    else
        FAILED=$(grep -oP '\d+(?= failing)' "${LOG_FILE}" 2>/dev/null | head -1 || echo "1")
        [ -z "${FAILED}" ] && FAILED=1
        generate_output "FAIL" "${EXIT_CODE}" "Tests failed" "${PROJECT_TYPE}" 0 "${FAILED}" 0
        exit 1
    fi

else
    echo "No recognized test configuration found"
    PROJECT_TYPE="unknown"
    generate_output "PASS" 0 "No test configuration found - skipping" "${PROJECT_TYPE}" 0 0 0
    exit 0
fi

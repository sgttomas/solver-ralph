#!/bin/bash
# oracle:build - Build Check Oracle
#
# Verifies that the project builds successfully.
# Supports: Rust (Cargo.toml), Node.js (package.json)
#
# Output: /scratch/reports/build.json (sr.oracle_result.v1 schema)
# Exit: 0 = PASS, non-zero = FAIL

set -euo pipefail

# Configuration
WORKSPACE="${WORKSPACE_MOUNT:-/workspace}"
REPORTS_DIR="${REPORTS_DIR:-/scratch/reports}"
OUTPUT_FILE="${REPORTS_DIR}/build.json"
LOG_FILE="/tmp/build.log"

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
  "oracle_id": "oracle:build",
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
    "workspace": "${WORKSPACE}"
  },
  "artifacts": []
}
EOF
}

# Change to workspace
cd "${WORKSPACE}"

# Detect project type and build
if [ -f "Cargo.toml" ]; then
    echo "Detected Rust project (Cargo.toml)"
    PROJECT_TYPE="rust"

    # Run cargo build
    if cargo build --release 2>&1 | tee "${LOG_FILE}"; then
        # Count warnings from build output
        WARNINGS=$(grep -c "warning:" "${LOG_FILE}" 2>/dev/null || echo "0")
        generate_output "PASS" 0 "Rust build completed successfully" "${PROJECT_TYPE}" "${WARNINGS}" 0
        exit 0
    else
        EXIT_CODE=$?
        ERRORS=$(grep -c "error\[" "${LOG_FILE}" 2>/dev/null || echo "1")
        generate_output "FAIL" "${EXIT_CODE}" "Rust build failed with errors" "${PROJECT_TYPE}" 0 "${ERRORS}"
        exit 1
    fi

elif [ -f "package.json" ]; then
    echo "Detected Node.js project (package.json)"
    PROJECT_TYPE="nodejs"

    # Install dependencies and build
    if npm ci 2>&1 | tee "${LOG_FILE}"; then
        if npm run build 2>&1 | tee -a "${LOG_FILE}"; then
            generate_output "PASS" 0 "Node.js build completed successfully" "${PROJECT_TYPE}" 0 0
            exit 0
        else
            EXIT_CODE=$?
            generate_output "FAIL" "${EXIT_CODE}" "Node.js build failed" "${PROJECT_TYPE}" 0 1
            exit 1
        fi
    else
        EXIT_CODE=$?
        generate_output "FAIL" "${EXIT_CODE}" "npm install failed" "${PROJECT_TYPE}" 0 1
        exit 1
    fi

elif [ -f "Makefile" ]; then
    echo "Detected Makefile project"
    PROJECT_TYPE="make"

    if make 2>&1 | tee "${LOG_FILE}"; then
        generate_output "PASS" 0 "Make build completed successfully" "${PROJECT_TYPE}" 0 0
        exit 0
    else
        EXIT_CODE=$?
        generate_output "FAIL" "${EXIT_CODE}" "Make build failed" "${PROJECT_TYPE}" 0 1
        exit 1
    fi

else
    echo "No recognized project configuration found"
    PROJECT_TYPE="unknown"
    generate_output "FAIL" 1 "No recognized project configuration (Cargo.toml, package.json, or Makefile)" "${PROJECT_TYPE}" 0 1
    exit 1
fi

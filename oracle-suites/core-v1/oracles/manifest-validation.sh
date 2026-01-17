#!/bin/bash
# oracle:manifest-validation - Evidence Manifest Validation Oracle
#
# Validates evidence.gate_packet manifests per SR-SPEC §1.9.1 and SR-CONTRACT C-EVID-1.
# Checks:
# - Version is "v1"
# - Artifact type is "evidence.gate_packet"
# - Required fields are present and non-empty
# - Timestamp ordering (run_started_at <= run_completed_at)
# - No duplicate artifact names
# - Verdict matches computed verdict from results
#
# Output: /scratch/reports/manifest-validation.json (sr.oracle_result.v1 schema)
# Exit: 0 = PASS, non-zero = FAIL
#
# Classification: required (per SR-SPEC §1.9.1 - validates evidence integrity)

set -uo pipefail

# Configuration
WORKSPACE="${WORKSPACE_MOUNT:-/workspace}"
REPORTS_DIR="${REPORTS_DIR:-/scratch/reports}"
OUTPUT_FILE="${REPORTS_DIR}/manifest-validation.json"
LOG_FILE="/tmp/manifest-validation.log"

# Initialize
mkdir -p "${REPORTS_DIR}"
STARTED_AT=$(date -u +%Y-%m-%dT%H:%M:%SZ)
START_SECONDS=$(date +%s)
ERRORS=0
WARNINGS=0
VALIDATED=0
VALIDATION_ERRORS=""

# Helper: Add validation error
add_error() {
    local msg="$1"
    VALIDATION_ERRORS="${VALIDATION_ERRORS}${msg}; "
    ((ERRORS++))
    echo "ERROR: ${msg}" | tee -a "${LOG_FILE}"
}

# Helper: Generate output JSON
generate_output() {
    local status="$1"
    local exit_code="$2"
    local summary="$3"

    COMPLETED_AT=$(date -u +%Y-%m-%dT%H:%M:%SZ)
    END_SECONDS=$(date +%s)
    DURATION_MS=$(( (END_SECONDS - START_SECONDS) * 1000 ))

    # Escape validation errors for JSON
    local escaped_errors
    escaped_errors=$(echo "${VALIDATION_ERRORS}" | sed 's/"/\\"/g' | tr '\n' ' ')

    cat > "${OUTPUT_FILE}" << EOF
{
  "schema": "sr.oracle_result.v1",
  "oracle_id": "oracle:manifest-validation",
  "status": "${status}",
  "started_at": "${STARTED_AT}",
  "completed_at": "${COMPLETED_AT}",
  "duration_ms": ${DURATION_MS},
  "exit_code": ${exit_code},
  "summary": "${summary}",
  "details": {
    "manifests_validated": ${VALIDATED},
    "errors": ${ERRORS},
    "warnings": ${WARNINGS},
    "validation_errors": "${escaped_errors}",
    "workspace": "${WORKSPACE}"
  },
  "artifacts": []
}
EOF
}

# Validate a single manifest file
validate_manifest() {
    local manifest_file="$1"
    local file_errors=0

    echo "Validating manifest: ${manifest_file}" | tee -a "${LOG_FILE}"

    # Check file exists and is valid JSON
    if ! jq empty "${manifest_file}" 2>/dev/null; then
        add_error "Invalid JSON in ${manifest_file}"
        return 1
    fi

    # Check version == "v1"
    local version
    version=$(jq -r '.version // empty' "${manifest_file}")
    if [[ "${version}" != "v1" ]]; then
        add_error "Invalid version in ${manifest_file}: expected 'v1', got '${version}'"
        ((file_errors++))
    fi

    # Check artifact_type == "evidence.gate_packet"
    local artifact_type
    artifact_type=$(jq -r '.artifact_type // empty' "${manifest_file}")
    if [[ "${artifact_type}" != "evidence.gate_packet" ]]; then
        add_error "Invalid artifact_type in ${manifest_file}: expected 'evidence.gate_packet', got '${artifact_type}'"
        ((file_errors++))
    fi

    # Check required fields are non-empty
    local required_fields=("bundle_id" "run_id" "candidate_id" "oracle_suite_id" "oracle_suite_hash")
    for field in "${required_fields[@]}"; do
        local value
        value=$(jq -r ".${field} // empty" "${manifest_file}")
        if [[ -z "${value}" ]]; then
            add_error "Missing or empty required field '${field}' in ${manifest_file}"
            ((file_errors++))
        fi
    done

    # Check timestamp ordering: run_started_at <= run_completed_at
    local started_at completed_at
    started_at=$(jq -r '.run_started_at // empty' "${manifest_file}")
    completed_at=$(jq -r '.run_completed_at // empty' "${manifest_file}")

    if [[ -n "${started_at}" && -n "${completed_at}" ]]; then
        # Convert to comparable format (works with ISO 8601)
        if [[ "${started_at}" > "${completed_at}" ]]; then
            add_error "Timestamp ordering error in ${manifest_file}: run_started_at (${started_at}) > run_completed_at (${completed_at})"
            ((file_errors++))
        fi
    else
        if [[ -z "${started_at}" ]]; then
            add_error "Missing required field 'run_started_at' in ${manifest_file}"
            ((file_errors++))
        fi
        if [[ -z "${completed_at}" ]]; then
            add_error "Missing required field 'run_completed_at' in ${manifest_file}"
            ((file_errors++))
        fi
    fi

    # Check for duplicate artifact names
    local artifact_count unique_count
    artifact_count=$(jq '.artifacts | length' "${manifest_file}")
    unique_count=$(jq '.artifacts | map(.name) | unique | length' "${manifest_file}")

    if [[ "${artifact_count}" -ne "${unique_count}" ]]; then
        add_error "Duplicate artifact names found in ${manifest_file}"
        ((file_errors++))
    fi

    # Verify verdict matches computed verdict from results
    local declared_verdict computed_verdict
    declared_verdict=$(jq -r '.verdict // empty' "${manifest_file}")

    # Compute verdict: ERROR > FAIL > PASS (SKIPPED doesn't affect verdict)
    local has_error has_fail
    has_error=$(jq '[.results[]? | select(.status == "ERROR")] | length' "${manifest_file}")
    has_fail=$(jq '[.results[]? | select(.status == "FAIL")] | length' "${manifest_file}")
    local result_count
    result_count=$(jq '.results | length' "${manifest_file}")

    if [[ "${result_count}" -eq 0 ]]; then
        computed_verdict="ERROR"
    elif [[ "${has_error}" -gt 0 ]]; then
        computed_verdict="ERROR"
    elif [[ "${has_fail}" -gt 0 ]]; then
        computed_verdict="FAIL"
    else
        computed_verdict="PASS"
    fi

    if [[ -n "${declared_verdict}" && "${declared_verdict}" != "${computed_verdict}" ]]; then
        add_error "Verdict mismatch in ${manifest_file}: declared '${declared_verdict}', computed '${computed_verdict}'"
        ((file_errors++))
    fi

    # Check environment_fingerprint exists
    local has_fingerprint
    has_fingerprint=$(jq 'has("environment_fingerprint")' "${manifest_file}")
    if [[ "${has_fingerprint}" != "true" ]]; then
        add_error "Missing required field 'environment_fingerprint' in ${manifest_file}"
        ((file_errors++))
    fi

    if [[ "${file_errors}" -eq 0 ]]; then
        echo "  PASS: ${manifest_file}" | tee -a "${LOG_FILE}"
        return 0
    else
        echo "  FAIL: ${manifest_file} (${file_errors} errors)" | tee -a "${LOG_FILE}"
        return 1
    fi
}

# Change to workspace
cd "${WORKSPACE}"

echo "Starting manifest validation..." > "${LOG_FILE}"
echo "Workspace: ${WORKSPACE}" | tee -a "${LOG_FILE}"

# Find and validate evidence manifest files
# Look for files that contain evidence.gate_packet manifests
MANIFESTS_FOUND=0

# Check for manifest files in common locations
while IFS= read -r -d '' manifest_file; do
    MANIFESTS_FOUND=1
    if validate_manifest "${manifest_file}"; then
        ((VALIDATED++))
    fi
done < <(find . -name "manifest.json" -type f ! -path "./node_modules/*" ! -path "./target/*" -print0 2>/dev/null)

# Also check for files with evidence in the name
while IFS= read -r -d '' manifest_file; do
    # Skip if already processed or if it's not an evidence manifest
    if jq -e '.artifact_type == "evidence.gate_packet"' "${manifest_file}" >/dev/null 2>&1; then
        MANIFESTS_FOUND=1
        if validate_manifest "${manifest_file}"; then
            ((VALIDATED++))
        fi
    fi
done < <(find . -name "*evidence*.json" -type f ! -path "./node_modules/*" ! -path "./target/*" -print0 2>/dev/null)

# Check fixtures directory specifically for testing
if [[ -d "oracle-suites/core-v1/fixtures" ]]; then
    while IFS= read -r -d '' manifest_file; do
        if jq -e '.artifact_type == "evidence.gate_packet"' "${manifest_file}" >/dev/null 2>&1; then
            MANIFESTS_FOUND=1
            if validate_manifest "${manifest_file}"; then
                ((VALIDATED++))
            fi
        fi
    done < <(find oracle-suites/core-v1/fixtures -name "manifest*.json" -type f -print0 2>/dev/null)
fi

# Generate result
echo "" | tee -a "${LOG_FILE}"
echo "Validation complete: ${VALIDATED} manifests validated, ${ERRORS} errors, ${WARNINGS} warnings" | tee -a "${LOG_FILE}"

if [[ "${MANIFESTS_FOUND}" -eq 0 ]]; then
    generate_output "PASS" 0 "No evidence manifests found to validate"
    exit 0
elif [[ "${ERRORS}" -gt 0 ]]; then
    generate_output "FAIL" 1 "${ERRORS} manifest validation errors found"
    exit 1
else
    generate_output "PASS" 0 "Validated ${VALIDATED} manifests successfully"
    exit 0
fi

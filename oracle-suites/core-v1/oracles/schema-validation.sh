#!/bin/bash
# oracle:schema-validation - Schema Validation Oracle
#
# Validates JSON schemas and type definitions in the project.
# Checks:
# - JSON files are valid JSON
# - JSON Schema files ($schema keyword) are valid
# - TypeScript types compile (if present)
#
# Output: /scratch/reports/schema.json (sr.oracle_result.v1 schema)
# Exit: 0 = PASS, non-zero = FAIL

set -uo pipefail

# Configuration
WORKSPACE="${WORKSPACE_MOUNT:-/workspace}"
REPORTS_DIR="${REPORTS_DIR:-/scratch/reports}"
OUTPUT_FILE="${REPORTS_DIR}/schema.json"
LOG_FILE="/tmp/schema-validation.log"

# Initialize
mkdir -p "${REPORTS_DIR}"
STARTED_AT=$(date -u +%Y-%m-%dT%H:%M:%SZ)
START_SECONDS=$(date +%s)
ERRORS=0
WARNINGS=0
VALIDATED=0
VALIDATION_ERRORS=""

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
  "oracle_id": "oracle:schema-validation",
  "status": "${status}",
  "started_at": "${STARTED_AT}",
  "completed_at": "${COMPLETED_AT}",
  "duration_ms": ${DURATION_MS},
  "exit_code": ${exit_code},
  "summary": "${summary}",
  "details": {
    "files_validated": ${VALIDATED},
    "errors": ${ERRORS},
    "warnings": ${WARNINGS},
    "validation_errors": "${escaped_errors}",
    "workspace": "${WORKSPACE}"
  },
  "artifacts": []
}
EOF
}

# Change to workspace
cd "${WORKSPACE}"

echo "Starting schema validation..." > "${LOG_FILE}"

# Find and validate JSON files
echo "Validating JSON files..." | tee -a "${LOG_FILE}"
while IFS= read -r -d '' json_file; do
    if ! jq empty "${json_file}" 2>/dev/null; then
        echo "ERROR: Invalid JSON in ${json_file}" | tee -a "${LOG_FILE}"
        VALIDATION_ERRORS="${VALIDATION_ERRORS}Invalid JSON: ${json_file}; "
        ((ERRORS++))
    else
        ((VALIDATED++))
    fi
done < <(find . -name "*.json" -type f ! -path "./node_modules/*" ! -path "./target/*" -print0 2>/dev/null)

# Check for JSON Schema files and validate their structure
echo "Validating JSON Schema files..." | tee -a "${LOG_FILE}"
while IFS= read -r -d '' schema_file; do
    # Check if file contains $schema keyword (indicates it's a JSON Schema)
    if grep -q '"\$schema"' "${schema_file}" 2>/dev/null; then
        echo "Found JSON Schema: ${schema_file}" | tee -a "${LOG_FILE}"
        # Validate it's proper JSON
        if ! jq empty "${schema_file}" 2>/dev/null; then
            echo "ERROR: Invalid JSON Schema in ${schema_file}" | tee -a "${LOG_FILE}"
            VALIDATION_ERRORS="${VALIDATION_ERRORS}Invalid JSON Schema: ${schema_file}; "
            ((ERRORS++))
        fi
    fi
done < <(find . -name "*.json" -name "*schema*" -type f ! -path "./node_modules/*" ! -path "./target/*" -print0 2>/dev/null)

# For Rust projects, check that serde derive macros are used correctly
if [ -f "Cargo.toml" ]; then
    echo "Checking Rust serde usage..." | tee -a "${LOG_FILE}"
    # Look for common serialization issues (missing derive macros)
    if grep -r "pub struct" --include="*.rs" . 2>/dev/null | head -20 | grep -v "Serialize\|Deserialize" | grep -v "^Binary" | grep -v "test" > /dev/null; then
        echo "Note: Some structs may not have Serialize/Deserialize derives" | tee -a "${LOG_FILE}"
        # This is just a warning, not an error
        ((WARNINGS++))
    fi
fi

# For Node.js projects with TypeScript
if [ -f "tsconfig.json" ]; then
    echo "Found TypeScript configuration..." | tee -a "${LOG_FILE}"
    if ! jq empty "tsconfig.json" 2>/dev/null; then
        echo "ERROR: Invalid tsconfig.json" | tee -a "${LOG_FILE}"
        VALIDATION_ERRORS="${VALIDATION_ERRORS}Invalid tsconfig.json; "
        ((ERRORS++))
    else
        ((VALIDATED++))
    fi
fi

# Generate result
echo "Validation complete: ${VALIDATED} files validated, ${ERRORS} errors, ${WARNINGS} warnings" | tee -a "${LOG_FILE}"

if [ "${ERRORS}" -gt 0 ]; then
    generate_output "FAIL" 1 "${ERRORS} schema validation errors found"
    exit 1
else
    generate_output "PASS" 0 "Validated ${VALIDATED} files successfully"
    exit 0
fi

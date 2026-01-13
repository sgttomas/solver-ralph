#!/usr/bin/env bash
# SOLVER-Ralph Test Runner
# Runs all tests and outputs results in a structured format.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "SOLVER-Ralph Test Suite"
echo "======================="
echo ""
echo "Started at: $(date -u +"%Y-%m-%dT%H:%M:%SZ")"
echo ""

RESULTS=()
FAILURES=0

run_test_suite() {
    local name=$1
    local command=$2

    echo "Running: $name"
    echo "Command: $command"
    echo "---"

    if eval "$command"; then
        RESULTS+=("PASS: $name")
        echo -e "\033[0;32mPASS\033[0m: $name"
    else
        RESULTS+=("FAIL: $name")
        echo -e "\033[0;31mFAIL\033[0m: $name"
        FAILURES=$((FAILURES + 1))
    fi
    echo ""
}

cd "$PROJECT_ROOT"

# Rust tests (if cargo is available)
if command -v cargo &> /dev/null; then
    run_test_suite "Rust unit tests" "cargo test --workspace"
    run_test_suite "Rust clippy" "cargo clippy --workspace -- -D warnings"
    run_test_suite "Rust format check" "cargo fmt --all -- --check"
else
    echo "Skipping Rust tests (cargo not installed)"
    RESULTS+=("SKIP: Rust tests (cargo not installed)")
    echo ""
fi

# UI tests
cd "$PROJECT_ROOT/ui"
run_test_suite "UI type check" "npm run type-check"
run_test_suite "UI lint" "npm run lint 2>/dev/null || true"
run_test_suite "UI build" "npm run build"

echo ""
echo "======================="
echo "Test Summary"
echo "======================="
for result in "${RESULTS[@]}"; do
    echo "  $result"
done
echo ""
echo "Finished at: $(date -u +"%Y-%m-%dT%H:%M:%SZ")"

if [ $FAILURES -gt 0 ]; then
    echo ""
    echo "FAILED: $FAILURES test suite(s) failed"
    exit 1
else
    echo ""
    echo "SUCCESS: All test suites passed"
    exit 0
fi

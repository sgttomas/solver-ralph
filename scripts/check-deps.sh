#!/usr/bin/env bash
# SOLVER-Ralph Dependency Checker
# Checks that all required development dependencies are installed.

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "SOLVER-Ralph Dependency Check"
echo "=============================="
echo ""

MISSING=0

check_command() {
    local cmd=$1
    local name=$2
    local install_hint=$3

    if command -v "$cmd" &> /dev/null; then
        local version
        version=$("$cmd" --version 2>&1 | head -1)
        echo -e "${GREEN}✓${NC} $name: $version"
    else
        echo -e "${RED}✗${NC} $name: NOT FOUND"
        echo -e "  ${YELLOW}Install:${NC} $install_hint"
        MISSING=$((MISSING + 1))
    fi
}

echo "Core tools:"
check_command "rustc" "Rust compiler" "https://rustup.rs/"
check_command "cargo" "Cargo (Rust package manager)" "https://rustup.rs/"
check_command "node" "Node.js" "https://nodejs.org/ or 'brew install node'"
check_command "npm" "npm" "Included with Node.js"

echo ""
echo "Optional tools (for full stack):"
check_command "docker" "Docker" "https://docs.docker.com/get-docker/"
check_command "podman" "Podman" "https://podman.io/getting-started/installation"
check_command "psql" "PostgreSQL client" "'brew install postgresql' or 'apt install postgresql-client'"

echo ""
echo "Linting tools:"
check_command "rustfmt" "Rust formatter" "'rustup component add rustfmt'"
check_command "clippy-driver" "Clippy (Rust linter)" "'rustup component add clippy'"

echo ""
if [ $MISSING -eq 0 ]; then
    echo -e "${GREEN}All required dependencies are installed!${NC}"
    exit 0
else
    echo -e "${YELLOW}Missing $MISSING required dependencies.${NC}"
    echo "Please install the missing dependencies before continuing."
    exit 1
fi

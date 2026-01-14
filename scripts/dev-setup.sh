#!/usr/bin/env bash
# SOLVER-Ralph Development Environment Setup
# Run this script to set up your local development environment.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "SOLVER-Ralph Development Setup"
echo "==============================="
echo ""

cd "$PROJECT_ROOT"

# Check dependencies
echo "Checking dependencies..."
if ! "$SCRIPT_DIR/check-deps.sh"; then
    echo ""
    echo "Please install missing dependencies before continuing."
    echo "Run 'scripts/check-deps.sh' to see what's missing."
    exit 1
fi

echo ""
echo "Installing Rust dependencies..."
if command -v cargo &> /dev/null; then
    cargo fetch
    echo "Rust dependencies installed."
else
    echo "Skipping Rust dependencies (cargo not found)."
fi

echo ""
echo "Installing UI dependencies..."
cd "$PROJECT_ROOT/ui"
npm install
echo "UI dependencies installed."

echo ""
echo "Building projects..."
cd "$PROJECT_ROOT"

if command -v cargo &> /dev/null; then
    echo "Building Rust workspace..."
    cargo build --workspace
fi

echo "Building UI..."
cd "$PROJECT_ROOT/ui"
npm run build

echo ""
echo "=============================="
echo "Setup complete!"
echo ""
echo "Available commands:"
echo "  make dev      - Start development servers"
echo "  make test     - Run all tests"
echo "  make build    - Build all components"
echo "  make lint     - Run linters"
echo ""
echo "For more commands, run: make help"

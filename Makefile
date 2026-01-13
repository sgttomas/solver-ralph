# SOLVER-Ralph Build Tooling
#
# Per D-02/D-04, provides a single documented command to build/test the project.
# Run `make help` to see available targets.

.PHONY: help build build-rust build-ui test test-rust test-ui lint clean dev dev-api dev-ui

# Default target
help:
	@echo "SOLVER-Ralph Build Commands"
	@echo ""
	@echo "Usage: make <target>"
	@echo ""
	@echo "Build targets:"
	@echo "  build       - Build all components (Rust + UI)"
	@echo "  build-rust  - Build Rust workspace"
	@echo "  build-ui    - Build UI (requires npm install first)"
	@echo ""
	@echo "Test targets:"
	@echo "  test        - Run all tests"
	@echo "  test-rust   - Run Rust tests"
	@echo "  test-ui     - Run UI tests"
	@echo ""
	@echo "Development targets:"
	@echo "  dev         - Start development environment"
	@echo "  dev-api     - Start API server in development mode"
	@echo "  dev-ui      - Start UI development server"
	@echo ""
	@echo "Other targets:"
	@echo "  lint        - Run linters"
	@echo "  clean       - Clean build artifacts"
	@echo "  install     - Install dependencies"

# Build all components
build: build-rust build-ui

# Build Rust workspace
build-rust:
	@echo "Building Rust workspace..."
	cargo build --workspace

# Build UI
build-ui:
	@echo "Building UI..."
	cd ui && npm run build

# Run all tests
test: test-rust

# Run Rust tests
test-rust:
	@echo "Running Rust tests..."
	cargo test --workspace

# Run UI tests (placeholder - no tests yet)
test-ui:
	@echo "Running UI tests..."
	cd ui && npm run type-check

# Run linters
lint:
	@echo "Running Rust lints..."
	cargo clippy --workspace -- -D warnings
	@echo "Running UI lints..."
	cd ui && npm run lint

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf ui/dist ui/node_modules

# Install all dependencies
install:
	@echo "Installing Rust dependencies..."
	cargo fetch
	@echo "Installing UI dependencies..."
	cd ui && npm install

# Development mode - start all services
dev: dev-api

# Start API server in development mode
dev-api:
	@echo "Starting API server..."
	cargo run --bin sr-api

# Start UI development server
dev-ui:
	@echo "Starting UI development server..."
	cd ui && npm run dev

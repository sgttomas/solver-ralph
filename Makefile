# SOLVER-Ralph Build Tooling
#
# Per D-02/D-04, provides a single documented command to build/test the project.
# Run `make help` to see available targets.

.PHONY: help build build-rust build-ui test test-rust test-ui test-integration test-e2e lint clean dev dev-api dev-ui deploy deploy-up deploy-down deploy-logs

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
	@echo "  test            - Run all tests"
	@echo "  test-rust       - Run Rust tests"
	@echo "  test-ui         - Run UI tests"
	@echo "  test-integration - Run integration tests (DB/MinIO/NATS/API)"
	@echo "  test-e2e        - Run end-to-end tests"
	@echo ""
	@echo "Development targets:"
	@echo "  dev         - Start development environment"
	@echo "  dev-api     - Start API server in development mode"
	@echo "  dev-ui      - Start UI development server"
	@echo ""
	@echo "Deployment targets:"
	@echo "  deploy      - Start infrastructure services (Postgres, MinIO, NATS, Zitadel)"
	@echo "  deploy-up   - Start full stack including SR services"
	@echo "  deploy-down - Stop all deployment services"
	@echo "  deploy-logs - Follow deployment service logs"
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

# Run integration tests (D-26)
# Requires: deploy (infrastructure must be running)
test-integration:
	@echo "Running integration tests..."
	cargo run --bin sr-oracles -- integration --output /tmp/integration-report.json
	@echo "Integration report: /tmp/integration-report.json"

# Run end-to-end tests (D-26)
# Requires: deploy-up (full stack must be running)
test-e2e:
	@echo "Running end-to-end tests..."
	cargo run --bin sr-oracles -- e2e --output /tmp/e2e-report.json
	@echo "E2E report: /tmp/e2e-report.json"

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

# Deploy infrastructure services (D-31)
deploy:
	@echo "Starting infrastructure services..."
	./deploy/start.sh

# Deploy full stack including SR services
deploy-up:
	@echo "Starting full deployment stack..."
	./deploy/start.sh --full

# Stop all deployment services
deploy-down:
	@echo "Stopping deployment services..."
	./deploy/start.sh --down

# Follow deployment logs
deploy-logs:
	./deploy/start.sh --logs

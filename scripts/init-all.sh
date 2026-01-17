#!/bin/bash
# SOLVER-Ralph Complete Initialization Script (V11-2)
#
# This is a convenience wrapper that performs pre-flight checks
# and then runs the main initialization script.
#
# Usage:
#   ./scripts/init-all.sh                # Full initialization
#   ./scripts/init-all.sh --check        # Check status only
#   ./scripts/init-all.sh --skip-deps    # Skip dependency check
#
# This script:
# 1. Verifies required dependencies are installed
# 2. Starts Docker services if not running
# 3. Runs the main deploy/init.sh script
# 4. Verifies initialization was successful

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_section() {
    echo ""
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}========================================${NC}"
}

# Check if Docker is running
check_docker() {
    if ! docker info &> /dev/null; then
        log_error "Docker is not running. Please start Docker first."
        return 1
    fi
    return 0
}

# Check if required services are up
check_services() {
    local all_up=true

    if ! docker ps --format '{{.Names}}' | grep -q "sr-postgres"; then
        log_warn "PostgreSQL container not running"
        all_up=false
    fi

    if ! docker ps --format '{{.Names}}' | grep -q "sr-minio"; then
        log_warn "MinIO container not running"
        all_up=false
    fi

    if ! docker ps --format '{{.Names}}' | grep -q "sr-nats"; then
        log_warn "NATS container not running"
        all_up=false
    fi

    if [ "$all_up" = false ]; then
        return 1
    fi
    return 0
}

# Start Docker services
start_services() {
    log_info "Starting Docker services..."
    cd "$PROJECT_ROOT/deploy"
    docker compose up -d

    # Wait for services to be ready
    log_info "Waiting for services to start..."
    sleep 10

    # Check if services are up
    if check_services; then
        log_info "All services started successfully"
        return 0
    else
        log_error "Some services failed to start"
        return 1
    fi
}

# Run pre-flight dependency check
run_preflight() {
    log_section "Pre-flight Checks"

    # Check Docker
    log_info "Checking Docker..."
    if ! check_docker; then
        exit 1
    fi
    log_info "Docker is running"

    # Run the dependency check script if it exists
    if [ -f "$PROJECT_ROOT/scripts/check-deps.sh" ]; then
        log_info "Running dependency check..."
        if bash "$PROJECT_ROOT/scripts/check-deps.sh"; then
            log_info "Dependency check passed"
        else
            log_warn "Some optional dependencies are missing (see above)"
        fi
    fi
}

# Run main initialization
run_init() {
    log_section "Running Initialization"

    # Check if services are running, start them if not
    if ! check_services; then
        log_info "Services not running, starting them..."
        if ! start_services; then
            log_error "Failed to start services"
            exit 1
        fi
    fi

    # Run the main init script
    log_info "Running deploy/init.sh..."
    cd "$PROJECT_ROOT/deploy"
    bash init.sh "$@"
}

# Verify initialization
verify_init() {
    log_section "Verifying Initialization"

    cd "$PROJECT_ROOT/deploy"
    bash init.sh --check
}

# Show usage
show_usage() {
    echo "SOLVER-Ralph Complete Initialization Script (V11-2)"
    echo ""
    echo "Usage: $0 [options]"
    echo ""
    echo "Options:"
    echo "  (none)        Full initialization with pre-flight checks"
    echo "  --check       Check initialization status only"
    echo "  --skip-deps   Skip dependency check"
    echo "  --help        Show this help message"
    echo ""
    echo "This script:"
    echo "  1. Verifies Docker is running"
    echo "  2. Checks required dependencies"
    echo "  3. Starts Docker services if needed"
    echo "  4. Runs deploy/init.sh for database/storage initialization"
    echo "  5. Verifies initialization was successful"
    echo ""
}

# Main
case "${1:-}" in
    --check)
        verify_init
        ;;
    --skip-deps)
        shift
        log_info "Skipping dependency check"
        check_docker || exit 1
        run_init "$@"
        verify_init
        ;;
    --help|-h)
        show_usage
        ;;
    "")
        run_preflight
        run_init
        verify_init
        log_section "Initialization Complete"
        log_info "SOLVER-Ralph is ready!"
        log_info ""
        log_info "Next steps:"
        log_info "  1. Configure Zitadel OIDC (see deploy/zitadel-config.json)"
        log_info "  2. Update .env with your configuration"
        log_info "  3. Start the API: cargo run --bin sr-api"
        log_info ""
        ;;
    *)
        # Pass through to init.sh
        run_preflight
        run_init "$@"
        ;;
esac

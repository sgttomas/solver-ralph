#!/bin/bash
# SOLVER-Ralph Self-Host Deployment Script (D-31)
#
# This script starts the self-host deployment stack.
#
# Usage:
#   ./start.sh           # Start infrastructure only
#   ./start.sh --full    # Start with SR services (requires build)
#   ./start.sh --down    # Stop all services
#   ./start.sh --logs    # Follow service logs

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
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

# Check for required tools
check_requirements() {
    if ! command -v docker &> /dev/null; then
        log_error "Docker is not installed. Please install Docker first."
        exit 1
    fi

    if ! docker compose version &> /dev/null; then
        log_error "Docker Compose is not available. Please install Docker Compose."
        exit 1
    fi
}

# Start infrastructure services only
start_infra() {
    log_info "Starting infrastructure services..."
    docker compose up -d postgres minio nats zitadel minio-init

    log_info "Waiting for services to be healthy..."
    sleep 5

    log_info "Service status:"
    docker compose ps

    echo ""
    log_info "Infrastructure services started!"
    echo ""
    echo "Available services:"
    echo "  - PostgreSQL: localhost:5432"
    echo "  - MinIO API:  localhost:9000"
    echo "  - MinIO Console: localhost:9001"
    echo "  - NATS:       localhost:4222"
    echo "  - Zitadel:    localhost:8080"
    echo ""
    echo "Default credentials:"
    echo "  PostgreSQL: postgres/postgres"
    echo "  MinIO:      minioadmin/minioadmin"
    echo "  Zitadel:    admin/Admin123!"
}

# Start all services including SR apps
start_full() {
    log_info "Starting full stack with SR services..."

    # Check if images need to be built
    if ! docker images | grep -q "sr-api"; then
        log_warn "SR API image not found. Building..."
        docker compose build sr-api
    fi

    docker compose --profile full up -d

    log_info "Waiting for services to be healthy..."
    sleep 10

    log_info "Service status:"
    docker compose ps

    echo ""
    log_info "Full stack started!"
    echo ""
    echo "Available services:"
    echo "  - SR API:     localhost:3000"
    echo "  - SR UI:      localhost:5173"
    echo "  - PostgreSQL: localhost:5432"
    echo "  - MinIO API:  localhost:9000"
    echo "  - MinIO Console: localhost:9001"
    echo "  - NATS:       localhost:4222"
    echo "  - Zitadel:    localhost:8080"
}

# Stop all services
stop_all() {
    log_info "Stopping all services..."
    docker compose --profile full down
    log_info "All services stopped."
}

# Follow logs
follow_logs() {
    log_info "Following service logs (Ctrl+C to exit)..."
    docker compose logs -f
}

# Show help
show_help() {
    echo "SOLVER-Ralph Self-Host Deployment"
    echo ""
    echo "Usage: $0 [option]"
    echo ""
    echo "Options:"
    echo "  (none)    Start infrastructure services only"
    echo "  --full    Start full stack including SR services"
    echo "  --down    Stop all services"
    echo "  --logs    Follow service logs"
    echo "  --help    Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0              # Start Postgres, MinIO, NATS, Zitadel"
    echo "  $0 --full       # Start everything including SR API"
    echo "  $0 --down       # Stop all services"
}

# Main
check_requirements

case "${1:-}" in
    --full)
        start_full
        ;;
    --down)
        stop_all
        ;;
    --logs)
        follow_logs
        ;;
    --help|-h)
        show_help
        ;;
    "")
        start_infra
        ;;
    *)
        log_error "Unknown option: $1"
        show_help
        exit 1
        ;;
esac

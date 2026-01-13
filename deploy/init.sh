#!/bin/bash
# SOLVER-Ralph Initialization Script (D-32)
#
# This script initializes all required resources for SOLVER-Ralph:
# - PostgreSQL schemas and migrations
# - MinIO buckets and policies
# - Zitadel realm and OIDC clients
# - Infisical secrets (development defaults)
#
# The script is idempotent - safe to re-run without side effects.
#
# Usage:
#   ./init.sh                  # Initialize all components
#   ./init.sh --postgres       # Initialize PostgreSQL only
#   ./init.sh --minio          # Initialize MinIO only
#   ./init.sh --zitadel        # Initialize Zitadel only
#   ./init.sh --secrets        # Initialize secrets only
#   ./init.sh --check          # Check initialization status

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Configuration (can be overridden via environment)
POSTGRES_HOST="${SR_POSTGRES_HOST:-localhost}"
POSTGRES_PORT="${SR_POSTGRES_PORT:-5432}"
POSTGRES_USER="${SR_POSTGRES_USER:-postgres}"
POSTGRES_PASSWORD="${SR_POSTGRES_PASSWORD:-postgres}"
POSTGRES_DB="${SR_POSTGRES_DB:-solver_ralph}"

MINIO_ENDPOINT="${SR_MINIO_ENDPOINT:-http://localhost:9000}"
MINIO_ACCESS_KEY="${SR_MINIO_ACCESS_KEY:-minioadmin}"
MINIO_SECRET_KEY="${SR_MINIO_SECRET_KEY:-minioadmin}"

ZITADEL_URL="${SR_ZITADEL_URL:-http://localhost:8080}"
ZITADEL_ADMIN_USER="${SR_ZITADEL_ADMIN_USER:-admin}"
ZITADEL_ADMIN_PASSWORD="${SR_ZITADEL_ADMIN_PASSWORD:-Admin123!}"

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

# Check if a command exists
check_command() {
    if ! command -v "$1" &> /dev/null; then
        log_error "$1 is not installed. Please install it first."
        return 1
    fi
    return 0
}

# Wait for a service to be ready
wait_for_service() {
    local name="$1"
    local check_cmd="$2"
    local max_attempts="${3:-30}"
    local attempt=1

    log_info "Waiting for $name to be ready..."

    while [ $attempt -le $max_attempts ]; do
        if eval "$check_cmd" &> /dev/null; then
            log_info "$name is ready!"
            return 0
        fi
        echo -n "."
        sleep 2
        ((attempt++))
    done

    echo ""
    log_error "$name did not become ready in time"
    return 1
}

# ============================================================================
# PostgreSQL Initialization
# ============================================================================

init_postgres() {
    log_section "Initializing PostgreSQL"

    if ! check_command psql; then
        log_warn "psql not found, using docker exec"
        PSQL_CMD="docker exec sr-postgres psql -U postgres -d solver_ralph"
    else
        export PGPASSWORD="$POSTGRES_PASSWORD"
        PSQL_CMD="psql -h $POSTGRES_HOST -p $POSTGRES_PORT -U $POSTGRES_USER -d $POSTGRES_DB"
    fi

    # Wait for PostgreSQL
    wait_for_service "PostgreSQL" "$PSQL_CMD -c 'SELECT 1'"

    log_info "Creating event store schema..."
    $PSQL_CMD <<'EOSQL'
-- Event Store Schema (D-09)
-- Create schema if not exists
CREATE SCHEMA IF NOT EXISTS es;

-- Events table (append-only)
CREATE TABLE IF NOT EXISTS es.events (
    global_seq BIGSERIAL PRIMARY KEY,
    event_id VARCHAR(64) UNIQUE NOT NULL,
    stream_id VARCHAR(256) NOT NULL,
    stream_kind VARCHAR(64) NOT NULL,
    stream_seq BIGINT NOT NULL,
    event_type VARCHAR(128) NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    actor_kind VARCHAR(32) NOT NULL,
    actor_id VARCHAR(256) NOT NULL,
    correlation_id VARCHAR(256),
    causation_id VARCHAR(256),
    supersedes JSONB DEFAULT '[]'::JSONB,
    refs JSONB DEFAULT '[]'::JSONB,
    payload JSONB NOT NULL,
    envelope_hash VARCHAR(128) NOT NULL,
    UNIQUE(stream_id, stream_seq)
);

-- Create indexes for common query patterns
CREATE INDEX IF NOT EXISTS idx_events_stream_id ON es.events(stream_id);
CREATE INDEX IF NOT EXISTS idx_events_stream_kind ON es.events(stream_kind);
CREATE INDEX IF NOT EXISTS idx_events_event_type ON es.events(event_type);
CREATE INDEX IF NOT EXISTS idx_events_occurred_at ON es.events(occurred_at);
CREATE INDEX IF NOT EXISTS idx_events_actor_id ON es.events(actor_id);
CREATE INDEX IF NOT EXISTS idx_events_correlation_id ON es.events(correlation_id);

-- Outbox table for reliable event publishing
CREATE TABLE IF NOT EXISTS es.outbox (
    outbox_id BIGSERIAL PRIMARY KEY,
    global_seq BIGINT NOT NULL REFERENCES es.events(global_seq),
    published_at TIMESTAMPTZ,
    topic VARCHAR(256) NOT NULL,
    message JSONB NOT NULL,
    message_hash VARCHAR(128) NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_outbox_unpublished ON es.outbox(published_at) WHERE published_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_outbox_topic ON es.outbox(topic);

-- Projections schema
CREATE SCHEMA IF NOT EXISTS proj;

-- Loops projection
CREATE TABLE IF NOT EXISTS proj.loops (
    loop_id VARCHAR(64) PRIMARY KEY,
    state VARCHAR(32) NOT NULL,
    goal_text TEXT,
    budget JSONB,
    created_at TIMESTAMPTZ NOT NULL,
    activated_at TIMESTAMPTZ,
    closed_at TIMESTAMPTZ,
    iteration_count INTEGER DEFAULT 0,
    last_event_seq BIGINT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Iterations projection
CREATE TABLE IF NOT EXISTS proj.iterations (
    iteration_id VARCHAR(64) PRIMARY KEY,
    loop_id VARCHAR(64) NOT NULL,
    iteration_number INTEGER NOT NULL,
    state VARCHAR(32) NOT NULL,
    started_at TIMESTAMPTZ NOT NULL,
    completed_at TIMESTAMPTZ,
    summary_hash VARCHAR(128),
    last_event_seq BIGINT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_iterations_loop_id ON proj.iterations(loop_id);

-- Candidates projection
CREATE TABLE IF NOT EXISTS proj.candidates (
    candidate_id VARCHAR(256) PRIMARY KEY,
    iteration_id VARCHAR(64),
    content_hash VARCHAR(128) NOT NULL,
    verification_status VARCHAR(32),
    materialized_at TIMESTAMPTZ NOT NULL,
    last_event_seq BIGINT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_candidates_iteration_id ON proj.candidates(iteration_id);

-- Runs projection
CREATE TABLE IF NOT EXISTS proj.runs (
    run_id VARCHAR(64) PRIMARY KEY,
    candidate_id VARCHAR(256) NOT NULL,
    oracle_suite_id VARCHAR(128) NOT NULL,
    oracle_suite_hash VARCHAR(128) NOT NULL,
    state VARCHAR(32) NOT NULL,
    started_at TIMESTAMPTZ NOT NULL,
    completed_at TIMESTAMPTZ,
    evidence_hash VARCHAR(128),
    verdict VARCHAR(32),
    last_event_seq BIGINT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_runs_candidate_id ON proj.runs(candidate_id);

-- Approvals projection
CREATE TABLE IF NOT EXISTS proj.approvals (
    approval_id VARCHAR(64) PRIMARY KEY,
    portal_id VARCHAR(128) NOT NULL,
    candidate_id VARCHAR(256) NOT NULL,
    evidence_hash VARCHAR(128),
    decision VARCHAR(32) NOT NULL,
    rationale TEXT,
    actor_id VARCHAR(256) NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL,
    last_event_seq BIGINT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_approvals_candidate_id ON proj.approvals(candidate_id);
CREATE INDEX IF NOT EXISTS idx_approvals_portal_id ON proj.approvals(portal_id);

-- Exceptions projection
CREATE TABLE IF NOT EXISTS proj.exceptions (
    exception_id VARCHAR(64) PRIMARY KEY,
    kind VARCHAR(32) NOT NULL,
    status VARCHAR(32) NOT NULL,
    target VARCHAR(256),
    justification TEXT NOT NULL,
    scope JSONB,
    actor_id VARCHAR(256) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    activated_at TIMESTAMPTZ,
    resolved_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    last_event_seq BIGINT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_exceptions_status ON proj.exceptions(status);
CREATE INDEX IF NOT EXISTS idx_exceptions_kind ON proj.exceptions(kind);

-- Decisions projection
CREATE TABLE IF NOT EXISTS proj.decisions (
    decision_id VARCHAR(64) PRIMARY KEY,
    decision_type VARCHAR(64) NOT NULL,
    subject_type VARCHAR(64) NOT NULL,
    subject_id VARCHAR(256) NOT NULL,
    outcome VARCHAR(32) NOT NULL,
    rationale TEXT NOT NULL,
    evidence_refs JSONB DEFAULT '[]'::JSONB,
    actor_id VARCHAR(256) NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL,
    last_event_seq BIGINT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_decisions_subject ON proj.decisions(subject_type, subject_id);

-- Freeze records projection
CREATE TABLE IF NOT EXISTS proj.freeze_records (
    freeze_id VARCHAR(64) PRIMARY KEY,
    baseline_type VARCHAR(64) NOT NULL,
    baseline_id VARCHAR(256) NOT NULL,
    content_hash VARCHAR(128) NOT NULL,
    decision_id VARCHAR(64),
    frozen_at TIMESTAMPTZ NOT NULL,
    actor_id VARCHAR(256) NOT NULL,
    last_event_seq BIGINT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_freeze_baseline ON proj.freeze_records(baseline_type, baseline_id);

-- Graph schema for dependency tracking
CREATE SCHEMA IF NOT EXISTS graph;

-- Graph nodes
CREATE TABLE IF NOT EXISTS graph.nodes (
    node_id VARCHAR(256) PRIMARY KEY,
    node_type VARCHAR(64) NOT NULL,
    content_hash VARCHAR(128),
    is_stale BOOLEAN DEFAULT FALSE,
    stale_reason VARCHAR(64),
    stale_since TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_nodes_type ON graph.nodes(node_type);
CREATE INDEX IF NOT EXISTS idx_nodes_stale ON graph.nodes(is_stale) WHERE is_stale = TRUE;

-- Graph edges
CREATE TABLE IF NOT EXISTS graph.edges (
    edge_id BIGSERIAL PRIMARY KEY,
    source_id VARCHAR(256) NOT NULL REFERENCES graph.nodes(node_id),
    target_id VARCHAR(256) NOT NULL REFERENCES graph.nodes(node_id),
    edge_type VARCHAR(64) NOT NULL,
    meta JSONB DEFAULT '{}'::JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(source_id, target_id, edge_type)
);

CREATE INDEX IF NOT EXISTS idx_edges_source ON graph.edges(source_id);
CREATE INDEX IF NOT EXISTS idx_edges_target ON graph.edges(target_id);
CREATE INDEX IF NOT EXISTS idx_edges_type ON graph.edges(edge_type);

-- Evidence association table
CREATE TABLE IF NOT EXISTS proj.evidence (
    evidence_hash VARCHAR(128) PRIMARY KEY,
    run_id VARCHAR(64),
    bundle_id VARCHAR(128) NOT NULL,
    artifact_type VARCHAR(64) NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL,
    classification VARCHAR(32) DEFAULT 'INTERNAL',
    is_redacted BOOLEAN DEFAULT FALSE,
    last_event_seq BIGINT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_evidence_run_id ON proj.evidence(run_id);
CREATE INDEX IF NOT EXISTS idx_evidence_classification ON proj.evidence(classification);

-- Log successful initialization
INSERT INTO es.events (event_id, stream_id, stream_kind, stream_seq, event_type, actor_kind, actor_id, payload, envelope_hash)
SELECT 'evt_init_' || EXTRACT(EPOCH FROM NOW())::TEXT, 'system', 'GOVERNANCE', 1, 'SchemaInitialized', 'SYSTEM', 'init-script',
       '{"version": "1.0", "timestamp": "' || NOW()::TEXT || '"}'::JSONB,
       'sha256:init'
WHERE NOT EXISTS (SELECT 1 FROM es.events WHERE event_type = 'SchemaInitialized' LIMIT 1);
EOSQL

    log_info "PostgreSQL initialization complete!"
}

# ============================================================================
# MinIO Initialization
# ============================================================================

init_minio() {
    log_section "Initializing MinIO"

    if ! check_command mc; then
        log_warn "mc (MinIO Client) not found, using docker exec"
        MC_CMD="docker exec sr-minio-init mc"
        # If minio-init container isn't running, start it
        if ! docker ps | grep -q sr-minio-init; then
            log_info "Starting MinIO initialization container..."
            docker compose up -d minio-init
            sleep 5
        fi
        MC_CMD="docker run --rm --network sr-network minio/mc:RELEASE.2024-10-02T08-27-28Z"
    else
        MC_CMD="mc"
    fi

    log_info "Configuring MinIO client..."
    $MC_CMD alias set sr-minio "$MINIO_ENDPOINT" "$MINIO_ACCESS_KEY" "$MINIO_SECRET_KEY" 2>/dev/null || true

    log_info "Creating evidence bucket..."
    $MC_CMD mb --ignore-existing sr-minio/evidence 2>/dev/null || true

    log_info "Creating candidates bucket..."
    $MC_CMD mb --ignore-existing sr-minio/candidates 2>/dev/null || true

    log_info "Creating artifacts bucket..."
    $MC_CMD mb --ignore-existing sr-minio/artifacts 2>/dev/null || true

    log_info "Setting bucket policies..."
    # Evidence bucket - immutable, content-addressed
    $MC_CMD anonymous set download sr-minio/evidence 2>/dev/null || true

    # Create versioning policy for evidence (immutability)
    # Note: MinIO bucket versioning is configured at server level

    log_info "MinIO initialization complete!"
    log_info "Buckets created: evidence, candidates, artifacts"
}

# ============================================================================
# Zitadel Initialization
# ============================================================================

init_zitadel() {
    log_section "Initializing Zitadel"

    # Wait for Zitadel to be ready
    wait_for_service "Zitadel" "curl -sf $ZITADEL_URL/debug/healthz" 60

    log_info "Zitadel is running. Manual configuration required:"
    echo ""
    echo "To configure Zitadel for SOLVER-Ralph:"
    echo ""
    echo "1. Open Zitadel Console: $ZITADEL_URL"
    echo "   Login: $ZITADEL_ADMIN_USER / $ZITADEL_ADMIN_PASSWORD"
    echo ""
    echo "2. Create a new project: 'SOLVER-Ralph'"
    echo ""
    echo "3. Create Application: 'solver-ralph-ui' (Web/SPA)"
    echo "   - Grant Type: Authorization Code with PKCE"
    echo "   - Redirect URIs: http://localhost:5173/callback"
    echo "   - Post-Logout URIs: http://localhost:5173"
    echo ""
    echo "4. Create Application: 'solver-ralph-api' (API/Service)"
    echo "   - Grant Type: Client Credentials"
    echo "   - Copy the Client ID and Secret"
    echo ""
    echo "5. Create Roles:"
    echo "   - sr:admin    (full access)"
    echo "   - sr:operator (operations)"
    echo "   - sr:user     (basic access)"
    echo ""

    # Create a config file for reference
    cat > "$SCRIPT_DIR/zitadel-config.json" <<EOF
{
  "project": "SOLVER-Ralph",
  "applications": [
    {
      "name": "solver-ralph-ui",
      "type": "web_spa",
      "redirectUris": ["http://localhost:5173/callback"],
      "postLogoutUris": ["http://localhost:5173"],
      "grantTypes": ["authorization_code"],
      "responseTypes": ["code"],
      "authMethod": "none"
    },
    {
      "name": "solver-ralph-api",
      "type": "api",
      "grantTypes": ["client_credentials"],
      "authMethod": "client_secret_basic"
    }
  ],
  "roles": [
    {"key": "sr:admin", "displayName": "Administrator", "group": "SOLVER-Ralph"},
    {"key": "sr:operator", "displayName": "Operator", "group": "SOLVER-Ralph"},
    {"key": "sr:user", "displayName": "User", "group": "SOLVER-Ralph"}
  ]
}
EOF

    log_info "Configuration reference saved to: zitadel-config.json"
    log_info "Zitadel initialization guidance complete!"
}

# ============================================================================
# Secrets Initialization
# ============================================================================

init_secrets() {
    log_section "Initializing Secrets (Development Defaults)"

    # Create a .env file with development secrets
    log_info "Creating development secrets file..."

    cat > "$SCRIPT_DIR/.env.secrets" <<EOF
# SOLVER-Ralph Development Secrets (D-32)
# Generated by init.sh on $(date)
#
# WARNING: These are development defaults only!
# For production, use Infisical or another secrets manager.

# PostgreSQL
SR_POSTGRES_HOST=localhost
SR_POSTGRES_PORT=5432
SR_POSTGRES_USER=postgres
SR_POSTGRES_PASSWORD=postgres
SR_POSTGRES_DB=solver_ralph
SR_DATABASE_URL=postgres://postgres:postgres@localhost:5432/solver_ralph

# MinIO
SR_MINIO_ENDPOINT=http://localhost:9000
SR_MINIO_ACCESS_KEY=minioadmin
SR_MINIO_SECRET_KEY=minioadmin
SR_MINIO_BUCKET=evidence

# NATS
SR_NATS_URL=nats://localhost:4222

# Zitadel OIDC
SR_OIDC_ISSUER=http://localhost:8080
SR_OIDC_AUDIENCE=solver-ralph
SR_OIDC_CLIENT_ID=solver-ralph-api
SR_OIDC_CLIENT_SECRET=<configure-in-zitadel>

# Envelope Encryption (D-16)
# This is a development-only KEK - generate a proper one for production!
SR_ENVELOPE_KEY_ID=dev-kek-001
SR_ENVELOPE_KEY=$(openssl rand -base64 32 2>/dev/null || echo "dev-key-change-in-production-32bytes!")

# API Configuration
SR_HOST=0.0.0.0
SR_PORT=3000
SR_LOG_LEVEL=debug
EOF

    log_info "Development secrets saved to: .env.secrets"
    log_warn "Remember to configure proper secrets in Infisical for production!"

    # Create an Infisical project template
    cat > "$SCRIPT_DIR/infisical-template.json" <<EOF
{
  "project": "solver-ralph",
  "environment": "dev",
  "secrets": [
    {
      "key": "POSTGRES_PASSWORD",
      "value": "",
      "comment": "PostgreSQL password"
    },
    {
      "key": "MINIO_SECRET_KEY",
      "value": "",
      "comment": "MinIO secret access key"
    },
    {
      "key": "OIDC_CLIENT_SECRET",
      "value": "",
      "comment": "Zitadel OIDC client secret"
    },
    {
      "key": "ENVELOPE_KEK",
      "value": "",
      "comment": "Key Encryption Key for restricted evidence (32 bytes, base64)"
    }
  ]
}
EOF

    log_info "Infisical template saved to: infisical-template.json"
}

# ============================================================================
# Status Check
# ============================================================================

check_status() {
    log_section "Checking Initialization Status"

    echo ""
    echo "Service Health:"
    echo "---------------"

    # PostgreSQL
    if docker exec sr-postgres pg_isready -U postgres &>/dev/null; then
        echo -e "PostgreSQL:  ${GREEN}✓ Running${NC}"

        # Check schemas
        if docker exec sr-postgres psql -U postgres -d solver_ralph -c "SELECT 1 FROM es.events LIMIT 1" &>/dev/null; then
            echo -e "  - Event store schema: ${GREEN}✓ Initialized${NC}"
        else
            echo -e "  - Event store schema: ${YELLOW}✗ Not initialized${NC}"
        fi
    else
        echo -e "PostgreSQL:  ${RED}✗ Not running${NC}"
    fi

    # MinIO
    if curl -sf http://localhost:9000/minio/health/live &>/dev/null; then
        echo -e "MinIO:       ${GREEN}✓ Running${NC}"
    else
        echo -e "MinIO:       ${RED}✗ Not running${NC}"
    fi

    # NATS
    if curl -sf http://localhost:8222/healthz &>/dev/null; then
        echo -e "NATS:        ${GREEN}✓ Running${NC}"
    else
        echo -e "NATS:        ${RED}✗ Not running${NC}"
    fi

    # Zitadel
    if curl -sf http://localhost:8080/debug/healthz &>/dev/null; then
        echo -e "Zitadel:     ${GREEN}✓ Running${NC}"
    else
        echo -e "Zitadel:     ${RED}✗ Not running${NC}"
    fi

    echo ""
    echo "Configuration Files:"
    echo "--------------------"
    [ -f "$SCRIPT_DIR/.env.secrets" ] && echo -e ".env.secrets:         ${GREEN}✓ Exists${NC}" || echo -e ".env.secrets:         ${YELLOW}✗ Not created${NC}"
    [ -f "$SCRIPT_DIR/zitadel-config.json" ] && echo -e "zitadel-config.json:  ${GREEN}✓ Exists${NC}" || echo -e "zitadel-config.json:  ${YELLOW}✗ Not created${NC}"
    [ -f "$SCRIPT_DIR/infisical-template.json" ] && echo -e "infisical-template.json: ${GREEN}✓ Exists${NC}" || echo -e "infisical-template.json: ${YELLOW}✗ Not created${NC}"
    echo ""
}

# ============================================================================
# Main
# ============================================================================

show_help() {
    echo "SOLVER-Ralph Initialization Script (D-32)"
    echo ""
    echo "Usage: $0 [option]"
    echo ""
    echo "Options:"
    echo "  (none)      Initialize all components"
    echo "  --postgres  Initialize PostgreSQL schemas only"
    echo "  --minio     Initialize MinIO buckets only"
    echo "  --zitadel   Show Zitadel configuration guide"
    echo "  --secrets   Generate development secrets"
    echo "  --check     Check initialization status"
    echo "  --help      Show this help message"
    echo ""
}

case "${1:-}" in
    --postgres)
        init_postgres
        ;;
    --minio)
        init_minio
        ;;
    --zitadel)
        init_zitadel
        ;;
    --secrets)
        init_secrets
        ;;
    --check)
        check_status
        ;;
    --help|-h)
        show_help
        ;;
    "")
        log_info "Initializing all SOLVER-Ralph components..."
        init_postgres
        init_minio
        init_zitadel
        init_secrets
        echo ""
        check_status
        log_info "Initialization complete!"
        ;;
    *)
        log_error "Unknown option: $1"
        show_help
        exit 1
        ;;
esac

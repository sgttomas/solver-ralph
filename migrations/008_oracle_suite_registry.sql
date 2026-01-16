-- Migration 008: Oracle Suite Registry
-- Per SR-PLAN-V8 Phase V8-1 and SR-CONTRACT C-OR-2
--
-- V8-1: Add persistent storage for oracle suite definitions.
-- Per C-OR-2: Runs MUST pin oracle suite identity at start.
-- This table provides the registry for suite definitions.

-- Oracle suites projection
CREATE TABLE proj.oracle_suites (
    suite_id                 TEXT PRIMARY KEY,
    suite_hash               TEXT NOT NULL UNIQUE,
    oci_image                TEXT NOT NULL,
    oci_image_digest         TEXT NOT NULL,
    environment_constraints  JSONB NOT NULL,
    oracles                  JSONB NOT NULL,
    metadata                 JSONB NOT NULL DEFAULT '{}',
    registered_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    registered_by_kind       TEXT NOT NULL,
    registered_by_id         TEXT NOT NULL,
    status                   TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'deprecated', 'archived'))
);

-- Index for hash lookups (C-OR-2: suite pinning by hash)
CREATE INDEX idx_oracle_suites_hash ON proj.oracle_suites(suite_hash);

-- Index for filtering by status
CREATE INDEX idx_oracle_suites_status ON proj.oracle_suites(status);

-- Index for registration time queries
CREATE INDEX idx_oracle_suites_registered_at ON proj.oracle_suites(registered_at);

COMMENT ON TABLE proj.oracle_suites IS 'Oracle suite registry per SR-PLAN-V8 V8-1 and SR-CONTRACT C-OR-2';
COMMENT ON COLUMN proj.oracle_suites.suite_id IS 'Suite identifier (e.g., suite:SR-SUITE-CORE)';
COMMENT ON COLUMN proj.oracle_suites.suite_hash IS 'Content hash of suite definition for integrity verification';
COMMENT ON COLUMN proj.oracle_suites.oci_image IS 'OCI container image reference';
COMMENT ON COLUMN proj.oracle_suites.oci_image_digest IS 'OCI image digest for pinning';
COMMENT ON COLUMN proj.oracle_suites.environment_constraints IS 'Runtime environment constraints (JSON)';
COMMENT ON COLUMN proj.oracle_suites.oracles IS 'Oracle definitions array (JSON)';
COMMENT ON COLUMN proj.oracle_suites.metadata IS 'Additional metadata (e.g., semantic_set_id)';
COMMENT ON COLUMN proj.oracle_suites.status IS 'Lifecycle status: active, deprecated, or archived';

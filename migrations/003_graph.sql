-- Migration 003: Dependency Graph Schema
-- Per SR-SPEC §1.8 - Graph model for dependency tracking and staleness propagation
--
-- The graph supports traversal for:
-- - Dependency analysis (what does X depend on?)
-- - Impact analysis (what is impacted if X changes?)
-- - Staleness propagation (which dependents need re-evaluation?)

-- Graph schema
CREATE SCHEMA IF NOT EXISTS graph;

-- Nodes table per SR-SPEC §1.8.2
CREATE TABLE graph.nodes (
    node_id     TEXT PRIMARY KEY,
    node_type   TEXT NOT NULL,
    label       TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    meta        JSONB NOT NULL DEFAULT '{}'::JSONB
);

-- Supported node types per SR-SPEC §1.8.1
COMMENT ON TABLE graph.nodes IS 'Graph nodes per SR-SPEC §1.8.1. Supported types: GovernedArtifact, WorkSurface, Intake, ProcedureTemplate, ProcedureStage, SemanticSet, Candidate, OracleSuite, EvidenceBundle, Approval, Decision, Deviation, Deferral, Waiver, Iteration, Loop';

CREATE INDEX idx_nodes_type ON graph.nodes(node_type);

-- Edges table per SR-SPEC §1.8.2
CREATE TABLE graph.edges (
    edge_id     BIGSERIAL PRIMARY KEY,
    src_id      TEXT NOT NULL REFERENCES graph.nodes(node_id),
    dst_id      TEXT NOT NULL REFERENCES graph.nodes(node_id),
    edge_type   TEXT NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    meta        JSONB NOT NULL DEFAULT '{}'::JSONB
);

-- Supported edge types per SR-SPEC Appendix B
COMMENT ON TABLE graph.edges IS 'Graph edges per SR-SPEC Appendix B. Types: about, depends_on, produces, verifies, approved_by, acknowledges, supersedes, releases, supported_by, governed_by, in_scope_of, affects, stale, root_cause, relates_to';

CREATE INDEX idx_edges_src ON graph.edges(src_id, edge_type);
CREATE INDEX idx_edges_dst ON graph.edges(dst_id, edge_type);
CREATE INDEX idx_edges_type ON graph.edges(edge_type);

-- Staleness markers per SR-SPEC §1.13.2
CREATE TABLE graph.stale_nodes (
    stale_id             TEXT PRIMARY KEY,
    root_kind            TEXT NOT NULL,
    root_id              TEXT NOT NULL,
    dependent_kind       TEXT NOT NULL,
    dependent_id         TEXT NOT NULL,
    reason_code          TEXT NOT NULL,
    reason_detail        TEXT,
    marked_at            TIMESTAMPTZ NOT NULL,
    marked_by_kind       TEXT NOT NULL,
    marked_by_id         TEXT NOT NULL,
    resolved_at          TIMESTAMPTZ,
    resolution_event_id  TEXT
);

-- Reason codes per SR-SPEC §1.13
COMMENT ON TABLE graph.stale_nodes IS 'Staleness markers per SR-SPEC §1.13.2. Common reason codes: GOVERNED_ARTIFACT_CHANGED, ORACLE_SUITE_REBASED, EXCEPTION_ACTIVATED, DEPENDENCY_STALE';

-- Indexes for staleness queries
CREATE INDEX idx_stale_root ON graph.stale_nodes(root_kind, root_id) WHERE resolved_at IS NULL;
CREATE INDEX idx_stale_dependent ON graph.stale_nodes(dependent_kind, dependent_id) WHERE resolved_at IS NULL;
CREATE INDEX idx_stale_unresolved ON graph.stale_nodes(marked_at) WHERE resolved_at IS NULL;

-- =============================================================================
-- Utility Functions for Graph Traversal
-- =============================================================================

-- Recursive dependency query (depth-limited) per SR-SPEC §1.8.3
-- Returns all nodes that src_id depends on (transitive)
CREATE OR REPLACE FUNCTION graph.get_dependencies(
    p_src_id TEXT,
    p_max_depth INTEGER DEFAULT 10
)
RETURNS TABLE (
    src_id TEXT,
    dst_id TEXT,
    edge_type TEXT,
    depth INTEGER
) AS $$
BEGIN
    RETURN QUERY
    WITH RECURSIVE walk AS (
        SELECT e.src_id, e.dst_id, e.edge_type, 1 AS depth
        FROM graph.edges e
        WHERE e.src_id = p_src_id
          AND e.edge_type = 'depends_on'

        UNION ALL

        SELECT e.src_id, e.dst_id, e.edge_type, w.depth + 1
        FROM graph.edges e
        JOIN walk w ON e.src_id = w.dst_id
        WHERE w.depth < p_max_depth
          AND e.edge_type = 'depends_on'
    )
    SELECT * FROM walk;
END;
$$ LANGUAGE plpgsql STABLE;

COMMENT ON FUNCTION graph.get_dependencies IS 'Get transitive dependencies of a node per SR-SPEC §1.8.3';

-- Reverse dependency query (depth-limited) per SR-SPEC §1.13.4
-- Returns all nodes that depend on dst_id (impacted dependents)
CREATE OR REPLACE FUNCTION graph.get_dependents(
    p_dst_id TEXT,
    p_max_depth INTEGER DEFAULT 10
)
RETURNS TABLE (
    dependent_id TEXT,
    dependency_id TEXT,
    edge_type TEXT,
    depth INTEGER
) AS $$
BEGIN
    RETURN QUERY
    WITH RECURSIVE dependents AS (
        SELECT e.src_id AS dependent_id, e.dst_id AS dependency_id, e.edge_type, 1 AS depth
        FROM graph.edges e
        WHERE e.dst_id = p_dst_id
          AND e.edge_type = 'depends_on'

        UNION ALL

        SELECT e.src_id, e.dst_id, e.edge_type, d.depth + 1
        FROM graph.edges e
        JOIN dependents d ON e.dst_id = d.dependent_id
        WHERE d.depth < p_max_depth
          AND e.edge_type = 'depends_on'
    )
    SELECT * FROM dependents;
END;
$$ LANGUAGE plpgsql STABLE;

COMMENT ON FUNCTION graph.get_dependents IS 'Get transitive dependents of a node (impact analysis) per SR-SPEC §1.13.4';

-- Function to check if a node has unresolved staleness
CREATE OR REPLACE FUNCTION graph.has_unresolved_staleness(p_node_id TEXT)
RETURNS BOOLEAN AS $$
BEGIN
    RETURN EXISTS (
        SELECT 1 FROM graph.stale_nodes
        WHERE dependent_id = p_node_id
          AND resolved_at IS NULL
    );
END;
$$ LANGUAGE plpgsql STABLE;

COMMENT ON FUNCTION graph.has_unresolved_staleness IS 'Check if a node has unresolved staleness markers';

-- Function to get unresolved staleness markers for a node
CREATE OR REPLACE FUNCTION graph.get_staleness_markers(p_node_id TEXT)
RETURNS TABLE (
    stale_id TEXT,
    root_kind TEXT,
    root_id TEXT,
    reason_code TEXT,
    reason_detail TEXT,
    marked_at TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT s.stale_id, s.root_kind, s.root_id, s.reason_code, s.reason_detail, s.marked_at
    FROM graph.stale_nodes s
    WHERE s.dependent_id = p_node_id
      AND s.resolved_at IS NULL
    ORDER BY s.marked_at DESC;
END;
$$ LANGUAGE plpgsql STABLE;

COMMENT ON FUNCTION graph.get_staleness_markers IS 'Get unresolved staleness markers for a node';

-- Migration 004: Evidence Bundle Projections
-- Per SR-SPEC §1.9 and SR-CONTRACT §7 - Evidence storage and tracking
--
-- D-20: Evidence API endpoints require projection for tracking evidence bundles
-- and their associations with runs, candidates, and iterations.

-- Evidence bundles projection
CREATE TABLE proj.evidence_bundles (
    content_hash         TEXT PRIMARY KEY,
    bundle_id            TEXT NOT NULL,
    run_id               TEXT NOT NULL,
    candidate_id         TEXT NOT NULL,
    iteration_id         TEXT,
    oracle_suite_id      TEXT NOT NULL,
    oracle_suite_hash    TEXT NOT NULL,
    verdict              TEXT NOT NULL CHECK (verdict IN ('PASS', 'FAIL', 'ERROR', 'SKIPPED')),
    artifact_count       INTEGER NOT NULL DEFAULT 0,
    run_completed_at     TIMESTAMPTZ NOT NULL,
    recorded_by_kind     TEXT NOT NULL,
    recorded_by_id       TEXT NOT NULL,
    recorded_at          TIMESTAMPTZ NOT NULL,
    last_event_id        TEXT NOT NULL,
    last_global_seq      BIGINT NOT NULL
);

CREATE INDEX idx_evidence_bundles_run ON proj.evidence_bundles(run_id);
CREATE INDEX idx_evidence_bundles_candidate ON proj.evidence_bundles(candidate_id);
CREATE INDEX idx_evidence_bundles_iteration ON proj.evidence_bundles(iteration_id);
CREATE INDEX idx_evidence_bundles_verdict ON proj.evidence_bundles(verdict);
CREATE INDEX idx_evidence_bundles_recorded_at ON proj.evidence_bundles(recorded_at);

COMMENT ON TABLE proj.evidence_bundles IS 'Evidence bundle projection per SR-SPEC §1.9 and D-20';

-- Evidence associations (for linking evidence to additional domain objects)
CREATE TABLE proj.evidence_associations (
    association_id       SERIAL PRIMARY KEY,
    content_hash         TEXT NOT NULL REFERENCES proj.evidence_bundles(content_hash),
    entity_type          TEXT NOT NULL CHECK (entity_type IN ('run', 'candidate', 'iteration')),
    entity_id            TEXT NOT NULL,
    associated_by_kind   TEXT NOT NULL,
    associated_by_id     TEXT NOT NULL,
    associated_at        TIMESTAMPTZ NOT NULL,
    last_event_id        TEXT NOT NULL,
    last_global_seq      BIGINT NOT NULL,
    UNIQUE (content_hash, entity_type, entity_id)
);

CREATE INDEX idx_evidence_assoc_entity ON proj.evidence_associations(entity_type, entity_id);
CREATE INDEX idx_evidence_assoc_hash ON proj.evidence_associations(content_hash);

COMMENT ON TABLE proj.evidence_associations IS 'Evidence association tracking for D-20 evidence API';

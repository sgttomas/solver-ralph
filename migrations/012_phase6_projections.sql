-- Migration 012: Phase 6 governance & projection extensions
-- - Persist stop trigger history with recommended portal routing
-- - Capture verification metadata and staleness flags on candidates
-- - Surface last recommended portal on loops

-- Stop trigger history (per SR-DIRECTIVE §4.2)
ALTER TABLE proj.loops
    ADD COLUMN IF NOT EXISTS last_recommended_portal TEXT;

CREATE TABLE IF NOT EXISTS proj.loop_stop_triggers (
    stop_id             TEXT PRIMARY KEY,
    loop_id             TEXT NOT NULL REFERENCES proj.loops(loop_id) ON DELETE CASCADE,
    trigger             TEXT NOT NULL,
    condition           TEXT,
    recommended_portal  TEXT,
    requires_decision   BOOLEAN NOT NULL DEFAULT TRUE,
    occurred_at         TIMESTAMPTZ NOT NULL,
    actor_kind          TEXT NOT NULL,
    actor_id            TEXT NOT NULL,
    resolution_event_id TEXT,
    resolved_at         TIMESTAMPTZ,
    last_event_id       TEXT NOT NULL,
    last_global_seq     BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_loop_stop_triggers_loop
    ON proj.loop_stop_triggers(loop_id);

CREATE INDEX IF NOT EXISTS idx_loop_stop_triggers_open
    ON proj.loop_stop_triggers(loop_id)
    WHERE resolved_at IS NULL;

COMMENT ON TABLE proj.loop_stop_triggers IS 'StopTriggered history with routing hints (SR-DIRECTIVE §4.2)';
COMMENT ON COLUMN proj.loops.last_recommended_portal IS 'Latest recommended portal from StopTriggered payload';

-- Candidate verification metadata + staleness flags
ALTER TABLE proj.candidates
    ADD COLUMN IF NOT EXISTS verification_profile_id TEXT,
    ADD COLUMN IF NOT EXISTS verification_integrity_conditions TEXT[] NOT NULL DEFAULT '{}',
    ADD COLUMN IF NOT EXISTS verification_evidence_hashes TEXT[] NOT NULL DEFAULT '{}',
    ADD COLUMN IF NOT EXISTS verification_waiver_ids TEXT[] NOT NULL DEFAULT '{}',
    ADD COLUMN IF NOT EXISTS verification_waived_oracle_ids TEXT[] NOT NULL DEFAULT '{}',
    ADD COLUMN IF NOT EXISTS verification_oracle_summaries JSONB NOT NULL DEFAULT '[]'::jsonb,
    ADD COLUMN IF NOT EXISTS has_unresolved_staleness BOOLEAN NOT NULL DEFAULT FALSE;

COMMENT ON COLUMN proj.candidates.verification_profile_id IS 'Verification profile used for latest computation';
COMMENT ON COLUMN proj.candidates.verification_integrity_conditions IS 'Integrity conditions detected during latest verification';
COMMENT ON COLUMN proj.candidates.verification_evidence_hashes IS 'Evidence bundle hashes considered during latest verification';
COMMENT ON COLUMN proj.candidates.verification_waiver_ids IS 'Waiver IDs applied to verification failures';
COMMENT ON COLUMN proj.candidates.verification_waived_oracle_ids IS 'Oracle IDs waived during verification';
COMMENT ON COLUMN proj.candidates.verification_oracle_summaries IS 'Suite ID/hash summaries for the verification run';
COMMENT ON COLUMN proj.candidates.has_unresolved_staleness IS 'TRUE when graph.stale_nodes has unresolved markers for this candidate';

-- Backfill stop trigger history from event log
INSERT INTO proj.loop_stop_triggers (
    stop_id, loop_id, trigger, condition, recommended_portal,
    requires_decision, occurred_at, actor_kind, actor_id,
    resolution_event_id, resolved_at, last_event_id, last_global_seq
)
SELECT
    e.event_id,
    e.stream_id,
    COALESCE(e.payload->>'trigger', 'UNKNOWN'),
    COALESCE(e.payload->>'condition', e.payload->>'trigger'),
    e.payload->>'recommended_portal',
    COALESCE((e.payload->>'requires_decision')::BOOLEAN, TRUE),
    e.occurred_at,
    e.actor_kind,
    e.actor_id,
    NULL,
    NULL,
    e.event_id,
    e.global_seq
FROM es.events e
WHERE e.event_type = 'StopTriggered'
ON CONFLICT (stop_id) DO NOTHING;

-- Backfill last_recommended_portal/stop trigger metadata on loops when missing
WITH latest_stop AS (
    SELECT
        stream_id AS loop_id,
        COALESCE(e.payload->>'trigger', 'UNKNOWN') AS trigger,
        e.payload->>'recommended_portal' AS recommended_portal,
        e.occurred_at,
        COALESCE((e.payload->>'requires_decision')::BOOLEAN, TRUE) AS requires_decision,
        ROW_NUMBER() OVER (PARTITION BY stream_id ORDER BY global_seq DESC) AS rn
    FROM es.events e
    WHERE e.event_type = 'StopTriggered'
)
UPDATE proj.loops l
SET last_stop_trigger = COALESCE(l.last_stop_trigger, s.trigger),
    paused_at = COALESCE(l.paused_at, s.occurred_at),
    requires_decision = CASE WHEN l.last_stop_trigger IS NULL THEN s.requires_decision ELSE l.requires_decision END,
    last_recommended_portal = COALESCE(l.last_recommended_portal, s.recommended_portal)
FROM latest_stop s
WHERE s.rn = 1
  AND l.loop_id = s.loop_id;

-- Backfill candidate verification metadata from latest CandidateVerificationComputed
WITH latest_verification AS (
    SELECT
        e.stream_id AS candidate_id,
        e.payload->>'verification_profile_id' AS verification_profile_id,
        ARRAY(
            SELECT jsonb_array_elements_text(COALESCE(e.payload->'integrity_conditions', '[]'::jsonb))
        ) AS integrity_conditions,
        ARRAY(
            SELECT jsonb_array_elements_text(COALESCE(e.payload->'evidence_bundle_hashes', '[]'::jsonb))
        ) AS evidence_hashes,
        ARRAY(
            SELECT jsonb_array_elements_text(COALESCE(e.payload->'waiver_ids', '[]'::jsonb))
        ) AS waiver_ids,
        ARRAY(
            SELECT jsonb_array_elements_text(COALESCE(e.payload->'waived_oracle_ids', '[]'::jsonb))
        ) AS waived_oracle_ids,
        COALESCE(e.payload->'oracle_suite_summaries', '[]'::jsonb) AS oracle_suite_summaries,
        ROW_NUMBER() OVER (PARTITION BY e.stream_id ORDER BY e.global_seq DESC) AS rn
    FROM es.events e
    WHERE e.event_type = 'CandidateVerificationComputed'
)
UPDATE proj.candidates c
SET verification_profile_id = lv.verification_profile_id,
    verification_integrity_conditions = COALESCE(lv.integrity_conditions, '{}'::text[]),
    verification_evidence_hashes = COALESCE(lv.evidence_hashes, '{}'::text[]),
    verification_waiver_ids = COALESCE(lv.waiver_ids, '{}'::text[]),
    verification_waived_oracle_ids = COALESCE(lv.waived_oracle_ids, '{}'::text[]),
    verification_oracle_summaries = lv.oracle_suite_summaries
FROM latest_verification lv
WHERE lv.rn = 1
  AND c.candidate_id = lv.candidate_id;

-- Backfill candidate staleness flag
UPDATE proj.candidates c
SET has_unresolved_staleness = EXISTS (
    SELECT 1
    FROM graph.stale_nodes s
    WHERE s.dependent_id = c.candidate_id
      AND s.resolved_at IS NULL
);

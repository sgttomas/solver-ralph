-- Migration 009: Loop Stop Trigger Support (V10-1, V10-2)
--
-- Per SR-CONTRACT C-LOOP-1 (bounded iteration with hard stop) and
-- C-LOOP-3 (mandatory stop triggers), add columns to track:
--   - consecutive_failures: for REPEATED_FAILURE trigger (N >= 3)
--   - last_stop_trigger: what caused the pause
--   - paused_at: when the Loop was paused
--   - requires_decision: whether Decision is required to resume
--
-- Also adds index for candidate traceability (V10-3, C-LOOP-4).

-- Add stop trigger columns to proj.loops
ALTER TABLE proj.loops
    ADD COLUMN IF NOT EXISTS consecutive_failures INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS last_stop_trigger TEXT,
    ADD COLUMN IF NOT EXISTS paused_at TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS requires_decision BOOLEAN NOT NULL DEFAULT false;

-- Add index for candidate -> iteration traceability (V10-3)
CREATE INDEX IF NOT EXISTS idx_candidates_produced_by_iteration
    ON proj.candidates(produced_by_iteration_id);

-- Add comment documenting the stop trigger columns
COMMENT ON COLUMN proj.loops.consecutive_failures IS 'Count of consecutive FAILED iterations for REPEATED_FAILURE trigger';
COMMENT ON COLUMN proj.loops.last_stop_trigger IS 'Trigger type that caused pause: BUDGET_EXHAUSTED, REPEATED_FAILURE, etc.';
COMMENT ON COLUMN proj.loops.paused_at IS 'Timestamp when Loop was paused by stop trigger';
COMMENT ON COLUMN proj.loops.requires_decision IS 'True if Decision required to resume (stop trigger-induced pause)';

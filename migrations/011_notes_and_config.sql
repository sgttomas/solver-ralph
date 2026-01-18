-- Migration 011: Note type extension + config definition registry
-- - Extend human_judgment_records to allow intervention notes (record.intervention_note)
-- - Add config_definitions projection table for config.* type keys

-- Extend human_judgment_records record_type check to include intervention notes
ALTER TABLE proj.human_judgment_records
  DROP CONSTRAINT IF EXISTS human_judgment_records_record_type_check;

ALTER TABLE proj.human_judgment_records
  ADD CONSTRAINT human_judgment_records_record_type_check
    CHECK (record_type IN ('record.evaluation_note', 'record.assessment_note', 'record.intervention_note'));

ALTER TABLE proj.human_judgment_records
  ADD COLUMN IF NOT EXISTS details JSONB NOT NULL DEFAULT '{}'::jsonb;

-- Config definitions projection table
CREATE TABLE IF NOT EXISTS proj.config_definitions (
    config_id         TEXT PRIMARY KEY,
    type_key          TEXT NOT NULL CHECK (type_key IN (
        'config.agent_definition',
        'config.oracle_definition',
        'config.portal_definition',
        'config.semantic_profile'
    )),
    name              TEXT NOT NULL,
    definition        JSONB NOT NULL,
    recorded_by_kind  TEXT NOT NULL,
    recorded_by_id    TEXT NOT NULL,
    recorded_at       TIMESTAMPTZ NOT NULL,
    last_event_id     TEXT NOT NULL,
    last_global_seq   BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_config_definitions_type_key
    ON proj.config_definitions(type_key);

COMMENT ON TABLE proj.config_definitions IS 'Config definition registry for config.* type keys (SR-TYPES §4.5)';

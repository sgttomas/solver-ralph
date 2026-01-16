-- Migration 005: Intakes Projection
-- Per SR-WORK-SURFACE §3 and SR-PLAN-V3
--
-- Intakes are commitment objects when activated.
-- This projection stores the read model for intake lifecycle management.

-- Intake status enum per SR-PLAN-V3 §1.2
-- Maps to SR-TYPES §3.1: draft/active/archived → draft/governed/archived
CREATE TYPE intake_status AS ENUM ('draft', 'active', 'archived');

-- Work kind enum per SR-WORK-SURFACE §3.1
CREATE TYPE work_kind AS ENUM (
    'research_memo',
    'decision_record',
    'ontology_build',
    'analysis_report',
    'technical_spec',
    'implementation_plan',
    'intake_processing',
    'design_document',
    'review_response'
);

-- Intakes projection table
CREATE TABLE proj.intakes (
    -- Identity
    intake_id           TEXT PRIMARY KEY,           -- format: intake:<ULID>
    work_unit_id        TEXT NOT NULL,              -- format: WU-<identifier>
    content_hash        TEXT,                       -- sha256:<hex> - set on activation

    -- Required fields per SR-WORK-SURFACE §3.1
    title               TEXT NOT NULL,
    kind                work_kind NOT NULL,
    objective           TEXT NOT NULL,              -- ONE sentence
    audience            TEXT NOT NULL,
    deliverables        JSONB NOT NULL DEFAULT '[]'::jsonb,
    constraints         JSONB NOT NULL DEFAULT '[]'::jsonb,
    definitions         JSONB NOT NULL DEFAULT '{}'::jsonb,
    inputs              JSONB NOT NULL DEFAULT '[]'::jsonb,  -- StrongTypedRef[]
    unknowns            JSONB NOT NULL DEFAULT '[]'::jsonb,
    completion_criteria JSONB NOT NULL DEFAULT '[]'::jsonb,

    -- Lifecycle
    status              intake_status NOT NULL DEFAULT 'draft',
    version             INTEGER NOT NULL DEFAULT 1,
    supersedes          TEXT REFERENCES proj.intakes(intake_id),

    -- Attribution (per C-EVT-1)
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by_kind     TEXT NOT NULL,              -- HUMAN | AGENT | SYSTEM
    created_by_id       TEXT NOT NULL,              -- per SR-SPEC §1.4.2
    activated_at        TIMESTAMPTZ,
    activated_by_kind   TEXT,
    activated_by_id     TEXT,
    archived_at         TIMESTAMPTZ,
    archived_by_kind    TEXT,
    archived_by_id      TEXT,

    -- Archive reason
    archive_reason      TEXT,

    -- Event tracking
    last_event_id       TEXT NOT NULL,
    last_global_seq     BIGINT NOT NULL,

    -- Constraints
    CONSTRAINT intake_id_format CHECK (intake_id ~ '^intake:[0-9A-Z]+$'),
    CONSTRAINT content_hash_format CHECK (
        content_hash IS NULL OR content_hash ~ '^sha256:[a-f0-9]{64}$'
    ),
    CONSTRAINT active_has_content_hash CHECK (
        status != 'active' OR content_hash IS NOT NULL
    ),
    CONSTRAINT active_has_activation CHECK (
        status != 'active' OR (activated_at IS NOT NULL AND activated_by_id IS NOT NULL)
    ),
    CONSTRAINT archived_has_archive_info CHECK (
        status != 'archived' OR (archived_at IS NOT NULL AND archived_by_id IS NOT NULL)
    )
);

-- Indexes
CREATE INDEX idx_intakes_work_unit ON proj.intakes(work_unit_id);
CREATE INDEX idx_intakes_status ON proj.intakes(status);
CREATE INDEX idx_intakes_kind ON proj.intakes(kind);
CREATE INDEX idx_intakes_created_at ON proj.intakes(created_at);

-- Index for by-hash retrieval (per SR-PLAN-V3 §1.5)
CREATE INDEX idx_intakes_content_hash ON proj.intakes(content_hash)
    WHERE content_hash IS NOT NULL;

-- Unique constraint: only one active intake per content_hash
-- This prevents duplicate activations with same content
CREATE UNIQUE INDEX uniq_intakes_active_content_hash ON proj.intakes(content_hash)
    WHERE status = 'active';

-- Composite index for common query patterns
CREATE INDEX idx_intakes_status_created ON proj.intakes(status, created_at DESC);

-- Comments
COMMENT ON TABLE proj.intakes IS 'Intake projection per SR-WORK-SURFACE §3 and SR-PLAN-V3';
COMMENT ON COLUMN proj.intakes.content_hash IS 'SHA-256 hash computed on activation - commitment object anchor';
COMMENT ON COLUMN proj.intakes.status IS 'Lifecycle status: draft→active→archived (maps to SR-TYPES governed status)';
COMMENT ON COLUMN proj.intakes.inputs IS 'Input references as StrongTypedRef[] per SR-SPEC §1.5.3';
COMMENT ON CONSTRAINT active_has_content_hash ON proj.intakes IS 'Active intakes must have content_hash for commitment object semantics';

-- Add checkpoint for intake projection
INSERT INTO proj.checkpoints (projection_name, last_global_seq, last_event_id, updated_at)
VALUES ('intakes', 0, 'evt_00000000000000000000000000', NOW())
ON CONFLICT (projection_name) DO NOTHING;

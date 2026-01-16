-- Migration: 006_work_surfaces.sql
-- Purpose: Work Surface Composition infrastructure per SR-PLAN-V4
--
-- Creates the proj.work_surfaces table and supporting types for
-- binding Intake + Procedure Template into Work Surface Instances.

-- ============================================================================
-- Enums
-- ============================================================================

-- Work Surface status enum per SR-PLAN-V4 §2.1
-- Uses lifecycle values (not SR-TYPES artifact status) because
-- Work Surfaces are always commitment objects once bound.
CREATE TYPE work_surface_status AS ENUM ('active', 'completed', 'archived');

-- Stage completion status enum per SR-PLAN-V4 §2.1
CREATE TYPE stage_completion_status AS ENUM ('pending', 'entered', 'completed', 'skipped');

-- ============================================================================
-- Work Surfaces Table per SR-PLAN-V4 §2.2
-- ============================================================================

CREATE TABLE proj.work_surfaces (
    -- Identity
    work_surface_id         TEXT PRIMARY KEY,           -- format: ws:<ULID>
    work_unit_id            TEXT NOT NULL,              -- format: WU-<id>

    -- Binding refs (immutable once bound)
    intake_id               TEXT NOT NULL REFERENCES proj.intakes(intake_id),
    intake_content_hash     TEXT NOT NULL,              -- sha256:<hex>
    procedure_template_id   TEXT NOT NULL,              -- format: proc:<NAME>
    procedure_template_hash TEXT NOT NULL,              -- sha256:<hex>

    -- Current state (mutable via events)
    current_stage_id        TEXT NOT NULL,              -- format: stage:<NAME>
    status                  work_surface_status NOT NULL DEFAULT 'active',
    stage_status            JSONB NOT NULL DEFAULT '{}'::jsonb, -- {stage_id: StageStatusRecord}

    -- Oracle context (for current stage, refreshed on stage entry)
    current_oracle_suites   JSONB NOT NULL DEFAULT '[]'::jsonb,

    -- Parameters
    params                  JSONB NOT NULL DEFAULT '{}'::jsonb,

    -- Content hash (of binding - intake + template + initial params)
    content_hash            TEXT NOT NULL,

    -- Attribution (per C-EVT-1)
    bound_at                TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    bound_by_kind           TEXT NOT NULL,              -- HUMAN | SYSTEM
    bound_by_id             TEXT NOT NULL,
    completed_at            TIMESTAMPTZ,
    archived_at             TIMESTAMPTZ,
    archived_by_kind        TEXT,
    archived_by_id          TEXT,

    -- Event tracking for projections
    last_event_id           TEXT NOT NULL,
    last_global_seq         BIGINT NOT NULL,

    -- Format constraints
    CONSTRAINT ws_id_format CHECK (work_surface_id ~ '^ws:[0-9A-Z]+$'),
    CONSTRAINT intake_hash_format CHECK (intake_content_hash ~ '^sha256:[a-f0-9]{64}$'),
    CONSTRAINT template_hash_format CHECK (procedure_template_hash ~ '^sha256:[a-f0-9]{64}$'),
    CONSTRAINT content_hash_format CHECK (content_hash ~ '^sha256:[a-f0-9]{64}$'),

    -- Lifecycle constraints
    CONSTRAINT completed_has_timestamp CHECK (status != 'completed' OR completed_at IS NOT NULL),
    CONSTRAINT archived_has_timestamp CHECK (status != 'archived' OR archived_at IS NOT NULL)
);

-- ============================================================================
-- Indexes per SR-PLAN-V4 §2.2
-- ============================================================================

-- Lookup by work unit (most common query pattern)
CREATE INDEX idx_work_surfaces_work_unit ON proj.work_surfaces(work_unit_id);

-- Lookup by intake (for showing which intakes have work surfaces)
CREATE INDEX idx_work_surfaces_intake ON proj.work_surfaces(intake_id);

-- Filter by status (for listing active/completed/archived)
CREATE INDEX idx_work_surfaces_status ON proj.work_surfaces(status);

-- Filter by current stage (for stage-based queries)
CREATE INDEX idx_work_surfaces_current_stage ON proj.work_surfaces(current_stage_id);

-- Filter by procedure template (for template usage analysis)
CREATE INDEX idx_work_surfaces_template ON proj.work_surfaces(procedure_template_id);

-- Combined index for common list query pattern
CREATE INDEX idx_work_surfaces_status_bound_at ON proj.work_surfaces(status, bound_at DESC);

-- ============================================================================
-- Unique Constraints per SR-PLAN-V4 §2.2
-- ============================================================================

-- Only one active work surface per work unit
-- Per SR-PLAN-V4: "A Work Unit has exactly one active Work Surface Instance at a time"
CREATE UNIQUE INDEX uniq_work_surfaces_active_per_work_unit
    ON proj.work_surfaces(work_unit_id)
    WHERE status = 'active';

-- ============================================================================
-- Stage History Table (Optional - for detailed audit trail)
-- ============================================================================

CREATE TABLE proj.work_surface_stage_history (
    id                      BIGSERIAL PRIMARY KEY,
    work_surface_id         TEXT NOT NULL REFERENCES proj.work_surfaces(work_surface_id),
    stage_id                TEXT NOT NULL,
    status                  stage_completion_status NOT NULL,
    entered_at              TIMESTAMPTZ,
    completed_at            TIMESTAMPTZ,
    evidence_bundle_ref     TEXT,                       -- sha256:<hex>
    iteration_count         INTEGER NOT NULL DEFAULT 0,
    event_id                TEXT NOT NULL,
    recorded_at             TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for stage history
CREATE INDEX idx_ws_stage_history_ws ON proj.work_surface_stage_history(work_surface_id);
CREATE INDEX idx_ws_stage_history_stage ON proj.work_surface_stage_history(work_surface_id, stage_id);

-- ============================================================================
-- Comments
-- ============================================================================

COMMENT ON TABLE proj.work_surfaces IS 'Work Surface Instances binding Intake + Procedure Template per SR-WORK-SURFACE §5';
COMMENT ON COLUMN proj.work_surfaces.work_surface_id IS 'Unique identifier (format: ws:<ULID>)';
COMMENT ON COLUMN proj.work_surfaces.intake_id IS 'Reference to the bound Intake (must be active status)';
COMMENT ON COLUMN proj.work_surfaces.intake_content_hash IS 'Content hash of Intake at binding time (immutable)';
COMMENT ON COLUMN proj.work_surfaces.procedure_template_id IS 'Reference to the bound Procedure Template';
COMMENT ON COLUMN proj.work_surfaces.procedure_template_hash IS 'Content hash of Template at binding time (immutable)';
COMMENT ON COLUMN proj.work_surfaces.current_stage_id IS 'Current procedure stage';
COMMENT ON COLUMN proj.work_surfaces.stage_status IS 'JSONB tracking status of each stage {stage_id: StageStatusRecord}';
COMMENT ON COLUMN proj.work_surfaces.current_oracle_suites IS 'Oracle suites for current stage (refreshed on stage entry)';
COMMENT ON COLUMN proj.work_surfaces.content_hash IS 'Hash of binding (intake_ref + template_ref + params) for content addressing';

COMMENT ON TABLE proj.work_surface_stage_history IS 'Audit trail of stage transitions for Work Surfaces';

-- Migration 014: Add work_surface_id to evidence_bundles
--
-- Part of Fresh UI Build: Link evidence bundles to work surfaces for auto-loading
-- Per SR-PLAN-MVP1 Task B1

-- ============================================================================
-- Add work_surface_id column to evidence_bundles
-- ============================================================================

ALTER TABLE proj.evidence_bundles
    ADD COLUMN work_surface_id TEXT;

-- ============================================================================
-- Add template context columns (for filtering by template/stage)
-- ============================================================================

ALTER TABLE proj.evidence_bundles
    ADD COLUMN template_id TEXT;

ALTER TABLE proj.evidence_bundles
    ADD COLUMN stage_id TEXT;

-- ============================================================================
-- Create indexes for work surface queries
-- ============================================================================

CREATE INDEX idx_evidence_bundles_work_surface
    ON proj.evidence_bundles(work_surface_id)
    WHERE work_surface_id IS NOT NULL;

CREATE INDEX idx_evidence_bundles_template_stage
    ON proj.evidence_bundles(template_id, stage_id)
    WHERE template_id IS NOT NULL;

-- ============================================================================
-- Add column comments
-- ============================================================================

COMMENT ON COLUMN proj.evidence_bundles.work_surface_id IS
    'Work Surface this evidence was recorded under (nullable for legacy data)';

COMMENT ON COLUMN proj.evidence_bundles.template_id IS
    'Template ID this evidence is associated with';

COMMENT ON COLUMN proj.evidence_bundles.stage_id IS
    'Stage ID within the template';

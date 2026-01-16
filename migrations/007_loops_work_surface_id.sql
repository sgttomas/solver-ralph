-- Add work_surface_id to loops table for Loop-Work Surface binding (SR-PLAN-V5 Phase 5b)
-- This enables explicit binding between Loops and their governing Work Surface

ALTER TABLE proj.loops
ADD COLUMN work_surface_id TEXT REFERENCES proj.work_surfaces(work_surface_id);

CREATE INDEX idx_loops_work_surface_id ON proj.loops(work_surface_id);

COMMENT ON COLUMN proj.loops.work_surface_id IS 'Reference to bound Work Surface when loop is created with explicit work_unit';

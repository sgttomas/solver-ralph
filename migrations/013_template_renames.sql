-- Migration 013: Rename procedure_template columns to template
--
-- Part of nomenclature refactor: ProcedureTemplate → Template
-- Per SR-PLAN-MVP1 Task A6

-- Rename columns in proj.work_surfaces
ALTER TABLE proj.work_surfaces
    RENAME COLUMN procedure_template_id TO template_id;

ALTER TABLE proj.work_surfaces
    RENAME COLUMN procedure_template_hash TO template_hash;

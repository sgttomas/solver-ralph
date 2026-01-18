-- Migration 010: Verification scope/basis columns
-- Adds projection fields to store verification mode, scope, and basis payload.

ALTER TABLE proj.candidates
    ADD COLUMN IF NOT EXISTS verification_mode TEXT,
    ADD COLUMN IF NOT EXISTS verification_scope JSONB,
    ADD COLUMN IF NOT EXISTS verification_basis JSONB,
    ADD COLUMN IF NOT EXISTS verification_computed_at TIMESTAMPTZ;

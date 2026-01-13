-- Migration 002: Projection Schema
-- Per SR-SPEC §1.7 - Read models derived from event streams
--
-- IMPORTANT: All projections MUST be rebuildable from es.events alone.
-- No projection may introduce state that cannot be reconstructed from events.

-- Projections schema
CREATE SCHEMA IF NOT EXISTS proj;

-- Loops projection: Work Unit (Loop) read model
CREATE TABLE proj.loops (
    loop_id              TEXT PRIMARY KEY,
    goal                 TEXT NOT NULL,
    work_unit            TEXT NOT NULL,
    state                TEXT NOT NULL CHECK (state IN ('CREATED', 'ACTIVE', 'PAUSED', 'CLOSED')),
    budgets              JSONB NOT NULL DEFAULT '{}'::JSONB,
    directive_ref        JSONB NOT NULL,
    created_by_kind      TEXT NOT NULL,
    created_by_id        TEXT NOT NULL,
    created_at           TIMESTAMPTZ NOT NULL,
    activated_at         TIMESTAMPTZ,
    closed_at            TIMESTAMPTZ,
    iteration_count      INTEGER NOT NULL DEFAULT 0,
    last_event_id        TEXT NOT NULL,
    last_global_seq      BIGINT NOT NULL
);

CREATE INDEX idx_loops_state ON proj.loops(state);
CREATE INDEX idx_loops_created_at ON proj.loops(created_at);

COMMENT ON TABLE proj.loops IS 'Loop/Work Unit projection per SR-SPEC §1.2.3 domain.work_unit';

-- Iterations projection
CREATE TABLE proj.iterations (
    iteration_id         TEXT PRIMARY KEY,
    loop_id              TEXT NOT NULL,
    sequence             INTEGER NOT NULL,
    state                TEXT NOT NULL CHECK (state IN ('STARTED', 'RUNNING', 'COMPLETED', 'FAILED')),
    started_at           TIMESTAMPTZ NOT NULL,
    completed_at         TIMESTAMPTZ,
    refs                 JSONB NOT NULL DEFAULT '[]'::JSONB,
    summary              JSONB,
    last_event_id        TEXT NOT NULL,
    last_global_seq      BIGINT NOT NULL
);

CREATE INDEX idx_iterations_loop_id ON proj.iterations(loop_id);
CREATE INDEX idx_iterations_state ON proj.iterations(state);

COMMENT ON TABLE proj.iterations IS 'Iteration projection per SR-SPEC §1.2.3 domain.loop_record';

-- Candidates projection
CREATE TABLE proj.candidates (
    candidate_id         TEXT PRIMARY KEY,
    content_hash         TEXT NOT NULL,
    produced_by_iteration_id TEXT,
    verification_status  TEXT NOT NULL DEFAULT 'UNVERIFIED' CHECK (verification_status IN ('UNVERIFIED', 'VERIFIED_STRICT', 'VERIFIED_WITH_EXCEPTIONS')),
    created_at           TIMESTAMPTZ NOT NULL,
    refs                 JSONB NOT NULL DEFAULT '[]'::JSONB,
    last_event_id        TEXT NOT NULL,
    last_global_seq      BIGINT NOT NULL
);

CREATE INDEX idx_candidates_verification ON proj.candidates(verification_status);
CREATE INDEX idx_candidates_content_hash ON proj.candidates(content_hash);

COMMENT ON TABLE proj.candidates IS 'Candidate projection per SR-SPEC §1.2.3 domain.candidate';

-- Runs projection
CREATE TABLE proj.runs (
    run_id               TEXT PRIMARY KEY,
    candidate_id         TEXT NOT NULL,
    oracle_suite_id      TEXT NOT NULL,
    oracle_suite_hash    TEXT NOT NULL,
    state                TEXT NOT NULL CHECK (state IN ('STARTED', 'RUNNING', 'COMPLETED', 'FAILED')),
    started_at           TIMESTAMPTZ NOT NULL,
    completed_at         TIMESTAMPTZ,
    actor_kind           TEXT NOT NULL,
    actor_id             TEXT NOT NULL,
    evidence_bundle_hash TEXT,
    last_event_id        TEXT NOT NULL,
    last_global_seq      BIGINT NOT NULL
);

CREATE INDEX idx_runs_candidate_id ON proj.runs(candidate_id);
CREATE INDEX idx_runs_oracle_suite ON proj.runs(oracle_suite_id);
CREATE INDEX idx_runs_state ON proj.runs(state);

COMMENT ON TABLE proj.runs IS 'Run projection for oracle suite executions';

-- Governed artifacts registry per SR-SPEC §1.10.3
CREATE TABLE proj.governed_artifacts (
    artifact_id          TEXT NOT NULL,
    artifact_type        TEXT NOT NULL,
    version              TEXT NOT NULL,
    content_hash         TEXT NOT NULL,
    status               TEXT NOT NULL CHECK (status IN ('draft', 'governed', 'superseded', 'deprecated', 'archived')),
    normative_status     TEXT NOT NULL CHECK (normative_status IN ('normative', 'directional', 'index', 'record', 'evidence')),
    authority_kind       TEXT NOT NULL CHECK (authority_kind IN ('content', 'process', 'record', 'config', 'index')),
    governed_by          TEXT[] NOT NULL DEFAULT '{}',
    tags                 TEXT[] NOT NULL DEFAULT '{}',
    supersedes           TEXT[] NOT NULL DEFAULT '{}',
    is_current           BOOLEAN NOT NULL DEFAULT FALSE,
    recorded_at          TIMESTAMPTZ NOT NULL,
    recorded_by_kind     TEXT NOT NULL,
    recorded_by_id       TEXT NOT NULL,
    last_event_id        TEXT NOT NULL,
    last_global_seq      BIGINT NOT NULL,
    PRIMARY KEY (artifact_id, version)
);

-- Unique index to enforce at most one current version per artifact
CREATE UNIQUE INDEX uniq_governed_artifacts_current
    ON proj.governed_artifacts(artifact_id)
    WHERE is_current = TRUE;

CREATE INDEX idx_governed_artifacts_type ON proj.governed_artifacts(artifact_type);
CREATE INDEX idx_governed_artifacts_status ON proj.governed_artifacts(status);

COMMENT ON TABLE proj.governed_artifacts IS 'Governed artifact registry per SR-SPEC §1.10.3';

-- Decisions projection per SR-SPEC §1.11.3
CREATE TABLE proj.decisions (
    decision_id              TEXT PRIMARY KEY,
    trigger                  TEXT NOT NULL,
    scope                    JSONB NOT NULL,
    decision                 TEXT NOT NULL,
    rationale                TEXT NOT NULL,
    is_precedent             BOOLEAN NOT NULL DEFAULT FALSE,
    applicability            TEXT,
    evidence_refs            TEXT[] NOT NULL DEFAULT '{}',
    exceptions_acknowledged  JSONB NOT NULL DEFAULT '[]'::JSONB,
    decided_by_kind          TEXT NOT NULL CHECK (decided_by_kind = 'HUMAN'),
    decided_by_id            TEXT NOT NULL,
    decided_at               TIMESTAMPTZ NOT NULL,
    last_event_id            TEXT NOT NULL,
    last_global_seq          BIGINT NOT NULL
);

CREATE INDEX idx_decisions_trigger ON proj.decisions(trigger);
CREATE INDEX idx_decisions_decided_at ON proj.decisions(decided_at);

COMMENT ON TABLE proj.decisions IS 'Decision projection per SR-SPEC §1.11.3';
COMMENT ON CONSTRAINT decisions_decided_by_kind_check ON proj.decisions IS 'Decisions require HUMAN actor per C-DEC-1';

-- Human judgment records projection per SR-SPEC §1.11.4
CREATE TABLE proj.human_judgment_records (
    record_id            TEXT PRIMARY KEY,
    record_type          TEXT NOT NULL CHECK (record_type IN ('record.evaluation_note', 'record.assessment_note')),
    subject_refs         JSONB NOT NULL,
    evidence_refs        TEXT[] NOT NULL,
    content              TEXT NOT NULL,
    severity             TEXT,
    fitness_judgment     TEXT,
    recommendations      TEXT,
    is_binding           BOOLEAN NOT NULL DEFAULT FALSE CHECK (is_binding = FALSE),
    recorded_by_kind     TEXT NOT NULL CHECK (recorded_by_kind = 'HUMAN'),
    recorded_by_id       TEXT NOT NULL,
    recorded_at          TIMESTAMPTZ NOT NULL,
    last_event_id        TEXT NOT NULL,
    last_global_seq      BIGINT NOT NULL
);

CREATE INDEX idx_human_judgment_type ON proj.human_judgment_records(record_type);

COMMENT ON TABLE proj.human_judgment_records IS 'Human judgment records (non-binding) per SR-SPEC §1.11.4';
COMMENT ON CONSTRAINT human_judgment_records_is_binding_check ON proj.human_judgment_records IS 'Evaluation/assessment notes are non-binding per C-TB-7';

-- Shippable status projection per SR-SPEC §1.12.5
CREATE TABLE proj.shippable_status (
    candidate_id             TEXT PRIMARY KEY,
    is_verified              BOOLEAN NOT NULL,
    verification_mode        TEXT CHECK (verification_mode IN ('STRICT', 'WITH_EXCEPTIONS')),
    latest_evidence_hash     TEXT,
    release_approval_id      TEXT,
    freeze_id                TEXT,
    has_unresolved_staleness BOOLEAN NOT NULL DEFAULT FALSE,
    computed_at              TIMESTAMPTZ NOT NULL,
    last_event_id            TEXT NOT NULL,
    last_global_seq          BIGINT NOT NULL
);

COMMENT ON TABLE proj.shippable_status IS 'Shippable status projection per SR-SPEC §1.12.5';

-- Approvals projection
CREATE TABLE proj.approvals (
    approval_id              TEXT PRIMARY KEY,
    portal_id                TEXT NOT NULL,
    decision                 TEXT NOT NULL CHECK (decision IN ('APPROVED', 'REJECTED', 'DEFERRED')),
    subject_refs             JSONB NOT NULL,
    evidence_refs            TEXT[] NOT NULL DEFAULT '{}',
    exceptions_acknowledged  TEXT[] NOT NULL DEFAULT '{}',
    rationale                TEXT,
    approved_by_kind         TEXT NOT NULL CHECK (approved_by_kind = 'HUMAN'),
    approved_by_id           TEXT NOT NULL,
    approved_at              TIMESTAMPTZ NOT NULL,
    last_event_id            TEXT NOT NULL,
    last_global_seq          BIGINT NOT NULL
);

CREATE INDEX idx_approvals_portal ON proj.approvals(portal_id);
CREATE INDEX idx_approvals_decision ON proj.approvals(decision);

COMMENT ON TABLE proj.approvals IS 'Portal approval projection per SR-SPEC §2.3.4';

-- Freeze records projection
CREATE TABLE proj.freeze_records (
    freeze_id                TEXT PRIMARY KEY,
    baseline_id              TEXT NOT NULL,
    candidate_id             TEXT NOT NULL,
    verification_mode        TEXT NOT NULL CHECK (verification_mode IN ('STRICT', 'WITH_EXCEPTIONS')),
    oracle_suite_id          TEXT NOT NULL,
    oracle_suite_hash        TEXT NOT NULL,
    evidence_bundle_refs     TEXT[] NOT NULL DEFAULT '{}',
    waiver_refs              TEXT[] NOT NULL DEFAULT '{}',
    release_approval_id      TEXT NOT NULL,
    artifact_manifest        JSONB NOT NULL,
    active_exceptions        JSONB NOT NULL DEFAULT '[]'::JSONB,
    frozen_by_kind           TEXT NOT NULL CHECK (frozen_by_kind = 'HUMAN'),
    frozen_by_id             TEXT NOT NULL,
    frozen_at                TIMESTAMPTZ NOT NULL,
    last_event_id            TEXT NOT NULL,
    last_global_seq          BIGINT NOT NULL
);

CREATE INDEX idx_freeze_candidate ON proj.freeze_records(candidate_id);
CREATE INDEX idx_freeze_baseline ON proj.freeze_records(baseline_id);

COMMENT ON TABLE proj.freeze_records IS 'Freeze record projection per SR-SPEC §1.12';

-- Exceptions projection
CREATE TABLE proj.exceptions (
    exception_id             TEXT PRIMARY KEY,
    kind                     TEXT NOT NULL CHECK (kind IN ('DEVIATION', 'DEFERRAL', 'WAIVER')),
    status                   TEXT NOT NULL CHECK (status IN ('CREATED', 'ACTIVE', 'RESOLVED', 'EXPIRED')),
    scope                    JSONB NOT NULL,
    rationale                TEXT NOT NULL,
    target_description       TEXT NOT NULL,
    created_by_kind          TEXT NOT NULL CHECK (created_by_kind = 'HUMAN'),
    created_by_id            TEXT NOT NULL,
    created_at               TIMESTAMPTZ NOT NULL,
    expires_at               TIMESTAMPTZ,
    resolved_at              TIMESTAMPTZ,
    resolved_by_kind         TEXT,
    resolved_by_id           TEXT,
    last_event_id            TEXT NOT NULL,
    last_global_seq          BIGINT NOT NULL
);

CREATE INDEX idx_exceptions_kind ON proj.exceptions(kind);
CREATE INDEX idx_exceptions_status ON proj.exceptions(status);

COMMENT ON TABLE proj.exceptions IS 'Exception projection (Deviation/Deferral/Waiver) per SR-CONTRACT';

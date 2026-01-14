-- Migration 001: Event Store Schema
-- Per SR-SPEC §1.6.2 - Append-only event streams with optimistic concurrency
--
-- IMPORTANT: This schema is append-only. Events MUST NOT be updated or deleted.
-- Corrections MUST be represented as new events with supersedes populated.

-- Event store schema
CREATE SCHEMA IF NOT EXISTS es;

-- Streams table: tracks stream versions for optimistic concurrency
CREATE TABLE es.streams (
    stream_id        TEXT PRIMARY KEY,
    stream_kind      TEXT NOT NULL,
    stream_version   BIGINT NOT NULL DEFAULT 0,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE es.streams IS 'Stream metadata for optimistic concurrency control per SR-SPEC §1.6.2';

-- Events table: append-only event log (source of truth)
CREATE TABLE es.events (
    global_seq       BIGSERIAL PRIMARY KEY,
    event_id         TEXT NOT NULL UNIQUE,
    stream_id        TEXT NOT NULL REFERENCES es.streams(stream_id),
    stream_seq       BIGINT NOT NULL,
    occurred_at      TIMESTAMPTZ NOT NULL,
    actor_kind       TEXT NOT NULL CHECK (actor_kind IN ('HUMAN', 'AGENT', 'SYSTEM')),
    actor_id         TEXT NOT NULL,
    event_type       TEXT NOT NULL,
    correlation_id   TEXT,
    causation_id     TEXT,
    supersedes       TEXT[] DEFAULT '{}',
    refs             JSONB NOT NULL DEFAULT '[]'::JSONB,
    payload          JSONB NOT NULL,
    envelope_hash    TEXT NOT NULL,
    inserted_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Enforce unique stream sequence per stream
    UNIQUE (stream_id, stream_seq)
);

-- Indexes for common query patterns
CREATE INDEX idx_events_stream_id ON es.events(stream_id);
CREATE INDEX idx_events_event_type ON es.events(event_type);
CREATE INDEX idx_events_occurred_at ON es.events(occurred_at);
CREATE INDEX idx_events_correlation_id ON es.events(correlation_id) WHERE correlation_id IS NOT NULL;
CREATE INDEX idx_events_causation_id ON es.events(causation_id) WHERE causation_id IS NOT NULL;
CREATE INDEX idx_events_refs ON es.events USING GIN (refs);

COMMENT ON TABLE es.events IS 'Append-only event log - sole source of truth per SR-SPEC §1.5.1';
COMMENT ON COLUMN es.events.global_seq IS 'Global sequence for total ordering (PostgreSQL reference implementation)';
COMMENT ON COLUMN es.events.stream_seq IS 'Per-stream sequence for stream ordering';
COMMENT ON COLUMN es.events.supersedes IS 'Explicit correction linkage - corrections are new events';
COMMENT ON COLUMN es.events.refs IS 'Typed references for dependency and audit per SR-SPEC §1.5.3';

-- Outbox table: for NATS publication per SR-SPEC §1.6.2
CREATE TABLE es.outbox (
    outbox_id        BIGSERIAL PRIMARY KEY,
    global_seq       BIGINT NOT NULL,
    published_at     TIMESTAMPTZ,
    topic            TEXT NOT NULL,
    message          JSONB NOT NULL,
    message_hash     TEXT NOT NULL
);

-- Index for unpublished messages
CREATE INDEX idx_outbox_unpublished ON es.outbox(published_at) WHERE published_at IS NULL;

COMMENT ON TABLE es.outbox IS 'Outbox for reliable NATS event publication per SR-SPEC §1.6.2';

-- Trigger to prevent updates/deletes on events (append-only enforcement)
CREATE OR REPLACE FUNCTION es.prevent_event_modification()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'Events are append-only and cannot be modified or deleted. Use supersedes for corrections.';
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER tr_events_immutable
    BEFORE UPDATE OR DELETE ON es.events
    FOR EACH ROW
    EXECUTE FUNCTION es.prevent_event_modification();

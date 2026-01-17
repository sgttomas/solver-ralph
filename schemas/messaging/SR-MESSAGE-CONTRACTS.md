---
doc_id: SR-MESSAGE-CONTRACTS
doc_kind: specification.schema
layer: adapter
status: active
normative_status: informative

refs:
  - rel: derived_from
    to: SR-SPEC
    section: "§4.6"
  - rel: implements
    to: nats.rs
---

# SR-MESSAGE-CONTRACTS

NATS JetStream message contracts for SOLVER-Ralph orchestration messaging.

## 1. Overview

This document formalizes the message contracts used by the NATS JetStream message bus adapter. These contracts are **adapter-layer** specifications, not domain types. The message bus is an infrastructure adapter; no domain invariant depends on NATS being present (per SR-SPEC §4.6).

**Schema Version:** `1.0`

**Implementation:** `crates/sr-adapters/src/nats.rs`

---

## 2. MessageEnvelope Schema

The `MessageEnvelope` wraps domain events for transport over NATS. It is a **transport wrapper**, distinct from the `EventEnvelope` used for persistence.

### 2.1 Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema_version` | string | Yes | Schema version for forward compatibility. Current: `"1.0"` |
| `message_type` | string | Yes | Message type (matches `event_type` from domain) |
| `message_id` | string | Yes | Unique message ID (from `event_id`) |
| `correlation_id` | string | No | Correlation ID for distributed tracing |
| `causation_id` | string | No | Causation ID for event chains |
| `timestamp` | ISO 8601 datetime | Yes | Timestamp of message creation (UTC) |
| `actor_id` | string | Yes | Actor who triggered this message |
| `actor_kind` | string | Yes | Actor type: `"HUMAN"`, `"AGENT"`, or `"SYSTEM"` |
| `payload` | object | Yes | The actual event payload (JSON object) |
| `idempotency_key` | string | Yes | Message hash from outbox for deduplication |

### 2.2 Example

```json
{
  "schema_version": "1.0",
  "message_type": "LoopCreated",
  "message_id": "evt_01HQGX8K9JNPQR5MWTVYZ3F4BC",
  "correlation_id": "req_01HQGX8K9JNPQR5MWTVYZ3F4AB",
  "causation_id": null,
  "timestamp": "2024-01-15T14:30:00Z",
  "actor_id": "user_01HQGW8K9JNPQR5MWTVYZ3F4AA",
  "actor_kind": "HUMAN",
  "payload": {
    "loop_id": "loop_01HQGX8K9JNPQR5MWTVYZ3F4BD",
    "work_unit_id": "wu_01HQGX8K9JNPQR5MWTVYZ3F4AE",
    "goal": "Implement user authentication"
  },
  "idempotency_key": "sha256:abc123def456..."
}
```

---

## 3. Stream Configuration

JetStream streams organize messages by domain. Each stream captures a category of messages for durability and replay.

### 3.1 Stream Definitions

| Stream Name | Subjects | Purpose |
|-------------|----------|---------|
| `sr-events` | `sr.events.*` | Domain events for projections and reactions |
| `sr-commands` | `sr.commands.*` | Orchestration commands |
| `sr-queries` | `sr.queries.*` | Request/reply queries (rarely used) |

### 3.2 Stream Configuration

```json
{
  "name": "sr-events",
  "subjects": ["sr.events.*"],
  "max_age": 604800000000000,
  "duplicate_window": 120000000000,
  "storage": "file",
  "retention": "limits"
}
```

- **max_age:** 7 days (604800 seconds) - configurable via `NATS_MESSAGE_TTL_SECS`
- **duplicate_window:** 2 minutes (120 seconds) - configurable via `NATS_DUPLICATE_WINDOW_SECS`

---

## 4. Subject Patterns

Subjects route messages to appropriate consumers. The pattern follows: `sr.<stream>.<domain>`.

### 4.1 Event Subjects

| Subject | Domain | Example Event Types |
|---------|--------|---------------------|
| `sr.events.loop` | Loop lifecycle | `LoopCreated`, `LoopActivated`, `LoopPaused`, `LoopClosed` |
| `sr.events.iteration` | Iteration lifecycle | `IterationStarted`, `IterationCompleted` |
| `sr.events.candidate` | Candidate management | `CandidateRegistered`, `CandidatePromoted` |
| `sr.events.run` | Oracle runs | `RunStarted`, `RunCompleted` |
| `sr.events.oracle` | Oracle results | `OracleResultRecorded` |
| `sr.events.governance` | Governance actions | `GovernanceActionRecorded` |
| `sr.events.freeze` | Freeze operations | `FreezeCreated`, `FreezeLifted` |
| `sr.events.staleness` | Staleness tracking | `StalenessDetected`, `StalenessCleared` |
| `sr.events.approval` | Approval workflow | `ApprovalRequested`, `ApprovalRecorded` |
| `sr.events.exception` | Exception handling | `ExceptionRaised`, `ExceptionResolved` |
| `sr.events.decision` | Decision records | `DecisionRecorded` |
| `sr.events.worksurface` | Work Surface lifecycle | `WorkSurfaceCreated`, `WorkSurfaceArchived` |
| `sr.events.other` | Uncategorized events | Any event not matching above patterns |

### 4.2 Command Subjects

| Subject | Purpose |
|---------|---------|
| `sr.commands.iteration.start` | Request iteration start |
| `sr.commands.iteration.complete` | Request iteration completion |
| `sr.commands.candidate.register` | Request candidate registration |
| `sr.commands.oracle.run` | Request oracle execution |

---

## 5. Idempotency & Deduplication

### 5.1 Idempotency Key

Each message includes an `idempotency_key` derived from the outbox message hash. This enables:

1. **Publisher deduplication:** NATS JetStream rejects duplicate messages within the duplicate window via `Nats-Msg-Id` header
2. **Consumer deduplication:** Consumers track processed keys to skip redelivered messages

### 5.2 Consumer Handling

Consumers must implement idempotent message handling:

```
1. Receive message
2. Check idempotency_key against processed set
3. If already processed: ACK and skip
4. Process message
5. On success: Mark as processed, ACK
6. On failure: NAK for redelivery
7. On permanent failure: TERM (no redelivery)
```

---

## 6. Message Acknowledgment

JetStream requires explicit acknowledgment. The adapter supports:

| Action | Method | When to Use |
|--------|--------|-------------|
| ACK | `msg.ack()` | Message processed successfully |
| NAK | `msg.nak()` | Temporary failure, request redelivery |
| TERM | `msg.term()` | Permanent failure, do not redeliver |

---

## 7. Schema Evolution

### 7.1 Versioning Strategy

The `schema_version` field enables forward compatibility:

- **Minor changes** (adding optional fields): Keep version, consumers ignore unknown fields
- **Breaking changes** (required field changes, type changes): Increment version

### 7.2 Migration Path

When `schema_version` changes:

1. Deploy consumers that support both old and new versions
2. Deploy producers emitting new version
3. After duplicate window expires, remove old version support

### 7.3 Compatibility Rules

Producers MUST:
- Always include `schema_version`
- Only emit supported schema versions

Consumers MUST:
- Check `schema_version` before processing
- Ignore unknown fields (forward compatibility)
- TERM messages with unsupported schema versions

---

## 8. Configuration

Environment variables for NATS adapter configuration:

| Variable | Default | Description |
|----------|---------|-------------|
| `NATS_URL` | `nats://localhost:4222` | NATS server URL |
| `NATS_STREAM_PREFIX` | `sr` | Prefix for stream names |
| `NATS_CONSUMER_PREFIX` | `sr-consumer` | Prefix for consumer names |
| `NATS_MESSAGE_TTL_SECS` | `604800` (7 days) | Message retention period |
| `NATS_MAX_MSGS_PER_SUBJECT` | `-1` (unlimited) | Max messages per subject |
| `NATS_DUPLICATE_WINDOW_SECS` | `120` (2 minutes) | Duplicate detection window |

---

## 9. References

- SR-SPEC §4.6: Message bus pattern
- SR-SPEC §1.5.2: Event envelope (persistence layer)
- `crates/sr-adapters/src/nats.rs`: Implementation
- `schemas/messaging/message-envelope.schema.json`: JSON Schema for validation

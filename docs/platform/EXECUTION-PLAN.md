# SOLVER-Ralph Implementation Execution Plan

## Overview

This document provides a high-level plan for implementing the SOLVER-Ralph platform based on the governing specification documents in this folder. SOLVER-Ralph is a **governance-first, event-sourced, evidence-backed platform** for controlled agentic work, with a primary focus on **Semantic Ralph Loops** — bounded workflow instances for knowledge work that enforce human authority at trust boundaries.

---

## Document Hierarchy (Binding Precedence)

Understanding the document precedence is critical before implementation:

1. **SR-CONTRACT** — Binding invariants (what MUST always be true)
2. **SR-SPEC** — Binding mechanics (how invariants are enforced)
3. **SR-TYPES** — Binding vocabulary and type registry
4. **SR-WORK-SURFACE** — Work surface schemas (Intake, Procedure Template, Work Surface Instance)
5. **SR-SEMANTIC-ORACLE-SPEC** — Semantic oracle interfaces
6. **SR-EVENT-MANAGER** — State/eligibility projection mechanics
7. **SR-PROCEDURE-KIT** — Reusable procedure templates
8. **SR-AGENT-WORKER-CONTRACT** — Agent-platform interface
9. **SR-INTENT** — Design rationale (non-binding)

---

## Phase 1: Core Infrastructure (Foundation)

### 1.1 Event Store Layer
**Objective:** Implement the append-only event store as the single source of truth.

**Key deliverables:**
- PostgreSQL schema implementation (`es.streams`, `es.events`, `es.outbox`)
- Event envelope schema (v1) with all required fields
- Optimistic concurrency control for event append
- Stream management (LOOP, ITERATION, CANDIDATE, RUN, APPROVAL, etc.)

**Contract alignment:** C-ARCH-3, C-EVT-1, C-EVT-2, C-EVT-4

---

### 1.2 Identity Model
**Objective:** Implement stable, auditable actor identity.

**Key deliverables:**
- ULID-based identifier generation (`loop_<ULID>`, `iter_<ULID>`, etc.)
- Actor identity formats (HUMAN, AGENT, SYSTEM)
- OIDC integration with Zitadel for human identity
- Content-addressed hash computation (SHA-256)

**Contract alignment:** C-TB-5, C-EVT-1

---

### 1.3 Projection Infrastructure
**Objective:** Build rebuildable projections from the event stream.

**Key deliverables:**
- Projection schema (`proj.*` tables)
- Synchronous projection mechanism (v1)
- Projection rebuild tooling
- `proj.governed_artifacts` table for governed artifact registry

**Contract alignment:** C-EVT-7, C-ARCH-3

---

## Phase 2: Domain Core (Hexagonal Architecture)

### 2.1 Domain Core Module
**Objective:** Implement pure domain logic with no infrastructure dependencies.

**Key deliverables:**
- Command handlers as pure functions: `(state, command) → Result<Vec<Event>, DomainError>`
- State derivation via event folding: `apply(state, event) → state`
- Domain ports (traits) for EventStore, EvidenceStore, OracleRunner, MessageBus, IdentityProvider, Clock
- Invariant enforcement in command validation

**Contract alignment:** C-ARCH-1, C-ARCH-2

---

### 2.2 Ralph Loop State Machine
**Objective:** Implement the Loop lifecycle.

**States:** `CREATED → ACTIVE → PAUSED → CLOSED`

**Key deliverables:**
- Loop creation and activation
- Iteration start (SYSTEM-only)
- Iteration completion with summary recording
- Stop-trigger evaluation and PAUSED state transition
- Decision-based resumption

**Events:** `LoopCreated`, `LoopActivated`, `IterationStarted`, `IterationCompleted`, `StopTriggered`, `LoopPaused`, `LoopResumed`, `LoopClosed`

**Contract alignment:** C-LOOP-1, C-LOOP-2, C-LOOP-3

---

### 2.3 Iteration Context Provenance
**Objective:** Enforce fresh-context iterations with no ghost inputs.

**Key deliverables:**
- `IterationStarted.refs[]` as authoritative context (C-CTX-1)
- ContextCompiler implementation (deterministic)
- Iteration Summary schema enforcement
- Memory discipline (controlled carry-forward via typed refs only)

**Required ref categories:**
- Loop, Governing artifacts in force, Prior iteration summaries
- Base Candidate, Oracle suite, Active exceptions
- Human intervention notes, Agent definition, Gating policy
- Intake, Procedure Template (with current stage)

**Contract alignment:** C-CTX-1, C-CTX-2

---

## Phase 3: Work Surface and Stage-Gated Procedures

### 3.1 Intake Schema Implementation
**Objective:** Implement structured work unit intake.

**Key deliverables:**
- Intake schema (v1) with required fields
- Content-addressed storage for Intake artifacts
- `record.intake` type registration

**Required fields:** `work_unit_id`, `title`, `kind`, `objective`, `audience`, `deliverables[]`, `constraints[]`, `definitions{}`, `inputs[]`, `unknowns[]`, `completion_criteria[]`

---

### 3.2 Procedure Template Implementation
**Objective:** Implement stage-gated procedure definitions.

**Key deliverables:**
- Procedure Template schema (v1)
- Stage definitions with required outputs and oracle suites
- Gate rule evaluation logic
- `config.procedure_template` type registration
- GENERIC-KNOWLEDGE-WORK baseline template

**Stages (baseline):** `FRAME → OPTIONS → DRAFT → SEMANTIC_EVAL → FINAL`

---

### 3.3 Work Surface Binding
**Objective:** Implement Work Surface Instance as iteration context.

**Key deliverables:**
- Work Surface Instance schema
- `WorkSurfaceRecorded` event
- Binding to `IterationStarted.refs[]`
- Stage progression tracking

**Contract alignment:** C-CTX-1, C-CTX-2

---

## Phase 4: Candidate and Evidence Model

### 4.1 Candidate Management
**Objective:** Implement content-addressable candidate snapshots.

**Key deliverables:**
- Candidate identity computation (git + sha256)
- Candidate Manifest for non-VCS candidates
- `CandidateMaterialized` event
- Candidate projection table

**Contract alignment:** C-LOOP-4, C-VER-1

---

### 4.2 Evidence Bundle Model
**Objective:** Implement immutable, content-addressed evidence storage.

**Key deliverables:**
- Evidence Bundle manifest schema (v1)
- MinIO integration for evidence storage (content-addressed keys)
- `EvidenceBundleRecorded` event
- Evidence integrity checks (EVIDENCE_MISSING detection)
- Restricted store for evidence containing secrets

**Contract alignment:** C-EVID-1 through C-EVID-6

---

### 4.3 Verification Status Computation
**Objective:** Implement Verified (Strict) and Verified-with-Exceptions logic.

**Key deliverables:**
- Verification predicate evaluation
- Oracle result aggregation
- Waiver coverage check
- `CandidateVerificationComputed` event

**Contract alignment:** C-VER-1 through C-VER-4

---

## Phase 5: Oracle System

### 5.1 Oracle Suite Management
**Objective:** Implement oracle suite pinning and integrity.

**Key deliverables:**
- Oracle suite registration and versioning
- Suite hash computation (including semantic set definitions)
- Suite pinning at run start
- Integrity condition detection (ORACLE_TAMPER, ORACLE_GAP, ORACLE_ENV_MISMATCH, ORACLE_FLAKE)

**Contract alignment:** C-OR-1 through C-OR-7

---

### 5.2 Oracle Runner (Podman + gVisor)
**Objective:** Implement sandboxed oracle execution.

**Key deliverables:**
- OCI container execution with gVisor runtime
- Environment fingerprint recording
- Read-only candidate mount, write-only output volume
- `RunStarted` / `RunCompleted` events

---

### 5.3 Semantic Oracle Integration
**Objective:** Implement semantic oracle support for knowledge work.

**Key deliverables:**
- Semantic set binding via suite hash
- `sr.semantic_eval.v1` result schema support
- Structured measurement artifacts (residual, coverage, violations)
- Stage-specific semantic evaluation

---

## Phase 6: Portal and Approval System

### 6.1 Portal Infrastructure
**Objective:** Implement human authority boundaries.

**Required portals:**
- **Governance Change Portal** — for normative governance changes
- **Release Approval Portal** — for declaring Shippable

**Key deliverables:**
- Portal definition configuration
- Approval record schema (C-TB-6)
- `ApprovalRecorded` event
- Authorization enforcement (HUMAN actor only)

**Contract alignment:** C-TB-1 through C-TB-7

---

### 6.2 Exception System
**Objective:** Implement Deviation, Deferral, and Gate Waiver records.

**Key deliverables:**
- Exception schemas with required fields
- Waiver scope constraints (per-candidate, per-loop, per-baseline, time-boxed)
- Active exception tracking
- `WaiverCreated`, `ExceptionActivated`, `ExceptionResolved` events

**Contract alignment:** C-EXC-1 through C-EXC-5

---

### 6.3 Decision Records
**Objective:** Implement binding human decision recording.

**Key deliverables:**
- Decision record schema
- `DecisionRecorded` event
- Linkage to stop triggers and affected entities

**Contract alignment:** C-DEC-1

---

## Phase 7: Freeze and Shippable

### 7.1 Freeze Record
**Objective:** Implement baseline snapshot for release.

**Key deliverables:**
- Freeze Record schema
- `FreezeRecordCreated` event
- Artifact manifest enumeration
- Active exception acknowledgement check

**Contract alignment:** C-SHIP-1

---

### 7.2 Shippable Determination
**Objective:** Implement Shippable status computation.

**Shippable requires:**
1. Verified (Strict or With-Exceptions)
2. Release Approval at Portal
3. Freeze Record complete
4. No unresolved staleness markers

**Key deliverables:**
- Shippable status projection
- Blocking reasons enumeration
- `proj.shippable_status` table

**Contract alignment:** C-SHIP-1

---

## Phase 8: Dependency Graph and Staleness

### 8.1 Graph Model
**Objective:** Implement dependency graph for audit and staleness.

**Key deliverables:**
- `graph.nodes` and `graph.edges` tables
- Node types (GovernedArtifact, Candidate, EvidenceBundle, etc.)
- Edge types (depends_on, produces, verifies, approved_by, etc.)
- Recursive dependency queries

---

### 8.2 Staleness Marking
**Objective:** Implement staleness detection and propagation.

**Key deliverables:**
- `graph.stale_nodes` table
- `NodeMarkedStale` event
- Staleness propagation via dependency traversal
- `StalenessResolved` event
- Re-evaluation triggering

**Contract alignment:** C-EVT-6

---

## Phase 9: Event Manager / Projections

### 9.1 Status Projection
**Objective:** Implement deterministic work unit status computation.

**Key deliverables:**
- `status_by_work_unit` projection
- Stage status tracking
- Block reason computation

---

### 9.2 Eligibility Projection
**Objective:** Implement eligible set computation.

**Eligibility predicate:**
- Not COMPLETE
- Not BLOCKED by stop trigger requiring portal relief
- All `depends_on` prerequisites COMPLETE (or deferred)
- Valid Work Surface binding (for semantic work)

**Key deliverables:**
- `eligible_set` projection
- Dependency graph snapshot

---

## Phase 10: API Layer

### 10.1 HTTP API (v1)
**Objective:** Implement RESTful API under `/api/v1`.

**Endpoint groups:**
- Loops: creation, iteration management, completion
- Candidates: registration, status query
- Runs/Evidence: oracle run triggering, evidence retrieval
- Approvals: portal crossing, decision recording
- Exceptions: waiver creation, resolution
- Governed Artifacts: registration, versioning
- Decisions: binding decision recording
- Freeze Records: baseline creation
- Staleness: marking, resolution

---

### 10.2 Authentication/Authorization
**Objective:** Implement OIDC-based auth with trust boundary enforcement.

**Key deliverables:**
- JWT validation via Zitadel
- Actor kind derivation from token
- Trust boundary enforcement (HUMAN-only actions)
- Authorization event recording

**Contract alignment:** C-TB-1, C-TB-5

---

## Phase 11: Agent Worker Integration

### 11.1 Worker Integration Pattern
**Objective:** Implement agent worker integration per SR-AGENT-WORKER-CONTRACT.

**Key deliverables:**
- Worker event consumption (`IterationStarted`)
- Context compilation from refs
- Candidate registration flow
- Evidence bundle submission
- Iteration summary recording

---

### 11.2 Human Judgment Hooks
**Objective:** Implement gating policy hooks (soft/hard/hybrid).

**Hook classes:**
- `plan_review` — non-binding plan review
- `evaluation_on_verification` — human evaluation of verification evidence
- `assessment_on_validation` — human assessment of validation evidence
- `closeout` — final closeout via Portal approvals

---

## Phase 12: Operational Infrastructure

### 12.1 Message Bus (NATS)
**Objective:** Implement event publication for subscribers.

**Key deliverables:**
- Outbox publisher
- NATS subject structure (`sr.events.<stream_kind>.<event_type>`)
- JetStream durable consumers (optional)

---

### 12.2 Secrets Management
**Objective:** Implement secure secrets handling via Infisical.

**Key deliverables:**
- Envelope encryption for restricted evidence
- Data key management
- Redaction manifest support

---

## Risk Considerations

1. **Complexity:** The specification is comprehensive; prioritize core event sourcing and Loop lifecycle first.
2. **Semantic Oracle Implementation:** The platform defines interfaces, not oracle implementations — semantic oracle development is a parallel workstream.
3. **Human Authority Enforcement:** Trust boundaries are critical; automated tests for HUMAN-only constraints are essential.
4. **Staleness Propagation:** Graph traversal at scale requires performance testing.
5. **Evidence Availability:** Content-addressed storage must ensure long-term retrievability.

---

## Recommended Implementation Order

1. **Foundation:** Event store, identity model, projection infrastructure
2. **Core Domain:** State machines (Loop, Iteration, Candidate)
3. **Trust Boundaries:** Portal system, approval records
4. **Evidence System:** Oracle runner, evidence bundles
5. **Work Surface:** Intake, Procedure Template, stage-gating
6. **Graph/Staleness:** Dependency tracking, re-evaluation
7. **API Layer:** HTTP endpoints with auth
8. **Agent Integration:** Worker patterns, hooks
9. **Operational:** NATS, secrets management

---

## Success Criteria

The implementation is complete when:

- All C-* contract invariants from SR-CONTRACT are enforced
- Event store is append-only and rebuildable
- Trust boundaries (Portals) require HUMAN actors
- Candidates can be Verified → Approved → Frozen → Shippable
- Evidence bundles are immutable and retrievable
- Work Surfaces enable stage-gated semantic work
- Agent workers operate within controlled iteration boundaries

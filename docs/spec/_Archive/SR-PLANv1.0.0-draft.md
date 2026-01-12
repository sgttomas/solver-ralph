---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-PLAN"
  type: "work.implementation_plan"
  title: "SOLVER-Ralph Project Implementation Plan — MVP (First Working Release)"
  version: "1.0.0"
  status: "draft"
  normative_status: "directional"
  authority_kind: "process"
  governed_by:
    - "SR-CHANGE"
  created: "2026-01-10"
  updated: "2026-01-10"
  supersedes: []
  tags:
    - "solver-ralph"
    - "implementation-plan"
    - "mvp"
    - "operative"
    - "event-sourced"
  ext:
    declared_stack:
      source:
        artifact: "SR-SPEC"
        file: "SOLVER-Ralph_Tech_Spec_SR-SPEC_v1.2.1.md"
        note: "Extracted verbatim from SR-SPEC YAML frontmatter: ext.technical_spec.stack"
      items:
        - "domain_core: Rust (pure domain; ports + state machines)"
        - "api_layer: Rust + Axum (HTTP API + auth boundary)"
        - "adapters_cli: TypeScript (operator tooling + optional CLI)"
        - "event_store: PostgreSQL (append-only + per-stream sequencing + outbox)"
        - "graph_store: PostgreSQL (graph_node/graph_edge + recursive CTEs)"
        - "content_storage: MinIO (S3) (content-addressed, immutable buckets)"
        - "oracle_runtime: Podman + gVisor (runsc) (rootless where possible)"
        - "orchestration: Custom loop governor + NATS (Core + JetStream)"
        - "identity: Zitadel (OIDC)"
        - "secrets: Infisical (restricted evidence envelope keys)"
        - "ui: React + TypeScript"
    target_mvp_scope:
      - "End-to-end Ralph Loop for a single work_unit: LoopCreated → IterationStarted (SYSTEM) → Candidate registered → Run(s) requested → Evidence bundle recorded → Iteration summary submitted → Human portal approval → Freeze record created → Shippable status visible"
      - "Core SR-SPEC API endpoints required by that flow"
      - "Content-addressed evidence storage + manifest"
      - "Basic projections: loops/iterations/candidates/runs/approvals/freeze/shippable + graph refs"
      - "One reference Agent Worker integration (adapter) consuming IterationStarted"
    assumptions:
      - "All referenced governed artifacts (SR-INTENT/SR-CONTRACT/SR-SPEC/SR-TYPES/SR-DIRECTIVE/SR-CHANGE/README) are dereferenceable artifacts with stable ids + versions + content hashes."
      - "Infrastructure services (PostgreSQL, NATS, MinIO, Zitadel, Infisical) are available in dev via compose and in prod via managed/self-hosted equivalents."
      - "Candidates are produced from a Git repository that the system and/or oracle runner can access read-only for hash verification and reruns."
---

# SOLVER-Ralph Project Implementation Plan — MVP (First Working Release)

**Document classification:** Supporting controlled document (operative guidance).  
**Non-goal:** This plan does **not** define new governance rules, lifecycle states, or portal semantics.

> **Type note:** SR-TYPES does not appear to define an explicit artifact type for “implementation plan.”
> This document uses a proposed supporting type key `work.implementation_plan` to keep it clearly non-binding and non-governance.
> If you later want this plan to be registered as a governed artifact type, add an appropriate type key to SR-TYPES via SR-CHANGE.

---

## Table of Contents

- [1. Stack extraction (verbatim list)](#1-stack-extraction-verbatim-list)
- [2. Architecture decomposition](#2-architecture-decomposition)
- [3. Milestone plan (phased)](#3-milestone-plan-phased)
- [4. API + event flow checklist](#4-api--event-flow-checklist)
- [5. Data model + storage plan](#5-data-model--storage-plan)
- [6. Oracle plan](#6-oracle-plan)
- [7. Worker integration contract](#7-worker-integration-contract)
- [8. Governance compliance map](#8-governance-compliance-map)
- [9. Out of scope](#9-out-of-scope)
- [10. Normative section (minimal)](#10-normative-section-minimal)
- [11. Change control](#11-change-control)

---

## 1. Stack extraction (verbatim list)

### 1.1 Declared stack (verbatim list)

```yaml
ext:
  technical_spec:
    stack:
      domain_core:
        language: Rust
        notes: "Pure domain (ports + state machines)"
      api_layer:
        language: Rust
        framework: Axum
        notes: "HTTP API + auth boundary"
      adapters_cli:
        language: TypeScript
        notes: "Operator tooling + optional CLI"
      event_store:
        system: PostgreSQL
        pattern: "append-only + per-stream sequencing + outbox"
      graph_store:
        system: PostgreSQL
        method: "graph_node/graph_edge + recursive CTEs"
      content_storage:
        system: "MinIO (S3)"
        method: "content-addressed, immutable buckets"
      oracle_runtime:
        system: "Podman"
        sandbox: "gVisor (runsc)"
        notes: "rootless where possible"
      orchestration:
        system: "Custom loop governor"
        messaging: "NATS (Core + JetStream)"
      identity:
        system: "Zitadel"
        protocol: "OIDC"
      secrets:
        system: "Infisical"
        role: "secrets + envelope keys for restricted evidence"
      ui:
        system: "React"
        language: "TypeScript"
```

_Source: SR-SPEC YAML frontmatter: `ext.technical_spec.stack`._

### 1.2 Stack-to-plan mapping

- **Domain:** Rust domain core — event-sourced state machines, invariants, verification computation.
- **API:** Rust + Axum — SR-SPEC endpoints, auth boundary, command handlers.
- **Persistence:** Postgres append-only event log + outbox.
- **Graph/projections:** Postgres graph tables + staleness queries.
- **Object store:** MinIO/S3 content-addressed evidence blobs + manifests.
- **Sandbox:** Podman + gVisor/runsc oracle runner.
- **Messaging:** NATS Core + JetStream distribution.
- **Identity:** Zitadel OIDC.
- **Secrets:** Infisical.
- **UI:** React + TS portal.

---

## 2. Architecture decomposition

### 2.1 Implementable components

#### In-domain (domain core; infra-free)

1) **Domain Core (Rust library)**
- Event and command types (Loop, Iteration, Candidate, Run, Approval, Freeze, Artifacts, Exceptions).
- Fold/apply logic per stream_kind.
- Verification computation (STRICT mode for MVP).
- Stop-trigger evaluation primitives.

> Constraint alignment: domain core MUST NOT include infrastructure clients or LLM SDKs.

#### Out-of-domain (hex adapters, services, orchestration)

2) **API Service (Rust + Axum)**
- SR-SPEC endpoints.
- Auth boundary (human/system/worker).
- Command handlers (validate, call domain, persist events).

3) **Event Store Adapter (PostgreSQL)**
- `es.events` append-only.
- `es.outbox` for reliable publication to NATS.

4) **Outbox Publisher**
- Publishes outbox rows to NATS (Core/JetStream), idempotently.

5) **Projection Builder / Graph Projector**
- Rebuildable projections from event log.
- Graph nodes/edges; staleness traversal uses dependency edges only.

6) **Object Store Adapter (MinIO/S3)**
- Content-addressed evidence and manifests.
- Restricted evidence support via envelope encryption keys (Infisical).

7) **Oracle Runner Service (Podman + gVisor/runsc)**
- Executes oracle suite containers against candidate checkout.
- Emits evidence bundles.

8) **Loop Governor / SYSTEM Control Plane**
- Creates loops per work_unit.
- Assembles Iteration Context Ref Set.
- Starts iterations (SYSTEM-only).

9) **Agent Worker Integration Adapter (TypeScript)**
- Consumes IterationStarted from NATS.
- Compiles context deterministically.
- Produces candidate, requests runs, submits summary.

10) **Portal UI (React + TypeScript)**
- Human actions: inspect, approve, freeze, closeout.

11) **Identity integration (Zitadel OIDC)**
- JWT validation and actor mapping.

12) **Secrets integration (Infisical)**
- Restricted evidence envelope keys + service secrets.

### 2.2 In-domain vs out-of-domain boundary

- **In-domain:** component (1) only.
- **Out-of-domain:** components (2–12).

---

## 3. Milestone plan (phased)

### Milestone 1 — Bootstrap + local runtime

**Scope**
- Repo layout: `domain`, `api`, `projections`, `outbox-publisher`, `oracle-runner`, `loop-governor`, `worker`, `ui`.
- Compose stack: Postgres, NATS, MinIO, Zitadel, Infisical (or stub for dev).

**Acceptance checks**
- `docker compose up` starts dependencies.
- API health endpoint returns OK.
- SYSTEM token works for SYSTEM endpoints; HUMAN token works for portal endpoints.

**Implementation notes**
- Provide dev-only auth stub with strict role simulation.

**Risks / mitigations**
- Risk: OIDC integration delays.
  - Mitigation: keep stub for dev but ensure OIDC path is primary.

**Governance artifacts touched**
- None.

---

### Milestone 2 — Domain core MVP

**Scope**
- Implement domain events/commands for:
  - Loop create
  - Iteration start/summary
  - Candidate registration
  - Run lifecycle
  - Evidence bundle record
  - Approval record
  - Freeze record
- Enforce invariants: SYSTEM-only iteration start; lifecycle correctness.

**Acceptance checks**
- Unit tests for transitions, invalid actions, fold/apply determinism.
- Verification computation unit tests (synthetic evidence bundle).

**Implementation notes**
- Keep ports minimal (clock/id/policy provider).
- No DB/NATS/HTTP in domain.

**Risks / mitigations**
- Risk: overly broad modeling.
  - Mitigation: MVP-only event set; evolve with SR-CHANGE.

**Governance artifacts touched**
- None.

---

### Milestone 3 — Event store + outbox + projections + graph

**Scope**
- Implement event append + outbox in one transaction.
- Implement projector and graph builder.
- Implement rebuild-from-log.

**Acceptance checks**
- Write event → outbox row → NATS publish → projector updates.
- Rebuild projections from scratch matches prior state.

**Implementation notes**
- Store refs[] in event envelope and project into graph edges with rel.

**Risks / mitigations**
- Risk: projection drift.
  - Mitigation: “projection invariants oracle” integration test.

**Governance artifacts touched**
- None.

---

### Milestone 4 — APIs + SYSTEM loop governor + worker stub + oracle runner

**Scope**
- SR-SPEC endpoints for the end-to-end flow:
  - loops, iteration start/summary, candidates, runs, approvals, freezes, shippable status
- Loop governor: ref assembly + iteration start (SYSTEM-only).
- Worker: consumes IterationStarted and produces a trivial candidate + runs + summary.
- Oracle runner: executes `SR-SUITE-CORE` in sandbox; emits evidence manifest to MinIO.

**Acceptance checks**
- End-to-end smoke test:
  - create loop → start iteration → candidate → run → evidence → summary → approval → freeze → shippable computed.

**Implementation notes**
- Strict schema validation to prevent ghost inputs.
- Candidate identity verification best-effort.

**Risks / mitigations**
- Risk: sandbox mismatch.
  - Mitigation: store env fingerprint and require match in evidence.

**Governance artifacts touched**
- Add config artifacts (registered via registry):
  - `config.oracle_suite` for `SR-SUITE-CORE`
  - `config.gating_policy` for MVP work_unit

---

### Milestone 5 — Portal UI closeout + freeze + shippable

**Scope**
- UI flows:
  - list loops/iterations/candidates/runs
  - evidence viewing + manifest
  - record ReleaseApprovalPortal approvals
  - create freeze record
  - show shippable status and reasons

**Acceptance checks**
- Human can close out a candidate via approval + freeze.
- UI labels plan review as non-binding and does not introduce “Approval” semantics.

**Implementation notes**
- UI language must preserve semantics:
  - plan review ≠ approval; worker judgement ≠ verified.

**Risks / mitigations**
- Risk: accidental semantics drift in UI.
  - Mitigation: snapshot tests + lint for forbidden labels.

**Governance artifacts touched**
- None.

---

## 4. API + event flow checklist

### 4.1 Happy path

1) **Create Loop**
- API: `POST /loops`
- Event: `LoopCreated`

2) **SYSTEM starts Iteration**
- API: `POST /loops/{loop_id}/iterations/start` (SYSTEM-only)
- Event: `IterationStarted` (refs[] is authoritative provenance)
- NATS: publish IterationStarted to worker subject

3) **Worker compiles context**
- Deterministic compilation from refs + payload only.

4) **Worker registers candidate**
- API: `POST /candidates`
- Event: `CandidateMaterialized`

5) **Worker requests run**
- API: `POST /runs`
- Events: `RunRequested` → `RunStarted` → `RunCompleted`
- Evidence: `EvidenceBundleRecorded`

6) **Worker submits iteration summary**
- API: `POST /loops/{loop_id}/iterations/{iteration_id}/summary`
- Event: `IterationSummarySubmitted`

7) **Human portal approval**
- API: `POST /approvals` (human-only)
- Event: `ApprovalRecorded`

8) **Human freeze record**
- API: `POST /freeze-records` (human-only)
- Event: `FreezeRecordCreated`

9) **Shippable status visible**
- API: `GET /shippable-status/{candidate_id}`

---

### 4.2 Failure path A — Missing refs / ghost input attempt

**A1 Missing required ref categories**
- Iteration start request missing required ref categories.
- Expected:
  - 422 error lists missing categories.
  - No IterationStarted event emitted.

**A2 Ghost input attempt**
- Caller includes extra unmodeled input fields.
- Expected:
  - Strict JSON schema rejects unknown fields.
  - Only refs[] governs inputs.

---

### 4.3 Failure path B — Candidate hash mismatch

- Candidate registered with declared sha256 that does not match canonical digest.
- Expected:
  - If system can compute digest: reject registration or flag contract violation and pause.
  - If system cannot compute: accept but require verification oracle prior to Verified.

---

### 4.4 Failure path C — Gating/human judgment hook blocking

- Human attempts freeze without required closeout hook artifacts.
- Expected:
  - 422 error explaining missing hook artifacts.
  - UI shows “blocking: closeout hook”.

---

## 5. Data model + storage plan

### 5.1 Event store (PostgreSQL)

- `es.events`:
  - event_id, stream_id, stream_kind, stream_seq, occurred_at
  - actor_kind, actor_id, event_type
  - payload (JSONB)
  - refs (JSONB)
  - envelope_hash

- `es.outbox`:
  - outbox_id, event_id, subject, payload, created_at, published_at

### 5.2 Projections

- `proj.loops`, `proj.iterations`, `proj.candidates`, `proj.runs`
- `proj.evidence_bundles`, `proj.approvals`, `proj.freezes`
- `proj.shippable_status`

All projections MUST be rebuildable from event log only.

### 5.3 Graph projection

- `graph.nodes(node_id, node_kind, stable_id, version, content_hash, created_at, last_seen_event)`
- `graph.edges(from_node_id, to_node_id, edge_type, created_at, last_seen_event)`

Edge types include `depends_on` and `supported_by` explicitly.

### 5.4 Object storage addressing (MinIO)

Buckets:
- `evidence-public`
- `evidence-restricted`

Keys:
- `evidence/<bundle_sha256>/manifest.json`
- `evidence/<bundle_sha256>/objects/<object_sha256>`

### 5.5 Candidate identity

- Candidate identity is:
  - `git:<sha>`
  - `sha256:<hash>`

Stored fields:
- candidate_id
- git_sha
- content_sha256
- declared_hashes[]
- verified_hashes[] (optional)

### 5.6 Evidence bundle manifest (minimal)

Fields:
- bundle_sha256
- candidate_id, run_id
- suite_id, suite_hash
- env_fingerprint
- objects[]: sha256, key, media_type
- oracle_results[]: oracle_id, status, outputs[]

---

## 6. Oracle plan

### 6.1 Oracle suite: SR-SUITE-CORE (MVP)

Oracles:
1) **Build + Unit Tests**
2) **Lint (clippy)**
3) **Boundary Lint (domain core infra-free)**
4) **Artifact Metadata Validation (YAML frontmatter schema)**

### 6.2 Evidence emitted

- per-oracle logs (stdout/stderr)
- normalized result JSON
- optional reports (junit/xml)

All stored content-addressed; manifest links everything.

### 6.3 Sandbox execution

- Podman containers under gVisor/runsc.
- Record environment fingerprint and image digests.

---

## 7. Worker integration contract

### 7.1 Worker responsibilities

- Subscribe to IterationStarted.
- Dereference `IterationStarted.refs[]` and compile context deterministically.
- Produce candidate, compute sha256.
- Register candidate and request oracle runs.
- Submit iteration summary.

### 7.2 Forbidden actions

- Emit IterationStarted.
- Record approvals.
- Create freeze records.
- Mislabel non-binding review as approval or verified.

### 7.3 Summary schema expectations

- IterationSummary contains:
  - candidate references
  - evidence references
  - optional `ext.non_binding_reviews[]` (never “Approval”).

---

## 8. Governance compliance map

| Constraint | Component | Test / Oracle | Evidence |
|---|---|---|---|
| SYSTEM-only IterationStarted | API + Loop Governor | Auth + domain invariant tests | Event log actor_kind=SYSTEM |
| Provenance only from IterationStarted.refs[] | Governor + API schema + Worker | Schema rejection tests; ref completeness tests | IterationStarted.refs[] + context digest |
| Deterministic context from refs + payload | Worker ContextCompiler | Determinism test | Context digest |
| Plan review non-binding | UI + summary schema | Snapshot tests; lint checks | IterationSummary ext.non_binding_reviews |
| One loop per work unit | Loop Governor | Integration tests | LoopCreated(work_unit) |
| Orchestration outside domain | Repo boundary + oracle | BoundaryLintOracle | Oracle evidence |
| Candidate identity git+sha256 | API + oracle | Hash verification test | Candidate record + digest evidence |
| Dependency vs audit semantics distinct | Projector | GraphProjectionOracle | graph.edges with rel |
| Staleness traversal uses dependency edges only | Projector | Traversal oracle | staleness query evidence |
| Human-only approvals + freeze | API auth + UI | Auth tests | ApprovalRecorded + FreezeRecordCreated |

---

## 9. Out of scope

- Deterministic scheduler / assignment plane (advanced multi-worker coordination).
- Advanced exception lifecycle UI.
- Full oracle suite catalog beyond `SR-SUITE-CORE`.
- Cryptographic signing of events (beyond envelope_hash).
- Fully automated staleness-triggered rerun routing.

---

## 10. Normative section (minimal)

This section uses normative keywords intentionally and minimally.

### 10.1 Normative requirements

- The domain core **MUST** remain infrastructure-free (no DB, NATS, object store, or LLM SDK dependencies).
- The SYSTEM control plane **MUST** be the only component able to initiate `IterationStarted`.
- The Iteration context provenance **MUST** be restricted to `IterationStarted.refs[]`; APIs and workers **MUST NOT** rely on hidden transcript memory by default.
- Projections **MUST** be rebuildable from the event log alone.

---

## 11. Change control

This plan is a **supporting controlled document**. Changes to this plan (scope, milestones, assumptions, or design decisions that affect behavior) must follow **SR-CHANGE** to preserve auditability and prevent drift between implemented behavior and documented intent.

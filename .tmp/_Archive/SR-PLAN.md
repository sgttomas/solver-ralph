---
doc_id: SR-PLAN
doc_kind: governance.plan
layer: build
status: draft

refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: depends_on
    to: SR-CONTRACT
  - rel: depends_on
    to: SR-SPEC
  - rel: depends_on
    to: SR-TYPES
  - rel: depends_on
    to: SR-GUIDE
  - rel: depends_on
    to: SR-INTENT
  - rel: depends_on
    to: SR-AGENTS
  - rel: depends_on
    to: SR-DIRECTIVE
  - rel: informs
    to: SR-MODEL
---
# SR-PLAN — SOLVER-Ralph build ): Tightens SR-PLAN’s non-authority posture by:
  - clarifying that “portal touchpoints” are conditional; any binding decision attaches to the resulting change record, not to this plan,

- **): Consolidates prior build plan drafts into a single decomposition aligned with:
  - SR-PLAN template,
  - SR-SPEC (event-sourced runtime + stack constraints),
  - SR-CONTRACT (authority, evidence, and governance constraints),
  - the requirement that SR-PLAN remains **abstract** (SR-DIRECTIVE will define execution sequencing and closure).

---

## 1. Authoritative Planning Inputs

This plan depends on the following authoritative commitment objects and governed artifacts:

| ID | type_key | meta | content_hash (sha256) |
|---|---|---|---|
| SR-CONTRACT | governance.contract |  | f5b42fa4e9e162e63fcdc33366499b98f6a5f1fd2ca9c1b0b5d597776d55eaef |
| SR-SPEC | governance.spec |  | a3c45a21e88ea41f9915cbce456986e059f2a0d10e1af91f9f1908513fb59aec |
| SR-DIRECTIVE | governance.directive |  | 6ee1f4130d193e1fefd020f5eefa8628bb117cbec04b3d619fb1b879b3551eba |
| SR-TYPES | governance.types |  | ba60d69e15be9535bee32dd432192e87ec625712c1c860b3fdedc0f2b0da319c |
| SR-GUIDE | governance.development_paradigm |  | 27c4561bd883c72bba62453781abc898b5cd3f47b275810c394a9f6e5433abb1 |
| SR-INTENT | governance.intent |  | 7940bafceb5dda19e70744ccfd58304027b10f0376186d3e87c3dcce79e85d35 |
| SR-CHANGE | governance.change |  | c0f631225ac7f0c8f348c3c85b522d50bab3793cfcab7b531c8bef98e075d580 |
| SR-README | governance.readme |  | 19caba82029848f6e8e74184060e117cdaff4229e986f4a5f55fcb672ffce2ec |
| SR-CONTRACT/SR-SPEC trust boundaries | governance.trust_topology |  | 32844e0e71268b27586a00cdbb38d71cc4c868d2c31867c902a5ae02fc4a6b2b |
| SR-AGENTS | policy.agent_taxonomy |  | f2a303ad5fead512ed8250a3eee6db1c2de0b6851b302119024373594cd3ddd9 |
| SR-PLAN | governance.plan |  | 1607f32a0a50345ffc0e2ca841e3a4437dc8e94709c9bf68784ef64c50a25eab |


---

## 2. Orientation Summary (Non-Authoritative)

This instance plan decomposes the build of **SOLVER-Ralph build** into deliverables that collectively realize:

- a self-hostable system consistent with SR-SPEC’s event-sourced runtime,
- deterministic state progression given the recorded event/evidence stream,
- explicit human authority membranes (portals) and evidence-based verification.


---

## 3. Architecture and Tech Stack Assessment (Constraints)

The following are **constraints** (fixed unless changed via SR-CHANGE to SR-SPEC / SR-CONTRACT).

### 3.1 Stack (from SR-SPEC)

| Subsystem | Choice |
|---|---|
| domain_core | language=Rust; notes=Pure domain (ports + state machines) |
| api_layer | language=Rust; framework=Axum; notes=HTTP API + auth boundary |
| adapters_cli | language=TypeScript; notes=Operator tooling + optional CLI |
| event_store | system=PostgreSQL; pattern=append-only + per-stream sequencing + outbox |
| graph_store | system=PostgreSQL; method=graph_node/graph_edge + recursive CTEs |
| content_storage | system=MinIO (S3); method=content-addressed, immutable buckets |
| oracle_runtime | system=Podman; sandbox=gVisor (runsc); notes=rootless where possible |
| orchestration | system=Custom loop governor; messaging=NATS (Core + JetStream) |
| identity | system=Zitadel; protocol=OIDC |
| secrets | system=Infisical; role=secrets + envelope keys for restricted evidence |
| ui | system=React; language=TypeScript |

### 3.2 Planning implications

- **Rust domain + Axum API**: prioritize clear module boundaries (domain vs adapters vs transport) and deterministic error handling.
- **Postgres event store + projections**: design append-only streams, replay determinism tests, and projection rebuild consistency early.
- **Content-addressed evidence in MinIO**: evidence storage must be immutable, hash-addressed, and validated on ingest.
- **NATS JetStream**: orchestration messaging must tolerate redelivery; state transitions must be idempotent and deterministic.
- **Podman + gVisor**: oracle execution must run in sandboxed containers with explicit I/O capture and integrity checks.
- **Zitadel + Infisical**: auth and secrets handling must be explicit, auditable, and not “handwaved” into implicit trust.

---

## 4. Decomposition Strategy

- **One loop per deliverable:** each deliverable is intended to map 1:1 to a Ralph Loop `work_unit`.
- **Packages bias concurrency:** packages group deliverables primarily by output class and architectural layer (domain / adapters / API / UI / ops).
- **Dependencies are the only binding ordering:** package order is not binding; only `depends_on` edges are binding constraints.
- **SR-DIRECTIVE owns scheduling:** this plan intentionally does not define phases, budgets, retries, or global “done” criteria.

---

## 5. Packages Overview

| Package | Purpose | Deliverables |
|---|---|---|
| PKG-01 | Governance hygiene and unblockers | 1 |
| PKG-02 | Repo and CI substrate | 3 |
| PKG-03 | Domain core (deterministic rules) | 4 |
| PKG-04 | Persistence, projections, and graph | 5 |
| PKG-05 | Evidence storage and integrity | 3 |
| PKG-06 | API and identity boundary | 4 |
| PKG-07 | Orchestration runtime | 3 |
| PKG-08 | Oracles and verification substrate | 4 |
| PKG-09 | UI portals and human review surface | 3 |
| PKG-10 | Self-host and operations substrate | 3 |
| PKG-11 | End-to-end demonstration and determinism proof | 3 |

---

## 6. Packages and Deliverables

### PKG-01 — Governance hygiene and unblockers
**Grouping rationale:** Isolate governance/document maintenance work so it can be executed concurrently and does not pollute software deliverables. This package exists to keep the build effort from silently forking semantics.

**Package notes:**
- May be skipped if no blockers are encountered.

| Deliverable | Title | Output class | Depends on |
|---|---|---|---|
| D-01 | Governance hygiene patchset for build build | record |  |

#### D-01 — Governance hygiene patchset for build build
- **Output class:** `record`
- **Primary output:** A recorded governance-unblocker packet: rationale + proposed diffs for any required governance artifacts (e.g., SR-AGENTS, SR-DIRECTIVE, SR-TYPES alignment).
  **type_key:** `record.intervention_note`
- **Condition:** If governance doc/type mismatches are discovered that materially block implementation or increase drift risk.
- **Depends on:** *(none)*
- **Refs required:** `SR-CHANGE`, `SR-CONTRACT`, `SR-SPEC`, `SR-TYPES`, `SR-GUIDE`, `SR-DIRECTIVE`, `SR-README`, `SR-AGENTS`
- **Portal touchpoints (if applicable):** GovernanceChangePortal — only if this deliverable produces a governed change proposal; any binding decision attaches to the resulting change record (not to this plan).
- **Acceptance criteria:**
  - All proposed governance changes are expressed as explicit diffs or replacement files with versions bumped.
  - Conflicts are routed via SR-CHANGE (no informal reinterpretation).
  - Resulting docs remain coherent with SR-CONTRACT, SR-SPEC, SR-TYPES, SR-GUIDE.
- **Expected evidence:**
  - A governance change packet containing: rationale, diff/patch, and impacted-refs list (suitable for GovernanceChangePortal review).
- **Notes:**
  - This deliverable is **conditional**: execute only if governance inconsistencies block implementation or introduce drift risk.
### PKG-02 — Repo and CI substrate
**Grouping rationale:** Everything else depends on a reproducible build surface. This package creates the deterministic compilation + testing substrate required for reliable loops.

| Deliverable | Title | Output class | Depends on |
|---|---|---|---|
| D-02 | Repository scaffold and workspace layout | candidate |  |
| D-03 | Continuous integration baseline (build/test/lint) | candidate | D-02 |
| D-04 | Local developer tooling (scripts, dev env build) | candidate | D-02 |

#### D-02 — Repository scaffold and workspace layout
- **Output class:** `candidate`
- **Primary output:** A repo/workspace layout for Rust services + React UI + shared schemas, with consistent build tooling.
- **Depends on:** *(none)*
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Repo builds on a clean machine using a single documented command.
  - Rust workspace compiles (empty or stub modules acceptable).
  - UI project builds (skeleton acceptable).
  - Shared schema and codegen locations are defined (even if empty).
- **Expected evidence:**
  - Core build logs and artifact hashes in an evidence bundle (build-only acceptable for this scaffold).

#### D-03 — Continuous integration baseline (build/test/lint)
- **Output class:** `candidate`
- **Primary output:** CI pipeline that runs the baseline oracle suite(s) appropriate for the repo (format, lint, unit tests, minimal integration).
- **Depends on:** `D-02`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - CI runs automatically on main and on PR branches.
  - CI produces a machine-readable summary (pass/fail, durations, artifact hashes).
  - CI failures are surfaced deterministically (no hidden steps).
- **Expected evidence:**
  - Evidence bundle containing CI run logs and summary manifest.

#### D-04 — Local developer tooling (scripts, dev env build)
- **Output class:** `candidate`
- **Primary output:** A minimal set of local scripts/commands to run services, run tests, and start the self-host dev stack incrementally.
- **Depends on:** `D-02`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - A new developer can run `make dev` (or equivalent) to start the minimal stack.
  - A new developer can run `make test` (or equivalent) to execute the baseline oracle suite locally.
- **Expected evidence:**
  - Evidence bundle containing local run transcript (commands + outputs) and tool versions.
### PKG-03 — Domain core (deterministic rules)
**Grouping rationale:** The domain core is the deterministic supervisor. This package establishes the central state machine semantics and boundary interfaces before adapter work scales.

| Deliverable | Title | Output class | Depends on |
|---|---|---|---|
| D-05 | Domain model primitives and invariants | candidate | D-02 |
| D-06 | Deterministic state machines and transition validation | candidate | D-05 |
| D-07 | Ports and boundary interfaces (domain → adapters) | candidate | D-05 |
| D-08 | Context compilation rules (refs → deterministic context bundle) | candidate | D-06, D-07 |

#### D-05 — Domain model primitives and invariants
- **Output class:** `candidate`
- **Primary output:** Rust domain module defining the core entities and value objects (Loop, Iteration, Candidate, Run, Evidence, Approval, Freeze, Exception) and their invariants.
- **Depends on:** `D-02`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Core domain entities exist with explicit invariants and constructors.
  - Invariants correspond to SR-CONTRACT constraints where applicable (no premature authority claims; approvals are explicit records; etc.).
  - Domain types are serialization-friendly (for event storage) without embedding transport concerns.
- **Expected evidence:**
  - Evidence bundle containing unit test results and static checks for the domain crate.

#### D-06 — Deterministic state machines and transition validation
- **Output class:** `candidate`
- **Primary output:** Deterministic transition functions for Loop/Iteration/Candidate/Run lifecycles, with validation and property tests.
- **Depends on:** `D-05`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Each state machine has explicit allowed transitions and rejection reasons.
  - Invalid transitions are rejected deterministically with structured errors.
  - Property tests or model-based tests exist for at least the highest-risk transitions.
- **Expected evidence:**
  - Evidence bundle with unit + property test results for transition logic.

#### D-07 — Ports and boundary interfaces (domain → adapters)
- **Output class:** `candidate`
- **Primary output:** Trait interfaces for EventStore, EvidenceStore, OracleRunner, MessageBus, IdentityProvider, Clock, and any other required ports.
- **Depends on:** `D-05`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Ports are defined in a stable module with minimal coupling to adapter implementations.
  - Ports express the minimum operations required by SR-SPEC workflows.
  - Error types are explicit and suitable for deterministic handling.
- **Expected evidence:**
  - Evidence bundle with compilation + unit tests for ports module.

#### D-08 — Context compilation rules (refs → deterministic context bundle)
- **Output class:** `candidate`
- **Primary output:** Deterministic rules and helpers for turning typed refs into a context bundle for workers/oracles (selection, ordering, redaction).
- **Depends on:** `D-06`, `D-07`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Given the same ref set and artifact contents, the compiled context bundle is byte-identical (or content-hash identical).
  - Restricted/redacted material handling is explicit (no implicit leaks).
  - Compilation failures are deterministic and explainable.
- **Expected evidence:**
  - Evidence bundle with determinism tests over context compilation.
### PKG-04 — Persistence, projections, and graph
**Grouping rationale:** Event sourcing + projections are the mechanism for deterministic replayability. This package implements the storage substrate and derived views needed by API/UI and orchestration.

| Deliverable | Title | Output class | Depends on |
|---|---|---|---|
| D-09 | Postgres schemas and migrations (event store, projections, graph) | candidate | D-02 |
| D-10 | EventStore adapter (append-only streams + concurrency control) | candidate | D-07, D-09 |
| D-11 | Projection builder (read models from event streams) | candidate | D-10, D-06 |
| D-12 | Dependency graph projection + staleness traversal | candidate | D-10, D-09, D-06 |
| D-13 | Outbox publisher and NATS event publication | candidate | D-10 |

#### D-09 — Postgres schemas and migrations (event store, projections, graph)
- **Output class:** `candidate`
- **Primary output:** Database schemas + migrations for append-only event streams, outbox, read models, and the dependency graph projection.
- **Depends on:** `D-02`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Migrations are idempotent and versioned.
  - Schemas include constraints needed for event ordering and uniqueness.
  - Graph projection tables support staleness traversal queries as required by SR-SPEC.
- **Expected evidence:**
  - Evidence bundle with migration run logs against a clean Postgres instance.

#### D-10 — EventStore adapter (append-only streams + concurrency control)
- **Output class:** `candidate`
- **Primary output:** Postgres-backed EventStore adapter implementing the domain port: append, read, stream replay, optimistic concurrency as needed.
- **Depends on:** `D-07`, `D-09`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Events are stored append-only and retrievable by stream with deterministic ordering.
  - Concurrency conflicts are detected and surfaced deterministically.
  - A replay of an event stream reproduces the same aggregate state.
- **Expected evidence:**
  - Evidence bundle with adapter unit/integration tests and replay determinism test.

#### D-11 — Projection builder (read models from event streams)
- **Output class:** `candidate`
- **Primary output:** Projection builders that materialize loop/iteration/candidate/run read models from event streams.
- **Depends on:** `D-10`, `D-06`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Projection rebuild from scratch matches incremental projection results (consistency test).
  - Projection versioning / backfill strategy is explicit (even if simple initially).
- **Expected evidence:**
  - Evidence bundle with projection rebuild tests and sample datasets.

#### D-12 — Dependency graph projection + staleness traversal
- **Output class:** `candidate`
- **Primary output:** Graph projection builder and query surface for dependency traversal and staleness propagation (as per SR-SPEC).
- **Depends on:** `D-10`, `D-09`, `D-06`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Graph nodes/edges are derived deterministically from typed refs in events.
  - Traversal queries return correct transitive dependents on representative fixtures.
  - Staleness marking is representable as events and reflected in projections.
- **Expected evidence:**
  - Evidence bundle with graph projection + traversal tests.

#### D-13 — Outbox publisher and NATS event publication
- **Output class:** `candidate`
- **Primary output:** Outbox publisher that publishes selected domain events to NATS/JetStream reliably and idempotently.
- **Depends on:** `D-10`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Published event payloads are validated and include required metadata.
  - Publisher is resilient to restarts without duplicating logical events (idempotency keys).
- **Expected evidence:**
  - Evidence bundle with local NATS integration test logs.
### PKG-05 — Evidence storage and integrity
**Grouping rationale:** Evidence is the substrate for verification and human judgment. This package ensures evidence is durable, content-addressed, and (when required) access-controlled.

| Deliverable | Title | Output class | Depends on |
|---|---|---|---|
| D-14 | Evidence store adapter (MinIO, content-addressed bundles) | candidate | D-07, D-02 |
| D-15 | Evidence manifest v1 library + validation oracle | candidate | D-02 |
| D-16 | Restricted evidence handling (Infisical + envelope keys) | candidate | D-14 |

#### D-14 — Evidence store adapter (MinIO, content-addressed bundles)
- **Output class:** `candidate`
- **Primary output:** EvidenceStore adapter backed by MinIO/S3 with content-addressed objects and manifests.
- **Depends on:** `D-07`, `D-02`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Evidence bundles are stored immutably under content hashes.
  - Evidence manifests are validated on write and retrievable on read.
  - Access control story is explicit (even if coarse initially).
- **Expected evidence:**
  - Evidence bundle with upload/download integration tests and manifest validation results.

#### D-15 — Evidence manifest v1 library + validation oracle
- **Output class:** `candidate`
- **Primary output:** A shared library + oracle checks for evidence manifest v1 (hashing, required fields, deterministic serialization).
- **Depends on:** `D-02`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Manifest serialization is deterministic (stable ordering).
  - Hash computation is reproducible across runs.
  - Validation failures are deterministic and machine-readable.
- **Expected evidence:**
  - Evidence bundle with unit tests + oracle run logs for manifest validation.

#### D-16 — Restricted evidence handling (Infisical + envelope keys)
- **Output class:** `candidate`
- **Primary output:** Mechanism for restricted evidence: encryption at rest, envelope keys stored/managed via Infisical, and explicit redaction rules.
- **Depends on:** `D-14`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Restricted evidence cannot be retrieved in plaintext without authorized key access.
  - Key retrieval paths are auditable and minimal.
  - Redaction rules are explicit and test-covered.
- **Expected evidence:**
  - Evidence bundle with encryption/decryption tests and access-control checks.
### PKG-06 — API and identity boundary
**Grouping rationale:** The API is the external control plane and portal surface. Auth and strict transition enforcement must be in place before UI and orchestration can rely on it.

| Deliverable | Title | Output class | Depends on |
|---|---|---|---|
| D-17 | API service scaffold (Axum) + OIDC auth (Zitadel) | candidate | D-02 |
| D-18 | Core API endpoints (loops, iterations, candidates, runs) | candidate | D-17, D-10, D-06, D-11 |
| D-19 | Governance and portal-related API endpoints (approvals, freezes, exceptions, artifacts) | candidate | D-18, D-12, D-14 |
| D-20 | Evidence API (upload, retrieve, associate with runs/candidates) | candidate | D-14, D-18 |

#### D-17 — API service scaffold (Axum) + OIDC auth (Zitadel)
- **Output class:** `candidate`
- **Primary output:** Rust/Axum API service scaffold with OIDC authentication against Zitadel and request identity propagation.
- **Depends on:** `D-02`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Service starts and serves a health endpoint.
  - OIDC login flow works for a basic user and yields a verified identity in requests.
  - Auth failures are deterministic and logged.
- **Expected evidence:**
  - Evidence bundle with auth flow integration test transcript and service startup logs.

#### D-18 — Core API endpoints (loops, iterations, candidates, runs)
- **Output class:** `candidate`
- **Primary output:** Implement the primary SR-SPEC API endpoints for loops/iterations/candidates/runs, wired to domain core + persistence.
- **Depends on:** `D-17`, `D-10`, `D-06`, `D-11`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Endpoints enforce domain transition rules (invalid transitions rejected).
  - SYSTEM-only actions are enforced where required (per SR-SPEC).
  - All endpoints produce and consume typed refs where applicable.
- **Expected evidence:**
  - Evidence bundle with API contract tests and example traces.

#### D-19 — Governance and portal-related API endpoints (approvals, freezes, exceptions, artifacts)
- **Output class:** `candidate`
- **Primary output:** Endpoints to support portal workflows: approvals, exception requests/waivers, freeze creation, governed artifact registry queries, shippable computation.
- **Depends on:** `D-18`, `D-12`, `D-14`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Approvals are recorded as explicit durable objects (not implied).
  - Freeze objects are content-addressed and reference the exact artifact set.
  - Shippable computation is deterministic given current state.
- **Expected evidence:**
  - Evidence bundle with API tests covering approval/freeze/shippable flows.

#### D-20 — Evidence API (upload, retrieve, associate with runs/candidates)
- **Output class:** `candidate`
- **Primary output:** API surface for evidence bundle upload/retrieval and association with runs/candidates/iterations, using EvidenceStore adapter.
- **Depends on:** `D-14`, `D-18`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Evidence bundles can be uploaded and retrieved by content hash.
  - Association records link evidence to the relevant domain objects.
  - Manifest validation is enforced on ingest.
- **Expected evidence:**
  - Evidence bundle with evidence ingest/retrieve integration tests.
### PKG-07 — Orchestration runtime
**Grouping rationale:** Orchestration is where actor-driven choices become recorded transitions. This package implements the minimal runtime to start iterations and integrate a worker.

| Deliverable | Title | Output class | Depends on |
|---|---|---|---|
| D-21 | NATS/JetStream messaging integration (contracts + reliability) | candidate | D-13 |
| D-22 | Loop governor service (SYSTEM event emission + budgets) | candidate | D-21, D-18 |
| D-23 | Reference worker bridge (IterationStarted → context → candidate proposal) | candidate | D-08, D-22, D-20 |

#### D-21 — NATS/JetStream messaging integration (contracts + reliability)
- **Output class:** `candidate`
- **Primary output:** Integrate NATS JetStream as the message bus and define message contracts for orchestration events.
- **Depends on:** `D-13`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Message subjects and payload schemas are defined and versioned.
  - Publisher and consumer handle redelivery deterministically.
  - At-least-once semantics are handled without duplicating logical state transitions.
- **Expected evidence:**
  - Evidence bundle with message bus integration tests and redelivery simulations.

#### D-22 — Loop governor service (SYSTEM event emission + budgets)
- **Output class:** `candidate`
- **Primary output:** Implement the loop governor that decides when to start iterations, emits IterationStarted (SYSTEM-only), and enforces budgets/stop conditions as per SR-SPEC/SR-DIRECTIVE.
- **Depends on:** `D-21`, `D-18`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Governor emits IterationStarted only when preconditions hold (deterministic checks).
  - Governor respects configured budgets/limits (even if conservative initially).
  - Governor records its decisions as events or decision records (no silent actions).
- **Expected evidence:**
  - Evidence bundle with simulated loop runs and governor decision logs.

#### D-23 — Reference worker bridge (IterationStarted → context → candidate proposal)
- **Output class:** `candidate`
- **Primary output:** A reference worker that consumes IterationStarted, compiles context from refs, and submits a candidate + summary back to the system (may be stubbed initially).
- **Depends on:** `D-08`, `D-22`, `D-20`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Worker can deterministically compile the context bundle from refs.
  - Worker can submit a candidate proposal and a structured summary record.
  - Worker failures are recorded and do not corrupt state.
- **Expected evidence:**
  - Evidence bundle with end-to-end trace: iteration started → worker receives → candidate submitted.
### PKG-08 — Oracles and verification substrate
**Grouping rationale:** Oracles convert candidates into evidence. This package provides sandboxed execution, core suites, and integrity checks so the rest of the system can rely on verifiable outcomes.

| Deliverable | Title | Output class | Depends on |
|---|---|---|---|
| D-24 | Oracle runner service (Podman + gVisor) | candidate | D-02, D-14 |
| D-25 | Core oracle suite implementation (build/unit/schema/lint) | candidate | D-03, D-15 |
| D-26 | Integration/e2e oracle suite (DB/MinIO/NATS/API/UI) | candidate | D-24, D-25, D-31, D-18, D-28 |
| D-27 | Oracle integrity checks (TAMPER/GAP/FLAKE/ENV_MISMATCH) | candidate | D-24, D-25 |

#### D-24 — Oracle runner service (Podman + gVisor)
- **Output class:** `candidate`
- **Primary output:** Service that runs oracle suites in sandboxed containers, captures outputs, and emits evidence bundles.
- **Depends on:** `D-02`, `D-14`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Runner can execute a containerized oracle deterministically given the same inputs.
  - Runner captures stdout/stderr, exit codes, and artifact hashes.
  - Runner cannot access restricted evidence without explicit authorization.
- **Expected evidence:**
  - Evidence bundle with sample oracle run logs and container isolation checks.

#### D-25 — Core oracle suite implementation (build/unit/schema/lint)
- **Output class:** `candidate`
- **Primary output:** Implement the core oracle suite(s) used for most loops: build, unit tests, schema validation, lint/format.
- **Depends on:** `D-03`, `D-15`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Suite produces machine-readable results and deterministic summaries.
  - Failures are attributable and reproducible locally.
  - Suite output is stored as evidence bundles.
- **Expected evidence:**
  - Evidence bundle containing a passing run of the core suite on the repo.

#### D-26 — Integration/e2e oracle suite (DB/MinIO/NATS/API/UI)
- **Output class:** `candidate`
- **Primary output:** Implement integration and e2e oracle suites for the full stack in self-host mode.
- **Depends on:** `D-24`, `D-25`, `D-31`, `D-18`, `D-28`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-FULL`
- **Acceptance criteria:**
  - Suite can stand up the full stack and run e2e flows deterministically within tolerance.
  - Flake controls exist (timeouts, retries policy) and are recorded.
- **Expected evidence:**
  - Evidence bundle with an e2e run transcript and artifacts.

#### D-27 — Oracle integrity checks (TAMPER/GAP/FLAKE/ENV_MISMATCH)
- **Output class:** `candidate`
- **Primary output:** Implement oracle-integrity checks and stop triggers consistent with SR-SPEC/SR-DIRECTIVE (tamper detection, flake handling, environment mismatch).
- **Depends on:** `D-24`, `D-25`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Tamper detection is implemented for evidence bundles and candidate hashes.
  - Known integrity failure modes produce explicit failure records/flags.
  - Stop triggers route to the correct portal touchpoints (no silent overrides).
- **Expected evidence:**
  - Evidence bundle with simulated integrity failure cases and expected outputs.
### PKG-09 — UI portals and human review surface
**Grouping rationale:** Humans cross authority membranes through portals. This package provides the minimum UI needed to review evidence and record approvals/waivers.

| Deliverable | Title | Output class | Depends on |
|---|---|---|---|
| D-28 | UI scaffold (React) + OIDC login | candidate | D-02, D-17 |
| D-29 | Loop/iteration/candidate views + evidence viewer | candidate | D-28, D-20, D-18 |
| D-30 | Portal workflows UI (approvals, exceptions, oracle suite changes) | candidate | D-29, D-19 |

#### D-28 — UI scaffold (React) + OIDC login
- **Output class:** `candidate`
- **Primary output:** React UI scaffold with OIDC login (Zitadel) and basic routing/layout.
- **Depends on:** `D-02`, `D-17`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - UI can authenticate and display the logged-in identity.
  - UI build is reproducible and runs in CI.
- **Expected evidence:**
  - Evidence bundle with UI build/test logs.

#### D-29 — Loop/iteration/candidate views + evidence viewer
- **Output class:** `candidate`
- **Primary output:** UI pages to view loops/iterations/candidates and to inspect associated evidence bundles (manifests + artifacts).
- **Depends on:** `D-28`, `D-20`, `D-18`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - UI can render loop/iteration status from API read models.
  - Evidence bundles can be browsed by content hash with clear provenance.
- **Expected evidence:**
  - Evidence bundle with UI tests and a recorded demo trace.

#### D-30 — Portal workflows UI (approvals, exceptions, oracle suite changes)
- **Output class:** `candidate`
- **Primary output:** UI forms/workflows for submitting approvals and exception requests corresponding to SR-DIRECTIVE portals.
- **Depends on:** `D-29`, `D-19`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Approvals are explicit submissions with attributable identity.
  - Exception requests include required rationale and scope.
  - UI does not imply approval; it records submissions only.
- **Expected evidence:**
  - Evidence bundle with UI workflow tests and API integration traces.
### PKG-10 — Self-host and operations substrate
**Grouping rationale:** Instance-1 targets a fully usable self-hosted system. This package makes the stack runnable end-to-end with pinned dependencies and basic observability.

| Deliverable | Title | Output class | Depends on |
|---|---|---|---|
| D-31 | Self-host deployment stack (compose/podman) | candidate | D-02 |
| D-32 | Build/init scripts (DB, buckets, identity realm, secrets) | candidate | D-31, D-09, D-16 |
| D-33 | Operational logging + minimal observability | candidate | D-17, D-22, D-24 |

#### D-31 — Self-host deployment stack (compose/podman)
- **Output class:** `candidate`
- **Primary output:** Self-host deployment config for Postgres, MinIO, NATS, Zitadel, Infisical, and the SR services (API, governor, oracle runner, UI).
- **Depends on:** `D-02`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-FULL`
- **Acceptance criteria:**
  - A single command starts the full stack locally.
  - Service dependencies come up reliably (health checks).
  - Versions are pinned and recorded.
- **Expected evidence:**
  - Evidence bundle with startup logs and service health checks.

#### D-32 — Build/init scripts (DB, buckets, identity realm, secrets)
- **Output class:** `candidate`
- **Primary output:** Scripts to initialize Postgres schemas, MinIO buckets/policies, Zitadel realm, and Infisical secrets needed for local operation.
- **Depends on:** `D-31`, `D-09`, `D-16`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-FULL`
- **Acceptance criteria:**
  - Build is idempotent (safe to re-run).
  - Secrets are not stored in plaintext in repo; Infisical holds sensitive values.
- **Expected evidence:**
  - Evidence bundle with build run logs and resulting resource inventory.

#### D-33 — Operational logging + minimal observability
- **Output class:** `candidate`
- **Primary output:** Structured logs and minimal metrics endpoints for core services; correlation IDs across requests/events.
- **Depends on:** `D-17`, `D-22`, `D-24`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-CORE`
- **Acceptance criteria:**
  - Logs include request/trace identifiers and actor identity where applicable.
  - Key state transitions are logged with event ids (no silent transitions).
- **Expected evidence:**
  - Evidence bundle with sample logs from an end-to-end run.
### PKG-11 — End-to-end demonstration and determinism proof
**Grouping rationale:** The objective is a usable system that deterministically progresses by recorded events. This package proves the system works end-to-end and is replayable.

| Deliverable | Title | Output class | Depends on |
|---|---|---|---|
| D-34 | End-to-end harness (happy path) | candidate | D-22, D-23, D-24, D-25, D-19, D-30, D-31 |
| D-35 | End-to-end harness (failure modes: integrity + exception) | candidate | D-34, D-27 |
| D-36 | Replayability demonstration (event stream → reconstructed state) | candidate | D-10, D-11, D-34 |

#### D-34 — End-to-end harness (happy path)
- **Output class:** `candidate`
- **Primary output:** Automated harness to execute a representative end-to-end loop: create loop → start iteration → worker submits candidate → oracles run → evidence stored → approval recorded → freeze baseline.
- **Depends on:** `D-22`, `D-23`, `D-24`, `D-25`, `D-19`, `D-30`, `D-31`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-FULL`
- **Acceptance criteria:**
  - Harness can run in self-host mode and produce a deterministic event+evidence transcript.
  - Harness asserts key invariants (no approvals without evidence, etc.).
- **Expected evidence:**
  - Evidence bundle containing the harness run transcript and produced event/evidence ids.

#### D-35 — End-to-end harness (failure modes: integrity + exception)
- **Output class:** `candidate`
- **Primary output:** Automated harness cases for at least: oracle failure, integrity failure (tamper), and exception request/waiver flow.
- **Depends on:** `D-34`, `D-27`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-FULL`
- **Acceptance criteria:**
  - Failure cases are recorded as explicit events/records.
  - System routes to the correct portal touchpoints instead of silently proceeding.
- **Expected evidence:**
  - Evidence bundle with failure-case run transcripts and expected portal submissions.

#### D-36 — Replayability demonstration (event stream → reconstructed state)
- **Output class:** `candidate`
- **Primary output:** Tooling or procedure to replay the recorded event stream and reconstruct system state deterministically (including projections).
- **Depends on:** `D-10`, `D-11`, `D-34`
- **Refs required:** `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`
- **Recommended verification profile (non-binding):** `STRICT-FULL`
- **Acceptance criteria:**
  - Replaying the same event stream yields the same reconstructed state hash (or equivalent deterministic checksum).
  - Replay procedure is documented and runnable.
- **Expected evidence:**
  - Evidence bundle with replay run logs and resulting state checksum.

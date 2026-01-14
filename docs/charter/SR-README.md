---
doc_id: SR-README
doc_kind: governance.readme
layer: build
status: draft
normative_status: index

refs:
  - rel: governed_by
    to: SR-CHANGE
---

# SR-README

Canonical index for the SR-* document set. 

Begin your task assignment by reading SR-CHARTER.  The project documentation constitutes a total development plan and specification with detailed instructions on types and contracts.  Always read the SR-* files that appear related to the task before going to read the code files.  Documentation leads development for this project.  Documentation is how you know your ontology, epistemology, and semantics.

Once you have read the appropriate project docs, then do whatever deliverables and tasks you think should be done next.

Your task is done when there are no more deliverables to be assigned, which means all tests have passed and gates cleared.

You should push on to resolve any findings and consult the docs/ frequently for guidance and direction from the SR-* documents, several of which are normative.  They are typed documents and structured to aid in navigation.

You should git add && commit && push after completing each deliverable.  You can remain on the same branch throughout this development session.

If you cannot pass the tests for that deliverable then you must summarize what you did during that development session, delete the previous message where it says "Development History Summary for this Deliveralbe" and then append your new message including how to identify the task that was being worked on when the next instance of yourself begins the next iteration.

ALWAYS refer to the project docs/*/SR-* for the authoritative coding architecture, plan, and semantics.  Understand the full set of docs/ and refer to the applicable SR-* document instead of making assumptions.

When troubleshooting, refer to the appropriate SR-* documents.

## Canonical document paths

| doc_id | Folder | Purpose |
|--------|--------|---------|
| SR-CHARTER | `charter/` | Project scope and priorities |
| SR-CONTRACT | `platform/` | Binding invariants |
| SR-SPEC | `platform/` | Platform mechanics |
| SR-TYPES | `platform/` | Type registry and schemas |
| SR-WORK-SURFACE | `platform/` | Work surface definitions |
| SR-PROCEDURE-KIT | `platform/` | Procedure templates |
| SR-SEMANTIC-ORACLE-SPEC | `platform/` | Semantic oracle interface |
| SR-EVENT-MANAGER | `platform/` | Event projections spec |
| SR-AGENT-WORKER-CONTRACT | `platform/` | Agent/worker contract |
| SR-INTENT | `platform/` | Design rationale |
| SR-CHANGE | `build-governance/` | Change control process |
| SR-AGENTS | `build-governance/` | Agent actor model |
| SR-TASKS | `build-governance/` | Task assignment |
| SR-EXCEPTIONS | `build-governance/` | Exception ledger |
| SR-PLAN | `program/` | Build plan instance |
| SR-DIRECTIVE | `program/` | Execution policy |
| SR-README | `charter/` | This index |

## Development History Summary for this Deliverable

### Session 19 (2026-01-14)
**Completed:** D-36

**What was done:**

D-36: Replayability demonstration (event stream → reconstructed state)
- Created replay module in sr-e2e-harness with:
  - ReplayRunner: executes event stream replay and projection rebuild
  - ReplayConfig: configurable database URL, batch size, full/incremental mode
  - StateChecksum: deterministic SHA-256 of all projection state
  - TableChecksum: per-table row count and content hash
  - ReplayTranscript: audit trail of replay execution
  - DeterminismResult: verification that replays produce identical state
- CLI commands added to sr-e2e-harness:
  - `--replay`: Replay event stream, rebuild projections, compute state checksum
  - `--verify-determinism`: Run replay twice and verify identical checksums
  - `--database-url`: Database connection for replay mode
  - `--batch-size`: Event processing batch size
  - `--incremental`: Incremental update instead of full rebuild
- Per SR-SPEC §1.7 requirement implemented:
  - All projections rebuildable from es.events alone
  - Deterministic state checksum computation
  - Component-level checksums for debugging

**PKG-11 (Harness) progress: D-34 ✓, D-36 ✓**

**Next deliverables:**
- D-26: Integration/e2e oracle suite (PKG-08) - depends on D-24 ✓, D-25 ✓, D-31 ✓, D-18 ✓, D-28 ✓
- D-33: Operational logging + observability (PKG-10) - depends on D-17 ✓, D-22 ✓, D-24 ✓
- D-41: Reference semantic worker (PKG-12) - depends on D-23 ✓, D-38 ✓, D-39 ✓, D-40


---

### Session 18 (2026-01-13)
**Completed:** D-34

**What was done:**

D-34: End-to-end harness (happy path)
- Created sr-e2e-harness crate with full e2e test automation:
  - E2EClient: HTTP client for all API endpoints (loops, iterations, candidates, runs, evidence, approvals, freeze)
  - HarnessTranscript: Deterministic event+evidence transcript for audit
  - run_happy_path(): Complete flow execution with invariant checks
- Happy path flow implemented:
  1. Create loop → Activate loop
  2. Start iteration (SYSTEM actor)
  3. Register candidate (worker submission)
  4. Start oracle run → Upload evidence bundle → Complete run
  5. Record approval (HUMAN at Release Portal)
  6. Create freeze record (HUMAN-only, establishing baseline)
  7. Close loop
- Key invariants asserted:
  - no_approval_without_evidence: Approvals must have evidence refs
  - loop_active_after_activation: Loop state validation
  - run_has_evidence: Runs must have evidence bundle hash
  - freeze_has_approval: Freeze records must reference approval
  - freeze_has_evidence: Freeze records must have evidence refs
- CLI tool (sr-e2e-harness) with:
  - Configurable API URL, tokens, oracle suite
  - JSON transcript output option
  - Entity and event ID tracking
  - Invariant check reporting

**PKG-11 (Harness) progress: D-34 done**


---

### Session 17 (2026-01-13)
**Completed:** D-30

**What was done:**

D-30: Portal workflows UI (approvals, exceptions, freeze records)
- Approvals.tsx - Full portal workflows page:
  - Approval submission form (HUMAN-only per SR-CONTRACT C-TB-3)
  - Exception creation/activation/resolution (HUMAN-only per SR-SPEC §1.8)
  - Decision recording with precedent support (HUMAN-only per C-DEC-1)
  - Tabbed interface with counters for each entity type
  - Governance note warning about HUMAN-only requirements
- CandidateDetail.tsx - Freeze record creation:
  - Added "Create Freeze" form to freeze records tab
  - Baseline ID, verification mode, oracle suite, approval linkage
  - Integration with existing candidate context
- Per SR-SPEC §1.8, exceptions (DEVIATION, DEFERRAL, WAIVER) are narrowly scoped permissions; waivers cannot target integrity conditions

**PKG-09 (UI portals) progress: D-28 ✓, D-29 ✓, D-30 done**

**Next deliverables:**
- D-26: Integration/e2e oracle suite (PKG-08) - depends on D-24 ✓, D-25 ✓, D-31 ✓, D-18 ✓, D-28 ✓
- D-33: Operational logging + observability (PKG-10) - depends on D-17 ✓, D-22 ✓, D-24 ✓
- D-34: End-to-end harness happy path (PKG-11) - depends on D-22 ✓, D-23 ✓, D-24 ✓, D-25 ✓, D-19 ✓, D-30 ✓, D-31 ✓
- D-41: Reference semantic worker (PKG-12) - depends on D-23 ✓, D-38 ✓, D-39 ✓, D-40


---

### Session 16 (2026-01-13)
**Completed:** D-39

**What was done:**

D-39: Semantic oracle runner integration (meaning matrices/manifolds)
- Created semantic_oracle.rs module in sr-domain with:
  - SemanticSet schema (v1): manifold/meaning-matrix definition with axes, constraints, decision rules
  - SemanticAxis: dimension of semantic space with weight, required flag, min coverage, max residual
  - SemanticConstraint: required/preferred/prohibited constraint types with severity levels
  - DecisionRule: pass/fail derivation from measurements (max_residual_norm, min_coverage, max_violations)
  - SemanticEvalResult schema (`sr.semantic_eval.v1`): structured output per SR-SEMANTIC-ORACLE-SPEC §4
  - ResidualVector, CoverageMetrics, ConstraintViolation measurement types
  - Report artifacts: ResidualReport, CoverageReport, ViolationsReport
  - Content-addressed hashing for semantic set binding
  - intake_admissibility_semantic_set() for Branch 0
- Created semantic_suite.rs module in sr-adapters with:
  - SemanticOracleSuiteDefinition: extends OracleSuiteDefinition with semantic set binding
  - create_intake_admissibility_suite(): `oracle.suite.intake_admissibility.v1` for Branch 0
  - IntakeAdmissibilityRunner: deterministic local evaluation for intake artifacts
  - Six semantic oracles bound to axes: schema_compliance, traceability_coverage, contradiction_free, ambiguity_bounded, privacy_safe, term_map_aligned
  - Suite hash incorporates semantic set hash per SR-SEMANTIC-ORACLE-SPEC §2
  - SemanticReportBundle: generates residual, coverage, violations report artifacts
  - to_oracle_suite_definition(): converts semantic suite to standard OracleSuiteDefinition
- Updated OracleSuiteRegistry to register semantic suite alongside core suites
- 85 unit tests passing (8 new tests for semantic_suite, 12 new tests for semantic_oracle)

**PKG-12 (Semantic work surface) progress: D-37 ✓, D-38 ✓, D-39 done**

**Next deliverables:**
- D-26: Integration/e2e oracle suite (PKG-08) - depends on D-24 ✓, D-25 ✓, D-31 ✓, D-18 ✓, D-28 ✓
- D-30: Portal workflows UI (approvals, exceptions) (PKG-09) - depends on D-29 ✓, D-19 ✓
- D-33: Operational logging + observability (PKG-10) - depends on D-17 ✓, D-22 ✓, D-24 ✓
- D-41: Reference semantic worker (PKG-12) - depends on D-23 ✓, D-38 ✓, D-39 ✓


---

### Session 15 (2026-01-13)
**Completed:** D-38

**What was done:**

D-38: Prompt → Plan Instance decomposition (semantic work unit generator)
- Created plan_instance.rs module in sr-domain with:
  - PlanInstance schema (v1): commitment object with work units, dependency edges, source reference, content hash
  - WorkUnitPlan schema: individual work unit with intake_ref, procedure_template_ref, initial_stage_id, depends_on edges, status
  - DecompositionRationale: non-binding rationale stored separately from binding dependency edges (per D-38 acceptance criteria)
  - Identifier types: PlanInstanceId, SourceRefId
  - Status enums: PlanStatus, WorkUnitPlanStatus, DependencyEdgeType
- PlanDecomposer deterministic pipeline:
  - decompose(): multi-intake decomposition with dependency spec
  - decompose_single(): single intake decomposition
  - IntakeWithRef, ProcedureTemplateWithRef, DependencySpec helper types
  - DecompositionResult: plan + rationale pair
- Validators per D-38 acceptance criteria:
  - PlanInstanceValidator: acyclic graph validation, dependency target validation, required field validation
  - Cycle detection in dependency graph
  - Content hash computation for both PlanInstance and DecompositionRationale
- Key features:
  - Eligible set computation (get_eligible_work_units)
  - Deterministic content hashing for commitment object semantics
  - Separation of binding edges from non-binding rationale
- Added Hash derive to WorkKind for HashMap support
- 66 unit tests passing (14 new tests for plan_instance module)

**PKG-12 (Semantic work surface) progress: D-37 ✓, D-38 done**

**Next deliverables:**
- D-26: Integration/e2e oracle suite (PKG-08) - depends on D-24 ✓, D-25 ✓, D-31 ✓, D-18 ✓, D-28 ✓
- D-30: Portal workflows UI (approvals, exceptions) (PKG-09) - depends on D-29 ✓, D-19 ✓
- D-33: Operational logging + observability (PKG-10) - depends on D-17 ✓, D-22 ✓, D-24 ✓
- D-39: Semantic oracle runner integration (PKG-12) - depends on D-15 ✓, D-27 ✓, D-37 ✓


---

### Session 14 (2026-01-13)
**Completed:** D-37

**What was done:**

D-37: Work surface schemas (Intake + Procedure Template) + validators
- Created work_surface.rs module in sr-domain with core schemas:
  - Intake schema (v1): work_unit_id, title, kind (WorkKind enum), objective, audience, deliverables, constraints, definitions, inputs, unknowns, completion_criteria
  - ProcedureTemplate schema (v1): procedure_template_id, kind, stages with required_outputs, required_oracle_suites, gate_rule, transition_on_pass, portal configuration
  - WorkSurfaceInstance schema (v1): work_unit_id, intake_ref, procedure_template_ref, stage_id, oracle_suites with content-addressed refs
  - Identifier types: IntakeId, ProcedureTemplateId, StageId, WorkUnitId
- Validators per SR-WORK-SURFACE:
  - IntakeValidator: required fields (title, objective, audience, deliverables with paths)
  - ProcedureTemplateValidator: stage validation, terminal stage exists, transition targets valid, portal config consistency
  - WorkSurfaceInstanceValidator: refs integrity, content hashes required
- Created procedure_templates.rs module with Branch 0 templates:
  - Problem Statement Ingestion template: INGEST → VALIDATE → ACCEPT (with portal boundary)
  - Generic Knowledge Work template: FRAME → OPTIONS → DRAFT → SEMANTIC_EVAL → FINAL
  - Template registry with deterministic content hashing
  - Intake admissibility oracle suite bindings
- 53 unit tests passing covering all schemas, validators, and templates

**PKG-12 (Semantic work surface) progress: D-37 done**


---

### Session 13 (2026-01-13)
**Completed:** D-29

**What was done:**

D-29: Loop/iteration/candidate views + evidence viewer
- LoopDetail page: Single loop view with iterations list, budgets/usage stats, directive ref
- IterationDetail page: Single iteration view with candidates list, context refs, summary (rationale, actions, blockers, next steps)
- CandidateDetail page: Tabbed interface with oracle runs, evidence bundles, freeze records
- EvidenceDetail page: Full manifest viewer with oracle results (pass/fail/error counts), artifacts with download links, raw JSON toggle
- Updated Loops.tsx with navigation links to loop details
- Updated Evidence.tsx with navigation links to evidence details
- Added routes: /loops/:loopId, /iterations/:iterationId, /candidates/:candidateId, /evidence/:contentHash

**PKG-09 (UI portals) progress: D-28 ✓, D-29 done**


---

### Session 12 (2026-01-13)
**Completed:** D-28

**What was done:**

D-28: UI scaffold (React) + OIDC login
- AuthProvider using oidc-client-ts and react-oidc-context
- Authorization Code flow with PKCE for secure SPA authentication
- ProtectedRoute component for route guarding
- Layout with header, navigation, and auth controls
- User identity display with login/logout functionality
- Environment configuration (VITE_OIDC_ISSUER, VITE_OIDC_CLIENT_ID)
- Pages: Home, Callback, Loops, Evidence, Approvals (scaffolds)

**PKG-09 (UI portals) progress: D-28 done**


---

### Session 11 (2026-01-13)
**Completed:** D-23, D-24, D-25, D-27

**What was done:**

D-23: Reference worker bridge (IterationStarted → context → candidate)
- ReferenceWorkerBridge consumes IterationStarted from NATS
- Deterministic context compilation using D-08 ContextCompiler
- ContentResolver for ref-to-hash resolution with classification

D-24: Oracle runner service (Podman + gVisor)
- PodmanOracleRunner implementing OracleRunner port trait
- Container isolation: network disabled, read-only workspace, scratch volume
- EnvironmentFingerprint for reproducibility auditing

D-25: Core oracle suite implementation
- OracleSuiteRegistry for suite definitions
- SR-SUITE-GOV: meta_validate, refs_validate
- SR-SUITE-CORE: build, test, lint, schema, integrity_smoke
- SR-SUITE-FULL: + integration, e2e, replay_verify, sbom
- VerificationProfile: GOV-CORE, STRICT-CORE, STRICT-FULL
- WaivableCondition vs IntegrityCondition distinction
- Oracle report structures (BuildReport, UnitTestReport, etc.)
- Deterministic suite hashing via compute_suite_hash()

D-27: Oracle integrity checks (TAMPER/GAP/FLAKE/ENV_MISMATCH)
- IntegrityChecker for oracle run validation
- Tamper detection (hash mismatch, evidence hash verification)
- Gap detection (missing required oracles in suite)
- Flake detection (inconsistent oracle results across runs)
- Environment mismatch detection (fingerprint drift)
- IntegrityCondition enum (non-waivable per SR-CONTRACT)

**PKG-07 (Orchestration runtime) complete (D-21, D-22, D-23)**
**PKG-08 (Oracle substrate) progress: D-24, D-25, D-27 done**


---

### Session 10 (2026-01-13)
**Completed:** D-16, D-21, D-22, D-32

**What was done:**

D-16: Restricted evidence handling (Infisical + envelope keys)
- SecretProvider port trait (sr-ports) with envelope key management
- RestrictedEvidenceStore with AES-256-GCM envelope encryption
- RedactionManifest schema per C-EVID-5
- InfisicalSecretProvider adapter with KEK caching

D-21: NATS/JetStream messaging integration (contracts + reliability)
- NatsMessageBus adapter implementing MessageBus port
- MessageEnvelope schema (v1.0) with idempotency key
- NatsConsumer with redelivery handling via ack/nak/term

D-22: Loop governor service (SYSTEM event emission + budgets)
- LoopGovernor service for iteration orchestration
- IterationStarted emission with SYSTEM actor only (per SR-SPEC)
- LoopBudget, StopCondition, GovernorDecision audit records

D-32: Build/init scripts (DB, buckets, identity realm, secrets)
- init.sh: comprehensive initialization script
- PostgreSQL schemas: es.events, es.outbox, proj.*, graph.*
- MinIO buckets: evidence, candidates, artifacts
- Zitadel config template with roles and OIDC clients
- Development secrets with envelope KEK
- Idempotent (safe to re-run), --check for status

**PKG-05 (Evidence storage and integrity) complete**
**PKG-07 (Orchestration runtime) progress: D-21, D-22 done**
**PKG-10 (Self-host instance) progress: D-31, D-32 done**


---

### Session 9 (2026-01-13)
**Completed:** D-20

**What was done:**

D-20: Evidence API endpoints (upload, retrieve, associate with runs/candidates)

- Evidence handlers: upload, get, list, associate, verify, get_blob
- UploadEvidenceRequest with manifest validation + base64 blob encoding
- EvidenceBundleRecorded event on upload
- EvidenceAssociated event for explicit associations
- Content-addressed storage via MinioEvidenceStore
- Manifest validation using EvidenceManifest.validate()
- Integrity verification endpoint

- Added EvidenceProjection struct and query methods to ProjectionBuilder
- Added migration 004_evidence.sql for evidence_bundles and evidence_associations tables
- Event handlers for EvidenceBundleRecorded and EvidenceAssociated
- Added MinioEvidenceStore to AppState

**Routes implemented:**
- POST /api/v1/evidence (upload with manifest + blobs)
- GET /api/v1/evidence (list all evidence)
- GET /api/v1/evidence/:hash (get evidence manifest)
- POST /api/v1/evidence/:hash/associate (link to run/candidate/iteration)
- POST /api/v1/evidence/:hash/verify (integrity check)
- GET /api/v1/evidence/:hash/blobs/:name (get specific blob)
- GET /api/v1/runs/:id/evidence (list evidence for run)
- GET /api/v1/candidates/:id/evidence (list evidence for candidate)

**PKG-06 (API) is now fully complete (D-17, D-18, D-19, D-20 done)**

**Next deliverables to work on:**
- D-16: Restricted evidence handling (depends on D-14) - PKG-05
- D-28: UI scaffold + OIDC login (depends on D-02, D-17) - PKG-09
- D-32: Build/init scripts (depends on D-31, D-09, D-16) - PKG-10
- D-21: NATS/JetStream messaging integration (depends on D-13) - PKG-07


---

### Session 8 (2026-01-13)
**Completed:** D-19

**What was done:**

D-19: Governance and portal-related API endpoints (approvals, exceptions, decisions, freeze records)

- Approval handlers: record (HUMAN-only per SR-CONTRACT C-TB-3), get, list, list_for_portal
- Exception handlers: create (HUMAN-only per SR-SPEC §1.8), get, list, activate, resolve
  - Integrity conditions (ORACLE_TAMPER, ORACLE_GAP, etc.) cannot be waived
  - Exception kinds: DEVIATION, DEFERRAL, WAIVER with proper validation
- Decision handlers: record (HUMAN-only per SR-CONTRACT C-DEC-1), get, list
  - Support for precedent decisions with applicability clauses
- Freeze record handlers: create (HUMAN-only per SR-CONTRACT C-SHIP-1), get, list, list_for_candidate
  - Verification modes: STRICT, WITH_EXCEPTIONS
  - Artifact manifest and active exceptions captured at freeze time

- Added projection types: ApprovalProjection, ExceptionProjection, DecisionProjection, FreezeRecordProjection
- Added query methods to ProjectionBuilder for all new entities
- Wired all D-19 routes in main.rs

**Routes implemented:**
- POST/GET /api/v1/approvals, GET approvals/:id, GET portals/:id/approvals
- POST/GET /api/v1/exceptions, GET exceptions/:id, POST exceptions/:id/activate, POST exceptions/:id/resolve
- POST/GET /api/v1/decisions, GET decisions/:id
- POST/GET /api/v1/freeze-records, GET freeze-records/:id, GET candidates/:id/freeze-records

**PKG-06 (API) is now fully complete (D-17, D-18, D-19 done)**

**Next deliverables to work on:**
- D-16: Restricted evidence handling (depends on D-14) - PKG-05
- D-20: Evidence management API endpoints (depends on D-14, D-15) - PKG-06
- D-28: UI scaffold + OIDC login (depends on D-02, D-17) - PKG-09
- D-32: Build/init scripts (depends on D-31, D-09, D-16) - PKG-10


---

### Session 7 (2026-01-13)
**Completed:** D-18

**What was done:**

D-18: Core API endpoints for loops, iterations, candidates, runs
- Loop handlers: create, get, list, activate, pause, resume, close
- Iteration handlers: start (SYSTEM-only per SR-SPEC §2.2), get, list, complete
- Candidate handlers: register, get, list (by iteration or all)
- Run handlers: start, get, list (by candidate or all), complete
- API error types with proper HTTP status codes
- Request/response types with JSON serialization
- State transition validation using domain state machines
- Event appending to PostgresEventStore
- Projection updates after state changes
- Added query methods to ProjectionBuilder: list_loops, get_iteration, get_candidates_for_iteration, list_candidates, get_run, list_runs
- Updated main.rs with full route wiring and database initialization
- AppState includes event_store and projections adapters

**Routes implemented:**
- POST/GET /api/v1/loops, GET/POST loops/:id/*, loops/:id/iterations
- POST /api/v1/iterations, GET iterations/:id, POST iterations/:id/complete
- POST/GET /api/v1/candidates, GET candidates/:id, candidates/:id/runs
- POST/GET /api/v1/runs, GET runs/:id, POST runs/:id/complete

**PKG-06 (API) is now complete (D-17, D-18 done)**

---

### Session 6 (2026-01-13)
**Completed:** D-14, D-15, D-17, D-31

**What was done:**

D-14: Evidence store adapter (MinIO, content-addressed bundles)
- MinioEvidenceStore implementing EvidenceStore port
- Content-addressed storage using SHA-256 hashes
- Immutable object storage (no overwrites)
- Blob storage and retrieval per evidence bundle
- Integrity verification via hash recomputation
- Unit tests for hash determinism and order independence

D-15: Evidence manifest v1 library + validation oracle
- EvidenceManifest schema (evidence.gate_packet artifact type)
- EvidenceManifestBuilder for constructing manifests
- OracleResult with Pass/Fail/Error/Skipped status
- EvidenceArtifact descriptors with content hashes
- Deterministic JSON serialization (sorted keys)
- ManifestValidationOracle for schema validation
- Verdict computation from oracle results
- Unit tests for validation, serialization, roundtrip

D-17: API service scaffold (Axum) + OIDC auth (Zitadel)
- OidcProvider with JWKS fetching and JWT validation
- AuthenticatedUser extractor with actor kind derivation
- Role-based authorization helpers (require_role, require_human)
- OptionalAuth extractor for unauthenticated endpoints
- ApiConfig with environment variable loading
- Health, info, whoami, and protected endpoints
- Test mode for development without Zitadel
- Unit tests for auth and endpoints

D-31: Self-host deployment stack (compose/podman)
- docker-compose.yml with all services:
  - PostgreSQL 16.4 (event store + projections)
  - MinIO (evidence storage)
  - NATS with JetStream (message bus)
  - Zitadel (OIDC identity provider)
- Dockerfile.api (multi-stage Rust build)
- Dockerfile.ui (React with nginx)
- Init scripts for PostgreSQL databases
- start.sh helper script for deployment
- Makefile targets: deploy, deploy-up, deploy-down, deploy-logs

**PKG-05 (Evidence storage) is now partially complete (D-14, D-15 done; D-16 pending)**
**PKG-06 (API) is now partially complete (D-17 done)**
**PKG-10 (Self-host) is now partially complete (D-31 done)**

**Next deliverables to work on (per SR-PLAN dependency graph):**
- D-16: Restricted evidence handling (depends on D-14) - PKG-05
- D-18: Core API endpoints (depends on D-17, D-10, D-06, D-11) - PKG-06
- D-28: UI scaffold + OIDC login (depends on D-02, D-17) - PKG-09
- D-32: Build/init scripts (depends on D-31, D-09, D-16) - PKG-10

---

### Session 5 (2026-01-13)
**Completed:** D-11, D-12, D-13

**What was done:**

D-11: Projection builder for read models from event streams
- ProjectionBuilder with checkpoint-based incremental processing
- Event handlers for Loop, Iteration, Candidate, Run, Approval, Exception, Freeze, Decision, GovernedArtifact events
- Query helpers: LoopProjection, IterationProjection, CandidateProjection, RunProjection
- Added proj.checkpoints table for tracking rebuild state

D-12: Dependency graph projection + staleness traversal
- GraphProjection with node/edge management
- EdgeType enum with all SR-SPEC Appendix B types
- Transitive dependency queries (get_dependencies, get_dependents)
- Staleness propagation on artifact changes

D-13: Outbox publisher and NATS event publication
- OutboxWriter: writes events to outbox within transaction
- OutboxPublisher: reads from outbox and publishes to NATS
- Topic mapping per event type
- Hash-based idempotency

Commits: be4c3ed (D-11), 00ea2f0 (D-12), 1a9f4a0 (D-13)

**PKG-04 (Persistence, projections, graph) is now complete!**

---

### Session 4 (2026-01-13)
**Completed:** D-08, D-10

**What was done:**

D-08: Context compilation rules
- ContextCompiler with deterministic ref ordering (kind, id, rel)
- ItemClassification enum (Public, Internal, Restricted, Confidential)
- RedactionRecord for audit trail of redacted content
- RefSelector for work unit context selection and topological sorting
- Unit tests for determinism, redaction, cycle detection

D-10: PostgreSQL EventStore adapter
- PostgresEventStore implementing EventStore port
- Append-only streams with optimistic concurrency control
- read_stream and replay_all for deterministic event retrieval
- Stream kind inference, actor kind conversions
- Unit tests for conversions and stream handling

Commits: a388e5f (D-08), 5b9ca56 (D-10)

---

### Session 3 (2026-01-13)
**Completed:** D-04, D-05, D-06, D-07, D-09

**What was done:**

D-04: Local developer tooling
- Added scripts/check-deps.sh, dev-setup.sh, run-tests.sh
- Makefile provides `make dev`, `make test`, `make build` targets

D-05: Domain model primitives and invariants
- Added all domain entities: Iteration, Candidate, Run, EvidenceBundle, Approval, FreezeRecord, Exception, Decision
- Added entity identifiers, state enums, invariants with unit tests

D-06: Deterministic state machines
- Added IterationStateMachine, RunStateMachine, ExceptionStateMachine with transition validation
- Added VerificationComputer for verification status computation per SR-SPEC §3.3
- Added InvariantValidator for enforcing human actor requirements and waiver constraints
- Unit tests cover valid/invalid transitions

D-07: Ports and boundary interfaces
- sr-ports crate already implemented with: EventStore, EvidenceStore, OracleRunner, MessageBus, IdentityProvider, Clock traits
- Error types are explicit and suitable for deterministic handling

D-09: Postgres schemas and migrations
- 001_event_store.sql: Event store schema (es.*) with append-only enforcement, streams, events, outbox
- 002_projections.sql: All projections (loops, iterations, candidates, runs, governed_artifacts, decisions, approvals, freeze_records, exceptions, etc.)
- 003_graph.sql: Dependency graph (graph.*) with nodes, edges, staleness markers, utility functions for traversal

Commits: 57d1ba8 (D-04/D-05), 979e0c4 (README update), 1aeb3f0 (D-09)

---

### Session 2 (2026-01-13)
**Completed:** D-03 (Continuous integration baseline)

**What was done:**
- Created GitHub Actions CI workflow (.github/workflows/ci.yml)
- Rust job: format check, clippy lint, build, test with caching
- UI job: npm install, type-check, eslint, build
- Summary job: produces machine-readable JSON with pass/fail, artifact hashes
- Fixed Rust edition 2024 -> 2021 in Cargo.toml
- Added ESLint configuration for UI (ui/.eslintrc.cjs)
- Committed and pushed to solver-ralph-1 branch (commit 3692c0b)

---

### Session 1 (2026-01-13)
**Completed:** D-02 (Repository scaffold and workspace layout)

**What was done:**
- Created Rust workspace with 4 crates: sr-domain, sr-ports, sr-adapters, sr-api
- Implemented stub domain entities, events, state machines, commands, errors
- Created port traits for EventStore, EvidenceStore, OracleRunner, MessageBus, etc.
- Set up React/TypeScript UI scaffold with Vite (builds successfully)
- Created shared schemas directory structure
- Added Makefile with build/test/dev targets
- Committed and pushed to solver-ralph-1 branch (commit 608f083)


## Cross-document reference convention

When referencing another SR-* document:

- Use the **doc_id** (e.g., `SR-CONTRACT`, `SR-SPEC`)
- Resolve to physical path using the table below
- If multiple candidates exist (duplicates, forks, renamed copies), treat this table as authoritative and record any deviation via SR-EXCEPTIONS

---



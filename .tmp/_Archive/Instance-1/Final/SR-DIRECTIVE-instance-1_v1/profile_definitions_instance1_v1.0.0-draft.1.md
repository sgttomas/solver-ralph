# SR-DIRECTIVE Profile Definitions (Instance-1) — v1.0.0-draft.1
*Generated:* 2026-01-11
This artifact defines **verification profiles** and **oracle suites** referenced by the instance-1 Gate Registry.
## Verification Profiles
### profile:STRICT-CORE@v1
- Default verification profile for most candidate deliverables. Requires deterministic core checks (build/tests/lint/schema).
- Required suites: suite:STRICT-CORE@v1
- Advisory suites: (none)
- verification_mode_default: `STRICT`
- allow_with_exceptions: `False`

### profile:STRICT-FULL@v1
- High-assurance verification profile for deliverables that are on the critical path to end-to-end proofs or release readiness.
- Required suites: suite:STRICT-FULL@v1
- Advisory suites: suite:STOP-TRIGGERS@v1
- verification_mode_default: `STRICT`
- allow_with_exceptions: `True`

## Oracle Suites
### suite:STRICT-CORE@v1
- Core deterministic checks intended to be REQUIRED for most candidate deliverables: build, unit tests, formatting/lint, and schema validation.
- determinism_required: `True`
- network: `disabled`
- oracles:
  - `oracle:rust-build` (required) — Build all Rust workspace targets under a pinned toolchain and lockfile.
  - `oracle:rust-unit-tests` (required) — Run Rust unit tests deterministically for the workspace.
  - `oracle:rust-fmt` (required) — Enforce Rust formatting (no diffs).
  - `oracle:rust-clippy` (required) — Enforce Rust linting (deny warnings).
  - `oracle:schema-validate` (required) — Validate governed schemas (events/records) and local JSON schemas against SR-TYPES/SR-SPEC expectations.
  - `oracle:ui-build` (required) — Build the Portal/UI workspace (React) with pinned lockfile to ensure UI compile integrity.
  - `oracle:ui-unit-tests` (advisory) — Run UI unit tests (advisory by default; reclassify only via governance).

### suite:STRICT-FULL@v1
- Extended verification for high-stakes deliverables (integration, e2e, self-host boot, and replayability).
- determinism_required: `True`
- network: `restricted`
- oracles:
  - `oracle:strict-core-suite` (required) — Run the STRICT-CORE suite as a prerequisite.
  - `oracle:selfhost-smoke` (required) — Bring up the full stack and verify health checks and pinned versions.
  - `oracle:integration-suite` (required) — Run integration suite across DB/MinIO/NATS/API (and UI if applicable).
  - `oracle:e2e-suite` (required) — Run end-to-end suite exercising loop→iteration→candidate→oracles→approval→freeze (or representative subset).
  - `oracle:replayability-check` (required) — Replay a recorded event stream and assert deterministic reconstructed state checksum.
  - `oracle:security-scan` (advisory) — Run security scanning (advisory by default; treat HIGH/CRITICAL findings as stop triggers via SR-DIRECTIVE).

### suite:CTX-REFSET-VALIDATION@v1
- Context admissibility and reference-set validation for IterationStarted refs and deterministic context compilation.
- determinism_required: `True`
- network: `disabled`
- oracles:
  - `oracle:refs-required-present` (required) — Verify required refs are present (SAPS, SR-CONTRACT, SR-SPEC, SR-PLAN instance, SR-DIRECTIVE, SR-ETT) and properly typed.
  - `oracle:refs-dereferenceable` (required) — Dereference each depends_on ref and verify meta.content_hash matches fetched content.
  - `oracle:context-compiler-determinism` (required) — Run ContextCompiler twice from the same refs/payload and ensure identical ContextBundle content hash.

### suite:RUNTIME-SUBSTRATE@v1
- Runtime substrate verification for persistence primitives: event store invariants, projection rebuild determinism, and outbox publish semantics.
- determinism_required: `True`
- network: `restricted`
- oracles:
  - `oracle:strict-core-suite` (required) — Baseline STRICT-CORE checks prior to runtime substrate tests.
  - `oracle:eventstore-invariants` (required) — Verify event store is append-only, enforces optimistic concurrency, and preserves ordering per stream.
  - `oracle:projection-rebuild-determinism` (required) — Rebuild projections from the same event stream and assert identical projection checksums.
  - `oracle:outbox-publish-sim` (advisory) — Simulate outbox publication to NATS/JetStream and verify at-least-once semantics and idempotent consumer behavior.

### suite:STALENESS-GRAPH@v1
- Dependency graph + staleness traversal verification. Ensures staleness propagation is correct and Shippable blocks on unresolved staleness.
- determinism_required: `True`
- network: `restricted`
- oracles:
  - `oracle:dependency-graph-tests` (required) — Build dependency graph projection and validate edge semantics for depends_on vs supported_by.
  - `oracle:staleness-propagation-tests` (required) — Validate staleness propagation rules on governed artifact changes and dependency updates.
  - `oracle:shippable-blocks-on-stale` (required) — Assert candidates with unresolved staleness cannot be Shippable=true.

### suite:EVIDENCE-INTEGRITY@v1
- Evidence ingestion, manifest validation, content-address verification, and retrievability checks for evidence bundles.
- determinism_required: `True`
- network: `restricted`
- oracles:
  - `oracle:evidence-manifest-validate` (required) — Validate evidence bundle manifest schema and required fields.
  - `oracle:evidence-content-hash-verify` (required) — Verify each artifact listed is retrievable and matches its content hash.
  - `oracle:evidence-retrievability-walk` (required) — Walk a referenced evidence bundle and ensure all artifacts can be fetched under declared access policy.
  - `oracle:evidence-missing-negative-test` (advisory) — Negative-path: simulate missing evidence and confirm EVIDENCE_MISSING stop trigger.

### suite:STOP-TRIGGERS@v1
- Oracle integrity and stop-trigger verification (TAMPER/GAP/FLAKE/ENV_MISMATCH) plus correct routing to portals without silent override.
- determinism_required: `True`
- network: `restricted`
- oracles:
  - `oracle:tamper-detection-test` (required) — Simulate evidence hash mismatch and confirm ORACLE_TAMPER is emitted and blocks progression.
  - `oracle:gap-detection-test` (required) — Simulate missing required oracle result and confirm ORACLE_GAP is emitted and blocks progression.
  - `oracle:flake-detection-test` (required) — Run a purposely non-deterministic oracle twice and confirm ORACLE_FLAKE is emitted.
  - `oracle:env-mismatch-detection-test` (required) — Simulate environment constraint mismatch and confirm ORACLE_ENV_MISMATCH is emitted.
  - `oracle:trigger-routing-test` (required) — Verify stop triggers route to the correct portal touchpoints and that no silent override occurs.

### suite:PORTAL-WORKFLOWS@v1
- Portal/gov API and UI workflow verification for approvals, exceptions, oracle suite changes, and freeze submission semantics.
- determinism_required: `True`
- network: `restricted`
- oracles:
  - `oracle:portal-api-contract-tests` (required) — Verify portal/gov API endpoints implement SR-SPEC semantics for approvals/exceptions/freezes/shippable computation.
  - `oracle:exception-acknowledgement-check` (required) — Ensure approvals and freezes explicitly acknowledge active exceptions (even if empty).
  - `oracle:portal-ui-workflow-tests` (advisory) — UI workflow tests for portal submissions (advisory if UI flakiness is observed).

### suite:SELFHOST-SMOKE@v1
- Self-host smoke suite: boot stack, health checks, idempotent init, and version pinning evidence.
- determinism_required: `True`
- network: `restricted`
- oracles:
  - `oracle:selfhost-boot` (required) — Start self-host stack and wait for readiness.
  - `oracle:selfhost-health` (required) — Run health checks for core services (API, DB, MinIO, NATS, Zitadel).
  - `oracle:versions-pinned-report` (required) — Emit a machine-readable report of pinned service versions/digests.

### suite:E2E-HAPPY@v1
- End-to-end 'happy path' harness execution: loop→iteration→candidate→oracles→approval→freeze.
- determinism_required: `True`
- network: `restricted`
- oracles:
  - `oracle:e2e-happy-harness` (required) — Run the happy-path harness and record a full transcript with object IDs.
  - `oracle:freeze-validated-post-harness` (required) — Validate the produced freeze record and approval linkage from the harness run.

### suite:E2E-FAILURE@v1
- End-to-end 'failure path' harness execution: inject integrity failures, exercise waiver/exception flows, and prove stop-the-line behavior.
- determinism_required: `True`
- network: `restricted`
- oracles:
  - `oracle:e2e-failure-harness` (required) — Run failure-mode harness with deterministic fault injection and record transcripts + portal submissions.
  - `oracle:nonwaivable-integrity-enforced` (required) — Prove integrity conditions cannot be waived and always stop the line.

### suite:REPLAYABILITY@v1
- Replayability and determinism proof: replay event stream and verify reconstructed state/projections match checksum and invariants.
- determinism_required: `True`
- network: `disabled`
- oracles:
  - `oracle:replay-event-stream` (required) — Replay recorded event stream into empty store and rebuild all projections.
  - `oracle:replay-checksum-compare` (required) — Compare reconstructed projection checksums against baseline and assert equality.

### suite:FREEZE-VALIDATION@v1
- Freeze gating validation: freeze record completeness, explicit exception acknowledgement, no unresolved staleness, and Shippable computation correctness.
- determinism_required: `True`
- network: `disabled`
- oracles:
  - `oracle:freeze-record-completeness` (required) — Validate FreezeRecordCreated contains required fields and enumerates governed artifacts + evidence pointers.
  - `oracle:approval-acknowledges-exceptions` (required) — Ensure ReleaseApprovalPortal approval explicitly acknowledges active exceptions (including empty list).
  - `oracle:staleness-none-check` (required) — Ensure no unresolved staleness markers affect the candidate, its dependencies, the oracle suite, or governed artifacts.
  - `oracle:shippable-computation-check` (required) — Compute Shippable per SR-SPEC and assert Shippable=true only when all preconditions hold.

## Gate → Suite/Profile binding (from Gate Registry v1.0.0-draft.1)
- **G-00** → suite:CTX-REFSET-VALIDATION@v1
- **G-10** → suite:RUNTIME-SUBSTRATE@v1
- **G-15** → suite:STALENESS-GRAPH@v1
- **G-20** → suite:EVIDENCE-INTEGRITY@v1
- **G-30** → profile:STRICT-CORE@v1
- **G-31** → profile:STRICT-FULL@v1
- **G-40** → suite:STOP-TRIGGERS@v1
- **G-50** → suite:PORTAL-WORKFLOWS@v1
- **G-60** → suite:SELFHOST-SMOKE@v1
- **G-70** → suite:E2E-HAPPY@v1
- **G-71** → suite:E2E-FAILURE@v1
- **G-80** → suite:REPLAYABILITY@v1
- **G-90** → suite:FREEZE-VALIDATION@v1 + ReleaseApprovalPortal

## TODO before operational use
- Pin `oci_image_digest` for each suite to an actual OCI image digest.
- Replace `make …` / `srctl …` placeholder commands with implemented scripts.
- Ensure required oracles are deterministic; otherwise reclassify via governance change.

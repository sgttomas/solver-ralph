# SR-PLAN-V12 Consistency Review

**Reviewer:** Agent (solver-ralph-12 branch)
**Review Date:** 2026-01-17
**Document Under Review:** `docs/planning/SR-PLAN-V12.md`
**Prior Review:** `docs/reviews/SR-PLAN-V12-COHERENCE-REVIEW.md` (codebase coherence)
**Status:** Complete

---

## Executive Summary

SR-PLAN-V12 has been analyzed for consistency with the canonical SR-* documents across three dimensions: ontology, epistemology, and semantics. The plan proposes operational refinements to three partial deliverables (D-15, D-21, D-22) that formalize and harden existing functionality.

**Verdict: APPROVE**

The plan is consistent with the canonical SR-* documents. All proposed changes align with SR-CONTRACT invariants, SR-SPEC mechanics, SR-TYPES vocabulary, and SR-SEMANTIC-ORACLE-SPEC oracle interface requirements. Minor recommendations are provided to strengthen alignment.

---

## 1. Ontology Findings (SR-TYPES Alignment)

### O-1: V12-1 Evidence Manifest Validation Oracle — ALIGNED

**V12-1 proposes:** Package existing `EvidenceManifest::validate()` as a containerized oracle producing `sr.oracle_result.v1` output.

**SR-TYPES §4.3 and §7.3 define:**
- `domain.evidence_bundle` — Oracle verification output
- Evidence Bundles contain `results[]` with per-oracle results

**SR-SPEC §1.9.1.1 defines:**
- `sr.oracle_result.v1` schema with required fields: `schema`, `oracle_id`, `status`, timestamps, `duration_ms`, `exit_code`, `summary`

**Finding:** The proposed oracle output schema (`sr.oracle_result.v1`) is the canonical format for oracle results per SR-SPEC. The oracle validates `evidence.gate_packet` manifests which is the correct artifact type per SR-SPEC §1.9.1.

**Severity:** N/A (aligned)

---

### O-2: V12-2 Message Contract Types — ALIGNED

**V12-2 proposes:** Externalize `MessageEnvelope` schema to `schemas/messaging/SR-MESSAGE-CONTRACTS.md`

**SR-CONTRACT §2.11 (Canonical Terminology Mapping) establishes:**
- Canonical terms for cross-document consistency in schemas and API signatures

**SR-TYPES §3.1 (Metadata Standard) establishes:**
- Schema requirements for governed artifacts

**Finding:** `MessageEnvelope` is an implementation-internal type (adapter layer), not a platform domain type. Externalizing its schema as documentation is appropriate. The type does not need to be registered in SR-TYPES §4 as it is infrastructure, not domain semantics.

**Severity:** N/A (aligned)

---

### O-3: V12-3 Governor Service Types — ALIGNED

**V12-3 proposes:** Extract `LoopGovernor` into standalone service binary

**SR-TYPES §4.3 defines:**
- `domain.work_unit` — State machine for tracked work
- `domain.loop_record` — Ralph-loop iteration summary

**SR-SPEC §3.1 (Ralph Loop lifecycle) defines:**
- States: `CREATED`, `ACTIVE`, `PAUSED`, `CLOSED`
- Transitions via events: `LoopCreated`, `LoopActivated`, `IterationStarted`, etc.

**Finding:** The governor service operates on existing domain types. No new domain types are introduced. The proposal correctly identifies that `IterationStarted` must be emitted with `actor_kind=SYSTEM` per SR-SPEC §2.2.

**Severity:** N/A (aligned)

---

## 2. Epistemology Findings (SR-CONTRACT Alignment)

### E-1: V12-1 Manifest Validation — ALIGNED WITH C-EVID-1

**V12-1 proposes:** Oracle validates evidence manifests for required fields, timestamp ordering, verdict consistency

**SR-CONTRACT C-EVID-1 requires Evidence Bundles include:**
- Candidate reference (Yes)
- Oracle suite hash (Yes)
- Governed artifact references (Yes)
- Exception references (Yes)
- Per-oracle results (Yes)
- Attribution (Yes)
- Content hash (Yes)

**Finding:** The proposed validation oracle directly enforces C-EVID-1 requirements. The oracle produces evidence about conformance (verification) per the SR-CONTRACT verification/validation distinction.

**Severity:** N/A (aligned)

---

### E-2: V12-1 Oracle Determinism — ALIGNED WITH C-OR-1

**V12-1 proposes:** Oracle produces deterministic output for same input

**SR-CONTRACT C-OR-1 requires:**
> Required oracles MUST be deterministic within declared environment constraints.

**Finding:** The manifest validation oracle is inherently deterministic — same manifest JSON produces same validation result. No external dependencies or non-deterministic operations are involved.

**Severity:** N/A (aligned)

---

### E-3: V12-2 Contract Documentation — SUPPORTS C-EVID-4

**V12-2 proposes:** Document message contracts with JSON Schema for polyglot consumers

**SR-CONTRACT C-EVID-4 requires:**
> Evidence MUST include typed references for dependency queries.

**SR-SPEC §4.6 (Message bus pattern) states:**
> The bus is an adapter. No domain invariant depends on NATS being present.

**Finding:** Externalizing message contracts supports interoperability and auditability without introducing new domain invariants. The bus remains an adapter; contract documentation does not elevate it to domain semantics.

**Severity:** N/A (aligned)

---

### E-4: V12-3 Governor Decisions — ALIGNED WITH C-LOOP-1, C-LOOP-2

**V12-3 proposes:** Governor records all decisions as events, respects budgets, is idempotent

**SR-CONTRACT C-LOOP-1 requires:**
> Agentic iteration MUST be bounded by explicit budgets. BUDGET_EXHAUSTED requires human decision.

**SR-CONTRACT C-LOOP-2 requires:**
> Each Iteration SHOULD operate in fresh context... The system MUST record what was attempted, candidates produced, evidence produced, stop triggers fired.

**Finding:** The governor enforces budgets and records decisions as events, satisfying C-LOOP-1 and C-LOOP-2. Idempotency prevents duplicate iterations, which supports the "no duplicate logical state transitions" requirement from SR-SPEC §4.6.

**Severity:** N/A (aligned)

---

### E-5: V12-3 SYSTEM Actor for IterationStarted — ALIGNED WITH C-CTX-1

**V12-3 proposes:** Governor emits `IterationStarted` events via NATS

**SR-CONTRACT C-CTX-1 requires:**
> `IterationStarted.actor_kind` MUST be `SYSTEM`.

**SR-SPEC §2.2 states:**
> `POST /loops/{loop_id}/iterations` MUST be callable only by a SYSTEM service.
> The emitted `IterationStarted` event MUST have `actor_kind=SYSTEM`.

**Finding:** The standalone governor service operates as a SYSTEM actor, satisfying the C-CTX-1 requirement that iteration creation is SYSTEM-mediated.

**Severity:** N/A (aligned)

---

## 3. Semantics Findings (SR-SPEC Alignment)

### S-1: V12-1 Oracle Output Path — ALIGNED

**V12-1 proposes:** Oracle outputs to `reports/manifest-validation.json` (implied from pattern)

**SR-SPEC §1.9.1.1 states:**
> The `PodmanOracleRunner` collects these outputs from `/scratch/reports/<name>.json`

**Existing pattern (schema-validation.sh):**
- Output to `${REPORTS_DIR}/schema.json` (i.e., `/scratch/reports/schema.json`)

**Finding:** The proposed oracle follows the established output path convention. The manifest should be added to `oracle-suites/core-v1/suite.json` with `expected_outputs[].path: "reports/manifest-validation.json"`.

**Severity:** N/A (aligned)

---

### S-2: V12-1 Oracle Classification — REQUIRES CLARIFICATION

**V12-1 proposes:** Manifest validation oracle in `oracle-suites/core-v1`

**SR-SPEC §1.9.1 (Evidence bundle manifest) requires:**
- `classification: "required"` for oracles that participate in Verified claims

**Existing core-v1 suite has:**
- `oracle:build` — `classification: required`
- `oracle:unit-tests` — `classification: required`
- `oracle:schema-validation` — `classification: required`
- `oracle:lint` — `classification: advisory`

**Finding:** The plan should clarify whether `manifest-validation` is `required` or `advisory`. Given it validates evidence integrity (C-EVID-1), it should likely be `required` for evidence-consuming workflows.

**Recommendation:** Explicitly state `classification: required` in the V12-1 tasks.

**Severity:** Low (clarification needed)

---

### S-3: V12-2 MessageEnvelope Schema — ALIGNED WITH SR-SPEC §1.5.2

**V12-2 proposes:** Document `MessageEnvelope` with fields including `schema_version`, `message_type`, `message_id`, `correlation_id`, `causation_id`, `timestamp`, `actor_id`, `actor_kind`, `payload`, `idempotency_key`

**SR-SPEC §1.5.2 (Event envelope) requires:**
- `event_id`, `stream_id`, `stream_kind`, `stream_seq`, `event_type`, `occurred_at`, `actor_kind`, `actor_id`, `correlation_id`, `causation_id`, `refs[]`, `payload`

**Finding:** `MessageEnvelope` is a message bus wrapper around domain events, not the event envelope itself. The schema correctly includes actor attribution and correlation tracking. The distinction between event envelope (persistence) and message envelope (transport) is appropriate.

**Severity:** N/A (aligned)

---

### S-4: V12-2 Stream Names — ALIGNED WITH SR-SPEC §4.6

**V12-2 proposes:** Document streams: `sr-events`, `sr-commands`, `sr-queries`

**SR-SPEC §4.6 states:**
> The outbox publisher MUST publish each committed event to NATS subjects: `sr.events.<stream_kind>.<event_type>`

**Existing implementation (`nats.rs`):**
- `streams::EVENTS`, `streams::COMMANDS`, `streams::QUERIES`
- Subject patterns: `sr.events.loop`, `sr.events.iteration`, etc.

**Finding:** Stream naming and subject patterns are consistent with SR-SPEC §4.6.

**Severity:** N/A (aligned)

---

### S-5: V12-3 Governor Polling Model — IMPLEMENTATION DETAIL

**V12-3 proposes:**
- Poll interval (default: 1s)
- Max concurrent loops to process
- Enable/disable dry-run mode

**SR-SPEC §1.7.3 (Event Manager) states:**
> Implementation details MAY vary (synchronous or asynchronous projection), but the computed results MUST be deterministic functions of the event stream + governed inputs.

**Finding:** The polling model is an implementation detail not constrained by SR-SPEC. The governor's decisions must be deterministic functions of loop state (derived from events), which the existing implementation satisfies.

**Severity:** N/A (implementation detail, not constrained)

---

### S-6: V12-3 API /start Endpoint Compatibility — ALIGNED

**V12-3 proposes:** API `/start` endpoint continues to work without governor service

**SR-SPEC §2.3.12 (`POST /work-surfaces/{work_surface_id}/start`) states:**
> Creates Loop bound to `work_unit_id`, activates Loop, and starts Iteration as SYSTEM actor.

**V12-3 constraints:**
> Governor must be **optional** — API `/start` endpoint continues to work without governor service

**Finding:** Making the governor optional preserves the API's ability to function as specified. This is a deployment configuration, not a semantic change.

**Severity:** N/A (aligned)

---

### S-7: V12-3 Docker Compose Integration — OPERATIONAL

**V12-3 proposes:** Add `sr-governor` service to `deploy/docker-compose.yml`

**SR-SPEC §5.2 (Service configuration) mentions:**
- PostgreSQL, MinIO, NATS as infrastructure dependencies

**Finding:** Adding a governor service to docker-compose is an operational concern. SR-SPEC permits additional services beyond the minimum infrastructure.

**Severity:** N/A (operational, not semantically constrained)

---

## 4. Summary of Findings

| ID | Dimension | Severity | Summary |
|----|-----------|----------|---------|
| O-1 | Ontology | N/A | Manifest validation oracle uses correct `sr.oracle_result.v1` schema |
| O-2 | Ontology | N/A | MessageEnvelope is implementation-internal; external docs appropriate |
| O-3 | Ontology | N/A | Governor operates on existing domain types; no new types introduced |
| E-1 | Epistemology | N/A | Manifest validation enforces C-EVID-1 requirements |
| E-2 | Epistemology | N/A | Validation oracle is inherently deterministic (C-OR-1) |
| E-3 | Epistemology | N/A | Message contract docs support interoperability without domain changes |
| E-4 | Epistemology | N/A | Governor enforces budgets, records decisions (C-LOOP-1, C-LOOP-2) |
| E-5 | Epistemology | N/A | Governor emits IterationStarted as SYSTEM (C-CTX-1) |
| S-1 | Semantics | N/A | Oracle output path follows established convention |
| S-2 | Semantics | Low | Oracle classification should be explicitly stated |
| S-3 | Semantics | N/A | MessageEnvelope schema is transport wrapper, not event envelope |
| S-4 | Semantics | N/A | Stream names consistent with SR-SPEC §4.6 |
| S-5 | Semantics | N/A | Polling model is implementation detail, not constrained |
| S-6 | Semantics | N/A | Optional governor preserves API compatibility |
| S-7 | Semantics | N/A | Docker compose integration is operational |

---

## 5. Recommendations

### Should Address (Low Severity)

1. **S-2:** Explicitly state `classification: required` for the manifest-validation oracle in V12-1 tasks, given its role in enforcing C-EVID-1.

### Optional Enhancements

2. **V12-1:** Consider adding the manifest-validation oracle to the Integration Suite (`SR-SUITE-INTEGRATION`) as well as the Core Suite, since evidence validation is relevant for integration testing.

3. **V12-2:** Include schema evolution guidelines in `SR-MESSAGE-CONTRACTS.md` to address how `SCHEMA_VERSION` changes should be handled by consumers.

4. **V12-3:** Consider adding Prometheus metrics export from the governor service to align with the observability patterns established in V11-3 (`/ready` endpoint, domain metrics).

---

## 6. Verdict

### **APPROVE**

SR-PLAN-V12 is consistent with the canonical SR-* documents. The plan proposes operational refinements that:

1. **V12-1** — Correctly packages validation logic as an oracle following SR-SPEC oracle interface patterns
2. **V12-2** — Documents implementation-internal contracts without introducing domain semantic changes
3. **V12-3** — Extracts governor logic while preserving SR-CONTRACT invariants (SYSTEM actor, budget enforcement)

No high-severity inconsistencies were identified. The single low-severity finding (S-2) is a clarification, not a blocking issue.

The plan can proceed to implementation.

---

## Appendix: Documents Consulted

| Document | Sections Referenced |
|----------|---------------------|
| SR-PLAN-V12 | §1-8 (all phases) |
| SR-CONTRACT | §2.3, §2.6, §2.11, C-EVID-1, C-EVID-4, C-OR-1, C-LOOP-1, C-LOOP-2, C-CTX-1 |
| SR-TYPES | §1.1-1.3, §4.3, §4.4, §7.3 |
| SR-SPEC | §1.5.2, §1.7.3, §1.9.1, §1.9.1.1, §2.2, §2.3.12, §3.1, §4.6, §5.2 |
| SR-SEMANTIC-ORACLE-SPEC | §2, §3, §4 |
| oracle-suites/core-v1/suite.json | Oracle definition patterns |
| oracle-suites/core-v1/oracles/schema-validation.sh | Oracle implementation patterns |
| crates/sr-adapters/src/nats.rs | MessageEnvelope, streams, subjects |
| crates/sr-adapters/src/governor.rs | LoopGovernor, LoopBudget, StopCondition |

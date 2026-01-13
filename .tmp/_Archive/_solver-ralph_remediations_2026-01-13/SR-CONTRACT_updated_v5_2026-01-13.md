---
doc_id: SR-CONTRACT
doc_kind: governance.arch_contract
layer: platform
status: draft
refs:
- rel: governed_by
  to: SR-CHANGE
- rel: depends_on
  to: SR-TYPES
- rel: informs
  to: SR-SPEC
- rel: informs
  to: SR-DIRECTIVE
- rel: informs
  to: SR-INTENT
- rel: informs
  to: SR-WORK-SURFACE
- rel: informs
  to: SR-SEMANTIC-ORACLE-SPEC
- rel: informs
  to: SR-EVENT-MANAGER
- rel: informs
  to: SR-AGENT-WORKER-CONTRACT
---

# SOLVER-Ralph Architectural Contract 

**Purpose:** Define the binding invariants that SOLVER-Ralph (the platform) MUST enforce. This contract constrains what implementations may do, regardless of language, framework, storage, or model provider.

**Normative Status:** **Normative (binding).** Highest binding precedence among the governed set.

**Interpretation Guidance:** Consult **SR-INTENT** for rationale and tradeoffs. Consult **SR-TYPES** for artifact taxonomy and metadata conventions.

---

## 0.1 Contract as Platform Specification

This contract specifies **Layer 2** of SOLVER-Ralph: the invariants that the running platform enforces.


---

## 1. Scope

This contract is **implementation-agnostic**. It defines:

- authority boundaries (who can do what),
- verification semantics (what "verified" means),
- candidate/run binding and oracle integrity requirements,
- audit and traceability invariants,
- exception handling invariants,
- required architectural separation (hexagonal boundary).

This contract does **not** define:

- concrete API schemas, database schemas, or command lines (belongs in SR-SPEC),
- exact phase plans, gate names, or command targets (belongs in SR-DIRECTIVE),
- operational playbooks (Directive or adjacent artifacts).

```
┌─────────────────────────────────────────────────────────────────┐
│  LAYER 1: Building SOLVER-Ralph                                 │
│                                                                  │
│  This contract constrains agents building the platform.         │
│  The contract is a specification document at this layer.        │
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ becomes
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  LAYER 2: The Platform (what this contract specifies)           │
│                                                                  │
│  The invariants in this contract become ENFORCEMENT CODE.       │
│  The platform embodies these constraints; it doesn't read this  │
│  document at runtime.                                           │
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ enables
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  LAYER 3: Usage                                                 │
│                                                                  │
│  Users interact with the platform.                               │
│  They experience these invariants as platform behavior.         │
└─────────────────────────────────────────────────────────────────┘
```

**Implication:** Every invariant (C-*) must be either:
- Enforced by code in the domain core, or
- Verified by oracles and/or portal review

If an invariant cannot be enforced or verified, it is not a valid contract requirement.

---

## 1.1 Invariants Index (navigation)

- **Architecture**: C-ARCH-1, C-ARCH-2, C-ARCH-3
- **Trust boundaries / portals / approvals**: C-TB-1, C-TB-2, C-TB-3, C-TB-4, C-TB-5, C-TB-6, C-TB-7
- **Verification states**: C-VER-1, C-VER-2, C-VER-3, C-VER-4
- **Shippable**: C-SHIP-1
- **Oracle suite integrity**: C-OR-1, C-OR-2, C-OR-3, C-OR-4, C-OR-5, C-OR-6, C-OR-7
- **Evidence integrity**: C-EVID-1, C-EVID-2, C-EVID-3, C-EVID-4, C-EVID-5, C-EVID-6
- **Event / audit / dependency graph**: C-EVT-1, C-EVT-2, C-EVT-3, C-EVT-4, C-EVT-5, C-EVT-6, C-EVT-7
- **Ralph Loop + iteration context**: C-LOOP-1, C-LOOP-2, C-CTX-1, C-CTX-2, C-LOOP-3, C-LOOP-4
- **Exceptions**: C-EXC-1, C-EXC-2, C-EXC-3, C-EXC-4, C-EXC-5
- **Decisions**: C-DEC-1
- **Metadata and typing**: C-META-1, C-META-2, C-META-3

---

## 2. Definitions

### 2.1 Standing and Claims

SOLVER-Ralph separates **evidence** from **authority**.

- **Evidence:** recorded outputs (oracle logs, scans, checklists) that support a claim.
- **Verified (Strict):** required deterministic oracles PASS, with complete evidence and no unresolved integrity conditions.
- **Verified-with-Exceptions:** required oracles executed; one or more FAILED with explicit, human-approved Gate Waiver.
- **Approved:** a human authorizes a Candidate at a **Portal** with reference to evidence and any waivers/exceptions.
- **Shippable:** Verified + Approved at Release Portal + Freeze Record complete.

**Note:** "Verified" is not metaphysical correctness; it is a bounded, evidence-grounded claim.

### 2.2 Actors

- **Human:** can approve and make binding decisions.
- **Agent:** can propose and iterate; cannot approve trust boundary actions.
- **System:** orchestrates and records; cannot substitute for human approval.

### 2.3 Candidate, Run, Ralph Loop, Iteration

- **Candidate:** a content‑addressable snapshot of work products (code *or* knowledge artifacts). A Candidate MAY be a VCS commit, a bundle of files, or an artifact manifest with content hashes. The unit of verification and approval.
- **Run:** execution of a declared oracle suite against a Candidate (and, when applicable, a declared *procedure stage*), producing an Evidence Bundle.
- **Ralph Loop:** a bounded workflow instance (a Work Unit) with goal, budgets, stop triggers, and controlled memory. A Ralph Loop operates over a **Work Surface** (Intake + Procedure) and advances work through stage‑gated candidate generation and oracle‑backed evidence.
- **Iteration:** one fresh‑context cycle within a Ralph Loop. An Iteration typically targets a single procedure stage or sub‑goal, and MUST record what was attempted, what candidate was produced/updated, what evidence was produced, and what stop triggers (if any) fired.

Additional semantic-work definitions (normative for interpretation of the platform terms above):

- **Work Surface:** the governed artifacts that define *what* is being worked on and *how*: Intake (problem statement decomposition for the work unit), a Procedure Template (stage-gated), and the selected oracle profile/suites (including any semantic set definitions used by those suites).
- **Procedure Stage:** a named gate in a Procedure Template. Each stage defines required intermediate artifacts and required oracle suites for progressing beyond that stage.
- **Semantic Ralph Loop:** a Ralph Loop whose primary candidates are semantic artifacts (documents, structured representations, decision records, analyses). It does not assume “tests” exist; instead it relies on stage-gated procedures and semantic oracle suites.

**Naming normalization:** The canonical surface form is `semantic-ralph-loop` (hyphenated, lowercase). Canonical cross-document term normalization is defined in **§2.11**, and **SR-TYPES** defines the authoritative `type_key` namespaces.

### 2.4 Oracles, Oracle Suites, Environment Constraints

- **Oracle:** a procedure that produces attributable evidence about a Candidate’s conformance to declared constraints. An oracle MUST produce a structured result record; it MAY also produce a binary PASS/FAIL outcome derived from that record under declared decision rules.
  - *Note:* Semantic oracles MAY emit measurements (e.g., residual vectors, coverage metrics, constraint violations) that are subsequently mapped to PASS/FAIL by a gate rule.
- **Required Oracle:** participates in determining Verified claim (Strict/With-Exceptions). Missing required oracle output is an integrity gap.
- **Advisory Oracle:** recorded for audit/diagnostics but does not block Verified claim.
- **Oracle Suite:** a named set of oracle definitions with stable identity and hash. For semantic oracles, the suite identity/hash MUST incorporate any semantic-set / meaning-matrix definitions that materially affect evaluation.
- **Environment Constraints:** declared context required for oracle determinism (or bounded nondeterminism) and replayability.

### 2.5 Integrity Conditions

Contract-mandated minimum set:

- **ORACLE_TAMPER:** suite definition changed during run.
- **ORACLE_GAP:** required oracle has no recorded result.
- **ORACLE_FLAKE:** required oracle non-deterministic for identical inputs.
- **ORACLE_ENV_MISMATCH:** run violated declared environment constraints.
- **EVIDENCE_MISSING:** referenced evidence cannot be retrieved.

### 2.6 Gates, Portals, Approvals, Freeze, Waivers

- **Gate:** checkpoint with criteria and evidence requirements.
- **Portal:** gate requiring human arbitration; every Portal is a trust boundary.
- **Approval:** binding, attributable record of human decision at Portal.
- **Evidence Bundle:** structured evidence artifact from a Run.
- **Freeze Record:** binding baseline snapshot.
- **Gate Waiver:** binding permission to proceed despite FAIL.

### 2.7 Exceptions and Decisions

- **Deviation:** binding exception from a governed requirement.
- **Deferral:** binding postponement.
- **Decision:** binding human judgment resolving escalation.

### 2.8 Proposal vs Commitment Object

- **Proposal:** any content whose meaning is not yet stabilized as a durable object. Proposals are **non-authoritative**.
- **Commitment Object:** durable, content-addressed, referenceable object safe for downstream reliance.

**Rule:** Binding claims (Verified/Approved/Shippable) MUST be derivable from commitment objects. Proposals MUST be treated as non-authoritative.

### 2.9 Verification, Evaluation, Validation, Assessment

- **Verification (oracle):** evidence about conformance. Output is evidence, not authority.
- **Evaluation (human):** interpretation of verification evidence. Non-binding unless elevated to Approval.
- **Validation (oracle):** evidence about fitness. Output is evidence, not authority.
- **Assessment (human):** interpretation of validation evidence. Non-binding unless elevated to Approval.

**Important:** Evaluation/Assessment are not substitutes for Portal Approval.

### 2.10 Governed Set vs Deployment

- **Governed set:** the SR-* documents that specify the platform.
- **Deployment:** a running installation of the platform that enforces the governed set.

The contract specifies invariants. The deployment enforces them as code.

### 2.11 Canonical Terminology Mapping

To prevent drift between documents, schemas, and code, the following terms are canonical and must be used consistently:

| Canonical Term | Aliases / Prior Usage | Definition Reference |
|----------------|----------------------|---------------------|
| `semantic-ralph-loop` | Semantic Ralph Loop, Work Unit (Semantic Loop), Ralph Loop | §2.3 |
| `evidence_bundle` | evidence packet, evidence artifact | §2.6 |
| `commitment_object` | record (when instantiated) | §2.8 |
| `proposal` | draft, non-binding output | §2.8 |
| `approval` | portal approval, approval record | §2.6 |
| `freeze_record` | baseline snapshot | §2.6 |
| `gate_waiver` | waiver, exception (when gate-specific) | §2.6 |
| `deviation` | exception (when requirement-scoped) | §2.7 |
| `deferral` | postponement | §2.7 |
| `portal` | trust boundary, authority port | §2.6 |
| `work_surface` | intake + procedure + oracle profile | §2.3 |

**Interpretation note (canonical tokens vs SR-TYPES type keys):**

- The **Canonical Term** column defines *surface-form tokens* for cross-document consistency in **prose** and in **schema field names / API signatures**.
- These tokens are **not** SR-TYPES `type_key` values. Type keys are governed by **SR-TYPES** and use dotted namespaces (e.g., `domain.evidence_bundle`, `record.deviation`).
- When an artifact is persisted, it MUST carry the appropriate `meta.type_key` (SR-SPEC) and/or manifest `artifact_type`.

**Where the mapping matters (common cases):**
- `evidence_bundle` ↔ `domain.evidence_bundle` (domain object) ↔ manifest `artifact_type = evidence.gate_packet`
- `work_surface` ↔ `domain.work_surface`
- `approval` ↔ `domain.portal_decision`
- `freeze_record` ↔ `record.freeze`
- `deviation`/`deferral`/`gate_waiver` ↔ `record.deviation`/`record.deferral`/`record.waiver`

**Rule:** Schemas, code, and governed documents MUST use canonical terms. Aliases are permitted in prose for readability but MUST NOT appear in type names, field names, or API signatures.

---

## 3. Architectural Structure

### C-ARCH-1: Hexagonal Separation

**Requirement:** The system MUST be architected as a hexagon:

- **domain core** containing authoritative rules and state machines,
- **driving ports** for commands/queries into the domain,
- **driven ports** for domain dependencies (storage, execution),
- **adapters** that implement ports and may be swapped.

**Verified by:** Oracle (architecture conformance) + Portal (review).

### C-ARCH-2: Domain Purity

**Requirement:** The domain core MUST NOT depend on infrastructure concerns (database clients, network calls, LLM SDKs, filesystem). The domain MAY define interfaces/ports.

**Verified by:** Oracle (unit tests + static checks) + Portal (review).

### C-ARCH-3: Event Store as Source of Truth

**Requirement:** The system MUST use an append-only event store as source of truth.

- Derived stores MUST be reconstructible from the event log.
- The event store MUST be a driven port (adapter-swappable).

**Verified by:** Oracle (reconstruction tests) + Portal (review).

---

## 4. Trust Boundaries and Portals

### C-TB-1: Human-Only Binding Authority

**Requirement:** Any action that creates, removes, or changes binding authority MUST require a Human actor, including:

- approving/rejecting a Candidate at Release Portal,
- approving governance changes,
- approving Deviations, Deferrals, Gate Waivers,
- finalizing a Freeze Record,
- registering/modifying oracle suites,
- extending Ralph Loop budgets.

**Verified by:** Portal (approval record) + Oracle (authorization checks).

### C-TB-2: Non-Authoritative Agent Output

**Requirement:** Agent-generated statements MUST be treated as **non-authoritative proposals** unless backed by recorded oracle evidence and/or recorded human approvals.

**Verified by:** Oracle (no code path accepts agent claims as evidence) + Portal (review).

### C-TB-3: Portal Crossings Produce Approvals

**Requirement:** Every Portal crossing MUST produce a binding Approval record that is:

- attributable to a Human with stable identity,
- linked to the subject (Candidate and/or governance change),
- linked to Evidence Bundle(s) reviewed,
- linked to applicable active exceptions.

**Verified by:** Oracle (linkage validation) + Portal (audit).

### C-TB-4: Minimum Required Portals

**Requirement:** An implementation MUST define, at minimum:

1. **Governance Change Portal** for normative governance changes
2. **Release Approval Portal** for declaring Shippable

**Verified by:** Portal (review) + Oracle (presence checks).

### C-TB-5: Stable Actor Identity

**Requirement:** Actor identity for Humans MUST be verifiable, stable, and auditable.

**Verified by:** Oracle (identity format validation) + Portal (auth review).

### C-TB-6: Approval Record Minimum Fields

**Requirement:** Approval records MUST include:

- Portal identifier
- Decision (approved/rejected)
- Subject reference(s)
- Evidence reviewed
- Exceptions acknowledged
- Approver identity (satisfying C-TB-5)
- Timestamp

**Verified by:** Oracle (schema validation) + Portal (audit).

### C-TB-7: Evaluation and Assessment Are Not Approval

**Requirement:** Human Evaluation and Assessment (non-binding interpretations) MUST NOT be treated as substitutes for Portal Approval. Only Portal Approvals/Decisions create binding state.

**Verified by:** Oracle (state-transition constraints) + Portal (audit).

---

## 5. Verification Semantics

### C-VER-1: Verification Is Evidence-Based and Candidate-Bound

**Requirement:** A Candidate MAY be marked "Verified" only when:

1. Evidence Bundle exists for a Run against that Candidate
2. Evidence Bundle is attributable and integrity-checked
3. Evidence Bundle declares oracle suite identity/hash
4. All required oracles have recorded results
5. No unresolved integrity conditions
6. Candidate identity is stable and immutable

**Verified by:** Oracle (verification logic tests).

### C-VER-2: Verified (Strict)

**Requirement:** Verified (Strict) requires C-VER-1 + every required oracle PASS.

**Verified by:** Oracle (verification logic tests).

### C-VER-3: Verified-with-Exceptions

**Requirement:** Verified-with-Exceptions requires C-VER-1 + at least one required oracle FAIL + every FAIL covered by binding Gate Waiver.

**Verified by:** Oracle (waiver linkage) + Portal (waiver approval).

### C-VER-4: Verified Claims Must Declare Mode and Basis

**Requirement:** Every Verified claim MUST declare:

- mode (Strict / Verified-with-Exceptions)
- oracle suite identity and hash
- candidate identity
- waiver refs (if applicable)
- **verification scope** (at minimum: work_unit_id; and, for stage-gated procedures, the `stage_id` and `procedure_template_id` that the evidence pertains to)

**Verified by:** Oracle (metadata validation).

### C-SHIP-1: Shippable Requires Freeze + Approval + Verified

**Requirement:** Shippable requires:

1. Verified (Strict or With-Exceptions)
2. Human Approval at Release Portal
3. Finalized Freeze Record with governed artifacts, active exceptions, evidence refs, approval ref, candidate identity

**Verified by:** Oracle (linkage validation) + Portal (release audit).

---

## 6. Oracle Suite and Integrity

### C-OR-1: Required Oracles Must Be Deterministic

**Requirement:** Required oracles MUST be deterministic within declared environment constraints. Non-deterministic checks MUST be Advisory or Portal-reviewed.

**Verified by:** Oracle (flake detection) + Portal (classification).

### C-OR-2: Suite Pinning and Integrity

**Requirement:** Runs MUST pin oracle suite identity at start. If suite changes (hash mismatch), raise ORACLE_TAMPER.

**Verified by:** Oracle (hash comparison) + Portal (rebase approval).

### C-OR-3: Environment Constraints Must Be Declared

**Requirement:** Oracle suites MUST declare environment constraints. Violations raise ORACLE_ENV_MISMATCH.

**Verified by:** Oracle (env checks) + Portal (review).

### C-OR-4: Oracle Gaps Are Blocking

**Requirement:** Missing required oracle result raises ORACLE_GAP; no Verified claim until resolved.

**Verified by:** Oracle (completeness checks).

### C-OR-5: Oracle Flake Is Stop-the-Line

**Requirement:** Non-deterministic required oracle raises ORACLE_FLAKE; no Verified claim until resolved.

**Verified by:** Oracle (flake classification) + Portal (reclassification approval).

### C-OR-6: No Silent Oracle Weakening

**Requirement:** The system MUST NOT silently disable required oracles, downgrade FAIL to PASS, suppress failures, or change classification without explicit governance and human approval.

**Verified by:** Oracle (policy checks) + Portal (review).

### C-OR-7: Oracle Integrity Conditions Halt and Escalate

**Requirement:** Integrity conditions MUST halt progression, record context, and route escalation.

**Verified by:** Oracle (condition detection) + Portal (resolution).

---

## 7. Evidence Integrity

### C-EVID-1: Evidence Bundle Minimum Manifest

**Requirement:** Evidence Bundles MUST include:

| Field | Required |
|---|---|
| Candidate reference | Yes |
| Oracle suite hash | Yes |
| Governed artifact references | Yes |
| Exception references | Yes |
| Per-oracle results | Yes |
| Attribution | Yes |
| Content hash | Yes |

**Verified by:** Oracle (schema validation) + Portal (audit).

### C-EVID-2: Evidence Immutability

**Requirement:** Evidence Bundles MUST be immutable, content-addressed, and protected against modification.

**Verified by:** Oracle (hash validation) + Portal (policy audit).

### C-EVID-3: Evidence Attribution

**Requirement:** Evidence MUST include actor type, stable identity, and timestamp.

**Verified by:** Oracle (field validation).

### C-EVID-4: Evidence Dependency References

**Requirement:** Evidence MUST include typed references for dependency queries.

**Verified by:** Oracle (presence checks) + Portal (audit).

### C-EVID-5: Evidence Containing Secrets

**Requirement:** Evidence with secrets requires restricted original + optional redacted copy + redaction manifest.

**Verified by:** Portal (secret handling audit).

### C-EVID-6: Evidence Availability

**Requirement:** Referenced evidence MUST remain retrievable. If not retrievable, record EVIDENCE_MISSING, treat affected claims as invalid, escalate.

**Verified by:** Oracle (retrievability checks) + Portal (policy audit).

---

## 8. Event, Audit, and Graph Model

### C-EVT-1: Event Attribution

**Requirement:** Every state-changing event MUST include actor type, stable identity, and timestamp.

**Verified by:** Oracle (schema validation).

### C-EVT-2: Append-Only Event Log

**Requirement:** Events MUST be append-only; corrections represented as new events.

**Verified by:** Portal (audit) + Oracle (adapter tests).

### C-EVT-3: Explicit Supersession

**Requirement:** Corrections MUST carry explicit supersedes/retracts references.

**Verified by:** Oracle (linkage tests) + Portal (audit).

### C-EVT-4: Sequence-First Ordering

**Requirement:** Event ordering MUST use monotonic sequence, not timestamps alone.

**Verified by:** Oracle (store tests).

### C-EVT-5: Event Graph References

**Requirement:** Events MUST include typed references supporting dependency queries and re-evaluation.

**Verified by:** Oracle (graph tests) + Portal (audit).

### C-EVT-6: Staleness Marking

**Requirement:** When referenced nodes change, system MUST mark dependents as potentially stale and route re-evaluation.

**Verified by:** Oracle (traversal tests) + Portal (audit).

### C-EVT-7: Projections Derivable From Audit Trail

**Requirement:** Derived projections MUST be reconstructible from the event log.

**Verified by:** Portal (architecture review) + Oracle (replay tests).

---

## 9. Ralph Loop Governance

### C-LOOP-1: Bounded Iteration With Hard Stop

**Requirement:** Agentic iteration MUST be bounded by explicit budgets. BUDGET_EXHAUSTED requires human decision to extend, terminate, or proceed.

**Verified by:** Oracle (governor tests) + Portal (audit).

### C-LOOP-2: Fresh-Context Iterations

**Requirement:** Each Iteration SHOULD operate in fresh context with controlled memory. The system MUST record what was attempted, candidates produced, evidence produced, stop triggers fired, and summary.

**Verified by:** Portal (architecture review) + Oracle (recording tests).

### C-CTX-1: Iteration Context Provenance

**Requirement:** The `IterationStarted` event MUST include `refs[]` constituting authoritative provenance for the iteration’s effective context, and `IterationStarted.actor_kind` MUST be `SYSTEM`.

For Semantic Ralph Loops (stage-gated knowledge work), this provenance MUST include references sufficient to reconstruct the **Work Surface** for the iteration, including:

- the Intake for the selected work unit
- the Procedure Template (and the current `stage_id`)
- the selected oracle suite(s) and their hashes (including any semantic-set / meaning-matrix definitions that materially affect evaluation)

**Verified by:** Oracle (schema validation) + Portal (review).

### C-CTX-2: No Ghost Inputs

**Requirement:** Iteration context MUST be derivable solely from the `IterationStarted` event payload and the dereferenced `refs[]`. Unrepresented inputs MUST NOT influence work.

For Semantic Ralph Loops, “context” includes not only documents and notes, but also the active Procedure Template stage and the semantic oracle semantic set definitions used for evaluation.

**Verified by:** Oracle (context compilation tests) + Portal (audit).

### C-LOOP-3: Mandatory Stop Triggers

**Requirement:** System MUST implement stop triggers for: ORACLE_TAMPER, ORACLE_GAP, ORACLE_ENV_MISMATCH, ORACLE_FLAKE, REPEATED_FAILURE (N≥3), BUDGET_EXHAUSTED.

**Verified by:** Oracle (trigger tests) + Portal (review).

### C-LOOP-4: Candidate Production Traceable

**Requirement:** Candidates MUST be materialized with stable identity and related to Loop, Iteration, Run(s), and Approval(s).

For Candidates that cannot be represented as a single VCS commit (e.g., general knowledge-work artifacts), the system MUST record a **Candidate Manifest** listing included artifacts with content hashes sufficient to re-identify the Candidate deterministically.

**Verified by:** Oracle (graph validation) + Portal (audit).

---

## 10. Exceptions

### C-EXC-1: Exceptions Are Records

**Requirement:** Sustained mismatches between governed requirements and actual behavior MUST be recorded as Deviation, Deferral, or Gate Waiver.

**Verified by:** Portal (review) + Oracle (presence checks).

### C-EXC-2: Exceptions Visible at Baseline

**Requirement:** Active exceptions MUST be listed in Freeze Record and acknowledged in Approvals.

**Verified by:** Oracle (freeze validation) + Portal (checklist).

### C-EXC-3: Exceptions Don't Rewrite Governance

**Requirement:** Exceptions are scoped permissions, not silent rewrites.

**Verified by:** Portal (review).

### C-EXC-4: Gate Waiver Required Fields

**Requirement:** Waivers MUST reference specific failure(s), define scope, state risk/mitigation, define resolution criteria, include expiry/review date, be human-approved.

**Verified by:** Portal (approval record) + Oracle (field validation).

### C-EXC-5: Waiver Scope Constraints

**Requirement:** Waivers MUST have explicit scope (per-candidate, per-loop, per-baseline, or time-boxed). Unbounded scope prohibited.

**Verified by:** Oracle (scope validation) + Portal (approval).

---

## 11. Decisions

### C-DEC-1: Binding Decisions Are Recorded

**Requirement:** Binding human judgments MUST be recorded with stable ID, context, decision, rationale, and affected artifact/exception refs.

**Verified by:** Portal (review).

---

## 12. Metadata and Typing

### C-META-1: Machine-Readable Metadata

**Requirement:** Governance-relevant markdown artifacts MUST include YAML frontmatter per SR-TYPES schema.

**Verified by:** Oracle (validation).

### C-META-2: Stable Identity and Lineage

**Requirement:** Governed artifacts MUST have stable IDs and clear lineage.

**Verified by:** Oracle (lineage checks) + Portal (review).

### C-META-3: Binding Records Distinguishable

**Requirement:** Binding records MUST be distinguishable by type/metadata for deterministic queries.

**Verified by:** Oracle (type validation) + Portal (audit).

---

## 13. Notes for Downstream Documents

This contract leaves to lower layers:

- exact entity schemas (SR-SPEC)
- exact port signatures and error models (SR-SPEC)
- canonical serialization choices (SR-SPEC)
- gate names, verification profiles, thresholds (SR-DIRECTIVE)
- operational playbooks (Directive / adjacent)

Contract compliance is demonstrated by gates, audits, and verification profiles defined downstream.
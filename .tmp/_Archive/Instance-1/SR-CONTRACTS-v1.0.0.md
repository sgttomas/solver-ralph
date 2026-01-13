---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-CONTRACT"
  type: "governance.arch_contract"
  title: "SOLVER-Ralph Architectural Contract"
  version: "1.0.0"
  status: "governed"
  normative_status: "normative"
  authority_kind: "content"
  governed_by: ["SR-CHANGE"]
  supersedes: ["SR-CONTRACT@1.0.0-draft.6"]
  created: "2026-01-09"
  updated: "2026-01-10"
  tags:
    ["solver-ralph", "architectural-contract", "governance", "hexagonal-architecture", "event-store", "oracles", "trust-boundary", "event-graph", "evidence", "candidates", "waivers"]
  ext:
    arch_contract:
      verification_methods: ["oracle", "portal"]
      precedence:
        binding_precedence_rank: 1
      scope:
        implementation_agnostic: true
        archetype: true
      notes:
        - "Draft.5 incorporates select clarifications from draft.3 where they improve enforceability and cross-doc coherence: event-store source-of-truth, stable actor identity + approval record minimum fields, evidence bundle minimum manifest aligned to evidence.gate_packet, and waiver lifecycle/expiry expectations."
        - "Draft.5 adds REPEATED_FAILURE (N≥3, Directive-defined) as a mandatory stop-the-line trigger, aligning the loop governor with conservative escalation principles." 
        - "Draft.5 clarifies that ORACLE_TAMPER and ORACLE_ENV_MISMATCH cannot be bypassed by waivers; resolution requires restart or explicit suite rebase." 
        - "Draft.6 makes iteration context provenance binding via `IterationStarted.refs[]` (SYSTEM-only) and prohibits ghost inputs as a contract requirement."
---

# SOLVER-Ralph Architectural Contract (Why)

**Purpose:** Define the binding invariants (“MUST/SHALL”) that make SOLVER-Ralph a **high-assurance, agentic, governance-first** development system. This contract constrains what implementations may do, regardless of language, framework, storage, or model provider.

**Normative Status:** **Normative (binding).** Highest binding precedence among the governed set.

**Interpretation Guidance:** Consult **SOLVER-Ralph Design Intent (Why²)** for rationale and tradeoffs. Consult **SOLVER-Ralph Types** for artifact taxonomy, metadata conventions, and the authority model.

---

## 0. Version Changes

### 1.0.0-draft.6 (2026-01-10)

- Adds binding requirements for **iteration context provenance**: the authoritative input set for an Iteration MUST be recorded as typed references on `IterationStarted.refs[]`.
- Makes `IterationStarted` a **SYSTEM-only** event: `IterationStarted.actor_kind` MUST be `SYSTEM`.
- Adds a binding **no ghost inputs** constraint: an iteration’s agent context MUST be derivable solely from the `IterationStarted` payload plus dereferenced typed references.

### 1.0.0-draft.5 (2026-01-09)

- Adds an explicit **Event Store as Source of Truth** architectural contract (C-ARCH-3) to make the audit log epistemically primary.
- Strengthens trust boundary semantics with explicit requirements for **Stable Actor Identity** (C-TB-5) and **Approval Record Minimum Fields** (C-TB-6).
- Strengthens evidence semantics by requiring a minimum **Evidence Bundle manifest** aligned to `evidence.gate_packet` and content-addressed immutability (C-EVID-1..C-EVID-5).
- Adds `REPEATED_FAILURE` (N consecutive failures, with N ≥ 3) to the **mandatory stop-the-line trigger minimum set** (C-LOOP-3).
- Tightens Gate Waiver semantics by requiring expiry/review constraints and separating required waiver fields vs scope constraints (C-EXC-4, C-EXC-5).
- Clarifies integrity-condition handling so that `ORACLE_TAMPER` and `ORACLE_ENV_MISMATCH` cannot be bypassed via waivers (C-OR-7).

### 1.0.0-draft.4 (2026-01-09)

- Defined **Candidate**, **Run**, **Ralph Loop**, and **Iteration** as first-class domain terms and bound verification/approval semantics to them.
- Tightened **Shippable**: not Shippable without a complete **Freeze Record** (or equivalent binding baseline snapshot) listing all active exceptions (including Gate Waivers).
- Clarified that **every Portal is a trust boundary** and every Portal crossing MUST yield a recorded, attributable **Approval** linked to evidence and exceptions.
- Defined a minimal oracle integrity taxonomy: `ORACLE_TAMPER`, `ORACLE_GAP`, `ORACLE_FLAKE`, `ORACLE_ENV_MISMATCH` (all stop-the-line).
- Strengthened event semantics for corrections and re-evaluation (supersession + staleness marking).

### 1.0.0-draft.2 (2026-01-09)

- Added Verified-with-Exceptions and Gate Waivers (human approved).
- Strengthened event integrity semantics for dependency re-evaluation graphs.

---

## 1. Scope

This contract is intentionally **implementation-agnostic**. It defines:

- authority boundaries (who can do what),
- verification semantics (what “verified” means),
- candidate/run binding and oracle integrity requirements,
- audit and traceability invariants,
- exception handling invariants (deviation/deferral/waiver),
- required architectural separation (hexagonal boundary).

This contract does **not** define:

- concrete API schemas, database schemas, or command lines (belongs in the Technical Specification),
- exact phase plans, gate names, or command targets (belongs in the Development Directive),
- operational playbooks (may live in Directive or adjacent governance artifacts).

---

## 2. Definitions

### 2.1 Standing and Claims

SOLVER-Ralph separates **evidence** from **authority**.

- **Evidence:** recorded outputs (oracle logs, scans, checklists) that support a claim.
- **Verified (Strict):** required deterministic oracles PASS, with complete evidence and no unresolved integrity conditions.
- **Verified-with-Exceptions:** required deterministic oracles were executed and evidence is complete, but one or more required oracles FAILED *and* each required-oracle failure is covered by an explicit, human-approved **Gate Waiver** that is within scope.
- **Approved:** a human actor authorizes a candidate (or governance change) at a trust boundary (portal/gate) with reference to evidence (and any waivers/exceptions).
- **Shippable:** **Verified** (Strict or With-Exceptions) + **Approved at the Release Approval Portal** + baseline conditions satisfied (including a complete Freeze Record and explicit acknowledgement of active exceptions applicable to the release).

**Note:** “Verified” is not metaphysical correctness; it is a bounded, evidence-grounded claim relative to declared oracle suite(s) and declared environment constraints.

### 2.2 Actors

- **Human:** can approve and can make binding decisions.
- **Agent:** can propose and iterate; cannot approve trust boundary actions.
- **System:** orchestrates and records; cannot substitute for human approval.

### 2.3 Candidate, Run, Ralph Loop, Iteration

- **Candidate:** a **content-addressable snapshot** of one or more work products, identified by a stable identifier that changes if the underlying content changes (e.g., a content hash; a VCS commit identifier is acceptable only if it is unambiguously bound to immutable content). A Candidate is the unit of verification and approval.
- **Run:** an execution of a declared oracle suite against a specific Candidate, producing an Evidence Bundle.
- **Ralph Loop:** a bounded workflow instance with a goal, explicit budgets, stop-the-line triggers, and controlled memory.
- **Iteration:** one fresh-context cycle within a Ralph Loop that produces attempted work (and may produce a Candidate), executes evaluation (Runs), and records outcomes and summaries.

### 2.4 Oracles, Oracle Suites, Environment Constraints

- **Oracle:** an evaluator that produces:
  - a binary result (PASS/FAIL), and
  - attributable evidence (logs/outputs).
- **Required Oracle:** an Oracle whose result participates in determining a Verified claim for a declared verification mode.
- **Advisory Oracle:** an Oracle whose results are recorded and visible, but whose FAIL does not block a Verified claim.
- **Oracle Suite:** a named set of oracle definitions with a stable identity and a deterministic hash of its canonical definition. The suite definition MUST include declared environment constraints required for determinism.
- **Environment Constraints:** a declared description of the execution context under which required oracles are expected to be deterministic (e.g., pinned runtime/container digest, toolchain versions, network isolation).

### 2.5 Oracle Integrity Conditions

The following integrity conditions are **contract-mandated minimums**:

- **ORACLE_TAMPER:** the oracle suite definition or identity/hash used for a Run changed or failed integrity checks relative to what was pinned at the start of the Run.
- **ORACLE_GAP:** a required oracle has no recorded result in the evidence for the declared suite/mode.
- **ORACLE_FLAKE:** a required oracle is observed to be non-deterministic for identical inputs within declared environment constraints (e.g., conflicting results across repeated Runs).
- **ORACLE_ENV_MISMATCH:** a Run did not satisfy the oracle suite’s declared environment constraints.

### 2.6 Gates, Portals, Approvals, Freeze, Waivers

- **Gate:** a named checkpoint with explicit criteria and evidence requirements.
- **Portal:** a Gate at which **human arbitration is required** (because acceptability is not fully oracle-verifiable, or because risk/stakes demand human authority). Every Portal is a trust boundary.
- **Approval:** a binding, attributable record of a Human’s accept/reject decision at a Portal, explicitly linked to the subject (Candidate and/or governed artifact), the Evidence Bundle(s) reviewed, and any applicable exceptions acknowledged.
- **Evidence Bundle:** the structured evidence artifact produced by a Run (minimum manifest semantics are specified in C-EVID-1). The canonical evidence artifact type is `evidence.gate_packet`.
- **Freeze Record:** a binding baseline snapshot record that enumerates governed artifacts (IDs + versions + hashes) in force, and lists active exceptions (Deviations/Deferrals/Waivers) applicable to the baseline/release.
- **Gate Waiver:** a binding record that permits progression despite failure of a required oracle/gate, under explicit human approval and explicit scope.

### 2.7 Exceptions and Decisions

- **Deviation:** a binding exception from a governed requirement.
- **Deferral:** a binding postponement of a requirement/deliverable.
- **Decision:** a binding human judgment that resolves a stop-the-line escalation or otherwise resolves ambiguity/tradeoffs.

---

## 3. Architectural Structure

### C-ARCH-1: Hexagonal Separation

**Requirement:** The system MUST be architected as a **hexagon**:

- a **domain core** that contains authoritative rules and state machines,
- **driving ports** representing allowed commands/queries into the domain,
- **driven ports** representing the domain’s dependencies (storage, execution, runtimes),
- **adapters** that implement ports and may be swapped without changing domain rules.

**Verified by:** Oracle (architecture conformance checks) + Portal (architecture review).

### C-ARCH-2: Domain Purity

**Requirement:** The domain core MUST NOT depend on infrastructure concerns (database clients, network calls, LLM SDKs, filesystem, subprocess). The domain MAY define interfaces/ports for those concerns.

**Verified by:** Oracle (unit tests + static checks) + Portal (architecture review).

### C-ARCH-3: Event Store as Source of Truth

**Requirement:** The system MUST use an append-only event store as the source of truth for governance-relevant state.

- Derived/query stores (including graph databases) MUST be reconstructible from the event log.
- The event store MUST be a driven port in the hexagonal architecture (adapter-swappable) without changing domain invariants.

**Verified by:** Oracle (reconstruction tests) + Portal (architecture review).

---

## 4. Trust Boundary and Portals

### C-TB-1: Human-Only Binding Authority

**Requirement:** Any action that creates, removes, or changes **binding authority** MUST require a Human actor, including at minimum:

- approving (or rejecting) a Candidate at the Release Approval Portal,
- approving governance changes that alter normative meaning or required evidence,
- approving Deviations, Deferrals, and Gate Waivers as binding (and closing/resolving them),
- finalizing a Freeze Record (or equivalent binding baseline snapshot),
- registering, modifying, or designating as “current” an oracle suite used for governed verification,
- rebasing verification from one pinned oracle suite to another (mid-flight),
- extending or overriding Ralph Loop budgets after `BUDGET_EXHAUSTED`.

**Verified by:** Portal (human approval record exists) + Oracle (authorization checks in code paths).

### C-TB-2: Non-Authoritative Agent Output

**Requirement:** Agent-generated statements (e.g., “tests pass,” “compliant,” “approved”) MUST be treated as **non-authoritative proposals** unless backed by recorded oracle evidence and/or recorded human approvals.

**Verified by:** Oracle (no code path accepts agent claims as evidence) + Portal (governance review).

### C-TB-3: Portal Crossings Produce Approvals

**Requirement:** Every Portal crossing MUST produce a binding Approval record that is:

- attributable to a Human with a stable actor identity,
- linked to the subject being approved/rejected (Candidate and/or governed artifact change),
- linked to the Evidence Bundle(s) reviewed,
- linked to all applicable active exceptions (Deviations/Deferrals/Waivers) that affect the decision.

**Verified by:** Oracle (approval/evidence linkage validation) + Portal (audit review).

### C-TB-4: Minimum Required Portals

**Requirement:** An implementation MUST define, at minimum:

1) a **Governance Change Portal** (or equivalent) for normative governance changes; and
2) a **Release Approval Portal** (or equivalent) for declaring a Candidate Shippable.

The Development Directive MAY define additional portals, but MUST NOT remove these minima.

**Verified by:** Portal (governance review) + Oracle (presence checks in configuration/spec where automatable).

### C-TB-5: Stable Actor Identity

**Requirement:** Actor identity for Humans (and any other actors who perform governance-relevant actions) MUST be:

- **verifiable** against an authentication system,
- **stable** across sessions,
- **auditable**, such that the system can definitively answer “who did this?”

**Implementation note (non-binding):** acceptable identity formats typically include OIDC subject identifiers, SSH key fingerprints, or authenticated platform identities (e.g., `github:<user>`). Unauthenticated email strings are insufficient as the sole identity.

**Verified by:** Oracle (identity format validation) + Portal (authentication review).

### C-TB-6: Approval Record Minimum Fields

**Requirement:** Approval records MUST include, at minimum:

- **Portal identifier** (which Portal was crossed),
- **Decision** (`approved` or `rejected`),
- **Subject reference(s)**:
  - Candidate reference (for Candidate approvals), and/or
  - governed artifact reference(s) (for governance changes),
- **Evidence reviewed**: references to Evidence Bundle(s) considered,
- **Exceptions acknowledged**: references to active Deviations/Deferrals/Waivers applicable to the decision (MUST be explicit even if empty),
- **Approver identity** satisfying C-TB-5,
- **Timestamp** (ISO 8601).

**Verified by:** Oracle (approval record schema validation) + Portal (audit review).

---

## 5. Verification Semantics

### C-VER-1: Verification Is Evidence-Based and Candidate-Bound

**Requirement:** A Candidate MAY be marked “Verified” only when:

1) an Evidence Bundle exists for a Run against that Candidate,
2) the Evidence Bundle is attributable and integrity-checked,
3) the Evidence Bundle declares the oracle suite identity/hash(es) used,
4) all required oracles in the declared suite(s) have recorded results,
5) the Run has no unresolved oracle integrity conditions (at minimum: `ORACLE_TAMPER`, `ORACLE_GAP`, `ORACLE_FLAKE`, `ORACLE_ENV_MISMATCH`) for the scope claimed,
6) the Candidate identity is stable and immutable for the scope claimed (content-addressable or equivalent).

**Verified by:** Oracle (verification logic tests).

### C-VER-2: Verified (Strict)

**Requirement:** A Candidate MUST be marked **Verified (Strict)** if and only if:

- C-VER-1 holds, and
- every required oracle result in the declared verification suite(s) is PASS.

**Verified by:** Oracle (verification logic tests).

### C-VER-3: Verified-with-Exceptions

**Requirement:** A Candidate MUST be marked **Verified-with-Exceptions** if and only if:

- C-VER-1 holds, and
- at least one required oracle result is FAIL, and
- every required-oracle failure is covered by a **binding, human-approved Gate Waiver** whose scope includes:
  - the specific oracle(s)/gate(s),
  - the specific Candidate (default) **or** an explicitly declared, constrained superset scope,
  - the declared environment constraints (if any).

**Additional constraints:**

- Evidence MUST include failing outputs; failures MUST NOT be suppressed.
- A Gate Waiver MAY cover multiple failures only if each failure is explicitly listed and the waiver explains why grouping is appropriate.

**Verified by:** Oracle (waiver + evidence linkage validation) + Portal (human waiver approval exists).

### C-VER-4: Verified Claims Must Declare Mode and Basis

**Requirement:** Every “Verified” claim MUST declare:

- verification mode (Strict vs With-Exceptions),
- the oracle suite identity/hash(es) used,
- and (if With-Exceptions) the waiver record reference(s).

**Verified by:** Oracle (metadata validation).

### C-SHIP-1: Shippable Requires Freeze + Approval + Verified

**Requirement:** A Candidate MUST NOT be marked **Shippable** unless all of the following hold:

1) The Candidate is Verified (Strict or With-Exceptions).
2) A Human Approval exists for the Candidate at the Release Approval Portal.
3) A **Freeze Record** (or equivalent binding baseline snapshot) exists and is finalized, immutable, and complete for the release. At minimum, it MUST:
   - enumerate the governed artifacts in force (IDs + versions + hashes),
   - list all active, applicable exceptions (Deviations, Deferrals, and Gate Waivers),
   - reference the Evidence Bundle(s) supporting the verification claim,
   - reference the Approval record for the release decision,
   - reference the Candidate identity being released.

**Additional constraint:** The Release Approval record MUST explicitly acknowledge the active exceptions applicable to the release (C-TB-6).

**Verified by:** Oracle (freeze/approval linkage validation) + Portal (release audit).

---

## 6. Oracle Suite and Integrity

### C-OR-1: Required Oracles Must Be Deterministic; Advisory Oracles Are Allowed

**Requirement:** Oracles designated as **required** for a verification mode MUST be deterministic within their declared environment constraints.

If an oracle is not deterministic, it MUST NOT be required; it MUST be either:

- classified as Advisory, or
- moved behind a Portal as a human-reviewed acceptability judgment.

**Verified by:** Oracle (flake detection tooling where feasible) + Portal (manual classification).

### C-OR-2: Suite Pinning and Integrity

**Requirement:** Any governed Run that produces a verification claim MUST pin the oracle suite identity/hash(es) used at the start of the Run.

If the oracle suite definition changes during the Run (hash mismatch), the system MUST:

- raise `ORACLE_TAMPER` (or equivalent), and
- prevent further progression **under the assumption of equivalence**.

**Allowed resolution paths:**

- restart the Run under a newly pinned suite, or
- record an explicit human-approved decision that rebases verification to the new suite (treated as a new verification basis).

**Clarification:** `ORACLE_TAMPER` is an integrity condition and MUST NOT be bypassed via Gate Waiver. It requires restart or explicit rebase.

**Verified by:** Oracle (suite hash comparison tests) + Portal (approval record exists for rebases).

### C-OR-3: Environment Constraints Must Be Declared and Enforced

**Requirement:** Oracle suite definitions used for governed verification MUST declare environment constraints sufficient to support determinism of required oracles.

If a Run is executed in a context that violates declared constraints, the system MUST raise `ORACLE_ENV_MISMATCH` and MUST NOT treat the Run as valid evidence for a Verified claim.

**Clarification:** `ORACLE_ENV_MISMATCH` MUST NOT be bypassed via Gate Waiver. It requires rerun in a compliant environment, or governance change to constrain/adjust the suite definition.

**Verified by:** Oracle (environment assertion tests) + Portal (governance review).

### C-OR-4: Oracle Gaps Are Blocking

**Requirement:** If any required oracle result is missing for a declared suite/mode, the system MUST raise `ORACLE_GAP` and MUST NOT produce a Verified claim until the gap is resolved.

**Clarification:** An `ORACLE_GAP` cannot be “covered” by a Gate Waiver, because a Waiver applies to known failures (FAIL) with preserved evidence. Gap resolution requires executing the missing oracle, or changing the suite/mode definition via governance.

**Verified by:** Oracle (required-oracle completeness checks).

### C-OR-5: Oracle Flake Is a Stop-the-Line Integrity Condition

**Requirement:** If a required oracle is detected to be non-deterministic for identical inputs within declared constraints, the system MUST raise `ORACLE_FLAKE` and MUST NOT produce a Verified claim until it is resolved through one of the following:

- the oracle is repaired to be deterministic and the Run is re-executed, or
- the oracle is reclassified (e.g., to Advisory) via an approved governance change and a new suite is pinned, or
- the check is moved behind a Portal via an approved governance change.

**Verified by:** Oracle (flake classification checks where feasible) + Portal (governance approval exists for reclassification).

### C-OR-6: No Silent Oracle Weakening

**Requirement:** The system MUST NOT silently:

- disable required oracles,
- downgrade FAIL to PASS,
- suppress failing categories,
- reduce oracle scope/coverage, or
- change required/advisory classification

without explicit governance routing and explicit human approval recorded as a binding record.

**Important:** A Gate Waiver is not “silent weakening.” It preserves the oracle’s execution and evidence while allowing progression under explicit acknowledgement.

**Verified by:** Oracle (policy checks) + Portal (governance review).

### C-OR-7: Oracle Integrity Conditions Halt and Escalate

**Requirement:** When any oracle integrity condition (at minimum: `ORACLE_TAMPER`, `ORACLE_GAP`, `ORACLE_FLAKE`, `ORACLE_ENV_MISMATCH`) is detected, the system MUST:

- halt the affected Run’s progression to any Verified claim,
- record the condition with sufficient context to audit,
- route escalation to the appropriate Portal and/or human decision authority.

**Verified by:** Oracle (condition detection tests) + Portal (resolution review).

---

## 7. Evidence Integrity

### C-EVID-1: Evidence Bundle Minimum Manifest

**Requirement:** Evidence Bundles MUST be structured artifacts aligned to the `evidence.gate_packet` semantics. At minimum, an Evidence Bundle manifest MUST include:

| Field | Requirement |
|---|---|
| Candidate reference | ID of the Candidate verified |
| Oracle suite hash | Hash/ID of the suite used |
| Governed artifact references | ID + version + hash for each governed artifact in effect |
| Exception references | IDs of active Deviations/Deferrals/Waivers |
| Results | Per-oracle: oracle ID, result (PASS/FAIL), and a log/output reference |
| Attribution | Actor type, actor identity, timestamp |
| Content hash | Stable content hash of the bundle |

**Clarification:** Logs/outputs MAY be stored as separate content-addressed blobs referenced by the manifest, but the manifest itself is the minimum structured record.

**Verified by:** Oracle (evidence schema validation) + Portal (audit review).

### C-EVID-2: Evidence Immutability and Content Addressing

**Requirement:** Evidence Bundles MUST be:

- immutable once recorded,
- stored in content-addressed storage (identified by content hash or equivalent collision-resistant digest),
- protected against modification (storage system MUST NOT allow overwrite at the same hash).

**Verified by:** Oracle (content hash validation) + Portal (storage policy audit).

### C-EVID-3: Evidence Attribution

**Requirement:** Evidence MUST be attributable with at minimum:

| Field | Required | Notes |
|---|:---:|---|
| Actor type | Yes | Human / Agent / System |
| Actor identity | Yes | Stable identity; humans must satisfy C-TB-5 |
| Timestamp | Yes | ISO 8601 |

**Recommended fields (non-binding):** model identity (for agents), repository state (commit SHA), host/runner identity (for CI), runtime image digest, environment ID.

**Verified by:** Oracle (attribution field validation).

### C-EVID-4: Evidence Dependency References

**Requirement:** Evidence Bundles MUST include typed references sufficient for dependency queries and re-evaluation:

| Reference | Required |
|---|:---:|
| Candidate ID | Yes |
| Oracle suite hash | Yes |
| Governed artifact refs (ID + version + hash) | Yes |
| Active exception refs | Yes |

**Recommended:** Loop ID, Iteration ID, Environment ID.

**Verified by:** Oracle (reference presence checks) + Portal (audit review).

### C-EVID-5: Evidence Containing Secrets

**Requirement:** When evidence contains secrets (tokens, credentials, proprietary data):

- Original evidence MUST be stored in a restricted vault with hash H1.
- Redacted copy MAY be stored in the general evidence store with hash H2.
- The evidence record MUST include:
  - `original_content_hash` (restricted reference),
  - `redacted_content_hash` (if redacted copy exists),
  - `redaction_manifest` (list of redacted fields/regions).

Normal audit uses redacted copy; privileged audit can access original with authorization.

**Verified by:** Portal (secret handling audit).

---

## 8. Event, Audit, and Graph Model

### C-EVT-1: Event Attribution

**Requirement:** Every state-changing event MUST include:

- an actor type (Human/Agent/System),
- a stable actor identity,
- and an event timestamp.

The stable actor identity for Humans MUST satisfy C-TB-5.

**Verified by:** Oracle (event schema validation).

### C-EVT-2: Append-Only Event Log

**Requirement:** Governance-relevant events MUST be append-only and durable. Past events MUST NOT be mutated in-place; corrections MUST be represented as new events.

**Verified by:** Portal (audit procedure) + Oracle (adapter contract tests where feasible).

### C-EVT-3: Explicit Supersession for Corrections

**Requirement:** If an event corrects, replaces, or retracts a prior event, the new event MUST carry an explicit reference relationship (e.g., `supersedes` or `retracts`) to the prior event(s), such that an “effective state” view can be computed without deleting history.

**Verified by:** Oracle (supersession linkage tests) + Portal (audit review).

### C-EVT-4: Sequence-First Ordering (Per Stream)

**Requirement:** Within an event stream, event ordering MUST be based on a monotonic sequence assigned by the event store (or equivalent), not on timestamps alone.

Implementations MAY use multiple streams (e.g., per-loop streams), but this requirement applies independently to each stream used for governance-relevant events.

**Verified by:** Oracle (event store contract tests).

### C-EVT-5: Event Graph References for Impact Analysis

**Requirement:** Governance-relevant events MUST include typed references to related nodes (governed artifacts, Candidates, Evidence Bundles, Approvals, Decisions, Deviations/Deferrals/Waivers, Loops) sufficient to support:

- “what depended on this?” queries, and
- re-evaluation workflows when a referenced node changes.

**Minimum semantic node categories:** the dependency graph MUST be able to distinguish at least:

- governed artifact,
- candidate,
- oracle suite,
- evidence bundle,
- approval,
- decision,
- deviation,
- deferral,
- waiver,
- loop (and optionally iteration).

**Minimum semantic relationship classes:** the reference graph MUST distinguish at least:

- **semantic dependency** (eligible for staleness propagation and blocking effects by default), and
- **audit provenance** (recorded for explanation/audit and non-blocking by default).

**Constraint:** Implementations MUST NOT treat audit provenance as semantic dependency by default.
If a relationship is intended to participate in staleness propagation, it MUST be represented as
a semantic dependency.

**Implementation note:** This can be modeled as a property graph, an adjacency list, or event metadata fields, but the semantics MUST be preserved.

**Verified by:** Oracle (reference graph query tests) + Portal (audit review).

### C-EVT-6: Staleness Marking and Re-Evaluation Routing

**Requirement:** When a referenced node changes (e.g., a governed artifact version changes, an oracle suite changes, an exception is closed), the system MUST support:

- marking downstream dependent nodes as **potentially stale**, and
- routing an appropriate re-evaluation workflow (Ralph Loop and/or Portal) before those downstream nodes are treated as current for high-stakes decisions (e.g., release).

**Default traversal rule:** Staleness marking and re-evaluation routing MUST traverse
**semantic dependency** edges by default and MUST NOT traverse **audit provenance** edges by default.
Governance MAY define cases where an input must be modeled as a semantic dependency; in those cases
it MUST be represented as a semantic dependency relationship (not audit provenance).

**Verified by:** Oracle (dependency traversal tests) + Portal (audit review).

### C-EVT-7: Projections Must Be Derivable From the Audit Trail

**Requirement:** Implementations MAY maintain derived/query projections (including graph databases). If they do, those projections MUST be reconstructible from the durable, append-only governance event log, and MUST NOT be treated as the only authoritative record of governance-relevant history.

**Verified by:** Portal (architecture review) + Oracle (replay/projection tests where feasible).

**Optional strengthening (recommended):** Events MAY include per-event content hashes and/or hash chaining for tamper detection. This is encouraged but not required by this contract.

---

## 9. Ralph Loop Governance

### C-LOOP-1: Bounded Iteration With Hard Stop

**Requirement:** Agentic iteration MUST be bounded by explicit budgets (iterations, time, and/or cost).

When budgets are exhausted, the system MUST raise `BUDGET_EXHAUSTED` (or equivalent), MUST stop autonomous iteration, and MUST require an explicit Human decision to:

- extend budgets (recorded as a binding decision/approval), or
- terminate the loop as failed/abandoned, or
- proceed with a Candidate that already satisfies all release requirements.

The system MUST NOT silently or automatically extend budgets.

**Verified by:** Oracle (loop governor tests) + Portal (audit review).

### C-LOOP-2: Fresh-Context Iterations With Controlled Memory

**Requirement:** Each Iteration SHOULD operate in fresh context with controlled memory, and the system MUST record:

- what was attempted,
- what Candidate(s) (if any) were produced,
- what Evidence Bundles were produced,
- what stop triggers fired,
- and an Iteration summary suitable for re-entry.

**Controlled memory minimums (contract-level):**

- Governed artifacts MUST be available.
- The iteration context construction mechanism MUST be explicit, auditable, and replayable from events + typed references.
- Iteration context provenance MUST be recorded via typed references on `IterationStarted.refs[]` (see C-CTX-1).
- Raw, unbounded conversation history MUST NOT be the default memory mechanism for normative execution; typed iteration summaries and referenced artifacts/evidence are the default.

**Verified by:** Portal (architecture review) + Oracle (iteration recording tests).



### C-CTX-1: Iteration Context Provenance via IterationStarted Refs

**Requirement:** For every Iteration, the system MUST emit an `IterationStarted` event whose `refs[]` constitute the authoritative provenance record for that iteration’s context inputs (“Iteration Context Ref Set”).

- `IterationStarted.actor_kind` MUST be `SYSTEM`.
- `IterationStarted.refs[]` MUST include typed references sufficient to reconstruct (for audit and replay) what governed artifacts, carried-forward iteration summaries, base candidates, oracle suites, active exceptions, and configuration were in semantic scope.

**Verified by:** Oracle (event schema + required ref-set validation) + Portal (architecture review).

### C-CTX-2: No Ghost Inputs

**Requirement:** An iteration’s agent context MUST be derivable solely from:

1) the `IterationStarted` event payload, plus  
2) dereferenced typed references in `IterationStarted.refs[]`  

(as implemented by the SR-SPEC ContextCompiler semantics).

Inputs not represented in `IterationStarted.refs[]` MUST NOT influence the work.

**Verified by:** Oracle (context compilation enforcement tests) + Portal (audit review).

### C-LOOP-3: Mandatory Stop-the-Line Triggers

**Requirement:** The system MUST implement stop-the-line triggers for at least:

- `ORACLE_TAMPER`
- `ORACLE_GAP`
- `ORACLE_ENV_MISMATCH`
- `ORACLE_FLAKE`
- `REPEATED_FAILURE`
- `BUDGET_EXHAUSTED`

`REPEATED_FAILURE` means N consecutive iterations fail to advance toward a Verified candidate under the active suite/mode (N MUST be defined in the Development Directive, and MUST be ≥ 3).

The Development Directive MAY add additional triggers (e.g., security critical findings), but MUST NOT remove these minima.

**Verified by:** Oracle (trigger evaluation tests) + Portal (governance review).

### C-LOOP-4: Candidate Production Must Be Traceable

**Requirement:** If an Iteration produces a work product that will be subject to verification or approval, the system MUST materialize it as a Candidate with a stable identity and MUST relate it in the event graph to:

- the Ralph Loop,
- the Iteration,
- the Run(s) executed against it,
- and any resulting Approval(s).

**Verified by:** Oracle (graph linkage validation) + Portal (audit review).

---

## 10. Exceptions: Deviation, Deferral, Gate Waiver

### C-EXC-1: Exceptions Are Records, Not Drift

**Requirement:** Any sustained mismatch between:

- governed requirements (Contract/Spec/Directive), and
- actual system behavior or delivery scope

MUST be represented as a **binding record artifact**:

- **Deviation** (exception from a requirement),
- **Deferral** (postponement of a requirement/deliverable), or
- **Gate Waiver** (permission to proceed despite failure of a required oracle/gate).

**Verified by:** Portal (review) + Oracle (presence checks where automatable).

### C-EXC-2: Binding Exceptions Are Visible at Baseline and Approval

**Requirement:** While an exception is active and binding:

- it MUST be listed in the Freeze Record for any baseline/release created during its activity, and
- it MUST be included in any Approval record for Candidates whose scope is affected.

**Verified by:** Oracle (freeze record validation) + Portal (approval checklist).

### C-EXC-3: Exceptions Do Not Silently Rewrite Governance

**Requirement:** Exception records MUST NOT be treated as silent rewrites of the Contract/Spec. They are operational permissions with explicit scope, risk, and resolution criteria.

**Verified by:** Portal (governance review).

### C-EXC-4: Gate Waiver Required Fields

**Requirement:** A Gate Waiver MUST:

- reference the specific gate/oracle and failure condition(s),
- explicitly list the failure(s) being waived (if multiple),
- define scope (see C-EXC-5),
- define any environment constraints required for applicability,
- state risk and mitigation,
- define resolution criteria (how it will be removed),
- name a resolution owner,
- include an expiry/sunset date **or** a required review date,
- be approved by a Human (with approver identity + timestamp),
- be included in all verification/approval records that rely on it.

**Multiple failures:** A single waiver MAY cover multiple related failures only if:

- each failure is explicitly listed,
- failures share a common root cause or mitigation,
- and the waiver documents why grouping is appropriate.

**Verified by:** Portal (waiver approval record) + Oracle (linkage + field validation).

### C-EXC-5: Waiver Scope Constraints

**Requirement:** Waivers MUST have explicit scope.

Permitted scope patterns:

- **Per-candidate (default)** — preferred.
- **Per-loop** — MUST specify the loop ID and MUST expire when the loop terminates (or earlier).
- **Per-baseline** — MUST specify the baseline/freeze ID and MUST expire when superseded (or earlier).
- **Time-boxed class** — MUST specify an expiry date and MUST require escalated approval.

**Prohibited:** Unbounded class scope.

**Superset-scope constraints:** If a waiver scope is broader than a single Candidate, it MUST be constrained (baseline-bound and/or time-boxed) and MUST NOT be indefinite.

**Verified by:** Oracle (scope validation) + Portal (approval review, including escalated approval where required).

---

## 11. Decisions

### C-DEC-1: Binding Decisions Are Recorded

**Requirement:** Any binding human judgment that resolves a stop-the-line escalation MUST be recorded as a decision record with:

- stable ID,
- context (trigger, scope),
- decision and rationale,
- references to affected governed artifacts and affected exception records (if any).

**Verified by:** Portal (review).

**Best practice (non-binding):** decisions intended as precedent should include explicit applicability conditions.

---

## 12. Metadata and Typing

### C-META-1: Machine-Readable Metadata

**Requirement:** Governance-relevant markdown artifacts MUST include YAML frontmatter metadata conforming to the SOLVER-Ralph Types schema (core required fields + type-specific extensions).

**Verified by:** Oracle (frontmatter validation).

### C-META-2: Stable Identity and Lineage

**Requirement:** Governed artifacts MUST have stable IDs and clear lineage. There MUST be at most one “current” artifact per lineage.

**Verified by:** Oracle (lineage checks) + Portal (review).

### C-META-3: Binding Records Are Distinguishable

**Requirement:** Binding record artifacts (e.g., Decisions, Deviations, Deferrals, Waivers, Approvals, Freeze Records) MUST be distinguishable by type and/or explicit metadata fields such that event/graph queries can deterministically answer:

- “what is the current binding state?” and
- “what approvals and exceptions applied to this Candidate?”

**Verified by:** Oracle (type/metadata validation) + Portal (audit review).

---

## 13. Conformance Checklist

An implementation is contract-conformant when:

- [ ] Trust boundary actions are human-only and enforced.
- [ ] Every Portal crossing yields an attributable Approval record with required fields.
- [ ] Human approvals are attributable to stable, verifiable identities.
- [ ] “Verified” is evidence-based, integrity-checked, and Candidate-bound.
- [ ] Both verification modes exist: Strict and With-Exceptions (human-approved waivers).
- [ ] Oracle suites are pinned; suite changes raise integrity conditions and require restart/rebase.
- [ ] Minimal oracle integrity conditions are implemented and are stop-the-line: TAMPER/GAP/FLAKE/ENV_MISMATCH.
- [ ] Oracles cannot be silently weakened; waivers preserve evidence and are explicit records with scope + expiry/review.
- [ ] Evidence Bundles have the required manifest and are immutable + content-addressed.
- [ ] Evidence is dependency-queryable (governed refs + exceptions + candidate + suite).
- [ ] Events are append-only, attributable, sequence-ordered, supersedable, and graph-referenceable.
- [ ] Dependency staleness can be marked and routed to re-evaluation.
- [ ] Loops are bounded with hard stop on budget exhaustion; no auto-extension.
- [ ] Mandatory loop triggers include REPEATED_FAILURE (N≥3) and route escalation.
- [ ] Freeze Records list active Deviations/Deferrals/Waivers and bind release to governed artifacts + evidence + approvals.
- [ ] All governance artifacts are typed and metadata-valid.

---

## 14. Notes for Downstream Documents

This contract intentionally leaves the following to lower layers:

- exact entity schemas (Technical Spec),
- exact port signatures and error models (Technical Spec),
- canonical serialization choices for suite hashing and evidence manifests (Technical Spec),
- gate names/commands, verification profile composition, and operational thresholds (Development Directive),
- operational playbooks (Directive / adjacent policies).

Contract compliance is demonstrated by gates, audits, and verification profiles defined downstream.
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-ETT"
  type: "governance.epistemic_trust_topology"
  title: "SOLVER-Ralph Epistemic Trust Topology"
  version: "1.1.0-draft.1"
  status: "draft"
  normative_status: "directional"
  authority_kind: "process"
  governed_by:
    - "SR-CHANGE"
  supersedes:
    - "SR-ETT@1.0.1"
  created: "2026-01-11"
  updated: "2026-01-11"
  tags:
    - "governance"
    - "policy"
    - "epistemology"
    - "trust"
    - "topology"
    - "harness"
    - "hexagonal-architecture"
    - "ports"
    - "adapters"
  ext:
    intent:
      summary: >
        Defines the constraint topology (nine harness membranes) that converts fluid,
        stochastic proposals into durable, attributable commitment objects. Clarifies
        how membranes are typically implemented at hexagonal ports/adapters and how
        SR-ETT serves as a design lens for SR-DIRECTIVE workflows that execute SR-PLAN
        under SR-CONTRACT/SR-SPEC.
      scope:
        - "Applies to all SOLVER-Ralph implementations and governed workflows."
        - "Used to place coercion boundaries so the system harnesses without strangling."
    harness_members:
      - key: "intent_objective"
        name: "Intent & Objective harness"
        modality: "Goal/fitness (telos control)"
      - key: "operational"
        name: "Operational harness"
        modality: "Process/state-machine (temporal control)"
      - key: "architectural"
        name: "Architectural harness"
        modality: "Structure/boundaries (spatial control)"
      - key: "ontological"
        name: "Ontological harness"
        modality: "Types/relations (semantic control)"
      - key: "isomorphic"
        name: "Isomorphic harness"
        modality: "Representation coupling (drift control)"
      - key: "change"
        name: "Change harness"
        modality: "Evolution/versioning (time-over-time control)"
      - key: "authority_integrity"
        name: "Authority & Integrity harness"
        modality: "Authority + integrity (admissibility control)"
      - key: "resource"
        name: "Resource harness"
        modality: "Budgets/economics (feasibility control)"
      - key: "accountability"
        name: "Accountability harness"
        modality: "Proof + completeness (legibility/replay control)"
    commitment_objects:
      - "IterationStarted (refs-bound context provenance)"
      - "Governed artifacts (controlled documents) in force for a work_unit/iteration"
      - "Candidate identity (git:<sha> + sha256:<hash>)"
      - "Evidence bundle manifest (content-addressed proof object)"
      - "Portal approval record (human responsibility boundary)"
      - "Freeze record (baseline snapshot + declared exceptions)"
      - "Change record (how the system’s own rules evolve)"
    assumptions:
      - "Agents are stochastic generators; they produce proposals, not authoritative claims."
      - "Ralph Loops are the unit of work; iteration context is derived only from refs + payload."
      - "The governed artifact set (INTENT/CONTRACT/SPEC/TYPES/DIRECTIVE/CHANGE/README) is the source of truth for semantics."
---

# SOLVER-Ralph Epistemic Trust Topology

**Classification:** Governance-adjacent policy (directional).  
**Purpose:** Provide a shared model for placing coercion boundaries so SOLVER-Ralph harnesses stochastic agents *without* strangling exploration.

---

## 0. Version changes

### 1.1.0-draft.1 (2026-01-11)

- Adds an explicit **hexagonal architecture mapping**: membranes are typically enforced at **ports/adapters** with **domain-core invariants**.
- Aligns terminology with SR-CONTRACT/SR-SPEC for **Proposal vs Commitment Object** and **Verification/Evaluation/Validation/Assessment**.
- Adds per-membrane **"hex surface"** placement hints and example enforcement mechanisms.
- Strengthens accountability guidance around **evidence vs authority** and highlights **EVIDENCE_MISSING** as a first-class membrane failure mode (as defined in SR-CONTRACT/SR-SPEC).
- Adds practical checklists: **agent capability checklist** and **SR-DIRECTIVE author checklist**.
- Adds a short section clarifying SR-ETT’s role relative to SR-CONTRACT / SR-SPEC / SR-DIRECTIVE / SR-PLAN.

---

## 1. Core idea

SOLVER-Ralph separates work into two regimes:

- **Exploration (fluid meaning):** proposals, hypotheses, drafts, partial implementations.
- **Commitment (solid meaning):** durable statements that downstream work can safely rely on (e.g., “this Candidate is the one we evaluated,” “these oracles produced this evidence,” “a Human accepted responsibility at a Portal”).

A **membrane** is the point where fluid meaning is converted into a **commitment object**.
The system applies coercion primarily at membranes, not throughout exploration.

This document names nine membranes together with their harnesses. The combined effect of the membranes is **epistemic trust**: what the system is allowed to treat as reliable knowledge. Trust is an emergent property when the system validates the current state and then a Human judges alignment with intent and objectives.

---

## 2. Definitions (alignment-first)

### 2.1 Proposal vs Commitment Object

This document uses **Proposal** and **Commitment Object** as defined *normatively* in **SR-CONTRACT §2.8** (and repeated in **SR-SPEC §1.2.1**).

Informal summary (non-binding, for readability):

- **Proposal:** any draft statement, plan, or output whose meaning is not yet stabilized as a durable, attributable object.
- **Commitment Object:** a durable, content-addressed, referenceable object recorded in the system and safe for downstream reliance (e.g., governed artifacts, Candidates, Evidence Bundles, Approvals, Freeze Records, Decisions, Exceptions).

Rule of thumb (binding rule lives in SR-CONTRACT): downstream “binding claims” must be derivable from commitment objects; proposals are non-authoritative until captured as commitment objects via the appropriate membranes.

### 2.2 Membrane

A **membrane** is a conversion boundary: the system refuses to treat an input as load-bearing unless it arrives in (or yields) a required commitment-object form.

### 2.3 Harness

A **harness** is the rule-enforced mechanism at a membrane: it constrains admissibility and ensures the conversion (proposal → commitment object) is typed, attributable, and replayable.

---

## 3. Design principle: harness without strangling

Coercion is valuable only when it prevents **false foundations**.

A practical placement rule:

> Coerce **when outputs become inputs** (i.e., when something becomes load-bearing), especially across actor boundaries.

Corollaries:

- Coerce **claims**, not creative exploration.
- Keep coercion objects small, typed, and replayable.
- Provide relief valves (exception/deferral records) so reality stays honest under constraints.
- Treat the governance system itself as evolvable: changes are recorded and reviewable.

### 3.1 Governed documents are infrastructure

In SOLVER-Ralph, the governed document set is not peripheral documentation. It is the **canonical substrate of meaning** that the system can safely rely upon across long runs and across stochastic workers.

Practically:

- semantic commitments are expressed as **typed, versioned artifacts** (or binding records) rather than implicit chat/session memory,
- iteration context is compiled from explicit references, so the documents in force act as **runtime inputs** to the system,
- and membranes are the points where fluid proposals must be “written into” durable forms (events/evidence/approvals/freezes/changes) that carry references to the governing documents.

### 3.2 Relationship to hexagonal architecture (ports/adapters as membrane surfaces)

SR-CONTRACT makes hexagonal architecture binding (C-ARCH-1..3):

- **Domain core**: authoritative rules + state machines
- **Driving ports**: allowed commands/queries into the domain
- **Driven ports**: domain dependencies (event store, evidence store, oracle runner, identity, clock)
- **Adapters**: swappable implementations (DB adapter, HTTP adapter, agent workers, oracle runtime, portals/UI)

SR-ETT’s membranes are not “modules.” They are **trust boundaries**: where the system must refuse to accept narrative/proposal as binding state unless it crosses in the correct commitment-object form. In practice, the default implementation strategy is:

- **Enforce membranes at ports first** (admissibility checks at boundary crossings).
- Keep **invariants in the domain core**.
- Keep **workers/tools/UI as adapters**.

Simple mental model:

```
  outside actors (Humans / Agents / Oracles)
              │
              ▼
        [Driving ports]  ←─ (membranes enforced here)
              │
              ▼
          Domain core    ←─ (invariants + state machines)
              │
              ▼
         [Driven ports]  ←─ (evidence/event/identity/time)
              │
              ▼
            Adapters
```

This is the main structural reason SR-ETT “fits” hexagonal architecture: ports are where actor boundaries meet the domain, and actor boundaries are exactly where SR-ETT says coercion belongs.

---

## 4. The nine membranes (harnesses)

Each harness describes:

- what it constrains,
- its default **hex surface** (where enforcement typically lives in a hexagon),
- where it lives (typical implementation surfaces),
- the commitment objects it produces or protects,
- example mechanisms,
- the failure modes it prevents.

### 4.1 Intent & Objective harness

**What it constrains:** the reference signal: what “success” means and what tradeoffs are acceptable among compliant options.

**Hex surface (default):** driving ports that start/advance work; context compilation for Iteration start.

**Where it lives:** design intent artifacts, work_unit goal declarations, iteration start payload + refs.

**Commitment objects:** referenceable goal statements; scope/budget declarations; stop triggers tied to objective failure.

**Mechanisms (examples):**

- “No-goal, no-work” guard: refuse to start a Loop/Iteration without explicit objective refs.
- Goal drift checks: require any scope change to be expressed as a change/decision record.

**Failure modes it prevents:**

- goal drift across long runs,
- proxy collapse (“tests passing” mistaken for “problem solved”),
- conflicting local objectives.

---

### 4.2 Operational harness

**What it constrains:** execution over time: how work advances and stops.

**Hex surface (default):** domain core state machines + driving port validation for transitions.

**Where it lives:** loop/iteration lifecycle state machine; gate hooks; stop triggers; closeout procedures.

**Commitment objects:** lifecycle events (loop created, iteration started, summary submitted), run lifecycle records.

**Mechanisms (examples):**

- Deterministic transition guards: invalid transitions are rejected at the port.
- Stop-the-line triggers emitted as events; progression blocked until resolved.

**Failure modes it prevents:**

- unbounded “keep going” behavior,
- undefined handoffs between agents and humans,
- silent “done” states.

---

### 4.3 Architectural harness

**What it constrains:** where responsibilities may live; separation of concerns.

**Hex surface (default):** domain purity + boundary enforcement at build/test time (architecture conformance “oracle”).

**Where it lives:** hex boundaries; ports/adapters; “domain core stays infra-free.”

**Commitment objects:** boundary tests; dependency rules; build-time enforcement artifacts.

**Mechanisms (examples):**

- Static dependency rules (“domain crate cannot import infra crates”).
- Boundary tests and CI checks; architecture review at Portal when required.

**Failure modes it prevents:**

- orchestration logic leaking into invariant-bearing domain logic,
- “implementation gravity” pulling everything into one layer,
- untestable coupling.

---

### 4.4 Ontological harness

**What it constrains:** what kinds of objects exist and what relations mean.

**Hex surface (default):** port schemas + typed refs validation (both ingress and event emission).

**Where it lives:** artifact type system; ref typing; relationship semantics (dependency vs provenance).

**Commitment objects:** typed refs; typed relationships; explicit categories for evidence/records/config.

**Mechanisms (examples):**

- Schema validation of `refs[]` (kind/rel/meta) on event acceptance.
- Enforce dependency vs provenance semantics (`depends_on` vs `supported_by`) in projections.

**Failure modes it prevents:**

- vocabulary drift (“approval” used where only a non-binding review exists),
- semantic ambiguity becoming institutional fact,
- staleness and dependence being conflated with audit provenance.

---

### 4.5 Isomorphic harness

**What it constrains:** drift between the governed model and the implemented system.

**Hex surface (default):** conformance oracles + projection rebuild tests at boundaries.

**Where it lives:** schema alignment; “governance ↔ runtime” mapping; projection rebuild guarantees.

**Commitment objects:** generated schemas; conformance tests; projection invariants.

**Mechanisms (examples):**

- “Spec↔runtime” conformance suite that ensures adapters implement ports as specified.
- Replay/rebuild tests that derive projections from the event log and compare results.

**Failure modes it prevents:**

- “spec says X, system does Y” without detection,
- informal side channels becoming de facto behavior,
- governance turning into narrative rather than executable constraint.

---

### 4.6 Change harness

**What it constrains:** how the system (and its governance) evolves without losing meaning.

**Hex surface (default):** Governance Change Portal + governed artifact registry/selection ports.

**Where it lives:** change requests; versioning rules; migration records; freeze/baseline procedures.

**Commitment objects:** change records; supersedence chains; declared compatibility.

**Mechanisms (examples):**

- “No silent meaning shifts”: changes must be recorded, reviewed, and attributable.
- Explicit migration notes when meaning changes require downstream re-evaluation.

**Failure modes it prevents:**

- silent reinterpretation of words and states,
- untracked breaking changes,
- accumulating “tribal knowledge” that cannot be reconstructed.

---

### 4.7 Authority & Integrity harness

**What it constrains:** who may commit binding state and what claims are authentic and untampered.

**Hex surface (default):** driving ports (authorization), Portal adapters (human approvals), driven ports (identity and integrity verification).

**Where it lives:** actor roles; endpoint permissions; event emission constraints; sandbox integrity; identity verification.

**Commitment objects:** authoritative events; portal approval records; integrity evidence (hashes, digests, environment fingerprints).

**Mechanisms (examples):**

- Actor-kind checks: reject “human-only” transitions from non-Human actors.
- Candidate identity binding: ensure approvals reference immutable Candidate identity.
- Oracle suite integrity checks and environment fingerprinting.

**Failure modes it prevents:**

- unauthorized commitments (agents performing human-only actions),
- forged or replayed claims,
- identity confusion (candidate/evidence mismatch),
- unsafe execution outside containment.

---

### 4.8 Resource harness

**What it constrains:** feasibility: cost, time, and concurrency.

**Hex surface (default):** driving ports for budget allocation/extension; domain invariants for exhaustion; orchestrator adapter for scheduling/backpressure.

**Where it lives:** budgets per work_unit/loop/iteration; quotas; stop triggers; queueing/backpressure.

**Commitment objects:** budget declarations; spend ledgers; stop-trigger events.

**Mechanisms (examples):**

- Budget accounting: refuse to schedule new work when exhausted without explicit authorized extension.
- Backpressure signals: prevent infinite retries from collapsing the system.

**Failure modes it prevents:**

- runaway exploration,
- starvation of higher-priority work,
- “infinite debugging” loops.

---

### 4.9 Accountability harness

**What it constrains:** legibility and replayability: what must be recorded for claims to count.

**Hex surface (default):** driven ports (event store + evidence store), plus driving-port checks before accepting binding transitions.

**Where it lives:** evidence bundle manifests; completeness checks; invariants; rebuildable projections.

**Commitment objects:** evidence bundles; manifests; deterministic context digests; audit-ready projections.

**Mechanisms (examples):**

- Evidence manifest completeness checks before claiming Verified.
- “No ghost inputs” context compilation: only `IterationStarted` payload + dereferenced typed refs.
- Projection rebuildability tests.

**Failure modes it prevents:**

- “it passed” without proof objects,
- irreproducible evaluations,
- missing links between approvals and evidence.

#### 4.9.1 Evidence vs Authority (epistemic boundary)

SOLVER-Ralph separates **evidence production** from **authority**:

- **Verification / Validation** produce evidence (oracle outputs).
- **Evaluation / Assessment** are Human interpretations of evidence, but are **non-binding** unless elevated into a Portal Approval/Decision.
- **Approval** is a binding Human action at a Portal.

This distinction is contract-controlled (SR-CONTRACT §2.9). SR-ETT’s role is to keep the membrane placement obvious: interpretations do not cross the Authority membrane unless recorded as an Approval/Decision at a Portal.

#### 4.9.2 Evidence availability (EVIDENCE_MISSING) as a first-class failure mode

If evidence referenced by a claim cannot be retrieved, the system must treat it as an integrity condition (e.g., `EVIDENCE_MISSING`) and block progression until resolved (contract/spec controlled). In topology terms: the Accountability membrane must not allow “phantom proof objects” to become foundations.

---

## 5. Membrane placement heuristics (practical)

When deciding whether to introduce a new coercion point, ask:

1) **Will downstream work rely on this as true?**
2) **Does this cross an actor boundary (SYSTEM ↔ worker ↔ human ↔ oracle)?**
3) **Can this be expressed as a small, typed commitment object?**
4) **Is there an honest relief valve when requirements cannot be met?**
5) **Does the change increase forced surface area inside exploration?** If yes, it risks strangling.

Default bias:

- keep exploration permissive inside iterations,
- harden at claim boundaries (identity, evidence, approval, freeze, change).

**Port-first default (hex alignment):** in a hexagonal harness, actor boundaries are implemented as ports/adapters; default membrane enforcement should occur at ports, with invariants in the domain core.

---

## 6. How to use this document (operational)

### 6.1 Agent capability checklist (design-time and runtime self-check)

Before an agent output is treated as load-bearing, confirm:

- [ ] **Intent:** What objective does this serve, and where is that objective referenced as a commitment object?
- [ ] **Operational:** What lifecycle state am I in, and is this action valid in that state?
- [ ] **Architectural:** Does this respect hex boundaries (no infra logic in domain; use ports)?
- [ ] **Ontological:** Are all artifacts, refs, and relationships correctly typed (dependency vs provenance correct)?
- [ ] **Isomorphic:** Am I relying on a behavior not represented in SR-SPEC / governed artifacts?
- [ ] **Change:** If this alters governed meaning, is it routed through a change record / Governance Change Portal?
- [ ] **Authority & Integrity:** Am I implicitly claiming approval/verification/authority? If yes, which commitment objects back it?
- [ ] **Resource:** Is this within declared budgets? If not, what explicit record authorizes an extension?
- [ ] **Accountability:** Will the system be able to replay this claim later from evidence and refs alone (no ghost inputs)?

If any answer is “no/unclear,” keep the output explicitly labeled as a **proposal**.

### 6.2 SR-DIRECTIVE author checklist (turn topology into workflows)

For each Directive rule/gate/stop trigger, record (informatively) the following:

1) Which membrane(s) it enforces.
2) What commitment object(s) it **requires** to pass and/or **produces** on success.
3) Where it is enforced in the hexagon:
   - driving port validation,
   - domain core invariant,
   - driven port check (evidence/event/identity),
   - adapter mechanism (CI/oracle/portal UI).
4) Which actor class is allowed to perform the action (Human / System / Agent).
5) What is the relief valve (deviation/deferral/waiver/decision) if the constraint cannot be met.

This is how SR-ETT acts as an upstream lens for SR-DIRECTIVE without redefining binding semantics.

### 6.3 Debug taxonomy (quick classification)

- **Goal drift / wrong target:** Intent & Objective harness
- **Infinite loop / invalid transitions:** Operational harness
- **Domain polluted with infra / untestable coupling:** Architectural harness
- **Type confusion / wrong ref semantics:** Ontological harness
- **Spec/runtime mismatch / projection drift:** Isomorphic harness
- **Silent meaning shift / version confusion:** Change harness
- **Agent smuggled authority / identity mismatch / oracle tamper:** Authority & Integrity harness
- **Runaway cost / endless retries:** Resource harness
- **Missing evidence / irreproducible claims:** Accountability harness

### 6.4 Governance alignment method

When people disagree about “what should happen,” first identify:

1) Which membrane is implicated.
2) Which commitment object is missing or underspecified.
3) Whether the issue is a binding conflict (route to SR-CHANGE) or an implementation gap.

---

## 7. Role in the governed stack (interpretation guidance)

SR-ETT is a **supporting topology lens**:

- **SR-CONTRACT** defines binding invariants (what MUST hold).
- **SR-SPEC** defines binding mechanics and schemas (how the invariants are realized).
- **SR-DIRECTIVE** defines execution discipline (how work is sequenced and gated).
- **SR-PLAN** defines what must be produced (deliverables and dependencies), intentionally without execution closure semantics.

SR-ETT’s contribution is to help authors and implementers place coercion boundaries so that:

- agent outputs remain proposals until converted into commitment objects,
- ports/adapters enforce admissibility at actor boundaries,
- and workflows (SR-DIRECTIVE) that execute the plan (SR-PLAN) remain congruent with contract/spec.

If SR-ETT ever appears to contradict binding artifacts, treat it as a signal to route the discrepancy through SR-CHANGE rather than patching meaning locally.

---

## 8. Normative section (minimal)

This section uses normative keywords intentionally and minimally.

- A change that affects binding meanings (types, authority boundaries, evidence requirements, lifecycle semantics) **MUST** be represented as a change record governed by SR-CHANGE.
- Commitments that cross trust boundaries (e.g., approvals, freezes) **MUST** be attributable to the correct actor class and recorded as durable records.
- This document **MUST NOT** be used to introduce new portal kinds or redefine existing binding semantics (e.g., “Verified,” “Approved,” “Shippable”).

---

## 9. References (informative)

- SR-TYPES (Type System + precedence)
- SR-INTENT (Design Intent)
- SR-PARADIGM (Development Paradigm)
- SR-CONTRACT (Architectural Contract)
- SR-SPEC (Technical Spec)
- SR-DIRECTIVE (Dev Directive)
- SR-PLAN (Plan / decomposition)
- SR-CHANGE (Change Management)
- SR-README (Registry / orientation)

---

## 10. Change control

This is a **supporting governed policy document**. Modifications must follow SR-CHANGE so that:

- the topology remains stable enough to serve as a shared lens,
- and any refinements to membrane definitions remain auditable and reconstructable.

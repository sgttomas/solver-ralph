---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-ETT"
  type: "governance.epistemic_trust_topology"
  title: "SOLVER-Ralph Epistemic Trust Topology"
  version: "1.0.1"
  status: "governed"
  normative_status: "directional"
  authority_kind: "process"
  governed_by:
    - "SR-CHANGE"
  supersedes: ["SR-ETT@1.0.0"]
  created: "2026-01-10"
  updated: "2026-01-10"
  tags:
    - "governance"
    - "policy"
    - "epistemology"
    - "trust"
    - "topology"
    - "harness"
  ext:
    intent:
      summary: >
        Defines the constraint topology (nine harness membranes) that converts fluid,
        stochastic proposals into durable, attributable commitments. “Trust” is treated
        as an emergent property of the combined membranes, not a single mechanism.
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

## 1. Core idea

SOLVER-Ralph separates work into two regimes:

- **Exploration (fluid meaning):** proposals, hypotheses, drafts, partial implementations.
- **Commitment (solid meaning):** durable statements that other work can rely on (e.g., “this candidate is the one we evaluated,” “these oracles produced this evidence,” “a human accepted responsibility for release”).

A **membrane** is the point where fluid meaning is converted into a commitment object.  
The system applies coercion primarily at membranes, not throughout exploration.

This document names nine orthogonal membranes together with their harnesses. The combined effect of the membranes is **epistemic trust**: what the system is allowed to treat as reliable knowledge.  This trust is an emergent property when the system validates the current state and then the human judges that the output is aligned with their intent and the objectives have been met.

---

## 2. Definitions

### 2.1 Proposal vs commitment

- **Proposal:** A candidate action or explanation produced by an agent (or human) that is not yet a binding fact.
- **Commitment:** A proposal that has been converted into a durable, attributable, replayable record in a form the system accepts as “real.”

### 2.2 Harness

A **harness** is a rule-enforced conversion point expressed in a specific, typed form preventing an agent or human from acting in a certain manner.


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
- and the membranes described here are the points where fluid proposals must be “written into” durable forms (events/evidence/approvals/freezes/changes) that carry references to the governing documents.

This is one of the primary reasons the topology harnesses without strangling: exploration can remain fluid inside an iteration, while anything that becomes load-bearing is anchored to referenceable, replayable artifacts.

---

## 4. The nine membranes (harnesses)

Each harness describes:
- what it constrains,
- where it lives (typical implementation surfaces),
- what commitment objects it produces or protects.

### 4.1 Intent & Objective harness

**What it constrains:** the reference signal: what “success” means and what tradeoffs are acceptable among compliant options.  
**Where it lives:** design intent artifacts, work_unit goal declarations, iteration start payload + refs.  
**Commitment objects:** explicit goal statements that are referenceable; scope/budget declarations; stop triggers tied to objective failure.

**Failure modes it prevents:**
- goal drift across long runs,
- proxy collapse (“tests passing” mistaken for “problem solved”),
- conflicting local objectives.

---

### 4.2 Operational harness

**What it constrains:** execution over time: how work advances and stops.  
**Where it lives:** loop/iteration lifecycle state machine; gate hooks; stop triggers; closeout procedures.  
**Commitment objects:** lifecycle events (loop created, iteration started, summary submitted), run lifecycle records.

**Failure modes it prevents:**
- unbounded “keep going” behavior,
- undefined handoffs between agents and humans,
- silent “done” states.

---

### 4.3 Architectural harness

**What it constrains:** where responsibilities may live; separation of concerns.  
**Where it lives:** hex boundaries; port/adapters; “domain core stays infra-free.”  
**Commitment objects:** boundary tests; dependency rules; build-time enforcement.

**Failure modes it prevents:**
- orchestration logic leaking into invariant-bearing domain logic,
- “implementation gravity” pulling everything into one layer,
- untestable coupling.

---

### 4.4 Ontological harness

**What it constrains:** what kinds of objects exist and what relations mean.  
**Where it lives:** artifact type system; ref typing; relationship semantics (dependency vs provenance).  
**Commitment objects:** typed refs; typed relationships; explicit categories for evidence/records/config.

**Failure modes it prevents:**
- vocabulary drift (“approval” used where only a non-binding review exists),
- semantic ambiguity becoming institutional fact,
- staleness and dependence being conflated with audit provenance.

---

### 4.5 Isomorphic harness

**What it constrains:** drift between the governed model and the implemented system.  
**Where it lives:** schema alignment; “governance ↔ runtime” mapping; projection rebuild guarantees.  
**Commitment objects:** generated schemas; conformance tests; projection invariants.

**Failure modes it prevents:**
- “spec says X, system does Y” without detection,
- informal side channels becoming de facto behavior,
- governance turning into narrative rather than executable constraint.

---

### 4.6 Change harness

**What it constrains:** how the system (and its governance) evolves without losing meaning.  
**Where it lives:** change requests; versioning rules; migration records; freeze/baseline procedures.  
**Commitment objects:** change records; supersedence chains; declared compatibility.

**Failure modes it prevents:**
- silent reinterpretation of words and states,
- untracked breaking changes,
- accumulating “tribal knowledge” that cannot be reconstructed.

---

### 4.7 Authority & Integrity harness

**What it constrains:** who may commit binding state and what claims are authentic and untampered.  
**Where it lives:** actor roles; endpoint permissions; event emission constraints; sandbox integrity; identity verification.  
**Commitment objects:** authoritative events; portal approval records; integrity evidence (hashes, digests, environment fingerprints).

**Failure modes it prevents:**
- unauthorized commitments (agents performing human-only actions),
- forged or replayed claims,
- identity confusion (candidate/evidence mismatch),
- unsafe execution outside containment.

---

### 4.8 Resource harness

**What it constrains:** feasibility: cost, time, and concurrency.  
**Where it lives:** budgets per work_unit/loop/iteration; quotas; stop triggers; queueing/backpressure.  
**Commitment objects:** budget declarations; spend ledgers; stop-trigger events.

**Failure modes it prevents:**
- runaway exploration,
- starvation of higher-priority work,
- “infinite debugging” loops.

---

### 4.9 Accountability harness

**What it constrains:** legibility and replayability: what must be recorded for claims to count.  
**Where it lives:** evidence bundle manifests; completeness checks; invariants; rebuildable projections.  
**Commitment objects:** evidence bundles; manifests; deterministic context digests; audit-ready projections.

**Failure modes it prevents:**
- “it passed” without proof objects,
- irreproducible evaluations,
- missing links between approvals and evidence.

---

## 5. Membrane placement heuristics (practical)

When deciding whether to introduce a new coercion point, ask:

1) **Will downstream work rely on this as true?**  
2) **Does this cross an actor boundary (SYSTEM ↔ worker ↔ human ↔ oracle)?**  
3) **Can this be expressed as a small, typed commitment object?**  
4) **Is there an honest relief valve when requirements cannot be met?**  
5) **Does the change increase forced surface area inside exploration?** If yes, it risks strangling.

A default bias:
- keep exploration permissive inside iterations,
- harden at claim boundaries (identity, evidence, approval, freeze, change).

---

## 6. How to use this document

This topology is intended to be used as:

- **A design checklist for agents** confirm each new capability respects all nine membranes (even if trivially).
- **A debugging lens:** classify failures by membrane (“goal drift,” “integrity gap,” “accountability gap,” etc.).
- **A governance alignment tool:** when people disagree, identify which membrane is under-specified rather than arguing in prose.

---

## 7. Normative section (minimal)

This section uses normative keywords intentionally and minimally.

- A change that affects binding meanings (types, authority boundaries, evidence requirements, lifecycle semantics) **MUST** be represented as a change record governed by SR-CHANGE.
- Commitments that cross trust boundaries (e.g., approvals, freezes) **MUST** be attributable to the correct actor class and recorded as durable records.
- This document **MUST NOT** be used to introduce new portal kinds or redefine existing binding semantics (e.g., “Verified,” “Approved,” “Shippable”).

---

## 8. References (informative)

- SR-INTENT (Design Intent)
- SR-CONTRACT (Architecture Contracts)
- SR-SPEC (Technical Spec)
- SR-TYPES (Type System)
- SR-DIRECTIVE (Dev Directive)
- SR-CHANGE (Change Management)
- README (Orientation)

---

## 9. Change control

This is a **supporting governed policy document**. Modifications must follow SR-CHANGE so that:
- the topology remains stable enough to serve as a shared lens,
- and any refinements to membrane definitions remain auditable and reconstructable.

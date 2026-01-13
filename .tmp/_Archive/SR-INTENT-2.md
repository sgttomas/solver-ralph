---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-INTENT"
  type: "governance.design_intent"
  title: "SOLVER-Ralph Design Intent"
  version: "1.3.0-draft.1"
  status: "draft"
  normative_status: "directional"
  authority_kind: "content"
  governed_by: ["SR-CHANGE"]
  supersedes: ["SR-INTENT@1.2.0-draft.1"]
  created: "2026-01-09"
  updated: "2026-01-12"
  tags: ["solver-ralph", "design-intent", "governance", "agentic-coding", "ralph-loop", "hexagonal-architecture", "trust-boundaries", "state-machine", "deterministic-progression"]
  ext:
    design_intent:
      primary_problem: "How to generate trustworthy knowledge artifacts using non-deterministic agents."
      scope_notes:
        - "This document provides rationale and interpretation guidance for the SOLVER-Ralph platform."
        - "Binding constraints belong in SR-CONTRACT and SR-SPEC."
        - "Agent constraints belong in SR-AGENTS."
      assumptions:
        - id: "A-001"
          statement: "Deterministic oracles can be defined for the majority of intended work."
          trigger: "Frequent ORACLE-GAP escalations or inability to define executable gates for core requirements."
        - id: "A-002"
          statement: "Human approvals at portals are available for non-oracle-verifiable acceptability judgments."
          trigger: "Operational reality prevents timely human review, or approvals become rubber-stamped and meaningless."
        - id: "A-003"
          statement: "Agents remain non-authoritative; their outputs are treated as proposals until verified and approved."
          trigger: "Systems or teams begin treating agent statements (\"it's compliant\") as equivalent to verifiable evidence or contractual authority."
        - id: "A-004"
          statement: "State transition rules can be defined deterministically even when actor decisions are non-deterministic."
          trigger: "State machine semantics become ambiguous or unenforceable, allowing invalid transitions."
        - id: "A-005"
          statement: "Work can be decomposed into loop-completable units (Ralph Loops) such that per-unit success probability is high."
          trigger: "Work units remain too large or ambiguous to reach Verified outcomes within budgets; repeated loop failures without convergence."
---

# SOLVER-Ralph Design Intent — v1.3.0-draft.1

**Purpose:** Design rationale and first principles for SOLVER-Ralph: a platform for controlled agentic work using Ralph Loops (fresh-context iteration) with oracle-defined verification and human approvals at trust boundaries.

**Normative Status:** Directional (non-binding). This document explains *why* the platform works the way it does. Binding constraints belong in the **Architectural Contract** and **Technical Specification**.

**Audience:** Architects, engineers, and agents who need to understand *why the platform rules exist*.

---

## 1. The Core Problem

### 1.1 What We're Protecting Against

SOLVER-Ralph exists to produce **high-assurance work** using agents while preserving human authority where determinism cannot be guaranteed.

The critical failure mode is not "the agent writes a bug." The critical failure mode is:

> **A false claim of verification or authorization becomes a foundation for downstream work.**

Examples of "foundation corruption":
- work shipped because an agent *claimed* tests passed (but they didn't),
- approvals recorded against the wrong artifact state or wrong version,
- agents making authority claims they're not permitted to make,
- evidence referenced but not actually present.

This is a propagation risk: once the wrong foundation is accepted, downstream work becomes unreliable.

### 1.2 The Cost Asymmetry

SOLVER-Ralph assumes an extreme cost asymmetry:

| Outcome | Cost | Why it matters |
|---|---:|---|
| Wrong approval / wrong baseline | Catastrophic | Silent drift; bad state becomes "truth" |
| Blocked progress due to conservative governance | Minor | Friction; re-run loop; retry |
| Extra escalation / "ask human too often" | Low–Medium | Time cost; improves clarity over time |

**Design direction:** prefer conservative blocking and escalation over permissive acceptance. Iteration is cheap; wrong work can be practically unbounded in cost.

---

## 2. Design Inputs

### 2.1 Trust Model

| We do not rely on… | Because |
|---|---|
| Agent "confidence" or self-assessment | It is non-deterministic and not authoritative |
| Prompt rules as security guarantees | They are suggestions, not enforcement |
| "Tests exist" as a proxy for verification | Passing evidence is what matters, not existence |

| We can rely on… | Because |
|---|---|
| Deterministic oracle outputs | They provide repeatable pass/fail evidence |
| Explicit human approvals at portals | They establish authorization |
| Deterministic state transition rules | They make validity enforceable regardless of actor decisions |
| Event-sourced state | It provides complete audit trail |

### 2.2 Failure Modes Under Consideration

#### Foundation corruption
**Scenario:** approval or baseline created against stale/incorrect artifact content.  
**Design direction:** approvals and baselines are coupled to immutable identifiers + evidence.

#### Oracle gaming and oracle erosion
**Scenario:** tests/scans weakened, bypassed, or suppressed to "get green."  
**Design direction:** oracle integrity is treated as a first-class concern.

#### Agent authority claims
**Scenario:** agent asserts "this is verified" or "this is approved" without evidence or human action.  
**Design direction:** agents cannot claim Verified/Approved/Shippable; those states require evidence or portal action.

#### Evidence availability
**Scenario:** claim references evidence that cannot be retrieved.  
**Design direction:** EVIDENCE_MISSING blocks progression; phantom proof objects cannot become foundations.

#### Overbaking / context collapse in long agent loops
**Scenario:** long-running conversations produce incoherent compounding changes.  
**Design direction:** Ralph Loops: iterate in fresh context, bounded iterations.

---

## 3. Key Decisions

### 3.1 Separate "generation" from "evaluation" from "authority"

**Problem:** agents are powerful generators, but not reliable judges of their own outputs. Human interpretation of evidence is not the same as human approval.

**Decision:** establish a 4-way distinction:

| Activity | Actor | What it produces | Is it binding? |
|----------|-------|------------------|----------------|
| **Verification** | oracles | Evidence about conformance | No (evidence, not authority) |
| **Evaluation** | human | Interpretation of verification evidence | No (informs, but not approval) |
| **Validation** | oracles | Evidence about fitness in context | No (evidence, not authority) |
| **Assessment** | human | Interpretation of validation evidence | No (informs, but not approval) |
| **Approval** | human at portal | Authorization to proceed | **Yes** (trust boundary action) |

**Rationale:** This separation prevents conflation errors:
1. Agent claims treated as evidence (they are proposals)
2. Evidence treated as authority (evidence informs; approval binds)
3. Human interpretation treated as approval (evaluation/assessment inform; portal approval binds)

### 3.2 Oracle-defined verification, not oracle-defined truth

**Problem:** "done = tests pass" can be confused as "correctness proven."  
**Decision:** define **Verified** strictly as "oracles passed for the declared scope," not "the system is correct."  
**Rationale:** oracle completeness is not knowable. Verification is bounded and honest; approval is a separate act.

### 3.3 Human portals exist for acceptability, ambiguity, and high stakes

**Problem:** not all requirements are fully oracle-verifiable.  
**Decision:** preserve explicit **human approval portals** as part of normal operation.  
**Rationale:** human judgment is an irreducible primitive in high-stakes systems; make it explicit, minimal, and auditable.

### 3.4 Hexagonal architecture as enforcement boundary

**Problem:** governance logic becomes brittle if interwoven with infrastructure.  
**Decision:** define domain rules and state machines independent of adapters; treat persistence, LLM runtimes, and UIs as replaceable.  
**Rationale:** makes verification and audit easier; prevents tool coupling from becoming governance coupling.

```
  outside actors (Humans / Agents / Oracles)
              │
              ▼
        [Driving ports]  ←─ (trust boundaries enforced here)
              │
              ▼
          Domain core    ←─ (invariants + state machines)
              │
              ▼
         [Driven ports]  ←─ (evidence/event storage)
              │
              ▼
            Adapters
```

### 3.5 Ralph Loops as bounded iteration

**Problem:** long-running agent sessions accumulate errors; "sticky" mistakes compound.  
**Decision:** structure work as **Ralph Loops** — bounded iterations with fresh context.  
**Rationale:** 
- Short loops prevent error accumulation
- Fresh context prevents "overbaking"
- Bounded iterations are auditable
- State is explicit (events), not implicit (conversation history)

### 3.6 Event-sourced state

**Problem:** mutable state makes audit difficult; "what happened" becomes ambiguous.  
**Decision:** all state changes recorded as immutable events; current state computed from event history.  
**Rationale:**
- Any state can be reconstructed by replay
- Audit trail is complete
- Same events in same order → same state (deterministic)

### 3.7 Agents cannot claim authority

**Problem:** agents might assert "tests passed" or "this is approved" without evidence or human action.  
**Decision:** agents can produce work and request verification/approval; they cannot claim Verified/Approved/Shippable states.  
**Rationale:** authority claims require either oracle evidence (Verified) or human action (Approved). Agent narrative is not evidence.

### 3.8 Trust Boundaries and Harnesses

**Problem:** where should the platform enforce constraints? Too much enforcement strangles exploration; too little allows foundation corruption.

**Decision:** separate work into two regimes:

| Regime | Character | Enforcement |
|--------|-----------|-------------|
| **Exploration** | Fluid; proposals, drafts, iterations | Permissive within loop constraints |
| **Commitment** | Solid; durable statements downstream work relies on | Strict at boundary crossing |

A **harness** is the enforcement mechanism at the boundary where exploration becomes commitment. The platform applies coercion primarily at these boundaries, not throughout exploration.

**Key commitment objects:**
- **Candidate** — content-addressed snapshot of work
- **Evidence Bundle** — oracle output proving verification
- **Portal Decision** — human authorization record
- **Event** — recorded state change

**Rationale:** coercion is valuable only when it prevents false foundations. Proposals inside a loop don't need enforcement; claims that cross boundaries do.

### 3.9 Coercion at Boundaries, Not Throughout

**Problem:** how to decide where to add enforcement?

**Decision:** apply coercion when outputs become inputs — when something becomes load-bearing for downstream work.

**Placement heuristics:**
1. Will downstream work rely on this as true?
2. Does this cross an actor boundary (system ↔ agent ↔ human ↔ oracle)?
3. Can this be expressed as a typed commitment object?

**Corollaries:**
- Coerce **claims**, not creative exploration
- Keep coercion objects small, typed, and replayable
- Provide relief valves (exception records) when constraints cannot be met

**Hexagonal alignment:** actor boundaries are implemented as ports/adapters. Trust boundaries are enforced at ports, with invariants in the domain core.

### 3.10 Deterministic State Progression with Non-Deterministic Actors

**Problem:** actors (humans, agents) make non-deterministic decisions, but state validity must be enforceable.

**Decision:** separate the state transition rules (deterministic) from the actor decisions that trigger transitions (non-deterministic).

**Key properties:**

| Property | What it means |
|----------|---------------|
| **Rules are deterministic** | Given a state and an action, whether the transition is valid is computable |
| **Actor decisions are non-deterministic** | Which actions actors choose is not predetermined |
| **Resulting state is deterministic given inputs** | Same actions replayed → same state |
| **Validity is system-enforced** | Invalid transitions rejected regardless of actor intent |

**Rationale:** The platform doesn't need to predict what actors will do; it only needs to validate what they attempt. This achieves enforcement without omniscience.

---

## 4. Trade-offs

SOLVER-Ralph optimizes for safe iteration, not fastest path.

**Trade-offs we accept:**
- More portal friction in exchange for bounded authority and defensible decisions
- More explicit escalation in exchange for fewer silent failures
- Deterministic rules limiting actor freedom in exchange for enforceable validity and auditable paths
- Fresh-context loops (potential rework) in exchange for preventing error accumulation

**Trade-offs we avoid:**
- Pretending that prompt instructions create determinism
- Collapsing verification and authorization into "agent says it's done"
- Treating human interpretation as equivalent to approval
- Allowing actor decisions to bypass state transition rules

---

## 5. Rejected Alternatives

### 5.1 Prompt-only governance
Rejected because it does not produce deterministic enforcement and cannot be treated as a security boundary.

### 5.2 Agent self-approval
Rejected because approval is a trust boundary action; it creates new binding state and must remain human-authorized.

### 5.3 Monolithic long-context agent sessions
Rejected because long sessions accumulate errors; fresh-context iteration is preferred.

### 5.4 Treating evaluation/assessment as approval
Rejected because conflating human interpretation with human authorization creates false foundations.

### 5.5 Non-deterministic state transition rules
Rejected because enforcement becomes impossible and audit becomes meaningless.

### 5.6 Coercion throughout exploration
Rejected because it strangles creative work without preventing foundation corruption. Coerce at claim boundaries, not throughout.

### 5.7 Separating governance from enforcement
Rejected because the platform must embody the rules, not just document them. Specifications become code; the running system enforces constraints.

---

## 6. Assumptions & Re-evaluation Triggers

Assumptions and triggers are listed in the YAML frontmatter under `ext.design_intent.assumptions`.

If a trigger is observed:
- create a Decision Record or Deviation Record,
- route implications through Change Management,
- update this Design Intent if the rationale has changed.

---

## 7. Terminology (Cross-Reference)

This is a directional document; it does not define binding vocabulary.

For authoritative definitions, see:

| Term | Authoritative Source |
|------|---------------------|
| Verified, Approved, Shippable | SR-CONTRACT |
| Proposal, Commitment Object | SR-CONTRACT |
| Verification, Evaluation, Validation, Assessment | SR-CONTRACT |
| Oracle, Portal, Candidate, Evidence Bundle | SR-CONTRACT, SR-SPEC |
| Agent invariants | SR-AGENTS |
| Work Unit, Loop, Event | SR-SPEC |

---

## 8. What This Document Is Not

- It is not a requirements list
- It is not a substitute for Contract or Spec
- It is not an agent definition (that's SR-AGENTS)
- It is not the software implementation

It is the reasoning that explains why the platform works the way it does.

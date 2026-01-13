---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-AGENTS"
  type: "governance.agents"
  title: "SOLVER-Ralph Agents"
  version: "1.1.0-draft.2"
  status: "draft"
  normative_status: "normative"
  authority_kind: "process"
  governed_by:
    - "SR-CHANGE"
  supersedes:
    - "SR-AGENTS@1.1.0-draft.1"
  created: "2026-01-11"
  updated: "2026-01-11"
  tags:
    - "solver-ralph"
    - "governance"
    - "agents"
    - "actor-kinds"
    - "trust-boundary"
    - "hexagonal-architecture"
    - "membranes"
  ext:
    agents:
      invariants:
        - "agents_produce_proposals"
        - "agents_cannot_assert_verified"
        - "agents_cannot_assert_approved"
        - "agents_cannot_assert_shippable"
        - "agents_cannot_cross_portal_boundaries"
      evidence_expectations: []
    agent_model:
      summary: >
        Defines what an Agent is in the SOLVER‑Ralph paradigm as a single actor kind
        (AGENT) whose outputs are proposals and whose effects on system state occur
        only through coerced interfaces (ports, gates, oracles, and human portals).
        This artifact specifies the ontological and epistemic boundaries of agents,
        and how those boundaries are made structurally enforceable via SR-ETT membranes.
      scope:
        - "Applies to all SOLVER‑Ralph implementations and governed workflows."
        - "Defines agent semantics and constraints; does not define workflow parameters."
      aligns_with:
        - "SR-CONTRACT (actors, trust boundaries, binding semantics)"
        - "SR-SPEC (actor identity, ContextCompiler semantics, Agent Worker integration)"
        - "SR-ETT (membrane topology; coercion placement)"
        - "SR-TYPES (config.agent_definition semantics)"
        - "SR-DIRECTIVE (instance workflows; gate/portal control surfaces)"
---

# SOLVER-Ralph Agents (SR-AGENTS)

## 0. Purpose, scope, and precedence

### 0.1 Purpose

SR-AGENTS defines the **agent concept** in SOLVER‑Ralph: what an agent is, what its outputs mean, and why agent authority is constrained.

This document is intentionally **not** a workflow manual. Execution policies (budgets, gates, portal routing) live in SR‑DIRECTIVE for a given instance.

### 0.2 Precedence

If there is a conflict, resolve in this order:

1. **SR-CONTRACT** (binding invariants; what must always be true)
2. **SR-SPEC** (binding mechanics; how invariants are realized)
3. **SR-CHANGE** (change routing for governed artifacts)
4. **SR-DIRECTIVE** (instance operational policy; how the system is run)
5. **SR-ETT** (membrane topology lens; where coercion belongs)
6. **SR-INTENT** (directional rationale)
7. **SR-README** (narrative overview)

SR-AGENTS must remain consistent with SR-CONTRACT/SR-SPEC. Any needed semantic changes must route through SR‑CHANGE.

---

## 1. Agent as an actor kind

### 1.1 Actor kinds

SOLVER‑Ralph distinguishes actor kinds at trust boundaries:

- **HUMAN**: attributable, can perform portal-bound binding actions.
- **SYSTEM**: deterministic control-plane and projection computation.
- **ORACLE**: deterministic verifiers that produce evidence (not authority).
- **AGENT**: non-authoritative worker that produces proposals.

This artifact is about **AGENT**.

### 1.2 Definition: Agent

An **Agent** is an actor-kind whose outputs are **non-binding proposals**.

- Agents may draft artifacts, analyses, patches, and candidate changes.
- Agents may request or suggest verification activity.
- Agents may never create binding records (e.g., approvals, freezes, governance changes).
- Agents are not a privileged source of truth; their statements have no institutional status.

> Note: stochasticity is a common implementation detail, but **not** the defining property.
> The defining property is **non-authority** and membrane-constrained interaction with the system.

---

## 2. Epistemology: why agents cannot create “truth” in SOLVER‑Ralph

### 2.1 Institutional truth is constituted by records

In SOLVER‑Ralph, institutional claims (e.g., “Verified”, “Approved”, “Shippable”) attach to artifacts only when the system has the required commitment objects (evidence bundles, approvals, freeze records) as defined by SR-CONTRACT and realized by SR-SPEC.

An agent’s narrative (“tests passed”, “approved”, “ready to ship”) is always a proposal unless it is grounded in the relevant recorded commitment objects.

### 2.2 Prohibition vs structural impossibility

SR-AGENTS treats agent authority limits as **structural impossibilities**, not behavioral rules:

- At the **Authority & Integrity membrane**, portals accept only HUMAN actor-kind for binding submissions; agent-sourced submissions are rejected.
- At the **Accountability membrane**, verification claims require oracle-produced evidence objects; agent text cannot substitute.
- At the **Change membrane**, modifications to governed artifacts require the governance change workflow (human approvals and recorded change objects).

The system is designed so that “agent authority leakage” is prevented by coercion at membrane boundaries, not by trusting agent compliance.

---

## 3. Hexagonal architecture placement

Agents live outside the domain core, interacting through coercion surfaces.

### 3.1 Agents are adapters behind driving ports

In hexagonal terms:

- The **domain core** holds invariant logic and state semantics.
- **Driving ports** accept commands/requests that propose state transitions.
- **Adapters** implement external actors and tooling.

An Agent is an adapter-side worker whose outputs become meaningful only when admitted through ports and gates.

### 3.2 Membranes are typically enforced at ports and gateways

SR‑ETT’s membranes are commonly implemented as:

- **Port validation** (schema/type/actor-kind checks)
- **Gate evaluation** (required evidence present, oracle suite pinned, staleness resolved)
- **Portal decisions** (human approvals/waivers/decisions, attributable identity)

SR‑DIRECTIVE configures the operational parameters; SR‑AGENTS defines what “agent” means relative to those boundaries.

---

## 4. Uniform agent ontology

SOLVER‑Ralph does **not** ontologically recognize “subagent types” as first‑class governance concepts.

All agents share the same modality:

- consume task context (via system-provided context compilation),
- produce proposals,
- interact with the harness only through coerced interfaces.

What varies across deployments is not agent ontology, but the **work envelope**: task, context, tool access, required output form, and required verification.

---


### 4.3 Membrane → agent boundary map (SR-ETT cross-reference)

This table cross-references the **named SR-ETT membranes by their `membrane_id`** (as used in SR‑ETT’s harness registry) and states the corresponding **agent-side boundary**. It does **not** introduce agent subtypes; it applies uniformly to the single `AGENT` actor kind.

| SR-ETT membrane_id | SR-ETT membrane name | Agent boundary (uniform across all agents) | Primary enforcement surface |
|---|---|---|---|
| `intent_objective` | Intent & Objective | Agent may propose goals/plans, but **cannot instantiate or change the binding objective** for a work_unit; goal/acceptance criteria are treated as commitments only when carried in governed artifacts / loop metadata and referenced by `IterationStarted.refs[]`. | Driving-port admission rules; loop/work_unit metadata; directive gates for drift |
| `operational` | Operational | Agent may propose steps, but **cannot advance loop / iteration lifecycle state** or override stop triggers/budgets; operational progression is a SYSTEM state-machine concern. | Domain-core state machine; governor; stop-trigger routing |
| `architectural` | Architectural | Agent may propose patches, but **cannot bypass hex boundaries** (domain purity, ports/adapters separation) and cannot make “architecture-compliant” claims binding; compliance must be established by conformance oracles. | Architecture boundary tests; build/lint/static checks as oracles |
| `ontological` | Ontological | Agent may draft artifacts, but **cannot mint new governed type meanings** or treat ad‑hoc schemas as authoritative; all typed records/events must conform to SR‑TYPES/SR‑SPEC. | Schema/type validation at ports; event validation; governed schema checks |
| `isomorphic` | Isomorphic | Agent may describe reality/behavior, but **cannot establish model↔runtime correspondence** by assertion; drift/conformance is demonstrated via deterministic tests and reconstruction. | Conformance test oracles; projection rebuild/replay checks |
| `change` | Change | Agent may propose governance edits, but **cannot cause a governance semantic change** without SR‑CHANGE routing and human portal decision. | GovernanceChangePortal workflow; SR‑CHANGE routing rules |
| `authority_integrity` | Authority & Integrity | Agent **cannot author binding approvals, decisions, waivers, freezes**, or other portal-gated commitments; actor-kind restrictions and integrity checks make this structurally inadmissible. | Portal actor-kind enforcement; identity/integrity checks; admission control |
| `resource` | Resource | Agent operates within declared budgets; it **cannot allocate extensions** or exceed limits without an explicit human decision path. | Budget accounting; BUDGET_EXHAUSTED stop-trigger; extension portal routing |
| `accountability` | Accountability | Agent outputs are proposals; **verification/validation is only institutional via oracle evidence** and durable evidence bundles; “ghost inputs” are disallowed—context must be derivable from `IterationStarted` payload+refs. | Evidence bundle requirements; oracle runs; deterministic context compilation; append-only event log |

## 5. Work envelopes (variation without agent subtypes)

A **Work Envelope** is the unit of variation that constrains agent activity without introducing agent taxonomies.

A work envelope is defined by:

1. **Task statement**: what the iteration is trying to produce (from SR‑PLAN + SR‑DIRECTIVE).
2. **Context profile**: the authoritative input ref-set and constraints (SR‑SPEC ContextCompiler semantics).
3. **Allowed interfaces**: which commands/ports the agent may invoke (SR‑SPEC + SR‑DIRECTIVE).
4. **Expected outputs**: what artifact forms are acceptable (candidate changes, draft docs, structured packets).
5. **Verification profile**: which oracle suite(s) are required/advisory and when (SR‑DIRECTIVE).
6. **Budgets and stop triggers**: operational bounds (SR‑DIRECTIVE).
7. **Escalation routing**: which portal(s) handle blocks and decisions (SR‑DIRECTIVE).

Agents may propose envelope adjustments (e.g., “include these refs”, “run these oracles”), but those proposals have no effect unless admitted through the relevant membranes.

---

## 6. System vs agent responsibilities

### 6.1 Deterministic context compilation is a SYSTEM responsibility

The SYSTEM compiles the authoritative context bundle deterministically from:

- the iteration payload,
- `IterationStarted.refs[]`,
- and governed context rules (per SR‑SPEC).

Agents may **suggest** additional context, but they must not be treated as the context compiler. Any new context must be admitted through the appropriate workflow (e.g., next iteration context refs; governed artifact updates through SR‑CHANGE).

### 6.2 Evidence and judgment separation

Agents may:
- summarize evidence,
- propose interpretations,
- draft portal packets (e.g., “what I think should be approved and why”),

but agents must not be treated as issuers of **human judgment records** or binding approvals.

Human judgment (evaluation, assessment, approval, waiver decisions) is performed by HUMAN actors at defined portals and recorded as commitment objects.

---

## 7. Provenance and reference discipline

### 7.1 Agent configuration provenance

Agent configuration artifacts (e.g., `config.agent_definition`) are typically **audit provenance**:

- Prefer `supported_by` relationships unless the system’s correctness genuinely depends on a specific agent configuration.
- Do not make execution correctness depend on mutable or informal agent-side state.

### 7.2 No ghost inputs

All agent-relevant authoritative inputs must be derivable from the iteration payload and refs. If additional inputs are needed, the workflow must:
- record them as proposal artifacts, and
- admit them through the correct membrane (e.g., next iteration refs, governance change).

---

## 8. Failure modes and guardrails (membrane-oriented)

Common failure patterns and the membrane that should stop them:

- **Authority smuggling** (agent text treated as approval) → Authority & Integrity membrane / portal actor-kind checks.
- **Evidence smuggling** (agent claim treated as verification) → Accountability membrane / oracle evidence requirements.
- **Semantic drift** (meaning changes without governance) → Change membrane / SR‑CHANGE routing.
- **Ghost inputs** (untracked dependencies) → Accountability + Ontological membranes / deterministic context compilation and typed refs.

---

## 9. Change log (this revision)

- Removed agent subtype taxonomy and cohort model; SOLVER‑Ralph treats agent as a single actor kind and pushes variation into work envelopes.
- Reframed constraints as structurally enforced via SR‑ETT membranes and hexagonal ports, not as prohibitions an agent might violate.
- Clarified SYSTEM ContextCompiler vs agent “context suggestions”.
- Raised SR‑ETT above SR‑INTENT in precedence and added explicit alignment references.


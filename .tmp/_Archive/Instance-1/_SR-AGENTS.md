---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-AGENTS"
  type: "policy.agent_taxonomy"
  title: "SOLVER-Ralph Agents"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "normative"
  authority_kind: "content"
  governed_by: ["SR-CHANGE"]
  created: "2026-01-10"
  updated: "2026-01-10"
  tags:
    - "solver-ralph"
    - "governance"
    - "agents"
    - "taxonomy"
    - "agent-worker"
    - "agent-definition"
    - "trust-boundary"
  ext:
    agent_taxonomy:
      summary: >
        Defines what an Agent is in the SOLVER‑Ralph paradigm as an object category
        (actor kind) and specifies a foundational taxonomy of agent subtypes.
        This artifact focuses on ontology (what an agent is), not on workflow mechanics.
      scope:
        - "Applies to all SOLVER‑Ralph implementations and SOLVER‑Ralph-governed workflows."
        - "Defines agent object model + subtypes used by the paradigm."
      aligns_with:
        - "SR-CONTRACT (actors + trust boundaries)"
        - "SR-SPEC (actor identity + Agent Worker integration + no-ghost-inputs)"
        - "SR-TYPES (config.agent_definition semantics)"
        - "SR-PARADIGM (agent waves/cohorts)"
---

# SOLVER-Ralph Agents (SR-AGENTS) — v1.0.0-draft.1

**Purpose:** Define the **Agent** object category in the SOLVER‑Ralph paradigm and specify a small, foundational set of **agent subtypes** that the paradigm expects to exist (conceptually and/or operationally).

**Normative status:** **Normative (binding) for terminology and classification** within this artifact’s scope.  
**Important constraint:** This document MUST NOT introduce new trust-boundary authorities or redefine **Verified / Approved / Shippable**. Binding semantics for those terms remain in **SR-CONTRACT** and **SR-SPEC**.

**Primary question:** “What kind of thing is an *Agent* here?”

---

## 0. How to interpret this document

### 0.1 What this document is (and is not)

This document **is**:

- an ontology for “Agent” as a first-class object category,
- a taxonomy of agent subtypes that are core to SOLVER‑Ralph,
- a set of scope boundaries that prevent category errors (“agent” vs “system” vs “oracle” vs “human”).

This document **is not**:

- a prompt catalog,
- an operational guide for running agents,
- a definition of portal semantics or oracle semantics,
- a definition of memory rules beyond what is needed to keep “agent” distinct from “system” and “governance artifacts.”

### 0.2 Compatibility and precedence

If anything in this document conflicts with binding artifacts, the binding precedence remains:

1) SR-CONTRACT  
2) SR-SPEC  
3) SR-DIRECTIVE  
4) SR-INTENT / SR-ETT  
5) SR-README

Conflicts MUST be resolved via SR-CHANGE; do not resolve by interpretation.

---

## 1. Definitions

### 1.1 Actor kinds (context)

SOLVER‑Ralph uses three actor kinds:

- **HUMAN** — can cross trust boundaries via portal approvals; carries responsibility.
- **SYSTEM** — orchestrates, enforces, records; emits SYSTEM-only events.
- **AGENT** — provides labor; proposes actions; may generate drafts and implementations.

This document defines “Agent” and its subtypes; it does not redefine Human or System.

### 1.2 Agent (base definition)

An **Agent** is a **stochastic generator** instantiated as a runtime actor (`actor_kind=AGENT`) that can:

- propose actions and interpretations,
- draft or modify artifacts (code and/or governance-adjacent documents),
- register candidates and request evaluation runs *through the system interfaces*,
- produce structured iteration outputs (summaries, next steps, open risks),
- but **cannot** create binding authority at trust boundaries.

#### 1.2.1 Agents produce proposals; they do not produce institutional facts

An agent statement such as “tests passed,” “compliant,” “approved,” or “ready to ship” is always a **proposal** unless it is grounded in:

- recorded oracle evidence (Evidence Bundles), and/or
- recorded human approvals at portals (Approval Records),
- and properly linked to the candidate/governed artifacts in force.

This is a definition-level property of Agents in SOLVER‑Ralph: they are **generators**, not **judges**.

### 1.3 Agent vs System vs Oracle vs Human (scope boundary)

To avoid “foundation corruption,” SOLVER‑Ralph treats these as distinct object categories:

- **Agent**: proposes and executes work; cannot grant authority.
- **System**: emits authoritative orchestration events and enforces invariants; cannot substitute for humans.
- **Oracle**: deterministic evaluator(s) that produce evidence; an oracle is a *mechanism*, not an actor category.
- **Human**: the only actor who can approve trust-boundary actions.

**Prohibition:** A deployed component MUST NOT simultaneously present itself as both `SYSTEM` and `AGENT` for the same action stream. If a component performs SYSTEM duties, it is SYSTEM, not AGENT.

---

## 2. Agent object model

This section defines the minimal object model that allows Agents to be audited and substituted safely.

### 2.1 Agent Instance

An **Agent Instance** is a specific runtime instantiation of an agent, recorded as:

- `actor_kind = AGENT`
- `actor_id = agent:<deployment_id>:<agent_instance_id>`

An Agent Instance is expected to be ephemeral and replaceable.

**Design intent:** The system must be able to answer: *“Which agent instance produced this output?”* without relying on transcript memory.

### 2.2 Agent Deployment

An **Agent Deployment** is the stable container/environment and identity namespace within which Agent Instances are created (e.g., a worker fleet, a specific orchestrated service, a pinned container image digest, etc.).

**Requirement:** A deployment SHOULD be stable enough that different Agent Instances can be compared and audited, but the paradigm does not require a single global deployment model.

### 2.3 Agent Role

An **Agent Role** is a stable semantic label describing the intended function of an agent subtype instance (e.g., “co-developer,” “governance scribe,” “context scout”).

Roles are **not authority**: they describe intent, not permissions. Permissions are enforced by the SYSTEM + API boundary + portal rules.

### 2.4 Agent Definition (configuration artifact)

An **Agent Definition** is a versioned, auditable configuration artifact (typically `config.agent_definition`) that parameterizes an Agent Worker’s behavior **without creating new authority**.

At minimum, an Agent Definition SHOULD declare:

- `agent_role_id`
- `capability_profile_id`
- `context_profile_id` or `context_profile_rules`
- `tool_access_profile_id`
- references to any prompt/template catalogs (if used)

**Audit semantics:** Agent Definitions are typically referenced as **audit provenance** (`rel=supported_by`) rather than semantic dependency.

---

## 3. Foundational taxonomy of agents

This section defines the subtypes of Agents that are core to the SOLVER‑Ralph paradigm. These are “foundational” in the sense that SOLVER‑Ralph’s governance + loop design explicitly anticipates their existence and benefits from their separation.

### 3.1 Taxonomy axes

Agents can be classified along two orthogonal axes:

1) **Plane of operation**
   - **Work-plane**: agents that execute work inside iterations (code, drafting, integration tasks).
   - **Advisory-plane**: agents that propose next actions, critique plans, or summarize evidence, but do not execute changes.

2) **Primary function**
   - Build / implement
   - Curate context
   - Evaluate evidence (non-binding)
   - Author governance drafts (non-binding)
   - Triage decisions (non-binding)

A single deployed agent instance MAY play multiple functions, but SOLVER‑Ralph’s governance benefits from **conceptual separation**, because it prevents accidental elevation of agent output into authority.

---

## 4. Core agent subtypes

### 4.1 Agent Worker (work-plane)

**Stable subtype id:** `agent.worker`

**Definition:** A sandboxed runtime service that consumes SYSTEM-emitted iteration starts, compiles context from referenced artifacts, performs work, produces candidates, requests oracle runs, and submits iteration summaries.

**What makes it foundational:** SOLVER‑Ralph’s core loop structure assumes a replaceable “work executor” that is not the SYSTEM and not a human.

**Key properties:**
- Replaceable and non-authoritative.
- Operates under a declared Agent Definition and tool-access profile.
- Produces tangible outputs (candidates + summaries) that can be verified by oracles.

---

### 4.2 Context Scout Agent (advisory-plane)

**Stable subtype id:** `agent.context_scout`

**Definition:** An agent specialized for identifying context gaps and proposing improvements to the **next iteration’s** ref set, without introducing ghost inputs.

**Foundational benefit:** Context errors are a dominant failure mode in long-running agent work. A dedicated context scout helps the system remain replayable by forcing missing information into typed, referenceable artifacts rather than out-of-band memory.

**Typical outputs (non-binding):**
- proposed additions/removals to `IterationStarted.refs[]` (as suggestions),
- proposed `meta.selector` slices for referenced documents,
- proposed “missing artifact” creation (e.g., create a record note or add an evidence artifact).

---

### 4.3 Evidence Triage Agent (advisory-plane)

**Stable subtype id:** `agent.evidence_triage`

**Definition:** An agent specialized for reading Evidence Bundles and summarizing:

- which oracles passed/failed,
- which integrity conditions fired,
- what failure patterns suggest (flake vs real regression),
- and what the next iteration should attempt.

**Foundational benefit:** Agents are strong at synthesis across many logs, but SOLVER‑Ralph forbids treating synthesis as proof. This subtype explicitly produces **non-binding** analysis that helps humans and the system choose next actions without confusing it with verification.

**Important constraint:** This agent does not create “Verified” claims. It proposes interpretations and next steps only.

---

### 4.4 Governance Scribe Agent (advisory-plane)

**Stable subtype id:** `agent.governance_scribe`

**Definition:** An agent specialized for drafting governance artifacts and governance changes (new docs, amendments, change proposals), with explicit diffs and cross-reference hygiene.

**Foundational benefit:** SOLVER‑Ralph treats documentation as infrastructure. Keeping governance coherent is labor-intensive; a scribe subtype helps maintain structured, versioned artifacts without granting authority to the scribe itself.

**Output class:** Draft artifacts (e.g., new `SR-*` drafts), never direct selection/promotion.

---

### 4.5 Decision Router Agent (advisory-plane)

**Stable subtype id:** `agent.decision_router`

**Definition:** An agent specialized for recognizing when the loop has reached a decision point (e.g., stop trigger fired, governance touch, repeated failure), classifying it, and proposing how it should be routed (decision record vs portal).

**Foundational benefit:** SOLVER‑Ralph relies on conservative escalation. A router subtype makes escalation legible, consistent, and less likely to be “handled informally” in chat.

**Important constraint:** It cannot decide; it only routes and drafts decision record candidates.

---

## 5. Agent cohorts (deployment-time classes)

These are macro-classes (cohorts) describing how agents appear over the lifecycle of SOLVER/SOLVER‑Ralph work.

### 5.1 A1: Bootstrap Builder Agents

Agents used to build SOLVER‑Ralph itself (the governance runtime and loop machinery).  
These are typically `agent.worker` + `agent.governance_scribe` in early phases.

### 5.2 A2: Instance Domain Agents

Agents that natively use SOLVER‑Ralph in a specific domain; outputs are domain artifacts (not necessarily changes to SOLVER‑Ralph itself).

### 5.3 A3: Embedded Domain Worker Agents

Agents embedded into a specific SOLVER‑Ralph instance to repeatedly solve a class of domain problems, with required human inputs/approvals.

---

## 6. Interaction contracts (what agents can and cannot do)

This section defines agent boundaries in object terms (so downstream docs and implementations can reference them).

### 6.1 Allowed agent commitments

Agents MAY produce:

- draft artifacts (including governance drafts),
- candidates (content-addressed snapshots),
- iteration summaries,
- non-binding reviews / critiques,
- proposals for change, decisions, or exceptions (as drafts).

### 6.2 Disallowed agent commitments (trust boundary)

Agents MUST NOT:

- produce portal approvals,
- finalize freeze records,
- activate exceptions / waivers as binding,
- change “current” selection pointers for normative artifacts,
- emit SYSTEM-only orchestration events (e.g., starting an iteration in the authoritative event stream).

These are definition-level constraints: if a system permits an agent to do these things, it has violated the SOLVER‑Ralph model of what an Agent is.

---

## 7. Provenance rules for agents

### 7.1 Attribution requirements (identity)

All agent-generated events, summaries, and evidence attributions SHOULD carry:

- `actor_kind=AGENT`
- `actor_id` as defined above
- a stable reference to the Agent Definition used (when applicable)

### 7.2 Agent configuration as audit provenance

The configuration used to parameterize an Agent Worker (Agent Definition, gating policy, template catalogs) SHOULD be recorded as **audit provenance** rather than semantic dependency unless an implementation explicitly needs staleness propagation.

Rationale: changing prompts or tool toggles should not automatically invalidate previously-verified candidates, but the system must retain the ability to answer “what configuration produced this work.”

---

## 8. Appendix: Minimal `config.agent_definition` skeleton (non-binding)

```yaml
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "AGENTDEF-ralph.co_developer"
  type: "config.agent_definition"
  title: "Agent Definition — ralph.co_developer"
  version: "1.0.0"
  status: "draft"
  normative_status: "directional"
  authority_kind: "config"
  governed_by: ["SR-CHANGE"]
  created: "2026-01-10"
  updated: "2026-01-10"
  tags: ["agent-definition", "co-developer"]
  ext:
    agent_definition:
      agent_role_id: "ralph.co_developer"
      capability_profile_id: "cap.standard_dev"
      context_profile_id: "ctx.sr_default"
      tool_access_profile_id: "tools.standard_sandbox"
      template_catalog_ref:
        kind: "GovernedArtifact"
        id: "TEMPLATES-DEFAULT"
        version: "1.0.0"
        content_hash: "sha256:<hex>"
---
```

---

## 9. Appendix: Design rationale (why this taxonomy is conservative)

SOLVER‑Ralph’s central failure mode is “foundation corruption” — agent-produced narrative being mistaken for evidence or authority. This document’s main job is to make that category error structurally difficult by:

- treating “Agent” as a distinct object with explicit boundaries,
- separating work-plane execution from advisory-plane synthesis,
- and encouraging explicit, versioned Agent Definitions that remain auditable but non-authoritative.

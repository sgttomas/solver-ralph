---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-AGENTS"
  type: "policy.agent_taxonomy"
  title: "SOLVER-Ralph Agents"
  version: "1.1.0-draft.2"
  status: "draft"
  normative_status: "normative"
  authority_kind: "content"
  governed_by: ["SR-CHANGE"]
  supersedes:
    - "SR-AGENTS@1.1.0-draft.1"
  created: "2026-01-11"
  updated: "2026-01-11"
  tags:
    - "solver-ralph"
    - "governance"
    - "agents"
    - "ontology"
    - "epistemology"
    - "membranes"
    - "hex-architecture"
  ext:
    agent_taxonomy:
      summary: >
        Defines what an Agent is in SOLVER‑Ralph (single actor kind, single modality),
        clarifies the epistemic boundary that agent outputs are proposals, and introduces
        the normative Agent Envelope concept as the correct place to vary context, tools,
        and task constraints under SR‑ETT membranes and hexagonal architecture.
      scope:
        - "Applies to all SOLVER‑Ralph implementations and SOLVER‑Ralph-governed workflows."
        - "Defines the Agent concept, constraints, and envelope semantics."
      aligns_with:
        - "SR-CONTRACT (trust boundaries; proposal vs commitment; human-only binding)"
        - "SR-SPEC (system vs agent separation; evidence/oracles; event log)"
        - "SR-ETT (membrane topology; coercion surfaces)"
        - "SR-DIRECTIVE (workflow selection of envelopes; budgets/stop triggers)"
        - "SR-TYPES (config profiles referenced by envelopes)"
---

# SOLVER-Ralph Agents (SR-AGENTS) — v1.1.0-draft.2

**Purpose:** Define the **Agent** concept in SOLVER‑Ralph as a **single actor kind with a single modality** (non‑authoritative work execution), and specify the **Agent Envelope** as the **normative mechanism** for varying context, tools, and task constraints under SR‑ETT membranes and hexagonal architecture.

**Normative status:** **Normative (binding)** for terminology and classification within this artifact’s scope.

---

## 0. Interpretation and precedence

### 0.1 What this document is (and is not)

This document **is**:
- an ontology for **Agent** as a first-class actor kind with a single modality,
- an epistemic boundary: **agent outputs are proposals** until admitted through membranes,
- a semantic alignment statement tying agents to **hex architecture** and **membrane coercion**,
- a definition of the **Agent Envelope** (normative), which is the correct axis of variability.

This document **is not**:
- a list of subagent types,
- a prompt catalog,
- an operational guide for running iterations (SR‑DIRECTIVE),
- a mechanism spec for evidence/oracles/events (SR‑SPEC),
- a change-control policy (SR‑CHANGE).

### 0.2 Precedence

If anything here conflicts with binding artifacts, precedence is:

1) SR‑CONTRACT  
2) SR‑SPEC  
3) SR‑DIRECTIVE  
4) SR‑ETT / SR‑INTENT  
5) SR‑README

Conflicts MUST be resolved via SR‑CHANGE.

---

## 1. Definitions

### 1.1 Actor kinds (context)

SOLVER‑Ralph distinguishes actor kinds:

- **HUMAN** — the only actor kind that may perform binding trust-boundary actions via portals.
- **SYSTEM** — authoritative orchestrator/enforcer; emits SYSTEM control-plane events and enforces invariants.
- **AGENT** — non-authoritative work executor; produces proposals and candidate changes through coerced interfaces.

This document defines **AGENT**.

### 1.2 Agent (single-modality base definition) — *binding*

An **Agent** is a runtime actor with `actor_kind=AGENT` whose defining property is **non-authority**.

An Agent MAY:
- propose actions, interpretations, and next steps,
- draft or modify artifacts (code and/or governed drafts) as **candidate changes**,
- request oracle runs through SYSTEM interfaces,
- package structured proposal packets (evidence summaries, exception requests, release checklists).

An Agent MUST NOT:
- create binding approvals, freezes, decisions, or exceptions,
- emit SYSTEM-only authoritative orchestration events in the canonical event stream,
- redefine binding semantics or bypass SR‑ETT membranes.

> The Agent is not defined by stochasticity or intelligence; it is defined by **where authority does not live**.

### 1.3 Proposal vs commitment objects (membrane alignment) — *binding*

**Agent outputs are proposals** unless and until they are admitted as **commitment objects** through the appropriate membrane surface.

Commitment objects include (non-exhaustive):
- evidence bundles and oracle outputs (mechanism-produced),
- gate evaluations (system-enforced),
- portal approvals/decisions (human-only),
- freeze records and “current” selections (human + system-controlled).

Agents may draft or assemble inputs to these, but agents do not create them as binding facts.

---

## 2. Coherence of the Agent concept under hex architecture + membranes

### 2.1 Hexagonal placement (ports/adapters) — *binding*

In SOLVER‑Ralph, an Agent is an **adapter-side component** that interacts with the domain exclusively through **driving ports** defined and enforced by the SYSTEM.

Therefore:
- The **domain model** MUST NOT depend on “which agent” performed work.
- The domain only depends on the **typed artifacts** and **events** admitted through ports.

### 2.2 Coercion belongs to membranes and ports — *binding*

All coercion (safety, integrity, admissibility) belongs to:
- SR‑ETT membranes, and
- SYSTEM ports/gates/portals/oracles.

Agents remain replaceable; constraints are enforced at interfaces.

### 2.3 No agent-internal authority — *binding*

An Agent MUST NOT be treated as authoritative by virtue of:
- name, deployment, role label, “subtype”, or reputation.
Authority is created only by:
- evidence + oracles,
- gate enforcement,
- and human portals (binding records).

### 2.4 System/Agent semantic separation — *binding*

A deployed component MUST NOT present itself as both `SYSTEM` and `AGENT` for the same authoritative action stream.

Clarification:
- Deterministic **context compilation** is a SYSTEM semantic (e.g., ContextCompiler).  
  Agents may suggest context refs, but the SYSTEM compiles and admits context.

---

## 3. Agent identity and provenance

### 3.1 Agent instance identity (recommended)

Agent instances SHOULD be recorded as:
- `actor_kind = AGENT`
- `actor_id = agent:<deployment_id>:<instance_id>`

### 3.2 Provenance attachment (recommended)

Agent-generated outputs SHOULD carry:
- agent instance identity,
- envelope identity (see §4),
- and references to any relevant configuration artifacts.

---

## 4. Agent Envelope (normative)

### 4.1 Definition — *binding*

An **Agent Envelope** is the normative configuration boundary that defines:
- what context the agent may use,
- what tools and ports the agent may invoke,
- what output schemas are acceptable,
- what verification profile applies to resulting candidates,
- and what budgets/stop triggers constrain the work unit.

**Key rule:** Variability belongs to the **Envelope**, not to “agent subtypes.”  
All agents share the same modality; only envelopes vary.

### 4.2 Envelope selection — *binding*

An envelope is selected by the SYSTEM as part of executing SR‑DIRECTIVE (and may be referenced from SR‑PLAN work units). Agents do not self-select envelopes.

Envelope selection MUST be recorded as part of the work unit context (e.g., IterationStarted refs or equivalent), so that:
- replay is possible,
- provenance is auditable,
- and “no ghost policy” holds.

### 4.3 Envelope contents (minimum fields) — *binding*

An envelope MUST specify at minimum:

1) **context_policy**
   - allowed ref kinds / sources,
   - whether additional refs may be suggested vs admitted,
   - restrictions on unreferenced external context.

2) **tool_access_policy**
   - allowed tools and constrained capabilities,
   - prohibited side effects,
   - network / filesystem / sandbox constraints (if applicable).

3) **port_access_policy**
   - which SYSTEM driving ports the agent may call (e.g., submit candidate, request oracles),
   - rate/volume constraints.

4) **output_policy**
   - allowed output kinds (candidate types, draft artifact types),
   - required structured fields (evidence pointers, change summaries),
   - required redaction / secret-handling constraints.

5) **verification_policy**
   - required verification profile(s) / oracle suite(s) for candidates created under this envelope,
   - whether WITH_EXCEPTIONS is ever eligible (never for integrity conditions).

6) **budget_policy**
   - iteration caps, oracle-run caps, wall-time caps,
   - stop triggers and routing requirements.

### 4.4 Envelope invariants — *binding*

- An envelope MUST NOT grant binding authority.
- An envelope MUST NOT disable integrity conditions.
- An envelope MUST NOT change the meaning of gates/profiles/portals; that requires SR‑CHANGE and GovernanceChangePortal decisions.

---

## 5. Interaction contract (agents in workflows)

### 5.1 Allowed agent outputs (proposal class)

Agents MAY produce:
- candidate artifacts and patches (as candidates),
- draft documents (including governed drafts),
- structured summaries and analyses (non-binding),
- draft portal request packets (exception, governance change, release), with evidence refs.

### 5.2 Disallowed commitments (trust boundary)

Agents MUST NOT:
- approve, decide, freeze, or mark current,
- emit binding exceptions or waivers,
- register or modify governed oracle suites as binding,
- emit SYSTEM-only control-plane events in the canonical event stream.

These prohibitions define the Agent concept: if a system permits them, it is not SOLVER‑Ralph-conformant.

---

## 6. Minimal envelope skeleton (non-binding example)

```yaml
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "ENVELOPE-default.worker"
  type: "config.agent_envelope"
  title: "Agent Envelope — default.worker"
  version: "1.0.0"
  status: "draft"
  normative_status: "directional"
  authority_kind: "config"
  governed_by: ["SR-CHANGE"]
  tags: ["agent-envelope", "worker"]
  ext:
    agent_envelope:
      context_policy:
        admitted_refs_only: true
        allow_suggested_refs: true
      tool_access_policy:
        allowed_tools: ["repo_read", "repo_write", "runner"]
        network: "restricted"
      port_access_policy:
        allowed_ports: ["SubmitCandidate", "RequestOracleRun", "SubmitSummary"]
      output_policy:
        allowed_output_kinds: ["candidate.code_patch", "artifact.draft_md"]
        require_change_summary: true
      verification_policy:
        required_profile_id: "STRICT-CORE"
      budget_policy:
        max_iterations: 5
        max_oracle_runs: 25
        stop_triggers: ["BUDGET_EXHAUSTED", "REPEATED_FAILURE>=3"]
---
```

---

## 7. Design rationale (why this is conservative)

SOLVER‑Ralph’s dominant failure mode is **semantic drift**: narrative being mistaken for evidence or authority.

This document prevents drift by:
- defining Agent by **non-authority**,
- placing all coercion at membranes + ports (hex architecture),
- and making the **Agent Envelope** the only normative axis of variability.

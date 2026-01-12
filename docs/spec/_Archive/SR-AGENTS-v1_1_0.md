---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-AGENTS"
  type: "policy.agent_taxonomy"
  title: "SOLVER-Ralph Agents"
  version: "1.1.0"
  status: "draft"
  normative_status: "normative"
  authority_kind: "content"
  governed_by: ["SR-CHANGE"]
  supersedes:
    - "SR-AGENTS@1.0.0-draft.1"
  created: "2026-01-10"
  updated: "2026-01-11"
  tags:
    - "solver-ralph"
    - "governance"
    - "agents"
    - "trust-boundary"
    - "membranes"
  refs:
    - kind: "GovernedArtifact"
      id: "SR-ETT"
      rel: "depends_on"
      meta: { purpose: "membrane definitions that constrain agents" }
    - kind: "GovernedArtifact"
      id: "SR-CONTRACT"
      rel: "depends_on"
      meta: { purpose: "actor kinds and trust boundaries" }
    - kind: "GovernedArtifact"
      id: "SR-SPEC"
      rel: "depends_on"
      meta: { purpose: "agent integration with loop mechanics" }
  ext:
    agent_taxonomy:
      summary: >
        Defines what an Agent is in the SOLVER‑Ralph paradigm: a stochastic generator
        that operates within structural constraints (membranes) that make certain
        claims impossible rather than merely prohibited. Agents have uniform ontology;
        what varies is context and task, not the nature of the agent.
      scope:
        - "Applies to all SOLVER‑Ralph implementations and SOLVER‑Ralph-governed workflows."
        - "Defines agent object model and its relationship to SR-ETT membranes."
---

# SOLVER-Ralph Agents (SR-AGENTS) — v1.1.0

**Purpose:** Define the **Agent** object category in the SOLVER‑Ralph paradigm and explain how agents are structurally constrained by SR-ETT membranes to produce proposals rather than institutional facts.

**Normative status:** **Normative (binding) for terminology and classification** within this artifact's scope.

**Primary question:** "What kind of thing is an *Agent* here, and how does the system constrain it?"

---

## 0. Version changes

### 1.1.0 (2026-01-11)

- **Membrane integration:** Reframes agent constraints around SR-ETT membranes. Agents are constrained by structure, not by rules they might violate.
- **Alethic framing:** Clarifies that agents cannot produce false institutional claims because the system won't accept them—not because agents follow rules.
- **Uniform ontology:** Removes agent subtype taxonomy. All agents have the same fundamental nature; what varies is context and task, not ontological category.
- **Removes cohorts:** Deletes A1/A2/A3 cohort model (orphaned from current architecture).
- **Fixes precedence:** SR-ETT is normative and placed appropriately in precedence list.
- **Adds explicit refs:** Document now declares dependencies on SR-ETT, SR-CONTRACT, SR-SPEC.

### 1.0.0-draft.1 (2026-01-10)

- Initial draft defining Agent object category and foundational taxonomy.

---

## 1. How to interpret this document

### 1.1 What this document is (and is not)

This document **is**:

- an ontology for "Agent" as a first-class object category,
- an explanation of how SR-ETT membranes constrain agent action,
- a definition of the agent object model and configuration,
- a set of scope boundaries that prevent category errors ("agent" vs "system" vs "oracle" vs "human").

This document **is not**:

- a prompt catalog,
- an operational guide for running agents,
- a definition of portal semantics or oracle semantics,
- a taxonomy of agent subtypes (agents have uniform ontology).

### 1.2 Compatibility and precedence

If anything in this document conflicts with binding artifacts, the binding precedence remains:

1) SR-CONTRACT (invariants and safety contracts)
2) SR-SPEC (schemas and state machines)
3) SR-ETT (membrane topology and harnesses)
4) SR-DIRECTIVE (gates, profiles, budgets)
5) SR-INTENT (rationale; directional)
6) SR-README (navigation; index)

Conflicts MUST be resolved via SR-CHANGE; do not resolve by interpretation.

---

## 2. The Core Insight: Structural Constraint, Not Prohibition

### 2.1 Why agents don't need to be trustworthy

SOLVER‑Ralph does not attempt to make agents trustworthy. Instead, it creates an external structure where agent claims cannot become institutional facts without passing through verification machinery and human judgment.

This is the difference between:

| Approach | Mechanism | Failure mode |
|----------|-----------|--------------|
| **Prohibition** | Rules agents must follow | Agent violates rule; system fails |
| **Structural constraint** | System rejects agent claims at boundary | Agent cannot violate; claim never accepted |

SOLVER‑Ralph uses structural constraint. An agent cannot produce a "Verified" artifact by claiming it—the property "Verified" attaches only when oracles pass and evidence is recorded. The agent's belief is irrelevant.

### 2.2 Membranes enforce constraints

SR-ETT defines nine membranes that constrain all actors, including agents. For agents specifically:

| Membrane | What it constrains |
|----------|-------------------|
| **Authority & Integrity** | Agent submissions to portals are rejected (HUMAN-only) |
| **Accountability** | Agent outputs require evidence bundles to become Verified |
| **Operational** | Agent cannot emit SYSTEM-only events (IterationStarted) |
| **Ontological** | Agent outputs must conform to SR-TYPES schemas |
| **Isomorphic** | Agent code claims are tested against oracle execution |
| **Change** | Agent cannot modify governed artifacts without GovernanceChangePortal |
| **Resource** | Agent operates within budget limits enforced by SYSTEM |
| **Intent & Objective** | Agent context is constrained to IterationStarted.refs[] |

These are not rules agents follow—they are boundaries the system enforces regardless of agent behavior.

### 2.3 Agents produce proposals; the system constitutes facts

An agent statement such as "tests passed," "compliant," "approved," or "ready to ship" is always a **proposal**. The corresponding **institutional fact** can only be constituted by:

- **"Verified"**: Oracle evidence bundle recorded with all required oracles PASS
- **"Approved"**: Human approval recorded via portal
- **"Shippable"**: Freeze record created with release approval

The agent can believe whatever it wants. The system doesn't ask agents what's true—it checks what passed the gates.

---

## 3. Definitions

### 3.1 Actor kinds

SOLVER‑Ralph uses three actor kinds:

| Actor Kind | Role | Trust Boundary |
|------------|------|----------------|
| **HUMAN** | Can cross trust boundaries via portal approvals; carries responsibility | Source of binding authority |
| **SYSTEM** | Orchestrates, enforces, records; emits SYSTEM-only events | Enforces invariants |
| **AGENT** | Provides labor; proposes actions; generates drafts and implementations | Constrained to proposals |

This document defines "Agent" and its relationship to membranes; it does not redefine Human or System.

### 3.2 Agent (definition)

An **Agent** is a **stochastic generator** instantiated as a runtime actor (`actor_kind=AGENT`) that:

- proposes actions and interpretations,
- drafts or modifies artifacts (code and/or governance-adjacent documents),
- registers candidates and requests evaluation runs *through system interfaces*,
- produces structured iteration outputs (summaries, next steps, open risks),
- operates within membrane-enforced boundaries that prevent elevation to authority.

**Key property:** Agents are generators, not judges. They produce candidates for evaluation, not verdicts.

### 3.3 Uniform agent ontology

All agents have the same fundamental nature. What varies between agents:

| Varies | Does not vary |
|--------|---------------|
| Context (IterationStarted.refs[]) | Trust status (non-authoritative) |
| Task (what iteration produces) | Membrane constraints (all nine apply) |
| Configuration (Agent Definition) | Relationship to verification (proposals require evidence) |
| | Relationship to authority (cannot cross portals) |

There are no "types" of agents in the ontological sense. An agent configured to draft governance documents and an agent configured to write code are the same *kind of thing* doing different *tasks* with different *context*.

Role differentiation is a matter of configuration and task assignment, not ontology.

### 3.4 Agent vs System vs Oracle vs Human

To maintain structural integrity, SOLVER‑Ralph treats these as distinct categories:

| Category | What it does | What it cannot do |
|----------|--------------|-------------------|
| **Agent** | Proposes and executes work | Grant authority; emit SYSTEM events |
| **System** | Emits authoritative events; enforces invariants | Substitute for human judgment |
| **Oracle** | Deterministic evaluator producing evidence | Act as an agent or make claims |
| **Human** | Approves trust-boundary actions | Be automated away |

**Prohibition:** A deployed component MUST NOT simultaneously present itself as both `SYSTEM` and `AGENT` for the same action stream. This prevents authority laundering.

### 3.5 Oracle relationship

Agents and oracles have a specific asymmetric relationship:

- **Agent** produces candidate outputs
- **Oracle** evaluates candidate outputs deterministically
- **Evidence bundle** records oracle results
- **Verification status** derives from evidence, not agent claims

This asymmetry is load-bearing: the agent cannot verify its own work. Verification requires an independent mechanism (oracle) that the agent does not control.

---

## 4. Agent object model

### 4.1 Agent Instance

An **Agent Instance** is a specific runtime instantiation of an agent, recorded as:

- `actor_kind = AGENT`
- `actor_id = agent:<deployment_id>:<agent_instance_id>`

Agent Instances are ephemeral and replaceable. The system must be able to answer: *"Which agent instance produced this output?"* without relying on agent memory.

### 4.2 Agent Deployment

An **Agent Deployment** is the stable container/environment and identity namespace within which Agent Instances are created (e.g., a worker fleet, a specific orchestrated service, a pinned container image digest).

**Requirement:** A deployment SHOULD be stable enough that different Agent Instances can be compared and audited.

### 4.3 Agent Role

An **Agent Role** is a stable semantic label describing the intended function of an agent instance for a particular task (e.g., "co-developer", "implementer", "scribe").

Roles describe task intent, not ontological category. All roles share the same agent ontology.

### 4.4 Agent Definition (configuration artifact)

An **Agent Definition** is a versioned, auditable configuration artifact (`config.agent_definition`) that parameterizes an agent's behavior **without creating new authority**.

At minimum, an Agent Definition SHOULD declare:

- `agent_role_id` (task intent label)
- `capability_profile_id`
- `context_profile_id` or `context_profile_rules`
- `tool_access_profile_id`
- references to any prompt/template catalogs (if used)

**Audit semantics:** Agent Definitions are referenced as **audit provenance** (`rel=supported_by`) so configuration changes don't invalidate previously-verified work while maintaining full traceability.

---

## 5. Agent in the loop

### 5.1 Loop integration

An agent operates within the SOLVER-Ralph loop:

1. **SYSTEM** emits `IterationStarted` with refs[]
2. **Agent** receives iteration, compiles context from refs[] only
3. **Agent** performs work, produces candidate
4. **Agent** registers candidate via API
5. **Agent** requests oracle runs
6. **Oracle** runs execute, produce evidence
7. **Agent** submits iteration summary
8. **SYSTEM** records `IterationCompleted`

The agent's scope is steps 2-7. It cannot emit the SYSTEM events (steps 1, 8).

### 5.2 Context constraint (no ghost inputs)

Per SR-CONTRACT C-LOOP-2, agent context MUST derive only from IterationStarted.refs[]. This is enforced by the Intent & Objective membrane.

An agent cannot:
- Use information not in refs[]
- Access external resources not declared
- Rely on memory from prior conversations (unless summarized in refs[])

This constraint ensures replayability: given the same refs[], the same work should be reproducible (within stochastic variance).

### 5.3 Candidate production

Agents produce candidates—content-addressed snapshots of proposed work. A candidate:

- Has a stable identity (content hash)
- Is registered via API (not by agent declaration)
- Requires oracle verification to become Verified
- Requires human approval to become Shippable

The agent cannot declare a candidate Verified or Shippable; those states are constituted by system machinery.

---

## 6. Structural constraints

### 6.1 Framing

These are not prohibitions that agents might violate. These are actions the system makes structurally impossible.

### 6.2 Impossible agent actions

| Action | Why impossible |
|--------|----------------|
| Produce portal approvals | Portals reject non-HUMAN actor_kind at API level |
| Finalize freeze records | Freeze creation requires prior ReleaseApprovalRecorded (HUMAN-only) |
| Activate exceptions/waivers | ExceptionApprovalPortal is HUMAN-only |
| Change "current" selection | GovernanceChangePortal is HUMAN-only |
| Emit SYSTEM-only events | Event schema enforces actor_kind = SYSTEM for IterationStarted |
| Use undeclared context | ContextBundle compilation only admits refs[] contents |
| Bypass oracle verification | Verified status requires EvidenceBundleRecorded with oracle results |

### 6.3 Permitted agent actions

| Action | Outcome |
|--------|---------|
| Draft artifacts | Creates candidate; requires verification |
| Draft governance changes | Creates proposal; requires GovernanceChangePortal |
| Draft exception requests | Creates proposal; requires ExceptionApprovalPortal |
| Claim "tests passed" | Claim is ignored; only oracle evidence matters |
| Request oracle runs | Permitted; results recorded as evidence |
| Produce iteration summaries | Recorded as audit provenance |

---

## 7. Provenance rules

### 7.1 Attribution requirements

All agent-generated events, summaries, and evidence attributions MUST carry:

- `actor_kind = AGENT`
- `actor_id` (deployment + instance)
- Reference to Agent Definition used (when applicable)

### 7.2 Agent configuration as audit provenance

Agent Definition, gating policy, and template catalogs SHOULD be recorded as `rel=supported_by` (audit provenance) rather than `rel=depends_on` (semantic dependency).

**Rationale:** Changing prompts or tool toggles should not automatically invalidate previously-verified candidates, but the system must retain the ability to answer "what configuration produced this work."

---

## 8. Interaction with SR-ETT harnesses

SR-ETT defines nine harnesses that enforce membrane constraints. For agents:

| Harness | Agent-Relevant Enforcement |
|---------|---------------------------|
| Intent Harness | Validates SAPS presence in iteration context |
| Operational Harness | Enforces loop state machine; blocks agent from SYSTEM actions |
| Architectural Harness | Runs hex boundary conformance oracles on agent code |
| Ontological Harness | Validates agent outputs against SR-TYPES schemas |
| Isomorphic Harness | Tests agent code against spec via oracles |
| Change Harness | Routes governance-touching agent work to GovernanceChangePortal |
| Authority Harness | Rejects agent submissions at portal boundaries |
| Resource Harness | Enforces budget limits on agent iterations |
| Accountability Harness | Requires evidence bundles for verification claims |

The harnesses are the *mechanism*; the membranes are the *constraint*. Agent behavior is contained by harness enforcement of membrane rules.

---

## Appendix A: Minimal `config.agent_definition` skeleton

```yaml
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "AGENTDEF-instance1-default"
  type: "config.agent_definition"
  title: "Agent Definition — Instance-1 Default"
  version: "1.0.0"
  status: "draft"
  normative_status: "directional"
  authority_kind: "config"
  governed_by: ["SR-CHANGE"]
  created: "2026-01-11"
  updated: "2026-01-11"
  tags: ["agent-definition", "instance-1"]
  ext:
    agent_definition:
      agent_role_id: "default.instance1"
      capability_profile_id: "cap.standard"
      context_profile_id: "ctx.sr_default"
      tool_access_profile_id: "tools.standard_sandbox"
      template_catalog_ref: null  # Optional
---
```

---

## Appendix B: Design rationale

### B.1 Why structural constraint instead of prohibition

Prohibition-based systems fail when:
- Rules are ambiguous
- Enforcement is inconsistent
- Agents find loopholes
- Trust is misplaced

Structural constraint avoids these failure modes by making violation impossible at the type/API level. An agent cannot produce a portal approval because the portal API rejects non-HUMAN submissions—not because the agent follows a rule.

### B.2 Why uniform ontology (no subtypes)

The original SR-AGENTS draft defined agent subtypes (Context Scout, Evidence Triage, Governance Scribe, Decision Router). This was removed because:

1. **All agents have the same nature**: stochastic generators constrained by membranes
2. **What varies is configuration and task**: not ontological category
3. **Subtypes create false distinctions**: implying categorical differences where none exist
4. **Agent Definition handles parameterization**: subtypes add taxonomy without value

An agent configured to draft governance documents is the same kind of thing as an agent configured to write code. The task differs; the ontology doesn't.

### B.3 Why agents are generators, not judges

The core problem SOLVER‑Ralph solves: "How to generate trustworthy knowledge artifacts using non-deterministic agents."

If agents were judges (could determine truth), the problem would be: "How to make agents trustworthy." That's unsolved.

By making agents generators (produce candidates for evaluation), the problem becomes: "How to verify agent output." That's solvable—oracles, evidence, human review.

The generator/judge distinction is the architectural decision that makes the paradigm work.

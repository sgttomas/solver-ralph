---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-AGENTS"
  type: "governance.type_definition"
  title: "SOLVER Agent Type Definitions"
  version: "1.0.0-draft"
  status: "draft"
  normative_status: "normative"
  authority_kind: "definition"
  governed_by: ["SR-CHANGE"]
  depends_on:
    - "SR-PARADIGM"
    - "SR-TYPES"
    - "SR-ETT"
  created: "2026-01-10"
  updated: "2026-01-10"
  supersedes: null
  tags:
    - agents
    - types
    - ontology
    - trust
    - definitions
  ext:
    agent_ontology:
      core_assertion: >
        An agent is a bounded computational actor that transforms governed inputs
        into candidate outputs under explicit constraints, producing evidence of
        its work but never holding authority over commitments.
      type_hierarchy:
        - "Agent Definition (governed artifact)"
        - "Agent Instance (runtime activation)"
        - "Agent Role (reusable capability pattern)"
      invariant: >
        No agent, regardless of wave, role, or context, can claim Verified,
        Approved, or Shippable status. These are trust-boundary states that
        require external oracle evidence and/or human judgment.
---

# SOLVER Agent Type Definitions

## 1. Purpose

This document defines **what an agent is** as a typed object in the SOLVER paradigm.

It establishes the ontological foundation for agents: their essential properties, structural relationships, and invariants. Other documents (SR-AGENT-BUILDER, domain-specific agent specs) define how to create agent definitions and how agents operate in specific contexts.

This document answers: *What kind of thing is an "agent" in SOLVER?*

### 1.1 Scope

This document governs:

- The definition of "agent" as a typed concept
- The distinction between agent definitions, instances, and roles
- The structural properties that all agents share
- The type-level invariants that constrain all agent behavior
- The relationship between agents and the wave model

This document does not govern:

- Specific agent role definitions (see domain-specific specs)
- Agent configuration file formats (see SR-AGENT-BUILDER)
- Runtime orchestration protocols (see SR-SPEC)
- Trust membrane topology (see SR-ETT)

### 1.2 Relationship to SR-PARADIGM

SR-PARADIGM Section 9 establishes the three-wave agent model and the principle that agents are "stochastic generators" whose outputs require evidence and human approval to become commitments.

This document formalizes that principle into a type definition that can be referenced by SR-TYPES and instantiated by agent configuration artifacts.

---

## 2. Core Definition

### 2.1 What is an agent?

**Agent**: A bounded computational actor that transforms governed inputs into candidate outputs under explicit constraints, producing evidence of its work but never holding authority over commitments.

This definition has five essential components:

| Component | Meaning |
|-----------|---------|
| **Bounded** | Operates within explicit limits (capabilities, trust ceiling, context scope) |
| **Computational actor** | Performs work; not passive data |
| **Transforms governed inputs** | Consumes only inputs with explicit provenance; no ghost context |
| **Produces candidate outputs** | Outputs are proposals until externally verified/approved |
| **Never holds authority** | Cannot self-certify; commitments require external evidence or human judgment |

### 2.2 What an agent is NOT

An agent is not:

- **An oracle**: Agents propose; oracles verify. Oracles produce evidence about candidates; agents produce the candidates.
- **An authority**: Agents cannot approve, publish, verify, or commit. These are human or system-level actions at trust boundaries.
- **A memory**: Agents do not persist semantic context across activations. Meaning lives in governed artifacts, not agent state.
- **A policy**: Agents execute under policy constraints; they do not define or modify policy.

### 2.3 The stochastic generator principle

From SR-PARADIGM: "Agents are stochastic generators; authoritative commitments require recorded evidence and/or recorded human approvals."

This means:

1. Agent outputs are probabilistic, not deterministic
2. Two activations of the same agent with the same inputs may produce different outputs
3. Therefore, agent outputs cannot be trusted on their own
4. Trust emerges from external verification (oracles) and human judgment (approvals)

This principle is not a bug to be fixed; it is a structural feature that motivates the evidence-and-approval architecture.

---

## 3. Agent Type Hierarchy

The concept "agent" decomposes into three related types:

```
┌─────────────────────────────────────────────────────────────────┐
│                      AGENT (abstract)                           │
│  "A bounded computational actor that transforms governed        │
│   inputs into candidate outputs under explicit constraints"     │
└─────────────────────────────────────────────────────────────────┘
                              │
          ┌───────────────────┼───────────────────┐
          ▼                   ▼                   ▼
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│ Agent Definition│  │ Agent Instance  │  │   Agent Role    │
│   (artifact)    │  │   (runtime)     │  │   (pattern)     │
├─────────────────┤  ├─────────────────┤  ├─────────────────┤
│ What you write  │  │ What runs       │  │ What you name   │
│ and version     │  │ and produces    │  │ and reuse       │
│                 │  │ evidence        │  │                 │
│ type: config.   │  │ Has identity,   │  │ A capability    │
│ agent_definition│  │ context, task   │  │ cluster that    │
│                 │  │                 │  │ definitions     │
│ Subject to      │  │ Bound by its    │  │ implement       │
│ SR-CHANGE       │  │ definition      │  │                 │
└─────────────────┘  └─────────────────┘  └─────────────────┘
```

### 3.1 Agent Definition

**Agent Definition**: A governed artifact of type `config.agent_definition` that specifies the purpose, capabilities, constraints, and trust ceiling for a class of agent instances.

Properties of an agent definition:

| Property | Description | Example |
|----------|-------------|---------|
| `id` | Unique identifier | `AG-EVIDENCE-LIBRARIAN` |
| `purpose` | What the agent is for | "Package outputs into evidence manifests" |
| `capabilities` | What actions are permitted | `[Read, Grep, Glob, Bash, Write]` |
| `constraints` | What actions are forbidden | "No Task tool; no self-approval" |
| `trust_ceiling` | Maximum authority level | "Proposal authority only" |
| `input_contract` | What inputs are valid | "Only refs provided by main agent" |
| `output_contract` | What outputs must include | "SubAgentResult structure" |

Agent definitions are:

- **Governed artifacts**: Subject to SR-CHANGE; versioned; content-addressed
- **Declarative**: They specify what, not how
- **Instantiable**: Runtime instances are created from definitions

### 3.2 Agent Instance

**Agent Instance**: A runtime activation of an agent definition in a specific context, with unique identity, bound inputs, and the ability to produce evidence-backed outputs.

Properties of an agent instance:

| Property | Description | Example |
|----------|-------------|---------|
| `instance_id` | Unique activation identifier | `uuid-v4` |
| `definition_ref` | The agent definition being instantiated | `AG-EVIDENCE-LIBRARIAN@1.0.0` |
| `wave` | The wave context constraining this activation | `2` (instance/domain) |
| `scope` | The bounded context of work | `domain:acme-corp-invoicing` |
| `task` | The specific work being performed | `package-milestone-3-evidence` |
| `activated_at` | Timestamp of instantiation | `2026-01-10T14:30:00Z` |
| `inputs_received` | Governed refs provided | `[IterationStarted.refs]` |

Agent instances are:

- **Ephemeral**: They exist for a task/session, not permanently
- **Attributed**: All outputs include instance identity
- **Constrained**: Bound by their definition AND their wave context
- **Evidence-producing**: Must generate audit trail of actions

### 3.3 Agent Role

**Agent Role**: A named capability pattern that agent definitions can implement, providing a reusable vocabulary for describing agent purposes.

Core roles in the SOLVER paradigm:

| Role | Purpose | Typical capabilities |
|------|---------|---------------------|
| `governance-author` | Draft and iterate governed documents | Read, Write, propose changes |
| `governance-navigator` | Locate applicable clauses in governed artifacts | Read, Grep, Glob |
| `compliance-checker` | Check work against CONTRACT/SPEC invariants | Read, Grep, Glob |
| `evidence-librarian` | Package outputs into evidence manifests | Read, Grep, Glob, Bash, Write |
| `code-reviewer` | Review changes for correctness and safety | Read, Grep, Glob, Bash |
| `change-scout` | Detect when work crosses into change management | Read, Grep, Glob |
| `verification-runner` | Execute oracle tests and collect evidence | Read, Bash, Write |

Agent roles are:

- **Patterns, not definitions**: A role is a template; a definition is a concrete artifact
- **Wave-agnostic**: The same role may be implemented in different waves with different constraints
- **Composable**: A definition may implement multiple roles (though this increases complexity)

---

## 4. Structural Properties

All agents share these structural properties regardless of wave, role, or context.

### 4.1 Boundedness

Every agent has explicit bounds:

| Bound type | What it limits | Defined by |
|------------|----------------|------------|
| **Capability bound** | What actions are permitted | Agent definition |
| **Trust ceiling** | Maximum authority level | Wave + definition |
| **Context scope** | What domain/problem space | Instance context |
| **Input bound** | What inputs are valid | Input contract + provenance rules |
| **Output bound** | What outputs are valid | Output contract |
| **Temporal bound** | How long the agent may run | Task timeout or termination |

An agent that attempts to exceed any bound must fail explicitly, not silently expand.

### 4.2 Identity

Every agent instance has verifiable identity:

```yaml
agent_identity:
  instance_id: "uuid-v4"           # Unique to this activation
  definition_id: "AG-EXAMPLE"      # Which definition
  definition_version: "1.0.0"      # Which version of definition
  wave: 2                          # Wave context
  scope: "domain:example"          # Bounded context
  activated_at: "2026-01-10T..."   # When instantiated
  activated_by: "human:alice"      # Who/what triggered activation
```

Identity is:
- **Required**: No anonymous agents
- **Immutable**: Cannot change during activation
- **Included in outputs**: All evidence is attributed

### 4.3 Provenance discipline

Agents must maintain explicit input provenance:

**Valid inputs**:
- Governed artifacts referenced by stable id + version
- Iteration context from `IterationStarted.refs[]`
- Explicitly provided task parameters
- Own agent definition

**Invalid inputs (ghost context)**:
- Conversation history not captured in governed artifacts
- Assumptions not explicitly declared
- External knowledge not referenced as typed artifact
- Prior session state not persisted as artifact

When an agent needs information not in valid inputs, it must:
1. Declare the need explicitly
2. Request the missing artifact/ref
3. Return `status: blocked` until provided

### 4.4 Output structure

All agent outputs must be:

| Property | Requirement |
|----------|-------------|
| **Attributed** | Include agent identity |
| **Timestamped** | Include ISO 8601 timestamp |
| **Structured** | Machine-parseable (not just prose) |
| **Status-bearing** | Include `status: ok | needs_human | blocked` |
| **Traceable** | Link to inputs consumed and artifacts created |

Recommended output structure (SubAgentResult):

```yaml
status: ok | needs_human | blocked
summary:
  - "What was done or found"
inputs_used:
  - "artifact-id@version"
artifacts_created:
  - path: "relative/path"
    intended_sha256: "hash or 'unknown'"
recommended_next_steps:
  - "What should happen next"
open_risks:
  - "What might go wrong"
```

---

## 5. Type-Level Invariants

These invariants hold for ALL agents, regardless of wave, role, definition, or context. They are not constraints to be relaxed; they are definitional properties.

### 5.1 The claims prohibition

**Invariant**: No agent may claim Verified, Approved, or Shippable status for any artifact or deliverable.

These terms have specific meanings at trust boundaries:

| Term | What it means | Who/what can assert it |
|------|---------------|------------------------|
| **Verified** | Oracle evidence exists and passes | Oracle system (not agent) |
| **Approved** | Human has judged and recorded approval | Human at portal |
| **Shippable** | Release criteria met; deployment authorized | Human + system gate |

Agents may:
- State "verification evidence exists at [ref]"
- State "tests pass according to [output]"
- Recommend approval based on evidence
- Package evidence for human review

Agents must not:
- Claim their output is verified
- Claim something is approved
- Assert that deployment should proceed

### 5.2 The authority prohibition

**Invariant**: No agent may hold, acquire, or delegate authority over commitments.

Authority in SOLVER flows from:
- Governed artifacts (define what is binding)
- Human judgment at trust boundaries (approvals)
- Oracle evidence (verification)

Agents are outside this authority flow. They are proposal generators and evidence collectors, not decision-makers.

This means:
- Agents cannot approve anything (including other agents' outputs)
- Agents cannot publish governance artifacts
- Agents cannot modify trust boundaries
- Agents cannot delegate authority they don't have

### 5.3 The recursion prohibition (for subagents)

**Invariant**: An agent activated as a subagent must not spawn additional subagents.

This is enforced by:
- Excluding the `Task` tool from subagent capability sets
- Detecting and failing recursive activation attempts

Rationale:
- Unbounded recursion creates unauditable action chains
- Trust boundaries become unclear if agents spawn agents spawn agents
- Evidence attribution becomes diluted

The main agent (Claude Code session with Task tool) may invoke subagents. Subagents may not invoke further subagents.

### 5.4 The evidence-before-completion invariant

**Invariant**: An agent may not claim task completion without corresponding evidence.

"I have done X" requires:
- Artifacts demonstrating X (files, logs, outputs)
- Attribution linking artifacts to this agent instance
- Inclusion in the SubAgentResult.artifacts_created list

Claims without evidence are invalid and should be treated as failures.

---

## 6. Wave Context

The three waves defined in SR-PARADIGM are not types of agents; they are contexts that constrain agent activation.

### 6.1 Wave as constraint context

| Wave | Context | What it constrains |
|------|---------|-------------------|
| **Wave 1: Bootstrap** | Building SOLVER/SOLVER-Ralph itself | Highest trust ceiling; direct human supervision; least infrastructure |
| **Wave 2: Instance** | Using SOLVER-Ralph for a domain problem | Bounded by SOLVER-Ralph governance; cannot modify platform |
| **Wave 3: Embedded** | Operating within deployed instance | Strictest bounds; policy-only execution; mandatory logging |

### 6.2 Trust ceiling by wave

Trust ceiling is the maximum authority level an agent can achieve in a given wave:

| Wave | Trust ceiling | What this means |
|------|---------------|-----------------|
| **Wave 1** | Proposal + recommendation | May draft governance; may recommend architecture; human approves all |
| **Wave 2** | Proposal within domain | May draft domain artifacts; bounded by platform governance; human approves |
| **Wave 3** | Execution within policy | May only execute policy-permitted operations; no proposal authority |

Trust ceiling constrains what agent definitions are valid in each wave:
- A definition with `trust_ceiling: governance_proposal` is invalid in Wave 3
- A definition with `trust_ceiling: policy_execution` is valid in any wave

### 6.3 Wave transitions

An agent definition does not "move" between waves. Rather:
- Different definitions are appropriate for different waves
- The same role (e.g., evidence-librarian) may have different definitions per wave
- Wave context is set at instance activation, not definition time

---

## 7. Relationship to Other Types

### 7.1 Agent vs Oracle

| Aspect | Agent | Oracle |
|--------|-------|--------|
| Nature | Stochastic generator | Deterministic evaluator |
| Outputs | Candidate proposals | Evidence (pass/fail + artifacts) |
| Authority | None | Can assert "passes" or "fails" |
| Trust | Proposals require verification | Outputs are verification |

Agents produce candidates. Oracles evaluate candidates. These are complementary, not competing.

### 7.2 Agent vs Human

| Aspect | Agent | Human |
|--------|-------|-------|
| Authority | Cannot approve | Can approve |
| Judgment | Generates options | Selects among options |
| Trust boundary | Outside | Defines and operates |
| Persistence | Ephemeral instances | Persistent identity |

Humans approve, publish, and commit. Agents prepare, propose, and package.

### 7.3 Agent vs Governed Artifact

| Aspect | Agent | Governed Artifact |
|--------|-------|-------------------|
| Nature | Actor (does work) | Document (defines meaning) |
| Persistence | Ephemeral | Versioned and retained |
| Authority | None | Defines what is binding |
| Relationship | Consumes and produces | Is consumed and produced |

Agent definitions are governed artifacts. Agent instances are not artifacts—they are activations that reference artifacts.

---

## 8. Normative Requirements

### 8.1 Definition requirements

1. **[AG-N01]** Every agent definition MUST be a governed artifact of type `config.agent_definition`.

2. **[AG-N02]** Agent definitions MUST specify: purpose, capabilities, constraints, trust_ceiling, input_contract, output_contract.

3. **[AG-N03]** Agent definitions MUST be subject to SR-CHANGE for modifications.

### 8.2 Instance requirements

4. **[AG-N04]** Every agent instance MUST have a unique, verifiable identity per Section 4.2.

5. **[AG-N05]** All agent outputs MUST include instance identity attribution.

6. **[AG-N06]** Agent instances MUST NOT persist semantic context across activations except via governed artifacts.

### 8.3 Invariant requirements

7. **[AG-N07]** Agents MUST NOT claim Verified, Approved, or Shippable status (Section 5.1).

8. **[AG-N08]** Agents MUST NOT hold, acquire, or delegate authority over commitments (Section 5.2).

9. **[AG-N09]** Subagents MUST NOT spawn additional subagents (Section 5.3).

10. **[AG-N10]** Agents MUST NOT claim completion without corresponding evidence (Section 5.4).

### 8.4 Provenance requirements

11. **[AG-N11]** Agents MUST accept only inputs with explicit provenance per Section 4.3.

12. **[AG-N12]** Agents MUST declare and flag assumptions when required inputs are missing.

13. **[AG-N13]** Agents MUST return `status: blocked` when unable to proceed without ghost context.

### 8.5 Wave requirements

14. **[AG-N14]** Agent definitions MUST specify a trust_ceiling compatible with intended wave context.

15. **[AG-N15]** Agent instances MUST NOT exceed the trust ceiling of their wave context.

---

## 9. Type Registration

This document establishes the following types for registration in SR-TYPES:

### 9.1 config.agent_definition

```yaml
type_id: "config.agent_definition"
description: "A governed specification of an agent's purpose, capabilities, constraints, and trust boundaries"
required_fields:
  - id
  - purpose
  - capabilities
  - constraints
  - trust_ceiling
  - input_contract
  - output_contract
governed_by: SR-CHANGE
template_location: "SR-AGENT-BUILDER"
```

### 9.2 runtime.agent_instance

```yaml
type_id: "runtime.agent_instance"
description: "A runtime activation of an agent definition (not persisted as artifact; referenced in evidence)"
required_fields:
  - instance_id
  - definition_ref
  - wave
  - scope
  - activated_at
properties:
  - ephemeral: true
  - attributed: true
```

---

## 10. Change Control

This document is governed by **SR-CHANGE**.

Changes to agent type definitions or invariants are high-impact governance changes requiring:

1. Impact analysis across all waves
2. Coherence audit against SR-PARADIGM and SR-ETT
3. Review of all existing agent definitions for continued validity
4. Human approval at appropriate portal

---

*End of document.*

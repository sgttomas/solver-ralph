---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-AGENTS"
  type: "policy.agent_ontology"
  title: "SOLVER-Ralph Agent Ontology"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "normative"
  authority_kind: "content"
  governed_by: ["SR-CHANGE"]
  supersedes: []
  created: "2026-01-10"
  updated: "2026-01-10"
  tags:
    - "solver-ralph"
    - "solver"
    - "agents"
    - "ontology"
    - "roles"
    - "agent-definition"
    - "trust-boundary"
    - "evidence"
  ext:
    agent_ontology:
      summary: >
        Defines what an “Agent” is in the SOLVER-Ralph paradigm, the core object
        model (role, definition, instance), and a minimal set of foundational
        agent subtypes that support trustworthy knowledge production in Ralph Loops.
      objectives:
        - "Make agent identity, authority boundaries, and configuration explicit and auditable."
        - "Standardize a small, composable set of foundational agent roles used across loops."
        - "Keep agent semantics compatible with SR-CONTRACT/SR-SPEC (no new authority)."
      assumptions:
        - "Binding semantics for actors, portals, verification, and event provenance are defined by SR-CONTRACT and SR-SPEC."
        - "Agent configuration is expressed as versioned `config.agent_definition` artifacts (SR-TYPES) and referenced for audit."
        - "Claude Code subagents are an implementation detail (adapter), but follow the same non-authoritative model."
---

# SOLVER-Ralph Agent Ontology {#sr-agents}

## 1. Purpose and scope {#purpose-scope}

### 1.1 Purpose {#purpose}

SOLVER-Ralph’s platform problem statement is to **consistently generate trustworthy knowledge in agentic workflows**.

This document defines what an **Agent** is in that paradigm: a bounded computational actor whose outputs are *proposals* and whose trustworthiness is established by **typed provenance, deterministic oracles, and human authority at trust boundaries**, not by self-assertion.

### 1.2 Scope {#scope}

In scope:

- A **minimal ontology** of agent objects (Agent, Agent Role, Agent Definition, Agent Instance, Subagent).
- A **taxonomy** of foundational agent subtypes that are broadly useful across SOLVER-Ralph development and operation.
- How agent configuration is represented as governed, versioned artifacts (`config.agent_definition`) and how that relates to auditability and reproducibility.

Out of scope:

- The full operational Ralph Loop protocol (see SR-SPEC and SR-DIRECTIVE).
- Portal lifecycle semantics, including meanings of **Verified / Approved / Shippable** (see SR-CONTRACT and SR-SPEC).
- The detailed authoring format for Claude Code subagents and the full 5-pack definitions (kept as separate config artifacts).
- Any requirement to use a specific model provider or agent runtime beyond what SR-SPEC specifies for SOLVER-Ralph itself.

### 1.3 Relationship to the governed set {#relationship}

This document is **subordinate** to binding requirements in:

- **SR-CONTRACT** (architectural invariants and trust boundaries)
- **SR-SPEC** (event model, actor identity model, and iteration context provenance)
- **SR-DIRECTIVE** (how work is abstracted into executable phases/packages with gates, oracle verification, and human approvals)
- **SR-CHANGE** (how governed meaning and selections evolve)
- **SR-TYPES** (artifact taxonomy, including `config.agent_definition`)

If any statement here conflicts with SR-CONTRACT or SR-SPEC, the conflict must be reconciled through SR-CHANGE; SR-CONTRACT and SR-SPEC win by binding precedence.

---

## 2. Normative keywords {#normative-keywords}

The words **MUST**, **MUST NOT**, **SHOULD**, **SHOULD NOT**, and **MAY** are normative and interpreted as in RFC 2119.

Normative keywords are used sparingly in this document and only in explicitly labeled normative sections.

---

## 3. Agent object model (normative) {#agent-object-model}

### 3.1 Agent {#agent}

**Definition (normative):** An **Agent** is an actor with `actor_kind=AGENT` (per SR-SPEC) that performs bounded work in SOLVER-Ralph. An Agent:

- produces **proposals** (drafts, candidates, checklists, summaries, patches),
- may produce or request **evidence** (oracle outputs), and
- MUST NOT be treated as a source of binding authority.

Binding authority remains with **Humans** at portals and the **System** as the orchestrating control plane (as defined elsewhere).

### 3.2 Agent Role {#agent-role}

**Definition (normative):** An **Agent Role** is a stable, named category of agent behavior defined by:

- mission/purpose (what the role is for),
- capability envelope (what the role is allowed to do),
- constraints and prohibitions (what the role must not do),
- expected output forms (what artifacts the role typically produces).

A role is stable across time; implementations may evolve via versioned Agent Definitions.

### 3.3 Agent Definition {#agent-definition}

**Definition (normative):** An **Agent Definition** is a versioned configuration artifact that specifies how a role is instantiated in a particular runtime/harness.

- In SOLVER-Ralph, the canonical governed container for this is `config.agent_definition` (SR-TYPES).
- Agent Definitions are referenced for **audit provenance** and tooling selection; they do not grant new authority beyond the governed system.

**Constraint:** If an Agent Definition would change binding meaning (e.g., iteration context compilation rules, trust-boundary behavior, portal routing, what counts as verified), that change is governance-impacting and MUST route through SR-CHANGE.

### 3.4 Agent Instance {#agent-instance}

**Definition (normative):** An **Agent Instance** is a concrete runtime instantiation of an Agent Role using a specific Agent Definition version.

An Agent Instance MUST be attributable via the SR-SPEC actor identity model for agents (e.g., `actor_id = agent:<deployment_id>:<agent_instance_id>`). Additional identity fields (role id, definition id/version, model name) SHOULD be included in evidence manifests and logs to support audit/replay.

### 3.5 Subagent {#subagent}

**Definition (normative):** A **Subagent** is an agent instance invoked by another agent as an internal assistant.

Subagents are treated as **work-plane assistants**:

- they operate under a narrower tool and context envelope than the parent agent,
- their outputs are inputs to the parent agent’s work, not binding statements to the system,
- they MUST NOT introduce “ghost inputs” (any semantic input they use must be visible to the parent agent via explicit files/refs).

> Note: In Claude Code, subagents are typically defined as separate files under `.claude/agents/`. This is an adapter detail; the ontology here is runtime-agnostic.

---

## 4. Foundational agent subtypes {#foundational-subtypes}

This section is **informative**. It defines a small set of roles that are repeatedly useful as building blocks.

### 4.1 Taxonomy axes {#taxonomy-axes}

SOLVER-Ralph commonly classifies agents along two axes:

1) **Wave (deployment intent)**  
   - **Wave 1 (bootstrap):** agents used to build SOLVER / SOLVER-Ralph itself (platform + governance substrate).  
   - **Wave 2 (instance):** agents used to solve a specific problem within a deployed SOLVER-Ralph instance (domain package outputs).  
   - **Wave 3 (embedded):** agents shipped inside an instance so that class of problems can be solved repeatedly with baked-in human inputs and approvals.

2) **Function (what the agent contributes)**  
   - **Work producer:** drafts and changes the candidate work product.  
   - **Governance navigator:** retrieves authoritative clauses and precedence.  
   - **Compliance checker:** checks invariants and gaps against SR-CONTRACT/SR-SPEC/SR-DIRECTIVE.  
   - **Evidence librarian:** packages outputs into admissible evidence objects/manifests.  
   - **Reviewer / risk scout:** flags correctness, safety, testability, and change-management triggers.

These axes intentionally do **not** define new authority. They are a way to name predictable capability envelopes.

### 4.2 Foundational role registry (recommended identifiers) {#foundational-role-registry}

The following stable `agent_role_id` values are recommended for use in `config.agent_definition.ext.agent_definition.agent_role_id` (SR-TYPES).

| Foundational role | Suggested `agent_role_id` | Typical wave(s) | Typical outputs |
|---|---|---|---|
| Ralph Worker (Work Producer) | `ralph.worker` | 1 / 2 / 3 | Candidate edits, run requests, iteration summaries |
| Governance Navigator | `ralph.gov.navigator` | 1 / 2 | Clause excerpts, precedence notes, “blocked: missing ref” reports |
| Compliance Checker | `ralph.gov.compliance_checker` | 1 / 2 | Pass/fail matrix, invariant gaps list |
| Evidence Librarian | `ralph.evidence.librarian` | 1 / 2 / 3 | Evidence manifest drafts (paths + hashes + relationships) |
| Code Reviewer | `ralph.review.code` | 1 / 2 / 3 | Findings list, testability risks, oracle-ready checklist gaps |
| Change Scout | `ralph.change.scout` | 1 / 2 / 3 | “Change-management needed” signals + suggested scope/severity |

> These are *role identifiers*, not artifact IDs. The corresponding Agent Definition artifacts (type `config.agent_definition`) should use stable artifact IDs (e.g., `AG-...`) and SemVer versions.

### 4.3 Primary work-plane agent {#primary-agent}

#### 4.3.1 Ralph Worker (Work Producer) {#ralph-worker}

**Role intent:** Turn an `IterationStarted` event into a concrete outcome for the iteration: candidate changes, evidence requests/runs, and an iteration summary.

**Why it is foundational:** SOLVER-Ralph is built around Ralph Loops; every loop needs a work-producing agent that can operate under explicit provenance and gate/evidence expectations.

### 4.4 Governance and coherence assistants {#gov-coherence}

#### 4.4.1 Governance Navigator {#governance-navigator}

**Role intent:** Quickly locate and quote applicable clauses from governed artifacts so the parent agent stays grounded in authoritative text.

**Primary value:** Reduces semantic drift and “invented rules” during long runs; supports the **Ontological** and **Change** harnesses (ETT).

#### 4.4.2 Change Scout {#change-scout}

**Role intent:** Detect when work crosses into change-management territory (normative meaning changes, exceptions needed) and route to the appropriate change process.

**Primary value:** Preserves the **Change** harness: reality stays honest and versioned rather than silently diverging.

### 4.5 Compliance and evidence assistants {#compliance-evidence}

#### 4.5.1 Compliance Checker {#compliance-checker}

**Role intent:** Run checklist-style checks against SR-CONTRACT/SR-SPEC invariants and report pass/fail + gaps.

**Primary value:** Supports the **Authority & Integrity** harness (ETT) by preventing “accidental violations” from shipping as if compliant.

#### 4.5.2 Evidence Librarian {#evidence-librarian}

**Role intent:** Package outputs into an evidence-style manifest (paths, hashes, relationships) suitable for later ingestion and audit.

**Primary value:** Supports the **Accountability** harness (ETT): reproducibility and replay improve as a first-class output.

#### 4.5.3 Code Reviewer {#code-reviewer}

**Role intent:** Review for correctness, safety, and oracle-ready testability; flags risks and gaps but does not approve.

**Primary value:** Improves the probability that verification gates can be satisfied without human time being wasted on preventable failures.

---

## 5. Agent configuration and representation {#agent-configuration}

### 5.1 Governed agent definitions (`config.agent_definition`) {#config-agent-definition}

SOLVER-Ralph treats agent behavior configuration as a **versioned, auditable configuration artifact**.

Directional requirements for `config.agent_definition` are defined in SR-TYPES. This document adds one interpretation guideline:

- An Agent Definition should be referenced from `IterationStarted.refs[]` for audit provenance (typically `rel=supported_by`) so that later reviewers can reconstruct “which agent profile was used” without relying on transcript memory.

### 5.2 Claude Code subagents as an adapter {#claude-code-subagents}

Claude Code subagent files are a runtime-level representation of Agent Definitions.

A Claude Code subagent definition should:

- include stable identity metadata (so it can be selected, versioned, and audited),
- declare tool access explicitly,
- declare a structured output contract so the parent agent can consume results deterministically.

### 5.3 Examples (informative) {#examples}

#### 5.3.1 Example: `config.agent_definition` (Ralph Worker) {#example-agent-definition}

```yaml
---
name: ralph-worker
description: Primary work producer for Ralph Loops (drafts candidates, triggers oracles, writes iteration summaries).
tools: Read, Grep, Glob, Bash, Write
model: sonnet
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "AG-RALPH-WORKER"
  type: "config.agent_definition"
  title: "Ralph Worker"
  version: "1.0.0"
  status: "governed"
  normative_status: "directional"
  authority_kind: "config"
  governed_by: ["SR-CHANGE"]
  created: "2026-01-10"
  updated: "2026-01-10"
  ext:
    agent_definition:
      agent_role_id: "ralph.worker"
      capability_profile_id: "cap.full_stack_dev"
      context_profile_id: "ctx.iteration_refs_default"
      tool_access_profile_id: "tools.dev_full"
      template_catalog_ref:
        kind: "GovernedArtifact"
        id: "SR-TEMPLATES@1.0.0"
        meta: { content_hash: "sha256:<...>" }
---
```

#### 5.3.2 Example: Claude Code subagent file {#example-claude-subagent}

```markdown
<!-- FILE: .claude/agents/governance-navigator.md -->
---
name: governance-navigator
description: Locate and quote applicable clauses from governed artifacts for the current task.
tools: Read, Grep, Glob
model: sonnet
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "AG-GOVERNANCE-NAVIGATOR"
  type: "config.agent_definition"
  title: "Governance Navigator"
  version: "1.0.0"
  status: "governed"
  normative_status: "directional"
  authority_kind: "config"
  governed_by: ["SR-CHANGE"]
  created: "2026-01-10"
  updated: "2026-01-10"
  ext:
    agent_definition:
      agent_role_id: "ralph.gov.navigator"
      capability_profile_id: "cap.read_only"
      context_profile_id: "ctx.gov_docs_only"
      tool_access_profile_id: "tools.read_only"
---
You are the Governance Navigator subagent for SOLVER-Ralph.

Return:
- `status: ok | blocked`
- `quotes[]`: short excerpts with file + anchor
- `notes`: precedence conflicts, missing refs
```

---

## 6. Minimal compatibility constraints {#compatibility-constraints}

This section is **normative**, but intentionally small. These constraints exist to prevent category errors where an “agent” is treated as a control-plane authority.

1. **No new authority by configuration**  
   Agent Definitions MUST NOT grant or imply binding authority (approvals, portal crossings, verified claims) beyond what SR-CONTRACT/SR-SPEC allow.

2. **Actor identity must be attributable**  
   Any agent participating in SOLVER-Ralph event production MUST be representable as `actor_kind=AGENT` with a stable `actor_id` per SR-SPEC.

3. **No ghost semantic inputs**  
   Any semantic input used by an agent that affects outputs intended for verification/approval MUST be referenceable (documented, filed, or attached) so it can be included in `refs[]` or evidence manifests where appropriate.

---

## 7. Design notes {#design-notes}

### 7.1 Agents as epistemic instruments {#agents-epistemic}

In SOLVER-Ralph, an agent is not defined primarily by “intelligence” but by its **epistemic interface**:

- What can it legitimately take as input?
- What does it produce that can become an input to others?
- How do we bind its outputs to evidence and authority?

The ontology here is designed to support the broader goal: **trustworthy knowledge generation** under stochastic generation.

### 7.2 Why the foundational subtype set is small {#small-set}

A small set of agent subtypes is a feature, not a limitation:

- It minimizes coordination overhead (lower “entropy”).
- It encourages composability (a primary worker + assistants).
- It keeps enforcement surfaces and audit expectations clear.

---

## 8. Change control {#change-control}

This document is governed by **SR-CHANGE**.

Any modification that changes:
- the definition of Agent / Subagent,
- the foundational role registry (§4.2),
- or the normative constraints (§6),

must be processed as a governed change under SR-CHANGE (with appropriate scope/severity classification).

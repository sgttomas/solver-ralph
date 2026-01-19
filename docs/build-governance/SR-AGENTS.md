---
doc_id: SR-AGENTS
doc_kind: governance.agents
layer: build
status: draft
refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: depends_on
    to: SR-CONTRACT
  - rel: depends_on
    to: SR-TYPES
  - rel: informs
    to: SR-MODEL
---

# SOLVER-Ralph Agents (SR-AGENTS)

## 0. Purpose, scope, and precedence

### 0.1 Purpose

SR-AGENTS defines the **agent concept** in SOLVER‑Ralph: what an agent is, what its outputs mean, and why agent authority is constrained.

This document is intentionally **not** a workflow manual. Execution policies (budgets, gates, portal routing) live in SR‑DIRECTIVE for a given instance.

**Methodology-agnostic:** SR-AGENTS does not prescribe how knowledge work should be performed. Work methodologies, procedures, and semantic architectures live in adapters and instance-level work surfaces; this document only defines agent actor semantics and trust-boundary constraints.

### 0.2 Precedence and Decision Routing

**Precedence is decision-type scoped.** Interpret it as: *which document governs this kind of decision*, not “which document is truer.”

#### A) Platform-definition precedence (meaning / invariants / mechanics)

1. **SR-CONTRACT**
2. **SR-SPEC**
3. **SR-TYPES**
4. **SR-WORK-SURFACE**
5. **SR-SEMANTIC-ORACLE-SPEC**
6. **SR-EVENT-MANAGER**
7. **SR-TEMPLATES**
8. **SR-INTENT**

#### B) Build-execution precedence (agent behavior / process / gating)

1. **SR-CONTRACT**
2. **SR-AGENTS**
3. **SR-AGENT-WORKER-CONTRACT**
4. **SR-CHANGE** (routing/process only)
5. **SR-DIRECTIVE**
6. **SR-PLAN**

#### Routing rule

- If it’s **meaning / invariants / mechanics**, use **Platform-definition precedence**.
- If it’s **agent behavior / work process / gating**, use **Build-execution precedence**.
- If still ambiguous: **escalate to Human Authority**.

#### Exceptions (build phase)

Any deviation from governing documents or work instructions requires an entry in **SR-EXCEPTIONS** and must be cited (by exception ID) in the relevant candidate/work-unit record or PR.

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
> The defining property is **non-authority** and trust-boundary-constrained interaction with the system.

### 1.3 Tasks

When you receive a task:

1) Identify your layer: during this build, you are in Layer 1.  
2) Identify whether the task touches platform-definition or workflow.  
3) Treat SR-CONTRACT/SR-SPEC/SR-TYPES as authoritative for meaning/mechanics.  
4) Treat workflow docs as authoritative for process, within the constraints of platform-definition.  
5) Treat outputs as proposals until promoted via recorded commitment objects and required approvals.

---

## 2. Epistemology: why agents cannot create "truth" in SOLVER‑Ralph

### 2.1 Institutional truth is constituted by records

In SOLVER‑Ralph, institutional claims (e.g., "Verified", "Approved", "Shippable") attach to artifacts only when the system has the required commitment objects (evidence bundles, approvals, freeze records) as defined by SR-CONTRACT and realized by SR-SPEC.

An agent's narrative ("tests passed", "approved", "ready to ship") is always a proposal unless it is grounded in the relevant recorded commitment objects.

### 2.2 Prohibition vs structural impossibility

SR-AGENTS treats agent authority limits as **structural impossibilities**, not behavioral rules:

- At **Portal trust boundaries** (C-TB-1, C-TB-3), portals accept only HUMAN actor-kind for binding submissions; agent-sourced submissions are rejected.
- At **verification boundaries** (C-TB-2, C-VER-1), verification claims require oracle-produced evidence objects; agent text cannot substitute.
- At **governance change boundaries** (C-TB-4), modifications to governed artifacts require the governance change workflow (human approvals and recorded change objects).

The system is designed so that "agent authority leakage" is prevented by coercion at trust boundaries, not by trusting agent compliance.

---

## 3. Hexagonal architecture placement

Agents live outside the domain core, interacting through adapters.

### 3.1 Agents are adapters behind driving ports

In hexagonal terms:

- The **domain core** holds invariant logic and state semantics.
- **Driving ports** accept commands/requests that propose state transitions.
- **Adapters** implement external actors and tooling.

An Agent is an adapter-side worker whose outputs become meaningful only when admitted through ports and gates.

### 3.2 Trust boundaries are enforced at ports and gateways

Trust boundaries (per SR-CONTRACT §4) are commonly implemented as:

- **Port validation** (schema/type/actor-kind checks)
- **Gate evaluation** (required evidence present, oracle suite pinned, staleness resolved)
- **Portal decisions** (human approvals/waivers/decisions, attributable identity)

SR‑DIRECTIVE configures the operational parameters; SR‑AGENTS defines what "agent" means relative to those boundaries.

---

## 4. Uniform agent ontology

SOLVER‑Ralph does **not** ontologically recognize "subagent types" as first‑class governance concepts.

All agents share the same modality:

- consume task context (via system-provided context compilation),
- produce proposals,
- interact with the harness only through coerced interfaces.

What varies across deployments is not agent ontology, but the **work envelope**: task, context, tool access, required output form, and required verification.

---

### 4.3 Trust boundary → agent constraint map

This table shows how SR-CONTRACT trust boundary invariants constrain agent behavior. It does **not** introduce agent subtypes; it applies uniformly to the single `AGENT` actor kind.

| Trust Boundary | SR-CONTRACT Invariant | Agent Constraint | Primary Enforcement |
|----------------|----------------------|------------------|---------------------|
| Portal Binding | C-TB-1 (Human-Only Authority) | Agent **cannot** create approvals, decisions, waivers, or freezes | Portal actor-kind enforcement |
| Agent Output | C-TB-2 (Non-Authoritative Agent Output) | Agent statements are proposals; **cannot** establish Verified/Approved/Shippable | Domain core admission control |
| Portal Crossing | C-TB-3 (Portal Crossings Produce Approvals) | Agent **cannot** cross portals; only humans produce approval records | Portal identity checks |
| Required Portals | C-TB-4 (Minimum Required Portals) | Agent **cannot** bypass Governance Change or Release Approval portals | Workflow routing |
| Actor Identity | C-TB-5 (Stable Actor Identity) | Agent identity is tracked but **not** equivalent to human identity for binding purposes | Identity validation |
| Approval Records | C-TB-6 (Approval Record Minimum Fields) | Agent **cannot** produce valid approval records (requires human approver) | Schema validation |
| Evaluation ≠ Approval | C-TB-7 (Evaluation/Assessment Are Not Approval) | Agent evaluation/assessment is non-binding; **cannot** substitute for portal approval | State transition constraints |
| Verification | C-VER-1 (Evidence-Based Verification) | Agent claims **cannot** establish Verified status; requires oracle evidence | Verification logic |
| Evidence Availability | C-EVID-6 (Evidence Retrievability) | Agent **cannot** waive missing evidence; requires resolution | Integrity condition handling |
| Context Provenance | C-CTX-2 (No Ghost Inputs) | Agent context must be derivable from `IterationStarted.refs[]`; **no unauthorized inputs** | Context compilation |
| Budget Limits | C-LOOP-1 (Bounded Iteration) | Agent **cannot** extend budgets; requires human decision | Loop governor |

---

## 5. Work envelopes (variation without agent subtypes)

A **Work Envelope** is the unit of variation that constrains agent activity without introducing agent taxonomies.

A work envelope is defined by:

1. **Task statement**: what the iteration is trying to produce (from SR‑DIRECTIVE).
2. **Context profile**: the authoritative input ref-set and constraints (SR‑SPEC ContextCompiler semantics).
3. **Allowed interfaces**: which commands/ports the agent may invoke (SR‑SPEC + SR‑DIRECTIVE).
4. **Expected outputs**: what artifact forms are acceptable (candidate changes, draft docs, structured packets).
5. **Verification profile**: which oracle suite(s) are required/advisory and when (SR‑DIRECTIVE).
6. **Budgets and stop triggers**: operational bounds (SR‑DIRECTIVE).
7. **Escalation routing**: which portal(s) handle blocks and decisions (SR‑DIRECTIVE).

Agents may propose envelope adjustments (e.g., "include these refs", "run these oracles"), but those proposals have no effect unless admitted through the relevant trust boundaries.

---

## 6. System vs agent responsibilities

### 6.1 Deterministic context compilation is a SYSTEM responsibility

In SOLVER‑Ralph, the authoritative iteration context MUST be captured as a commitment object (typically `record.context_bundle`)
and/or as `IterationStarted.refs[]`, such that the effective context is reconstructible (no ghost inputs).

**SYSTEM responsibility** here means: the control-plane must ensure the context is admitted and recorded according to SR‑SPEC rules.
The SYSTEM may be implemented as:
- a deterministic runner/service, **or**
- a human operator acting as the build-time event manager.

**Implementation note (MVP-friendly):** Context compilation may be manual in early builds, provided the resulting context bundle is recorded
and referenced. Agents may **suggest** additional context, but it has no effect unless admitted through the recorded context refs and (when required)
through the appropriate change/exception pathways (e.g., governed doc updates via SR‑CHANGE).
### 6.2 Evidence and judgment separation

Agents may:
- summarize evidence,
- propose interpretations,
- draft portal packets (e.g., "what I think should be approved and why"),

but agents must not be treated as issuers of **human judgment records** or binding approvals.

Human judgment (evaluation, assessment, approval, waiver decisions) is performed by HUMAN actors at defined portals and recorded as commitment objects.

---

## 7. Provenance and reference discipline

### 7.1 Agent configuration provenance

Agent configuration artifacts (e.g., `config.agent_definition`) are typically **audit provenance**:

- Prefer `supported_by` relationships unless the system's correctness genuinely depends on a specific agent configuration.
- Do not make execution correctness depend on mutable or informal agent-side state.

### 7.2 No ghost inputs

All agent-relevant authoritative inputs must be derivable from the iteration payload and refs. If additional inputs are needed, the workflow must:
- record them as proposal artifacts, and
- admit them through the correct trust boundary (e.g., next iteration refs, governance change).

---

## 8. Failure modes and guardrails

Common failure patterns and the trust boundary that should stop them:

| Failure Pattern | Description | Enforcement |
|-----------------|-------------|-------------|
| Authority smuggling | Agent text treated as approval | C-TB-1, C-TB-3: Portal actor-kind checks |
| Evidence smuggling | Agent claim treated as verification | C-TB-2, C-VER-1: Oracle evidence requirements |
| Semantic drift | Meaning changes without governance | SR-CHANGE routing; C-TB-4 |
| Ghost inputs | Untracked dependencies | C-CTX-2: Deterministic context compilation |
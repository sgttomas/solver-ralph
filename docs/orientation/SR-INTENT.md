---
doc_id: SR-INTENT
doc_kind: governance.design_intent
layer: build
status: draft
normative_status: directional

refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: depends_on
    to: SR-CONTRACT
  - rel: depends_on
    to: SR-SPEC
  - rel: depends_on
    to: SR-TYPES
  - rel: informs
    to: SR-CHARTER
  - rel: informs
    to: SR-MODEL
  - rel: informs
    to: SR-AGENTS
  - rel: informs
    to: SR-PLAN
  - rel: informs
    to: SR-DIRECTIVE
---

# SR-INTENT (Semantic)
## Purpose

This document records **design intent** for SOLVER-Ralph in its current direction: a platform whose core capability is running **Semantic Ralph Loops** (work units) that:
- perform bounded tasks with fresh-context iterations,
- record durable artifacts and evidence,
- and **request human approval at trust boundaries** before a work unit can be treated as complete.

This is a **directional** document: it explains *why* we are building specific core behaviors, and what risks those behaviors mitigate.

## Scope and Non-Authority

**This document is not binding.** It MUST NOT be used to:
- define or modify platform invariants,
- define or modify runtime types,
- define state machines, gates, or lifecycle rules,
- or impose a methodology for “how knowledge work should be done.”

**Binding meaning lives elsewhere:**
- SR-CONTRACT defines invariants and authority boundaries.
- SR-SPEC defines mechanics (events, projections, artifacts, gates).
- SR-TYPES defines the type registry.

If SR-INTENT conflicts with SR-CONTRACT / SR-SPEC / SR-TYPES, SR-INTENT is wrong.

## The Core Problem (Why SOLVER-Ralph exists)

Agents are non-deterministic and can produce outputs that *sound* correct while being wrong or unsafe. The platform’s job is not to “make agents smart,” but to make **agent work governable**:

- outputs are treated as **proposals** unless they are promoted via recorded commitment objects,
- verification is performed by **oracles that emit evidence** (which supports decisions but is not authority),
- binding decisions cross a **human authority boundary** and must be recorded.

## What the Core Hexagon Is (and is not)

### The core is minimal by design

The platform core (the hexagon) provides:
1. **Work Unit runtime** (Semantic Ralph Loops): bounded, iterative execution.
2. **Event-sourced state** and deterministic projections (status, eligibility).
3. **Evidence intake**: a standard way to attach oracle outputs to candidates.
4. **Authority ports**: a standard way to request and record approvals/exceptions.

The core does **not** prescribe:
- how to decompose a problem into tasks,
- how to conduct knowledge work (research, writing, analysis),
- what an ontology must look like,
- or what semantic evaluation “should” be.

Those choices belong in **adapters** and configuration layered around the core.

### Adapter-first stance for knowledge-work semantics

Domain-specific semantics (including any meaning-space architecture, ontologies, matrices/manifolds, rubrics, or procedures) are treated as:
- **oracle implementations** (adapters) and their configurations,
- **work-surface definitions** (adapters) and their schemas,
- and application-level policies.

The platform provides ports and auditability; adapters supply methodology.

## Key Design Decisions (Rationale)

1) **Separate generation, evaluation, and authority**
- Agents generate candidates (proposal-capable).
- Oracles generate evidence (verification-capable).
- Humans authorize binding transitions (authority-capable).

2) **Event-sourced state**
- The system state is derived by replaying recorded events.
- “What is done?” is computed, not narrated.

3) **“Verified” is indexed and conditional**
Verification is meaningful only relative to:
- a declared oracle suite/profile, and
- (when applicable) the stage or context in which it was evaluated.

Verification is not truth; it is evidence-backed admissibility under declared rules.

4) **Hexagonal boundary as enforcement boundary**
The platform core enforces invariants and authority boundaries.
Everything else is an adapter.

## Non-binding Heuristics (do not promote to platform ontology)

This section uses descriptive metaphors to guide intuition during build. These are not runtime types unless explicitly defined in SR-TYPES/SR-SPEC.

- **Exploration vs Commitment:** within an iteration, outputs are provisional; when outputs become load-bearing inputs for downstream work, stricter enforcement applies.
- **Boundary-first enforcement:** enforce at transitions (candidate submission, evidence recording, approval), not by prescribing how a task must be done.

## Assumptions and Re-evaluation Triggers

These assumptions guide the build; violations should trigger escalation to human authority.

- Deterministic state progression is possible even with non-deterministic actors.
  - Trigger: state becomes non-replayable or depends on unstated inputs.
- Oracles can be defined (at least for governance-level checks) for most work units.
  - Trigger: frequent ORACLE_GAP that blocks core operation.
- Human approvals remain meaningful and are not bypassed by evidence.
  - Trigger: processes drift toward “auto-approval” without authority records.

## For Agents

When you receive a task:
1. Identify your layer: during build, you are in **Layer 1**.
2. Treat SR-CONTRACT/SR-SPEC/SR-TYPES as authoritative; treat SR-INTENT as rationale only.
3. Do not infer requirements or methodologies from this document.
4. If you need a new requirement, propose it as a change to SR-CONTRACT/SR-SPEC/SR-TYPES (and escalate if it changes meaning/invariants).
5. If you are unsure whether something is a binding claim, stop and escalate to human authority.

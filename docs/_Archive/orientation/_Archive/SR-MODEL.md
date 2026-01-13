---
doc_id: SR-MODEL
doc_kind: governance.conceptual_model
layer: build
status: draft
normative_status: directional

refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: depends_on
    to: SR-CHARTER
  - rel: depends_on
    to: SR-CONTRACT
  - rel: depends_on
    to: SR-SPEC
  - rel: depends_on
    to: SR-TYPES
  - rel: informs
    to: SR-AGENTS
---

# SR-MODEL (Semantic)

Conceptual foundation for building SOLVER-Ralph without confusing:
- build-time scaffolding with runtime behavior,
- directional guidance with binding semantics,
- and domain-specific methodology with platform core.

---

## The Three Layers

### Layer 1: Build (where we are now)

In Layer 1, the SR-* documents exist as artifacts (files). Agents and humans read them and produce code and documentation increments under governance.

The SR-* documents are **inputs**; the running platform is the **output**.

### Layer 2: Platform (what we are building)

In Layer 2, the platform exists as enforcement behavior:
- the work unit runtime (Semantic Ralph Loops),
- event recording and deterministic projections,
- evidence intake and gating,
- authority ports for approvals/exceptions.

Users do not “read SR-* docs”; they interact with platform ports/APIs and experience the spec as behavior.

### Layer 3: Usage (what users experience)

Layer 3 is the product surface: the APIs and tooling that applications and users call to create work units, attach evidence, request approvals, and read records.

The platform is intentionally **methodology-agnostic**: domain semantics are expressed through adapters and configuration, not by baking a single theory of knowledge work into the core.

---

## Core vs Adapters (Hexagonal Architecture)

### Core (the hexagon)

The core provides governed primitives:
- Work Units / Iterations
- Events and replayable state projection
- Candidate and evidence attachment
- Approval/exception request and recording

### Adapters

Adapters connect the core to particular domains and workflows:
- oracle implementations (including semantic evaluation)
- external systems (docs, storage, ticketing, UI)
- domain-specific work-surface templates and policies

The core enforces invariants; adapters provide domain meaning.

---

## Two Specifications

This effort contains two distinct specifications:

### Platform-definition (meaning + mechanics)

Defines what SOLVER-Ralph *is* and what it *does*:
- SR-CONTRACT (invariants, authority boundaries)
- SR-SPEC (mechanics: events, projections, artifacts, gates)
- SR-TYPES (schemas and canonical types)

These define the platform’s semantics and must not be contradicted by other documents.

### Build-execution (agent workflow)

Defines how agents and humans work to build the platform:
- SR-AGENTS (agent operating constraints)
- SR-CHANGE (how governed docs are changed)
- SR-PLAN (scope and dependencies)
- SR-DIRECTIVE (execution policy and gating during build)
- SR-GUIDE / SR-INTENT / SR-CHARTER (directional)

Workflow documents must not introduce new platform semantics.

---

## Document Dependencies (conceptual, not a substitute for SR-SPEC)

A simplified dependency shape:

- SR-CONTRACT constrains everything.
- SR-TYPES registers canonical schemas used by SR-SPEC.
- SR-SPEC defines mechanics and references SR-TYPES.
- SR-DIRECTIVE governs build-time execution and references platform-definition docs.
- SR-PLAN defines scope/dependencies and is operationalized by SR-DIRECTIVE.

If you need the authoritative dependency graph, use `refs` in document front matter and consult the Event Manager/state projections once available.

---

## For Agents

When you receive a task:

1) Identify your layer: during this build, you are in Layer 1.  
2) Identify whether the task touches platform-definition or workflow.  
3) Treat SR-CONTRACT/SR-SPEC/SR-TYPES as authoritative for meaning/mechanics.  
4) Treat workflow docs as authoritative for process, within the constraints of platform-definition.  
5) Treat outputs as proposals until promoted via recorded commitment objects and required approvals.

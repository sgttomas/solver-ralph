---
doc_id: SR-GUIDE
doc_kind: governance.usage_guide
layer: build
status: draft
normative_status: directional

refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: depends_on
    to: SR-CHARTER
  - rel: constrained_by
    to: SR-CONTRACT
  - rel: depends_on
    to: SR-SPEC
  - rel: depends_on
    to: SR-TYPES
  - rel: informs
    to: SR-AGENTS
  - rel: informs
    to: SR-PLAN
  - rel: informs
    to: SR-DIRECTIVE
---

# SR-GUIDE (Semantic)
## Purpose

SR-GUIDE is a **directional orientation** for humans and agents participating in the build of SOLVER-Ralph.

It exists to prevent two common failure modes:
- confusing *documents* for *platform behavior* (“the recursion trap”),
- and accidentally treating non-binding explanations as binding platform semantics.

## Scope and Non-Authority

This document is **not** a platform definition and must not be used as a substitute for:
- SR-CONTRACT (binding invariants),
- SR-SPEC (binding mechanics),
- SR-TYPES (binding schemas).

If you need a definitive answer about behavior, ignore SR-GUIDE and consult those documents.

## What SOLVER-Ralph Is

SOLVER-Ralph is a platform whose core capability is running **Semantic Ralph Loops** (work units):

- A work unit is a bounded unit of agent work.
- It runs in iterations with fresh-context boundaries.
- It produces **candidates** (artifacts) and **evidence** (oracle outputs).
- It requests and records **human approval** at trust boundaries before a work unit can be treated as complete.
- It records an audit trail of events and computes state deterministically from that record.

## What SOLVER-Ralph Is Not

SOLVER-Ralph does **not** prescribe:
- how knowledge work must be performed,
- how to decompose problems into tasks,
- what ontology or semantic model you must use,
- what “good” means for a given domain.

Those choices belong to **adapters** and application policy layered around the core (e.g., domain-specific oracle suites, work-surface templates, UI/UX, integrations).

The platform provides *governance infrastructure* (loops, evidence, approvals, state). You provide the domain semantics and decision criteria.

## How to Use This Documentation Set (quick map)

### Platform-definition (binding)
- SR-CONTRACT — invariants, authority boundaries
- SR-SPEC — mechanics: events, projections, artifacts, gates
- SR-TYPES — schema registry and canonical types

### Build-execution (binding for process, not meaning)
- SR-AGENTS — agent operating constraints
- SR-CHANGE — how governed docs are modified
- SR-DIRECTIVE — execution policy (budgets, stop triggers, routing)
- SR-PLAN — current build scope and dependencies

### Directional (non-binding)
- SR-CHARTER — human intent and priorities for this build
- SR-INTENT — rationale and design intent
- SR-GUIDE — this orientation guide
- SR-MODEL — conceptual map of layers and document roles

## Practical Rule

If a paragraph in any document can be interpreted as:
- defining a new lifecycle state,
- defining a new type,
- changing what “Verified/Approved/Complete” means,
- or defining enforcement rules,

then it **does not belong in SR-GUIDE** and must be moved into SR-CONTRACT/SR-SPEC/SR-TYPES (or removed).

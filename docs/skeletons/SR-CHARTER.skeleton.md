---
doc_id: SR-CHARTER
doc_kind: governance.charter
layer: build
status: draft

refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: depends_on
    to: SR-TYPES
  - rel: constrained_by
    to: SR-CONTRACT
  - rel: informs
    to: SR-WORK-SURFACE
  - rel: informs
    to: SR-PROCEDURE-KIT
  - rel: informs
    to: SR-SEMANTIC-ORACLE-SPEC
  - rel: informs
    to: SR-EVENT-MANAGER
  - rel: informs
    to: SR-MODEL
  - rel: informs
    to: SR-AGENTS
  - rel: informs
    to: SR-PLAN
  - rel: informs
    to: SR-DIRECTIVE
  - rel: informs
    to: SR-EXCEPTIONS
---

# SR-CHARTER (Semantic)

## About

**Purpose:** SR-CHARTER is the typed, governed entry-point for the project. It translates a human free-form problem statement into a stable charter that sets scope, milestones, and authority boundaries for building SOLVER-Ralph (semantic-ralph-loops).

**Normative status:** Binding for *scope and priorities* during build, but it may not redefine platform invariants or mechanics (those are defined by SR-CONTRACT / SR-SPEC / SR-TYPES).

**Current build focus (MVP):** implement the **semantic-ralph-loop core** (bounded semantic work → structured oracle evidence → human approval → completion event). All other components are treated as adapters/commodities and may be stubbed unless explicitly authorized.


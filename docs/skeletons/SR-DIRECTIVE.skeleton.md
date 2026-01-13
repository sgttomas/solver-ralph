---
doc_id: SR-DIRECTIVE
doc_kind: governance.execution_directive
layer: program
status: draft
refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: depends_on
    to: SR-CONTRACT
  - rel: depends_on
    to: SR-SPEC
  - rel: depends_on
    to: SR-TYPES
  - rel: depends_on
    to: SR-WORK-SURFACE
  - rel: depends_on
    to: SR-EVENT-MANAGER
---

# SR-DIRECTIVE — Execution Policy for Semantic Ralph Loops

## About

**Purpose:** Define the operational policy for running Semantic Ralph Loops: scheduling/eligibility rules, iteration budgets, stop triggers, required artifacts, and gating/approval routing.

**Normative status:** **Normative (binding)** within the scope of a given plan instance. SR-DIRECTIVE constrains “how to run,” while SR-PLAN constrains “what exists,” and SR-SPEC constrains “what it means operationally.”

**Core execution commitments:**
- **Dependency-first scheduling:** only eligible work units (deps satisfied) may run.
- **Deterministic eligibility input:** the Event Manager computes `eligible_set` from events + dependency graph.
- **One work unit per iteration** (and typically one stage per iteration when stage-gated).
- **Stop-the-line triggers** route to defined escalation/portal policy.
- **Completion is computed** from recorded artifacts and evidence, not model narrative.

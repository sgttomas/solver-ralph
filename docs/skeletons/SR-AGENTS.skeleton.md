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
7. **SR-PROCEDURE-KIT**
8. **SR-GUIDE**
9. **SR-INTENT**

#### B) Build-execution precedence (agent behavior / process / gating)

## About

(TODO)

---
doc_id: SR-EVENT-MANAGER
doc_kind: governance.event_manager_spec
layer: platform
status: draft
refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: depends_on
    to: SR-SPEC
  - rel: depends_on
    to: SR-TYPES
  - rel: depends_on
    to: SR-PLAN
  - rel: depends_on
    to: SR-DIRECTIVE
---

# SR-EVENT-MANAGER — Deterministic State, Eligibility, and Dependency Graph Computation

## About

**Purpose:** Specify the deterministic **Event Manager** responsibilities: compute the current state of work units and the dependency graph solely from the ordered event stream, and derive the `eligible_set` used for scheduling Semantic Ralph Loops.

**Normative status:** **Normative (binding).** Eligibility and status computations MUST be deterministic and rebuildable by replaying recorded events; projections are non-authoritative but must be reproducible.

**Core outputs:**
- `status_by_work_unit` (including stage position where applicable)
- `dependency_graph_projection` (from SR-PLAN)
- `eligible_set` (deps satisfied + not blocked + not complete)
- optional: runlist views and staleness/invalidation traversals as configured

---
doc_id: SR-TASKS
doc_kind: governance.usage_guide
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
  - rel: depends_on
    to: SR-EVENT-MANAGER
  - rel: depends_on
    to: SR-DIRECTIVE
  - rel: depends_on
    to: SR-AGENT-WORKER-CONTRACT
  - rel: informs
    to: SR-PLAN
---

# SR-TASKS — Task tracking and assignment for Ralph-loop coders

## 0. Purpose

This document defines a **coherent, drift-resistant method** for:

1) tracking **total completion** and **work remaining** during the SOLVER-Ralph build, and  
2) assigning new work efficiently to agentic coders (Ralph-loop workers),

without creating a second “shadow truth source” (e.g., a manual task board).

This document is **directional**:
- It does **not** redefine platform invariants (SR-CONTRACT),
- does **not** redefine mechanics (SR-SPEC),
- and does **not** redefine the type registry (SR-TYPES).

If this document conflicts with SR-CONTRACT / SR-SPEC / SR-TYPES, this document is wrong.

---

## 1. Canonical mapping: “Task” means “Work Unit”

**Canonical rule:** a “task” is a **Work Unit** from the active **Plan Instance**.

- Task identity is the **work_unit_id**.
- Task scope and dependency structure come from the Plan Instance (`depends_on` edges).
- Task state is derived from the **event stream + governed policy snapshots** via deterministic projections (Event Manager).

**Consequences:**
- You do not maintain a separate authoritative task list.
- Progress is not narrated; it is **computed**.

---

## 2. Sources of truth for tracking

Task tracking is computed from exactly these inputs:

1) **Plan Instance (scope)**  
   Defines what tasks exist and how they depend on each other.

2) **Event Stream (what happened)**  
   Append-only event log recording IterationStarted, EvidenceBundleRecorded, StopTriggered, portal decisions, etc.

3) **Directive policy snapshot (how to judge)**  
   A content-addressed snapshot derived from SR-DIRECTIVE that pins:
   - stop triggers,
   - gate rules / completion requirements,
   - routing policy (seeded portals + request_type discipline).

No other hidden state may influence task status, completion, or eligibility.

---

## 3. Projections that power task tracking

The canonical tracking surface is the Event Manager’s deterministic projections:

### 3.1 `status_by_work_unit`

For each work unit id, a computed record including (minimum):

- `coarse_status`: `TODO | ELIGIBLE | IN_PROGRESS | BLOCKED | COMPLETE`
- `current_stage_id` (if stage-gated)
- `stage_status` map (if stage-gated)
- `deps_satisfied`
- `block_reasons[]`
- `staleness_markers[]`
- `last_iteration_id`
- `last_candidate_id`
- `last_evidence_bundle_id`

### 3.2 `eligible_set`

A deterministic set of work unit ids eligible **now** under the policy snapshot.  
Agents **must not** invent eligibility.

### 3.3 `dependency_graph_snapshot`

A reconstructible snapshot of nodes + edges with satisfaction annotations.

### 3.4 `runlist`

A human-friendly view grouped by `coarse_status`, with pointers to evidence bundles and block reasons that justify each status.

**Implementation note:** The runlist is a *view*. The authoritative content is the event stream and derived projections.

---

## 4. Tracking total completion vs remaining work

### 4.1 Canonical counts

All counts are derived from projections.

- `total_tasks` = count of work units in the Plan Instance
- `complete` = count of work units where `coarse_status == COMPLETE` **and** `staleness_markers[]` is empty
- `remaining` = `total_tasks - complete`

For operational scheduling you also track:

- `eligible` = count where `coarse_status == ELIGIBLE`
- `in_progress` = count where `coarse_status == IN_PROGRESS`
- `blocked` = count where `coarse_status == BLOCKED`
- `stale` = count where `staleness_markers[]` is non-empty (even if otherwise COMPLETE)

### 4.2 Recommended `runlist` summary block

Include a summary object alongside the runlist (machine and human friendly):

```json
{
  "summary": {
    "total": 142,
    "complete": 31,
    "remaining": 111,
    "eligible": 9,
    "in_progress": 4,
    "blocked": 18,
    "stale": 6
  }
}
```

### 4.3 Optional: stage-weighted progress

For stage-gated procedures, you may compute an additional progress metric:

- `stage_progress(work_unit)` = completed_stages / total_stages_in_procedure_template

Then an aggregate:

- `overall_stage_progress` = mean(stage_progress over all work units)

This is optional and must not become a “new truth source.” It is a convenience metric only.

---

## 5. Efficient assignment to agentic coders

### 5.1 Assignment pool is `eligible_set`

**Canonical rule:** new work is assigned only from `eligible_set`.

Agents must not:
- select non-eligible tasks, or
- recompute eligibility, or
- bypass stop triggers / portal requirements.

### 5.2 Deterministic prioritization

To avoid “random choice” and reduce wasted work, rank eligible tasks deterministically.

Recommended priority order:

1) **Unblock / critical-path score**  
   Prefer tasks whose completion unlocks the most downstream work.

2) **Explicit plan priority (if present)**  
   If the Plan Instance includes priority metadata, use it as a tie-breaker.

3) **Smallest-scope-first (if size estimates exist)**  
   If tasks have a size tag, prefer smaller tasks when unblock scores are equal.

4) **Stable tie-breaker**  
   Lexicographic order by `work_unit_id` (ensures determinism).

The goal is not “optimal scheduling” but **consistent scheduling**.

### 5.3 Concurrency control: leases (supporting, not authoritative)

In multi-agent mode, two workers could choose the same eligible work unit.

Use a **short-lived lease/claim** as an operational coordination tool:

- A scheduler issues a lease on a `(work_unit_id, lease_id, ttl)` tuple.
- Leases are **supporting** (non-authoritative) artifacts; they do not change truth.
- If a lease expires, the work unit may be re-assigned.

Leases prevent duplicated work without introducing a second semantic authority layer.

### 5.4 Work packets (what the scheduler gives the worker)

Each assignment should be handed to the worker as a **work packet** containing:

- `projection_snapshot_ref` (the exact projection that produced eligible_set)
- the selected `work_unit_id`
- `work_surface_instance_ref` (intake + procedure template + current stage + oracle suites/profile binding)
- `directive_policy_snapshot_ref`
- a reference to the current **docs baseline / freeze record** (so all agents code against the same pinned meanings)

This makes assignment reproducible and audit-friendly.

---

## 6. What SR-CHANGE governs vs what projections govern

### 6.1 Plan changes

Changes to the scope of work (tasks) must go through SR-CHANGE:

- add/remove work units
- change dependencies (`depends_on`)
- change binding acceptance criteria or gate rules

### 6.2 Progress changes

Progress is not manually edited. It occurs when:

- events are recorded (iteration, evidence bundle, decisions),
- and projections update deterministically.

### 6.3 “Discovered work”

If implementation reveals missing tasks:
- propose a Plan change via SR-CHANGE,
- attach evidence (failing tests, schema mismatch, missing type keys),
- once approved, the new work unit enters the Plan Instance and appears in projections.

---

## 7. Boot prompt integration (iterative but pinned)

The “boot prompt” given to Ralph-loop coders is an **operational guide** that must:

- instruct workers to treat the kernel as authoritative (SR-CONTRACT/SR-SPEC/SR-TYPES),
- require workers to use `eligible_set` and not invent eligibility,
- require stop-and-escalate when unknown event names or type keys are encountered,
- reference the current docs baseline (freeze record) and projection snapshot.

**Iterative rule:** the boot prompt may evolve as new failure modes are discovered, but each revision must be:
- content-addressed/pinned, and
- associated with the docs baseline used during that run.

Do not allow “floating prompts” that silently change meaning mid-build.

---

## 8. Minimal implementation checklist

A build is ready for efficient task assignment when:

- [ ] Event stream is append-only and replayable.
- [ ] Plan Instance exists as a commitment object with `depends_on` edges.
- [ ] Directive policy snapshot is content-addressed and referenced.
- [ ] Event Manager can produce: `status_by_work_unit`, `eligible_set`, `dependency_graph_snapshot`, `runlist`.
- [ ] Workers are invoked with a projection snapshot + work packet.
- [ ] Stop triggers reliably pause work until a portal decision is recorded.
- [ ] A freeze record exists for the docs baseline used by the coding agents.

---

## 9. Notes on drift prevention

If any of these occur, stop and route via SR-CHANGE (do not “fix in code” silently):

- SR-DIRECTIVE references event names that differ from SR-SPEC canonical events.
- A type key used in any doc or artifact is not registered in SR-TYPES.
- A directional doc appears to add binding semantics (kernel conflict).
- A new portal identity is introduced instead of using seeded portals + request_type.


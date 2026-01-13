---
doc_id: SR-EVENT-MANAGER
doc_kind: governance.event_manager_spec
layer: platform
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
    to: SR-DIRECTIVE
---

# SR-EVENT-MANAGER — Deterministic state + eligibility computation

**Purpose:** Specify the deterministic **Event Manager / Projection Builder** that computes:

- work-unit state (including stage state for stage-gated procedures),
- dependency satisfaction,
- eligibility sets for agent scheduling,

as a deterministic function of the ordered event stream + governed plan inputs.

This is the “small truth source” scaling mechanism: agents do not carry status in their context windows; they consume projections derived from events.

---

## 1. Inputs (normative)

The Event Manager MUST consume:

1) **Event stream**: the ordered append-only event store (`es.events` or equivalent).
2) **Plan Instance**: a commitment object containing work units and `depends_on` edges (SR-PLAN instance).
3) **Directive policy**: the active SR-DIRECTIVE (stop triggers, completion criteria, gate rules).

No other hidden state may influence computed results.

---

## 2. Outputs (normative projections)

The Event Manager MUST be able to produce the following projections:

### 2.1 status_by_work_unit

For each work unit id:

- `coarse_status`: TODO | ELIGIBLE | IN_PROGRESS | BLOCKED | COMPLETE
- `current_stage_id` (if stage-gated)
- `stage_status` map `{stage_id -> {status, last_evidence_bundle_ref}}`
- `deps_satisfied`: boolean
- `block_reasons[]`: stop triggers / integrity conditions / missing portal decisions
- `last_iteration_id`
- `last_candidate_id`
- `last_evidence_bundle_id`

### 2.2 eligible_set

A set of work unit ids that are eligible to be selected **now** under SR-DIRECTIVE policy.

### 2.3 dependency_graph_snapshot

A machine-readable snapshot of the dependency graph with:

- nodes (work units)
- edges (`depends_on`)
- computed satisfaction annotations (deps complete / deferred / blocked)

### 2.4 runlist (human-friendly view)

A reconstructible summary grouped by status (TODO/ELIGIBLE/IN_PROGRESS/BLOCKED/COMPLETE) with pointers to the evidence bundles that justify each status.

---

## 3. Determinism and rebuildability (normative)

- All projections MUST be rebuildable from scratch by replaying the event stream in order.
- Incremental projection updates (checkpoints/caches) are permitted, but MUST be observationally equivalent to full replay.
- Projection results MUST be deterministic for a given (event stream + governed inputs) tuple.

---

## 4. Completion predicate (normative)

A work unit is **COMPLETE** iff:

- the terminal stage (as declared by its Procedure Template / Work Surface) has a recorded Evidence Bundle whose required oracle suites PASS under declared gate rules, AND
- any required portal decisions for completion/release (if applicable) are recorded.

“Agent says done” is not a completion predicate.

---

## 5. Eligibility predicate (normative)

A work unit is **ELIGIBLE** iff:

- it is not COMPLETE, and
- it is not BLOCKED by a stop trigger requiring portal relief, and
- all `depends_on` prerequisites are COMPLETE (or explicitly deferred), and
- it has a valid Work Surface binding (intake + procedure + stage), if it is a semantic work unit.

Eligibility MUST be computed by the Event Manager and supplied to the agent as a projection artifact. The agent may choose among eligible work units, but must not invent eligibility.

---

## 6. Event folding model (recommended minimal approach)

A minimal v1 projector may:

1) parse events sequentially,
2) update in-memory maps (status_by_work_unit, last_candidate, last_evidence),
3) compute deps_satisfied via the plan graph,
4) compute eligible_set from the predicate in §5.

Thousands of events are expected to be tractable by replay + cache.

---

## 7. Required event touchpoints (non-exhaustive)

The Event Manager relies on at least:

- `IterationStarted` (context provenance)
- `CandidateMaterialized`
- `EvidenceBundleRecorded`
- `StopTriggered`
- `ApprovalRecorded` / waiver/deferral events

Semantic-specific events MAY be present (e.g., `WorkSurfaceRecorded`, `StageCompleted`), but stage completion can also be derived from evidence bundles bound to stage ids.


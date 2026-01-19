---
doc_id: SR-WORK-SURFACE
doc_kind: governance.work_surface_spec
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
  to: SR-TEMPLATES
- rel: depends_on
  to: SR-SEMANTIC-ORACLE-SPEC
---

# SR-WORK-SURFACE — Work Surface for Semantic Ralph Loops

**Purpose:** Define the governed *work surface* artifacts that a Semantic Ralph Loop operates on: **Intake**, **Procedure Template**, **Procedure Stages**, and the binding **Work Surface Instance** used by the runtime to compile iteration context and to bind evidence to stage gates.

This document defines **what must exist** (schemas + invariants). It does not prescribe *which* semantic sets you use (that belongs to oracle suites / SR-SEMANTIC-ORACLE-SPEC) and it does not prescribe global scheduling policy (SR-DIRECTIVE).

---

## 1. Normative keywords

The keywords **MUST**, **SHOULD**, **MAY** are to be interpreted as in RFC 2119.

---

## 2. Core concepts

### 2.1 Work Surface

A **Work Surface** is the binding context for an iteration in a Semantic Ralph Loop. It is the minimum set of commitment objects required to make:

- candidate generation procedurally constrained, and
- semantic oracle evaluation stage-specific and reproducible.

A Work Surface binds:

- a **Work Unit** (the loop scope),
- an **Intake** (what the work is),
- a **Procedure Template** (how candidates are generated and gated),
- a **Stage** (what is being attempted this iteration),
- the **Oracle Profile / Suite(s)** for that stage (including semantic set definitions).

### 2.2 Stage-gated procedure

A **Procedure Template** is a *stage machine*. Each **Stage**:

- declares required intermediate artifacts,
- declares required oracle suite(s),
- declares a gate decision rule (how oracle results determine pass/fail for that stage).

The stage gate is the knowledge-work analogue of “tests pass”.

---

## 3. Intake schema (v1)

The Intake is a work-unit-scoped artifact. It MAY be authored by a human, derived from a prompt decomposition pipeline, or edited over time — but **when used as binding iteration context it MUST be treated as a commitment object and referenced by hash**.

### 3.1 Required fields

Intake MUST include:

- `work_unit_id` (stable id)
- `title`
- `kind` (work kind taxonomy string; e.g., `research_memo`, `decision_record`, `ontology_build`)
- `objective` (one sentence)
- `audience`
- `deliverables[]` (exact required outputs; include format and paths if applicable)
- `constraints[]` (length, tone, required sections, prohibited content, time/recency, etc.)
- `definitions{}` (term → definition; may be empty)
- `inputs[]` (provided context refs; may be empty)
- `unknowns[]` (questions to resolve; may be empty)
- `completion_criteria[]` (human-facing acceptance criteria; not the authoritative gate)

### 3.2 Recommended carrier format

The canonical carrier format is Markdown with YAML front matter:

```yaml
---
artifact_type: record.intake
artifact_version: v1
work_unit_id: WU-...
title: ...
kind: research_memo
audience: ...
objective: ...
deliverables:
  - path: candidate/main.md
    media_type: text/markdown
constraints:
  - "Must include sections: Background, Options, Recommendation"
definitions:
  termA: "..."
inputs:
  - rel: depends_on
    kind: context
    locator: refs/problem_statement.md
unknowns:
  - "What time horizon matters?"
completion_criteria:
  - "Reviewer can act on recommendation."
---
# Intake
...
```

---

## 4. Procedure Template schema (v1)

Procedure Templates are governed configuration artifacts. They SHOULD be reusable across work units of the same kind.

### 4.1 Required fields

A Procedure Template MUST include:

- `procedure_template_id` (stable id)
- `kind` (what work kinds it applies to)
- `stages[]`, where each stage includes:
  - `stage_id`
  - `stage_name`
  - `purpose`
  - `required_outputs[]` (artifacts the worker must produce at this stage)
  - `steps[]` (proceduralized instructions for the worker)
  - `required_oracle_suites[]` (suite ids; semantic suite ids permitted)
  - `gate_rule` (how to decide pass/fail for the stage from oracle outputs)
  - `transition_on_pass` (next stage_id or terminal)
- `terminal_stage_id`

### 4.2 Recommended carrier format

Markdown with YAML front matter is recommended:

```yaml
---
artifact_type: config.procedure_template
artifact_version: v1
procedure_template_id: proc:GENERIC-KNOWLEDGE-WORK
kind: [research_memo, decision_record]
terminal_stage_id: stage:FINAL
stages:
  - stage_id: stage:FRAME
    stage_name: Frame the problem
    required_outputs:
      - path: artifacts/context/frame.md
        role: context
    required_oracle_suites: ["suite:SR-SUITE-STRUCTURE"]
    gate_rule: "all_required_oracles_pass"
    transition_on_pass: stage:OPTIONS
  - ...
---
# Procedure Template
...
```

---

## 5. Work Surface Instance schema (v1)

A Work Surface Instance is the binding of a specific work unit to:

- an Intake,
- a Procedure Template,
- the current stage,
- and the oracle profile/suites for that stage.

### 5.1 Required fields

Work Surface Instance MUST include:

- `artifact_type: domain.work_surface` (aligns with SR-TYPES `domain.work_surface`)
- `work_unit_id`
- `intake_ref` (content-addressed ref)
- `procedure_template_ref` (content-addressed ref)
- `stage_id`
- `oracle_suites[]` (suite ids + hashes; MUST include any semantic set bindings via suite hash)
- `params{}` (optional; stage parameters, semantic set selectors, thresholds)

### 5.2 Recommended carrier format

```yaml
---
artifact_type: domain.work_surface
artifact_version: v1
work_unit_id: WU-...
intake_ref:
  id: intake:...
  content_hash: sha256:...
procedure_template_ref:
  id: proc:...
  content_hash: sha256:...
stage_id: stage:FRAME
oracle_suites:
  - suite_id: suite:SR-SUITE-STRUCTURE
    suite_hash: sha256:...
params:
  threshold: 0.9
---
```

### 5.3 Relationship to SR-SPEC events

- **Suite-hash binding:** When semantic oracles are used, the Work Surface instance and any `EvidenceBundleRecorded` events MUST declare the oracle suite IDs **and** `suite_hash`. The `suite_hash` is the epistemic anchor for semantic set definitions (see SR-SEMANTIC-ORACLE-SPEC).
- `IterationStarted` MUST reference the Work Surface Instance (directly or via its component refs), per SR-CONTRACT C-CTX-1/C-CTX-2.
- `EvidenceBundleRecorded` MUST bind evidence to (`candidate_id`, `procedure_template_id`, `stage_id`) per SR-SPEC §1.9.1.

(Canonical evidence artifact type key: `domain.evidence_bundle`.)

### 5.4 Loop ↔ Work Surface binding (normative)

When a Loop is created with an explicit `work_unit` parameter, the platform validates that an active Work Surface exists for that work unit and captures the binding:

**Binding semantics:**
- If `work_unit` is explicitly provided during Loop creation, the platform MUST validate an active Work Surface exists for that `work_unit_id`.
- If validation succeeds, the Loop's `work_surface_id` field is populated with the Work Surface identifier.
- If no active Work Surface exists, Loop creation MUST fail with error code `WORK_SURFACE_MISSING` (HTTP 412).
- If `work_unit` is omitted, it defaults to the `loop_id` and no Work Surface binding occurs (the Loop is "unbound").

**Iteration context inheritance:**
- When a Loop is bound to a Work Surface (`work_surface_id` is set), iterations started without an explicit `work_unit_id` MUST auto-inherit the Loop's `work_unit` for Work Surface context resolution.
- This enables the platform to automatically include the Work Surface refs (Intake, Procedure Template, current stage, oracle suites) in the iteration context without requiring the caller to specify them explicitly.

**Projection model:**
- The Loop projection (`proj.loops`) includes `work_surface_id` as an optional field.
- The relationship is unidirectional: Loop → Work Surface. The Work Surface does not maintain a back-reference to Loops.
- Multiple Loops MAY reference the same Work Surface (e.g., parallel exploration of different approaches for the same work unit).

### 5.5 Starting work via /start endpoint (normative)

The platform provides a single orchestration endpoint to fully initialize work on a Work Surface:

`POST /work-surfaces/{work_surface_id}/start`

This endpoint performs the complete initialization sequence:

1. **Creates a Loop** bound to the Work Surface's `work_unit_id`
   - Uses authenticated caller's identity (typically `actor_kind=HUMAN`) on `LoopCreated` event
   - Sets default `directive_ref` to `{ kind: "doc", id: "SR-DIRECTIVE", rel: "governs", meta: {} }`

2. **Activates the Loop** (CREATED → ACTIVE transition)

3. **Starts an Iteration** as SYSTEM actor
   - `IterationStarted` event uses `actor_kind=SYSTEM`, `actor_id=system:work-surface-start`
   - Iteration refs are populated from the Work Surface context (Intake, Procedure Template, current stage, oracle suites)

**Response:**
```json
{
  "work_surface_id": "ws_...",
  "loop_id": "loop_...",
  "iteration_id": "iter_...",
  "already_started": false
}
```

**Idempotency:** The endpoint is idempotent. If a Loop already exists for the work unit:
- If ACTIVE with `iteration_count > 0`: returns existing IDs with `already_started: true`
- If exists but not ACTIVE: activates and starts iteration
- If exists and ACTIVE but no iteration: starts iteration

**Precondition:** Work Surface MUST be in `active` status. Returns HTTP 412 with error code `WORK_SURFACE_NOT_ACTIVE` if the Work Surface is not active.

**Actor mediation rationale:** Per SR-SPEC §2.2, iteration creation MUST be SYSTEM-mediated. The `/start` endpoint handles this automatically while preserving human audit trail on Loop creation. This is the recommended way for UI clients to orchestrate work initiation.

**Relationship to direct Loop/Iteration APIs:**
- Clients MAY still use `POST /loops` and `POST /loops/{id}/iterations` directly for advanced use cases
- The `/start` endpoint is a convenience orchestration that handles the common case correctly
- When using `/start`, clients do not need to separately manage Loop creation, activation, and iteration start

---

## 6. Artifact layout conventions (recommended)

To make procedure outputs and evidence machine-checkable, a work unit SHOULD use a stable directory layout:

- `candidate/` — primary deliverable artifacts
- `artifacts/` — intermediate artifacts required by procedure stages
- `evidence/` — structured evidence outputs (claims ledgers, traceability maps, etc.)
- `logs/` and `reports/` — oracle outputs referenced by evidence bundles

---

## 7. What this enables

By making Intake + Procedure Template + stage explicit and referenceable, the platform can:

- enforce “no ghost inputs”,
- compute stage-specific completion from evidence,
- attach semantic oracle suites with high specificity,
- and support deterministic replay of “what was believed and why” at each stage gate.


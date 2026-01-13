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
  to: SR-PROCEDURE-KIT
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

### 5.2 Relationship to SR-SPEC events

- `IterationStarted` MUST reference the Work Surface Instance (directly or via its component refs), per SR-CONTRACT C-CTX-1/C-CTX-2.
- `EvidenceBundleRecorded` MUST bind evidence to (`candidate_id`, `procedure_template_id`, `stage_id`) per SR-SPEC §1.9.1.

(Canonical evidence artifact type key: `domain.evidence_bundle`.)

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


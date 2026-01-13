---
doc_id: SR-PROCEDURE-KIT
doc_kind: governance.procedure_kit
layer: platform
status: draft
refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: depends_on
    to: SR-WORK-SURFACE
  - rel: depends_on
    to: SR-SEMANTIC-ORACLE-SPEC
---

# SR-PROCEDURE-KIT — Procedure Templates for Semantic Ralph Loops

**Purpose:** Provide a governed registry of reusable **Procedure Templates** for semantic knowledge work. These templates proceduralize candidate generation so that semantic oracle suites can be attached with specificity.

This is a *kit*: it supplies canonical skeletons and conventions. Concrete domain manifolds and oracle suites are referenced, not defined here.

---

## 1. Registry conventions

Each procedure template is identified by a stable id:

- `proc:<NAME>`

A procedure template MUST declare:

- supported `kind` values (work kinds),
- `stages[]` with required outputs and required oracle suites,
- `terminal_stage_id`.

---

## 2. Baseline template: GENERIC-KNOWLEDGE-WORK (v1)

This baseline template is intentionally abstract. It is suitable when you want a consistent stage structure across many kinds of knowledge work, while allowing different semantic manifolds per stage.

### Template id

- `procedure_template_id: proc:GENERIC-KNOWLEDGE-WORK`

### Stages

#### stage:FRAME
**Goal:** Restate objective, audience, non-goals; extract constraints and definitions into machine-checkable artifacts.

**Required outputs:**
- `artifacts/context/frame.md`
- `artifacts/context/constraints.json`
- `artifacts/context/definitions.json`

**Required oracle suites (examples):**
- `suite:SR-SUITE-STRUCTURE` (required sections present; schema validation)

**Transition on pass:** `stage:OPTIONS`

---

#### stage:OPTIONS
**Goal:** Generate multiple candidate approaches/outlines *before* drafting.

**Required outputs:**
- `artifacts/candidates/option_A.md`
- `artifacts/candidates/option_B.md`
- `artifacts/candidates/selection.md` (chosen option + why)

**Required oracle suites (examples):**
- `suite:SR-SUITE-NONTRIVIALITY` (guards against vacuous options)

**Transition on pass:** `stage:DRAFT`

---

#### stage:DRAFT
**Goal:** Produce the candidate deliverable(s) in requested structure; produce traceability artifacts.

**Required outputs:**
- `candidate/main.<md|pdf|...>`
- `evidence/traceability.json`

**Required oracle suites:**
- `suite:SR-SUITE-STRUCTURE`
- `suite:SR-SUITE-TRACEABILITY`

**Transition on pass:** `stage:SEMANTIC_EVAL`

---

#### stage:SEMANTIC_EVAL
**Goal:** Evaluate candidate against the stage manifold / meaning-matrix suite(s). Capture structured semantic measurements and derived pass/fail.

**Required outputs:**
- `reports/semantic/residual.json`
- `reports/semantic/coverage.json`
- `reports/semantic/violations.json`

**Required oracle suites:**
- `suite:SR-SUITE-SEMANTIC:<domain_or_stage>` (implementation-defined; see SR-SEMANTIC-ORACLE-SPEC)

**Transition on pass:** `stage:FINAL`

---

#### stage:FINAL
**Goal:** Package final candidate + summary; ensure evidence bundle references everything required to reconstruct context and evaluation.

**Required outputs:**
- `candidate/final.md` (or equivalent)
- `evidence/gate_packet.json` (recorded by platform as `evidence.gate_packet`)

**Required oracle suites:**
- `suite:SR-SUITE-REFS` (no ghost inputs; refs integrity)
- stage-appropriate semantic suite (if required by profile)

**Transition on pass:** terminal

---

## 3. How to add a new procedure template

To add a new procedure:

1) Define its stages and required outputs in the format specified by SR-WORK-SURFACE.
2) Define which oracle suites apply at each stage.
3) Register the template under `proc:<NAME>` and treat it as a governed artifact (versioned, hashed).
4) Update any plan/decomposition rules that map work kinds to procedure template ids.

---

## 4. Notes on stage granularity

Stages should be designed so that:

- each stage can be completed within one or a small number of iterations,
- each stage has oracle suites that can be run deterministically (or with bounded nondeterminism),
- stage completion can be computed from recorded evidence bundles and portal decisions.


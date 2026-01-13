---
doc_id: SR-PROCEDURE-KIT
doc_kind: governance.procedure_kit
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
    to: SR-WORK-SURFACE
  - rel: depends_on
    to: SR-SEMANTIC-ORACLE-SPEC
---

# SR-PROCEDURE-KIT — Procedure Templates for Semantic Ralph Loops

**Naming note:** Canonical work-unit surface form is `semantic-ralph-loop` (other variants are aliases).
**Evidence note:** Evidence Bundle is the domain object (`domain.evidence_bundle`). The Evidence Bundle manifest uses `artifact_type = evidence.gate_packet`.

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

**Schema alignment note:** The full Procedure Template schema is defined in SR-WORK-SURFACE §4.1. This document uses an abbreviated format for readability; concrete implementations MUST include all required fields per SR-WORK-SURFACE:

| SR-WORK-SURFACE Field | This Document | Notes |
|----------------------|---------------|-------|
| `stage_id` | ✅ Present | e.g., `stage:FRAME` |
| `stage_name` | Implied from header | e.g., "Frame the problem" |
| `purpose` | "Purpose:" | Maps to `purpose` field |
| `required_outputs[]` | "Required outputs:" | |
| `steps[]` | Omitted | Implementation-specific; add per work kind |
| `required_oracle_suites[]` | "Required oracle suites:" | |
| `gate_rule` | Implicit | Default: `all_required_oracles_pass` unless stated |
| `transition_on_pass` | "Transition on pass:" | |

---

## 2. Baseline template: GENERIC-KNOWLEDGE-WORK (v1)

This baseline template is intentionally abstract. It is suitable when you want a consistent stage structure across many kinds of knowledge work, while allowing different semantic manifolds per stage.

### Template id

- `procedure_template_id: proc:GENERIC-KNOWLEDGE-WORK`

### Stages

#### stage:FRAME
**Purpose:** Restate objective, audience, non-goals; extract constraints and definitions into machine-checkable artifacts.

**Required outputs:**
- `artifacts/context/frame.md`
- `artifacts/context/constraints.json`
- `artifacts/context/definitions.json`

**Required oracle suites (examples):**
- `suite:SR-SUITE-STRUCTURE` (required sections present; schema validation)

**Transition on pass:** `stage:OPTIONS`

---

#### stage:OPTIONS
**Purpose:** Generate multiple candidate approaches/outlines *before* drafting.

**Required outputs:**
- `artifacts/candidates/option_A.md`
- `artifacts/candidates/option_B.md`
- `artifacts/candidates/selection.md` (chosen option + why)

**Required oracle suites (examples):**
- `suite:SR-SUITE-NONTRIVIALITY` (guards against vacuous options)

**Transition on pass:** `stage:DRAFT`

---

#### stage:DRAFT
**Purpose:** Produce the candidate deliverable(s) in requested structure; produce traceability artifacts.

**Required outputs:**
- `candidate/main.<md|pdf|...>`
- `evidence/traceability.json`

**Required oracle suites:**
- `suite:SR-SUITE-STRUCTURE`
- `suite:SR-SUITE-TRACEABILITY`

**Transition on pass:** `stage:SEMANTIC_EVAL`

---

#### stage:SEMANTIC_EVAL
**Purpose:** Evaluate candidate against the stage manifold / meaning-matrix suite(s). Capture structured semantic measurements and derived pass/fail.

**Required outputs:**
- `reports/semantic/residual.json`
- `reports/semantic/coverage.json`
- `reports/semantic/violations.json`

**Required oracle suites:**
- `suite:SR-SUITE-SEMANTIC:<domain_or_stage>` (implementation-defined; see SR-SEMANTIC-ORACLE-SPEC)

**Transition on pass:** `stage:FINAL`

---

#### stage:FINAL
**Purpose:** Package final candidate + summary; ensure evidence bundle references everything required to reconstruct context and evaluation.

**Required outputs:**
- `candidate/final.md` (or equivalent)
- `evidence/gate_packet.json` (recorded by platform as `evidence.gate_packet`)

*Clarification:* `evidence/gate_packet.json` is the **manifest carrier** for the iteration’s Evidence Bundle(s) (domain object `domain.evidence_bundle`). The bundle MUST bind to `candidate_id`, `procedure_template_id`, `stage_id`, and any relevant `suite_hash` values so replay can determine “what was verified”.

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

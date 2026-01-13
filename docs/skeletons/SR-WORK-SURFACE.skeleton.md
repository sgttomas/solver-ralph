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
    to: SR-TYPES
---

# SR-WORK-SURFACE — Intake + Procedure Surface for Semantic Ralph Loops

## About

**Purpose:** Define the **work surface** for knowledge work: (1) a standardized **Intake** template, and (2) a standardized **Procedure** template that proceduralizes candidate generation into **stage-gated steps**, enabling semantic oracles to be defined with high specificity per stage.

**Normative status:** **Normative (binding).** Workers MUST operate on a declared work surface (intake + procedure template) and MUST emit required work-surface artifacts and evidence.

**Work-surface principle:** Oracles evaluate outputs *relative to a stage-defined semantic manifold/profile*. Therefore the procedure must specify what artifacts exist at each stage (candidate artifacts + structured semantic artifacts), so oracle suites can operate deterministically and audibly.

### Schema commitment: `record.context_bundle` (Context Artifact)

Each iteration MUST produce a **Context Bundle** record that captures the complete effective context used for that iteration (no ghost inputs). The canonical schema is:

```yaml
type_id: record.context_bundle

required_fields:
  - context_bundle_id
  - iteration_id
  - work_unit_id
  - work_kind
  - procedure_template_id
  - stage_id
  - oracle_profile_id
  - oracle_suite_refs
  - created_at
  - actor
  - authoritative_inputs
  - completion_criteria

typed_ref_minimum_fields:
  - rel          # depends_on | supported_by
  - kind         # governed_doc | context | candidate | evidence | approval | oracle_suite | output | source
  - locator      # file path, URI, or store locator
  - content_hash # REQUIRED for commitment objects

recommended_fields:
  - task_local_inputs
  - constraints
  - budgets
  - notes
```

**Binding rule:** Anything materially relied upon (docs, sources, datasets, prior artifacts, suites) MUST appear in `authoritative_inputs` or `task_local_inputs` as typed refs with hashes where applicable.

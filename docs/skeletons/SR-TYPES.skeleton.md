---
doc_id: SR-TYPES
doc_kind: governance.schema_registry
layer: platform
status: draft
refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: depends_on
    to: SR-CONTRACT
---

# SR-TYPES — Type Registry for Semantic Ralph Loops

## About

**Purpose:** Define the canonical schema registry for SOLVER-Ralph: the types of governed documents, runtime artifacts, records, candidates, evidence packets, and projections used by **Semantic Ralph Loops**.

**Normative status:** **Normative (binding).** Types defined here constrain SR-SPEC, SR-DIRECTIVE, SR-EVENT-MANAGER, and all adapters.

**Key idea:** Semantic Ralph Loops depend on a **work surface** (intake + procedure stages) and **semantic oracle outputs**. SR-TYPES makes these first-class so completion and eligibility can be computed deterministically from recorded events.

### Schema commitment: `record.context_bundle` (Context Artifact)

The system standardizes each iteration’s “what the agent relied on” artifact as a **Context Bundle** record.

```yaml
type_id: record.context_bundle
description: >
  A content-addressed, auditable context artifact capturing the complete
  effective context for a single iteration: governed inputs, task-local inputs,
  procedure/stage binding, oracle profile binding, and declared completion criteria.
required: true

fields:
  context_bundle_id: { type: string, required: true }         # stable identifier (may equal content hash)
  iteration_id:      { type: string, required: true }
  work_unit_id:      { type: string, required: true }

  work_kind:         { type: string, required: true }         # e.g., research_memo | decision_record | ontology_step | etc.
  procedure_template_id: { type: string, required: true }
  stage_id:          { type: string, required: true }         # stage within procedure template

  oracle_profile_id: { type: string, required: true }         # semantic oracle profile name/id
  oracle_suite_refs: { type: array,  required: true }         # typed refs to suite definitions (hash/versioned)

  created_at:        { type: string, required: true }         # ISO-8601 timestamp
  actor:             { type: string, required: true }         # SYSTEM | AGENT (or both with roles)

  authoritative_inputs:
    type: array
    required: true
    items:
      ref: typed_ref                                         # must include content_hash when dereferenceable

  task_local_inputs:
    type: array
    required: false
    items:
      ref: typed_ref                                         # includes sources, notes, datasets, etc.

  constraints:
    type: object
    required: false
    description: "Normalized constraints extracted from intake (optional but recommended)."

  completion_criteria:
    type: object
    required: true
    description: "Explicit pass conditions for this stage/work unit."

  budgets:
    type: object
    required: false
    description: "Iteration/time/resource budgets in effect."

  notes:
    type: string
    required: false
```

**Typed ref expectation:** `typed_ref` MUST support `rel`, `kind`, `locator`, and `content_hash` for commitment objects.

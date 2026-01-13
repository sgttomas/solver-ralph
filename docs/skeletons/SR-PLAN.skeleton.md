---
doc_id: SR-PLAN
doc_kind: governance.plan_instance
layer: program
status: draft
refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: depends_on
    to: SR-CONTRACT
  - rel: depends_on
    to: SR-DIRECTIVE
  - rel: depends_on
    to: SR-WORK-SURFACE
---

# SR-PLAN — Plan Instance for Semantic Ralph Loops

## About

**Purpose:** Define the decomposed inventory of **knowledge-work deliverables** (work units), their dependency graph (`depends_on`), and the mapping from work units to **work surface** procedure templates and semantic oracle profiles.

**Normative status:** **Normative (binding)** for scope and dependency structure. SR-PLAN defines *what exists and what depends on what*. It does not define platform mechanics (SR-SPEC) or runtime policy (SR-DIRECTIVE).

**Key commitments:**
- Dependencies are the only binding ordering.
- Each work unit declares: `work_kind`, `procedure_template_id`, `oracle_profile_id`.
- The decomposition is produced from a problem statement/prompt and recorded as a plan instance; subsequent scheduling is based on eligibility computed from events.

---
doc_id: SR-PROCEDURE-KIT
doc_kind: governance.procedure_registry
layer: platform
status: draft
refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: depends_on
    to: SR-WORK-SURFACE
  - rel: depends_on
    to: SR-TYPES
---

# SR-PROCEDURE-KIT — Registry of Procedure Templates

## About

**Purpose:** Provide a governed registry of reusable **procedure templates** for semantic knowledge work. Each template defines stage IDs, required outputs per stage, and the expected oracle profile bindings (or oracle requirements) that gate progression.

**Normative status:** **Normative (binding)** for any work unit that references a procedure template from this kit.

**What a procedure template specifies:**
- Stage sequence (and allowed iteration rules per stage)
- Required artifacts per stage (candidate outputs + structured semantic artifacts)
- Stage completion criteria (inputs required, outputs required)
- Required oracle profile(s) per stage (or constraints for selecting them)

---
doc_id: SR-AGENT-WORKER-CONTRACT
doc_kind: governance.worker_contract
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
    to: SR-WORK-SURFACE
  - rel: depends_on
    to: SR-EVENT-MANAGER
---

# SR-AGENT-WORKER-CONTRACT — Worker Behavior for Semantic Ralph Loops

## About

**Purpose:** Define the minimum behavioral contract for an agent worker executing Semantic Ralph Loops: selecting an eligible work unit, operating on the work surface (intake + procedure stage), producing candidates and evidence, and emitting events in a way that keeps the system deterministic, auditable, and stage-gated.

**Normative status:** **Normative (binding).** Workers that do not follow this contract must be treated as untrusted and their outputs must not advance binding state.

**Core duties (high level):**
- Consume the computed `eligible_set` and select exactly one work unit per iteration (record rationale).
- Produce a `record.context_bundle` prior to material work (no ghost inputs).
- Generate candidate artifacts according to the declared procedure stage.
- Run the configured semantic oracle suite(s) and produce an evidence packet bound to candidate + stage.
- Never declare “complete” without the required artifacts and gate satisfaction.

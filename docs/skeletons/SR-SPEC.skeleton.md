---
doc_id: SR-SPEC
doc_kind: governance.platform_spec
layer: platform
status: draft
refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: depends_on
    to: SR-CONTRACT
  - rel: depends_on
    to: SR-TYPES
  - rel: depends_on
    to: SR-WORK-SURFACE
  - rel: depends_on
    to: SR-SEMANTIC-ORACLE-SPEC
  - rel: depends_on
    to: SR-EVENT-MANAGER
---

# SR-SPEC — Technical Specification for Semantic Ralph Loops

## About

**Purpose:** Specify the deterministic mechanics of SOLVER-Ralph for **Semantic Ralph Loops**: events, state machines, artifacts, evidence recording, gating, and stage progression across a proceduralized knowledge-work “work surface.”

**Normative status:** **Normative (binding)** for platform behavior, bounded by SR-CONTRACT. Implementation details (languages, frameworks, storage engines) are non-binding unless elevated by SR-CONTRACT.

**Scope (what this spec covers):**
- Event model and rebuildable projections (state derived by replaying events).
- Artifact identity for semantic candidates (including manifest-based candidates).
- Evidence bundles (e.g., `evidence.gate_packet`) binding oracle outputs to candidate + stage.
- Stage-gated procedure execution (work surface integration).
- Integration points for deterministic Event Manager and oracle runner adapters.

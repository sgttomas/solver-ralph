---
doc_id: SR-CONTRACT
doc_kind: governance.arch_contract
layer: platform
status: draft
refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: depends_on
    to: SR-TYPES
  - rel: depends_on
    to: SR-SPEC
  - rel: depends_on
    to: SR-WORK-SURFACE
---

# SR-CONTRACT — Architectural Contract for Semantic Ralph Loops

## About

**Purpose:** Define the binding invariants for SOLVER-Ralph when running **Semantic Ralph Loops**: stage-gated knowledge work that produces **semantic candidates** and validates them using **semantic oracles** (e.g., meaning-matrix/manifold evaluators) rather than code-only feedback loops.

**Normative status:** **Normative (binding).** If any other SR-* document conflicts with SR-CONTRACT, SR-CONTRACT wins.

**Key commitments:**
- **No ghost inputs:** any input relied upon by a loop must be represented as a referenced artifact.
- **Commitment objects vs proposals:** only content-addressed artifacts (candidates, evidence packets, approvals, context bundles) may drive binding state.
- **Verification is indexed:** “Verified” is always relative to a declared oracle suite/profile and (when applicable) a procedure stage.
- **Approval remains human authority:** verification evidence may support approval but does not replace it.

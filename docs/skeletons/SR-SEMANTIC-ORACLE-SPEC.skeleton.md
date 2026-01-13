---
doc_id: SR-SEMANTIC-ORACLE-SPEC
doc_kind: governance.oracle_spec
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
---

# SR-SEMANTIC-ORACLE-SPEC — Semantic Oracle Suites and Evidence Outputs

## About

**Purpose:** Define the normative interface for **semantic oracle suites** (including meaning-matrix / manifold evaluators) used by Semantic Ralph Loops: how suites are referenced and versioned, what structured outputs must be produced, and how those outputs are bound into evidence packets used for gating.

**Normative status:** **Normative (binding).** A “Verified” claim is only meaningful when indexed by the oracle suite/profile and (when applicable) the procedure stage.

**Core commitments:**
- Suites are **content-addressed** (or otherwise immutably versioned) and referenced by typed refs.
- Oracle outputs are **structured measurements** (e.g., residuals, coverage, violations) with optional derived pass/fail under declared thresholds.
- Evidence packets MUST bind: candidate identity + suite identity + stage identity + outputs.

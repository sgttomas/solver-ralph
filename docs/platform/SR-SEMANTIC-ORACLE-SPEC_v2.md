---
doc_id: SR-SEMANTIC-ORACLE-SPEC
doc_kind: governance.semantic_oracle_spec
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
---

# SR-SEMANTIC-ORACLE-SPEC — Semantic Oracles (Meaning Matrices / Semantic Sets)

**Purpose:** Define how semantic meaning matrices / semantic sets integrate into SOLVER-Ralph as oracle suites, and define the **minimum structured outputs** required so that stage gates can be evaluated deterministically and evidence can be audited.

This spec intentionally does **not** define your semantic-set mathematics. It defines the platform-visible interface: suite identity, required outputs, and evidence binding.

---

## 1. Semantic oracle as a platform concept

A **semantic oracle** is an oracle that emits structured semantic measurements about a Candidate’s position relative to a declared semantic set (meaning matrix).

Examples of measurements (non-exhaustive):
- residual vectors
- coverage metrics across semantic axes
- constraint violation lists
- coherence / transition constraints between stage semantic sets

Semantic oracles may be used as **Required Oracles** for stage gates.

---

## 2. Suite identity and semantic set binding (normative)

A semantic oracle suite MUST have:

- `oracle_suite_id` (stable name)
- `oracle_suite_hash` (content hash)

**Normative requirement:** the `oracle_suite_hash` MUST incorporate (directly or indirectly) all semantic set / meaning-matrix definitions that materially affect evaluation, including:
- semantic set version identifiers
- axis definitions / ontology bindings (opaque to platform, but hashed)
- decision-rule parameters for pass/fail derivation

If semantic set definitions change, the suite hash MUST change. This prevents “silent oracle drift.”

---

## 3. Required outputs (v1)


**Evidence terminology note:** “Evidence Bundle” refers to the platform domain object (`domain.evidence_bundle`). When referring to the serialized manifest/packet, the manifest may use `artifact_type = evidence.gate_packet`.


A semantic oracle run MUST produce machine-readable artifacts sufficient to:
- audit the measurement,
- compute pass/fail under declared rules,
- reproduce the run under declared environment constraints (as far as feasible).

Minimum required artifacts (paths are conventional; use manifest to bind):

- `reports/semantic/residual.json`
- `reports/semantic/coverage.json`
- `reports/semantic/violations.json`

These artifacts MUST be referenced in the Evidence Bundle manifest (domain object: `domain.evidence_bundle`; manifest `artifact_type`: `evidence.gate_packet`) via content hashes.

---

## 4. Standard result schema (normative for required semantic gates)

**Normative requirement:** If a semantic oracle suite is used as a **Required Oracle** for a stage gate, it MUST emit a stage evaluation result conforming to `sr.semantic_eval.v1` (or a declared compatible successor), and that result MUST include enough fields for gate pass/fail to be computed from recorded outputs without out-of-band assumptions.

The platform provides a canonical JSON schema for semantic stage evaluation:

```jsonc
{
  "schema": "sr.semantic_eval.v1",
  "candidate_id": "sha256:...",
  "procedure_template_id": "proc:...",
  "stage_id": "stage:SEMANTIC_EVAL",
  "oracle_suite_id": "suite:SR-SUITE-SEMANTIC:...",
  "oracle_suite_hash": "sha256:...",
  "semantic_set": {
    "semantic_set_id": "semantic set:...",
    "semantic_set_hash": "sha256:..."
  },
  "metrics": {
    "residual_norm": 0.12,
    "coverage": {"axisA": 0.9, "axisB": 0.7},
    "violations": [{"code": "MISSING_AXIS", "axis": "axisC"}]
  },
  "decision": {
    "status": "PASS",
    "rule_id": "rule:...",
    "thresholds": {"residual_norm_max": 0.2}
  },
  "notes": "optional human-readable notes"
}
```

The platform does not interpret `metrics` semantics; it only requires that the gate decision rule be computable from recorded outputs.

**Schema note:** `sr.semantic_eval.v1` is a wire format schema for semantic oracle outputs, defined locally within this specification. It is not a governed artifact type in SR-TYPES; it is a data schema for structured oracle results that are referenced within Evidence Bundles.

---

## 5. Determinism and bounded nondeterminism

Semantic oracle suites MUST declare their determinism assumptions, including:
- fixed seeds (if applicable),
- fixed model versions (if ML-based),
- fixed external dependencies (if any).

If a required semantic oracle is non-deterministic for identical inputs without a declared bounded nondeterminism policy, it MUST be treated as `ORACLE_FLAKE` per SR-CONTRACT.

---

## 6. Stage transition coherence (recommended oracle class)

Because semantic work is stage-gated, the platform RECOMMENDS a class of semantic oracles that check **stage transition coherence**, such as:
- definitions preserved or lawfully revised,
- commitments not contradicted without explanation,
- transformations between stage semantic sets are recorded.

If used, these should emit structured violation lists that can be audited and waived only via allowed portals.


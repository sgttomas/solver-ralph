---
doc_id: SR-AGENT-WORKER-CONTRACT
doc_kind: governance.agent_worker_contract
layer: build
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

# SR-AGENT-WORKER-CONTRACT — Worker behavior for Semantic Ralph Loops

**Purpose:** Define the minimum behavioral contract for an agent worker (e.g., Claude Code) operating inside SOLVER-Ralph, especially for **Semantic Ralph Loops**.

This contract defines what the worker must do per iteration so the system remains deterministic, auditable, and stage-gated.

---

## 1. Inputs the worker receives (per iteration)

The worker MUST be invoked with (directly or via refs):

- the active SR-* governed docs required by SR-DIRECTIVE,
- a projection artifact that includes the **eligible set** and status snapshot (non-authoritative),
- the selected work unit’s **Work Surface** (intake + procedure template + stage_id + oracle suites),
- any prior candidate reference (if continuing work).

---

## 2. Worker responsibilities (normative)

### 2.1 Choose one eligible target

- The worker MUST choose exactly one eligible work unit (and one stage target) per iteration.
- The worker MUST record why it chose that target (non-binding rationale) in the iteration summary artifact.

### 2.2 Execute the procedure stage

- The worker MUST follow the Procedure Template stage steps in order.
- The worker MUST produce all required stage outputs (candidate artifacts and intermediate artifacts) or explicitly stop with a stop trigger.

### 2.3 Run required oracle suites

- The worker (or the system on its behalf) MUST run all required oracle suites for the stage, including semantic suites.
- Oracle outputs MUST be captured as content-addressed artifacts.

### 2.4 Produce commitment objects

The worker MUST ensure the following artifacts exist (paths are conventional):

- **Context Artifact**: a machine-readable record of all inputs used (refs + hashes), the work surface binding, and the stage_id.
- **Candidate Manifest** (if candidate is not a single VCS commit): list of artifacts with hashes.
- **Evidence Bundle** (`evidence.gate_packet`) binding results to (`candidate_id`, `procedure_template_id`, `stage_id`).

### 2.5 No ghost inputs

The worker MUST NOT rely on information not represented in the iteration context refs. If additional context is needed, it must be added as a referenced artifact in a subsequent iteration.

---

## 3. Stop conditions (normative)

The worker MUST stop and emit a stop-trigger outcome when:

- required oracle suite cannot be run,
- required stage inputs are missing (work surface incomplete),
- repeated failures indicate thrashing,
- budget is exhausted,
- no eligible work units exist.

Stop triggers must be routed per SR-DIRECTIVE.

---

## 4. Output discipline (normative)

The worker MUST output:

- candidate artifacts (or patches) for the chosen work unit,
- oracle outputs and evidence packet,
- an iteration summary including selection rationale and next step recommendation.

The worker MUST NOT claim “complete” unless the platform completion predicate would be satisfied by the recorded evidence.


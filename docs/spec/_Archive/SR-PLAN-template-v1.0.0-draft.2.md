---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-PLAN"
  type: "governance.plan"
  title: "SR-PLAN — Planning Template (SOLVER-Ralph)"
  version: "1.0.0-draft.2"
  status: "draft"
  normative_status: "normative"
  authority_kind: "process"
  governed_by: ["SR-CHANGE"]
  supersedes: ['SR-PLAN@1.0.0-draft.1', 'SR-PLAN-TEMPLATE@1.0.0-draft.1']
  created: "2026-01-11"
  updated: "2026-01-11"
  tags: ["plan", "packages", "deliverables", "decomposition"]
  refs:
    - kind: "GovernedArtifact"
      id: "SR-PARADIGM"
      rel: "depends_on"
      meta:
        type_key: "governance.development_paradigm"
        version: "1.0.0-draft.8"
        content_hash: "sha256:27c4561bd883c72bba62453781abc898b5cd3f47b275810c394a9f6e5433abb1"
    - kind: "GovernedArtifact"
      id: "SR-TYPES"
      rel: "depends_on"
      meta:
        type_key: "governance.types"
        version: "3.3.0-draft.6"
        content_hash: "sha256:ba60d69e15be9535bee32dd432192e87ec625712c1c860b3fdedc0f2b0da319c"
    - kind: "GovernedArtifact"
      id: "SR-INTENT"
      rel: "depends_on"
      meta:
        type_key: "governance.intent"
        version: "1.1.0-draft.5"
        content_hash: "sha256:7940bafceb5dda19e70744ccfd58304027b10f0376186d3e87c3dcce79e85d35"
    - kind: "GovernedArtifact"
      id: "SR-CONTRACT"
      rel: "depends_on"
      meta:
        type_key: "governance.contract"
        version: "1.0.0"
        content_hash: "sha256:f5b42fa4e9e162e63fcdc33366499b98f6a5f1fd2ca9c1b0b5d597776d55eaef"
    - kind: "GovernedArtifact"
      id: "SR-SPEC"
      rel: "depends_on"
      meta:
        type_key: "governance.spec"
        version: "1.2.0"
        content_hash: "sha256:a3c45a21e88ea41f9915cbce456986e059f2a0d10e1af91f9f1908513fb59aec"
    - kind: "GovernedArtifact"
      id: "SR-DIRECTIVE"
      rel: "depends_on"
      meta:
        type_key: "governance.directive"
        version: "1.1.1"
        content_hash: "sha256:6ee1f4130d193e1fefd020f5eefa8628bb117cbec04b3d619fb1b879b3551eba"
    - kind: "GovernedArtifact"
      id: "SR-CHANGE"
      rel: "depends_on"
      meta:
        type_key: "governance.change"
        version: "2.0.0"
        content_hash: "sha256:c0f631225ac7f0c8f348c3c85b522d50bab3793cfcab7b531c8bef98e075d580"
    - kind: "GovernedArtifact"
      id: "SR-README"
      rel: "supported_by"
      meta:
        type_key: "governance.readme"
        version: "1.0.0"
        content_hash: "sha256:19caba82029848f6e8e74184060e117cdaff4229e986f4a5f55fcb672ffce2ec"
    - kind: "GovernedArtifact"
      id: "SR-ETT"
      rel: "supported_by"
      meta:
        type_key: "governance.trust_topology"
        version: "1.0.1"
        content_hash: "sha256:32844e0e71268b27586a00cdbb38d71cc4c868d2c31867c902a5ae02fc4a6b2b"
    - kind: "GovernedArtifact"
      id: "SR-AGENTS"
      rel: "supported_by"
      meta:
        type_key: "policy.agent_taxonomy"
        version: "1.0.0-draft.1"
        content_hash: "sha256:f2a303ad5fead512ed8250a3eee6db1c2de0b6851b302119024373594cd3ddd9"
  ext:
    plan:
      template: true
      scope:
        program_id: "SOLVER-Ralph"
      deliverable_output_classes: ["candidate", "service_capability", "governed_artifact", "record", "evidence"]
      dependency_edge_kinds: ["depends_on", "supported_by", "informs"]
      notes:
        - "SR-PLAN defines decomposition. SR-DIRECTIVE defines execution sequencing and state progression."
---

# SR-PLAN — Planning Template (SOLVER-Ralph) v1.0.0-draft.2

## 0. Change Log

- **v1.0.0-draft.2** (2026-01-11): Consolidates prior SR-PLAN drafts into a single template that:
  - cleanly separates **decomposition** (SR-PLAN) from **execution sequencing** (SR-DIRECTIVE),
  - requires **problem statement commitment objects** as authoritative planning inputs (no ghost meaning),
  - standardizes deliverable typing via **output_class** + **type_key when applicable**,
  - preserves the invariant: **SR-PLAN remains abstract** (no global “done” criteria, no schedule).

---

## 1. Purpose

SR-PLAN defines how a typed, aligned problem statement is decomposed into:

- **Packages**: groupings that bias concurrency (often by output type or architectural layer).
- **Deliverables**: loop-completable work units; each deliverable is intended to map 1:1 to a Ralph Loop `work_unit`.

SR-PLAN is designed to be usable by **humans and agents** as a stable “decomposition substrate” for planning work that will later be executed under SR-DIRECTIVE and the runtime specified by SR-SPEC.

---

## 2. Scope and Non-Scope

### 2.1 In scope

SR-PLAN specifies:

- required fields and structure for plans,
- required semantics of packages and deliverables,
- constraints on “what a plan may claim” (and what it must not claim),
- how plans relate to committed problem statements (`record.problem_statement`) and governed artifacts.

### 2.2 Out of scope

SR-PLAN does **not**:

- prescribe the linear or concurrent **execution sequence** of deliverables (owned by SR-DIRECTIVE and runtime scheduling),
- define runtime events, endpoints, or state machines (owned by SR-SPEC),
- redefine Verified / Approved / Shippable or Portal semantics (owned by SR-CONTRACT + SR-SPEC + SR-DIRECTIVE),
- enumerate the canonical governed document set (owned by SR-README + SR-TYPES).

---

## 3. Planning as Deterministic Rules over a Non-Deterministic Path

A plan defines a **state space of deliverables** and their dependency constraints.

- The **validity** of a plan is deterministic (it can be checked).
- The **path** taken through the plan is actor-driven and non-deterministic (humans decide; agents propose; oracles succeed/fail).
- Replayability comes from recording **commitment objects** (problem statements, governed artifacts, events, evidence) rather than relying on narrative memory.

SR-PLAN MUST be written so that a future actor can reconstruct “what we planned to do” from the plan artifact alone, without relying on chat context.

---

## 4. Definitions

### 4.1 Package

A **Package** is a grouping construct used to bias concurrency. Packages are organizational: they do not imply ordering.

Packages SHOULD generally group deliverables by:
- output class (governance docs vs code candidates vs evidence),
- architectural layer (domain / adapters / UI),
- operational concern (auth, storage, orchestration),
- or another rationale explicitly stated in the package.

### 4.2 Deliverable

A **Deliverable** is the unit of planned work intended to be achievable as one Ralph Loop `work_unit`.

Deliverables SHOULD be:
- small enough that success probability is high,
- loop-completable: candidate + oracles/evidence + summary, with any required human decision at portals deferred to SR-DIRECTIVE-defined gates.

### 4.3 Output class

Each deliverable MUST declare an `output_class`:

- `candidate`: code/config changes delivered as a candidate snapshot (and later verified by oracle evidence).
- `service_capability`: an operational capability (e.g., a running service endpoint); usually realized via a candidate + deployment config.
- `governed_artifact`: a governed document change (subject to SR-CHANGE).
- `record`: a durable record artifact (e.g., `record.problem_statement`).
- `evidence`: an evidence artifact (e.g., a gate packet / evidence bundle manifest).

---

## 5. Normative Requirements

### 5.1 Authoritative planning inputs (no ghost meaning)

For instance-scoped plans, SR-PLAN MUST reference a committed **SAPS** problem statement:

- The authoritative problem statement MUST exist as `record.problem_statement` with `ext.problem_statement.stage = "SAPS"`.
- The plan MUST reference that record via typed refs.
- Re-paraphrasing or “restating” the problem statement in place of referencing the committed record is prohibited.

(You may include a brief orientation summary, but it MUST be clearly marked as **non-authoritative** and MUST NOT replace the committed record.)

### 5.2 One loop per deliverable (planning granularity)

A plan MUST be decomposed so that:
- **each deliverable is intended to map 1:1 to a Ralph Loop `work_unit`**, and
- packages group deliverables for concurrency, not for atomicity.

### 5.3 Deliverables must be loop-completable

Each deliverable MUST include:
- acceptance criteria (oracle-verifiable when feasible),
- expected evidence outputs (what evidence exists when the deliverable is “done”),
- required inputs (`refs_required[]`), at least including relevant governed artifacts,
- dependency edges (`depends_on[]`) to other deliverables when blocking.

### 5.4 Deliverable typing discipline

- If `output_class ∈ {governed_artifact, record, evidence}`, the deliverable MUST declare `primary_output.type_key` from SR-TYPES.
- If `output_class ∈ {candidate, service_capability}`, `primary_output.type_key` MAY be omitted; the deliverable MUST still specify the *intended* artifact(s) or capability in concrete terms (paths, modules, services, endpoints).

### 5.5 Plan must remain abstract (no execution schedule, no closure semantics)

A plan MUST NOT:
- specify a global execution sequence (“Phase 1 then Phase 2”) as binding instruction,
- encode budgets, retries, or stop conditions as binding instructions,
- define “resolution achieved” / “system done” criteria.

Those belong to SR-DIRECTIVE and runtime state.

A plan MAY include non-binding *suggestions* (clearly marked as such) to help SR-DIRECTIVE authoring.

### 5.6 Change control

- SR-PLAN is governed by SR-CHANGE.
- Any change that alters planned deliverables, dependencies, or acceptance criteria MUST be made as a new plan version that supersedes the prior plan (or as a governed change record that produces a superseding plan).
- If a plan conflicts with SR-CONTRACT or SR-SPEC, the correct response is to route via SR-CHANGE (not to “interpret around” the conflict).

---

## 6. Minimum Template Schema

### 6.1 Plan header

A plan MUST declare:
- scope (program/project/instance),
- whether it is a template (`ext.plan.template: true|false`),
- for instance plans: the `record.problem_statement` (SAPS) ref.

### 6.2 Package schema (minimum)

Each package MUST include:
- `package_id`
- `title`
- `grouping_rationale` (why this grouping exists)
- `deliverables[]`

### 6.3 Deliverable schema (minimum binding fields)

Each deliverable MUST include:

- `deliverable_id`
- `title`
- `output_class` (enum)
- `primary_output`:
  - `description` (always)
  - `type_key` (required when output_class is governed_artifact/record/evidence)
- `refs_required[]` (typed refs; at minimum, relevant governing artifacts)
- `depends_on[]` (deliverable ids)
- `acceptance_criteria[]`
- `expected_evidence[]`

Optional (recommended):
- `recommended_verification_profile` (non-binding; see SR-DIRECTIVE)
- `portal_touchpoints[]` (non-binding; where human authority is expected)
- `risks[]` / `open_questions[]`

---

## 7. Anti-Patterns

- **Plan-as-schedule:** embedding execution sequencing and budgets as binding text.
- **Ghost meaning:** planning from chat summaries rather than committed `record.problem_statement`.
- **Untyped outputs:** deliverables that do not state what artifact or capability they produce.
- **Evidence-free planning:** deliverables without acceptance criteria and expected evidence.
- **Dependency ambiguity:** missing `depends_on` edges leading to hidden ordering constraints.
- **Directive leakage:** redefining portal rules, Verified/Approved/Shippable semantics, or runtime events.

---

## Appendix A. Instance Plan Skeleton

> This appendix is a copy/paste scaffold. Replace bracketed placeholders.

```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-PLAN-{INSTANCE}"
  type: "governance.plan"
  title: "{Instance} Plan"
  version: "{X.Y.Z}"
  status: "draft|governed"
  normative_status: "normative"
  authority_kind: "process"
  governed_by: ["SR-CHANGE"]
  supersedes: []
  created: "YYYY-MM-DD"
  updated: "YYYY-MM-DD"
  refs:
    - kind: "Record"
      id: "{PS-...}"
      rel: "depends_on"
      meta:
        type_key: "record.problem_statement"
        stage: "SAPS"
        version: "{X.Y.Z}"
        content_hash: "sha256:..."
  ext:
    plan:
      template: false
      template_ref: "SR-PLAN@{X.Y.Z}"
      scope:
        instance_id: "{Instance}"
      input_problem_statement_ref:
        kind: "Record"
        id: "{PS-...}"
        rel: "depends_on"
        meta:
          type_key: "record.problem_statement"
          stage: "SAPS"
          version: "{X.Y.Z}"
          content_hash: "sha256:..."
---

# {Instance} Plan v{X.Y.Z}

## 1. Planning Inputs (authoritative)
- {PS-... SAPS ref}
- {governance docs refs}

## 2. Architecture & Tech Stack Assessment (constraints)
- ...

## 3. Decomposition Strategy
- ...

## 4. Packages Overview
| Package | Purpose | Deliverables |
|---|---|---:|
| PKG-... | ... | N |

## 5. Packages and Deliverables

### PKG-...
**Grouping rationale:** ...
| Deliverable | Output class | Depends on |
|---|---|---|
| D-... | candidate | D-... |

#### D-... — {title}
- **Output class:** ...
- **Primary output:** ...
- **Refs required:** ...
- **Depends on:** ...
- **Acceptance criteria:** ...
- **Expected evidence:** ...
```

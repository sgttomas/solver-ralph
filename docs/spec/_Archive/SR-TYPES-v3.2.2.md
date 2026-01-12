---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-TYPES"
  type: "governance.types"
  title: "SOLVER-Ralph Types"
  version: 3.2.2
  status: "draft"
  normative_status: "normative"
  authority_kind: "process"
  governed_by:
    - "SR-CHANGE"   # see 6_SOLVER-Change-Management-*.md
  supersedes:
    - "SR-TYPES@3.2.1"
    - "SR-TYPES@3.0.0"
    - "0_Document-Type-Specifications-v2.1.1.md"
  created: "2026-01-09"
  updated: '2026-01-10'
  tags: ["governance", "types", "ontology", "metadata"]
---

# SOLVER-Ralph Types v3.2.1

**Purpose:** Define the **artifact type system** for SOLVER-Ralph: what kinds of knowledge artifacts exist, how they relate, what authority they carry, and the minimal metadata required to make them machine-indexable.

**Status:** Draft (proposed update replacing `0_Document-Type-Specifications-*`).

---

## 0. How to Use This Document

This document is written for three audiences:

- **Humans** writing or reviewing governed artifacts (what to write, where it goes, how it’s evaluated).
- **Agents** generating artifacts (how to classify “what I’m about to write” and what constraints apply).
- **Tooling** (future): graph ingestion, semantic retrieval, baseline verification.

**If you are creating or modifying any governed artifact, start here** to confirm:
1) the artifact’s type, 2) the authority and binding rules, and 3) the required metadata.

---

## 1. Foundations

This section is the “type theory” underpinning the rest of SOLVER-Ralph.

### 1.1 Ontology: What Exists Here

**Actors**
- **Human**: the only actor that can grant binding approvals at trust boundaries (portals/gates).
- **Agent**: proposes, drafts, iterates; cannot self-approve boundary-crossing actions.
- **System**: orchestrates, enforces rules, records evidence.

**Artifacts**
- **Policy artifacts**: define rules (normative constraints and/or process constraints).
- **Record artifacts**: bind state or decisions (“what is true *now* in this governance context”).
- **Evidence artifacts**: outputs of evaluation (tests, scans, checklists, logs).
- **Work product artifacts**: code, schemas, configuration, documentation outside governance.
- **Implementation configuration artifacts**: agent prompts, CI wiring, tool harness settings.

**Boundaries**
- **Trust boundary**: agents cannot perform or approve certain actions; humans must approve.
- **Oracle boundary**: what can be deterministically evaluated vs what requires human arbitration.
- **Scope boundary**: what belongs in which artifact type (prevents category errors).

### 1.2 Epistemology: How “Knowledge” Is Established

SOLVER-Ralph treats correctness as **constructed and bounded**:

- **Verified** means: required **oracles** (deterministic evaluators) have passed for the scope claimed.
- **Approved** means: a **human** has reviewed the relevant evidence and accepted the outcome at a portal/gate.
- **Shippable** means: verified + approved + no unresolved blocking deviations (as defined by governance).

**Important:** Oracle pass establishes *verification*, not metaphysical correctness. Humans remain the final arbiters at boundaries.

### 1.3 Semantics: Words With Hard Meanings

These terms are used consistently across artifacts:

- **Intent (Meaning)**: what the human wants (may not be perfectly captured).
- **Intent Artifact**: the Design Intent document (a versioned approximation of intent).
- **Oracle**: a deterministic evaluator that yields pass/fail (tests, linters, scans, scripted checks).
- **Portal**: a required human review point for outcomes not fully oracle-verifiable (or where risk demands it).
- **Gate**: a named checkpoint with explicit criteria (some oracle-verifiable, some portal-based).
- **Deviation**: a binding exception from a governed requirement, valid until resolved.
- **Deferral**: an intentional postponement of a requirement or deliverable (still tracked; not forgotten).
- **Decision**: a binding human judgment that resolves an ambiguity or tradeoff (often creates precedent).

- **Semantic dependency vs audit provenance**: SOLVER-Ralph distinguishes relationships that *block by default* (semantic dependencies) from those that are *audit-only* (audit provenance).
- **Reference relationship `rel` (SR-SPEC glossary)**: in event `refs[]`, `rel=depends_on` denotes *semantic dependency* (eligible for staleness traversal and blocking by default), while `rel=supported_by` denotes *audit provenance* (non-blocking by default). The binding semantics live in **SR-SPEC §1.5.3.2** and **SR-CONTRACT C-EVT-5/C-EVT-6**; SR-TYPES records this mapping to prevent vocabulary drift.

---

## 2. Authority Model

### 2.1 Authority Kinds

Artifacts can be authoritative in different ways. We separate **what kind of authority** an artifact has from **whether it is normative**.

- **Content authority**: defines what the system must be (requirements, invariants, interfaces).
- **Process authority**: defines how work and change are governed (freeze rules, change control).
- **Record authority**: defines binding state/commitments (baseline manifests, deviations, decisions).
- **Configuration authority**: affects behavior via tooling, but is governed indirectly.

### 2.2 Normative Status Classes

We use a small set of normative-status classes. They are *typed*, so agents can reason about them.

- **Normative**: contains binding MUST/SHALL requirements.
- **Directional**: guidance, rationale, heuristics; not binding, but consultative.
- **Index**: pointers and navigation; not binding; consultative.
- **Record**: binding record of a decision/state/exception (not a requirement doc).
- **Evidence**: outputs of verification and review (used to justify “verified/approved”).

### 2.3 Purpose Priority, Binding Precedence, Override Rule

These rules apply when documents disagree.

**Purpose Priority** (what exists to serve what):
1. **Design Intent (Why²)** — why the system exists; problem framing and rationale.
2. **Architectural Contract (Why)** — invariants and safety contracts.
3. **Technical Specification (What)** — implementation-ready requirements.
4. **Development Directive (How)** — plan, phases, gates, sequencing.
5. **Project Orientation (README)** — navigation and onboarding.

**Binding Precedence** (what wins in conflicts):
1. **Architectural Contract**
2. **Technical Specification**
3. **Development Directive**
4. **Design Intent**
5. **README**

**Override Rule:** if Design Intent conflicts with Contract/Spec, you **do not “override”**; you open a change request and reconcile explicitly.

### 2.4 The Must-Consult Rule

Non-normative does **not** mean ignorable.

- **Directional** artifacts may contain the reasoning needed to interpret binding requirements.
- **Index** artifacts may contain the only reliable navigation to binding content.

Agents and humans **must consult** relevant directional/index artifacts when they are referenced.

---

## 3. Metadata Standard

### 3.1 Why Metadata Exists

Metadata is required to support:
- deterministic retrieval (“find the active contract that governs X”),
- graph ingestion (“what depends on what?”),
- freeze baselining (“what versions were in force?”),
- auditability (“who decided what, when?”).

### 3.2 YAML Frontmatter Is the Canonical Metadata Carrier

All governance-relevant markdown artifacts MUST begin with YAML frontmatter:

```yaml
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-<STABLE-ID>"
  type: "<artifact-type>"
  title: "<human title>"
  version: "<semver | date | unversioned>"
  status: "draft | governed | superseded | deprecated | archived"
  normative_status: "normative | directional | index | record | evidence"
  authority_kind: "content | process | record | config"
  governed_by: ["SR-CHANGE"]
  supersedes: ["<prior-artifact-ref>"]
  created: "YYYY-MM-DD"
  updated: "YYYY-MM-DD"
  tags: ["..."]
---
```

**Core required fields (minimum viable schema):**
- `schema`, `id`, `type`, `version`, `status`

Everything else is strongly recommended for governance artifacts.

### 3.3 Core + Extensions (Chosen Standard)

We adopt **core required fields + type-specific extensions**.

Rationale:
- It keeps baseline authoring lightweight.
- It allows the type system to evolve without forcing every artifact to carry irrelevant fields.
- It supports later emergence of a more formal schema once patterns stabilize.

**Rule:** type-specific fields MUST be placed under:

```yaml
solver_ralph:
  ext:
    <type_key>:
      ...
```

This prevents key collisions and makes ingestion consistent.

### 3.4 Migration Note

Legacy governed documents that used the old “Governance Placard” block may be grandfathered temporarily, but:
- new artifacts MUST use YAML frontmatter,
- the next major update of any canonical governed artifact SHOULD add frontmatter.

---

## 4. The Type Registry

This registry defines what artifact types exist *in scope* for SOLVER-Ralph governance.

### 4.1 Canonical Governed Set

These are the baseline, governed documents (0–6 plus SR-ETT plus root README).

| Type Key | Canonical? | Authority Kind | Normative Status | Primary Question |
|---|---:|---|---|---|
| `governance.types` | Yes | process | normative | “What artifact types exist and how do they relate?” |
| `governance.readme` | Yes | index | index | “How do I orient and navigate the project?” |
| `governance.design_intent` | Yes | content | directional | “Why is the architecture this way?” |
| `governance.arch_contract` | Yes | content | normative | “What must always be true for correctness?” |
| `governance.technical_spec` | Yes | content | normative | “What exactly are we building?” |
| `governance.dev_directive` | Yes | process/content | mixed | “How do we build and verify it?” |
| `governance.change_mgmt` | Yes | process | normative | “How do we change the governed set?” |
| `governance.epistemic_trust_topology` | Yes | process | directional | “How do the harness membranes produce epistemic trust, and where do coercion boundaries belong?” |

### 4.2 Baseline Output Artifacts

These artifacts are produced by governance processes (especially freeze). They are **not** members of the canonical governed set, but they are required for establishing and auditing a baseline.

| Type Key | Canonical? | Authority Kind | Normative Status | Notes |
|---|---:|---|---|---|
| `record.freeze` | No | record | record | Snapshot manifest: hashes of canonical set + lists of active deferrals & deviations + gate evidence pointers. |
| `evidence.gate_packet` | No | evidence | evidence | Collected outputs demonstrating gate criteria satisfied (may be embedded in Freeze Record or referenced externally). |

### 4.3 Adjacent Governance Artifacts

These artifacts are governance-adjacent: versioned, referenced, and useful, but **not baseline members** (for now).

| Type Key | Canonical? | Authority Kind | Normative Status | Minimal Description |
|---|---:|---|---|---|
| `policy.decision_heuristic` | No | process | directional | How decisions are made/routed/escalated in the SOLVER paradigm. |
| `policy.oracle_integrity` | No | process | normative (local) | Oracle integrity policy: determinism requirements, evidence handling, flake handling. |
| `record.decision` | No | record | record | Binding decision record resolving ambiguity/tradeoff; may create precedent. |
| `record.intervention_note` | No | record | record | Non-binding human intervention input intended to be included in a subsequent `IterationStarted.refs[]` (does not create portal approval semantics). |
| `record.evaluation_note` | No | record | record | Non-binding human evaluation of verification evidence (or explicitly “evidence missing”). |
| `record.assessment_note` | No | record | record | Non-binding human assessment of validation evidence in context (or explicitly “evidence missing”). |
| `record.deviation` | No | record | record | Binding exception from a governed requirement; valid until resolved. |
| `record.deferral` | No | record | record | Binding postponement; must remain visible (esp. at freeze). |
| `index.quick_reference` | No | index | index | Fast lookup for key rules/thresholds; no new authority. |

**Note:** these are versioned, but change control is currently managed manually by the maintainer. Once stable, some may become canonical.

#### 4.3.1 `record.intervention_note` (non-binding record)

A `record.intervention_note` captures a **human intervention** intended to influence work **without** creating new authority (no Portal approvals, no “Verified” claims).
It is included as a **typed reference** in the next iteration’s `IterationStarted.refs[]`.

**Required metadata (YAML frontmatter):**
- `solver_ralph.id`: stable id for this record artifact
- `solver_ralph.type`: `record.intervention_note`
- `solver_ralph.version`: SemVer
- `solver_ralph.normative_status`: `record`
- `solver_ralph.authority_kind`: `record`

**Required extension shape (under `ext.intervention`):**
- `scope` (string): e.g., `{ loop_id | work_unit | package }`
- `intent` (string): short description of what the intervention is trying to change/avoid
- `instruction` (string): actionable guidance for the next iteration

**Optional fields (under `ext.intervention`):**
- `applies_to` (string): default `next_iteration_only`
- `refs[]` (array): optional typed refs to related artifacts/evidence/decisions (for audit navigation)

**Prohibitions (normative):**
- MUST NOT be represented as a Portal approval.
- MUST NOT be described as “Verified” or evidence-backed unless an Evidence Bundle exists and is referenced separately.
- If the intervention changes binding governance (SR-CONTRACT/SR-SPEC/SR-DIRECTIVE/SR-CHANGE), it MUST be processed via SR-CHANGE.

**SR-SPEC ref mapping (normative):**
- When referenced from events, use `ref.kind=Record` with `meta.type_key=record.intervention_note`.
- `meta.content_hash` is REQUIRED for dereferenceable replay of what was used.




#### 4.3.2 `record.evaluation_note` (non-binding record)

A `record.evaluation_note` captures **human evaluation** of **verification evidence** (e.g., oracle results). It is a **non-binding** record that may be referenced as an input to the next iteration without creating Portal approval semantics.

**Required metadata (YAML frontmatter):**
- `solver_ralph.id`: stable id for this record artifact
- `solver_ralph.type`: `record.evaluation_note`
- `solver_ralph.version`: SemVer
- `solver_ralph.normative_status`: `record`
- `solver_ralph.authority_kind`: `record`

**Required extension shape (under `ext.evaluation`):**
- `scope` (string): e.g., `{ loop_id | work_unit }`
- `evidence_refs[]` (array): typed refs to Evidence Bundle(s) reviewed (**MAY be empty** to record “evidence missing”)
- `judgment` (string): the human conclusion
- `recommendation` (enum): `proceed | iterate | hold | escalate`
- `evidence_status` (enum, optional): `present | missing | partial`

**Prohibitions (normative):**
- MUST NOT be represented as a Portal approval.
- MUST NOT be described as “Verified.”
- MUST NOT be treated as binding authorization.

**SR-SPEC ref mapping (normative):**
- When referenced from events, use `ref.kind=Record` with `meta.type_key=record.evaluation_note`.
- `meta.content_hash` is REQUIRED for dereferenceable replay of what was used.


#### 4.3.3 `record.assessment_note` (non-binding record)

A `record.assessment_note` captures **human assessment** of **validation evidence** in context (intent, risk, user impact). It is a **non-binding** record that may be referenced as an input to the next iteration without creating Portal approval semantics.

**Required metadata (YAML frontmatter):**
- `solver_ralph.id`: stable id for this record artifact
- `solver_ralph.type`: `record.assessment_note`
- `solver_ralph.version`: SemVer
- `solver_ralph.normative_status`: `record`
- `solver_ralph.authority_kind`: `record`

**Required extension shape (under `ext.assessment`):**
- `scope` (string): e.g., `{ loop_id | work_unit }`
- `evidence_refs[]` (array): typed refs to Evidence Bundle(s) reviewed (**MAY be empty** to record “evidence missing”)
- `judgment` (string): the human conclusion
- `recommendation` (enum): `proceed | iterate | hold | escalate`
- `evidence_status` (enum, optional): `present | missing | partial`

**Prohibitions (normative):**
- MUST NOT be represented as a Portal approval.
- MUST NOT be described as “Verified.”
- MUST NOT be treated as binding authorization.

**SR-SPEC ref mapping (normative):**
- When referenced from events, use `ref.kind=Record` with `meta.type_key=record.assessment_note`.
- `meta.content_hash` is REQUIRED for dereferenceable replay of what was used.

### 4.4 Implementation Configuration Artifacts

These artifacts affect behavior but are governed indirectly.

| Type Key | Canonical? | Authority Kind | Normative Status | Examples |
|---|---:|---|---|---|
| `config.agent_definition` | No | config | directional | Change Manager / Freeze Steward / Co-Developer prompts |
| `config.gating_policy` | No | config | directional | work-unit gating mode + overrides + deterministic triggers for hook escalation |
| `config.ci_policy` | No | config | directional | build targets, hooks, CI scripts enforcing oracles |

#### 4.4.1 `config.agent_definition` (directional config artifact)

A `config.agent_definition` artifact is a **directional** configuration record that defines how a sandboxed Agent Worker is expected to behave **without creating new authority**.

**Intended use:**
- referenced from `IterationStarted.refs[]` to make the worker configuration auditable,
- used by worker runtimes/harnesses to select tools, templates, and context profiles,
- treated as **audit provenance** by default (`rel=supported_by`), not as a staleness-propagating dependency.

**Required metadata (YAML frontmatter):**
- `solver_ralph.id`: stable id for the agent definition artifact
- `solver_ralph.type`: `config.agent_definition`
- `solver_ralph.version`: SemVer
- `solver_ralph.normative_status`: `directional`
- `solver_ralph.authority_kind`: `config`

**Required extension shape (under `ext.agent_definition`):**
- `agent_role_id` (stable role identifier; e.g., `ralph.co_developer`)
- `capability_profile_id` (stable capability profile identifier)
- `context_profile_id` *or* `context_profile_rules` (how the worker selects which ref slices to load)
- `tool_access_profile_id` (declared tool access toggles/limits)
- `template_catalog_ref` (versioned reference to prompt/template catalog, if used)

**Stability expectations:**
- `agent_role_id` and `capability_profile_id` SHOULD remain stable across versions.
- Variations SHOULD be expressed as versioned overlays (e.g., tool access changes, template catalog bumps), not by rewriting role identity.




#### 4.4.2 `config.gating_policy` (directional config artifact)

A `config.gating_policy` artifact defines **human-selectable** gating behavior for a **work unit**, with optional overrides. It provides deterministic “hooks” (stop conditions) without introducing new Portal semantics.

**Intended use:**
- referenced from `IterationStarted.refs[]` so the gating policy in force is auditable,
- used by SYSTEM to decide whether missing human judgments are **soft** (record as pending) or **hard** (block progression) for specific hook classes,
- treated as **audit provenance** by default (`rel=supported_by`), not as a staleness-propagating dependency.

**Required extension shape (under `ext.gating_policy`):**
- `scope.work_unit` (string): stable identifier for the work unit the policy applies to
- `default_mode` (enum): `soft | hard | hybrid`
- `overrides` (object, optional): per-hook-class overrides; keys MAY include:
  - `plan_review`
  - `evaluation_on_verification`
  - `assessment_on_validation`
  - `closeout`
  Each value: `{ mode: soft|hard|hybrid, hard_triggers?: string[] }`
- `hard_triggers` (string[], optional): triggers that escalate `hybrid` to `hard` (e.g., `EXCEPTIONS_ACTIVE`, `OPEN_RISK_HIGH`, `REPEATED_FAILURE`, `GOVERNANCE_TOUCH`, `CLOSEOUT_PENDING`)

**Prohibitions (normative):**
- MUST NOT be represented as a Portal approval.
- MUST NOT redefine Verified/Approved semantics.
- MUST NOT introduce new lifecycle states.

---

## 5. Canonical Document Type Specifications

This section provides the scope boundaries and templates for the canonical governed set.

### 5.1 Project Orientation README

**Purpose:** Provide onboarding, navigation, and “how to use the governed set.”

**Normative status:** **Index** (consultative, not binding requirements).

**Quality criteria:**
- [ ] Has a document index with current versions
- [ ] Explains “when to consult which document”
- [ ] Links to change management + freeze record
- [ ] Does not introduce new MUST-level requirements

**Template:**
```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-README"
  type: "governance.readme"
  title: "{Project} README"
  version: "unversioned"
  status: "governed"
  normative_status: "index"
  authority_kind: "index"
  governed_by: ["SR-CHANGE"]
  created: "YYYY-MM-DD"
  updated: "YYYY-MM-DD"
---

# {Project}

## What is {Project}?

## Project Documentation Structure

## The Document Index

## For Different Audiences

## Document Maintenance

## Getting Started

## Contributing
```

---

### 5.2 Design Intent

**Purpose:** Capture the reasoning: “why must the architecture be this way?”

**Normative status:** **Directional** (non-binding, but must-consult when referenced).

**Scope boundaries**
- Belongs: first principles, threat model, failure modes, rationale, rejected alternatives, assumptions.
- Does NOT belong: MUST requirements (Contract), schemas/endpoints (Spec), sequencing (Directive).

**Template (outline):**
```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-INTENT"
  type: "governance.design_intent"
  title: "{Project} Design Intent"
  version: "{X.Y.Z}"
  status: "governed"
  normative_status: "directional"
  authority_kind: "content"
  governed_by: ["SR-CHANGE"]
  created: "YYYY-MM-DD"
  updated: "YYYY-MM-DD"
  ext:
    design_intent:
      assumptions:
        - id: "A-001"
          statement: "..."
          trigger: "What would cause re-evaluation?"
---

# {Project} Design Intent v{X.Y.Z}

## 0. Version Changes

## 1. Problem Statement

## 2. First Principles

## 3. Threat / Trust Model

## 4. Design Rationale

## 5. Rejected Alternatives

## 6. Assumptions & Triggers
```

---

### 5.3 Architectural Contract

**Purpose:** Define invariants, safety contracts, and MUST/SHALL constraints.

**Normative status:** **Normative** (highest binding precedence).

**Scope boundaries**
- Belongs: invariants, correctness conditions, contract checklists, trust boundaries.
- Does NOT belong: implementation detail (Spec), sequencing (Directive), extended rationale (Intent).

**Verification rule (high assurance):**
Every Contract MUST SHOULD be either:
- **oracle-verifiable**, or
- explicitly tagged as **portal-reviewed** (human arbitration required).

**Template (outline):**
```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-CONTRACT"
  type: "governance.arch_contract"
  title: "{Project} Architectural Contract"
  version: "{X.Y.Z}"
  status: "governed"
  normative_status: "normative"
  authority_kind: "content"
  governed_by: ["SR-CHANGE"]
  created: "YYYY-MM-DD"
  updated: "YYYY-MM-DD"
  ext:
    arch_contract:
      verification_methods: ["oracle", "portal"]
---

# {Project} Architectural Contract v{X.Y.Z}

## 0. Version Changes

## 1. Architecture Overview

## 2. Trust Boundaries

## 3. Core Invariants

## 4. Contracts (MUST/SHALL)

## 5. Conformance Checklists
```

---

### 5.4 Technical Specification

**Purpose:** Implementation-ready details: schemas, APIs, state machines, code patterns.

**Normative status:** **Normative** (must satisfy Contract; wins conflicts with Directive).

**Template:** (identical to prior v2.1.1 template, with frontmatter added)
```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-SPEC"
  type: "governance.technical_spec"
  title: "{Project} Technical Specification"
  version: "{X.Y.Z}"
  status: "governed"
  normative_status: "normative"
  authority_kind: "content"
  governed_by: ["SR-CHANGE"]
  created: "YYYY-MM-DD"
  updated: "YYYY-MM-DD"
---

# {Project} Technical Specification v{X.Y.Z}

**Purpose:** Implementation-ready specification.

**Normative Status:** Normative. Must satisfy Architectural Contract.

**Satisfies Contracts:** {List of Contract sections this Spec implements}

## 0. Version Changes

## 1. Data Model

## 2. API Specification

## 3. State Machines

## 4. Code Patterns

## 5. Configuration
```

---

### 5.5 Development Directive

**Purpose:** Execution plan: phases, packages, gates, dependencies.

**Normative status:** **Mixed**
- Gate definitions are **binding**.
- Package sequencing is strong guidance; may change if gates remain satisfied.

**Template:** (identical to prior v2.1.1 template, with frontmatter added)
```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-DIRECTIVE"
  type: "governance.dev_directive"
  title: "{Project} Development Directive"
  version: "{X.Y.Z}"
  status: "governed"
  normative_status: "normative"
  authority_kind: "process"
  governed_by: ["SR-CHANGE"]
  created: "YYYY-MM-DD"
  updated: "YYYY-MM-DD"
---

# {Project} Development Directive v{X.Y.Z}

**Purpose:** Gated execution plan.

## 0. Version Changes

## 1. Phases Overview

## 2. Gates and Criteria

## 3. Package Breakdown

## 4. Dependency Graph

## 5. Execution Checklist
```

---

### 5.6 Change Management

**Purpose:** Define change categories, freeze rules, and governance procedures.

**Normative status:** **Normative** (process authority).

**Template:** (identical to prior v2.1.1 template, with frontmatter added)
```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-CHANGE"
  type: "governance.change_mgmt"
  title: "{Project} Change Management"
  version: "{X.Y.Z}"
  status: "governed"
  normative_status: "normative"
  authority_kind: "process"
  governed_by: ["SR-CHANGE"]
  created: "YYYY-MM-DD"
  updated: "YYYY-MM-DD"
---

# {Project} Change Management v{X.Y.Z}

## 0. Version Changes

## 1. Change Categories

## 2. Freeze Policy

## 3. Change Control Procedure

## 4. Governance Roles
```

### 5.7 Epistemic Trust Topology

**Purpose:** Define the nine harness membranes that convert fluid proposals into durable, attributable commitments, treating “trust” as an emergent property of the combined membranes.

**Normative status:** Directional (consultative; does not redefine binding semantics).

**Authority kind:** Process.

**Template (minimum):**
```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-ETT"
  type: "governance.epistemic_trust_topology"
  title: "{Project} Epistemic Trust Topology"
  version: "{X.Y.Z}"
  status: "governed"
  normative_status: "directional"
  authority_kind: "process"
  governed_by: ["SR-CHANGE"]
  created: "YYYY-MM-DD"
  updated: "YYYY-MM-DD"
  tags: ["governance", "policy", "epistemology", "trust", "topology"]
  ext:
    harness_members: []   # list of the nine membranes (names + modality)
    commitment_objects: []
    assumptions: []
---

# {Project} Epistemic Trust Topology v{X.Y.Z}

## 1. Core idea

## 2. Definitions

## 3. Design principle: harness without strangling

## 4. The nine membranes (harnesses)

## 5. Membrane placement heuristics

## 6. How to use this document

## 7. Normative section (minimal)

## 8. References (informative)

## 9. Change control
```


---

## 6. Baseline Output Artifact Specifications

### 6.1 Freeze Record

**Purpose:** Provide an auditable, deterministic baseline snapshot:
- hashes of the canonical governed set,
- active **deferrals** *and* active **deviations** at baseline,
- pointers to gate evidence.

**Normative status:** Record (binding snapshot).

**Template (minimum):**
```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-FREEZE-{BASELINE_ID}"
  type: "record.freeze"
  title: "{Project} Specification Freeze Record"
  version: "{BASELINE_ID}"
  status: "governed"
  normative_status: "record"
  authority_kind: "record"
  governed_by: ["SR-CHANGE"]
  created: "YYYY-MM-DD"
  updated: "YYYY-MM-DD"
  ext:
    freeze:
      baseline_id: "{baseline-id}"
      previous_baseline: "{baseline-id}"
      freeze_status: "FROZEN|UNFROZEN"
---

# {Project} Specification Freeze Record

**Freeze Status:** {FROZEN/UNFROZEN}
**Freeze Date:** YYYY-MM-DD
**Baseline Identifier:** `{baseline-id}`
**Previous Baseline:** `{baseline-id}` (YYYY-MM-DD)

## Document Manifest
| Document | Version | SHA-256 |
|---|---|---|

## Deferrals Active at This Baseline
- ...

## Deviations Active at This Baseline
- ...

## Gate Evidence
- ...
```

---

## 7. Adjacent Governance Artifact Notes

These artifacts are intentionally **not** baseline members, but they must remain visible, referenceable, and versioned.

### 7.1 Decision Records

A Decision Record MUST:
- have a stable ID,
- state the question, options considered, decision, and rationale,
- reference affected governed requirements (Contract/Spec/Directive),
- be treated as binding precedent until superseded.

### 7.2 Deviation Records

A Deviation Record MUST:
- identify the governing requirement it deviates from,
- state why compliance is not currently pursued,
- state risk and mitigation,
- define explicit resolution criteria and a resolution owner,
- remain “active” until closed, and
- be listed in the Freeze Record while active.

### 7.3 Deferral Records

A Deferral Record MUST:
- identify what is deferred (deliverable or requirement),
- state why it is deferred,
- state whether the deferral is time-bound or permanent,
- remain listed in the Freeze Record while active.

---

## 8. Anti-Patterns

- **Type confusion:** putting MUST-level requirements into README or Design Intent.
- **Hidden binding state:** deviations/deferrals that exist only in chat or memory (must be recorded).
- **Undocumented oracle gaps:** claiming “verified” when there is no oracle or portal definition.
- **Metadata-free artifacts:** anything governance-relevant without frontmatter becomes unretrievable and unauditable.

---

## 9. Version History

### v3.0.0 (draft) — 2026-01-09
- Renamed `0_doc-types` → **SOLVER-Ralph Types**
- Introduced YAML frontmatter metadata standard (core + extensions)
- Added explicit type categories: policy / record / evidence / config
- Added deviation listing requirement for Freeze Record
- Added adjacent governance artifact registry entries

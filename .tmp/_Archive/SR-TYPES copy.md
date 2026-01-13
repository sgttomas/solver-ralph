---
solver_ralph:
  schema: solver-ralph.artifact-metadata/v1
  id: SR-TYPES
  type: governance.types
  title: SOLVER-Ralph Types
  version: "3.4.0-draft.1"
  status: draft
  normative_status: normative
  authority_kind: process
  governed_by:
  - SR-CHANGE
  supersedes:
  - SR-TYPES@3.3.0-draft.8
  created: '2026-01-10'
  updated: "2026-01-12"
  tags:
  - solver-ralph
  - types
  - ontology
  - governance
  - artifacts
---
# SOLVER-Ralph Types v3.4.0-draft.1

## 0. Version Changes

### 3.4.0-draft.1 (2026-01-12)

This version implements **Type Coherence Remediation** changes:

- **Added §1.0 Definitional Delegation**: Clarifies that SR-TYPES registers type keys and provides templates; it does not define the semantics of trust/evidence terms owned by SR-CONTRACT.
- **Refactored §1.1–1.3**: Removed duplicated definitions for concepts owned by other documents:
  - Verified/Approved/Shippable → now references SR-CONTRACT §2.1
  - Proposal/Commitment Object → now references SR-CONTRACT §2.8
  - Verification/Evaluation/Validation/Assessment → now references SR-CONTRACT §2.9
- **Removed §1.3.4 "Paradigm vs Instance"**: This vocabulary implied a false two-level abstraction. Replaced with §1.3.4 "Governed Set vs Deployment" using clearer terminology.
- **Registered new type key**: `governance.authoring_guide` (SR-GUIDE, replacing SR-PARADIGM).
- **Deprecated type key**: `governance.development_paradigm` is superseded by `governance.authoring_guide`.
- **Updated all SR-PARADIGM references** to SR-GUIDE throughout.

### 3.3.0-draft.8 (2026-01-11)
- Marked `governance.plan` as **boot-required** in §4.1 (core alignment substrate) to reflect SR-PLAN's role in controlled decomposition.
- Minor editorial fixes (version header coherence; wording around plan template vs plan instance).

### 3.3.0-draft.7 (2026-01-11)
- Registered `governance.plan` (SR-PLAN) as a core governance document type and added a controlled-document template.
- Fixed subsection heading levels for Freeze Record required sections (Document Manifest / Deferrals / Deviations / Gate Evidence).

### 3.3.0-draft.5 (2026-01-11)
- Clarified the Problem Statement typing rule to align with SR-GUIDE: IPS and SAPS are commitment objects stored as `record.problem_statement` and treated as authoritative inputs for governance authoring.
- Clarified that paraphrase/summaries do not replace the committed record; downstream artifacts should reference the authoritative record id/version to prevent ghost meaning.

### 3.3.0-draft.3 (2026-01-10)
- Clarified that `record.problem_statement` is not boot-required for the governance substrate, but is required to proceed beyond S0 (S1+) under SR-GUIDE; added an explicit `SR-GUIDE S1+ required?` flag in §4.3.

### 3.3.0-draft.2 (2026-01-10)
- Reframed §4.1 to avoid claiming ownership of the canonical governed set; SR-README is the registry of selected governed artifacts.
- Renamed §4.1 to "Core Governance Document Types (boot alignment substrate)" and aligned the table to list boot-required *type keys* (not a canonical file list).
- Clarified that SR-TYPES registers type keys and templates, while SR-README selects the active governed set and pins versions.

---

## 1. Foundations

### 1.0 Definitional Delegation

SR-TYPES is the **type registry** for SOLVER-Ralph. It defines:

- **Type keys** (the taxonomy of artifact kinds)
- **Metadata conventions** (required YAML frontmatter fields)
- **Templates** (minimum structure for each type)
- **Authority model** (what kinds of authority artifacts can claim)

SR-TYPES does **not** define the semantics of:

| Concept Domain | Owning Document | SR-TYPES Role |
|----------------|-----------------|---------------|
| Trust and evidence terms (Verified, Approved, Shippable) | SR-CONTRACT §2.1 | Register as vocabulary; do not redefine |
| Proposal vs Commitment Object | SR-CONTRACT §2.8 | Register as vocabulary; do not redefine |
| Verification / Evaluation / Validation / Assessment | SR-CONTRACT §2.9 | Register as vocabulary; do not redefine |
| Agent ontology and constraints | SR-AGENTS §1–2 | Register type key; do not redefine agent semantics |
| Membrane topology | SR-ETT §4 | Reference only |
| Portal semantics and approval mechanics | SR-CONTRACT §2.6, SR-SPEC | Reference only |

If a type-key description in this document appears to conflict with its owning document, the owning document governs. Apparent conflicts should be routed through SR-CHANGE.

---

### 1.1 Type-Registry Vocabulary

This section registers vocabulary used across the governed set for **type classification purposes**. These are not authoritative definitions—they are registry entries that point to where authoritative semantics live.

| Term | Registry Definition | Authoritative Source |
|------|---------------------|---------------------|
| **Governed Artifact** | A versioned, typed artifact with controlled change semantics and a stable ID | SR-TYPES (this document) |
| **Canonical** | A governed artifact selected into the current governing set | SR-README (selection), SR-CHANGE (routing) |
| **Baseline / Freeze** | A deterministic snapshot of the canonical set and active exceptions/evidence | SR-CONTRACT §2.6, SR-CHANGE §5 |
| **Evidence Bundle** | Content-addressed evidence produced by deterministic oracles | SR-CONTRACT §2.6, SR-SPEC |
| **Candidate** | A content-addressed snapshot of work output that may be verified/released | SR-CONTRACT §2.3 |
| **Portal** | A human trust-boundary where authorization is recorded | SR-CONTRACT §2.6 |
| **Staleness** | A computed marking indicating upstream change may invalidate a state | SR-SPEC |
| **Semantic dependency vs audit provenance** | Dependency propagates staleness; provenance is audit-only | SR-SPEC |
| **Verified / Approved / Shippable** | Trust and evidence state claims | SR-CONTRACT §2.1 |
| **Proposal / Commitment Object** | Fluid vs durable epistemic status | SR-CONTRACT §2.8 |
| **Agent** | Stochastic generator producing proposals; no authority for binding claims | SR-AGENTS §1.2, SR-CONTRACT §C-TB-2 |

---

### 1.2 Evidence vs Authority (Reference)

SOLVER-Ralph distinguishes evidence from authority. For authoritative definitions, see **SR-CONTRACT §2.1** and **§2.9**.

For type-classification purposes:
- **Evidence artifacts** (`normative_status: evidence`) contain oracle outputs; they do not carry authority.
- **Record artifacts** (`normative_status: record`) are durable records of what happened/was decided; they may be binding (approvals, decisions) or non-binding (notes, assessments).
- **Normative artifacts** (`normative_status: normative`) contain binding requirements.

---

### 1.3 Semantic Commitments (Vocabulary Constraints)

This section registers vocabulary constraints for terms that are frequently confused in stochastic-agent workflows. These constraints help prevent "foundation corruption" through casual reinterpretation.

#### 1.3.1 Proposal vs Commitment Object (Reference)

For the authoritative distinction, see **SR-CONTRACT §2.8**.

**Type-classification guidance:**
- Proposals are not typed artifacts in the registry sense—they are any content that has not yet crossed an evidence or authority membrane.
- Commitment objects include: governed artifacts, Candidates, Evidence Bundles, Approval records, Decision records, Freeze records, Exception records, and typed problem-statement records.

#### 1.3.2 Verification, Evaluation, Validation, Assessment (Reference)

For the authoritative 4-way distinction, see **SR-CONTRACT §2.9**.

**Type-classification guidance:**
- Verification/validation outputs are typed as `evidence.*` artifacts.
- Evaluation/assessment outputs may be typed as `record.evaluation_note` or `record.assessment_note`—these are non-binding records.
- Approvals are typed as `record.approval` or captured in event records—these are binding.

#### 1.3.3 Problem Statement Family (RPS → IPS → SAPS → INPS)

SR-GUIDE defines a staged compilation of a human problem statement. This section registers the type semantics.

- **RPS (Raw Problem Statement)**: the initial human statement (ambiguous, incomplete).
- **IPS (Interpreted Problem Statement)**: clarified statement that can be read back without contradiction.
- **SAPS (Structured, Aligned Problem Statement)**: a forced intermediate representation (IR) that classifies requirements, evidence plans, authority boundaries, and decomposition.
- **INPS (Instantiated Problem Statement)**: the SAPS expressed across the governed artifact set.

**Typing rule:**
- Under SR-GUIDE S0, IPS and SAPS MUST be captured as versioned `record.problem_statement` artifacts with `ext.problem_statement.stage` set appropriately (`IPS` / `SAPS`).
- RPS SHOULD be captured as `record.problem_statement` (`stage: RPS`) when the raw statement will be referenced later, or when stakes/complexity warrant preserving the original ambiguity as an auditable artifact.
- Paraphrase/summaries MAY be written for readability, but the committed `record.problem_statement` is authoritative; downstream artifacts SHOULD reference the authoritative record id + version to prevent "ghost meaning."

#### 1.3.4 Governed Set vs Deployment

The governed set specifies *one system*. There is no abstract "paradigm" with multiple possible "instances."

- **Governed set**: The collection of SR-* documents (as selected and versioned by SR-README) that specify the system's invariants, mechanics, and processes.
- **Deployment**: A running installation of the software that enforces the governed set. Deployments may have deployment-specific configuration (connection strings, endpoints) but MUST comply with the governed set.

**Constraint:** Do not use "paradigm" and "instance" as document-layer concepts. Use "governed set" for the documents and "deployment" (or "running system") for the software.

---

## 2. Authority Model

### 2.1 Authority Kinds

Authority kind indicates *what kind of authority an artifact claims to exercise*:

- **content**: binding meaning about invariants, definitions, and technical semantics (when normative).
- **process**: binding meaning about procedures and enforcement (when normative).
- **record**: durable, attributable records (some binding, some non-binding) used for audit and controlled memory.
- **config**: configuration that affects behavior without being a trust-boundary authority.
- **index**: navigation/orientation only; MUST NOT introduce new binding semantics.

### 2.2 Normative Status Classes

Normative status indicates *how binding the artifact's contents are*:

- **normative**: binding requirements.
- **directional**: guidance and rationale; not binding unless referenced by a normative requirement.
- **index**: navigation; not binding.
- **record**: durable record artifact (binding *as a record of what happened/was decided*, not automatically a Portal authorization).
- **evidence**: evidence artifact produced by oracles; not authority.

### 2.3 Purpose Priority and Binding Precedence

SOLVER-Ralph uses a binding precedence order (used by deterministic context compilation):

1. **Architectural Contract (Why)** — SR-CONTRACT
2. **Technical Specification (What)** — SR-SPEC
3. **Development Directive (How)** — SR-DIRECTIVE
4. **Design Intent (Why² / rationale)** — SR-INTENT
5. **README / Index** — SR-README

**Conflict rule:** If binding artifacts conflict (e.g., SR-CONTRACT vs SR-SPEC), agents MUST stop and escalate. Conflicts are resolved via SR-CHANGE, not by ad hoc interpretation.

**Note on meta-governance artifacts:** SR-TYPES, SR-CHANGE, SR-GUIDE, SR-AGENTS, and SR-ETT may impose additional constraints within their scope, but they MUST NOT be used to "reinterpret away" conflicts among binding artifacts. Route conflicts through SR-CHANGE.

---

## 3. Metadata Standard (Controlled Documents)

### 3.1 Required YAML Frontmatter

Every governed markdown artifact MUST include YAML frontmatter with the following minimum fields:

- `solver_ralph.schema` (current: `"solver-ralph.artifact-metadata/v1"`)
- `solver_ralph.id` (stable ID, e.g., `"SR-SPEC"`)
- `solver_ralph.type` (type key from this registry)
- `solver_ralph.title`
- `solver_ralph.version` (SemVer string for versioned artifacts; index artifacts MAY use `"unversioned"` while in early drafts)
- `solver_ralph.status` (enum: `draft | governed | superseded | deprecated | archived`)
- `solver_ralph.normative_status` (enum: `normative | directional | index | record | evidence`)
- `solver_ralph.authority_kind` (enum: `content | process | record | config | index`)
- `solver_ralph.governed_by` (typically `["SR-CHANGE"]`)
- `solver_ralph.created`, `solver_ralph.updated` (ISO date strings)

### 3.2 Extension Namespace (`ext`)

Type-specific fields MUST be placed under `solver_ralph.ext.<type_key_shortname>` (e.g., `ext.design_intent`, `ext.arch_contract`).

**Constraint:** An artifact MUST NOT introduce untyped/unstable extension shapes that become relied upon as semantics without routing through SR-CHANGE. If a legacy artifact deviates from this pattern, it MUST be migrated via SR-CHANGE before tooling may rely on its extension fields.

---

## 4. Type Registry

### 4.1 Core Governance Document Types (boot alignment substrate)

This section registers the **core governance document type keys** that a SOLVER-Ralph deployment relies on to establish a shared ontology, trust model, execution discipline, and change control.

**Important (registry vs selection):** SR-TYPES defines **type keys and templates**. It does **not** enumerate the canonical governed set for a deployment. The canonical governed set (selected artifacts, pinned versions, and any deployment-specific additions) is owned by **SR-README**, and changes to that selection are routed through **SR-CHANGE**.

The table below therefore answers: *"Which governance document types exist, and which are expected to be present to boot a deployment?"* — not *"Which concrete files are currently canonical?"*.

| Type Key | Boot-required? | Authority Kind | Normative Status | Purpose |
|---|---:|---|---|---|
| `governance.readme` | Yes | index | index | Orientation and navigation index (registry pointer). |
| `governance.types` | Yes | process | normative | Artifact taxonomy, metadata rules, and type registry (this document). |
| `governance.epistemic_trust_topology` | Yes | process | directional | Trust membrane model; coercion placement rationale. |
| `governance.authoring_guide` | Yes | process | normative | Problem-statement compilation and governance authoring workflow (SR-GUIDE). |
| `governance.plan` | Yes | process | normative | Typed planning/decomposition artifact (SR-PLAN): packages → deliverables → deps; binds the *what* before execution sequencing. |
| `governance.agents` | Yes | process | normative | Agent object model + hard constraints on authority/evidence claims (SR-AGENTS). |
| `governance.design_intent` | Yes | content | directional | Rationale and interpretation guidance (SR-INTENT). |
| `governance.arch_contract` | Yes | content | normative | Binding architectural invariants (SR-CONTRACT). |
| `governance.technical_spec` | Yes | content | normative | Binding mechanics, schemas, and interfaces (SR-SPEC). |
| `governance.dev_directive` | Yes | process | normative | Execution/gating policy and evidence expectations (SR-DIRECTIVE). |
| `governance.change_mgmt` | Yes | process | normative | Change control for governed set and meaning evolution (SR-CHANGE). |

**Deprecated type keys:**

| Type Key | Superseded By | Notes |
|----------|---------------|-------|
| `governance.development_paradigm` | `governance.authoring_guide` | SR-PARADIGM renamed to SR-GUIDE; existing references should be updated |

### 4.2 Baseline Output Artifacts

| Type Key | Boot-required? | Authority Kind | Normative Status | Examples |
|---|---:|---|---|---|
| `record.freeze` | Yes | record | record | Freeze record snapshot. |
| `evidence.gate_packet` | Yes | record | evidence | Gate packet / evidence manifest bundle. |

### 4.3 Alignment & Intake Artifacts

**Boot-required?** indicates whether the artifact must exist to *stand up the governance substrate*.
**SR-GUIDE S1+ required?** indicates whether the artifact must exist to *proceed beyond S0* (i.e., begin governance authoring/instantiation stages) under SR-GUIDE.

| Type Key | Boot-required? | SR-GUIDE S1+ required? | Authority Kind | Normative Status | Examples |
|---|---:|---:|---|---|---|
| `record.problem_statement` | No | Yes | record | record | RPS/IPS/SAPS record (stage-tagged). |

### 4.4 Adjacent Governance Artifacts

These artifacts are intentionally **not** baseline members, but they must remain visible, referenceable, and versioned.

| Type Key | Boot-required? | Authority Kind | Normative Status | Examples |
|---|---:|---|---|---|
| `policy.decision_heuristic` | No | process | directional | heuristics for decision routing/escalation |
| `policy.oracle_integrity` | No | process | normative | oracle integrity conditions (TAMPER/GAP/FLAKE/ENV_MISMATCH) |
| `record.intervention_note` | No | record | record | human intervention note (non-binding) |
| `record.evaluation_note` | No | record | record | human evaluation of verification evidence |
| `record.assessment_note` | No | record | record | human assessment of validation evidence |
| `record.decision` | No | record | record | binding human decision record |
| `record.waiver` | No | record | record | binding gate waiver for Verified-with-Exceptions |
| `record.deviation` | No | record | record | binding exception from a governed requirement |
| `record.deferral` | No | record | record | binding postponement |
| `index.quick_reference` | No | index | index | fast lookup for key rules |

### 4.5 Implementation Configuration Artifacts

These artifacts affect behavior but are governed indirectly.

| Type Key | Boot-required? | Authority Kind | Normative Status | Examples |
|---|---:|---|---|---|
| `config.agent_definition` | No | config | directional | agent capability/profile definitions |
| `config.template_catalog` | No | config | directional | prompt/template catalog referenced by agent definitions |
| `config.gating_policy` | No | config | directional | work-unit gating mode + overrides |
| `config.ci_policy` | No | config | directional | build targets, CI scripts enforcing oracles |

---

## 5. Governance Document Type Specifications

This section provides scope boundaries and templates for governance documents that shape meaning and process.

### 5.1 Project Orientation README

**Purpose:** Provide onboarding, navigation, and "how to use the governed set."

**Normative status:** **Index** (consultative, not binding requirements).

**Quality criteria:**
- [ ] Has a document index with current versions
- [ ] Explains "when to consult which document"
- [ ] Links to change management + freeze record
- [ ] Does not introduce new MUST-level requirements

**Template (minimum):**
```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-README"
  type: "governance.readme"
  title: "{Project} SOLVER-Ralph README"
  version: "{X.Y.Z}"
  status: "governed"
  normative_status: "index"
  authority_kind: "index"
  governed_by: ["SR-CHANGE"]
  created: "YYYY-MM-DD"
  updated: "YYYY-MM-DD"
  tags: ["solver-ralph", "readme", "index", "governance"]
---
# {Project} SOLVER-Ralph README v{X.Y.Z}
```

### 5.2 Design Intent

**Purpose:** Capture rationale, tradeoffs, and interpretive notes; does not redefine binding semantics.

**Normative status:** **Directional**.

**Template (minimum):**
```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-INTENT"
  type: "governance.design_intent"
  title: "{Project} SOLVER-Ralph Intent"
  version: "{X.Y.Z}"
  status: "governed"
  normative_status: "directional"
  authority_kind: "content"
  governed_by: ["SR-CHANGE"]
  created: "YYYY-MM-DD"
  updated: "YYYY-MM-DD"
  ext:
    design_intent:
      primary_problem: ""
      scope_notes: []
      tradeoffs: []
      non_goals: []
---
# {Project} SOLVER-Ralph Intent v{X.Y.Z}
```

### 5.3 Architectural Contract

**Purpose:** Binding invariants and prohibitions.

**Normative status:** **Normative** (highest precedence).

**Verification rule (high assurance):** Every Contract MUST be either:
- deterministically checkable by an oracle, OR
- explicitly portal-reviewed with a recorded decision/approval.

**Template (minimum):**
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
      invariants: []
      trust_boundaries: []
      non_waivable_conditions: []
---
# {Project} Architectural Contract v{X.Y.Z}
```

### 5.4 Technical Specification

**Purpose:** Define mechanics, schemas, and state machines.

**Normative status:** **Normative**.

**Template:** (see SR-SPEC)

### 5.5 Development Directive

**Purpose:** Gated execution plan.

**Normative status:** **Normative** (process authority).

**Note:** A directive may include both binding gates and informative guidance; the artifact-level `normative_status` remains **normative** because it contains binding process constraints.

**Template (minimum):**
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
  ext:
    dev_directive:
      phases: []
      portals: []
      stop_triggers: {}
      verification_profiles: []
---
# {Project} Development Directive v{X.Y.Z}
```

### 5.6 Change Management

**Purpose:** Define change categories, freeze rules, and governance procedures.

**Normative status:** **Normative** (process authority).

**Template:** (see SR-CHANGE)

### 5.7 Epistemic Trust Topology

**Purpose:** Define the harness membranes that convert fluid proposals into durable, attributable commitments.

**Normative status:** **Directional** (consultative; does not redefine binding semantics).

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
  ext:
    epistemic_trust_topology:
      harness_members: []        # list of membranes (names + modality)
      commitment_objects: []     # canonical commitment object classes
      assumptions: []
---
# {Project} Epistemic Trust Topology v{X.Y.Z}
```

### 5.8 Governance Authoring Guide

**Type key:** `governance.authoring_guide`

**Purpose:** Define how to compile a human problem statement into a Structured, Aligned Problem Statement (SAPS) and author governed artifacts (see SR-GUIDE).

**Normative status:** **Normative** (process authority).

**Template (minimum):**
```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-GUIDE"
  type: "governance.authoring_guide"
  title: "{Project} Governance Authoring Guide"
  version: "{X.Y.Z}"
  status: "governed"
  normative_status: "normative"
  authority_kind: "process"
  governed_by: ["SR-CHANGE"]
  created: "YYYY-MM-DD"
  updated: "YYYY-MM-DD"
  ext:
    authoring_guide:
      objective: ""
      problem_statement_ir: "SAPS"
      required_alignment_artifacts: ["record.problem_statement"]
---
# {Project} Governance Authoring Guide v{X.Y.Z}
```

### 5.9 Agents

**Purpose:** Define what an "agent" is (object model), and the binding constraints on agent authority claims (see SR-AGENTS).

**Normative status:** **Normative** (process authority).

**Template (minimum):**
```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-AGENTS"
  type: "governance.agents"
  title: "{Project} Agents"
  version: "{X.Y.Z}"
  status: "governed"
  normative_status: "normative"
  authority_kind: "process"
  governed_by: ["SR-CHANGE"]
  created: "YYYY-MM-DD"
  updated: "YYYY-MM-DD"
  ext:
    agents:
      invariants:
        - "agents_produce_proposals"
        - "agents_cannot_assert_verified"
        - "agents_cannot_assert_approved"
        - "agents_cannot_assert_shippable"
      evidence_expectations: []
---
# {Project} Agents v{X.Y.Z}
```

---

### 5.10 Plan

**Type key:** `governance.plan`  
**Authority kind:** process  
**Normative status:** normative  

**Purpose:** Provide a typed, governed plan object model for decomposing an authoritative SAPS into packages and loop-completable deliverables. SR-PLAN is *about decomposition* (the "what"), not execution (the "how").

**Invariants**
- The plan MUST reference the authoritative `record.problem_statement` (stage `SAPS`) via typed `refs[]`.
- Every deliverable MUST declare a target type key (from this registry) and a completion condition that yields evidence hooks.
- SR-PLAN MUST NOT define portal semantics, approval semantics, or closure rules. Execution/state progression belongs to SR-DIRECTIVE and the runtime.

**Template (minimum):**
```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-PLAN"
  type: "governance.plan"
  title: "SR-PLAN"
  version: "1.0.0"
  status: "draft"
  normative_status: "normative"
  authority_kind: "process"
  governed_by: ["SR-CHANGE"]
  supersedes: []
  created: "{YYYY-MM-DD}"
  updated: "{YYYY-MM-DD}"
  refs:
    # For a plan template, refs SHOULD include SR-TYPES and SR-GUIDE.
    # For a plan instance, refs MUST include the authoritative SAPS record.
    - "SR-TYPES@{version}"
    - "SR-GUIDE@{version}"
  tags: ["solver-ralph", "plan", "decomposition"]
  ext:
    plan:
      plan_kind: "template"   # "template" | "instance"
      instance_id: null       # required for plan_kind="instance"
      problem_statement_ref: null  # required for plan_kind="instance"
      packages: []            # list[package]
      deliverables: []        # list[deliverable]
      relationships:
        depends_on: []        # semantic dependency edges among deliverables/packages
      invariants:
        closure_prohibited: true
        execution_semantics_prohibited: true
---

# SR-PLAN v{X.Y.Z}

(Body describes the plan schema, invariants, and examples. Concrete plans populate `ext.plan.packages` and `ext.plan.deliverables`.)
```


## 6. Baseline & Alignment Output Artifact Specifications

### 6.1 Freeze Record

**Purpose:** Provide an auditable, deterministic baseline snapshot:
- hashes of the canonical governed set,
- active deferrals and deviations at baseline,
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

#### 6.1.1 Document Manifest
| Document | Version | SHA-256 |
|---|---|---|

#### 6.1.2 Deferrals Active at This Baseline
- ...

#### 6.1.3 Deviations Active at This Baseline
- ...

#### 6.1.4 Gate Evidence
- ...
```

### 6.2 Problem Statement Record (`record.problem_statement`)

**Purpose:** Provide a durable, typed, replayable capture of the problem statement at a defined compilation stage (RPS/IPS/SAPS).

**Normative status:** Record.

**Required metadata (YAML frontmatter):**
- `solver_ralph.id`: stable id for this record artifact
- `solver_ralph.type`: `record.problem_statement`
- `solver_ralph.version`: SemVer
- `solver_ralph.normative_status`: `record`
- `solver_ralph.authority_kind`: `record`

**Required extension shape (under `ext.problem_statement`):**
- `stage` (enum): `RPS | IPS | SAPS`
- `scope` (object): minimally one of `{ program_id | project_id | work_unit_id | loop_id }`
- `source_refs[]` (array, optional): typed refs for provenance (e.g., decisions, prior problem statements, evidence)
- `owner` (object, optional): `{ actor_kind: HUMAN, actor_id: "..." }` when a human "owns" the statement

**Template (minimum):**
```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "PS-{ULID}"
  type: "record.problem_statement"
  title: "{Short Name} Problem Statement ({STAGE})"
  version: "{X.Y.Z}"
  status: "governed"
  normative_status: "record"
  authority_kind: "record"
  governed_by: ["SR-CHANGE"]
  created: "YYYY-MM-DD"
  updated: "YYYY-MM-DD"
  ext:
    problem_statement:
      stage: "SAPS"
      scope:
        work_unit_id: "{work-unit-id}"
      source_refs: []
---

# {Short Name} Problem Statement ({STAGE}) v{X.Y.Z}

(Body content follows SR-GUIDE's SAPS field set. The depth of each section is proportional to the stakes and ambiguity.)
```

---

## 7. Adjacent Governance Artifact Notes

These artifacts are intentionally not baseline members, but they must remain visible, referenceable, and versioned.

### 7.1 Decision Records

A Decision Record MUST:
- have a stable ID,
- state the question, options considered, decision, and rationale,
- reference affected governed requirements (Contract/Spec/Directive),
- be treated as binding precedent until superseded.

### 7.2 Deviation Records

Deviation records represent explicit non-compliance with a governed requirement; they MUST specify scope, risk, mitigation, and a resolution condition.

### 7.3 Deferral Records

Deferrals represent explicit postponement of a requirement or deliverable; they MUST include target condition/date and expiry/review constraints.

### 7.4 Waiver Records (Verified-with-Exceptions)

Waivers are exception records used to treat a candidate as Verified-with-Exceptions under an approved mode; they MUST NOT be used to bypass non-waivable integrity conditions.

---

## 8. Implementation Configuration Notes

### 8.1 `config.template_catalog` (directional config artifact)

A `config.template_catalog` provides a versioned catalog of templates/prompts used by agent runtimes and document generators.

**Intended use:**
- referenced by `config.agent_definition.ext.agent_definition.template_catalog_ref`,
- treated as audit provenance by default (`rel=supported_by`), not as a staleness-propagating dependency.

**Required extension shape (under `ext.template_catalog`):**
- `catalog_id` (string): stable identifier
- `entries[]` (array): `{ template_id, kind, description, content_ref }`
  - `kind` MAY include: `prompt | doc_template | snippet`
  - `content_ref` SHOULD be content-addressed (hash) or a governed-artifact ref.

**Prohibitions (normative):**
- MUST NOT be used to introduce new authority or redefine governed semantics.
- If a template change effectively changes binding governance meaning, that meaning MUST be expressed in governed artifacts and routed via SR-CHANGE.

---

## 9. Final Notes

This type system is intentionally conservative: adding new type keys is permitted, but new types MUST NOT be used as a backdoor for new authority claims.

If a mismatch is discovered between:
- this type registry,
- SR-README selections,
- SR-CHANGE freeze/canonical rules,
- or SR-SPEC schema validation,

the correct response is to open a governed change and reconcile explicitly.

---
solver_ralph:
  schema: solver-ralph.artifact-metadata/v1
  id: SR-TYPES
  type: governance.types
  title: SOLVER-Ralph Types
  version: 3.3.0-draft.2
  status: draft
  normative_status: normative
  authority_kind: process
  governed_by:
  - SR-CHANGE
  supersedes:
  - SR-TYPES@3.3.0-draft.1
  created: '2026-01-11'
  updated: '2026-01-11'
  tags:
  - solver-ralph
  - types
  - ontology
  - governance
  - artifacts
---
# SOLVER-Ralph Types v3.3.0-draft.2

## 0. Version Changes


### 3.3.0-draft.2 (2026-01-11)
- Reframed §4.1 to avoid claiming ownership of the canonical governed set; SR-README is the registry of selected governed artifacts.
- Renamed §4.1 to “Core Governance Document Types (boot alignment substrate)” and aligned the table to list boot-required *type keys* (not a canonical file list).
- Clarified that SR-TYPES registers type keys and templates, while SR-README selects the active governed set and pins versions.
### 0.1 Changes since v3.2.2

**Corrections**
- Fixed document heading/version mismatch (v3.2.1 → v3.2.2 inconsistency).
- Corrected invalid type-registry values:
  - `governance.dev_directive` authority kind is **process** (not `process/content`) and normative status is **normative** (not `mixed`).
  - `policy.oracle_integrity` normative status is **normative** (replacing the non-enum label `normative (local)`).
- Added `authority_kind: "index"` to the allowed authority kinds (aligns with `SR-README` being an index artifact).

**Additions (non-breaking)**
- Registered new type keys required for alignment with SR-PARADIGM work:
  - `governance.development_paradigm` (SR-PARADIGM)
  - `governance.agents` (SR-AGENTS)
  - `record.problem_statement` (RPS/IPS/SAPS record family)
  - `config.template_catalog` (template/prompt catalog referenced by `config.agent_definition`)
- Expanded semantic definitions to reduce drift:
  - Proposal vs Commitment Object
  - Verification vs Evaluation vs Validation vs Assessment
  - Problem Statement family: RPS → IPS → SAPS → INPS

**Non-goals**
- This revision does **not** change portal semantics, Verified/Approved/Shippable definitions, or event schemas (those are governed by SR-CONTRACT / SR-SPEC).

---

## 1. Foundations

### 1.1 The Ontology Baseline

SOLVER-Ralph constrains work by forcing a stable ontology. The following terms are used across the governed set:

- **Governed Artifact**: A versioned, typed artifact with controlled change semantics and a stable ID.
- **Canonical**: A governed artifact selected into the current governing set for an instance.
- **Baseline / Freeze**: A deterministic snapshot of the canonical set and active exceptions/evidence required to support “Shippable.”
- **Evidence Bundle**: Content-addressed evidence produced by deterministic oracles (manifests + results + environment fingerprint).
- **Candidate**: A content-addressed snapshot of work output (e.g., git commit + hash identity) that may be verified/released.
- **Portal**: A human trust-boundary action where authorization is recorded.
- **Staleness**: A computed marking indicating that a previously accepted state may no longer be valid due to upstream change.
- **Semantic dependency vs audit provenance**:
  - dependency participates in staleness/impact propagation by default;
  - provenance is non-blocking by default.
- **Agent**: A stochastic generator producing proposals; agents do not possess authority to create binding trust-boundary state.

### 1.2 Epistemology (Evidence vs Authority)

SOLVER-Ralph distinguishes evidence from authority:

- **Verified** means deterministic oracles ran and produced recorded evidence (an evidence claim).
- **Approved** means a human crossed a trust boundary and accepted responsibility (an authority claim).
- **Shippable** is a computed outcome derived from Verified + Approved + freeze/baseline rules.

These terms are defined *normatively* in SR-CONTRACT / SR-SPEC and are referenced here only for ontology coherence.

### 1.3 Semantic Commitments (Vocabulary Constraints)

This section defines terms that are frequently confused in stochastic-agent workflows. These are vocabulary constraints intended to prevent “foundation corruption” through casual reinterpretation.

#### 1.3.1 Proposal vs Commitment Object

- **Proposal**: Any draft, statement, or artifact whose meaning is not yet stabilized as a durable, attributable object in the system.
  - Examples: agent narrative claims (“tests passed”), untracked notes, ad hoc chat summaries, unreferenced external links.

- **Commitment Object**: Any durable object that is content-addressed and referenceable such that downstream work can cite it without relying on “trust me.”
  - Examples: governed artifacts (SR-* docs), Candidates, Evidence Bundles, Approval records, Decision records, Freeze records, Exception records, and typed intake/problem-statement records.

**Rule of thumb:** downstream work MUST treat proposals as non-authoritative unless and until the relevant content has been converted into a commitment object (typed, stable ID, content hash where applicable) and any required evidence/portal membrane has been satisfied.

#### 1.3.2 Verification, Evaluation, Validation, Assessment

SOLVER-Ralph uses a 4-way distinction:

- **Verification (agentic / oracle-driven)**: Evidence production about *conformance* to explicit requirements (tests, linters, schema checks, reproducibility checks). Output is evidence (not authority).
- **Evaluation (human)**: Human interpretation of verification evidence. Output is a record (e.g., `record.evaluation_note`) and is non-binding unless it is elevated into a Portal approval/Decision.
- **Validation (agentic / evidence-backed)**: Evidence production about *fitness in context* (does this satisfy the real objective, constraints, user impact). May include empirical checks, domain tests, or structured probes.
- **Assessment (human)**: Human interpretation of validation evidence in context. Output is a record (e.g., `record.assessment_note`) and is non-binding unless elevated into a Portal approval/Decision.

**Important:** Evaluation/Assessment are not substitutes for Portal approvals. They are legible, typed inputs that may be referenced by later work, but they do not create Verified/Approved/Shippable states.

#### 1.3.3 Problem Statement Family (RPS → IPS → SAPS → INPS)

SR-PARADIGM defines a staged compilation of a human problem statement:

- **RPS (Raw Problem Statement)**: the initial human statement (ambiguous, incomplete).
- **IPS (Interpreted Problem Statement)**: clarified statement that can be read back without contradiction.
- **SAPS (Structured, Aligned Problem Statement)**: a forced intermediate representation (IR) that classifies requirements, evidence plans, authority boundaries, and decomposition.
- **INPS (Instantiated Problem Statement)**: the SAPS “compiled” into governed artifacts, work units, and verification plans.

**Typing rule:** RPS/IPS/SAPS SHOULD be captured as a versioned `record.problem_statement` with `ext.problem_statement.stage` indicating the stage, so alignment is auditable and replayable.

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

Normative status indicates *how binding the artifact’s contents are*:

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

**Note on meta-governance artifacts:** SR-TYPES, SR-CHANGE, SR-PARADIGM, SR-AGENTS, and SR-ETT may impose additional constraints within their scope, but they MUST NOT be used to “reinterpret away” conflicts among binding artifacts. Route conflicts through SR-CHANGE.

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

This section registers the **core governance document type keys** that a SOLVER-Ralph instance relies on to establish a shared ontology, trust model, execution discipline, and change control.

**Important (registry vs selection):** SR-TYPES defines **type keys and templates**. It does **not** enumerate the canonical governed set for an instance. The canonical governed set (selected artifacts, pinned versions, and any instance-specific additions) is owned by **SR-README**, and changes to that selection are routed through **SR-CHANGE**.

The table below therefore answers: *“Which governance document types exist, and which are expected to be present to boot an instance?”* — not *“Which concrete files are currently canonical?”*.

| Type Key | Boot-required? | Authority Kind | Normative Status | Purpose |
|---|---:|---|---|---|
| `governance.readme` | Yes | index | index | Orientation and navigation index (registry pointer). |
| `governance.types` | Yes | process | normative | Artifact taxonomy, metadata rules, and type registry (this document). |
| `governance.epistemic_trust_topology` | Yes | process | directional | Trust membrane model; coercion placement rationale. |
| `governance.development_paradigm` | Yes | process | normative | Problem-statement compilation and governance authoring workflow (SR-PARADIGM). |
| `governance.agents` | Yes | process | normative | Agent object model + hard constraints on authority/evidence claims (SR-AGENTS). |
| `governance.design_intent` | Yes | content | directional | Rationale and interpretation guidance (SR-INTENT). |
| `governance.arch_contract` | Yes | content | normative | Binding architectural invariants (SR-CONTRACT). |
| `governance.technical_spec` | Yes | content | normative | Binding mechanics, schemas, and interfaces (SR-SPEC). |
| `governance.dev_directive` | Yes | process | normative | Execution/gating policy and evidence expectations (SR-DIRECTIVE). |
| `governance.change_mgmt` | Yes | process | normative | Change control for governed set and meaning evolution (SR-CHANGE). |

### 4.2 Baseline Output Artifacts

| Type Key | Boot-required? | Authority Kind | Normative Status | Examples |
|---|---:|---|---|---|
| `record.freeze` | Yes | record | record | Freeze record snapshot. |
| `evidence.gate_packet` | Yes | record | evidence | Gate packet / evidence manifest bundle. |

### 4.3 Alignment & Intake Artifacts

| Type Key | Boot-required? | Authority Kind | Normative Status | Examples |
|---|---:|---|---|---|
| `record.problem_statement` | No | record | record | RPS/IPS/SAPS record (stage-tagged). |

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

**Purpose:** Provide onboarding, navigation, and “how to use the governed set.”

**Normative status:** **Index** (consultative, not binding requirements).

**Quality criteria:**
- [ ] Has a document index with current versions
- [ ] Explains “when to consult which document”
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
      commitment_objects: []     # canonical commitment object classes for this instance
      assumptions: []
---
# {Project} Epistemic Trust Topology v{X.Y.Z}
```

### 5.8 Development Paradigm

**Purpose:** Define how to compile a human problem statement into a Structured, Aligned Problem Statement (SAPS) and instantiate governed artifacts (see SR-PARADIGM).

**Normative status:** **Normative** (process authority).

**Template (minimum):**
```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-PARADIGM"
  type: "governance.development_paradigm"
  title: "{Project} Development Paradigm"
  version: "{X.Y.Z}"
  status: "governed"
  normative_status: "normative"
  authority_kind: "process"
  governed_by: ["SR-CHANGE"]
  created: "YYYY-MM-DD"
  updated: "YYYY-MM-DD"
  ext:
    development_paradigm:
      objective: ""
      problem_statement_ir: "SAPS"
      required_alignment_artifacts: ["record.problem_statement"]
---
# {Project} Development Paradigm v{X.Y.Z}
```

### 5.9 Agents

**Purpose:** Define what an “agent” is in this paradigm (object model), and the binding constraints on agent authority claims (see SR-AGENTS).

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
- `owner` (object, optional): `{ actor_kind: HUMAN, actor_id: "..." }` when a human “owns” the statement

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

(Body content follows SR-PARADIGM’s SAPS field set. The depth of each section is proportional to the stakes and ambiguity.)
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

---
doc_id: SR-TYPES
doc_kind: governance.types
layer: platform
status: draft
refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: depends_on
    to: SR-CONTRACT
---

# SOLVER-Ralph Types 

**Scope note:** SR-TYPES registers both (a) **runtime-intrinsic** artifact types and (b) **build scaffolding** artifact types used to construct and govern the platform itself. Runtime invariants live in **SR-CONTRACT**; mechanics live in **SR-SPEC**. When in conflict, CONTRACT/SPEC control.

**Naming note:** The canonical surface form for the core work unit is `semantic-ralph-loop` (hyphenated, lowercase). Do not introduce new variants; normalize via SR-CONTRACT §2.11.


## 1. Foundations

### 1.0 Domains of Statements (Meaning Ownership)


A) Platform-definition semantics

Meaning ownership: SR-TYPES (terms), SR-CONTRACT (invariants), SR-SPEC (mechanics).
Other docs may only reference these meanings.

B) Build-execution governance

Process ownership: SR-AGENTS/SR-CHANGE/SR-EXCEPTIONS/SR-DIRECTIVE (how agents work, how changes happen, how exceptions are granted, what gates/budgets apply).
These docs may constrain work, but cannot alter platform semantics.

C) Tracking/projections

State ownership: SR-RUNLIST (and/or ledger) as a projection over PLAN scope + DIRECTIVE dependencies + recorded evidence/approvals.
These docs record progress; they do not define scope or semantics.

And then a single enforcement line:

If a proposed change introduces or modifies platform-definition semantics outside SR-TYPES/SR-CONTRACT/SR-SPEC, it is invalid and must be rerouted to the owning document kind.

**Expand → Contract discipline (applies to all governed work):**
- **Expansion** (exploring candidate meanings) is permitted only in **SR-INTENT** (non-binding) and must not introduce normative requirements.
- **Contraction** (fixing meaning) occurs only in **SR-TYPES** (ontology), **SR-CONTRACT** (invariants), and **SR-SPEC** (mechanics).
- **Projection/config/tracking** documents (**SR-PLAN**, **SR-DIRECTIVE**, **SR-RUNLIST**) must never define meaning; they only reference contracted meaning and record/configure execution.


The **Layer 1/2/3** narrative is maintained as a **non-binding mapping appendix**; see **Appendix A**.



**Key distinction:**
- **Platform types** (Layer 2) are what the running software tracks
- **Specification types** (Layer 1) define how we build the platform
- **User types** (Layer 3) are defined by users, not the platform

This document registers all three categories but distinguishes them clearly.

**Scope clarification:** This document includes both runtime-intrinsic platform domain types (§4.3, §7) and build-time/governance typing (§4.1, §4.2, §6). The platform domain types are intrinsic to the running platform; the specification and scaffolding types support building the platform but are not tracked at runtime. See §4 for the complete registry.

---

### 1.1 Type-Registry Vocabulary

This section registers vocabulary for type classification. These are registry entries pointing to where authoritative semantics live.

| Term | Registry Definition | Authoritative Source |
|------|---------------------|---------------------|
| **Governed Artifact** | A versioned, typed artifact with controlled change semantics | SR-TYPES (this document) |
| **Canonical** | A governed artifact selected into the current set | SR-GUIDE |
| **Candidate** | Content-addressed snapshot of work output | SR-SPEC |
| **Evidence Bundle** | Content-addressed oracle output | SR-SPEC |
| **Portal Decision** | Human authorization record | SR-SPEC |
| **Work Unit** | State machine for tracked work | SR-SPEC |
| **Loop Record** | Ralph-loop iteration summary | SR-SPEC |
| **Event** | Immutable state change record | SR-SPEC |
| **Verified / Approved / Shippable** | Trust and evidence states | SR-CONTRACT |
| **Proposal / Commitment Object** | Fluid vs durable status | SR-CONTRACT |
| **Agent** | Stochastic generator; no authority for binding claims | SR-AGENTS, SR-CONTRACT |

**Commitment Object note:** “Commitment Object” is a cross-cutting *durability* concept (defined in SR-CONTRACT §2.8), not a unique SR-TYPES `type_key`. Many types (Candidates, Evidence Bundles, Portal Decisions, Events, and `record.*` exception/decision records) are commitment objects.

---

### 1.2 Evidence vs Authority (Reference)

For authoritative definitions, see **SR-CONTRACT**.

For type-classification purposes:
- **Evidence artifacts** (`normative_status: evidence`) contain oracle outputs; they do not carry authority
- **Record artifacts** (`normative_status: record`) are durable records; may be binding (decisions) or non-binding (notes)
- **Normative artifacts** (`normative_status: normative`) contain binding requirements

---

### 1.3 Vocabulary Constraints

#### 1.3.1 Proposal vs Commitment Object (Reference)

For the authoritative distinction, see **SR-CONTRACT §2.8**.

**Type-classification guidance:**
- Proposals are not typed artifacts—they are content that has not crossed a trust boundary
- Commitment objects include: Candidates, Evidence Bundles, Portal Decisions, Events

**Clarification:** Commitment Object is a durability/referenceability category, not a normative_status value. Commitment Objects span both `normative_status: record` (e.g., Portal Decisions, Candidates) and `normative_status: evidence` (e.g., Evidence Bundles). The defining characteristic is that they are durable, content-addressed, and safe for downstream reliance—see SR-CONTRACT §2.8.

#### 1.3.2 Verification, Evaluation, Validation, Assessment (Reference)

For the authoritative 4-way distinction, see **SR-CONTRACT**.

**Type-classification guidance:**
- Verification/validation outputs → `evidence.*` types
- Evaluation/assessment outputs → `record.evaluation_note`, `record.assessment_note`
- Approvals → captured as Portal Decisions or events

#### 1.3.3 Specifications vs Running System

The specifications (SR-CONTRACT, SR-SPEC, SR-AGENTS) define the platform. Once built, the specifications become code:

- SR-CONTRACT invariants → enforcement logic in domain core
- SR-SPEC mechanics → implementation code
- SR-AGENTS constraints → agent authority checks

Users of the platform interact with the running system, not the specification documents.

---

## 2. Authority Model

### 2.1 Authority Kinds

| Kind | Meaning |
|------|---------|
| **content** | Binding meaning about invariants and semantics (when normative) |
| **process** | Binding meaning about procedures (when normative) |
| **record** | Durable, attributable records; may be binding or non-binding |
| **config** | Configuration affecting behavior; not a trust-boundary authority |
| **index** | Navigation only; no binding semantics |

### 2.2 Normative Status Classes

| Status | Meaning |
|--------|---------|
| **normative** | Binding requirements |
| **directional** | Guidance; not binding unless referenced by normative requirement |
| **index** | Navigation; not binding |
| **record** | Durable record of what happened/was decided |
| **evidence** | Oracle output; not authority |

#### Mapping note: authority_kind × normative_status

These are orthogonal classification axes:
- **authority_kind** describes *what kind of authority* the artifact could carry (content, process, record, config, index)
- **normative_status** describes *binding-ness / evidentiary role* (normative, directional, index, record, evidence)

For example, a `governance.plan` has `authority_kind: process` (it governs process) and `normative_status: normative` (it is binding for build-execution). An `evidence_bundle` has `authority_kind: record` (it is a durable record) and `normative_status: evidence` (it carries oracle output, not authority).

#### Terminology normalization: Portal

**Canonical term:** `portal` (lowercase in schemas/code; "Portal" in prose)
**Aliases:** "trust boundary", "authority port" (deprecated; retained for backward compatibility with SR-CHARTER)

A Portal is a gate requiring human arbitration; every Portal is a trust boundary whose recorded output is binding. Portal crossings produce Approval records. See SR-CONTRACT §2.6 for the authoritative definition.

### 2.3 Binding Precedence and Decision Routing

**Precedence is decision-type scoped.** Do not interpret this as “which document is truer.” Interpret it as: *which document governs this kind of decision.*

#### A) Platform-definition precedence (meaning / invariants / mechanics)

Use this scope when the question is about **platform meaning**, **binding invariants**, or **mechanics**.

1. **SR-CONTRACT** (binding invariants; what must always be true)
2. **SR-SPEC** (binding mechanics; how invariants are realized/enforced)
3. **SR-TYPES** (binding vocabulary and typing; canonical terms)
4. **SR-WORK-SURFACE** (work surface schemas and procedure templates)
5. **SR-SEMANTIC-ORACLE-SPEC** (semantic oracle interfaces and evidence schemas)
6. **SR-EVENT-MANAGER** (event store and projection mechanics)
7. **SR-TEMPLATES** (template definitions, stage mechanics, and configuration registry)
8. **SR-AGENT-WORKER-CONTRACT** (agent-platform interface contract)
9. **SR-GUIDE** / **SR-INTENT** / **SR-MODEL** (non-binding: usage guidance, rationale, conceptual models)

#### B) Build-execution precedence (agent behavior / process / gating)

Use this scope when the question is about **agent behavior**, **work process**, **gating**, or **execution policy** during the build.

1. **SR-CONTRACT** (still highest: invariants may never be violated)
2. **SR-AGENTS** (agent constraints; operating model)
3. **SR-AGENT-WORKER-CONTRACT** (agent-platform interface; worker integration patterns)
4. **SR-CHANGE** (routing/process for governed changes; not semantic truth)
5. **SR-DIRECTIVE** (execution configuration: gates, budgets, stop triggers)
6. **SR-PLAN** (decomposition and deliverables)
7. **Task-local instructions** (issue/PR-specific instructions)

#### Routing rule

- If it’s **meaning / invariants / mechanics**, use **Platform-definition precedence**.
- If it’s **agent behavior / work process / gating**, use **Build-execution precedence**.
- If still ambiguous: **escalate to Human Authority**.

---


## 3. Metadata Standard

### 3.1 Required YAML Frontmatter

Every governed markdown artifact MUST include:

- `solver_ralph.schema` — current: `"solver-ralph.artifact-metadata/v1"`
- `solver_ralph.id` — stable ID
- `solver_ralph.type` — type key from this registry
- `solver_ralph.title`
- `solver_ralph.version` — SemVer string
- `solver_ralph.status` — enum: `draft | governed | superseded | deprecated | archived`
- `solver_ralph.normative_status` — enum: `normative | directional | index | record | evidence`
- `doc_id` — canonical document identifier (e.g., `SR-SPEC`)
- `doc_kind` — document kind/type key (e.g., `governance.platform_spec`)
- `layer` — `platform | build | usage`
- `status` — `draft | stable | deprecated`
- `refs[]` — relationship records: `{rel, to}` (e.g., `depends_on`, `governed_by`, `informs`)


### 3.2 Extension Namespace (`ext`)

Type-specific fields go under `solver_ralph.ext.<type_key_shortname>`.

---

## 4. Type Registry

### 4.1 Platform Specification Types

These types define the platform. They are used during Layer 1 (building) and become code in Layer 2.

| Type Key | Authority Kind | Normative Status | Purpose |
|---|---|---|---|
| `governance.charter` | content | normative | Project charter; scope, milestones, authority model (SR-CHARTER) |
| `governance.types` | process | normative | This document; type registry |
| `governance.arch_contract` | content | normative | Binding invariants (SR-CONTRACT) |
| `governance.technical_spec` | content | normative | Binding mechanics (SR-SPEC) |
| `governance.agents` | process | normative | Agent constraints (SR-AGENTS) |
| `governance.work_surface_spec` | content | normative | Work Surface schemas: Intake, Template, Work Surface Instance (SR-WORK-SURFACE) |
| `governance.templates_registry` | content | normative | User configuration registry; templates, schemas, config artifacts (SR-TEMPLATES) |
| `governance.design_intent` | content | directional | Design rationale (SR-INTENT) |
| `governance.usage_guide` | content | directional | Platform usage guide (SR-GUIDE) |
| `governance.readme` | index | index | Navigation and orientation |

### 4.2 Build Scaffolding Types (Layer 1)

These types support building the platform. They are not part of the running platform.

| Type Key | Authority Kind | Normative Status | Purpose |
|---|---|---|---|
| `governance.plan` | process | normative | Build plan (SR-PLAN) — Layer 1 scaffolding |
| `governance.dev_directive` | process | normative | Build execution governance (SR-DIRECTIVE) |
| `governance.change_mgmt` | process | normative | Specification change control (SR-CHANGE) |
| `record.freeze` | record | record | Baseline snapshot during build |

**Note:** These types are useful for building SOLVER-Ralph but are not intrinsic to the platform's runtime behavior.

### 4.3 Platform Domain Types (Layer 2)

These types are intrinsic to the running platform. The platform tracks and manages these at runtime.

| Type Key | Authority Kind | Normative Status | What it represents |
|---|---|---|---|
| `domain.work_unit` | record | record | State machine for tracked work (coding or semantic) |
| `domain.work_surface` | record | record | Intake + Procedure context + stage parameters for a work unit |
| `domain.candidate` | record | record | Content-addressed work snapshot (commit or manifest) |
| `domain.evidence_bundle` | record | evidence | Oracle evidence bundle for a candidate (incl. semantic measurements) |
| `domain.portal_decision` | record | record | Human authorization record |
| `domain.loop_record` | record | record | Ralph-loop iteration summary |
| `domain.event` | record | record | Immutable state change |

**Schemas for these types are defined in SR-SPEC.**

### 4.4 Operational Record Types

These record types support platform operation.

| Type Key | Authority Kind | Normative Status | Purpose |
|---|---|---|---|
| `record.decision` | record | record | Binding human decision |
| `record.waiver` | record | record | Exception for Verified-with-Exceptions |
| `record.deviation` | record | record | Exception from governed requirement |
| `record.deferral` | record | record | Binding postponement |
| `record.evaluation_note` | record | record | Human evaluation of verification evidence |
| `record.assessment_note` | record | record | Human assessment of validation evidence |
| `record.intervention_note` | record | record | Human intervention note |
| `record.intake` | record | record | Structured intake for a work unit (objective, scope, constraints) |
| `record.procedure_instance` | record | record | Work-unit-specific binding of a procedure template and stage state |
| `record.attachment` | record | record | Human-uploaded supporting file (does NOT satisfy C-VER-1) |

### 4.5 Configuration Types

| Type Key | Authority Kind | Normative Status | Purpose |
|---|---|---|---|
| `config.agent_definition` | config | directional | Agent capability profiles |
| `config.oracle_definition` | config | directional | Oracle configurations |
| `config.portal_definition` | config | directional | Portal configurations |
| `config.procedure_template` | config | record | Stage-gated procedure template definitions (work-surface kits) |
| `config.semantic_set` | config | record | Meaning-matrix / semantic set definitions used by semantic oracles (versioned) |
| `config.semantic_profile` | config | record | Mapping from work kinds/stages → required semantic oracle suites + thresholds |

### 4.6 Deprecated Type Keys

| Type Key | Status | Notes |
|----------|--------|-------|
| `governance.development_paradigm` | Superseded | Use `governance.usage_guide` |
| `governance.authoring_guide` | Superseded | Use `governance.usage_guide` |
| `governance.epistemic_trust_topology` | Removed | Content absorbed into SR-INTENT, SR-CONTRACT, SR-SPEC |
| `record.problem_statement` | Removed | Methodology artifact; users define their own input formats |

---

## 5. Governance Document Templates

### 5.1 README

**Index note:** This template is for repository `README.md` as a navigation/index artifact (`normative_status: index`). The canonical usage guide is **SR-GUIDE**. There is no separate SR-README governed artifact in the current canonical set unless reintroduced via SR-CHANGE.

**Purpose:** Navigation and orientation.

**Template:**
```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-GUIDE"
  type: "governance.readme"
  title: "SOLVER-Ralph README"
  status: "governed"
  normative_status: "index"
  authority_kind: "index"
  governed_by: ["SR-CHANGE"]
  created: "YYYY-MM-DD"
  updated: "YYYY-MM-DD"
---
# SOLVER-Ralph README
```

### 5.2 Design Intent

**Purpose:** Design rationale; explains *why* the platform works this way.

**Template:**
```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-INTENT"
  type: "governance.design_intent"
  title: "SOLVER-Ralph Design Intent"
  status: "governed"
  normative_status: "directional"
  authority_kind: "content"
  governed_by: ["SR-CHANGE"]
  created: "YYYY-MM-DD"
  updated: "YYYY-MM-DD"
  ext:
    design_intent:
      primary_problem: ""
      assumptions: []
---
# SOLVER-Ralph Design Intent
```

### 5.3 Architectural Contract

**Purpose:** Binding invariants the platform enforces.

**Template:**
```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-CONTRACT"
  type: "governance.arch_contract"
  title: "SOLVER-Ralph Architectural Contract"
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
---
# SOLVER-Ralph Architectural Contract
```

### 5.4 Technical Specification

**Purpose:** Binding mechanics, schemas, state machines.

**Template:** See SR-SPEC.

### 5.5 Agents

**Purpose:** Agent object model and authority constraints.

**Template:**
```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-AGENTS"
  type: "governance.agents"
  title: "SOLVER-Ralph Agents"
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
---
# SOLVER-Ralph Agents
```

### 5.6 Usage Guide

**Purpose:** How to use the platform.

**Template:**
```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-GUIDE"
  type: "governance.usage_guide"
  title: "SOLVER-Ralph Usage Guide"
  status: "governed"
  normative_status: "directional"
  authority_kind: "content"
  governed_by: ["SR-CHANGE"]
  created: "YYYY-MM-DD"
  updated: "YYYY-MM-DD"
  ext:
    usage_guide:
      purpose: ""
      core_abstraction: "ralph-loop"
---
# SOLVER-Ralph Usage Guide
```

---

## 6. Build Scaffolding Templates (Build-execution)

These templates support building SOLVER-Ralph. They are not part of the platform itself.

### 6.1 Freeze Record

**Purpose:** Baseline snapshot during build.

**Template:**
```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-FREEZE-{BASELINE_ID}"
  type: "record.freeze"
  title: "Specification Freeze Record"
  status: "governed"
  normative_status: "record"
  authority_kind: "record"
  governed_by: ["SR-CHANGE"]
  created: "YYYY-MM-DD"
  updated: "YYYY-MM-DD"
  ext:
    freeze:
      baseline_id: "{baseline-id}"
      freeze_status: "FROZEN|UNFROZEN"
---
# Specification Freeze Record

**Freeze Status:** {FROZEN/UNFROZEN}
**Baseline Identifier:** `{baseline-id}`

## Document Manifest
| Document | Version | SHA-256 |
|---|---|---|
```

### 6.2 Development Directive

**Purpose:** Build execution governance.

**Template:**
```markdown
---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-DIRECTIVE"
  type: "governance.dev_directive"
  title: "Development Directive"
  status: "governed"
  normative_status: "normative"
  authority_kind: "process"
  governed_by: ["SR-CHANGE"]
  created: "YYYY-MM-DD"
  updated: "YYYY-MM-DD"
---
# Development Directive
```

### 6.3 Change Management

**Purpose:** Specification change control.

**Template:** See SR-CHANGE.

---

## 7. Platform Domain Type Schemas

Schemas for platform domain types (§4.3) are defined in **SR-SPEC**.

This section provides overview only.

### 7.1 Work Unit

A work unit tracks the state of a piece of work through the platform. A work unit may be *coding work* or *semantic knowledge work*.

In semantic work, a work unit is executed via a **Work Surface** (Intake + Procedure Template + stage parameters) and progresses through declared procedure stages.

**States (coarse):** DRAFT → ACTIVE → BLOCKED → COMPLETE (or REJECTED)

**Stage model (semantic work):**
- `procedure_template_ref` — binding reference to the procedure template
- `current_stage_id` — stage currently targeted
- `stage_status` — map `{stage_id -> {state, last_evidence_ref}}` (implementation may be projection-derived)

**Key fields:**
- `id` — stable identifier
- `state` — current coarse state
- `work_kind` — kind of semantic work (e.g., research_memo, decision_record, ontology_build)
- `work_unit` — work unit identifier (defaults to `id` if not explicitly provided)
- `work_surface_id` — identifier of the bound Work Surface, if any (populated when Loop is created with explicit `work_unit` that has an active Work Surface; enables iteration context inheritance)
- `work_surface_ref` — reference to the work surface / procedure instance (incl. intake)
- `depends_on` — prerequisite work units (semantic dependency ordering)
- `constraints` — scope, budget, tools, stop triggers
- `evidence_refs` — linked evidence bundles (including stage-scoped)
- `portal_refs` — linked portal decisions

### 7.2 Candidate

A content-addressed snapshot of work output. Candidates are the unit of verification and approval.

**Key fields:**
- `id` — content-addressed identifier (MUST include `sha256:`; MAY include `git:`)
- `work_unit_ref` — which work unit produced/updated this
- `content_hash` — SHA-256 of the candidate identity basis (commit hash or canonical manifest hash)
- `candidate_manifest_ref` — optional reference to a manifest listing included artifacts with per-file hashes and media types
- `created_at` — timestamp

### 7.3 Evidence Bundle

Oracle verification output. Canonical type: `domain.evidence_bundle` (see SR-CONTRACT §2.11 for terminology mapping).

**Key fields:**
- `id` — content-addressed identifier
- `oracle_suite_id` — suite identity
- `oracle_suite_hash` — suite hash (must incorporate semantic set definitions when applicable)
- `candidate_ref` — candidate being evaluated
- `procedure_template_ref` — procedure template under which evaluation occurred (semantic work)
- `stage_id` — stage evaluated (semantic work)
- `results[]` — per-oracle results (PASS/FAIL/ERROR) and structured measurement records
- `environment` — execution environment fingerprint
- `produced_at` — timestamp

### 7.4 Portal Decision

Human authorization record.

**Key fields:**
- `id` — stable identifier
- `portal_id` — which portal
- `subject_ref` — what was decided
- `decision` — APPROVE / REJECT / DEFER / etc.
- `actor_id` — who decided
- `rationale` — optional explanation
- `timestamp`

### 7.5 Loop Record

Summary of a semantic-ralph-loop iteration.

**Naming normalization:** The canonical surface form is `semantic-ralph-loop` (hyphenated, lowercase). Acceptable aliases: "Semantic Ralph Loop" (prose), "Ralph Loop" (informal). See SR-CONTRACT §2.3 and §2.11 for the full terminology mapping.

**Key fields:**
- `id` — stable identifier
- `work_unit_ref` — which work unit
- `started_at`, `ended_at` — timestamps
- `termination_reason` — COMPLETE / BUDGET_EXHAUSTED / PORTAL_BLOCKED / ERROR
- `outputs` — what was produced
- `events` — state changes during loop

### 7.6 Event

Immutable state change record.

**Key fields:**
- `id` — content-addressed or sequential
- `type` — event type (e.g., WorkUnitCreated, CandidateSubmitted)
- `timestamp`
- `payload` — event-specific data
- `actor` — who/what caused this

---



### 7.7 Work Surface

A Work Surface is the binding context for a semantic-ralph-loop iteration. Per SR-CONTRACT §2.3 and SR-CHARTER, a Work Surface comprises: (1) Intake, (2) Procedure Template, and (3) oracle profile/suites.

**Identifier format:** `ws:<ULID>`

**Key fields:**
- `work_surface_id` — unique identifier (format: `ws:<ULID>`)
- `work_unit_id` — the work unit this surface is bound to
- `intake_ref` — content-addressed reference to the bound Intake
- `procedure_template_ref` — content-addressed reference to the Procedure Template
- `current_stage_id` — current procedure stage (format: `stage:<NAME>`)
- `status` — lifecycle status: `active` | `completed` | `archived`
- `stage_status` — map `{stage_id -> StageStatusRecord}` tracking completion per stage
- `current_oracle_suites` — oracle suites resolved for current stage
- `params` — stage parameters (may include semantic set/version selectors)
- `content_hash` — hash of binding (intake_ref + template_ref + params)
- `bound_at` — timestamp of binding
- `bound_by` — actor who bound (per C-EVT-1 attribution)
- `completed_at` — timestamp of completion (if completed)
- `archived_at` — timestamp of archival (if archived)
- `archived_by` — actor who archived (if archived)

**WorkSurfaceStatus enum:**
- `active` — currently in progress
- `completed` — terminal stage passed
- `archived` — superseded or abandoned

**StageCompletionStatus enum:**
- `pending` — stage not yet entered
- `entered` — currently in stage
- `completed` — stage gate passed
- `skipped` — stage skipped per procedure rules

**StageStatusRecord:**
- `stage_id` — stage identifier
- `status` — StageCompletionStatus
- `entered_at` — when stage was entered (if entered)
- `completed_at` — when stage was completed (if completed)
- `evidence_bundle_ref` — evidence proving gate passage (if completed)
- `iteration_count` — number of iterations in this stage

### 7.8 Procedure Template

A stage-gated procedure definition used to generate candidates and define oracle checkpoints.

**Key fields:**
- `id`
- `stages[]` with `stage_id`, required outputs, required oracle suites, and gate criteria
- `transition_rules` (how stages advance)

### 7.9 Semantic Set

A versioned meaning-matrix / semantic-set definition used by semantic oracle suites.

**Key fields:**
- `id`
- `version` / `content_hash`
- `stage_id` applicability (optional)
- `axes` / ontology bindings (opaque to the platform; interpreted by the oracle implementation)
- `decision_rules` (how structured measurements map to PASS/FAIL, if applicable)

## 8. Final Notes

This type system distinguishes:
- **Platform types** (what the running system tracks)
- **Specification types** (how we define the platform)
- **Scaffolding types** (how we build the platform)
- **User types** (what users define for their work)

Adding new type keys is permitted, but new types MUST NOT be used to introduce authority claims that bypass the platform's trust boundaries.


## Appendix A: Layer Mapping (Non-binding)

This appendix preserves the Layer 1/2/3 narrative as a **human-friendly mapping**. It is **not** an enforcement boundary and must not be used to justify semantic changes outside the owning document kind.

```
┌─────────────────────────────────────────────────────────────────┐
│  LAYER 1: Building SOLVER-Ralph (scaffolding)                   │
│                                                                  │
│  Types used to build the platform:                               │
│  - governance.plan (SR-PLAN) — our build plan                    │
│  - governance.dev_directive (SR-DIRECTIVE) — build execution     │
│  - governance.change_mgmt (SR-CHANGE) — spec evolution           │
│  - record.freeze — baseline snapshots during build               │
│                                                                  │
│  These are scaffolding. They help us build; they are not the     │
│  platform itself.                                                │
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ produces
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  LAYER 2: The Platform (domain model)                           │
│                                                                  │
│  Types intrinsic to the running platform:                        │
│  - domain.work_unit — state machine for tracked work             │
│  - domain.candidate — content-addressed work snapshot            │
│  - domain.evidence_bundle — oracle verification output           │
│  - domain.portal_decision — human authorization record           │
│  - domain.loop_record — ralph-loop iteration summary             │
│  - domain.event — state change record                            │
│                                                                  │
│  These are what the platform tracks at runtime.                  │
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ enables
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  LAYER 3: Usage (user-defined)                                  │
│                                                                  │
│  Users may define their own types for:                           │
│  - Work products (what agents produce)                           │
│  - Input formats (how work is specified)                         │
│  - Custom records (domain-specific tracking)                     │
│                                                                  │
│  The platform is agnostic to user-defined types.                 │
└─────────────────────────────────────────────────────────────────┘
```


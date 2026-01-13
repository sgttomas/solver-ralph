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

## 1. Foundations

### 1.0 Three Layers

SOLVER-Ralph types exist across three layers:

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

**Key distinction:**
- **Platform types** (Layer 2) are what the running software tracks
- **Specification types** (Layer 1) define how we build the platform
- **User types** (Layer 3) are defined by users, not the platform

This document registers all three categories but distinguishes them clearly.

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

For the authoritative distinction, see **SR-CONTRACT**.

**Type-classification guidance:**
- Proposals are not typed artifacts—they are content that has not crossed a trust boundary
- Commitment objects include: Candidates, Evidence Bundles, Portal Decisions, Events

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

### 2.3 Binding Precedence and Decision Routing

**Precedence is decision-type scoped.** Do not interpret this as “which document is truer.” Interpret it as: *which document governs this kind of decision.*

#### A) Platform-definition precedence (meaning / invariants / mechanics)

Use this scope when the question is about **platform meaning**, **binding invariants**, or **mechanics**.

1. **SR-CONTRACT** (binding invariants; what must always be true)
2. **SR-SPEC** (binding mechanics; how invariants are realized/enforced)
3. **SR-TYPES** (binding vocabulary and typing; canonical terms)
4. **SR-GUIDE** (usage guidance; how to apply governed artifacts)
5. **SR-INTENT** (rationale; directional only when conflicting)

#### B) Build-execution precedence (agent behavior / process / gating)

Use this scope when the question is about **agent behavior**, **work process**, **gating**, or **execution policy** during the build.

1. **SR-CONTRACT** (still highest: invariants may never be violated)
2. **SR-AGENTS** (agent constraints; operating model)
3. **SR-CHANGE** (routing/process for governed changes; not semantic truth)
4. **SR-DIRECTIVE** (execution configuration: gates, budgets, stop triggers)
5. **SR-PLAN** (decomposition and deliverables)
6. **Task-local instructions** (issue/PR-specific instructions)

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
| `governance.types` | process | normative | This document; type registry |
| `governance.arch_contract` | content | normative | Binding invariants (SR-CONTRACT) |
| `governance.technical_spec` | content | normative | Binding mechanics (SR-SPEC) |
| `governance.agents` | process | normative | Agent constraints (SR-AGENTS) |
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
| `domain.work_unit` | record | record | State machine for tracked work |
| `domain.candidate` | record | record | Content-addressed work snapshot |
| `domain.evidence_bundle` | record | evidence | Oracle verification output |
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

### 4.5 Configuration Types

| Type Key | Authority Kind | Normative Status | Purpose |
|---|---|---|---|
| `config.agent_definition` | config | directional | Agent capability profiles |
| `config.oracle_definition` | config | directional | Oracle configurations |
| `config.portal_definition` | config | directional | Portal configurations |

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

## 6. Build Scaffolding Templates (Layer 1)

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

A work unit tracks the state of a piece of work through the platform.

**States:** DRAFT → SUBMITTED → VERIFIED → APPROVED → COMPLETE (or REJECTED)

**Key fields:**
- `id` — stable identifier
- `state` — current state
- `instructions` — what to produce
- `constraints` — scope, budget, tools
- `evidence_refs` — linked evidence bundles
- `portal_refs` — linked portal decisions

### 7.2 Candidate

A content-addressed snapshot of work output.

**Key fields:**
- `id` — content-addressed identifier
- `work_unit_ref` — which work unit produced this
- `content_hash` — SHA-256 of contents
- `created_at` — timestamp

### 7.3 Evidence Bundle

Oracle verification output.

**Key fields:**
- `id` — content-addressed identifier
- `oracle_id` — which oracle produced this
- `subject_ref` — what was verified (candidate ref)
- `result` — PASS / FAIL / ERROR
- `details` — structured output
- `environment` — execution environment fingerprint

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

Summary of a ralph-loop iteration.

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

## 8. Final Notes

This type system distinguishes:
- **Platform types** (what the running system tracks)
- **Specification types** (how we define the platform)
- **Scaffolding types** (how we build the platform)
- **User types** (what users define for their work)

Adding new type keys is permitted, but new types MUST NOT be used to introduce authority claims that bypass the platform's trust boundaries.
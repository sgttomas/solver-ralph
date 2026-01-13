---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-TYPE-COHERENCE-PLAN"
  type: "record.problem_statement"
  title: "Type Coherence Remediation Plan"
  version: "1.0.0"
  status: "draft"
  normative_status: "record"
  authority_kind: "record"
  governed_by: ["SR-CHANGE"]
  created: "2026-01-12"
  updated: "2026-01-12"
  ext:
    problem_statement:
      stage: "SAPS"
      scope:
        program_id: "SOLVER-Ralph-governance-tightening"
---

# Type Coherence Remediation Plan

## Executive Summary

This plan addresses systematic type blurring across the SOLVER-Ralph governed document set. The root cause is a misleading "paradigm vs instance" conceptual distinction that created ambiguity about document scope and definitional ownership. The remediation involves:

1. Retiring the "paradigm vs instance" vocabulary
2. Establishing single-source definitional ownership for all key concepts
3. Removing duplicated definitions across documents
4. Clarifying each document's type and enforcing that type's constraints
5. Removing normative content from directional documents

---

## Part 1: Conceptual Clarification

### 1.1 Problem Statement

The governed set currently uses "paradigm" and "instance" as if they describe different document layers:

- "Paradigm-level binding invariants" (SR-CONTRACT §2.10)
- "The paradigm is the governed set" (SR-TYPES §1.3.4)
- "Paradigm defines deterministic validity rules" (SR-PARADIGM §0.5)
- "Instance selection is performed by the governed set" (SR-CONTRACT §2.10)

This framing implies there could be multiple "instances" of the SOLVER-Ralph "paradigm" — e.g., one using Rust/PostgreSQL, another using Java/MongoDB. But this is not the actual architecture:

- The governed set specifies *one system*
- SR-SPEC's stack choices (Rust, PostgreSQL, MinIO, NATS, Zitadel) are the specification, not instance configuration
- There is no abstract "SOLVER-Ralph paradigm" independent of this specification

### 1.2 Resolution

**Retire "paradigm vs instance" as a document-layer concept.**

Replace with clearer scoping vocabulary:

| Old Term | New Term | Meaning |
|----------|----------|---------|
| "paradigm-level" | "archetype-level" or simply "governed" | Defined in the governed set |
| "instance-level" | "deployment-level" or "runtime" | Configured at deployment or determined at runtime |
| "paradigm" (noun) | "governed set" or "governance framework" | The collection of SR-* documents |
| "instance" (noun) | "deployment" or "running system" | The software enforcing the governed set |

**Exception:** SR-PLAN may still use "instance" to refer to a specific plan (e.g., "SR-PLAN-instance-1") because plans are inherently specific to a project/scope. This is a different usage — "an instance of a plan" rather than "instance vs paradigm."

### 1.3 Documents Affected

The following documents contain "paradigm vs instance" language requiring update:

| Document | Sections Affected |
|----------|-------------------|
| SR-TYPES | §1.3.4 "Paradigm vs Instance" |
| SR-CONTRACT | §2.10 "Paradigm vs Instance" |
| SR-INTENT | §3.3 (references to "paradigm"), §5.9 "Merging paradigm and instance" |
| SR-PARADIGM | §0.5, §1.1, throughout (document is named "Development Paradigm") |
| SR-SPEC | §4.7 (references to "paradigm") |
| SR-ETT | §7 "Role in the governed stack" |

---

## Part 2: Definitional Ownership

### 2.1 Problem Statement

Key concepts are defined in multiple documents with subtle variations, creating ambiguity about authoritative meaning:

| Concept | Currently Defined In |
|---------|---------------------|
| Verified / Approved / Shippable | SR-CONTRACT §2.1, SR-TYPES §1.2, SR-INTENT §7, SR-SPEC §1.2 |
| Proposal vs Commitment Object | SR-CONTRACT §2.8, SR-TYPES §1.3.1, SR-SPEC §1.2.1, SR-ETT §2.1 |
| Verification / Evaluation / Validation / Assessment | SR-CONTRACT §2.9, SR-TYPES §1.3.2, SR-SPEC §1.2.2 |
| Agent (as actor kind) | SR-CONTRACT §2.2, SR-AGENTS §1.2, SR-TYPES §1.1 |
| Membrane | SR-ETT §2.2, §4.*, SR-AGENTS §4.3 |
| Freeze Record contents | SR-CONTRACT §2.6, SR-TYPES §6.1, SR-CHANGE §5, SR-SPEC (implicit) |

### 2.2 Resolution: Single-Source Ownership Model

Establish the following definitional ownership:

| Concept Domain | Owning Document | Other Documents May... |
|----------------|-----------------|------------------------|
| **Core trust/evidence semantics** (Verified, Approved, Shippable, Proposal, Commitment Object, Verification/Evaluation/Validation/Assessment) | **SR-CONTRACT** | Reference by section number; no redefinition |
| **Type keys and artifact taxonomy** (what types exist, metadata schema, extension shapes) | **SR-TYPES** | Reference type keys; no redefinition |
| **Event/record schemas** (what fields, what constraints) | **SR-SPEC** | Reference schemas; no redefinition |
| **Agent ontology** (what is an agent, actor kinds) | **SR-AGENTS** | Reference; SR-CONTRACT may state constraints on agents |
| **Membrane topology** (what membranes exist, their modalities) | **SR-ETT** | Reference membrane IDs; no redefinition |
| **Change routing** (what requires change control, portal functions) | **SR-CHANGE** | Reference; no redefinition |
| **Authoring workflow** (stages, SAPS structure) | **SR-PARADIGM** | Reference; no redefinition |
| **Design rationale** (why decisions were made) | **SR-INTENT** | N/A — this is the only rationale document |

### 2.3 Cross-Reference Protocol

When a document needs to use a concept owned elsewhere:

**Pattern A: Inline reference (for occasional use)**
```markdown
A **Candidate** (see SR-CONTRACT §2.3) is registered when...
```

**Pattern B: Terminology section with explicit delegation (for frequent use)**
```markdown
### 1.2 Terminology

This document uses terms defined normatively in other governed artifacts:

- **Candidate**, **Run**, **Evidence Bundle**: see SR-CONTRACT §2.3, §2.6
- **Verified**, **Approved**, **Shippable**: see SR-CONTRACT §2.1
- **Proposal**, **Commitment Object**: see SR-CONTRACT §2.8

These definitions are not repeated here. If apparent conflicts exist, SR-CONTRACT governs.
```

**Anti-pattern (remove throughout):**
```markdown
### 1.2 Terminology

- **Candidate:** a content-addressable snapshot of work products...
[Full redefinition that duplicates SR-CONTRACT]
```

---

## Part 3: Document-by-Document Remediation

### 3.1 SR-TYPES

**Current type:** `governance.types`
**Declared authority:** `process`
**Declared normative status:** `normative`

**Assessment:** Type is appropriate. The document correctly registers type keys and provides templates.

**Required Changes:**

| Section | Current State | Required Change | Rationale |
|---------|---------------|-----------------|-----------|
| §1.1 "The Ontology Baseline" | Defines Verified, Approved, Shippable, Agent, etc. | Remove definitions; replace with delegation statement pointing to SR-CONTRACT | SR-CONTRACT owns these |
| §1.2 "Epistemology" | Defines Verified/Approved/Shippable again | Remove; add cross-reference to SR-CONTRACT §2.1 | Duplication |
| §1.3.1 "Proposal vs Commitment Object" | Full definition | Remove; add cross-reference to SR-CONTRACT §2.8 | SR-CONTRACT owns this |
| §1.3.2 "Verification, Evaluation, Validation, Assessment" | Full definition | Remove; add cross-reference to SR-CONTRACT §2.9 | SR-CONTRACT owns this |
| §1.3.3 "Problem Statement Family" | Defines RPS/IPS/SAPS/INPS | **Keep** — this is type-key-specific content appropriate to SR-TYPES | Appropriate here |
| §1.3.4 "Paradigm vs Instance" | Defines paradigm/instance distinction | Remove entirely; replace with clarifying note about "governed set" vs "deployment" | Root cause of confusion |
| §6.1 "Freeze Record" | Provides template | Keep template; remove any definitional prose that duplicates SR-CONTRACT or SR-CHANGE | Templates belong here |
| §6.2 "Problem Statement Record" | Provides template | Keep | Appropriate here |

**New Section to Add:**

```markdown
### 1.0 Definitional Delegation

SR-TYPES registers type keys and provides templates. It does not define the semantics of:

- Trust and evidence terms (Verified, Approved, Shippable, Proposal, Commitment Object) — see **SR-CONTRACT §2**
- Verification/Evaluation/Validation/Assessment — see **SR-CONTRACT §2.9**
- Actor kinds and agent constraints — see **SR-AGENTS §1**, **SR-CONTRACT §4**
- Membrane topology — see **SR-ETT §4**

If a type key description in this document appears to conflict with its owning document, the owning document governs.
```

---

### 3.2 SR-INTENT

**Current type:** `governance.design_intent`
**Declared authority:** `content`
**Declared normative status:** `directional`

**Assessment:** Type is appropriate, but `authority_kind: content` is questionable for a directional/rationale document. Should be `authority_kind: rationale` or similar — but since that's not in the current authority_kind enum, accept `content` with a note.

**Required Changes:**

| Section | Current State | Required Change | Rationale |
|---------|---------------|-----------------|-----------|
| §3.3 "The Deterministic State Model" | Contains assertive statements ("Rules are deterministic", "Validity is system-enforced") that read as binding constraints | Reframe as rationale: "We chose deterministic rules because..." rather than asserting the rules themselves | Binding constraints belong in SR-CONTRACT |
| §5.9 "Merging paradigm and instance" | Uses paradigm/instance vocabulary | Reframe as: "Rejected: Merging governance and implementation concerns" with updated explanation | Vocabulary retirement |
| §7 "Terminology" | Defines Verified, Approved, Shippable, Oracle, Portal, Deviation, Deferral, Waiver, Decision, Proposal, Commitment Object, SAPS, Paradigm, Instance | Remove definitions entirely; replace with: "For term definitions, see SR-CONTRACT §2 and SR-TYPES §1" | Duplication; directional docs should not define terms |
| Throughout | References to "paradigm" | Review each usage; replace with "governed set" or "governance framework" where appropriate | Vocabulary retirement |
| §3.3 state machine diagram | Uses "PARADIGM" as a label | Replace with "GOVERNANCE RULES" or similar | Vocabulary retirement |

**Add to §0 or §8:**

```markdown
### Normative Posture

This document is **directional** (non-binding). It uses no normative keywords (MUST, SHALL, REQUIRED).

If any statement in this document appears to conflict with SR-CONTRACT or SR-SPEC, those documents govern. SR-INTENT explains *why*; it does not constrain *what*.
```

---

### 3.3 SR-CONTRACT

**Current type:** `governance.arch_contract`
**Declared authority:** `content`
**Declared normative status:** `normative`

**Assessment:** Type is appropriate. This is the correct home for binding invariants and authoritative definitions.

**Required Changes:**

| Section | Current State | Required Change | Rationale |
|---------|---------------|-----------------|-----------|
| §2.10 "Paradigm vs Instance" | Defines paradigm/instance distinction | Rewrite to clarify document scoping without the misleading two-level framing; see proposed text below | Root cause of confusion |
| §13 "Conformance Checklist" | Contains "Mechanics owner" column (SR-SPEC vs SR-DIRECTIVE) | Move the "Mechanics owner" column to SR-CHANGE as a routing reference, OR remove it and let documents self-declare | Meta-governance routing doesn't belong in the contract |
| §2.1–2.9 | Authoritative definitions | **Keep** — these are the canonical definitions | Appropriate here |
| C-* invariants | Binding requirements | **Keep** | Appropriate here |

**Proposed §2.10 Replacement:**

```markdown
### 2.10 Document Scoping

The SOLVER-Ralph governed set is a collection of documents that specify one system:

- **SR-CONTRACT** defines invariants (what must hold)
- **SR-SPEC** defines mechanics (how invariants are realized, including technology choices)
- **SR-DIRECTIVE** defines execution discipline (how work is sequenced and gated)
- **SR-CHANGE** defines evolution (how the governed set changes over time)

The term "deployment" refers to a running instance of the system. Deployments may have deployment-specific configuration (e.g., database connection strings, identity provider endpoints) but MUST comply with SR-SPEC mechanics.

The term "governed set" refers to the collection of SR-* documents as selected and versioned by SR-README.
```

---

### 3.4 SR-SPEC

**Current type:** `governance.technical_spec`
**Declared authority:** `content`
**Declared normative status:** `normative`

**Assessment:** Type is appropriate. Stack choices (Rust, PostgreSQL, etc.) are appropriate here — this is what a technical specification does.

**Required Changes:**

| Section | Current State | Required Change | Rationale |
|---------|---------------|-----------------|-----------|
| §1.2.1 "Proposal vs Commitment Object" | Full definition duplicating SR-CONTRACT §2.8 | Remove definition; replace with delegation statement | SR-CONTRACT owns this |
| §1.2.2 "Verification, Evaluation, Validation, Assessment" | Full definition with table duplicating SR-CONTRACT §2.9 | Remove; add cross-reference | SR-CONTRACT owns this |
| §1.2 "Terminology" | Mix of SR-SPEC-specific terms and duplicated terms | Keep SR-SPEC-specific terms (Candidate, Run, Evidence Bundle, etc. as they apply to the spec); add delegation statement for concepts owned by SR-CONTRACT | Partial duplication |
| §4.7 | References "paradigm" | Replace with "governed set" or remove | Vocabulary retirement |
| Frontmatter `ext.technical_spec.stack` | Lists technology choices | **Keep** — this is appropriate for a technical specification | No change needed |

**Proposed §1.2 Restructure:**

```markdown
### 1.2 Terminology

#### 1.2.1 Delegation to SR-CONTRACT

This specification implements concepts defined normatively in SR-CONTRACT §2:

- Candidate, Run, Evidence Bundle, Oracle Suite, Ralph Loop, Iteration (§2.3–2.4)
- Verified, Approved, Shippable (§2.1)
- Portal, Approval, Gate Waiver, Freeze Record (§2.6)
- Proposal, Commitment Object (§2.8)
- Verification, Evaluation, Validation, Assessment (§2.9)

Definitions are not repeated here. SR-CONTRACT governs if apparent conflicts exist.

#### 1.2.2 SR-SPEC-Specific Terms

The following terms are introduced or refined by this specification:

- **Event Envelope:** [definition specific to SR-SPEC event schema]
- **Evidence Manifest:** [definition specific to SR-SPEC evidence schema]
- **ContextCompiler:** [definition specific to SR-SPEC context compilation]
- ...
```

---

### 3.5 SR-CHANGE

**Current type:** `governance.change_policy`
**Declared authority:** `normative`
**Declared normative status:** `normative` (implied by authority_kind)

**Assessment:** Type is appropriate.

**Required Changes:**

| Section | Current State | Required Change | Rationale |
|---------|---------------|-----------------|-----------|
| §5 "Freeze policy" | Specifies freeze record contents | Keep policy constraints (what must be included); remove schema-level detail that duplicates SR-SPEC and SR-TYPES | Policy vs schema separation |
| §5.1 | "Freeze record's `artifact_manifest[]` MUST include..." | Keep as policy; reference SR-SPEC for schema | Appropriate here |
| §3.3 "Portal routing matrix" | Detailed routing table | Keep | Appropriate here |
| Throughout | No paradigm/instance vocabulary detected | No change | N/A |

**Consider Adding:**

A new section or appendix that captures the "Mechanics owner" information currently in SR-CONTRACT §13, if that routing information is removed from SR-CONTRACT.

---

### 3.6 SR-PARADIGM

**Current type:** `governance.development_paradigm`
**Declared authority:** `process`
**Declared normative status:** `normative`

**Assessment:** The document name includes "Paradigm" which is the problematic vocabulary. The content mixes authoring process definition with constraint duplication.

**Required Changes:**

| Section | Current State | Required Change | Rationale |
|---------|---------------|-----------------|-----------|
| Document title | "SOLVER-Ralph Development Paradigm" | Consider renaming to "SOLVER-Ralph Authoring Process" or "Governance Authoring Guide" | Vocabulary retirement |
| Frontmatter `type` | `governance.development_paradigm` | Consider changing to `governance.authoring_process` | Vocabulary retirement (optional — type key change requires SR-TYPES update) |
| §0.2 | Claims binding authority over "agent invariants" | Remove; add cross-reference to SR-AGENTS and SR-CONTRACT | SR-AGENTS owns agent ontology; SR-CONTRACT owns agent constraints |
| §0.3 | "SR-PARADIGM MUST be interpreted as compatible..." | Keep — this is appropriate meta-governance | Appropriate |
| §0.4 "Agents are not judges" | Restates agent constraints | Remove or convert to cross-reference: "Agent authority constraints are defined in SR-CONTRACT §C-TB-2 and SR-AGENTS §2" | Duplication |
| §0.5 | Uses "paradigm" extensively | Reframe using "governed set" / "authoring process" vocabulary | Vocabulary retirement |
| §1.3.4 table | Differentiates "Paradigm" vs "Instance" rows | Reframe as "Governance documents" vs "Running system" or similar | Vocabulary retirement |
| Appendix A "SAPS Minimum Field Set" | Defines required SAPS structure | Keep but consider whether this is a template (SR-TYPES territory) vs process requirement (SR-PARADIGM territory) | Boundary clarification |

**Proposed Title and Framing:**

```markdown
# SOLVER-Ralph Governance Authoring Process (SR-PARADIGM)

**Purpose:** Define the governed workflow that turns a human's problem statement into a 
structured, aligned problem statement (SAPS) and then into a coherent governed artifact set.

This is a **meta-governance artifact**: it governs how the governed set itself is authored.
```

---

### 3.7 SR-ETT

**Current type:** `governance.epistemic_trust_topology`
**Declared authority:** `process`
**Declared normative status:** `directional`

**Assessment:** Type is appropriate as a "lens" document. However, §8 contains normative (MUST) language which violates the declared `directional` status.

**Required Changes:**

| Section | Current State | Required Change | Rationale |
|---------|---------------|-----------------|-----------|
| §8 "Normative section (minimal)" | Contains three MUST/MUST NOT statements | Remove section entirely OR promote these constraints to SR-CONTRACT and remove from SR-ETT | Directional documents cannot contain normative constraints |
| §2.1 "Proposal vs Commitment Object" | References SR-CONTRACT but includes "Informal summary (non-binding)" | Keep the cross-reference; ensure the summary is clearly non-authoritative | Appropriate if framed correctly |
| §4.* Membrane definitions | Full definitions for 9 membranes | **Keep** — SR-ETT owns membrane definitions | Appropriate here |
| §6.1 "Agent capability checklist" | Operational guidance | Keep but ensure it's framed as guidance, not requirement | Appropriate for directional doc |
| §6.2 "SR-DIRECTIVE author checklist" | Operational guidance | Keep | Appropriate |
| §7 | References "paradigm" | Replace with "governed set" | Vocabulary retirement |

**Proposed §8 Replacement:**

Option A: Remove entirely (promote constraints to SR-CONTRACT)

Option B: Reframe as guidance:
```markdown
## 8. Boundaries of This Document

SR-ETT is a **design lens**, not a source of binding constraints.

The following principles are expressed as constraints in SR-CONTRACT and SR-CHANGE:

- Changes affecting binding meanings route through SR-CHANGE (see SR-CHANGE §1.1)
- Commitments crossing trust boundaries are attributable and recorded (see SR-CONTRACT §C-TB-3)
- Portals and binding terms are not redefined by supporting documents (see SR-CONTRACT §C-TB-4)

SR-ETT helps authors and implementers reason about *where* coercion belongs; 
SR-CONTRACT and SR-SPEC define *what* the coercion is.
```

---

### 3.8 SR-AGENTS

**Current type:** `governance.agents`
**Declared authority:** `process`
**Declared normative status:** `normative`

**Assessment:** Type is appropriate. This document should own the agent ontology (what is an agent). However, it currently duplicates membrane definitions from SR-ETT and states constraints that may belong in SR-CONTRACT.

**Required Changes:**

| Section | Current State | Required Change | Rationale |
|---------|---------------|-----------------|-----------|
| §1.2 "Definition: Agent" | Defines agent as actor kind | **Keep** — this is appropriate here | Appropriate |
| §2 "Epistemology" | Explains why agents cannot create truth | Keep as explanation; ensure it references SR-CONTRACT for binding constraints | Appropriate if framed correctly |
| §2.2 "Prohibition vs structural impossibility" | Good framing | Keep | Appropriate |
| §4.3 "Membrane → agent boundary map" | Table duplicating SR-ETT membrane definitions | Remove membrane definitions (columns 1-2); keep agent boundary information (columns 3-4) with cross-reference to SR-ETT for membrane definitions | SR-ETT owns membranes |
| §5 "Work Envelopes" | Describes variation without agent subtypes | Keep but consider whether this is configuration guidance (SR-DIRECTIVE territory) | Boundary clarification |
| Frontmatter `ext.agents.invariants` | Lists `agents_produce_proposals`, `agents_cannot_assert_verified`, etc. | Keep as index; ensure these are realized as constraints in SR-CONTRACT, not duplicated here as definitions | Index vs definition |
| Throughout | Uses "paradigm" | Replace with "governed set" where appropriate | Vocabulary retirement |

**Proposed §4.3 Restructure:**

```markdown
### 4.3 Agent Boundaries at SR-ETT Membranes

This table maps SR-ETT membranes (see SR-ETT §4) to agent-specific boundaries. 
Membrane definitions are owned by SR-ETT; this table specifies what the uniform 
AGENT actor kind may and may not do at each membrane.

| SR-ETT membrane_id | Agent boundary (uniform across all agents) | Primary enforcement surface |
|---|---|---|
| `intent_objective` | Agent may propose goals/plans, but **cannot instantiate or change the binding objective**... | Driving-port admission rules... |
| ... | ... | ... |
```

---

### 3.9 SR-PLAN (Template vs Instance)

**Current type:** `governance.plan`
**Current document:** SR-PLAN-instance-1 (an instance of a plan)

**Assessment:** There is confusion between the plan template (a governance artifact) and plan instances (project-specific artifacts). The current document appears to be an instance masquerading as a governance artifact.

**Required Changes:**

| Issue | Current State | Required Change | Rationale |
|-------|---------------|-----------------|-----------|
| Document identity | `id: SR-PLAN-instance-1`, `type: governance.plan` | Separate into two artifacts | Template vs instance |
| Frontmatter | `template: false`, `template_ref: "SR-PLAN@1.0.0-draft.2"` | The template should be a separate document | Clarity |
| Packages/Deliverables | PKG-01 through PKG-11, D-01 through D-36 | These belong in the instance, not a governance template | Separation of concerns |

**Proposed Separation:**

**Artifact 1: SR-PLAN (Template)**
- `id: SR-PLAN`
- `type: governance.plan_template`
- Contains: 
  - Required structure for a plan
  - Required metadata fields
  - Relationship to SAPS
  - Evidence hooks schema
  - Decomposition guidance
- Does NOT contain: Specific packages, deliverables, or dependencies

**Artifact 2: SR-PLAN-instance-1 (Instance)**
- `id: SR-PLAN-instance-1`
- `type: record.plan` (not `governance.*`)
- `template_ref: SR-PLAN@x.y.z`
- Contains: Specific packages, deliverables, dependencies for SOLVER-Ralph-instance-1
- Is a **record** (project artifact), not a governance document
- May or may not be in the governed set — it's execution data

**Implication for SR-TYPES:**

- Add type key: `governance.plan_template` (the reusable pattern)
- Add type key: `record.plan` or `execution.plan` (specific plan instances)
- Clarify that `governance.plan` is deprecated or an alias

---

## Part 4: Implementation Sequence

### Phase 1: Vocabulary and Framing (Low Risk)

These changes are primarily textual and don't affect binding semantics:

1. **Update SR-TYPES §1.3.4** — Remove "Paradigm vs Instance" section; add scoping clarification
2. **Update SR-CONTRACT §2.10** — Rewrite with new vocabulary
3. **Update SR-INTENT §5.9, §7** — Remove terminology section; update rejected alternatives
4. **Update SR-ETT §7, §8** — Remove normative section or convert to cross-references
5. **Update SR-PARADIGM title and framing** — Consider rename; update vocabulary throughout
6. **Review all documents** — Find/replace "paradigm" → "governed set" where appropriate

### Phase 2: Definitional Consolidation (Medium Risk)

These changes establish single-source ownership:

1. **Add delegation statements to SR-TYPES §1.0** — Clarify what SR-TYPES does and doesn't define
2. **Remove duplicate definitions from SR-TYPES §1.1–1.3** — Keep only type-key-specific content
3. **Remove duplicate definitions from SR-SPEC §1.2** — Restructure terminology section
4. **Remove duplicate definitions from SR-INTENT §7** — Convert to cross-references
5. **Restructure SR-AGENTS §4.3** — Remove membrane definitions; keep agent boundary map

### Phase 3: Structural Separation (Higher Risk)

These changes affect document structure and type keys:

1. **Separate SR-PLAN into template and instance** — Create two documents
2. **Update SR-TYPES** — Add new type keys for plan template vs plan instance
3. **Update SR-README** — Adjust governed set to reflect separation
4. **Remove "Mechanics owner" from SR-CONTRACT §13** — Move to SR-CHANGE or remove

### Phase 4: Normative Hygiene (Verification)

These are verification/audit tasks:

1. **Audit SR-INTENT** — Confirm no MUST/SHALL language remains
2. **Audit SR-ETT** — Confirm no MUST/SHALL language remains (or that §8 was migrated)
3. **Audit all cross-references** — Confirm references point to correct sections after restructuring
4. **Run coherence oracle** — Verify no definitional conflicts remain

---

## Part 5: Success Criteria

### 5.1 Definitional Uniqueness

For each core concept, there is exactly one document section that defines it:

| Concept | Single Owner | Other docs reference only |
|---------|--------------|---------------------------|
| Verified / Approved / Shippable | SR-CONTRACT §2.1 | ✓ |
| Proposal / Commitment Object | SR-CONTRACT §2.8 | ✓ |
| Verification / Evaluation / Validation / Assessment | SR-CONTRACT §2.9 | ✓ |
| Agent (actor kind) | SR-AGENTS §1.2 | ✓ |
| Membrane (topology) | SR-ETT §4 | ✓ |
| Type keys | SR-TYPES §4 | ✓ |

### 5.2 Normative Hygiene

- SR-INTENT contains zero MUST/SHALL statements
- SR-ETT contains zero MUST/SHALL statements (or §8 migrated to SR-CONTRACT)
- SR-PARADIGM's MUST/SHALL statements are limited to authoring process requirements, not redefinitions of concepts owned elsewhere

### 5.3 Vocabulary Consistency

- "Paradigm vs instance" framing is removed from all documents
- "Paradigm" as a noun is replaced with "governed set" or "governance framework"
- "Instance" is used only for specific instantiations (e.g., SR-PLAN-instance-1), not as a conceptual layer

### 5.4 Cross-Reference Integrity

- All cross-references use the pattern `SR-{DOC} §{section}` or `SR-{DOC} {C-*}`
- No broken references after restructuring
- Delegation statements are present where documents previously contained duplicate definitions

---

## Part 6: Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Breaking changes to SR-TYPES type keys | Medium | High | Treat as G:MAJOR change; ensure backward compatibility or explicit migration |
| Cross-reference breakage | High | Medium | Audit all references before finalizing; use stable section numbers |
| SR-PARADIGM rename causes confusion | Low | Low | If renaming, add prominent "formerly known as" note and update all references |
| Removing definitions causes ambiguity | Medium | Medium | Ensure delegation statements are clear; add "if conflict, X governs" statements |
| SR-PLAN separation causes governance gaps | Medium | Medium | Clearly define which artifact is governed vs execution data; update SR-README |

---

## Appendix A: Complete Vocabulary Mapping

| Term to Retire/Revise | Replacement | Documents Affected |
|-----------------------|-------------|-------------------|
| "paradigm-level" | "governed" or "archetype-level" | SR-CONTRACT, SR-TYPES, SR-INTENT, SR-PARADIGM |
| "instance-level" | "deployment-level" or "runtime" | SR-CONTRACT, SR-TYPES |
| "paradigm" (noun, as document layer) | "governed set" or "governance framework" | All documents |
| "instance" (noun, as document layer) | "deployment" or "running system" | SR-CONTRACT, SR-TYPES |
| "paradigm defines" | "the governed set specifies" | SR-PARADIGM, SR-INTENT |
| "the paradigm is" | "the governed set is" | SR-TYPES |

**Acceptable continued uses of "paradigm":**
- As part of the document name "SR-PARADIGM" (if not renamed)
- As part of the type key "governance.development_paradigm" (if not changed)
- In historical/philosophical discussion ("the paradigm shift in software governance")

**Acceptable continued uses of "instance":**
- SR-PLAN-instance-1 (a specific plan)
- "An instance of a loop" (a specific Ralph Loop)
- "Deploy an instance" (running software)

---

## Appendix B: Estimated Effort

| Phase | Documents | Estimated Changes | Complexity |
|-------|-----------|-------------------|------------|
| Phase 1: Vocabulary | 6 documents | ~50 text replacements, 3 section rewrites | Low |
| Phase 2: Definitions | 4 documents | ~8 section removals, 4 delegation statements | Medium |
| Phase 3: Structure | 3 documents (SR-PLAN, SR-TYPES, SR-README) | 1 document split, 2 type key additions | High |
| Phase 4: Verification | All | Audit passes | Low |

Total: Approximately 2-3 focused sessions to complete all phases.

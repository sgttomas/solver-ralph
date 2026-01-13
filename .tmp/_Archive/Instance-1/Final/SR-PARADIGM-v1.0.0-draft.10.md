---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-PARADIGM"
  type: "governance.development_paradigm"
  title: "SOLVER-Ralph Development Paradigm"
  version: "1.0.0-draft.10"
  status: "draft"
  normative_status: "normative"
  authority_kind: "process"
  governed_by: ["SR-CHANGE"]
  supersedes:
    - "SR-PARADIGM@1.0.0-draft.9"
  created: "2026-01-10"
  updated: "2026-01-11"
  tags:
    - "solver"
    - "solver-ralph"
    - "governance"
    - "development-paradigm"
    - "problem-statement"
    - "alignment"
    - "agentic-workflows"
    - "trustworthy-knowledge"
    - "documentation-as-infrastructure"
    - "coherence"
    - "meta-governance"
  ext:
    development_paradigm:
      intent:
        core_problem_statement: "How to generate trustworthy knowledge artifacts using non-deterministic agents."
        summary: >
          Meta-governance artifact defining how to translate a human problem statement into a
          Structured, Aligned Problem Statement (SAPS) and instantiate it into a coherent, selected
          governed artifact set (as indexed in SR-README), such that stochastic agent labor is converted
          into durable, attributable, replayable, evidence-backed commitments with explicit authority boundaries.
      boot_alignment_prerequisites:
        note: >
          SR-PARADIGM does not define the canonical governed set.
          Boot prerequisites are expressed as governance *categories* (type keys) that MUST be satisfied to boot a
          new SOLVER-Ralph instance (alignment substrate for humans + agents).
          The concrete artifacts/versions that satisfy these categories are indexed and selected in SR-README,
          and changes are governed by SR-CHANGE.
        required_categories:
          - id: "governance.readme"
            satisfies_by: ["SR-README"]
            purpose: "Canonical registry/index and current selection pointers."
          - id: "governance.types"
            satisfies_by: ["SR-TYPES"]
            purpose: "Forcing ontology, artifact taxonomy, metadata conventions, and conflict classification."
          - id: "governance.design_intent"
            satisfies_by: ["SR-INTENT"]
            purpose: "Directional purpose; interpretation guidance; failure-mode framing."
          - id: "governance.epistemic_trust_topology"
            satisfies_by: ["SR-ETT"]
            purpose: "Epistemic trust membranes/harnesses; coercion boundary placement rationale."
          - id: "governance.agents"
            satisfies_by: ["SR-AGENTS"]
            purpose: "Defines what an Agent is (as an object category) and the agent invariants/constraints used by the paradigm."
          - id: "governance.development_paradigm"
            satisfies_by: ["SR-PARADIGM"]
            purpose: "Problem-statement compilation workflow and recursive governance authoring model."
          - id: "governance.plan"
            satisfies_by: ["SR-PLAN"]
            purpose: "Typed decomposition compiler: SAPS → packages/deliverables/dependencies with evidence hooks (no execution closure semantics)."
          - id: "governance.arch_contract"
            satisfies_by: ["SR-CONTRACT"]
            purpose: "Binding invariants and authority boundary constraints."
          - id: "governance.technical_spec"
            satisfies_by: ["SR-SPEC"]
            purpose: "Binding mechanics (events/APIs/state machines) in the instantiated stack."
          - id: "governance.dev_directive"
            satisfies_by: ["SR-DIRECTIVE"]
            purpose: "Execution governance: sequencing/state progression rules; gates/portals/budgets; verification profiles."
          - id: "governance.change_mgmt"
            satisfies_by: ["SR-CHANGE"]
            purpose: "Change control, selection/freeze, exception routing, and reconciliation workflow."

      workflow_stages:
        - "C0_classification"
        - "S0_alignment"
        - "S1_types"
        - "S2_intent"
        - "S3_architecture_and_stack_research"
        - "S4_types_iteration"
        - "S5_intent_iteration"
        - "S6_contract"
        - "S7_spec"
        - "S8_directive"
        - "S9_change"
        - "S10_recursive_iteration"
        - "S11_coherence_audit"
        - "S12_publish_for_implementation"
        - "S13_agent_specs"
        - "S14_plan_and_tests"
      agent_waves:
        - id: "A1_bootstrap_builders"
          description: "Agents used to build SOLVER/SOLVER-Ralph itself (governance runtime + loop machinery)."
        - id: "A2_instance_domain_agents"
          description: "Agents that use a deployed SOLVER-Ralph instance in a specific domain; outputs are problem-resolution artifacts."
        - id: "A3_embedded_domain_workers"
          description: "Agents embedded into a specific instance to repeatedly solve a class of domain problems with required human inputs/approvals."
      assumptions:
        - "Agents are stochastic generators; authoritative commitments require recorded evidence and/or recorded human approvals."
        - "Documentation is infrastructure: governed artifacts are the canonical substrate of meaning for long-running work."
        - "A well-formed problem statement is solution-shaped: it already encodes deliverables, evidence, and authority boundaries."
        - "Work can be decomposed into deliverables with objective oracle tests, enabling loop completion."
---

# SOLVER-Ralph Development Paradigm (SR-PARADIGM) — v1.0.0-draft.10

**Purpose:** Define the governed workflow that turns a human's ambiguous problem statement into a **structured, aligned problem statement** and then into a set of **trustworthy knowledge artifacts** produced via SOLVER-Ralph.

This is a **meta-governance artifact**: it governs how the governed set itself is authored. It sits alongside SR-CHANGE (which governs how the governed set evolves) as a process-authority document that defines the "front door" to SOLVER-Ralph governance.

**Platform problem statement (meta-instantiation):**  
> **How to generate trustworthy knowledge artifacts using non-deterministic agents.**

This document is written so a new author (human or agent) can reproduce the paradigm without relying on prior chat/session context.

---

## 0. How to interpret this document

### 0.1 Position in the governance stack

SR-PARADIGM is a **meta-governance artifact** with process authority. It defines:

- how to author the governed set from a problem statement,
- how to compile a problem statement into SAPS (a forced intermediate representation),
- how to connect SAPS to downstream governed artifacts.

It is distinct from:

| Artifact | What it governs |
|----------|-----------------|
| **SR-INTENT** | Why the system is designed this way (rationale) |
| **SR-DIRECTIVE** | How to execute within published governance |
| **SR-CHANGE** | How to evolve the governed set after publication |

SR-PARADIGM governs *creation*; SR-CHANGE governs *evolution*.

### 0.2 Binding vs guidance

This document is **normative** (process authority), but it intentionally keeps its binding surface area small.

**Binding (MUST/SHALL) in this document constrains:**

- the minimum structure of SAPS (Section 4.3 + Appendix A),
- the minimum alignment protocol (C0 + S0) required before governance authoring,
- the agent invariants (agents produce proposals; they do not create institutional facts),
- conflict handling (route disagreements through SR-CHANGE; do not resolve by interpretation),
- the coherence audit (S11) as a publication gate.

Everything else (templates, checklists, recommended stage acceptance checks) is guidance unless explicitly marked as binding.

### 0.3 Compatibility and conflict handling

SR-PARADIGM MUST be interpreted as compatible with the SOLVER-Ralph governed artifacts selected in **SR-README**.

This document MUST NOT redefine canonical meanings of **Verified**, **Approved**, **Shippable**, **Portal**, **Candidate**, **Evidence Bundle**, or **typed refs**. Binding semantics for those terms live in SR-CONTRACT and SR-SPEC.

**Binding precedence and normative-status rules** are defined in SR-TYPES. If a conflict is discovered between this document and any binding artifact, do not “patch meaning” locally—route the conflict through SR-CHANGE.

### 0.4 Agents are not judges

Agent statements (e.g., “verified,” “approved,” “compliant,” “shippable”) are always **proposal** unless backed by:

- recorded deterministic oracle evidence (verification), and/or
- recorded human approvals at the defined portals (authority),

and properly linked to the relevant candidate and governed artifacts.

---


### 0.5 Stages are a governed state space, not a linear script

This document presents stages (C0, S0, S1–S14) in a canonical authoring/compilation order. **Do not** interpret that order as a single predetermined “workflow script.”

- The **paradigm** defines **deterministic validity rules** (what states exist, which transitions are valid, what evidence and authority are required).
- The **instance** enforces those rules and records commitments (records, evidence, approvals, freezes).
- **Actors** (SYSTEM, humans, agents, and oracles) choose actions and therefore determine the **path** taken through the state space.

Implications:

- A stage may be **revisited** (e.g., return to S0 to correct alignment, or route to SR-CHANGE when governance conflicts are discovered).
- C0 classification determines the appropriate **entry point** (e.g., domain instantiation may not require authoring the full governed set).
- The system is **replayable**: given the same recorded commitments in the same order, the computed state is the same—but the set of possible paths is large because actor choices are not predetermined.

What is binding in SR-PARADIGM is not “the path,” but **what counts as acceptable representation and progression** at each membrane (alignment → typing → evidence → authority → publication).

## 1. Scope

### 1.1 Platform scope (SOLVER / SOLVER-Ralph)

**Platform problem statement:** SOLVER-Ralph exists to make it possible to **consistently generate trustworthy knowledge artifacts** even when the primary producers (agents) are non-deterministic.

In this paradigm, “trustworthy knowledge” is not metaphysical truth. It is knowledge that is:

- **attributable** (who/what produced it),
- **replayable** (what inputs/evaluators were used),
- **evidence-backed** (oracle outputs are recorded), and
- **authority-bound** (humans approve at trust boundaries).

### 1.2 What this paradigm governs

This paradigm governs:

- how to classify a problem statement to determine governance depth,
- how to go from a human problem statement to a **published** governed artifact set,
- how to express the “solution shape” structurally (types → intent → contract → spec → directive → change),
- how to recursively tighten that set as reality changes (without semantic drift),
- how to introduce agent specifications after publication,
- how to produce implementation plans where deliverables are loop-completable and evidence-producing.

### 1.3 What this paradigm does not govern

This paradigm does **not** govern:

- runtime portal semantics (defined by SR-CONTRACT/SR-SPEC/SR-DIRECTIVE),
- the concrete tech stack for any particular instance (instantiated into that instance’s SR-SPEC),
- operational runbooks and incident response (may exist as adjacent artifacts),
- subjective acceptability judgments (those occur at portals and are recorded as approvals/decisions).

### 1.4 Boot alignment prerequisites

To “boot” a new SOLVER-Ralph instance in this paradigm is to establish an alignment substrate so humans and agents converge on stable meaning **without relying on transcript memory**.

SR-PARADIGM does not enumerate the canonical governed set. Instead:

- the canonical registry and current selections are maintained in **SR-README**,
- SR-TYPES governs artifact typing and conflict classification,
- SR-CHANGE governs how the registry and selections evolve.

This document defines boot prerequisites as governance **categories** (see YAML `ext.development_paradigm.boot_alignment_prerequisites`). A new instance MUST satisfy those categories with concrete governed artifacts selected in SR-README.

---

## 2. Relationship to the SOLVER-Ralph governed set

This paradigm is compatible with (and written to align with) the SOLVER-Ralph governed set indexed in SR-README.

The list below is conceptual and non-exhaustive:

- **SR-TYPES** — ontology / artifact taxonomy + metadata rules  
- **SR-ETT** — epistemic trust topology (trust membranes; coercion placement model)  
- **SR-AGENTS** — what an Agent is (object category) and what Agents cannot do  
- **SR-INTENT** — directional “why” and interpretation guidance  
- **SR-CONTRACT** — binding invariants + trust boundary rules  
- **SR-SPEC** — binding technical mechanics (events/APIs/state machines)  
- **SR-PLAN** — typed decomposition of SAPS into packages and deliverables (the *what*)  
- **SR-DIRECTIVE** — execution discipline: phases, gates, stop triggers, verification profiles (the *how*)  
- **SR-CHANGE** — how governance evolves; promotion, selection, and freeze rules  
- **SR-README** — registry and navigation for the set  

### 2.1 SR-PLAN vs SR-DIRECTIVE (disambiguation)

These two artifacts are intentionally separated to prevent drift and “shadow constitutions”.

| Concern | SR-PLAN | SR-DIRECTIVE |
|---|---|---|
| Primary question | **What must be produced** to satisfy the SAPS objectives? | **How the system runs work** to resolution (state progression discipline). |
| Authority kind | Content (normative decomposition) | Process (execution governance). |
| Stability | Stable once aligned; changes routed through SR-CHANGE. | May adapt execution strategy while still honoring the plan. |
| Outputs | Packages, deliverables, dependencies, evidence expectations; **no closure semantics**. | Sequencing (linear/concurrent), gates/triggers/budgets, verification profiles; ensures completeness. |

**Invariants**
- SR-PLAN MUST NOT define portal semantics, approval semantics, or closure rules.
- SR-DIRECTIVE MUST NOT silently change the required deliverable inventory (that is an SR-PLAN change routed through SR-CHANGE).


If a conflict is discovered between this document and binding artifacts, resolve it through SR-CHANGE rather than interpretation.

---

## 3. Definitions

### 3.1 Problem statement family

- **Raw problem statement (RPS):** the human’s initial description (ambiguity is normal).
- **Interpreted problem statement (IPS):** a clarified restatement with explicit scope, non-goals, constraints, and acceptance criteria.
- **Structured, aligned problem statement (SAPS):** the IPS compiled into a governance-ready intermediate representation. It classifies requirements by what can be verified with oracles vs what needs human arbitration; it names deliverables, evidence expectations, and authority boundaries.
- **Instantiated problem statement (INPS):** the SAPS expressed across the governed artifact set (as selected in SR-README), so SOLVER-Ralph can execute it.

**Principle:** a well-formed problem statement is **solution-shaped**: it carries the minimum structure required to build and to prove “done,” not just a narrative.

### 3.2 Proposal vs commitment

- **Proposal:** any draft statement/plan/output that has not crossed an evidence or authority membrane.
- **Commitment object:** a durable, attributable, replayable object the system is allowed to treat as “real,” including:
  - governed artifacts (typed, versioned documents),
  - candidate identities (content-addressed snapshots),
  - evidence bundle manifests (content-addressed proof objects),
  - portal approvals (human responsibility),
  - freeze records (baseline snapshots + declared exceptions),
  - decision and exception records.

### 3.3 Oracle-verifiable vs portal-arbitrated

- **Oracle-verifiable:** can be reduced to deterministic evaluation producing PASS/FAIL evidence.
- **Portal-arbitrated:** requires human judgment because it is not fully oracle-verifiable (or because risk/stakes demand human responsibility).

### 3.4 Evaluation and assessment (non-binding)

- **Evaluation {human}:** a non-binding human interpretation of verification evidence.
- **Assessment {human}:** a non-binding human interpretation of validation evidence in context.
- Evaluation and assessment are **not approvals** and MUST NOT be treated as authority-bearing actions.

---

## 4. Core design principles

### 4.1 Documentation is infrastructure

In SOLVER-Ralph, documents are not tertiary. They are the **meaning substrate** that:

- prevents vocabulary drift,
- replaces transcript memory as a source of truth,
- enables replayable audits and deterministic context compilation,
- makes agents substitutable without losing coherence.

### 4.2 Coerce at membranes, not everywhere

The system preserves exploration inside an iteration while coercing outputs **only when they become load-bearing inputs**.

Practically: proposals stay fluid until they are written into commitment objects (artifacts, events, evidence, approvals, freezes, change records).

### 4.3 SAPS is the forced intermediate representation (minimum binding fields)

SAPS is the paradigm’s **forced intermediate representation** between “human intent” and “governed instantiation.”

A SAPS MUST include the following **minimum binding field set** (sections may be brief; “N/A” is acceptable only if explicitly justified and does not create ghost meaning):

- Problem Statement interpretation and clarification
- Requirements
- Objectives
- Planning
- Development (Build)
- Verification {agentic} ↔ Evaluation {human}
- Validation {agentic} ↔ Assessment {human}
- Implementation (Integration)
- Reflection
- Resolution
- Change Management
- Iteration Preparation
- Close-out conditions

**Interpretation note:** “{agentic}” here means “produced through deterministic/recordable mechanisms (typically oracles + artifacts) without relying on human authority.” It does **not** mean “agents self-verify.”

These sections are binding because they are the minimal structure needed to:

- avoid “ghost inputs” (unstated assumptions becoming foundations),
- separate evidence from authority,
- and make “done” testable and attributable.

### 4.4 Verification/validation produce evidence; evaluation/assessment interpret evidence

- **Verification {agentic}** produces deterministic evidence.
- **Evaluation {human}** interprets verification evidence (non-binding) so humans can decide what it means.
- **Validation {agentic}** produces evidence about the fitness-in-context of the deterministic evidence from validation (tests, demos, trials, etc.).
- **Assessment {human}** interprets validation evidence (non-binding) so humans can decide acceptability.

Evaluation/assessment MUST NOT be treated as approval. Approval occurs only through the portal/record mechanisms defined elsewhere.  Evaluation and assessment are human-in-the-loop nodes where decisions are made on how agentic Ralph-loops will unfold.

### 4.5 Verification produces evidence; approval binds responsibility

- **Verification** is an evidence-backed claim grounded in deterministic oracle outputs.
- **Approval** is a human trust-boundary action that binds responsibility for moving forward.

Approval MUST NOT be used as a substitute for verification evidence.

### 4.6 The governed set is recursive by design

The governed artifact set is not written once. It is tightened through recursion:

- once meaning is instantiated into a tech stack and concrete interfaces,
- once change-control semantics exist,
- once implementation reveals mismatches and hidden assumptions.

### 4.7 Mapping to SR-ETT (nine harness membranes)

This paradigm is document-centric because governed artifacts are the primary mechanism for placing coercion boundaries.

A mapping from SR-ETT’s membranes to the authoring workflow:

| SR-ETT harness membrane | Where it is expressed in the workflow | What it stabilizes |
|---|---|---|
| Intent & Objective | C0/S0 → INTENT | what “success” means; prevents goal drift |
| Ontological | TYPES | what exists; prevents category errors |
| Architectural | CONTRACT (+ architecture proposal) | boundaries and invariants |
| Operational | DIRECTIVE | executable phases/gates/stop triggers |
| Authority & Integrity | CONTRACT + SPEC + portals | who can commit; admissibility |
| Accountability | SPEC evidence/records + freeze | replayability and completeness |
| Isomorphic | SPEC ↔ implementation conformance | prevents spec/runtime drift |
| Resource | DIRECTIVE budgets | feasibility; prevents runaway loops |
| Change | CHANGE | controlled evolution; prevents silent meaning shifts |

---

## 5. Problem statement classification (C0)

Before entering the governance authoring workflow, classify the problem statement to determine appropriate governance depth.

### 5.1 Classification categories

| Classification | Characteristics | Entry point | Governance depth |
|----------------|-----------------|-------------|------------------|
| **Platform construction** | Building SOLVER-Ralph itself; no prior governance exists | C0 → S0 → full workflow | Full artifact set |
| **Domain instantiation** | Using SOLVER-Ralph for a new domain; platform governance exists | C0 → S0 → domain-scoped workflow | Domain artifact set |
| **Governed execution** | Operating within existing domain governance; no new governance needed | C0 → S13-S14 or embedded work | Minimal; use existing artifacts |
| **Ad-hoc task** | One-shot work not requiring governance artifacts | May not require this paradigm | None; direct execution |

### 5.2 Classification decision tree

```
                    PROBLEM STATEMENT
                           │
                           ▼
              ┌────────────────────────┐
              │ Does SOLVER-Ralph      │
              │ governance exist?      │
              └────────────────────────┘
                    │           │
                   NO          YES
                    │           │
                    ▼           ▼
        ┌───────────────┐  ┌────────────────────────┐
        │   PLATFORM    │  │ Does domain-specific   │
        │ CONSTRUCTION  │  │ governance exist?      │
        │ (full S0-S14) │  └────────────────────────┘
        └───────────────┘        │           │
                                NO          YES
                                 │           │
                                 ▼           ▼
                      ┌───────────────┐  ┌────────────────────────┐
                      │    DOMAIN     │  │ Does this task need    │
                      │ INSTANTIATION │  │ new governance?        │
                      │(domain S0-S14)│  └────────────────────────┘
                      └───────────────┘        │           │
                                              NO          YES
                                               │           │
                                               ▼           ▼
                                   ┌───────────────┐  ┌───────────────┐
                                   │   GOVERNED    │  │    DOMAIN     │
                                   │  EXECUTION    │  │ INSTANTIATION │
                                   │ (S13-S14 or   │  │ (via SR-CHANGE│
                                   │  embedded)    │  │  for changes) │
                                   └───────────────┘  └───────────────┘
```

### 5.3 Classification outputs

| Output | Description |
|--------|-------------|
| `classification` | One of: `platform_construction`, `domain_instantiation`, `governed_execution`, `adhoc_task` |
| `entry_point` | Which stage to begin at |
| `governance_scope` | What artifacts will be produced or referenced |
| `agent_wave` | Which wave context applies (1, 2, or 3) |
| `existing_governance_refs` | If applicable, references to existing governed artifacts |

### 5.4 Classification acceptance check

Classification is complete when:

- [ ] The human confirms the classification category
- [ ] The entry point is identified
- [ ] If existing governance applies, the relevant artifacts are identified
- [ ] If new governance is needed, the scope is estimated

If classification is unclear, default to **domain instantiation** and let S0 reveal the true scope.

---

## 6. Problem statement alignment (S0)

S0 is the entry point for all problem statements that require governance. It transforms an ambiguous human problem statement into a structured, aligned problem statement (SAPS) suitable for governance authoring.

### 6.1 S0 Overview

S0 has five sub-stages:

| Sub-stage | Purpose | Output |
|-----------|---------|--------|
| **S0.1 Intake** | Capture the raw problem statement and essential context | Raw problem statement + initial context |
| **S0.2 Governability assessment** | Determine if the problem is suitable for governance | Governability score + issues |
| **S0.3 Interpreted problem statement** | Clarify and scope in human terms | IPS artifact |
| **S0.4 Structured problem statement** | Expand into governance-ready structure | SAPS artifact (Appendix A) |
| **S0.5 Alignment confirmation** | Obtain human confirmation | Alignment evidence |

### 6.2 S0.1 Intake

**Purpose:** Capture the raw problem statement and essential context from the human.

**Required questions (agents MUST ask; responsible humans MUST answer sufficiently to proceed):**

| Question | Why it matters |
|----------|----------------|
| What outcome would constitute success? | Defines the target; becomes basis for acceptance criteria |
| What outcomes are explicitly out of scope? | Prevents scope creep; establishes boundaries |
| What constraints are non-negotiable? | Identifies hard requirements vs preferences |
| Who has authority to approve the final result? | Establishes trust boundary; identifies portal routing |
| What evidence would convince you the result is correct? | Shapes evidence/oracle planning; makes success testable |
| What is the time horizon for this work? | Affects governance depth |
| What happens if this fails? | Reveals stakes; informs risk handling |

**Outputs:**

- Raw problem statement (verbatim from human)
- Answers to required questions (may be partial)
- Identified gaps requiring clarification

**Failure routing:**

If answers are not sufficient to proceed, do not advance. Continue intake dialogue until they are.

### 6.3 S0.2 Governability assessment

**Purpose:** Determine whether the problem statement is suitable for governance authoring.

**Governability criteria:**

| Criterion | Question | Pass condition |
|-----------|----------|----------------|
| **Scoped** | Is the problem bounded? | Clear boundaries exist; not “boil the ocean” |
| **Testable** | Can success be verified/evidenced? | At least one evidence path exists |
| **Authority-clear** | Is approval authority identified? | At least one human can approve the final result |
| **Stable** | Is the problem statement stable? | Not shifting uncontrollably; can be versioned |
| **Decomposable** | Can work be packaged? | At least one deliverable can be defined |

**Outputs:**
### 6.4 S0.3 Interpreted problem statement (IPS)

**Purpose:** Produce a clarified, scoped restatement in human terms that can be read back without contradiction.

**Binding requirement (commitment object):** The IPS MUST be stored as a typed, versioned commitment object: `record.problem_statement` with `ext.problem_statement.stage: IPS` (see SR-TYPES).

**IPS minimum contents:**

- **Objective:** one paragraph stating what success looks like
- **Scope:** what is in scope, what is explicitly out of scope
- **Acceptance criteria:** how the human will know the problem is solved
- **Authority:** who approves the final result (and which approvals are expected)
- **Constraints:** non-negotiable requirements and preferences
- **Assumptions:** what is being assumed (with triggers for re-evaluation)

**Outputs (MUST):**

- `PS-*` record: `record.problem_statement` at stage `IPS`

**Acceptance check:**

- [ ] The human can read the IPS and say “Yes, that’s what I mean” (or provide corrections)
- [ ] The scope is clear enough to estimate governance depth
- [ ] At least one acceptance criterion is testable/evidenced
- [ ] The IPS exists as a `record.problem_statement` (stage `IPS`) with a stable id and version

**Purpose:** Produce a clarified, scoped restatement in human terms.

**IPS minimum contents:**

- **Objective:** one paragraph stating what success looks like
- **Scope:** what is in scope, what is explicitly out of scope
- **Acceptance criteria:** how the human will know the problem is solved
- **Authority:** who approves the final result
- **Constraints:** non-negotiable requirements and preferences
- **Assumptions:** what is being assumed (with triggers for re-evaluation)

**Acceptance check:**

- [ ] The human can read the IPS and say “Yes, that’s what I mean” (or provide corrections)
- [ ] The scope is clear enough to estimate governance depth
- [ ] At least one acceptance criterion is testable/evidenced

### 6.5 S0.4 Structured, aligned problem statement (SAPS)

**Purpose:** Expand the IPS into a governance-ready, solution-shaped structure that can be instantiated into governed artifacts and loop-completable work.

**Binding requirement (commitment object):** The SAPS MUST be stored as a typed, versioned commitment object: `record.problem_statement` with `ext.problem_statement.stage: SAPS` (see SR-TYPES).

SAPS MUST include the minimum binding field set defined in Section 4.3, structured using Appendix A.

Agents should use judgment about how much depth each section requires given the problem’s scope and complexity.

**Authoritativeness rule (no ghost meaning):**

- The stage `SAPS` `record.problem_statement` is the **authoritative** statement of the problem for S1–S14 authoring work.
- Paraphrases/summaries MAY be written for readability, but MUST NOT substitute for referencing the committed SAPS record. If the paraphrase is wrong, the remedy is to **revise the SAPS record** (new version), not to “silently drift” downstream documents.

**Outputs (MUST):**

- `PS-*` record: `record.problem_statement` at stage `SAPS` (body follows Appendix A)
- Requirement classification (oracle-verifiable vs portal-arbitrated vs deferred), either embedded in the SAPS record body or attached and referenced by the SAPS record
- Trace linkage to IPS (the SAPS record SHOULD reference the IPS record in `ext.problem_statement.source_refs[]` or an equivalent typed reference mechanism)

**Acceptance checks (MUST):**

- Every requirement is classified (oracle vs portal vs deferred)
- SAPS contains at least one executable path to completion (deliverables + evidence plan + authority boundaries)
- SAPS identifies the earliest trust boundary (where human judgment is required)
- SAPS is traceable back to the IPS (and the IPS exists as a `record.problem_statement` stage `IPS`)
- The SAPS exists as a `record.problem_statement` (stage `SAPS`) with a stable id and version

**Failure routing:**

- If requirements cannot be classified, return to S0.3 and clarify
- If “done” cannot be reduced to any evidence plan, the problem is not yet SOLVER-Ralph compatible

### 6.6 S0.5 Alignment confirmation

**Purpose:** Obtain explicit human confirmation that the SAPS is correct and that the authoritative problem statement exists as commitment objects (IPS/SAPS records), not as conversational residue.

**Alignment dialogue:**

Present the IPS and SAPS to the human (as `record.problem_statement` artifacts) and explicitly confirm:

1. “Does this objective accurately capture what you want to achieve?”
2. “Are the scope boundaries correct?”
3. “Are the acceptance criteria testable and sufficient?”
4. “Is the authority assignment correct?”
5. “Are there any constraints missing?”

**Outputs (MUST):**

- Human confirmation (recorded) that references the authoritative SAPS record id + version (and IPS id + version if maintained separately)
- Any corrections incorporated into new versions of the IPS/SAPS records
- Final IPS/SAPS versions designated as the authoritative problem statement inputs for S1+

**Alignment evidence (MUST be recorded):**

| Evidence | Description |
|----------|-------------|
| `alignment_dialogue_summary` | Summary of the alignment conversation |
| `problem_statement_refs` | Authoritative `record.problem_statement` refs (ids + versions) for IPS and SAPS |
| `human_confirmation` | Explicit confirmation from human with timestamp |
| `governability_assessment` | Results of S0.2 assessment |
| `classification_record` | Results of C0 classification |

This evidence becomes input to S1 and is referenced throughout governance authoring. If later work discovers that the SAPS is wrong or incomplete, the remedy is to **revise the SAPS record** (new version) and propagate the reference update, not to silently reinterpret downstream artifacts.

---

## 7. Authoring workflow (S1–S14)

This section defines the recommended authoring sequence. Each stage describes:

- **Inputs:** what must exist before the stage begins
- **Outputs:** what the stage produces
- **Acceptance checks:** how to decide the stage is complete enough to proceed
- **Failure routing:** what to do when the stage reveals a mismatch

This workflow intentionally begins with ontology (TYPES) before intent, and with intent before contract/spec.

**Binding input rule (no ghost inputs):** S1–S14 authoring work MUST treat the SAPS `record.problem_statement` (stage `SAPS`) as an explicit input. In practice:

- Every authoring iteration/context bundle MUST include a reference to the authoritative SAPS record (and IPS record if maintained separately).
- Governance artifacts produced during S1–S14 MUST reference the authoritative SAPS record id (at minimum) so that audits can reconstruct “what problem we thought we were solving.”

Re-paraphrasing is allowed for explanation, but not as a substitute for these references.


Unless explicitly labeled as binding, the acceptance checks in S1–S10 and S12–S14 are **guidance**. S11 remains a **binding gate**.

### S1 TYPES initial

**Purpose:** establish the forcing ontology (what exists; what words mean).

**Inputs**
- SAPS (S0.4)
- Alignment evidence (S0.5)

**Outputs**
- TYPES artifact draft defining:
  - artifact types and authority model
  - metadata conventions
  - precedence and conflict handling (by reference to SR-TYPES rules)
  - relationship semantics (semantic dependency vs audit provenance)

**Acceptance checks (guidance)**
- A new author can classify any planned artifact into a type.
- The document prevents category errors (e.g., “approval” vs “review”, “verification” vs “belief”).

**Failure routing**
- If disagreement on terminology persists, resolve it in TYPES before writing INTENT.

---

### S2 INTENT initial

**Purpose:** anchor the reference signal (what matters; tradeoffs among compliant options).

**Inputs**
- SAPS (S0.4)
- TYPES (S1)

**Outputs**
- INTENT artifact draft defining:
  - what the system is protecting against (failure modes)
  - assumptions + triggers that cause escalation
  - rationale for constraints, boundaries, and trust model

**Acceptance checks (guidance)**
- The human can recognize their intent in the artifact.
- The artifact is explicit about how it should be interpreted (directional vs binding).

**Failure routing**
- If intent is not stable, iterate with S0 and then revise INTENT.

---

### S3 Architecture proposal and tech stack research

**Purpose:** instantiate abstract intent into the constraints of a real stack.

**Inputs**
- SAPS (S0.4)
- TYPES (S1)
- INTENT (S2)

**Outputs**
- A proposed architecture boundary model (ports/adapters, trust boundaries)
- Stack research and selection sufficient to write SPEC without inventing a new stack later
- Identified constraints: where determinism is hard; where portals are unavoidable

**Acceptance checks (guidance)**
- The stack is concrete enough to support:
  - event store + projections,
  - evidence storage,
  - oracle sandbox execution,
  - identity and approval recording.
- The architecture is compatible with the invariants expected by the contract model.

**Failure routing**
- If the stack cannot satisfy critical constraints, revise INTENT tradeoffs or select a different stack.

---

### S4 TYPES iteration for the chosen stack

**Purpose:** update ontology so it survives contact with implementation reality.

**Inputs**
- Stack + architecture proposal (S3)
- TYPES (S1)

**Outputs**
- TYPES iteration that adds whatever must exist for the SPEC to be concrete:
  - missing record/evidence types,
  - ref semantics clarifications,
  - boundary clarifications.

**Acceptance checks (guidance)**
- TYPES includes whatever object categories are needed for SPEC-level models.
- Terminology remains consistent with SR-TYPES invariants.

---

### S5 INTENT iteration for the chosen stack

**Purpose:** update intent so it is honest about real constraints and costs.

**Inputs**
- Stack + architecture proposal (S3)
- INTENT (S2)

**Outputs**
- Updated assumptions and triggers based on stack realities
- Clarified failure modes that become visible only after S3

**Acceptance checks (guidance)**
- INTENT remains concise but specific enough to guide contract tradeoffs.

---

### S6 CONTRACT

**Purpose:** bind invariants and trust boundaries.

**Inputs**
- TYPES (iterated)
- INTENT (iterated)
- Architecture boundary model + stack constraints

**Outputs**
- CONTRACT artifact specifying:
  - invariants implementations cannot violate,
  - trust boundary rules (who can do what),
  - oracle integrity conditions and stop-the-line requirements,
  - required evidence and approval linkage invariants.

**Acceptance checks (guidance)**
- The contract can be tested (at least in principle) by deterministic checks and/or portal records.
- The contract is implementation-agnostic: it binds behavior, not libraries.

**Failure routing**
- If an invariant cannot be enforced, revise architecture/stack or narrow scope.

---

### S7 SPEC

**Purpose:** specify the concrete, implementable mechanics.

**Inputs**
- TYPES (iterated)
- CONTRACT (S6)
- Stack decisions (S3)

**Outputs**
- SPEC artifact specifying:
  - event and record schemas,
  - API surfaces and auth boundaries,
  - evidence bundle formats,
  - staleness/dependency semantics,
  - candidate identity rules,
  - projection/rebuild rules.

**Acceptance checks (guidance)**
- A competent implementer can begin coding without inventing missing semantics.
- Every binding contract requirement is implemented or explicitly routed to a portal/decision.

**Failure routing**
- If SPEC cannot implement CONTRACT invariants, revise S6 or S3.

---

### S8 DIRECTIVE

**Purpose:** make execution loop-completable.

**Inputs**
- SPEC (S7)
- CONTRACT (S6)
- INTENT (S5)

**Outputs**
- DIRECTIVE artifact specifying:
  - implementation phases and gates,
  - stop triggers and thresholds,
  - budget defaults and override rules,
  - verification profiles (oracle suites + modes),
  - portal set and usage constraints.

**Acceptance checks (guidance)**
- Gates are executable (objective oracle checks exist or can exist).
- Stop triggers prevent silent drift and false foundations.
- The directive does not invent new portal semantics.

**Failure routing**
- If gates cannot be made executable, revise deliverables decomposition (SAPS) and/or SPEC.

---

### S9 CHANGE

**Purpose:** make governance evolution explicit and safe.

**Inputs**
- The selected governed artifacts (S1–S8)

**Outputs**
- CHANGE artifact specifying:
  - how versions evolve and supersede,
  - how “current” selection is made,
  - how freezes/baselines are recorded,
  - how exceptions (deviation/deferral/waiver) are created and reviewed,
  - portal routing for governance-impacting changes.

**Acceptance checks (guidance)**
- Every governed artifact can evolve without silent reinterpretation.
- Promotion and freeze procedures are explicit and enforceable.

**Failure routing**
- If change routing is ambiguous, update TYPES (type/authority classification) and/or DIRECTIVE (process entry points).

---

### S10 Recursive iteration across the governed set

**Purpose:** tighten coherence after CHANGE exists.

**Inputs**
- All artifacts through CHANGE (S9)

**Outputs**
- Revisions to TYPES/INTENT/CONTRACT/SPEC/DIRECTIVE/CHANGE where inconsistencies exist
- Recorded decisions where tradeoffs must be explicitly settled

**Acceptance checks (guidance)**
- Cross-artifact terms are consistent.
- No binding artifact relies on unrecorded “chat memory.”

---

### S11 Coherence audit (BINDING GATE)

**Purpose:** run an explicit coherence check before publication/implementation.

**Normative status:** This stage is a **binding gate**. The governed set MUST NOT be published (S12) until the coherence audit passes or failures are explicitly recorded as deviations.

**Inputs**
- A candidate governed snapshot (selected versions)

**Outputs**
- A coherence audit report (evidence artifact)
- If failures exist: recorded deviations or a remediation plan

**Coherence dimensions (MUST be checked):**

| Dimension | What it checks | Oracle-verifiable? |
|-----------|----------------|-------------------|
| **Ontological** | Types complete; no category errors | Partial (metadata validation) |
| **Epistemic** | Evidence/authority separation is consistent | Partial (term consistency) |
| **Semantic** | Terms used consistently across artifacts | Yes (text analysis) |
| **Isomorphic** | SPEC implements CONTRACT; DIRECTIVE can produce required evidence | Partial (traceability) |
| **Change** | Evolution paths explicit and routed correctly | Yes (routing validation) |
| **Resource** | Budgets/stop triggers make loops feasible | Yes (constraint validation) |

**Acceptance checks (MUST):**

- [ ] All six coherence dimensions have been checked
- [ ] Failures are either remediated or recorded as deviations with rationale
- [ ] The coherence audit report is recorded as an evidence artifact
- [ ] A human has reviewed and accepted the coherence report

**Failure routing:**

- If coherence failures exist and cannot be remediated, they MUST be recorded as deviations per SR-CHANGE before S12 can proceed.

---

### S12 Publish for implementation

**Purpose:** declare a coherent, usable governance snapshot as the baseline for implementation work.

**Inputs**
- Coherence audit results (S11)

**Outputs**
- A published snapshot selection (and/or freeze record when applicable)
- Explicit references to the governed artifact versions that are in force

**Acceptance checks (guidance)**
- Implementers can rely on stable meanings without guessing.
- Any known exceptions are explicit and visible.
- The coherence audit passed or failures are recorded as deviations.

---

### S13 Agent specifications

**Purpose:** define how agents will operate within the now-stable governance semantics.

**Inputs**
- Published governance snapshot (S12)

**Outputs**
- `config.agent_definition` artifacts consistent with SR-TYPES (or its successor)
- Tooling constraints: sandbox, allowed tools, context compilation rules

**Acceptance checks (guidance)**
- Agent capabilities and prohibitions are stated in terms of stable governed semantics (not ad-hoc rules).
- Agent definitions do not grant human-only authority.
- All agents are subject to the same invariants: cannot claim Verified, Approved, or Shippable.

---

### S14 Plan and tests

**Purpose:** compile the authoritative SAPS into a typed deliverable inventory (SR-PLAN), and introduce the earliest test/oracle scaffolding as explicit deliverables.

**Inputs**
- Published governance snapshot (S12)
- Agent specs (S13)
- `record.problem_statement` (stage `SAPS`) (authoritative problem statement)

**Outputs**
- **SR-PLAN instance** (`governance.plan`): packages → deliverables → dependencies, where each deliverable is a loop-completable `work_unit` and declares its target `type_key`(s) and expected evidence hooks.
- Initial test/oracle scaffolding captured as deliverables in SR-PLAN (e.g., `test.oracle_suite`, `code.package`, `record.oracle_run`, etc.) as appropriate.

**Acceptance checks (guidance)**
- The SR-PLAN instance references the authoritative SAPS record via typed `refs[]` and does not re-paraphrase it.
- Each deliverable is small enough to converge within loop budgets and has an evidence-producing completion condition.
- Dependencies are encoded as `depends_on` edges; concurrency is enabled where dependency structure and type grouping allow it.
- SR-PLAN remains abstract about execution sequencing and closure; SR-DIRECTIVE owns execution/state progression.

**Failure routing**
- If planning reveals missing governance meaning (types, terms, gates), route changes through SR-CHANGE.
- If planning reveals infeasible constraints, revise the SAPS and re-commit `record.problem_statement` rather than “patching” meaning in place.

## 8. Agents and operational contexts

### 8.1 Agent invariants

All agents share these invariants:

| Invariant | What it means |
|-----------|---------------|
| **Cannot claim Verified** | Verification requires oracle evidence; agents are not oracles |
| **Cannot claim Approved** | Approval is a human trust-boundary action |
| **Cannot claim Shippable** | Shippable requires verification + approval + baseline conditions |
| **Outputs are proposals** | Agent outputs have not crossed evidence or authority membranes |
| **Must produce evidence** | Claims without evidence are invalid |

These invariants are derived from the separation of verification (evidence) from approval (authority).

### 8.2 Agent wave model

SOLVER-Ralph uses a three-wave agent strategy:

| Wave | Context | Entry point | Governance depth |
|------|---------|-------------|------------------|
| **A1: Bootstrap builders** | Building SOLVER-Ralph itself | C0 → full workflow | Full governance authoring |
| **A2: Instance/domain agents** | Using SOLVER-Ralph for a domain problem | C0 → domain-scoped workflow | Domain governance |
| **A3: Embedded workers** | Operating within deployed instance | S13-S14 or embedded work | Minimal; execute within existing governance |

Across all waves:

- agents generate proposals and work products,
- oracles generate evidence,
- humans approve at trust boundaries.

The waves differ in governance scope, not in the fundamental nature of what agents are.

---

## 9. Recursion and coherence

### 9.1 Recursion model

Recursion occurs when:

- the chosen stack forces an update to types or interfaces,
- contract invariants prove unenforceable without a design change,
- directive gates are not actually executable (no objective tests),
- change control reveals missing classification or routing rules,
- implementation reveals drift between enforcement and intended meaning.

**Recursion principle:**  
> When meaning changes, write it into governed artifacts and advance versions rather than letting it live in transient conversation context.

### 9.2 Why coherence audits are binding

The coherence audit (S11) is a binding gate because:

1. **Incoherent governance defeats the purpose.** If artifacts contradict each other, agents cannot reliably follow them.

2. **Evidence makes coherence auditable.** The coherence report is an evidence artifact that can be referenced.

3. **Failures must be explicit.** If coherence cannot be achieved, deviations must be recorded—not silently ignored.

---

## 10. Publication, commitment objects, and approval order

### 10.1 Publication and commitment objects

This paradigm assumes the system’s “real knowledge” is expressed as commitment objects:

- governed artifact versions (with metadata),
- candidates (content-addressed snapshots),
- evidence bundles (oracle outputs with manifests),
- approvals (portal decisions),
- freeze records (baseline snapshot + exceptions),
- decisions and exception records (deviation/deferral/waiver).

### 10.2 Deliverables, evidence, and approval order

**Closure rule (cross-document alignment):**  
A deliverable is only presented for human approval after its objective verification evidence exists. Approval is not used as a substitute for verification.

Minimal loop-compatible order:

1) Produce a candidate work product (code/doc/config) with stable identity.
2) Run oracles and capture evidence bundle(s).
3) Compile the gate packet (candidate + evidence + governed refs + exceptions).
4) Present the verified output for human judgment at the relevant portal when required.
5) Record the approval decision, referencing subject and evidence explicitly.

---

## 11. Normative requirements (summary)

This section is normative. The keywords **MUST**, **SHOULD**, and **MAY** are used intentionally.

### 11.1 Registry and conflict requirements

1) SR-PARADIGM MUST NOT act as the canonical governed-artifact registry. The canonical registry and current selections MUST be maintained in SR-README.

2) When governed artifacts disagree, conflicts MUST NOT be resolved by interpretation; they MUST be reconciled via SR-CHANGE.

### 11.2 Classification and alignment requirements

3) A SOLVER-Ralph effort MUST begin with **C0** (classification) to determine governance depth.

4) If governance authoring is required, **S0** (alignment) MUST be completed before S1.

5) S0 MUST produce a SAPS artifact using Appendix A, containing the minimum binding field set defined in Section 4.3.

6) S0 MUST produce alignment evidence including explicit human confirmation.

### 11.3 SAPS requirements

7) SAPS MUST classify requirements as oracle-verifiable vs portal-arbitrated vs deferred.

8) SAPS MUST include a definition of “done” that includes an evidence plan (verification/validation) and responsibility boundaries (authority).

### 11.4 Coherence requirements

9) The coherence audit (S11) is a **binding gate**. The governed set MUST NOT be published until S11 passes or failures are recorded as deviations.

10) The coherence audit MUST check all six dimensions (ontological, epistemic, semantic, isomorphic, change, resource).

### 11.5 Agent requirements

11) Agent narration MUST NOT be treated as evidence or authority.

12) Agent definitions MUST NOT grant human-only authority.

---

## 12. Change control

This document is governed by **SR-CHANGE**.

- Changes to this paradigm are governance changes and MUST be versioned.
- If a change alters the meaning of binding terms or modifies how trust boundaries/evidence/verification are interpreted, it MUST be routed through the appropriate portal(s) per SR-CHANGE and SR-DIRECTIVE.

---

## Appendix A — SAPS Intake Form (Structured, Aligned Problem Statement)

This template is normative for stage S0.4.

**Storage requirement:** The SAPS is stored as `record.problem_statement` with `ext.problem_statement.stage: SAPS` (see SR-TYPES). The authoritative SAPS record id + version MUST be referenced by all subsequent authoring work (S1–S14).

If you also maintain a separate IPS record, it is stored as `record.problem_statement` with stage `IPS`, and the SAPS record SHOULD reference it as a source ref.


A SAPS MUST contain the 13 section headings below (the minimum binding field set). Within each section:

- **Binding fields (MUST)** are the minimal durable facts that downstream governance relies on.
- **Optional prompts (SHOULD)** are suggestions to improve completeness; agents should apply judgment about depth based on scope and risk.

---

### A.1 Problem Statement interpretation and clarification

**Purpose:** Capture the interpreted problem statement and ensure alignment.

**Binding fields (MUST):**
- Raw problem statement (verbatim from human)
- Interpreted problem statement (IPS-quality restatement)
- Clarifications made (what was ambiguous and how it was resolved)
- Open ambiguities (what remains unclear and how it will be resolved)
- Alignment confirmation (who confirmed; when; what changed)

**Optional prompts (SHOULD):**
- Problem ID and versioning notes
- Stakeholders and risk framing

---

### A.2 Requirements

**Purpose:** Capture and classify all requirements.

**Binding fields (MUST):**
- Requirements list
- Classification for each requirement (oracle-verifiable vs portal-arbitrated vs deferred)

**Classification table (MUST):**

| Requirement | Oracle-verifiable? | Portal-arbitrated? | Deferred? | Rationale |
|-------------|-------------------|-------------------|-----------|-----------|
| ... | | | | |

**Optional prompts (SHOULD):**
- Functional vs non-functional grouping
- Governance requirements (required artifacts/records)

---

### A.3 Objectives

**Purpose:** Define success criteria and non-goals.

**Binding fields (MUST):**
- Primary objective (one paragraph)
- Non-goals / explicit non-scope
- Definition of “done” including:
  - outcome-level (human-facing)
  - artifact-level (what artifacts exist when done)
  - evidence-level (what evidence exists)

**Optional prompts (SHOULD):**
- Secondary objectives

---

### A.4 Planning

**Purpose:** Outline the approach and decomposition. In SOLVER-Ralph, this planning content is compiled into an **SR-PLAN instance** (typed decomposition into packages/deliverables/dependencies).

**Binding fields (MUST):**
- Decomposition into work units / deliverables
- Dependencies
- Risks + mitigation triggers

**Optional prompts (SHOULD):**
- Phases/milestones

---

### A.5 Development (Build)

**Purpose:** Specify what will be built and how.

**Binding fields (MUST):**
- Deliverables list (what will be produced)
- For each deliverable: completion condition

**Optional prompts (SHOULD):**
- Technical approach and tool/stack assumptions (or refs)

---

### A.6 Verification {agentic} ↔ Evaluation {human}

**Purpose:** Define how verification will produce evidence, and how that evidence will be evaluated.

**Binding fields (MUST):**
- Verification plan (oracle-verifiable evidence path)
- Evidence bundle expectations (what is recorded)
- Evaluation approach (what humans look for when interpreting verification evidence; non-binding)

**Optional prompts (SHOULD):**
- Candidate identity boundaries (what is a Candidate)
- Determinism risks (flake, env mismatch) and handling assumptions

---

### A.7 Validation {agentic} ↔ Assessment {human}

**Purpose:** Define how validation will confirm fitness for purpose, and how that will be assessed.

**Binding fields (MUST):**
- Validation plan (what evidence will be produced about fitness-in-context)
- What humans must decide at portals (do not invent new portals)
- Assessment approach (how fitness will be judged; non-binding)

**Optional prompts (SHOULD):**
- Fitness criteria examples and rejection conditions

---

### A.8 Implementation (Integration)

**Purpose:** Define how deliverables will be integrated and deployed.

**Binding fields (MUST):**
- Integration approach
- Rollback/recovery approach (conceptual is acceptable)

**Optional prompts (SHOULD):**
- External dependencies and deployment considerations

---

### A.9 Reflection

**Purpose:** Define how lessons will be captured.

**Binding fields (MUST):**
- Reflection points (when)
- How reflection is recorded and fed back into change

**Optional prompts (SHOULD):**
- Reflection questions

---

### A.10 Resolution

**Purpose:** Define how issues and escalations will be resolved.

**Binding fields (MUST):**
- Escalation thresholds
- Resolution authority mapping

**Optional prompts (SHOULD):**
- Resolution recording patterns (decision records)
- Exception handling expectations (deviation/deferral/waiver routing)

---

### A.11 Change Management

**Purpose:** Define how changes to scope, requirements, or governance will be handled.

**Binding fields (MUST):**
- Change triggers
- Change routing discipline (SR-CHANGE reference)

**Optional prompts (SHOULD):**
- Impact assessment approach

---

### A.12 Iteration Preparation

**Purpose:** Define how iterations will be structured and prepared.

**Binding fields (MUST):**
- Iteration scope (what an iteration covers)
- Iteration inputs and outputs
- Budget constraints (or “use defaults”)

**Optional prompts (SHOULD):**
- Context compilation notes (what refs must be included, per SR-SPEC)

---

### A.13 Close-out conditions

**Purpose:** Define when the work is complete.

**Binding fields (MUST):**
- Completion criteria (requirements satisfied or explicitly deferred)
- Evidence completeness criteria (verification/validation evidence exists)
- Authority completeness criteria (required approvals obtained)

**Optional prompts (SHOULD):**
- Close-out checklist (e.g., freeze record created if applicable; reflection completed)
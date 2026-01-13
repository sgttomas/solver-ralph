---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-PARADIGM"
  type: "governance.development_paradigm"
  title: "SOLVER-Ralph Development Paradigm"
  version: "1.0.0-draft.4"
  status: "draft"
  normative_status: "normative"
  authority_kind: "process"
  governed_by: ["SR-CHANGE"]
  supersedes:
    - "SR-PARADIGM@1.0.0-draft.3"
  created: "2026-01-10"
  updated: "2026-01-10"
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
  ext:
    development_paradigm:
      intent:
        core_problem_statement: "How to generate trustworthy knowledge artifacts using non-deterministic agents."
        summary: >
          Guidance for translating a human problem statement into a structured,
          aligned problem statement and then instantiating it into the SOLVER‑Ralph
          governed artifact set (TYPES/INTENT/CONTRACT/SPEC/DIRECTIVE/CHANGE/ETT/README)
          so stochastic agent labor can be harnessed into durable, attributable,
          replayable, evidence-backed commitments.
      governed_artifacts_minimum_set:
        - "SR-TYPES"
        - "SR-INTENT"
        - "SR-CONTRACT"
        - "SR-SPEC"
        - "SR-DIRECTIVE"
        - "SR-CHANGE"
        - "SR-ETT"
        - "SR-README"
      workflow_stages:
        - "S0_alignment"
        - "S0a_structured_problem_statement"
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
          description: "Agents used to build SOLVER/SOLVER‑Ralph itself (governance runtime + loop machinery)."
        - id: "A2_instance_domain_agents"
          description: "Agents that use a deployed SOLVER‑Ralph instance in a specific domain; outputs are problem-resolution artifacts."
        - id: "A3_embedded_domain_workers"
          description: "Agents embedded into a specific instance to repeatedly solve a class of domain problems with required human inputs/approvals."
      assumptions:
        - "Agents are stochastic generators; authoritative commitments require recorded evidence and/or recorded human approvals."
        - "Documentation is infrastructure: governed artifacts are the canonical substrate of meaning for long-running work."
        - "Work can be decomposed into deliverables with objective oracle tests, enabling loop completion."
---

# SOLVER-Ralph Development Paradigm (SR-PARADIGM) — v1.0.0-draft.4

**Purpose:** Define the governed workflow that turns a human’s ambiguous problem statement into a **structured, aligned problem statement** and then into a set of **trustworthy knowledge artifacts** produced via SOLVER‑Ralph.

This is the “agent guidance” layer: it explains *how to translate and structure problems* so they can be typed, specified, implemented, verified with evidence, and approved with explicit human authority.

**Platform problem statement (meta-instantiation):**  
> **How to generate trustworthy knowledge artifacts using non-deterministic agents.**  

(Equivalent phrasing used elsewhere: “consistently generate trustworthy knowledge in agentic workflows.”)

---

## 0. How to interpret this document

### 0.1 Binding vs guidance

This paradigm is **normative process guidance**.

- **Binding (MUST/SHALL)** in this document constrains:
  - what a “structured, aligned problem statement” must include,
  - the minimum workflow stages and acceptance checks,
  - how agents must treat their own outputs (proposal vs commitment),
  - how to handle conflicts (route through change control).

- **Guidance (SHOULD/MAY)** provides:
  - templates and checklists,
  - recommended agent role mappings,
  - examples.

### 0.2 Compatibility and precedence

This document MUST be interpreted as compatible with the canonical SOLVER‑Ralph governed set:

1) **SR-CONTRACT** (binding invariants)  
2) **SR-SPEC** (binding mechanics)  
3) **SR-DIRECTIVE** (execution policy)  
4) **SR-INTENT** (directional rationale)  
5) **SR-README** (index/navigation)

This paradigm MUST NOT redefine canonical meanings of **Verified**, **Approved**, **Shippable**, **Portal**, **Candidate**, **Evidence Bundle**, or **typed refs**.

**Override rule:** If two artifacts disagree on a binding term or lifecycle meaning, do not “resolve by interpretation.” Route the change through **SR-CHANGE**.

### 0.3 Agents are not judges

Agent statements (e.g., “verified,” “approved,” “compliant,” “shippable”) are always **proposal** unless backed by:

- recorded deterministic oracle evidence (verification), and/or
- recorded human approvals at the defined portals (authority).

---

## 1. Scope

### 1.1 Platform scope (SOLVER / SOLVER‑Ralph)

**Platform problem statement:** SOLVER‑Ralph exists to make it possible to **consistently generate trustworthy knowledge artifacts** even when the primary producers (agents) are non-deterministic.

In this paradigm, “trustworthy knowledge” is not metaphysical truth. It is knowledge that is:

- **attributable** (who/what produced it),
- **replayable** (what inputs/evaluators were used),
- **evidence-backed** (oracle outputs are recorded), and
- **authority-bound** (humans approve at trust boundaries).

### 1.2 What this paradigm governs

This paradigm governs:

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

---

## 2. Relationship to the SOLVER‑Ralph governed set

This paradigm is compatible with (and written to align with) the SOLVER‑Ralph governed set:

- **SR-TYPES** — ontology / artifact taxonomy + metadata rules  
- **SR-ETT** — epistemic trust topology (nine membranes; coercion placement model)  
- **SR-INTENT** — directional “why” and interpretation guidance  
- **SR-CONTRACT** — binding invariants + trust boundary rules  
- **SR-SPEC** — binding technical mechanics (events/APIs/state machines)  
- **SR-DIRECTIVE** — phases, gates, stop triggers, verification profiles  
- **SR-CHANGE** — how governance evolves; promotion, selection, and freeze rules  
- **SR-README** — index/navigation for the set  

If a conflict is discovered between this document and binding artifacts, resolve it through **SR-CHANGE** rather than interpretation.

---

## 3. Definitions

### 3.1 Problem statement family

- **Raw problem statement (RPS):** the human’s initial description (ambiguous is normal).
- **Interpreted problem statement (IPS):** a clarified restatement in the human’s language, with explicit scope and non-goals.
- **Structured, aligned problem statement (SAPS):** an IPS rewritten into a governance-ready structure: requirements are classified by what can be verified with oracles vs what needs human arbitration; it names deliverables, evidence expectations, and authority boundaries.
- **Instantiated problem statement (INPS):** the SAPS expressed across the governed artifact set (TYPES/INTENT/CONTRACT/SPEC/DIRECTIVE/CHANGE), so SOLVER‑Ralph can execute it.

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

---

## 4. Core design principles

### 4.1 Documentation is infrastructure

In SOLVER‑Ralph, documents are not tertiary. They are the **meaning substrate** that:

- prevents vocabulary drift,
- replaces transcript memory as a source of truth,
- enables replayable audits and deterministic context compilation,
- makes agents substitutable without losing coherence.

### 4.2 Coerce at membranes, not everywhere

The system preserves exploration inside an iteration while coercing outputs **only when they become load-bearing inputs**.

Practically: proposals stay fluid until they are written into commitment objects (artifacts, events, evidence, approvals, freezes, change records).

### 4.3 Verification produces evidence; approval binds responsibility

- **Verification** is an evidence-backed claim grounded in deterministic oracle outputs.
- **Approval** is a human trust-boundary action that binds responsibility for moving forward.

Approval MUST NOT be used as a substitute for verification evidence.

### 4.4 The governed set is recursive by design

The governed artifact set is not written once. It is tightened through recursion:

- once meaning is instantiated into a tech stack and concrete interfaces,
- once change-control semantics exist,
- once implementation reveals mismatches and hidden assumptions.

### 4.5 Mapping to SR-ETT (nine harness membranes)

This paradigm is document-centric because governed artifacts are the primary mechanism for placing coercion boundaries.

A mapping from SR-ETT’s membranes to the authoring workflow:

| SR-ETT harness membrane | Where it is expressed in the workflow | What it stabilizes |
|---|---|---|
| Intent & Objective | S0/S0a → INTENT | what “success” means; prevents goal drift |
| Ontological | TYPES | what exists; prevents category errors |
| Architectural | CONTRACT (+ architecture proposal) | boundaries and invariants |
| Operational | DIRECTIVE | executable phases/gates/stop triggers |
| Authority & Integrity | CONTRACT + SPEC + portals | who can commit; admissibility |
| Accountability | SPEC evidence/records + freeze | replayability and completeness |
| Isomorphic | SPEC ↔ implementation conformance | prevents spec/runtime drift |
| Resource | DIRECTIVE budgets | feasibility; prevents runaway loops |
| Change | CHANGE | controlled evolution; prevents silent meaning shifts |

---

## 5. Canonical authoring workflow

This section defines the recommended authoring sequence. Each stage describes:

- **Inputs:** what must exist before the stage begins
- **Outputs:** what the stage produces
- **Acceptance checks:** how to decide the stage is complete enough to proceed
- **Failure routing:** what to do when the stage reveals a mismatch

This workflow intentionally begins with ontology (TYPES) before intent, and with intent before contract/spec.

### S0 Alignment on interpreted problem statement (RPS → IPS)

**Purpose:** reach agreement on what problem is being solved before writing governance.

**Inputs**
- Raw human problem statement (may be incomplete)
- Candidate constraints, stakes, and non-goals

**Outputs**
- Interpreted problem statement (IPS) (scoped, testable, with explicit non-goals)
- A short list of unknowns that will be resolved through governance iteration
- Assumptions + triggers (what would cause a revisit)

**Acceptance checks (MUST)**
- The human can state “what done means” without contradiction.
- The IPS can be read back to the human and the human can say “Yes, that’s what I mean” (or provide corrections).
- The scope is small enough to be governed with the intended artifact set.

**Failure routing**
- If alignment cannot be reached, do not start governance authoring. Continue clarification.

---

### S0a Structured, aligned problem statement (IPS → SAPS)

**Purpose:** turn the IPS into a governance-ready structure that already implies the downstream governed artifacts.

**Inputs**
- Interpreted problem statement (S0)

**Outputs**
- A Structured, Aligned Problem Statement (SAPS) using the template in **Appendix A**
- A classification table of requirements:
  - oracle-verifiable,
  - portal-arbitrated,
  - out-of-scope / deferred (with rationale)

**Acceptance checks (MUST)**
- Every requirement is classified (oracle vs portal vs out-of-scope/deferred).
- SAPS contains at least one executable path to completion:
  - deliverables are listed,
  - and an oracle/evidence plan exists in principle.
- SAPS identifies the earliest trust boundary (where human judgment is required).

**Failure routing**
- If requirements cannot be classified, return to S0 and clarify.
- If “done” cannot be reduced to any evidence plan, the problem statement is not yet SOLVER‑Ralph compatible (or assumptions must change).

---

### S1 TYPES initial

**Purpose:** establish the forcing ontology (what exists; what words mean).

**Inputs**
- SAPS (S0a)

**Outputs**
- TYPES artifact draft defining:
  - artifact types and authority model
  - metadata conventions
  - precedence and conflict handling
  - relationship semantics (semantic dependency vs audit provenance)

**Acceptance checks**
- A new author can classify any planned artifact into a type.
- The document prevents category errors (e.g., “approval” vs “review”, “verification” vs “belief”).

**Failure routing**
- If disagreement on terminology persists, resolve it in TYPES before writing INTENT.

---

### S2 INTENT initial

**Purpose:** anchor the reference signal (what matters; tradeoffs among compliant options).

**Inputs**
- SAPS (S0a)
- TYPES (S1)

**Outputs**
- INTENT artifact draft defining:
  - what the system is protecting against (failure modes)
  - assumptions + triggers that cause escalation
  - rationale for constraints, boundaries, and trust model

**Acceptance checks**
- The human can recognize their intent in the artifact.
- The artifact is explicit about how it should be interpreted (directional vs binding).

**Failure routing**
- If intent is not stable, iterate with S0/S0a and then revise INTENT.

---

### S3 Architecture proposal and tech stack research

**Purpose:** instantiate abstract intent into the constraints of a real stack.

**Inputs**
- SAPS (S0a)
- TYPES (S1)
- INTENT (S2)

**Outputs**
- A proposed architecture boundary model (ports/adapters, trust boundaries)
- Stack research and selection sufficient to write SPEC without inventing a new stack later
- Identified constraints: where determinism is hard; where portals are unavoidable

**Acceptance checks**
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

**Acceptance checks**
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

**Acceptance checks**
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

**Acceptance checks**
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

**Acceptance checks**
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

**Acceptance checks**
- Gates are executable (objective oracle checks exist or can exist).
- Stop triggers prevent silent drift and false foundations.
- The directive does not invent new portal semantics.

**Failure routing**
- If gates cannot be made executable, revise deliverables decomposition (SAPS) and/or SPEC.

---

### S9 CHANGE

**Purpose:** make governance evolution explicit and safe.

**Inputs**
- The canonical governed artifacts (S1–S8)

**Outputs**
- CHANGE artifact specifying:
  - how versions evolve and supersede,
  - how “current” selection is made,
  - how freezes/baselines are recorded,
  - how exceptions (deviation/deferral/waiver) are created and reviewed,
  - portal routing for governance-impacting changes.

**Acceptance checks**
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

**Acceptance checks**
- Cross-artifact terms are consistent.
- No binding artifact relies on unrecorded “chat memory.”

---

### S11 Coherence audit

**Purpose:** run an explicit coherence check before publication/implementation.

**Inputs**
- A candidate governed snapshot (selected versions)

**Outputs**
- A coherence report (may be checklist-based)
- If failures exist: a change request or remediation plan

**Acceptance checks**
- Ontology/epistemology/semantics/isomorphism/change/resource coherence all pass or are explicitly addressed.

---

### S12 Publish for implementation

**Purpose:** declare a coherent, usable governance snapshot as the baseline for implementation work.

**Inputs**
- Coherence audit results (S11)

**Outputs**
- A published snapshot selection (and/or freeze record when applicable)
- Explicit references to the governed artifact versions that are in force

**Acceptance checks**
- Implementers can rely on stable meanings without guessing.
- Any known exceptions are explicit and visible.

---

### S13 Agent specifications

**Purpose:** define how agents will operate within the now-stable governance semantics.

**Inputs**
- Published governance snapshot (S12)
- SR-AGENTS (agent object model + taxonomy)

**Outputs**
- `config.agent_definition` artifacts and/or role specs consistent with SR-AGENTS and SR-TYPES
- Tooling constraints: sandbox, allowed tools, context compilation rules

**Acceptance checks**
- Agent roles/prohibitions are stated in terms of stable governed semantics (not ad-hoc chat rules).
- Agent definitions do not grant human-only authority.

---

### S14 Plan and tests

**Purpose:** compile work into loop-completable tasks with evidence plans.

**Inputs**
- Published governance snapshot (S12)
- Agent specs (S13)

**Outputs**
- Implementation plan: phases → packages → tasks
- Oracle plan: what evidence will be produced per task/gate
- Initial test scaffolding sufficient to make progress measurable

**Acceptance checks**
- Tasks are sized so that progress can be observed within loop budgets.
- Each task has an evidence-producing completion condition.

---

## 6. Appendix A — SAPS template (Structured, Aligned Problem Statement)

This template is normative for stage S0a.

> **Design note:** The template forces “solution shape” by requiring explicit evidence and authority boundaries.

### A.1 Header

- **Problem ID:** stable short identifier (string)
- **Version:** SemVer string (drafts allowed)
- **Status:** draft / proposed / accepted (local lifecycle; not a portal)
- **Author(s):** human + agent attributions (not authority claims)
- **Date:** ISO 8601 date

### A.2 Objective and non-goals

**MUST include:**
- **Objective:** what outcome is desired (one paragraph).
- **Non-goals:** what is explicitly not being solved.
- **Definition of “done”:**
  - outcome-level (human-facing),
  - artifact-level (what artifacts exist when done),
  - verification-level (what evidence exists).

### A.3 Stakeholders and risk

**MUST include:**
- Stakeholders (who cares; who approves)
- Risk classification (low/medium/high) and why
- “Cost of wrong work” narrative (why conservative blocking might be acceptable)

### A.4 Scope and constraints

**MUST include:**
- In-scope boundaries
- Hard constraints (legal/security/operational constraints)
- Soft constraints (preferences/tradeoffs)
- Known assumptions + triggers that cause re-evaluation

### A.5 Deliverables and artifact types

**MUST include:**
- Primary deliverables (e.g., code, report, dataset, policy)
- For each deliverable:
  - whether it is a governed artifact, a record/evidence artifact, or a work product,
  - what audience consumes it (humans, system, or both),
  - what it depends on (semantic dependencies vs audit provenance).

### A.6 Verification and validation strategy

**MUST include:**
- **Verification plan** (oracle-verifiable):
  - candidate identity boundaries (what is a Candidate),
  - oracle suite candidates (required vs advisory groups),
  - evidence bundle expectations.
- **Validation plan** (portal-arbitrated):
  - what must be decided by humans,
  - what evidence humans will review,
  - what portals (existing) are relevant (do not invent new portals).

### A.7 Authority and responsibility boundaries

**MUST include:**
- Which actions are human-only trust boundary actions
- Which actions are system-only
- Which actions agents may propose/execute

### A.8 Decomposition into work units

**MUST include:**
- proposed decomposition into work units that are loop-completable:
  - each work unit has a goal,
  - expected deliverables,
  - expected oracles for progress and completion.

### A.9 Budgets and stop conditions

**MUST include:**
- max iterations / time / cost (or explicitly accept defaults)
- any additional stop-the-line triggers or escalation thresholds (must remain compatible with SR-DIRECTIVE + SR-CONTRACT)

### A.10 Open questions and resolution plan

**MUST include:**
- open questions that block SPEC-level instantiation,
- how they will be answered (research, oracle prototype, human decision).

---

## 7. Mapping SAPS → governed artifacts

Agents SHOULD use SAPS as the “compiler input” to governance authoring:

- SAPS terms → TYPES
- SAPS objective/tradeoffs/failure modes → INTENT
- SAPS invariants/authority boundaries → CONTRACT
- SAPS mechanics/evidence expectations → SPEC
- SAPS execution/budgets/profiles → DIRECTIVE
- SAPS evolution/selection/exceptions → CHANGE

Any new type keys proposed by SAPS MUST be routed through SR-CHANGE if they are intended to become machine-enforced.

---

## 8. Recursion and coherence

### 8.1 Recursion model

Recursion occurs when:

- the chosen stack forces an update to types or interfaces,
- contract invariants prove unenforceable without a design change,
- directive gates are not actually executable (no objective tests),
- change control reveals missing classification or routing rules,
- implementation reveals drift between enforcement and intended meaning.

**Recursion principle:**  
> When meaning changes, write it into governed artifacts and advance versions rather than letting it live in transient conversation context.

### 8.2 Coherence audit checklist (recommended)

Before publishing a governance snapshot (or declaring a work unit ready for implementation), the following coherence checks SHOULD be applied and automated as oracles where feasible:

1) **Ontological coherence:** types complete; no category errors.
2) **Epistemic coherence:** verification ↔ oracles/evidence; approval ↔ portals/records.
3) **Semantic coherence:** terms used consistently across artifacts.
4) **Isomorphic coherence:** SPEC implements CONTRACT; DIRECTIVE can produce required evidence.
5) **Change coherence:** evolution paths are explicit and routed correctly.
6) **Resource coherence:** budgets/stop triggers make loops feasible.

---

## 9. Agents and why SR-AGENTS looks the way it does

SR-AGENTS defines **what an Agent is** (object category) and a foundational taxonomy of agent subtypes.

This paradigm depends on that taxonomy because it divides labor **by membrane function**, not by “model capability”:

- **Work-plane agents** produce candidates and drafts.
- **Advisory-plane agents** reduce ambiguity, improve evidence legibility, and route decisions—without gaining authority.

### 9.1 Agent role mapping to stages (guidance)

| Stage | Primary agent subtype(s) | Primary outputs |
|---|---|---|
| S0 (RPS→IPS) | Context Scout, Governance Scribe | IPS draft; unknowns; assumptions + triggers |
| S0a (IPS→SAPS) | Context Scout, Governance Scribe, Decision Router | SAPS draft; requirement classification |
| S1 TYPES | Governance Scribe | type keys; relationship semantics; metadata skeletons |
| S2 INTENT | Governance Scribe | failure modes; tradeoffs; assumptions + triggers |
| S3–S5 stack research | Agent Worker (research), Context Scout | candidate stack options; constraint fit analysis |
| S6 CONTRACT | Governance Scribe, Decision Router | invariant list; trust boundary mapping |
| S7 SPEC | Governance Scribe + Agent Worker | event/state/evidence models; API surfaces |
| S8 DIRECTIVE | Governance Scribe | phases/gates; budgets; profiles |
| S9 CHANGE | Governance Scribe | change classes; portal routing; selection/freeze constraints |
| Loop execution | Agent Worker, Evidence Triage | candidates; evidence summaries; iteration summaries |
| Escalations | Decision Router | decision options; portal routing recommendation |

### 9.2 Agent wave model

SOLVER‑Ralph uses a three-wave agent strategy:

1) **A1: Bootstrap builder agents** — build SOLVER‑Ralph itself (runtime + governance harness).  
2) **A2: Instance/domain agents** — use SOLVER‑Ralph to resolve a domain problem statement.  
3) **A3: Embedded domain workers** — agents embedded into an instance to repeatedly solve a class of problems.

Across all waves:

- agents generate proposals and work products,
- oracles generate evidence,
- humans approve at trust boundaries.

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

### 10.2 Deliverables, verification, and approval order

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

1) A SOLVER‑Ralph effort MUST begin with **S0** and **S0a** (alignment + structured problem statement).
2) SAPS MUST classify requirements as oracle-verifiable vs portal-arbitrated vs out-of-scope/deferred.
3) SAPS MUST include a definition of “done” that includes an evidence plan (verification) and responsibility boundaries (authority).
4) The governed artifacts MUST be version-controlled documents with machine-readable metadata consistent with SR-TYPES/SR-SPEC rules.
5) When governed artifacts disagree, conflicts MUST NOT be resolved by interpretation; they MUST be reconciled via SR-CHANGE.
6) Deliverables MUST be defined so objective oracle tests can exist; non-oracle acceptability MUST be routed to portals/decisions rather than implied by agent narration.
7) Agent specifications SHOULD be authored only after the governed set is coherent enough that agent roles/prohibitions can reference stable semantics.
8) The paradigm MAY be instantiated with different stacks/domains, but MUST preserve the authority model, evidence model, and trust boundary constraints defined by SOLVER‑Ralph governance.

---

## 12. Change control

This document is governed by **SR-CHANGE**.

- Changes to this paradigm are governance changes and MUST be versioned.
- If a change alters the meaning of binding terms or modifies how trust boundaries/evidence/verification are interpreted, it MUST be routed through the appropriate portal(s) per SR-CHANGE and SR-DIRECTIVE.

---

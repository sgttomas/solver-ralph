---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-PARADIGM"
  type: "governance.development_paradigm"
  title: "SOLVER Development Paradigm"
  version: "1.0.0-draft.3"
  status: "draft"
  normative_status: "normative"
  authority_kind: "process"
  governed_by: ["SR-CHANGE"]
  supersedes:
    - "SR-PARADIGM@1.0.0-draft.2"
  created: "2026-01-10"
  updated: "2026-01-10"
  tags:
    - "solver"
    - "governance"
    - "development-paradigm"
    - "agentic-workflows"
    - "trustworthy-knowledge"
    - "documentation-as-infrastructure"
    - "coherence"
  ext:
    development_paradigm:
      intent:
        core_problem_statement: "Consistently generate trustworthy knowledge in agentic workflows."
        summary: >
          Defines a governed, recursive development workflow for SOLVER and SOLVER instances
          (including SOLVER‑Ralph) whose core purpose is to consistently generate trustworthy knowledge
          in agentic workflows by converting ambiguous problem statements into controlled, typed, evidence-backed
          commitments with explicit authority boundaries.
      governed_artifacts_minimum_set:
        - "TYPES"
        - "INTENT"
        - "CONTRACT"
        - "SPEC"
        - "DIRECTIVE"
        - "CHANGE"
        - "README/INDEX"
      workflow_stages:
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
          description: "Agents used to build SOLVER/SOLVER‑Ralph itself (the governance runtime + loop machinery)."
        - id: "A2_instance_domain_agents"
          description: "Agents that natively use SOLVER‑Ralph in a specific domain; outputs are problem-resolution artifacts."
        - id: "A3_embedded_domain_workers"
          description: "Agents embedded into a specific instance to repeatedly solve a class of domain problems with required human inputs/approvals."
      assumptions:
        - "Agents are stochastic generators; authoritative commitments require recorded evidence and/or recorded human approvals."
        - "Documentation is infrastructure: governed artifacts are the canonical substrate of meaning for long-running work."
        - "Work can be decomposed into deliverables with objective oracle tests, enabling loop completion."
---

# SOLVER Development Paradigm — SR-PARADIGM v1.0.0-draft.3

**Classification:** Governance policy (process).  
**Status:** Draft (not yet promoted).  
**Purpose:** Define a governed, recursive workflow for developing **SOLVER** and any **SOLVER instance** (including SOLVER‑Ralph) such that:

- meaning is stabilized by controlled documents (not transcript memory),
- work is compiled into executable structures (phases → packages → tasks),
- verification yields objective evidence (oracles),
- approvals remain human-only trust boundary actions,
- the whole system can iterate without semantic drift.


**Core problem statement (for SOLVER itself):** Consistently generate **trustworthy knowledge** in agentic workflows — i.e., outcomes whose admissibility is grounded in typed provenance, evidence-backed verification (oracles), and human authority at trust boundaries.

This document is written so a new author (human or agent) can reproduce the paradigm without needing prior chat/session context.

---


## 0. Version changes

### 1.0.0-draft.3 (2026-01-10)

- Fixes YAML frontmatter to conform to `solver-ralph.artifact-metadata/v1` nesting requirements.
- Aligns the artifact type naming to `governance.development_paradigm` (with a note that type registration occurs via SR-CHANGE).
- Clarifies publication semantics as a change-controlled selection of a coherent governance snapshot (with evidence + human approval records), without inventing new portal semantics.
- Adds an explicit coherence-audit checklist to make ontology/epistemology/semantics/isomorphism review executable as oracles.
- Adds an explicit mapping from the authoring workflow to SR-ETT harness membranes to explain why documentation is a central control surface.

### 1.0.0-draft.2 (2026-01-10)

- Adds explicit statement that SOLVER’s platform-level problem statement is to consistently generate trustworthy knowledge in agentic workflows.

---

## 1. Scope

This paradigm governs:

### 1.1 Platform problem statement (SOLVER itself)

**Problem statement (platform scope):** SOLVER exists to **consistently generate trustworthy knowledge in agentic workflows**.

In this paradigm, “trustworthy knowledge” is not metaphysical truth. It is knowledge that is:
- attributable (who/what produced it),
- replayable (what inputs and evaluators were used),
- evidence-backed (oracle outputs are recorded), and
- authority-bound (humans approve at trust boundaries).


- how to go from a problem statement to a *published* governed artifact set,
- how to recursively iterate that set as reality changes,
- how to introduce agent specifications after publication,
- how to produce an implementation plan where deliverables are loop-completable.

This paradigm does **not** govern:

- runtime portal semantics, verification mode semantics, or candidate identity rules (those live in SR-CONTRACT / SR-SPEC),
- the concrete implementation tech stack for a particular instance (those choices are instantiated into that instance’s SPEC),
- operational incident response or deployment runbooks (those may exist as adjacent artifacts).

---

## 2. Relationship to the SOLVER-Ralph governed set

SOLVER‑Ralph is a concrete instance of the SOLVER paradigm.

This document is compatible with (and written to align with) the SOLVER‑Ralph governed set:

- **SR-TYPES** — artifact taxonomy + metadata rules  
- **SR-INTENT** — directional “why” and interpretation guidance  
- **SR-CONTRACT** — binding invariants and trust boundary constraints  
- **SR-SPEC** — binding technical mechanics (events/APIs/state machines)  
- **SR-DIRECTIVE** — phased execution model + gates + stop triggers + verification profiles  
- **SR-CHANGE** — change control, versioning, portal routing, and selection/freeze rules  
- **SR-ETT** — epistemic trust topology (membranes / harnesses)  
- **SR-README** — index/navigation for the set

If a conflict is discovered between this document and binding artifacts, resolve it through SR-CHANGE rather than by interpretation.

---

## 3. Definitions

### 3.1 Problem statement and alignment

- **Problem statement:** the human’s initial description of what needs to be solved.
- **Interpreted problem statement:** a clarified, scoped interpretation of the problem statement that is stable enough to begin governance authoring.
- **Alignment:** a human judgment that the interpreted problem statement is sufficiently correct that writing governance artifacts will reduce entropy rather than create it.

### 3.2 Governed artifact set

- **Governed artifact:** a controlled document with stable id, versioning, and machine-readable metadata (YAML frontmatter) governed by SR-CHANGE.
- **Governed artifact set:** the minimal set of artifacts required to stabilize meaning and constrain implementation (TYPES, INTENT, CONTRACT, SPEC, DIRECTIVE, CHANGE, plus index/orientation and adjacent policy as needed).

### 3.3 Instance, iteration, and recursion

- **SOLVER instance:** a specific instantiation of SOLVER governance + runtime in a given tech stack and domain.  
  Example: SOLVER‑Ralph is an instance optimized for Ralph Loops + oracle evidence + human portals.
- **Governance iteration:** a controlled evolution of the governed artifact set (and its selected “current” versions) via SR-CHANGE.
- **Recursion:** the intentional cycle of revisiting and tightening artifacts after the system’s constraints become fully expressed (especially after CHANGE exists).

### 3.4 Proposal vs commitment

- **Proposal:** an uncommitted artifact or statement (including agent output) that has not crossed an evidence or authority membrane.
- **Commitment object:** a durable, attributable, replayable object the system is allowed to treat as “real.”  
  Examples: a governed artifact version, an evidence bundle manifest, an approval record, a freeze record, a decision record.

---

## 4. Core design principles

### 4.1 Documentation is infrastructure

In SOLVER, documents are not tertiary. They are the **meaning substrate** that:

- prevents vocabulary drift,
- replaces transcript memory as a source of truth,
- enables replayable audits and deterministic context compilation,
- makes agents substitutable without losing state coherence.

### 4.2 Coerce at membranes, not everywhere

The system preserves exploration inside an iteration while coercing outputs **only when they become load-bearing inputs**.

Practically: proposals stay fluid until they are written into commitment objects (artifacts, events, evidence, approvals, freezes, change records).

### 4.3 Verification produces evidence; approval binds responsibility

- Verification is an evidence-backed claim grounded in oracle outputs.
- Approval is a human trust-boundary action that binds responsibility for moving forward.

This separation prevents “foundation corruption” where narrative plausibility is mistaken for reality, and is the primary mechanism by which SOLVER makes knowledge outputs trustworthy under stochastic labor.

### 4.4 The governed set is recursive by design

The governed artifact set is not written once. It is tightened through recursion:

- once meaning is instantiated into a tech stack and concrete interfaces,
- once change-control semantics exist,
- once implementation reveals mismatches and hidden assumptions.

---


### 4.5 Mapping to SR-ETT (nine harness membranes)

This paradigm is deliberately *document-centric* because the governed artifacts are the primary mechanism for placing coercion boundaries.

A quick mapping from SR-ETT’s membranes to the authoring workflow:

| SR-ETT harness membrane | Where it is expressed in the workflow | What it stabilizes |
|---|---|---|
| Intent & Objective | **Problem statement alignment → INTENT** | what “success” means; prevents goal drift |
| Ontological | **TYPES** | the vocabulary of objects/relations; prevents category errors |
| Architectural | **CONTRACT** (+ architecture proposal) | boundaries, invariants, separation of concerns |
| Operational | **DIRECTIVE** | executable phases/packages/gates for agents; stop triggers |
| Authority & Integrity | **CONTRACT + SPEC + portals** | who can make commitments; admissibility of claims |
| Accountability | **SPEC evidence/record models + coherence audit** | replayability, completeness, audit trails |
| Isomorphic | **SPEC ↔ implementation conformance + tests** | prevents spec/runtime drift |
| Resource | **DIRECTIVE budgets** | feasibility constraints; prevents runaway loops |
| Change | **CHANGE** | controlled evolution; prevents silent meaning shifts |

This mapping is not “extra documentation.” It is the mechanism by which meaning remains stable across long runs with stochastic workers.

---

## 5. Canonical authoring workflow

This section defines the recommended authoring sequence. Each stage describes:

- **Inputs:** what must exist before the stage begins
- **Outputs:** what the stage produces
- **Acceptance checks:** how to decide the stage is complete enough to proceed
- **Failure routing:** what to do when the stage reveals a mismatch

This workflow intentionally begins with ontology (TYPES) before intent, and with intent before contract/spec.

### S0 Alignment on interpreted problem statement

**Inputs**
- Initial human problem statement (may be incomplete)
- Candidate constraints, stakes, and non-goals

**Outputs**
- Interpreted problem statement (scoped, testable, with explicit non-goals)
- A short list of “unknowns” that will be resolved through governance iteration

**Acceptance checks**
- The human can state “what done means” without contradiction.
- The scope is small enough to be governed with the intended artifact set.

**Failure routing**
- If alignment cannot be reached, do not start governance authoring. Continue clarification.

---

### S1 TYPES initial

**Purpose:** establish the forcing ontology (what exists; what words mean).

**Inputs**
- Interpreted problem statement (S0)

**Outputs**
- TYPES artifact draft defining:
  - artifact types and authority model
  - metadata conventions
  - precedence and conflict handling
  - core relationship semantics (dependency vs provenance)

**Acceptance checks**
- A new author can classify any planned artifact into a type.
- The document prevents category errors (e.g., “approval” vs “review”, “verification” vs “belief”).

**Failure routing**
- If disagreement on terminology persists, resolve in TYPES before writing INTENT.

---

### S2 INTENT initial

**Purpose:** anchor the reference signal (what matters; tradeoffs among compliant options).

**Inputs**
- Interpreted problem statement (S0)
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
- If intent is not stable, iterate with S0 and then revise INTENT.

---

### S3 Architecture proposal and tech stack research

**Purpose:** instantiate abstract intent into the constraints of a real stack.

**Inputs**
- TYPES (S1)
- INTENT (S2)

**Outputs**
- A proposed architecture boundary model (ports/adapters, trust boundaries)
- Tech stack research and selection sufficient to write SPEC without inventing a new stack later

**Acceptance checks**
- The stack is concrete enough to support:
  - event store + projections
  - evidence storage
  - oracle sandbox execution
  - identity and approval recording
- The architecture is compatible with the invariants required by the contract model.

**Failure routing**
- If the stack cannot satisfy critical constraints, revise S2 (intent tradeoffs) or select a different stack.

---

### S4 Iterate TYPES and INTENT for the chosen stack

**Purpose:** update ontology and intent so they survive contact with implementation reality.

**Inputs**
- Stack + architecture proposal (S3)
- TYPES (S1)
- INTENT (S2)

**Outputs**
- TYPES iteration: new object categories, refs, records, evidence types, boundary clarifications
- INTENT iteration: updated assumptions/triggers, clarified failure modes

**Acceptance checks**
- TYPES now includes whatever must exist for the SPEC to be concrete.
- INTENT remains concise but specific enough to guide contract tradeoffs.

---

### S5 CONTRACT

**Purpose:** bind invariants and trust boundaries.

**Inputs**
- TYPES (iterated)
- INTENT (iterated)
- Architecture boundary model + stack constraints

**Outputs**
- CONTRACT artifact specifying:
  - invariants that implementations cannot violate
  - trust boundary rules (who can do what)
  - integrity conditions and stop-the-line conditions
  - core evidence + approval linkage requirements

**Acceptance checks**
- The contract can be tested (at least in principle) by deterministic checks and/or portal records.
- The contract is implementation-agnostic: it binds behavior, not libraries.

**Failure routing**
- If an invariant cannot be enforced, revise architecture/stack or narrow scope.

---

### S6 SPEC

**Purpose:** define concrete interfaces and mechanics that implement the contract in the chosen stack.

**Inputs**
- CONTRACT
- TYPES + INTENT
- Stack and architecture proposal

**Outputs**
- SPEC artifact with:
  - events, schemas, state machines
  - API surface and authorization boundaries
  - storage model (event store, projections, object store)
  - oracle suite definitions and evidence models
  - worker integration contract (inputs/outputs, forbidden actions)

**Acceptance checks**
- A team could implement without “inventing missing semantics.”
- The spec preserves the contract’s authority model and integrity constraints.

**Failure routing**
- If SPEC adds semantics that change the contract meaning, route through change control or revise CONTRACT.

---

### S7 DIRECTIVE

**Purpose:** compile work into an executable structure.

**Inputs**
- CONTRACT + SPEC
- TYPES + INTENT

**Outputs**
- DIRECTIVE artifact defining:
  - phase structure and deliverables as packages
  - gate/exit criteria and stop triggers
  - verification profiles (oracle suites + modes)
  - portal inventory and routing (without new portal semantics)

**Acceptance checks**
- Each deliverable has:
  - a concrete output definition
  - objective tests (oracles) that generate evidence
  - an explicit gate/exit check
- The directive enables loop completion (work can advance deterministically or escalate cleanly).

---

### S8 CHANGE

**Purpose:** define how the governed set evolves without drift.

**Inputs**
- TYPES + INTENT + CONTRACT + SPEC + DIRECTIVE

**Outputs**
- CHANGE artifact defining:
  - what requires change control
  - versioning rules and classification
  - portal routing for governance changes and exceptions
  - selection/freeze rules and staleness obligations

**Acceptance checks**
- The system can evolve without rewriting history.
- It is clear how to change meaning safely.

---

### S9 Post-change intent refresh

**Purpose:** update intent once the system’s change mechanics exist.

**Inputs**
- CHANGE + prior artifacts

**Outputs**
- INTENT iteration that:
  - reflects the realities of change control and evidence demands
  - captures newly discovered triggers and tradeoffs

**Acceptance checks**
- Intent and change control are consistent: the paradigm does not rely on “tribal knowledge.”

---

### S10 Recursive iteration across the governed set

**Purpose:** tighten coherence and eliminate contradictions.

**Inputs**
- Full governed set from S1–S9

**Outputs**
- Iterated versions of:
  - TYPES → CONTRACT → SPEC → DIRECTIVE → CHANGE (and INTENT as needed)

**Acceptance checks**
- Definitions are consistent across artifacts.
- No category errors remain (review vs approval, evidence vs authority).

---

### S11 Coherence audit

**Purpose:** explicit audit for ontology, epistemology, semantics, and spec↔runtime isomorphism.

**Inputs**
- Iterated governed set (TYPES/INTENT/CONTRACT/SPEC/DIRECTIVE/CHANGE and any declared adjuncts such as ETT/README/PARADIGM)

**Outputs**
- A structured coherence report (record artifact or release note) covering:
  - ontological coherence (types and relations)
  - epistemological coherence (what counts as commitment vs proposal)
  - semantic coherence (terminology, precedence, conflict handling)
  - isomorphism (spec ↔ intended runtime behavior)
- Remediation edits to the governed set where required

**Acceptance checks**
- A new reader can answer: “what is real, what is proposal, and what is binding?”
- Cross-references resolve; no duplicate/conflicting definitions remain.

#### S11.1 Coherence audit checklist (executable targets)

This checklist is written to be directly convertible into a “coherence” oracle suite (lint + graph checks + simple parsers).

**Ontological coherence**
- [ ] Every governed artifact’s `type` and `normative_status` matches the SR-TYPES intent (or is explicitly documented as an extension).
- [ ] No “category errors” in language: e.g., *review* is not labeled *approval*; agent judgments are not labeled *verified*.
- [ ] Relationship semantics are consistent: semantic dependencies vs audit provenance are not conflated.

**Epistemological coherence**
- [ ] “Verified / Approved / Shippable” are referenced consistently with SR-CONTRACT/SR-SPEC (no redefinitions).
- [ ] Verification is always evidence-backed; approval always binds a human actor to a subject and evidence.
- [ ] The doc set clearly distinguishes proposals/drafts from commitments/records.

**Semantic coherence**
- [ ] The same term does not carry two incompatible meanings across documents.
- [ ] Precedence and conflict handling is stated consistently (Contract > Spec > Directive > Intent > README).
- [ ] Non-negotiable constraints (e.g., no ghost inputs, SYSTEM-only iteration start) are referenced consistently where relevant.

**Isomorphic coherence (governance ↔ implementation)**
- [ ] Every “binding” rule stated in CONTRACT/SPEC/DIRECTIVE has a plausible enforcement surface (domain invariant, adapter enforcement, oracle, or portal record).
- [ ] Any implementation behavior described as required is also stated in SPEC/CONTRACT (or explicitly flagged as a planned change).
- [ ] Candidate identity, evidence hashing, and provenance rules are consistent end-to-end (no missing fields in the intended data model).

**Process coherence (execution and change)**
- [ ] Change control routing is complete: which changes require SR-CHANGE and which portal(s) apply.
- [ ] Publication criteria are explicit and reference SR-CHANGE (no “hand-wave” publishing).
- [ ] Agent specifications and PLAN/Test artifacts are sequenced after publication and are traceable to DIRECTIVE requirements.

---

### S12 Publish for implementation

**Purpose:** designate a governance snapshot as stable enough to implement against (while remaining change-controlled).

**Inputs**
- Coherent, iterated governed set
- Coherence report from S11

**Outputs**
- A published governance snapshot:
  - versioned governed artifacts with stable ids and content hashes,
  - an updated index/README that points to the exact versions,
  - recorded evidence that coherence checks passed, and
  - the required human approval records for any selection changes (per SR-CHANGE).

**Acceptance checks (publication gate)**
- Coherence audit checklist (S11.1) is completed and issues are either resolved or explicitly recorded as exceptions/decisions.
- The governed set is internally consistent and navigable from the README/index without ambiguity.
- Any change to which versions are designated as “current” (or otherwise selected for implementation) is processed via SR-CHANGE and linked to the appropriate portal approval record(s).

**Implementation note:** publishing does not freeze reality; it establishes a baseline. Divergence discovered during implementation is routed back through SR-CHANGE as a governed update, not silently patched in code.

---

### S13 Agent specifications

**Purpose:** define agent roles only after the governing semantics exist.

**Inputs**
- Published governed set

**Outputs**
- Agent definitions for:
  1) bootstrap builders (building SOLVER itself)
  2) instance/domain agents (solving a specific domain problem using SOLVER‑Ralph)
  3) embedded agents (baked into the instance for recurring domain work)

**Acceptance checks**
- Agent capabilities and prohibitions are explicit.
- Agents do not obtain new authority; they remain proposal generators.

---

### S14 Plan and tests

**Purpose:** define implementation execution so deliverables are loop‑completable.

**Inputs**
- Published governed set
- Agent specs (S13)

**Outputs**
- Implementation plan that:
  - decomposes work into deliverable packages
  - defines objective oracle tests per package
  - defines evidence packaging expectations
  - defines the gating/approval sequence

**Acceptance checks**
- Each deliverable can reach completion inside a loop:
  - evidence exists before approval is requested
  - approvals bind to specific candidates and evidence

---

## 6. Recursion model

Recursion occurs when:

- the chosen stack forces an update to types or interfaces,
- contract invariants prove unenforceable without a design change,
- the directive’s gates are not actually executable (no objective tests),
- change control reveals missing classification or routing rules,
- implementation reveals “drift” between enforcement and intended meaning.

The recursion principle is simple:

> When meaning changes, write it into governed artifacts and advance versions rather than letting it live in transient conversation context.

---

## 7. Publication and commitment objects

Publication is the membrane where others are allowed to rely on the governed set **without re-negotiating meaning**.

In SOLVER terms, publication is not just “we wrote documents.” It is:

- a *selection* of versions (what is considered “in force”),
- with evidence that the selection is coherent,
- and with recorded human approvals where the selection changes binding behavior.

Publication produces (or makes eligible) the core commitment objects (aligned to SR-ETT):

- **Governed artifacts** with stable ids, versions, and content hashes (the meaning substrate).
- **Evidence bundles** that bind oracle outcomes to specific candidates and environments.
- **Approval records** that bind a human decision to a subject and evidence at a trust boundary.
- **Freeze/baseline records** that snapshot what was in force (and enumerate active exceptions).
- **Change records** that describe how the system’s own rules evolved.

---
## 8. Deliverables, verification, and approval order

A deliverable in this paradigm is a **package** with:

- a concrete output definition,
- objective oracle tests that can generate evidence, and
- an explicit gate/exit check that determines whether it can progress.

**Closure rule (cross-document alignment):**  
A deliverable is only presented for human approval after its objective tests have produced recorded evidence. Approval is not used as a substitute for verification.

A minimal execution order (loop-compatible):

1) Produce a candidate work product (code/doc/config) with stable identity.  
2) Run oracles and capture the resulting evidence bundle(s).  
3) Compile the gate packet (candidate + evidence + governed refs + exceptions).  
4) Present the verified output for human judgment at the relevant trust boundary (portal) when required.  
5) Record the approval decision, referencing the subject and evidence explicitly.

This ordering exists so loops can run to completion: agents can iterate deterministically until evidence is ready, and humans evaluate evidence rather than narratives.

---
## 9. Agent wave model

SOLVER uses a three-wave agent strategy:

1) **Bootstrap builder agents**  
   Build SOLVER / SOLVER‑Ralph itself: governance runtime, event store, projections, oracle runner, portals, and loop machinery.

2) **Instance/domain agents**  
   Use a deployed SOLVER‑Ralph instance to resolve a specific problem statement in a particular domain.  
   Output: a set of artifacts that constitutes the resolution (software, report, dataset, analysis, etc.).

3) **Embedded domain workers**  
   Built into a specific SOLVER‑Ralph instance so the system can repeatedly produce work artifacts for that entire class of problems, while preserving required human inputs and approvals.

Across all waves:

- agents generate proposals and artifacts,
- oracles generate evidence,
- humans approve at trust boundaries.

---

## 10. Normative requirements

This section is normative. The keywords **MUST**, **SHOULD**, and **MAY** are used intentionally and sparingly.

1) A SOLVER instance **MUST** define and maintain a governed artifact set sufficient to stabilize meaning and constrain implementation, including at minimum: TYPES, INTENT, CONTRACT, SPEC, DIRECTIVE, and CHANGE (plus an index/orientation artifact).

2) The governed artifacts **MUST** be versioned controlled documents with stable identity and machine-readable metadata (YAML frontmatter) consistent with SR-TYPES/SR-SPEC metadata rules.

3) When governed artifacts disagree, the conflict **MUST NOT** be resolved by informal interpretation; it **MUST** be reconciled via change control (SR-CHANGE).

4) Deliverables **MUST** be defined so that objective oracle tests exist and can be executed to produce recorded evidence. Where objective testing is not possible, the acceptability judgment **MUST** be explicitly routed to a human trust boundary (portal) rather than being implied by agent narration.

5) A deliverable **MUST** be verified by testing (evidence produced and recorded) before it is presented for approval, so that a loop can run to completion without relying on approval as a substitute for verification.

6) Agent specifications **SHOULD** be authored only after the governed set has been published for implementation, so agent roles and prohibitions can reference stable semantics.

7) The paradigm **MAY** be instantiated with different stacks or orchestration substrates, provided the instance’s SPEC and CONTRACT preserve the authority model, evidence model, and trust boundary constraints.

---

## 11. Change control

This document is governed by **SR-CHANGE**.

- Changes to this paradigm are governance changes and must be versioned.
- If a change alters the meaning of binding terms or modifies trust boundary behavior, route through the appropriate portals as defined by SR-CHANGE and SR-DIRECTIVE.

---

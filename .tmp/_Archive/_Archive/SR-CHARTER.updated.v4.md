---
doc_id: SR-CHARTER
doc_kind: governance.charter
layer: build
status: draft

refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: depends_on
    to: SR-TYPES
  - rel: constrained_by
    to: SR-CONTRACT
  - rel: informs
    to: SR-SPEC
  - rel: informs
    to: SR-WORK-SURFACE
  - rel: informs
    to: SR-PROCEDURE-KIT
  - rel: informs
    to: SR-SEMANTIC-ORACLE-SPEC
  - rel: informs
    to: SR-EVENT-MANAGER
  - rel: informs
    to: SR-MODEL
  - rel: informs
    to: SR-AGENTS
  - rel: informs
    to: SR-PLAN
  - rel: informs
    to: SR-DIRECTIVE
  - rel: informs
    to: SR-EXCEPTIONS
---

# SR-CHARTER (Semantic)
## Purpose

## Canonical index and navigation

This charter stays intentionally concise. The canonical document index and folder meanings for this spec set live in `README.md`.

Quick map for documents referenced in this charter (per the README folder meanings):

- `charter/`: SR-CHARTER
- `platform/` (**binding**): SR-CONTRACT, SR-SPEC, SR-TYPES, SR-EVENT-MANAGER, SR-WORK-SURFACE, SR-PROCEDURE-KIT, SR-SEMANTIC-ORACLE-SPEC
- `build-governance/` (**binding for development**): SR-CHANGE, SR-AGENTS, SR-EXCEPTIONS
- `program/` (**binding for a specific build/run**): SR-PLAN, SR-DIRECTIVE
- `orientation/` (**non-binding**): SR-MODEL


SOLVER-Ralph exists to make **agentic semantic knowledge work** controlled, governable, and auditable.

It is a platform where:
- agents can perform real knowledge work and produce **semantic artifacts** (analysis, decisions, plans, narratives, ontologies),
- **semantic oracles** (e.g., meaning-matrix / manifold evaluators) produce structured evidence deterministically under a declared procedure stage,
- and **human authority** is required at trust boundaries for any binding decision.

This charter defines the initial human inputs and boundaries that initiate development, and the rules that govern development while agents write **100% of the code** and draft **100% of the governed documentation**, in service of running **Semantic Ralph Loops**: stage-gated, proceduralized work units operating on a defined **work surface** (intake + procedure) with evidence-backed gating.

---

## First Principles

### The platform is governed semantics, not narrative
The platform is not a document viewer, and a “good story” is not correctness. The governed documents define the normative semantic substrate from which enforcement behavior is derived: how work is decomposed, how candidates are generated, how oracles evaluate, and how state transitions are authorized.

### Authority is a human boundary, not an emergent property
Evidence supports decisions. Evidence does not become authority.
Binding outcomes require explicit human authorization at defined trust boundaries.

### Agents are actuators under governance
Agents may propose, implement, and draft.
Agents may not self-grant correctness, safety, approval, release readiness, authority, or semantic validity.
“Verified” means “verified under a declared oracle suite/profile and stage,” not “true in the world.”

### Semantic work requires a work surface
Semantic work is made auditable by constraining it to a declared **work surface**:
- a typed intake (human problem statement, free-form allowed) that is normalized into a structured charter/work unit intake,
- and a proceduralized, stage-gated candidate-generation procedure whose artifacts enable specific oracles.

---

## Layers and Domains

SOLVER-Ralph is understood through three layers:

- **Layer 1 — Build:** agents read governed docs and produce code/docs under a governed workflow, including work-surface schemas and semantic oracle interfaces.
- **Layer 2 — Platform:** the spec is realized as enforcement logic (invariants, gates, records, oracles, portals, projections).
- **Layer 3 — Usage:** users experience the spec as platform behavior and API affordances for semantic work units, evidence, and decisions.

The build domain is governed so that the runtime semantic domain can be made trustworthy.

---

## Immediate Objective

### Milestone 1: MPV = Get Semantic Work Units (“Semantic Ralph Loops”) functioning as intended
The first deliverable is our MVP which is a functioning **semantic work-unit runtime**:
- bounded agentic work,
- iteration cycling with controlled memory,
- a declared work surface (intake + procedure stages),
- deterministic event recording and governance state projection,
- semantic oracle evidence capture and gating,
- explicit human authority points for binding decisions.

“Build Focus: Semantic Ralph Loop MVP”
	•	The only innovation that must be built now is the semantic-ralph-loop runtime: bounded semantic work → structured evidence → human approval → completion event.
	•	All other components (storage engines, UIs, orchestration frameworks, decomposition automation, adapter ecosystems) are commodities and may be implemented as minimal stubs to support proving the loop.
	•	Any work that does not directly enable the semantic-ralph-loop MVP is out of scope unless explicitly authorized by human authority.

Non-goals (for this MVP phase):
	•	not building a full platform
	•	not building a general adapter ecosystem
	•	not optimizing infra
	•	not automating decomposition unless required to demo one loop end-to-end

Stop Rule (for this MVP phase):
	•	“If implementation work starts to drift into adapter/platform build-out before the semantic-ralph-loop MVP is demonstrated end-to-end, stop and escalate.”

Once operational, it becomes the **semantic worker substrate** upon which subsequent platform capabilities and external applications can be built.

### Milestone 2: Enable external application development via API in a hexagonal architecture
After Milestone 1, SOLVER-Ralph becomes a platform other applications integrate with through APIs:
- SOLVER-Ralph exposes ports for semantic work units, candidates, evidence, approvals, and records.
- External apps operate as adapters that call into the platform rather than embedding governance logic.

---

## Canonical Type Seeds (Minimum Vocabulary)

These terms must be used consistently during development:

- **Work Unit (Semantic Loop):** the bounded governed instance of semantic knowledge work.
  - It has a control-plane lifecycle and a governance-progress projection.
- **Iteration:** a fresh-context execution cycle within a work unit.
- **Work Surface:** the declared intake + procedure template (stage-gated) that constrains how candidates are produced.
- **Stage:** a gated phase of the procedure; semantic evaluation is indexed by stage.
- **Proposal:** non-binding intent or draft output produced by an agent.
- **Commitment Object:** a recorded, typed artifact that can support binding claims (candidate, evidence packet, approval, exception, context bundle).
- **Context Bundle (`record.context_bundle`):** the standardized artifact capturing the complete effective context for an iteration (no ghost inputs).
- **Evidence:** oracle output; non-authoritative verification artifacts (including semantic measurements).
- **Semantic Oracle / Suite:** an evaluator that measures candidate adherence to stage-defined semantic constraints (e.g., manifold distance/residuals/coverage), producing structured evidence.
- **Gate:** a required condition for a state transition, computed from records/evidence.
- **Trust Boundary / Authority Port:** a human decision point whose recorded output is binding.
- **Exception:** a narrowly scoped, explicitly recorded permission to deviate from governing documents or work instructions.

Normative status categories:
- **record:** authoritative commitments (decisions, approvals, exceptions, candidates, context bundles)
- **evidence:** oracle outputs and verification artifacts (including semantic measurement traces)
- **directional:** build guidance and scaffolding (plans/directives), still governed but not platform-defining semantics

---

## Authority Model

### Human Authority
A single human authority exists at build time: the project owner (Ryan). Human authority:
- approves binding decisions,
- resolves ambiguity,
- grants or denies exceptions,
- and approves releases/baselines.

### What requires Human Authority (minimum set)
Human authority is required for:
- changes to SR-TYPES and SR-CONTRACT that alter meaning/invariants,
- approval of any binding release/baseline decision,
- any exception to governing documents or work instructions (must be recorded),
- resolving contradictions between governed documents,
- any alteration to trust boundary definitions,
- acceptance of any semantic gate that cannot be satisfied without changing oracle criteria or procedure semantics.

### Exceptions
During the build phase, exceptions are not processed through a portal product.
The human authority functions as the exception port, and each exception must be recorded in **SR-EXCEPTIONS** with an EX identifier.

---

## Precedence and Decision Routing

When documents conflict or guidance is ambiguous:

### Platform-definition precedence (meaning, invariants, mechanics)
1) SR-CONTRACT  
2) SR-SPEC  
3) SR-TYPES  
4) SR-WORK-SURFACE  
5) SR-SEMANTIC-ORACLE-SPEC  
6) SR-EVENT-MANAGER  
7) SR-AGENT-WORKER-CONTRACT  
8) SR-GUIDE / SR-INTENT / SR-MODEL (non-binding)

### Build-execution precedence (agent behavior, process, gating)
1) SR-CONTRACT  
2) SR-AGENTS  
3) SR-CHANGE  
4) SR-DIRECTIVE  
5) SR-PLAN  
6) Task-local instructions (must not violate higher precedence)

### Routing rule
- If the question is about **meaning/invariant/mechanics**, use platform-definition precedence.
- If the question is about **agent behavior / work process / gating**, use build-execution precedence.
- If still ambiguous: stop and escalate to human authority.

---

## Acceptance Contract (What “Counts”)

### A. What counts as a valid document increment
A document increment is acceptable when:
- it uses canonical terminology (work unit/loop, iteration, stage, work surface, evidence, authority),
- it respects precedence and scope (platform-definition vs build-execution),
- it uses canonical frontmatter schema and relationship semantics (`refs`),
- it introduces no version references as authority (versions must be represented as refs/hashes),
- and it does not assert authority that belongs at a trust boundary.

### B. What counts as a valid code increment
A code increment is acceptable when:
- it implements platform semantics as specified (not “whatever passes tests”),
- it produces or consumes evidence via oracle mechanisms where required (including semantic suites),
- it preserves auditability via records/events sufficient for replay and attribution,
- it does not bypass human authority requirements.

### C. What counts as a binding claim
A claim is binding only when:
- it is supported by commitment objects (records),
- required evidence is attached (when verification is required),
- required human authority records exist (when a trust boundary applies).

Agents may propose; agents may not declare binding status.

---

## Stop Rules (Mandatory Escalation Triggers)

Agents must stop and escalate to human authority when any of the following occur:

- a proposed change alters SR-TYPES or SR-CONTRACT semantics,
- a contradiction between governed documents cannot be resolved via precedence,
- a plan/directive instruction would require violating a platform-definition invariant,
- any exception is needed (must be recorded in SR-EXCEPTIONS first),
- a gate cannot be satisfied without changing governing criteria,
- a trust boundary requirement is unclear,
- the work surface is missing/underspecified for a stage that must run,
- semantic oracle criteria would need to change to make progress,
- or the agent is uncertain whether a claim would be interpreted as binding.

---

## Development Modality: Agent-Written Everything (Governed)

### What agents do
Agents:
- draft and revise all documents under governance,
- implement all code,
- run semantic oracles to produce evidence,
- propose stage transitions and candidate promotions,
- assemble change proposals and evidence packets.

### What humans do
Humans:
- define scope and priorities,
- adjudicate ambiguities,
- approve binding transitions and releases,
- grant/deny exceptions (recorded).

---

## Work Units to Initiate Development (Initial Build Queue)

This is the minimal sequence to reach Milestone 1 (functioning semantic work units):

1) **Define semantic work-surface schemas as code targets**
   - Intake + procedure template + stage gating + required artifacts.

2) **Implement semantic work-unit runtime semantics**
   - Work unit lifecycle + stage progression + progress projection.

3) **Implement iteration cycling**
   - Fresh-context iteration execution within a work unit, with controlled memory boundaries.

4) **Implement record store, event manager, and state projection**
   - Append-only events/commitments; deterministic replay; dependency graph state; eligibility computation.

5) **Implement semantic oracle interface**
   - Suite binding by refs/hashes; evidence bundle capture; structured measurement outputs.

6) **Implement authority ports (build-time)**
   - Human decision capture as binding records (approvals, denials, exceptions).

7) **Implement minimal API surface (ports)**
   - External calls to: create work units, submit candidates, attach evidence, request decisions, read records.

8) **Integration test via self-hosted governance**
   - Use SOLVER-Ralph to govern changes to its own docs and code at least for the semantic work-unit runtime scope.

---

## Commitments of This Charter

- The system will be built with agents writing all code and drafting all governed documents.
- Governance is not optional during build; it is the mechanism that makes agent work reliable.
- The first platform behavior to realize is the **semantic work-unit runtime** (Semantic Ralph Loops) as the agentic semantic worker substrate.
- External applications will integrate through APIs consistent with a hexagonal architecture once the work-unit runtime exists.
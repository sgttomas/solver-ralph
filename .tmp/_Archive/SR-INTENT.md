---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-INTENT"
  type: "governance.design_intent"
  title: "SOLVER-Ralph Design Intent"
  version: "1.2.0-draft.1"
  status: "draft"
  normative_status: "directional"
  authority_kind: "content"
  governed_by: ["SR-CHANGE"]
  supersedes: ["SR-INTENT@1.1.0-draft.6"]
  created: "2026-01-09"
  updated: "2026-01-12"
  tags: ["solver-ralph", "design-intent", "governance", "agentic-coding", "ralph-loop", "hexagonal-architecture", "problem-statement", "alignment", "state-machine", "deterministic-progression"]
  ext:
    design_intent:
      primary_problem: "How to generate trustworthy knowledge artifacts using non-deterministic agents."
      instance_problem: "Specify and implement SOLVER-Ralph as software that deterministically governs state progression while actors drive transitions."
      scope_notes:
        - "This document provides rationale and interpretation guidance for the SOLVER-Ralph governed set."
        - "Binding constraints belong in SR-CONTRACT and SR-SPEC."
        - "Authoring workflow belongs in SR-GUIDE."
        - "Agent constraints belong in SR-AGENTS."
      assumptions:
        - id: "A-001"
          statement: "Deterministic oracles can be defined for the majority of intended work."
          trigger: "Frequent ORACLE-GAP escalations or inability to define executable gates for core requirements."
        - id: "A-002"
          statement: "Human approvals at portals are available for non-oracle-verifiable acceptability judgments."
          trigger: "Operational reality prevents timely human review, or approvals become rubber-stamped and meaningless."
        - id: "A-003"
          statement: "Governance artifacts can remain concise while still being high-assurance when paired with strong evidence."
          trigger: "Governance overhead dominates outcomes, or teams start bypassing documentation due to friction."
        - id: "A-004"
          statement: "Agents remain non-authoritative; their outputs are treated as proposals until verified and approved."
          trigger: "Systems or teams begin treating agent statements (\"it's compliant\") as equivalent to verifiable evidence or contractual authority."
        - id: "A-005"
          statement: "Machine-readable metadata enables reliable retrieval and audit without requiring a centralized knowledge keeper."
          trigger: "Metadata becomes inconsistent or is not maintained, causing retrieval ambiguity or audit gaps."
        - id: "A-006"
          statement: "Work can be decomposed into scoped work units coordinated through explicit typed relationships (dependency vs provenance), enabling deterministic scheduling when needed."
          trigger: "Teams cannot decompose work without hidden dependencies causing deadlocks, repeated reversals, or un-auditable coupling between units."
        - id: "A-007"
          statement: "A well-formed problem statement is solution-shaped: it already encodes deliverables, evidence plans, and authority boundaries."
          trigger: "Problem statements remain ambiguous through governance authoring, causing repeated scope churn or ghost inputs."
        - id: "A-008"
          statement: "Coherent governance is achievable before publication; incoherence is detectable and remediable."
          trigger: "Coherence audits consistently fail without remediation path, or teams bypass audits to ship."
        - id: "A-009"
          statement: "State transition rules can be defined deterministically even when actor decisions are non-deterministic."
          trigger: "State machine semantics become ambiguous or unenforceable, allowing invalid transitions."
        - id: "A-010"
          statement: "Work can be decomposed into loop-completable units (Ralph Loops) such that per-unit success probability is high and the overall plan converges through iteration, evidence, and explicit escalation."
          trigger: "Work units remain too large or ambiguous to reach Verified outcomes within budgets; repeated loop failures without convergence; or persistent cross-unit coupling prevents completion."
---

# SOLVER-Ralph Design Intent — v1.2.0-draft.1

**Purpose:** Design rationale and first principles for SOLVER-Ralph: governance-first, architecturally harnessed agentic development using Ralph Loops (fresh-context iteration) with oracle-defined verification and human approvals at trust boundaries.

**Normative Status:** Directional (non-binding). This document explains *why* and provides interpretation guidance. Binding constraints belong in the **Architectural Contract** and **Technical Specification**. Authoring workflow belongs in the **Governance Authoring Guide**. Execution rules and gates belong in the **Development Directive**.

**Audience:** Architects, senior engineers, governance designers, and AI agents who need to understand *why the rules exist* and *when to escalate*.

**Entry Point:** Start with the Project README / orientation document, then SR-TYPES for artifact taxonomy, then this Design Intent.

---

## 0. Version Changes

### 1.2.0-draft.1 (2026-01-12)

This version implements **Type Coherence Remediation** changes:

- **Updated references from SR-PARADIGM to SR-GUIDE** throughout (the authoring guide was renamed).
- **Refactored §7 Terminology**: Removed definitions (this is a directional document; definitions belong in SR-CONTRACT and SR-TYPES). Replaced with cross-references to authoritative sources.
- **Updated §5.9**: Reframed "Merging paradigm and instance" as "Merging governance and enforcement" with clearer vocabulary.
- **Updated §3.7, §3.12**: Replaced "paradigm/instance" vocabulary with "governed set/deployment" or "rules/enforcement" throughout.
- **Updated diagrams**: Replaced "SOLVER-Ralph PARADIGM" label with "GOVERNED SET (SR-* Documents)".
- **Verified normative posture**: This document contains no binding MUST/SHALL constraints; where such language appears, it describes constraints defined elsewhere.

---

## 0.1 Relationship to Other Artifacts

This Design Intent is the "why" layer of the governed set. It provides interpretation guidance among compliant options and explains the reasoning behind constraints defined elsewhere.

It is intentionally directional: binding mechanics live in other artifacts.

**The governed set is indexed in SR-README.**

---

## 1. The Core Problem

### 1.1 What We're Protecting Against

SOLVER-Ralph exists to produce **high-assurance software and knowledge artifacts** using agents as labor while preserving human authority where determinism cannot be guaranteed.

More precisely, SOLVER-Ralph is a **consistency and integrity harness** for knowledge work under non-deterministic constraints and actors. It does not attempt to establish philosophical truth; it ensures that outputs are **replayable and comparable** because provenance, identity, evidence linkage, and authority boundaries are recorded and applied consistently.

The critical failure mode is not "the agent writes a bug." The critical failure mode is:

> **A false claim of verification or authorization becomes a foundation for downstream work.**

Examples of "foundation corruption" include:
- work shipped because an agent *claimed* tests passed (but they didn't),
- gates skipped or weakened ("make it green") without detection,
- approvals recorded against the wrong artifact state or wrong version,
- governance drift where "what we meant" silently diverges from "what the system enforces,"
- problem statements remaining ambiguous through governance authoring, causing ghost inputs.

This is a propagation risk: once the wrong foundation is accepted, downstream reasoning, planning, implementation, and audit become unreliable.

### 1.2 The Cost Asymmetry

SOLVER-Ralph assumes an extreme cost asymmetry typical of high-stakes engineering:

| Outcome | Cost | Why it matters |
|---|---:|---|
| Wrong approval / wrong baseline | Catastrophic | Silent drift; bad state becomes "truth" |
| Blocked progress due to conservative governance | Minor | Friction; re-run loop; retry |
| Extra escalation / "ask human too often" | Low–Medium | Time cost; improves clarity over time |
| Extra alignment effort at problem statement | Low | Front-loads clarity; prevents downstream churn |

**Design direction:** prefer conservative blocking and escalation over permissive acceptance. Prefer alignment effort early over scope churn late. Iteration is cheap; wrong work can be practically unbounded in cost.

### 1.3 The Propagation Problem

This approach deliberately shifts labor to agents and relies on iteration.

That means errors can compound quickly:

```
Ambiguous intent → weak oracle → false verification → approval → baseline → downstream work
```

The architecture must therefore ensure:
- problem statements are compiled into governance-ready structure before authoring begins,
- verification is evidence-based (not narrative-based),
- approvals are bound to specific states/versions,
- exceptions are explicit records (deviation/deferral/waiver), not silent drift,
- coherence is audited before publication.

---

## 2. Design Inputs

### 2.1 Trust Model

SOLVER-Ralph assumes the following about what can and cannot be trusted:

| We do not rely on… | Because |
|---|---|
| Agent "confidence" or self-assessment | It is non-deterministic and not authoritative |
| Prompt rules as security guarantees | They are suggestions, not enforcement |
| Human memory of decisions | It is not durable or auditable |
| Unversioned text artifacts | They drift; retrieval becomes ambiguous |
| "Tests exist" as a proxy for verification | Passing evidence is what matters, not existence |
| Transcript-memory or paraphrased problem statements (not committed) | They create ghost inputs, scope churn, and non-replayable semantics |

| We can rely on… | Because |
|---|---|
| Deterministic oracle outputs | They provide repeatable pass/fail evidence |
| Explicit human approvals at portals | They establish authorization in non-oracle domains |
| Typed artifacts with stable IDs + metadata | They support deterministic retrieval and audit |
| Immutable records of decisions/exceptions | They preserve intent and prevent silent drift |
| Structured, aligned problem statements recorded as typed records (`record.problem_statement`) | They encode deliverables, evidence plans, and authority boundaries and prevent "ghost meaning" |
| Deterministic state transition rules | They make validity enforceable regardless of actor decisions |

### 2.2 Failure Modes Under Consideration

This section names the failure modes that shape the Contract and Spec.

#### Foundation corruption
**Scenario:** approval, merge, or baseline created against stale/incorrect artifact content.  
**Why it matters:** downstream work inherits wrong premises.  
**Design direction:** approvals and baselines are coupled to immutable identifiers + evidence.

#### Oracle gaming and oracle erosion
**Scenario:** tests/scans weakened, bypassed, or suppressed to "get green."  
**Why it matters:** verification becomes meaningless; agents can self-approve indirectly.  
**Design direction:** oracle integrity is treated as a first-class governance concern and routed as a high-stakes issue.

#### Semantic drift of governance
**Scenario:** the meaning of requirements changes without an explicit version change; cross-references rot.  
**Why it matters:** compliance becomes interpretive and inconsistent.  
**Design direction:** typed artifacts, explicit precedence rules, and machine-readable metadata reduce drift.

#### Ghost inputs / provenance gaps
**Scenario:** work is influenced by inputs that are not recorded as typed references (e.g., untracked documents, ad-hoc context, or transcript memory).  
**Why it matters:** results become non-replayable; audits cannot reconstruct what was semantically in scope.  
**Design direction:** require explicit context provenance via typed refs and deterministic context compilation from those refs; prohibit raw transcript memory as default state.

#### Provenance treated as dependency (staleness churn)
**Scenario:** audit-only inputs (prompts/configs/background) are treated as semantic dependencies and therefore propagate blocking staleness constantly.  
**Why it matters:** concurrency collapses; teams either freeze helpful inputs or bypass governance to ship.  
**Design direction:** distinguish semantic dependency from audit provenance (e.g., `depends_on` vs `supported_by`) so only true semantic dependencies block by default.

#### Overbaking / context collapse in long agent loops
**Scenario:** long-running conversations produce incoherent compounding changes and "sticky" mistakes.  
**Why it matters:** agents become less reliable the longer they run without reset.  
**Design direction:** Ralph Loops: iterate in fresh context, record summaries and evidence, and re-enter with controlled state.

#### Ambiguous problem statements persisting through authoring
**Scenario:** governance artifacts are authored against an unclear or shifting problem statement.  
**Why it matters:** the governed set inherits ambiguity; scope churn and ghost inputs propagate.  
**Design direction:** require structured alignment (SAPS) before governance authoring begins; make alignment a binding gate.

#### Incoherent governance published for implementation
**Scenario:** governed artifacts contradict each other; agents cannot reliably follow them.  
**Why it matters:** implementation diverges from intent; audit fails.  
**Design direction:** require coherence audit (S11) before publication; make it a binding gate.

#### State transition rules ambiguous or unenforceable
**Scenario:** the runtime allows transitions that should be invalid, or blocks transitions that should be valid.  
**Why it matters:** the governance loses its enforcement value; actors can circumvent it.  
**Design direction:** define state machines with deterministic transition rules; enforce at the domain layer independent of adapters.

---

## 3. Key Decisions

This section captures major design choices and why they exist.

### 3.1 Separate "generation" from "evaluation" from "authority"

**Problem:** agents are powerful generators, but not reliable judges of their own outputs. And human interpretation of evidence is not the same as human approval.

**Decision:** establish a 4-way distinction:

| Activity | Actor | What it produces | Is it binding? |
|----------|-------|------------------|----------------|
| **Verification** | {agentic} / oracles | Evidence about conformance to plans | No (evidence, not authority) |
| **Evaluation** | {human} | Interpretation of verification evidence | No (informs, but not approval) |
| **Validation** | {agentic} / oracles | Evidence about fitness in context of the plans | No (evidence, not authority) |
| **Assessment** | {human} | Interpretation of validation evidence | No (informs, but not approval) |
| **Approval** | {human} at portal | Authorization to proceed | **Yes** (trust boundary action) |

**Rationale:** This separation prevents three conflation errors:
1. Agent claims treated as evidence (they are proposals)
2. Evidence treated as authority (evidence informs; approval binds)
3. Human interpretation treated as approval (evaluation/assessment inform; portal approval binds)

The binding semantics are defined in SR-CONTRACT §2.9 and SR-SPEC. The record types (`record.evaluation_note`, `record.assessment_note`) are defined in SR-TYPES.

### 3.2 Oracle-defined verification, not oracle-defined truth

**Problem:** "done = tests pass" can be confused as "correctness proven."  
**Decision:** define **Verified** strictly as "oracles passed for the declared scope," not "the system is correct."  
**Rationale:** oracle completeness is not knowable. Verification is bounded and honest; approval is a separate act.

This decision supports determinism without overstating epistemic certainty.

### 3.3 Human portals exist for acceptability, ambiguity, and high stakes

**Problem:** not all requirements are fully oracle-verifiable (or it is not economical to formalize everything).  
**Decision:** preserve explicit **human review and approval** portals as part of normal operation.  
**Rationale:** human judgment is an irreducible primitive in high-stakes systems; SOLVER-Ralph should make it explicit, minimal, and auditable—not pretend it can be eliminated.

### 3.4 Typed artifacts over ad-hoc documentation

**Problem:** untyped documentation collapses categories ("requirements in intent," "policy hidden in chat").  
**Decision:** define a type system for artifacts and enforce scope boundaries.  
**Rationale:** category errors cause semantic drift and make agentic workflows unsafe and irreproducible.

**Clarification:** in SOLVER-Ralph, "documentation" is not a tertiary description of the codebase. The governed documents are the **primary substrate of shared meaning**. They function as:

- **Canonical semantics:** stable definitions, precedence rules, and constraints that prevent vocabulary drift.
- **Runtime inputs:** explicit, referenceable artifacts that can be dereferenced and compiled into iteration context (instead of relying on transcript memory).
- **Commitment surfaces:** the place where intent, decisions, exceptions, and verification expectations become durable and attributable.

This is why documentation is central and pervasive: it is how a stochastic, narrative-capable agent workforce is anchored to replayable, auditable reality. Unrecorded rationale remains a local proposal and must not be treated as a binding foundation for downstream work.

Typed references and relationship semantics make dependency and provenance explicit, enabling parallel work without losing track of what blocks versus what merely informs.

### 3.5 Exceptions are records, not constant rewrites of governance

**Problem:** continuously updating governance for every mismatch creates churn and increases the chance of missing the forest for the trees.  
**Decision:** treat **Deviation Records**, **Deferral Records**, and **Waiver Records** as binding operational exceptions until resolved; keep governance stable where possible.  
**Rationale:** the cost of rework is low; the cost of wrong work is high; stabilizing meaning matters more than perfect document "freshness."

### 3.6 Metadata-first, but schema-emergent

**Problem:** semantic retrieval and audit require structured metadata, but premature over-structuring can create friction and false precision.  
**Decision:** require minimal core metadata on all governance-relevant artifacts, with extensions that emerge as patterns stabilize.  
**Rationale:** this aligns with a high-assurance workflow while allowing ontology to evolve without freezing too early.

### 3.7 Hexagonal architecture as governance enforcement boundary

**Problem:** governance logic becomes brittle if interwoven with infrastructure.  
**Decision:** define domain rules and state machines independent of adapters; treat persistence, LLM runtimes, and UIs as replaceable.  
**Rationale:** makes verification and audit easier; prevents tool coupling from becoming governance coupling.

This decision is essential for separating the **governed set** (the rules) from the **running system** (the enforcement): the governed set defines the domain rules, and the running system implements them through adapters that can be replaced without changing the rules.

### 3.8 Problem statement compilation (RPS → IPS → SAPS → INPS)

**Problem:** ambiguous problem statements persist through governance authoring, causing ghost inputs, scope churn, and incoherent governance.

**Decision:** require a staged compilation of problem statements before governance authoring:

| Stage | What it is | What it prevents |
|-------|------------|------------------|
| **RPS** (Raw) | Initial human statement (ambiguous) | — |
| **IPS** (Interpreted) | Clarified restatement with scope and acceptance criteria | Misalignment on what "done" means |
| **SAPS** (Structured, Aligned) | Governance-ready IR: requirements classified, evidence plans, authority boundaries | Ghost inputs; unclassified requirements; missing evidence plans |
| **INPS** (Instantiated) | SAPS compiled into governed artifacts and work units | Governance artifacts drifting from problem statement |

**Rationale:** A well-formed problem statement is **solution-shaped**: it already encodes the minimum structure needed to build and to prove "done." SAPS is the "forced intermediate representation" that ensures alignment before governance authoring begins.

This front-loads alignment effort rather than discovering scope problems during implementation. The cost of alignment is low; the cost of wrong governance is high.

The authoring workflow is defined in SR-GUIDE. The record type (`record.problem_statement`) is defined in SR-TYPES.

**Clarification (anti-ghost-meaning):** IPS and SAPS are *authoritative* only when they exist as committed, typed records. SR-GUIDE requires S0 outputs to be stored as `record.problem_statement` with the appropriate `stage` value and to be referenced downstream via typed `refs[]`. Re-paraphrasing or summarizing the problem statement is allowed for readability, but it must not replace the committed record as the semantic foundation for binding work.


### 3.9 Coherence audits are binding gates

**Problem:** incoherent governance defeats the purpose. If governed artifacts contradict each other, agents cannot reliably follow them, and implementation diverges from intent.

**Decision:** make the coherence audit (S11 in SR-GUIDE) a **binding gate**. The governed set is not published until coherence passes or failures are explicitly recorded as deviations.

**Rationale:**
1. **Incoherence is detectable.** Many coherence dimensions can be checked by oracles (semantic consistency, routing validation, constraint validation).
2. **Failures must be explicit.** Silent incoherence is worse than explicit deviation. If coherence cannot be achieved, record it as a deviation—don't pretend it doesn't exist.
3. **Evidence makes it auditable.** The coherence audit report is an evidence artifact that can be referenced.

The coherence dimensions are defined in SR-GUIDE §S11.

### 3.10 Agent wave model (bootstrap → domain → embedded)

**Problem:** agents operating at different levels of the stack have different governance relationships and different entry points into the workflow.

**Decision:** define three waves of agent deployment:

| Wave | Context | Entry point | Governance depth |
|------|---------|-------------|------------------|
| **A1: Bootstrap builders** | Building SOLVER-Ralph itself | C0 → full workflow | Full governance authoring |
| **A2: Domain agents** | Using SOLVER-Ralph for a domain problem | C0 → domain-scoped workflow | Domain governance |
| **A3: Embedded workers** | Operating within deployed system | S13-S14 or embedded work | Minimal; execute within existing governance |

**Rationale:** The waves differ in governance scope, not in the fundamental nature of what agents are. All agents share the same invariants (cannot claim Verified/Approved/Shippable), but they interact with the governance authoring workflow at different points.

This model enables:
- SOLVER-Ralph to build itself (A1 agents author the governed set)
- Domain problems to be solved with appropriate governance (A2 agents author domain governance)
- Routine work to proceed without full governance authoring (A3 agents execute within existing governance)

The agent invariants are defined in SR-AGENTS. The wave model is defined in SR-GUIDE §8.2.

### 3.11 Boot alignment prerequisites

**Problem:** a SOLVER-Ralph deployment requires a minimum governance substrate before humans and agents can converge on stable meaning without relying on transcript memory.

**Decision:** define boot alignment prerequisites as **governance categories** that must be satisfied to boot a deployment. The categories (not specific files) are:

- Ontology and meta-governance: types, authoring guide, agents, trust topology
- Governance content: intent, contract, spec
- Governance process: directive, change
- Navigation: readme (registry)

**Rationale:** This ensures that:
1. New deployments have a complete alignment substrate
2. The substrate is defined by category (what function must be satisfied), not by specific files
3. SR-README serves as the registry that selects concrete artifacts satisfying each category
4. Changes to the selection are routed through SR-CHANGE

The boot prerequisites are defined in SR-GUIDE §1.4 (YAML frontmatter). The type registry is defined in SR-TYPES §4.1.

### 3.12 Deterministic state progression with non-deterministic actors

**Planning vs execution (disambiguation):**
- **SR-PLAN** (a `governance.plan` artifact) is the binding decomposition of the SAPS into packages and deliverables (the "what must be produced"), expressed as typed `work_unit` deliverables with dependencies and evidence expectations.
- **SR-DIRECTIVE** is the execution discipline that determines sequencing (linear/concurrent), gates, budgets, and verification profiles (the "how work is run to resolution"), while ensuring the SR-PLAN inventory is fully addressed.

This separation is a drift-control membrane: execution strategy may adapt, but required deliverables must not change without an explicit plan change routed through governance.

**Problem:** the path from problem statement to resolution involves non-deterministic actors (humans decide, agents propose), but the validity of state transitions must be enforceable.

**Decision:** separate the state transition rules (deterministic) from the actor decisions that trigger transitions (non-deterministic):

```
┌─────────────────────────────────────────────────────────────────┐
│              GOVERNED SET (SR-* Documents)                       │
│         (defines valid states and transition rules)              │
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ specifies rules for
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    RUNNING SYSTEM                                │
│              (enforces rules, records events)                    │
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ enables actors to
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      ACTOR DECISIONS                             │
│   Human: approve, reject, escalate, intervene                    │
│   Agent: propose, draft, iterate, request verification           │
│   System: orchestrate, verify preconditions, emit events         │
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ trigger (if valid)
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    STATE TRANSITIONS                             │
│        (deterministic given inputs; recorded as events)          │
└─────────────────────────────────────────────────────────────────┘
```

**Key properties:**

| Property | What it means |
|----------|---------------|
| **Rules are deterministic** | Given a state and an action, whether the transition is valid is computable |
| **Actor decisions are non-deterministic** | Which actions actors choose to take is not predetermined |
| **Resulting state is deterministic given inputs** | If the same actions are replayed against the same state, the same outcome results |
| **Path is actor-determined** | The sequence of states visited depends on actor decisions |
| **Validity is system-enforced** | The running system rejects invalid transitions regardless of actor intent |

**Note on path space and convergence:**
Because actors are non-deterministic, the number of possible valid event sequences between "Problem Statement" and "Resolution" is enormous. SOLVER-Ralph does not try to eliminate this variability. Instead, it makes successful completion *probable* by constraining work into **small, loop-completable units** with explicit evidence expectations and explicit escalation routes. In practice, this is expressed through SR-GUIDE's SAPS-driven decomposition and, for a given deployment, an explicit SR-PLAN that sequences and coordinates those units under the governed state machine.

**Rationale:** This model achieves:

1. **Enforcement without omniscience.** The system doesn't need to predict what actors will do; it only needs to validate what they attempt.
2. **Auditability.** Because all state changes are recorded as events, any path can be replayed.
3. **Flexibility within constraints.** Actors have freedom to choose their approach; the system ensures they stay within valid bounds.
4. **Separation of concerns.** The governed set defines "what is valid"; the running system enforces it; actors decide "what to attempt."

This is the core insight of SOLVER-Ralph: **deterministic supervision of stochastic generators**. Agents propose, oracles verify, humans approve—but the rules governing what counts as valid verification and valid approval are fixed by the governed set.

---

## 4. Trade-offs

SOLVER-Ralph is intentionally not optimized for "fastest path to a merge." It optimizes for safe iteration.

Trade-offs we accept:
- **More upfront structure** (types, metadata, gates, SAPS) in exchange for faster downstream iteration and safer delegation.
- **More explicit escalation** in exchange for fewer silent failures and less "tribal knowledge."
- **Some human friction** (portals) in exchange for bounded authority and defensible decisions.
- **Alignment effort before authoring** (C0/S0) in exchange for coherent governance and fewer ghost inputs.
- **Coherence audit before publication** (S11) in exchange for reliable implementation.
- **Deterministic rules limiting actor freedom** in exchange for enforceable validity and auditable paths.

Trade-offs we avoid:
- pretending that prompt instructions create determinism,
- collapsing verification and authorization into a single "agent says it's done,"
- treating human interpretation (evaluation/assessment) as equivalent to approval,
- publishing incoherent governance and hoping implementation will "figure it out,"
- allowing actor decisions to bypass state transition rules.

---

## 5. Rejected Alternatives

This section records patterns that were considered and rejected.

### 5.1 Prompt-only governance ("rules in context are enough")
Rejected because it does not produce deterministic enforcement and cannot be treated as a security boundary.

### 5.2 Agent self-approval ("the agent can approve once tests pass")
Rejected because approval is a trust boundary action; it creates new binding state and must remain human-authorized.

### 5.3 Continuous governance rewrites for every mismatch
Rejected because it increases cognitive load, encourages accidental semantic drift, and can cause the team to lose the core invariants.

### 5.4 Monolithic long-context agent sessions
Rejected because long sessions tend to accumulate unforced errors and increase the risk of overbake; fresh-context iteration is preferred.

### 5.5 Governance authoring without structured alignment
Rejected because ambiguous problem statements persist through authoring, causing ghost inputs and scope churn. SAPS is required as the forced IR.

### 5.6 Optional coherence audits
Rejected because incoherent governance is worse than no governance. If artifacts contradict, agents cannot follow them. Coherence must be a binding gate.

### 5.7 Treating evaluation/assessment as approval
Rejected because conflating human interpretation with human authorization creates false foundations. Evaluation informs; approval binds.

### 5.8 Non-deterministic state transition rules
Rejected because if transition validity depends on non-deterministic factors, enforcement becomes impossible and audit becomes meaningless. Rules must be deterministic even when actors are not.

### 5.9 Merging governance and enforcement

Rejected because separating the **governed set** (the rules defined in SR-* documents) from the **running system** (the software that enforces them) enables:

1. **Replacement of infrastructure without changing semantics** — adapters can be swapped; the rules remain stable.
2. **Audit of any path through the system** — because rules are explicit and enforcement is deterministic.
3. **Clear reasoning about what is fixed vs what is actor-determined** — the governed set defines validity; actors choose actions; the running system validates.

This separation also makes it possible to reason about governance independently of any specific technology stack, and to specify the system before building it.

---

## 6. Assumptions & Re-evaluation Triggers

Assumptions and triggers are listed in the YAML frontmatter under `ext.design_intent.assumptions`.

Operationally, if a trigger is observed:
- create a Decision Record or Deviation Record (as appropriate),
- route governance implications through Change Management,
- update this Design Intent if the rationale itself has changed.

---

## 7. Terminology (Cross-Reference)

This is a directional document; it does not define binding vocabulary.

For authoritative definitions of key terms, see:

| Term | Authoritative Source |
|------|---------------------|
| Verified, Approved, Shippable | SR-CONTRACT §2.1 |
| Proposal, Commitment Object | SR-CONTRACT §2.8 |
| Verification, Evaluation, Validation, Assessment | SR-CONTRACT §2.9 |
| Oracle, Portal, Candidate, Evidence Bundle | SR-CONTRACT §2.3, §2.6 |
| Deviation, Deferral, Waiver, Decision | SR-CONTRACT §2.6, SR-TYPES §7 |
| SAPS, Problem Statement Family | SR-GUIDE §3.1, SR-TYPES §1.3.3 |
| Agent invariants | SR-AGENTS §1–2 |
| Governed set, Deployment | SR-TYPES §1.3.4 |

If any statement in this Design Intent appears to conflict with the authoritative definitions, the owning document governs.

---

## 8. Appendix: What This Document Is Not

- It is not a requirements list.
- It is not a substitute for Contract or Spec.
- It is not an execution plan (that's the Directive).
- It is not an authoring workflow (that's SR-GUIDE).
- It is not an agent definition (that's the Agents doc).
- It is not a record of "what happened" (that belongs to records and evidence).
- It is not the software implementation (that's the running system).

It is a map of the reasoning that keeps the governed set coherent under scrutiny.

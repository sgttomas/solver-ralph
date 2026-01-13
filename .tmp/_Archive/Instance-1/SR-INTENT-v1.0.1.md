---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-INTENT"
  type: "governance.design_intent"
  title: "SOLVER-Ralph Design Intent"
  version: "1.0.1"
  status: "governed"
  normative_status: "directional"
  authority_kind: "content"
  governed_by: ["SR-CHANGE"]
  supersedes: ["SR-INTENT@1.0.0"]
  created: "2026-01-09"
  updated: "2026-01-10"
  tags: ["solver-ralph", "design-intent", "governance", "agentic-coding", "ralph-loop", "hexagonal-architecture"]
  ext:
    design_intent:
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
          trigger: 'Systems or teams begin treating agent statements ("it’s compliant") as equivalent to oracle evidence.'
        - id: "A-005"
          statement: "Machine-readable metadata enables reliable retrieval and audit without requiring a centralized knowledge keeper."
          trigger: "Metadata becomes inconsistent or is not maintained, causing retrieval ambiguity or audit gaps."
        - id: "A-006"
          statement: "Work can be decomposed into scoped work units coordinated through explicit typed relationships (dependency vs provenance), enabling deterministic scheduling when needed."
          trigger: "Teams cannot decompose work without hidden dependencies causing deadlocks, repeated reversals, or un-auditable coupling between units."
---

# SOLVER-Ralph Design Intent (Why²) — v1.0.1

**Purpose:** Design rationale and first principles for the SOLVER-Ralph paradigm: governance-first, architecturally harnessed agentic development using Ralph Loops (fresh-context iteration) with oracle-defined verification and human approvals at trust boundaries.

**Normative Status:** Directional (non-binding). This document explains *why* and provides interpretation guidance. Binding constraints belong in the **Architectural Contract** and **Technical Specification**. Execution rules and gates belong in the **Development Directive**.

**Audience:** Architects, senior engineers, governance designers, and AI agents who need to understand *why the rules exist* and *when to escalate*.

**Entry Point:** Start with the Project README / orientation document, then `SOLVER-Ralph_types` for artifact taxonomy, then this Design Intent.

---

## 0. Relationship to Other Artifacts

This Design Intent is the top layer of the governed set and serves as the tie-breaker and interpretation guide among compliant options.

It is intentionally directional: SR-CONTRACT and SR-SPEC define the binding mechanics that realize this intent (explicit context provenance via typed refs, deterministic context semantics, and deterministic hooks for human judgment without new portal semantics).

- **Types (What exists / how it’s typed):** `SOLVER-Ralph_types` (governance.types)
- **Contract (What must remain invariant):** `SOLVER-Ralph_contract` (governance.arch_contract)
- **Spec (What we are building concretely):** `SOLVER-Ralph_spec` (governance.technical_spec)
- **Directive (How we execute and verify):** `SOLVER-Ralph_directive` (governance.dev_directive)
- **Change management (How governance evolves):** `SOLVER-Ralph_change` (governance.change_mgmt)
- **Baseline record (What was frozen):** `SOLVER-Ralph_freeze-record` (record.freeze)
- **Adjacent governance (non-baseline, for now):** decision heuristics, oracle integrity policy, deviation/deferral/decision records

---

## 1. The Core Problem

### 1.1 What We’re Protecting Against

SOLVER-Ralph exists to produce **high-assurance software and knowledge artifacts** using agents as labor while preserving human authority where determinism cannot be guaranteed.
More precisely, SOLVER-Ralph is a **consistency and integrity harness** for knowledge work under non-deterministic constraints and actors. It does not attempt to establish philosophical truth; it ensures that outputs are **replayable and comparable** because provenance, identity, evidence linkage, and authority boundaries are recorded and applied consistently.


The critical failure mode is not “the agent writes a bug.” The critical failure mode is:

> **A false claim of verification or authorization becomes a foundation for downstream work.**

Examples of “foundation corruption” in this paradigm include:
- work shipped because an agent *claimed* tests passed (but they didn’t),
- gates skipped or weakened (“make it green”) without detection,
- approvals recorded against the wrong artifact state or wrong version,
- governance drift where “what we meant” silently diverges from “what the system enforces.”

This is a propagation risk: once the wrong foundation is accepted, downstream reasoning, planning, implementation, and audit become unreliable.

### 1.2 The Cost Asymmetry

SOLVER-Ralph assumes an extreme cost asymmetry typical of high-stakes engineering:

| Outcome | Cost | Why it matters |
|---|---:|---|
| Wrong approval / wrong baseline | Catastrophic | Silent drift; bad state becomes “truth” |
| Blocked progress due to conservative governance | Minor | Friction; re-run loop; retry |
| Extra escalation / “ask human too often” | Low–Medium | Time cost; improves clarity over time |

**Design direction:** prefer conservative blocking and escalation over permissive acceptance. Iteration is cheap; wrong work can be practically unbounded in cost.

### 1.3 The Propagation Problem

This paradigm deliberately shifts labor to agents and relies on iteration.

That means errors can compound quickly:

```
Ambiguous intent → weak oracle → false verification → approval → baseline → downstream work
```

The architecture must therefore ensure:
- verification is evidence-based (not narrative-based),
- approvals are bound to specific states/versions,
- exceptions are explicit records (deviation/deferral), not silent drift.

---

## 2. Design Inputs

### 2.1 Trust Model

SOLVER-Ralph assumes the following about what can and cannot be trusted:

| We do not rely on… | Because |
|---|---|
| Agent “confidence” or self-assessment | It is non-deterministic and not authoritative |
| Prompt rules as security guarantees | They are suggestions, not enforcement |
| Human memory of decisions | It is not durable or auditable |
| Unversioned text artifacts | They drift; retrieval becomes ambiguous |
| “Tests exist” as a proxy for verification | Passing evidence is what matters, not existence |

| We can rely on… | Because |
|---|---|
| Deterministic oracle outputs | They provide repeatable pass/fail evidence |
| Explicit human approvals at portals | They establish authorization in non-oracle domains |
| Typed artifacts with stable IDs + metadata | They support deterministic retrieval and audit |
| Immutable records of decisions/exceptions | They preserve intent and prevent silent drift |

### 2.2 Failure Modes Under Consideration

This section names the failure modes that shape the Contract and Spec.

#### Foundation corruption
**Scenario:** approval, merge, or baseline created against stale/incorrect artifact content.  
**Why it matters:** downstream work inherits wrong premises.  
**Design direction:** approvals and baselines are coupled to immutable identifiers + evidence.

#### Oracle gaming and oracle erosion
**Scenario:** tests/scans weakened, bypassed, or suppressed to “get green.”  
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
**Scenario:** long-running conversations produce incoherent compounding changes and “sticky” mistakes.  
**Why it matters:** agents become less reliable the longer they run without reset.  
**Design direction:** Ralph Loops: iterate in fresh context, record summaries and evidence, and re-enter with controlled state.

---

## 3. Key Decisions

This section captures major design choices and why they exist.

### 3.1 Separate “generation” from “evaluation”

**Problem:** agents are powerful generators, but not reliable judges of their own outputs.  
**Decision:** treat agent output as proposal; treat oracle outputs as evidence; treat evaluation/assessment as human judgment records about evidence.  
**Rationale:** correctness claims must be grounded in deterministic outputs (or explicit human arbitration), not agent narratives.

This does *not* require agents to “understand” in a human sense. It requires systems to:
- ask for the right evidence,
- refuse to advance state without that evidence,
- record what happened for later audit.

### 3.2 Oracle-defined verification, not oracle-defined truth

**Problem:** “done = tests pass” can be confused as “correctness proven.”  
**Decision:** define **Verified** strictly as “oracles passed for the declared scope,” not “the system is correct.”  
**Rationale:** oracle completeness is not knowable. Verification is bounded and honest; approval is a separate act.

This decision supports determinism without overstating epistemic certainty.

### 3.3 Human portals exist for acceptability, ambiguity, and high stakes

**Problem:** not all requirements are fully oracle-verifiable (or it is not economical to formalize everything).  
**Decision:** preserve explicit **human review and approval** portals as part of normal operation.  
**Rationale:** human judgment is an irreducible primitive in high-stakes systems; SOLVER-Ralph should make it explicit, minimal, and auditable—not pretend it can be eliminated.

### 3.4 Typed artifacts over ad-hoc documentation

**Problem:** untyped documentation collapses categories (“requirements in intent,” “policy hidden in chat”).  
**Decision:** define a type system for artifacts and enforce scope boundaries.  
**Rationale:** category errors cause semantic drift and make agentic workflows unsafe and irreproducible.


**Clarification:** in SOLVER-Ralph, “documentation” is not a tertiary description of the codebase. The governed documents are the **primary substrate of shared meaning**. They function as:

- **Canonical semantics:** stable definitions, precedence rules, and constraints that prevent vocabulary drift.
- **Runtime inputs:** explicit, referenceable artifacts that can be dereferenced and compiled into iteration context (instead of relying on transcript memory).
- **Commitment surfaces:** the place where intent, decisions, exceptions, and verification expectations become durable and attributable.

This is why documentation is central and pervasive: it is how a stochastic, narrative-capable agent workforce is anchored to replayable, auditable reality. Unrecorded rationale remains a local proposal and must not be treated as a binding foundation for downstream work.

Typed references and relationship semantics make dependency and provenance explicit, enabling parallel work without losing track of what blocks versus what merely informs.

### 3.5 Exceptions are records, not constant rewrites of governance

**Problem:** continuously updating governance for every mismatch creates churn and increases the chance of missing the forest for the trees.  
**Decision:** treat **Deviation Records** as binding operational exceptions until resolved; keep governance stable where possible.  
**Rationale:** the cost of rework is low; the cost of wrong work is high; stabilizing meaning matters more than perfect document “freshness.”

### 3.6 Metadata-first, but schema-emergent

**Problem:** semantic retrieval and audit require structured metadata, but premature over-structuring can create friction and false precision.  
**Decision:** require minimal core metadata on all governance-relevant artifacts, with extensions that emerge as patterns stabilize.  
**Rationale:** this aligns with a high-assurance workflow while allowing ontology to evolve without freezing too early.

### 3.7 Hexagonal architecture as governance enforcement boundary

**Problem:** governance logic becomes brittle if interwoven with infrastructure.  
**Decision:** define domain rules and state machines independent of adapters; treat persistence, LLM runtimes, and UIs as replaceable.  
**Rationale:** makes verification and audit easier; prevents tool coupling from becoming governance coupling.

---

## 4. Trade-offs

SOLVER-Ralph is intentionally not optimized for “fastest path to a merge.” It optimizes for safe iteration.

Trade-offs we accept:
- **More upfront structure** (types, metadata, gates) in exchange for faster downstream iteration and safer delegation.
- **More explicit escalation** in exchange for fewer silent failures and less “tribal knowledge.”
- **Some human friction** (portals) in exchange for bounded authority and defensible decisions.

Trade-offs we avoid:
- pretending that prompt instructions create determinism,
- collapsing verification and authorization into a single “agent says it’s done.”

---

## 5. Rejected Alternatives

This section records patterns that were considered and rejected.

### 5.1 Prompt-only governance (“rules in context are enough”)
Rejected because it does not produce deterministic enforcement and cannot be treated as a security boundary.

### 5.2 Agent self-approval (“the agent can approve once tests pass”)
Rejected because approval is a trust boundary action; it creates new binding state and must remain human-authorized.

### 5.3 Continuous governance rewrites for every mismatch
Rejected because it increases cognitive load, encourages accidental semantic drift, and can cause the team to lose the core invariants.

### 5.4 Monolithic long-context agent sessions
Rejected because long sessions tend to accumulate unforced errors and increase the risk of overbake; fresh-context iteration is preferred.

---

## 6. Assumptions & Re-evaluation Triggers

Assumptions and triggers are listed in the YAML frontmatter under `ext.design_intent.assumptions`.

Operationally, if a trigger is observed:
- create a Decision Record or Deviation Record (as appropriate),
- route governance implications through Change Management,
- update this Design Intent if the rationale itself has changed.

---

## 7. Terminology

This section defines terms as operational meanings, not as philosophy.

- **Verified:** required oracles passed for the declared scope.  
- **Approved:** a human accepted the evidence at a portal/gate.  
- **Shippable:** verified + approved + baseline conditions satisfied (including active deviations/deferrals being acknowledged).  
- **Oracle:** deterministic evaluator producing pass/fail evidence.  
- **Portal:** required human arbitration point where acceptability cannot be fully defined by oracles.  
- **Deviation:** binding exception from a governed requirement, active until resolved.  
- **Deferral:** binding postponement, active until resolved or declared permanent.  
- **Decision:** binding judgment; may be designated as precedent when explicitly scoped.

---

## 8. Appendix: What This Document Is Not

- It is not a requirements list.  
- It is not a substitute for Contract or Spec.  
- It is not an execution plan.  
- It is not a record of “what happened” (that belongs to records and evidence).

It is a map of the reasoning that keeps the governed set coherent under scrutiny.
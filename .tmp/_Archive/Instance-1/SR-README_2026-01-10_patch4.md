---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-README"
  type: "governance.readme"
  title: "SOLVER-Ralph"
  version: "unversioned"
  status: "draft"
  normative_status: "index"
  authority_kind: "index"
  governed_by: ["SR-CHANGE"]
  created: "2026-01-09"
  updated: "2026-01-10"
  tags: ["solver-ralph", "readme", "governance", "agentic-coding", "ralph-loop"]
---

# SOLVER-Ralph

**SOLVER-Ralph** is an archetypal **governance-first** software development paradigm and harness:

- **Humans provide**: *intent*, *judgment when escalated*, and *approval at trust boundaries*.
- **Agents provide**: *labor* (drafting/iteration), bounded by governance and evidence requirements.
- **The system provides**: deterministic orchestration, verification evidence, and auditability.

The workflow is designed for high-scrutiny knowledge work (e.g., regulated engineering), where the cost of "wrong work" can be effectively unbounded.

> **Core idea:** *Ideas + Decisions → Software*  
> Implemented as: **Ralph Loops** (fresh-context iterations) inside the **SOLVER governance harness**.

---

## What "Done" Means Here

SOLVER-Ralph separates **evidence** from **authority**:

- **Verified**: deterministic oracles ran and produced an evidence bundle (PASS/FAIL recorded).
- **Approved**: a human approves at a trust boundary based on evidence.
- **Shippable**: verified (strict or with exceptions) + approved + baseline/exception conditions satisfied.

**Note:** This README is **not normative**. For binding definitions and requirements, see:
- **Architectural Contract (SR-CONTRACT)** — `SOLVER-Ralph_architecture-contracts-v1.0.0.md`
- **Technical Specification (SR-SPEC)** — `SOLVER-Ralph_Tech_Spec_SR-SPEC_v1.2.1.md`
- **Development Directive (SR-DIRECTIVE)** — `SOLVER-Ralph_dev_directive-v1.1.0.md`
- **Change Management (SR-CHANGE)** — `SOLVER-Ralph_change_mgmt-v1.2.0.md`

This avoids the failure mode where "the agent says it's done" becomes a substitute for evidence.

---

## Inception: How SOLVER-Ralph Emerged

SOLVER-Ralph is the fusion of three threads:

1) **SOLVER (governance-first)**  
   SOLVER began as a document-governed framework for building high-assurance software with explicit authority, precedence, and change control. The core move is *types-first* governance: intent → contract → spec → directive → change management → freeze/baselines.

2) **Ralph Loops (fresh-context iteration)**  
   As agentic coding matured, a key reliability improvement emerged: run development as short **while-loops** in *fresh context*, recording summaries and evidence each iteration. This avoids "overbaking" and long-context drift while making progress measurable via oracles.

3) **Hexagonal architecture (ports & adapters)**  
   To make the paradigm implementation-agnostic and testable, the governance logic and state machines live in a domain core, with adapters for storage, oracle execution, and agent runtimes. This keeps the system auditable and swappable across environments.

The result is a deterministic supervisor for stochastic generators: agents can propose, but only oracles + humans can move the system across trust boundaries.

---

## Repository Goals

This repository is intended to be:

- a **reference implementation** of the SOLVER-Ralph paradigm,
- an **archetype** you can instantiate for new projects,
- a **living set of governed artifacts** and record/evidence outputs,
- a base for **event-graph** storage so changes can trigger re-evaluation of downstream/horizontally connected nodes.

### Non-goals

- This is not "secure code generation via prompts." Prompt rules are not enforcement.
- This is not "agent self-approval." Trust boundaries are human-only.
- This is not "perfect correctness." Verification is bounded; humans arbitrate acceptability.

---

## Documentation Index

### Canonical governed set (binding precedence: Contract > Spec > Directive > Intent > README)

- **SR-TYPES** — document type system and templates  
  `0_SOLVER-Ralph_types-v3.2.2.md`

- **SR-ETT** — epistemic trust topology (nine membranes; trust as emergent property)  
  `SOLVER-Ralph_epistemic-trust-topology_SR-ETT_v1.0.0.md`

- **SR-INTENT** — directional “why” and philosophy  
  `SOLVER-Ralph_intent-v1.0.0.md`

- **SR-CONTRACT** — binding architecture invariants  
  `SOLVER-Ralph_architecture-contracts-v1.0.0.md`

- **SR-SPEC** — binding technical mechanics (events/APIs/state machines)  
  `SOLVER-Ralph_Tech_Spec_SR-SPEC_v1.2.1.md`

- **SR-DIRECTIVE** — binding operational protocol for implementers  
  `SOLVER-Ralph_dev_directive-v1.1.0.md`

- **SR-CHANGE** — binding change management and promotion rules  
  `SOLVER-Ralph_change_mgmt-v1.2.0.md`

### Output artifacts (runtime-produced, content-addressed)

- **Freeze Record** (baseline manifest of governed artifacts + exceptions)
- **Evidence Bundles** (oracle outputs)
- **Decision Records** (human judgments without creating new portals)
- **Candidates** (snapshots identified by git + sha256)

### Implementation configuration (directional, auditable)

- `config.agent_definition` — sandboxed agent worker role/capability/tool/context profiles
- `config.template_catalog` — versioned prompt/template catalogs (if used)
- `config.ci_policy` — CI/oracle wiring


## Human Judgment Hooks

SOLVER-Ralph supports **deterministic hooks** that increase trust without introducing new Portals or changing lifecycle states.

- A **work unit** MAY specify a `config.gating_policy` (soft/hard/hybrid) that governs when missing human judgments are blocking versus advisory.
- Agents perform verification/validation work and produce deterministic oracle evidence (Evidence Bundles).
- Humans may provide non-binding **evaluation** and **assessment** records:
  - `record.evaluation_note` (evaluation of verification evidence)
  - `record.assessment_note` (assessment of validation evidence)
  These records are **auditable** but are **not carried forward as iteration memory by default**; they are referenced only when needed (typically the next iteration).
- Final closeout remains a **Portal approval** (e.g., ReleaseApprovalPortal). For closeout, approvals SHOULD reference validation evidence and attest that validation was assessed.

## The SOLVER-Ralph Workflow

SOLVER-Ralph is an **agent harness** for high-assurance development: it separates **control-plane governance** (SYSTEM + portals + oracles) from **work-plane execution** (sandboxed Agent Workers), and it makes iteration inputs explicit and replayable.

### Agent Harness

```mermaid
flowchart LR
  UI[Human / UI / queue] -->|work request| SYS[SYSTEM Loop Governor]
  SYS -->|IterationStarted (actor_kind=SYSTEM, refs[])| BUS[(Event Bus)]
  BUS --> W[Agent Worker (sandbox)]
  W -->|register Candidate| API[SR-SPEC API]
  W -->|request Runs (oracles)| API
  API --> O[Oracles]
  O -->|EvidenceBundle(s)| API
  W -->|IterationSummary + complete| API
  API --> P[Human Portals]
```

**Key mechanics:**

- **SYSTEM emits `IterationStarted`** (SYSTEM-only), and the authoritative input set for the iteration is `IterationStarted.refs[]`.
- **Agent Workers compile context from refs**: the semantic `ContextBundle` is derived deterministically from the `IterationStarted` payload + dereferenced `refs[]` (no ghost inputs).
- **Oracles produce evidence bundles** via deterministic runs.
- **Humans approve at portals** (trust boundary). Internal plan review is allowed, but it is **non-binding** and recorded as a review note — not as “Approval” and not as “Verified”.

### High-level workflow

1) **Create a Loop** scoped to a work unit (task/package/module).  
2) **SYSTEM starts an Iteration** by emitting `IterationStarted` with the Iteration Context Ref Set (`refs[]`).  
3) **Agent Worker executes** using only the derived context:
   - produces Candidate snapshot(s) (git + sha256),
   - requests oracle Runs,
   - records a structured IterationSummary (actions/outcomes/next_steps/open_risks).  
4) **Oracles verify** and emit evidence bundles.  
5) **Humans intervene/approve** at portals when required (governance changes, oracle suite changes, releases).  

## The loop (conceptual)

Below is an intentionally simplified conceptual loop that reflects the SR-SPEC rules:

```python
# Control-plane: SYSTEM
loop = SYSTEM.create_loop(goal, budgets, directive_ref)

while True:
    # SYSTEM assembles the authoritative input set for the iteration.
    refs = SYSTEM.assemble_iteration_context_ref_set(loop_id=loop.id)

    # SYSTEM-only event emission.
    iteration = SYSTEM.start_iteration(loop_id=loop.id, refs=refs)  # emits IterationStarted(actor_kind=SYSTEM)

    # Work-plane: sandboxed Agent Worker
    ctx = AgentWorker.compile_context(iteration.started_event.payload,
                                      deref(iteration.started_event.refs))  # deterministic, no ghost inputs

    # Optional non-binding plan discipline (record only; not approval)
    plan = AgentWorker.draft_plan(ctx)
    review = NonBindingReviewer.semantic_review(plan)   # human or agent
    AgentWorker.record_non_binding_review(review)       # -> IterationSummary.ext.non_binding_reviews[]

    candidate = AgentWorker.produce_candidate(ctx)

    # Candidate identity: git + sha256 (system should verify hash when feasible)
    SYSTEM.register_candidate(candidate)

    run_ids = SYSTEM.request_oracle_runs(candidate, oracle_suite_ref=ctx.oracle_suite)

    evidence = SYSTEM.collect_evidence(run_ids)

    if SYSTEM.stop_triggers_fired(evidence):
        SYSTEM.pause_iteration(iteration.id)
        HUMAN.decide_next_step(evidence)  # Decision record; portals if required
        continue

    if SYSTEM.oracle_verification_passed(evidence) and HUMAN.portal_release_approved(candidate):
        SYSTEM.mark_shippable(candidate)
        break

    SYSTEM.complete_iteration(iteration.id, summary=AgentWorker.iteration_summary())
```

Key points:

- `IterationStarted` is **SYSTEM-only**.
- The only semantic inputs are those in `IterationStarted.refs[]` (plus its payload), deterministically compiled into context.
- “Verified” is oracle evidence–backed. “Approved” is portal/human-only. Plan review is **non-binding**.

## Roles and Responsibilities

SOLVER-Ralph uses roles to keep authority clean:

- **Human**: decides and approves at boundaries.
- **Developer (external)**: implements code (may be an agentic "labor" role).
- **Code Governor**: verifies compliance and runs oracles (evidence-based review).
- **Change Manager**: governs changes to governance (spec amendments, deviations).
- **Freeze Steward**: manages baselines and freeze records.
- **Decision Router**: classifies decision points, sets stakes, routes to the right actor.

Each role has an explicit "can do / cannot do" boundary to prevent authority leakage.

---

## Architecture at a Glance

SOLVER-Ralph is a hexagonal harness with a strict separation of concerns:

- **Domain core**: loops/iterations/state machines, governed semantics, staleness rules, and event emission rules (infra-free).
- **SYSTEM services (control plane)**: assemble ref sets and emit `IterationStarted` (SYSTEM-only), manage routing to oracles/portals.
- **Agent Workers (work plane)**: sandboxed services that consume `IterationStarted`, compile context from refs, produce Candidates, request Runs, and submit Iteration summaries.
- **Oracles**: deterministic runners that produce evidence bundles.
- **Portals**: human trust-boundary actions (governance, oracle suite changes, release).

**Context provenance:** the authoritative input set for each iteration is captured as typed references on `IterationStarted.refs[]` (no ghost inputs).

**Important guardrails:** do not create new Portals for plan approval; do not redefine Verified/Approved semantics; do not move agent orchestration into the domain core.

## How to Start (for new instantiations)

1) Fork or copy this repo as a template.
2) Author the canonical governed set:
   - types → intent → contract → spec → directive → change mgmt
3) Choose an oracle suite and define your first "verification profile."
4) Implement minimal adapters:
   - artifact store (filesystem is fine),
   - event store (append-only),
   - oracle runner (subprocess),
   - agent worker runtime (sandboxed; Claude Code / other).
5) Run your first loop and capture a Freeze Record.

---

## Questions for Clarifying Intent (for teams adopting this)

If you are instantiating SOLVER-Ralph for a new project, you must be able to answer:

- What are the required oracles for *Strict verification*?
- Under what conditions are *waivers* permitted (and who approves them)?
- What triggers require immediate escalation?
- What is the minimum evidence a human must review at approval?

---

## References & Influences

- SOLVER governance framework (document-typed high assurance development)
- Ralph Loop technique (fresh-context iteration)
- Hexagonal architecture (ports & adapters)

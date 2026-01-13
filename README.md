# SOLVER-Ralph

**SOLVER-Ralph is a governance-first, event-sourced work surface for agentic workflows, by Chirality AI (author: Ryan Tufts).**

## What SOLVER-Ralph is

SOLVER-Ralph is a *platform kernel* that lets agents, humans, and verification systems (oracles) collaborate on complex work under explicit rules about **evidence**, **authority**, and **traceability**.

It turns agentic work into a replayable, evidence-backed, authority-gated process.  

It keeps the platform simple and pushes complexity where it belongs: into typed, structured documentation that defines what “doing the work correctly” means.

It provides:
- **Semantic Ralph Loops**: the unit of governed work (indefinite fresh-context attempts inside a loop with a declared context reference set including summary of previous attempt, and a declarable - and ideally testable - stopping condition)
- **Candidates**: proposed outputs (documents, code, decisions, structured artifacts)
- **Oracles + evidence bundles**: structured verification artifacts that support claims but never *become* authority.
- **Portals (trust boundaries)**: explicit human approval boundaries where binding decisions are recorded.
- **Freeze baselines**: immutable “what we accepted” snapshots for replay, audit, and downstream dependency tracking.

## What SOLVER-Ralph requires to run effective

Typed and structured documentation.  
Refer to the repo for the archetype to follow for documentation of work and task discretization for agents.
You need to build a workflow out of typed documentation so that the problem can easily be chunked and you need to create those rules for the semantic-ralph-loop

An adjacent project SOLVER-engine will take a generic problem statement and produce typed documentation as an output, suitable for an agent to be assigned the problem statement and find a solution (if that type of problem permits such a thing to begin with - it can't wash  your dishes)

## Typed documentation and why it matters

SOLVER-Ralph aims to make knowledge work **typed and structured**:
- inputs are expressed as typed problem statements,
- procedures are expressed as typed stage-gated templates,
- evaluation is expressed as typed oracle suites and gate criteria,
- outputs are expressed as typed candidates and records.

Typed documentation matters because it:
- reduces ambiguity and drift in long-running work,
- enables validation and linting of procedures,
- makes replay and audit meaningful (you can re-run *the same* procedure),
- and gives agents a stable substrate to operate on beyond freeform prompts.

## Typed documentation as “design of agency”

Typed documentation lets you design agency in a disciplined way:
- you can specify what an agent is allowed to do at each stage,
- what evidence it must produce,
- what it must escalate to humans,
- and how to detect failure modes (stalling, flaking, integrity issues).

## Ralph Loops and Semantic Ralph Loops

### Ralph Loops (general)
A **Ralph Loop** is the generic governed loop pattern:
- propose candidates,
- gather evidence,
- cross trust boundaries for binding decisions,
- freeze accepted baselines,
- repeat with fresh-context iterations and a summary of the previous loop

### Semantic Ralph Loops (specialized for knowledge work)
A **Semantic Ralph Loop** is a Ralph Loop whose primary candidates are **semantic artifacts**:
documents, structured representations, analyses, decision records, and other “knowledge” objects where correctness is not reducible to unit tests.

#### What’s invariant between them (and why that matters)
Invariant across both:
- evidence is mandatory for claims,
- authority is explicit and human-gated,
- context is pinned (no ghost inputs),
- events provide replayability and auditability.

This invariance is significant for LLM agents: it gives them a stable governance kernel regardless of domain, while keeping domain variability in typed procedures and oracles.

## What SOLVER-Ralph is not

SOLVER-Ralph is **not**:
- a chatbot UI,
- a monolithic “agent that does everything,”
- a replacement for domain expertise,
- a place where “the model said so” counts as verification,
- a system that can *permit* hidden context, invisible edits, or unrecorded authority.

It is intentionally *not* prescriptive about an agent’s internal tactics (prompting strategy, decomposition heuristics, tool choice). Those live outside the core as replaceable workers/adapters.

## What SOLVER-Ralph does (and does not do)

### It does
- enforce **no-ghost-inputs** discipline: an iteration’s admissible context must be reconstructible from its declared refs;
- make “what happened” **replayable** via an append-only event log and deterministic projections;
- require **evidence** to be packaged, addressable, and integrity-checked;
- require **human authority** for binding claims at explicit trust boundaries;
- enable staleness tracking and dependency reasoning by representing artifacts and relationships as typed references.

### It does not
- guarantee correctness of the world or truth of a claim without evidence;
- guarantee task success without adequate procedures and domain-specific oracles;
- remove the need to decide what counts as “good enough” (it forces that to be explicit and governed).

## Who it’s for

SOLVER-Ralph is designed for **private, on‑prem AI** inside organizations that want:
- strong auditability and reproducibility,
- explicit separation of evidence from authority,
- workflows expressed as *typed, structured documentation* (not just ad-hoc prompts),
- and agentic execution that can scale to long-running, multi-step knowledge work.

## How agents interface with SOLVER-Ralph

Agents don’t “run inside” SOLVER-Ralph. They **navigate** it.

A typical agent lifecycle is:
1. **Create / select a loop** (a case file for the task).
2. **Start an iteration** with a complete `refs[]` context set (intake, procedure template, stage + parameters, oracle profile/suites, plan instance, directive policy, prior freeze baseline).
3. **Generate a candidate** (proposal) and register it with provenance.
4. **Run the required oracle suite(s)** and record evidence bundles.
5. **Evaluate gates** and request portal decisions when binding transitions are needed.
6. **Freeze** accepted baselines for audit and future dependency tracking.
7. Repeat until the loop closes.

The “workflow structure” is expressed through the platform’s control-plane protocol (events/records) plus the declared, governed artifacts that define the work surface.

## Technical and architectural notes

### Hexagonal architecture (ports & adapters)
SOLVER-Ralph uses a **hexagonal architecture** to keep the *domain core* pure:
- **Domain core**: invariants, commands, events, and state machines.
- **Adapters**: storage, messaging, identity/auth, evidence backends, oracle runners, UIs, and agent workers.

This separation is key: it prevents tool/UI choices from becoming semantics, and makes it possible to swap implementations without changing what “Verified” or “Approved” means.

### Event sourcing and deterministic projections
The system is **event-sourced**:
- the append-only event stream is the source of truth,
- projections are derived views,
- replay is a first-class capability, not an afterthought.

This matters for agentic work because it lets you answer:
- *what did the agent use as context?*
- *what evidence justified a claim?*
- *who authorized a binding decision and when?*
- *what became stale when an upstream artifact changed?*

### “Small API, big program”
SOLVER-Ralph intentionally keeps the *control plane* relatively small.
The variability and complexity live in the **typed documentation** that defines work surfaces, procedures, and oracle suites.

That shifts effort toward writing good, governed instructions—and away from building ever-more bespoke agent code.

## Ontological, epistemological, and semantic characteristics

### Ontology (what exists in the system)
SOLVER-Ralph treats governance objects as first-class:
- loops, iterations, candidates, evidence bundles, gate evaluations, exceptions, approvals, and freeze baselines
- plus typed references linking them (dependencies, provenance, approvals, verification relations)

### Epistemology (what counts as knowledge)
SOLVER-Ralph enforces a strict separation:
- **Evidence**: produced by oracles and artifacts; supports claims; is never itself authoritative.
- **Authority**: exercised only through explicit human trust boundaries (portals) whose outputs are binding records.

### Semantics (what words mean)
Status words like **Verified**, **Approved**, and **Shippable** are not vibes.
They are **governed claims**: each has required evidence and required authority transitions.

This is especially important for LLM-based agents because it prevents the system from “believing” fluent text.

### A fair assessment of likelihood of success on complex long-run non-coding tasks
Typed, structured procedures **increase** the probability of success for long-run semantic tasks by:
- constraining ambiguity,
- forcing explicit intermediate artifacts,
- enabling systematic verification,
- and preventing hidden-context drift.

But they do not make hard problems easy:
- the bottleneck becomes high-quality procedure design and oracle design,
- and some domains will remain difficult until you have strong, domain-specific evaluation and escalation patterns.

SOLVER-Ralph is best understood as an engine for *turning “how we decide” into executable, auditable structure*—not a guarantee that the underlying domain is solved.

## Privacy and Powerful Agents

If you’re building private, on‑prem AI for real organizations, SOLVER-Ralph is the work surface that keeps agents honest, makes decisions attributable, and makes long-running semantic work tractable without pretending it’s effortless.

---

**License:** MIT (see `LICENSE`).
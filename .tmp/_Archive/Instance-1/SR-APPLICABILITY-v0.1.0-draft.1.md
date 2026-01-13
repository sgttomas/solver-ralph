---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-APPLICABILITY"
  type: "guidance.applicability"
  title: "SOLVER-Ralph Applicability and Orientation"
  version: "0.1.0-draft.1"
  status: "draft"
  normative_status: "directional"
  authority_kind: "content"
  governed_by: ["SR-CHANGE"]
  supersedes: []
  created: "2026-01-11"
  updated: "2026-01-11"
  tags:
    - "solver-ralph"
    - "applicability"
    - "orientation"
    - "adoption"
    - "governance"
    - "agentic-workflows"
    - "verification"
    - "authority"
  ext:
    applicability:
      summary: >
        Provides orientation for SOLVER‑Ralph from multiple vantage points and defines
        problem signatures for when SOLVER‑Ralph is well‑suited vs poorly‑suited.
        This artifact is directional (non‑binding) and must not introduce new authorities,
        gates, portal requirements, or semantic invariants beyond the governed set.
      scope:
        - "How to understand SOLVER‑Ralph at a high level (meaning layer vs mechanism layer)."
        - "When to use SOLVER‑Ralph (problem signatures) and when not to."
        - "Tradeoffs and failure modes of adoption."
      non_goals:
        - "Defining new binding semantics (see SR-CONTRACT / SR-SPEC)."
        - "Defining operational control surfaces (see SR-DIRECTIVE)."
        - "Changing governance routing or authorities (see SR-CHANGE)."
      precedence_note: >
        If anything here appears to conflict with SR-CONTRACT or SR-SPEC, this document
        is explanatory only; treat the binding artifacts as authoritative.
---

# SOLVER-Ralph Applicability and Orientation (SR-APPLICABILITY) — v0.1.0-draft.1

This document captures **orientation-level** understanding of SOLVER‑Ralph and provides **applicability guidance**:
what kinds of knowledge-work problems SOLVER‑Ralph is especially well-designed for, and what kinds it is not.

**Status:** Directional (non‑binding).  
**Design intent:** Preserve conceptual coherence without adding new requirements.

---

## 0. How to use this document

### 0.1 Audience

- Newcomers trying to understand what SOLVER‑Ralph “is” without reading the full governed set first.
- Implementers choosing whether to adopt SOLVER‑Ralph for a workstream.
- Reviewers assessing fit (“is this too heavy / too light?”).

### 0.2 What is binding vs explanatory

- **Binding rules** live in SR‑CONTRACT / SR‑SPEC / SR‑CHANGE (and SR‑DIRECTIVE for operations).
- Everything here is **explanatory** and must not be used to justify bypassing binding artifacts.

---

## 1. Orientation: what SOLVER‑Ralph is (high-signal overview)

SOLVER‑Ralph is a **governed development harness** for building software with agents in the loop, designed so that:

- agents can do lots of work (search, drafting, coding, synthesis),
- but **agents never become the authority** that defines what’s true, what’s approved, or what’s released,
- and every important claim can be **reconstructed, audited, and replayed** from durable records and evidence.

It’s not “an agent framework” in the usual sense (prompts + tools + heuristics). It’s closer to a
**truth‑maintenance and change‑control operating system** for agentic development.

### 1.1 Two representations that are intentionally coupled

**A) Meaning layer: governance as typed semantics**  
A governed set of documents defines what exists (ontology/types), what counts as knowledge (evidence rules),
what must always be true (invariants), where coercion belongs (trust topology), how to operate work
(control surfaces), and how governance changes (change policy).

**B) Mechanism layer: code as enforced semantics**  
The system implementation enforces those meanings via ports/adapters (hex), event logging, gates/verification profiles,
deterministic oracles and evidence bundles, and human portal approvals.

**Why this matters:** It prevents drift into “the agent said it was fine” governance. Meaning becomes computable.

### 1.2 The core architectural move: variation / selection / binding

1) **Agents = variation / motion (proposers)**  
2) **Oracles + gates = selection (evidence + admissibility)**  
3) **Humans via portals = binding (authority + responsibility)**

SOLVER‑Ralph refuses to let “search” collapse into “truth.”

### 1.3 Agents are defined by non‑authority

An agent is defined by a single invariant:

> Agents are non‑authoritative actors whose outputs are proposals unless admitted through membranes.

Variability belongs in **work envelopes** (context, allowed actions, required outputs, verification profile),
not in agent ontology.

### 1.4 Membranes: admissibility and load‑bearing meaning

“Membranes” are places where fluid proposals become durable, downstream‑reliable meaning
(records, evidence, approvals, freezes) **only when admitted** through the correct boundary.
Membranes do not delete natural-language meanings; they define institutional admissibility and state change.

### 1.5 “Semantic valley” (non‑normative metaphor)

A helpful intuition is that typing + admissibility shape a corridor where coherent moves are easier to generate
and incoherent moves are rejected by structure. This is a metaphor; it must not be treated as a normative claim
about model internals.

---

## 2. Applicability: problem signatures where SOLVER‑Ralph shines

SOLVER‑Ralph is particularly well‑designed for problems with this signature:

### 2.1 Candidate/evidence separability

You can express progress as:
- produce a candidate artifact/change,
- evaluate it via evidence-producing mechanisms,
- decide and bind via explicit authority when required.

### 2.2 High drift risk

The dominant risk is that meaning changes silently over time:
- “done” shifts,
- exceptions accumulate,
- narrative confidence becomes policy.

SOLVER‑Ralph imposes explicit change control + explicit exceptions as records.

### 2.3 High need for replayability and justification

It must be possible to answer later:
- Why was this accepted?
- What evidence supported it?
- What rules/versions were in force?
- Which exceptions existed, who approved them, and when do they expire?

### 2.4 Many interdependent constraints

Changes have non-local effects and require disciplined constraint management
(types + gates + suites + provenance) to keep work “on-manifold.”

### 2.5 Bounded exploration + forced convergence

You need both:
- exploration (generate options, decompose, hypothesize),
- convergence (verify, select, bind, freeze/release).

### 2.6 Exceptions are inevitable but must not become the system

You can’t eliminate waivers/deferrals—but you must prevent informal bypass from becoming the real policy.

---

## 3. Anti-signatures: when SOLVER‑Ralph is a poor fit (or needs tailoring)

SOLVER‑Ralph is often a poor default when:

### 3.1 No meaningful verification surface

If you cannot define any credible oracles/evidence, the system degenerates into narrative governance.

### 3.2 Pure exploration with no convergence semantics

If there is no notion of “admissible completion” or no value in binding baselines, the harness overhead dominates.

### 3.3 Latency dominates correctness/auditability

If decisions must be made faster than evidence + portal routing can support, SOLVER‑Ralph may be too heavy unless slimmed.

### 3.4 Types cannot stabilize

If the ontology is so unstable that types cannot be maintained even directionally, typing ceases to be a shortcut.

---

## 4. Tradeoffs and cost model (directional)

Adopting SOLVER‑Ralph trades:
- extra up-front structure (typing, gates, profiles, records),
for:
- lower drift, higher replayability, and higher auditability under scale.

Common costs:
- writing/maintaining governance artifacts,
- running verification suites and managing oracle stability,
- portal overhead for binding decisions,
- stronger operational discipline (budgets, stop triggers).

---

## 5. Failure modes (what to watch for)

### 5.1 Policy theater

Documents exist but do not constrain reality (no enforcement at ports/gates).

### 5.2 Authority smuggling

Agent narratives or role labels become de facto approvals.

### 5.3 Exception flood

Exceptions become the normal path; integrity conditions are treated as negotiable.

### 5.4 Ghost inputs

Agents rely on context that is not captured in refs/payload (loss of replayability).

---

## 6. Mapping guidance to knobs (directional)

This section is intentionally non-prescriptive; it provides a pattern:

- Higher drift risk → tighter gating + stricter verification profiles + stronger change routing
- Higher replayability need → stricter ref discipline + more durable evidence bundling
- Higher time pressure → smaller envelopes, fewer required suites, clearer stop triggers, narrower scope per iteration

Concrete selections belong in SR‑DIRECTIVE for an instance.

---

## 7. Appendix: Quick fit checklist (directional)

Answer “yes/no”:

1) Can we define candidate artifacts and evidence-producing checks?
2) Does silent drift pose major risk?
3) Do we need a durable justification trail?
4) Do constraints interact non-locally?
5) Do we need bounded exploration + forced convergence?
6) Are exceptions inevitable, and do we need them formalized?

If most are “yes,” SOLVER‑Ralph is likely a strong fit.

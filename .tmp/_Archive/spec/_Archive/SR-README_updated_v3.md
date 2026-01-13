# SOLVER-Ralph Spec Set (docs/spec)

This directory contains the governed SR-* documents that define SOLVER-Ralph.
The goal is **trustworthy, auditable agentic work** under explicit human authority.

**Current build focus (MVP):** implement the **semantic-ralph-loop core**:
bounded semantic work → structured evidence → **human approval** → completion event.
Everything else (storage/UI/orchestration/decomposition automation/adapters) is a **stub/commodity** unless explicitly authorized.

---

## How to use this spec set

### What is binding vs non-binding
- **Binding (platform-definition):** documents that define meaning/invariants/mechanics/types (CONTRACT, SPEC, TYPES).
  - These constrain what the code **must** do.
- **Binding (build-execution):** documents that constrain how agents work during development (PLAN, DIRECTIVE, AGENTS, CHANGE).
  - These are `normative_status: normative` but do not define platform semantics.
- **Directional:** guidance and rationale; not binding unless referenced by a normative requirement (INTENT, GUIDE).
- **Non-binding (orientation/index):** navigation and helpful explanations; may not introduce mechanics or override binding docs (README, MODEL).

See SR-TYPES §2.2 for the authoritative normative_status enum and SR-CHARTER § Normative status categories for summary.

If you see a conflict: **binding docs win**. If still ambiguous: **stop and escalate** to human authority.

---

## Quick start (what to read first)

1) **SR-CHARTER** — project purpose, scope, milestones, and build focus  
2) **SR-CONTRACT** — invariants + authority boundaries (highest precedence)  
3) **SR-SPEC** — platform mechanics: events, records, gates, replay/projections  
4) **SR-TYPES** — canonical schema/type registry  
5) **SR-DIRECTIVE** / **SR-PLAN** — runtime/build execution policy + plan instance  
6) Work-surface/oracle docs for semantic loops:
   - **SR-WORK-SURFACE**
   - **SR-PROCEDURE-KIT**
   - **SR-SEMANTIC-ORACLE-SPEC**
   - **SR-EVENT-MANAGER**
   - **SR-AGENT-WORKER-CONTRACT**

---

## Folder meanings 

- `charter/`  
  SR-CHARTER (typed problem statement; sets scope and priorities)

- `platform/` (**binding**)  
  Meaning, invariants, mechanics, schema, deterministic projections:
  SR-CONTRACT, SR-SPEC, SR-TYPES, SR-EVENT-MANAGER, SR-WORK-SURFACE,
  SR-PROCEDURE-KIT, SR-SEMANTIC-ORACLE-SPEC, SR-AGENT-WORKER-CONTRACT

- `build-governance/` (**binding for development**)  
  How changes/exceptions/agent constraints work during build:
  SR-CHANGE, SR-AGENTS, SR-EXCEPTIONS

- `program/` (**binding for a specific build/run**)  
  Plan instance + execution policy:
  SR-PLAN, SR-DIRECTIVE

- `orientation/` (**non-binding**)  
  Helpful explanations/rationale only:
  SR-GUIDE, SR-INTENT, SR-MODEL

---

## Precedence rules (conflict resolution)

### Platform-definition precedence (meaning / invariants / mechanics)
1) SR-CONTRACT  
2) SR-SPEC  
3) SR-TYPES  
4) SR-WORK-SURFACE  
5) SR-SEMANTIC-ORACLE-SPEC  
6) SR-EVENT-MANAGER  
7) SR-AGENT-WORKER-CONTRACT  
8) SR-GUIDE / SR-INTENT / SR-MODEL (non-binding)

### Build-execution precedence (agent behavior / process / gating)
1) SR-CONTRACT  
2) SR-AGENTS  
3) SR-CHANGE  
4) SR-DIRECTIVE  
5) SR-PLAN  
6) Task-local instructions (must not violate higher precedence)

---

## Change control and exceptions (mandatory)

- Any change to binding docs MUST follow **SR-CHANGE**.
- Any deviation from governed documents or required gates requires a recorded exception in **SR-EXCEPTIONS**.
- Agents may propose and implement, but may not declare binding status or completion without required evidence and approvals.

---

## One sentence definition of the core (semantic-ralph-loop)

A **semantic-ralph-loop** is a bounded work unit that:
1) binds to a work surface (intake + procedure stage),
2) produces a candidate and structured oracle evidence,
3) requests human approval at trust boundaries,
4) and is considered complete only when evidence + approval records exist.

If you are unsure whether work is in-scope for the MVP: **stop and escalate** to human authority.
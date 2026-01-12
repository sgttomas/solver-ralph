---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-DIRECTIVE"
  type: "governance.dev_directive"
  title: "Development Directive (instance-specific)"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "normative"
  authority_kind: "process"
  governed_by: ["SR-CHANGE"]
  created: "2026-01-11"
  updated: "2026-01-11"
  tags: ["directive", "execution", "governance", "instance-1"]
  refs:
    # REQUIRED (depends_on)
    - kind: "GovernedArtifact"
      id: "SR-TYPES@<PINNED_VERSION>"
      rel: "depends_on"
      meta: { content_hash: "<sha256:...>" }
    - kind: "GovernedArtifact"
      id: "SR-CONTRACT@<PINNED_VERSION>"
      rel: "depends_on"
      meta: { content_hash: "<sha256:...>" }
    - kind: "GovernedArtifact"
      id: "SR-SPEC@<PINNED_VERSION>"
      rel: "depends_on"
      meta: { content_hash: "<sha256:...>" }
    - kind: "GovernedArtifact"
      id: "SR-PLAN-INSTANCE-1@<PINNED_VERSION>"
      rel: "depends_on"
      meta: { content_hash: "<sha256:...>" }
    # SUPPORTING (supported_by)
    - kind: "GovernedArtifact"
      id: "SR-ETT@<PINNED_VERSION>"
      rel: "supported_by"
      meta: { content_hash: "<sha256:...>" }
    - kind: "ProcessState"
      id: "PS-SOLVER-RALPH-INSTANCE-1-SAPS"
      rel: "supported_by"
      meta: { content_hash: "<sha256:...>" }
---

# SR-DIRECTIVE (Scaffold Template)

> Draft this document by **filling tables** (Gate Registry, Portal Playbooks, Profile Definitions, Plan‑to‑Workflow mapping) and keeping prose thin.

## 0. Change log

- 2026-01-11: initial scaffold.

## 1. Scope and authority

### 1.1 Purpose

(Describe: “how we execute SR‑PLAN instance‑1 using SR‑SPEC while satisfying SR‑CONTRACT; SR‑ETT provides the constraint‑placement lens.”)

### 1.2 Non-goals

- SR‑DIRECTIVE does not redefine binding semantics.
- SR‑DIRECTIVE does not invent new portal kinds.
- SR‑DIRECTIVE does not modify the SR‑PLAN deliverable inventory.

### 1.3 Precedence and conflict resolution

- If SR‑DIRECTIVE conflicts with SR‑CONTRACT or SR‑SPEC, SR‑CONTRACT/SR‑SPEC control.
- If the directive needs a new binding semantic, route through SR‑CHANGE.

## 2. Execution model

### 2.1 Canonical loop

Define the canonical workflow skeleton for a work unit, using SR‑SPEC objects/events (IterationStarted → Candidate → Oracle Run(s) → Evidence Bundle(s) → Portal decisions/approvals → Freeze).

### 2.2 Concurrency policy

(How many parallel loops; how attribution/evidence is kept separate; how budgets apply.)

### 2.3 Dependency-first scheduling

(How SR‑PLAN dependency edges map to workflow ordering; topological progression.)

## 3. Inputs and refs discipline

### 3.1 Required IterationStarted refs

List the minimum refs that must appear in IterationStarted.refs[] (governed artifacts + oracle suite(s) + loop + optional base candidate).

### 3.2 depends_on vs supported_by policy

Define how refs are classified:
- depends_on: blocking dependencies; changes can cause staleness propagation
- supported_by: provenance/audit; does not cause staleness blocking

## 4. Budgets and stop triggers

### 4.1 Budget policy

Fill in:
- per-loop defaults (iterations/time/cost)
- per-phase or per-package caps (optional)
- budget extension workflow + portal routing

### 4.2 Stop-the-line trigger registry

List mandatory triggers and any instance-specific triggers. Define:
- N for REPEATED_FAILURE
- routing: which portal handles which trigger
- waiver policy (what is waivable vs never waivable)

## 5. Verification profiles and oracle suites

> Paste the completed `profile_definitions_*.yaml` content here (or reference it as an appendix).

## 6. Portals and human judgment hooks

> Paste completed portal playbooks here (or reference appendices).

## 7. Gate registry

> Paste the filled Gate Registry table here.

## 8. Plan-to-workflow mapping

> Paste the filled Plan‑to‑Workflow mapping table here.

## 9. Exceptions, deviations, deferrals, waivers

Define:
- when exceptions are allowed
- required fields (scope, expiry, rationale)
- how exceptions are surfaced in approvals/freezes

## 10. SR-ETT membrane coverage matrix

Provide a short matrix mapping each harness to:
- gates enforcing it
- portals enforcing it
- profiles/oracles supporting it
- relief valves

## 11. Directive self-verification

List the oracle checks and human review steps required before adopting this SR‑DIRECTIVE as “current”.

## Appendices

- A. Gate Registry (CSV)
- B. Portal Playbooks
- C. Profile Definitions
- D. Plan‑to‑Workflow mapping (CSV)

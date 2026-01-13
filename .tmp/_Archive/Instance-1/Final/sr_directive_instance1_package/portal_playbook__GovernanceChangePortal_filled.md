---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "GovernanceChangePortal"
  type: "config.portal_playbook"
  title: "GovernanceChangePortal — Playbook"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "binding"
  authority_kind: "config"
  created: "2026-01-11"
  updated: "2026-01-11"
  tags: ["sr-directive", "portal", "playbook"]
---

# GovernanceChangePortal — Playbook

## 1) Portal identification

- **portal_id:** `GovernanceChangePortal`
- **portal_kind:** `governance_change`
- **scope:** `instance:PS-SOLVER-RALPH-INSTANCE-1-SAPS (governance artifacts + policy)`

## 2) Purpose and boundary

- **Purpose (one sentence):**  
  Provide an audited, human-only workflow to propose and ratify changes to governed policy (gates, profiles, portal policies, stop-trigger thresholds, and directive structure).

- **Trust boundary being crossed (SR-ETT membranes):**  
  Change; Authority Boundary; Evidence Integrity; Accountability

- **What this portal MUST NOT do:**  
  - Grant waivers for integrity conditions.
- Edit evidence bundles or alter run outcomes.
- Allow uncontrolled changes without versioning + lineage.

## 3) Allowed request types

Allowed request types (each produces a binding record):

- **GOVERNANCE_CHANGE** — change SR-DIRECTIVE policy prose, gate registry entries, or plan-to-workflow mapping.
- **ORACLE_SUITE_OR_PROFILE_CHANGE** — change oracle suite definitions or verification profile selection matrix.
- **PORTAL_POLICY_CHANGE** — adjust allowed request types, role rules, or checklists for portals.
- **STOPTRIGGER_POLICY_CHANGE** — change thresholds (e.g., REPEATED_FAILURE N), budgets policy, or escalation routing.


## 4) Actor rules

- **Binding actor kind:** `HUMAN` only.
- **Minimum roles (suggested):**
  - `GovernanceSteward` (required approver)
  - `EngineeringLead` (recommended co-approver for changes affecting enforcement)
- **Change control:**
  - All approved changes MUST be applied via new governed artifact versions (pinned IDs, supersedes lineage).


## 5) Preconditions

- Request must include:
  - target artifact(s) + version(s)
  - proposed change set (diff or structured patch)
  - impact analysis: which gates/workflows/deliverables are affected
  - migration plan (if any)
- If the change affects verification (suites/profiles), request must include:
  - new suite/profile definitions
  - determinism + environment pinning rationale
  - updated waiver policy implications
- If the change affects stop triggers/budgets, request must include:
  - updated thresholds/ceilings
  - failure-mode routing expectations


## 6) Evidence review checklist (Accountability harness)

Reviewers MUST confirm:

1. **Versioning + lineage**
   - new versions have stable IDs and a clear supersedes chain.
2. **Coherence**
   - proposed changes do not contradict SR-CONTRACT or SR-SPEC.
   - plan-to-workflow, gate registry, and profile definitions remain consistent.
3. **Enforceability**
   - each referenced gate has a realizable enforcement mechanism and evidence plan.
4. **Blast radius**
   - list deliverables/work-units affected and any migration steps.
5. **Integrity preserved**
   - no change permits bypassing integrity conditions.


## 7) Decision procedure (what to do)

- Triage:
  - If request is actually an exception/waiver for a single scope → route to ExceptionApprovalPortal.
  - If request is a release action → route to ReleaseApprovalPortal.
- For governance changes:
  1) Require a structured change proposal (diff/patch).
  2) Require at least one enforcement-owner sign-off when enforcement changes.
  3) Approve or reject.
- On approval, require:
  - new versions of affected governed artifacts
  - updated pinned references in SR-DIRECTIVE assembly


## 8) Outputs (binding records emitted)

Portal MUST emit binding records/events:

- `record.governance_change_request` + `event.GovernanceChangeRequested`
- `record.decision(kind=GOVERNANCE_CHANGE_APPROVAL|REJECTION)` + `event.DecisionRecorded`
- (When applicable) `record.governed_artifact_version` updates (new versions with supersedes lineage)

Each decision record MUST include:
- approver identity (HUMAN)
- rationale + change summary
- affected gate_ids / profile_ids / portal_ids
- migration notes (if any)


## 9) Failure handling and routing

- If proposal is incoherent with SR-CONTRACT/SR-SPEC → **REJECT** and record rationale.
- If proposal introduces unenforceable gates/profiles → **REJECT**.
- If proposal attempts to waive integrity conditions → **REJECT** and route to remediation.
- Rejections must be recorded; “silent decline” is forbidden.


## 10) Auditability

- Maintain an immutable change log: every request, comment, diff, and final decision is an event-linked artifact.
- Provide audit views:
  - by artifact id/version chain
  - by gate_id/profile_id touched
  - by approver identity


## 11) Cross-references

- **SR-CONTRACT:** C-META-* (versioning/lineage), C-EXC-* (exception policy boundaries), C-DEC-1 (decision recording), C-TB-5 (human binding).
- **SR-SPEC:** §3.3 (metadata/lineage), §2.3.5 (governed artifacts), §1.11 (verification profiles), Appendix A/C.
- **SR-ETT:** Change + Authority Boundary membranes.


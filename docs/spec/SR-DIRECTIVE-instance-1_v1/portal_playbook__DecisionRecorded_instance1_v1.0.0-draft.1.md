---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-DIRECTIVE.PLAYBOOK.DecisionRecorded.instance1"
  type: "config.portal_playbook"
  title: "Portal Playbook — DecisionRecorded (Stop/Arbitration) (instance-1)"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "directional"
  authority_kind: "config"
  created: "2026-01-11"
  updated: "2026-01-11"
  tags: ["sr-directive", "portal", "playbook", "decisions", "stop-triggers", "instance-1"]
---

# DecisionRecorded — Decision Recording Playbook (first pass)

> **Purpose:** Provide a human-only, auditable mechanism to resolve **stop-the-line escalations** and other binding arbitration outcomes by emitting `DecisionRecorded`.

## 1) Portal identification

- **portal_id:** `DecisionRecorded` *(semantic label: “decision recording”; the binding artifact is the `DecisionRecorded` event)*  
- **portal_kind:** `decision_portal` *(records `DecisionRecorded`)*  
- **scope:** `instance` *(SOLVER‑Ralph instance‑1)*

## 2) Purpose and boundary

- **Purpose (one sentence):**  
  Record binding arbitration decisions that unblock (or terminate) work when automation cannot safely decide.

- **Trust boundary being crossed (SR‑ETT membranes):**  
  **Authority & Integrity**; **Operational**; **Accountability** *(secondary: Change; Resource)*

- **What this portal MUST NOT do:**  
  - MUST NOT “approve release” (ReleaseApprovalPortal does that).  
  - MUST NOT grant waivers (ExceptionApprovalPortal does that).  
  - MUST NOT silently weaken oracle integrity rules; systemic changes route to GovernanceChangePortal.

## 3) Allowed request types

- [x] `decision_record` *(resolve stop triggers / arbitration)*  
- [ ] `approval_request`  
- [ ] `waiver_request`  
- [ ] `freeze_request`  
- [ ] `governance_change_request`

## 4) Actor rules

- **Allowed actor kinds:** `HUMAN only`
- **Identity requirements:** stable, verifiable identity; authorized to arbitrate stop triggers for this instance.
- **Attribution policy:** decision must be attributable to the human actor id.

## 5) Preconditions

- A triggering condition exists (e.g., `StopTriggered` event, blocked gate, budget exhaustion, repeated failure threshold, integrity fault, staleness gating block).
- The decision request includes:
  - trigger code (e.g., `STOP_TRIGGER:REPEATED_FAILURE`, `STOP_TRIGGER:ORACLE_FLAKE`, etc.)
  - scope (loop_id, iteration_id; candidate_id if relevant)
  - subject_refs[] and evidence_refs[] adequate to justify the decision
- Evidence referenced must be retrievable (EVIDENCE_MISSING blocks).
- Decision MUST NOT claim to waive a non-waivable integrity condition.

## 6) Inputs required at submission time

Minimum payload for `POST /decisions`:

- `trigger` (string; include stop trigger id)
- `scope` (loop_id, iteration_id, candidate_id as applicable)
- `decision` (e.g., continue | pause | terminate | rerun_oracles | require_human_review | route_to_portal:<id>)
- `rationale` (plain text)
- `subject_refs[]` (what the decision is about)
- `evidence_refs[]` (evidence considered)
- `exceptions_acknowledged[]` (explicit; may be empty)
- optional: `is_precedent`, `applicability` (if the decision sets a reusable policy)

## 7) Procedure

1) **Intake + validation (SYSTEM):**
   - validate actor identity and payload shape
   - validate evidence retrievability

2) **Review (HUMAN):**
   - review evidence, history of attempts, stop trigger conditions
   - determine safest next action under a conservative posture

3) **Decision (HUMAN):**
   - choose an action; document rationale and scope
   - if decision implies governance change or semantics change → route to GovernanceChangePortal instead

4) **Record decision (SYSTEM emits from HUMAN action):**
   - emit `DecisionRecorded`
   - follow-on events may occur (LoopResumed, LoopClosed, etc.) according to the governor and SR‑SPEC state machine rules.

## 8) Outputs (binding records emitted)

- **Primary record type:** `DecisionRecorded`
- **Required fields (per SR‑SPEC):**
  - stable decision_id
  - trigger + scope
  - decision + rationale
  - subject_refs[] + evidence_refs[]
  - exceptions_acknowledged[] (explicit)

- **Follow-on events (examples):**  
  - `LoopResumed` / `LoopClosed`  
  - `ReEvaluationTriggered` (if staleness or re-verify chosen)

## 9) Failure handling and routing

- **If preconditions fail:** reject; require missing scope/evidence.
- **If evidence missing/unfetchable:** treat as non-waivable; halt and resolve evidence availability first.
- **If integrity conditions detected:** do not proceed via waiver; require remediation or governance-level incident handling.
- **If systemic change needed (stop trigger definitions, oracle policy weakening, semantics changes):** route to GovernanceChangePortal.

## 10) Auditability

Store:

- decision payload hash
- reviewer identity + timestamp
- evidence bundle ids reviewed
- resulting decision_id + scope

Retention expectation: baseline-grade (decisions justify continuation/termination and must be replayable).

## 11) Cross-references

- **Gate routing (from Gate Registry):** arbitration referenced by gates: `G-00, G-10, G-15, G-20, G-30, G-31, G-40, G-50, G-60, G-70, G-71, G-80, G-90`  
- **SR‑CONTRACT clauses:** `C-DEC-1; C-LOOP-1; C-LOOP-3; C-EVID-6`  
- **SR‑SPEC sections:** `§1.11.2 (DecisionRecorded event); §2.3.7 (Decisions API); stop trigger + loop lifecycle sections`  
- **SR‑ETT membranes:** Authority & Integrity; Operational; Accountability  
- **SR‑PLAN items:** gates and deliverables that exercise stop triggers and arbitration (notably D‑22, D‑27, D‑35)  

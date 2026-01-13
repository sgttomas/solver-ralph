---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "ExceptionApprovalPortal"
  type: "config.portal_playbook"
  title: "ExceptionApprovalPortal — Playbook"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "binding"
  authority_kind: "config"
  created: "2026-01-11"
  updated: "2026-01-11"
  tags: ["sr-directive", "portal", "playbook"]
---

# ExceptionApprovalPortal — Playbook

## 1) Portal identification

- **portal_id:** `ExceptionApprovalPortal`
- **portal_kind:** `exception_approval`
- **scope:** `instance:PS-SOLVER-RALPH-INSTANCE-1-SAPS (all phases)`

## 2) Purpose and boundary

- **Purpose (one sentence):**  
  Provide a human-only, fully-audited relief valve for *explicit* oracle FAILs and time-boxed deviations/deferrals, without ever bypassing integrity conditions.

- **Trust boundary being crossed (SR-ETT membranes):**  
  Authority Boundary; Evidence Integrity; Change; Accountability

- **What this portal MUST NOT do:**  
  - Mint or modify verification evidence.
- Waive integrity conditions (EVIDENCE_MISSING, ORACLE_TAMPER, ORACLE_GAP, ORACLE_FLAKE, ORACLE_ENV_MISMATCH).
- Create implicit approvals (all outcomes must be emitted as binding records + events).

## 3) Allowed request types

The portal accepts the following request types (each produces a binding record):

- **WAIVER_ORACLE_FAIL** — waive a *required oracle FAIL outcome* for a specific scope (deliverable/work-unit/candidate), with explicit rationale, conditions, and expiry.
- **DEFERRAL** — defer a binding requirement to a later deliverable/phase with a time-box and compensating controls.
- **DEVIATION** — approve a deviation from a workflow requirement (e.g., CI shape) with compensating controls.
- **ENG_ACCEPT** — record engineering acceptance approval for plan deliverables that require human sign-off (treated as a request type here to stay within the 3 seeded portals).
- **BUDGET_ESCALATION** — approve a budget increase / scope change with explicit accounting and stop-trigger reassessment.


## 4) Actor rules

- **Binding actor kind:** `HUMAN` only (portal refuses non-human submissions for binding outcomes).
- **Minimum roles (suggested):**
  - `ExceptionReviewer` for WAIVER/DEFERRAL/DEVIATION
  - `EngineeringApprover` for ENG_ACCEPT
  - `BudgetApprover` for BUDGET_ESCALATION
- **Separation of duties (recommended):**
  - Requester SHOULD NOT be the sole approver for WAIVER/DEVIATION.
- **Attribution requirements:**
  - portal must record: human identity, timestamp, rationale, and linked evidence bundle IDs.


## 5) Preconditions

- **Integrity conditions are non-waivable.** If any integrity condition is active for the scope, the portal MUST refuse WAIVER/DEFERRAL/ENG_ACCEPT until the integrity condition is resolved (or escalated to GovernanceChangePortal if policy must change).
- Request must reference a concrete scope:
  - deliverable_id (D-##) and/or work_unit_id and/or candidate_id
  - gate_id(s) impacted
- Request must include links to the relevant evidence bundles (gate packets) and the current verification profile selection for that scope.
- For **BUDGET_ESCALATION**, request must include current budget burn and proposed new ceilings.


## 6) Evidence review checklist (Accountability harness)

Reviewers MUST confirm (and the portal SHOULD checklist):

1. **Scope clarity**
   - Which deliverable/work-unit/candidate is affected?
   - Which gate(s) are being relieved?
2. **Evidence completeness**
   - Evidence bundle manifest(s) present and validated (no EVIDENCE_MISSING).
   - Runs referenced include environment fingerprint + suite hash.
3. **Failure semantics**
   - Confirm the failure is an *oracle FAIL outcome* (waivable) vs an *integrity condition* (non-waivable).
4. **Compensating controls**
   - Additional tests / additional reviewers / narrower rollout / monitoring requirements.
5. **Expiry & revisit**
   - Explicit expiry date/time or “next deliverable by-id” for deferrals.
6. **Staleness impact**
   - Whether the exception creates downstream staleness and how it will be routed.


## 7) Decision procedure (what to do)

**WAIVER_ORACLE_FAIL**
- Verify: requested relief is ONLY for explicit oracle FAIL(s).
- Require: rationale, conditions, expiry, and scope narrowing (prefer smallest scope).
- Emit: `GateWaiverRecorded` with:
  - gate_id, oracle_id(s), suite_id+version, run_id(s), scope refs, rationale, conditions, expiry, approver identity.

**DEFERRAL**
- Require: target deliverable/phase, time-box, and compensating controls.
- Emit: `DeferralRecorded` with scope + destination + controls.

**DEVIATION**
- Require: exact workflow delta and compensating controls.
- Emit: `DeviationRecorded`.

**ENG_ACCEPT**
- Require: acceptance checklist completed for the deliverable(s), including links to required evidence bundles.
- Emit: `ApprovalRecorded(kind=ENG_ACCEPT)`.

**BUDGET_ESCALATION**
- Require: updated budget ceilings, justification, and stop-trigger policy re-check.
- Emit: `DecisionRecorded(kind=BUDGET_ESCALATION)` + `BudgetPolicyUpdated` (if your event taxonomy includes it) OR a `DecisionRecorded` that carries the new ceilings.


## 8) Outputs (binding records emitted)

Portal MUST emit **binding records** (and corresponding events) — no implicit outcomes:

- `record.gate_waiver` + `event.GateWaiverRecorded`
- `record.deferral` + `event.DeferralRecorded`
- `record.deviation` + `event.DeviationRecorded`
- `record.approval(kind=ENG_ACCEPT)` + `event.ApprovalRecorded`
- `record.decision(kind=BUDGET_ESCALATION)` + `event.DecisionRecorded`

Each record MUST include:
- actor_kind = HUMAN
- stable identity reference for the approver
- linked evidence bundle IDs / run IDs
- scope refs (deliverable/work-unit/candidate)
- rationale + conditions + expiry (where applicable)


## 9) Failure handling and routing

- If the request attempts to waive an integrity condition → **REJECT** and instruct remediation; optionally escalate to GovernanceChangePortal if policy itself is inconsistent.
- If evidence bundles are missing/invalid → **REJECT** and route to remediation (integrity conditions).
- If scope is ambiguous → **REJECT**; request must be precise.
- If the request is actually a governance change (new gate, new profile, new portal policy) → **ROUTE** to **GovernanceChangePortal**.
- All rejections must be recorded as `DecisionRecorded(kind=REJECTED)` with rationale.


## 10) Auditability

- All requests, comments, and final outcomes are written to the event log.
- Portal must provide a stable audit view keyed by:
  - request_id, approver identity, gate_id(s), run_id(s), evidence_bundle_id(s), deliverable_id(s).
- Portal must not store “final authority” in UI state; authority lives in records/events.


## 11) Cross-references

- **SR-CONTRACT:** C-EXC-4, C-EXC-5 (waivers), C-EXC-2 (freeze must surface exceptions), C-TB-5 (human-only binding), C-DEC-1 (decisions).
- **SR-SPEC:** §1.9 (waiver scope + integrity conditions), §2.3.4 (Approvals/Decisions), Appendix C (integrity conditions).
- **SR-ETT:** Authority Boundary; Evidence Integrity harness membranes.


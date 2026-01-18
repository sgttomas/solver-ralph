## 6. Portals and human judgment hooks

Only the portals with provided seed documents are defined for this instance; additional operational needs are expressed as **request types** within these portals (see playbooks).

**Classification note:** Portal playbooks use `authority_kind: "config"` because they configure portal behavior (allowed request types, actor rules, decision procedures). They are not process definitions themselves but rather configuration artifacts that parameterize the portal machinery defined in SR-SPEC.

### 6.1 HumanAuthorityExceptionProcess

---
solver_ralph:
 schema: "solver-ralph.artifact-metadata/v1"
 id: "HumanAuthority"
 type: "config.portal_playbook"
 title: "Human Authority — Exception Process"
 version: ""
 status: "draft"
 normative_status: "normative"
 authority_kind: "config"
 created: "2026-01-11"
 updated: "2026-01-11"
 tags: ["sr-directive", "portal", "playbook"]
---

# Human Authority Exception Process — Playbook

## 1) Portal identification

- **authority:** Human Authority (Ryan) recorded in SR-EXCEPTIONS
- **portal_kind:** `exception_approval`
## 2) Purpose and boundary

- **Purpose (one sentence):**
 Provide a human-only, fully-audited relief valve for *explicit* oracle FAILs and time-boxed deviations/deferrals, without ever bypassing integrity conditions.

- **Trust boundary being crossed (SR-CONTRACT/SR-SPEC trust boundaries membranes):**
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
- **STOP_TRIGGER_RECOVERY** — resolve stop triggers routed here (e.g., BUDGET_EXHAUSTED, NO_ELIGIBLE_WORK, REPEATED_FAILURE when exception is allowed) with explicit DecisionRecorded/Loop resume path.


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
- For **STOP_TRIGGER_RECOVERY**, honor the `recommended_portal` in the StopTriggered payload; integrity-driven triggers (ORACLE_*, EVIDENCE_MISSING, profile/stage misbinding) route to GovernanceChangePortal.


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
- **SR-SPEC:** §1.9 (waiver scope + integrity conditions), § (Approvals/Decisions), Appendix C (integrity conditions).
- **SR-CONTRACT/SR-SPEC trust boundaries:** Authority Boundary; Evidence Integrity harness membranes.



### 6.2 GovernanceChangePortal

---
solver_ralph:
 schema: "solver-ralph.artifact-metadata/v1"
 id: "GovernanceChangePortal"
 type: "config.portal_playbook"
 title: "GovernanceChangePortal — Playbook"
 version: ""
 status: "draft"
 normative_status: "normative"
 authority_kind: "config"
 created: "2026-01-11"
 updated: "2026-01-11"
 tags: ["sr-directive", "portal", "playbook"]
---

# GovernanceChangePortal — Playbook

## 1) Portal identification

- **portal_id:** `GovernanceChangePortal`
- **portal_kind:** `governance_change`
## 2) Purpose and boundary

- **Purpose (one sentence):**
 Provide an audited, human-only workflow to propose and ratify changes to governed policy (gates, profiles, portal policies, stop-trigger thresholds, and directive structure) and to adjudicate stop triggers routed here (integrity/stage/profile gaps).

- **Trust boundary being crossed (SR-CONTRACT/SR-SPEC trust boundaries membranes):**
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
- **STOP_TRIGGER_RESPONSE** — process StopTriggered events that name GovernanceChangePortal (integrity violations, EVIDENCE_MISSING, STAGE_UNKNOWN, SEMANTIC_PROFILE_MISSING, WORK_SURFACE_MISSING) and record the required governance decision.


## 4) Actor rules

- **Binding actor kind:** `HUMAN` only.
- **Minimum roles (suggested):**
 - `GovernanceSteward` (required approver)
 - `EngineeringLead` (recommended co-approver for changes affecting enforcement)
- **Change control:**
 - All approved changes MUST be applied via new governed artifact revisions (stable IDs; content hashes recorded).


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
- StopTriggered payloads with `recommended_portal=GovernanceChangePortal` must include the trigger code, affected loop/work surface context, and the integrity/stage/profile gap that caused the stop.


## 6) Evidence review checklist (Accountability harness)

Reviewers MUST confirm:

1. **Versioning + lineage**
 - new revisions have stable IDs and a clear revision trail in git.
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
 - If request is actually an exception/waiver for a single scope → route to HumanAuthorityExceptionProcess.
 - If request is a release action → route to ReleaseApprovalPortal.
- For governance changes:
 1) Require a structured change proposal (diff/patch).
 2) Require at least one enforcement-owner sign-off when enforcement changes.
 3) Approve or reject.
- On approval, require:
 - new revisions of affected governed artifacts
 - updated pinned references in SR-DIRECTIVE assembly


## 8) Outputs (binding records emitted)

Portal MUST emit binding records/events:

- `record.governance_change_request` + `event.GovernanceChangeRequested`
- `record.decision(kind=GOVERNANCE_CHANGE_APPROVAL|REJECTION)` + `event.DecisionRecorded`
- (When applicable) `record.governed_artifact_version` updates (new revisions with supersedes lineage)

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
- **SR-SPEC:** §3.3 (metadata/lineage), § (governed artifacts), §1.11 (verification profiles), Appendix A/C.
- **SR-CONTRACT/SR-SPEC trust boundaries:** Change + Authority Boundary membranes.



### 6.3 ReleaseApprovalPortal

---
solver_ralph:
 schema: "solver-ralph.artifact-metadata/v1"
 id: "ReleaseApprovalPortal"
 type: "config.portal_playbook"
 title: "ReleaseApprovalPortal — Playbook"
 version: ""
 status: "draft"
 normative_status: "normative"
 authority_kind: "config"
 created: "2026-01-11"
 updated: "2026-01-11"
 tags: ["sr-directive", "portal", "playbook"]
---

# ReleaseApprovalPortal — Playbook

## 1) Portal identification

- **portal_id:** `ReleaseApprovalPortal`
- **portal_kind:** `release_approval`
## 2) Purpose and boundary

- **Purpose (one sentence):**
 Provide a human-only, audited release decision that references a FreezeRecord and explicitly acknowledges the verification posture and active exceptions.

- **Trust boundary being crossed (SR-CONTRACT/SR-SPEC trust boundaries membranes):**
 Authority Boundary; Change; Accountability; Event Integrity

- **What this portal MUST NOT do:**
 - Change verification results.
- Release without a FreezeRecord.
- Hide or omit active exceptions from the release decision.

## 3) Allowed request types

Allowed request types:

- **RELEASE_APPROVAL** — approve publishing/shipping a baseline snapshot identified by `freeze_id`.
- **RELEASE_HOLD** — place a hold on a freeze_id with rationale (optional but useful).


## 4) Actor rules

- **Binding actor kind:** `HUMAN` only.
- **Minimum roles (suggested):**
 - `ReleaseApprover` (required)
 - `EngineeringApprover` (recommended co-approver for high-risk releases)
- Release approval MUST reference the approver’s identity and the exact freeze_id.


## 5) Preconditions

- A `FreezeRecord` exists for the proposed release baseline:
 - lists included candidates/artifacts by content hash
 - lists active exceptions/waivers/deferrals affecting included items
 - is itself content-addressed and recorded in the event log
- For every included candidate:
 - Verified(STRICT) computed and recorded OR explicitly listed as exception with scope/expiry
 - no active integrity conditions


## 6) Evidence review checklist (Accountability harness)

Reviewers MUST confirm:

1. **Freeze completeness**
 - freeze_id exists; artifact list complete; hashes present.
2. **Verification posture**
 - required suites PASS per profile OR covered by recorded waivers (FAIL only).
3. **Integrity conditions**
 - confirm none of the non-waivable integrity conditions are active for included items.
4. **Exceptions surfaced**
 - every waiver/deferral/deviation is listed and acknowledged.
5. **Rollback / recovery**
 - rebuild/replay evidence available (where applicable).


## 7) Decision procedure (what to do)

- Validate freeze_id and fetch its contents.
- Validate verification summary (computed) and integrity-condition summary.
- Validate exception acknowledgements:
 - approver must explicitly check “I acknowledge active exceptions listed in FreezeRecord”.
- Approve or hold:
 - On approval: emit `ApprovalRecorded(kind=RELEASE_APPROVAL)` referencing freeze_id.
 - On hold: emit `DecisionRecorded(kind=RELEASE_HOLD)` with rationale and scope.


## 8) Outputs (binding records emitted)

Portal MUST emit binding records/events:

- `record.approval(kind=RELEASE_APPROVAL)` + `event.ApprovalRecorded`
 - refs: freeze_id, included candidates, verification summary artifact, exception list
- `record.decision(kind=RELEASE_HOLD)` + `event.DecisionRecorded` (optional)

All outputs MUST be event-linked and attributable to a HUMAN approver.


## 9) Failure handling and routing

- If FreezeRecord missing/incomplete → **REJECT** and route to remediation (no release).
- If any non-waivable integrity condition is active → **REJECT** (stop-the-line).
- If verification evidence missing → **REJECT** (integrity).
- If governance changes are required to proceed → **ROUTE** to GovernanceChangePortal.


## 10) Auditability

- Release approvals are immutable records; UI state is non-binding.
- Provide audit views:
 - by freeze_id
 - by included candidates and evidence bundle IDs
 - by approver identity and timestamp


## 11) Cross-references

- **SR-CONTRACT:** C-SHIP-1 (Shippable), C-EXC-2 (freeze surfaces exceptions), C-TB-5 (human binding).
- **SR-SPEC:** § (Freeze), §1.12 (Shippable), Appendix C (integrity conditions).
- **SR-CONTRACT/SR-SPEC trust boundaries:** Authority Boundary + Event Integrity membranes.

---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "ReleaseApprovalPortal"
  type: "config.portal_playbook"
  title: "ReleaseApprovalPortal — Playbook"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "binding"
  authority_kind: "config"
  created: "2026-01-11"
  updated: "2026-01-11"
  tags: ["sr-directive", "portal", "playbook"]
---

# ReleaseApprovalPortal — Playbook

## 1) Portal identification

- **portal_id:** `ReleaseApprovalPortal`
- **portal_kind:** `release_approval`
- **scope:** `instance:PS-SOLVER-RALPH-INSTANCE-1-SAPS (release baselines only)`

## 2) Purpose and boundary

- **Purpose (one sentence):**  
  Provide a human-only, audited release decision that references a FreezeRecord and explicitly acknowledges the verification posture and active exceptions.

- **Trust boundary being crossed (SR-ETT membranes):**  
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
- **SR-SPEC:** §2.3.6 (Freeze), §1.12 (Shippable), Appendix C (integrity conditions).
- **SR-ETT:** Authority Boundary + Event Integrity membranes.


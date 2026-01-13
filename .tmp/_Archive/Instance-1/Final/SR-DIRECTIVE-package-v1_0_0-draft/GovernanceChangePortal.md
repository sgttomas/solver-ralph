---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "PLAYBOOK-GovernanceChangePortal"
  type: "config.portal_playbook"
  title: "GovernanceChangePortal Playbook"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "normative"
  authority_kind: "config"
  governed_by: ["SR-CHANGE"]
  created: "2026-01-11"
  updated: "2026-01-11"
  tags: ["sr-directive", "portal", "playbook", "governance-change"]
  refs:
    - kind: "GovernedArtifact"
      id: "SR-CONTRACT"
      rel: "depends_on"
    - kind: "GovernedArtifact"
      id: "SR-SPEC"
      rel: "depends_on"
---

# GovernanceChangePortal — Playbook

Human-only binding boundary for changes that alter governed artifacts or binding meanings.

## 1) Portal identification

- **portal_id:** `GovernanceChangePortal`
- **portal_kind:** `governance_change`
- **scope:** `global` (applies to all governance-touching work across instance-1)

## 2) Purpose and boundary

- **Purpose (one sentence):**
  Authorize changes to governed artifacts (SR-* documents) or any modification that would alter binding meanings, invariants, or semantics.

- **Trust boundary being crossed (SR-ETT membranes):**
  - Change (evolution/versioning control)
  - Authority & Integrity (admissibility control)
  - Accountability (proof/completeness control)

- **What this portal MUST NOT do:**
  - Mint verification evidence (that's oracle's job)
  - Override integrity conditions (non-waivable)
  - Approve changes without coherence evidence
  - Delegate approval authority to agents or SYSTEM

## 3) Allowed request types

- [x] governance_change_request
- [x] decision_request (stop-the-line arbitration when governance contradiction discovered)
- [ ] approval_request (use ReleaseApprovalPortal for release approvals)
- [ ] exception_request (use ExceptionApprovalPortal)
- [ ] budget_extension_request (use BudgetExtensionPortal)
- [ ] oracle_suite_change_request (use OracleSuiteChangePortal)
- [ ] freeze_request (use ReleaseApprovalPortal)

## 4) Actor rules

- **Allowed actor kinds:** `HUMAN only`
- **Identity requirements:**
  - Must be authenticated via OIDC (Zitadel)
  - Must have `governance:approve` permission
  - Identity MUST be bound to a durable actor_id
- **Attribution policy:**
  - `actor_kind`: HUMAN
  - `actor_id`: OIDC subject identifier
  - `timestamp`: ISO 8601 with timezone
  - `session_id`: Current session identifier
  - `approval_method`: UI portal interaction (not API bypass)

## 5) Preconditions

**Required commitment objects present:**
- Change request document (content-addressed)
- Impacted governed artifacts list (with current versions and content hashes)
- Coherence evidence (S11 audit output, if change affects cross-artifact consistency)
- Impact assessment (staleness fan-out analysis)

**Required refs present (request payload):**
- `SR-TYPES@{current_version}`
- `SR-CONTRACT@{current_version}`
- `SR-SPEC@{current_version}`
- `SR-DIRECTIVE@{current_version}`
- `SR-PLAN@{current_version}` (if plan change)
- All impacted governed artifacts with content hashes

**Staleness rules:**
- Request MUST be rejected if any impacted artifact has pending unapproved changes
- Request MUST be rejected if coherence evidence is stale relative to impacted artifacts
- Request MUST include staleness fan-out analysis (what downstream artifacts become stale)

## 6) Evidence review checklist (Accountability harness)

The human MUST review:

| Evidence | Required | Purpose |
|----------|----------|---------|
| Change request document | YES | Understand what is being changed |
| Impacted artifacts list | YES | Understand scope of change |
| Coherence audit (S11) | YES (if cross-artifact) | Verify consistency maintained |
| Staleness fan-out | YES | Understand downstream impact |
| Rationale/justification | YES | Understand why change is needed |
| Risk assessment | YES (for normative changes) | Understand risk profile |
| Rollback approach | YES (for normative changes) | Understand recovery path |

## 7) Decision procedure (what to do)

**Decision options:**
- **Approve**: Change is authorized; proceed with artifact modification
- **Reject**: Change is not authorized; must revise or abandon
- **Request changes**: Non-binding; ask for clarification or modification before re-submission
- **Escalate to**: Multi-stakeholder review (if change affects multiple owners)

**Decision rubric:**

1. **Coherence check**: Does the change maintain cross-artifact coherence? (S11)
2. **Contract compliance**: Does the change violate any SR-CONTRACT invariants?
3. **Scope appropriateness**: Is the change scoped correctly (not too broad, not too narrow)?
4. **Justification adequacy**: Is the rationale sufficient for the risk level?
5. **Rollback feasibility**: Can the change be reversed if problems are discovered?
6. **Staleness acknowledgment**: Are downstream staleness effects understood and acceptable?

## 8) Outputs (binding records emitted)

**Primary record type:** `GovernanceChangeApproved` or `GovernanceChangeRejected`

**Required fields:**

```yaml
governance_change_decision:
  decision_id: "{uuid}"
  decision_type: "governance_change"
  decision_outcome: "{approved | rejected | escalated}"
  change_request_id: "{uuid}"
  change_request_hash: "{sha256:...}"
  impacted_artifacts:
    - artifact_id: "{SR-*}"
      current_version: "{version}"
      current_hash: "{sha256:...}"
      proposed_version: "{version}"
  coherence_evidence_hash: "{sha256:...}"
  staleness_fanout:
    - artifact_id: "{...}"
      relationship: "depends_on"
  rationale: "{human-provided text}"
  conditions: []  # Optional conditions on approval
  actor:
    actor_kind: "HUMAN"
    actor_id: "{oidc_subject}"
    timestamp: "{iso8601}"
```

**Follow-on events:**
- `GovernanceChangeApproved` → Unlocks artifact modification workflow
- `GovernanceChangeRejected` → Loop may continue with alternative approach or pause

## 9) Failure handling and routing

| Condition | Action |
|-----------|--------|
| Preconditions fail | Reject request; return to requester with missing items list |
| Coherence evidence missing | Treat as incomplete; require S11 audit before re-submission |
| Impacted artifact has pending changes | Block until pending changes resolved |
| Integrity condition in evidence | Halt; route to investigation (cannot approve) |
| Multi-owner conflict | Escalate to multi-stakeholder review |
| Approval authority unclear | Escalate to governance owner |

## 10) Auditability

**What should be stored as evidence of the portal interaction:**
- Full request payload (content-addressed)
- All evidence bundles reviewed (content-addressed)
- Decision record (content-addressed)
- UI interaction log (submission timestamp, review duration, decision timestamp)
- Reviewer identity (actor_id, session_id)

**Retention expectation:** Indefinite (governance decisions are part of the audit trail)

## 11) Cross-references

- **SR-CONTRACT clauses:** C-TB-1, C-TB-2, C-TB-4, C-DEC-1, C-META-2
- **SR-SPEC sections:** §3.6 (Portals and Authority), §3.8 (Governance Change)
- **SR-ETT membranes:** Change, Authority & Integrity, Accountability
- **SR-PLAN items:** D-01 (Governance hygiene), D-19 (Governance API endpoints), D-30 (Portal workflows UI)

## 12) Escalation triggers

The following conditions require escalation beyond standard approval:

| Trigger | Escalation Target | Rationale |
|---------|-------------------|-----------|
| Change to SR-CONTRACT | Multi-stakeholder + architecture review | Invariant changes are foundational |
| Change to SR-PARADIGM | Multi-stakeholder + governance owner | Process changes affect all work |
| Change affecting > 3 artifacts | Multi-stakeholder review | Broad impact requires broader review |
| Change to binding semantics | Architecture review + governance owner | Semantic changes are high-risk |
| Rollback infeasible | Risk review + governance owner | Irreversible changes require extra scrutiny |

---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "PLAYBOOK-BudgetExtensionPortal"
  type: "config.portal_playbook"
  title: "BudgetExtensionPortal Playbook"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "normative"
  authority_kind: "config"
  governed_by: ["SR-CHANGE"]
  created: "2026-01-11"
  updated: "2026-01-11"
  tags: ["sr-directive", "portal", "playbook", "budget-extension"]
  refs:
    - kind: "GovernedArtifact"
      id: "SR-CONTRACT"
      rel: "depends_on"
    - kind: "GovernedArtifact"
      id: "SR-SPEC"
      rel: "depends_on"
---

# BudgetExtensionPortal — Playbook

Human-only boundary for budget extension decisions after BUDGET_EXHAUSTED.

## 1) Portal identification

- **portal_id:** `BudgetExtensionPortal`
- **portal_kind:** `budget_extension`
- **scope:** `loop` (applies per-loop for budget decisions)

## 2) Purpose and boundary

- **Purpose (one sentence):**
  Authorize budget extensions for loops that have exhausted their iteration, time, or cost budgets, after reviewing progress and determining whether extension is justified.

- **Trust boundary being crossed (SR-ETT membranes):**
  - Resource (budgets/economics control)
  - Operational (process/state-machine control)

- **What this portal MUST NOT do:**
  - Grant unlimited budgets
  - Override budget policy without governance change
  - Extend budgets without reviewing progress
  - Delegate budget authority to agents or SYSTEM

## 3) Allowed request types

- [x] budget_extension_request
- [x] decision_request (arbitration on loop continuation vs termination)
- [ ] approval_request (use ReleaseApprovalPortal)
- [ ] governance_change_request (use GovernanceChangePortal for policy changes)
- [ ] exception_request (use ExceptionApprovalPortal)
- [ ] oracle_suite_change_request (use OracleSuiteChangePortal)

## 4) Actor rules

- **Allowed actor kinds:** `HUMAN only`
- **Identity requirements:**
  - Must be authenticated via OIDC (Zitadel)
  - Must have `budget:extend` permission
  - Identity MUST be bound to a durable actor_id
- **Attribution policy:**
  - `actor_kind`: HUMAN
  - `actor_id`: OIDC subject identifier
  - `timestamp`: ISO 8601 with timezone

## 5) Preconditions

**Required commitment objects present:**
- BUDGET_EXHAUSTED stop-trigger event
- Loop state (PAUSED)
- Iteration history for loop
- Progress evidence (candidates produced, verification status)

**Required refs present (request payload):**
- `loop_id`: The loop requesting extension
- `budget_type`: iteration | time | cost
- `original_budget`: What was allocated
- `consumed_budget`: What has been used
- `requested_extension`: How much more is requested
- `iteration_history[]`: Summary of iterations so far
- `current_candidate`: Best candidate produced (if any)
- `SR-DIRECTIVE@{current_version}`: To verify budget policy

**Staleness rules:**
- Request must reflect current loop state
- Extension decision applies only to current loop instance

## 6) Evidence review checklist (Accountability harness)

The human MUST review:

| Evidence | Required | Purpose |
|----------|----------|---------|
| BUDGET_EXHAUSTED event | YES | Verify budget actually exhausted |
| Iteration history | YES | Understand progress made |
| Current candidate quality | YES | Assess whether extension likely to help |
| Failure patterns | YES | Identify if stuck in loop |
| Original budget rationale | RECOMMENDED | Understand initial estimate |
| Comparison to similar loops | RECOMMENDED | Calibrate expectation |

## 7) Decision procedure (what to do)

**Decision options:**
- **Extend**: Grant requested extension (or modified amount)
- **Terminate**: End loop without further iterations
- **Accept current**: If candidate exists and is acceptable, proceed to release
- **Escalate to**: GovernanceChangePortal (if budget policy needs revision)

**Decision rubric:**

1. **Progress evident**: Has the loop made meaningful progress?
2. **Not stuck**: Is the loop not in REPEATED_FAILURE pattern?
3. **Extension reasonable**: Is the requested extension reasonable given progress?
4. **Cost justified**: Is the cost of extension justified by value?
5. **Alternative available**: Is there a better alternative (accept current, redesign)?
6. **Total within limits**: Is total (original + extension) within global limits?

## 8) Outputs (binding records emitted)

**Primary record type:** `BudgetExtensionDecision`

```yaml
budget_extension_decision:
  decision_id: "{uuid}"
  decision_type: "budget_extension"
  decision_outcome: "{extended | terminated | accepted_current}"
  loop_id: "{uuid}"
  budget_type: "{iteration | time | cost}"
  original_budget: "{value}"
  consumed_budget: "{value}"
  extension_granted: "{value | null}"
  new_total_budget: "{value | null}"
  iteration_count_at_decision: "{n}"
  best_candidate_at_decision:
    candidate_id: "{uuid | null}"
    verification_status: "{...}"
  rationale: "{human-provided text}"
  conditions: []  # Optional conditions on extension
  actor:
    actor_kind: "HUMAN"
    actor_id: "{oidc_subject}"
    timestamp: "{iso8601}"
```

**Follow-on events:**
- `BudgetExtended` → Loop transitions from PAUSED to ACTIVE; new budget limit set
- `LoopTerminated` → Loop transitions to CLOSED (failed); deliverable not produced
- `CandidateAccepted` → Proceed to release workflow with current candidate

## 9) Failure handling and routing

| Condition | Action |
|-----------|--------|
| Loop not PAUSED | Reject; loop must be in BUDGET_EXHAUSTED state |
| Extension > 2x original | Escalate to PhaseGatePortal for review |
| REPEATED_FAILURE active | Consider termination; require explicit justification for extension |
| Total exceeds global limit | Escalate to GovernanceChangePortal |
| No progress evident | Recommend termination or redesign |

## 10) Auditability

**What should be stored as evidence of the portal interaction:**
- Full request payload (content-addressed)
- Iteration history snapshot (content-addressed)
- Decision record (content-addressed)
- UI interaction log

**Retention expectation:** Indefinite (budget decisions are part of the audit trail)

## 11) Cross-references

- **SR-CONTRACT clauses:** C-LOOP-1, C-LOOP-3
- **SR-SPEC sections:** §3.2.1 (Loop Mechanics), §3.2.1.2 (Budget Policy)
- **SR-ETT membranes:** Resource, Operational
- **SR-PLAN items:** All deliverables (budget applies to each loop)

## 12) Budget extension limits

| Extension Type | Default Limit | Escalation Threshold |
|---------------|---------------|---------------------|
| First extension | Up to 100% of original | No escalation |
| Second extension | Up to 50% of original | PhaseGatePortal review |
| Third+ extension | Up to 25% of original | GovernanceChangePortal |
| Total > 3x original | Requires governance review | GovernanceChangePortal |

## 13) Integration with REPEATED_FAILURE

If REPEATED_FAILURE (N≥3) is also active:
- Extension should not be granted without addressing failure pattern
- Consider: scope reduction, requirement clarification, approach change
- May require human intervention note for next iteration

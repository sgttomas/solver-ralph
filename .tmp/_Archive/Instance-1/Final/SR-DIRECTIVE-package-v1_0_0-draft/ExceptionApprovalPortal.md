---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "PLAYBOOK-ExceptionApprovalPortal"
  type: "config.portal_playbook"
  title: "ExceptionApprovalPortal Playbook"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "normative"
  authority_kind: "config"
  governed_by: ["SR-CHANGE"]
  created: "2026-01-11"
  updated: "2026-01-11"
  tags: ["sr-directive", "portal", "playbook", "exception-approval"]
  refs:
    - kind: "GovernedArtifact"
      id: "SR-CONTRACT"
      rel: "depends_on"
    - kind: "GovernedArtifact"
      id: "SR-SPEC"
      rel: "depends_on"
---

# ExceptionApprovalPortal — Playbook

Human-only boundary for deviation/deferral/waiver records.

## 1) Portal identification

- **portal_id:** `ExceptionApprovalPortal`
- **portal_kind:** `exception_approval`
- **scope:** `work_unit` (applies per-exception request)

## 2) Purpose and boundary

- **Purpose (one sentence):**
  Authorize formal exceptions (deviations, deferrals, waivers) that acknowledge a gap between actual state and governed requirements while maintaining accountability.

- **Trust boundary being crossed (SR-ETT membranes):**
  - Authority & Integrity (admissibility control)
  - Change (evolution/versioning control)
  - Accountability (proof/completeness control)

- **What this portal MUST NOT do:**
  - Approve exceptions for integrity conditions (ORACLE_TAMPER, ORACLE_GAP, ORACLE_ENV_MISMATCH, ORACLE_FLAKE, EVIDENCE_MISSING)
  - Create exceptions without scope and expiry
  - Approve exceptions that would rewrite governance (use GovernanceChangePortal)
  - Delegate exception authority to agents or SYSTEM
  - Approve exceptions without risk assessment

## 3) Allowed request types

- [x] exception_request (deviation | deferral | waiver)
- [x] decision_request (arbitration on exception scope or eligibility)
- [ ] approval_request (use ReleaseApprovalPortal)
- [ ] governance_change_request (use GovernanceChangePortal)
- [ ] budget_extension_request (use BudgetExtensionPortal)
- [ ] oracle_suite_change_request (use OracleSuiteChangePortal)
- [ ] freeze_request (use ReleaseApprovalPortal)

## 4) Actor rules

- **Allowed actor kinds:** `HUMAN only`
- **Identity requirements:**
  - Must be authenticated via OIDC (Zitadel)
  - Must have `exception:approve` permission
  - Identity MUST be bound to a durable actor_id
- **Attribution policy:**
  - `actor_kind`: HUMAN
  - `actor_id`: OIDC subject identifier
  - `timestamp`: ISO 8601 with timezone
  - `session_id`: Current session identifier
  - `approval_method`: UI portal interaction (not API bypass)

## 5) Preconditions

**Required commitment objects present:**
- Exception request document (content-addressed)
- Reference to requirement/oracle being excepted
- Evidence of the condition triggering the exception
- Risk assessment document

**Required refs present (request payload):**
- `exception_type`: deviation | deferral | waiver
- `requirement_ref`: The requirement being excepted (artifact + section + version)
- `oracle_ref` (for waivers): The oracle/gate producing FAIL
- `candidate_id` (for candidate-scoped exceptions): The candidate affected
- `loop_id` (for loop-scoped exceptions): The loop affected
- `scope`: per-candidate | per-loop | per-baseline | time-boxed
- `expiry_date`: When exception expires (required for all scopes)
- `resolution_criteria`: How exception will be resolved
- `SR-CONTRACT@{current_version}`: To verify exception eligibility

**Staleness rules:**
- Request MUST be rejected if requirement_ref is stale
- Request MUST be rejected if oracle_ref (for waivers) has changed since failure
- Exception MUST be re-evaluated if governing artifacts change

## 6) Evidence review checklist (Accountability harness)

The human MUST review:

| Evidence | Required | Purpose |
|----------|----------|---------|
| Exception request document | YES | Understand what is being excepted |
| Requirement reference | YES | Verify requirement exists and is waivable |
| Oracle failure evidence (waivers) | YES | Understand the specific failure |
| Risk assessment | YES | Understand risk of proceeding |
| Mitigation plan | YES | Understand how risk is managed |
| Resolution criteria | YES | Understand how exception ends |
| Expiry date | YES | Ensure exception is time-bounded |
| Impact on downstream | RECOMMENDED | Understand propagation effects |

## 7) Decision procedure (what to do)

**Decision options:**
- **Approve**: Exception is authorized; record created
- **Reject**: Exception is not authorized; must fix underlying issue
- **Request changes**: Non-binding; ask for narrower scope or additional mitigation
- **Escalate to**: GovernanceChangePortal (if exception implies governance gap)

**Decision rubric:**

### For Deviations
1. **Requirement exists**: Is the requirement clear and documented?
2. **Gap is real**: Does the actual state truly deviate from requirement?
3. **Fix infeasible now**: Is fixing the deviation genuinely blocked?
4. **Risk acceptable**: Is operating with deviation acceptable?
5. **Mitigation adequate**: Is the mitigation plan sufficient?
6. **Resolution path clear**: Is there a path to closing the deviation?

### For Deferrals
1. **Requirement exists**: Is the requirement clear and documented?
2. **Deferral justified**: Is there a valid reason to defer?
3. **Scope bounded**: Is deferral scoped (not indefinite)?
4. **Risk during deferral**: Is risk during deferral period acceptable?
5. **Resolution timeline**: Is there a clear timeline for completion?

### For Waivers
1. **Oracle failure real**: Did the oracle actually produce FAIL?
2. **Not integrity condition**: Is this NOT an integrity condition (non-waivable)?
3. **Failure understood**: Is the failure root cause understood?
4. **Risk acceptable**: Is risk of waiving acceptable?
5. **Scope minimal**: Is waiver scoped as narrowly as possible?
6. **Expiry defined**: Is there a clear expiry/review date?

## 8) Outputs (binding records emitted)

**Primary record types:**

### DeviationRecorded
```yaml
deviation_record:
  exception_id: "{uuid}"
  exception_type: "deviation"
  requirement_ref:
    artifact_id: "{SR-*}"
    section: "{section_ref}"
    version: "{version}"
    content_hash: "{sha256:...}"
  actual_state_description: "{what the actual state is}"
  scope: "{per-candidate | per-loop | per-baseline | time-boxed}"
  scope_refs:
    - "{candidate_id | loop_id | baseline_id}"
  expiry_date: "{iso8601}"
  resolution_criteria: "{how this ends}"
  risk_assessment_hash: "{sha256:...}"
  mitigation_plan_hash: "{sha256:...}"
  rationale: "{human-provided text}"
  actor:
    actor_kind: "HUMAN"
    actor_id: "{oidc_subject}"
    timestamp: "{iso8601}"
```

### DeferralRecorded
```yaml
deferral_record:
  exception_id: "{uuid}"
  exception_type: "deferral"
  requirement_ref:
    artifact_id: "{SR-*}"
    section: "{section_ref}"
    version: "{version}"
    content_hash: "{sha256:...}"
  deferral_reason: "{why deferring}"
  scope: "{per-candidate | per-loop | per-baseline | time-boxed}"
  scope_refs:
    - "{candidate_id | loop_id | baseline_id}"
  expiry_date: "{iso8601}"
  resolution_timeline: "{when will be completed}"
  resolution_criteria: "{how this ends}"
  risk_assessment_hash: "{sha256:...}"
  rationale: "{human-provided text}"
  actor:
    actor_kind: "HUMAN"
    actor_id: "{oidc_subject}"
    timestamp: "{iso8601}"
```

### WaiverRecorded
```yaml
waiver_record:
  exception_id: "{uuid}"
  exception_type: "waiver"
  oracle_ref:
    oracle_id: "{oracle_id}"
    oracle_suite_id: "{suite_id}"
    oracle_suite_version: "{version}"
  gate_ref: "{gate_id}"  # The gate that would block
  failure_details:
    run_id: "{uuid}"
    failure_type: "{FAIL}"
    failure_message: "{...}"
    evidence_hash: "{sha256:...}"
  scope: "{per-candidate | per-loop | per-baseline | time-boxed}"
  scope_refs:
    - candidate_id: "{uuid}"
      candidate_hash: "{sha256:...}"
  expiry_date: "{iso8601}"
  review_date: "{iso8601}"  # Required for waivers
  resolution_criteria: "{how this ends}"
  environment_constraints: []  # Optional narrowing
  risk_assessment_hash: "{sha256:...}"
  mitigation_plan_hash: "{sha256:...}"
  rationale: "{human-provided text}"
  actor:
    actor_kind: "HUMAN"
    actor_id: "{oidc_subject}"
    timestamp: "{iso8601}"
```

**Follow-on events:**
- `DeviationRecorded` → Visible in all downstream approvals; staleness trigger if requirement changes
- `DeferralRecorded` → Visible in all downstream approvals; resolution tracked
- `WaiverRecorded` → Candidate eligible for Verified-with-Exceptions; visible in Freeze Record

## 9) Failure handling and routing

| Condition | Action |
|-----------|--------|
| Integrity condition waiver attempt | REJECT immediately; non-waivable |
| Requirement not found | Reject; verify requirement reference |
| Oracle not in suite | Reject; verify oracle reference |
| Scope too broad | Request changes; narrow scope |
| No expiry date | Reject; all exceptions must expire |
| No resolution criteria | Reject; must have path to resolution |
| Risk assessment missing | Reject; must assess risk |
| Implies governance change | Escalate to GovernanceChangePortal |

## 10) Auditability

**What should be stored as evidence of the portal interaction:**
- Full request payload (content-addressed)
- Risk assessment document (content-addressed)
- Mitigation plan (content-addressed)
- Exception record (content-addressed)
- UI interaction log (submission timestamp, review duration, decision timestamp)
- Reviewer identity (actor_id, session_id)

**Retention expectation:** Indefinite (exception records are part of the permanent audit trail)

## 11) Cross-references

- **SR-CONTRACT clauses:** C-EXC-1, C-EXC-2, C-EXC-3, C-EXC-4, C-EXC-5, C-VER-2, C-VER-3
- **SR-SPEC sections:** §3.6 (Portals and Authority), §3.4.3 (Exception Records)
- **SR-ETT membranes:** Authority & Integrity, Change, Accountability
- **SR-PLAN items:** D-19 (Governance API endpoints), D-30 (Portal workflows UI)

## 12) Non-waivable integrity conditions

The following conditions MUST NEVER be waived:

| Condition | Reason | Alternative |
|-----------|--------|-------------|
| ORACLE_TAMPER | Suite integrity violated | Investigate and fix suite |
| ORACLE_GAP | Required oracle missing | Run missing oracle |
| ORACLE_ENV_MISMATCH | Environment constraint violated | Fix environment |
| ORACLE_FLAKE | Determinism violated | Fix flaky oracle |
| EVIDENCE_MISSING | Evidence not retrievable | Fix evidence storage |

If any of these are present, the portal MUST reject the waiver request and route to investigation.

## 13) Scope hierarchy and constraints

| Scope | Meaning | Constraints |
|-------|---------|-------------|
| per-candidate | Applies only to specific candidate | DEFAULT; most conservative |
| per-loop | Applies to all candidates in loop | Requires additional justification |
| per-baseline | Applies to all work on baseline | Requires escalation review |
| time-boxed | Applies until date regardless of scope | MUST have expiry; MUST have review date |

**Scope escalation:**
- per-candidate → per-loop: Requires additional risk review
- per-loop → per-baseline: Requires PhaseGatePortal review
- time-boxed > 30 days: Requires periodic review (minimum monthly)

## 14) Exception lifecycle

```
Exception requested
    │
    ├─► ExceptionApprovalPortal review
    │       │
    │       ├─► Approved: ExceptionRecorded (active)
    │       │       │
    │       │       ├─► Visible in all approvals/freezes in scope
    │       │       │
    │       │       ├─► Resolution criteria met?
    │       │       │       │
    │       │       │       YES ──► ExceptionResolved (closed)
    │       │       │       │
    │       │       │       NO ──► Continue monitoring
    │       │       │
    │       │       └─► Expiry reached?
    │       │               │
    │       │               YES ──► ExceptionExpired; blocks further progress
    │       │               │
    │       │               NO ──► Continue
    │       │
    │       └─► Rejected: Must fix underlying issue
    │
    └─► Governing artifact changes?
            │
            YES ──► Exception marked stale; requires re-review
```

---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "PLAYBOOK-ReleaseApprovalPortal"
  type: "config.portal_playbook"
  title: "ReleaseApprovalPortal Playbook"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "normative"
  authority_kind: "config"
  governed_by: ["SR-CHANGE"]
  created: "2026-01-11"
  updated: "2026-01-11"
  tags: ["sr-directive", "portal", "playbook", "release-approval"]
  refs:
    - kind: "GovernedArtifact"
      id: "SR-CONTRACT"
      rel: "depends_on"
    - kind: "GovernedArtifact"
      id: "SR-SPEC"
      rel: "depends_on"
---

# ReleaseApprovalPortal — Playbook

Human-only binding boundary for release/freeze: approve a specific candidate state for baseline/shippable progression.

## 1) Portal identification

- **portal_id:** `ReleaseApprovalPortal`
- **portal_kind:** `release_approval`
- **scope:** `work_unit` (applies per-candidate for release decisions)

## 2) Purpose and boundary

- **Purpose (one sentence):**
  Authorize a verified candidate to become Shippable by creating a Freeze Record, after human review of verification evidence and acknowledgment of any active exceptions.

- **Trust boundary being crossed (SR-ETT membranes):**
  - Authority & Integrity (admissibility control)
  - Accountability (proof/completeness control)
  - Operational (process/state-machine control)

- **What this portal MUST NOT do:**
  - Mint verification evidence (that's oracle's job)
  - Override integrity conditions (non-waivable)
  - Approve candidates without Verified status
  - Approve candidates with unacknowledged exceptions
  - Bypass staleness checks
  - Delegate approval authority to agents or SYSTEM

## 3) Allowed request types

- [x] approval_request (release approval for verified candidate)
- [x] freeze_request (create Freeze Record for approved candidate)
- [x] decision_request (stop-the-line arbitration for release blockers)
- [ ] governance_change_request (use GovernanceChangePortal)
- [ ] exception_request (use ExceptionApprovalPortal)
- [ ] budget_extension_request (use BudgetExtensionPortal)
- [ ] oracle_suite_change_request (use OracleSuiteChangePortal)

## 4) Actor rules

- **Allowed actor kinds:** `HUMAN only`
- **Identity requirements:**
  - Must be authenticated via OIDC (Zitadel)
  - Must have `release:approve` permission
  - Identity MUST be bound to a durable actor_id
- **Attribution policy:**
  - `actor_kind`: HUMAN
  - `actor_id`: OIDC subject identifier
  - `timestamp`: ISO 8601 with timezone
  - `session_id`: Current session identifier
  - `approval_method`: UI portal interaction (not API bypass)

## 5) Preconditions

**Required commitment objects present:**
- Candidate (with content-addressed identifier)
- EvidenceBundleRecorded (linked to candidate)
- VerificationCompleted (status: Verified or Verified-with-Exceptions)
- Active exception records (if Verified-with-Exceptions)
- OracleRunCompleted events for all required oracles in the suite

**Required refs present (request payload):**
- `candidate_id`: The candidate being approved
- `candidate_hash`: Content hash of candidate state
- `evidence_bundle_id`: Evidence bundle for verification
- `evidence_manifest_hash`: Content hash of evidence manifest
- `oracle_suite_id`: The suite used for verification
- `oracle_suite_hash`: Content hash of suite definition (for tamper detection)
- `verification_mode`: STRICT or WITH_EXCEPTIONS
- `active_exceptions[]`: List of active exception records (if any)
- `SR-DIRECTIVE@{current_version}`: Governing directive

**Staleness rules:**
- Request MUST be rejected if candidate is stale on any `depends_on` refs
- Request MUST be rejected if evidence bundle is stale relative to candidate
- Request MUST be rejected if oracle suite has changed since verification
- Request MUST include staleness check timestamp

## 6) Evidence review checklist (Accountability harness)

The human MUST review:

| Evidence | Required | Purpose |
|----------|----------|---------|
| Evidence bundle manifest | YES | Understand what was verified |
| Oracle run results (all required) | YES | Verify all checks passed |
| Oracle run transcript(s) | YES | Understand any warnings or notes |
| Determinism/environment fingerprint | YES | Verify oracle environment compliance |
| Candidate diff/patchset | YES | Understand what changed |
| Active exceptions (if any) | YES | Acknowledge and accept risk |
| Waiver justifications (if any) | YES | Understand why waivers were granted |
| Risk assessment | RECOMMENDED | Understand risk profile |
| Integration test results | RECOMMENDED | Verify integration works |

## 7) Decision procedure (what to do)

**Decision options:**
- **Approve**: Candidate is authorized for release; proceed to Freeze Record creation
- **Reject**: Candidate is not authorized; must fix issues or request exception
- **Request changes**: Non-binding; ask for clarification or modification before re-submission
- **Escalate to**: PhaseGatePortal (if phase-level concerns) or GovernanceChangePortal (if governance issues discovered)

**Decision rubric:**

1. **Verification complete**: Are all required oracles PASS (or covered by approved waivers)?
2. **Evidence integrity**: Is evidence bundle complete and content-addressed correctly?
3. **No integrity conditions**: Are there any ORACLE_TAMPER, ORACLE_GAP, ORACLE_ENV_MISMATCH, ORACLE_FLAKE, or EVIDENCE_MISSING?
4. **Exceptions acknowledged**: Are all active exceptions explicitly acknowledged?
5. **Staleness clear**: Is the candidate current relative to all dependencies?
6. **Scope appropriate**: Does the candidate implement what it claims?
7. **Risk acceptable**: Is the risk profile acceptable for release?

## 8) Outputs (binding records emitted)

**Primary record type:** `ReleaseApprovalRecorded`

**Required fields:**

```yaml
release_approval:
  approval_id: "{uuid}"
  approval_type: "release"
  candidate_id: "{uuid}"
  candidate_hash: "{sha256:...}"
  evidence_bundle_id: "{uuid}"
  evidence_manifest_hash: "{sha256:...}"
  oracle_suite_id: "{suite_id}"
  oracle_suite_hash: "{sha256:...}"
  verification_mode: "{STRICT | WITH_EXCEPTIONS}"
  acknowledged_exceptions:
    - exception_id: "{uuid}"
      exception_type: "{deviation | deferral | waiver}"
      scope: "{...}"
  rationale: "{human-provided text}"
  actor:
    actor_kind: "HUMAN"
    actor_id: "{oidc_subject}"
    timestamp: "{iso8601}"
```

**Follow-on records (if approval granted):**

```yaml
freeze_record:
  freeze_id: "{uuid}"
  baseline_id: "{baseline_id}"
  candidate_id: "{uuid}"
  candidate_hash: "{sha256:...}"
  verification:
    mode: "{STRICT | WITH_EXCEPTIONS}"
    suite_id: "{suite_id}"
    suite_hash: "{sha256:...}"
    evidence_bundle_id: "{uuid}"
    evidence_manifest_hash: "{sha256:...}"
    waiver_ids: []  # If WITH_EXCEPTIONS
  release_approval_id: "{approval_id}"
  artifact_manifest:
    - artifact_id: "{...}"
      version: "{...}"
      content_hash: "{sha256:...}"
  active_exceptions: []
  frozen_by:
    actor_kind: "HUMAN"
    actor_id: "{oidc_subject}"
  frozen_at: "{iso8601}"
```

**Follow-on events:**
- `ReleaseApprovalRecorded` → Unlocks FreezeRecordCreated
- `FreezeRecordCreated` → Candidate becomes Shippable
- `ReleaseRejected` → Loop continues or pauses for remediation

## 9) Failure handling and routing

| Condition | Action |
|-----------|--------|
| Candidate not Verified | Reject; candidate must complete verification |
| Evidence bundle missing | Reject; evidence must be recorded |
| Evidence bundle stale | Reject; must re-verify with current candidate |
| Oracle suite tampered | HALT; ORACLE_TAMPER stop-the-line; route to OracleSuiteChangePortal |
| Integrity condition present | HALT; cannot approve; route to investigation |
| Unacknowledged exceptions | Reject; must acknowledge exceptions or remove them |
| Staleness on depends_on | Reject; must resolve staleness |
| Approval authority unclear | Escalate to governance owner |

## 10) Auditability

**What should be stored as evidence of the portal interaction:**
- Full request payload (content-addressed)
- All evidence bundles reviewed (content-addressed)
- Approval record (content-addressed)
- Freeze record (content-addressed)
- UI interaction log (submission timestamp, review duration, decision timestamp)
- Reviewer identity (actor_id, session_id)
- Evidence items explicitly reviewed (for audit trail)

**Retention expectation:** Indefinite (release decisions are part of the permanent audit trail)

## 11) Cross-references

- **SR-CONTRACT clauses:** C-TB-1, C-TB-2, C-TB-4, C-VER-1, C-VER-2, C-VER-3, C-SHIP-1, C-SHIP-2, C-EVID-1, C-EVID-5
- **SR-SPEC sections:** §3.6 (Portals and Authority), §3.4 (Evidence), §3.2.3 (Freeze Records)
- **SR-ETT membranes:** Authority & Integrity, Accountability, Operational
- **SR-PLAN items:** D-19 (Governance API endpoints), D-30 (Portal workflows UI)

## 12) Verification mode guidance

### STRICT mode
- All required oracles MUST be PASS
- No waivers present
- Evidence bundle complete with all required artifacts
- Candidate determinism verified

### WITH_EXCEPTIONS mode
- Required oracles executed; some may be FAIL
- Each FAIL MUST be covered by an approved Gate Waiver
- Waivers MUST NOT bypass integrity conditions
- All waivers MUST have explicit scope and expiry
- Human MUST explicitly acknowledge waiver risks

## 13) Security-critical release

For deliverables tagged as security-critical (D-16, D-32, or any with SECURITY_CRITICAL trigger):

- Require additional security review evidence
- Require explicit security risk acknowledgment
- Consider requiring multi-person approval
- Document security-specific rationale

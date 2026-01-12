---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "PLAYBOOK-PhaseGatePortal"
  type: "config.portal_playbook"
  title: "PhaseGatePortal Playbook"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "normative"
  authority_kind: "config"
  governed_by: ["SR-CHANGE"]
  created: "2026-01-11"
  updated: "2026-01-11"
  tags: ["sr-directive", "portal", "playbook", "phase-gate"]
  refs:
    - kind: "GovernedArtifact"
      id: "SR-CONTRACT"
      rel: "depends_on"
    - kind: "GovernedArtifact"
      id: "SR-SPEC"
      rel: "depends_on"
    - kind: "GovernedArtifact"
      id: "SR-PLAN"
      rel: "depends_on"
---

# PhaseGatePortal — Playbook

Human-only boundary for phase completion approval and cross-phase progression.

## 1) Portal identification

- **portal_id:** `PhaseGatePortal`
- **portal_kind:** `phase_gate`
- **scope:** `phase` (applies per-phase for phase completion decisions)

## 2) Purpose and boundary

- **Purpose (one sentence):**
  Authorize phase completion after verifying all phase deliverables are verified, reviewing aggregate evidence, and unlocking the next phase.

- **Trust boundary being crossed (SR-ETT membranes):**
  - Operational (process/state-machine control)
  - Accountability (proof/completeness control)
  - Architectural (structure/boundaries control) — for architecture-sensitive phases

- **What this portal MUST NOT do:**
  - Approve phase completion with unverified deliverables
  - Skip deliverables without formal deferral/exception
  - Approve if stop-triggers are unresolved
  - Delegate phase gate authority to agents or SYSTEM

## 3) Allowed request types

- [x] approval_request (phase gate approval)
- [x] decision_request (arbitration on phase completion blockers)
- [ ] governance_change_request (use GovernanceChangePortal)
- [ ] exception_request (use ExceptionApprovalPortal first)
- [ ] budget_extension_request (use BudgetExtensionPortal)
- [ ] oracle_suite_change_request (use OracleSuiteChangePortal)
- [ ] freeze_request (use ReleaseApprovalPortal for individual releases)

## 4) Actor rules

- **Allowed actor kinds:** `HUMAN only`
- **Identity requirements:**
  - Must be authenticated via OIDC (Zitadel)
  - Must have `phase:approve` permission
  - For architecture-sensitive phases (P1), must have `architecture:review` permission
  - Identity MUST be bound to a durable actor_id
- **Attribution policy:**
  - `actor_kind`: HUMAN
  - `actor_id`: OIDC subject identifier
  - `timestamp`: ISO 8601 with timezone
  - `session_id`: Current session identifier

## 5) Preconditions

**Required commitment objects present:**
- All phase deliverables in Verified (or Verified-with-Exceptions) status
- Evidence bundles for all phase deliverables
- ReleaseApprovalRecorded for each deliverable requiring release
- Active exception records acknowledged
- No unresolved stop-triggers for phase deliverables

**Required refs present (request payload):**
- `phase_id`: The phase being completed (e.g., P1-DOMAIN-CORE)
- `deliverable_ids[]`: List of all deliverables in phase
- `candidate_ids[]`: Candidates for each deliverable
- `evidence_bundle_ids[]`: Evidence bundles for each deliverable
- `active_exceptions[]`: Any active exceptions in scope
- `SR-PLAN@{current_version}`: To verify deliverable inventory
- `SR-DIRECTIVE@{current_version}`: To verify phase requirements

**Staleness rules:**
- Request MUST be rejected if any deliverable is stale
- Request MUST be rejected if phase definition has changed
- Phase gate MUST be re-evaluated if upstream phases change

## 6) Evidence review checklist (Accountability harness)

The human MUST review:

| Evidence | Required | Purpose |
|----------|----------|---------|
| Phase deliverable list | YES | Verify all deliverables included |
| Verification status for each | YES | Verify all are Verified |
| Evidence bundles (summary) | YES | Understand verification coverage |
| Active exceptions | YES | Acknowledge phase-level exceptions |
| Architecture review (P1) | PHASE-SPECIFIC | Verify hex boundary compliance |
| Integration test summary (P2+) | PHASE-SPECIFIC | Verify integration works |
| Stop-trigger resolution | YES | Verify no blockers |

## 7) Decision procedure (what to do)

**Decision options:**
- **Approve**: Phase is complete; next phase unlocked
- **Reject**: Phase is not complete; must resolve issues
- **Conditional approval**: Phase approved with noted conditions for next phase
- **Escalate to**: GovernanceChangePortal (if phase requirements need change)

**Decision rubric:**

1. **Deliverable coverage**: Are all phase deliverables present?
2. **Verification complete**: Are all deliverables Verified (or with approved exceptions)?
3. **Architecture compliance** (P1): Does domain core maintain hex purity?
4. **Integration verified** (P2+): Do integrations work correctly?
5. **Exceptions acceptable**: Are active exceptions acceptable for phase progression?
6. **No blockers**: Are all stop-triggers resolved?
7. **Next phase ready**: Are dependencies for next phase satisfied?

## 8) Outputs (binding records emitted)

**Primary record type:** `PhaseGateApprovalRecorded`

```yaml
phase_gate_approval:
  approval_id: "{uuid}"
  approval_type: "phase_gate"
  phase_id: "{P0-FOUNDATION | P1-DOMAIN-CORE | ...}"
  deliverables_verified:
    - deliverable_id: "{D-...}"
      candidate_id: "{uuid}"
      candidate_hash: "{sha256:...}"
      verification_mode: "{STRICT | WITH_EXCEPTIONS}"
  active_exceptions:
    - exception_id: "{uuid}"
      exception_type: "{deviation | deferral | waiver}"
  conditions: []  # Optional conditions for next phase
  architecture_review_notes: "{...}"  # For P1
  rationale: "{human-provided text}"
  actor:
    actor_kind: "HUMAN"
    actor_id: "{oidc_subject}"
    timestamp: "{iso8601}"
```

**Follow-on events:**
- `PhaseGateApprovalRecorded` → Next phase deliverables unlocked (G-PHASE-N-COMPLETE satisfied)
- `PhaseGateRejected` → Phase remains incomplete; issues must be resolved

## 9) Failure handling and routing

| Condition | Action |
|-----------|--------|
| Deliverable not Verified | Reject; must complete verification |
| Evidence missing | Reject; must record evidence |
| Stop-trigger unresolved | Reject; must resolve trigger |
| Exception not approved | Route to ExceptionApprovalPortal first |
| Architecture concern (P1) | Reject with architecture review notes |
| Phase definition unclear | Escalate to GovernanceChangePortal |

## 10) Auditability

**What should be stored as evidence of the portal interaction:**
- Full request payload (content-addressed)
- Deliverable verification summary (content-addressed)
- Exception acknowledgments (content-addressed)
- Phase gate approval record (content-addressed)
- Architecture review notes (if applicable)
- UI interaction log

**Retention expectation:** Indefinite (phase gates are part of the permanent audit trail)

## 11) Cross-references

- **SR-CONTRACT clauses:** C-SHIP-1, C-LOOP-1
- **SR-SPEC sections:** §3.2 (Loop Mechanics), §3.6 (Portals)
- **SR-ETT membranes:** Operational, Accountability, Architectural
- **SR-PLAN items:** All deliverables

## 12) Phase-specific requirements

| Phase | Additional Requirements |
|-------|------------------------|
| P0-FOUNDATION | CI must be green; governance hygiene complete |
| P1-DOMAIN-CORE | Architecture review required; hex boundary audit |
| P2-PERSISTENCE | Replay determinism proof required |
| P3-API | API contract verification required |
| P4-ORCHESTRATION | Loop governor tests; SYSTEM-only enforcement |
| P4-ORACLES | Oracle determinism proof; integrity detection |
| P5-UI | Portal workflow verification |
| P5-OPS | Self-host boot verification |
| P6-E2E | Full replayability demonstration |

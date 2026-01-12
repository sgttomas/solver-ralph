---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "PLAYBOOK-OracleSuiteChangePortal"
  type: "config.portal_playbook"
  title: "OracleSuiteChangePortal Playbook"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "normative"
  authority_kind: "config"
  governed_by: ["SR-CHANGE"]
  created: "2026-01-11"
  updated: "2026-01-11"
  tags: ["sr-directive", "portal", "playbook", "oracle-suite-change"]
  refs:
    - kind: "GovernedArtifact"
      id: "SR-CONTRACT"
      rel: "depends_on"
    - kind: "GovernedArtifact"
      id: "SR-SPEC"
      rel: "depends_on"
---

# OracleSuiteChangePortal — Playbook

Human-only boundary for oracle suite modifications, especially weakening or integrity issue resolution.

## 1) Portal identification

- **portal_id:** `OracleSuiteChangePortal`
- **portal_kind:** `oracle_suite_change`
- **scope:** `global` (oracle suite changes affect all loops using that suite)

## 2) Purpose and boundary

- **Purpose (one sentence):**
  Authorize modifications to oracle suite definitions, especially reclassifying oracles (required→advisory), resolving integrity conditions (ORACLE_TAMPER, ORACLE_GAP, ORACLE_FLAKE, ORACLE_ENV_MISMATCH), or adding/removing oracles.

- **Trust boundary being crossed (SR-ETT membranes):**
  - Authority & Integrity (admissibility control)
  - Accountability (proof/completeness control)
  - Change (evolution/versioning control)

- **What this portal MUST NOT do:**
  - Silently weaken verification coverage
  - Approve suite changes without impact analysis
  - Remove required oracles without governance review
  - Approve flaky oracles as deterministic
  - Delegate oracle suite authority to agents

## 3) Allowed request types

- [x] oracle_suite_change_request
- [x] decision_request (arbitration on integrity condition resolution)
- [ ] approval_request (use ReleaseApprovalPortal)
- [ ] governance_change_request (escalate to GovernanceChangePortal for policy changes)
- [ ] exception_request (use ExceptionApprovalPortal)
- [ ] budget_extension_request (use BudgetExtensionPortal)

## 4) Actor rules

- **Allowed actor kinds:** `HUMAN only`
- **Identity requirements:**
  - Must be authenticated via OIDC (Zitadel)
  - Must have `oracle:modify` permission
  - Identity MUST be bound to a durable actor_id
- **Attribution policy:**
  - `actor_kind`: HUMAN
  - `actor_id`: OIDC subject identifier
  - `timestamp`: ISO 8601 with timezone

## 5) Preconditions

**Required commitment objects present:**
- Current oracle suite definition (content-addressed)
- Proposed oracle suite definition (content-addressed)
- Diff between current and proposed
- Impact analysis (which loops/deliverables affected)
- Integrity condition evidence (if resolving ORACLE_* condition)

**Required refs present (request payload):**
- `suite_id`: The suite being modified
- `current_version`: Current suite version
- `current_hash`: Content hash of current suite
- `proposed_version`: New suite version
- `proposed_hash`: Content hash of proposed suite
- `change_type`: add_oracle | remove_oracle | reclassify | env_change | fix_flake
- `affected_loops[]`: Loops currently using this suite
- `affected_deliverables[]`: Deliverables using this suite
- `SR-DIRECTIVE@{current_version}`: To verify profile bindings

**Staleness rules:**
- Request must reflect current suite state
- All affected loops should be notified of suite change

## 6) Evidence review checklist (Accountability harness)

The human MUST review:

| Evidence | Required | Purpose |
|----------|----------|---------|
| Suite diff | YES | Understand exact changes |
| Impact analysis | YES | Understand scope of change |
| Integrity condition evidence (if applicable) | YES | Understand what triggered issue |
| Flake analysis (if ORACLE_FLAKE) | YES | Understand non-determinism source |
| Coverage impact | YES | Ensure verification not weakened |
| Affected deliverables | YES | Understand downstream impact |

## 7) Decision procedure (what to do)

**Decision options:**
- **Approve**: Suite change authorized; new version becomes active
- **Reject**: Suite change not authorized; must revise
- **Escalate to**: GovernanceChangePortal (if change affects binding semantics)

**Decision rubric:**

### For adding oracles
1. **Oracle well-defined**: Is oracle deterministic and correctly specified?
2. **Environment specified**: Are environment constraints complete?
3. **Classification correct**: Is required/advisory classification appropriate?
4. **Coverage improves**: Does addition improve verification coverage?

### For removing oracles
1. **Removal justified**: Why is oracle being removed?
2. **Coverage maintained**: Is equivalent coverage maintained elsewhere?
3. **No regression**: Does removal not create verification gaps?
4. **Affected parties notified**: Are users of affected deliverables aware?

### For reclassifying (required → advisory)
1. **Reclassification justified**: Why is weakening acceptable?
2. **Risk assessed**: What is risk of not requiring this check?
3. **Alternative exists**: Is there alternative coverage?
4. **Not integrity-critical**: Is this not an integrity-critical oracle?

### For resolving integrity conditions
1. **Root cause identified**: Is the cause of ORACLE_TAMPER/GAP/FLAKE/ENV_MISMATCH understood?
2. **Fix verified**: Has the fix been tested?
3. **No recurrence**: Are measures in place to prevent recurrence?
4. **Suite integrity restored**: Is suite integrity verifiable after fix?

## 8) Outputs (binding records emitted)

**Primary record type:** `OracleSuiteChangeApproved`

```yaml
oracle_suite_change:
  change_id: "{uuid}"
  change_type: "{add_oracle | remove_oracle | reclassify | env_change | fix_flake}"
  suite_id: "{suite_id}"
  previous_version: "{version}"
  previous_hash: "{sha256:...}"
  new_version: "{version}"
  new_hash: "{sha256:...}"
  changes:
    - change_kind: "{oracle_added | oracle_removed | classification_changed | env_changed}"
      oracle_id: "{oracle_id}"
      old_value: "{...}"
      new_value: "{...}"
  integrity_condition_resolved: "{ORACLE_TAMPER | ORACLE_GAP | ... | null}"
  affected_loops:
    - loop_id: "{uuid}"
      action: "{will_use_new_suite | requires_re-verification}"
  affected_deliverables:
    - deliverable_id: "{D-...}"
  impact_assessment_hash: "{sha256:...}"
  rationale: "{human-provided text}"
  actor:
    actor_kind: "HUMAN"
    actor_id: "{oidc_subject}"
    timestamp: "{iso8601}"
```

**Follow-on events:**
- `OracleSuiteVersionCreated` → New suite version available
- `OracleSuiteActivated` → New version becomes default for profile
- `LoopSuiteUpdated` → Active loops using suite are notified
- `StalenessPropagated` → Candidates verified with old suite marked stale

## 9) Failure handling and routing

| Condition | Action |
|-----------|--------|
| Suite not found | Reject; verify suite reference |
| Integrity condition not resolved | Cannot approve; must fix underlying issue |
| Weakening without justification | Reject; require risk assessment |
| Binding semantic change | Escalate to GovernanceChangePortal |
| Multi-deliverable impact | Require PhaseGatePortal review |

## 10) Auditability

**What should be stored as evidence of the portal interaction:**
- Full request payload (content-addressed)
- Suite diff (content-addressed)
- Impact analysis (content-addressed)
- Decision record (content-addressed)
- UI interaction log

**Retention expectation:** Indefinite (oracle suite changes are part of the audit trail)

## 11) Cross-references

- **SR-CONTRACT clauses:** C-OR-1, C-OR-2, C-OR-3, C-OR-4, C-OR-5, C-OR-6, C-OR-7
- **SR-SPEC sections:** §3.5 (Oracles), §3.5.3 (Oracle Integrity)
- **SR-ETT membranes:** Authority & Integrity, Accountability, Change
- **SR-PLAN items:** D-24, D-25, D-26, D-27 (Oracle deliverables)

## 12) Integrity condition resolution workflows

### ORACLE_TAMPER
```
ORACLE_TAMPER detected
    │
    ├─► Investigate: How was suite hash changed during run?
    │
    ├─► Fix: Ensure suite is immutable during run
    │
    ├─► Verify: Run determinism check on fixed suite
    │
    └─► Request approval with evidence of fix
```

### ORACLE_GAP
```
ORACLE_GAP detected
    │
    ├─► Identify: Which required oracle is missing?
    │
    ├─► Options:
    │   ├─► Add missing oracle to suite
    │   └─► Reclassify oracle as advisory (requires justification)
    │
    └─► Request approval with coverage analysis
```

### ORACLE_FLAKE
```
ORACLE_FLAKE detected
    │
    ├─► Investigate: What causes non-determinism?
    │
    ├─► Options:
    │   ├─► Fix oracle to be deterministic
    │   ├─► Add environment constraints to ensure determinism
    │   └─► Reclassify as advisory (requires justification)
    │
    └─► Request approval with determinism proof
```

### ORACLE_ENV_MISMATCH
```
ORACLE_ENV_MISMATCH detected
    │
    ├─► Investigate: Which environment constraint was violated?
    │
    ├─► Options:
    │   ├─► Fix runner environment
    │   ├─► Update environment constraints to match reality
    │   └─► Investigate if constraint is necessary
    │
    └─► Request approval with environment verification
```

## 13) Escalation to GovernanceChangePortal

The following changes require escalation:
- Removing a required oracle without replacement
- Changing oracle classification for integrity-critical oracles
- Modifying environment constraints that affect security
- Any change that affects binding semantics of verification

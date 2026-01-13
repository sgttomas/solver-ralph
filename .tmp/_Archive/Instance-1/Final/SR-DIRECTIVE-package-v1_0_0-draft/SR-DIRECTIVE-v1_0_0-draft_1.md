---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-DIRECTIVE"
  type: "governance.dev_directive"
  title: "Development Directive — Instance-1"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "normative"
  authority_kind: "process"
  governed_by: ["SR-CHANGE"]
  created: "2026-01-11"
  updated: "2026-01-11"
  tags: ["directive", "execution", "governance", "instance-1"]
  refs:
    # REQUIRED (depends_on)
    - kind: "GovernedArtifact"
      id: "SR-TYPES@3.3.0"
      rel: "depends_on"
      meta: { content_hash: "sha256:TBD" }
    - kind: "GovernedArtifact"
      id: "SR-CONTRACT@1.1.0"
      rel: "depends_on"
      meta: { content_hash: "sha256:TBD" }
    - kind: "GovernedArtifact"
      id: "SR-SPEC@1.3.0"
      rel: "depends_on"
      meta: { content_hash: "sha256:TBD" }
    - kind: "GovernedArtifact"
      id: "SR-PLAN-INSTANCE-1@1.0.0"
      rel: "depends_on"
      meta: { content_hash: "sha256:TBD" }
    # SUPPORTING (supported_by)
    - kind: "GovernedArtifact"
      id: "SR-ETT@1.1.0"
      rel: "supported_by"
      meta: { content_hash: "sha256:TBD" }
    - kind: "ProcessState"
      id: "PS-SOLVER-RALPH-INSTANCE-1-SAPS"
      rel: "supported_by"
      meta: { content_hash: "sha256:TBD" }
---

# SR-DIRECTIVE — Development Directive (Instance-1)

> **Summary:** This document defines the operational policy for executing SR-PLAN instance-1 deliverables using SR-SPEC mechanics while satisfying SR-CONTRACT invariants. SR-ETT provides the constraint-placement lens for membrane enforcement.

---

## 0. Change Log

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0.0-draft.1 | 2026-01-11 | Claude | Initial draft |

---

## 1. Scope and Authority

### 1.1 Purpose

SR-DIRECTIVE specifies **how** the SR-PLAN instance-1 deliverables are executed to resolution:

- **Sequencing**: Which deliverables execute in which phases, respecting dependency edges
- **Gates**: What conditions must be satisfied to start, accept, and release work
- **Budgets**: Iteration, time, and cost limits per loop
- **Verification profiles**: Which oracle suites apply to which deliverables
- **Human judgment hooks**: When and how humans review and approve
- **Exception handling**: Workflows for deviations, deferrals, and waivers
- **Portal routing**: Which portal handles which decision type

### 1.2 Non-goals

SR-DIRECTIVE does **NOT**:

- Redefine binding semantics (Verified, Approved, Shippable — those are in SR-CONTRACT/SR-SPEC)
- Invent new portal kinds (portal semantics are defined in SR-CONTRACT/SR-SPEC)
- Modify the SR-PLAN deliverable inventory (changes require SR-CHANGE)
- Create new governance types (those are in SR-TYPES)

### 1.3 Precedence and Conflict Resolution

| Situation | Resolution |
|-----------|------------|
| SR-DIRECTIVE conflicts with SR-CONTRACT | SR-CONTRACT controls |
| SR-DIRECTIVE conflicts with SR-SPEC | SR-SPEC controls |
| SR-DIRECTIVE needs new binding semantic | Route through SR-CHANGE |
| SR-DIRECTIVE needs new portal kind | Route through SR-CHANGE |
| SR-PLAN deliverable change needed | Route through SR-CHANGE |

---

## 2. Execution Model

### 2.1 Canonical Loop Workflow

Each deliverable is produced through a governed loop following this skeleton:

```
LoopCreated (for deliverable)
    │
    ├─► IterationStarted (SYSTEM-only)
    │       refs[]: governed artifacts, oracle suite, loop, base candidate, exceptions
    │       │
    │       ├─► G-00: Context admissible? (no ghost inputs)
    │       │       │
    │       │       FAIL ──► StopTriggered (EVIDENCE_MISSING or ORACLE_GAP)
    │       │       │
    │       │       PASS
    │       │         │
    │       │         ▼
    │       ├─► G-DEP-READY: Dependencies satisfied?
    │       │       │
    │       │       FAIL ──► Wait (loop paused until deps ready)
    │       │       │
    │       │       PASS
    │       │         │
    │       │         ▼
    │       ├─► Agent produces CandidateRegistered
    │       │       │
    │       │       ▼
    │       ├─► OracleRunRequested → OracleRunCompleted (per oracle)
    │       │       │
    │       │       ▼
    │       ├─► EvidenceBundleRecorded
    │       │       │
    │       │       ▼
    │       ├─► Accept gates evaluated (G-BUILD-PASS, G-UNIT-PASS, etc.)
    │       │       │
    │       │       ALL PASS ──► VerificationCompleted (Verified or Verified-with-Exceptions)
    │       │       │
    │       │       ANY FAIL ──► IterationCompleted (continue or stop-trigger)
    │       │
    │       └─► Budget check
    │               │
    │               EXHAUSTED ──► StopTriggered (BUDGET_EXHAUSTED) → BudgetExtensionPortal
    │               │
    │               OK ──► Continue to next iteration or release
    │
    ├─► Release gates evaluated (G-*-RELEASE)
    │       │
    │       PASS + Human approval ──► ReleaseApprovalRecorded → FreezeRecordCreated
    │       │
    │       └─► Candidate becomes Shippable
    │
    └─► LoopCompleted
```

### 2.2 Concurrency Policy

| Policy | Value | Rationale |
|--------|-------|-----------|
| Parallel loops (global) | 3 | Resource constraint; evidence attribution |
| Within-phase concurrency | Allowed for independent deliverables | Respect dependency DAG |
| Cross-phase concurrency | Not allowed | Phase gates enforce sequencing |
| Evidence attribution | Independent per loop | Each loop produces isolated evidence |

### 2.3 Dependency-First Scheduling

SR-PLAN deliverables form a DAG via `depends_on` edges. SR-DIRECTIVE enforces:

1. **Topological ordering**: A deliverable's loop cannot start until all `depends_on` deliverables are Verified
2. **Phase grouping**: Deliverables are grouped into phases; phase N+1 cannot start until phase N completes
3. **Gate enforcement**: G-DEP-READY gate blocks iteration start if dependencies not satisfied

---

## 3. Inputs and Refs Discipline

### 3.1 Required IterationStarted Refs

Every `IterationStarted` event MUST include these refs (per C-CTX-1, C-CTX-2):

| Ref Category | Refs | Relationship | Required |
|--------------|------|--------------|----------|
| Governing artifacts | SR-TYPES, SR-CONTRACT, SR-SPEC, SR-DIRECTIVE | depends_on | YES |
| Oracle suite | Active suite for deliverable's profile | depends_on | YES |
| Loop | Current loop | in_scope_of | YES |
| Prior iteration summaries | Controlled memory | depends_on | YES (if iteration > 1) |
| Base candidate | For incremental work | depends_on | Optional |
| Active exceptions | Deviation/Deferral/Waiver records | depends_on | If any active |
| Human intervention notes | record.intervention_note | depends_on | If any |
| Agent definition | Agent spec | supported_by | Audit only |
| Gating policy | Current gates | supported_by | Audit only |

### 3.2 Depends_on vs Supported_by Policy

| Relationship | Semantics | Staleness Propagation | Blocking |
|--------------|-----------|----------------------|----------|
| `depends_on` | Semantic dependency | YES — changes propagate staleness | YES |
| `supported_by` | Audit provenance | NO — non-blocking by default | NO |

---

## 4. Budgets and Stop Triggers

### 4.1 Budget Policy

#### Default Budgets (per loop)

| Budget Type | Default | Extension Authority |
|-------------|---------|---------------------|
| Iterations | 5 | BudgetExtensionPortal |
| Time | 4 hours wall-clock | BudgetExtensionPortal |
| Cost | $50 compute equivalent | BudgetExtensionPortal |

#### Extended Budgets (by deliverable class)

| Deliverable Class | Iterations | Time | Cost | Rationale |
|-------------------|------------|------|------|-----------|
| code.domain | 5 | 4h | $50 | Standard |
| code.persistence (STRICT-FULL) | 7 | 6h | $75 | Integration complexity |
| code.api (STRICT-FULL) | 7 | 6h | $75 | API contract testing |
| code.orchestration | 7 | 6h | $75 | Loop governor complexity |
| code.oracle | 7 | 6h | $75 | Integrity-critical |
| test.e2e | 10 | 8h | $100 | Full system complexity |
| ops.deploy | 7 | 6h | $75 | Deployment testing |

#### Budget Extension Rules

| Extension | Limit | Escalation |
|-----------|-------|------------|
| First | Up to 100% of original | BudgetExtensionPortal |
| Second | Up to 50% of original | PhaseGatePortal review |
| Third+ | Up to 25% of original | GovernanceChangePortal |
| Total > 3x original | Requires governance review | GovernanceChangePortal |

### 4.2 Stop-the-Line Trigger Registry

#### Mandatory Triggers (per C-LOOP-3)

| Trigger | Condition | Waivable | N Value | Resolution Portal |
|---------|-----------|----------|---------|-------------------|
| ORACLE_TAMPER | Suite hash mismatch during run | NO | N/A | OracleSuiteChangePortal |
| ORACLE_GAP | Required oracle result missing | NO | N/A | OracleSuiteChangePortal |
| ORACLE_ENV_MISMATCH | Environment constraint violated | NO | N/A | OracleSuiteChangePortal |
| ORACLE_FLAKE | Non-deterministic required oracle | NO | N/A | OracleSuiteChangePortal |
| EVIDENCE_MISSING | Referenced evidence not retrievable | NO | N/A | GovernanceChangePortal |
| REPEATED_FAILURE | N consecutive iterations without progress | YES (via Decision) | 3 | PhaseGatePortal |
| BUDGET_EXHAUSTED | Any budget limit reached | YES (via extension) | N/A | BudgetExtensionPortal |

#### Instance-Specific Triggers

| Trigger | Condition | Waivable | Resolution Portal |
|---------|-----------|----------|-------------------|
| GOVERNANCE_TOUCH | Deliverable modifies governed artifacts | YES | GovernanceChangePortal |
| SECURITY_CRITICAL | Oracle flags security issue (severity ≥ HIGH) | NO | ReleaseApprovalPortal |
| DEPENDENCY_CYCLE | Circular dependency detected | NO | GovernanceChangePortal |

---

## 5. Verification Profiles and Oracle Suites

### 5.1 Profile Summary

| Profile ID | Description | Required Suites | Use Case |
|------------|-------------|-----------------|----------|
| STRICT-CORE | Core checks (build, unit, lint, format, schema) | SR-SUITE-STRICT-CORE | Standard code |
| STRICT-FULL | Core + integration + e2e + determinism | SR-SUITE-STRICT-FULL | Critical path |
| STRICT-DOCS | Schema, link, coherence checks | SR-SUITE-DOCS | Governance docs |
| STRICT-OPS | Deploy, health, idempotency checks | SR-SUITE-OPS | Operations |

### 5.2 Profile Selection Matrix

| Work Unit Type | Deliverables | Default Profile | Override Condition |
|----------------|--------------|-----------------|-------------------|
| code.domain | D-05, D-06, D-07, D-08 | STRICT-CORE | D-08 → STRICT-FULL |
| code.infra | D-02, D-03, D-04 | STRICT-CORE | — |
| code.persistence | D-09..D-13 | STRICT-CORE | D-10, D-11, D-12 → STRICT-FULL |
| code.evidence | D-14, D-15, D-16 | STRICT-CORE | D-16 → STRICT-FULL |
| code.api | D-17..D-20 | STRICT-CORE | D-18, D-19, D-20 → STRICT-FULL |
| code.orchestration | D-21, D-22, D-23 | STRICT-CORE | D-22, D-23 → STRICT-FULL |
| code.oracle | D-24..D-27 | STRICT-CORE | D-24, D-26, D-27 → STRICT-FULL |
| code.ui | D-28, D-29, D-30 | STRICT-CORE | D-30 → STRICT-FULL |
| ops.deploy | D-31, D-32 | STRICT-OPS | — |
| ops.observe | D-33 | STRICT-CORE | — |
| test.e2e | D-34, D-35, D-36 | STRICT-FULL | — |
| governance.revision | D-01 | STRICT-DOCS | — |

### 5.3 Oracle Suite Definitions

See **Appendix C: Profile Definitions (YAML)** for complete oracle suite specifications including:
- Oracle commands and timeouts
- Environment constraints
- Evidence capture policy
- Flake handling policy

### 5.4 Non-Waivable Integrity Conditions

The following MUST NEVER be waived (per C-EVID-6, C-OR-4..7):

| Condition | Reason |
|-----------|--------|
| ORACLE_TAMPER | Suite integrity violated |
| ORACLE_GAP | Required oracle missing |
| ORACLE_ENV_MISMATCH | Environment constraint violated |
| ORACLE_FLAKE | Determinism violated |
| EVIDENCE_MISSING | Evidence not retrievable |

---

## 6. Portals and Human Judgment Hooks

### 6.1 Portal Registry

| Portal ID | Purpose | Actor Kind | Trigger |
|-----------|---------|------------|---------|
| GovernanceChangePortal | Approve governance artifact changes | HUMAN only | GOVERNANCE_TOUCH, contradictions |
| ReleaseApprovalPortal | Approve candidate release | HUMAN only | Verified status + release request |
| ExceptionApprovalPortal | Approve deviations/deferrals/waivers | HUMAN only | Exception request |
| PhaseGatePortal | Approve phase completion | HUMAN only | Phase completion request |
| BudgetExtensionPortal | Approve budget extensions | HUMAN only | BUDGET_EXHAUSTED |
| OracleSuiteChangePortal | Approve oracle suite modifications | HUMAN only | Suite change request, integrity conditions |

### 6.2 Human Judgment Hook Configuration

| Hook Class | Purpose | Binding | Trigger Condition |
|------------|---------|---------|-------------------|
| plan_review | Non-binding review of iteration plan | NO | Before first iteration of complex deliverables |
| evaluation_on_verification | Interpretation of oracle results | NO | After verification with findings |
| assessment_on_validation | Judgment on fitness | NO | Before closeout of high-stakes deliverables |
| closeout | Final approval via Portal | YES | Before Shippable determination |

### 6.3 Gating Mode by Deliverable Category

| Category | Default Mode | Hard Triggers |
|----------|--------------|---------------|
| Foundation (D-01..D-04) | Soft | GOVERNANCE_TOUCH |
| Domain Core (D-05..D-08) | Hybrid | EXCEPTIONS_ACTIVE, REPEATED_FAILURE |
| Persistence (D-09..D-16) | Hybrid | OPEN_RISK_HIGH |
| API (D-17..D-20) | Hybrid | SECURITY_CRITICAL |
| Orchestration (D-21..D-23) | Hard | All (critical path) |
| Oracles (D-24..D-27) | Hard | All (integrity critical) |
| UI (D-28..D-30) | Soft | CLOSEOUT_PENDING |
| Ops (D-31..D-33) | Hybrid | SECURITY_CRITICAL |
| E2E (D-34..D-36) | Hard | All (release gate) |

See **Appendix B: Portal Playbooks** for complete playbook specifications.

---

## 7. Gate Registry

### 7.1 Gate Categories

| Gate Kind | Purpose | When Evaluated |
|-----------|---------|----------------|
| work_start | Authorize iteration start | Before IterationStarted processing |
| accept | Verify candidate quality | After oracle runs |
| release | Authorize candidate release | Before FreezeRecordCreated |
| phase_complete | Check phase status | At phase boundary |
| phase_release | Authorize phase progression | At phase boundary |

### 7.2 Core Gates

| Gate ID | Purpose | Membranes Enforced |
|---------|---------|-------------------|
| G-00 | Context admissible (no ghost inputs) | Intent, Ontological, Accountability, Authority |
| G-DEP-READY | Dependencies satisfied | Operational, Ontological |
| G-BUILD-PASS | Build succeeds | Operational, Isomorphic |
| G-UNIT-PASS | Unit tests pass | Operational, Isomorphic |
| G-LINT-PASS | Linting passes | Operational, Architectural |
| G-INTEGRATION-PASS | Integration tests pass | Operational, Isomorphic |
| G-DETERMINISM-PASS | Determinism verified | Operational, Isomorphic, Accountability |
| G-APPEND-ONLY-PASS | Append-only enforcement | Accountability, Authority |
| G-REPLAY-DETERMINISM-PASS | Replay determinism | Accountability, Isomorphic |

See **Appendix A: Gate Registry (CSV)** for complete gate specifications.

---

## 8. Plan-to-Workflow Mapping

### 8.1 Phase Structure

| Phase | Name | Packages | Deliverables | Gate |
|-------|------|----------|--------------|------|
| P0 | FOUNDATION | PKG-01, PKG-02 | D-01..D-04 | G-PHASE-0-RELEASE |
| P1 | DOMAIN-CORE | PKG-03 | D-05..D-08 | G-DOMAIN-RELEASE |
| P2 | PERSISTENCE | PKG-04, PKG-05 | D-09..D-16 | G-PERSIST-RELEASE, G-EVIDENCE-RELEASE |
| P3 | API | PKG-06 | D-17..D-20 | G-API-RELEASE |
| P4 | ORCHESTRATION + ORACLES | PKG-07, PKG-08 | D-21..D-27 | G-ORCH-RELEASE, G-ORACLE-RELEASE |
| P5 | UI + OPS | PKG-09, PKG-10 | D-28..D-33 | G-UI-RELEASE, G-OPS-RELEASE |
| P6 | E2E | PKG-11 | D-34..D-36 | G-RELEASE-FINAL |

### 8.2 Phase Gate Requirements

| Phase Gate | Required Evidence | Human Review |
|------------|-------------------|--------------|
| G-PHASE-0-RELEASE | D-02..D-04 Verified; CI green | Optional |
| G-DOMAIN-RELEASE | D-05..D-08 Verified; hex boundary audit | Required (architecture) |
| G-PERSIST-RELEASE | D-09..D-13 Verified; replay determinism proof | Required |
| G-EVIDENCE-RELEASE | D-14..D-16 Verified; content addressing proof | Required |
| G-API-RELEASE | D-17..D-20 Verified; API contract tests | Required |
| G-ORCH-RELEASE | D-21..D-23 Verified; SYSTEM-only proof | Required |
| G-ORACLE-RELEASE | D-24..D-27 Verified; integrity detection proof | Required |
| G-UI-RELEASE | D-28..D-30 Verified; portal workflow tests | Required |
| G-OPS-RELEASE | D-31..D-33 Verified; self-host boot | Required |
| G-RELEASE-FINAL | D-34..D-36 Verified; replayability proof | Required |

See **Appendix D: Plan-to-Workflow Mapping (CSV)** for complete deliverable mapping.

---

## 9. Exceptions, Deviations, Deferrals, Waivers

### 9.1 Exception Types

| Type | Purpose | Scope Options | Required Fields |
|------|---------|---------------|-----------------|
| Deviation | Acknowledge gap between actual and required | per-candidate, per-loop, per-baseline, time-boxed | requirement_ref, actual_state, risk_assessment, mitigation, resolution_criteria, expiry |
| Deferral | Defer requirement completion | per-candidate, per-loop, per-baseline, time-boxed | requirement_ref, deferral_reason, resolution_timeline, expiry |
| Waiver | Accept oracle FAIL for release | per-candidate (default), per-loop, time-boxed | oracle_ref, failure_details, risk_assessment, expiry, review_date |

### 9.2 Exception Constraints

| Constraint | Rule |
|------------|------|
| All exceptions must have expiry | No indefinite exceptions |
| Waivers require review date | Periodic review for continued validity |
| Integrity conditions non-waivable | ORACLE_TAMPER, ORACLE_GAP, ORACLE_ENV_MISMATCH, ORACLE_FLAKE, EVIDENCE_MISSING |
| Exceptions visible in Freeze | All active exceptions acknowledged in FreezeRecord |
| Scope escalation requires review | per-loop or per-baseline requires additional justification |

### 9.3 Exception Workflow

```
Exception needed
    │
    ├─► Create exception request
    │       │
    │       ▼
    ├─► ExceptionApprovalPortal review
    │       │
    │       ├─► Approved: ExceptionRecorded (active)
    │       │       │
    │       │       ├─► Visible in approvals/freezes
    │       │       │
    │       │       ├─► Resolution criteria monitored
    │       │       │
    │       │       └─► Expiry tracked
    │       │
    │       └─► Rejected: Fix underlying issue
    │
    └─► Exception lifecycle
            │
            ├─► Resolved: ExceptionResolved (closed)
            │
            └─► Expired: ExceptionExpired (blocks progress)
```

---

## 10. SR-ETT Membrane Coverage Matrix

### 10.1 Membrane Enforcement Summary

| Membrane | Primary Enforcement | Gates | Portals | Profiles/Oracles |
|----------|---------------------|-------|---------|------------------|
| Intent & Objective | SAPS ref in IterationStarted | G-00 | GovernanceChangePortal | — |
| Operational | Loop state machine, stop triggers | G-DEP-READY, G-*-PASS | PhaseGatePortal, BudgetExtensionPortal | All |
| Architectural | Hex boundary tests | G-HEX-BOUNDARY-PASS | PhaseGatePortal (P1) | STRICT-CORE |
| Ontological | Schema validation, typed refs | G-00, G-SCHEMA-VALID | GovernanceChangePortal | STRICT-DOCS |
| Isomorphic | Spec↔runtime conformance | G-REPLAY-DETERMINISM-PASS, G-CONTRACT-PASS | — | STRICT-FULL |
| Change | Governance change routing | G-GOV-START, G-GOV-RELEASE | GovernanceChangePortal | — |
| Authority & Integrity | Actor-kind enforcement | G-ACTOR-KIND-PASS, G-SYSTEM-ONLY-PASS | All (HUMAN-only) | — |
| Resource | Budget accounting | Budget limits | BudgetExtensionPortal | — |
| Accountability | Evidence bundles, no ghost inputs | G-00, G-CONTENT-ADDRESS-PASS | All | All |

### 10.2 Phase-Specific Membrane Focus

| Phase | Primary Membranes | Critical Gates |
|-------|-------------------|----------------|
| P0 | Ontological, Resource, Accountability | G-00, G-BUILD-PASS |
| P1 | Architectural, Ontological, Operational | G-HEX-BOUNDARY-PASS, G-DETERMINISM-PASS |
| P2 | Accountability, Isomorphic | G-APPEND-ONLY-PASS, G-REPLAY-DETERMINISM-PASS |
| P3 | Authority & Integrity, Isomorphic | G-ACTOR-KIND-PASS, G-CONTRACT-PASS |
| P4 | Operational, Authority & Integrity, Resource | G-SYSTEM-ONLY-PASS, G-INTEGRITY-DETECTION-PASS |
| P5 | All (integration) | G-PORTAL-WORKFLOW-PASS |
| P6 | All (comprehensive) | G-STATE-RECONSTRUCTION-PASS |

---

## 11. Directive Self-Verification

Before SR-DIRECTIVE is adopted as "current," the following must be verified:

### 11.1 Oracle Checks

| Check | Method | Required Outcome |
|-------|--------|------------------|
| Schema validation | STRICT-DOCS suite | All metadata valid |
| Internal link check | Link oracle | All internal refs resolve |
| Cross-reference check | Coherence oracle | All SR-* refs valid |
| Gate coverage | Coverage analysis | All deliverables have gates |
| Portal coverage | Coverage analysis | All triggers have portals |

### 11.2 Human Review

| Review | Reviewer | Focus |
|--------|----------|-------|
| Contract compliance | Governance owner | C-TB-4, C-LOOP-1, C-LOOP-3, C-OR-* satisfied |
| Plan coverage | Plan owner | All deliverables mapped |
| Membrane coverage | Architecture owner | All nine membranes enforced |
| Exception policy | Governance owner | Constraints appropriate |
| Budget policy | Resource owner | Defaults reasonable |

### 11.3 Adoption Gate

SR-DIRECTIVE adoption requires:
1. STRICT-DOCS profile verification PASS
2. S11 coherence audit PASS (or recorded deviations)
3. GovernanceChangePortal approval
4. PhaseGateApproval for SR-DIRECTIVE itself

---

## Appendices

### Appendix A: Gate Registry (CSV)

See attached file: `gate_registry.csv`

The gate registry defines all gates with:
- Gate ID and name
- Gate kind (work_start, accept, release, phase_complete, phase_release)
- Membranes enforced
- Enforcement surface (hex layer)
- Allowed actor kinds
- Required refs and commitment objects
- Success/failure conditions
- Stop triggers
- Relief valves and routing portals

### Appendix B: Portal Playbooks

See attached files:
- `GovernanceChangePortal.md`
- `ReleaseApprovalPortal.md`
- `ExceptionApprovalPortal.md`
- `PhaseGatePortal.md`
- `BudgetExtensionPortal.md`
- `OracleSuiteChangePortal.md`

Each playbook defines:
- Portal identification and scope
- Purpose and trust boundary
- Allowed request types
- Actor rules
- Preconditions
- Evidence review checklist
- Decision procedure
- Output records
- Failure handling
- Cross-references

### Appendix C: Profile Definitions (YAML)

See attached file: `profile_definitions.yaml`

The profile definitions include:
- Oracle suite specifications (oracles, commands, timeouts, environment constraints)
- Verification profiles (required/advisory suites, waiver policy)
- Profile selection matrix (work unit type → profile mapping)

### Appendix D: Plan-to-Workflow Mapping (CSV)

See attached file: `plan_to_workflow_mapping_instance1.csv`

The mapping defines for each deliverable:
- Workflow phase
- Work unit type
- Start, accept, and release gates
- Required portals
- Oracle suite and verification mode
- Required evidence bundles
- Budget defaults
- Stop trigger overrides

---

## Document Governance

| Aspect | Value |
|--------|-------|
| Owner | Governance authority (instance-1) |
| Review cadence | Per SR-PLAN phase completion |
| Change process | GovernanceChangePortal → SR-CHANGE |
| Staleness propagation | depends_on SR-PLAN, SR-CONTRACT, SR-SPEC |

---

*End of SR-DIRECTIVE v1.0.0-draft.1*

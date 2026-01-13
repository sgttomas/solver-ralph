---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-DIRECTIVE.PLAYBOOK.ExceptionApprovalPortal.instance1"
  type: "config.portal_playbook"
  title: "Portal Playbook — ExceptionApprovalPortal (instance-1)"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "directional"
  authority_kind: "config"
  created: "2026-01-11"
  updated: "2026-01-11"
  tags: ["sr-directive", "portal", "playbook", "exceptions", "waivers", "instance-1"]
---

# ExceptionApprovalPortal — Portal Playbook (first pass)

> **Purpose:** Human-only boundary for creating and managing **exceptions as records** (deviations/deferrals/waivers).  
> This portal is the only allowed place to grant a waiver that enables **Verified-with-Exceptions**.

## 1) Portal identification

- **portal_id:** `ExceptionApprovalPortal`
- **portal_kind:** `exception_portal` *(records `WaiverCreated` / `DeviationCreated` / `DeferralCreated` etc.)*  
- **scope:** `instance` *(SOLVER‑Ralph instance‑1)*

## 2) Purpose and boundary

- **Purpose (one sentence):**  
  Create and approve **scoped, time-boxed exception records** so the system can proceed without silently rewriting governance.

- **Trust boundary being crossed (SR‑ETT membranes):**  
  **Authority & Integrity**; **Change**; **Accountability** *(secondary: Resource; Operational)*

- **What this portal MUST NOT do:**  
  - MUST NOT waive integrity conditions (ORACLE_TAMPER, EVIDENCE_MISSING, ORACLE_ENV_MISMATCH, etc.).  
  - MUST NOT grant unbounded scope waivers (“class-wide indefinite waivers”).  
  - MUST NOT implicitly change SR‑CONTRACT/SR‑SPEC meaning; if that’s required, route to GovernanceChangePortal.

## 3) Allowed request types

- [x] `waiver_request` *(gate/oracle waiver; FAIL outcomes only; scoped)*  
- [x] `deviation_request` *(exception from a requirement)*  
- [x] `deferral_request` *(postponement of a requirement/deliverable)*  
- [x] `exception_resolution_request` *(resolve/expire exception)*  
- [ ] `approval_request` *(ReleaseApprovalPortal / GovernanceChangePortal)*  
- [ ] `freeze_request` *(ReleaseApprovalPortal)*  
- [ ] `decision_record` *(DecisionRecorded flow)*

## 4) Actor rules

- **Allowed actor kinds:** `HUMAN only`
- **Identity requirements:** stable, verifiable identity; authorized to approve exceptions.
- **Attribution policy:** exception create/activate/resolve MUST be attributable to a human actor.

## 5) Preconditions

### 5.1 Global preconditions (all exception types)

- Request MUST reference:
  - the exact governed requirement/gate/oracle being excepted,
  - the scope (candidate/loop/baseline/time-boxed class),
  - risk + mitigation,
  - expiry/sunset or review date,
  - resolution criteria + owner.

- Evidence referenced MUST be retrievable (EVIDENCE_MISSING blocks).

### 5.2 Waiver-specific preconditions

- Waiver MUST apply only to an explicit **FAIL outcome** (not “missing evidence” or “integrity faults”).  
- Waiver MUST NOT be used to bypass any integrity condition.
- Scope MUST satisfy Contract waiver scope constraints (default per-candidate).

### 5.3 Deviation/deferral preconditions

- Must reference the specific requirement (Contract/SPEC/Directive/Plan item).  
- Must define scope and expiry/review and resolution.

## 6) Inputs required at submission time

### 6.1 Waiver request payload (minimum)

- references the specific gate/oracle and failure(s) being waived
- lists failure(s) waived (if multiple)
- includes scope + env constraints
- states risk/mitigation
- includes resolution criteria + owner
- includes expiry/review date
- evidence references supporting the FAIL outcome (run/evidence bundle ids)

### 6.2 Deviation/deferral payload (minimum)

- requirement reference(s)
- scope (deliverable/phase/loop/baseline)
- rationale + impact
- expiry/review date
- resolution plan

## 7) Procedure

1) **Intake + validation (SYSTEM):**
   - validate required fields
   - validate scope constraints
   - validate evidence retrievability

2) **Review (HUMAN):**
   - ensure the exception is necessary and appropriately scoped
   - ensure it does not waive integrity conditions
   - ensure expiry/review and resolution are credible

3) **Decision (HUMAN):**
   - approve or reject

4) **Record exception (SYSTEM emits from HUMAN action):**
   - Emit `WaiverCreated` / `DeviationCreated` / `DeferralCreated`
   - If immediately in force: emit `ExceptionActivated`

5) **Resolution (when applicable):**
   - Use resolve/expire paths to emit `ExceptionResolved` / `ExceptionExpired`

## 8) Outputs (binding records emitted)

- **Primary record types:** `WaiverCreated`, `DeviationCreated`, `DeferralCreated`, `ExceptionActivated`, `ExceptionResolved`, `ExceptionExpired`
- **Required fields (waiver):**
  - gate/oracle reference + waived failures
  - scope + env constraints
  - risk/mitigation + resolution criteria + owner
  - expiry or review date
  - approver identity + timestamp
- **Follow-on effects:**
  - Enables `Verified-with-Exceptions` only when waiver is active and in-scope.

## 9) Failure handling and routing

- **If preconditions fail:** reject; requester must correct fields/scope/evidence.
- **If evidence is missing/unfetchable:** block; treat as non-waivable; route to GovernanceChangePortal for incident/policy handling if needed.
- **If the request attempts to waive integrity conditions:** reject; record as a policy violation; consider GovernanceChangePortal if systemic.
- **If the exception implies governance semantics change:** route to GovernanceChangePortal.

## 10) Auditability

Store:

- exception request payload hash
- reviewer identity + timestamp
- evidence bundle ids reviewed
- resulting exception ids and their scope/expiry

Retention expectation: baseline-grade (exceptions must remain visible at approval/freeze).

## 11) Cross-references

- **Gate routing (from Gate Registry):** waiver routing referenced by gates: `G-30, G-31` (as `/exceptions/waivers`)  
- **SR‑CONTRACT clauses:** `C-EXC-1..5; C-EVID-6; C-TB-1; C-TB-6`  
- **SR‑SPEC sections:** `§2.3.5 (Exceptions); §1.14 (Gate Waiver scope/constraints); Appendix C (integrity conditions)`  
- **SR‑ETT membranes:** Authority & Integrity; Change; Accountability  
- **SR‑PLAN items:** deliverables that define exception + portal workflows (notably D‑19, D‑30; and any deliverable using WITH_EXCEPTIONS flows)  

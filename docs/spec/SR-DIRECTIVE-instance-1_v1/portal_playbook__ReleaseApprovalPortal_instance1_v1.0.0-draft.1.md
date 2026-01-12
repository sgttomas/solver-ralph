---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-DIRECTIVE.PLAYBOOK.ReleaseApprovalPortal.instance1"
  type: "config.portal_playbook"
  title: "Portal Playbook — ReleaseApprovalPortal (instance-1)"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "directional"
  authority_kind: "config"
  created: "2026-01-11"
  updated: "2026-01-11"
  tags: ["sr-directive", "portal", "playbook", "release-approval", "freeze", "instance-1"]
---

# ReleaseApprovalPortal — Portal Playbook (first pass)

> **Purpose:** Human-only binding boundary for release / baseline freeze progression.  
> Approves a **specific Candidate** for release, explicitly acknowledging active exceptions, enabling FreezeRecord creation.

## 1) Portal identification

- **portal_id:** `ReleaseApprovalPortal`
- **portal_kind:** `approval_portal` *(records `ApprovalRecorded` with `portal_id=ReleaseApprovalPortal`)*  
- **scope:** `instance` *(SOLVER‑Ralph instance‑1)*

## 2) Purpose and boundary

- **Purpose (one sentence):**  
  Approve (or reject) a Candidate for **baseline freezing / shippable progression** using evidence-bound verification results and explicit exception acknowledgement.

- **Trust boundary being crossed (SR‑ETT membranes):**  
  **Authority & Integrity**; **Accountability**; **Operational** *(secondary: Change; Isomorphic)*

- **What this portal MUST NOT do:**  
  - MUST NOT approve without evidence references (“no approval by narrative”).  
  - MUST NOT bypass integrity conditions (e.g., ORACLE_TAMPER, EVIDENCE_MISSING).  
  - MUST NOT approve a stale Candidate as shippable unless staleness is explicitly resolved per mechanics.

## 3) Allowed request types

- [x] `approval_request` *(release approval for candidate)*  
- [x] `freeze_request` *(create FreezeRecordCreated for an approved candidate; human-only)*  
- [ ] `waiver_request` *(ExceptionApprovalPortal)*  
- [ ] `governance_change_request` *(GovernanceChangePortal)*  
- [ ] `decision_record` *(DecisionRecorded flow)*

## 4) Actor rules

- **Allowed actor kinds:** `HUMAN only` *(for the portal action(s): Approval and Freeze)*  
- **Identity requirements:** stable, verifiable identity; authorized to approve releases.  
- **Attribution policy:** the approval/freeze MUST be attributable to the human actor id.

## 5) Preconditions

The portal MUST reject the request unless all preconditions hold.

### 5.1 Required commitment objects present

- `CandidateMaterialized` exists and Candidate identity is stable and referenceable.
- Candidate is **Verified (Strict)** or **Verified-with-Exceptions** (per Contract/SPEC).
- For Verified-with-Exceptions:
  - all required waivers are present and active and correctly scoped.

### 5.2 Required refs present

Approval request MUST include refs to:

- `Candidate` (subject)
- `EvidenceBundleRecorded` for the relevant verification Run(s) (STRICT‑CORE / STRICT‑FULL as applicable)
- Active governed artifacts in force (SR‑CONTRACT, SR‑SPEC, SR‑TYPES, SR‑DIRECTIVE, and the pinned oracle suite/profile identifiers)

### 5.3 Evidence integrity + retrievability

- Evidence must be retrievable; **EVIDENCE_MISSING is non‑waivable**.
- Required oracle suite hash must match the declared suite/profile.

### 5.4 Staleness constraints

- The Candidate MUST NOT have unresolved staleness on:
  - any `rel=depends_on` dependency, including governed artifacts, and
  - the oracle suite/profile used for verification.

If staleness exists: require resolution/re-verification before approval and freeze.

### 5.5 Exception acknowledgement constraint

- The Approval request MUST contain `exceptions_acknowledged[]` explicitly (including empty list).  
- If a subsequent `FreezeRecordCreated.active_exceptions[]` is non-empty, the approval’s acknowledgements MUST cover them.

## 6) Inputs required at submission time

### 6.1 Release approval request payload (minimum)

- `portal_id = "ReleaseApprovalPortal"`
- `decision ∈ {approve, reject}`
- `subject_refs[]` MUST include:
  - `candidate_id`
  - `run_id`(s) / `evidence_bundle_id`(s) used to compute verification
- `evidence_refs[]` MUST include:
  - evidence bundle(s) from the required verification suite(s)
  - any integrity-condition records (if present)
- `exceptions_acknowledged[]` MUST be present (explicit empty list allowed)

### 6.2 Freeze request payload (minimum)

- `baseline_id` (freeze id / baseline label)
- `candidate_id`
- `verification` (STRICT or WITH_EXCEPTIONS) + suite/profile reference
- `release_approval_id` (the approval just created)
- `artifact_manifest[]` (the governed artifact versions in force)
- `active_exceptions[]` (in scope)

## 7) Procedure

1) **Intake + validation (SYSTEM):**  
   Validate required refs, evidence retrievability, suite hash match, and staleness.

2) **Review (HUMAN):**  
   - Review verification evidence bundle(s).  
   - Review active exceptions and associated waivers/deviations/deferrals.  
   - Confirm `exceptions_acknowledged[]` matches what is active/in-scope.

3) **Decision (HUMAN):**  
   - Approve only if:
     - candidate verification is acceptable (strict or with-exceptions),
     - no non-waivable integrity faults exist,
     - no unresolved staleness exists,
     - exceptions are explicitly acknowledged.

4) **Record approval (SYSTEM emits from HUMAN action):**  
   Emit `ApprovalRecorded` with `portal_id=ReleaseApprovalPortal`.

5) **Freeze baseline (HUMAN → SYSTEM):**  
   If approval is `approve`, submit `FreezeRecordCreated` referencing the approval, pinned artifact manifest, and active exceptions.  
   System MUST enforce the exception‑acknowledgement constraint.

## 8) Outputs (binding records emitted)

- **Primary record types:**  
  - `ApprovalRecorded` *(release approval)*  
  - `FreezeRecordCreated` *(baseline snapshot)*

- **Required fields (ApprovalRecorded):**  
  - `portal_id="ReleaseApprovalPortal"`  
  - `decision`  
  - `subject_refs[]`  
  - `evidence_refs[]`  
  - `exceptions_acknowledged[]` (explicit)

- **Required fields (FreezeRecordCreated):**  
  - `baseline_id`, `candidate_id`, `verification`, `release_approval_id`  
  - `artifact_manifest[]`, `active_exceptions[]`

- **Follow-on events:** shippable computation is a projection; baseline becomes binding snapshot for replay/audit.

## 9) Failure handling and routing

- **If preconditions fail:** reject; do not freeze.  
- **If evidence is missing/unfetchable:** halt; treat as non-waivable; route to **GovernanceChangePortal** for policy/incident handling if needed.  
- **If integrity conditions detected:** halt; re-run verification or remediate; do not approve/freeze.  
- **If staleness present:** resolve staleness (re-verify/re-evaluate) before approval.

## 10) Auditability

Store:

- approval payload hash + identity + timestamp
- freeze payload hash + identity + timestamp
- evidence bundle ids reviewed
- exception set acknowledged (explicit list)
- resulting approval_id and freeze_id

Retention expectation: baseline-grade retention (treat as long‑lived).

## 11) Cross-references

- **Gate routing (from Gate Registry):** routed-to by gates: `G-90` (and indirectly by failures in upstream gates requiring arbitration)  
- **SR‑CONTRACT clauses:** `C-TB-4; C-TB-6; C-VER-1..3; C-SHIP-1; C-EXC-2; C-EVT-6; C-EVID-6`  
- **SR‑SPEC sections:** `§2.3.4 (Approvals); §2.3.8 (Freeze records + shippable endpoint); §1.12.3 (exception acknowledgement constraint); §1.12.4 (shippable rule); §1.13.5 (staleness + shippable gating)`  
- **SR‑ETT membranes:** Authority & Integrity; Accountability; Operational  
- **SR‑PLAN items:** D‑34..D‑36 (E2E + replay + freeze path); D‑19 (portal API); D‑30 (portal UI)  

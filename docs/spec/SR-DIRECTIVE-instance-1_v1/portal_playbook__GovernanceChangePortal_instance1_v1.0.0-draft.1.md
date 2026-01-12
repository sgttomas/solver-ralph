---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-DIRECTIVE.PLAYBOOK.GovernanceChangePortal.instance1"
  type: "config.portal_playbook"
  title: "Portal Playbook — GovernanceChangePortal (instance-1)"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "directional"
  authority_kind: "config"
  created: "2026-01-11"
  updated: "2026-01-11"
  tags: ["sr-directive", "portal", "playbook", "governance-change", "instance-1"]
---

# GovernanceChangePortal — Portal Playbook (first pass)

> **Purpose:** Human-only binding boundary for **normative governance changes** (governed artifacts, binding meanings, and systemic policy changes).  
> **Derived from:** SR-CONTRACT portal minimums + SR-SPEC approval/registry mechanics + gate-routing in the instance‑1 Gate Registry.

## 1) Portal identification

- **portal_id:** `GovernanceChangePortal`
- **portal_kind:** `approval_portal` *(records `ApprovalRecorded` with `portal_id=GovernanceChangePortal`)*  
- **scope:** `instance` *(SOLVER‑Ralph instance‑1)*

## 2) Purpose and boundary

- **Purpose (one sentence):**  
  Approve or reject a **governance change packet** so that changes to governed artifacts and binding meanings are **durable, attributable, evidence‑bound, and replayable**.

- **Trust boundary being crossed (SR‑ETT membranes):**  
  **Change**; **Authority & Integrity**; **Accountability** *(secondary: Ontological; Isomorphic)*

- **What this portal MUST NOT do:**  
  - MUST NOT treat an agent narrative as sufficient basis for changing governed artifacts (“no approval by narrative”).  
  - MUST NOT mint “Verified/Approved/Shippable” state by itself (it only records approvals; status comes from the governed mechanics).  
  - MUST NOT introduce new binding semantics; if the requested change alters meaning beyond current SR‑CONTRACT/SR‑SPEC, route via **SR‑CHANGE**.

## 3) Allowed request types

This portal accepts **only** the following request types:

- [x] `approval_request` *(portal approval for a governance change packet)*
- [x] `governed_artifact_current_pointer_change` *(approve setting `is_current=true` for a governed artifact version)*
- [x] `governed_artifact_semantics_change` *(approve a new SR‑* artifact version that changes normative meaning)*
- [x] other: `governance_change_arbitration` *(approve a decision to route/sequence governance work without redefining semantics; still recorded as approval)*

Explicitly rejected at this portal:

- [ ] `freeze_request` *(belongs to ReleaseApprovalPortal / Freeze workflow)*
- [ ] `waiver_request` *(belongs to ExceptionApprovalPortal)*
- [ ] `decision_record` *(belongs to Decision recording flow)*

## 4) Actor rules

- **Allowed actor kinds:** `HUMAN only`
- **Identity requirements:** authenticated + authorized per SR‑SPEC authn/authz (OIDC); stable actor identity required.
- **Attribution policy:** portal approval MUST record (directly or by derivation): `actor_kind=HUMAN`, stable `actor_id`, timestamp.

## 5) Preconditions

The portal MUST reject the request unless all preconditions hold.

### 5.1 Required commitment objects present

- A **governance change packet** MUST be present and referenceable, consisting of at least:
  - the proposed governed artifact version(s) (content-hash pinned), and
  - a change rationale and impact summary (can be a document/candidate ref).
- If the request is “set is_current=true”:
  - the target governed artifact version MUST already be registered (or included with registration evidence).

### 5.2 Required refs present

The approval request MUST reference (directly or indirectly) the active:

- `SR-CONTRACT@…`
- `SR-SPEC@…`
- `SR-TYPES@…`
- `SR-ETT@…`
- `SR-PLAN instance-1@…` *(when change affects plan or plan execution)*

### 5.3 Evidence retrievability

- Evidence referenced by the request MUST be retrievable; **EVIDENCE_MISSING is non‑waivable**.

### 5.4 Staleness / coherence

- If the requested change depends on superseded governed artifacts, the request MUST either:
  - update to current governed artifacts, or
  - explicitly request to change the “current” pointer as part of the packet.
- Cross‑reference integrity MUST pass (no broken refs in the new artifact versions).

## 6) Inputs required at submission time

### 6.1 Request payload (minimum)

For an `approval_request` to GovernanceChangePortal:

- `portal_id = "GovernanceChangePortal"`
- `decision ∈ {approve, reject}`
- `subject_refs[]` including:
  - governed artifact ids + versions being approved (or current-pointer change target)
  - any affected directive/spec/contract artifacts
- `evidence_refs[]` including:
  - coherence / cross‑reference integrity evidence (or manual review packet pointers)
  - any required oracle outputs validating schema/frontmatter
- `exceptions_acknowledged[]` MUST be present *(explicit empty list allowed)*

### 6.2 Recommended supporting evidence

- Schema/frontmatter validation results for each artifact version
- Cross-reference integrity report
- Coherence/S11-style audit report (even if lightweight)

## 7) Procedure

1) **Intake + validation (SYSTEM):**  
   Validate payload shape, actor identity, required refs, and evidence retrievability.

2) **Change packet review (HUMAN):**  
   - Review what artifacts change, what semantics change, and what downstream impacts occur.
   - Confirm the change does not silently redefine binding meanings outside SR‑CHANGE.

3) **Decision (HUMAN):**  
   - Approve only if the packet is coherent, attributable, and properly evidenced.  
   - Otherwise reject with rationale (non‑binding “request changes” may be recorded separately as notes).

4) **Record binding outcome (SYSTEM emits from HUMAN action):**  
   - Emit `ApprovalRecorded` with `portal_id=GovernanceChangePortal`.

5) **Follow‑on actions (SYSTEM, if approved):**  
   - If the packet includes governed artifact registration: register versions (if not already).  
   - If approval includes `is_current=true` changes: update registry current pointer per SR‑SPEC constraints.  
   - Trigger staleness propagation if changing current governed artifacts affects downstream dependents.

## 8) Outputs (binding records emitted)

- **Primary record type:** `ApprovalRecorded`
- **Required fields:**  
  - `portal_id="GovernanceChangePortal"`  
  - `decision`  
  - `subject_refs[]`  
  - `evidence_refs[]`  
  - `exceptions_acknowledged[]` (explicit; may be empty)

- **Follow‑on events (as applicable):**  
  - `GovernedArtifactVersionRecorded` *(registry ingestion)*  
  - staleness events (`NodeMarkedStale`, etc.) when current pointers change

## 9) Failure handling and routing

- **If preconditions fail:** reject; requester must supply missing refs/evidence or correct schema.  
- **If evidence is missing/unfetchable:** treat as **non‑waivable**; block until resolved; incident handling may be required.  
- **If the request implies new binding semantics:** reject and require a governance change routed via SR‑CHANGE (contract/spec amendments).

## 10) Auditability

Store (as evidence/provenance):

- portal submission payload hash
- reviewer identity + timestamp
- links to reviewed evidence bundle(s) and change packet refs
- approval record id

Retention expectation: same as governed artifact + baseline audit retention (treat as long‑lived).

## 11) Cross-references

- **Gate routing (from Gate Registry):** routed-to by gates: `G-00, G-10, G-15, G-20, G-30, G-31, G-40, G-50, G-60, G-70, G-71, G-80, G-90`  
- **SR‑CONTRACT clauses:** `C-TB-4; C-TB-6; C-META-1..3; C-EXC-3; C-DEC-1 (when approval accompanies systemic stop arbitration)`  
- **SR‑SPEC sections:** `§2.3.4 (Approvals); §2.3.6 (Governed artifacts); §1.10 (Registry semantics); §1.13 (Staleness)`  
- **SR‑ETT membranes:** Change; Authority & Integrity; Accountability  
- **SR‑PLAN items:** governance-touch deliverables + portal/API/UI deliverables (notably D‑19, D‑30)  

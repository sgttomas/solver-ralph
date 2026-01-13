---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "TEMPLATE-PORTAL-PLAYBOOK"
  type: "config.template"
  title: "SR-DIRECTIVE Portal Playbook Template"
  version: "1.0.0-draft.1"
  status: "draft"
  normative_status: "directional"
  authority_kind: "config"
  created: "2026-01-11"
  updated: "2026-01-11"
  tags: ["sr-directive", "portal", "playbook", "template"]
---

# Portal Playbook Template (fill-in)

> **Reminder:** SR-ETT forbids introducing new portal *kinds* or redefining binding semantics in a directive/template. If this playbook implies new binding meaning, route via SR-CHANGE and update SR-CONTRACT/SR-SPEC.  

## 1) Portal identification

- **portal_id:** `{PORTAL_ID}`  
- **portal_kind:** `{PORTAL_KIND}`  
  - Must be an existing kind/semantic from the governed set.  
- **scope:** `{instance | global | phase | work_unit_type | deliverable_id}`

## 2) Purpose and boundary

- **Purpose (one sentence):**  
  `{WHY_THIS_PORTAL_EXISTS}`

- **Trust boundary being crossed (SR-ETT membranes):**  
  `{Authority & Integrity | Change | Accountability | ...}`

- **What this portal MUST NOT do:**  
  `{e.g., "mint verification evidence", "override integrity conditions", "invent new lifecycle states"}`

## 3) Allowed request types

List the request types this portal accepts (explicitly deny all others):

- [ ] approval_request  
- [ ] decision_request (stop-the-line arbitration)  
- [ ] exception_request (deviation | deferral | waiver)  
- [ ] governance_change_request  
- [ ] budget_extension_request  
- [ ] oracle_suite_change_request  
- [ ] freeze_request  
- [ ] other: `{...}`

## 4) Actor rules

- **Allowed actor kinds:** `{HUMAN only | HUMAN + SYSTEM}`  
- **Identity requirements:** `{authn/authz policy; identity provider assumptions}`  
- **Attribution policy:** `{what fields are required to bind an action to identity}`

## 5) Preconditions

State what MUST be true before the portal will accept a request.

- **Required commitment objects present:**  
  - `{Candidate}`  
  - `{EvidenceBundleRecorded}`  
  - `{Exception records in scope}`  
  - `{...}`

- **Required refs present (IterationStarted / request payload):**  
  - `{SR-TYPES@...}`  
  - `{SR-CONTRACT@...}`  
  - `{SR-SPEC@...}`  
  - `{SR-DIRECTIVE@...}`  
  - `{OracleSuite@...}`  
  - `{...}`

- **Staleness rules:**  
  - `{e.g., "request MUST be rejected if candidate is stale on any depends_on refs"}`

## 6) Evidence review checklist (Accountability harness)

Define what evidence the human MUST review (or explicitly state “none”).

- Evidence bundle manifest(s): `{required}`  
- Oracle run transcript(s): `{required}`  
- Determinism / environment fingerprint: `{required}`  
- Diff / patchset: `{required}`  
- Risk notes / assessments: `{optional}`  

## 7) Decision procedure (what to do)

- **Decision options:**  
  - Approve  
  - Reject  
  - Request changes (non-binding)  
  - Escalate to `{OTHER_PORTAL_ID}`  

- **Decision rubric (how to decide):**  
  `{short rubric}`

## 8) Outputs (binding records emitted)

List the durable commitment objects the system MUST record if the request is accepted.

- **Primary record type:** `{ApprovalRecorded | DecisionRecorded | DeviationRecorded | ...}`  
- **Required fields:**  
  - `{candidate_id}`  
  - `{evidence_manifest_hash}`  
  - `{acknowledged_exceptions[]}`  
  - `{scope}`  
  - `{expiry}` (for waivers/deferrals)  
  - `{rationale}`  

- **Follow-on events:** `{LoopResumed | FreezeRecordCreated | ...}`

## 9) Failure handling and routing

- **If preconditions fail:** `{reject; emit StopTriggered?; route to ...}`  
- **If evidence is missing/unfetchable:** `{treat as non-waivable; route to GovernanceChangePortal}`  
- **If integrity conditions detected:** `{halt; route to oracle integrity portal}`  

## 10) Auditability

- **What should be stored as evidence of the portal interaction:**  
  - `{portal UI log / submission payload hash / reviewer identity / timestamp}`  
- **Retention expectation:** `{duration}`

## 11) Cross-references

- **SR-CONTRACT clauses:** `{C-...}`  
- **SR-SPEC sections:** `{§...}`  
- **SR-ETT membranes:** `{...}`  
- **SR-PLAN items:** `{deliverable_ids/packages}`  

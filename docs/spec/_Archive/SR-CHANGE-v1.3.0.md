---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-CHANGE"
  type: "governance.change_mgmt"
  title: "SOLVER-Ralph Change Management"
  version: 1.3.0
  status: "governed"
  normative_status: "normative"
  authority_kind: "process"
  governed_by: ["SR-CHANGE"]
  supersedes:
    - "SR-CHANGE@1.2.0"
    - "SR-CHANGE@1.0.0"
    - "SR-CHANGE@1.0.0-draft.2"
    - "SR-CHANGE@1.0.0-draft.1"
  created: "2026-01-11"
  updated: "2026-01-11"
  tags:
    - "solver-ralph"
    - "governance"
    - "change-management"
    - "versioning"
    - "portal-routing"
    - "freeze"
    - "baseline"
    - "staleness"
    - "exceptions"
    - "decisions"
    - "emergency"
  ext:
    change_mgmt:
      # portal_ids are concrete portal IDs required for conformance.
      portal_ids:
        - "GovernanceChangePortal"
        - "ReleaseApprovalPortal"
        - "ExceptionApprovalPortal"
      # portal_functions are the semantic portal responsibilities.
      # A function MAY be implemented as a dedicated portal_id or as a typed request within another portal_id.
      portal_functions:
        - "GovernanceChangePortal"
        - "ReleaseApprovalPortal"
        - "ExceptionApprovalPortal"
        - "OracleSuiteApproval"  # function; may be implemented within GovernanceChangePortal
      portal_function_bindings:
        OracleSuiteApproval:
          implemented_by:
            - "GovernanceChangePortal"
          request_types:
            - "ORACLE_SUITE_REGISTRATION"
            - "ORACLE_SUITE_REQUIRED_CHANGE"
            - "ORACLE_SUITE_ADVISORY_CHANGE"
            - "ORACLE_SUITE_ENV_CONSTRAINT_CHANGE"
      classification:
        scope_classes: ["G", "O", "I", "R"]
        severity_classes: ["PATCH", "MINOR", "MAJOR"]
---

# SOLVER-Ralph Change Management v1.3.0

**Purpose:** Define **how the governed set evolves**: what requires change control, classification + versioning rules, portal routing + approval semantics, freeze/baseline policy, exception handling (deviations/deferrals/waivers), decision records, staleness/rollback obligations, and the emergency change path.

**Normative status:** **Normative (binding).** This document has **process authority** over how governance artifacts and governed verification controls may change.

**Primary question:** “How do we change the governed set?”

**Scope note:** This document defines **policy and semantics**. Operational playbooks/templates MAY be generated as adjacent artifacts at project initialization, but MUST NOT contradict the binding rules here, SR-CONTRACT, or SR-SPEC.

---

## How to interpret this document

### Binding vs guidance

- **Binding (MUST/SHALL):**
  - what counts as a governed change
  - classification + versioning rules
  - portal routing rules
  - minimum approval semantics (incl. evidence + exception acknowledgement)
  - coupling rules (approvals ↔ selection ↔ freeze)
  - freeze manifest completeness rules
  - exception/waiver visibility requirements
  - decision-record requirements
  - rollback + staleness obligations
  - emergency constraints

- **Guidance (SHOULD/MAY):**
  - recommended checklists and review habits
  - role separation suggestions
  - example structures (non-binding)

### Precedence and the override rule

**Binding precedence** is defined in **SR-TYPES** (“Binding Precedence”). In short:

1) **SR-CONTRACT** (wins on invariants and safety contracts)  
2) **SR-SPEC** (wins on schemas/state machines and what is “implemented”)  
3) **SR-DIRECTIVE** (wins on gates/profiles/budgets and execution policy)  
4) **SR-INTENT** (explains rationale; directional)  
5) **SR-README** (navigation; index)

**Override rule:** If documents disagree, you do not “hand-wave the conflict away.” You open a governed change and reconcile explicitly.

### Agents are non-authoritative

Agents MAY draft artifacts, run oracles, and propose changes. Agent output is always **proposal** unless backed by recorded oracle evidence and/or recorded human approvals.

**Trust-boundary actions are human-only** (e.g., approvals, selection changes for normative artifacts, freeze finalization, exception activation).

### Self-governance

SR-CHANGE is self-governing (`governed_by: ["SR-CHANGE"]`). See **§10** for bootstrapping and self-amendment constraints.

---

## 0. Version changes
### 1.3.0 (2026-01-11)

- Reframes the “minimum portal set” as a set of **portal functions** with allowed implementation bindings.
- Removes the requirement for a dedicated `OracleSuiteApproval (function)` portal_id; oracle suite approval may be implemented as typed requests within `GovernanceChangePortal` while preserving human-only binding, evidence linkage, and auditability.
- Updates the portal routing matrix accordingly.


### 1.1.0 (2026-01-10)

- Adds promotion/selection policy for `config.agent_definition` artifacts (versioned, auditable, default selection is change-controlled).
- Adds promotion/selection policy for `config.gating_policy` artifacts (work-unit scoped; versioned, auditable, default selection is change-controlled).
- Declares ContextCompiler semantic changes as governance-impacting (require GovernanceChangePortal approval).
- Clarifies staleness semantics so audit-only references (`rel=supported_by`) do not propagate blocking staleness by default.

### 1.0.0 (2026-01-09)

- Promotes the consolidated draft series to a **governed** baseline for implementation.
- Uses **two-axis classification** (Scope × Severity) with explicit SemVer mapping.
- Requires approval records to include **evidence_refs** and explicit **exceptions_acknowledged** (via SR-CONTRACT C-TB-6).
- Requires Freeze manifests to include the **full canonical governed set** (per SR-TYPES baseline semantics).
- Adds explicit rules for **rollback**, **staleness**, and **decision records**.
- Keeps governance lean: templates may exist, but **policy is here**.

---

## 1. What requires change control

### 1.1 What counts as a governed change (binding)

A **governed change** is any change that affects:

1) **The canonical governed set** (membership, stable identities, or canonical document meaning), or  
2) Any governed artifact where `normative_status = normative`, or  
3) Any rule that affects what can be claimed as **Verified / Approved / Shippable**, or  
4) Any **portal definition**, portal routing, or trust-boundary enforcement rule, or  
5) Any **oracle suite** used for governed verification (registration, required/advisory classification, environment constraints, suite pin/rebase), or  
6) Any change to which version is selected as **current** (`is_current=true`) for:
   - any `normative_status=normative` artifact (SR-SPEC trust boundary), **or**
   - any artifact in the canonical governed set (auditability rule).

7) Any change to **ContextCompiler semantics** (what counts as iteration context; precedence rules; inclusion/exclusion of ref kinds/selectors), or
8) Any change to the default selection/promotion policy for `config.agent_definition` artifacts that affects SYSTEM/worker behavior.


If you’re unsure whether something is a governed change, treat it as governed (conservative default).

### 1.2 Change classification (binding)

Every governed change MUST be classified on **two axes**:

- **Scope:** what domain is changing (**G/O/I/R**)
- **Severity:** how breaking it is (**PATCH/MINOR/MAJOR**)

#### 1.2.1 Scope classes (binding)

| Scope | Meaning | Examples (non-exhaustive) |
|---|---|---|
| **G** | Governance (canonical docs + their meaning/selection) | Changing SR-CONTRACT/SR-SPEC/SR-DIRECTIVE/SR-TYPES/SR-CHANGE; changing `is_current` for canonical artifacts |
| **O** | Oracle suites and verification policy | Registering a suite; changing required/advisory; changing environment constraints; suite pin/rebase rules |
| **I** | Implementation configuration that affects enforcement | CI policy that changes what counts as “PASS”; authz policy for portals; evidence retention policy |
| **R** | Records & evidence outputs (typed, binding state) | Creating a Freeze Record; recording a Decision; creating or modifying a Deviation/Deferral/Waiver |

> **Note:** Some “R” changes (records) are not edits to canonical docs but are still governed changes because they create binding operational state.

#### 1.2.2 Severity classes and SemVer mapping (binding)

| Severity | SemVer increment | Trigger pattern |
|---|---|---|
| **PATCH** | `x.y.(z+1)` | editorial / clarification with no semantic change |
| **MINOR** | `x.(y+1).0` | backward-compatible additive change (new requirement, new field, new portal, expanded schema) |
| **MAJOR** | `(x+1).0.0` | breaking change (changed meaning, removed requirement, incompatible schema, altered trust boundary semantics) |

**Conservative rule:** If there is disagreement, the higher (more conservative) increment wins.

#### 1.2.3 Canonical set membership/identity changes are G:MAJOR by default (binding)

Changes that alter the **canonical governed set’s membership**, stable identity scheme, or canonical naming (e.g., adding/removing a canonical artifact, renaming `SR-*` IDs) MUST be treated as **G:MAJOR** by default.

---

## 2. Standard change workflow

### 2.1 Standard workflow (binding)

All governed changes MUST follow this workflow (activities; ordering is binding):

1) **Propose**  
   Create a change proposal with: scope/severity classification, rationale, impact assessment, and affected artifacts/records.

2) **Draft**  
   Produce draft artifact(s) / record(s) with updated metadata, explicit diffs, and updated cross-references.

3) **Verify**  
   Run the relevant oracle suites and assemble Evidence Bundles that demonstrate:
   - the change is internally consistent, and
   - verification rules were not silently weakened.

4) **Approve**  
   Cross the required portal(s) per §3, producing approval records that satisfy SR-CONTRACT C-TB-6.

5) **Register / Select**  
   - Record new governed artifact version(s) (SR-SPEC `GovernedArtifactVersionRecorded`).
   - If selection changes (`is_current=true`) are made, enforce:
     - at most one current per lineage,
     - human-only selection change for normative artifacts (SR-SPEC trust boundary),
     - portal linkage to the relevant approval (see §4.4),
     - staleness propagation (SR-SPEC §1.13; see §5.4 and §4.5).

6) **Communicate**  
   Update navigation/index (SR-README) if needed and record any binding interpretations as Decision Records (SR-SPEC §1.11) instead of “tribal knowledge”.

---

## 3. Portal routing and approval semantics

### 3.1 Portal set and portal functions (binding)

SOLVER-Ralph defines **portal functions** (semantic responsibilities) and a **minimum conformance set** of concrete portal IDs.

**Minimum required portal IDs (conformance):**
- `GovernanceChangePortal`
- `ReleaseApprovalPortal`
- `ExceptionApprovalPortal`

**Portal functions (semantic responsibilities):**
- `GovernanceChangePortal` (function): approval/ratification for changes that alter binding meaning or governed policy.
- `ReleaseApprovalPortal` (function): binding release decision for a freeze/baseline.
- `ExceptionApprovalPortal` (function): binding exceptions (waivers/deferrals/deviations) without bypassing integrity conditions.
- `OracleSuiteApproval` (function): approval for oracle suite registration and any required/advisory/environment-constraint changes.

**Implementation binding rule (binding):**
A portal function MAY be implemented as either:
1) a dedicated portal ID, or
2) a typed request within another portal ID,
provided that:
- binding records remain **human-only**, attributable, and durable,
- evidence references and suite hashes are explicitly bound,
- routing remains explicit and auditable (no implicit approvals).

For this baseline, the `OracleSuiteApproval` function is permitted to be implemented as typed requests within `GovernanceChangePortal` (see §3.2).

### 3.2 Portal routing matrix (binding)

If a change spans categories, **each applicable portal MUST be satisfied**.

| Change kind | Required portal(s) | Notes |
|---|---|---|
| Change to a normative artifact’s binding meaning | `GovernanceChangePortal` | Includes SR-CONTRACT, SR-SPEC, SR-DIRECTIVE, SR-TYPES, SR-CHANGE |
| Change to SR-INTENT (directional) or SR-README (index) | `GovernanceChangePortal` | Required if the artifact is canonical (auditability), even if not normative |
| Change to which version is `is_current=true` for any canonical artifact | `GovernanceChangePortal` | Human-approved selection change (especially for normative artifacts) |
| Oracle suite registration / required change / advisory change / environment-constraint change | `GovernanceChangePortal` *(OracleSuiteApproval function)* | Implemented as typed requests (e.g., `ORACLE_SUITE_*`) within GovernanceChangePortal; requires evidence refs + suite hash binding; MUST route to GovernanceChangePortal if it changes a canonical rule |
| Deviation / Deferral / Gate Waiver creation or modification | `ExceptionApprovalPortal` | Creates binding state; does not edit canonical governed docs |
| Candidate release (Shippable declaration) | `ReleaseApprovalPortal` | Freeze creation depends on this approval (see §5.3) |

### 3.3 Approval records (binding via SR-CONTRACT)

Approval records MUST satisfy **SR-CONTRACT C-TB-6** (minimum fields). SR-CHANGE adds the following binding clarifications for implementers:

- **`portal_id` MUST NOT be hardcoded.** It MUST reflect the actual portal crossed.
- **`evidence_refs[]` is required** (explicit empty list is allowed only when the portal’s evidence requirements allow it).
- **`exceptions_acknowledged[]` is required and MUST be explicit** (empty list allowed).

**For governance changes**, approvals SHOULD also include artifact diffs or references, but at minimum MUST include enough subject refs to uniquely bind the decision.

#### 3.3.1 Minimum approval record fields (binding)

| Field | Required | Meaning |
|---|---:|---|
| `approval_id` | Yes | stable identifier (`appr_<ULID>`) |
| `portal_id` | Yes | which portal was crossed |
| `decision` | Yes | `approved` or `rejected` |
| `subject_refs[]` | Yes | candidate refs and/or governed artifact refs being approved/rejected |
| `evidence_refs[]` | Yes | evidence bundle refs reviewed (explicit empty list allowed only when permissible) |
| `exceptions_acknowledged[]` | Yes | active exceptions acknowledged (MUST be explicit even if empty) |
| `approved_by` | Yes | stable HUMAN identity (C-TB-5) |
| `approved_at` | Yes | ISO 8601 timestamp |

**Governance-change subject refinement (binding):** If the approval is about governed artifacts, `subject_refs[]` MUST include, for each affected artifact: `{artifact_id, old_version, new_version, content_hash}` (inline or as a referenced structured attachment).

---

## 4. Drafts, versions, and “current” selection

### 4.1 Version strings (binding)

Versioned governance artifacts MUST use SemVer strings. Draft versions SHOULD use `-draft.N` suffix.

### 4.2 Draft mutability rule (binding)

**Draft does not mean “free to edit.”** Draft status means **not release-eligible**.

- **Unregistered drafts** (not recorded in the governed artifact registry; not referenced by evidence; not selected as current) MAY be edited freely.
- **Registered or referenced drafts** (recorded in the governed artifact registry **or** referenced by evidence **or** selected as current) MUST be treated as governed artifacts for change control purposes:
  - edits MUST go through the appropriate portal(s),
  - versions MUST advance (do not mutate a recorded content hash).

This rule exists to prevent “foundation corruption” via silent drift.

### 4.3 Status transitions (binding)

Permitted status transitions for governed artifacts:

- `draft` → `governed` (requires GovernanceChangePortal approval and registry update)
- `governed` → `superseded` (by recording a newer version and optionally selecting it current)
- `governed` → `deprecated` (planned removal; must be communicated)
- `deprecated` → `archived` (historical retention; not used for new work)

### 4.4 “Current” selection changes (binding)

Changing `is_current=true` for any canonical governed artifact is a governed change.

Additional trust-boundary constraint (from SR-SPEC): for any artifact where `normative_status = normative`, changing `is_current` MUST:

- be performed by a HUMAN actor,
- be linked to an Approval at GovernanceChangePortal,
- trigger staleness marking and re-evaluation routing for downstream dependents (SR-SPEC §1.13).



#### 4.4.1 `config.agent_definition` promotion/selection policy (binding)

`config.agent_definition` artifacts are directional configuration inputs for sandboxed Agent Workers. They are referenced from `IterationStarted.refs[]` for audit (typically with `rel=supported_by`), and therefore do not invalidate historical iterations.

**Policy:**

- `config.agent_definition` artifacts MUST be versioned (SemVer) and recorded as governed artifacts with stable ids.
- The system MAY maintain a default “current” selection for operational convenience (e.g., which agent definition a SYSTEM Loop Governor uses when assembling an Iteration Context Ref Set).
- Changing the default “current” selection for `config.agent_definition` is a governed change of **scope I (implementation configuration)**:
  - it MUST be performed by a HUMAN actor, and
  - it MUST be recorded (at minimum) as a Decision record with references to the promoted artifact version and supporting evidence (lint/schema checks).
- A `config.agent_definition` selection change MUST NOT be treated as a portal “approval” of plans.
- If a `config.agent_definition` change also changes ContextCompiler semantics (i.e., changes what counts as iteration context), it becomes a governance-impacting change and MUST go through GovernanceChangePortal approval.

### 4.5 Rollback semantics (binding)

A “rollback” is **not** rewriting history. A rollback is selecting a prior recorded version as `is_current=true`.

Rollback actions MUST:

- be treated as governed selection changes (GovernanceChangePortal),
- reference the reason (e.g., regression, ambiguity, unsafe change),
- trigger staleness propagation for any work (candidates, evidence, freezes) that depended on the rolled-back version.

If a rollback affects a previously frozen baseline, the affected baseline MUST be treated as **stale** and MUST NOT be used as the basis for a Shippable claim without re-evaluation (see §5.4).

---

## 5. Freeze policy (Shippable baselines)


#### 4.4.2 `config.gating_policy` promotion/selection policy (binding)

`config.gating_policy` artifacts are directional configuration used to enforce **human judgment hooks** (soft/hard/hybrid) per work unit. They SHOULD be referenced as audit provenance by default (`rel=supported_by`) so policy churn does not retroactively invalidate historical iterations.

**Selection scope:** `config.gating_policy` “current” pointers MAY be maintained per `work_unit`.

**Changing the default selection (binding):**
- MUST be performed by a HUMAN actor,
- SHOULD be linked to a Decision record explaining the change and scope,
- MUST be accompanied by evidence that the artifact passes schema/lint validation (recommended oracle: GatingPolicySchemaOracle).

**Governance-impacting changes (binding):**
- Any change that alters the meaning of gating modes, trigger semantics, or enforcement rules is a governance-impacting change and MUST be approved via GovernanceChangePortal (and version-bumped).

**Staleness note:** Because gating policy is a control-plane preference, it SHOULD NOT participate in blocking staleness propagation by default (use `supported_by` unless intentionally treated as a semantic dependency).

### 5.1 Freeze is required for Shippable (binding)

A **Freeze Record** is the binding baseline snapshot required to declare a candidate **Shippable** (SR-CONTRACT C-SHIP-1; SR-SPEC §1.12).

Freeze record schema and minimum fields are defined in SR-SPEC §1.12. SR-CHANGE adds the baseline content rules below.

### 5.2 Freeze manifest completeness (binding)

A Freeze Record’s `artifact_manifest[]` MUST include, at minimum:

- every artifact in the **canonical governed set** (SR-TYPES §4.1), each as `{artifact_id, version, content_hash}`:
  - `SR-TYPES`
  - `SR-README`
  - `SR-INTENT`
  - `SR-CONTRACT`
  - `SR-SPEC`
  - `SR-DIRECTIVE`
  - `SR-CHANGE`

### 5.3 Freeze coupling (binding)

A Freeze Record MUST NOT be created unless:

- a Release Approval exists for the candidate (ReleaseApprovalPortal), and
- the Release Approval’s `exceptions_acknowledged[]` includes **all** active exceptions listed in the Freeze Record’s `active_exceptions[]` (explicit acknowledgement).

### 5.4 Freeze and staleness (binding)

If any upstream dependency in a Freeze’s `artifact_manifest[]` changes **after** the freeze is created (including “current” selection changes), the system MUST:

- mark impacted dependents as **stale**, and
- MUST NOT treat the stale freeze as a valid basis for “Shippable” without re-evaluation and (typically) a new freeze.


**Audit-only reference staleness policy (binding):** Inputs referenced only as audit provenance (e.g., `rel=supported_by` in SR-SPEC) MUST NOT propagate blocking staleness by default. This prevents non-semantic churn (e.g., agent definition tweaks) from invalidating Shippable computations while preserving full auditability via immutable refs + hashes.


---

## 6. Exceptions: deviations, deferrals, gate waivers

### 6.1 Exceptions are records, not drift (binding)

Sustained mismatch between governed requirements and reality MUST be represented as binding record artifacts (Deviation / Deferral / Gate Waiver), not by silently changing definitions, skipping gates, or ignoring failures.

### 6.2 Exception kinds (binding)

| Kind | Purpose | Typical scope |
|---|---|---|
| **Deviation** | binding exception from a governed requirement | until resolved/superseded |
| **Deferral** | binding postponement of a requirement/deliverable | until target milestone/date |
| **Gate Waiver** | permission to treat a candidate as Verified-with-Exceptions despite FAIL of a required oracle | per-candidate by default |

### 6.3 Required fields (binding minimums)

Exception record schema is defined in SR-SPEC and scope constraints in SR-CONTRACT (C-EXC-4, C-EXC-5). At minimum, every exception record MUST include:

- `exception_id` (`exc_<ULID>`)
- `kind` (`DEVIATION` | `DEFERRAL` | `WAIVER`)
- `status` (`DRAFT` | `ACTIVE` | `RESOLVED` | `EXPIRED` | `SUPERSEDED`)
- `requirement_ref` (what requirement/gate/oracle is being excepted)
- `summary`, `risk`, `mitigation`
- `scope` (explicit and bounded)
- `resolution_criteria` + `resolution_owner`
- `expiry_or_review` (expiry date or required review date)
- `approved_by` (HUMAN) + `approved_at`
- `evidence_refs[]` (explicit empty list allowed)

### 6.4 Scope constraints (binding)

Exception scopes MUST be explicit and bounded per SR-CONTRACT C-EXC-5.

Unbounded “class-wide forever” exceptions are prohibited.

### 6.5 Exception visibility at approvals/freezes (binding)

Any approval or freeze that is materially affected by an active exception MUST:

- reference it in `exceptions_acknowledged[]` (approval) and/or `active_exceptions[]` (freeze),
- treat missing acknowledgement as an integrity failure (stop-the-line).

---

## 7. Oracle suite change constraints

### 7.1 Integrity conditions are not waivable (binding)

Oracle integrity conditions (at minimum: `ORACLE_TAMPER`, `ORACLE_ENV_MISMATCH`, `ORACLE_GAP`) MUST NOT be bypassed via Gate Waiver.

Resolution requires restart/re-run in compliance or an explicit, human-approved suite rebase/change via the appropriate portal(s).

### 7.2 No silent oracle weakening (binding)

The system MUST NOT silently:

- disable required oracles,
- downgrade FAIL to PASS,
- suppress failing categories,
- reduce oracle scope/coverage, or
- change required/advisory classification,

without explicit governed routing and explicit human approval recorded as a binding record.

A Gate Waiver is **not** silent weakening: it preserves execution and evidence while allowing progression under explicit acknowledgement.

---

## 8. Decision records

### 8.1 When decisions are required (binding)

A **Decision Record** is required when a human judgment is being used to:

- authorize continuation after a stop-the-line trigger that requires human arbitration (e.g., `REPEATED_FAILURE`, `BUDGET_EXHAUSTED`),
- authorize an oracle suite **rebase** (changing the verification basis mid-flight),
- interpret or operationalize an ambiguity in governed requirements (to prevent inconsistent future application),
- declare or override **precedent** for a recurring tradeoff,
- accept an emergency change as valid (post-hoc ratification; see §9.2).

### 8.2 Decision record required fields (binding)

Decision record schema is defined in SR-SPEC §1.11. At minimum, a Decision Record MUST include:

- `decision_id` (`dec_<ULID>`)
- `trigger`
- `scope` (bounded)
- `decision` + `rationale`
- `decided_by` (HUMAN) + `decided_at`
- `subject_refs[]`
- `evidence_refs[]` (explicit empty list allowed)
- `exceptions_acknowledged[]` (explicit empty list allowed)
- `is_precedent` (boolean)

### 8.3 Precedent decisions (binding)

If `is_precedent=true`:

- the decision MUST include explicit applicability conditions (when it applies / when it doesn’t),
- future similar situations SHOULD reference the precedent decision,
- deviating from precedent requires a new decision that explains why the precedent does not apply.

---

## 9. Emergency change procedure

### 9.1 Emergency definition (binding)

An emergency change is a governed change that must be applied before normal review cadence can occur (e.g., integrity/security fixes, safety-critical governance corrections).

Emergency procedure MUST NOT be used to bypass integrity requirements.

### 9.2 Emergency steps (binding)

1) **Declare emergency**  
   The proposer documents why normal procedure is insufficient.

2) **Expedited approval**  
   A HUMAN authorized for the relevant portal MAY approve with reduced review, **but still MUST** record:
   - portal id,
   - subject refs,
   - evidence refs (as applicable),
   - exceptions acknowledged (explicitly),
   - emergency justification.

3) **Post-hoc ratification**  
   Within **5 business days**, the emergency change MUST be re-reviewed under the standard workflow. If post-hoc review rejects the change, a rollback change request MUST be filed and tracked as governed work.

4) **Tagging**  
   Emergency approvals MUST set an explicit marker (e.g., `emergency: true`) in the approval record metadata.

---

## 10. Bootstrapping and self-amendment (SR-CHANGE)

SR-CHANGE is self-governing (`governed_by: ["SR-CHANGE"]`).

1) The first accepted SR-CHANGE version is created by a HUMAN and recorded in the governed artifact registry.
2) Once an SR-CHANGE version is selected as current, subsequent changes to SR-CHANGE MUST follow SR-CHANGE’s own procedure, including GovernanceChangePortal approval.
3) SR-CHANGE amendments MUST include explicit acknowledgement that the system is amending its own change rules.

---

## Appendix A: Recommended (non-binding) checklists

### A.1 Minimal change proposal packet

- Change classification (scope × severity) + rationale
- Affected artifacts/records (ids + current versions)
- Impact assessment (downstream dependents + staleness implications)
- Diffs / concrete “what changed”
- Verification plan + expected oracle suites
- Risks and mitigations
- Exceptions required (or explicit “none”)

### A.2 Minimal exception review packet

- Requirement refs + why exception is necessary
- Scope (as narrow as possible)
- Risk acceptance justification
- Resolution plan + owner + date
- Evidence refs (even if “none”)

### A.3 Separation of duties (recommended)

Projects SHOULD avoid having a single person act as the sole approver across all portals for extended periods.

If consolidation is unavoidable (small teams), approval records SHOULD explicitly note the consolidation rationale.

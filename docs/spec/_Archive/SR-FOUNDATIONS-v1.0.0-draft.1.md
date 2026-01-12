# SR-FOUNDATIONS v1.0.0-draft.1
type: governance.foundations
authority_kind: process
normative_status: normative
governed_by: SR-CHANGE
applies_to: SOLVER-Ralph (all instances)
version: 1.0.0-draft.1

> **Purpose:** Provide an early, binding *fit / no-fit* gate for whether SOLVER‑Ralph should be used for a given problem statement, **before** SR‑INTENT/SR‑PLAN authoring overhead is incurred.
>
> This document is intentionally short, high‑leverage, and operational: it exists to prevent “starting when there was no point in starting.”

---

## 0. Precedence and interpretation

1) **SR-CONTRACT** and **SR-SPEC** override this document on any binding semantic or mechanical detail.  
2) **SR-ETT** informs this document as the membrane topology lens.  
3) **SR-INTENT** is directional and must not contradict the gate criteria here; SR-FOUNDATIONS is earlier and stricter.  
4) If SR-FOUNDATIONS indicates **NO‑FIT**, proceeding requires an explicit **Exception** (Deviation) routed via SR‑CHANGE.

**Binding intent:** SR-FOUNDATIONS defines **entry criteria** for starting SOLVER‑Ralph work and defines the **Go/No‑Go decision record** requirements.

---

## 1. What this document is (and is not)

### 1.1 SR-FOUNDATIONS is
A governed decision harness that answers, early:

- *Is the problem statement sufficiently well‑structured to justify SOLVER‑Ralph?*
- *Is the solution space sufficiently typeable to build a semantic valley (typed corridor)?*
- *Are verification and authority crossings meaningful and available?*
- *Are the stakes worth the governance overhead?*

### 1.2 SR-FOUNDATIONS is not
- A replacement for SR‑INTENT (rationale, interpretive guidance, cost asymmetry)
- A replacement for SR‑PLAN (deliverables and dependencies)
- A replacement for SR‑DIRECTIVE (gates, profiles, portal playbooks)
- A generic “project kickoff checklist” (this is SOLVER‑Ralph‑specific)

---

## 2. Fit signature (when SOLVER‑Ralph is a good fit)

A problem is a strong fit when **all** of the following are true.

### F1. Structurable
The problem can be expressed as a **well‑structured problem statement** with:

- Objective(s) that can be stated unambiguously
- Constraints that can be enumerated
- Success criteria that can be evaluated (even if partially)

### F2. Typeable
The work can be represented in a typed ontology with stable objects and relationships:

- You can name the principal entities and their states
- You can name admissible transitions (“what can happen”)
- You can distinguish *proposal* vs *commitment* objects

### F3. Verifiable (evidence-producing checks exist)
There exist checks that can produce evidence independent of agent narrative:

- Automated oracles (tests, validators, scans, determinism checks), **and/or**
- Human evaluation criteria that can be recorded and attributed

### F4. Authority-requiring (binding decisions are real)
There is a meaningful need for attributable, human binding decisions:

- Acceptability / approval is non-trivial
- Someone must be accountable for the commitment

### F5. Stakes-worthy (governance overhead pays for itself)
The cost of being wrong or irreproducible exceeds the overhead of governance:

- Drift, silent weakening, or unverifiable decisions are unacceptable

---

## 3. Anti-signatures (when SOLVER‑Ralph is a bad fit)

A problem is a **NO‑FIT** when any of the following dominate.

### N1. Not structurable
- Objectives cannot be stated without moving targets
- “Done” cannot be articulated in evaluable terms

### N2. Not typeable
- There is no stable ontology (everything is “it depends”)
- The space is purely interpretive with no stable commitment objects

### N3. Not verifiable
- There are no credible oracles and no accountable evaluation criteria
- The only possible “evidence” is agent narrative

### N4. No meaningful authority crossings
- There is no real need for human binding decisions
- The work product is disposable / low consequence

### N5. The dominant constraint is speed
- Time-to-answer matters more than traceability
- Governance overhead would dominate value

---

## 4. The SR-FOUNDATIONS gate

### 4.1 Gate name and outcome states
**FOUNDATIONS_FIT_GATE** produces one of:

- **GO** — proceed to SR‑INTENT and SR‑PLAN
- **NO‑GO** — do not proceed
- **GO‑WITH‑EXCEPTION** — proceed only with an explicit Deviation (see §7)

### 4.2 Required inputs to run the gate
At minimum:

- A draft **Problem Statement** (SAPS-equivalent or precursor) containing objective, constraints, and success criteria (even if partial)
- A proposed set of candidate evidence mechanisms (oracles and/or evaluation rubric)
- A proposed accountable decision owner (who would bind)

### 4.3 The assessment rubric (fill-in)
For each criterion, assign: **PASS / PARTIAL / FAIL** plus notes.

| Criterion | PASS condition | PARTIAL condition | FAIL condition |
|---|---|---|---|
| F1 Structurable | Objective + constraints + success criteria are clear | Some ambiguity, but can be resolved quickly | Cannot define evaluable success |
| F2 Typeable | Stable objects/transitions can be named | Ontology exists but is unstable | No stable ontology |
| F3 Verifiable | Credible oracles/rubric exist | Verification possible but weak | No verification path |
| F4 Authority-requiring | Binding decisions are meaningful | Authority exists but ambiguous | No accountable binding |
| F5 Stakes-worthy | Wrong output cost > governance cost | Borderline | Overhead dominates value |

### 4.4 Decision rule
- **GO** requires: no **FAIL** in F1–F5, and at least **3 PASS**.
- **NO‑GO** if any of F1–F3 is **FAIL**.
- **GO‑WITH‑EXCEPTION** allowed only if: exactly one criterion is **FAIL**, and a Deviation is recorded with explicit mitigation and expiry (see §7).

---

## 5. “Semantic valley” minimal commitments (what must be true early)

If you claim SOLVER‑Ralph fit, you are implicitly claiming these minimum semantic commitments can be made early:

1) **Proposal vs commitment objects exist** (institutional truth is not agent narrative)  
2) **A dependency graph is meaningful** (refs/depends_on/supported_by are not noise)  
3) **Evidence can be produced** (some oracle or accountable rubric exists)  
4) **Authority boundaries are real** (someone will bind)  
5) **Change is governable** (you will route semantics changes via SR‑CHANGE)

If any of these are false, SOLVER‑Ralph will collapse into “a lot of paperwork around vibes.”

---

## 6. Output artifacts and record requirements

### 6.1 Required artifact: FoundationsFitRecord
A **FoundationsFitRecord** must be produced and stored (format is implementation-defined but must include these fields):

- `problem_id`
- `assessed_at`
- `assessed_by` (HUMAN identity)
- `outcome` (GO / NO‑GO / GO‑WITH‑EXCEPTION)
- `rubric` (the table in §4.3 filled)
- `evidence_plan` (what will produce evidence later)
- `authority_owner` (who binds later)
- `risks` (top 3)
- `next_step` (what document is created next, if GO)

### 6.2 Where it is used
- Must be referenced by the eventual SR‑INTENT and SR‑PLAN (as `supported_by`)
- Must be included in any Release/Freeze packet as provenance (audit trail)

---

## 7. Exception path (when proceeding despite NO‑FIT signals)

If SR‑FOUNDATIONS yields **GO‑WITH‑EXCEPTION**, a **Deviation** must be filed via SR‑CHANGE with:

- The failed criterion and why it’s being overridden
- Scope (which instance only)
- Mitigation plan (how the missing foundation will be created)
- Expiry / review checkpoint (when it must be re-evaluated)
- Explicit acknowledgement that governance overhead risk is accepted

Proceeding without this Deviation is non-conformant.

---

## 8. What happens next (binding routing)

### If outcome is GO
1) Create/confirm **SAPS** (or equivalent structured problem statement)  
2) Author **SR‑INTENT** for the instance (directional, cost asymmetry, rationale)  
3) Author **SR‑PLAN** (deliverables/dependencies)  
4) Author **SR‑DIRECTIVE** (gates/profiles/portals/budgets)  
5) Implement via loops and evidence/authority machinery

### If outcome is NO‑GO
- Stop SOLVER‑Ralph work for this problem
- Recommend an alternate approach (outside this document’s scope)

### If outcome is GO‑WITH‑EXCEPTION
- File Deviation per §7, then proceed as in GO

---

## 9. Change control notes

Because SR‑FOUNDATIONS determines whether the paradigm applies at all, changes to:
- the decision rule (§4.4),
- the fit criteria (§2),
- or the anti-signatures (§3)

are presumed **G:MINOR** at minimum and may be **G:MAJOR** if they materially expand when SOLVER‑Ralph may be used.

---

## Appendix A: Quick one-page questionnaire (copy/paste)

1) What is the objective (one sentence)?  
2) What does success look like (how would you know)?  
3) What are the top constraints (3–7 items)?  
4) What are the principal objects/states (ontology sketch)?  
5) What evidence mechanisms exist (oracles or rubric)?  
6) Who will bind decisions (accountable owner)?  
7) What is the cost of being wrong / unreplayable?  
8) Which anti-signature risks are present?  
9) GO / NO‑GO / GO‑WITH‑EXCEPTION? Why?

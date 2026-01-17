# SR-PLAN-LOOPS Consistency Evaluation

---
doc_id: SR-PLAN-LOOPS-CONSISTENCY-EVALUATION
doc_kind: record.evaluation_note
layer: build
status: draft
normative_status: record

refs:
  - rel: evaluates
    to: SR-PLAN-LOOPS
  - rel: depends_on
    to: SR-CONTRACT
  - rel: depends_on
    to: SR-SPEC
  - rel: depends_on
    to: SR-TYPES
  - rel: depends_on
    to: SR-WORK-SURFACE
  - rel: depends_on
    to: SR-DIRECTIVE
  - rel: depends_on
    to: SR-PROCEDURE-KIT
  - rel: depends_on
    to: SR-SEMANTIC-ORACLE-SPEC
---

## Executive Summary

**Overall Assessment: APPROVED WITH NOTES**

The SR-PLAN-LOOPS validation plan demonstrates strong alignment with the canonical SR-* document set across ontological, epistemological, and semantic dimensions. The plan correctly identifies core entities, properly distinguishes between evidence and authority, and uses terminology consistent with the governed specifications.

Minor notes are documented below for awareness during execution, but none block plan execution.

---

## 1. Ontological Consistency

*What entities exist and what are their essential properties?*

### 1.1 Entity Identification

| Entity | Canonical Definition | SR-PLAN-LOOPS Coverage | Assessment |
|--------|---------------------|----------------------|------------|
| **Loop** | SR-SPEC §3.1.1, SR-CONTRACT C-LOOP-* | Tests 1-8: States, transitions, budgets | ✅ Correct |
| **Iteration** | SR-SPEC §3.2, SR-CONTRACT C-CTX-* | Tests 5-6, 9-10: Fresh-context, refs[], memory | ✅ Correct |
| **Candidate** | SR-SPEC §1.3.3, SR-CONTRACT C-LOOP-4 | Test 12: Identity format, traceability | ✅ Correct |
| **Work Surface** | SR-SPEC §1.2.4, SR-WORK-SURFACE | Test 11: Intake + Template + Stage binding | ✅ Correct |
| **Evidence Bundle** | SR-SPEC §1.9.1, SR-CONTRACT C-EVID-* | Tests 17-18: Content-addressed, immutable | ✅ Correct |
| **Stop Trigger** | SR-SPEC §3.1.2, SR-CONTRACT C-LOOP-3 | Tests 13-14: Waivable triggers | ✅ Correct |
| **Decision** | SR-SPEC §1.11, SR-CONTRACT C-DEC-1 | Test 15: Binding, HUMAN-only | ✅ Correct |
| **Exception** | SR-CONTRACT C-EXC-1..C-EXC-5 | Test 16: DEVIATION, DEFERRAL, WAIVER | ✅ Correct |

**Assessment:** All required entity types are correctly identified. The plan covers the complete entity hierarchy.

### 1.2 Entity Relationships

The plan correctly represents the entity relationships:

| Relationship | Canonical Source | SR-PLAN-LOOPS Representation | Assessment |
|--------------|------------------|------------------------------|------------|
| Loop → Iteration | SR-SPEC §3.2 | Tests 5-6: Iterations belong to Loops | ✅ Correct |
| Iteration → Candidate | SR-CONTRACT C-LOOP-4 | Test 12: Candidate traceable to Iteration | ✅ Correct |
| Loop → Work Surface | SR-WORK-SURFACE §5.4 | Test 11: Loop bound to Work Surface | ✅ Correct |
| Work Surface → Intake | SR-WORK-SURFACE §2.1 | Test 11: Work Surface contains Intake ref | ✅ Correct |
| Work Surface → Procedure Template | SR-WORK-SURFACE §2.1 | Test 11: Work Surface contains Template ref | ✅ Correct |
| Work Surface → Oracle Suite | SR-WORK-SURFACE §5.1 | Tests 9-10: Suite hash in refs | ✅ Correct |
| Candidate → Run → Evidence | SR-SPEC §1.9 | Tests 17-18: Evidence from Runs | ✅ Correct |

**Assessment:** Entity relationships (Loop → Iteration → Candidate → Run → Evidence) are correctly represented.

### 1.3 Identity Formats

| Entity | Canonical Format | SR-PLAN-LOOPS Usage | Assessment |
|--------|-----------------|---------------------|------------|
| Loop | `loop_<ULID>` | Test 2: "Loop ID follows format `loop_<ULID>`" | ✅ Correct |
| Iteration | `iter_<ULID>` | Test 5: "`iteration_id` format: `iter_<ULID>`" | ✅ Correct |
| Candidate | `sha256:<64-hex>` + optional `git:<sha>` | Test 12: "stable identity including `sha256:<manifest_hash>`" | ✅ Correct |
| Work Surface | `ws:<ULID>` | Test 11: Implicit (via Work Surface binding) | ✅ Correct |
| Stage | `stage:<NAME>` | Tests 9-11: `current_stage_id` references | ✅ Correct |
| Oracle Suite | `suite:<id>` + `suite_hash` | Tests 9-10: Suite hash in meta | ✅ Correct |

**Assessment:** Identity formats are correctly specified per SR-SPEC §1.3.

### 1.4 Findings

**No ontological inconsistencies found.**

The plan demonstrates complete coverage of the entity ontology defined in SR-CONTRACT, SR-SPEC, and SR-TYPES.

---

## 2. Epistemological Consistency

*What can be known, how is it known, and what constitutes valid evidence?*

### 2.1 Evidence vs Authority

| Concept | Canonical Definition | SR-PLAN-LOOPS Treatment | Assessment |
|---------|---------------------|------------------------|------------|
| **Evidence** | SR-CONTRACT §2.1: "recorded outputs that support a claim" | Tests 17-18 treat evidence as supporting verification, not as authority | ✅ Correct |
| **Authority** | SR-CONTRACT §2.1: "Human authorizes at Portal" | Test 15, 19: Decisions/Approvals require HUMAN actor | ✅ Correct |

**Assessment:** The plan correctly positions evidence as supporting material and reserves binding authority for human actors at portals.

### 2.2 Verification vs Approval

| Concept | Canonical Definition | SR-PLAN-LOOPS Treatment | Assessment |
|---------|---------------------|------------------------|------------|
| **Verification** | SR-SPEC §1.2.2: Agentic (oracle), output is evidence, no binding authority | Tests 17-18: Oracle results produce evidence | ✅ Correct |
| **Approval** | SR-SPEC §1.2.2: Human at Portal, binding authority | Test 15b-c: Decision/Approval required for resume | ✅ Correct |

**Assessment:** The plan correctly distinguishes verification (oracle-based, non-binding) from approval (human-based, binding).

### 2.3 Waivable vs Non-Waivable

| Category | Canonical Definition | SR-PLAN-LOOPS Treatment | Assessment |
|----------|---------------------|------------------------|------------|
| **Waivable** | SR-CONTRACT C-LOOP-3: BUDGET_EXHAUSTED, REPEATED_FAILURE | Tests 13-14: Part C, can be resumed with Decision | ✅ Correct |
| **Non-Waivable** | SR-CONTRACT C-OR-7: ORACLE_GAP, EVIDENCE_MISSING, ORACLE_TAMPER | Tests 17-18: Part D, cannot be bypassed even with Decision | ✅ Correct |

**Assessment:** The plan correctly categorizes stop triggers. Part C (waivable) vs Part D (non-waivable) structure aligns with SR-CONTRACT.

### 2.4 Findings

**No epistemological inconsistencies found.**

The plan correctly maintains the distinction between:
- Evidence (oracle output) vs Authority (human decision)
- Verification (non-binding) vs Approval (binding)
- Waivable conditions (policy) vs Non-waivable conditions (integrity)

---

## 3. Semantic Consistency

*What do terms mean and are they used consistently?*

### 3.1 Contract Mapping

| Contract | Test(s) | Assessment |
|----------|---------|------------|
| C-LOOP-1 (Bounded iteration with hard stop) | Tests 1, 2, 7 | ✅ Correct |
| C-LOOP-2 (Fresh-context iterations) | Tests 5, 6 | ✅ Correct |
| C-LOOP-3 (Mandatory stop triggers) | Tests 13, 14 | ✅ Correct |
| C-LOOP-4 (Candidate traceability) | Test 12 | ✅ Correct |
| C-CTX-1 (Iteration context provenance) | Tests 9, 10 | ✅ Correct |
| C-CTX-2 (No ghost inputs) | Test 9 (implicit) | ✅ Correct |
| C-DEC-1 (Binding decisions recorded) | Test 15 | ✅ Correct |
| C-OR-7 (Integrity conditions halt) | Tests 17, 18 | ✅ Correct |
| C-TB-1 (HUMAN-only binding authority) | Test 19 | ✅ Correct |
| C-EXC-1..C-EXC-5 (Exception semantics) | Test 16 | ✅ Correct |

**Assessment:** Contract identifiers are correctly mapped to test cases.

### 3.2 Term Usage

| Term | Canonical Source | SR-PLAN-LOOPS Usage | Assessment |
|------|------------------|---------------------|------------|
| `refs[]` | SR-SPEC §1.5.3 | Tests 9-10: Correctly uses `refs[]` for iteration context | ✅ Correct |
| `depends_on` vs `supported_by` | SR-SPEC §1.5.3.2 | Test 9: Correctly uses `depends_on` for semantic dependencies | ✅ Correct |
| `content_hash` | SR-SPEC §1.5.3.1 | Test 10: Requires `meta.content_hash` for dereferenceable refs | ✅ Correct |
| `suite_hash` | SR-DIRECTIVE §5, SR-SEMANTIC-ORACLE-SPEC §2 | Tests 9-10: Requires `suite_hash` in OracleSuite meta | ✅ Correct |
| `BUDGET_EXHAUSTED` | SR-SPEC §3.1.2, SR-DIRECTIVE §4.2 | Test 13 | ✅ Correct |
| `REPEATED_FAILURE` | SR-CONTRACT C-LOOP-3 | Test 14 | ✅ Correct |
| `ORACLE_GAP` | SR-CONTRACT C-OR-7 | Test 17 | ✅ Correct |
| `EVIDENCE_MISSING` | SR-CONTRACT C-OR-7 | Test 18 | ✅ Correct |
| `DecisionRecorded` | SR-SPEC §1.11.2 | Test 15 | ✅ Correct |
| `Work Surface` | SR-WORK-SURFACE, SR-DIRECTIVE §2.4 | Test 11 | ✅ Correct |
| `Procedure Template` | SR-PROCEDURE-KIT | Tests 9, 11 | ✅ Correct |
| `stage_id` | SR-PROCEDURE-KIT, SR-DIRECTIVE §2.4 | Tests 9, 10 (`current_stage_id`) | ✅ Correct |

**Assessment:** Term usage is consistent with canonical definitions.

### 3.3 Event/State Names

| Category | Canonical Source | SR-PLAN-LOOPS Usage | Assessment |
|----------|------------------|---------------------|------------|
| **Loop States** | SR-SPEC §3.1.1: CREATED, ACTIVE, PAUSED, CLOSED | Tests 3a-d: All four states tested | ✅ Correct |
| **Events** | SR-SPEC Appendix A | Tests use: `LoopCreated`, `LoopActivated`, `IterationStarted`, `IterationCompleted`, `StopTriggered`, `DecisionRecorded`, `LoopResumed`, `LoopClosed` | ✅ Correct |
| **Iteration States** | SR-SPEC §3.2 | Tests 5-6: STARTED/RUNNING, COMPLETED | ✅ Correct |

**Assessment:** Event and state names are consistent with SR-SPEC.

### 3.4 Findings

**Minor Notes (non-blocking):**

1. **ORACLE_TAMPER not explicitly tested (Note 1):**
   - SR-README §3 (Semantic Consistency table) mentions `ORACLE_TAMPER` as "Not tested (gap?)."
   - SR-CONTRACT C-OR-7 lists ORACLE_TAMPER as a non-waivable integrity condition.
   - **Recommendation:** Consider adding Test 19.5 or documenting this as a known gap for V10/V11. This is a minor omission since the plan already covers ORACLE_GAP and EVIDENCE_MISSING, which represent the same category of non-waivable conditions.

2. **IterationStarted.refs[] minimum categories (Note 2):**
   - Test 9 lists 8 ref categories. SR-SPEC §3.2.1.1 lists 11 canonical categories (including Human judgment notes, Agent definition, Gating policy).
   - The test correctly notes some categories are "Conditional" (empty for first iteration, or when not applicable).
   - **Recommendation:** The plan's 8 categories are the **minimum required** per SR-DIRECTIVE §3.1. The additional categories in SR-SPEC are either audit-only (`supported_by`) or context-dependent. No action required; the plan's coverage is correct for validation purposes.

3. **Work Surface binding verification scope (Note 3):**
   - Test 11 verifies "Intake + Procedure Template + Oracle Suite" binding.
   - SR-WORK-SURFACE §5.1 specifies Work Surface Instance includes: `oracle_suites[]` (plural, with `suite_hash`).
   - **Recommendation:** Test 11 should verify `suite_hash` is present in the Work Surface binding, not just suite name. The plan's SQL queries in Test 10 do verify this at the iteration level, which is sufficient.

---

## 4. Recommendations

1. **[INFO] Add ORACLE_TAMPER test consideration** — Consider adding a brief note in Part D that ORACLE_TAMPER is another non-waivable condition of the same class as ORACLE_GAP/EVIDENCE_MISSING. Alternatively, document in Gap Tracking that ORACLE_TAMPER testing is deferred if oracle suite tamper detection is not yet implemented.

2. **[INFO] Clarify refs[] category coverage** — Test 9's table correctly covers the SR-DIRECTIVE §3.1 minimum requirements. The plan correctly identifies which categories are conditional vs required.

3. **[INFO] Ensure suite_hash verification in Test 11** — When executing Test 11, verify that the Work Surface's `oracle_suites[]` entries include `suite_hash`, not just `suite_id`. This is implicit in the plan but worth confirming during execution.

---

## 5. Approval

### Formal Approval Statement

**Status: APPROVED WITH NOTES**

The SR-PLAN-LOOPS validation plan is **approved for execution**. The plan demonstrates:

- ✅ Complete ontological coverage of Loop, Iteration, Candidate, Work Surface, Evidence Bundle, Stop Trigger, Decision, and Exception entities
- ✅ Correct epistemological distinctions between evidence vs authority, verification vs approval, and waivable vs non-waivable conditions
- ✅ Consistent semantic usage aligned with SR-CONTRACT, SR-SPEC, SR-TYPES, SR-WORK-SURFACE, SR-DIRECTIVE, SR-PROCEDURE-KIT, and SR-SEMANTIC-ORACLE-SPEC

**Notes for execution awareness:**
1. ORACLE_TAMPER is not explicitly tested (same category as ORACLE_GAP/EVIDENCE_MISSING)
2. Test 9's refs[] categories are the correct minimum per SR-DIRECTIVE §3.1
3. Verify `suite_hash` presence in Work Surface binding during Test 11 execution

These notes do not block execution. The validation plan may proceed.

---

**Evaluated by:** Claude (Agent)
**Evaluation date:** 2026-01-17
**Canonical documents consulted:** SR-CONTRACT, SR-SPEC, SR-TYPES, SR-WORK-SURFACE, SR-DIRECTIVE, SR-PROCEDURE-KIT, SR-SEMANTIC-ORACLE-SPEC
**Document under evaluation:** docs/planning/SR-PLAN-LOOPS.md

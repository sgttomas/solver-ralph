---
doc_id: SR-README
doc_kind: governance.readme
layer: build
status: draft
normative_status: index

refs:
  - rel: governed_by
    to: SR-CHANGE
---

# SR-README

Tasks are no longer assigned by SR-PLAN because the build out phase is complete.  See below for the details of your current assignment.

Start by reviewing docs/charter/SR-CHARTER.md

The project documentation constitutes a total development plan and specification with detailed instructions on types and contracts.  Always read the SR-* files that appear related to the task before going to read the code files.  Documentation leads development for this project.  Documentation is how you know your ontology, epistemology, and semantics.

Once you have read the appropriate project docs, then do whatever deliverables and tasks you think should be done next.

Your task is done when there are no more deliverables to be assigned, which means all tests have passed and gates cleared.

You should push on to resolve any findings and consult the docs/ frequently for guidance and direction from the SR-* documents, several of which are normative.  They are typed documents and structured to aid in navigation.

You should git add && commit && push after completing each deliverable.  You can remain on the same branch throughout this development session.

ALWAYS refer to the project docs/*/SR-* for the authoritative coding architecture, plan, and semantics.  Understand the full set of docs/ and refer to the applicable SR-* document instead of making assumptions.

When troubleshooting, refer to the appropriate SR-* documents.

---

## Canonical document paths

Canonical index for the SR-* document set.

| doc_id | Folder | Purpose |
|--------|--------|---------|
| SR-CHARTER | `charter/` | Project scope and priorities |
| SR-CONTRACT | `platform/` | Binding invariants |
| SR-SPEC | `platform/` | Platform mechanics |
| SR-TYPES | `platform/` | Type registry and schemas |
| SR-WORK-SURFACE | `platform/` | Work surface definitions |
| SR-PROCEDURE-KIT | `platform/` | Procedure templates |
| SR-SEMANTIC-ORACLE-SPEC | `platform/` | Semantic oracle interface |
| SR-EVENT-MANAGER | `platform/` | Event projections spec |
| SR-REPLAY-PROOF | `platform/` | Determinism proof (C-EVT-7) |
| SR-AGENT-WORKER-CONTRACT | `platform/` | Agent/worker contract |
| SR-INTENT | `platform/` | Design rationale |
| SR-CHANGE | `build-governance/` | Change control process |
| SR-AGENTS | `build-governance/` | Agent actor model |
| SR-TASKS | `build-governance/` | Task assignment |
| SR-EXCEPTIONS | `build-governance/` | Exception ledger |
| SR-PLAN | `program/` | Build plan instance |
| SR-DIRECTIVE | `program/` | Execution policy |
| SR-TEMPLATES | `platform/` | User configuration registry |
| SR-README | `charter/` | This index |


---


## Current Status: Loop Validation Plan Ready for Review

**Branch 0 Acceptance:** COMPLETE (V9-4)
**Current Focus:** SR-PLAN-LOOPS validation plan evaluation

### Recent Development

A comprehensive Loop Functionality Validation Plan (`docs/planning/SR-PLAN-LOOPS.md`) has been revised to address coherence gaps identified during review. The plan now covers:

| Part | Focus | Tests | Contracts |
|------|-------|-------|-----------|
| **A** | Happy Path — Loop Basics | 1-8 | C-LOOP-1, C-LOOP-2, State Machine |
| **B** | Context & Work Surface Binding | 9-12 | C-CTX-1, C-CTX-2, C-LOOP-4 |
| **C** | Failure Modes (Waivable) | 13-16 | C-LOOP-3, C-DEC-1, C-EXC-* |
| **D** | Integrity Conditions (Non-Waivable) | 17-19 | C-OR-7, C-TB-1 |

**Key additions in revision:**
- Part B: Iteration context refs validation (minimum categories, meta requirements)
- Part B: Work Surface binding verification (Intake + Template + Oracle Suite)
- Part B: Candidate traceability (C-LOOP-4)
- Part D: Non-waivable integrity conditions (ORACLE_GAP, EVIDENCE_MISSING)
- Part D: Actor kind enforcement (HUMAN-only for binding authority)

---

## Next Instance Prompt: SR-PLAN-LOOPS Consistency Evaluation

> **Session Type:** Evaluation (read-only analysis)
> **Estimated Effort:** 1 session
> **Prerequisite:** None (standalone evaluation task)

### Assignment

**Systematic Consistency Evaluation** — Evaluate the revised `SR-PLAN-LOOPS.md` against the canonical SR-* documents for consistency in ontology, epistemology, and semantics.

### Context

The SR-PLAN-LOOPS validation plan has been revised to address coherence gaps. Before execution, the plan requires formal evaluation to ensure it correctly interprets and tests the contracts defined in the canonical document set.

**Documents revised:** `docs/planning/SR-PLAN-LOOPS.md`

### Evaluation Framework

You must evaluate SR-PLAN-LOOPS for consistency across three dimensions:

#### 1. Ontological Consistency

*What entities exist and what are their essential properties?*

Evaluate whether SR-PLAN-LOOPS correctly identifies and uses:

| Entity | Canonical Definition | Check |
|--------|---------------------|-------|
| Loop | SR-SPEC §1.2, SR-CONTRACT C-LOOP-* | States, transitions, budgets |
| Iteration | SR-SPEC §3.2, SR-CONTRACT C-CTX-* | Fresh-context, refs[], memory discipline |
| Candidate | SR-SPEC §1.3.3, SR-CONTRACT C-LOOP-4 | Identity format, traceability |
| Work Surface | SR-SPEC §1.2.4, SR-WORK-SURFACE | Binding (Intake + Template + Stage) |
| Evidence Bundle | SR-SPEC §1.2, SR-CONTRACT C-EVID-* | Content-addressed, immutable |
| Stop Trigger | SR-SPEC §3.1.2, SR-CONTRACT C-LOOP-3 | Waivable vs non-waivable |
| Decision | SR-SPEC §1.8, SR-CONTRACT C-DEC-1 | Binding, HUMAN-only |
| Exception | SR-CONTRACT C-EXC-1..C-EXC-5 | DEVIATION, DEFERRAL, WAIVER |

**Questions to answer:**
- Does the plan correctly identify all entity types involved in Loop validation?
- Are entity relationships (Loop→Iteration→Candidate→Run→Evidence) correctly represented?
- Are entity identity formats (ULID prefixes, sha256 hashes) correctly specified?

#### 2. Epistemological Consistency

*What can be known, how is it known, and what constitutes valid evidence?*

Evaluate whether SR-PLAN-LOOPS correctly distinguishes:

| Concept | Canonical Definition | Check |
|---------|---------------------|-------|
| Evidence vs Authority | SR-CONTRACT C-TB-* | Evidence supports; authority binds |
| Verification vs Approval | SR-SPEC §1.2.2 | Agentic vs Human |
| Waivable vs Non-waivable | SR-CONTRACT C-OR-7 | Policy vs Integrity |
| Proposal vs Commitment | SR-SPEC §1.2.1 | Draft vs Binding object |

**Questions to answer:**
- Do the tests correctly distinguish evidence (oracle output) from authority (human approval)?
- Are verification steps correctly positioned as supporting (not substituting for) approval?
- Does the plan correctly test that non-waivable conditions cannot be bypassed?
- Are actor kinds (HUMAN, SYSTEM, AGENT) correctly constrained in test expectations?

#### 3. Semantic Consistency

*What do terms mean and are they used consistently?*

Cross-reference term usage in SR-PLAN-LOOPS against canonical definitions:

| Term | Canonical Source | Check Usage In |
|------|------------------|----------------|
| `refs[]` | SR-SPEC §1.5.3, SR-DIRECTIVE §3.1 | Tests 9-10 (minimum categories, meta) |
| `depends_on` vs `supported_by` | SR-SPEC §1.5.3.2 | Test 9 (rel values) |
| `content_hash` | SR-SPEC §1.5.3.1 | Test 10 (meta requirements) |
| `suite_hash` | SR-DIRECTIVE §3.1, SR-SEMANTIC-ORACLE-SPEC | Test 10 (oracle suite meta) |
| `IterationStarted` required refs | SR-DIRECTIVE §3.1 | Test 9 (minimum categories) |
| `BUDGET_EXHAUSTED` | SR-SPEC §3.1.2, SR-DIRECTIVE §4 | Test 13 |
| `REPEATED_FAILURE` | SR-SPEC §3.1.2, SR-DIRECTIVE §4 | Test 14 |
| `ORACLE_GAP` | SR-CONTRACT C-OR-7 | Test 17 |
| `EVIDENCE_MISSING` | SR-CONTRACT C-OR-7 | Test 18 |
| `ORACLE_TAMPER` | SR-CONTRACT C-OR-7 | Not tested (gap?) |
| `DecisionRecorded` | SR-SPEC §1.5, §3.1.2 | Test 15 |
| `Work Surface` | SR-WORK-SURFACE, SR-DIRECTIVE §2.4 | Test 11 |
| `Procedure Template` | SR-PROCEDURE-KIT | Tests 9, 11 |
| `stage_id` | SR-PROCEDURE-KIT, SR-DIRECTIVE §2.4 | Tests 9, 10 (current_stage_id) |

**Questions to answer:**
- Are contract identifiers (C-LOOP-1, C-CTX-1, etc.) correctly mapped to test cases?
- Are event type names consistent with SR-SPEC §1.5 / Appendix A?
- Are state names (CREATED, ACTIVE, PAUSED, CLOSED) consistent with SR-SPEC §3.1.1?
- Are stop trigger names consistent with SR-CONTRACT C-LOOP-3 and C-OR-7?
- Are the required `IterationStarted.refs[]` categories consistent with SR-DIRECTIVE §3.1?
- Is the Work Surface binding (Intake + Template + Stage + Oracle Suite) consistent with SR-DIRECTIVE §2.4?

### Canonical Documents to Consult

Read these documents in the order shown. The first tier is mandatory; the second tier provides supporting context.

#### Tier 1: Primary (Mandatory)

| # | Document | Key Sections | Relevance |
|---|----------|--------------|-----------|
| 1 | `docs/platform/SR-CONTRACT.md` | C-LOOP-*, C-CTX-*, C-TB-*, C-OR-7, C-EXC-*, C-DEC-1 | Binding invariants being tested |
| 2 | `docs/platform/SR-SPEC.md` | §1.2 (terminology), §1.5 (events/refs), §3.1 (Loop state machine), §3.2 (iteration memory) | Platform mechanics |
| 3 | `docs/platform/SR-TYPES.md` | §4.3 (platform domain types), identity formats | Type definitions |
| 4 | `docs/platform/SR-WORK-SURFACE.md` | Work Surface binding semantics | Tests 9-12 context |
| 5 | `docs/program/SR-DIRECTIVE.md` | §2.1 (canonical loop), §2.4 (semantic loop), §3.1 (required refs) | Execution model, refs discipline |
| 6 | `docs/platform/SR-PROCEDURE-KIT.md` | Stage definitions, gate criteria | Procedure template semantics |

#### Tier 2: Supporting (Consult as needed)

| Document | When to Consult |
|----------|-----------------|
| `docs/platform/SR-SEMANTIC-ORACLE-SPEC.md` | Tests 17-18 (oracle integrity conditions) |
| `docs/platform/SR-AGENT-WORKER-CONTRACT.md` | Test 19 (actor kind constraints) |
| `docs/platform/SR-EVENT-MANAGER.md` | If evaluating projection/eligibility semantics |

#### Document Under Evaluation

| Document | Purpose |
|----------|---------|
| `docs/planning/SR-PLAN-LOOPS.md` | The validation plan being evaluated |

### Deliverable

Produce a formal evaluation report at `docs/reviews/SR-PLAN-LOOPS-CONSISTENCY-EVALUATION.md` with:

```markdown
# SR-PLAN-LOOPS Consistency Evaluation

## Executive Summary
[Overall assessment: APPROVED / APPROVED WITH NOTES / REVISION REQUIRED]

## 1. Ontological Consistency
### 1.1 Entity Identification
[Assessment of entity coverage]

### 1.2 Entity Relationships
[Assessment of relationship correctness]

### 1.3 Identity Formats
[Assessment of ID format consistency]

### 1.4 Findings
[List any ontological inconsistencies]

## 2. Epistemological Consistency
### 2.1 Evidence vs Authority
[Assessment]

### 2.2 Verification vs Approval
[Assessment]

### 2.3 Waivable vs Non-Waivable
[Assessment]

### 2.4 Findings
[List any epistemological inconsistencies]

## 3. Semantic Consistency
### 3.1 Contract Mapping
[Assessment of C-* to test mapping]

### 3.2 Term Usage
[Assessment of term consistency]

### 3.3 Event/State Names
[Assessment of naming consistency]

### 3.4 Findings
[List any semantic inconsistencies]

## 4. Recommendations
[Ordered list of any changes needed]

## 5. Approval
[Formal approval statement or revision requirements]
```

### Evaluation Criteria

| Rating | Criteria |
|--------|----------|
| **APPROVED** | No inconsistencies found; plan is ready for execution |
| **APPROVED WITH NOTES** | Minor inconsistencies that don't block execution; document for awareness |
| **REVISION REQUIRED** | Significant inconsistencies that must be corrected before execution |

### Key Files

#### Canonical Documents

| File | Purpose |
|------|---------|
| `docs/planning/SR-PLAN-LOOPS.md` | Plan under evaluation |
| `docs/platform/SR-CONTRACT.md` | Contract definitions (C-* invariants) |
| `docs/platform/SR-SPEC.md` | Platform mechanics (state machines, events, refs) |
| `docs/platform/SR-TYPES.md` | Type definitions and identity formats |
| `docs/platform/SR-WORK-SURFACE.md` | Work Surface binding spec |
| `docs/program/SR-DIRECTIVE.md` | Execution model, required refs discipline |
| `docs/platform/SR-PROCEDURE-KIT.md` | Procedure templates, stage definitions |
| `docs/platform/SR-SEMANTIC-ORACLE-SPEC.md` | Oracle suite semantics (for integrity tests) |

#### Implementation Reference (for verification)

| File | Purpose |
|------|---------|
| `crates/sr-api/src/main.rs` | Route definitions |
| `crates/sr-api/src/handlers/loops.rs` | Loop handler implementations |
| `crates/sr-api/src/handlers/iterations.rs` | Iteration handler implementations |
| `crates/sr-api/src/handlers/work_surfaces.rs` | Work Surface handler implementations |
| `crates/sr-domain/src/events.rs` | Event type definitions |
| `crates/sr-domain/src/state_machines.rs` | State transition logic |

### Do NOT

- Execute the validation plan (that's a separate session)
- Modify SR-PLAN-LOOPS unless revision is required
- Skip reading the canonical documents
- Make assumptions about contract meanings without verification

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

### Feature Implementation Plans

The `docs/planning/` folder contains feature-specific implementation plans that are subordinate to SR-PLAN. These plans detail specific feature implementations and are not permanent governance documents — they become historical artifacts once implementation is complete.

| doc_id | Status | Purpose |
|--------|--------|---------|
| SR-PLAN-GAP-ANALYSIS | **living** | Deliverable status tracking & roadmap |
| SR-PLAN-V7 | **complete** | MVP Stabilization & Attachment Foundation |
| SR-PLAN-V6 | **complete** | UI Integration for MVP Workflow (V6-1, V6-2, V6-3 complete) |
| SR-PLAN-V5 | **complete** | Semantic Ralph Loop End-to-End Integration (Phases 5a-5d) |
| SR-PLAN-V4 | **complete** | Work Surface Composition (Phase 4) — All phases complete |
| SR-PLAN-V3 | **complete** | Intakes & References implementation (Phases 0-3) |
| SR-PLAN-V2 | superseded | Intakes & References draft (10 unresolved issues) |

### Roadmap (from Gap Analysis)

Per `SR-PLAN-GAP-ANALYSIS.md`, the path to Milestone 1 completion:

| Plan | Scope | Key Deliverables | Status |
|------|-------|------------------|--------|
| SR-PLAN-V7 | Stabilization & Attachments | Tests, UX, `record.attachment` | **Complete** |
| SR-PLAN-V8 | Oracle Runner & Semantic Suites | D-24, D-25, D-27, D-39 | **Under Review** |
| SR-PLAN-V9 | Semantic Worker & Branch 0 | D-23, D-41, D-36 | Proposed |

**Milestone 1 (MVP) projected completion:** After V9

---

## SR-PLAN-V8 Implementation Status (AMENDED 2026-01-16)

| Phase | Status | Description | Amendment |
|-------|--------|-------------|-----------|
| V8-1: Oracle Suite Registry | ⏳ Pending | Extract port trait, add PostgreSQL persistence | A-4 |
| V8-2: Event-Driven Worker | ⏳ Pending | Worker subscribes to `RunStarted` events | A-1 |
| V8-3: Integrity Checks | ⏳ Pending | TAMPER/GAP/FLAKE/ENV_MISMATCH detection | — |
| V8-4: Core Oracle Suite | ⏳ Pending | Build/unit/schema/lint oracles + container | — |
| V8-5: Semantic Oracles | ⏳ Pending | Use existing types, focus on container packaging | A-3 |

**SR-PLAN-V8 has been amended following coherence assessment. Ready for philosophical review.**

### Amendments Applied

| ID | Issue | Resolution |
|----|-------|------------|
| A-1 | V8-2 assumed direct API call | Use Event-Driven Worker pattern |
| A-2 | Type relationship unclear | `OracleSuiteDefinition` = config, `OracleSuiteRecord` = stored entity |
| A-3 | Semantic types already exist | Use `sr-domain/src/semantic_oracle.rs` |
| A-4 | Registry partially implemented | Extract port from existing `OracleSuiteRegistry` |

**Revised Effort:** 7-10 sessions (was 7-9)

---

## Previous Session Summary (V8 Coherence Assessment)

**Session Goal:** Review SR-PLAN-V8 for coherence with existing codebase

### What Was Accomplished

1. **Performed codebase coherence review:**
   - Verified existing `OracleRunner` trait and `PodmanOracleRunner` implementation
   - Discovered `POST /runs` uses event sourcing (creates `RunStarted` event, does NOT call oracle runner)
   - Found existing `OracleSuiteRegistry` struct with 6 working API endpoints
   - Found comprehensive semantic types in `sr-domain/src/semantic_oracle.rs` (~1024 lines)

2. **Identified 4 required amendments:**

   | Amendment | Finding |
   |-----------|---------|
   | A-1 | V8-2 must use Event-Driven Worker pattern (not direct API call) |
   | A-2 | Clarify `OracleSuiteDefinition` vs `OracleSuiteRecord` relationship |
   | A-3 | V8-5 should use existing semantic types, not create new ones |
   | A-4 | V8-1 should extract port trait from existing registry implementation |

3. **Revised SR-PLAN-V8:**
   - Added Amendments Summary section
   - Rewrote V8-2 for event-driven architecture
   - Updated V8-5 to reference existing types
   - Revised effort estimates (7-10 sessions)
   - Added critical files appendix

### Verdict

**COHERENT WITH AMENDMENTS** — Plan is sound but required 4 specific corrections before implementation.

### Files Modified

| File | Changes |
|------|---------|
| `docs/planning/SR-PLAN-V8.md` | Added amendments, rewrote V8-2, updated V8-5, revised estimates |

### Estimated Effort (Revised)

| Phase | Sessions | Change |
|-------|----------|--------|
| V8-1 | 1 | — |
| V8-2 | 2-3 | +1 (event-driven complexity) |
| V8-3 | 1-2 | — |
| V8-4 | 2 | — |
| V8-5 | 1-2 | -1 (types exist) |
| **Total** | **7-10** | +1 |

---

## Next Instance Prompt: Philosophical Coherence Review of SR-PLAN-V8

### Context

SR-PLAN-V8 has been amended following codebase coherence review. Before implementation begins, we need a philosophical coherence review to ensure the plan aligns with the canonical SR-* documents in terms of ontology, epistemology, and semantics.

### Current State

- Branch: `solver-ralph-8` (V7 complete, V8 plan amended)
- SR-PLAN-V7: **Complete** (all phases)
- SR-PLAN-V8: **Amended** (pending philosophical review)

### Assignment

**Review SR-PLAN-V8 for philosophical coherence with canonical documents**

Evaluate SR-PLAN-V8 (`docs/planning/SR-PLAN-V8.md`) against the canonical governance documents along three philosophical dimensions.

### Canonical Documents to Review

| Document | Path | Purpose |
|----------|------|---------|
| SR-SPEC | `docs/platform/SR-SPEC.md` | Platform specification |
| SR-CONTRACT | `docs/platform/SR-CONTRACT.md` | Invariant contracts |
| SR-CHARTER | `docs/charter/SR-CHARTER.md` | Project charter |
| SR-DIRECTIVE | `docs/program/SR-DIRECTIVE.md` | Operational directives |
| SR-SEMANTIC-ORACLE-SPEC | `docs/platform/SR-SEMANTIC-ORACLE-SPEC.md` | Semantic oracle spec |
| SR-TYPES | `docs/platform/SR-TYPES.md` | Type definitions |

### Evaluation Dimensions

**Ontology (What exists):**
- Are the entities proposed in V8 (OracleSuiteRecord, OracleExecutionWorker, IntegrityCondition, etc.) consistent with the domain model defined in SR-SPEC?
- Do the new event types (OracleExecutionStarted, OracleExecutionCompleted) align with the event sourcing ontology?
- Is there entity duplication or confusion between sr-ports, sr-domain, and sr-adapters?
- Do the proposed entities respect the authority boundaries defined in SR-DIRECTIVE?

**Epistemology (How we know):**
- Does the evidence model in V8 satisfy C-EVID-* contracts for what constitutes proof?
- Are the integrity conditions (TAMPER/GAP/FLAKE/ENV_MISMATCH) sufficient to establish trust?
- Does the event-driven worker pattern preserve the auditability guarantees?
- Can verification gates derive decisions solely from recorded evidence (no out-of-band knowledge)?
- Does the plan respect SR-DIRECTIVE's requirements for what must be recorded vs. computed?

**Semantics (What it means):**
- Do the semantic oracle outputs (residual/coverage/violations) correctly operationalize the meaning-matrix concept from SR-SEMANTIC-ORACLE-SPEC?
- Is the relationship between `DecisionStatus` (Pass/Fail/Indeterminate/Waived) and gate outcomes well-defined?
- Are the integrity condition names and their blocking behaviors semantically precise?
- Do the oracle profiles (GOV-CORE, STRICT-CORE, STRICT-FULL) align with SR-DIRECTIVE's verification requirements?

### Deliverables

1. **Coherence Report** — Structured analysis identifying:
   - Alignments (where V8 correctly implements canonical requirements)
   - Tensions (where V8 may conflict with or underspecify canonical requirements)
   - Gaps (canonical requirements not addressed by V8)

2. **SR-README Update** — Revise this file with:
   - V8 status update (philosophical review complete)
   - Summary of findings (if noteworthy)
   - Next prompt for V8 implementation

### Constraints

- **Do NOT implement code** — this is a review and documentation task only
- Focus on conceptual coherence, not implementation details
- Be precise about which SR-* section supports or contradicts each claim
- If you find the plan philosophically sound, say so briefly and move on

### First Actions

1. Read `docs/planning/SR-PLAN-V8.md` (the amended plan)
2. Read `docs/platform/SR-SPEC.md` (domain model, event sourcing ontology)
3. Read `docs/platform/SR-CONTRACT.md` (C-OR-*, C-EVID-*, C-VER-* invariants)
4. Read `docs/program/SR-DIRECTIVE.md` (operational policies, authority boundaries)
5. Read `docs/platform/SR-SEMANTIC-ORACLE-SPEC.md` (semantic oracle requirements)


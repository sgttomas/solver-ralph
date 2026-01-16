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

Tasks are no longer assigned by SR-PLAN because the build out phase is complete.  See below for the Comprehensive Implementation Plan for the next phase of build out and implementation.

Begin your task assignment by reading SR-CHARTER.  The project documentation constitutes a total development plan and specification with detailed instructions on types and contracts.  Always read the SR-* files that appear related to the task before going to read the code files.  Documentation leads development for this project.  Documentation is how you know your ontology, epistemology, and semantics.

Once you have read the appropriate project docs, then do whatever deliverables and tasks you think should be done next.

Your task is done when there are no more deliverables to be assigned, which means all tests have passed and gates cleared.

You should push on to resolve any findings and consult the docs/ frequently for guidance and direction from the SR-* documents, several of which are normative.  They are typed documents and structured to aid in navigation.

You should git add && commit && push after completing each deliverable.  You can remain on the same branch throughout this development session.

If you cannot pass the tests for that deliverable then you must summarize what you did during that development session, delete the previous message where it says "Development History Summary for this Deliveralbe" and then append your new message including how to identify the task that was being worked on when the next instance of yourself begins the next iteration.

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
| SR-PLAN-V3 | active | Intakes & References implementation (Phases 0-3) |
| SR-PLAN-V2 | superseded | Intakes & References draft (10 unresolved issues) |

---

## Prompt for Next Instance: Intakes & References Implementation

### Task

Continue implementing the Intakes & References infrastructure per **SR-PLAN-V3.md**.

### Current Progress

| Phase | Status | Description |
|-------|--------|-------------|
| Phase 0a | **Complete** | Core Infrastructure — TypedRef module, Intake domain, database migrations, event definitions |
| Phase 0b | **Complete** | Intake API — Intake handler with CRUD + lifecycle operations |
| Phase 0c | Pending | References API — References browser backend |
| Phase 1 | Pending | UI Structure — Sidebar and route reorganization |
| Phase 2 | Pending | Intakes UI — Full Intake CRUD UI |
| Phase 3 | Pending | References UI — References browser UI |

### Next Step

Begin **Phase 0c: References API** — implement the References browser backend per SR-PLAN-V3 §2.

### Required Reading

1. **`docs/planning/SR-PLAN-V3.md`** §2 — References API specification
2. **`docs/platform/SR-SPEC.md`** §1.5.3 — TypedRef schema and RefRelation enum
3. Review existing implementation:
   - `crates/sr-domain/src/refs.rs` — StrongTypedRef, RefKind, RefRelation
   - `crates/sr-api/src/handlers/intakes.rs` — Pattern for handler implementation

### Verification

After Phase 0c, run:
- `cargo build` — No errors
- `cargo test` — All pass

### Constraints

- **Backend-first** — Complete Phase 0 before UI phases
- **Follow SR-PLAN-V3** — All types, schemas, and APIs are specified
- **Stop after each phase** — Wait for instructions before proceeding

---

## Summary of Previous Development Iterations

### Session: 2026-01-16 (Part 3) — Phase 0a & 0b Implementation

**Objective:** Implement Phases 0a and 0b of SR-PLAN-V3 (Core Infrastructure and Intake API).

**Work Performed:**

1. **Phase 0a: Core Infrastructure** (Complete)
   - Created `crates/sr-domain/src/refs.rs` — StrongTypedRef with RefKind, RefRelation enums
   - Created `crates/sr-domain/src/intake.rs` — IntakeUlidId, IntakeStatus enum, ManagedIntake, lifecycle events
   - Created `migrations/005_intakes.sql` — intake_status enum, work_kind enum, proj.intakes table
   - Updated `crates/sr-domain/src/events.rs` — Added StreamKind::Intake and new EventTypes

2. **Phase 0b: Intake API** (Complete)
   - Created `crates/sr-api/src/handlers/intakes.rs` (~940 lines) — Full CRUD + lifecycle handler
   - Updated `crates/sr-api/src/main.rs` — Added intake routes
   - Updated `crates/sr-api/src/handlers/mod.rs` — Registered intakes module
   - Updated `crates/sr-adapters/src/projections.rs` — Added intake event projection handlers
   - Updated `crates/sr-adapters/src/postgres.rs` — Added StreamKind::Intake handling

3. **Documentation Updates**
   - Updated `docs/charter/SR-README.md` — Added Feature Implementation Plans section
   - Updated `docs/program/SR-PLAN.md` — Added `implemented_by` ref to SR-PLAN-V3

**Verification:** `cargo build` and `cargo test --workspace` pass (30 tests in sr-api).

---

### Session: 2026-01-16 (Part 2) — SR-PLAN-V3 Finalization

**Objective:** Resolve all 10 issues from SR-PLAN-V2 and produce an implementation-ready V3 plan.

**Work Performed:**

1. **Document Review**
   - Read SR-PLAN-V2 and identified all 10 `[REVIEW NOTE]` issues
   - Read SR-CONTRACT for binding invariants (C-EVT-1, C-EVID-6, commitment objects)
   - Read SR-SPEC for TypedRef schema (§1.5.3), RefRelation enum, event patterns
   - Read SR-TYPES for status enums (§3.1) and type taxonomy
   - Read SR-WORK-SURFACE for Intake schema (§3.1)
   - Read SR-PROCEDURE-KIT for procedure template structure
   - Reviewed existing `handlers/templates.rs` (Issue #9)

2. **Issue Resolutions**
   - **Issue #1:** Unified InputRef and TypedRef into single schema
   - **Issue #2:** Defined complete event model (IntakeCreated, IntakeUpdated, IntakeActivated, IntakeArchived, IntakeForked)
   - **Issue #3:** Created full PostgreSQL schema for `proj.intakes`
   - **Issue #4:** Aligned status terminology with SR-TYPES §3.1 (draft/active/archived → draft/governed/archived)
   - **Issue #5:** Clarified by-hash retrieval returns all statuses per C-EVID-6
   - **Issue #6:** Standardized all References API responses to `{ refs, total, page, page_size }`
   - **Issue #7:** Verified RefRelation enum is complete per SR-SPEC §1.5.3
   - **Issue #8:** Split Phase 0 into 0a (Core Infrastructure), 0b (Intake API), 0c (References API)
   - **Issue #9:** Confirmed templates.rs handles schemas, not runtime; new handler needed
   - **Issue #10:** Added Agent Definitions and Gating Policies to ref categories

3. **Artifacts Created**
   - `docs/planning/SR-PLAN-V3.md` — Implementation-ready plan with all issues resolved
   - Updated `docs/charter/SR-README.md` — Updated prompt for implementation phase

**No code was modified.** This was a planning-only session.

---

### Session: 2026-01-16 (Part 1) — Initial Planning

**Objective:** Develop a comprehensive implementation plan for Intakes UI/API and References browser.

**Work Performed:**
- Context gathering from SR-* governance docs
- Codebase exploration (found missing backend endpoints)
- V1 and V2 plan development
- Identified 10 issues requiring resolution

**Artifacts Created:**
- `docs/planning/SR-PLAN-V2.md` — Plan with embedded `[REVIEW NOTE]` markers

**User Decisions Established:**
- Intakes are a top-level nav item (separate from References)
- Show all intakes regardless of status, with filter
- No backward compatibility needed — clean implementation
- Backend-first (Phase 0 before UI)
- "References" is acceptable user-facing term (renamed from "Context")
- "Prompts" stays as-is (lower priority)


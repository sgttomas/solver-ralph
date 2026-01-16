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

## Prompt for Next Instance: Intakes & References Plan Review

### Task

Review and finalize the implementation plan for **Intakes** and **References** UI/API infrastructure. Produce a V3 plan that resolves all identified issues and is implementation-ready.

### Required Reading (in order)

1. **This file** (SR-README) - Project context and canonical document index
2. **`docs/planning/SR-PLAN-V2.md`** - The current plan with embedded review notes marking 10 issues to resolve
3. **Core governance docs** (for verification):
   - `docs/platform/SR-CONTRACT.md` - Binding invariants (highest precedence)
   - `docs/platform/SR-SPEC.md` - Platform mechanics, TypedRef schema (§1.5.3), iteration context refs (§3.2.1.1)
   - `docs/platform/SR-TYPES.md` - Type taxonomy (`record.intake`, status enums)
   - `docs/platform/SR-WORK-SURFACE.md` - Intake schema definition (§3)

### Deliverable

Write `docs/planning/SR-PLAN-V3.md` that:

1. Resolves all 10 issues marked as `[REVIEW NOTE]` in SR-PLAN-V2
2. Includes complete Rust structs, PostgreSQL schemas, event definitions, API specs with JSON examples
3. Splits Phase 0 into manageable sub-phases
4. Documents any intentional deviations from SR-* specifications

### Constraints

- **Do NOT begin implementation** - This is planning only
- **Backend-first** - Phase 0 before UI phases
- **Intakes are top-level nav** - Separate from References
- **No backward compatibility** - Clean implementation aligned with specs

### Success Criteria

V3 is complete when all items in the "Review Issues Summary" table in SR-PLAN-V2 are resolved and the plan includes verification checklists for each phase.

---

## Summary of Previous Development Iteration

### Session: 2026-01-16 — Intakes & References Implementation Planning

**Objective:** Develop a comprehensive implementation plan for Intakes UI/API and References browser, aligned with SR-* governance framework.

**Work Performed:**

1. **Context Gathering**
   - Read SR-README and SR-CHARTER for project orientation
   - Read SR-WORK-SURFACE for authoritative Intake schema (§3)
   - Read SR-CONTRACT for binding invariants (commitment objects, event attribution, typed refs)
   - Read SR-SPEC for platform mechanics (TypedRef schema §1.5.3, iteration context refs §3.2.1.1)
   - Read SR-TYPES for type taxonomy (`record.intake`, status enums)
   - Read SR-PROCEDURE-KIT for procedure template structure
   - Read SR-SEMANTIC-ORACLE-SPEC for oracle interface

2. **Codebase Exploration**
   - Discovered `Context.tsx` exists with Intakes tab, but **NO backend `/api/v1/context` endpoint**
   - Found existing `IntakeDetail.tsx` (read-only)
   - Confirmed no `handlers/intakes.rs` or `handlers/references.rs` exist
   - Identified that `Sidebar.tsx` shows "Prompts" not "Intakes"

3. **V1 Plan Development**
   - Identified need for Phase 0 (backend) before UI work
   - Proposed renaming Context → References
   - Proposed Intakes as top-level nav item

4. **V2 Plan Development**
   - Added detailed Rust struct definitions
   - Added API endpoint specifications
   - Added UI component layouts
   - Aligned terminology with SR-* documents

5. **Ontological/Epistemological/Semantic Review**
   - Identified 10 issues requiring resolution:
     - High priority: Unify ref schemas, add event specs, add PostgreSQL schema, align status terminology
     - Medium priority: Clarify by-hash retrieval, standardize API responses, complete RefRelation enum
     - Lower priority: Split Phase 0, review existing templates.rs, add missing ref categories

6. **Artifacts Created**
   - `docs/planning/SR-PLAN-V2.md` — Full implementation plan with embedded `[REVIEW NOTE]` markers for each issue
   - Updated `docs/charter/SR-README.md` — Added prompt for next instance

**User Decisions Established:**
- Intakes are a top-level nav item (separate from References)
- Show all intakes regardless of status, with filter
- No backward compatibility needed — clean implementation
- Backend-first (Phase 0 before UI)
- "References" is acceptable user-facing term (renamed from "Context")
- "Prompts" stays as-is (lower priority)

**Next Steps:**
The next instance should read SR-PLAN-V2.md, verify the 10 identified issues against the governance docs, and produce SR-PLAN-V3.md with all issues resolved and implementation-ready detail.

**No code was modified.** This was a planning-only session.


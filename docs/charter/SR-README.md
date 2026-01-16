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
| SR-PLAN-V6 | **draft** | UI Integration for MVP Workflow (pending research & validation) |
| SR-PLAN-V5 | **complete** | Semantic Ralph Loop End-to-End Integration (Phases 5a-5d) |
| SR-PLAN-V4 | **complete** | Work Surface Composition (Phase 4) — All phases complete |
| SR-PLAN-V3 | **complete** | Intakes & References implementation (Phases 0-3) |
| SR-PLAN-V2 | superseded | Intakes & References draft (10 unresolved issues) |

---

## SR-PLAN-V6 Implementation Status

details to be added here

---

## Next Instance Prompt: Coherence & Consistency Check for SR-PLAN-V6

### Context

SR-PLAN-V6 has been validated and refined with design decisions resolved (§3.6-3.8). Before implementation begins, the plan requires a **coherence and consistency check** against the canonical SR-* documents to ensure alignment with the project's ontology, epistemology, and semantics.

### Current State

- Branch: `solver-ralph-7`
- SR-PLAN-V6 status: "Ready for Implementation" with design resolved
- Design decisions documented in §3.6 (SYSTEM mediation), §3.7 (directive_ref), §3.8 (idempotency)
- Implementation pseudocode provided for Phase V6-1

### Assignment

Perform a coherence and consistency check of `docs/planning/SR-PLAN-V6.md` against the canonical SR-* documents. This is a **verification and alignment task**, not an implementation task.

### Canonical Documents to Review

Per SR-CHARTER precedence order, check SR-PLAN-V6 against:

**Platform-definition (meaning, invariants, mechanics):**
1. `docs/platform/SR-CONTRACT.md` — Binding invariants (C-TB-*, C-CTX-*, etc.)
2. `docs/platform/SR-SPEC.md` — Platform mechanics (§2.2 SYSTEM actor requirements)
3. `docs/platform/SR-TYPES.md` — Type registry and schemas
4. `docs/platform/SR-WORK-SURFACE.md` — Work surface definitions
5. `docs/platform/SR-PROCEDURE-KIT.md` — Procedure templates

**Build-execution (agent behavior, process):**
1. `docs/build-governance/SR-AGENTS.md` — Agent actor model
2. `docs/program/SR-DIRECTIVE.md` — Execution policy

### Verification Tasks

**1. Ontological Consistency**

Verify SR-PLAN-V6 uses canonical terminology correctly:
- [ ] "Work Surface" usage matches SR-WORK-SURFACE definition
- [ ] "Loop" and "Iteration" usage matches SR-SPEC §2/§3
- [ ] "SYSTEM actor" semantics match SR-SPEC §2.2
- [ ] "directive_ref" semantics match SR-TYPES
- [ ] Event types (LoopCreated, IterationStarted) match SR-SPEC Appendix A

**2. Epistemological Consistency**

Verify knowledge claims and evidence requirements:
- [ ] §3.6 SYSTEM mediation claim is supported by SR-SPEC §2.2
- [ ] §3.7 directive_ref default is consistent with SR-DIRECTIVE
- [ ] Idempotency design doesn't violate event sourcing invariants (C-EVT-*)
- [ ] Audit trail preservation (HUMAN on LoopCreated) satisfies C-TB-* requirements

**3. Semantic Consistency**

Verify the proposed implementation preserves platform semantics:
- [ ] `start_work_surface` handler respects Work Surface lifecycle (SR-WORK-SURFACE §5)
- [ ] Loop creation with `work_surface_id` matches SR-SPEC §2.3.1 binding semantics
- [ ] Iteration context inheritance matches C-CTX-1, C-CTX-2
- [ ] Portal/approval requirements not bypassed (C-TB-3)

**4. Contract Compliance**

Check against specific SR-CONTRACT invariants:
- [ ] C-TB-3: Portal crossings produce Approval records (not bypassed by `/start`)
- [ ] C-CTX-1: All refs are content-addressed
- [ ] C-CTX-2: All context derivable from IterationStarted.refs[]
- [ ] C-EVT-*: Event sourcing invariants preserved

### Deliverables

1. **Coherence Report** appended to SR-README.md documenting:
   - Findings from each verification task
   - Any inconsistencies discovered
   - Recommended corrections (if any)
   - Final assessment: PASS / PASS_WITH_NOTES / FAIL

2. **SR-PLAN-V6 updates** (if inconsistencies found):
   - Correct any terminology misuse
   - Add missing contract references
   - Align pseudocode with canonical patterns

3. **SR-CHANGE entry** (if SR-PLAN-V6 is modified):
   - Log any changes per SR-CHANGE process

### Guidelines

- This is **verification**, not implementation
- Read canonical documents carefully before making judgments
- When in doubt, cite the specific section of the canonical document
- If SR-PLAN-V6 proposes something that contradicts a canonical document, flag it — don't assume the plan is correct
- The goal is confidence that implementation will produce semantically correct behavior

### Verification Complete When

- [ ] All ontological checks passed
- [ ] All epistemological checks passed
- [ ] All semantic checks passed
- [ ] All contract compliance checks passed
- [ ] Coherence report written
- [ ] SR-PLAN-V6 status confirmed or updated
- [ ] Ready for implementation (or blocked with documented issues)


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
| SR-PLAN-V6 | **ready** | UI Integration for MVP Workflow (coherence verified, ready for implementation) |
| SR-PLAN-V5 | **complete** | Semantic Ralph Loop End-to-End Integration (Phases 5a-5d) |
| SR-PLAN-V4 | **complete** | Work Surface Composition (Phase 4) — All phases complete |
| SR-PLAN-V3 | **complete** | Intakes & References implementation (Phases 0-3) |
| SR-PLAN-V2 | superseded | Intakes & References draft (10 unresolved issues) |

---

## SR-PLAN-V6 Implementation Status

| Phase | Status | Description |
|-------|--------|-------------|
| Coherence Review | ✅ Complete | `docs/reviews/SR-PLAN-V6-COHERENCE-REVIEW.md` — PASS_WITH_NOTES |
| V6-1: Backend | ⏳ Pending | Add `POST /work-surfaces/{id}/start` endpoint |
| V6-2: Frontend | ⏳ Pending | Wire wizard to call `/start` after creation |
| V6-3: E2E Verification | ⏳ Pending | Document and verify complete human workflow |

---

## Next Instance Prompt: Implement SR-PLAN-V6 Phase V6-1 (Backend)

### Context

SR-PLAN-V6 has passed coherence and consistency review (see `docs/reviews/SR-PLAN-V6-COHERENCE-REVIEW.md`). The plan is verified to be consistent with canonical SR-* documents and ready for implementation.

### Current State

- Branch: `solver-ralph-7`
- SR-PLAN-V6 status: "Ready for Implementation" — coherence verified
- Coherence review: PASS_WITH_NOTES (minor terminology variances follow existing codebase patterns)
- Implementation pseudocode provided in SR-PLAN-V6 §4 (Phase V6-1)

### Assignment

Implement **Phase V6-1: Backend — Start Work Endpoint** as specified in `docs/planning/SR-PLAN-V6.md` §4.

This phase adds the `POST /work-surfaces/{id}/start` endpoint that creates a Loop, activates it, and starts an Iteration as SYSTEM actor — enabling the UI wizard to fully orchestrate work surface creation.

### Files to Modify

Per SR-PLAN-V6 §4 (Phase V6-1):

| File | Action | Description |
|------|--------|-------------|
| `crates/sr-adapters/src/projections.rs` | EDIT | Add `get_loop_by_work_unit` method for idempotency |
| `crates/sr-api/src/handlers/work_surfaces.rs` | EDIT | Add `start_work_surface` handler |
| `crates/sr-api/src/main.rs` | EDIT | Register `/work-surfaces/{id}/start` route |

### Implementation Requirements

Per SR-PLAN-V6 §3.6-3.8 and §4:

1. **SYSTEM Actor Mediation (§3.6)**
   - `IterationStarted` event MUST use `actor_kind: ActorKind::System`
   - Use `actor_id: "system:work-surface-start"` as the system identity
   - HUMAN actor is recorded on `LoopCreated` event (audit trail)

2. **Directive Ref Default (§3.7)**
   - Use default directive_ref: `{kind: "doc", id: "SR-DIRECTIVE", rel: "governs", meta: {}}`
   - This follows existing codebase pattern in `prompt_loop.rs`

3. **Idempotency (§3.8)**
   - Query for existing Loop bound to `work_unit_id` before creating
   - If Loop exists and is ACTIVE with iteration, return existing IDs (`already_started: true`)
   - If Loop exists but not ACTIVE, activate and start iteration
   - If no Loop exists, create → activate → start iteration

4. **Iteration Context Refs**
   - Populate `refs[]` from Work Surface context via `fetch_work_surface_refs`
   - Must include: Intake, Procedure Template, oracle suites, governing artifacts
   - Per C-CTX-1/C-CTX-2: all context derivable from `IterationStarted.refs[]`

### Acceptance Criteria

From SR-PLAN-V6 §4 (Phase V6-1):

- [ ] `POST /work-surfaces/{id}/start` creates Loop, activates, starts Iteration
- [ ] Iteration is emitted with `actor_kind=SYSTEM`, `actor_id="system:work-surface-start"`
- [ ] Loop uses default `directive_ref` pointing to SR-DIRECTIVE
- [ ] Returns `{ work_surface_id, loop_id, iteration_id, already_started }`
- [ ] 412 if Work Surface not active
- [ ] Idempotent: returns existing IDs with `already_started: true` if called again
- [ ] HUMAN actor recorded on LoopCreated event (audit trail)

### Reference Documents

- `docs/planning/SR-PLAN-V6.md` — Implementation plan with pseudocode
- `docs/reviews/SR-PLAN-V6-COHERENCE-REVIEW.md` — Coherence verification
- `docs/platform/SR-SPEC.md` §2.2 — SYSTEM actor requirements
- `docs/platform/SR-SPEC.md` §2.3.1 — Loop creation with work_unit binding
- `docs/platform/SR-WORK-SURFACE.md` §5.4 — Loop ↔ Work Surface binding semantics

### Reference Code

- `crates/sr-api/src/handlers/loops.rs` — Loop creation/activation patterns
- `crates/sr-api/src/handlers/iterations.rs` — Iteration start patterns
- `crates/sr-domain/src/governor.rs` — SYSTEM actor pattern (line ~729)
- `crates/sr-domain/src/prompt_loop.rs` — directive_ref default pattern (line ~92)

### Guidelines

- Follow existing codebase patterns for consistency
- Use the pseudocode in SR-PLAN-V6 §4 as implementation guide
- Run `cargo build --package sr-api` and `cargo test --package sr-api` to verify
- Commit after implementation with descriptive message

### Phase Complete When

- [ ] `get_loop_by_work_unit` projection method added
- [ ] `start_work_surface` handler implemented
- [ ] Route registered in main.rs
- [ ] All acceptance criteria met
- [ ] `cargo build` passes
- [ ] `cargo test` passes
- [ ] Committed and pushed


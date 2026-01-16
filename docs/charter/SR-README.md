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
| SR-PLAN-V7 | **ready** | MVP Stabilization & Attachment Foundation |
| SR-PLAN-V6 | **complete** | UI Integration for MVP Workflow (V6-1, V6-2, V6-3 complete) |
| SR-PLAN-V5 | **complete** | Semantic Ralph Loop End-to-End Integration (Phases 5a-5d) |
| SR-PLAN-V4 | **complete** | Work Surface Composition (Phase 4) — All phases complete |
| SR-PLAN-V3 | **complete** | Intakes & References implementation (Phases 0-3) |
| SR-PLAN-V2 | superseded | Intakes & References draft (10 unresolved issues) |

---

## SR-PLAN-V7 Implementation Status

| Phase | Status | Description |
|-------|--------|-------------|
| Coherence Review | ✅ Complete | Ontological review completed, plan amended |
| V7-1: Integration Tests | ⏳ Pending | Integration tests for `/start` endpoint |
| V7-2: Error Handling | ⏳ Pending | Toast notifications, loading states, retry logic |
| V7-3: Attachment Backend | ⏳ Pending | `POST /attachments` endpoint |
| V7-4: Attachment Frontend | ⏳ Pending | AttachmentUploader, AttachmentPreview components |
| V7-5: Multiple Iterations | ⏳ Pending | Iteration history and new iteration support |

---

## Previous Session Summary (V7 Coherence Review)

**Session Goal:** Evaluate SR-PLAN-V7 for ontological, epistemological, and semantic consistency with canonical SR-* documents before implementation.

### What Was Accomplished

1. **Read and analyzed canonical documents:**
   - SR-README (assignment and context)
   - SR-PLAN-V7 (original plan)
   - SR-SPEC (platform mechanics, especially §1.9 Evidence bundle model)
   - SR-SEMANTIC-ORACLE-SPEC (oracle interface requirements)
   - SR-CONTRACT (binding invariants, especially C-EVID-1, C-EVID-2, C-VER-1)

2. **Identified ontological gap in original SR-PLAN-V7:**
   - Original plan proposed `artifact_type: "evidence.human_upload"` for human file uploads
   - This conflated two ontologically distinct concepts:
     - **Evidence Bundles** (`domain.evidence_bundle`): Oracle output with full manifests per C-EVID-1
     - **Human uploads**: Supporting files that are NOT oracle output
   - Human-uploaded files cannot satisfy C-EVID-1 requirements (no candidate ref, no oracle suite hash, no per-oracle results)
   - Using "evidence" terminology for human uploads would violate C-VER-1 semantics

3. **Produced coherence review with verdict: COHERENT WITH NOTES**
   - Ontology: Gap identified (Evidence Bundle definition)
   - Epistemology: Consistent (no authority leakage)
   - Semantics: Minor naming clarification needed

4. **Amended SR-PLAN-V7 with ontological corrections:**
   - Introduced `record.attachment` as distinct artifact type
   - Changed endpoint from `POST /evidence/files` to `POST /attachments`
   - Changed `artifact_type` from `evidence.human_upload` to `record.attachment`
   - Renamed UI components: `EvidenceUploader.tsx` → `AttachmentUploader.tsx`
   - Added clear semantic distinction in UI between Evidence Bundles (oracle) and Attachments (human)
   - Added Appendix C documenting the amendment rationale

### Key Ontological Distinction (Amendment)

| Concept | Type Key | Source | Satisfies C-VER-1? |
|---------|----------|--------|-------------------|
| **Evidence Bundle** | `domain.evidence_bundle` | Oracle output | ✅ Yes |
| **Attachment** | `record.attachment` | Human upload | ❌ No |

This preserves SR-CONTRACT's epistemological clarity: only oracle-produced Evidence Bundles can satisfy verification gates.

### Files Modified

| File | Change |
|------|--------|
| `docs/planning/SR-PLAN-V7.md` | Comprehensive amendment with ontological corrections |

### Contract Compliance After Amendment

| Contract | Status |
|----------|--------|
| C-EVID-1 | ✅ N/A (attachments are not Evidence Bundles) |
| C-EVID-2 | ✅ Satisfied (same storage semantics for immutability) |
| C-VER-1 | ✅ Clear (only oracle Evidence satisfies verification) |
| C-CTX-1 | ✅ Satisfied (V7-5 iteration creation remains SYSTEM-mediated) |

---

## Next Instance Prompt: Execute SR-PLAN-V7 Phase V7-1

### Context

SR-PLAN-V7 has been reviewed for coherence and **amended** to correct an ontological gap. The plan now correctly distinguishes:
- **Evidence Bundles** (`domain.evidence_bundle`): Oracle-generated, satisfies C-VER-1
- **Attachments** (`record.attachment`): Human-uploaded supporting files, does NOT satisfy C-VER-1

The coherence review is complete. The plan is ready for implementation.

### Current State

- Branch: `solver-ralph-7`
- SR-PLAN-V6: **Complete** (MVP UI Integration)
- SR-PLAN-V7: **Ready for Implementation (Amended)**
- Coherence Review: **Complete** — ontological corrections applied

### Assignment

**Execute SR-PLAN-V7 Phase V7-1: Integration Tests for `/start` Endpoint**

Create integration tests to ensure the `/start` orchestration endpoint is regression-proof before extending the platform.

### Deliverables

| File | Action | Description |
|------|--------|-------------|
| `crates/sr-api/tests/integration/work_surface_start_test.rs` | CREATE | Integration tests for `/start` |

### Test Cases to Implement

| Test | Description | Validates |
|------|-------------|-----------|
| `start_happy_path` | Active Work Surface → Loop created → activated → Iteration started | Core flow |
| `start_idempotent` | Call twice → second returns `already_started: true` | Idempotency |
| `start_rejects_inactive` | Non-active Work Surface → HTTP 412 | Precondition |
| `start_activates_created_loop` | Existing CREATED Loop → activates and starts | Edge case |
| `start_human_on_loop_created` | Verify `LoopCreated` has HUMAN actor | Audit trail |
| `start_system_on_iteration` | Verify `IterationStarted` has SYSTEM actor | C-CTX-1 compliance |

### Acceptance Criteria

- [ ] All 6 test cases pass
- [ ] `cargo test --package sr-api` passes
- [ ] Tests cover the acceptance criteria from SR-PLAN-V6 §4

### First Action

1. Read `docs/planning/SR-PLAN-V7.md` §V7-1 for detailed requirements
2. Examine existing test patterns in `crates/sr-api/tests/`
3. Create `work_surface_start_test.rs` with the 6 test cases

### Reference Documents

- `docs/planning/SR-PLAN-V7.md` — Implementation plan (amended)
- `docs/platform/SR-SPEC.md` — §2.3.12 Work Surfaces API
- `docs/platform/SR-WORK-SURFACE.md` — §5.5 Starting work via /start endpoint
- `crates/sr-api/src/handlers/work_surfaces.rs` — Implementation to test

### Important Notes

- The `/start` endpoint was implemented in SR-PLAN-V6 Phase V6-1
- Actor mediation pattern: `LoopCreated` uses HUMAN actor, `IterationStarted` uses SYSTEM actor
- Idempotency: calling `/start` twice should return `already_started: true` on second call
- Existing integration test patterns may be in `crates/sr-api/tests/integration/`


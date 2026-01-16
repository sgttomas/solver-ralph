# Next Instance Prompt: SR-PLAN-V5 Phase 5a Implementation

## Your Assignment

You are continuing work on the **Semantic Ralph Loop MVP** project. The planning and review phase is complete. Your task is to **implement Phase 5a: Stage Advancement UI** as specified in `docs/planning/SR-PLAN-V5.md`.

## Orientation

### Git Branch
You are on branch `solver-ralph-6`. Confirm with `git branch --show-current`.

### What Has Been Done
1. **SR-PLAN-V5** was created defining four phases (5a-5d) to complete the MVP
2. **Coherence Review** (`docs/reviews/SR-PLAN-V5a-COHERENCE-REVIEW.md`) validated Phase 5a design
3. **Systematic Evaluation** (`docs/reviews/SR-PLAN-V5-SYSTEMATIC-EVALUATION.md`) checked ontology/epistemology/semantics consistency
4. **SR-PLAN-V5 was revised** to address all 9 findings from the evaluation:
   - Stage IDs normalized to UPPERCASE (`stage:FRAME`, `stage:EVAL`, `stage:FINAL`)
   - Terminology mapping added (PASS_WITH_WAIVERS ↔ Verified-with-Exceptions)
   - Portal ID convention documented (`portal:STAGE_COMPLETION:{stage_id}`)
   - FAIL behavior clarified (records evidence but doesn't advance stage)
   - MVP limitations documented in Appendix E

### Key Documents to Read
1. **`docs/planning/SR-PLAN-V5.md`** — The implementation plan (especially §3 Phase 5a)
2. **`docs/charter/SR-README.md`** — Assignment orientation and canonical document index
3. **`docs/platform/SR-CONTRACT.md`** — Binding invariants (C-EVID-*, C-TB-*, C-VER-*)

### What You Need to Implement (Phase 5a)

**Goal:** Add UI to complete stages with evidence from the WorkSurfaceDetail page.

**Deliverables (from SR-PLAN-V5 §3.8):**
| File | Action | Description |
|------|--------|-------------|
| `ui/src/pages/WorkSurfaceDetail.tsx` | EDIT | Add StageCompletionForm integration |
| `ui/src/components/StageCompletionForm.tsx` | CREATE | Reusable stage completion form |
| `ui/src/components/EvidenceBundleSelector.tsx` | CREATE | Evidence bundle picker with dropdown |

**Key Implementation Details:**
- Form visible only when: Work Surface status is "active" AND current stage status is "entered"
- Pre-populate oracle IDs from `workSurface.current_oracle_suites`
- Gate result status options: PASS, PASS_WITH_WAIVERS, FAIL
- If PASS_WITH_WAIVERS selected, waiver refs field is required
- API endpoint: `POST /api/v1/work-surfaces/:work_surface_id/stages/:stage_id/complete`

**Acceptance Criteria (from SR-PLAN-V5 §3.9):**
- [ ] "Complete Stage" button visible only when conditions met
- [ ] Form pre-populates oracle IDs from `current_oracle_suites`
- [ ] Form validates required fields before submission
- [ ] If `PASS_WITH_WAIVERS` selected, waiver refs field is required
- [ ] Successful completion refreshes page showing next stage
- [ ] Terminal stage completion shows Work Surface as "completed"
- [ ] Error states handled with appropriate messages
- [ ] Form state preserved on error for retry

### Existing Code to Reference
- **Backend API:** `crates/sr-api/src/handlers/work_surfaces.rs` — `complete_stage` endpoint exists
- **Evidence API:** `crates/sr-api/src/handlers/evidence.rs` — `GET /api/v1/evidence` for bundle list
- **Current UI:** `ui/src/pages/WorkSurfaceDetail.tsx` — where form will be integrated

### Implementation Order (from SR-PLAN-V5 §7)
1. Create `EvidenceBundleSelector.tsx` component (§3.4)
2. Create `StageCompletionForm.tsx` component with state management (§3.5) and validation (§3.6)
3. Integrate into `WorkSurfaceDetail.tsx` with success/error handling (§3.7)
4. Test stage completion flow against acceptance criteria (§3.9)

### MVP Limitations (Accepted)
- Evidence bundle existence not validated by backend
- Waiver refs not validated to exist
- Oracle results are user-entered, not from actual oracle runs
- FAIL records evidence but does NOT advance the stage

## Commands to Verify Environment
```bash
# Confirm branch
git branch --show-current

# Build backend
cargo build

# Build frontend
cd ui && npm run build

# Run tests
cargo test --workspace
```

## Begin Implementation
Start by reading `docs/planning/SR-PLAN-V5.md` §3 (Phase 5a) in full, then examine `ui/src/pages/WorkSurfaceDetail.tsx` to understand the current UI structure before creating the new components.

# SR-PLAN-V5 Phase 5a Coherence Review

**Review Date:** 2026-01-16
**Reviewer:** solver-ralph-6
**Document Under Review:** SR-PLAN-V5 §3 (Phase 5a: Stage Advancement UI)
**Review Type:** Coherence Review (pre-implementation validation)

---

## Executive Summary

**Verdict: COHERENT WITH MINOR ISSUES — Ready for Implementation**

Phase 5a of SR-PLAN-V5 is well-aligned with the canonical specifications. The plan correctly identifies the gap (no UI to complete stages) and proposes a solution that integrates properly with the existing `complete_stage` API endpoint. However, I identified several minor issues that should be addressed during implementation to ensure full compliance with SR-* specifications.

---

## 1. Documents Reviewed

| Document | Sections | Purpose |
|----------|----------|---------|
| SR-PLAN-V5 | §3 (Phase 5a) | Plan under review |
| SR-CONTRACT | §2.6, §4, §5, §7 | Binding invariants for evidence, gates, approvals |
| SR-PROCEDURE-KIT | §1, §2 | Stage machine semantics, `requires_approval` field |
| SR-WORK-SURFACE | §2, §3, §4, §5 | Work Surface binding, stage-gated procedures |
| SR-SPEC | (implied) | TypedRef, event patterns |
| work_surfaces.rs | `complete_stage` handler | Existing API contract |
| evidence.rs | `list_evidence`, `EvidenceSummary` | Evidence listing endpoint |
| WorkSurfaceDetail.tsx | Full file | Current UI state |

---

## 2. Ontology Alignment (Type/Schema Coherence)

### 2.1 Stage Completion Request Schema

**Plan proposes:**
```typescript
interface CompleteStageRequest {
  evidence_bundle_ref: string;
  gate_result: {
    status: 'PASS' | 'PASS_WITH_WAIVERS' | 'FAIL';
    oracle_results: Array<{
      oracle_id: string;
      status: string;
      evidence_ref?: string;
    }>;
    waiver_refs: string[];
  };
}
```

**Backend expects (work_surfaces.rs:152-174):**
```rust
pub struct CompleteStageRequest {
    pub evidence_bundle_ref: String,
    pub gate_result: GateResultRequest,
}

pub struct GateResultRequest {
    pub status: String,
    pub oracle_results: Vec<OracleResultRequest>,
    pub waiver_refs: Vec<String>,
}

pub struct OracleResultRequest {
    pub oracle_id: String,
    pub status: String,
    pub evidence_ref: Option<String>,
}
```

**Verdict:** ✅ **ALIGNED** — The TypeScript interface in SR-PLAN-V5 exactly matches the Rust backend types. The status enum values (`PASS`, `PASS_WITH_WAIVERS`, `FAIL`) match `GateResultStatus` parsing in `parse_gate_result_status()`.

---

### 2.2 Stage Completion Response Schema

**Plan proposes:**
```typescript
interface StageCompletionResponse {
  work_surface_id: string;
  completed_stage_id: string;
  next_stage_id: string | null;
  is_terminal: boolean;
  work_surface_status: 'active' | 'completed';
}
```

**Backend provides (work_surfaces.rs:177-184):**
```rust
pub struct StageCompletionResponse {
    pub work_surface_id: String,
    pub completed_stage_id: String,
    pub next_stage_id: Option<String>,
    pub is_terminal: bool,
    pub work_surface_status: String,
}
```

**Verdict:** ✅ **ALIGNED** — Exact match.

---

### 2.3 Evidence Bundle Summary Schema

**Plan references:** `GET /api/v1/evidence?limit=20` for evidence selection

**Backend provides (evidence.rs:124-134):**
```rust
pub struct EvidenceSummary {
    pub content_hash: String,
    pub bundle_id: String,
    pub run_id: String,
    pub candidate_id: String,
    pub oracle_suite_id: String,
    pub verdict: String,
    pub run_completed_at: String,
    pub artifact_count: usize,
}
```

**Plan UI mockup shows:**
```
sha256:abc123... (uploaded 2 hours ago, verdict: PASS)
```

**Issue:** ⚠️ **MINOR MISMATCH** — The `EvidenceSummary` struct has `run_completed_at`, not `uploaded_at` or `recorded_at`. The UI should display `run_completed_at` or the plan should clarify this is acceptable.

**Recommendation:** Update UI mockup to use `run_completed_at` timestamp, which represents when the evidence was produced.

---

## 3. Epistemology Alignment (What Can Be Known)

### 3.1 Evidence Bundle Selection

**Plan proposes:** A dropdown fetching `GET /api/v1/evidence?limit=20`

**Verification:**
- ✅ Endpoint exists (evidence.rs:711-748)
- ✅ Returns `EvidenceSummary[]` with `content_hash` (the required ref)
- ✅ Supports `limit` and `offset` pagination

**Issue:** ⚠️ **MINOR ENHANCEMENT NEEDED** — The plan shows displaying evidence as:
```
sha256:abc123... (uploaded 2 hours ago, verdict: PASS)
```

The evidence list endpoint does NOT filter by Work Surface or stage. Users might see evidence bundles from unrelated work. For MVP this is acceptable, but ideally the UI should:
1. Show which evidence bundles are associated with the current Work Surface/stage
2. Or at minimum, display `run_id` so users can distinguish them

**Recommendation:** For MVP, proceed as planned. Document that evidence selection shows all recent evidence regardless of Work Surface. Future enhancement: add `work_surface_id` filter to evidence list endpoint.

---

### 3.2 Current Stage Identification

**Plan proposes:** "Complete Stage" button visible only for current (entered) stage

**Verification:**
- ✅ `WorkSurfaceDetail` already has `workSurface.current_stage_id`
- ✅ `stage_status` map has `status: 'entered'` for current stage
- ✅ Backend validates `stage_id` matches `current_stage_id` (work_surfaces.rs:617-624)

**Verdict:** ✅ **ALIGNED** — The plan correctly identifies how to determine the current stage.

---

### 3.3 Oracle Results Population

**Plan proposes:** Users manually add oracle results via `[+ Add]` button

**Concern:** The plan doesn't specify where oracle IDs come from for the form. The current stage's required oracle suites are available via:
- `workSurface.current_oracle_suites` (array of `{ suite_id, suite_hash }`)

**Issue:** ⚠️ **MINOR GAP** — The plan should clarify:
1. Should the form pre-populate oracle IDs from `current_oracle_suites`?
2. Or is manual entry acceptable for MVP?

**Recommendation:** For MVP, allow manual oracle result entry. The form could optionally suggest oracle IDs from `current_oracle_suites` as a convenience, but this is not required for basic functionality.

---

## 4. Semantics Alignment (Meaning Consistency)

### 4.1 Gate Result Status Values

**Plan uses:** `'PASS' | 'PASS_WITH_WAIVERS' | 'FAIL'`

**SR-CONTRACT §2.1 defines:**
- **Verified (Strict):** all required oracles PASS
- **Verified-with-Exceptions:** at least one FAIL with Gate Waiver

**Backend maps (work_surfaces.rs:1063-1072):**
```rust
match status.to_uppercase().as_str() {
    "PASS" => Ok(GateResultStatus::Pass),
    "PASS_WITH_WAIVERS" => Ok(GateResultStatus::PassWithWaivers),
    "FAIL" => Ok(GateResultStatus::Fail),
    ...
}
```

**Verdict:** ✅ **ALIGNED** — Status values match.

---

### 4.2 Waiver Refs Requirement

**SR-CONTRACT C-VER-3:** "Verified-with-Exceptions requires... every FAIL covered by binding Gate Waiver"

**SR-CONTRACT C-EXC-4:** Waivers MUST reference specific failure(s)

**Plan proposes:** `waiver_refs: string[]` when status is `PASS_WITH_WAIVERS`

**Backend behavior (work_surfaces.rs):** Accepts `waiver_refs` array but does NOT validate:
1. That waivers exist
2. That waivers cover the specific failed oracles

**Issue:** ⚠️ **MINOR DEVIATION** — The backend doesn't enforce waiver validation per SR-CONTRACT C-VER-3 and C-EXC-4. This is acceptable for MVP but should be documented.

**Recommendation:** Add a note in the UI that waiver validation is not enforced. Future work: backend should validate waiver refs exist and cover failures.

---

### 4.3 Evidence Bundle Reference

**SR-CONTRACT C-EVID-1:** Evidence Bundles MUST include candidate reference, oracle suite hash, etc.

**Plan usage:** The `evidence_bundle_ref` is a content hash (e.g., `sha256:...`)

**Backend validation (work_surfaces.rs):** Does NOT validate that the evidence bundle:
1. Exists in the evidence store
2. Relates to the current Work Surface/stage
3. Has the expected oracle suite hash

**Issue:** ⚠️ **MINOR DEVIATION** — No validation of evidence bundle existence or relevance. Acceptable for MVP.

**Recommendation:** Document this limitation. The current design trusts users to provide valid evidence refs.

---

## 5. API Contract Verification

### 5.1 Endpoint Path

**Plan:** `POST /api/v1/work-surfaces/:work_surface_id/stages/:stage_id/complete`

**Backend (work_surfaces.rs header):** `POST /api/v1/work-surfaces/:id/stages/:stage_id/complete`

**Verdict:** ✅ **ALIGNED** — Path matches.

---

### 5.2 Error Cases

**Plan mentions:** "Error states handled (invalid evidence, already completed, etc.)"

**Backend handles (work_surfaces.rs):**
- 404: Work Surface not found
- 400: Stage ID doesn't match current stage
- 422: Invalid gate result status
- 409: Invalid state transition (not active)

**Verdict:** ✅ **ALIGNED** — Error cases are documented and handled.

---

## 6. UI State Machine Coherence

### 6.1 Current WorkSurfaceDetail.tsx State

The existing page has:
- ✅ Stage progress visualization
- ✅ Current stage info display
- ✅ Stage history table
- ❌ NO "Complete Stage" button (the gap)

**Plan correctly identifies this gap.**

---

### 6.2 Form Visibility Logic

**Plan:** "Complete Stage" button visible for current (entered) stage only

**Implementation guidance:**
```typescript
// Show form only when:
// 1. Work Surface status === 'active'
// 2. Stage status === 'entered'
const currentStageRecord = stages.find(s => s.status === 'entered');
const canComplete = workSurface.status === 'active' && currentStageRecord;
```

**Verdict:** ✅ **ALIGNED** — Logic is straightforward given existing data structures.

---

## 7. Issues Summary

| # | Severity | Issue | Recommendation |
|---|----------|-------|----------------|
| 1 | Minor | Evidence summary uses `run_completed_at`, not "uploaded" | Use `run_completed_at` in UI display |
| 2 | Minor | Evidence list not filtered by Work Surface | Acceptable for MVP; document limitation |
| 3 | Minor | Oracle ID source unclear | Pre-populate from `current_oracle_suites` or allow manual entry |
| 4 | Minor | Waiver validation not enforced | Document; future backend enhancement |
| 5 | Minor | Evidence bundle existence not validated | Document; trusts user input for MVP |

---

## 8. Recommended Additions to Plan

### 8.1 Form State Management

Add specification for form state:
```typescript
interface StageCompletionFormState {
  evidenceBundleRef: string;
  gateResultStatus: 'PASS' | 'PASS_WITH_WAIVERS' | 'FAIL';
  oracleResults: OracleResultInput[];
  waiverRefs: string[];
  isSubmitting: boolean;
  error: string | null;
}
```

### 8.2 Validation Rules

Add client-side validation:
1. `evidence_bundle_ref` must not be empty
2. `gate_result.status` must be selected
3. If status is `PASS_WITH_WAIVERS`, `waiver_refs` must not be empty
4. If status is `FAIL`, show warning that stage will not advance

### 8.3 Success Behavior

Clarify post-submission behavior:
1. On success: Refresh `WorkSurfaceDetail` data
2. If `is_terminal`: Show completion message
3. If not terminal: Show "Advancing to [next_stage_id]" message

---

## 9. Verdict

**Phase 5a is COHERENT and READY FOR IMPLEMENTATION.**

All identified issues are minor and can be addressed during implementation. The plan correctly:
- Identifies the API contract
- Proposes appropriate UI components
- Aligns with existing data structures
- Matches SR-CONTRACT and SR-PROCEDURE-KIT semantics

**Implementation can proceed.**

---

## 10. Change Log Entry (for SR-CHANGE)

If this review results in specification changes, log:

```yaml
- version: "0.5"
  date: 2026-01-16
  classification: "G:MINOR"
  summary: "SR-PLAN-V5 Phase 5a coherence review"
  changes:
    - "Reviewed Phase 5a (Stage Advancement UI) against canonical specs"
    - "Identified 5 minor issues (documented)"
    - "Approved for implementation"
```

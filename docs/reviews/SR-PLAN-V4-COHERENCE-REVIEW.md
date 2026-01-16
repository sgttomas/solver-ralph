# SR-PLAN-V4 Coherence Review

**Review Date:** 2026-01-16
**Reviewer:** Claude (automated coherence review)
**Plan Under Review:** `docs/planning/SR-PLAN-V4.md`
**Status:** Complete

---

## Summary

**Overall Assessment: Coherent with Minor Issues**

SR-PLAN-V4 (Work Surface Composition) is well-aligned with the canonical SR-* specifications. The plan correctly implements the Work Surface composition model defined in SR-WORK-SURFACE §5, properly distinguishes commitment objects from proposals per SR-CONTRACT §2.8, and uses appropriate event patterns consistent with SR-SPEC Appendix A.

Three minor issues require attention before implementation:
1. Missing `RefKind::Stage` in TypedRef kinds (SR-SPEC §1.5.3 lists `ProcedureStage` as a graph node type)
2. `stage_completion_status` enum uses `stage_status` as SQL type name (potential collision with broader status semantics)
3. Status enum values use `active/completed/archived` instead of SR-TYPES §3.1 canonical `draft/governed/superseded/deprecated/archived`

These issues are minor and can be addressed during implementation without requiring plan revision.

---

## Ontology Findings

*Do the domain model entities correctly represent the concepts in the specifications?*

| Item | Specification | Plan | Status | Notes |
|------|---------------|------|--------|-------|
| WorkSurfaceId format | SR-WORK-SURFACE §5.1 requires `work_surface_id` | Plan: `ws:<ULID>` | ✅ | Consistent with ULID patterns in SR-SPEC §1.3.1 |
| ManagedWorkSurface fields | SR-WORK-SURFACE §5.1 requires: work_unit_id, intake_ref, procedure_template_ref, stage_id, oracle_suites[], params{} | Plan includes all required fields plus lifecycle additions | ✅ | Plan extends with status, stage_status, attribution |
| Work Surface → Work Unit relationship | SR-CONTRACT §2.3: "Ralph Loop operates over a Work Surface" | Plan: 1:1 Work Surface ↔ Work Unit | ✅ | Correctly interpreted; Work Unit may have multiple Loops sharing same Work Surface |
| Work Surface → Loop relationship | SR-CONTRACT §2.3: Work Unit may have multiple Loop restarts | Plan: Work Surface shared across Loop restarts | ✅ | Diagram in §5 correctly shows "Loop (many) → references Work Surface" |
| StageStatusRecord entity | SR-PROCEDURE-KIT: stages track completion | Plan defines StageStatusRecord with stage_id, status, timestamps, evidence_bundle_ref | ✅ | Appropriate for stage progression tracking |
| ContentAddressedRef usage | SR-SPEC §1.3.2: `sha256:<64-hex>` format | Plan uses ContentAddressedRef for intake_ref, procedure_template_ref | ✅ | Correct pattern |
| OracleSuiteBinding | SR-SEMANTIC-ORACLE-SPEC §2: suite_id + suite_hash required | Plan: OracleSuiteBinding with suite_id, suite_hash | ✅ | Includes semantic set binding via hash |
| RefKind values | SR-SPEC §1.5.3 lists valid kinds | Plan uses: Loop, GovernedArtifact, Iteration, Intake, ProcedureTemplate, OracleSuite | ⚠️ | Missing `ProcedureStage` (listed in SR-SPEC §1.8.1 as graph node type); consider adding `Stage` ref kind for stage-specific refs |
| RefRelation values | SR-SPEC §1.5.3 lists valid relations | Plan uses: InScopeOf, DependsOn | ✅ | Correct subset for iteration context |

---

## Epistemology Findings

*What can be known and how it is established?*

| Item | Specification | Plan | Status | Notes |
|------|---------------|------|--------|-------|
| Commitment Object semantics | SR-CONTRACT §2.8: "durable, content-addressed, referenceable" | Plan: binding refs immutable once bound; stage transitions via events | ✅ | Correct immutability boundary |
| Proposal vs Commitment transition | SR-CONTRACT §2.8: "Proposals are non-authoritative" | Plan: Work Surface becomes commitment object on `WorkSurfaceBound` event | ✅ | Event marks the commitment boundary |
| Evidence binding to stage | SR-SPEC §1.9.1: EvidenceBundleRecorded must include procedure_template_id, stage_id | Plan §4.2 EvidenceBundleWorkSurfaceContext includes both | ✅ | Correct binding structure |
| C-CTX-1 satisfaction | SR-CONTRACT: IterationStarted.refs[] must include Work Surface components | Plan §4.1 compile_iteration_context_refs includes Intake, ProcedureTemplate, OracleSuite refs | ✅ | All required refs present |
| C-CTX-2 satisfaction | SR-CONTRACT: "No ghost inputs" - all context derivable from refs | Plan: refs include content_hash for each component | ✅ | Content hashes enable deterministic replay |
| Stage gate evidence | SR-SPEC §1.9.1: evidence must bind to candidate + procedure + stage | Plan: StageCompleted carries evidence_bundle_ref, gate_result | ✅ | Gate result structure includes oracle_results |
| Oracle suite per-stage binding | SR-SEMANTIC-ORACLE-SPEC §2: suite_hash must incorporate semantic set definitions | Plan: suites resolved dynamically per-stage, hash captured at stage entry | ✅ | Per-stage resolution prevents stale suite binding |
| Verification scope | SR-CONTRACT C-VER-4: "verification scope" must include stage_id, procedure_template_id | Plan: EvidenceBundleWorkSurfaceContext includes both | ✅ | Correct scope binding |

---

## Semantics Findings

*Are terms, status values, and event patterns used consistently?*

| Item | Specification | Plan | Status | Notes |
|------|---------------|------|--------|-------|
| WorkSurfaceStatus enum | SR-TYPES §3.1 defines status: draft/governed/superseded/deprecated/archived | Plan uses: Active/Completed/Archived | ⚠️ | Plan uses lifecycle-appropriate values but differs from canonical artifact status. This is acceptable as Work Surface lifecycle differs from artifact governance lifecycle. Recommend documenting this intentional divergence. |
| StageCompletionStatus enum | SR-PROCEDURE-KIT implies stage states | Plan: Pending/Entered/Completed/Skipped | ✅ | Appropriate stage lifecycle states |
| GateResultStatus enum | SR-PROCEDURE-KIT: gate_rule semantics | Plan: Pass/PassWithWaivers/Fail | ✅ | Aligns with SR-CONTRACT C-VER-2/C-VER-3 (Verified Strict vs With-Exceptions) |
| Event names | SR-SPEC Appendix A patterns | Plan: WorkSurfaceBound, StageEntered, StageCompleted, WorkSurfaceCompleted, WorkSurfaceArchived | ✅ | Follows `<Entity><Action>` pattern; consistent with existing events |
| Stream kind | SR-SPEC §1.5.2: stream_kind enum | Plan adds: StreamKind::WorkSurface | ✅ | Appropriate new stream kind |
| SQL enum type names | PostgreSQL naming conventions | Plan: `work_surface_status`, `stage_completion_status` | ⚠️ | `stage_completion_status` is verbose; consider `stage_status` but verify no collision with existing types |
| Content hash format | SR-SPEC §1.3.2: `sha256:<64-hex>` | Plan: `sha256:<hex>` in constraints | ✅ | Correct format |
| Actor attribution | SR-SPEC §1.4: actor_kind + actor_id | Plan: bound_by_kind/bound_by_id, archived_by_kind/archived_by_id | ✅ | Consistent with SR-SPEC actor model |
| Canonical term: Work Surface | SR-CONTRACT §2.11: `work_surface` | Plan uses "Work Surface Instance", "ManagedWorkSurface" | ✅ | Acceptable variations for domain model types |
| procedure_template_id format | SR-PROCEDURE-KIT §1: `proc:<NAME>` | Plan uses same format | ✅ | Consistent |
| stage_id format | SR-PROCEDURE-KIT §2: `stage:<NAME>` | Plan uses same format | ✅ | Consistent |

---

## Recommended Changes

### Before Implementation (Required)

1. **Document status enum divergence**: Add a note to SR-PLAN-V4 §1.1 explaining that `WorkSurfaceStatus` uses lifecycle-appropriate values (Active/Completed/Archived) rather than the artifact governance status values from SR-TYPES §3.1, because Work Surface lifecycle semantics differ from governed artifact lifecycle semantics.

### During Implementation (Minor Fixes)

2. **Consider adding RefKind::Stage**: When implementing the iteration context compilation (§4.1), consider whether a `RefKind::Stage` (or `RefKind::ProcedureStage`) should be added to enable stage-specific refs. SR-SPEC §1.8.1 lists `ProcedureStage` as a graph node type. This may be useful for stage-scoped dependency tracking.

3. **Verify SQL type naming**: Before creating the `stage_completion_status` enum, verify no collision with any existing `stage_status` type. Consider using `stage_completion_status` as planned for clarity.

### Not Required (Acceptable Deviations)

- The use of `Active/Completed/Archived` for Work Surface status is **acceptable** because:
  - Work Surface is a runtime domain object, not a governed artifact
  - SR-TYPES §3.1 status values are for governed artifact lifecycle, not runtime entity lifecycle
  - The plan's status values correctly model Work Surface lifecycle (in-progress → terminal-reached → superseded/abandoned)

---

## Conclusion

**Ready for implementation with minor documentation update.**

SR-PLAN-V4 is coherent with the canonical SR-* specifications. The domain model correctly represents the Work Surface composition concepts, the epistemological boundaries (commitment vs mutable state) are correctly placed, and the semantic terms are used consistently with minor acceptable deviations.

**Recommended Action:**
1. Add a brief note to SR-PLAN-V4 §1.1 documenting the intentional `WorkSurfaceStatus` enum values (see Recommended Changes #1)
2. Proceed to Phase 4a implementation

---

## Document References

| Document | Sections Reviewed | Verification Status |
|----------|-------------------|---------------------|
| SR-PLAN-V4 | Full document | ✅ Reviewed |
| SR-CONTRACT | §2.3, §2.8, C-CTX-1, C-CTX-2, C-VER-2, C-VER-3 | ✅ Verified |
| SR-WORK-SURFACE | §2, §3.1, §4.1, §5 | ✅ Verified |
| SR-SPEC | §1.3, §1.4, §1.5.3, §1.9.1, §3.2.1.1, Appendix A | ✅ Verified |
| SR-TYPES | §3.1, §4.3 | ✅ Verified |
| SR-PROCEDURE-KIT | §1, §2 | ✅ Verified |
| SR-SEMANTIC-ORACLE-SPEC | §2, §3, §4 | ✅ Verified |

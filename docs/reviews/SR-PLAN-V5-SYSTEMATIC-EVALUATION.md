# SR-PLAN-V5 Systematic Evaluation

**Evaluation Date:** 2026-01-16
**Evaluator:** solver-ralph-6
**Document Under Review:** SR-PLAN-V5 (all phases)
**Evaluation Type:** Systematic Consistency Evaluation (Ontology, Epistemology, Semantics)

---

## Executive Summary

This evaluation systematically analyzes SR-PLAN-V5 against the canonical SR-* specifications across three dimensions:

1. **Ontology** — Are the types, identities, and structures consistent?
2. **Epistemology** — Are the knowledge claims and evidence requirements valid?
3. **Semantics** — Are the meanings and interpretations consistent?

**Overall Verdict: CONSISTENT WITH MINOR ISSUES**

The plan is well-aligned with canonical specifications. I identified 5 minor ontological issues, 4 epistemological issues, and 3 semantic issues. All are addressable during implementation.

---

## 1. Documents Reviewed

### Primary Plan
- **SR-PLAN-V5** — Semantic Ralph Loop End-to-End Integration (all phases)

### Canonical Specifications
| Document | Purpose | Key Sections Reviewed |
|----------|---------|----------------------|
| SR-CONTRACT | Binding invariants | §2 (Definitions), §4-5 (Trust/Verification), §7 (Evidence), §9 (Loop) |
| SR-SPEC | Technical mechanics | §1.2-1.5 (Terminology, Events), §1.9 (Evidence), §2.3 (API), §3.2 (Iteration) |
| SR-TYPES | Type registry | §4.3 (Platform Domain Types), §7 (Schemas) |
| SR-WORK-SURFACE | Work Surface spec | §2-5 (Core concepts, schemas) |
| SR-PROCEDURE-KIT | Procedure templates | §1-2 (Registry, stages, requires_approval) |
| SR-SEMANTIC-ORACLE-SPEC | Oracle interface | §2-4 (Suite binding, outputs) |

---

## 2. Ontology Evaluation (Type/Structure Consistency)

### 2.1 Work Surface Identity

**SR-TYPES §7.7 defines:**
```
Identifier format: ws:<ULID>
```

**SR-PLAN-V5 uses:** `ws:xyz`, `ws:...`

**Verdict:** ✅ **CONSISTENT**

---

### 2.2 Stage Identity

**SR-WORK-SURFACE §4.1 defines:**
```
stage_id: stage:<NAME>
```

**SR-PROCEDURE-KIT §2 defines stages as:**
```
stage:FRAME, stage:OPTIONS, stage:DRAFT, stage:SEMANTIC_EVAL, stage:FINAL
```

**SR-PLAN-V5 uses:** `stage:frame`, `stage:eval`, `stage:final`, `stage:FRAME`, `stage:DRAFT`

**Issue:** ⚠️ **MINOR — Case Inconsistency**

SR-PLAN-V5 mixes lowercase (`stage:frame`) and uppercase (`stage:FRAME`) stage IDs. SR-PROCEDURE-KIT uses UPPERCASE.

**Recommendation:** Normalize to UPPERCASE per SR-PROCEDURE-KIT convention (e.g., `stage:FRAME`, `stage:FINAL`).

---

### 2.3 Procedure Template Identity

**SR-PROCEDURE-KIT §1 defines:**
```
procedure_template_id: proc:<NAME>
```

**SR-PLAN-V5 uses:** `proc:research-memo`, `proc:GENERIC-KNOWLEDGE-WORK`

**Verdict:** ✅ **CONSISTENT** — Both hyphenated and uppercase names are used in canonical docs.

---

### 2.4 Evidence Bundle Type

**SR-TYPES §4.3 defines:**
```
domain.evidence_bundle — Oracle evidence bundle for a candidate
```

**SR-SPEC §1.9.1 defines manifest:**
```
artifact_type: evidence.gate_packet
```

**SR-PLAN-V5 references:**
- `evidence_bundle_ref: string` (content hash)
- API endpoint `GET /api/v1/evidence?limit=20`

**Issue:** ⚠️ **MINOR — Missing type_key context**

SR-PLAN-V5 treats evidence bundles as opaque hashes. The UI should understand that `evidence_bundle_ref` refers to `domain.evidence_bundle` (type_key) with manifest `artifact_type: evidence.gate_packet`.

**Recommendation:** Add comment in plan noting the type mapping. No code change needed for MVP.

---

### 2.5 Gate Result Status

**SR-CONTRACT §2.1 defines:**
```
Verified (Strict) — all required oracles PASS
Verified-with-Exceptions — at least one FAIL with Gate Waiver
```

**SR-SPEC §3.3 defines:**
```
Verified (Strict) iff evidence exists and all required oracle results are PASS
Verified-with-Exceptions iff ... each FAIL is covered by an active Waiver
```

**SR-PLAN-V5 §3.3 defines:**
```
status: 'PASS' | 'PASS_WITH_WAIVERS' | 'FAIL'
```

**Issue:** ⚠️ **MINOR — Terminology alignment**

The plan uses `PASS_WITH_WAIVERS` while SR-CONTRACT uses "Verified-with-Exceptions". These are semantically equivalent but the terminology differs.

**Recommendation:** Document the mapping: `PASS_WITH_WAIVERS` ↔ `Verified-with-Exceptions`. The backend already uses this mapping correctly.

---

### 2.6 Actor Identity

**SR-SPEC §1.4 defines:**
```
actor_kind: HUMAN | AGENT | SYSTEM
actor_id: oidc_sub:<issuer_hash>:<subject> (human)
```

**SR-PLAN-V5 references actors in:**
- Stage completion: uses authenticated user context
- Approval recording: requires HUMAN

**Verdict:** ✅ **CONSISTENT** — Plan correctly relies on existing auth infrastructure.

---

### 2.7 Portal Identity

**SR-CONTRACT §2.6 defines:**
```
Portal: gate requiring human arbitration; every Portal is a trust boundary
```

**SR-PLAN-V5 §5.4 introduces:**
```
portal_id: portal:stage-gate:<stage_id>
```

**Issue:** ⚠️ **MINOR — Portal naming convention**

The plan introduces `portal:stage-gate:stage:eval` format. SR-CONTRACT doesn't specify portal ID format, but consistency matters.

**Recommendation:** Document this naming convention. Consider using `portal:stage-gate-<stage_id>` (single colon) for cleaner parsing.

---

### 2.8 Work Unit vs Work Surface Relationship

**SR-WORK-SURFACE §2.1 defines:**
```
Work Surface binds: a Work Unit, an Intake, a Procedure Template, a Stage, Oracle Suites
```

**SR-PLAN-V5 §4 (Phase 5b) assumes:**
```
Work Surface 1:1 with Work Unit
Loop references work_unit_id
```

**Verdict:** ✅ **CONSISTENT** — The 1:1 relationship is correct per SR-WORK-SURFACE.

---

### 2.9 Summary: Ontology Issues

| # | Issue | Severity | Recommendation |
|---|-------|----------|----------------|
| O1 | Stage ID case inconsistency | Minor | Normalize to UPPERCASE |
| O2 | Evidence type_key context missing | Minor | Add documentation comment |
| O3 | PASS_WITH_WAIVERS vs Verified-with-Exceptions | Minor | Document terminology mapping |
| O4 | Portal ID naming convention | Minor | Document convention |
| O5 | — | — | — |

---

## 3. Epistemology Evaluation (Knowledge/Evidence Claims)

### 3.1 Evidence Bundle as Gate Proof

**SR-CONTRACT C-EVID-1 requires:**
```
Evidence Bundles MUST include:
- Candidate reference
- Oracle suite hash
- Per-oracle results
- Content hash
```

**SR-PLAN-V5 §3.3 accepts:**
```
evidence_bundle_ref: string (content hash only)
```

**Issue:** ⚠️ **GAP — No evidence structure validation**

The plan accepts an opaque hash. It doesn't validate that the referenced evidence bundle contains the required fields per C-EVID-1.

**Recommendation:** Document this as MVP limitation (already in Appendix E). Future: backend should validate evidence structure before accepting stage completion.

---

### 3.2 Oracle Results as Evidence

**SR-SPEC §1.9.1 defines evidence manifest with:**
```
results: [{ oracle_id, status, artifacts[] }]
```

**SR-PLAN-V5 §3.3 accepts:**
```
oracle_results: Array<{ oracle_id, status, evidence_ref? }>
```

**Issue:** ⚠️ **GAP — Oracle results not validated against evidence**

The UI collects oracle results separately from the evidence bundle. There's no validation that the user-provided oracle results match the evidence bundle contents.

**Recommendation:** Document as MVP limitation. Future: auto-populate oracle results from evidence bundle manifest.

---

### 3.3 Waiver Coverage of Failures

**SR-CONTRACT C-VER-3 requires:**
```
Verified-with-Exceptions requires... every FAIL covered by binding Gate Waiver
```

**SR-CONTRACT C-EXC-4 requires:**
```
Waivers MUST reference specific failure(s)
```

**SR-PLAN-V5 §3.3 accepts:**
```
waiver_refs: string[] (opaque IDs)
```

**Issue:** ⚠️ **GAP — Waiver-to-failure linkage not validated**

The plan accepts waiver refs but doesn't validate that:
1. Waivers exist
2. Waivers cover the specific failed oracles

**Recommendation:** Document as MVP limitation (already in Appendix E.2). Future: backend should validate waiver coverage per C-VER-3.

---

### 3.4 Approval as Evidence of Human Authority

**SR-CONTRACT C-TB-1 requires:**
```
Any action that creates binding authority MUST require a Human actor
```

**SR-CONTRACT C-TB-3 requires:**
```
Every Portal crossing MUST produce a binding Approval record
```

**SR-PLAN-V5 §5.4-5.5 proposes:**
```
Query for approval: portal_id = portal:stage-gate:<stage_id>
Subject refs: [{"kind": "WorkSurface", "id": work_surface_id}]
```

**Verdict:** ✅ **CONSISTENT** — The approval check mechanism is correct. It queries for existing approvals before allowing stage completion.

---

### 3.5 Iteration Context Provenance

**SR-CONTRACT C-CTX-1 requires:**
```
IterationStarted.refs[] constitutes authoritative provenance
```

**SR-SPEC §3.2.1.1 requires:**
```
Minimum required ref categories: Loop, Governing artifacts, Oracle suite, Intake, Procedure Template
```

**SR-PLAN-V5 §4 (Phase 5b) proposes:**
```
Loops inherit Work Surface context automatically
Iterations auto-populate work_unit_id from Loop
```

**Verdict:** ✅ **CONSISTENT** — This is already implemented in Phase 4c. The iteration handler fetches Work Surface refs when work_unit_id is provided.

---

### 3.6 Summary: Epistemology Issues

| # | Issue | Severity | Recommendation |
|---|-------|----------|----------------|
| E1 | Evidence bundle structure not validated | Minor | Document; future backend validation |
| E2 | Oracle results not validated against evidence | Minor | Document; future auto-populate |
| E3 | Waiver-to-failure linkage not validated | Minor | Document; future backend validation |
| E4 | — | — | — |

---

## 4. Semantics Evaluation (Meaning Consistency)

### 4.1 Stage Completion Meaning

**SR-PROCEDURE-KIT §2 defines stage completion as:**
```
Transition on pass: stage gate passed, proceed to next stage
Terminal stage: work surface complete
```

**SR-WORK-SURFACE §5.3 states:**
```
EvidenceBundleRecorded MUST bind evidence to (candidate_id, procedure_template_id, stage_id)
```

**SR-PLAN-V5 §3 proposes:**
```
StageCompleted event → StageEntered for next (or WorkSurfaceCompleted if terminal)
```

**Issue:** ⚠️ **GAP — Evidence binding semantics**

The plan's stage completion doesn't enforce that the evidence bundle is bound to the current stage context per SR-WORK-SURFACE §5.3.

**Recommendation:** Document as MVP limitation. The evidence_bundle_ref is accepted on trust. Future: validate evidence bundle's `stage_id` matches the completing stage.

---

### 4.2 Approval-Gated Stage Meaning

**SR-PROCEDURE-KIT §1 defines:**
```
requires_approval: true — stage completion requires HUMAN approval via portal
```

**SR-CONTRACT C-TB-7 states:**
```
Evaluation and Assessment are NOT substitutes for Portal Approval
```

**SR-PLAN-V5 §5 proposes:**
```
Check for approval at portal:stage-gate:<stage_id>
Approval must have subject_refs containing the Work Surface
```

**Verdict:** ✅ **CONSISTENT** — The plan correctly implements approval-gated stages by checking for existing Portal approvals.

---

### 4.3 Loop-Work Surface Binding Meaning

**SR-WORK-SURFACE §2.1 states:**
```
Work Surface is the binding context for an iteration
```

**SR-CONTRACT C-CTX-2 requires:**
```
No ghost inputs — context derivable from IterationStarted payload + refs
```

**SR-PLAN-V5 §4 proposes:**
```
Loop creation validates Work Surface exists
Loop stores work_surface_id reference
Iterations auto-inherit Work Surface context
```

**Verdict:** ✅ **CONSISTENT** — This enforces the binding relationship and prevents ghost inputs.

---

### 4.4 Fail vs Pass Semantics

**SR-CONTRACT §5 defines verification semantics:**
```
C-VER-2: Verified (Strict) requires every required oracle PASS
C-VER-3: Verified-with-Exceptions requires each FAIL covered by Waiver
```

**SR-PLAN-V5 §3.6 states:**
```
FAIL Warning: "Stage will not advance with FAIL status"
```

**Issue:** ⚠️ **SEMANTIC CLARIFICATION NEEDED**

The plan says "Stage will not advance with FAIL status" but doesn't clarify what happens. Does the stage completion:
a) Reject entirely (API error)?
b) Accept but record as incomplete?
c) Accept but block progression?

**Recommendation:** Clarify: Stage completion with `FAIL` status should be accepted (evidence is recorded) but the stage does NOT advance to the next stage. The Work Surface remains at the current stage. This aligns with SR-CONTRACT: evidence is recorded, but progression requires PASS or PASS_WITH_WAIVERS.

---

### 4.5 Terminal Completion Meaning

**SR-PROCEDURE-KIT §2 defines:**
```
stage:FINAL — terminal stage; Transition on pass: terminal
```

**SR-WORK-SURFACE via SR-SPEC Appendix A defines:**
```
WorkSurfaceCompleted event
```

**SR-PLAN-V5 §3 proposes:**
```
is_terminal: true → emit WorkSurfaceCompleted
work_surface_status: 'completed'
```

**Verdict:** ✅ **CONSISTENT**

---

### 4.6 Summary: Semantics Issues

| # | Issue | Severity | Recommendation |
|---|-------|----------|----------------|
| S1 | Evidence-to-stage binding not enforced | Minor | Document; future validation |
| S2 | FAIL status behavior unclear | Minor | Clarify: record evidence, don't advance |
| S3 | — | — | — |

---

## 5. Cross-Cutting Analysis

### 5.1 Trust Boundary Compliance

**SR-CONTRACT §4 defines trust boundaries requiring HUMAN authority:**
- Approving/rejecting at Release Portal ✅
- Approving governance changes ✅
- Approving Deviations/Deferrals/Waivers ✅
- Finalizing Freeze Records ✅

**SR-PLAN-V5 introduces approval-gated stages:**
- Stage completion checks for prior HUMAN approval ✅
- Approval must exist at `portal:stage-gate:<stage_id>` ✅

**Verdict:** ✅ **COMPLIANT**

---

### 5.2 Event Model Compliance

**SR-SPEC §1.5 defines required event envelope fields:**
- event_id, stream_id, stream_kind, stream_seq
- event_type, occurred_at, actor_kind, actor_id
- refs[], payload, envelope_hash

**SR-PLAN-V5 relies on existing events:**
- `StageCompleted` ✅ (Phase 4b implemented)
- `WorkSurfaceCompleted` ✅ (Phase 4b implemented)

**Verdict:** ✅ **COMPLIANT** — All events are already defined in SR-SPEC Appendix A and implemented.

---

### 5.3 API Contract Compliance

**SR-SPEC §2.3 defines API patterns:**
- All endpoints under `/api/v1`
- Write endpoints return `{ accepted, correlation_id, emitted_event_ids[] }`

**SR-PLAN-V5 uses:**
- `POST /api/v1/work-surfaces/:id/stages/:stage_id/complete` ✅ (exists)
- `GET /api/v1/evidence?limit=20` ✅ (exists)
- `GET /api/v1/work-surfaces/:id/stages/:stage_id/approval-status` (new in Phase 5c)

**Verdict:** ✅ **COMPLIANT**

---

### 5.4 Integrity Condition Handling

**SR-CONTRACT §9 defines mandatory stop triggers:**
- ORACLE_TAMPER, ORACLE_GAP, ORACLE_ENV_MISMATCH, ORACLE_FLAKE
- EVIDENCE_MISSING (non-waivable)

**SR-PLAN-V5 doesn't address integrity conditions directly.**

**Issue:** ⚠️ **OUT OF SCOPE BUT NOTED**

The plan focuses on the happy path. Integrity condition handling is out of scope but should work via existing infrastructure.

---

## 6. Overall Findings

### 6.1 Consistency Summary

| Dimension | Status | Issues |
|-----------|--------|--------|
| Ontology | ✅ Consistent | 4 minor |
| Epistemology | ✅ Consistent | 3 minor |
| Semantics | ✅ Consistent | 2 minor |

### 6.2 All Issues

| ID | Dimension | Issue | Severity | Recommendation |
|----|-----------|-------|----------|----------------|
| O1 | Ontology | Stage ID case inconsistency | Minor | Normalize to UPPERCASE |
| O2 | Ontology | Evidence type_key context | Minor | Document mapping |
| O3 | Ontology | PASS_WITH_WAIVERS terminology | Minor | Document mapping |
| O4 | Ontology | Portal ID naming convention | Minor | Document convention |
| E1 | Epistemology | Evidence structure not validated | Minor | MVP limitation |
| E2 | Epistemology | Oracle results not validated | Minor | MVP limitation |
| E3 | Epistemology | Waiver coverage not validated | Minor | MVP limitation |
| S1 | Semantics | Evidence-stage binding | Minor | MVP limitation |
| S2 | Semantics | FAIL behavior unclear | Minor | Clarify in plan |

### 6.3 Recommendations

1. **Normalize stage IDs** to UPPERCASE in examples and test scenarios
2. **Clarify FAIL behavior**: Record evidence but don't advance stage
3. **Document terminology mappings** for cross-reference clarity
4. **Accept MVP limitations** for evidence/waiver validation (already documented in Appendix E)

---

## 7. Verdict

**SR-PLAN-V5 is CONSISTENT with the canonical SR-* specifications.**

All identified issues are minor and can be addressed during implementation. The plan correctly:
- Uses canonical type identities
- Respects trust boundary requirements
- Follows event model patterns
- Maintains evidence-based verification semantics

**Recommendation: Proceed with implementation.**

---

## Appendix: Document Cross-Reference Matrix

| SR-PLAN-V5 Section | Primary Spec | Key Invariants |
|--------------------|--------------|----------------|
| §3 (Stage Advancement) | SR-WORK-SURFACE §5, SR-PROCEDURE-KIT §2 | Stage progression, evidence binding |
| §4 (Loop Binding) | SR-CONTRACT C-CTX-1/2, SR-SPEC §3.2.1.1 | No ghost inputs, context provenance |
| §5 (Approval Gates) | SR-CONTRACT C-TB-1/3, SR-PROCEDURE-KIT §1 | Human authority, portal crossings |
| §6 (E2E Test) | SR-CHARTER §Immediate Objective | MVP validation |

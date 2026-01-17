# SR-PLAN-V11 Consistency Review

**Reviewer:** Agent (solver-ralph-11 branch)
**Review Date:** 2026-01-17
**Document Under Review:** `docs/planning/SR-PLAN-V11.md` (revised)
**Prior Review:** `docs/planning/SR-PLAN-V11-COHERENCE-REVIEW.md` (codebase coherence)
**Status:** Complete

---

## Executive Summary

SR-PLAN-V11 has been analyzed for consistency with the canonical SR-* documents across three dimensions: ontology, epistemology, and semantics. The analysis identified several inconsistencies that require correction before implementation.

**Verdict: REVISE**

The plan contains terminology and schema inconsistencies that conflict with SR-TYPES, SR-CONTRACT, and SR-SPEC. These must be corrected to ensure implementation aligns with the platform's normative definitions.

---

## 1. Ontology Findings (SR-TYPES Alignment)

### O-1: `GovernedArtifact` as TypedRef `kind` — INCONSISTENT

**V11-6 proposes:**
```
kind: "GovernedArtifact"
rel: "governed_by"
meta: { doc_id, content_hash, version }
```

**SR-SPEC §1.5.2 defines valid `kind` values:**
```
"kind": "GovernedArtifact|Candidate|OracleSuite|EvidenceBundle|Approval|Record|Decision|Deviation|Deferral|Waiver|Loop|Iteration|Run|Freeze"
```

**Finding:** `GovernedArtifact` IS a valid `kind` value per SR-SPEC §1.5.2. However, the `rel` value proposed is inconsistent.

**Severity:** Medium

---

### O-2: `rel: "governed_by"` — INCONSISTENT

**V11-6 proposes:** `rel: "governed_by"` for GovernedArtifact refs

**SR-SPEC §1.5.3 states:**
> **Constraint:** `rel=governed_by` is a label for governance relationships and MUST NOT be relied upon for staleness propagation. If a reference is intended to participate in dependency semantics, use `rel=depends_on`.

**SR-SPEC §3.2.1.1 (Iteration Context Ref Set) uses:**
- `rel=depends_on` for artifacts that affect iteration semantics
- `rel=supported_by` for audit-only provenance

**Finding:** `governed_by` is explicitly deprecated for dependency semantics. The plan should use `rel=depends_on` (if the GovernedArtifact affects iteration semantics) or `rel=supported_by` (if audit-only).

**Recommendation:** Change `rel: "governed_by"` to `rel: "depends_on"` since SR-DIRECTIVE affects agent behavior and should participate in staleness propagation.

**Severity:** High — directly violates SR-SPEC §1.5.3 constraint

---

### O-3: `Exception` as TypedRef `kind` — INCONSISTENT

**V11-6 proposes:**
```
kind: "Exception"
rel: "waived_by"
meta: { exception_id, scope, expires_at }
```

**SR-SPEC §1.5.2 valid `kind` values:**
```
Deviation|Deferral|Waiver
```

**Finding:** `Exception` is not a valid `kind` value. SR-SPEC uses the specific exception types: `Deviation`, `Deferral`, `Waiver`. These are distinct types per SR-CONTRACT §2.7 and SR-TYPES §4.4.

**Recommendation:** Use the specific exception kind (`Deviation`, `Deferral`, or `Waiver`) rather than generic `Exception`.

**Severity:** Medium — type vocabulary mismatch

---

### O-4: `rel: "waived_by"` — UNDEFINED

**V11-6 proposes:** `rel: "waived_by"` for exception refs

**SR-SPEC §1.5.3 canonical `rel` values:**
- `depends_on` — semantic dependency
- `supported_by` — audit-only provenance
- `about`, `produces`, `approved_by`, `releases`, `acknowledges`, `affects`, `in_scope_of`, `root_cause`, `stale`, `relates_to`

**Finding:** `waived_by` is not in the canonical `rel` vocabulary. The closest match is:
- `rel=acknowledges` (used in Freeze Records for active exceptions per SR-SPEC §1.12)
- `rel=depends_on` (if the exception affects iteration semantics)

**Recommendation:** Use `rel=depends_on` (per SR-SPEC §3.2.1.1 item 6: "Active exceptions... `rel=depends_on`").

**Severity:** Medium — introduces non-canonical vocabulary

---

### O-5: `GovernedManifest` Type — UNREGISTERED

**V11-6 proposes a new struct:**
```rust
struct GovernedManifest {
    artifacts: Vec<GovernedArtifactRef>,
    computed_at: DateTime<Utc>,
}
```

**Finding:** This is a new internal implementation type, not a platform domain type. It does not need to be registered in SR-TYPES §4, but the plan should clarify that `GovernedManifest` is an **implementation-internal** structure (not a domain artifact type).

**Recommendation:** Add clarification that `GovernedManifest` is internal to the API service and is not a typed domain artifact.

**Severity:** Low — clarification needed

---

## 2. Epistemology Findings (SR-CONTRACT Alignment)

### E-1: Active Exceptions in `IterationStarted.refs[]` — ALIGNED

**V11-6 proposes:** Including active exceptions in `IterationStarted.refs[]`

**SR-CONTRACT C-CTX-1 requires:**
> The `IterationStarted` event MUST include `refs[]` constituting authoritative provenance for the iteration's effective context

**SR-SPEC §3.2.1.1 (item 6) specifies:**
> Active exceptions (Waivers, Deviations, Deferrals)... `kind=Deviation|Deferral|Waiver`, `rel=depends_on`

**Finding:** The intent aligns with SR-CONTRACT and SR-SPEC. The specific `kind` and `rel` values need correction (see O-3, O-4).

**Severity:** N/A (aligned in intent)

---

### E-2: GovernedArtifact Content Hashing — REQUIRES CONTRACT INTERPRETATION

**V11-6 proposes:** Compute `ContentHash` of SR-DIRECTIVE at API startup, include in `IterationStarted.refs[]`

**SR-CONTRACT C-CTX-2 (No Ghost Inputs) requires:**
> Iteration context MUST be derivable solely from the `IterationStarted` event payload and the dereferenced `refs[]`. Unrepresented inputs MUST NOT influence work.

**Finding:** Including GovernedArtifact refs with content hashes supports C-CTX-2 by making the governing documents explicit and reproducible. This is epistemologically sound.

**However:** SR-SPEC §3.2.1.1 does not currently list `GovernedArtifact` refs as a "minimum required ref category" for `IterationStarted.refs[]`. The plan proposes adding a new category.

**Recommendation:** V11-6 implementation should:
1. Propose updating SR-SPEC §3.2.1.1 to add GovernedArtifact as a required ref category, OR
2. Document this as an optional/extended ref that goes beyond the minimum required set

**Severity:** Medium — requires specification update decision

---

### E-3: E2E Harness Assertions — ALIGNED WITH C-OR-7

**V11-4 proposes:** E2E scenarios for `ORACLE_GAP` and `EVIDENCE_MISSING`

**SR-CONTRACT C-OR-7 requires:**
> Integrity conditions MUST halt progression, record context, and route escalation.

**SR-CONTRACT §2.5 defines integrity conditions:**
- `ORACLE_GAP` — required oracle has no recorded result
- `EVIDENCE_MISSING` — referenced evidence cannot be retrieved

**Finding:** V11-4's E2E scenarios directly test C-OR-7 compliance. Aligned.

**Severity:** N/A (aligned)

---

### E-4: V11-3 `/ready` Endpoint — NO CONTRACT CONFLICT

**V11-3 proposes:** `/ready` endpoint checking PostgreSQL, MinIO, NATS connectivity

**Finding:** SR-CONTRACT does not define health/readiness semantics. This is an operational concern outside contract scope. No conflict.

**Severity:** N/A (not in CONTRACT scope)

---

## 3. Semantics Findings (SR-SPEC Alignment)

### S-1: Iteration Context Ref Schema — PARTIALLY INCONSISTENT

**V11-6 proposes GovernedArtifact ref:**
```
kind: "GovernedArtifact"
rel: "governed_by"
meta: { doc_id, content_hash, version }
```

**SR-SPEC §1.5.2 TypedRef schema:**
```json
{
  "kind": "...",
  "id": "...",
  "rel": "...",
  "meta": {
    "content_hash": "sha256:...",
    "version": "...",
    "type_key": "..."
  }
}
```

**Finding:** V11-6 is missing required `id` field. Per SR-SPEC §1.5.2:
> `id` — REQUIRED. Stable identifier (format varies by `kind`).

For `kind=GovernedArtifact`, the `id` should be the document identifier (e.g., `"SR-DIRECTIVE"`).

**Recommendation:** Update V11-6 schema to:
```
kind: "GovernedArtifact"
id: "SR-DIRECTIVE"
rel: "depends_on"
meta: { content_hash: "sha256:...", version: "...", type_key: "governance.dev_directive" }
```

**Severity:** High — schema incomplete

---

### S-2: Exception Ref Schema — PARTIALLY INCONSISTENT

**V11-6 proposes Exception ref:**
```
kind: "Exception"
rel: "waived_by"
meta: { exception_id, scope, expires_at }
```

**SR-SPEC §3.2.1.1 (item 6) defines:**
```
kind=Deviation|Deferral|Waiver, rel=depends_on
meta.expires_at, meta.scope
```

**Finding:** V11-6 puts `exception_id` in `meta`, but per SR-SPEC §1.5.2, the exception ID should be in the `id` field, not `meta`.

**Recommendation:** Update V11-6 schema to:
```
kind: "Waiver" (or "Deviation" or "Deferral")
id: "exc_01J..."
rel: "depends_on"
meta: { scope: "...", expires_at: "..." }
```

**Severity:** High — schema mismatch

---

### S-3: `SR-SUITE-INTEGRATION` Naming — ALIGNED

**V11-5 proposes:** Register as `suite:SR-SUITE-INTEGRATION`

**SR-SPEC §1.8 oracle suite ID format:**
> Oracle suite identifiers use `suite:` prefix

**Finding:** Naming convention is consistent with existing suites (`SR-SUITE-GOV`, `SR-SUITE-CORE`, `SR-SUITE-FULL`).

**Severity:** N/A (aligned)

---

### S-4: Metrics Naming — NO CONFLICT

**V11-3 proposes metrics:**
- `loop_lifecycle_duration_seconds`
- `iteration_total`
- `oracle_run_duration_seconds`
- `event_store_append_duration_seconds`
- `projection_rebuild_duration_seconds`

**Finding:** SR-SPEC does not define metrics naming conventions. These are implementation details. The names use reasonable patterns (snake_case, unit suffixes).

**Severity:** N/A (not in SPEC scope)

---

### S-5: `IterationStarted.refs[]` Extension — REQUIRES SPECIFICATION UPDATE

**V11-6 proposes:** Add `GovernedArtifact` refs to `IterationStarted.refs[]`

**SR-SPEC §3.2.1.1 lists minimum required ref categories:**
1. Loop
2. SR-DIRECTIVE (already uses `kind=GovernedArtifact`)
3. Previous Iteration (if any)
4. Input Candidate (if any)
5. Required Oracle Suite(s)
6. Active exceptions
7. Intervention notes (if any)
8. Evaluation/Assessment notes (when required)
9. Agent/worker definitions
10. Gating policy (if explicit)
11. Procedure template binding (if any)
12. Work Surface binding (for semantic work)

**Finding:** Item 2 already includes `kind=GovernedArtifact` for SR-DIRECTIVE! The plan's V11-6 appears to be extending this to additional governed artifacts (SR-CONTRACT, SR-SPEC).

**Clarification needed:** Is V11-6 adding *new* GovernedArtifact refs beyond SR-DIRECTIVE, or formalizing what SR-SPEC already requires?

**Recommendation:** Clarify V11-6 scope:
- If adding SR-CONTRACT/SR-SPEC refs: Document as extension beyond minimum required set
- If just ensuring SR-DIRECTIVE is included: Already required by SR-SPEC §3.2.1.1

**Severity:** Medium — scope clarification needed

---

## 4. Summary of Findings

| ID | Dimension | Severity | Summary |
|----|-----------|----------|---------|
| O-1 | Ontology | Medium | `GovernedArtifact` kind is valid; `rel` is not |
| O-2 | Ontology | **High** | `rel=governed_by` explicitly deprecated; use `rel=depends_on` |
| O-3 | Ontology | Medium | `Exception` not valid kind; use `Deviation\|Deferral\|Waiver` |
| O-4 | Ontology | Medium | `rel=waived_by` undefined; use `rel=depends_on` |
| O-5 | Ontology | Low | `GovernedManifest` needs clarification as internal type |
| E-1 | Epistemology | N/A | Active exceptions in refs — aligned |
| E-2 | Epistemology | Medium | GovernedArtifact hashing — may require spec update |
| E-3 | Epistemology | N/A | E2E harness assertions — aligned with C-OR-7 |
| E-4 | Epistemology | N/A | `/ready` endpoint — not in contract scope |
| S-1 | Semantics | **High** | GovernedArtifact ref missing `id` field |
| S-2 | Semantics | **High** | Exception ref schema mismatch (id in wrong place) |
| S-3 | Semantics | N/A | Oracle suite naming — aligned |
| S-4 | Semantics | N/A | Metrics naming — not in spec scope |
| S-5 | Semantics | Medium | GovernedArtifact refs scope needs clarification |

---

## 5. Required Corrections

### Must Fix (High Severity)

1. **O-2:** Change `rel: "governed_by"` to `rel: "depends_on"` for GovernedArtifact refs

2. **S-1:** Add required `id` field to GovernedArtifact ref schema:
   ```
   kind: "GovernedArtifact"
   id: "SR-DIRECTIVE"  // <-- ADD THIS
   rel: "depends_on"   // <-- CHANGE FROM governed_by
   meta: { content_hash: "sha256:...", version: "...", type_key: "governance.dev_directive" }
   ```

3. **S-2:** Fix Exception ref schema:
   ```
   kind: "Waiver"       // <-- NOT "Exception"
   id: "exc_01J..."     // <-- ID goes here, not in meta
   rel: "depends_on"    // <-- NOT "waived_by"
   meta: { scope: "...", expires_at: "..." }
   ```

### Should Fix (Medium Severity)

4. **O-3:** Use specific exception kind (`Waiver`, `Deviation`, or `Deferral`) instead of generic `Exception`

5. **O-4:** Replace `rel: "waived_by"` with `rel: "depends_on"` per SR-SPEC §3.2.1.1

6. **E-2:** Decide whether GovernedArtifact refs (beyond SR-DIRECTIVE) require SR-SPEC update or are documented as optional extension

7. **S-5:** Clarify V11-6 scope: Is this extending beyond minimum required refs, or just implementing existing requirement?

### Optional (Low Severity)

8. **O-5:** Add clarification that `GovernedManifest` is an internal implementation type

---

## 6. Recommended V11-6 Schema Revisions

### GovernedArtifact Refs (corrected)

```rust
// In IterationStarted.refs[]
{
    kind: "GovernedArtifact",
    id: "SR-DIRECTIVE",
    rel: "depends_on",
    meta: {
        content_hash: "sha256:abc123...",
        version: "1.0.0",
        type_key: "governance.dev_directive"
    }
}
```

### Active Exception Refs (corrected)

```rust
// In IterationStarted.refs[]
{
    kind: "Waiver",  // or "Deviation" or "Deferral"
    id: "exc_01J...",
    rel: "depends_on",
    meta: {
        scope: "per-candidate|per-loop|per-baseline|time-boxed",
        expires_at: "2026-02-01T00:00:00Z"
    }
}
```

---

## 7. Verdict

### **REVISE**

SR-PLAN-V11 contains terminology and schema inconsistencies that conflict with SR-TYPES, SR-CONTRACT, and SR-SPEC. The high-severity findings (O-2, S-1, S-2) must be corrected before implementation.

**Required actions:**
1. Update V11-6 Part A (GovernedArtifact refs) with corrected schema per §6
2. Update V11-6 Part B (Active Exception refs) with corrected schema per §6
3. Clarify whether V11-6 extends beyond minimum required refs (requires spec update decision)

Once these corrections are incorporated, the plan will be consistent with the canonical SR-* documents.

---

## Appendix: Documents Consulted

| Document | Sections Referenced |
|----------|---------------------|
| SR-PLAN-V11 | §2 (V11-6), §8 |
| SR-CONTRACT | §2.3, §2.5, §2.6, §2.7, §2.11, C-CTX-1, C-CTX-2, C-OR-7 |
| SR-TYPES | §1.1, §2.3, §4.3, §4.4 |
| SR-SPEC | §1.5.2, §1.5.3, §1.8, §1.12, §3.2.1.1 |
| SR-PLAN-V11-COHERENCE-REVIEW | §1.6 (V11-6 findings) |

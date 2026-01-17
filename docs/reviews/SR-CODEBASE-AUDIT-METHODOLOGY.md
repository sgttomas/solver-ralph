# SR-CODEBASE-AUDIT-METHODOLOGY

**Purpose:** Define the methodology for auditing SOLVER-Ralph codebase coherence and consistency against the canonical SR-* documents.

**Scope:** Contract compliance, schema alignment, type registry conformance, and gap identification.

**Validation:** This methodology was tested on 2026-01-17 against the actual codebase. The methods described below are proven to find real gaps.

---

## 1. Audit Objectives

1. **Contract Compliance:** Verify each SR-CONTRACT invariant (C-*) has corresponding enforcement in code
2. **Schema Alignment:** Verify SR-SPEC schemas match Rust struct definitions
3. **Type Conformance:** Verify SR-TYPES runtime type_keys have corresponding implementations
4. **API Coverage:** Verify SR-SPEC endpoints exist with correct behavior
5. **Gap Identification:** Identify missing implementations, partial implementations, and untested code paths

---

## 2. Audit Layers

### Layer 1: Contract → Code (SR-CONTRACT Invariants)

**Source:** SR-CONTRACT §1.1 (Invariants Index)

| Category | Invariants | Detection Method |
|----------|------------|------------------|
| Architecture | C-ARCH-1, C-ARCH-2, C-ARCH-3 | **Implicit** — module structure |
| Trust Boundaries | C-TB-1 through C-TB-7 | **Explicit** — grep + handler checks |
| Verification | C-VER-1 through C-VER-4 | **Explicit** — grep + logic review |
| Shippable | C-SHIP-1 | **Explicit** — grep + handler checks |
| Oracle Integrity | C-OR-1 through C-OR-7 | **Mixed** — grep + oracle runner review |
| Evidence Integrity | C-EVID-1 through C-EVID-6 | **Explicit** — grep + evidence.rs review |
| Event/Audit | C-EVT-1 through C-EVT-7 | **Explicit** — grep + event emission review |
| Loop Governance | C-LOOP-1 through C-LOOP-4, C-CTX-1, C-CTX-2 | **Explicit** — grep + governor review |
| Exceptions | C-EXC-1 through C-EXC-5 | **Explicit** — grep + exception handler review |
| Decisions | C-DEC-1 | **Explicit** — grep + decision handler review |
| Metadata | C-META-1 through C-META-3 | **Implicit** — frontmatter parsing |

#### 1.1 Explicit Detection Method

```bash
# Search for invariant ID citations in code
grep -rn "C-ARCH-1\|C-TB-1\|C-VER-1\|..." crates/
```

**For each match:** Verify the code actually enforces the invariant (not just comments).

**For each invariant with NO matches:** Use implicit detection or manual review.

#### 1.2 Implicit Detection Method (Architecture Invariants)

**C-ARCH-1 (Hexagonal Separation):**
```bash
# Verify sr-domain does NOT import infrastructure
grep -r "use sqlx\|use async_nats\|use aws_sdk\|use reqwest" crates/sr-domain/
# Expected: No matches

# Verify sr-ports does NOT import infrastructure
grep -r "use sqlx\|use async_nats\|use aws_sdk\|use reqwest" crates/sr-ports/
# Expected: No matches

# Verify sr-adapters DOES import infrastructure
grep -r "use sqlx\|use async_nats\|use aws_sdk\|use reqwest" crates/sr-adapters/
# Expected: Multiple matches
```

**C-ARCH-2 / C-ARCH-3:** Review crate dependency graph in Cargo.toml files.

#### 1.3 Invariant Checklist

For each invariant, record:
- [ ] Detection method used (explicit grep / implicit structure / manual review)
- [ ] Enforcement location (file:line or "module boundary")
- [ ] Test coverage (unit / integration / E2E / none)
- [ ] Status: ✅ Enforced | ⚠️ Partial | ❌ Missing | 🔍 Needs Review

---

### Layer 2: Spec → Code (SR-SPEC Schemas)

**Source:** SR-SPEC §1.5-1.12

**Method:** For each schema in SR-SPEC, find the corresponding Rust struct and compare fields.

| Section | Schema | Search Pattern | Expected Location |
|---------|--------|----------------|-------------------|
| §1.5.2 | Event envelope | `struct EventEnvelope` | `sr-domain/src/events.rs` |
| §1.5.3 | TypedRef | `struct TypedRef` | `sr-domain/src/events.rs` |
| §1.6.2 | PostgreSQL tables | `CREATE TABLE es.events` | `deploy/migrations/` |
| §1.9.1 | Evidence manifest | `struct EvidenceManifest` | `sr-adapters/src/evidence.rs` |
| §3.2.2 | Iteration summary | `struct IterationSummary` | `sr-domain/src/iteration.rs` |

**For each schema:**
```bash
# Find the struct definition
grep -rn "struct EventEnvelope" crates/

# Extract field list (manual: read 20 lines after match)
# Compare against SR-SPEC schema field by field
```

**Checklist per schema:**
- [ ] Struct exists with correct name
- [ ] All required fields present
- [ ] Field types match (timestamps, hashes, enums)
- [ ] Serialization annotations correct (`#[serde(...)]`)
- [ ] Canonical terms used (per SR-CONTRACT §2.11)

---

### Layer 3: Types → Code (SR-TYPES Registry)

**Source:** SR-TYPES §4

**Scope clarification:**
- §4.1 (Platform Spec) — **Document types only**, no code implementation needed
- §4.2 (Build Scaffolding) — **Document types only**, no code implementation needed
- §4.3 (Platform Domain) — **Runtime types**, code implementation REQUIRED
- §4.4 (Operational Records) — **Runtime types**, code implementation REQUIRED
- §4.5 (Configuration) — **Runtime types**, code implementation REQUIRED

**Method:** For each runtime type_key, search for usage in code.

```bash
# Search for type_key usage
grep -rn "domain\.work_unit\|domain\.candidate\|record\.waiver" crates/

# Search for corresponding Rust types
grep -rn "struct WorkUnit\|struct Candidate\|struct Waiver" crates/
```

**§4.3 Platform Domain Types:**

| Type Key | Search Pattern | Status |
|----------|----------------|--------|
| `domain.work_unit` | `domain.work_unit` or `WorkUnit` | |
| `domain.work_surface` | `domain.work_surface` or `WorkSurface` | |
| `domain.candidate` | `domain.candidate` or `Candidate` | |
| `domain.evidence_bundle` | `domain.evidence_bundle` or `EvidenceBundle` | |
| `domain.portal_decision` | `domain.portal_decision` or `PortalDecision` | |
| `domain.loop_record` | `domain.loop_record` or `LoopRecord` | |
| `domain.event` | `domain.event` or `EventEnvelope` | |

**§4.4 Operational Records:**

| Type Key | Search Pattern | Status |
|----------|----------------|--------|
| `record.decision` | `record.decision` or `Decision` | |
| `record.waiver` | `record.waiver` or `Waiver` | |
| `record.deviation` | `record.deviation` or `Deviation` | |
| `record.deferral` | `record.deferral` or `Deferral` | |
| `record.evaluation_note` | `evaluation_note` or `EvaluationNote` | |
| `record.assessment_note` | `assessment_note` or `AssessmentNote` | |
| `record.intervention_note` | `intervention_note` or `InterventionNote` | |
| `record.intake` | `record.intake` or `Intake` | |
| `record.procedure_instance` | `procedure_instance` or `ProcedureInstance` | |
| `record.attachment` | `record.attachment` or `Attachment` | |

---

### Layer 4: API → Spec (SR-SPEC Endpoints)

**Source:** SR-SPEC §2.3

**Method:** Extract all routes from sr-api/src/main.rs and compare against SR-SPEC §2.3.

```bash
# List all routes in code
grep -n "\.route\(" crates/sr-api/src/main.rs

# Compare against SR-SPEC endpoint list
grep -n "POST /\|GET /\|PUT /\|PATCH /\|DELETE /" docs/platform/SR-SPEC.md
```

**Endpoint checklist:**

| Spec Endpoint | Code Route | Handler | Status |
|---------------|------------|---------|--------|
| `POST /loops` | `/api/v1/loops` | `loops::create_loop` | |
| `GET /loops/{id}` | `/api/v1/loops/:loop_id` | `loops::get_loop` | |
| `POST /records/evaluation-notes` | ? | ? | |
| `POST /records/assessment-notes` | ? | ? | |
| `POST /staleness/mark` | ? | ? | |
| `GET /staleness/dependents` | ? | ? | |
| `POST /staleness/{id}/resolve` | ? | ? | |

**For each endpoint:**
- [ ] Route exists in main.rs
- [ ] Handler function exists
- [ ] Request body matches spec
- [ ] Response body matches spec
- [ ] Events emitted match SR-SPEC Appendix A
- [ ] Actor constraints enforced (HUMAN-only checks)

---

## 3. Audit Execution Process

### Phase 1: Systematic Search (2-3 hours)

1. **Layer 1:** Grep for all C-* invariant IDs, record matches
2. **Layer 1:** Run implicit checks for C-ARCH-* invariants
3. **Layer 2:** For each SR-SPEC schema, find and compare Rust struct
4. **Layer 3:** For each §4.3/§4.4 type_key, search for implementation
5. **Layer 4:** Compare main.rs routes against SR-SPEC §2.3 endpoints

### Phase 2: Gap Analysis (1-2 hours)

1. For invariants with no grep matches, determine if implicitly enforced
2. For schemas with missing fields, document the delta
3. For missing type_keys, document as unimplemented
4. For missing endpoints, document as unimplemented

### Phase 3: Prioritization (30 min)

Classify findings:
- **P0:** Invariant violations (code violates CONTRACT)
- **P1:** Missing enforcement (CONTRACT requirement not enforced)
- **P2:** Missing implementation (SPEC feature not built)
- **P3:** Test gaps (enforcement exists but no test)

---

## 4. Deliverable Format

The audit produces `docs/reviews/SR-CODEBASE-AUDIT-FINDINGS.md` with:

### Section A: Contract Compliance Matrix

| Invariant | Description | Detection | Enforcement | Test | Status |
|-----------|-------------|-----------|-------------|------|--------|
| C-ARCH-1 | Hexagonal separation | Implicit | Module boundaries | — | ✅ |
| C-TB-3 | Human-only approval | Explicit | `approvals.rs:122` | `branch_0_e2e_test.rs:854` | ✅ |
| C-OR-7 | Integrity halt | Explicit | `integrity.rs` | `e2e_harness` | ⚠️ |

### Section B: Schema Alignment Report

| Schema | Spec Section | Code Location | Fields Match | Issues |
|--------|--------------|---------------|--------------|--------|
| EventEnvelope | §1.5.2 | `events.rs:57` | 15/15 | None |
| TypedRef | §1.5.3 | `events.rs:82` | 5/5 | None |

### Section C: Type Registry Coverage

| Type Key | Category | Rust Type | Usage Count | Status |
|----------|----------|-----------|-------------|--------|
| `domain.work_unit` | §4.3 | `WorkUnit` | 47 | ✅ |
| `record.assessment_note` | §4.4 | — | 0 | ❌ Missing |
| `record.evaluation_note` | §4.4 | — | 0 | ❌ Missing |
| `record.intervention_note` | §4.4 | — | 0 | ❌ Missing |

### Section D: API Endpoint Coverage

| Spec Endpoint | Code Route | Status |
|---------------|------------|--------|
| `POST /loops` | `/api/v1/loops` | ✅ |
| `POST /records/evaluation-notes` | — | ❌ Missing |
| `POST /records/assessment-notes` | — | ❌ Missing |
| `POST /staleness/mark` | — | ❌ Missing |
| `GET /staleness/dependents` | — | ❌ Missing |
| `POST /staleness/{id}/resolve` | — | ❌ Missing |

### Section E: Prioritized Remediation List

| Priority | Finding | Invariant/Spec | Effort | Action |
|----------|---------|----------------|--------|--------|
| P2 | Missing staleness endpoints | SR-SPEC §2.3.11 | 1 session | Implement `src/handlers/staleness.rs` |
| P2 | Missing record note types | SR-TYPES §4.4 | 0.5 session | Add structs + endpoints |
| P3 | C-OR-7 test incomplete | C-OR-7 | 0.5 session | Extend E2E harness |

---

## 5. Success Criteria

The audit is complete when:

1. Every C-* invariant has a status determination (explicit or implicit method documented)
2. Every SR-SPEC §1.5-1.12 schema has been compared to code
3. Every SR-TYPES §4.3/§4.4/§4.5 type_key has been traced
4. Every SR-SPEC §2.3 endpoint has been checked
5. All findings are classified by priority (P0/P1/P2/P3)
6. Remediation list is actionable (specific files, specific changes)

---

## 6. Known Gaps (from methodology validation)

The following gaps were identified during methodology testing on 2026-01-17:

**Missing Types (SR-TYPES §4.4):**
- `record.assessment_note` — no implementation
- `record.evaluation_note` — no implementation
- `record.intervention_note` — no implementation

**Missing Endpoints (SR-SPEC §2.3):**
- `POST /records/evaluation-notes` — no handler
- `POST /records/assessment-notes` — no handler
- `POST /staleness/mark` — no handler
- `GET /staleness/dependents` — no handler
- `POST /staleness/{stale_id}/resolve` — no handler

These should be verified (or implemented) during the full audit.

---

## 7. Related Documents

- `docs/platform/SR-CONTRACT.md` — Invariant definitions (C-* identifiers)
- `docs/platform/SR-SPEC.md` — Schema and endpoint definitions
- `docs/platform/SR-TYPES.md` — Type registry (§4.3/§4.4/§4.5 = runtime types)
- `docs/planning/SR-PLAN-GAP-ANALYSIS.md` — Deliverable status

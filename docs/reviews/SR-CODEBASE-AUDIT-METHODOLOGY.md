# SR-CODEBASE-AUDIT-METHODOLOGY

**Purpose:** Define the methodology for auditing SOLVER-Ralph codebase coherence and consistency against the canonical SR-* documents.

**Scope:** Four-dimensional analysis across ontology, epistemology, semantics, and praxeology.

**Validation:** This methodology was tested on 2026-01-17 against the actual codebase. The methods described below are proven to find real gaps.

---

## 1. Theoretical Framework

The audit analyzes codebase alignment across four philosophical dimensions, each governed by a canonical document:

| Dimension | Document | Question | Audit Focus |
|-----------|----------|----------|-------------|
| **Ontological** | SR-TYPES | *What kinds of things exist?* | Type registry coverage |
| **Epistemological** | SR-CONTRACT | *What can we know/guarantee?* | Invariant enforcement |
| **Semantic** | SR-SPEC | *What do things mean?* | Schema and API alignment |
| **Praxeological** | SR-DIRECTIVE | *How must agents act?* | Execution model compliance |

**Praxeology** (from Greek *praxis* "action" + *logos* "study") is the study of the logic of purposeful action. SR-DIRECTIVE governs how agents pursue goals within constraints — the execution loop, budget enforcement, stop triggers, gate evaluation, and portal routing. This dimension is essential because SOLVER-Ralph is a platform for governed agent execution; verifying correct action logic is as fundamental as verifying correct data structures.

**Note on methodology-specific terms:** This document introduces a priority classification (P0/P1/P2/P3) for audit findings. This classification is specific to this audit methodology and is not defined in the canonical SR-* documents. It is a tool for triage, not a governance primitive.

---

## 2. Audit Objectives

1. **Ontological (SR-TYPES):** Verify runtime type_keys have corresponding implementations
2. **Epistemological (SR-CONTRACT):** Verify each invariant (C-*) has corresponding enforcement in code
3. **Semantic (SR-SPEC):** Verify schemas match Rust structs; endpoints exist with correct behavior
4. **Praxeological (SR-DIRECTIVE):** Verify execution model, budgets, stop triggers, and gates are correctly implemented
5. **Gap Identification:** Identify missing implementations, partial implementations, and untested code paths

---

## 3. Audit Layers

### Layer 1: Ontological — SR-TYPES → Code

**Source:** SR-TYPES §4

**Question:** *Do the kinds of things the spec says exist actually exist in code?*

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

| Type Key | Search Pattern | Status | Notes |
|----------|----------------|--------|-------|
| `domain.work_unit` | `domain.work_unit` or `WorkUnit` | | |
| `domain.work_surface` | `domain.work_surface` or `WorkSurface` | | Critical for SR-DIRECTIVE §2.4 |
| `domain.candidate` | `domain.candidate` or `Candidate` | | |
| `domain.evidence_bundle` | `domain.evidence_bundle` or `EvidenceBundle` | | |
| `domain.portal_decision` | `domain.portal_decision` or `PortalDecision` | | |
| `domain.loop_record` | `domain.loop_record` or `LoopRecord` | | |
| `domain.event` | `domain.event` or `EventEnvelope` | | |
| `domain.intake` | `domain.intake` or `Intake` | | Work Surface component |
| `domain.procedure_template` | `procedure_template` or `ProcedureTemplate` | | Work Surface component |

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

### Layer 2: Epistemological — SR-CONTRACT → Code

**Source:** SR-CONTRACT §1.1 (Invariants Index)

**Question:** *Are the guarantees the contract makes actually enforced?*

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

#### 2.1 Explicit Detection Method

```bash
# Search for invariant ID citations in code
grep -rn "C-ARCH-1\|C-TB-1\|C-VER-1\|..." crates/
```

**For each match:** Verify the code actually enforces the invariant (not just comments).

**For each invariant with NO matches:** Use implicit detection or manual review.

#### 2.2 Implicit Detection Method (Architecture Invariants)

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

#### 2.3 Invariant Checklist

For each invariant, record:
- [ ] Detection method used (explicit grep / implicit structure / manual review)
- [ ] Enforcement location (file:line or "module boundary")
- [ ] Test coverage (unit / integration / E2E / none)
- [ ] Status: Enforced | Partial | Missing | Needs Review

---

### Layer 3: Semantic — SR-SPEC → Code

**Source:** SR-SPEC §1.5-1.12 (Schemas), §2.3 (Endpoints)

**Question:** *Do the structures and behaviors in code match what the spec says they mean?*

#### 3.1 Schema Alignment

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

#### 3.2 API Endpoint Alignment

**Source:** SR-SPEC §2.3

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

### Layer 4: Praxeological — SR-DIRECTIVE → Code

**Source:** SR-DIRECTIVE §2-§9

**Question:** *Does the code correctly implement the logic of action — how agents must behave within the governed execution model?*

This layer audits the execution dynamics: loops, iterations, budgets, stop conditions, gates, portals, and decision procedures.

#### 4.1 Canonical Loop (§2.1)

The canonical loop skeleton must be enforceable by the governor:

| Step | Event | Code Location | Verification |
|------|-------|---------------|--------------|
| 1 | IterationStarted | `governor.rs` | Verify refs include governed artifacts, oracle suites, base candidate |
| 2 | CandidateSubmitted | `candidates.rs` | Verify candidate lifecycle |
| 3 | OracleRunRequested/Completed | `oracle_runner.rs` | Verify oracle execution flow |
| 4 | EvidenceBundleRecorded | `evidence.rs` | Verify bundle aggregation |
| 5 | Gate evaluation | `gates.rs` or handler | Verify Start → Accept → Release flow |
| 6 | Portal crossing | `portals.rs` | Verify human-only routing |
| 7 | FreezeRecordCreated | `freeze.rs` | Verify human-only + approval acknowledgment |

```bash
# Verify event sequence is emittable
grep -rn "IterationStarted\|CandidateSubmitted\|OracleRunRequested\|EvidenceBundleRecorded" crates/
```

#### 4.2 Budget Enforcement (§4.1)

SR-DIRECTIVE specifies default per-work-unit budgets:

| Budget Parameter | Directive Value | Code Default | Location |
|------------------|-----------------|--------------|----------|
| `max_iterations` | 5 | ? | `governor.rs` LoopBudget |
| `max_oracle_runs` | 25 | ? | `governor.rs` or oracle_runner |
| `max_wallclock_hours` | 16 | ? | `governor.rs` max_duration_secs |

```bash
# Find budget defaults
grep -rn "max_iterations\|max_oracle_runs\|max_duration" crates/sr-adapters/src/governor.rs
```

**Note:** Code defaults may be more permissive if Plan-to-Workflow mapping overrides are intended. Document the rationale.

#### 4.3 Stop Triggers (§4.2)

SR-DIRECTIVE §4.2 defines 12 stop triggers. Verify all are implemented:

| Stop Trigger | Code Enum Variant | Routing Target |
|--------------|-------------------|----------------|
| BUDGET_EXHAUSTED | `StopCondition::BudgetExhausted` | HumanAuthorityExceptionProcess |
| EVIDENCE_MISSING | ? | HumanAuthorityExceptionProcess |
| INTEGRITY_VIOLATION | `StopCondition::IntegrityCondition` | GovernanceChangePortal |
| ORACLE_ENV_MISMATCH | `IntegrityCondition::OracleEnvMismatch` | GovernanceChangePortal |
| ORACLE_FLAKE | `IntegrityCondition::OracleFlake` | GovernanceChangePortal |
| ORACLE_GAP | `IntegrityCondition::OracleGap` | GovernanceChangePortal |
| ORACLE_TAMPER | `IntegrityCondition::OracleTamper` | GovernanceChangePortal |
| REPEATED_FAILURE | ? | HumanAuthorityExceptionProcess |
| NO_ELIGIBLE_WORK | ? | (loop terminates) |
| WORK_SURFACE_MISSING | `StopCondition::WorkSurfaceMissing` | HumanAuthorityExceptionProcess |
| STAGE_UNKNOWN | ? | HumanAuthorityExceptionProcess |
| SEMANTIC_PROFILE_MISSING | ? | HumanAuthorityExceptionProcess |

```bash
# Find StopCondition enum
grep -rn "enum StopCondition" crates/

# Find IntegrityCondition enum
grep -rn "enum IntegrityCondition" crates/
```

#### 4.4 Non-Waivable Conditions (§5.2)

SR-DIRECTIVE §5.2 declares these conditions as non-waivable:
- ORACLE_TAMPER
- ORACLE_GAP
- ORACLE_ENV_MISMATCH
- ORACLE_FLAKE
- EVIDENCE_MISSING

Verify that `IntegrityCondition::requires_escalation()` returns `true` for all of these.

```bash
# Find requires_escalation method
grep -rn "requires_escalation\|fn.*escalat" crates/sr-adapters/src/integrity.rs
```

#### 4.5 Portal Routing (§6)

SR-DIRECTIVE §6 defines exactly three seeded portal identities:
- `HumanAuthorityExceptionProcess`
- `GovernanceChangePortal`
- `ReleaseApprovalPortal`

Verify code only accepts these portal IDs for routing.

```bash
# Find portal ID validation
grep -rn "HumanAuthorityExceptionProcess\|GovernanceChangePortal\|ReleaseApprovalPortal" crates/
```

#### 4.6 Deterministic Eligibility (§2.3)

SR-DIRECTIVE §2.3 requires:
- Eligibility computation MUST be performed by a deterministic system component (Event Manager / Projection Builder)
- Agent MAY choose eligible work unit but MUST NOT compute eligibility

Verify this separation exists in code.

#### 4.7 Oracle Suites and Profiles (§5)

SR-DIRECTIVE §5 defines canonical oracle suite IDs. Verify code recognizes these:

| Suite ID | Purpose | Code Recognition |
|----------|---------|------------------|
| `suite:SR-SUITE-GOV` | Governance/metadata checks | |
| `suite:SR-SUITE-CORE` | Core deterministic checks | |
| `suite:SR-SUITE-FULL` | Core + integration + proofs | |
| `suite:SR-SUITE-SEMANTIC` | Semantic evaluation oracles | |
| `suite:SR-SUITE-INTEGRATION` | Integration tests | |

```bash
# Find suite ID references
grep -rn "SR-SUITE-GOV\|SR-SUITE-CORE\|SR-SUITE-FULL\|SR-SUITE-SEMANTIC\|SR-SUITE-INTEGRATION" crates/
```

#### 4.8 Gate Registry (§7)

SR-DIRECTIVE §7 references the Gate Registry (defined in `SR-DIRECTIVE.KIT-GATE-REGISTRY.md`). Verify gate evaluation logic exists:

```bash
# Find gate evaluation code
grep -rn "gate_id\|GateEvaluation\|StartGate\|AcceptGate\|ReleaseGate" crates/
```

#### 4.9 Praxeological Checklist

For each SR-DIRECTIVE section, record:
- [ ] Section reference (§X.Y)
- [ ] Code implementation location
- [ ] Alignment status: Aligned | Divergent | Missing
- [ ] If divergent: document rationale or flag as P1

**Note:** Some praxeological requirements (e.g., §2.3 "agent MUST NOT compute eligibility") cannot be verified by grep alone. These require manual code review to confirm the separation of concerns is architecturally enforced.

---

## 4. Audit Execution Process

### Phase 1: Systematic Search (3-4 hours)

1. **Layer 1 (Ontological):** For each §4.3/§4.4 type_key, search for implementation
2. **Layer 2 (Epistemological):** Grep for all C-* invariant IDs; run implicit checks for C-ARCH-*
3. **Layer 3 (Semantic):** Compare SR-SPEC schemas to Rust structs; compare endpoints to routes
4. **Layer 4 (Praxeological):** Verify execution model, budgets, stop triggers, gates, portals

### Phase 2: Gap Analysis (1-2 hours)

1. For missing type_keys, document as unimplemented (ontological gap)
2. For invariants with no enforcement, document as violation (epistemological gap)
3. For schemas with missing fields or endpoints, document delta (semantic gap)
4. For execution model divergence, document and assess severity (praxeological gap)

### Phase 3: Prioritization (30 min)

Classify findings:
- **P0:** Invariant violations (code actively violates CONTRACT or DIRECTIVE)
- **P1:** Missing enforcement (CONTRACT/DIRECTIVE requirement not enforced)
- **P2:** Missing implementation (SPEC/TYPES feature not built)
- **P3:** Test gaps (enforcement exists but no test)

### Phase 4: Escalation (if P0 found)

If any P0 finding is identified:

1. **STOP** further audit execution
2. **Document** the P0 finding with full context
3. **Escalate** to `GovernanceChangePortal` with `request_type: INTEGRITY_VIOLATION`
4. **Do not proceed** with remediation until governance review completes

This mirrors the C-OR-7 "integrity halt" principle: the audit itself is governed work, and integrity violations require escalation before continuation.

### Phase 5: Spec-vs-Spec Inconsistency Handling

If the audit reveals inconsistencies *between* canonical documents (e.g., SR-DIRECTIVE says X, SR-CONTRACT says Y):

1. **Document** both sources and the contradiction
2. **Escalate** to `GovernanceChangePortal` with `request_type: GOVERNANCE_CHANGE`
3. **Suspend** findings in the affected area until resolution
4. **Do not assume** which document takes precedence (per SR-DIRECTIVE §1.3, CONTRACT/SPEC control, but the conflict must be explicitly resolved)

---

## 5. Deliverable Format

### 5.1 Recording Location

The audit may be executed by multiple agents concurrently. Each agent performs the **complete audit across all four dimensions** and produces an independent findings document.

**Before beginning the audit, the agent MUST obtain from the human:**
1. **Recording location** — the file path where this agent's findings should be written

If this information has not been provided, the agent MUST ask before proceeding.

The human will consolidate findings from multiple agents after completion.

### 5.2 Findings Structure

Each agent's findings document should include a header identifying:
- Agent identifier or session ID
- Assigned scope
- Audit date
- Commit hash of codebase audited
- Canonical document versions referenced (content hashes if available)

Then, for each layer in scope, use the appropriate section template below.

---

#### Template A: Ontological Coverage (SR-TYPES)

| Type Key | Category | Rust Type | Usage Count | Status |
|----------|----------|-----------|-------------|--------|
| `domain.work_unit` | §4.3 | `WorkUnit` | 47 | Implemented |
| `record.assessment_note` | §4.4 | — | 0 | Missing |

#### Template B: Epistemological Compliance (SR-CONTRACT)

| Invariant | Description | Detection | Enforcement | Test | Status |
|-----------|-------------|-----------|-------------|------|--------|
| C-ARCH-1 | Hexagonal separation | Implicit | Module boundaries | — | Enforced |
| C-TB-3 | Human-only approval | Explicit | `approvals.rs:122` | E2E | Enforced |
| C-OR-7 | Integrity halt | Explicit | `integrity.rs` | E2E | Partial |

#### Template C: Semantic Alignment (SR-SPEC)

| Schema/Endpoint | Spec Section | Code Location | Match | Issues |
|-----------------|--------------|---------------|-------|--------|
| EventEnvelope | §1.5.2 | `events.rs:57` | 15/15 | None |
| `POST /staleness/mark` | §2.3.11 | — | Missing | Not implemented |

#### Template D: Praxeological Compliance (SR-DIRECTIVE)

| Directive Section | Requirement | Code Location | Status | Notes |
|-------------------|-------------|---------------|--------|-------|
| §4.1 Budget defaults | max_iterations=5 | `governor.rs:45` | Divergent | Code uses 100; verify if override intended |
| §4.2 Stop triggers | 12 triggers | `governor.rs`, `integrity.rs` | Partial | Missing: REPEATED_FAILURE, NO_ELIGIBLE_WORK, STAGE_UNKNOWN, SEMANTIC_PROFILE_MISSING |
| §5.2 Non-waivable | 5 conditions | `integrity.rs` | Aligned | requires_escalation() returns true |
| §6 Portal IDs | 3 seeded portals | — | Needs Review | Verify validation exists |

#### Template E: Prioritized Remediation List

| Priority | Dimension | Finding | Source | Effort | Action |
|----------|-----------|---------|--------|--------|--------|
| P1 | Praxeological | Missing stop triggers | SR-DIRECTIVE §4.2 | 1 session | Add missing StopCondition variants |
| P2 | Semantic | Missing staleness endpoints | SR-SPEC §2.3.11 | 1 session | Implement handlers |
| P2 | Ontological | Missing record note types | SR-TYPES §4.4 | 0.5 session | Add structs + endpoints |

---

## 6. Success Criteria

The audit is complete when:

1. **Ontological:** Every SR-TYPES §4.3/§4.4/§4.5 type_key has been traced
2. **Epistemological:** Every C-* invariant has a status determination
3. **Semantic:** Every SR-SPEC schema and endpoint has been compared to code
4. **Praxeological:** Every SR-DIRECTIVE §2-§9 section has been verified
5. All findings are classified by priority (P0/P1/P2/P3) and dimension
6. Remediation list is actionable (specific files, specific changes)

---

## 7. Known Gaps (from methodology validation)

The following gaps were identified during methodology testing on 2026-01-17:

**Ontological (SR-TYPES §4.4):**
- `record.assessment_note` — no implementation
- `record.evaluation_note` — no implementation
- `record.intervention_note` — no implementation

**Semantic (SR-SPEC §2.3):**
- `POST /records/evaluation-notes` — no handler
- `POST /records/assessment-notes` — no handler
- `POST /staleness/mark` — no handler
- `GET /staleness/dependents` — no handler
- `POST /staleness/{stale_id}/resolve` — no handler

**Praxeological (SR-DIRECTIVE):**
- §4.1 Budget defaults diverge (code: 100/3600, directive: 5/57600)
- §4.2 Missing stop triggers: EVIDENCE_MISSING, REPEATED_FAILURE, NO_ELIGIBLE_WORK, STAGE_UNKNOWN, SEMANTIC_PROFILE_MISSING

These should be verified (or implemented) during the full audit.

---

## 8. Related Documents

| Dimension | Document | Role |
|-----------|----------|------|
| Ontological | `docs/platform/SR-TYPES.md` | Type registry (§4.3/§4.4/§4.5 = runtime types) |
| Epistemological | `docs/platform/SR-CONTRACT.md` | Invariant definitions (C-* identifiers) |
| Semantic | `docs/platform/SR-SPEC.md` | Schema and endpoint definitions |
| Praxeological | `docs/program/SR-DIRECTIVE.md` | Execution model and governance |

Supporting:
- `docs/planning/SR-PLAN-GAP-ANALYSIS.md` — Deliverable status

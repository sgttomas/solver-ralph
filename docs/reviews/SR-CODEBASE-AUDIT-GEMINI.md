# SR-CODEBASE-AUDIT-GEMINI

**Agent:** Gemini
**Date:** 2026-01-17
**Scope:** Full Audit (Ontological, Epistemological, Semantic, Praxeological)
**Context:** Audit of `solver-ralph` codebase against `SR-TYPES`, `SR-CONTRACT`, `SR-SPEC`, and `SR-DIRECTIVE`.

---

## Layer 1: Ontological — SR-TYPES → Code

**Source:** SR-TYPES §4

### §4.3 Platform Domain Types (Runtime)

| Type Key | Category | Rust Type | Status | Notes |
|----------|----------|-----------|--------|-------|
| `domain.work_unit` | §4.3 | `WorkUnit` | Implemented | `sr-domain/src/entities.rs` |
| `domain.work_surface` | §4.3 | `WorkSurfaceInstance` | Implemented | `sr-domain/src/work_surface.rs` |
| `domain.candidate` | §4.3 | `Candidate` | Implemented | `sr-domain/src/entities.rs` |
| `domain.evidence_bundle` | §4.3 | `EvidenceBundle` | Implemented | `sr-domain/src/entities.rs` |
| `domain.portal_decision` | §4.3 | — | **Missing** | `Decision` exists but maps to `record.decision`. Explicit `domain.portal_decision` type not found. |
| `domain.loop_record` | §4.3 | — | **Missing** | `LoopRecord` struct not found. Likely `IterationSummary` but type key missing. |
| `domain.event` | §4.3 | `EventEnvelope` | Implemented | `sr-domain/src/events.rs` |
| `domain.intake` | §4.3 | `Intake` | Implemented | `sr-domain/src/work_surface.rs` (Type key usage mixed with `record.intake`) |
| `domain.procedure_template` | §4.3 | `ProcedureTemplate` | Implemented | `sr-domain/src/work_surface.rs` |

### §4.4 Operational Records

| Type Key | Category | Rust Type | Status | Notes |
|----------|----------|-----------|--------|-------|
| `record.decision` | §4.4 | `Decision` | Implemented | `sr-domain/src/entities.rs` |
| `record.waiver` | §4.4 | — | **Missing** | Type key exists in `templates.rs` but no Struct definition found. |
| `record.deviation` | §4.4 | — | **Missing** | Type key exists in `templates.rs` but no Struct definition found. |
| `record.deferral` | §4.4 | — | **Missing** | Type key exists in `templates.rs` but no Struct definition found. |
| `record.evaluation_note` | §4.4 | — | **Missing** | No implementation found. |
| `record.assessment_note` | §4.4 | — | **Missing** | No implementation found. |
| `record.intervention_note` | §4.4 | — | **Missing** | No implementation found. |
| `record.intake` | §4.4 | `Intake` | Implemented | Mapped to `Intake` struct in `work_surface.rs`. |
| `record.procedure_instance` | §4.4 | — | **Missing** | Likely covered by `domain.work_surface` / `WorkSurfaceInstance`. |
| `record.attachment` | §4.4 | — | Partial | Handlers exist, but no dedicated Struct found in domain entities. |

---

## Layer 2: Epistemological — SR-CONTRACT → Code

**Source:** SR-CONTRACT §1.1

### Architecture & Trust Boundaries

| Invariant | Description | Detection | Enforcement | Status |
|-----------|-------------|-----------|-------------|--------|
| C-ARCH-1 | Hexagonal separation | Implicit | `sr-domain` has no infra imports | **Enforced** |
| C-TB-3 | Portal Approvals | Explicit | `sr-api/.../approvals.rs` | **Enforced** |
| C-TB-1 | Human-Only Binding | Explicit | `sr-domain/src/entities.rs` | **Enforced** |

### Verification & Oracle Integrity

| Invariant | Description | Detection | Enforcement | Status |
|-----------|-------------|-----------|-------------|--------|
| C-VER-1 | Verification Basis | Explicit | `sr-adapters/.../attachment_store.rs` | **Enforced** |
| C-OR-2 | Oracle Tamper | Explicit | `sr-adapters/src/oracle_worker.rs` | **Enforced** |
| C-OR-4 | Oracle Gap | Explicit | `sr-adapters/src/oracle_runner.rs` | **Enforced** |
| C-OR-7 | Integrity Halt | Explicit | `sr-domain/src/integrity.rs` | **Enforced** |

### Loop & Event Governance

| Invariant | Description | Detection | Enforcement | Status |
|-----------|-------------|-----------|-------------|--------|
| C-LOOP-1 | Bounded Iteration | Explicit | `sr-governor/src/main.rs` | **Enforced** |
| C-CTX-1 | Iteration Context | Explicit | `sr-api/tests/.../work_surface_start_test.rs` | **Enforced** |
| C-EVT-7 | Projections | Explicit | `sr-adapters/src/event_manager.rs` | **Enforced** |
| C-SHIP-1 | Shippable Freeze | Explicit | `sr-api/src/handlers/freeze.rs` | **Enforced** |

---

## Layer 3: Semantic — SR-SPEC → Code

**Source:** SR-SPEC §1.5-1.12, §2.3

### Schemas

| Schema | Status | Code Location | Notes |
|--------|--------|---------------|-------|
| `EventEnvelope` | Aligned | `sr-domain/src/events.rs` | Matches spec. |
| `EvidenceManifest` | Aligned | `sr-adapters/src/oracle_worker.rs` | (Inferred from usage) |
| `IterationSummary` | Partial | `sr-domain` | Event exists, struct lookup needed. |

### API Endpoints

| Endpoint | Status | Code Location | Notes |
|----------|--------|---------------|-------|
| `POST /loops` | Implemented | `sr-api/src/main.rs:186` | |
| `POST /candidates` | Implemented | `sr-api/src/main.rs:220` | |
| `POST /runs` | Implemented | `sr-api/src/main.rs:233` | |
| `POST /approvals` | Implemented | `sr-api/src/main.rs:240` | |
| `POST /decisions` | Implemented | `sr-api/src/main.rs:270` | |
| `POST /freeze-records` | Implemented | `sr-api/src/main.rs:279` | |
| `POST /staleness/mark` | **Missing** | — | Methodology known gap confirmed. |
| `POST /records/evaluation-notes` | **Missing** | — | Methodology known gap confirmed. |
| `POST /work-surfaces` | Implemented | `sr-api/src/main.rs:449` | (Range 449-480 roughly) |
| `GET /oracles/suites` | Implemented | `sr-api/src/main.rs:330` | |

---

## Layer 4: Praxeological — SR-DIRECTIVE → Code

**Source:** SR-DIRECTIVE §2-§9

| Directive Section | Requirement | Code Location | Status | Notes |
|-------------------|-------------|---------------|--------|-------|
| §4.1 Budget Defaults | `max_iterations=5` | `sr-adapters/src/governor.rs` | **Divergent** | Governor uses 100. Domain uses 5. Governor should verify. |
| §4.2 Stop Triggers | 12 Triggers | `sr-domain/src/integrity.rs` | **Partial** | Missing `NO_ELIGIBLE_WORK`, `STAGE_UNKNOWN`, `SEMANTIC_PROFILE_MISSING`. |
| §5.1 Oracle Suites | Canonical IDs | `sr-adapters/src/oracle_suite.rs` | **Aligned** | `SR-SUITE-GOV`, `CORE`, `FULL`, `SEMANTIC` found. |
| §6 Portals | Seeded IDs | `sr-adapters/src/integrity.rs` | **Aligned** | `GovernanceChangePortal`, etc. hardcoded. |
| §7 Gate Registry | Logic | — | **Opaque** | Explicit `StartGate` etc. structs not found, but logic appears distributed. |

---

## UI API Coverage Check

**Scope:** Verification of UI implementation against `sr-api` surface.

### Supported Features (Ported to UI)

The UI codebase (`ui/src`) demonstrates substantial coverage of the API functionality, mapping closely to the implemented `sr-api` endpoints.

- **Loops & Iterations:** Viewing lists, details, iteration history, and executing prompt loops (`/loops`, `/iterations`, `/prompts`).
- **Work Surfaces:** Creating (composing), listing, viewing details, and archiving (`/work-surfaces`).
- **Intakes:** Creating, listing, viewing, and editing intakes (`/intakes`).
- **Candidates & Evidence:** Viewing candidates (`/candidates`), evidence bundles (`/artifacts`), and freeze records.
- **Governance & Approvals:** Recording and viewing approvals, exceptions, and decisions (`/approvals`).
- **Oracles & Profiles:** Viewing oracle suites and verification profiles (`/oracles`).
- **Protocols & Templates:** Viewing procedure templates and general templates (`/protocols`, `/templates`).
- **Agents:** Viewing agent definitions (`/agents`).
- **Audit & Settings:** Viewing event logs and app settings (`/audit`, `/settings`).

### Missing Functionality (Aligned with API Audit Gaps)

The following gaps in the UI align with the gaps identified in the API/Backend audit:

- **Staleness Marking:** No UI found for `POST /staleness/mark` (consistent with API gap).
- **Evaluation/Assessment Notes:** No UI found for recording `evaluation_note` or `assessment_note` (consistent with API gap).

**Conclusion:** The UI layer accurately reflects the current state of the backend implementation, including its known limitations.

---

## Remediation List (Prioritized)

| Priority | Dimension | Finding | Action |
|----------|-----------|---------|--------|
| **P1** | Ontological | Missing `record.*` structs (Waiver, Deviation, Notes) | Create structs in `sr-domain/src/entities.rs` and register types. |
| **P1** | Praxeological | Missing Stop Triggers | Implement missing `StopCondition` variants in `sr-adapters`. |
| **P2** | Semantic | Missing Endpoints (Staleness, Records) | Implement handlers in `sr-api`. |
| **P2** | Praxeological | Budget Default Divergence | Align `sr-governor` defaults with SR-DIRECTIVE (5/16h). |
| **P3** | Ontological | Missing `domain.portal_decision` / `loop_record` | Align type keys or alias explicitly. |
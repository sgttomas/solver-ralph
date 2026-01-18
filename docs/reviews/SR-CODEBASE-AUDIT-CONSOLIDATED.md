# SR-CODEBASE-AUDIT-CONSOLIDATED

**Consolidation Date:** 2026-01-17  
**Source Audits:** SR-CODEBASE-AUDIT-CLAUDETERMINAL.md, SR-CODEBASE-AUDIT-CODEX.md, SR-CODEBASE-AUDIT-CLAUDEAPP.md  
**Commit:** `bc1ba8436435a276aee7edeffe8fe18a10cb0404`  
**Branch:** `solver-ralph-consistency`  
**Methodology:** `docs/reviews/SR-CODEBASE-AUDIT-METHODOLOGY.md`

---

## Executive Summary

| Dimension | Document | Coverage | Status |
|-----------|----------|----------|--------|
| Ontological | SR-TYPES | 14/19 runtime types | Partial — missing LoopRecord, evaluation/assessment/intervention notes, procedure_instance, config agent/oracle/portal/semantic_profile |
| Epistemological | SR-CONTRACT | Partial | Trust-boundary and verification enforcement gaps (C-TB-2, C-TB-4, C-VER/S-ship), integrity flake detection not wired |
| Semantic | SR-SPEC | 52/57 endpoints | Missing staleness + human-judgment note endpoints; no evidence-availability status; verification events absent |
| Praxeological | SR-DIRECTIVE | Partial | Governor ignores requested budgets and max_oracle_runs; several stop triggers absent |
| UI Coverage | API→Portal | Complete for implemented APIs | UI gaps match API gaps (staleness + note flows) |

**P0 Findings:** None.  
**Critical Gaps (P1):** budget mis-enforcement, missing portal whitelist, absent verification computation/guarding, unwired ORACLE_FLAKE/stop triggers, missing staleness + note APIs.

---

## Cross-Agent Reconciliation (verified against code)

| Issue | Agent Positions | Verified Outcome | Evidence |
|-------|-----------------|------------------|----------|
| Budget defaults | CLAUDETERMINAL/CODEX: divergent (100/3600), CLAUDEAPP: aligned (5/25/16) | Governor reads `payload["budget"]` (singular) and falls back to `LoopBudget::default()` = 100 iterations / 3600 secs / no max_oracle_runs; ignores API `budgets` payload (5/25/16). | crates/sr-adapters/src/governor.rs:43-50,343-350; crates/sr-api/src/handlers/loops.rs:18-63 |
| C-TB-2 (agent output non-authoritative) | CLAUDETERMINAL/CLAUDEAPP: enforced, CODEX: missing | Evidence upload accepts any actor kind; no guard ties EvidenceBundleRecorded to oracle runner or trusted actor, so agents can submit “evidence” directly. | crates/sr-api/src/handlers/evidence.rs:1-214 (no ActorKind check) |
| C-TB-4 portal validation | CLAUDETERMINAL/CLAUDEAPP: enforced, CODEX: missing | `record_approval` enforces HUMAN actor but accepts arbitrary `portal_id`; no whitelist of seeded portals. | crates/sr-api/src/handlers/approvals.rs:113-180 |
| C-VER-* invariants | CLAUDETERMINAL/CLAUDEAPP: enforced, CODEX: needs review | No producer for `CandidateVerificationComputed`; `VerificationComputer` unused; freeze creation does not require Verified or staleness checks. | repo-wide `rg "CandidateVerificationComputed"` shows no emitter; crates/sr-domain/src/state_machines.rs:176-257 (unused); crates/sr-api/src/handlers/freeze.rs:1-120 (no verification enforcement) |
| Oracle flake detection | CLAUDETERMINAL/CLAUDEAPP: enforced/aligned, CODEX: partial | IntegrityChecker implements flake detection but is never invoked by runner/worker; ORACLE_FLAKE never emitted. | `rg "IntegrityChecker" crates` (only definitions/tests), crates/sr-adapters/src/oracle_runner.rs & oracle_worker.rs (no usage) |

---

## Layer 1: Ontological Coverage (SR-TYPES)

### §4.3 Platform Domain Types

| Type Key | Struct | Location | Status |
|----------|--------|----------|--------|
| `domain.work_unit` | `WorkUnit` | crates/sr-domain/src/entities.rs | Implemented |
| `domain.work_surface` | `ManagedWorkSurface`, `WorkSurfaceInstance` | crates/sr-domain/src/work_surface.rs | Implemented |
| `domain.candidate` | `Candidate` | crates/sr-domain/src/entities.rs | Implemented |
| `domain.evidence_bundle` | `EvidenceBundle` | crates/sr-domain/src/entities.rs | Implemented |
| `domain.portal_decision` | `Approval` | crates/sr-domain/src/entities.rs | Implemented (portal_id not validated downstream) |
| `domain.loop_record` | — | — | Missing |
| `domain.event` | `EventEnvelope` | crates/sr-domain/src/events.rs | Implemented |
| `domain.intake` | `Intake` / `ManagedIntake` | crates/sr-domain/src/work_surface.rs | Implemented |
| `domain.procedure_template` | `ProcedureTemplate` | crates/sr-domain/src/work_surface.rs | Implemented |

### §4.4 Operational Record Types

| Type Key | Status | Notes |
|----------|--------|-------|
| `record.decision` | Implemented | crates/sr-domain/src/entities.rs |
| `record.waiver` | Implemented | ExceptionKind::Waiver |
| `record.deviation` | Implemented | ExceptionKind::Deviation |
| `record.deferral` | Implemented | ExceptionKind::Deferral |
| `record.evaluation_note` | Missing | No struct/endpoints |
| `record.assessment_note` | Missing | No struct/endpoints |
| `record.intervention_note` | Missing | No struct/endpoints |
| `record.intake` | Implemented | Shared with domain.intake |
| `record.procedure_instance` | Missing | StageStatusRecord substitutes but not typed as record |
| `record.attachment` | Implemented | AttachmentManifest |

### §4.5 Configuration Types

| Type Key | Status | Notes |
|----------|--------|-------|
| `config.agent_definition` | Missing | RefKind exists, no type/endpoint |
| `config.oracle_definition` | Missing | Suite registry uses ad-hoc structs |
| `config.portal_definition` | Missing | No schema/registry |
| `config.procedure_template` | Implemented | ProcedureTemplate |
| `config.semantic_set` | Implemented | semantic_oracle.rs |
| `config.semantic_profile` | Missing | No work-kind/stage profile schema |

**Ontological Findings:** Runtime coverage is strong for core loop/candidate/evidence types but operational note records, loop record, procedure_instance, and several config registries are absent, blocking full SR-TYPES §4.4/§4.5 completeness.

---

## Layer 2: Epistemological Compliance (SR-CONTRACT)

### Summary by Category

| Category | Invariants | Status | Notes |
|----------|------------|--------|-------|
| Architecture (C-ARCH-*) | 3 | Enforced | Domain/ports free of infra deps |
| Trust Boundaries (C-TB-*) | 7 | Partial | Portal whitelist absent; evidence upload accepts Agent actor (C-TB-2/4 risk) |
| Verification (C-VER-*) | 4 | Missing | No verification computation or mode declaration; Verified claims not derivable |
| Shippable (C-SHIP-1) | 1 | Partial | Freeze creation lacks Verified/staleness checks |
| Oracle Integrity (C-OR-*) | 7 | Partial | ORACLE_FLAKE detection never invoked; integrity conditions not propagated to stops |
| Evidence (C-EVID-*) | 6 | Partial | Evidence availability/status endpoint missing; refs/meta not validated |
| Events (C-EVT-*) | 7 | Partial | Staleness marking endpoints absent despite graph tables |
| Loop Governance (C-LOOP-*, C-CTX-*) | 6 | Partial | Governor ignores provided budgets; stop triggers incomplete |
| Exceptions (C-EXC-*) | 5 | Enforced | Human-only creation + waiver scope guard |
| Decisions (C-DEC-1) | 1 | Enforced | HUMAN actor check present |
| Metadata (C-META-*) | 3 | Partial | TypedRef meta/content_hash not validated on ingest |

### Key Enforcement / Gap References

| Invariant | Location | Mechanism / Gap |
|-----------|----------|-----------------|
| C-ARCH-2 (Domain purity) | crates/sr-domain/Cargo.toml | No sqlx/reqwest/infra deps |
| C-TB-2 (Agent output non-authoritative) | crates/sr-api/src/handlers/evidence.rs:1-214 | No actor_kind restriction on EvidenceBundleRecorded; agents can upload evidence directly |
| C-TB-4 (Minimum portals) | crates/sr-api/src/handlers/approvals.rs:113-180 | No seeded-portal whitelist |
| C-VER-1..4 | repo-wide | `CandidateVerificationComputed` never emitted; `VerificationComputer` unused |
| C-OR-5 (ORACLE_FLAKE) | crates/sr-adapters/src/integrity.rs (unused) | Detection implemented but not called by runner/worker |
| C-LOOP-1 (Budgets) | crates/sr-adapters/src/governor.rs:43-50,343-350 | Uses default 100/3600; ignores API budgets/max_oracle_runs |

**Epistemological Findings:** Architectural separation holds, but trust boundary and verification invariants are not enforced: agents can submit evidence, portal IDs are unchecked, verification status is never computed or required for freeze, and integrity flake detection is unwired.

---

## Layer 3: Semantic Alignment (SR-SPEC)

### Schema Alignment

| Schema | Location | Field Match | Status |
|--------|----------|-------------|--------|
| EventEnvelope §1.5.2 | crates/sr-domain/src/events.rs | 15/15 | Aligned |
| TypedRef §1.5.3 | crates/sr-domain/src/entities.rs, refs.rs | 9/9 | Aligned (meta not validated on ingest) |
| PostgreSQL Tables §1.6.2 | deploy/migrations/001_event_store.sql | 3/3 | Aligned |
| EvidenceManifest §1.9.1 | crates/sr-adapters/src/evidence.rs | 8/8 | Aligned |
| IterationSummary §3.2.2 | crates/sr-domain/src/commands.rs | 10/10 | Aligned |
| WorkSurface §1.2.4 | crates/sr-domain/src/work_surface.rs | 10/10 | Aligned |
| Intake | crates/sr-domain/src/work_surface.rs | 11/11 | Aligned |

### API Endpoint Alignment (SR-SPEC §2.3)

| Endpoint Group | Implemented | Missing | Status |
|----------------|-------------|---------|--------|
| Loop/Iteration | ✓ | — | Complete |
| Candidates | ✓ | — | Complete |
| Runs/Evidence | ✓ | GET /evidence/{hash}/status | Partial |
| Approvals/Decisions | ✓ | — | Complete (portal whitelist missing) |
| Exceptions/Freeze | ✓ | — | Complete |
| Staleness | — | mark/dependents/resolve | Missing |
| Human Judgment Notes | — | evaluation/assessment note routes | Missing |

**Semantic Findings:** Core schemas and routes align with SR-SPEC, but staleness management and human-judgment note endpoints are absent, and evidence availability status is not exposed.

---

## Layer 4: Praxeological Compliance (SR-DIRECTIVE)

### Summary

| Section | Requirement | Status |
|---------|-------------|--------|
| §2.1 Canonical Loop | Event sequence | Mostly aligned (uses CandidateMaterialized/RunStarted variants) |
| §4.1 Budget Defaults | 5 iterations / 25 runs / 16 hours | Divergent — governor defaults 100 iterations / 3600s, ignores max_oracle_runs and `budgets` payload |
| §4.2 Stop Triggers | 12 triggers | Partial — missing EVIDENCE_MISSING, REPEATED_FAILURE gate emission, NO_ELIGIBLE_WORK, STAGE_UNKNOWN, SEMANTIC_PROFILE_MISSING; ORACLE_FLAKE not surfaced |
| §5.1 Oracle Suites | suite:SR-SUITE-* | Aligned (suite IDs registered) |
| §5.2 Non-waivable | requires_escalation | Aligned in validator, but integrity wiring incomplete |
| §6 Portal IDs | 3 seeded portals | Not validated (free-form portal_id accepted) |
| §7 Gate Registry | gate_id system | Registry exists; enforcement hooks not wired to governor/handlers |

### Budget Defaults (§4.1)

| Parameter | Directive | Implementation | Status |
|-----------|-----------|----------------|--------|
| max_iterations | 5 | 100 (governor default) | Divergent |
| max_duration_secs | 57600 | 3600 (governor default) | Divergent |
| max_oracle_runs | 25 | Not tracked | Missing |

### Stop Triggers (§4.2)

**Present:** BudgetExhausted, IntegrityCondition (tamper/gap/env mismatch only), WorkSurfaceMissing, HumanStop, GoalAchieved, LoopClosed  
**Missing:** EVIDENCE_MISSING, ORACLE_FLAKE propagation, REPEATED_FAILURE (though counter tracked), NO_ELIGIBLE_WORK, STAGE_UNKNOWN, SEMANTIC_PROFILE_MISSING, max_oracle_runs exhaustion.

**Praxeological Findings:** The canonical loop flow exists, but the governor uses permissive defaults, ignores requested budgets, lacks max_oracle_runs tracking, and omits several stop triggers and portal validation, leaving SR-DIRECTIVE gating unenforced.

---

## Layer 5: UI Coverage (API to Portal Alignment)

| API Functionality | UI Coverage | UI Location | Status |
|-------------------|-------------|-------------|--------|
| Loops/Iterations/Candidates/Evidence/Approvals/Decisions/Exceptions/Freeze/Work Surfaces/Intakes/Oracles/Templates/References/Attachments/Prompt Loop | Complete | `ui/src/pages/*` | Aligned |
| Staleness Mgmt | Not implemented | — | API missing |
| Evaluation/Assessment Notes | Not implemented | — | API missing |

**UI Findings:** UI mirrors implemented APIs and omits the same missing surfaces (staleness + human-judgment notes). No UI-specific gaps beyond API deficiencies.

---

## Prioritized Remediation List

| Priority | Dimension | Finding | Evidence/Location | Action |
|----------|-----------|---------|-------------------|--------|
| P1 | Praxeological/Contract | Governor ignores `budgets` and max_oracle_runs; defaults 100/3600 | crates/sr-adapters/src/governor.rs:43-50,343-350; crates/sr-api/src/handlers/loops.rs:18-63 | Parse `budgets` payload, align defaults to 5/25/16, track max_oracle_runs, and add tests. |
| P1 | Trust Boundary | Portal IDs unvalidated; approvals accept arbitrary portal_id | crates/sr-api/src/handlers/approvals.rs:113-180 | Whitelist seeded portals (HumanAuthorityExceptionProcess/GovernanceChangePortal/ReleaseApprovalPortal) and validate schema fields per C-TB-4/6. |
| P1 | Verification | No candidate verification computation; freeze ignores Verified/staleness | `rg "CandidateVerificationComputed"` (no emitter); crates/sr-api/src/handlers/freeze.rs:1-120 | Implement verification status calculation from oracle runs/waivers, emit CandidateVerificationComputed, and require Verified + staleness clear for freeze/release. |
| P1 | Integrity/Stops | ORACLE_FLAKE/EVIDENCE_MISSING not surfaced; stop trigger set incomplete | crates/sr-adapters/src/integrity.rs (unused); crates/sr-adapters/src/governor.rs:54-70 | Invoke IntegrityChecker in runner/worker, emit StopTriggered for integrity conditions (incl. ORACLE_FLAKE/EVIDENCE_MISSING), and add missing stop triggers. |
| P1 | Semantic | Staleness endpoints absent | — (no `/staleness` routes) | Implement mark/dependents/resolve endpoints and graph propagation per SR-SPEC §2.3.11. |
| P1 | Trust Boundary | Agent-submitted evidence accepted as authoritative | crates/sr-api/src/handlers/evidence.rs:1-214 | Restrict evidence ingestion to SYSTEM/oracle actors or bind to RunStarted + suite hash; reject unpinned agent evidence for verification claims (C-TB-2). |
| P2 | Ontological/Semantic | Human-judgment notes + config registries missing | Missing structs/routes for record.evaluation_note/assessment_note/intervention_note, config.agent/oracle/portal/semantic_profile | Add types, events, endpoints, and UI to capture non-binding human judgments and config registries per SR-TYPES §4.4/§4.5 and SR-SPEC §2.3.10. |
| P2 | Metadata | TypedRef meta/content_hash/type_key not validated | ingest paths across handlers | Enforce meta validation on refs (content_hash/type_key) to satisfy C-META-1..3 and C-EVID-4. |

---

## Success Criteria Verification

| Criterion (Methodology §6) | Status | Evidence |
|----------------------------|--------|----------|
| 1. Every SR-TYPES §4.3/§4.4/§4.5 type_key traced | Partial | Missing loop_record, note/procedure_instance types, config registries (see Layer 1) |
| 2. Every C-* invariant has status determination | Partial | Trust-boundary, verification, integrity gaps identified (Layer 2) |
| 3. Every SR-SPEC schema compared to code | Complete | Schema table in Layer 3 |
| 4. Every SR-DIRECTIVE §2-§9 section verified | Partial | Budgets/stop triggers/portal validation gaps (Layer 4) |
| 5. All findings classified by priority | Complete | Remediation list |
| 6. Remediation list is actionable | Complete | Concrete file targets and actions above |

---

## P0 Findings

None identified.

---

## Spec Conflicts

None detected across SR-* documents.

---

## Audit Sources

| Agent | Session | Date | Report |
|-------|---------|------|--------|
| Claude Opus 4.5 | ClaudeTerminal | 2026-01-17 | docs/reviews/SR-CODEBASE-AUDIT-CLAUDETERMINAL.md |
| Codex (GPT-5) | CLI | 2026-01-18 | docs/reviews/SR-CODEBASE-AUDIT-CODEX.md |
| Claude Desktop (Cowork) | ClaudeApp | 2026-01-17 | docs/reviews/SR-CODEBASE-AUDIT-CLAUDEAPP.md |

---

## Conclusion

Core architecture and schemas align with the governed SR-* set, and the UI fully covers implemented APIs. However, enforcement of trust boundaries, verification, and directive-driven budgets/stop triggers is incomplete: the governor ignores requested budgets, portal IDs are unvalidated, verification status is never computed, integrity flake detection is unwired, and staleness + human-judgment note surfaces are missing. Addressing the P1 remediation items will close the remaining compliance gaps and enable reliable Verified/Shippable assertions.

---

*Consolidated audit report per SR-CODEBASE-AUDIT-METHODOLOGY.*

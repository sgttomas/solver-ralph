# SR-CODEBASE-AUDIT-CODEX

- Agent: Codex (GPT-5)  
- Scope: Full SR-CODEBASE audit (ontology, epistemology, semantics, praxeology)  
- Date: 2026-01-18  
- Commit: bc1ba8436435a276aee7edeffe8fe18a10cb0404  
- Canonical docs: SR-TYPES, SR-CONTRACT, SR-SPEC, SR-DIRECTIVE (repo HEAD)  
- Recording: docs/reviews/SR-CODEBASE-AUDIT-CODEX.md  

## Ontological Coverage (SR-TYPES)

| Type Key | Category | Rust Type | Usage Count | Status |
|----------|----------|-----------|-------------|--------|
| domain.work_unit | §4.3 | `WorkUnit` (crates/sr-domain/src/entities.rs) | 204 | Implemented |
| domain.work_surface | §4.3 | `ManagedWorkSurface` / `WorkSurfaceInstance` | 265 | Implemented |
| domain.candidate | §4.3 | `Candidate` | 184 | Implemented |
| domain.evidence_bundle | §4.3 | `EvidenceBundle` | 39 | Implemented |
| domain.portal_decision | §4.3 | — | 0 | Missing (approvals use ad-hoc payloads, no `type_key`) |
| domain.loop_record | §4.3 | — | 0 | Missing (no LoopRecord/iteration aggregate type) |
| domain.event | §4.3 | `EventEnvelope` | 120 | Partial (matches SR-SPEC but `global_seq` optional) |
| domain.intake | §4.3 | `Intake` (work_surface.rs) | 364 | Implemented |
| domain.procedure_template | §4.3 | `ProcedureTemplate` | 112 | Implemented |
| record.decision | §4.4 | `Decision` | 147 | Implemented |
| record.waiver | §4.4 | `ExceptionKind::Waiver` | 52 | Implemented |
| record.deviation | §4.4 | `ExceptionKind::Deviation` | 19 | Implemented |
| record.deferral | §4.4 | `ExceptionKind::Deferral` | 18 | Implemented |
| record.evaluation_note | §4.4 | — | 0 | Missing |
| record.assessment_note | §4.4 | — | 0 | Missing |
| record.intervention_note | §4.4 | — | 0 | Missing |
| record.intake | §4.4 | Intake manifests (work_surface.rs) | 20 | Implemented |
| record.procedure_instance | §4.4 | — | 0 | Missing |
| record.attachment | §4.4 | `AttachmentManifest` (attachments.rs) | 34 | Implemented |
| config.agent_definition | §4.5 | — | 0 | Missing (references endpoint is stubbed) |
| config.oracle_definition | §4.5 | — | 0 | Missing (suite builder uses ad-hoc structs) |
| config.portal_definition | §4.5 | — | 0 | Missing |
| config.procedure_template | §4.5 | Template registry | 31 | Implemented |
| config.semantic_set | §4.5 | Template registry | 15 | Implemented |
| config.semantic_profile | §4.5 | — | 0 | Missing |

## Epistemological Compliance (SR-CONTRACT)

| Invariant | Description | Detection | Enforcement | Test | Status |
|-----------|-------------|-----------|-------------|------|--------|
| C-ARCH-1 | Hexagonal separation | Implicit | Crate deps; no infra in sr-domain/sr-ports | — | Enforced |
| C-ARCH-2 | Domain purity | Implicit | sr-domain has only serde/chrono deps | — | Enforced |
| C-ARCH-3 | Event store source of truth | Explicit | migrations/001_event_store.sql (append-only trigger + unique stream_seq) | — | Enforced |
| C-TB-1 | Human-only binding authority | Explicit | approvals.rs, decisions.rs, freeze.rs enforce HUMAN actor | Integration tests present | Enforced |
| C-TB-2 | Non-authoritative agent output | Manual | No guard preventing agents from emitting binding claims | — | Missing |
| C-TB-3 | Portal crossings produce approvals | Explicit | ApprovalRecorded event exists but gates don’t require portal approval | E2E smoke only | Partial |
| C-TB-4 | Minimum required portals | Explicit | No whitelist on portal_id in approvals.rs | — | Missing |
| C-TB-5 | Stable actor identity | Manual | Actor IDs flow from JWT but not validated for stability | — | Needs Review |
| C-TB-6 | Approval record minimum fields | Explicit | approval payload stores decision/subject_refs but no type_key/meta validation | — | Partial |
| C-TB-7 | Eval/assessment not approval | Explicit | Eval/assessment record types and endpoints absent | — | Missing |
| C-VER-1 | Verification evidence-bound | Explicit | EvidenceBundleRecorded emitted; candidate status derivation unclear | — | Needs Review |
| C-VER-2 | Verified (Strict) all required oracles pass | Explicit | No visible computation tying candidate status to required suite outcomes | — | Needs Review |
| C-VER-3 | Verified-with-Exceptions gated by waivers | Explicit | Waivers exist but not linked in candidate status logic | — | Needs Review |
| C-VER-4 | Verified claims declare mode/basis | Explicit | API responses expose `verification_status` string only | — | Needs Review |
| C-SHIP-1 | Shippable = Freeze + Approval + Verified | Explicit | freeze.rs checks HUMAN + approval existence, but doesn’t verify candidate Verified or staleness | Integration | Partial |
| C-OR-1 | Required oracles deterministic | Manual | No determinism check beyond suite hash; flake detection optional | — | Partial |
| C-OR-2 | Suite pinning/integrity | Explicit | oracle_runner.rs validates suite hash; IntegrityCondition::OracleTamper | Unit | Enforced |
| C-OR-3 | Environment constraints declared/enforced | Explicit | oracle_runner.rs env fingerprint check | Unit | Enforced |
| C-OR-4 | Oracle gaps blocking | Explicit | oracle_runner.rs builds OracleGap integrity condition | Unit | Enforced |
| C-OR-5 | Oracle flake halt | Explicit | IntegrityCondition::OracleFlake exists but no detector wired | Unit stub | Partial |
| C-OR-6 | No silent oracle weakening | Manual | No guard when registering suites/profiles | — | Missing |
| C-OR-7 | Integrity conditions halt/escalate | Explicit | IntegrityCondition requires escalation, but EVIDENCE_MISSING absent | Unit | Partial |
| C-EVID-1 | Evidence manifest minimum | Explicit | evidence.rs validate() | Unit | Enforced |
| C-EVID-2 | Evidence immutability | Explicit | evidence.rs content_hash + sha256 hashing | Unit | Enforced |
| C-EVID-3 | Evidence attribution | Manual | EvidenceBundleRecorded includes actor, but refs/meta not enforced | — | Partial |
| C-EVID-4 | Evidence dependency refs | Manual | TypedRef meta not validated for content_hash/type_key | — | Partial |
| C-EVID-5 | Restricted evidence handling | Explicit | restricted.rs provides redaction but not integrated into upload path | — | Partial |
| C-EVID-6 | Evidence availability | Explicit | No `GET /evidence/{hash}/status`; availability not tracked | — | Missing |
| C-EVT-1 | Event attribution | Explicit | EventEnvelope carries actor fields; set in all handlers | Integration | Enforced |
| C-EVT-2 | Append-only event log | Explicit | migrations/001_event_store.sql trigger prevents update/delete | — | Enforced |
| C-EVT-3 | Explicit supersession | Manual | `supersedes` field present, no enforcement/check on corrections | — | Partial |
| C-EVT-4 | Sequence-first ordering | Explicit | stream_seq unique + global_seq bigserial | — | Enforced |
| C-EVT-5 | Event graph references | Manual | refs stored but meta requirements not validated | — | Partial |
| C-EVT-6 | Staleness marking | Explicit | Staleness endpoints absent; graph has stale_nodes table unused | — | Missing |
| C-EVT-7 | Projections rebuildable | Explicit | sr-adapters/src/replay.rs + replay_determinism_test | Integration | Enforced |
| C-LOOP-1 | Bounded iteration w/ hard stop | Explicit | Governor defaults 100/3600 and reads `budget` not `budgets`, so LoopCreated uses fallback; StopTriggered only covers few triggers | Unit | Partial |
| C-LOOP-2 | Fresh-context iterations | Explicit | start_iteration checks prior iteration completed/failed | Integration | Enforced |
| C-CTX-1 | Iteration context provenance | Explicit | start_iteration auto-injects WorkSurface refs but does not require governed doc hashes | — | Partial |
| C-CTX-2 | No ghost inputs | Manual | Iteration refs meta unchecked; relies on caller-provided refs | — | Partial |
| C-LOOP-3 | Mandatory stop triggers | Explicit | Only BUDGET_EXHAUSTED, REPEATED_FAILURE, WorkSurfaceMissing implemented | — | Missing |
| C-LOOP-4 | Candidate production traceable | Manual | Candidates store produced_by_iteration_id; no guard on completeness | — | Partial |
| C-EXC-1 | Exceptions are records | Explicit | exceptions.rs emits Deviation/Deferral/Waiver events | Integration | Enforced |
| C-EXC-2 | Exceptions visible at baseline | Manual | freeze payload includes active_exceptions but not validated | — | Partial |
| C-EXC-3 | Exceptions don’t rewrite governance | Manual | No guard to prevent policy changes via exceptions | — | Needs Review |
| C-EXC-4 | Waiver required fields | Explicit | Exception creation accepts scope/rationale but lacks integrity-condition block list check in API | Unit | Partial |
| C-EXC-5 | Waiver scope constraints | Manual | Scope structure present, not validated for over-breadth | — | Partial |
| C-DEC-1 | Binding decisions recorded | Explicit | decisions.rs HUMAN-only + DecisionRecorded event | Integration | Enforced |
| C-META-1 | Machine-readable metadata | Manual | Metadata carried as serde_json::Value without validation | — | Partial |
| C-META-2 | Stable identity/lineage | Manual | No content_hash/version enforcement on refs | — | Partial |
| C-META-3 | Binding records distinguishable | Manual | record type_keys not enforced in refs/meta | — | Partial |

## Semantic Alignment (SR-SPEC)

| Schema/Endpoint | Spec Section | Code Location | Match | Issues |
|-----------------|--------------|---------------|-------|--------|
| EventEnvelope | §1.5.2 | crates/sr-domain/src/events.rs | Partial | `global_seq` optional; refs accept arbitrary meta |
| TypedRef | §1.5.3 | crates/sr-domain/src/entities.rs / refs.rs | Partial | No runtime validation of kind/rel/meta.content_hash/type_key |
| PostgreSQL event store | §1.6.2 | migrations/001_event_store.sql | Aligned | Matches schema incl. global_seq, append-only trigger |
| EvidenceManifest | §1.9.1 | crates/sr-adapters/src/evidence.rs | Aligned | Includes stage context fields; validation present |
| IterationSummary | §3.2.2 | crates/sr-domain/src/commands.rs | Aligned | Matches required fields; no `ext` but optional |
| POST /loops | §2.3.1 | crates/sr-api/src/handlers/loops.rs | Aligned | Budget monotonicity enforced on PATCH |
| POST /loops/{id}/iterations | §2.3.1 | crates/sr-api/src/handlers/iterations.rs | Divergent | Implemented as `/api/v1/iterations` body param; path mismatch |
| POST /runs | §2.3.3 | crates/sr-api/src/handlers/runs.rs | Aligned | RunStarted/Completed events emitted |
| GET /evidence/{hash}/status | §2.3.3 | — | Missing | No availability/status route |
| POST /records/evaluation-notes | §2.3.10 | — | Missing | No record type/handler |
| POST /records/assessment-notes | §2.3.10 | — | Missing | No record type/handler |
| POST /staleness/mark | §2.3.9 | — | Missing | No staleness routes |
| GET /staleness/dependents | §2.3.9 | — | Missing | No route |
| POST /staleness/{id}/resolve | §2.3.9 | — | Missing | No route |
| Work Surface endpoints | §2.3.12 | crates/sr-api/src/handlers/work_surfaces.rs | Aligned | CRUD + iteration helpers present |

## Praxeological Compliance (SR-DIRECTIVE)

| Directive Section | Requirement | Code Location | Status | Notes |
|-------------------|-------------|---------------|--------|-------|
| §4.1 Budgets | Defaults max_iterations=5, max_oracle_runs=25, max_wallclock_hours=16 | sr-adapters/src/governor.rs | Divergent | Governor defaults 100/3600 and reads `budget` (missing “s”) from LoopCreated payload, so falls back to divergent defaults |
| §4.2 Stop triggers | 12 triggers incl. EVIDENCE_MISSING, ORACLE_* gaps/flake/tamper, NO_ELIGIBLE_WORK, STAGE_UNKNOWN, SEMANTIC_PROFILE_MISSING | sr-adapters/src/governor.rs; sr-api/src/handlers/work_surfaces.rs | Partial | Only BUDGET_EXHAUSTED, REPEATED_FAILURE, WorkSurfaceMissing represented; no routing to GovernanceChangePortal/HumanAuthority for other triggers |
| §5 Non-waivable integrity | ORACLE_TAMPER/GAP/ENV_MISMATCH/FLAKE/EVIDENCE_MISSING require escalation | sr-domain/src/integrity.rs | Partial | IntegrityCondition omits EVIDENCE_MISSING; requires_escalation always true for other four |
| §6 Portal IDs | Only three seeded portal identities accepted | approvals.rs | Missing | portal_id is free-form; no validation against seeded IDs |
| §2.3 Deterministic eligibility | Eligibility computed by deterministic component, not agent | sr-adapters/src/event_manager.rs | Aligned | Event Manager computes eligible_set deterministically with replay tests |
| §5 Oracle suite/profile IDs | Recognize suite:SR-SUITE-* constants | sr-adapters/src/oracle_suite.rs | Aligned | Suite IDs/constants present |
| §7 Gate registry | Gate evaluation consults registry | — | Needs Review | Gate evaluation logic not surfaced; registry kits referenced but not enforced in code |

## Prioritized Remediation List

| Priority | Dimension | Finding | Source | Effort | Action |
|----------|-----------|---------|--------|--------|--------|
| P1 | Praxeological | Governor ignores SR-DIRECTIVE budgets due to `budget` vs `budgets` payload and divergent defaults (100/3600) | SR-DIRECTIVE §4.1 / sr-adapters/src/governor.rs | 0.5 session | Align field name with LoopCreated payload, set defaults to 5/25/16, add test |
| P1 | Praxeological | Stop trigger registry incomplete (missing EVIDENCE_MISSING, ORACLE_* conditions, NO_ELIGIBLE_WORK, STAGE_UNKNOWN, SEMANTIC_PROFILE_MISSING) | SR-DIRECTIVE §4.2 | 1–2 sessions | Extend StopCondition/StopTriggered emission + projection routing; wire to portal escalation per directive |
| P1 | Semantic | Staleness API endpoints absent (mark/list/resolve) | SR-SPEC §2.3.9 | 1 session | Implement /staleness routes, emit NodeMarkedStale/StalenessResolved, hook graph projection |
| P2 | Ontological | Runtime types missing (`domain.portal_decision`, `domain.loop_record`, record.* note types, `record.procedure_instance`, config.agent/oracle/portal/semantic_profile) | SR-TYPES §4.3–4.5 | 2–3 sessions | Add structs/schemas + events/endpoints for missing types; update TypedRef meta validation |
| P2 | Praxeological | Portal ID not validated against seeded set | SR-DIRECTIVE §6 / SR-CONTRACT C-TB-4 | 0.5 session | Add whitelist for `HumanAuthorityExceptionProcess`, `GovernanceChangePortal`, `ReleaseApprovalPortal` in approvals.rs and related projections/tests |
| P2 | Semantic | Iteration start path diverges from spec | SR-SPEC §2.3.1 | 0.5 session | Add `/loops/{loop_id}/iterations` alias or adjust router; keep SYSTEM-only guard |
| P2 | Epistemological | Evidence availability and evaluation/assessment note flows missing | SR-CONTRACT C-EVID-6 / SR-SPEC §2.3.10 | 1–2 sessions | Add status endpoint for evidence availability; implement record note types + handlers |

### UI Coverage Parity

- The UI (ui/src) currently exposes loops/iterations/work surfaces, candidates/runs/evidence, approvals/exceptions/decisions, templates, references, and prompt-loop flows, but **does not** surface several API areas:
  - Staleness management (`/staleness/*` routes) is absent.
  - Evaluation/assessment/intervention note flows (record.* note types) are absent.
  - Oracle suite/profile registry views are missing beyond template placeholders.
  - Portal-specific routing/whitelisting UX is missing (approvals form is generic).
- Consider adding staleness screens, note record forms, oracle registry views, and seeded-portal-aware approval UX to reach API parity.

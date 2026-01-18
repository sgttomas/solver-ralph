# SR-CODEBASE-AUDIT-PLAN

**Purpose:** Execution plan to remediate consolidated audit findings across trust boundaries, verification, directive alignment, semantic/API gaps, and ontological completeness.

## Phase 1 — Trust Boundary & Verification Enforcement (blocking P1)
- [ ] **P1-TB-PORTALS** — Enforce portal whitelist: restrict `portal_id` to seeded portals (`HumanAuthorityExceptionProcess`, `GovernanceChangePortal`, `ReleaseApprovalPortal`) in approval handlers and projections; add tests that non-whitelisted IDs fail and whitelisted pass. (C-TB-4/6)
- [ ] **P1-TB-EVIDENCE** — Harden evidence ingestion: require SYSTEM/oracle actors or RunStarted lineage for EvidenceBundleRecorded; reject agent-submitted evidence for verification claims; tests cover agent upload rejection and valid RunStarted-linked upload success. (C-TB-2)
- [ ] **P1-VER-COMPUTE** — Implement verification computation: derive required oracle outcomes + waivers → emit `CandidateVerificationComputed`; update projections to store mode/basis; expose verification status with scope (candidate/stage/template); tests assert status transitions for strict pass, waiver-covered fail, uncovered fail, and integrity condition blocking. (C-VER-1..4)
- [ ] **P1-SHIP-GATE** — Gate freeze/release on verification + staleness: freeze creation requires Verified (Strict/With-Exceptions), release approval present, and no unresolved staleness; return explicit failure reasons; tests cover freeze rejection when unverified, when stale, and when approval missing, plus success path. (C-SHIP-1, C-EVT-6)

## Phase 2 — Directive Alignment: Budgets & Stop Triggers (blocking P1)
- [ ] **P1-BUDGET-GOV** — Align governor budget defaults to SR-DIRECTIVE (5 iterations / 25 oracle runs / 16 hours) and parse `budgets` payload (plural); track `max_oracle_runs` exhaustion in stop logic; tests cover: default budgets, honoring requested budgets, max_oracle_runs exhaustion triggering stop, and monotonic budget patching. (C-LOOP-1, SR-DIRECTIVE §4.1)
- [ ] **P1-STOPS-COMPLETE** — Complete stop-trigger set: add EVIDENCE_MISSING, ORACLE_FLAKE propagation, REPEATED_FAILURE activation, NO_ELIGIBLE_WORK, STAGE_UNKNOWN, SEMANTIC_PROFILE_MISSING, and max_oracle_runs exhaustion; emit StopTriggered with recommended portal routing; tests assert trigger emission for each condition and pause state change. (SR-DIRECTIVE §4.2, C-OR-7)
- [ ] **P1-INTEGRITY-WIRE** — Wire integrity checker: invoke IntegrityChecker (incl. flake check) in runner/worker; emit IntegrityViolationDetected/StopTriggered on ORACLE_FLAKE/ENV_MISMATCH/TAMPER/GAP/EVIDENCE_MISSING; tests simulate flake/gap/tamper/env mismatch and verify stop + event emission. (C-OR-5, C-OR-7)

## Phase 3 — Semantic/API Gaps (blocking P1)
- [ ] **P1-STALENESS-API** — Implement staleness APIs: `POST /staleness/mark`, `GET /staleness/dependents`, `POST /staleness/{id}/resolve`; hook graph/projections to set/clear staleness flags that block shippable; tests cover marking, dependent listing, resolving, and shippable recomputation. (SR-SPEC §2.3.11)
- [ ] **P1-EVID-STATUS** — Add evidence availability/status endpoint (`GET /evidence/{hash}/status`) returning existence and integrity flags; tests for existing evidence, missing evidence, and integrity violation scenario. (C-EVID-6)
- [ ] **P1-NOTES-API** — Add human-judgment note APIs: `POST /records/evaluation-notes`, `POST /records/assessment-notes`, retrieval; ensure notes are non-binding and distinguished from approvals; tests for creation, retrieval, and rejection of approval-like decisions. (C-TB-7, SR-SPEC §2.3.10)

## Phase 4 — Ontological Completeness (P2)
- [ ] **P2-TYPES-NOTES** — Add missing record types: EvaluationNote, AssessmentNote, InterventionNote; register type keys, schemas, and serialization. (SR-TYPES §4.4)
- [ ] **P2-TYPES-CONFIG** — Add missing config types: AgentDefinition, OracleDefinition, PortalDefinition, SemanticProfile; register type keys and minimal CRUD/registry backing. (SR-TYPES §4.5)
- [ ] **P2-TYPES-PROC/LOOPREC** — Add ProcedureInstance representation or explicit mapping to StageStatusRecord; add LoopRecord representation or document Iteration equivalence with projection exposure. (SR-TYPES §4.3/§4.4)
- [ ] **P2-REFS-VALIDATION** — Validate TypedRef meta: enforce `meta.content_hash` and `meta.type_key` where applicable on ingest; reject incomplete refs; tests ensure invalid refs fail and valid refs pass. (C-META-1..3, C-EVID-4)

## Phase 5 — UI Parity & Tests (P2/P3)
- [ ] **P2-UI-PARITY** — Extend UI for new endpoints: staleness management screens, evaluation/assessment note flows, portal dropdown with seeded whitelist; include client-side validation and error surfacing.
- [ ] **P2-TEST-SUITE** — Add automated tests: governor budget/stop-trigger coverage, verification computation + freeze gating, portal whitelist enforcement, integrity flake/evidence-missing triggers, staleness API behavior, note API behavior, evidence status endpoint; prefer integration/E2E where applicable.

## Phase 6 — Governance & Migration (P3)
- [ ] **P3-MIGRATIONS** — Adjust projections/tables for new stop triggers, verification status fields, staleness flags, and new record/config types; include data backfill scripts if needed.
- [ ] **P3-GOV-DOCS** — Update documentation kits (Gate Registry, portal playbooks) to reflect new stop triggers and portal enforcement hooks; record exceptions in SR-EXCEPTIONS if temporary waivers are needed during rollout.

## Risk & Rollback
- **Rollback plan:** For each phase, maintain schema and projection migrations as reversible (down migrations). Keep feature flags/toggles for new enforcement (portal whitelist, evidence actor checks, stop triggers) to allow staged rollout and quick disable if regressions occur.
- **Risk hotspots:** Budget enforcement changes can pause/stop loops unexpectedly; portal whitelisting may block existing approvals with non-seeded IDs; verification gating can block releases if evidence lineage is incomplete. Mitigate with shadow-mode logging before hard enforcement where feasible and add migration scripts to normalize existing data (portal IDs, refs).

## Milestones
- **M1:** Phases 1–3 completed → Trust boundary/verification/directive/staleness gaps closed (blocks P1s).
- **M2:** Phase 4 completed → Ontology/type registry coverage complete.
- **M3:** Phase 5–6 completed → UI parity, tests, migrations, and governed docs updated.

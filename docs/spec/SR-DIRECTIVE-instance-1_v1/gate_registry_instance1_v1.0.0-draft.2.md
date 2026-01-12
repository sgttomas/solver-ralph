# SR-DIRECTIVE Gate Registry — Instance-1 (v1.0.0-draft.2)

*Generated:* 2026-01-11

This registry defines the binding gate control surface for executing SR-PLAN instance-1.
Each gate specifies membranes (SR-ETT), enforcement surfaces, required refs/evidence, and routing/relief valves.

## G-00: Context admissible (IterationStarted ref-set; no ghost inputs)

- **Gate kind:** work_start

- **Applies to:** D-01..D-36

- **Purpose/decision:** Establish an admissible, deterministic Iteration context by enforcing the normative IterationStarted.refs[] checklist and prohibiting any unreferenced inputs.

- **Membranes enforced (SR-ETT):** Intent & Objective; Ontological; Accountability; Authority & Integrity; Change

- **Enforcement mechanism:** IterationStarted schema validation; ref dereference + content_hash validation; ContextCompiler deterministic compilation; staleness preflight on depends_on refs

- **Allowed actor kinds:** SYSTEM

- **Required refs (depends_on):** Loop (rel=in_scope_of); GovernedArtifact SR-TYPES/SR-CONTRACT/SR-SPEC/SR-DIRECTIVE (rel=depends_on, version+content_hash); SR-PLAN instance (rel=depends_on); SAPS / problem_statement record (rel=depends_on); OracleSuite intended for verification (rel=depends_on); Prior Iteration summaries carried forward (rel=depends_on, optional); Base Candidate (rel=depends_on, optional but required for incremental work); Active exceptions (Deviation/Deferral/Waiver, rel=depends_on, optional/empty allowed); Human notes (intervention/evaluation/assessment, rel=depends_on when present/required by gating policy)

- **Required commitment objects:** LoopCreated; IterationStarted (actor_kind=SYSTEM); referenced governed artifacts are registered/available with matching content_hash

- **Required evidence:** N/A (capture preflight logs as evidence.gate_packet when blocked; optional for audit)

- **Verification profile / suite:** suite:CTX-REFSET-VALIDATION@v1

- **Success outputs:** IterationStarted accepted; ContextBundle compiled deterministically; iteration may proceed

- **Success state effect:** Iteration may start; agent work is permitted only within compiled ContextBundle

- **Failure conditions:** Missing required refs or missing meta.content_hash; refs not dereferenceable; governed artifact versions/hashes unknown; oracle suite ref missing; active exceptions referenced but unavailable; required human note missing per gating policy; depends_on refs stale with unresolved staleness markers

- **Stop triggers on failure:** EVIDENCE_MISSING (if any required ref cannot be fetched); ORACLE_GAP (if required OracleSuite ref missing); REPEATED_FAILURE (if repeated inability to compile context)

- **Relief valve:** DecisionRecorded (human) to amend ref set / resolve staleness / correct scope; GovernanceChangePortal for systemic ref-set/schema issues

- **Routing authority boundary on block:** GovernanceChangePortal (systemic); otherwise DecisionRecorded (human) for this loop/iteration

- **Contract refs:** C-CTX-1; C-CTX-2; C-EVID-6; C-EVT-1

- **Spec refs:** SR-SPEC §3.2.1.1; SR-SPEC ContextCompiler semantics; SR-SPEC §1.13 (staleness)

- **Plan refs:** D-01..D-36

- **Notes:** Default conservative: if context cannot be proven admissible, block iteration start rather than proceed on implicit/ambient inputs.


## G-10: Runtime substrate verified (event log + projections + outbox invariants)

- **Gate kind:** foundation_accept

- **Applies to:** D-09..D-13; D-18; D-21..D-23; D-32; D-34; D-36

- **Purpose/decision:** Accept foundational runtime/persistence components only when the event log is append-only, projection rebuild is deterministic, and outbox publication invariants hold under the declared environment constraints.

- **Membranes enforced (SR-ETT):** Architectural; Operational; Accountability; Authority & Integrity; Isomorphic

- **Enforcement mechanism:** oracle_run (integration suite); projection rebuild checks; event-envelope conformance tests; outbox/at-least-once tests

- **Allowed actor kinds:** SYSTEM

- **Required refs (depends_on):** Candidate (rel=depends_on); OracleSuite suite:RUNTIME-SUBSTRATE (rel=depends_on); governing artifacts in force (rel=depends_on)

- **Required commitment objects:** RunStarted + RunCompleted; EvidenceBundleRecorded; (if applicable) projection tables created/migrated

- **Required evidence:** Evidence bundle containing: event-store invariant tests; projection rebuild determinism logs; outbox publish simulation logs

- **Verification profile / suite:** suite:RUNTIME-SUBSTRATE@v1

- **Success outputs:** EvidenceBundleRecorded with PASS for required runtime invariants

- **Success state effect:** Candidate verification status may be computed as Verified (Strict) for the runtime substrate scope; downstream deliverables may depend_on this substrate

- **Failure conditions:** Any required runtime invariant oracle FAIL; inability to rebuild projections deterministically; non-monotonic global_seq; event-envelope schema violations; outbox duplication/ordering violations outside declared guarantees; oracle integrity faults (tamper/gap/flake/env mismatch); evidence not retrievable

- **Stop triggers on failure:** ORACLE_TAMPER; ORACLE_GAP; ORACLE_ENV_MISMATCH; ORACLE_FLAKE; EVIDENCE_MISSING

- **Relief valve:** Fix implementation; if a requirement is infeasible, route Deviation/Deferral via GovernanceChangePortal (do not silently weaken invariants).

- **Routing authority boundary on block:** GovernanceChangePortal (for requirement conflicts) / DecisionRecorded (human) for stop-trigger arbitration

- **Contract refs:** C-ARCH-1; C-ARCH-3; C-EVT-3; C-OR-1; C-OR-2; C-EVID-6

- **Spec refs:** SR-SPEC §1.2 (event envelope); SR-SPEC §1.5 (event log); SR-SPEC §1.6–§1.8 (projections/outbox); SR-SPEC §3.3 (Verified rules)

- **Plan refs:** D-09..D-13; D-18; D-21..D-23; D-32; D-34; D-36

- **Notes:** This gate is meant for *substrate* components whose failures poison downstream work; treat failures as stop-the-line until resolved. | [vpos-note] includes STRICT-CORE baseline + runtime invariants


## G-15: Dependency graph + staleness routing verified

- **Gate kind:** invariant_accept

- **Applies to:** D-12; D-19

- **Purpose/decision:** Accept dependency-graph and staleness traversal behavior only when staleness propagation, re-evaluation triggering, and unresolved-staleness blocking semantics match SR-SPEC.

- **Membranes enforced (SR-ETT):** Isomorphic; Ontological; Operational; Accountability; Change

- **Enforcement mechanism:** oracle_run (staleness traversal suite); projection conformance checks; reference-relationship validation (depends_on vs supported_by)

- **Allowed actor kinds:** SYSTEM

- **Required refs (depends_on):** Candidate (rel=depends_on); OracleSuite suite:STALENESS-GRAPH (rel=depends_on); governing artifacts in force (rel=depends_on)

- **Required commitment objects:** RunStarted + RunCompleted; EvidenceBundleRecorded; dependency graph projection populated

- **Required evidence:** Evidence bundle containing: dependency graph build tests; staleness traversal tests; re-evaluation routing tests

- **Verification profile / suite:** suite:STALENESS-GRAPH@v1

- **Success outputs:** EvidenceBundleRecorded with PASS for staleness/graph semantics

- **Success state effect:** Staleness markers can be relied upon to block Shippable and trigger re-evaluation deterministically

- **Failure conditions:** Incorrect traversal/propagation; failure to block when upstream dependency changes; incorrect handling of rel=depends_on vs rel=supported_by; non-deterministic projection rebuild; evidence not retrievable

- **Stop triggers on failure:** EVIDENCE_MISSING; ORACLE_GAP; ORACLE_TAMPER; ORACLE_FLAKE

- **Relief valve:** Fix implementation; if semantics need change, route governance change (SR-SPEC/SR-CONTRACT) via GovernanceChangePortal.

- **Routing authority boundary on block:** GovernanceChangePortal (semantic conflicts) / DecisionRecorded (human) for arbitration

- **Contract refs:** C-EVT-6; C-SHIP-1; C-CTX-1

- **Spec refs:** SR-SPEC §1.13 (staleness); SR-SPEC §1.12.4 (Shippable requires no unresolved staleness); SR-SPEC §3.2.1.1 (rel semantics)

- **Plan refs:** D-12; D-19

- **Notes:** This gate is load-bearing for staleness-aware Shippable gating; keep it strict and avoid waiver by default.


## G-20: Evidence store integrity + retrievability verified

- **Gate kind:** foundation_accept

- **Applies to:** D-14..D-16; D-19..D-20; D-23..D-27; D-29; D-32..D-34

- **Purpose/decision:** Accept evidence subsystem components only when evidence bundles are content-addressed, validated on ingest, remain retrievable, and missing evidence is detected and blocks progression.

- **Membranes enforced (SR-ETT):** Accountability; Authority & Integrity; Operational

- **Enforcement mechanism:** oracle_run (evidence integrity suite); evidence manifest schema validation; retrievability reference-walk tests; negative tests for missing evidence

- **Allowed actor kinds:** SYSTEM

- **Required refs (depends_on):** Candidate (rel=depends_on); OracleSuite suite:EVIDENCE-INTEGRITY (rel=depends_on); governing artifacts in force (rel=depends_on)

- **Required commitment objects:** RunStarted + RunCompleted; EvidenceBundleRecorded; evidence objects stored content-addressed

- **Required evidence:** Evidence bundle containing: ingest validation tests; content-hash verification logs; retrieval checks; EVIDENCE_MISSING negative-path proof

- **Verification profile / suite:** suite:EVIDENCE-INTEGRITY@v1

- **Success outputs:** EvidenceBundleRecorded with PASS for evidence ingest/validation/retrievability

- **Success state effect:** Evidence may be used as basis for Verified claims; missing evidence will be caught and block binding claims

- **Failure conditions:** Evidence manifest schema invalid; content hash mismatch; evidence objects not retrievable; presigned-url or storage layer breaks auditability; missing evidence not detected or does not block; oracle integrity faults

- **Stop triggers on failure:** EVIDENCE_MISSING; ORACLE_GAP; ORACLE_TAMPER; ORACLE_ENV_MISMATCH; ORACLE_FLAKE

- **Relief valve:** Fix storage/ingest; if retention constraints conflict, route Deviation/Deferral via GovernanceChangePortal (do not allow silent loss of evidence).

- **Routing authority boundary on block:** GovernanceChangePortal (retention/policy conflicts) / DecisionRecorded (human) for incident handling

- **Contract refs:** C-EVID-1; C-EVID-2; C-EVID-6; C-OR-2; C-EVT-1

- **Spec refs:** SR-SPEC §1.9 (EvidenceBundle manifest); SR-SPEC §2.3.3 (Runs and EvidenceBundleRecorded); SR-SPEC §3.4 (EVIDENCE_MISSING handling)

- **Plan refs:** D-14..D-16; D-19..D-20; D-23..D-27; D-29; D-32..D-34

- **Notes:** Evidence availability is non-negotiable for binding claims; treat storage incidents as stop-the-line for affected claims.


## G-30: Core verification PASS (STRICT-CORE) for deliverable acceptance

- **Gate kind:** verification_accept

- **Applies to:** D-02..D-12; D-14..D-15; D-17..D-20; D-22..D-31; D-33..D-34

- **Purpose/decision:** Accept a candidate deliverable only when the assigned core verification profile passes with deterministic required oracles, suite pinning is intact, and evidence is recorded and retrievable.

- **Membranes enforced (SR-ETT):** Architectural; Ontological; Accountability; Authority & Integrity; Operational

- **Enforcement mechanism:** RunStarted/RunCompleted; required oracle suite execution; EvidenceBundleRecorded; candidate status computation

- **Allowed actor kinds:** SYSTEM

- **Required refs (depends_on):** Candidate (rel=depends_on); OracleSuite profile/suite for STRICT-CORE (rel=depends_on); governing artifacts in force (rel=depends_on); active exceptions in scope (rel=depends_on, optional)

- **Required commitment objects:** RunStarted + RunCompleted; EvidenceBundleRecorded; (if WITH_EXCEPTIONS) WaiverCreated + WaiverActivated

- **Required evidence:** Evidence bundle manifest + logs for all required oracles in STRICT-CORE (build/test/lint/schema etc, as defined in the profile)

- **Verification profile / suite:** profile:STRICT-CORE@v1

- **Success outputs:** EvidenceBundleRecorded; Candidate computed as Verified (Strict) OR Verified-with-Exceptions (only if waiver(s) exist and are in-scope)

- **Success state effect:** Deliverable may be marked Accepted for its workflow phase; downstream depends_on edges may proceed

- **Failure conditions:** Any required oracle FAIL without an in-scope active waiver; any oracle integrity fault (tamper/gap/flake/env mismatch); evidence missing/unretrievable; suite hash mismatch; attempt to treat advisory or human notes as replacing evidence

- **Stop triggers on failure:** ORACLE_TAMPER; ORACLE_GAP; ORACLE_ENV_MISMATCH; ORACLE_FLAKE; EVIDENCE_MISSING; REPEATED_FAILURE (if N consecutive no-progress iterations)

- **Relief valve:** Fix failing checks; if acceptability requires exceptions, route WaiverCreated (human) for specific FAIL outcomes only; for systemic changes, route GovernanceChangePortal.

- **Routing authority boundary on block:** DecisionRecorded (human) for stop-trigger arbitration; GovernanceChangePortal for systemic requirement changes; /exceptions/waivers (human) for waiver requests

- **Contract refs:** C-VER-1; C-VER-2; C-OR-1; C-OR-2; C-EVID-6; C-TB-7

- **Spec refs:** SR-SPEC §3.3 (Candidate verification status); SR-SPEC §3.5 (suite pinning); SR-SPEC §2.3.3 (EvidenceBundleRecorded)

- **Plan refs:** D-02..D-12; D-14..D-15; D-17..D-20; D-22..D-31; D-33..D-34

- **Notes:** Do not downgrade required oracles to advisory without governance change routing; keep STRICT as default unless explicitly allowed for a work-unit. | [vpos-note] required oracles deterministic; advisory allowed


## G-31: Full verification PASS (STRICT-FULL) for high-stakes acceptance

- **Gate kind:** verification_accept

- **Applies to:** D-26

- **Purpose/decision:** Accept a candidate only when extended integration/e2e verification passes under deterministic required oracles, with evidence recorded and suite pinned.

- **Membranes enforced (SR-ETT):** Architectural; Isomorphic; Accountability; Authority & Integrity; Operational

- **Enforcement mechanism:** RunStarted/RunCompleted; extended oracle suite execution; EvidenceBundleRecorded; candidate status computation

- **Allowed actor kinds:** SYSTEM

- **Required refs (depends_on):** Candidate (rel=depends_on); OracleSuite profile/suite for STRICT-FULL (rel=depends_on); governing artifacts in force (rel=depends_on); active exceptions (rel=depends_on, optional)

- **Required commitment objects:** RunStarted + RunCompleted; EvidenceBundleRecorded; (if WITH_EXCEPTIONS) WaiverCreated + WaiverActivated

- **Required evidence:** Evidence bundle manifest + logs for all required oracles in STRICT-FULL (integration/e2e/security etc as defined in the profile)

- **Verification profile / suite:** profile:STRICT-FULL@v1

- **Success outputs:** EvidenceBundleRecorded; Candidate computed as Verified (Strict) OR Verified-with-Exceptions (waiver-scoped)

- **Success state effect:** High-stakes deliverable may be accepted and used as basis for end-to-end gates

- **Failure conditions:** Required oracle FAIL without in-scope waiver; integrity faults; evidence missing; suite hash mismatch; non-deterministic required oracle

- **Stop triggers on failure:** ORACLE_TAMPER; ORACLE_GAP; ORACLE_ENV_MISMATCH; ORACLE_FLAKE; EVIDENCE_MISSING

- **Relief valve:** Fix; or request tightly-scoped waiver (human) for specific FAIL results; otherwise route governance change for profile/suite modification.

- **Routing authority boundary on block:** DecisionRecorded (human) / GovernanceChangePortal / /exceptions/waivers (human)

- **Contract refs:** C-VER-1; C-OR-1; C-OR-2; C-EVID-6

- **Spec refs:** SR-SPEC §3.3 (Verified rules); SR-SPEC §3.5 (suite pinning); SR-SPEC §2.3.3

- **Plan refs:** D-26

- **Notes:** Use sparingly for gates/deliverables that would otherwise create high-cost false positives (e.g., integration suite itself, release candidates).


## G-40: Stop-the-line triggers + integrity conditions enforced

- **Gate kind:** integrity_enforcement

- **Applies to:** D-22..D-23; D-27; D-33..D-35

- **Purpose/decision:** Confirm that mandatory stop triggers and integrity conditions are implemented end-to-end: integrity faults halt progression, StopTriggered is emitted, loops pause, and human DecisionRecorded is required to proceed.

- **Membranes enforced (SR-ETT):** Operational; Authority & Integrity; Accountability; Resource; Change

- **Enforcement mechanism:** oracle_run (stop-trigger suite); simulated fault injection; loop lifecycle assertions (PAUSED gating); decision/arbitration workflow checks

- **Allowed actor kinds:** SYSTEM (trigger detection) + HUMAN (DecisionRecorded to resume)

- **Required refs (depends_on):** Candidate (rel=depends_on); OracleSuite suite:STOP-TRIGGERS (rel=depends_on); governing artifacts in force (rel=depends_on)

- **Required commitment objects:** StopTriggered; Loop state transitions to PAUSED; DecisionRecorded (human) required for resume; EvidenceMissingDetected (when applicable)

- **Required evidence:** Evidence bundle containing: fault-injection transcripts for each mandatory trigger; lifecycle projection assertions; decision workflow proofs

- **Verification profile / suite:** suite:STOP-TRIGGERS@v1

- **Success outputs:** EvidenceBundleRecorded (from stop-trigger suite); demonstrated StopTriggered→PAUSED→DecisionRecorded path

- **Success state effect:** Stop-the-line discipline may be relied upon during plan execution; integrity faults are non-waivable

- **Failure conditions:** Any mandatory trigger not emitted when condition occurs; loop does not pause; system proceeds without DecisionRecorded; attempted waiver of non-waivable integrity; missing evidence does not block; REPEATED_FAILURE threshold undefined in directive

- **Stop triggers on failure:** ORACLE_TAMPER; ORACLE_GAP; ORACLE_ENV_MISMATCH; ORACLE_FLAKE; EVIDENCE_MISSING; REPEATED_FAILURE; BUDGET_EXHAUSTED

- **Relief valve:** Fix; if arbitration semantics need change, route governance change via GovernanceChangePortal (do not bypass stop-the-line).

- **Routing authority boundary on block:** DecisionRecorded (human) for arbitration; GovernanceChangePortal for systemic trigger/semantics changes

- **Contract refs:** C-LOOP-3; C-DEC-1; C-OR-2; C-EVID-6

- **Spec refs:** SR-SPEC §3.4 (mandatory stop triggers; PAUSED semantics); SR-SPEC Loop lifecycle (§3.1); SR-SPEC DecisionRecorded model (§1.11)

- **Plan refs:** D-22..D-23; D-27; D-33..D-35

- **Notes:** This gate is itself integrity-critical; treat failures as blockers for any downstream work that depends on budgets/stop triggers.


## G-50: Portal workflows functional (approvals/exceptions/freeze submission paths)

- **Gate kind:** integration_accept

- **Applies to:** D-19; D-30; D-34

- **Purpose/decision:** Accept portal-facing surfaces only when portal workflows can create the required binding records (ApprovalRecorded, WaiverCreated, FreezeRecordCreated) with correct field constraints and actor-kind enforcement.

- **Membranes enforced (SR-ETT):** Authority & Integrity; Accountability; Ontological; Change

- **Enforcement mechanism:** oracle_run (portal workflow integration suite); API contract tests; UI-to-API trace tests; actor-kind enforcement checks

- **Allowed actor kinds:** SYSTEM (tests) + HUMAN (binding actions at runtime)

- **Required refs (depends_on):** Candidate (rel=depends_on); governing artifacts in force (rel=depends_on); OracleSuite suite:PORTAL-WORKFLOWS (rel=depends_on)

- **Required commitment objects:** ApprovalRecorded (human) created via portal API; WaiverCreated (human) where applicable; FreezeRecordCreated (human) where applicable

- **Required evidence:** Evidence bundle containing: API integration traces for approvals/waivers/freeze; schema validation logs; negative tests for actor_kind!=HUMAN

- **Verification profile / suite:** suite:PORTAL-WORKFLOWS@v1

- **Success outputs:** EvidenceBundleRecorded; verified ability to produce required binding records through portal workflows

- **Success state effect:** Portal boundary crossings are operationally usable for later gates (release/freeze/exception workflows)

- **Failure conditions:** Cannot create ApprovalRecorded; approval missing required fields; portal allows agent/system to spoof human actor_kind; FreezeRecordCreated rejected due to missing exception acknowledgement but UI/API fails to capture; waiver scope constraints not enforced

- **Stop triggers on failure:** ORACLE_GAP; ORACLE_TAMPER; ORACLE_FLAKE; EVIDENCE_MISSING

- **Relief valve:** Fix implementation; if portal field requirements change, route SR-SPEC/SR-CONTRACT update via GovernanceChangePortal.

- **Routing authority boundary on block:** GovernanceChangePortal (schema/semantics conflicts) / DecisionRecorded (human) for operational arbitration

- **Contract refs:** C-TB-1; C-TB-4; C-TB-6; C-TB-7; C-SHIP-1

- **Spec refs:** SR-SPEC §2.3.4 (Approvals); SR-SPEC §2.3.5 (Exceptions); SR-SPEC §1.12 (Freeze record fields and exception acknowledgement)

- **Plan refs:** D-19; D-30; D-34

- **Notes:** This gate is about *functional correctness* of portal workflows, not about granting approvals for a real release.


## G-60: Self-host deploy smoke verified (single-command up + health checks)

- **Gate kind:** ops_accept

- **Applies to:** D-26; D-31..D-34

- **Purpose/decision:** Accept self-host/ops deliverables only when the full stack boots deterministically with pinned versions and passes health checks in the declared environment.

- **Membranes enforced (SR-ETT):** Resource; Operational; Accountability; Authority & Integrity

- **Enforcement mechanism:** oracle_run (selfhost smoke suite); deployment harness; health-check probes; version pinning validation

- **Allowed actor kinds:** SYSTEM

- **Required refs (depends_on):** Candidate (rel=depends_on); OracleSuite suite:SELFHOST-SMOKE (rel=depends_on); governing artifacts in force (rel=depends_on)

- **Required commitment objects:** RunStarted + RunCompleted; EvidenceBundleRecorded; deployment logs captured

- **Required evidence:** Evidence bundle containing: bootstrap logs; service health checks; API ping tests; version/digest pinning report

- **Verification profile / suite:** suite:SELFHOST-SMOKE@v1

- **Success outputs:** EvidenceBundleRecorded with PASS for self-host boot + health checks

- **Success state effect:** Self-host environment may be used as a substrate for e2e and replayability harnesses

- **Failure conditions:** Stack does not boot; health checks fail; non-deterministic boot; required secrets not handled per policy; evidence missing

- **Stop triggers on failure:** BUDGET_EXHAUSTED; REPEATED_FAILURE; EVIDENCE_MISSING; ORACLE_ENV_MISMATCH

- **Relief valve:** Fix; if environment constraints need adjustment, route suite/environment changes via GovernanceChangePortal.

- **Routing authority boundary on block:** DecisionRecorded (human) for budget/stop arbitration; GovernanceChangePortal for environment constraint changes

- **Contract refs:** C-LOOP-1; C-OR-1; C-EVID-6

- **Spec refs:** SR-SPEC §3.5 (environment constraints); SR-SPEC §2.3 (API surface)

- **Plan refs:** D-26; D-31..D-34

- **Notes:** Keep this gate fast (smoke) but strict on determinism and pinning to prevent 'works on my machine' drift.


## G-70: End-to-end happy path transcript verified

- **Gate kind:** e2e_accept

- **Applies to:** D-34..D-36

- **Purpose/decision:** Accept end-to-end harness only when the canonical happy path executes: Loop→IterationStarted→Candidate submission→Run(oracles)→EvidenceBundleRecorded→ApprovalRecorded→FreezeRecordCreated, producing a replayable transcript and IDs.

- **Membranes enforced (SR-ETT):** Intent & Objective; Operational; Architectural; Ontological; Isomorphic; Change; Authority & Integrity; Resource; Accountability

- **Enforcement mechanism:** oracle_run (e2e harness); transcript capture; invariant assertions across projections; simulated human actor for portal steps in test env

- **Allowed actor kinds:** SYSTEM (harness) + HUMAN (simulated test identity for portal steps) + ORACLE (run execution)

- **Required refs (depends_on):** Candidate (rel=depends_on); OracleSuite suite:E2E-HAPPY (rel=depends_on); governing artifacts in force (rel=depends_on); self-host environment ref (rel=depends_on)

- **Required commitment objects:** IterationStarted; RunStarted/RunCompleted; EvidenceBundleRecorded; ApprovalRecorded; FreezeRecordCreated

- **Required evidence:** Evidence bundle containing: full harness transcript/logs; produced object IDs; assertions for each membrane boundary; retrievability check results

- **Verification profile / suite:** suite:E2E-HAPPY@v1

- **Success outputs:** EvidenceBundleRecorded + harness transcript; produced ApprovalRecorded and FreezeRecordCreated in test

- **Success state effect:** Demonstrated canonical workflow is executable; unlocks failure-path and replayability proofs

- **Failure conditions:** Any step in canonical workflow fails; approvals/freeze cannot be recorded; harness cannot demonstrate replayable transcript; integrity faults unhandled; staleness incorrectly ignored

- **Stop triggers on failure:** ORACLE_GAP; ORACLE_TAMPER; ORACLE_ENV_MISMATCH; ORACLE_FLAKE; EVIDENCE_MISSING; REPEATED_FAILURE

- **Relief valve:** Fix; if workflow requires governance semantic changes, route via GovernanceChangePortal (do not weaken harness expectations silently).

- **Routing authority boundary on block:** GovernanceChangePortal (semantic conflicts) / DecisionRecorded (human) for stop-trigger arbitration

- **Contract refs:** C-ARCH-1; C-CTX-1; C-VER-1; C-TB-4; C-SHIP-1; C-EVID-6

- **Spec refs:** SR-SPEC §3.1–§3.3 (loop/iteration/candidate lifecycle); SR-SPEC §2.3.3–§2.3.5 (runs/evidence/approvals/exceptions); SR-SPEC §1.12 (freeze/shippable)

- **Plan refs:** D-34..D-36

- **Notes:** Test harness may use a fixed test human identity; production releases still require real human actors with stable identities. | [vpos-note] uses STRICT-CORE + portal/freeze validation


## G-71: End-to-end failure/exception path transcript verified

- **Gate kind:** e2e_accept

- **Applies to:** D-35

- **Purpose/decision:** Accept failure-path harness only when integrity faults and waiver-eligible FAIL outcomes follow the required routes: StopTriggered→PAUSED, EvidenceMissingDetected blocks claims, WaiverCreated enables Verified-with-Exceptions only when allowed, and human DecisionRecorded/exception resolution is required to proceed.

- **Membranes enforced (SR-ETT):** Operational; Authority & Integrity; Accountability; Resource; Change; Isomorphic

- **Enforcement mechanism:** oracle_run (e2e failure harness); fault injection; transcript capture; projection assertions

- **Allowed actor kinds:** SYSTEM (harness) + HUMAN (simulated test identity) + ORACLE

- **Required refs (depends_on):** Candidate (rel=depends_on); OracleSuite suite:E2E-FAILURE (rel=depends_on); governing artifacts in force (rel=depends_on)

- **Required commitment objects:** StopTriggered; DecisionRecorded; (when applicable) WaiverCreated/WaiverActivated; EvidenceMissingDetected

- **Required evidence:** Evidence bundle containing: failure-case transcripts; stop-trigger assertions; waiver scope assertions; portal/decision traces

- **Verification profile / suite:** suite:E2E-FAILURE@v1

- **Success outputs:** EvidenceBundleRecorded + transcripts demonstrating correct failure routing and relief valves

- **Success state effect:** Confirms system cannot silently progress under integrity faults; validates exception/decision mechanisms

- **Failure conditions:** Integrity faults fail to halt; system progresses without decision; waiver used to bypass non-waivable integrity; missing evidence does not block

- **Stop triggers on failure:** ORACLE_GAP; ORACLE_TAMPER; ORACLE_ENV_MISMATCH; ORACLE_FLAKE; EVIDENCE_MISSING

- **Relief valve:** Fix; if exception semantics change is required, route via GovernanceChangePortal.

- **Routing authority boundary on block:** GovernanceChangePortal / DecisionRecorded (human)

- **Contract refs:** C-LOOP-3; C-DEC-1; C-EXC-1..C-EXC-5; C-EVID-6

- **Spec refs:** SR-SPEC §3.4 (stop triggers); SR-SPEC §2.3.5 (waivers); SR-SPEC §1.9 (evidence); SR-SPEC §1.11 (decisions)

- **Plan refs:** D-35

- **Notes:** This gate should include explicit negative tests proving that non-waivable conditions cannot be waived.


## G-80: Replayability proof verified (event stream → identical reconstruction)

- **Gate kind:** proof_accept

- **Applies to:** D-36

- **Purpose/decision:** Accept replayability deliverable only when replaying the same event stream reconstructs identical projections/checksums under declared environment constraints.

- **Membranes enforced (SR-ETT):** Accountability; Isomorphic; Operational; Architectural

- **Enforcement mechanism:** oracle_run (replayability suite); snapshot+replay checksum comparison; projection rebuild assertions

- **Allowed actor kinds:** SYSTEM

- **Required refs (depends_on):** Candidate (rel=depends_on); OracleSuite suite:REPLAYABILITY (rel=depends_on); governing artifacts in force (rel=depends_on)

- **Required commitment objects:** RunStarted + RunCompleted; EvidenceBundleRecorded; archived event stream snapshot used for replay proof

- **Required evidence:** Evidence bundle containing: event stream capture hash; replay logs; projection checksum comparisons; determinism assertions

- **Verification profile / suite:** suite:REPLAYABILITY@v1

- **Success outputs:** EvidenceBundleRecorded with PASS for replay proof; archived replay artifacts

- **Success state effect:** System may claim audit replayability for covered flows; supports release readiness

- **Failure conditions:** Replay produces different projection/checksum; missing events; non-deterministic ordering; evidence missing; staleness not handled

- **Stop triggers on failure:** EVIDENCE_MISSING; ORACLE_ENV_MISMATCH; ORACLE_FLAKE

- **Relief valve:** Fix determinism; if non-determinism is unavoidable, route governance change (do not relax replayability silently).

- **Routing authority boundary on block:** GovernanceChangePortal / DecisionRecorded (human)

- **Contract refs:** C-ARCH-3; C-EVT-1; C-EVT-3; C-EVID-6

- **Spec refs:** SR-SPEC §1.5 (event log source of truth); SR-SPEC §1.6–§1.8 (projections); SR-SPEC §3.3 (status computed from events)

- **Plan refs:** D-36

- **Notes:** Replay proof should be run on the same pinned oracle suite/environment to avoid false drift signals.


## G-90: Freeze baseline created (Release Approval + FreezeRecordCreated complete)

- **Gate kind:** release_freeze

- **Applies to:** D-34..D-36

- **Purpose/decision:** Permit baseline freezing / release readiness only when a Verified candidate has an explicit human Release Approval and a complete Freeze Record enumerating governed artifacts, evidence, and active exceptions, with no unresolved staleness.

- **Membranes enforced (SR-ETT):** Authority & Integrity; Accountability; Change; Operational; Isomorphic; Resource

- **Enforcement mechanism:** portal_action (ReleaseApprovalPortal); freeze submission; oracle validation of freeze/approval linkage; staleness check

- **Allowed actor kinds:** HUMAN (approval + freeze) + SYSTEM (validation/projection)

- **Required refs (depends_on):** Candidate (rel=releases/depends_on); EvidenceBundle(s) supporting verification (rel=supported_by or approved_by as appropriate); OracleSuite used for verification (rel=depends_on); governing artifacts enumerated in artifact_manifest (rel=depends_on); Active exceptions (Deviation/Deferral/Waiver) in scope (rel=depends_on)

- **Required commitment objects:** ApprovalRecorded (portal_id=ReleaseApprovalPortal, actor_kind=HUMAN, exceptions_acknowledged[] explicit); FreezeRecordCreated (actor_kind=HUMAN); Candidate Verified (Strict or With-Exceptions); no unresolved staleness markers affecting candidate/suite/governed artifacts

- **Required evidence:** Evidence bundle refs recorded in approval/freeze; oracle validation output for freeze record completeness + exception acknowledgement; retrievability check

- **Verification profile / suite:** suite:FREEZE-VALIDATION@v1

- **Success outputs:** ApprovalRecorded; FreezeRecordCreated; Candidate computed Shippable=true (subject to no unresolved staleness)

- **Success state effect:** Candidate may be treated as Shippable for the declared baseline_id; release baseline becomes binding snapshot

- **Failure conditions:** Candidate not Verified; approval missing or not human; approval missing required fields or exceptions_acknowledged[]; freeze record incomplete; freeze references evidence that is missing; approval does not acknowledge active exceptions; unresolved staleness present

- **Stop triggers on failure:** EVIDENCE_MISSING; ORACLE_GAP; ORACLE_TAMPER; ORACLE_ENV_MISMATCH; ORACLE_FLAKE

- **Relief valve:** Resolve staleness; fix missing evidence; create/resolve exceptions; defer release; do not ship without a valid freeze baseline.

- **Routing authority boundary on block:** ReleaseApprovalPortal (for approval) + GovernanceChangePortal (if rule/field conflicts) / DecisionRecorded (human) for arbitration

- **Contract refs:** C-SHIP-1; C-TB-1; C-TB-4; C-TB-6; C-VER-1; C-EVID-6; C-EVT-6

- **Spec refs:** SR-SPEC §1.12 (Freeze record + Shippable rules); SR-SPEC §1.13 (staleness); SR-SPEC §2.3.4 (approvals); SR-SPEC §2.3.6? (freeze API); SR-SPEC §3.3.1 (Shippable)

- **Plan refs:** D-34..D-36

- **Notes:** Freeze baseline is binding; require explicit exceptions acknowledgement even when empty. Never allow agents to perform approval/freeze actions. | [vpos-note] portal:ReleaseApprovalPortal



# Step 1 Outputs — Plan→Workflow Mapping (Instance-1)

## Files produced

- `plan_to_workflow_mapping_instance1_filled.csv`

## Conventions used

### Workflow phases

- **PKG-01** — Governance hygiene and unblockers
- **PKG-02** — Repo and CI substrate
- **PKG-03** — Domain core (deterministic rules)
- **PKG-04** — Persistence, projections, and graph
- **PKG-05** — Evidence storage and integrity
- **PKG-06** — API and identity boundary
- **PKG-07** — Orchestration runtime
- **PKG-08** — Oracles and verification substrate
- **PKG-09** — UI portals and human review surface
- **PKG-10** — Self-host and operations substrate
- **PKG-11** — End-to-end demonstration and determinism proof

### Gate IDs referenced (to be defined in Gate Registry later)

- `G-API-CONTRACTS`
- `G-ASYNC-RELIABILITY`
- `G-AUTHZ-HUMAN-BINDING`
- `G-BUDGETS-STOPTRIGGERS`
- `G-BUILD-REPRO`
- `G-CI-GREEN`
- `G-CTX-ADMISSIBLE`
- `G-CTX-COMPILE-DETERMINISTIC`
- `G-DETERMINISM`
- `G-DOMAIN-PURITY`
- `G-ENG-APPROVED`
- `G-EVID-CONTENT-ADDRESSED`
- `G-EVID-MANIFEST-V1`
- `G-EVID-SECRET-HANDLING`
- `G-EVIDENCE-SCHEMA-VALIDATOR`
- `G-EVT-APPENDONLY`
- `G-FREEZE-READY`
- `G-GOV-COHERENCE`
- `G-GRAPH-SEMANTICS`
- `G-HARNESS-FAILURE-MODES`
- `G-HARNESS-HAPPY`
- `G-OBSERVABILITY-MIN`
- `G-ORACLE-ENV-PINNED`
- `G-ORACLE-INTEGRITY`
- `G-ORACLE-SANDBOXED`
- `G-PORTAL-SUBMISSIONS-RECORDED`
- `G-PROJECTIONS-REBUILDABLE`
- `G-REBUILD-FROM-EVENTS-ONLY`
- `G-RELEASE-APPROVED`
- `G-REPLAYABILITY-PROOF`
- `G-STACK-HEALTH`
- `G-STOPTRIGGER-N-DEFINED`
- `G-UI-NONBINDING`
- `G-VERIFIED-STRICT`
- `G-VERSIONS-PINNED`

### Portal IDs referenced (to be defined via Portal Playbooks later)

- `PORTAL-ENG-ACCEPT`
- `PORTAL-GOVERNANCE-CHANGE`
- `PORTAL-ORACLE-SUITE-CHANGE`
- `PORTAL-PORTAL-POLICY`
- `PORTAL-RELEASE`

### Oracle suite IDs referenced (to be defined in Step 2)

- `suite:SR-SUITE-CORE`
- `suite:SR-SUITE-FULL`
- `suite:SR-SUITE-GOV`

### Budget schema

Budgets are encoded as JSON objects in the `budgets_default` cell, using:

- `max_iterations`
- `max_oracle_runs`
- `max_wallclock_hours`

Default budgets by phase:

- **PKG-01**: `{"max_iterations":1,"max_oracle_runs":5,"max_wallclock_hours":4}`
- **PKG-02**: `{"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16}`
- **PKG-03**: `{"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16}`
- **PKG-04**: `{"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16}`
- **PKG-05**: `{"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16}`
- **PKG-06**: `{"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16}`
- **PKG-07**: `{"max_iterations":5,"max_oracle_runs":25,"max_wallclock_hours":16}`
- **PKG-08**: `{"max_iterations":6,"max_oracle_runs":40,"max_wallclock_hours":24}`
- **PKG-09**: `{"max_iterations":6,"max_oracle_runs":40,"max_wallclock_hours":24}`
- **PKG-10**: `{"max_iterations":6,"max_oracle_runs":40,"max_wallclock_hours":32}`
- **PKG-11**: `{"max_iterations":8,"max_oracle_runs":80,"max_wallclock_hours":48}`

## Notes

Where SR-PLAN marked a deliverable as conditional or called out portal semantics explicitly, those are captured in the `notes` column.

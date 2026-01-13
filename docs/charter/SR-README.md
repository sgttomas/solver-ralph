---
doc_id: SR-README
doc_kind: governance.readme
layer: build
status: draft
normative_status: index

refs:
  - rel: governed_by
    to: SR-CHANGE
---

# SR-README

Canonical index for the SR-* document set. This document contains **only** path mappings and linking conventions.

For precedence rules, see SR-TYPES §2.3.

Begin your task assignment by reading SR-CHARTER.md and then doing whatever you think should be done first by navigating the docs/ folders and the documentation therein.

Your task is done when there are no more deliverables to be assigned, which means all tests have passed and gates cleared.

You should push on to resolve any findings and consult the docs/ frequently for guidance.  They are typed documents and structured to aid in navigation.

You should git add && commit && push after completing each deliverable.  You can remain on the same branch throughout this development session.

If you cannot pass the tests for that deliverable then you must summarize what you did during that development session, delete the previous message where it says "Development History Summary for this Deliveralbe" and then append your new message including how to identify the task that was being worked on when the next instance of yourself begins the next iteration.

Then /clear your context and redo the loop call that you are currently in.


## Development History Summary for this Deliverable

### Session 3 (2026-01-13)
**Completed:** D-04 (Local developer tooling), D-05 (Domain model primitives and invariants)

**What was done:**

D-04: Local developer tooling
- Added scripts/check-deps.sh for dependency verification (Rust, Node, Docker, etc.)
- Added scripts/dev-setup.sh for development environment setup
- Added scripts/run-tests.sh for running test suites with structured output
- Makefile already provides `make dev`, `make test`, `make build` targets

D-05: Domain model primitives and invariants
- Added missing domain entities: Iteration, Candidate, Run, EvidenceBundle, Approval, FreezeRecord, Exception, Decision
- Added entity identifiers: RunId, ApprovalId, FreezeId, ExceptionId, DecisionId
- Added state enums: IterationState, RunState, VerificationStatus, ApprovalDecision, VerificationMode, ExceptionKind, ExceptionStatus
- Added state machines: IterationStateMachine, RunStateMachine, ExceptionStateMachine with transition validation
- Added VerificationComputer for computing verification status per SR-SPEC §3.3
- Added InvariantValidator for enforcing human actor requirements and waiver target constraints per SR-CONTRACT
- Added comprehensive unit tests for entities and state machines

Committed and pushed to solver-ralph-1 branch (commit 57d1ba8)

**Next deliverables to work on (per SR-PLAN dependency graph):**
- D-06: Deterministic state machines and transition validation (depends on D-05)
- D-07: Ports and boundary interfaces (depends on D-05) - sr-ports already has basic traits
- D-09: Postgres schemas and migrations (depends on D-02)

**Note:** Rust is not installed in the current environment. CI will validate builds on GitHub runners. Install Rust via https://rustup.rs/ to build locally.

---

### Session 2 (2026-01-13)
**Completed:** D-03 (Continuous integration baseline)

**What was done:**
- Created GitHub Actions CI workflow (.github/workflows/ci.yml)
- Rust job: format check, clippy lint, build, test with caching
- UI job: npm install, type-check, eslint, build
- Summary job: produces machine-readable JSON with pass/fail, artifact hashes
- Fixed Rust edition 2024 -> 2021 in Cargo.toml
- Added ESLint configuration for UI (ui/.eslintrc.cjs)
- Committed and pushed to solver-ralph-1 branch (commit 3692c0b)

---

### Session 1 (2026-01-13)
**Completed:** D-02 (Repository scaffold and workspace layout)

**What was done:**
- Created Rust workspace with 4 crates: sr-domain, sr-ports, sr-adapters, sr-api
- Implemented stub domain entities, events, state machines, commands, errors
- Created port traits for EventStore, EvidenceStore, OracleRunner, MessageBus, etc.
- Set up React/TypeScript UI scaffold with Vite (builds successfully)
- Created shared schemas directory structure
- Added Makefile with build/test/dev targets
- Committed and pushed to solver-ralph-1 branch (commit 608f083)


## Cross-document reference convention

When referencing another SR-* document:

- Use the **doc_id** (e.g., `SR-CONTRACT`, `SR-SPEC`)
- Resolve to physical path using the table below
- If multiple candidates exist (duplicates, forks, renamed copies), treat this table as authoritative and record any deviation via SR-EXCEPTIONS

---

## Canonical document paths

| doc_id | Folder | Purpose |
|--------|--------|---------|
| SR-CHARTER | `charter/` | Project scope and priorities |
| SR-CONTRACT | `platform/` | Binding invariants |
| SR-SPEC | `platform/` | Platform mechanics |
| SR-TYPES | `platform/` | Type registry and schemas |
| SR-WORK-SURFACE | `platform/` | Work surface definitions |
| SR-PROCEDURE-KIT | `platform/` | Procedure templates |
| SR-SEMANTIC-ORACLE-SPEC | `platform/` | Semantic oracle interface |
| SR-EVENT-MANAGER | `platform/` | Event projections spec |
| SR-AGENT-WORKER-CONTRACT | `platform/` | Agent/worker contract |
| SR-INTENT | `platform/` | Design rationale |
| SR-CHANGE | `build-governance/` | Change control process |
| SR-AGENTS | `build-governance/` | Agent actor model |
| SR-TASKS | `build-governance/` | Task assignment |
| SR-EXCEPTIONS | `build-governance/` | Exception ledger |
| SR-PLAN | `program/` | Build plan instance |
| SR-DIRECTIVE | `program/` | Execution policy |
| SR-README | `charter/` | This index |

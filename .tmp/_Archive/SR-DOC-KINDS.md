# SOLVER-Ralph Document Kind Registry (Doc Types)

This file defines the **document kinds** for the SOLVER-Ralph documentation set so they can remain aligned as the project evolves.

**Key rule:** A document may only *define* content categories that its doc kind allows. If a document needs to introduce or change a definition outside its allowed categories, that change must be routed to the owning doc kind (usually SR-TYPES / SR-CONTRACT / SR-SPEC) and approved per SR-CHANGE.

---

## Canonical meta-type: `governance.doc_kind`

Every SR document kind is specified by the fields below:

- **doc_kind**: stable identifier (e.g., `spec.platform.contract`)
- **layer**: `platform | build | usage`
- **domain**: `platform_definition | workflow_governance | build_scaffolding`
- **authority_scope**: what it can bind and what it cannot
- **may_define**: allowed content categories
- **must_not_define**: forbidden content categories
- **required_sections**: headings that must exist
- **required_refs**: minimum `refs` relationships
- **alignment_invariants**: cross-doc consistency rules that must hold
- **oracles**: checks that should run on changes to this doc kind
- **human_authority_required_for**: change classes that require Human Authority

---

## Content categories

These tags are used by `may_define` and `must_not_define` across doc kinds:

- `ontology` — terms, entities, identity, vocabulary
- `invariants` — must-always-hold constraints
- `mechanics` — state machines, event/record semantics, enforcement logic
- `usage_constraints` — how users/agents must behave when using governed artifacts
- `rationale` — why choices were made
- `decision_log` — non-binding record of decisions, alternatives considered, tradeoffs, and consequences
- `agent_constraints` — what agents may/may-not do; stop rules
- `change_process` — how governed docs change
- `exceptions_process` — how deviations are requested/recorded
- `deliverable_catalog` — what must be built (PKG/D inventory)
- `dependency_graph` — what depends on what (DAG)
- `execution_config` — budgets, gates, oracle selection, dispatch rules
- `completion_accounting` — what counts as done and how it’s recorded

---

## Relationship semantics (for `refs`)

- `depends_on`: semantic dependency; should propagate “needs review” when upstream meaning changes
- `constrained_by`: normative constraint (stronger than `informs`); does not necessarily imply staleness propagation
- `governed_by`: governance/process relationship; should **not** imply semantic staleness
- `informs`: non-binding guidance
- `supersedes`: used only when a document **ID** is replaced (no version pinning)

---

## Precedence scopes (decision routing)

Platform-definition precedence applies to: `ontology`, `invariants`, `mechanics`, `usage_constraints`.

Build-execution precedence applies to: `agent_constraints`, `change_process`, `exceptions_process`, `deliverable_catalog`, `dependency_graph`, `execution_config`, `completion_accounting`.

---

# Document kinds

## SR-CHARTER — `governance.charter`
- **layer:** build
- **domain:** workflow_governance
- **authority_scope:** binds purpose, milestones, initial authority model, stop rules; does **not** define platform mechanics
- **may_define:** `rationale`, high-level `usage_constraints`, high-level `agent_constraints`, milestone-level `completion_accounting`
- **must_not_define:** detailed `mechanics`, detailed `execution_config`, detailed `ontology` beyond seeds
- **required_sections:** Purpose; First Principles; Authority Model; Milestones; Stop Rules
- **required_refs:** `governed_by -> SR-CHANGE`; `depends_on -> SR-TYPES`; `constrained_by -> SR-CONTRACT`
- **alignment_invariants:** platform docs must remain reviewable against the charter
- **oracles:** no-platform-mechanics; no-versions
- **human_authority_required_for:** all changes

## SR-TYPES — `spec.platform.types`
- **layer:** platform
- **domain:** platform_definition
- **authority_scope:** binds vocabulary/ontology, relationship semantics, and this doc kind registry
- **may_define:** `ontology`
- **must_not_define:** `execution_config`, `deliverable_catalog`
- **required_sections:** Core Vocabulary; Authority/Evidence/Record lattice; Relationship semantics; Doc kind registry
- **required_refs:** `governed_by -> SR-CHANGE`; `constrained_by -> SR-CONTRACT`
- **alignment_invariants:** all other docs must use SR-TYPES terms
- **oracles:** undefined-term; cross-doc-term-consistency
- **human_authority_required_for:** semantic changes to ontology

## SR-CONTRACT — `spec.platform.contract`
- **layer:** platform
- **domain:** platform_definition
- **authority_scope:** binds invariants and trust boundaries
- **may_define:** `invariants`
- **must_not_define:** build workflow policy, detailed implementation mechanics
- **required_sections:** Invariants; Trust Boundaries / Authority Ports; Binding Conditions
- **required_refs:** `depends_on -> SR-TYPES`; `governed_by -> SR-CHANGE`
- **alignment_invariants:** SR-SPEC must implement these invariants
- **oracles:** invariant-enforceability
- **human_authority_required_for:** all invariant edits

## SR-SPEC — `spec.platform.mechanics`
- **layer:** platform
- **domain:** platform_definition
- **authority_scope:** binds operational semantics that realize invariants
- **may_define:** `mechanics`
- **must_not_define:** build dispatch policy, deliverable catalog
- **required_sections:** Domain model; State machines; Evidence/Oracles; Authority Ports; Record Store + Projection
- **required_refs:** `depends_on -> SR-CONTRACT`; `depends_on -> SR-TYPES`; `governed_by -> SR-CHANGE`
- **alignment_invariants:** must not weaken SR-CONTRACT invariants
- **oracles:** invariant-coverage
- **human_authority_required_for:** changes affecting binding mechanics

## SR-GUIDE — `spec.platform.guide`
- **layer:** usage
- **domain:** platform_definition
- **authority_scope:** binds usage constraints and interpretation guidance; does not define mechanics
- **may_define:** `usage_constraints`
- **must_not_define:** new `ontology`, new `invariants`, new `mechanics`
- **required_sections:** Usage of governed artifacts; Agent claim boundaries (usage framing); Examples
- **required_refs:** `depends_on -> SR-TYPES`; `constrained_by -> SR-CONTRACT`
- **alignment_invariants:** must not contradict SR-CONTRACT/SR-SPEC
- **oracles:** no-new-definitions
- **human_authority_required_for:** changes that alter interpretation constraints

## SR-INTENT — `workflow.intent_log`
- **layer:** build
- **domain:** workflow_governance
- **authority_scope:** non-binding **decision log** and rationale registry used to preserve design pressure (tradeoffs, rejected alternatives, ambiguity heuristics) without reopening contracted meaning; may be cited only as rationale when multiple options already comply with contracted docs
- **may_define:** `rationale`, `decision_log`
- **must_not_define:** `ontology`, `invariants`, `mechanics`, `usage_constraints`, `deliverable_catalog`, `dependency_graph`, `execution_config`
- **required_sections:** Decision Entries; Tradeoffs; Rejected Alternatives; Consequences
- **required_refs:** `depends_on -> SR-CHARTER`; `governed_by -> SR-CHANGE`; `informs -> SR-AGENTS`; `informs -> SR-TYPES`; `informs -> SR-CONTRACT`; `informs -> SR-SPEC`; `informs -> SR-DIRECTIVE`
- **alignment_invariants:**
  - contains no normative language that could be interpreted as binding (no MUST/SHALL/REQUIRED)
  - must not be used to override SR-TYPES/SR-CONTRACT/SR-SPEC or build-execution rules; if conflict exists, INTENT loses
  - each decision entry is explicitly labeled (e.g., INT-####) and states: decision, alternatives, rationale, consequences
- **oracles:** no-normative-language; intent-entry-schema
- **human_authority_required_for:** none by default (apply SR-CHANGE rules)


## SR-AGENTS — `workflow.agents`
- **layer:** build
- **domain:** workflow_governance
- **authority_scope:** binds agent constraints, escalation rules, stop triggers
- **may_define:** `agent_constraints`
- **must_not_define:** platform `ontology`, `invariants`, `mechanics` (must reference instead)
- **required_sections:** Constraints; Stop Rules; Precedence Routing (summary)
- **required_refs:** `depends_on -> SR-CHARTER`; `depends_on -> SR-TYPES`; `constrained_by -> SR-CONTRACT`; `governed_by -> SR-CHANGE`
- **alignment_invariants:** reinforces evidence ≠ authority; no self-approval
- **oracles:** no-platform-semantics
- **human_authority_required_for:** changes that weaken constraints

## SR-CHANGE — `workflow.change`
- **layer:** build
- **domain:** workflow_governance
- **authority_scope:** binds how governed docs change and how conflicts are resolved
- **may_define:** `change_process`
- **must_not_define:** platform semantics; deliverables
- **required_sections:** Change Proposal; Conflict Resolution; Approval Rules by doc_kind
- **required_refs:** `depends_on -> SR-TYPES`; `informs -> SR-AGENTS`
- **alignment_invariants:** must be able to govern itself (self-application)
- **oracles:** doc-kind-coverage (every doc_kind has change rules)
- **human_authority_required_for:** changes to approval thresholds / authority routing

## SR-EXCEPTIONS — `workflow.exceptions`
- **layer:** build
- **domain:** workflow_governance
- **authority_scope:** binds recorded exceptions as explicit deviation permits
- **may_define:** `exceptions_process`
- **must_not_define:** new `invariants`, new `mechanics`, new `ontology`
- **required_sections:** Rules; Exception Entries
- **required_refs:** `governed_by -> SR-CHANGE`; `informs -> SR-AGENTS`
- **alignment_invariants:** deviations must cite EX-#### entries
- **oracles:** closure-required; no-open-ended-exceptions
- **human_authority_required_for:** all exception entries (approval)

## SR-PLAN — `build.plan`
- **layer:** build
- **domain:** build_scaffolding
- **authority_scope:** binds the deliverable catalog and acceptance criteria; does not define platform meaning
- **may_define:** `deliverable_catalog`, deliverable-level `completion_accounting`
- **must_not_define:** `ontology`, `invariants`, `mechanics`, `usage_constraints`, `execution_config`
- **required_sections:** Deliverables; Acceptance Criteria; Output Artifact Map
- **required_refs:** `depends_on -> SR-CHARTER`; `constrained_by -> SR-CONTRACT`; `governed_by -> SR-CHANGE`
- **alignment_invariants:** any platform-meaning statements must be references, never definitions
- **oracles:** no-platform-semantics-in-plan; acceptance-criteria-present
- **human_authority_required_for:** none by default (apply SR-CHANGE)

## SR-DIRECTIVE — `build.directive`
- **layer:** build
- **domain:** build_scaffolding
- **authority_scope:** binds execution configuration and dependency policy under platform semantics
- **may_define:** `dependency_graph`, `execution_config`
- **must_not_define:** platform `ontology`, `invariants`, `mechanics`, `usage_constraints`
- **required_sections:** Dependency Model; Dispatch Policy; Gates/Oracles Config; Budgets + Stop Triggers
- **required_refs:** `depends_on -> SR-PLAN`; `depends_on -> SR-AGENTS`; `depends_on -> SR-EXCEPTIONS`; `constrained_by -> SR-CONTRACT`; `governed_by -> SR-CHANGE`
- **alignment_invariants:** must not create a path that violates SR-CONTRACT/SR-SPEC
- **oracles:** no-semantic-overrides; exceptions-route-to-sr-exceptions
- **human_authority_required_for:** changes to stop triggers / budgets / gating rules

## SR-RUNLIST — `build.runlist`
- **layer:** build
- **domain:** build_scaffolding
- **authority_scope:** binds the current execution view (linear projection + progress tracker) of SR-PLAN scope; does not define scope, dependencies, or platform meaning
- **may_define:** `completion_accounting` (status/progress tracking), execution notes
- **must_not_define:** `deliverable_catalog` (scope), authoritative `dependency_graph`, platform `ontology/invariants/mechanics`
- **required_sections:** Rules; Execution Rule (single-agent mode); Items
- **required_refs:** `governed_by -> SR-CHANGE`; `depends_on -> SR-PLAN`; `depends_on -> SR-DIRECTIVE`; `depends_on -> SR-AGENTS`; `depends_on -> SR-EXCEPTIONS`; `constrained_by -> SR-CONTRACT`
- **alignment_invariants:**
  - every tracked item must reference an ID defined in SR-PLAN
  - SR-DIRECTIVE dependency graph and gates remain authoritative; ordering here is convenience only
  - agents may advance items to REVIEW; only Human Authority marks DONE
- **oracles:** ids-exist-in-plan; no-new-scope; approvals-required-for-done; no-versions
- **human_authority_required_for:** marking items DONE; changes that weaken authority gating

---

## Minimal oracle checklist (recommended)

- frontmatter-schema
- no-versions
- category-ownership (doc_kind may_define/must_not_define)
- undefined-term (no new term without SR-TYPES)
- precedence-consistency (if precedence is present)
- exceptions-citation-required (if deviation language appears)

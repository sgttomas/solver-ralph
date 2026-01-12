# SR-DIRECTIVE Authoring Kit (v1.0.0-draft.1)

This kit is a **fill‑in toolkit** for drafting an SR‑DIRECTIVE with minimal freeform prose.

It assumes the governed‑stack separation:

- **SR‑PLAN**: deliverables + dependencies (abstract; does **not** specify closure/execution semantics)
- **SR‑CONTRACT**: invariants (what must always be true)
- **SR‑SPEC**: mechanics (how invariants are enforced in the harness)
- **SR‑ETT**: membrane topology (where coercion belongs; how to place constraints)
- **SR‑DIRECTIVE**: operational policy (how we execute SR‑PLAN *using* SR‑SPEC, while never violating SR‑CONTRACT, with constraint placement guided by SR‑ETT)

## What’s in this kit

- `sr_directive_scaffold_template.md`  
  A section‑complete SR‑DIRECTIVE scaffold with placeholders for the tables below.

- `plan_to_workflow_mapping_template.csv`  
  Blank template mapping SR‑PLAN deliverables → workflows.

- `plan_to_workflow_mapping_instance1_seed.csv`  
  Seeded table containing **D‑01..D‑36** extracted from SR‑PLAN instance‑1 (ids/titles/deps/profile recommendations).  
  Fill in phases, gates, portals, evidence requirements, and budgets.

- `profile_definitions_template.yaml`  
  Template for defining Verification Profiles + Oracle Suites.

- `profile_definitions_seed.yaml`  
  Minimal starter seed (STRICT‑CORE / STRICT‑FULL placeholders).

- `gate_registry_template.csv`  
  Blank Gate Registry with SR‑ETT‑aligned fields (membranes, enforcement surface, actor kind, relief valve).

- `gate_registry_seed.csv`  
  A tiny starter seed with a single canonical “Context admissible / No ghost inputs” gate.

- `portal_playbook_template.md`  
  Template for portal playbooks (approval/decision/exception/freeze/budget/oracle‑suite change).

- `portal_playbook_seed__*.md`  
  Seeds for the minimum portals you will almost certainly need (governance, release, exceptions).

## How to use (recommended workflow)

1) **Fill the Plan‑to‑Workflow mapping**  
   Start with `plan_to_workflow_mapping_instance1_seed.csv` and fill:
   - `workflow_phase`
   - `start_gate_ids` / `accept_gate_ids` / `release_gate_ids`
   - `required_portals`
   - `required_oracle_suite_id` / `recommended_profile` → final profile binding
   - `required_evidence_bundle_kinds`
   - `budgets_default` and any `stop_triggers_overrides`

2) **Define Verification Profiles + Oracle Suites**  
   Use `profile_definitions_template.yaml` (or seed) to define:
   - OracleSuite(s): deterministic, pinned, with environment constraints
   - Verification Profiles: which suites are required/advisory for each work‑unit/deliverable class
   - Waiver policy (STRICT vs WITH_EXCEPTIONS) and which failures are non‑waivable

3) **Write Portal Playbooks**  
   For each portal you will use in the directive, complete a playbook from `portal_playbook_template.md`:
   - allowed request types
   - required evidence + refs
   - binding record(s) emitted
   - failure handling and escalation

4) **Define the Gate Registry**  
   Use `gate_registry_template.csv`:
   - Every gate must name: membranes enforced, enforcement surface (hex), allowed actor kind(s), required commitment objects, and the relief valve (if any).
   - Prefer port‑level enforcement for boundary crossings; keep domain‑core invariant logic infrastructure‑free.

5) **Assemble SR‑DIRECTIVE**  
   Copy `sr_directive_scaffold_template.md` and paste in:
   - the filled Plan‑to‑Workflow table
   - the filled Gate Registry
   - the profile definitions
   - the portal playbooks
   - any additional policy prose strictly needed to disambiguate execution (keep prose thin)

## Quick “don’t get stuck” rules

- If you need to change *binding meaning* (e.g., what Verified/Approved/Shippable mean), **don’t encode that in SR‑DIRECTIVE** — route as governance change.
- Keep the default posture conservative: if evidence/refs are missing or non‑dereferenceable, block and escalate rather than “trusting” narrative.
- Ensure IterationStarted refs include the active governed artifacts and the active oracle suite(s) for that work unit.


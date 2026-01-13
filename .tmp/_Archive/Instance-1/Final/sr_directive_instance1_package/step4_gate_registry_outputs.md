# Step 4 — Gate Registry (instance-1)

Generated on: 2026-01-11

## Inputs
- `gate_registry_template.csv`
- `gate_registry_seed.csv`
- `plan_to_workflow_mapping_instance1_filled.csv` (to extract referenced gate ids and back-reference plan deliverables)

## Output
- `gate_registry_instance1_filled.csv`

## Notes
- Gate rows include membrane coverage, enforcement surfaces, required commitment objects, and relief valves, per kit guidance.
- `plan_refs` back-references the SR-PLAN instance-1 deliverables that cite each gate in the Plan→Workflow mapping.
- `G-CTX-ADMISSIBLE` is the canonical context-start gate used by the instance-1 mapping; if `G-00` is present, it is treated as a legacy alias with equivalent intent.

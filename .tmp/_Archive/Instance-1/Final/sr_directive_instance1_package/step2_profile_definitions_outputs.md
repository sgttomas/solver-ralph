# Step 2 — Verification Profiles + Oracle Suites (instance-1)

Generated: 2026-01-11

## Outputs
- `profile_definitions_instance1_filled.yaml`

## What changed vs seed
- Added explicit suite IDs aligned to Step 1 mapping:
  - `suite:SR-SUITE-GOV`
  - `suite:SR-SUITE-CORE`
  - `suite:SR-SUITE-FULL`
- Added a governance-only profile `GOV-CORE` used for non-code governance intervention records.
- Expanded `oracles[]` lists for each suite with:
  - deterministic core checks (meta/schema/build/unit/lint)
  - integration/e2e + replay checks for FULL
  - advisory SBOM oracle in FULL
- Populated `profile_selection_matrix` for all work-unit types in the Plan→Workflow mapping.

## Suite conventions
- Suites are designed to be:
  - deterministic for all **required** oracles,
  - pinned by OCI digest, and
  - executed in a sandboxed environment with a recorded environment fingerprint.

Commands are expressed as `sr-oracles ...` entrypoints as placeholders; actual entrypoints MUST be implemented and kept stable once pinned.

## Profile selection policy (binding defaults)
- `STRICT-FULL` defaults for the deliverables that SR-PLAN recommends STRICT-FULL:
  - D-26, D-31, D-32, D-34, D-35, D-36
- `STRICT-CORE` defaults for all other implementation work-unit types.
- `GOV-CORE` for governance-only work units.

## Waivers
- Both STRICT profiles allow WITH_EXCEPTIONS as an **explicit** mode (not default).
- Integrity conditions remain non-waivable.


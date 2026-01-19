---
doc_id: SR-PLAN-MVP1
doc_kind: governance.plan
layer: program
status: draft
refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: implements
    to: docs/charter/SR-README
  - rel: depends_on
    to: SR-TEMPLATES
  - rel: depends_on
    to: SR-WORK-SURFACE
  - rel: depends_on
    to: SR-CONTRACT
---

# SR-PLAN-MVP1 — Nomenclature Refactor + Fresh UI Build

**Purpose:** Comprehensive implementation plan for the nomenclature refactor (Part A) and fresh UI build (Part B) as specified in docs/charter/SR-README.

**Scope:** This document provides discrete, executable tasks with sufficient context that a future agent instance can execute without rediscovery.

---

## Executive Summary

This plan implements two interconnected changes:

| Part | Description | Estimated Files | Key Change |
|------|-------------|-----------------|------------|
| **A** | Nomenclature Refactor | ~75 files, ~481 occurrences | `ProcedureTemplate` → `Template` |
| **B** | Fresh UI Build | ~10 new/modified files | New WorkScreen with auto-linked evidence |

**Critical Path:** A2 (domain types) → A3 (module rename) → A4-A5 (handlers/adapters) → A6 (migration) → A7 (UI) → Part B

---

## Progress Log

| Date | Task | Status | Notes |
|------|------|--------|-------|
| 2026-01-18 | A1 | ✅ COMPLETE | SR-PROCEDURE-KIT.md deleted; SR-TEMPLATES.md updated with new terminology; 15+ doc refs updated across platform/, charter/, build-governance/, program/, skeletons/ |
| 2026-01-18 | A2 | ✅ COMPLETE | work_surface.rs fully updated: `ProcedureTemplate`→`Template`, `ProcedureTemplateId`→`TemplateId`, `ProcedureStep`→`Guidepost`, `GateRule` removed entirely, all field renames done, tests updated |
| 2026-01-18 | A3 | ✅ COMPLETE | procedure_templates.rs renamed to templates.rs via `git mv`; lib.rs updated; all types/functions updated to new nomenclature |
| 2026-01-18 | A4 | ✅ COMPLETE | sr-api handlers updated: imports changed to `TemplateId`, module paths to `templates::`, struct fields renamed `procedure_template_*` → `template_*`, SQL row.get() updated |
| 2026-01-18 | A5 | ✅ COMPLETE | sr-adapters updated: EvidenceManifest struct, builder methods, semantic_worker.rs, semantic_suite.rs, integrity.rs |
| 2026-01-18 | A5.5 | ✅ COMPLETE | Backward compat added in projections.rs: dual-read pattern for `template_ref`/`procedure_template_ref` |
| 2026-01-18 | A6 | ✅ COMPLETE | migrations/013_template_renames.sql created |
| 2026-01-18 | A7 | ✅ COMPLETE | UI JSON field names updated in 11 files (`procedure_template_id` → `template_id`, etc.); TypeScript interface names renamed `ProcedureTemplate` → `Template` in 4 files |
| 2026-01-18 | A8 | ✅ COMPLETE | API route renamed: `/api/v1/references/procedure-templates` → `/api/v1/references/templates` |
| 2026-01-19 | A-Tests | ✅ COMPLETE | Test files updated: `ProcedureTemplateId` → `TemplateId` in event_manager.rs, replay_determinism_test.rs; struct fields `procedure_template_id` → `template_id` in 3 integration tests |
| 2026-01-19 | A6-Apply | ✅ COMPLETE | Migration 013 applied to database; columns renamed `procedure_template_id` → `template_id`, `procedure_template_hash` → `template_hash` |
| 2026-01-19 | A-SQL | ✅ COMPLETE | SQL queries updated in work_surfaces.rs, iterations.rs, projections.rs to use new column names |
| 2026-01-19 | B1 | ✅ COMPLETE | migrations/014_evidence_work_surface_linking.sql created and applied; added `work_surface_id`, `template_id`, `stage_id` columns to `proj.evidence_bundles` |
| 2026-01-19 | B2 | ✅ COMPLETE | Evidence projection handler updated to store work_surface_id, template_id, stage_id from event payload |
| 2026-01-19 | B3 | ✅ COMPLETE | Added `GET /api/v1/work-surfaces/{id}/evidence` endpoint for auto-loaded evidence |
| 2026-01-19 | B4 | ✅ COMPLETE | Created WorkScreen.tsx with unified work view: context, auto-loaded evidence, loop state, judgment actions |
| 2026-01-19 | B5 | ✅ COMPLETE | Supporting components integrated into WorkScreen (inline sections rather than separate files) |
| 2026-01-19 | B6 | ✅ COMPLETE | Route `/work/:workSurfaceId` added to routes.tsx; WorkScreen exported from pages/index.ts |
| 2026-01-18 | A7-UI | ✅ COMPLETE | TypeScript interfaces renamed: `ProcedureTemplate` → `Template` in Protocols.tsx, ProtocolDetail.tsx, LoopCreateModal.tsx, LoopDetail.tsx; variable names `procedureTemplateId` → `templateId` |
| 2026-01-18 | A-RefKind | ✅ COMPLETE | `RefKind::ProcedureTemplate` → `RefKind::Template` in refs.rs; UI string literals updated in References.tsx, InputsEditor.tsx |
| 2026-01-18 | B7 | ✅ COMPLETE | Deleted unused PromptLoopScreen.tsx and PromptLoopScreen.module.css; EvidenceBundleSelector and StageCompletionForm retained (still in use) |

**Current State (2026-01-18):**
- **Part A:** COMPLETE - All nomenclature refactoring done, including UI interfaces and RefKind
- **Part B:** COMPLETE - WorkScreen functional at `/work/:workSurfaceId`
- **Cleanup:** COMPLETE - Tasks A7 and B7 finished
- **Migration 013:** Applied (column renames)
- **Migration 014:** Applied (evidence work surface linking)
- **Verification:** `cargo test --workspace` ✅, `npm run type-check` ✅, `grep ProcedureTemplate ui/src/` = 0 results ✅

**Remaining Work:** None. All `ProcedureTemplate` references removed.

**Completed Cleanup:**
- ✅ UI TypeScript interfaces renamed (`ProcedureTemplate` → `Template`)
- ✅ UI variable names renamed (`procedureTemplateId` → `templateId`)
- ✅ UI string literals updated (`'ProcedureTemplate'` → `'Template'`)
- ✅ `RefKind::ProcedureTemplate` → `RefKind::Template` in refs.rs
- ✅ `ProcedureTemplateSelected` → `TemplateSelected` in events.rs
- ✅ `ProcedureTemplateWithRef` → `TemplateWithRef` in plan_instance.rs
- ✅ `procedure_template_ref` → `template_ref` in events.rs
- ✅ Dual-read fallback patterns removed from projections.rs and work_surfaces.rs
- ✅ PromptLoopScreen.tsx deleted (unused)
- ✅ EvidenceBundleSelector.tsx, StageCompletionForm.tsx retained (still actively used by WorkSurfaceDetail)

**Resume Point:** MVP1 nomenclature cleanup complete. Zero `ProcedureTemplate` occurrences remain.

---

## Part A: Nomenclature Refactor

### Context

The term "Procedure Template" conflates the core loop with future orchestration. "Template" clarifies that it defines **what a good candidate looks like**, not how to produce it. The human is the gate; oracles inform, they don't decide.

**Rename Summary:**

| Old | New | Rationale |
|-----|-----|-----------|
| `ProcedureTemplate` | `Template` | Clarity: defines expected output, not process |
| `ProcedureTemplateId` | `TemplateId` | Consistency |
| `procedure_template_id` | `template_id` | Consistency |
| `procedure_template_ref` | `template_ref` | Consistency |
| `ProcedureStep` | `Guidepost` | Advisory landmarks, not prescriptive steps |
| `steps: Vec<ProcedureStep>` | `guideposts: Vec<Guidepost>` | Consistency |
| `GateRule` | **REMOVE** | Human is the gate; redundant |
| `gate_rule` field | **REMOVE** | Human is the gate; redundant |

---

### Task A1: Documentation — Merge SR-PROCEDURE-KIT into SR-TEMPLATES

**Context:** SR-PROCEDURE-KIT content moves into SR-TEMPLATES with new nomenclature. All references to SR-PROCEDURE-KIT must be updated or removed.

**Files to Modify:**

| File | Lines | Changes |
|------|-------|---------|
| `docs/platform/SR-TEMPLATES.md` | 16, 116 | Remove SR-PROCEDURE-KIT refs; update §2.2 terminology |
| `docs/platform/SR-WORK-SURFACE.md` | 16 | Remove ref to SR-PROCEDURE-KIT |
| `docs/platform/SR-SEMANTIC-ORACLE-SPEC.md` | 18 | Remove ref to SR-PROCEDURE-KIT |
| `docs/platform/SR-EVENT-MANAGER.md` | 22 | Remove ref to SR-PROCEDURE-KIT |
| `docs/platform/SR-TYPES.md` | 182, 247 | Update references, remove doc_kind entry |
| `docs/platform/SR-BRANCH-0-ACCEPTANCE.md` | 40 | Update reference to SR-TEMPLATES |
| `docs/platform/EXECUTION-PLAN.md` | 19 | Update reference to SR-TEMPLATES |

**File to Delete:**
- `docs/platform/SR-PROCEDURE-KIT.md`

**SR-TEMPLATES.md Updates:**

1. Update frontmatter refs (line 16): Remove `to: SR-PROCEDURE-KIT`
2. Update §2.2 "Procedure Template" → "Template" throughout
3. Update field names in schema tables:
   - `procedure_template_id` → `template_id`
   - `procedure_template_ref` → `template_ref`
   - `steps[]` → `guideposts[]`
   - Remove `gate_rule` field
4. Incorporate SR-PROCEDURE-KIT §2 content (GENERIC-KNOWLEDGE-WORK template)
5. Add new §2.2.1 explaining `requires_approval` replaces `gate_rule`

**Verification:**
```bash
grep -r "SR-PROCEDURE-KIT" docs/
# Expected: 0 results (except this plan file)
```

**Dependencies:** None (start here)

---

### Task A2: Domain Renames — work_surface.rs

**File:** `crates/sr-domain/src/work_surface.rs` (1970 lines)

**Type Definitions to Rename:**

| Old Name | New Name | Lines | Notes |
|----------|----------|-------|-------|
| `ProcedureTemplateId` | `TemplateId` | 43-60 | Struct + impl block |
| `ProcedureTemplate` | `Template` | 442-488 | Main struct |
| `ProcedureStep` | `Guidepost` | 352-363 | Struct definition |
| `ProcedureTemplateValidator` | `TemplateValidator` | 1179-1289 | Validator struct + impl |

**Field Renames in Structs:**

| Struct | Old Field | New Field | Line |
|--------|-----------|-----------|------|
| `Template` (was ProcedureTemplate) | `procedure_template_id` | `template_id` | 458 |
| `Stage` | `steps` | `guideposts` | 416 |
| `ContentAddressedRef` uses | `procedure_template_ref` | `template_ref` | Various |
| `ManagedWorkSurface` | `procedure_template_ref` | `template_ref` | 853 |
| `WorkSurfaceInstance` | `procedure_template_ref` | `template_ref` | 583 |

**Items to REMOVE:**

| Item | Lines | Reason |
|------|-------|--------|
| `GateRule` enum | 366-386 | Human is the gate |
| `GateRule` impl Default | 382-386 | Removed with enum |
| `gate_rule` field in `Stage` | 422-423 | Removed with enum |

**Impl Blocks to Update:**

| Impl | Lines | Changes |
|------|-------|---------|
| `ProcedureTemplateId` → `TemplateId` | 48-60 | Rename impl block |
| `ProcedureTemplate` → `Template` | 490-533 | Rename, update method refs |
| `ProcedureTemplateValidator` → `TemplateValidator` | 1182-1288 | Rename, update error messages |

**Serde Attributes:**
- Line 450-451: Update `default_artifact_type()` to return `"config.template"` (was `"config.procedure_template"`)
- Line 368: Remove `#[serde(rename_all = "snake_case")]` from GateRule (deleting)

**Tests to Update (lines 1362-1970):**
- `test_procedure_template_id_format()` → `test_template_id_format()` (line 1374)
- `test_procedure_template_creation()` → `test_template_creation()` (line 1467)
- `test_procedure_template_invalid_terminal()` → `test_template_invalid_terminal()` (line 1515)
- `test_gate_rule_serialization()` → DELETE (line 1622)
- All assertions checking `"config.procedure_template"` → `"config.template"`

**Verification:**
```bash
cargo check --package sr-domain
grep -n "ProcedureTemplate" crates/sr-domain/src/work_surface.rs
grep -n "GateRule" crates/sr-domain/src/work_surface.rs
grep -n "gate_rule" crates/sr-domain/src/work_surface.rs
# Expected: 0 results for each
```

**Dependencies:** A1 (documentation provides rationale reference)

---

### Task A3: Domain Renames — procedure_templates.rs → templates.rs

**Current File:** `crates/sr-domain/src/procedure_templates.rs` (620 lines)
**New File:** `crates/sr-domain/src/templates.rs`

**Step 1: Rename file**
```bash
git mv crates/sr-domain/src/procedure_templates.rs crates/sr-domain/src/templates.rs
```

**Step 2: Update lib.rs module declaration**

File: `crates/sr-domain/src/lib.rs`
- Line 28: `pub mod procedure_templates;` → `pub mod templates;`
- Line 45: `pub use procedure_templates::*;` → `pub use templates::*;`

**Step 3: Update imports in templates.rs**

Lines 10-13 (current):
```rust
use crate::work_surface::{
    GateRule, OracleSuiteBinding, ProcedureTemplate, ProcedureTemplateId, RequiredOutput, Stage,
    StageId, TransitionTarget, WorkKind,
};
```

Change to:
```rust
use crate::work_surface::{
    OracleSuiteBinding, Template, TemplateId, RequiredOutput, Stage,
    StageId, TransitionTarget, WorkKind, Guidepost,
};
```

**Step 4: Update function signatures and bodies**

| Function | Line | Changes |
|----------|------|---------|
| `problem_statement_ingestion_template()` | 31 | Return `Template`, use `TemplateId`, remove `gate_rule` assignments |
| `generic_knowledge_work_template()` | 151 | Return `Template`, use `TemplateId`, remove `gate_rule` assignments |
| `get_registered_templates()` | 345 | Return type unchanged (uses TemplateRegistryEntry) |
| `get_template_by_id()` | 353 | Parameter: `&TemplateId` |
| `create_entry()` | 360 | Parameter/return: `Template` |
| `compute_template_hash()` | 370 | Parameter: `&Template` |

**Step 5: Remove GateRule pattern matching**

Lines 400-408 (current):
```rust
let gate_str = match &stage.gate_rule {
    GateRule::AllRequiredOraclesPass => "all_required_oracles_pass",
    GateRule::AllOraclesPass => "all_oracles_pass",
    GateRule::PortalApprovalRequired => "portal_approval_required",
    GateRule::Custom(s) => s.as_str(),
};
```

Remove this block entirely from `compute_template_hash()`.

**Step 6: Update Stage definitions**

Remove all `gate_rule: GateRule::*` lines:
- Line 66: `gate_rule: GateRule::AllRequiredOraclesPass,`
- Line 97: `gate_rule: GateRule::AllRequiredOraclesPass,`
- Line 125: `gate_rule: GateRule::PortalApprovalRequired,`
- (Similar in generic_knowledge_work_template)

Change `steps: vec![],` to `guideposts: vec![],` at:
- Lines 64, 92, 123 (problem_statement_ingestion_template)
- Lines 193, 224, 251, 287, 315 (generic_knowledge_work_template)

**Step 7: Update TemplateRegistryEntry struct**

Line 339:
```rust
pub struct TemplateRegistryEntry {
    pub template: Template,  // was ProcedureTemplate
    pub content_hash: ContentHash,
}
```

**Verification:**
```bash
cargo check --package sr-domain
grep -rn "procedure_templates" crates/sr-domain/
grep -rn "GateRule" crates/sr-domain/src/templates.rs
# Expected: 0 results
```

**Dependencies:** A2 (work_surface.rs types must exist first)

---

### Task A4: API Handler Updates

**Files to Modify:**

#### A4.1: `crates/sr-api/src/handlers/templates.rs` (1782 lines)

| Location | Current | New |
|----------|---------|-----|
| Line 88 | `"config.procedure_template"` | `"config.template"` |
| Line 249 | `"procedure_template_id": "proc:..."` | `"template_id": "proc:..."` |
| Line 803 | `FieldSchema { name: "procedure_template_id"...}` | `FieldSchema { name: "template_id"...}` |
| Line 822 | `"procedure_template_ref"` | `"template_ref"` |

#### A4.2: `crates/sr-api/src/handlers/work_surfaces.rs` (2118 lines)

**Request/Response Structs:**

| Struct | Line | Field Change |
|--------|------|--------------|
| `CreateWorkSurfaceRequest` | 71 | `procedure_template_id` → `template_id` |
| `WorkSurfaceResponse` | 84-85 | `procedure_template_id` → `template_id`, `procedure_template_hash` → `template_hash` |
| `WorkSurfaceSummary` | 115-116 | `procedure_template_id` → `template_id`, `procedure_template_name` → `template_name` |
| `ListWorkSurfacesQuery` | 130 | `procedure_template_id` → `template_id` |
| `ProcedureTemplateSummary` | ~223 | Rename struct to `TemplateSummary`, rename field |
| `CompatibleTemplatesResponse` | ~220 | Update to use `TemplateSummary` |

**Function Renames:**
- Line 1251: `get_procedure_template_from_registry()` → `get_template_from_registry()`
- Line 1290: `list_procedure_templates_from_registry()` → `list_templates_from_registry()`

**SQL Queries (column aliases for backward compat during migration):**
- Line 585: Keep SQL column name, alias in SELECT if needed
- Lines 1369-1389: Same approach

**Row Mapping:**
- Line 1402-1403: Update field names in mapping
- Line 1438-1439: Update field names in mapping

#### A4.3: `crates/sr-api/src/handlers/evidence.rs` (1072 lines)

| Location | Change |
|----------|--------|
| Line 54 | `procedure_template_id` → `template_id` in struct |
| Lines 271-272 | Update conditional field name |
| Line 346 | Update response construction |

#### A4.4: `crates/sr-api/src/handlers/iterations.rs` (605 lines)

| Location | Change |
|----------|--------|
| Line 229 | Update column selection or alias |
| Line 249-250 | Update row extraction |

#### A4.5: `crates/sr-api/src/handlers/verification.rs`

| Location | Change |
|----------|--------|
| Line 64 | `procedure_template_id` → `template_id` |

#### A4.6: `crates/sr-api/src/handlers/references.rs` (998 lines)

| Location | Change |
|----------|--------|
| Lines 487-497 | Update `list_procedure_templates()` → `list_templates()`, filter key |

#### A4.7: `crates/sr-api/src/handlers/prompt_loop.rs` (1321 lines)

| Location | Change |
|----------|--------|
| Line 51 | `procedure_template_id` → `template_id` |
| Line 102 | Update domain function call |
| Line 1317 | Update default initialization |

**Verification:**
```bash
cargo check --package sr-api
grep -rn "procedure_template" crates/sr-api/src/handlers/
# Expected: 0 results (except comments)
```

**Dependencies:** A2, A3 (domain types must be renamed first)

---

### Task A5: Adapter/Projection Updates

**File:** `crates/sr-adapters/src/projections.rs`

#### A5.1: WorkSurfaceBound Event Handler (lines 2173-2227)

**Payload Extraction (lines 2178-2188):**
```rust
// Current
let template_ref = &payload["procedure_template_ref"];

// Change to
let template_ref = &payload["template_ref"];
```

**INSERT Query (lines 2199-2224):**

Column list (line 2203):
```sql
-- Current
procedure_template_id, procedure_template_hash, current_stage_id

-- Note: Keep column names until migration A6 runs
-- After A6, change to:
template_id, template_hash, current_stage_id
```

**Binding (lines 2213-2214):**
```rust
// Extract from payload with new field name
.bind(template_ref["id"].as_str())
.bind(template_ref["content_hash"].as_str())
```

#### A5.2: `crates/sr-adapters/src/evidence.rs`

**EvidenceManifest struct (lines 27-87):**
- Line 76: `procedure_template_id` → `template_id`
- Line ~80: `stage_id` (unchanged)
- Line ~84: `work_surface_id` (unchanged)

**Builder methods (lines 426-445):**
- Line 429: `with_procedure_template()` → `with_template()`
- Parameter name: `procedure_template_id` → `template_id`

#### A5.3: `crates/sr-adapters/src/semantic_worker.rs`

| Location | Change |
|----------|--------|
| Line 197 | `procedure_template_id` → `template_id` |
| Line 1230 | Update default value reference |

#### A5.4: `crates/sr-adapters/src/semantic_suite.rs`

| Location | Change |
|----------|--------|
| Line 523 | `procedure_template_id` → `template_id` |

#### A5.5: `crates/sr-adapters/src/integrity.rs`

| Location | Change |
|----------|--------|
| Line 1162 | `procedure_template_id` → `template_id` |

**Verification:**
```bash
cargo check --package sr-adapters
cargo test --package sr-adapters
grep -rn "procedure_template" crates/sr-adapters/
# Expected: 0 results (except comments)
```

**Dependencies:** A2, A3, A4 (all code changes before migration)

---

### Task A5.5: Event Payload Backward Compatibility

**Context:** Existing events in the event store contain `procedure_template_id` and `procedure_template_ref` in their JSON payloads. Projection handlers must handle both old and new field names during transition to ensure event replay works correctly.

**Files to Modify:**

#### `crates/sr-adapters/src/projections.rs`

**Dual-Read Pattern for Payload Extraction:**

When extracting template references from event payloads, check for both field names:

```rust
// In WorkSurfaceBound handler (lines 2178-2188)
// Support both old and new field names for backward compatibility
let template_ref = payload.get("template_ref")
    .or_else(|| payload.get("procedure_template_ref"))
    .ok_or_else(|| /* error handling */)?;

let template_id = template_ref.get("id")
    .or_else(|| template_ref.get("procedure_template_id"))
    .and_then(|v| v.as_str());
```

**Affected Event Types:**
- `WorkSurfaceBound` - contains `procedure_template_ref`
- `IterationStarted` - may reference template context
- `EvidenceBundleRecorded` - contains template context fields

**Verification:**

```bash
# 1. Count legacy events BEFORE rebuild (baseline)
psql -c "SELECT COUNT(*) FROM evt.events WHERE payload::text LIKE '%procedure_template%'"
# Record this count - these are the events that must replay correctly

# 2. Count current work surfaces BEFORE rebuild (baseline)
psql -c "SELECT COUNT(*) FROM proj.work_surfaces"
# Record this count for comparison

# 3. Drop and rebuild projections from scratch
psql -c "TRUNCATE proj.work_surfaces CASCADE"
cargo run --bin sr-projection-rebuild
# OR if using the API:
# curl -X POST http://localhost:8080/api/v1/admin/rebuild-projections

# 4. Verify work surface count matches (no data loss)
psql -c "SELECT COUNT(*) FROM proj.work_surfaces"
# Must match the count from step 2

# 5. Verify template_id populated correctly
psql -c "SELECT COUNT(*) FROM proj.work_surfaces WHERE template_id IS NULL"
# Expected: 0 (all work surfaces should have template_id)

# 6. Spot-check a legacy work surface
psql -c "SELECT work_surface_id, template_id, template_hash FROM proj.work_surfaces LIMIT 3"
# Verify template_id and template_hash are populated correctly

# 7. Run adapter tests
cargo test --package sr-adapters
# All tests must pass
```

**Full Projection Rebuild Test Script:**

Create `scripts/test_projection_rebuild.sh` for repeatable testing:

```bash
#!/bin/bash
set -e

echo "=== A5.5 Projection Rebuild Verification ==="

# Baseline counts
LEGACY_EVENTS=$(psql -t -c "SELECT COUNT(*) FROM evt.events WHERE payload::text LIKE '%procedure_template%'")
WS_COUNT=$(psql -t -c "SELECT COUNT(*) FROM proj.work_surfaces")

echo "Legacy events with procedure_template: $LEGACY_EVENTS"
echo "Work surfaces before rebuild: $WS_COUNT"

# Rebuild
echo "Truncating projections..."
psql -c "TRUNCATE proj.work_surfaces CASCADE"

echo "Rebuilding from event store..."
cargo run --bin sr-projection-rebuild

# Verify
WS_COUNT_AFTER=$(psql -t -c "SELECT COUNT(*) FROM proj.work_surfaces")
NULL_TEMPLATE=$(psql -t -c "SELECT COUNT(*) FROM proj.work_surfaces WHERE template_id IS NULL")

echo "Work surfaces after rebuild: $WS_COUNT_AFTER"
echo "Work surfaces with NULL template_id: $NULL_TEMPLATE"

if [ "$WS_COUNT" != "$WS_COUNT_AFTER" ]; then
    echo "FAIL: Work surface count mismatch ($WS_COUNT vs $WS_COUNT_AFTER)"
    exit 1
fi

if [ "$NULL_TEMPLATE" != "0" ]; then
    echo "FAIL: Found $NULL_TEMPLATE work surfaces with NULL template_id"
    exit 1
fi

echo "=== PASS: Projection rebuild successful ==="
```

**Note:** This dual-read support should remain in place until all legacy events have been processed at least once. It can be removed in a future cleanup task after confirming no production replay issues.

**Dependencies:** A4, A5 (handlers must be updated to use new names for new events)

---

### Task A6: Database Migration

**New File:** `migrations/013_template_renames.sql`

```sql
-- Migration: 013_template_renames.sql
-- Purpose: Rename procedure_template columns to template per SR-PLAN-MVP1

-- ============================================================================
-- Rename columns in proj.work_surfaces
-- ============================================================================

ALTER TABLE proj.work_surfaces
    RENAME COLUMN procedure_template_id TO template_id;

ALTER TABLE proj.work_surfaces
    RENAME COLUMN procedure_template_hash TO template_hash;

-- ============================================================================
-- Update constraint names (PostgreSQL renames automatically with column)
-- ============================================================================

-- The CHECK constraint on template_hash format is named template_hash_format
-- It will continue to work after column rename

-- ============================================================================
-- Rename index
-- ============================================================================

ALTER INDEX idx_work_surfaces_template
    RENAME TO idx_work_surfaces_template_id;

-- ============================================================================
-- Update column comments
-- ============================================================================

COMMENT ON COLUMN proj.work_surfaces.template_id IS
    'Reference to the bound Template (was procedure_template_id)';

COMMENT ON COLUMN proj.work_surfaces.template_hash IS
    'Content hash of Template at binding time (immutable, was procedure_template_hash)';

-- ============================================================================
-- Note: No data transformation needed - values remain the same
-- Format still: proc:<NAME> for template_id, sha256:<hex> for template_hash
-- ============================================================================
```

**Verification:**
```bash
# Apply migration
sqlx migrate run

# Verify columns renamed
psql -c "\d proj.work_surfaces" | grep -E "template_id|template_hash"

# Verify no old column names
psql -c "\d proj.work_surfaces" | grep procedure_template
# Expected: 0 results
```

**Dependencies:** A4, A5 (code must handle both old and new column names during transition)

**Risk Mitigation:**
- Test on copy of production data first
- Ensure adapters/handlers can work with renamed columns
- Migration is idempotent (can re-run safely)

**Rollback Strategy:**

If migration fails or causes issues, create `migrations/013_template_renames_down.sql`:

```sql
-- Rollback: 013_template_renames_down.sql
-- Purpose: Revert template column renames if needed

ALTER TABLE proj.work_surfaces
    RENAME COLUMN template_id TO procedure_template_id;

ALTER TABLE proj.work_surfaces
    RENAME COLUMN template_hash TO procedure_template_hash;

ALTER INDEX idx_work_surfaces_template_id
    RENAME TO idx_work_surfaces_template;

COMMENT ON COLUMN proj.work_surfaces.procedure_template_id IS
    'Reference to the bound Procedure Template';

COMMENT ON COLUMN proj.work_surfaces.procedure_template_hash IS
    'Content hash of Procedure Template at binding time (immutable)';
```

**Note:** Rollback requires reverting Rust code changes (A4, A5) to use old field names. The dual-read pattern in A5.5 provides a grace period where both names work, reducing rollback urgency.

---

### Task A7: UI Type/Component Updates

**Files with procedure_template references (13 files):**

| File | Occurrences | Key Changes |
|------|-------------|-------------|
| `ui/src/pages/WorkSurfaceDetail.tsx` | Multiple | API response field names |
| `ui/src/pages/WorkSurfaceCompose.tsx` | Multiple | Request body field names |
| `ui/src/pages/WorkSurfaces.tsx` | Multiple | List/filter fields |
| `ui/src/pages/LoopDetail.tsx` | Few | Display fields |
| `ui/src/pages/Templates.tsx` | Multiple | Type definitions |
| `ui/src/pages/PromptLoop.tsx` | Few | Request fields |
| `ui/src/pages/ProtocolDetail.tsx` | Few | Display fields |
| `ui/src/pages/Protocols.tsx` | Few | List fields |
| `ui/src/pages/References.tsx` | Few | API calls |
| `ui/src/pages/Settings.tsx` | Few | Config display |
| `ui/src/components/LoopCreateModal.tsx` | Few | Form fields |
| `ui/src/components/InputsEditor.tsx` | Few | Type hints |
| `ui/src/hooks/useLoops.ts` | Few | Type definitions |

**Files with gate_rule references (4 files):**

| File | Changes |
|------|---------|
| `ui/src/pages/ProtocolDetail.tsx` | Remove gate_rule display |
| `ui/src/pages/Protocols.tsx` | Remove gate_rule column |
| `ui/src/components/StageProgress.tsx` | Remove gate_rule logic |
| `ui/src/components/StageProgress.module.css` | Remove related styles |

**TypeScript Interface Updates:**

**Note:** This codebase defines TypeScript interfaces inline within components rather than in a centralized types file. Update interfaces in each file where they are declared.

**Key Interface Locations:**

| Interface | File | Line (approx) |
|-----------|------|---------------|
| `WorkSurfaceSummary` | `ui/src/pages/WorkSurfaces.tsx` | 16-26 |
| `WorkSurfaceDetail` | `ui/src/pages/WorkSurfaceDetail.tsx` | ~20-35 |
| `LoopWithWorkSurface` | `ui/src/hooks/useLoops.ts` | ~15-30 |
| `CreateWorkSurfaceRequest` | `ui/src/pages/WorkSurfaceCompose.tsx` | ~25-35 |

**Example Change (WorkSurfaces.tsx lines 16-26):**

```typescript
// Before
interface WorkSurfaceSummary {
  work_surface_id: string;
  work_unit_id: string;
  intake_id: string;
  intake_title: string | null;
  procedure_template_id: string;      // ← rename
  procedure_template_name: string | null;  // ← rename
  current_stage_id: string;
  status: 'active' | 'completed' | 'archived';
  bound_at: string;
}

// After
interface WorkSurfaceSummary {
  work_surface_id: string;
  work_unit_id: string;
  intake_id: string;
  intake_title: string | null;
  template_id: string;           // ← renamed
  template_name: string | null;  // ← renamed
  current_stage_id: string;
  status: 'active' | 'completed' | 'archived';
  bound_at: string;
}
```

**Additional UI Code Updates:**

After renaming interface fields, update all usages within each file:
- Object property accesses (e.g., `ws.procedure_template_id` → `ws.template_id`)
- JSX interpolations displaying these fields
- Filter/sort functions referencing these properties

**Verification:**
```bash
cd ui
npm run build
npm run typecheck
grep -rn "procedure_template" src/
grep -rn "gate_rule" src/
# Expected: 0 results (except comments)
```

**Dependencies:** A4 (API must return new field names)

---

### Task A8: API Endpoint Renames

**File:** `crates/sr-api/src/main.rs`

**Current Routes (lines 354-368, 505-506, 429-430, 441-442):**

| Current | New | Handler |
|---------|-----|---------|
| `/api/v1/references/procedure-templates` | `/api/v1/references/templates` | `references::list_templates` |
| `/api/v1/procedure-instances/:work_surface_id` | `/api/v1/template-instances/:work_surface_id` | `work_surfaces::get_template_instance` |
| `/api/v1/work-surfaces/compatible-templates` | (unchanged) | (unchanged) |

**Note:** `/api/v1/templates` routes (lines 354-368) already use "templates" naming - no change needed.

**Handler Function Renames:**
- `work_surfaces::get_procedure_instance` → `work_surfaces::get_template_instance`

**Verification:**
```bash
cargo check --package sr-api
curl http://localhost:8080/api/v1/references/templates
# Should return template list
```

**Dependencies:** A4 (handlers must be updated first)

---

---

## Part A Completion Gate

**Before starting Part B, verify ALL of the following:**

```bash
# 1. All Rust code compiles
cargo check --workspace

# 2. All tests pass
cargo test --workspace

# 3. No old terminology in Rust code
grep -r "ProcedureTemplate" crates/ | grep -v "// " | wc -l
# Expected: 0

grep -r "procedure_template" crates/ | grep -v "// " | wc -l
# Expected: 0

grep -r "GateRule" crates/ | wc -l
# Expected: 0

# 4. Database migration applied
psql -c "\d proj.work_surfaces" | grep template_id
# Expected: shows template_id column

psql -c "\d proj.work_surfaces" | grep procedure_template
# Expected: 0 results

# 5. UI compiles
cd ui && npm run build && npm run typecheck
# Expected: no errors

# 6. SR-PROCEDURE-KIT.md deleted
ls docs/platform/SR-PROCEDURE-KIT.md 2>&1
# Expected: "No such file or directory"

# 7. API responds with new field names
curl -s http://localhost:8080/api/v1/work-surfaces | jq '.[0] | keys' | grep template
# Expected: shows template_id, template_hash (not procedure_template_*)
```

**If any check fails, return to the relevant Part A task before proceeding.**

---

## Part B: Fresh UI Build

> **Prerequisite:** Complete Part A Completion Gate above. The UI depends on the clarified Template model.

### Context

The target user experience:
1. **Create** — Define work (Work Surface = Intake + Template)
2. **Work** — Agent runs iterations autonomously
3. **Checkpoint** — User sees candidate + evidence, makes judgment
4. **Done** — Approval when complete

The user should **never** manually select evidence bundles from a hash list.

---

### Task B1: Database Migration — Add work_surface_id to Evidence Bundles

**Context:** Evidence bundles need to be linked to work surfaces for auto-loading.

**Current State:**
- `proj.evidence_bundles` table (migrations/004_evidence.sql) does NOT have `work_surface_id`
- `EvidenceManifest` Rust struct DOES have `work_surface_id: Option<String>` (evidence.rs line ~84)

**New File:** `migrations/014_evidence_work_surface_linking.sql`

```sql
-- Migration: 014_evidence_work_surface_linking.sql
-- Purpose: Link evidence bundles to work surfaces for auto-loading per SR-PLAN-MVP1 Part B

-- ============================================================================
-- Add work_surface_id column to evidence_bundles
-- ============================================================================

ALTER TABLE proj.evidence_bundles
    ADD COLUMN work_surface_id TEXT;

-- ============================================================================
-- Add template context columns (already in Rust struct, not in DB)
-- ============================================================================

ALTER TABLE proj.evidence_bundles
    ADD COLUMN template_id TEXT;

ALTER TABLE proj.evidence_bundles
    ADD COLUMN stage_id TEXT;

-- ============================================================================
-- Create index for work surface queries
-- ============================================================================

CREATE INDEX idx_evidence_bundles_work_surface
    ON proj.evidence_bundles(work_surface_id)
    WHERE work_surface_id IS NOT NULL;

CREATE INDEX idx_evidence_bundles_template_stage
    ON proj.evidence_bundles(template_id, stage_id)
    WHERE template_id IS NOT NULL;

-- ============================================================================
-- Add column comments
-- ============================================================================

COMMENT ON COLUMN proj.evidence_bundles.work_surface_id IS
    'Work Surface this evidence was recorded under (nullable for legacy data)';

COMMENT ON COLUMN proj.evidence_bundles.template_id IS
    'Template ID this evidence is associated with';

COMMENT ON COLUMN proj.evidence_bundles.stage_id IS
    'Stage ID within the template';
```

**Verification:**
```bash
sqlx migrate run
psql -c "\d proj.evidence_bundles" | grep -E "work_surface_id|template_id|stage_id"
# Should show all three columns
```

**Dependencies:** Part A complete (template terminology)

---

### Task B2: Backend — Update Evidence Projection Handler

**File:** `crates/sr-adapters/src/projections.rs`

**Location:** Find `EvidenceBundleRecorded` event handler (search for `"EvidenceBundleRecorded"`)

**Changes:**

1. Extract work surface context from event payload:
```rust
// In handle_evidence_bundle_recorded or equivalent
let work_surface_id = payload.get("work_surface_id")
    .and_then(|v| v.as_str())
    .map(|s| s.to_string());

let template_id = payload.get("template_id")
    .and_then(|v| v.as_str())
    .map(|s| s.to_string());

let stage_id = payload.get("stage_id")
    .and_then(|v| v.as_str())
    .map(|s| s.to_string());
```

2. Update INSERT query to include new columns:
```sql
INSERT INTO proj.evidence_bundles (
    content_hash, bundle_id, run_id, candidate_id, iteration_id,
    oracle_suite_id, oracle_suite_hash, verdict, artifact_count,
    run_completed_at, recorded_by_kind, recorded_by_id, recorded_at,
    last_event_id, last_global_seq,
    work_surface_id, template_id, stage_id  -- NEW
) VALUES (...)
```

3. Add bindings for new columns:
```rust
.bind(work_surface_id)
.bind(template_id)
.bind(stage_id)
```

**Verification:**
```bash
cargo test --package sr-adapters
# Create evidence bundle with work_surface_id, verify it's stored
```

**Dependencies:** B1 (columns must exist)

---

### Task B3: Backend — Add Evidence-by-WorkSurface Endpoint

**File:** `crates/sr-api/src/handlers/evidence.rs`

**New Endpoint:** `GET /api/v1/work-surfaces/{work_surface_id}/evidence`

**Implementation:**

```rust
/// List evidence bundles for a work surface
/// GET /api/v1/work-surfaces/:work_surface_id/evidence
pub async fn list_evidence_by_work_surface(
    Path(work_surface_id): Path<String>,
    Query(params): Query<ListEvidenceQuery>,
    State(state): State<AppState>,
) -> Result<Json<ListEvidenceResponse>, ApiError> {
    let query = r#"
        SELECT
            content_hash, bundle_id, run_id, candidate_id, iteration_id,
            oracle_suite_id, oracle_suite_hash, verdict, artifact_count,
            run_completed_at, recorded_by_kind, recorded_by_id, recorded_at,
            template_id, stage_id
        FROM proj.evidence_bundles
        WHERE work_surface_id = $1
        ORDER BY run_completed_at DESC
        LIMIT $2 OFFSET $3
    "#;

    // Execute query and return results
    // ...
}
```

**Response Type:**

```rust
#[derive(Debug, Serialize)]
pub struct WorkSurfaceEvidenceBundle {
    pub content_hash: String,
    pub bundle_id: String,
    pub run_id: String,
    pub candidate_id: String,
    pub iteration_id: Option<String>,
    pub oracle_suite_id: String,
    pub verdict: String,
    pub run_completed_at: String,
    pub template_id: Option<String>,
    pub stage_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListWorkSurfaceEvidenceResponse {
    pub work_surface_id: String,
    pub evidence: Vec<WorkSurfaceEvidenceBundle>,
    pub total: i64,
}
```

**Route Registration:**

File: `crates/sr-api/src/main.rs`

Add to work_surface_routes (around line 440):
```rust
.route(
    "/api/v1/work-surfaces/:work_surface_id/evidence",
    get(work_surfaces::list_evidence_by_work_surface)
)
```

**Verification:**
```bash
cargo test --package sr-api
curl http://localhost:8080/api/v1/work-surfaces/ws_01ABC/evidence
# Should return evidence list
```

**Dependencies:** B1, B2 (data must be stored correctly)

---

### Task B4: UI — Create WorkScreen

**New File:** `ui/src/screens/WorkScreen.tsx`

**Route:** `/work/:work_surface_id` (add to routes.tsx)

**Layout (two-column per SR-README):**

```
┌─────────────────────────────────────┬──────────────────────────┐
│ WORK CONTEXT                        │ CURRENT STATE            │
│                                     │                          │
│ Intake: [name]                      │ Loop: [status pill]      │
│ Template: [template name]           │ Budget: [progress bar]   │
│                                     │ Iterations: [count]      │
│ ─────────────────────────────────── │                          │
│                                     │ Stop Trigger: [if any]   │
│ CANDIDATE                           │ [Decision UI if stopped] │
│ [The actual work output to judge]   │                          │
│                                     ├──────────────────────────┤
│ EVIDENCE                            │ ACTIONS                  │
│ [Oracle results supporting judgment]│                          │
│   - Oracle 1: PASS/FAIL             │ [Approve]                │
│   - Oracle 2: PASS/FAIL             │ [Reject]                 │
│   - Artifacts link                  │ [Waive with reason]      │
│                                     │ [Request revision]       │
│ [If no evidence: "Awaiting work"]   │                          │
└─────────────────────────────────────┴──────────────────────────┘
```

**Data Flow:**

```typescript
// 1. Fetch work surface details
const workSurface = await fetch(`/api/v1/work-surfaces/${id}`);

// 2. Fetch associated loop (if exists)
const loops = await fetch(`/api/v1/loops?work_surface_id=${id}`);

// 3. Fetch candidate (latest for this work surface)
const candidates = await fetch(`/api/v1/candidates?work_surface_id=${id}`);

// 4. Fetch evidence (NEW endpoint from B3)
const evidence = await fetch(`/api/v1/work-surfaces/${id}/evidence`);

// 5. Render unified view
```

**Key Props/State:**

```typescript
interface WorkScreenState {
  workSurface: WorkSurfaceResponse | null;
  loop: LoopResponse | null;
  candidate: CandidateResponse | null;
  evidence: WorkSurfaceEvidenceBundle[];
  loading: boolean;
  error: string | null;
}
```

**Verification:**
- Navigate to `/work/{work_surface_id}`
- See unified context (intake, template, candidate, evidence)
- No hash selection dropdown anywhere

**Dependencies:** B3 (evidence endpoint), Part A (template terminology)

---

### Task B5: UI — Create Supporting Components

**New Files:**

#### B5.1: `ui/src/components/LoopStateCard.tsx`

**Purpose:** Display loop status, budget, iteration count

**Props:**
```typescript
interface LoopStateCardProps {
  loop: LoopResponse | null;
  workSurfaceStatus: string;
}
```

**Display:**
- Status pill (ACTIVE, STOPPED, COMPLETED)
- Budget progress bar (uses existing BudgetProgress.tsx)
- Iteration count
- Stop trigger (if fired)

---

#### B5.2: `ui/src/components/CandidateView.tsx`

**Purpose:** Display the actual work output for judgment

**Props:**
```typescript
interface CandidateViewProps {
  candidate: CandidateResponse | null;
  loading: boolean;
}
```

**Display:**
- Candidate content (markdown rendered)
- Artifact list with links
- "Awaiting work" placeholder if no candidate

---

#### B5.3: `ui/src/components/EvidenceSummary.tsx`

**Purpose:** Auto-loaded evidence display (NO hash selector)

**Props:**
```typescript
interface EvidenceSummaryProps {
  evidence: WorkSurfaceEvidenceBundle[];
  loading: boolean;
}
```

**Display:**
- List of oracle results with PASS/FAIL badges
- Grouped by stage if multiple stages
- Expandable details for each oracle
- Link to full evidence bundle

---

#### B5.4: `ui/src/components/JudgmentActions.tsx`

**Purpose:** Approve/Reject/Waive action buttons

**Props:**
```typescript
interface JudgmentActionsProps {
  workSurfaceId: string;
  stageId: string;
  canApprove: boolean;  // All oracles pass
  onAction: (action: 'approve' | 'reject' | 'waive', reason?: string) => void;
}
```

**Display:**
- "Approve" button (prominent if all oracles pass)
- "Reject" button
- "Waive with reason" button (opens modal for reason)
- Disabled state when no evidence

**Verification:**
```bash
cd ui
npm run build
npm run typecheck
```

**Dependencies:** B4 (WorkScreen uses these components)

---

### Task B6: UI — Update Routing

**File:** `ui/src/routes.tsx`

**Add route (around line 97):**

```typescript
// Work Screen (unified view)
{ path: "/work/:workSurfaceId", element: <WorkScreen /> },
```

**File:** `ui/src/layout/Sidebar.tsx`

**Add navigation item:**

```typescript
// In navigation items array
{ path: "/work-surfaces", label: "Work Surfaces", icon: FolderIcon },
// Note: Individual work screens accessed via /work/:id from list
```

**Verification:**
- Click work surface in list → navigate to `/work/{id}`
- WorkScreen renders

**Dependencies:** B4, B5 (components must exist)

---

### Task B7: UI — Remove Legacy Components (Deferred)

**Files to Eventually Remove:**

| File | Lines | Reason |
|------|-------|--------|
| `ui/src/components/EvidenceBundleSelector.tsx` | 178 | Replaced by auto-linking |
| `ui/src/components/StageCompletionForm.tsx` | 428 | Replaced by JudgmentActions |
| `ui/src/screens/PromptLoopScreen.tsx` | 81 | Replaced by WorkScreen |

**Note:** Keep these files until WorkScreen is fully functional and tested. Removal is a cleanup task after Part B is proven.

**Verification (when removing):**
```bash
grep -rn "EvidenceBundleSelector" ui/src/
grep -rn "StageCompletionForm" ui/src/
grep -rn "PromptLoopScreen" ui/src/
# Ensure no remaining imports before deletion
```

**Dependencies:** B4, B5, B6 proven working

---

## Execution Order

```
┌─────────────────────────────────────────────────────────────────┐
│ PART A: Nomenclature Refactor                                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  A1 (docs)                                                      │
│      │                                                          │
│      ▼                                                          │
│  A2 (work_surface.rs) ──┬──► A4 (API handlers)                 │
│      │                  │         │                             │
│      ▼                  │         ▼                             │
│  A3 (templates.rs) ─────┴──► A5 (adapters)                     │
│                                   │                             │
│                                   ▼                             │
│                              A5.5 (backward compat)             │
│                                   │                             │
│                                   ▼                             │
│                              A6 (migration)                     │
│                                   │                             │
│                                   ▼                             │
│                              A7 (UI types)                      │
│                                   │                             │
│                                   ▼                             │
│                              A8 (API routes)                    │
│                                                                 │
│                                   │                             │
│                                   ▼                             │
│                         ┌─────────────────┐                     │
│                         │ PART A GATE     │                     │
│                         │ (verify all     │                     │
│                         │  checks pass)   │                     │
│                         └────────┬────────┘                     │
│                                  │                              │
├──────────────────────────────────┼──────────────────────────────┤
│ PART B: Fresh UI Build           │                              │
├──────────────────────────────────┼──────────────────────────────┤
│                                  ▼                              │
│  B1 (migration) ──► B2 (projection) ──► B3 (endpoint)          │
│                                              │                  │
│                                              ▼                  │
│                          B4 (WorkScreen) ◄───┘                  │
│                               │                                 │
│                               ▼                                 │
│                          B5 (components)                        │
│                               │                                 │
│                               ▼                                 │
│                          B6 (routing)                           │
│                               │                                 │
│                               ▼                                 │
│                          B7 (cleanup - deferred)                │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Parallelization Opportunities:**
- A2 + A3 can run together (both domain, no cross-dependency)
- A4 + A5 can run together after A2/A3
- B4 + B5 can run together after B3

---

## Risk Areas

| Risk | Impact | Mitigation |
|------|--------|------------|
| Breaking API contracts | High | Run full API test suite after each handler update; version API if needed |
| Database migration on production data | High | Test migration on copy of production data first; rollback SQL provided in A6 |
| TypeScript type drift | Medium | Run `npm run typecheck` frequently; update types atomically with API changes |
| Event replay breaking | High | Task A5.5 implements dual-read pattern for backward compatibility; test projection rebuild |
| Missing occurrences | Medium | Use comprehensive grep searches; verify 0 hits before marking complete |
| Serialization format changes | Medium | Ensure serde attributes produce same JSON (except field names) |
| GateRule removal side effects | Medium | Search for all GateRule pattern matches; verify no runtime dependencies |
| Legacy events in event store | High | A5.5 dual-read pattern handles both old and new field names during replay |

---

## Test Strategy

### Part A Verification Checklist

| Step | Command | Expected |
|------|---------|----------|
| 1 | `cargo check` | Compiles without errors |
| 2 | `cargo test --workspace` | All tests pass |
| 3 | `grep -r "ProcedureTemplate" crates/` | 0 results (except comments) |
| 4 | `grep -r "procedure_template" crates/` | 0 results (except comments, dual-read fallbacks) |
| 5 | `grep -r "GateRule" crates/` | 0 results |
| 6 | `grep -r "gate_rule" crates/` | 0 results |
| 7 | `cd ui && npm run build && npm run typecheck` | Compiles without errors |
| 8 | `grep -r "procedure_template" ui/src/` | 0 results |
| 9 | `ls docs/platform/SR-PROCEDURE-KIT.md` | File not found (deleted) |
| 10 | `grep -r "SR-PROCEDURE-KIT" docs/` | 0 results (except this plan) |
| 11 | `./scripts/test_projection_rebuild.sh` | Projections rebuild with correct data |

**Note on Step 4:** The dual-read fallback pattern in A5.5 intentionally retains `procedure_template_ref` as a string literal for backward compatibility with legacy events. This is expected and correct.

**Note on Step 11:** This is a critical verification. The script truncates projections and rebuilds from the event store, verifying that legacy events with `procedure_template_ref` are correctly processed into the new `template_id` column. Run this on a test database first, never directly on production.

### Part A Integration Test

1. Create a work surface via API with `template_id` field
2. Verify response contains `template_id` (not `procedure_template_id`)
3. Verify database stores in `template_id` column
4. Verify UI displays correctly

### Part B Verification Checklist

| Step | Command | Expected |
|------|---------|----------|
| 1 | `sqlx migrate run` | Migration 014 applies |
| 2 | `psql -c "\d proj.evidence_bundles"` | work_surface_id column exists |
| 3 | `cargo test --package sr-adapters` | Projection tests pass |
| 4 | `curl /api/v1/work-surfaces/{id}/evidence` | Returns evidence list |
| 5 | `cd ui && npm run build` | Compiles with new components |
| 6 | Navigate to `/work/{id}` | WorkScreen renders |

### Part B End-to-End Test

1. Create work surface with intake and template
2. Start loop via `/work-surfaces/{id}/start`
3. Run iteration, produce evidence with `work_surface_id`
4. Navigate to WorkScreen at `/work/{id}`
5. Verify:
   - Intake and template info display
   - Candidate displays (if produced)
   - Evidence auto-loads (no hash selection)
   - Approve/Reject actions visible
6. Click Approve
7. Verify work surface status updates

---

## Success Criteria

### Part A Complete When:

- [ ] All `grep` searches for old terminology return 0 results
- [ ] `SR-PROCEDURE-KIT.md` deleted
- [ ] All tests pass (`cargo test --workspace`)
- [ ] UI compiles and typechecks
- [ ] API responds with new field names (`template_id`, `template_hash`)
- [ ] Database columns renamed (`template_id`, `template_hash`)

### Part B Complete When:

- [ ] WorkScreen renders at `/work/{id}`
- [ ] Evidence loads automatically (no hash selection)
- [ ] Candidate + evidence displayed together
- [ ] Approve/Reject/Waive actions functional
- [ ] Loop state (budget, stop triggers) visible
- [ ] End-to-end test passes

---

## Appendix: File Reference

### Domain Files (Part A)

| File | Lines | Primary Changes |
|------|-------|-----------------|
| `crates/sr-domain/src/work_surface.rs` | 1970 | Type renames, GateRule removal |
| `crates/sr-domain/src/procedure_templates.rs` | 620 | Rename to templates.rs |
| `crates/sr-domain/src/lib.rs` | ~50 | Module declaration update |

### API Handler Files (Part A)

| File | Lines | Primary Changes |
|------|-------|-----------------|
| `crates/sr-api/src/handlers/templates.rs` | 1782 | Type key, field names |
| `crates/sr-api/src/handlers/work_surfaces.rs` | 2118 | Request/response types, SQL |
| `crates/sr-api/src/handlers/evidence.rs` | 1072 | Field names |
| `crates/sr-api/src/handlers/iterations.rs` | 605 | Column names |
| `crates/sr-api/src/handlers/verification.rs` | ~100 | Field names |
| `crates/sr-api/src/handlers/references.rs` | 998 | Function rename |
| `crates/sr-api/src/handlers/prompt_loop.rs` | 1321 | Field names |
| `crates/sr-api/src/main.rs` | ~600 | Route renames |

### Adapter Files (Part A)

| File | Lines | Primary Changes |
|------|-------|-----------------|
| `crates/sr-adapters/src/projections.rs` | ~2500 | Column names, payload extraction, dual-read fallback (A5.5) |
| `crates/sr-adapters/src/evidence.rs` | ~900 | Field names, builder methods |
| `crates/sr-adapters/src/semantic_worker.rs` | ~1500 | Field names |
| `crates/sr-adapters/src/semantic_suite.rs` | ~600 | Field names |
| `crates/sr-adapters/src/integrity.rs` | ~1200 | Field names |

### Migration Files

| File | Purpose |
|------|---------|
| `migrations/013_template_renames.sql` | Part A: Column renames |
| `migrations/014_evidence_work_surface_linking.sql` | Part B: Evidence linking |

### UI Files (Part A)

13 files with procedure_template references, 4 files with gate_rule references (see Task A7)

### UI Files (Part B - New)

| File | Purpose |
|------|---------|
| `ui/src/screens/WorkScreen.tsx` | Main unified work view |
| `ui/src/components/LoopStateCard.tsx` | Loop status display |
| `ui/src/components/CandidateView.tsx` | Candidate display |
| `ui/src/components/EvidenceSummary.tsx` | Auto-loaded evidence |
| `ui/src/components/JudgmentActions.tsx` | Approve/Reject/Waive |

### Documentation Files (Part A)

| File | Changes |
|------|---------|
| `docs/platform/SR-TEMPLATES.md` | Merge content, update terminology |
| `docs/platform/SR-PROCEDURE-KIT.md` | DELETE |
| `docs/platform/SR-WORK-SURFACE.md` | Remove ref |
| `docs/platform/SR-SEMANTIC-ORACLE-SPEC.md` | Remove ref |
| `docs/platform/SR-EVENT-MANAGER.md` | Remove ref |
| `docs/platform/SR-TYPES.md` | Update references |
| `docs/platform/SR-BRANCH-0-ACCEPTANCE.md` | Update reference |
| `docs/platform/EXECUTION-PLAN.md` | Update reference |

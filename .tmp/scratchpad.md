SR-SEMANTIC-ORACLE-SPEC defines the **platform-visible interface** for semantic oracles:

---

## 3. Inconsistencies Identified

| # | Issue | Location | Nature | Severity |
|---|-------|----------|--------|----------|
| **A** | **doc_kind not registered in SR-TYPES** | SR-SEMANTIC-ORACLE-SPEC frontmatter line 3 | `governance.semantic_oracle_spec` is not in SR-TYPES §4.1-4.5 | **High** |
| **B** | **Circular dependency** | SR-SEMANTIC-ORACLE-SPEC depends_on SR-PROCEDURE-KIT; SR-PROCEDURE-KIT depends_on SR-SEMANTIC-ORACLE-SPEC | Circular refs between documents | **Medium** |
| **C** | **`sr.semantic_eval.v1` schema not registered** | §4 line 88 | New schema identifier for semantic evaluation results; not in SR-TYPES | **Low** |

### Analysis of Issue A (doc_kind not registered)

SR-TYPES §4.1 Platform Specification Types now includes:
- `governance.work_surface_spec`
- `governance.procedure_kit`

But `governance.semantic_oracle_spec` is not listed. This document needs a registered type.

### Analysis of Issue B (Circular dependency)

The dependency graph shows:
```
SR-PROCEDURE-KIT depends_on SR-SEMANTIC-ORACLE-SPEC
SR-SEMANTIC-ORACLE-SPEC depends_on SR-PROCEDURE-KIT
```

This creates a mutual dependency. While not strictly forbidden, it suggests these documents should either:
1. Be merged into one document
2. Have one designated as the "lower layer" (recommended: SR-SEMANTIC-ORACLE-SPEC should NOT depend on SR-PROCEDURE-KIT since it defines the platform-visible interface for oracles, which procedure templates then consume)
3. Have the circular dependency explicitly documented as intentional

**Recommendation:** Remove SR-PROCEDURE-KIT from SR-SEMANTIC-ORACLE-SPEC's depends_on list. The semantic oracle spec defines the oracle interface; procedure templates consume that interface. The oracle spec doesn't need to know about specific procedure template definitions.

### Analysis of Issue C (Schema not registered)

The schema identifier `sr.semantic_eval.v1` (line 88) is a data schema for semantic evaluation results. SR-TYPES §4.5 Configuration Types includes `config.semantic_set` and `config.semantic_profile`, but doesn't register this evaluation result schema.

Options:
1. Add to SR-TYPES as a new schema type (e.g., `schema.semantic_eval`)
2. Document it within SR-SEMANTIC-ORACLE-SPEC as a local schema definition (acceptable since it's a wire format, not a governed artifact type)

**Recommendation:** Option 2 — it's a wire format/data schema internal to semantic oracle outputs, not a governed artifact type.

---

## 4. Plan to Address Findings

| Issue | Proposed Fix | Document(s) Affected | Priority |
|-------|--------------|---------------------|----------|
| **A** | Add `governance.semantic_oracle_spec` to SR-TYPES §4.1 Platform Specification Types | SR-TYPES | **High** |
| **B** | Remove `depends_on: SR-PROCEDURE-KIT` from SR-SEMANTIC-ORACLE-SPEC frontmatter (oracle interface is consumed by procedure templates, not dependent on them) | SR-SEMANTIC-ORACLE-SPEC | **Medium** |
| **C** | Add note that `sr.semantic_eval.v1` is a wire format schema defined locally, not a governed artifact type | SR-SEMANTIC-ORACLE-SPEC | **Low** |

**Recommended implementation order:**
1. Fix A (High) — add governance.semantic_oracle_spec to SR-TYPES
2. Fix B (Medium) — remove circular dependency
3. Fix C (Low) — clarify schema status

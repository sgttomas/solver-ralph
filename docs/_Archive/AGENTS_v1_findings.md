## 1. My Understanding of the Complete Set (Updated)

**What it's about:**
SOLVER-Ralph is a governed, event-sourced platform for agentic semantic knowledge work. The document set now provides comprehensive coverage:

| Document | Role |
|----------|------|
| **SR-CHARTER** | Project purpose, scope, milestones, human authority model |
| **SR-CONTRACT** | ~50 binding invariants (C-*) the platform must enforce |
| **SR-TYPES** | Canonical type registry, vocabulary, metadata schemas |
| **SR-SPEC** | Technical implementation: events, state machines, APIs, projections |
| **SR-AGENTS** | Agent ontology, epistemology, and trust-boundary constraints |
| **README** | Navigation map and quick-start guide |

**What it will result in:**
A platform where agents perform substantive knowledge work but are **structurally prevented** from creating binding authority. SR-AGENTS completes the trust model by defining:
- Agent as an actor-kind whose outputs are *non-binding proposals*
- Authority limits as *structural impossibilities* (enforced by coercion at trust boundaries), not behavioral rules
- Work Envelopes as the unit of variation (task, context, tools, verification) without introducing agent subtypes

**How it attains this:**
SR-AGENTS maps SR-CONTRACT trust boundary invariants to concrete agent constraints:

| Trust Boundary | Agent Constraint |
|----------------|------------------|
| C-TB-1 (Human-Only Authority) | Agent cannot create approvals, decisions, waivers, freezes |
| C-TB-2 (Non-Authoritative Output) | Agent statements are proposals; cannot establish Verified/Approved/Shippable |
| C-TB-3 (Portal Crossings) | Agent cannot cross portals; only humans produce approval records |
| C-VER-1 (Evidence-Based Verification) | Agent claims cannot establish Verified; requires oracle evidence |
| C-CTX-2 (No Ghost Inputs) | Agent context must be derivable from IterationStarted.refs[] |
| C-LOOP-1 (Bounded Iteration) | Agent cannot extend budgets; requires human decision |

---

## 2. Document I Want to Read Next

**SR-DIRECTIVE**

Reasoning: SR-AGENTS explicitly defers execution policies to SR-DIRECTIVE (§0.1: "Execution policies (budgets, gates, portal routing) live in SR-DIRECTIVE"). Work Envelopes (§5) are defined by parameters that come from SR-DIRECTIVE. The build-execution precedence chain places SR-DIRECTIVE at position 5. I need to verify that SR-DIRECTIVE provides the operational parameters that SR-AGENTS references.

---

## 3. Inconsistencies Identified

| # | Issue | Location | Nature | Severity |
|---|-------|----------|--------|----------|
| **A** | **ORACLE as separate actor kind** | SR-AGENTS §1.1 lists `ORACLE` as an actor kind; SR-CONTRACT §2.2 and SR-SPEC §1.4.1 only define `HUMAN`, `AGENT`, `SYSTEM` | SR-AGENTS introduces a fourth actor kind not present in CONTRACT or SPEC. This could be intentional (oracles are a special case of SYSTEM) or an inconsistency. | **Medium** |
| **B** | **Precedence chain differences** | SR-AGENTS §0.2A includes `SR-PROCEDURE-KIT` at position 7; SR-TYPES §2.3A doesn't include it | SR-AGENTS has a document in the precedence chain that SR-TYPES omits. | **Low** |
| **C** | **SR-AGENT-WORKER-CONTRACT in build-execution precedence** | SR-AGENTS §0.2B places `SR-AGENT-WORKER-CONTRACT` at position 3; SR-TYPES §2.3B doesn't include it | Another precedence chain discrepancy between the two authoritative sources. | **Low** |
| **D** | **UTF-8 encoding artifacts** | Throughout SR-AGENTS | `â€™`, `â€'`, `â€œ` appear (curly quotes/dashes rendered incorrectly) | **Low** (cosmetic) |
| **E** | **`semantic-ralph-loop` naming not referenced** | SR-AGENTS doesn't mention the canonical `semantic-ralph-loop` term | Minor terminology drift; document focuses on agent semantics rather than loop mechanics. | **Very Low** |

### Analysis of Issue A (ORACLE actor kind)

This is the most substantive finding. Looking at the documents:

- **SR-CONTRACT §2.2** defines: HUMAN, AGENT, SYSTEM
- **SR-SPEC §1.4.1** defines `actor_kind` as: HUMAN, AGENT, SYSTEM
- **SR-AGENTS §1.1** defines: HUMAN, SYSTEM, ORACLE, AGENT

SR-AGENTS introduces ORACLE as a distinct actor kind with the note "deterministic verifiers that produce evidence (not authority)." However, in SR-SPEC, oracle execution is performed by the OracleRunner adapter (§4.4-4.5), which would emit events with `actor_kind=SYSTEM`.

This could be:
1. **Intentional specialization**: ORACLE is a semantic category within SYSTEM for clarity
2. **Inconsistency**: SR-AGENTS added a fourth actor kind without updating CONTRACT/SPEC

Given that SR-SPEC §4.4 shows oracle runs emit `EvidenceBundleRecorded` events attributed to the runner (which would be SYSTEM), I believe this is a **documentation inconsistency** that should be resolved.

---

## 4. Plan to Address Findings

| Issue | Proposed Fix | Document(s) Affected | Priority |
|-------|--------------|---------------------|----------|
| **A** | Two options: (1) Remove ORACLE from SR-AGENTS §1.1 actor kinds and clarify that oracles execute as SYSTEM actors producing evidence, or (2) Add ORACLE to SR-CONTRACT §2.2 and SR-SPEC §1.4.1 as a fourth actor kind. I recommend option (1) because SR-SPEC already models oracle execution as SYSTEM, and the key semantic point (oracles produce evidence, not authority) is preserved. Add a clarification note: "Oracles are deterministic verifiers that execute as SYSTEM actors; their outputs are Evidence Bundles, not binding authority." | SR-AGENTS | **Medium** |
| **B** | Add `SR-PROCEDURE-KIT` to SR-TYPES §2.3A precedence chain at position 7 (after SR-EVENT-MANAGER, before SR-GUIDE) to align with SR-AGENTS. | SR-TYPES | **Low** |
| **C** | Add `SR-AGENT-WORKER-CONTRACT` to SR-TYPES §2.3B precedence chain at position 3 (after SR-AGENTS, before SR-CHANGE) to align with SR-AGENTS and the existing SR-CHARTER/README references. | SR-TYPES | **Low** |
| **D** | Fix UTF-8 encoding artifacts in SR-AGENTS (cosmetic cleanup). | SR-AGENTS | **Low** |
| **E** | No action needed; SR-AGENTS focuses on agent semantics and appropriately defers loop mechanics to other documents. | None | **Very Low** |

**Recommended implementation order:**
1. Fix A (clarify ORACLE vs SYSTEM) — most important for ontological coherence
2. Fix B and C together (precedence chain alignment)
3. Fix D as cleanup

Shall I proceed with implementing these fixes?
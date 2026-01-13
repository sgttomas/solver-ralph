---
doc_id: SR-INTENT
doc_kind: workflow.intent_log
layer: build
status: draft

refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: depends_on
    to: SR-CHARTER
  - rel: informs
    to: SR-AGENTS
  - rel: informs
    to: SR-TYPES
  - rel: informs
    to: SR-CONTRACT
  - rel: informs
    to: SR-SPEC
  - rel: informs
    to: SR-DIRECTIVE
---

# SR-INTENT

## 0. What this document is (and is not)

SR-INTENT is the **non-binding intent log** for the SOLVER‑Ralph build: a compact record of *why* we made key choices, what pressures shaped them, what alternatives were rejected, and what consequences to expect.

It exists to prevent “meaning drift” during agent work by making the rationale *stable and checkable* without becoming a second ontology.

### Non-binding boundary

SR-INTENT **does not** define platform meaning.

- **Definitions / vocabulary / type keys** live in **SR-TYPES**.
- **Binding invariants / trust-boundary semantics** live in **SR-CONTRACT**.
- **Binding mechanics / algorithms / APIs / state machines** live in **SR-SPEC**.
- **Build execution policy** lives in **SR-DIRECTIVE**.

If SR-INTENT appears to define or override any of the above, it is in error and must be corrected (via SR-CHANGE).

---

## 1. Design pressures (stable drivers)

### I-001 — Foundation corruption is the critical failure mode
**Pressure:** the most damaging failure is when a false claim (verification/authorization) becomes a downstream foundation.  
**Implication:** prefer conservative blocking and escalation over permissive acceptance.

### I-002 — Cost asymmetry favors blocking
**Pressure:** iteration is cheap; wrong foundations are practically unbounded cost.  
**Implication:** optimize for preventing false commitments, even if it increases friction.

### I-003 — Actors are non-deterministic; validity must be deterministic
**Pressure:** agents and humans make non-deterministic choices; the platform must still enforce validity.  
**Implication:** deterministic validity checks + auditable state transitions.

### I-004 — Boundary coercion beats “coercion everywhere”
**Pressure:** enforcement is valuable when outputs become downstream inputs.  
**Implication:** apply coercion at *trust boundaries* and *commitment points*.

---

## 2. Key decision records (directional)

Each record states a *decision intent*, not a binding rule. Binding semantics must be encoded in SR-CONTRACT/SR-SPEC.

### D-001 — Separate generation, verification, and authority
**Decision:** treat agent outputs as proposals; verification as oracle evidence; authority as explicit human action at portals.  
**Rationale:** reduces conflation (claims ≠ evidence ≠ authorization).  
**Consequences:** more explicit portal friction; fewer silent failures.  
**Where binding lives:** SR-CONTRACT (roles/claims), SR-SPEC (enforcement surfaces).

### D-002 — “Verified” means “passed declared oracle scope,” not “true”
**Decision:** verification is bounded and honest; completeness is not assumed.  
**Rationale:** prevents “tests passed ⇒ correctness proven” collapse.  
**Consequences:** verification outputs are evidence, not authority.  
**Where binding lives:** SR-CONTRACT (status semantics), SR-SPEC (oracle integration).

### D-003 — Keep human portals as first-class primitives
**Decision:** preserve explicit human authority points for acceptability/ambiguity/high-stakes commitments.  
**Rationale:** some judgments are irreducible; make them minimal and auditable.  
**Consequences:** the system stays operable even with imperfect oracles.  
**Where binding lives:** SR-CONTRACT/SR-SPEC.

### D-004 — Use hexagonal architecture as the enforcement boundary
**Decision:** enforce invariants in the domain core; express trust boundaries as ports/adapters.  
**Rationale:** reduces coupling between governance and infrastructure; supports verification and audit.  
**Consequences:** adapters stay replaceable; enforcement stays consistent.  
**Where binding lives:** SR-SPEC (ports/adapters), SR-CONTRACT (invariants).

### D-005 — Prefer fresh-context, bounded loops for agent work
**Decision:** run work as bounded loops with fresh context to reduce compounding drift.  
**Rationale:** long-context “overbaking” compounds errors; bounded loops are auditable.  
**Consequences:** more explicit handoffs; easier reproducibility.  
**Where binding lives:** SR-SPEC (loop lifecycle + mechanics), SR-CONTRACT (claim semantics).

### D-006 — Event-sourced state for auditability
**Decision:** record immutable events; derive state deterministically from history.  
**Rationale:** makes “what happened” reconstructible and defensible.  
**Consequences:** replayability; stronger audit trail; more explicit modeling work.  
**Where binding lives:** SR-SPEC.

### D-007 — Prefer conservative escalation when routing is ambiguous
**Decision:** if a question cannot be cleanly routed to the right governing document, escalate to Human Authority.  
**Rationale:** ambiguity is a high-risk precursor to drift.  
**Consequences:** occasional latency; improved long-term clarity.  
**Where binding lives:** SR-TYPES routing rule / SR-CHANGE process.

---

## 3. Rejected alternatives (directional)

### R-001 — Prompt-only governance
Rejected because it cannot be treated as deterministic enforcement or a trust boundary.

### R-002 — Agent self-approval / agent authority claims
Rejected because authority must remain a human trust-boundary action.

### R-003 — Long-running, monolithic agent sessions
Rejected because drift compounds; replay and audit degrade.

### R-004 — “Coercion everywhere”
Rejected because it strangles exploration without proportionate reduction in foundation corruption.

---

## 4. Assumptions & re-evaluation triggers

These are *watch items*. If a trigger is observed, create an explicit record (Decision/Deviation/Deferral) and route through SR-CHANGE.

- **A-001:** “Executable oracles cover most intended work.”  
  **Trigger:** frequent ORACLE-GAP escalations for core requirements.

- **A-002:** “Human portals are available and meaningful.”  
  **Trigger:** approvals become rubber-stamps or review becomes impracticable.

- **A-003:** “Agents remain non-authoritative in practice.”  
  **Trigger:** teams begin treating agent statements as evidence/authority.

- **A-004:** “State transition rules remain enforceable and unambiguous.”  
  **Trigger:** repeated disputes about validity or bypass behavior.

- **A-005:** “Work can be decomposed into loop-completable units.”  
  **Trigger:** repeated loop failure without convergence within budgets.

---

## 5. How to use SR-INTENT during the build

- Use SR-INTENT **only** to answer “why did we choose this?” and “what tradeoff did we accept?”
- If you need a definition, a rule, or an enforcement mechanism:
  - go to SR-TYPES / SR-CONTRACT / SR-SPEC / SR-DIRECTIVE (per routing rule).
- If SR-INTENT suggests a new invariant/mechanic: treat it as a **proposal** and route it via SR-CHANGE into the correct document.

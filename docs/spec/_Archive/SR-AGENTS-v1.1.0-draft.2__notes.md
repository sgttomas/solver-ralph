# SR-AGENTS v1.1.0-draft.2 — Revision Notes (2026-01-11)

This revision implements requested changes:

- Deletes all subtype taxonomy (no agent subtypes or “subagents”).
- Introduces **Agent Envelope** as a normative concept and the correct axis of variability.
- Tightens coherence language under **SR‑ETT membranes** and **hexagonal architecture**:
  - agents are adapters; coercion belongs to ports/membranes,
  - agent identity never implies authority,
  - envelopes may vary context/tools/outputs/budgets but must not grant authority or weaken integrity.

No new portals, authorities, or binding semantics are introduced.

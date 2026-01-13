---
doc_id: SR-EXCEPTIONS
doc_kind: governance.exceptions
layer: build
status: draft
refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: informs
    to: SR-AGENTS
  - rel: informs
    to: SR-DIRECTIVE
---

# SR-EXCEPTIONS

This document is the governed exception ledger for the build phase.

During the build phase, the **Human Authority** for exceptions is the project owner (Ryan). Agents must not proceed with a deviation from governing documents or work instructions unless an exception entry exists here and is explicitly cited.

## Rules

- **When required:** Any deviation from SR-CONTRACT, SR-SPEC, SR-TYPES, SR-GUIDE, SR-AGENTS, SR-CHANGE, SR-DIRECTIVE, or SR-PLAN requires a recorded exception.
- **How to cite:** Agents must cite the exception ID (e.g., `EX-0007`) in the candidate record, work-unit notes, or PR description.
- **Closure condition:** Every exception must include an objective closure condition (e.g., “until PR-123 merges”).
- **Scope:** Exceptions must be as narrow as possible.

## Exception Entries

### EX-0001 — <short title>
- **Applies to:** <SR-DOC-ID> §<section> / rule: <identifier>
- **Deviation:** <what is being deviated from>
- **Justification:** <why deviation is necessary>
- **Constraints that still must hold:** <what cannot be violated>
- **Approval:** Ryan (Human Authority)
- **Closure condition:** <objective condition for closing/retiring this exception>
- **References:** <work unit id>, <candidate id>, <PR id/link>, <evidence bundle id> (as applicable)
- **Status:** OPEN | CLOSED

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
  - rel: informs
    to: SR-PLAN
  - rel: informs
    to: SR-WORK-SURFACE
---

# SR-EXCEPTIONS

## About

This document is the **exception ledger** for build-time and plan-instance governance.

Exceptions exist to prevent **silent drift**: if work must deviate from governed meaning, mechanics, or required process, the deviation must be explicit, attributable, and replayable.

## Policy

### When an exception is required
An exception ID is required before proceeding when an outcome would violate or bypass any **canonical governed artifact** (see SR-CHANGE §1.2), including at minimum:

- SR-CHARTER
- SR-CONTRACT, SR-SPEC, SR-TYPES
- SR-WORK-SURFACE, SR-PROCEDURE-KIT, SR-SEMANTIC-ORACLE-SPEC, SR-EVENT-MANAGER, SR-AGENT-WORKER-CONTRACT
- SR-AGENTS, SR-DIRECTIVE, SR-PLAN
- SR-CHANGE, SR-EXCEPTIONS

If unsure whether a deviation is “real” or merely editorial, **stop and ask** for human authority. If granted, record it here as an exception.

### How to use
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

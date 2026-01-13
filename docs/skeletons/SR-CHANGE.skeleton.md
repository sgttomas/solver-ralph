---
doc_id: SR-CHANGE
doc_kind: governance.change_mgmt
layer: build
status: draft
refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: depends_on
    to: SR-TYPES
  - rel: depends_on
    to: SR-CONTRACT
---


# SR-CHANGE

Normative change-control policy for SOLVER-Ralph.

This document defines:

- **What requires formal change control**
- **How a change is proposed, verified, approved, and made "current"**
- **How portal routing works** (including how an instance may *alias portal functions* into a smaller set of portal IDs)
- **How exceptions, oracle-suite changes, and freeze baselines are governed**

SR-CHANGE is itself governed. Changes to SR-CHANGE MUST follow SR-CHANGE.

---

## 0. Version changes

### 0.1  (this draft)

- **SR-ETT eliminated:** Removed SR-ETT from refs and canonical governed set. SR-ETT content has been absorbed into SR-INTENT (rationale), SR-CONTRACT (invariants), and SR-SPEC (mechanics).
- **SR-PARADIGM → SR-GUIDE:** Renamed in governed set. SR-PARADIGM is now SR-GUIDE (usage guidance document).
- **SR-README removed:** SR-README was a narrative overview; its role is subsumed by SR-GUIDE.
- **SR-AGENTS added:** Added to canonical governed set (agent semantics are now explicitly governed).
- Updated routing table to reflect current canonical set.
- Updated type key from `governance.change_policy` to `governance.change_mgmt` per SR-TYPES.

### 0.2  (2026-01-11)

- Reframed the required portal list as **portal functions** + **instance portal IDs**, enabling **portal-function aliasing**.

## About

(TODO)

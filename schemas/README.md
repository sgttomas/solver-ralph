# Shared Schemas

This directory contains shared schema definitions for SOLVER-Ralph.

## Purpose

Per SR-SPEC and the hexagonal architecture, schemas defined here are used by:
- Domain core (Rust)
- API layer (Rust/Axum)
- UI (TypeScript/React)
- CLI tooling (TypeScript)

## Directory Structure

```
schemas/
├── events/           # Event envelope and payload schemas
├── evidence/         # Evidence bundle manifest schemas (v1)
├── messaging/        # NATS message contracts (V12-2)
├── api/              # API request/response schemas
├── domain/           # Domain entity schemas
└── codegen/          # Generated code output
```

## Schema Format

Schemas are defined in JSON Schema format and can be used to generate:
- TypeScript types (for UI and CLI)
- Rust types (via schemars or manual mapping)

## Related Documents

- SR-SPEC §1.5: Event model and envelope schema
- SR-SPEC §1.9: Evidence bundle manifest schema
- SR-SPEC §4.6: Message bus pattern
- SR-TYPES: Canonical type registry
- SR-MESSAGE-CONTRACTS: NATS message contracts (schemas/messaging/)

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
- SR-TYPES: Canonical type registry

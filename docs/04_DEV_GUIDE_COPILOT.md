# DEV GUIDE — COPILOT (Derived from COPILOT_MASTER_INSTRUCTION_V5)

**Version:** 1.0.0 (derived)  
**Source:** feature/sot/COPILOT_MASTER_INSTRUCTION_V5_FINAL_OPTIMIZED.md  
**Deutsch (übersetzt):** `docs/04_DEV_GUIDE_COPILOT.de.md`

---

## 🎯 Purpose
This document is a compact, actionable developer guide for contributors and the Copilot automation in the SC Manager repo. It codifies mandatory rules, the exact tech stack, architecture constraints, and pre-flight & stop checks to be followed for any code or docs changes.

---

## 🔴 META-RULES (MANDATORY)
- **Hierarchy of truth:** 1) This guide (derived from Master Instruction) → 2) IDC-10 Guidelines → 3) Star Citizen ToS → 4) Best Practices
- **Self-check BEFORE any change:** confirm you read the relevant section, verify layer separation, check tech stack, include tests, add error handling, avoid unwrap() in production, respect performance budget, and verify ToS compliance.
- **Stop conditions:** stop and ask for clarification if: contradiction with this doc, unclear implementation, ToS violation, layer breach, security risk, or performance budget breach.
- **Forbidden:** skip tests, merge layers, add unapproved deps, business logic in adapters, direct DB from UI, unwrap()/expect()/panic!() in production.

---

## 1️⃣ TECH STACK (EXACT)
Use the exact technologies below; NO substitutes without explicit approval.

- Rust 1.75+ (edition 2021)
- Axum 0.7+ for HTTP
- PostgreSQL 16+
- DragonflyDB 1.13+ (redis-compatible)
- NATS JetStream for enterprise event bus (in-memory for desktop)
- serde + serde_json
- sqlx (compile-time checked)

Frontend/Desktop:
- Tauri 2.0+
- SolidJS 1.8+ (no React)
- shadcn/ui + Radix
- Tailwind CSS
- @tanstack/solid-query 5.0+
- TypeScript 5.3+ (strict mode)
- pnpm (8.0+) + Turborepo

Tooling (mandatory): rustfmt, clippy, cargo-audit, cargo-deny, cargo-mutants, biome, vitest, playwright, tracing, opentelemetry.

---

## 2️⃣ ARCHITECTURE & LAYER RULES (IMMUTABLE)
Follow strict layer separation:

Layers (top → bottom):
- **UI (SolidJS + Tauri)**: presentation, stores, commands (no business logic)
- **Application**: orchestration, handlers, transactions (no business logic)
- **Domain**: pure business logic, entities, aggregates, value objects
- **Event Bus**: async pub/sub
- **Adapters**: external integrations, transformations (no business logic)
- **Infrastructure**: DB, cache, event store

Domain rules (absolutely):
- All business logic belongs here
- No HTTP/SQL/JSON/infrastructure imports
- Prefer sync code (no async unless necessary)

Application rules:
- Orchestration only; publish events; call repos; no calculations

Adapter rules:
- Transformation and I/O only; no decisions

Event Rules (mandatory):
- All state changes MUST publish events
- Events immutable, versioned, include correlation + causation IDs
- Event schema example:

```json
{
  "event_id": "uuid",
  "event_type": "string",
  "version": "1.0.0",
  "schema_version": 1,
  "timestamp": "ISO8601",
  "correlation_id": "uuid",
  "causation_id": null,
  "payload": { }
}
```

---

## 3️⃣ PROJECT STRUCTURE (RECOMMENDED)
Follow the repo layout in the Master Instruction; keep domain code isolated under `services/core-domain` and application code under `services/core-application`. Adapters and infrastructure must live in `adapters/` and `infrastructure/` respectively.

---

## 4️⃣ IMPLEMENTATION STANDARDS (MANDATORY)
- **Errors:** three-tier error enums (DomainError, ApplicationError, InfrastructureError) and use `thiserror`. No panics or unwraps.
- **Logging:** use `tracing` with structured fields and `#[instrument]`. No println!/dbg! in production.
- **Type safety:** prefer strong typed wrappers for IDs and value objects.
- **Docs:** public functions must have Rust docstrings with Arguments, Errors and Examples.

Example error enum:

```rust
#[derive(Error, Debug)]
pub enum DomainError {
  #[error("Entity not found: {entity_type} with id {id}")]
  NotFound { entity_type: String, id: String },
  // ...
}
```

---

## 5️⃣ TESTING (MANDATORY)
Coverage targets (enforced):
- Domain: 100%
- Application: 95%
- Adapters: 85%
- UI: 75%
- Overall: ≥85%

Test types: unit, property-based (proptest), integration (tokio tests), E2E (playwright), mutation testing (cargo-mutants).

CI must block merges that fall below thresholds.

---

## 6️⃣ PERFORMANCE & MONITORING
Respect the performance budgets in the master file (API p95/p99 latencies, UI load, memory budgets). Instrument code with tracing and export metrics via OpenTelemetry.

---

## 7️⃣ SECURITY & TOS
- No secrets in code.
- All input validated.
- Enforce ToS at adapter layer via a `ToSGuard` that rejects forbidden automation.
- Run cargo-audit regularly and fail CI on critical issues.

Allowed / forbidden ToS actions must be enforced by adapters not domain.

---

## 8️⃣ CHECKPOINTS & DEFINITION OF DONE (DoD)
A change is DONE only when:
- Layer separation respected (no business logic in adapters)
- Tests added & passing (coverage targets met)
- Error handling and logging present
- No unwrap()/panic!()
- Documentation & ADRs updated if architectural
- Performance budgets met and tested
- Security checks pass (cargo-audit)

---

## ✅ PRE-CODE GENERATION CHECKLIST (MUST PASS)
- [ ] Read relevant sections of this doc
- [ ] Confirm layer & file targets
- [ ] Confirm tech stack used
- [ ] Test plan exists (unit + integration required)
- [ ] Error handling plan finalized
- [ ] Performance impact assessed
- [ ] ToS compliance verified

If any item is NO → STOP and ask for clarification.

---

## 🔧 HOW TO REQUEST DEVIATIONS
Deviations require explicit approval and must be documented in an ADR. If you think a rule cannot be followed, open an issue describing the constraint and propose an ADR.

---

_Last updated (derived): 2025-12-29 — Auto-generated from COPILOT_MASTER_INSTRUCTION_V5. For disputes, consult the original master instruction file._

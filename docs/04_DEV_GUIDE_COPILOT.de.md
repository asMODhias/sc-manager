# DEV GUIDE — COPILOT (Abgeleitet) 🇩🇪

**Version:** 1.0.0 (abgeleitet)  
**Quelle:** feature/sot/COPILOT_MASTER_INSTRUCTION_V5_FINAL_OPTIMIZED.md

---

## 🎯 Zweck
Kurzfassung des Entwicklerleitfadens für Contributor und Copilot-Automatisierung im SC Manager Repo. Enthält zwingende Regeln, exaktes Tech-Stack, Architekturprinzipien und Pre-Flight-Checks.

---

## 🔴 Meta-Regeln (VERPFLICHTEND)
- **Hierarchie:** 1) Dieses Dokument → 2) IDC-10 Guidelines → 3) Star Citizen ToS → 4) Best Practices
- **Selbst-Check VOR jeder Änderung:** Relevanten Abschnitt gelesen, Schichten geprüft, Tech-Stack bestätigt, Tests eingeplant, Fehlerbehandlung vorhanden, kein unwrap()/panic!, Performance & ToS geprüft.
- **Stop-Bedingungen:** Bei Widersprüchen, Unklarheiten, ToS-Verstoß, Schichtenbrüchen, Sicherheits- oder Performance-Risiken: STOP und Rückfrage.
- **Verboten:** Tests überspringen, Schichten vermischen, nicht genehmigte Abhängigkeiten hinzufügen, Geschäftslogik in Adaptern, direkte DB-Zugriffe aus UI, unwrap()/expect()/panic! in Produktion.

---

## 1️⃣ Tech-Stack (EXAKT)
(Kein Ersatz ohne explizite Genehmigung)

- Rust 1.75+ (edition 2021)
- Axum 0.7+
- PostgreSQL 16+
- DragonflyDB 1.13+
- NATS JetStream (Enterprise) / In-Memory (Desktop)
- serde, serde_json, sqlx

Frontend/Desktop:
- Tauri 2.0+, SolidJS 1.8+, shadcn/ui + Radix, Tailwind, @tanstack/solid-query, TypeScript 5.3+, pnpm + Turborepo

Tooling: rustfmt, clippy, cargo-audit, cargo-deny, cargo-mutants, biome, vitest, playwright, tracing, opentelemetry

---

## 2️⃣ Architektur & Schichtregeln (UNVERÄNDERLICH)
- **UI:** Präsentation, Stores, Commands — keine Geschäftslogik
- **Application:** Orchestrierung, Handler — keine Geschäftslogik
- **Domain:** Reine Geschäftslogik (Entities, Aggregates, Value Objects)
- **Adapters:** I/O und Transformation — keine Entscheidungen
- **Infrastructure:** DB, Cache, Event Store

Events müssen versioniert, unveränderlich und mit Korrelation/Causation IDs sein.

---

## 3️⃣ Implementierungsstandards
- Fehler-Hierarchie: DomainError / ApplicationError / InfrastructureError (thiserror)
- Logging: tracing + #[instrument]
- Typensicherheit: Strongly-typed IDs / Value Objects
- Dokumentation: public APIs kommentieren

---

## 4️⃣ Tests & CI
- Coverage-Ziele: Domain 100%, Application 95%, Adapters 85%, UI 75%, Gesamt ≥85%
- Testarten: Unit, Property-based, Integration (tokio), E2E (playwright), Mutation Testing
- CI blockiert Merges bei Unterschreitung

---

## 5️⃣ Sicherheit & ToS
- Keine Secrets im Code
- Input validieren
- ToS-Guard in Adaptern
- cargo-audit in CI

---

## 6️⃣ Checkliste vor Code-Generierung (MUSS erfüllt sein)
- [ ] Relevante Sektionen gelesen
- [ ] Ziel-Schicht bestätigt
- [ ] Tech-Stack bestätigt
- [ ] Testplan vorhanden
- [ ] Fehlerbehandlung definiert
- [ ] Performance bewertet
- [ ] ToS geprüft

Bei einem NO: STOP und Rückfrage.

---

## 🔧 Abweichungen
Abweichungen benötigen explizite Zustimmung und ADR. Öffne ein Issue mit Vorschlag und ADR, falls nötig.

---

_Last updated (abgeleitet): 2025-12-29 — basierend auf COPILOT_MASTER_INSTRUCTION_V5._

# ADR-0001: Add derived DEV GUIDE — COPILOT

Date: 2025-12-29
Status: Proposed

## Context
The repository referenced a COPILOT master instruction but lacked a concise, actionable developer guide for contributors and automation (Copilot). The master instruction is authoritative: `feature/sot/COPILOT_MASTER_INSTRUCTION_V5_FINAL_OPTIMIZED.md`.

## Decision
Create `docs/04_DEV_GUIDE_COPILOT.md` as a derived developer guide that summarizes mandatory rules, tech stack, architectural constraints, testing & security requirements, and a pre-code checklist. Add a German translation `docs/04_DEV_GUIDE_COPILOT.de.md` and document this decision in this ADR.

## Consequences
- Contributors and automation have a small, easy-to-read guide to follow.
- Changes to policy should be done via ADRs; this ADR tracks the decision.
- Any future deviation requires explicit approval and new ADR.

## Alternatives considered
- Only referencing the master instruction (rejected because it's large and verbose for day-to-day checks).
- Keeping the doc English-only (rejected; German translation added for contributors preferring DE).

## Links
- Master instruction: `feature/sot/COPILOT_MASTER_INSTRUCTION_V5_FINAL_OPTIMIZED.md`
- Dev guide: `docs/04_DEV_GUIDE_COPILOT.md`
- Dev guide (DE): `docs/04_DEV_GUIDE_COPILOT.de.md`


Approved-by: (TBD)

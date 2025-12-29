# ADR-013: Read-only Game.log parsing

Date: 2025-12-29
Status: Proposed

## Context

The Grinding plugin must remain ToS-safe: no gameplay automation and no automatic mission completion updates. Parsing the installer-local `Game.log` can help surface *suggested* mission completions to speed up manual reporting and verification, but must be constrained to avoid policy violations and preserve user privacy.

## Decision

Introduce a read-only adapter `adapter-gamelog` that:
- Runs locally and reads `Game.log` only on user opt-in.
- Performs **parse-only** operations and produces `MissionCompletionSuggestion` objects.
- Emits the parsed suggestions to a read-model or event stream as *suggestions* (not commands); the Grinding plugin stores suggestions separately and requires **officer verification** before applying any progress.
- Enforces strict sandboxing and no network or filesystem writes beyond local config and temporaries (and no background auto-processing). All parsing must be deterministic and tested.

## Constraints

- No automatic verification or conquest of game state.
- No network access (no telemetry upload). Local-only.
- No background automation without explicit user opt-in and explicit confirmation per parsing session.
- Respect performance budgets: parsing should be incremental (streaming) and not block UI.
- Must be audited and logged (only locally) and offer an opt-out and data deletion path.

## Implementation Notes

- Tech: Rust adapter inside `adapters/adapter-gamelog`.
- API: expose a simple `parse_line` + `parse_file` API that returns `MissionCompletionSuggestion`.
- Testing: unit tests for parser rules, fuzz tests for malformed lines, and performance tests for large logs.
- Privacy: store suggestions only in plugin-scoped storage; provide commands in UI for users to accept/reject suggestions. Suggestions are not authoritative.

## Consequences

- Improves UX by surfacing likely completions for officer verification.
- Must be carefully documented (privacy, opt-in, ToS compliance) and visible to users.

## Next steps

- Create adapter skeleton, tests, and parser rules.
- Add UI flows for user opt-in and suggestion review in the Grinding plugin.

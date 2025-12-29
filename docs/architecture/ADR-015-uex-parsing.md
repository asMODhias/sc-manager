# ADR-015: UEX Log Parsing

Status: Proposed → Accepted

Date: 2025-12-29

## Kontext

UEX (User Experience / telemetry-like) exports or local logs are used by squad tools and third-party utilities. For ToS-compliance and user privacy the adapter must be:

- Local-only (no network by default)
- Read-only and conservative in parsing
- Emit **suggestions / hints** only (no automatic actions)
- Deterministic and test-covered

## Entscheidung

- Implement `adapter-uex` to provide safe, local, read-only parsing of UEX-style log lines.
- Expose simple, streaming-friendly functions:
  - `parse_line(&str) -> Option<UexRecord>`
  - `parse_reader<R: BufRead>(reader: R) -> Result<Vec<UexRecord>, Error>`
  - `parse_file(path: &Path) -> Result<Vec<UexRecord>, Error>`
- Provide a small local CLI binary `adapter_uex_cli` that prints JSON objects (one per line) for easy inspection.
- Include unit tests + property tests (proptest) to validate robustness.

## Rationale

This mirrors the approach used in ADR-013 (Game.log) and ADR-014 (FleetYards), keeps parsing conservative, and preserves the rule that outputs are suggestions requiring manual/officer verification.

## Consequences

- Adapters must remain parse-only (no queries to remote services).
- Plugin authors may use adapter outputs as suggestion sources but must ensure opt-in and verification.

## Notes

- Follow implementation conventions from other adapters: use `serde` for JSON types, `proptest` for fuzzing, no business logic in adapter code.

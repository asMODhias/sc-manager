# ADR-014: FleetYards CSV parsing

Date: 2025-12-29
Status: Accepted

## Context

FleetYards exposes CSV exports of ship lists and activity that can be useful to populate local read-models or to suggest ship-related events. Any parsing must be local-only, conservative, and not perform any automatic state changes.

## Decision

Introduce `adapter-fleetyards` that:
- Exposes a parse-only API: `parse_reader`, `parse_file` returning `FleetyardsRecord` objects.
- Provides a small CLI `adapter_fleetyards_cli` for ad-hoc inspection (prints JSON one-per-line).
- Includes unit tests and property-based tests to avoid crashes on malformed input.
- Remains read-only and local-only. Data produced is a hint and must be verified by consuming systems before affecting state.

## Implementation Notes

- Tech: Rust, using `csv`, `serde`, `chrono` and `serde_json` for serialization.
- API: `parse_reader<R: BufRead>(reader: R) -> Vec<FleetyardsRecord>` and `parse_file(path: &Path)`.
- CLI: `src/bin/adapter_fleetyards_cli.rs` emits JSON records for a CSV file.
- Tests: unit tests for parsing and roundtrip, `proptest` fuzz test to ensure no panics on arbitrary input.

## Consequences

- Improves UX for ship inventory/activity inspection and downstream suggestion flows.
- Must be documented in the plugin integration docs and tested in CI.

## Next steps

- Add integration examples for the Desktop app
- Add performance tests for large exports
- Keep ADR and README synchronized with implementation

# ADR-017: Erkul Telemetry / Squad Tools Parsing

Status: Proposed → Accepted

Date: 2025-12-29

## Kontext

Erkul is a common third-party squad/fleet tool that can export telemetry or activity logs. To integrate suggestions into SC Manager in a ToS-safe way we need a conservative parser that reads local Erkul exports and emits suggestions only.

## Entscheidung

- Implement `adapter-erkul` to parse Erkul-style CSV or line exports into a `ErkulRecord` struct.
- Expose `parse_line`, `parse_reader`, and `parse_file` APIs and a small CLI `adapter_erkul_cli` that prints JSON per line.
- Include unit tests and property tests to ensure robustness and avoid panics on malformed inputs.

## Rationale

Consistent with prior adapters (ADR-013..016): local-only parsing, conservative extraction, and suggestion-only outputs requiring manual/officer verification.

## Konsequenzen

- No network access or automatic state changes from this adapter.
- Plugin authors must verify and require opt-in before using outputs.
- Keep parsing logic lightweight; prefer streaming-friendly APIs for large exports.

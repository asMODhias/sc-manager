# adapter-fleetyards

Purpose: Parse FleetYards CSV exports to produce structured `FleetyardsRecord` objects for downstream plugins or analysis. The adapter is intentionally parse-only and contains no business logic.

Key rules:
- Read-only parsing; no writes to game files
- Local-only; no network
- Conservative parsing: best-effort CSV mapping to fields (timestamp, member_rsi, ship, action)
- Suggestions/data are hints only and must be treated as unverified by consuming code

Usage (CLI): build with `cargo build --bin adapter_fleetyards_cli` and run `./target/debug/adapter_fleetyards_cli path/to/export.csv` to print JSON records (one per line).

Testing:
- Unit tests cover parsing and file handling
- Property tests (proptest) ensure parser does not panic on malformed input

Privacy & ToS: Do not perform automated in-game actions based on parsed data. Suggestion events should be reviewed/verified by officers before affecting state.

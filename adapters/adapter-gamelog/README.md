# adapter-gamelog

Purpose: Local, read-only parsing of Star Citizen `Game.log` to produce `MissionCompletionSuggestion` objects for the Grinding plugin.

Key rules:
- Read-only: no file modifications to game files
- Local-only: no network uploads
- Opt-in: user must enable parsing explicitly
- Suggestions-only: parsed data is a hint that requires officer verification

Files:
- `src/lib.rs` - parser implementation
- `tests/parser_test.rs` - unit tests for parser

Usage (internal or CLI): the adapter exposes `parse_line`, `parse_file` and `parse_reader` APIs for programmatic use. A small CLI is available at `src/bin/adapter_gamelog_cli.rs` (build with `cargo build --bin adapter_gamelog_cli`) that prints JSON suggestions (one-per-line) for a given `Game.log` file — still local-only and opt-in.

Privacy & ToS:
- Do not perform any automated actions in game.
- All outputs must be treated as unverified suggestions and require manual officer verification.

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

Usage (internal): called by a scheduled local job or on-demand via UI; returns suggestions for the plugin to display.

Privacy & ToS:
- Do not perform any automated actions in game.
- All outputs must be treated as unverified suggestions and require manual officer verification.

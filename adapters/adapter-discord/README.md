# adapter-discord

Conservative, local-only parser for Discord exported chat lines or exported logs.

Usage (library):

- `parse_line(&str) -> Option<DiscordMessage>`
- `parse_reader<R: BufRead>(reader) -> io::Result<Vec<DiscordMessage>>`
- `parse_file(path: &Path) -> io::Result<Vec<DiscordMessage>>`

CLI usage:

```bash
cargo run --bin adapter_discord_cli -- /path/to/discord_export.txt
```

Notes:
- This adapter is read-only and emits suggestions only.
- Parsing is conservative and intentionally simple; plugin authors must verify any suggestions before applying state changes.

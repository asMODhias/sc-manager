# adapter-uex

Conservative, local-only parser for UEX-style logs.

Usage (library):

- `parse_line(&str) -> Option<UexRecord>`
- `parse_reader<R: BufRead>(reader) -> io::Result<Vec<UexRecord>>`
- `parse_file(path: &Path) -> io::Result<Vec<UexRecord>>`

CLI usage:

```bash
cargo run --bin adapter_uex_cli -- /path/to/uex.log
```

Notes:
- This adapter is read-only and emits suggestions only.
- Parsing is conservative and intentionally simple; plugin authors must verify any suggestions.

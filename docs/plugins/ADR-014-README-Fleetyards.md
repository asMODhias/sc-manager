# FleetYards Adapter Integration (Summary)

- Source: `adapter-fleetyards` (local CSV parser)
- API: `parse_reader` / `parse_file` returning `FleetyardsRecord` (timestamp, member_rsi, ship, action, raw_line)
- CLI: `adapter_fleetyards_cli` prints JSON suggestions (one-per-line)
- Delivery: Consumer code may publish suggestions/events to the Core event bus (read-only) but must not rely on data as authoritative without verification
- Privacy & ToS: parsing is local-only and read-only; do not trigger automated in-game actions from parsed data

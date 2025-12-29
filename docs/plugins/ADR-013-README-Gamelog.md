# Game.log Adapter Integration (Summary)

Event: MissionSuggestion

- Source: `adapter-gamelog` (local, read-only parser)
- API: `parse_line`, `parse_reader`, `parse_file` return `MissionCompletionSuggestion` structs; a small CLI (`adapter_gamelog_cli`) emits JSON suggestions (one object per line) for ad-hoc inspection.
- Event payload: `MissionSuggestion` (id, mission_name, member_rsi, timestamp, raw_line)
- Delivery: Published to Core event bus as `MissionSuggestion` (read-only)
- Consumers: `plugins/grinding` subscribes to `MissionSuggestion` and stores suggestions if user opted-in

Behavioral rules:
- Adapter emits suggestions only; may not perform any state changes.
- Plugin must require explicit user opt-in (`gamelog:opt_in`) before accepting suggestions.
- All suggestions are stored as pending and require officer verification to become MissionProgress.

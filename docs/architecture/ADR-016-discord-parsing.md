# ADR-016: Discord Export / Chat Log Parsing

Status: Proposed → Accepted

Date: 2025-12-29

## Kontext

Discord allows organizations to export chat history or local clients may save log-like exports (e.g., moderation logs, message history). To enable audit and suggestion workflows (ToS-safe), we need a conservative, local-only parser that extracts message events without connecting to Discord APIs or performing any automated moderation actions.

## Entscheidung

- Implement `adapter-discord` to parse exported chat lines / message exports into `DiscordMessage` records.
- Expose `parse_line`, `parse_reader`, and `parse_file` APIs plus a local CLI `adapter_discord_cli` printing JSON lines.
- Parsing is conservative: extract timestamp (if present), author (if present), channel or context (if present), and message text; always include raw_line.
- Include unit tests and property tests (proptest) to ensure robustness against malformed input.

## Rationale

Keeps parsing local, deterministic, and safe. Matches existing adapter patterns (ADR-013/014/015) and enforces that outputs are suggestions requiring manual/officer verification.

## Konsequenzen

- Adapters must not access Discord network or credentials.
- Plugins using adapter output must ask users to opt-in and require officer verification before any state change.
- Parsing must be kept deliberately conservative to avoid false assertions about message semantics.

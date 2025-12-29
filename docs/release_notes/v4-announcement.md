---
title: v4-announcement
master: false
source_of_truth: false
sot_references:
  - .pull_requests\draft-feat-operation-command.md
  - .\scripts\docs\README.md
  - .\scripts\release\README-msi.md
  - .\COPILOT_MASTER_INSTRUCTION_V4_Final.md
  - .\desktop_installer_idc_10.md
  - .\grinding_domain.md
  - .\grinding_events.md
  - .\grinding_tests.md
  - .\grinding_ui.md
  - .\sc manager v4 construction set.md
  - .\SC_MANAGER_DESKTOP_V4.md
  - .\SC_MANAGER_DESKTOP_V4_WINDOWS.md
last_synced: 2025-12-27
---


# Release v4.0.0 â€” Team Announcement (Localâ€‘first)

**Date:** 2025-12-23

Kurz: v4 ist fertig lokalisiert. Die IDCâ€‘10 Spezifikation ist kanonisch in `docs/`; Rootâ€‘Duplikate archiviert. Alle Core/Adapters/UI/E2E smokeâ€‘tests laufen lokal grÃ¼n. Ein Gateway dependency blocker wurde behoben und dokumentiert (`docs/issues/0001-axum-feature.md`).

Was wurde getan:
- IDCâ€‘10 docs: frontmatter + TOC, canonical `docs/` set. (`docs/FINAL_REWORK_V2_IDC10*.md`)
- Release notes: `docs/release_notes/v4.md` (final).
- Changelog: `docs/SPEC_CHANGELOG.md` updated (v4 entry + patch fixing gateway dependencies).
- Gateway fix: removed invalid `axum` feature, aligned `hyper` to 1.x, added `tokio` features, gated binary behind feature `run_server`.

How to run locally (gateway):
- Start a NATS server locally (for event publish): `docker run --rm -p 4222:4222 nats` or use a local nats-server installation.
- Start the gateway: `.\












If you want, I can prepare a short PR/branch with these changes and a oneâ€‘liner message ready for the maintainers, or create the announcement message for the internal channel (Slack / email). Which do you prefer?- Gateway issue: `docs/issues/0001-axum-feature.md`- Changelog: `docs/SPEC_CHANGELOG.md`- Release notes: `docs/release_notes/v4.md`Links:- If you want a publish to a remote registry or a public release, we'll need to decide whether to push tags/remotes; current tag `v4.0.0` is local.- Gateway binary run is gated to avoid blocking tests; if you need a different local run pattern we can add a simple wrapper or a docker compose service.Notes / Open items:- Alternatively: `cargo run -p sc_manager_gateway --features run_server` (the binary is gated behind `run_server`).un-gateway.ps1` (sets defaults: GATEWAY_BIND=0.0.0.0:8080, NATS_URL=nats://127.0.0.1:4222).


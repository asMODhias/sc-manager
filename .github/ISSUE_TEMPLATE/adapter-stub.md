---
title: adapter-stub
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


**Adapter Name:** `adapter-<name>`

**Purpose:** Short description of external source (e.g., FleetYards ship database)

**Acceptance criteria:**
- New crate under `adapters/adapter-<name>` with `Cargo.toml`, `src/lib.rs` implementing `sc_manager_adapters::DataAdapter` stub
- `README.md` describing env vars, healthcheck, and contract
- `Dockerfile` for local dev (runs `cargo test`)
- Basic unit tests (tokio test) in `tests` or `src`
- Issue or PR references the service-inventory entry in `docs/service-inventory.md`



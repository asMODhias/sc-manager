---
title: v4-draft
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


# v4.0 â€” Draft Release Notes

**Status:** Draft (2025-12-23)

## Short summary
v4.0 is a consolidation release that stabilizes the IDC-10 product definition into a release-ready state, unifies documentation, and prepares the repo for the first public local-only release cycle. There are no gameplay-affecting features; all P2P and adapter behaviour remains read-only with local data sovereignty.

## Highlights
- Canonical spec consolidated under `docs/` (IDC-10 v2 finalized). Root duplicates archived.
- Added formal document metadata and TOC to IDC-10 docs.
- Release checklist and CI health targets formalized.

## Migration / Notes
- All users should migrate to the `docs/` canonical docs; root files are archived in `docs/archive/`.
- No DB or on-disk schema changes are expected for this release; if adapters introduce storage changes, a migration note will be added.

## Checklist (local-first)
- [ ] All unit tests (core) pass locally (coverage â‰¥ 80%).
- [ ] Adapter integration tests pass locally via `scripts/quick-tests.ps1`.
- [ ] E2E smoke tests pass locally (Playwright smoke runs).
- [ ] Changelog updated (`docs/SPEC_CHANGELOG.md`).
- [ ] Release notes reviewed and merged.
- [ ] Tag `v4.0.0` created on `main` after local verification.

## Release artifacts
- `docs/release_notes/v4.md` (final)
- Tag: v4.0.0

## Communication
- Publish short team announcement with PR/Release links and brief migration instructions.



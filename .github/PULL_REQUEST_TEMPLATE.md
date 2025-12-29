---
title: PULL_REQUEST_TEMPLATE
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


<!-- Describe the change in one or two sentences -->

### Related Issue / Feature
Link to issue or feature in `docs/03_FEATURES_V1.md` if applicable.

### Checklist
- [ ] Implementiert nur erlaubte Features (siehe `docs/03_FEATURES_V1.md`)
- [ ] Hub Policy: Any new data source must be approved per `docs/HUB_POLICY.md` and referenced in this PR
- [ ] Tests added: Unit / Integration
- [ ] All tests pass locally (`cargo test`, `npm test`)
- [ ] CI is green
- [ ] Copilot used? If yes, confirm `copilit_instructions.md` followed

### Pre-Code Checklist (MUST be completed for code changes)
- [ ] Relevant sections of `docs/04_DEV_GUIDE_COPILOT.md` have been read
- [ ] Target layer confirmed (UI / Application / Domain / Adapter / Infrastructure)
- [ ] Tech stack confirmed and used exactly per guide
- [ ] Unit and integration tests are included (per test coverage targets)
- [ ] Error handling and logging added (no unwrap()/panic!())
- [ ] Performance impact assessed and within budgets
- [ ] ToS compliance verified and enforced at adapter layer if applicable
- [ ] If deviating from the guide, an ADR is included or referenced
- [ ] Local CI executed and green: run `scripts/run-local-ci.sh` (Linux/macOS) or `scripts/run-local-ci.ps1` (PowerShell) and confirm results
- [ ] If using the VS Code tasks, run the "Local CI (PowerShell)" or "Local CI (Bash)" tasks before opening the PR


### Description
Provide a short description of the change and rationale.

### Deployment notes
Any migration, manual steps, or rollbacks?


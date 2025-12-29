---
title: adapter-registry
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


**Summary:** Implement and test the adapter fetch -> publish flow, ensure retries, metrics and signing.

**Acceptance criteria:**
- Adapter registry `fetch_and_update` uses registry to retrieve adapter by name, calls `fetch`, serializes and signs event, publishes to NATS subject `adapters.<name>.data` via `EventPublisher`.
- Retries/backoff implemented and metrics recorded (adapter_fetch_success_total, adapter_fetch_failure_total, adapter_publish_total, adapter_fetch_seconds)
- Unit tests and integration test validating end-to-end (mock publisher or NATS in test harness)



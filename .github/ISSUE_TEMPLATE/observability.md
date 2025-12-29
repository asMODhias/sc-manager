---
title: observability
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


**Summary:** Add Prometheus scrape config, Grafana provisioning and include both services in `docker/docker-compose.v4.yml` for local dev and CI.

**Acceptance criteria:**
- `docker/prometheus/prometheus.yml` exists and scrapes gateway & adapters
- `docker/docker-compose.v4.yml` includes `prometheus` and `grafana`
- Optional: a basic Grafana dashboard JSON under `docker/grafana/provisioning/dashboards` to validate metrics
- CI workflow updated to collect Prometheus/Grafana logs in case of failures



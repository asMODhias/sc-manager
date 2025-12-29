# Local CI — How to run the full test suite locally

We provide a local script and VS Code tasks to run the same checks as the GitHub Actions `Local CI` workflow. Use this before opening PRs to reduce CI cycles and catch issues early.

## Quick commands

- Bash (Linux / WSL / Git Bash / macOS):

```bash
./scripts/run-local-ci.sh
```

- PowerShell (Windows / pwsh):

```powershell
./scripts/run-local-ci.ps1
```

## VS Code Tasks
There are tasks defined in `tasks.json`/workspace (if provided):
- `Local CI (PowerShell)` — runs PowerShell local CI script
- `Local CI (Bash)` — runs Bash local CI script
- **New:** `Local CI (fast)` — runs the fast local script (`./scripts/run-local-ci.sh`).
- **New:** `Local GH Actions (act)` — runs the GitHub Actions `local-ci` job locally via `act` using `./scripts/run-gh-actions-local.sh`. See below for `act` usage and prerequisites.
- **New:** `Local CI (status)` — runs `./scripts/run-local-ci-status.sh` and prints a concise pass/fail summary for each major check and a path to a detailed log for troubleshooting.

## Notes
- The local scripts aim to mirror the GitHub Actions workflow but may skip some CI-only steps (e.g., certain secrets, docker builds).
- E2E tests require Playwright and may need additional setup (browsers installation). Use `npx playwright install` if needed.
- If the local script fails, fix issues locally and re-run until green before opening a PR.

---

If you want, I can add a `Makefile` target or a `poetry` helper to simplify running these on different platforms.
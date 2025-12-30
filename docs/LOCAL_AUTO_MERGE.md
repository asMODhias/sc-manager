# Local Auto-Merge

This document describes the local-auto-merge workflow used for exceptional merges (e.g., when remote GitHub Actions are unavailable).

Policy Summary
--------------
- We use this process only when remote CI is unavailable or quota exhausted.
- Before merge: run the full local CI (`./scripts/run-local-ci.sh` / PowerShell variant) and ensure it completes successfully.
- If local CI passes, the `scripts/auto-merge-local-ci.(ps1|sh)` script will post a PR comment and perform an admin-override merge (deleting the branch).

Usage
-----
PowerShell (Windows):

```powershell
./scripts/auto-merge-local-ci.ps1 -PRNumber 28
```

Bash (Unix):

```bash
./scripts/auto-merge-local-ci.sh 28
```

Requirements
------------
- `gh` (GitHub CLI) installed and authenticated with an account that is allowed to perform admin-override merges.
- Local CI scripts must be functional and trusted by the repo maintainers.

Audit & Governance
------------------
- Every auto-merge will post a comment to the PR stating that it was merged due to local-CI pass and admin override.
- Auto-merges should be recorded in `docs/ADMIN_OVERRIDE.md` for auditing and traceability.
- This should not replace restoring remote CI; re-enable and run remote checks when runner capacity returns.

Safety Note
-----------
Auto-merging bypasses remote checks. Use only when necessary and with explicit confirmation that legal/licensing checks have been performed for content-sensitive PRs.
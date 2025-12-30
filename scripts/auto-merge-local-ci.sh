#!/usr/bin/env bash
# Runs local CI and merges a PR via admin override when CI passes.
# Usage: ./scripts/auto-merge-local-ci.sh <pr-number>
set -euo pipefail
PR=${1:-28}
SCRIPT_DIR=$(dirname "$0")

echo "[auto-merge] Running local CI..."
"$SCRIPT_DIR/run-local-ci.sh"
EXIT=$?
if [ $EXIT -ne 0 ]; then
  echo "[auto-merge] Local CI failed with exit code $EXIT. Aborting merge." >&2
  exit $EXIT
fi

echo "[auto-merge] Local CI passed. Commenting and merging PR #$PR..."
gh pr comment $PR --body "Auto-merge: Local CI passed; merging via admin override as per local-only CI policy."
gh pr merge $PR --merge --delete-branch --admin || { echo "[auto-merge] Merge command failed" >&2; exit 2; }

echo "[auto-merge] PR #$PR merged and branch deleted."
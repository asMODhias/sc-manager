#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(git rev-parse --show-toplevel)
cd "$ROOT_DIR"

MISSING=0

check_file() {
  if [ ! -f "$1" ]; then
    echo "[ERROR] Required file missing: $1"
    MISSING=1
  else
    echo "[OK] Found: $1"
  fi
}

check_file "docs/04_DEV_GUIDE_COPILOT.md"
check_file "docs/04_DEV_GUIDE_COPILOT.de.md"
check_file "docs/adr/ADR-0001-COPILOT-DEV-GUIDE.md"

# Simple heuristic: ensure tests were changed or present when code changes occur
# If the PR only changes docs, this is fine; otherwise require at least one test file or 'tests' folder
CHANGED_FILES=$(git diff --name-only origin/${GITHUB_BASE_REF:-HEAD}...HEAD || true)
if echo "$CHANGED_FILES" | grep -E "^.*\b(test|tests|spec)\b.*" >/dev/null 2>&1; then
  echo "[OK] Tests detected in changed files"
else
  echo "[WARN] No test files detected in changed files. Ensure unit/integration tests are added if code changes were made."
fi

if [ "$MISSING" -ne 0 ]; then
  echo "\n[FAIL] Dev guide checks failed. See errors above."
  exit 2
fi

echo "\n[PASS] Dev guide sanity checks passed. If code changes exist, ensure tests and error handling are present."
#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(git rev-parse --show-toplevel)
cd "$ROOT_DIR"

TMPLOG=$(mktemp --suffix .local-ci.log)
trap 'rm -f "$TMPLOG"' EXIT

echo "==> Local CI Status run: $(date -u +"%Y-%m-%d %H:%M:%SZ")"

SUMMARY=()

MAX_STEP_SECONDS=${MAX_STEP_SECONDS:-300}

step() {
  local name="$1"
  shift
  local start=$(date +%s)
  echo "--- Running: $name (timeout: ${MAX_STEP_SECONDS}s)"

  if command -v timeout >/dev/null 2>&1; then
    # Use timeout utility when available
    if timeout "${MAX_STEP_SECONDS}s" "$@" 2>&1 | tee -a "$TMPLOG"; then
      local rc=0
    else
      local rc=$?
      if [ "$rc" -eq 124 ] || [ "$rc" -eq 137 ]; then
        echo "!!! $name: timed out after ${MAX_STEP_SECONDS}s" | tee -a "$TMPLOG"
      fi
    fi
  else
    # Fallback: run command in background and kill after timeout
    ("$@" 2>&1 | tee -a "$TMPLOG") &
    local pid=$!
    local waited=0
    while kill -0 "$pid" 2>/dev/null; do
      sleep 1
      waited=$((waited+1))
      if [ "$waited" -ge "$MAX_STEP_SECONDS" ]; then
        echo "!!! $name: timed out after ${MAX_STEP_SECONDS}s (killing $pid)" | tee -a "$TMPLOG"
        kill -9 "$pid" 2>/dev/null || true
        wait "$pid" 2>/dev/null || true
        local rc=124
        break
      fi
    done
    if [ -z "${rc+set}" ]; then
      wait "$pid"
      local rc=$?
    fi
  fi

  local end=$(date +%s)
  local dur=$((end-start))
  if [ "$rc" -eq 0 ]; then
    echo "--- $name: PASS ($dur s)" | tee -a "$TMPLOG"
    SUMMARY+=("$name: PASS ($dur s)")
  else
    echo "--- $name: FAIL ($dur s)" | tee -a "$TMPLOG"
    SUMMARY+=("$name: FAIL ($dur s)")
  fi
  return $rc
}

# 1) Check for forbidden patterns in changed non-test files
echo "==> Pre-check: forbidden patterns (unwrap/panic) in changed non-test files"
./scripts/check-pr-unwrap.sh 2>&1 | tee -a "$TMPLOG"
if grep -q "Forbidden patterns found" "$TMPLOG" || grep -E "\.unwrap\(|panic!\(|TODO\(SOT\)" "$TMPLOG"; then
  echo "==> Pre-check found potential issues (see log)" | tee -a "$TMPLOG"
else
  echo "==> Pre-check: no new forbidden patterns in changed non-test files" | tee -a "$TMPLOG"
fi

# 2) Formatting
step "Formatting check" cargo fmt -- --check || true

# 3) Clippy
step "Clippy" cargo clippy --workspace --all-targets -- -D warnings || true

# 4) Core tests
pushd core >/dev/null
step "Core tests" cargo test --verbose || true
popd >/dev/null

# 5) Adapters tests
pushd adapters >/dev/null
step "Adapters tests" cargo test --verbose || true
popd >/dev/null

# 6) App integration tests (best-effort)
pushd app >/dev/null
step "App integration tests" bash -lc "cargo test --test integration --verbose" || true
popd >/dev/null

# 7) UI tests (if present)
if [ -d ui ] && [ -f ui/package.json ]; then
  pushd ui >/dev/null
  step "UI: install & test" bash -lc "npm ci && npm test --if-present" || true
  popd >/dev/null
else
  SUMMARY+=("UI: SKIPPED")
fi

# 8) E2E prepare (if present)
if [ -d e2e ] && [ -f e2e/package.json ]; then
  step "E2E: prepare fixtures" bash -lc "python scripts/anonymize_game_log.py --in e2e/fixtures/raw_game.log --out e2e/fixtures/raw_game.anonymized.log --seed=local || true"
  SUMMARY+=("E2E: prepared (manual run needed)")
else
  SUMMARY+=("E2E: SKIPPED")
fi

# Summary
echo "\n==> Local CI Status Summary" | tee -a "$TMPLOG"
for s in "${SUMMARY[@]}"; do
  echo " - $s" | tee -a "$TMPLOG"
done

echo "\nDetailed log available at: $TMPLOG"
# keep the log around for inspection (do not delete it)
echo "Local CI Status completed. Log: $TMPLOG"

exit 0

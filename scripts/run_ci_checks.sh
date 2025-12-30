#!/usr/bin/env bash
set -euo pipefail

echo "==> 1) Run cargo clippy (deny warnings)"
cargo clippy --workspace --all-features -- -D warnings

echo "==> 2) Run forbidden-pattern checks"
./scripts/check_forbidden_patterns.sh

echo "==> 3) Run cargo tests (workspace)"
cargo test --workspace --all-features

# Try to run Tarpaulin for coverage; if not available, fail the job
if command -v cargo-tarpaulin >/dev/null 2>&1; then
  echo "==> 4) Run cargo-tarpaulin (fail if <85%)"
  cargo tarpaulin --workspace --all --ignore-tests --fail-under 85
else
  echo "cargo-tarpaulin not installed; install in CI for coverage checks (recommended)"
  exit 1
fi

# Run UI tests (if ui folder exists)
if [ -d "ui" ]; then
  echo "==> 5) Run UI tests (pnpm)"
  corepack enable || true
  corepack prepare pnpm@8.0.0 --activate || true
  pushd ui >/dev/null
  pnpm ci
  pnpm test
  popd >/dev/null
fi

echo "CI checks passed."
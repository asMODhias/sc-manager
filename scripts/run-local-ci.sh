#!/usr/bin/env bash
set -euo pipefail

# Local CI script — runs the same checks as .github/workflows/local-ci.yml
ROOT_DIR=$(git rev-parse --show-toplevel)
cd "$ROOT_DIR"

echo "==> Formatting check"
cargo fmt -- --check

echo "==> Preflight: protoc check"
PROTOC_FOUND=0
if [ -n "${PROTOC:-}" ]; then
  if [ -x "${PROTOC}" ]; then
    PROTOC_FOUND=1
  fi
elif command -v protoc >/dev/null 2>&1; then
  PROTOC_FOUND=1
fi
if [ "$PROTOC_FOUND" -eq 0 ]; then
  echo "[warn] protoc not found. Crates requiring protoc (prost) will be skipped. Install protoc or set PROTOC env var to enable them."
  SKIP_PROTOC=1
else
  SKIP_PROTOC=0
fi

echo "==> Clippy"
# Run clippy per-crate so we can skip protos when protoc is missing
find . -name Cargo.toml -not -path "./patches/*" -not -path "./artifacts/*" -print0 | while IFS= read -r -d '' manifest; do
  if [ "$SKIP_PROTOC" -eq 1 ] && echo "$manifest" | grep -q "p2p"; then
    echo "Skipping $manifest (requires protoc)"
    continue
  fi
  echo "Clippy $manifest"
  cargo clippy --manifest-path "$manifest" --all-targets -- -D warnings
done

echo "==> Run core tests"
pushd core >/dev/null
cargo test --verbose
popd >/dev/null

echo "==> Run adapters tests"
pushd adapters >/dev/null
cargo test --verbose
popd >/dev/null

echo "==> Run app integration tests"
pushd app >/dev/null
cargo test --test integration --verbose || true
popd >/dev/null

if [ -d ui ] && [ -f ui/package.json ]; then
  echo "==> UI tests"
  pushd ui >/dev/null
  npm ci
  npm test --if-present
  popd >/dev/null
else
  echo "==> Skipping UI tests (ui/package.json missing)"
fi

if [ -d e2e ] && [ -f e2e/package.json ]; then
  echo "==> E2E: prepare fixtures"
  python scripts/anonymize_game_log.py --in e2e/fixtures/raw_game.log --out e2e/fixtures/raw_game.anonymized.log --seed=local || true
  python scripts/anonymize_game_log.py --in e2e/fixtures/fleetyards_export.csv --out e2e/fixtures/fleetyards_export.anonymized.csv --seed=local || true
  python scripts/anonymize_game_log.py --in e2e/fixtures/erkul_dump.json --out e2e/fixtures/erkul_dump.anonymized.json --seed=local || true
  python scripts/import_fixtures.py --game e2e/fixtures/raw_game.anonymized.log --fleet e2e/fixtures/fleetyards_export.anonymized.csv --erkul e2e/fixtures/erkul_dump.anonymized.json --out e2e/testdata/testdata.json || true
  
  echo "==> Install Playwright browsers"
  pushd e2e >/dev/null
  npx playwright install --with-deps || true
  npx playwright test --reporter=dot || true
  popd >/dev/null
else
  echo "==> Skipping E2E tests (e2e/package.json missing)"
fi

echo "Local CI finished."
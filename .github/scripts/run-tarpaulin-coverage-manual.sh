#!/bin/bash
set -euo pipefail

# clean previous artifacts
rm -rf profraw artifacts/coverage/lcov artifacts/coverage/tarpaulin || true
mkdir -p artifacts/coverage/lcov artifacts/coverage/tarpaulin

DIRS=(core app adapters services/*)
for d in "${DIRS[@]}"; do
  if [ -f "$d/Cargo.toml" ]; then
    name=$(basename "$d")
    echo "Running tarpaulin for $d -> artifacts/coverage/lcov/${name}.info"
    (cd "$d" && cargo tarpaulin --out Lcov --ignore-tests) > "artifacts/coverage/lcov/${name}-tarpaulin.log" 2>&1 || true
    # tarpaulin writes lcov to lcov.info in crate dir
    if [ -f "$d/lcov.info" ]; then
      mv "$d/lcov.info" "artifacts/coverage/lcov/${name}.info" || true
    fi
  else
    echo "Skipping $d (no Cargo.toml)"
  fi
done

# Merge and generate HTML
chmod +x ./scripts/generate-coverage-tarpaulin.sh || true
./scripts/generate-coverage-tarpaulin.sh || true

echo "Done. Artifacts in artifacts/coverage/tarpaulin"
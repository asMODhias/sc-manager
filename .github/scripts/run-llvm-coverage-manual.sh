#!/bin/bash
set -euo pipefail

mkdir -p profraw
# remove any stale profraws from previous runs to avoid merge errors
rm -f profraw/*.profraw || true
# ensure nightly is up to date and selected
rustup update nightly || true
rustup default nightly || true
export RUSTFLAGS="-Z instrument-coverage -C link-dead-code"
export LLVM_PROFILE_FILE="profraw/%p-%m.profraw"
echo "RUSTFLAGS=$RUSTFLAGS"
echo "LLVM_PROFILE_FILE=$LLVM_PROFILE_FILE"

DIRS=(core app adapters services/*)
for d in "${DIRS[@]}"; do
  if [ -f "$d/Cargo.toml" ]; then
    echo "Running tests in $d"
    (cd "$d" && cargo +nightly test --tests) || true
  else
    echo "Skipping $d: no Cargo.toml"
  fi
done

chmod +x ./scripts/generate-coverage-llvm.sh || true
./scripts/generate-coverage-llvm.sh || true

echo "done"

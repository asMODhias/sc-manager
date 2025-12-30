#!/bin/sh
set -euo pipefail

echo "--- rustup show ---"
rustup show || true

echo "--- default to nightly if available ---"
rustup default nightly || true

mkdir -p profraw
export RUSTFLAGS="-Z instrument-coverage -C link-dead-code"
export LLVM_PROFILE_FILE="profraw/%p-%m.profraw"
echo "RUSTFLAGS: $RUSTFLAGS"
echo "LLVM_PROFILE_FILE: $LLVM_PROFILE_FILE"

# Run tests with coverage instrumentation
cargo +nightly test --workspace --tests || true

# Generate coverage using existing script
chmod +x ./scripts/generate-coverage-llvm.sh || true
./scripts/generate-coverage-llvm.sh || true

echo "--- done ---"
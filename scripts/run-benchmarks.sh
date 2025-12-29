#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(git rev-parse --show-toplevel)
cd "$ROOT_DIR/core"

# Run benchmarks (criterion-based)
export RUSTFLAGS="-C target-cpu=native"
cargo bench -- --noplot

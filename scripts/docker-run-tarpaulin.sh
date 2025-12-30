#!/usr/bin/env bash
set -euo pipefail

echo "Installing system packages (llvm, clang, ...). This may take a while..."
DEBIAN_FRONTEND=noninteractive apt-get update
DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends llvm clang pkg-config libssl-dev ca-certificates curl build-essential

# Ensure we have a new enough Rust toolchain (supports edition2024).
# Default to 'nightly' for maximum compatibility; override with RUST_TOOLCHAIN env var if necessary.
RUST_TOOLCHAIN="${RUST_TOOLCHAIN:-nightly}"

# Install rustup/toolchain if rustc is missing or too old
if ! command -v rustc >/dev/null 2>&1; then
  echo "rustc not found — installing rustup and ${RUST_TOOLCHAIN} toolchain..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
  source "$HOME/.cargo/env"
  rustup toolchain install "${RUST_TOOLCHAIN}"
  rustup default "${RUST_TOOLCHAIN}"
else
  echo "Found rustc: $(rustc --version)"
  # If rustc < 1.90, install the requested toolchain
  RUST_VERSION=$(rustc --version | sed -E 's/.* ([0-9]+)\.([0-9]+)\.([0-9]+).*/\1 \2/')
  RUST_MAJOR=$(echo "$RUST_VERSION" | awk '{print $1}')
  RUST_MINOR=$(echo "$RUST_VERSION" | awk '{print $2}')
  if [ "$RUST_MAJOR" -lt 1 ] || { [ "$RUST_MAJOR" -eq 1 ] && [ "$RUST_MINOR" -lt 90 ]; }; then
    echo "rustc is older than 1.90 — installing ${RUST_TOOLCHAIN} via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
    source "$HOME/.cargo/env"
    rustup toolchain install "${RUST_TOOLCHAIN}"
    rustup default "${RUST_TOOLCHAIN}"
  else
    echo "rustc is new enough"
  fi
fi

echo "Using $(rustc --version) and $(cargo --version)"

# Ensure cargo-tarpaulin is installed
if ! command -v cargo-tarpaulin >/dev/null 2>&1; then
  cargo install cargo-tarpaulin --version 0.23.0
fi

echo "cargo-tarpaulin version: $(cargo tarpaulin --version)"

# Run tarpaulin for selected crates and collect Cobertura XMLs
mkdir -p /work/artifacts/coverage/core
mkdir -p /work/artifacts/coverage/services-core-domain
mkdir -p /work/artifacts/coverage/core-application

set -x
cd /work/core
cargo tarpaulin --out Xml --output-dir /work/artifacts/coverage/core --timeout 1200 || true

cd /work/services/core-domain
cargo tarpaulin --out Xml --output-dir /work/artifacts/coverage/services-core-domain --timeout 1200 || true

cd /work/services/core-application
cargo tarpaulin --out Xml --output-dir /work/artifacts/coverage/core-application --timeout 1200 || true

echo "Done. Coverage artifacts are under /work/artifacts/coverage/"

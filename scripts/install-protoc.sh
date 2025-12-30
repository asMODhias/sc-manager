#!/usr/bin/env bash
set -euo pipefail

# Cross-platform helper to install protoc (Protocol Buffers compiler)
# Usage: PROTOC_VERSION=23.3 ./scripts/install-protoc.sh
# Set INSTALL_PREFIX to choose install location (default: $HOME/.local/bin)

PROTOC_VERSION=${PROTOC_VERSION:-23.3}
INSTALL_PREFIX=${INSTALL_PREFIX:-$HOME/.local}
BIN_DIR="$INSTALL_PREFIX/bin"
mkdir -p "$BIN_DIR"

echo "[install-protoc] installing protoc ${PROTOC_VERSION} to $BIN_DIR"

uname_s=$(uname -s)
case "$uname_s" in
  Darwin)
    if command -v brew >/dev/null 2>&1; then
      echo "[install-protoc] Homebrew detected — installing via brew"
      brew install protobuf || true
      if command -v protoc >/dev/null 2>&1; then
        echo "[install-protoc] protoc installed via brew"
        exit 0
      fi
    fi
    ;;
  Linux)
    # Try common package managers first
    if command -v apt-get >/dev/null 2>&1; then
      echo "[install-protoc] apt-get detected — trying apt-get (non-interactive)"
      if sudo -n true 2>/dev/null; then
        sudo apt-get update && sudo apt-get install -y protobuf-compiler || true
      elif [ "$(id -u)" -eq 0 ]; then
        apt-get update && apt-get install -y protobuf-compiler || true
      else
        echo "[install-protoc] sudo not available or requires password; skipping apt-get (will try download fallback)" >&2
      fi
      if command -v protoc >/dev/null 2>&1; then
        echo "[install-protoc] protoc installed via apt-get"
        exit 0
      fi
    fi
    if command -v yum >/dev/null 2>&1; then
      echo "[install-protoc] yum detected — trying yum (non-interactive)"
      if sudo -n true 2>/dev/null; then
        sudo yum install -y protobuf-compiler || true
      elif [ "$(id -u)" -eq 0 ]; then
        yum install -y protobuf-compiler || true
      else
        echo "[install-protoc] sudo not available or requires password; skipping yum (will try download fallback)" >&2
      fi
      if command -v protoc >/dev/null 2>&1; then
        echo "[install-protoc] protoc installed via yum"
        exit 0
      fi
    fi
    if command -v pacman >/dev/null 2>&1; then
      echo "[install-protoc] pacman detected — trying pacman (non-interactive)"
      if sudo -n true 2>/dev/null; then
        sudo pacman -S --noconfirm protobuf || true
      elif [ "$(id -u)" -eq 0 ]; then
        pacman -S --noconfirm protobuf || true
      else
        echo "[install-protoc] sudo not available or requires password; skipping pacman (will try download fallback)" >&2
      fi
      if command -v protoc >/dev/null 2>&1; then
        echo "[install-protoc] protoc installed via pacman"
        exit 0
      fi
    fi
    ;;
  *)
    echo "[install-protoc] Unknown platform: $uname_s — falling back to release download"
    ;;
esac

# Fallback: download prebuilt release archive from GitHub releases
OS_ARCH=""
case "$(uname -m)" in
  x86_64|amd64) ARCH="x86_64" ;;
  aarch64|arm64) ARCH="aarch_64" ;;
  *) ARCH="x86_64" ;;
esac
case "$uname_s" in
  Darwin) OS_TAG="osx" ;;
  Linux) OS_TAG="linux" ;;
  *) OS_TAG="linux" ;;
esac

RELEASE="protoc-${PROTOC_VERSION}-${OS_TAG}-x86_64.zip"
URL="https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/${RELEASE}"

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT
cd "$TMPDIR"

echo "[install-protoc] downloading $URL"
if command -v curl >/dev/null 2>&1; then
  curl -fsSL -O "$URL" || { echo "[install-protoc] download failed"; exit 1; }
elif command -v wget >/dev/null 2>&1; then
  wget -q "$URL" || { echo "[install-protoc] download failed"; exit 1; }
else
  echo "[install-protoc] wget/curl required to download release" >&2
  exit 1
fi

echo "[install-protoc] extracting archive"
unzip -q "$RELEASE" -d "$TMPDIR/extracted" || { echo "[install-protoc] unzip failed"; exit 1; }

# protoc binary is inside bin/; copy into BIN_DIR
if [ -f "$TMPDIR/extracted/bin/protoc" ]; then
  cp "$TMPDIR/extracted/bin/protoc" "$BIN_DIR/" || { echo "[install-protoc] copy failed"; exit 1; }
  chmod +x "$BIN_DIR/protoc"
  echo "[install-protoc] installed protoc to $BIN_DIR/protoc"
  echo "[install-protoc] ensure $BIN_DIR is in your PATH (e.g., export PATH=\"$BIN_DIR:\$PATH\")"
  exit 0
else
  echo "[install-protoc] expected protoc binary not found in archive" >&2
  exit 1
fi
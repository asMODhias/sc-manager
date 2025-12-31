#!/usr/bin/env bash
# Runs local CI and merges a PR via admin override when CI passes.
# Usage: ./scripts/auto-merge-local-ci.sh <pr-number>
set -euo pipefail
# Usage: ./scripts/auto-merge-local-ci.sh <pr-number> [--auto-install]
PR=${1:-28}
# Support optional --auto-install flag (either as second arg or flag anywhere)
AUTO_INSTALL=0
for arg in "$@"; do
  case "$arg" in
    -a|--auto-install)
      AUTO_INSTALL=1
      shift
      ;;
    *)
      shift
      ;;
  esac
done

SCRIPT_DIR=$(dirname "$0")
REPO_ROOT=$(cd "$SCRIPT_DIR/.." && pwd)
cd "$REPO_ROOT"

# Preflight checks
if ! command -v gh >/dev/null 2>&1; then
  echo "[auto-merge] gh CLI not found in PATH. Please install and authenticate gh before running this script." >&2
  exit 1
fi

if [ ! -f "$REPO_ROOT/scripts/run-local-ci.sh" ]; then
  echo "[auto-merge] run-local-ci.sh not found in scripts directory. Ensure you're running from repo root." >&2
  exit 1
fi

# protoc check (or PROTOC env var)
if [ -z "${PROTOC:-}" ]; then
  if ! command -v protoc >/dev/null 2>&1; then
    if [ "$AUTO_INSTALL" -eq 1 ]; then
      echo "[auto-merge] protoc not found — attempting automatic installation..." >&2
      if command -v brew >/dev/null 2>&1; then
        echo "[auto-merge] Installing protoc via brew..."
        brew install protobuf
      elif command -v apt-get >/dev/null 2>&1; then
        echo "[auto-merge] Installing protoc via apt-get (requires sudo)..."
        sudo apt-get update && sudo apt-get install -y protobuf-compiler
      elif command -v yum >/dev/null 2>&1; then
        echo "[auto-merge] Installing protoc via yum (requires sudo)..."
        sudo yum install -y protobuf-compiler
      elif command -v pacman >/dev/null 2>&1; then
        echo "[auto-merge] Installing protoc via pacman (requires sudo)..."
        sudo pacman -S --noconfirm protobuf
      else
        echo "[auto-merge] No supported package manager found for automatic installation. See docs/LOCAL_AUTO_MERGE.md" >&2
        exit 2
      fi
      # re-check
      if ! command -v protoc >/dev/null 2>&1; then
        echo "[auto-merge] protoc installation attempt failed. See docs/LOCAL_AUTO_MERGE.md for manual steps." >&2
        exit 2
      fi
    else
      echo "[auto-merge] protoc not found. Some crates require protoc to build (prost)." >&2
      echo "Either install protoc and add to PATH, or set PROTOC env var to the protoc binary path." >&2
      echo "Use --auto-install to attempt an automatic installation, or see docs/LOCAL_AUTO_MERGE.md for instructions." >&2
      exit 2
    fi
  fi
fi

echo "[auto-merge] Running local CI..."
"$SCRIPT_DIR/run-local-ci.sh"
EXIT=$?
if [ $EXIT -ne 0 ]; then
  echo "[auto-merge] Local CI failed with exit code $EXIT. Aborting merge." >&2
  exit $EXIT
fi

echo "[auto-merge] Local CI passed. Commenting and merging PR #$PR..."
gh pr comment $PR --body "Auto-merge: Local CI passed; merging via admin override as per local-only CI policy."
gh pr merge $PR --merge --delete-branch --admin || { echo "[auto-merge] Merge command failed" >&2; exit 2; }

echo "[auto-merge] PR #$PR merged and branch deleted."
#!/usr/bin/env bash
set -euo pipefail

# Run Local CI workflow using act (https://github.com/nektos/act)
# Falls act nicht installiert ist, schlägt das Script mit einer klaren Meldung fehl.

if ! command -v act >/dev/null 2>&1; then
  echo "ERROR: 'act' is not installed. Install from https://github.com/nektos/act or run 'brew install act' / 'choco install act' depending on your OS." >&2
  exit 2
fi

# Default runner image mapping - use a fuller ubuntu image for better parity
DEFAULT_IMAGE="ghcr.io/catthehacker/ubuntu:full-20.04"

# Use the Local CI job name from .github/workflows/local-ci.yml
JOB_NAME="local-ci"

echo "==> Running GitHub Actions workflow job '$JOB_NAME' locally using act"
# Map the runner image and run the job
act -j "$JOB_NAME" -P ubuntu-latest=$DEFAULT_IMAGE --env CI=true "$@"

exit 0

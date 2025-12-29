#!/usr/bin/env bash
set -euo pipefail

# Allowed files (relative to repo root)
ALLOWED=(
  "docs/SC_MANAGER_V7_PART1_FOUNDATION.md"
  "docs/SC_MANAGER_V7_FEATURE_MATRIX_AND_COMPLETION.md"
  "docs/README.md"
  "docs/index.md"
)

# Find any files under docs/ (regular files) excluding allowed list
mapfile -t FOUND < <(git ls-files docs | sed -e 's|\r$||')

BAD=()
for f in "${FOUND[@]}"; do
  skip=false
  for a in "${ALLOWED[@]}"; do
    if [[ "$f" == "$a" ]]; then
      skip=true
      break
    fi
  done
  if ! $skip; then
    BAD+=("$f")
  fi
done

if [ ${#BAD[@]} -ne 0 ]; then
  echo "ERROR: Unexpected documentation files found under docs/:"
  for b in "${BAD[@]}"; do
    echo "  - $b"
  done
  echo "Allowed files are:"
  for a in "${ALLOWED[@]}"; do
    echo "  - $a"
  done
  exit 1
fi

echo "Docs validation passed: only allowed V7 files present."
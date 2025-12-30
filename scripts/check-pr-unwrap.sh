#!/usr/bin/env bash
set -euo pipefail
BASE=master
FILES=$(git diff --name-only origin/${BASE}...HEAD || true)
# Exclude test files and markdown/docs
FILES=$(echo "$FILES" | grep -Ev '(^tests/|/tests/|\.md$)' || true)
if [ -z "$FILES" ]; then
  echo "No changed non-test files detected."
  exit 0
fi

echo "Checking files:"
echo "$FILES"

git grep -n -E '(\bunwrap\(|panic!\()' -- $FILES || true

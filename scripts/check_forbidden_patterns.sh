#!/usr/bin/env bash
set -euo pipefail

# Search for forbidden patterns in repo source files (Rust & TS/JS)
# Exclude tests and generated folders

# Patterns to check (expect/unwrap/panic are only forbidden in non-test files)
PATTERNS=("\.unwrap\s*\(" "\.expect\s*\(" "panic!\s*\(")
TODO_PATTERN="TODO\b"

# Find files to scan
mapfile -t FILES < <(git ls-files "**/*.rs" "**/*.ts" "**/*.tsx" "**/*.js" "**/*.jsx" | grep -v "node_modules" || true)

EXIT=0
for f in "${FILES[@]}"; do
  # Always check TODO everywhere
  if grep -nE -- "$TODO_PATTERN" "$f" >/dev/null 2>&1; then
    echo "TODO found in $f:" >&2
    grep -nE -- "$TODO_PATTERN" "$f" >&2
    EXIT=1
  fi

  # Determine if file contains test code (skip unwrap/expect/panic in that case)
  if grep -q "#\[cfg(test)\]" "$f" || grep -q "mod tests" "$f" || grep -q "#\[test\]" "$f" || grep -q "fn test_" "$f" || echo "$f" | grep -q "tests"; then
    # Skip unwrap/expect/panic checks for test-containing files and files with inline tests/doc examples
    continue
  fi

  # Skip vendor/patches directories for now (they contain upstream code modifications)
  if echo "$f" | grep -q "/patches/"; then
    continue
  fi

  for p in "${PATTERNS[@]}"; do
    if grep -nE -- "$p" "$f" >/dev/null 2>&1; then
      echo "Forbidden pattern '$p' found in $f:" >&2
      grep -nE -- "$p" "$f" >&2
      EXIT=1
    fi
  done
done

if [ $EXIT -ne 0 ]; then
  echo "Forbidden patterns detected. Fix before merging." >&2
  exit 1
fi

echo "No forbidden patterns found (excluding tests)."
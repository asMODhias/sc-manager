#!/usr/bin/env bash
set -euo pipefail

mkdir -p artifacts/coverage/lcov artifacts/coverage/html

# Collect per-crate lcov files
for f in profraw/*.profraw; do
  # tarpaulin generates lcov per crate; skip profraws here (if any)
  :
done

# Merge lcov files produced by tarpaulin
LCOV_OUT=artifacts/coverage/lcov/merged.info
# start with empty
> "$LCOV_OUT"
for f in artifacts/coverage/lcov/*.info; do
  echo "Adding $f"
  lcov -a "$LCOV_OUT" -a "$f" -o "$LCOV_OUT" || lcov -a "$f" -o "$LCOV_OUT"
done

# Generate HTML
if [ -s "$LCOV_OUT" ]; then
  genhtml -o artifacts/coverage/html "$LCOV_OUT"
  echo "HTML coverage generated in artifacts/coverage/html"
else
  echo "No lcov info found to generate HTML"
fi

# Also archive per-crate xml outputs if present
exit 0
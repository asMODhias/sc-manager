#!/usr/bin/env bash
set -euo pipefail

mkdir -p artifacts/coverage/lcov artifacts/coverage/html artifacts/coverage/tarpaulin

# Collect per-crate lcov files
for f in profraw/*.profraw; do
  # tarpaulin generates lcov per crate; skip profraws here (if any)
  :
done

# Move any tarpaulin lcov/info files into artifacts folder
find . -type f \( -name "*.lcov" -o -name "*.info" \) -exec mv -t artifacts/coverage/lcov/ --backup=numbered {} + || true

LCOV_OUT=artifacts/coverage/tarpaulin/merged.lcov
# start with first file
FIRST=1
for f in artifacts/coverage/lcov/*.{lcov,info}; do
  [ -e "$f" ] || continue
  if [ $FIRST -eq 1 ]; then
    cp "$f" "$LCOV_OUT"
    FIRST=0
  else
    lcov -a "$LCOV_OUT" -a "$f" -o "$LCOV_OUT.tmp" && mv "$LCOV_OUT.tmp" "$LCOV_OUT" || true
  fi
done

# Generate HTML
if [ -s "$LCOV_OUT" ]; then
  genhtml -o artifacts/coverage/tarpaulin/html "$LCOV_OUT" || true
  echo "HTML coverage generated in artifacts/coverage/tarpaulin/html"
else
  echo "No lcov files found to generate HTML"
fi

# Also archive per-crate xml outputs if present
exit 0
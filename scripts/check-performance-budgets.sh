#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(git rev-parse --show-toplevel)
cd "$ROOT_DIR"

FILE=docs/performance-budgets.yml
if [ ! -f "$FILE" ]; then
  echo "[ERROR] Missing performance budgets file: $FILE"
  exit 2
fi

echo "[OK] Found: $FILE"

# Basic content checks
missing=0
for key in "api_gateway:" "desktop_ui:" "event_bus:"; do
  if ! grep -q "^$key" "$FILE"; then
    echo "[ERROR] Missing key in $FILE: $key"
    missing=1
  fi
done

if [ $missing -ne 0 ]; then
  echo "\n[FAIL] Performance budget file incomplete"
  exit 2
fi

echo "\n[PASS] Performance budget file present and contains required sections."

echo "Note: This check validates presence only; run benchmarks and monitoring to validate actual performance."
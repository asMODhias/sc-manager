#!/usr/bin/env bash
set -euo pipefail

MANIFEST="assets/fankit/fankit-manifest.json"
LICENSE="assets/fankit/LICENSE_CIG_FANKIT.txt"

if [ ! -f "$MANIFEST" ]; then
  echo "ERROR: Manifest not found: $MANIFEST"; exit 2
fi
if [ ! -f "$LICENSE" ]; then
  echo "ERROR: License not found: $LICENSE"; exit 3
fi

# check each manifest entry
fail=0
jq -c '.[]' "$MANIFEST" | while read -r entry; do
  path=$(echo "$entry" | jq -r .path)
  sha=$(echo "$entry" | jq -r .sha256)
  # normalize path separators from manifest (windows style) to unix style for shell
  norm_path=$(echo "$path" | tr '\\' '/')
  file="assets/fankit/$norm_path"
  if [ ! -f "$file" ]; then
    # fallback: try using manifest path as-is (windows separators) in case of different environment
    file_win="assets/fankit/$path"
    if [ ! -f "$file_win" ]; then
      echo "MISSING: assets/fankit/$path"; fail=1; continue
    else
      file="$file_win"
    fi
  fi
  cursha=$(sha256sum "$file" | awk '{print $1}' | tr '[:upper:]' '[:lower:]')
  sha_lc=$(echo "$sha" | tr '[:upper:]' '[:lower:]')
  if [ "$cursha" != "$sha_lc" ]; then
    echo "SHA MISMATCH: $file (manifest: $sha, current: $cursha)"; fail=1
  fi
done

if [ "$fail" -eq 1 ]; then
  echo "Manifest validation failed"; exit 4
fi

echo "Manifest OK and license present"
exit 0

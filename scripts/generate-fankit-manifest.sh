#!/usr/bin/env bash
set -euo pipefail

DEST_DIR="assets/fankit"
MANIFEST="$DEST_DIR/fankit-manifest.json"

if [ ! -d "$DEST_DIR" ]; then
  echo "Error: $DEST_DIR does not exist"; exit 2
fi

# find license
LICENSE=$(find "$DEST_DIR" -maxdepth 2 -type f -iname "*license*" -o -iname "*tos*" | head -n1 || true)
if [ -z "$LICENSE" ]; then
  echo "WARNING: License/TOS not found inside $DEST_DIR"
else
  echo "Found license: $LICENSE"
fi

INCLUDE_EXTS="png|jpg|jpeg|svg|webp|gif|mp3|ogg|wav|txt|md|json|yml|yaml|pdf"

echo "[" > "$MANIFEST" 
first=true

# Walk files and include allowed extensions, excluding the manifest itself
find "$DEST_DIR" -type f ! -path "$MANIFEST" | while read -r f; do
  rel=$(realpath --relative-to="$DEST_DIR" "$f")
  # include LICENSE and README as metadata but avoid adding big binaries not allowed
  if [[ "$rel" =~ fankit-manifest.json$ ]]; then
    continue
  fi
  if [[ "$rel" =~ README.md$ ]] || [[ "$rel" =~ LICENSE ]]; then
    include=true
  else
    if [[ "$rel" =~ \.($INCLUDE_EXTS)$ ]]; then
      include=true
    else
      include=false
    fi
  fi
  if [ "$include" = true ]; then
    dest_path="$DEST_DIR/$rel"
    sha=$(sha256sum "$dest_path" | awk '{print $1}')
    size=$(stat -c%s "$dest_path")
    imported_at=$(date --iso-8601=seconds)
    obj=$(python - <<PY
import json
print(json.dumps({
  "path": "$rel",
  "size": $size,
  "sha256": "$sha",
  "source": "local-import",
  "imported_at": "$imported_at"
}))
PY
)
    if [ "$first" = true ]; then
      first=false
      echo "$obj" >> "$MANIFEST"
    else
      echo "," >> "$MANIFEST"
      echo "$obj" >> "$MANIFEST"
    fi
  fi
done

echo "]" >> "$MANIFEST"

echo "Manifest generated at $MANIFEST"

# add changelog entry
now=$(date --iso-8601=seconds)
num_files=$(jq '. | length' "$MANIFEST")
cat >> "docs/CIG-Fankit.md" <<EOF

## Manifest generated: $now
- Quelle: local-import (added directly to repo under assets/fankit)
- Dateianzahl aufgeführt im Manifest: $num_files
- Lizenz: ${LICENSE:-not-found}
EOF

exit 0

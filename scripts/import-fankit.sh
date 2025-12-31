#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 1 ]; then
  echo "Usage: $0 /path/to/Fankit_Source"
  exit 2
fi

SRC="$1"
DEST="assets/fankit"
INCLUDE_EXTS="png|jpg|jpeg|svg|webp|gif|mp3|ogg|wav|txt|md|json|yml|yaml"

if [ ! -d "$SRC" ]; then
  echo "Source path does not exist: $SRC"
  exit 2
fi

# Find LICENSE or TOS file
TOS_FILE=$(find "$SRC" -maxdepth 3 -type f -iregex ".*\(license\|tos\).*" | head -n1 || true)
if [ -z "$TOS_FILE" ]; then
  echo "ERROR: LICENSE/TOS not found in source folder. Place the license/TOS in the source before running the import."
  exit 3
fi

mkdir -p "$DEST"
cp "$TOS_FILE" "$DEST/LICENSE_CIG_FANKIT.txt"

# Temporary manifest build
MANIFEST_TMP="${DEST}/fankit-manifest.tmp"
MANIFEST_JSON="${DEST}/fankit-manifest.json"
rm -f "$MANIFEST_TMP" "$MANIFEST_JSON"

echo "[" > "$MANIFEST_TMP"
first=true

# Copy allowed files and build manifest entries
find "$SRC" -type f | while read -r f; do
  if [[ "$f" =~ \.($INCLUDE_EXTS)$ ]]; then
    rel_path=$(realpath --relative-to="$SRC" "$f")
    dest_path="$DEST/$rel_path"
    mkdir -p "$(dirname "$dest_path")"
    cp -p "$f" "$dest_path"
    sha=$(sha256sum "$dest_path" | awk '{print $1}')
    size=$(stat -c%s "$dest_path")
    imported_at=$(date --iso-8601=seconds)

    # produce JSON object via python to be safe about escaping
    obj=$(python - <<PY
import json,sys
print(json.dumps({
  "path": "$rel_path",
  "size": $size,
  "sha256": "$sha",
  "source": "$SRC",
  "imported_at": "$imported_at"
}))
PY
)
    if [ "$first" = true ]; then
      first=false
      echo "$obj" >> "$MANIFEST_TMP"
    else
      echo "," >> "$MANIFEST_TMP"
      echo "$obj" >> "$MANIFEST_TMP"
    fi
  fi
done

echo "]" >> "$MANIFEST_TMP"
mv "$MANIFEST_TMP" "$MANIFEST_JSON"

# Append changelog entry to docs
CHANGELOG_DOC="docs/CIG-Fankit.md"
now=$(date --iso-8601=seconds)
num_files=$(jq '. | length' "$MANIFEST_JSON")
cat >> "$CHANGELOG_DOC" <<EOF

## Import: $now
- Quelle: $SRC
- Dateianzahl importiert: $num_files
- Lizenz: `assets/fankit/LICENSE_CIG_FANKIT.txt`
EOF


echo "Imported $num_files file(s) to $DEST"
echo "Manifest written to $MANIFEST_JSON"
echo "License copied to $DEST/LICENSE_CIG_FANKIT.txt"

echo "Done. Please review and commit: git add $DEST && git commit -m 'Add CIG Fankit assets (import) + manifest + LICENSE'"

exit 0

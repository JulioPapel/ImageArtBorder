#!/usr/bin/env bash
# Batch border for a folder (Capture One exports, etc.)
# Usage: ./border-exported-images.sh /path/to/folder
set -euo pipefail

TOOL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXE="$TOOL_DIR/ImageArtBorder"
FOLDER="${1:-}"
BORDER="${BORDER:-6}"
COLOR="${COLOR:-#FFFFFF}"

if [[ -z "$FOLDER" || ! -d "$FOLDER" ]]; then
    echo "Usage: $0 <folder>" >&2
    exit 1
fi

shopt -s nullglob nocaseglob
count=0
while IFS= read -r -d '' file; do
    echo "Processing: $file"
    "$EXE" -b "$BORDER" -c "$COLOR" -f "$file"
    ((count++)) || true
done < <(find "$FOLDER" -type f \( \
    -iname '*.jpg' -o -iname '*.jpeg' -o \
    -iname '*.png' -o -iname '*.tif' -o -iname '*.tiff' \
    \) -print0)

echo "Done. $count file(s) processed."

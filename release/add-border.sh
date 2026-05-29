#!/usr/bin/env bash
# Add border to one image file.
# Usage: ./add-border.sh /path/to/image.jpg
set -euo pipefail

TOOL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXE="$TOOL_DIR/ImageArtBorder"
BORDER="${BORDER:-6}"
COLOR="${COLOR:-#FFFFFF}"

if [[ $# -lt 1 ]]; then
    echo "Usage: $0 <image-path>" >&2
    exit 1
fi

exec "$EXE" -b "$BORDER" -c "$COLOR" -f "$1"

#!/usr/bin/env bash
# Shared packaging logic for macOS and Linux release folders.
# Author: Júlio Papel <info@juliopapel.pt>
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RELEASE_DIR="$ROOT/release"
VERSION="$(grep '^version = ' "$ROOT/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')"

usage() {
    echo "Usage: $0 <platform-tag> <binary-path>"
    echo "  platform-tag examples: linux-x64, macos-arm64, macos-x64"
    exit 1
}

[[ $# -eq 2 ]] || usage
PLATFORM="$1"
BINARY_SRC="$2"
OUT_DIR="$RELEASE_DIR/dist/ImageArtBorder-${VERSION}-${PLATFORM}"

if [[ ! -f "$BINARY_SRC" ]]; then
    echo "Binary not found: $BINARY_SRC" >&2
    exit 1
fi

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR/docs"

cp "$BINARY_SRC" "$OUT_DIR/ImageArtBorder"
chmod +x "$OUT_DIR/ImageArtBorder"
echo -n "$VERSION" > "$OUT_DIR/VERSION"
cp "$ROOT/LICENSE" "$OUT_DIR/"
cp "$RELEASE_DIR/README-unix.txt" "$OUT_DIR/README.txt"
cp "$ROOT/docs/"*.md "$OUT_DIR/docs/"
cp "$RELEASE_DIR/install.sh" "$OUT_DIR/"
cp "$RELEASE_DIR/uninstall.sh" "$OUT_DIR/"
cp "$RELEASE_DIR/add-border.sh" "$OUT_DIR/"
cp "$RELEASE_DIR/border-exported-images.sh" "$OUT_DIR/"
chmod +x "$OUT_DIR/"*.sh

echo "Release ready: $OUT_DIR"
ls -la "$OUT_DIR"

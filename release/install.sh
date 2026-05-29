#!/usr/bin/env bash
# Install ImageArtBorder on macOS or Linux.
# Author: Júlio Papel <info@juliopapel.pt>
set -euo pipefail

INSTALL_DIR="${1:-${HOME}/.local/bin}"
SOURCE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP_DIR="${HOME}/.local/share/ImageArtBorder"

if [[ ! -f "$SOURCE_DIR/ImageArtBorder" ]]; then
    echo "ImageArtBorder binary not found in $SOURCE_DIR" >&2
    exit 1
fi

mkdir -p "$APP_DIR" "$INSTALL_DIR"
rm -rf "$APP_DIR"/*
cp -R "$SOURCE_DIR"/* "$APP_DIR/"
chmod +x "$APP_DIR/ImageArtBorder" "$APP_DIR"/*.sh 2>/dev/null || true

ln -sf "$APP_DIR/ImageArtBorder" "$INSTALL_DIR/ImageArtBorder"

echo "Installed ImageArtBorder $(cat "$APP_DIR/VERSION" 2>/dev/null || echo unknown)"
echo "  Binary:  $INSTALL_DIR/ImageArtBorder"
echo "  Support: $APP_DIR"
echo ""
echo "Ensure $INSTALL_DIR is on your PATH."
echo "Verify: ImageArtBorder --version"

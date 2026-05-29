#!/usr/bin/env bash
set -euo pipefail

INSTALL_DIR="${HOME}/.local/bin"
APP_DIR="${HOME}/.local/share/ImageArtBorder"

rm -f "$INSTALL_DIR/ImageArtBorder"
rm -rf "$APP_DIR"
echo "Removed ImageArtBorder from $INSTALL_DIR and $APP_DIR"

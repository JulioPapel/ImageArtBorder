#!/usr/bin/env bash
set -euo pipefail
ROOT="/build"
bash "$ROOT/scripts/common-pack.sh" "macos-arm64" \
    "$ROOT/target/aarch64-apple-darwin/release/ImageArtBorder"
bash "$ROOT/scripts/common-pack.sh" "macos-x64" \
    "$ROOT/target/x86_64-apple-darwin/release/ImageArtBorder"

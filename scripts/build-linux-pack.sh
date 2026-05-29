#!/usr/bin/env bash
# Pack step used inside Dockerfile.linux-build (binary already at target/release/).
set -euo pipefail
ROOT="/build"
bash "$ROOT/scripts/common-pack.sh" "linux-x64" "$ROOT/target/release/ImageArtBorder"

#!/usr/bin/env bash
# Build ImageArtBorder for Linux x86_64 (native or cross).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

export CARGO_HTTP_CHECK_REVOCATION=false

TARGET="${TARGET:-x86_64-unknown-linux-gnu}"
echo "=== Building ImageArtBorder for $TARGET ==="

if ! rustup target list --installed | grep -q "^${TARGET}$"; then
    rustup target add "$TARGET"
fi

cargo build --release --target "$TARGET"
BINARY="$ROOT/target/$TARGET/release/ImageArtBorder"

if [[ ! -f "$BINARY" ]]; then
    echo "Build failed: $BINARY not found" >&2
    exit 1
fi

"$BINARY" --version
bash "$ROOT/scripts/common-pack.sh" "linux-x64" "$BINARY"

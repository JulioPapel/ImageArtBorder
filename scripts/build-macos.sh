#!/usr/bin/env bash
# Build ImageArtBorder for macOS (native host arch).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

export CARGO_HTTP_CHECK_REVOCATION=false

ARCH="$(uname -m)"
case "$ARCH" in
    arm64) PLATFORM="macos-arm64" ;;
    x86_64) PLATFORM="macos-x64" ;;
    *) echo "Unsupported macOS arch: $ARCH" >&2; exit 1 ;;
esac

echo "=== Building ImageArtBorder for $PLATFORM ($ARCH) ==="
cargo build --release
BINARY="$ROOT/target/release/ImageArtBorder"
"$BINARY" --version
bash "$ROOT/scripts/common-pack.sh" "$PLATFORM" "$BINARY"

# Optional: build the other macOS arch via cross-target when on Apple Silicon / Intel
OTHER_TARGET=""
OTHER_PLATFORM=""
if [[ "$ARCH" == "arm64" ]]; then
    OTHER_TARGET="x86_64-apple-darwin"
    OTHER_PLATFORM="macos-x64"
elif [[ "$ARCH" == "x86_64" ]]; then
    OTHER_TARGET="aarch64-apple-darwin"
    OTHER_PLATFORM="macos-arm64"
fi

if [[ -n "$OTHER_TARGET" ]]; then
    if rustup target list --installed | grep -q "^${OTHER_TARGET}$" || rustup target add "$OTHER_TARGET"; then
        echo "=== Cross-building $OTHER_PLATFORM ==="
        cargo build --release --target "$OTHER_TARGET"
        bash "$ROOT/scripts/common-pack.sh" "$OTHER_PLATFORM" \
            "$ROOT/target/$OTHER_TARGET/release/ImageArtBorder"
    fi
fi

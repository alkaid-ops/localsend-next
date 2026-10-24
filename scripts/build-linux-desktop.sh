#!/bin/sh
set -eu

ROOT=$(cd "$(dirname "$0")/.." && pwd)
ARCH=${1:-$(uname -m)}
GUI_FEATURES=${LOCALSEND_GUI_FEATURES:-backend-femtovg-wgpu}

case "$ARCH" in
    x86_64)
        TARGET=x86_64-unknown-linux-gnu
        ;;
    aarch64)
        TARGET=aarch64-unknown-linux-gnu
        ;;
    *)
        printf 'usage: %s x86_64|aarch64\n' "$0" >&2
        exit 64
        ;;
esac

RELEASE="$ROOT/target/$TARGET/release"

command -v cargo >/dev/null 2>&1 || { echo 'cargo is required' >&2; exit 1; }

cargo build --locked --release --target "$TARGET" \
    -p localsendd -p localsend-tray -p localsend-launcher -p localsend-tui
cargo build --locked --release --target "$TARGET" \
    -p localsend-gui-slint --no-default-features --features "$GUI_FEATURES"

for binary in localsend-gui-slint localsendd localsend-tray localsend-launcher localsend-tui; do
    if [ ! -x "$RELEASE/$binary" ]; then
        echo "missing build output: $RELEASE/$binary" >&2
        exit 1
    fi
done

printf 'built Linux %s binaries in %s\n' "$ARCH" "$RELEASE"

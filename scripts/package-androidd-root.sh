#!/bin/sh
set -eu

ROOT=$(unset CDPATH; cd -- "$(dirname "$0")/.." && pwd)
ARCH=${1:-aarch64}

case "$ARCH" in
    aarch64|arm64)
        TARGET=aarch64-linux-android
        NDK_TRIPLE=aarch64-linux-android
        EXPECTED_MACHINE='AArch64'
        OUTPUT_ARCH=aarch64
        ;;
    *)
        printf 'usage: %s [aarch64]\n' "$0" >&2
        exit 64
        ;;
esac

if [ -z "${ANDROID_NDK_ROOT:-}" ]; then
    printf 'BLOCKED: ANDROID_NDK_ROOT is not set\n' >&2
    exit 2
fi
if [ ! -d "$ANDROID_NDK_ROOT" ]; then
    printf 'BLOCKED: ANDROID_NDK_ROOT does not exist: %s\n' "$ANDROID_NDK_ROOT" >&2
    exit 2
fi
if ! rustup target list --installed | grep -qx "$TARGET"; then
    printf 'BLOCKED: Rust target is not installed: %s\n' "$TARGET" >&2
    exit 2
fi
if ! command -v zip >/dev/null 2>&1; then
    printf 'BLOCKED: zip is required for module packaging\n' >&2
    exit 2
fi

HOST_TAG=
for candidate in darwin-x86_64 darwin-arm64 linux-x86_64; do
    if [ -d "$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/$candidate" ]; then
        HOST_TAG=$candidate
        break
    fi
done
if [ -z "$HOST_TAG" ]; then
    printf 'BLOCKED: no supported NDK LLVM prebuilt found in %s\n' "$ANDROID_NDK_ROOT" >&2
    exit 2
fi

API_LEVEL=${ANDROID_API_LEVEL:-29}
TOOLCHAIN="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/$HOST_TAG"
CC_BIN="$TOOLCHAIN/bin/${NDK_TRIPLE}${API_LEVEL}-clang"
AR_BIN="$TOOLCHAIN/bin/llvm-ar"
READELF_BIN="$TOOLCHAIN/bin/llvm-readelf"

for tool in "$CC_BIN" "$AR_BIN" "$READELF_BIN"; do
    if [ ! -x "$tool" ]; then
        printf 'BLOCKED: required NDK tool is missing: %s\n' "$tool" >&2
        exit 2
    fi
done

export CC_aarch64_linux_android="$CC_BIN"
export AR_aarch64_linux_android="$AR_BIN"
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$CC_BIN"

RELEASE="$ROOT/target/$TARGET/release"
PACKAGES="$ROOT/target/release/packages"
STAGE="$PACKAGES/.localsend-next-root-$OUTPUT_ARCH"
ZIP="$PACKAGES/localsend-next-root-$OUTPUT_ARCH.zip"
BIN="$RELEASE/localsend-androidd"

mkdir -p "$PACKAGES"
rm -rf "$STAGE"

cargo build --locked --release --target "$TARGET" -p localsend-androidd

if [ ! -x "$BIN" ]; then
    printf 'build did not produce %s\n' "$BIN" >&2
    exit 1
fi
if ! "$READELF_BIN" -h "$BIN" | grep -q "Machine:.*$EXPECTED_MACHINE"; then
    printf 'binary architecture mismatch: %s\n' "$BIN" >&2
    exit 1
fi

mkdir -p "$STAGE/bin" "$STAGE/config"
cp "$ROOT/packaging/android-root/module.prop" "$STAGE/module.prop"
cp "$ROOT/packaging/android-root/post-fs-data.sh" "$STAGE/post-fs-data.sh"
cp "$ROOT/packaging/android-root/service.sh" "$STAGE/service.sh"
cp "$ROOT/packaging/android-root/uninstall.sh" "$STAGE/uninstall.sh"
cp "$ROOT/packaging/android-root/README.zh-CN.md" "$STAGE/README.zh-CN.md"
cp "$ROOT/packaging/android-root/config/settings.json" "$STAGE/config/settings.json"
cp "$BIN" "$STAGE/bin/localsend-androidd"
chmod 0755 "$STAGE/post-fs-data.sh" "$STAGE/service.sh" "$STAGE/uninstall.sh" "$STAGE/bin/localsend-androidd"

for file in module.prop post-fs-data.sh service.sh uninstall.sh config/settings.json bin/localsend-androidd; do
    if [ ! -e "$STAGE/$file" ]; then
        printf 'module is missing %s\n' "$file" >&2
        exit 1
    fi
done

rm -f "$ZIP"
(cd "$STAGE" && zip -X -q -r "$ZIP" .)

FINAL_BIN="$STAGE/bin/localsend-androidd"
if ! "$READELF_BIN" -h "$FINAL_BIN" >/dev/null; then
    printf 'packaged binary is unreadable\n' >&2
    exit 1
fi

printf '%s\n' "$ZIP"
if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$ZIP"
elif command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$ZIP"
else
    printf 'sha256 checksum tool is unavailable\n' >&2
    exit 1
fi


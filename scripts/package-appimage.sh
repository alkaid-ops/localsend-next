#!/bin/sh
set -eu

ROOT=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
ARCH=${1:-$(uname -m)}

case "$ARCH" in
    x86_64|aarch64) ;;
    *)
        printf 'usage: %s x86_64|aarch64\n' "$0" >&2
        exit 64
        ;;
esac

TARGET="$ARCH-unknown-linux-gnu"
RELEASE="$ROOT/target/$TARGET/release"
PACKAGES="$ROOT/target/release/packages"
APPDIR="$PACKAGES/LocalSend-linux-$ARCH.AppDir"
OUTPUT="$PACKAGES/LocalSend-linux-$ARCH.AppImage"
TOOLS="$ROOT/target/release/tools"
APPIMAGETOOL=${APPIMAGETOOL:-"$TOOLS/appimagetool-$ARCH.AppImage"}
RUNTIME=${APPIMAGE_RUNTIME_FILE:-"$TOOLS/runtime-$ARCH"}
RUNTIME_URL=${APPIMAGE_RUNTIME_URL:-"https://github.com/AppImage/type2-runtime/releases/download/continuous/runtime-$ARCH"}
DESKTOP="$ROOT/apps/localsend-gui-slint/packaging/linux/org.localsend.pc.desktop"
METAINFO="$ROOT/apps/localsend-gui-slint/packaging/linux/org.localsend.pc.metainfo.xml"
ICON="$ROOT/apps/localsend-gui-slint/packaging/macos/app-icon.svg"

command -v cargo >/dev/null 2>&1 || { echo 'cargo is required' >&2; exit 1; }
command -v curl >/dev/null 2>&1 || { echo 'curl is required' >&2; exit 1; }

cargo build --locked --release --target "$TARGET" \
    -p localsendd -p localsend-tray -p localsend-launcher -p localsend-tui
cargo build --locked --release --target "$TARGET" \
    -p localsend-gui-slint --no-default-features --features backend-femtovg-wgpu

for binary in localsend-gui-slint localsendd localsend-tray localsend-launcher localsend-tui; do
    [ -x "$RELEASE/$binary" ] || { echo "missing build output: $RELEASE/$binary" >&2; exit 1; }
done

if [ ! -x "$APPIMAGETOOL" ]; then
    mkdir -p "$TOOLS"
    curl -L --fail --retry 3 --connect-timeout 20 \
        -o "$APPIMAGETOOL" \
        "https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-$ARCH.AppImage"
    chmod +x "$APPIMAGETOOL"
fi

if [ ! -f "$RUNTIME" ]; then
    mkdir -p "$TOOLS"
    curl -L --fail --retry 3 --connect-timeout 20 \
        -o "$RUNTIME.part" "$RUNTIME_URL"
    mv "$RUNTIME.part" "$RUNTIME"
fi

rm -rf "$APPDIR"
mkdir -p \
    "$APPDIR/usr/bin" \
    "$APPDIR/usr/share/applications" \
    "$APPDIR/usr/share/icons/hicolor/scalable/apps" \
    "$APPDIR/usr/share/metainfo"

install -m 0755 "$RELEASE/localsend-gui-slint" "$APPDIR/usr/bin/localsend"
install -m 0755 "$RELEASE/localsendd" "$APPDIR/usr/bin/localsendd"
install -m 0755 "$RELEASE/localsend-tray" "$APPDIR/usr/bin/localsend-tray"
install -m 0755 "$RELEASE/localsend-launcher" "$APPDIR/usr/bin/localsend-launcher"
install -m 0755 "$RELEASE/localsend-tui" "$APPDIR/usr/bin/localsend-tui"
install -m 0644 "$DESKTOP" "$APPDIR/usr/share/applications/org.localsend.pc.desktop"
install -m 0644 "$METAINFO" "$APPDIR/usr/share/metainfo/org.localsend.pc.metainfo.xml"
install -m 0644 "$ICON" "$APPDIR/usr/share/icons/hicolor/scalable/apps/org.localsend.pc.svg"
install -m 0644 "$ROOT/LICENSE" "$APPDIR/LICENSE"
install -m 0644 "$ROOT/THIRD_PARTY_NOTICES.md" "$APPDIR/THIRD_PARTY_NOTICES.md"

cat >"$APPDIR/AppRun" <<'EOF'
#!/bin/sh
set -eu
HERE=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
export PATH="$HERE/usr/bin:$PATH"
if [ -d "$HERE/usr/lib" ]; then
    export LD_LIBRARY_PATH="$HERE/usr/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
fi
exec "$HERE/usr/bin/localsend-launcher" "$@"
EOF
chmod +x "$APPDIR/AppRun"
ln -s usr/share/applications/org.localsend.pc.desktop "$APPDIR/org.localsend.pc.desktop"
ln -s usr/share/icons/hicolor/scalable/apps/org.localsend.pc.svg "$APPDIR/org.localsend.pc.svg"
ln -s org.localsend.pc.svg "$APPDIR/.DirIcon"

if [ -e "$OUTPUT" ]; then
    unlink "$OUTPUT"
fi
ARCH="$ARCH" APPIMAGE_EXTRACT_AND_RUN=1 \
    "$APPIMAGETOOL" --runtime-file "$RUNTIME" "$APPDIR" "$OUTPUT"
test -x "$OUTPUT"
printf '%s\n' "$OUTPUT"

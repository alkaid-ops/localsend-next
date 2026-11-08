#!/bin/sh
set -eu

ROOT=$(cd "$(dirname "$0")/.." && pwd)
ARCH=${1:-$(uname -m)}
VERSION=${PACKAGE_VERSION:-0.1.0}
VERSION=${VERSION#v}

case "$VERSION" in
    ''|*[!A-Za-z0-9._~-]*)
        printf 'invalid PACKAGE_VERSION: %s\n' "$VERSION" >&2
        exit 64
        ;;
esac

case "$ARCH" in
    x86_64) TARGET="x86_64-unknown-linux-gnu"; ASSET_ARCH=x86_64 ;;
    aarch64|arm64) TARGET="aarch64-unknown-linux-gnu"; ASSET_ARCH=aarch64 ;;
    *)
        printf 'usage: %s x86_64|aarch64\n' "$0" >&2
        exit 64
        ;;
esac

RELEASE="$ROOT/target/$TARGET/release"
PACKAGES="$ROOT/target/release/packages"
APPDIR="$PACKAGES/LocalSend-${VERSION}-linux-$ASSET_ARCH.AppDir"
OUTPUT="$PACKAGES/LocalSend-${VERSION}-linux-$ASSET_ARCH.AppImage"
TOOLS="$ROOT/target/release/tools"
APPIMAGETOOL=${APPIMAGETOOL:-"$TOOLS/appimagetool-$ASSET_ARCH.AppImage"}
RUNTIME=${APPIMAGE_RUNTIME_FILE:-"$TOOLS/runtime-$ASSET_ARCH"}
RUNTIME_URL=${APPIMAGE_RUNTIME_URL:-"https://github.com/AppImage/type2-runtime/releases/download/continuous/runtime-$ASSET_ARCH"}
DESKTOP="$ROOT/apps/localsend-gui-slint/packaging/linux/org.localsend.pc.desktop"
METAINFO="$ROOT/apps/localsend-gui-slint/packaging/linux/org.localsend.pc.metainfo.xml"
ICON="$ROOT/apps/localsend-gui-slint/packaging/macos/app-icon.svg"
ICON_PNG="$ROOT/apps/localsend-gui-slint/packaging/linux/app-icon-256.png"

command -v cargo >/dev/null 2>&1 || { echo 'cargo is required' >&2; exit 1; }
command -v curl >/dev/null 2>&1 || { echo 'curl is required' >&2; exit 1; }

if [ "${LOCALSEND_SKIP_BUILD:-0}" != "1" ]; then
    "$ROOT/scripts/build-linux-desktop.sh" "$ARCH"
fi

for binary in localsend-gui-slint localsendd localsend-tray localsend-launcher localsend-tui; do
    [ -x "$RELEASE/$binary" ] || { echo "missing build output: $RELEASE/$binary" >&2; exit 1; }
done
for asset in "$DESKTOP" "$METAINFO" "$ICON" "$ICON_PNG"; do
    [ -f "$asset" ] || { echo "missing packaging asset: $asset" >&2; exit 1; }
done
if [ "$(sed -n 's/^Exec=//p' "$DESKTOP" | head -n 1)" != "localsend-tray" ] \
    || [ "$(sed -n 's/^TryExec=//p' "$DESKTOP" | head -n 1)" != "localsend-tray" ]; then
    echo 'Linux desktop entry must launch localsend-tray so the tray owns the daemon lifecycle' >&2
    exit 1
fi

if [ ! -x "$APPIMAGETOOL" ]; then
    mkdir -p "$TOOLS"
    curl -L --fail --retry 3 --connect-timeout 20 \
        -o "$APPIMAGETOOL" \
        "https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-$ASSET_ARCH.AppImage"
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
install -D -m 0644 "$ICON_PNG" "$APPDIR/usr/share/icons/hicolor/256x256/apps/org.localsend.pc.png"
install -m 0644 "$ICON_PNG" "$APPDIR/org.localsend.pc.png"
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
case "${1:-}" in
    --launcher)
        shift
        exec "$HERE/usr/bin/localsend-launcher" "$@"
        ;;
    --gui)
        shift
        exec "$HERE/usr/bin/localsend" --window "$@"
        ;;
    --tui)
        shift
        exec "$HERE/usr/bin/localsend-tui" "$@"
        ;;
    --daemon)
        shift
        exec "$HERE/usr/bin/localsendd" "$@"
        ;;
    *)
        exec "$HERE/usr/bin/localsend-tray" "$@"
        ;;
esac
EOF
chmod +x "$APPDIR/AppRun"
ln -s usr/share/applications/org.localsend.pc.desktop "$APPDIR/org.localsend.pc.desktop"
# appimagetool >= 2026-10 rejects an SVG .DirIcon; ship a PNG at the AppDir root and
# let appimagetool derive .DirIcon from it (the scalable SVG stays for the desktop entry).
rm -f "$APPDIR/.DirIcon" "$APPDIR/org.localsend.pc.svg"

if [ -e "$OUTPUT" ]; then
    unlink "$OUTPUT"
fi
ARCH="$ASSET_ARCH" APPIMAGE_EXTRACT_AND_RUN=1 \
    "$APPIMAGETOOL" --runtime-file "$RUNTIME" "$APPDIR" "$OUTPUT"
test -x "$OUTPUT"
printf '%s\n' "$OUTPUT"

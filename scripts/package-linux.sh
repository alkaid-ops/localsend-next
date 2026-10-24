#!/bin/sh
set -eu

ROOT=$(cd "$(dirname "$0")/.." && pwd)
ARCH=${1:-x86_64}
VERSION=${PACKAGE_VERSION:-0.1.0}
VERSION=${VERSION#v}

case "$VERSION" in
    ''|*[!A-Za-z0-9._~-]*)
        printf 'invalid PACKAGE_VERSION: %s\n' "$VERSION" >&2
        exit 64
        ;;
esac

case "$ARCH" in
    x86_64)
        TARGET=x86_64-unknown-linux-gnu
        DEB_ARCH=amd64
        ;;
    aarch64)
        TARGET=aarch64-unknown-linux-gnu
        DEB_ARCH=arm64
        ;;
    *)
        printf 'usage: %s x86_64|aarch64\n' "$0" >&2
        exit 64
        ;;
esac

RELEASE="$ROOT/target/$TARGET/release"
PACKAGES="$ROOT/target/release/packages"
TAR_STAGE="$PACKAGES/LocalSend-linux-$ARCH"
DEB_STAGE="$PACKAGES/deb-$DEB_ARCH"
TARBALL="$PACKAGES/LocalSend-${VERSION}-linux-$ARCH.tar.gz"
DEB="$PACKAGES/localsend_${VERSION}_${DEB_ARCH}.deb"
DESKTOP="$ROOT/apps/localsend-gui-slint/packaging/linux/org.localsend.pc.desktop"
METAINFO="$ROOT/apps/localsend-gui-slint/packaging/linux/org.localsend.pc.metainfo.xml"
ICON="$ROOT/apps/localsend-gui-slint/packaging/macos/app-icon.svg"

command -v cargo >/dev/null 2>&1 || { echo 'cargo is required' >&2; exit 1; }
command -v dpkg-deb >/dev/null 2>&1 || { echo 'dpkg-deb is required' >&2; exit 1; }
command -v tar >/dev/null 2>&1 || { echo 'tar is required' >&2; exit 1; }

if [ "${LOCALSEND_SKIP_BUILD:-0}" != "1" ]; then
    "$ROOT/scripts/build-linux-desktop.sh" "$ARCH"
fi

for binary in localsend-gui-slint localsendd localsend-tray localsend-launcher localsend-tui; do
    if [ ! -x "$RELEASE/$binary" ]; then
        echo "missing build output: $RELEASE/$binary" >&2
        exit 1
    fi
done
for asset in "$DESKTOP" "$METAINFO" "$ICON"; do
    if [ ! -f "$asset" ]; then
        echo "missing packaging asset: $asset" >&2
        exit 1
    fi
done
if [ "$(sed -n 's/^Exec=//p' "$DESKTOP" | head -n 1)" != "localsend-tray" ] \
    || [ "$(sed -n 's/^TryExec=//p' "$DESKTOP" | head -n 1)" != "localsend-tray" ]; then
    echo 'Linux desktop entry must launch localsend-tray so the tray owns the daemon lifecycle' >&2
    exit 1
fi

rm -rf "$TAR_STAGE" "$DEB_STAGE"
mkdir -p \
    "$TAR_STAGE/bin" \
    "$TAR_STAGE/share/applications" \
    "$TAR_STAGE/share/icons/hicolor/scalable/apps" \
    "$TAR_STAGE/share/metainfo" \
    "$DEB_STAGE/DEBIAN" \
    "$DEB_STAGE/usr/bin" \
    "$DEB_STAGE/usr/share/applications" \
    "$DEB_STAGE/usr/share/icons/hicolor/scalable/apps" \
    "$DEB_STAGE/usr/share/metainfo"

install -m 0755 "$RELEASE/localsend-gui-slint" "$TAR_STAGE/bin/localsend"
install -m 0755 "$RELEASE/localsendd" "$TAR_STAGE/bin/localsendd"
install -m 0755 "$RELEASE/localsend-tray" "$TAR_STAGE/bin/localsend-tray"
install -m 0755 "$RELEASE/localsend-launcher" "$TAR_STAGE/bin/localsend-launcher"
install -m 0755 "$RELEASE/localsend-tui" "$TAR_STAGE/bin/localsend-tui"
install -m 0644 "$DESKTOP" "$TAR_STAGE/share/applications/org.localsend.pc.desktop"
install -m 0644 "$METAINFO" "$TAR_STAGE/share/metainfo/org.localsend.pc.metainfo.xml"
install -m 0644 "$ICON" "$TAR_STAGE/share/icons/hicolor/scalable/apps/org.localsend.pc.svg"
install -m 0644 "$ROOT/LICENSE" "$TAR_STAGE/LICENSE"
install -m 0644 "$ROOT/THIRD_PARTY_NOTICES.md" "$TAR_STAGE/THIRD_PARTY_NOTICES.md"

install -m 0755 "$RELEASE/localsend-gui-slint" "$DEB_STAGE/usr/bin/localsend"
install -m 0755 "$RELEASE/localsendd" "$DEB_STAGE/usr/bin/localsendd"
install -m 0755 "$RELEASE/localsend-tray" "$DEB_STAGE/usr/bin/localsend-tray"
install -m 0755 "$RELEASE/localsend-launcher" "$DEB_STAGE/usr/bin/localsend-launcher"
install -m 0755 "$RELEASE/localsend-tui" "$DEB_STAGE/usr/bin/localsend-tui"
install -m 0644 "$DESKTOP" "$DEB_STAGE/usr/share/applications/org.localsend.pc.desktop"
install -m 0644 "$METAINFO" "$DEB_STAGE/usr/share/metainfo/org.localsend.pc.metainfo.xml"
install -m 0644 "$ICON" "$DEB_STAGE/usr/share/icons/hicolor/scalable/apps/org.localsend.pc.svg"

INSTALLED_SIZE=$(du -sk "$DEB_STAGE/usr" | awk '{print $1}')
cat >"$DEB_STAGE/DEBIAN/control" <<EOF
Package: localsend
Version: $VERSION
Section: net
Priority: optional
Architecture: $DEB_ARCH
Installed-Size: $INSTALLED_SIZE
Maintainer: LocalSend Contributors <noreply@localhost>
Depends: libc6, libgcc-s1, libgtk-3-0, libayatana-appindicator3-1 | libappindicator3-1, libx11-6, libxkbcommon0, libwayland-client0, libgl1
Description: LocalSend desktop client for local network file sharing
 LocalSend is a cross-platform local network file transfer application.
EOF

tar --owner=0 --group=0 -C "$TAR_STAGE" -czf "$TARBALL" .
dpkg-deb --build --root-owner-group "$DEB_STAGE" "$DEB"
dpkg-deb --info "$DEB" >/dev/null
tar -tzf "$TARBALL" >/dev/null
printf '%s\n%s\n' "$TARBALL" "$DEB"

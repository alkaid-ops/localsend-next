#!/bin/zsh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
ARCH=${1:-$(uname -m)}
VERSION=${PACKAGE_VERSION:-0.1.0}
VERSION=${VERSION#v}
BUILD_NUMBER=${BUILD_NUMBER:-1}

case "$ARCH" in
    arm64|x86_64) ;;
    *)
        printf 'usage: %s arm64|x86_64\n' "$0" >&2
        exit 64
        ;;
esac
case "$VERSION" in
    ''|*[!A-Za-z0-9._-]*)
        printf 'invalid PACKAGE_VERSION: %s\n' "$VERSION" >&2
        exit 64
        ;;
esac
if [[ "$(uname -m)" != "$ARCH" ]]; then
    printf 'requested %s build on %s host\n' "$ARCH" "$(uname -m)" >&2
    exit 1
fi

BUILD_DIR="$REPO_ROOT/target/release/bundle/macos"
PACKAGES_DIR="$REPO_ROOT/target/release/packages"
APP_DIR="$BUILD_DIR/LocalSend.app"
CONTENTS_DIR="$APP_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"
HELPERS_DIR="$CONTENTS_DIR/Helpers"
TRAY_APP_DIR="$HELPERS_DIR/LocalSendTray.app"
TRAY_CONTENTS_DIR="$TRAY_APP_DIR/Contents"
TRAY_MACOS_DIR="$TRAY_CONTENTS_DIR/MacOS"
ICON_WORK_DIR="$BUILD_DIR/icon-work"
DMG_STAGE="$BUILD_DIR/dmg-$ARCH"
ZIP="$PACKAGES_DIR/LocalSend-${VERSION}-macos-${ARCH}.zip"
DMG="$PACKAGES_DIR/LocalSend-${VERSION}-macos-${ARCH}.dmg"

export PATH="$HOME/.cargo/bin:$PATH"

cargo build --locked --release --manifest-path "$REPO_ROOT/Cargo.toml" -p localsendd -p localsend-tray
cargo build --locked --release --manifest-path "$REPO_ROOT/Cargo.toml" -p localsend-gui-slint --no-default-features --features backend-femtovg-wgpu

mkdir -p "$MACOS_DIR" "$RESOURCES_DIR" "$TRAY_MACOS_DIR" "$ICON_WORK_DIR/raw" "$ICON_WORK_DIR/AppIcon.iconset" "$PACKAGES_DIR"

for stale in LocalSend LocalSendTray localsend-tray localsendd; do
    if [ -e "$MACOS_DIR/$stale" ]; then
        unlink "$MACOS_DIR/$stale"
    fi
done

cp "$REPO_ROOT/target/release/localsend-gui-slint" "$MACOS_DIR/LocalSend"
cp "$REPO_ROOT/target/release/localsendd" "$MACOS_DIR/localsendd"
cp "$REPO_ROOT/target/release/localsend-tray" "$TRAY_MACOS_DIR/LocalSendTray"
cp "$SCRIPT_DIR/packaging/macos/TrayInfo.plist" "$TRAY_CONTENTS_DIR/Info.plist"
cp "$SCRIPT_DIR/packaging/macos/Info.plist" "$CONTENTS_DIR/Info.plist"
/usr/libexec/PlistBuddy -c "Set :CFBundleShortVersionString $VERSION" "$CONTENTS_DIR/Info.plist"
/usr/libexec/PlistBuddy -c "Set :CFBundleVersion $BUILD_NUMBER" "$CONTENTS_DIR/Info.plist"
/usr/libexec/PlistBuddy -c "Set :CFBundleShortVersionString $VERSION" "$TRAY_CONTENTS_DIR/Info.plist"
/usr/libexec/PlistBuddy -c "Set :CFBundleVersion $BUILD_NUMBER" "$TRAY_CONTENTS_DIR/Info.plist"
chmod +x "$MACOS_DIR/LocalSend" "$MACOS_DIR/localsendd" "$TRAY_MACOS_DIR/LocalSendTray"
printf 'APPL????' > "$CONTENTS_DIR/PkgInfo"

qlmanage -t -s 1024 -o "$ICON_WORK_DIR/raw" "$SCRIPT_DIR/packaging/macos/app-icon.svg" >/dev/null 2>&1
SOURCE_PNG="$ICON_WORK_DIR/raw/app-icon.svg.png"

for spec in \
    "16 icon_16x16.png" \
    "32 icon_16x16@2x.png" \
    "32 icon_32x32.png" \
    "64 icon_32x32@2x.png" \
    "128 icon_128x128.png" \
    "256 icon_128x128@2x.png" \
    "256 icon_256x256.png" \
    "512 icon_256x256@2x.png" \
    "512 icon_512x512.png" \
    "1024 icon_512x512@2x.png"
do
    size="${spec%% *}"
    name="${spec#* }"
    sips -z "$size" "$size" "$SOURCE_PNG" --out "$ICON_WORK_DIR/AppIcon.iconset/$name" >/dev/null
done

if [ -e "$RESOURCES_DIR/AppIcon.icns" ]; then
    unlink "$RESOURCES_DIR/AppIcon.icns"
fi
iconutil -c icns "$ICON_WORK_DIR/AppIcon.iconset" -o "$RESOURCES_DIR/AppIcon.icns"

codesign --force --deep --sign - "$APP_DIR" >/dev/null

if [ -e "$ZIP" ]; then
    unlink "$ZIP"
fi
ditto -c -k --sequesterRsrc --keepParent "$APP_DIR" "$ZIP"

rm -rf "$DMG_STAGE"
mkdir -p "$DMG_STAGE"
ditto "$APP_DIR" "$DMG_STAGE/LocalSend.app"
ln -s /Applications "$DMG_STAGE/Applications"
if [ -e "$DMG" ]; then
    unlink "$DMG"
fi
hdiutil create \
    -volname "LocalSend $VERSION" \
    -srcfolder "$DMG_STAGE" \
    -ov \
    -fs HFS+ \
    -format UDZO \
    "$DMG" >/dev/null
hdiutil verify "$DMG" >/dev/null

echo "$APP_DIR"
echo "$ZIP"
echo "$DMG"

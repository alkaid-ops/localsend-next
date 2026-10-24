#!/bin/sh
set -eu

ROOT=$(cd "$(dirname "$0")/.." && pwd)
ARCH=${1:-$(uname -m)}
VERSION=${PACKAGE_VERSION:-0.1.0}
VERSION=${VERSION#v}
RPM_VERSION=$(printf '%s' "$VERSION" | tr '-' '~')

case "$ARCH" in
    x86_64)
        TARGET=x86_64-unknown-linux-gnu
        RPM_ARCH=x86_64
        ;;
    aarch64)
        TARGET=aarch64-unknown-linux-gnu
        RPM_ARCH=aarch64
        ;;
    *)
        printf 'usage: %s x86_64|aarch64\n' "$0" >&2
        exit 64
        ;;
esac

case "$VERSION" in
    ''|*[!A-Za-z0-9._-]*)
        printf 'invalid PACKAGE_VERSION: %s\n' "$VERSION" >&2
        exit 64
        ;;
esac

command -v cargo >/dev/null 2>&1 || { echo 'cargo is required' >&2; exit 1; }
command -v rpmbuild >/dev/null 2>&1 || { echo 'rpmbuild is required' >&2; exit 1; }
command -v rpm >/dev/null 2>&1 || { echo 'rpm is required' >&2; exit 1; }

RELEASE="$ROOT/target/$TARGET/release"
PACKAGES="$ROOT/target/release/packages"
RPMROOT="$ROOT/target/release/rpmbuild-$ARCH"
STAGE="$RPMROOT/stage"
SPEC="$RPMROOT/SPECS/localsend.spec"
OUTPUT="$PACKAGES/localsend-${RPM_VERSION}-1.${RPM_ARCH}.rpm"
DESKTOP="$ROOT/apps/localsend-gui-slint/packaging/linux/org.localsend.pc.desktop"
METAINFO="$ROOT/apps/localsend-gui-slint/packaging/linux/org.localsend.pc.metainfo.xml"
ICON="$ROOT/apps/localsend-gui-slint/packaging/macos/app-icon.svg"

if [ "${LOCALSEND_SKIP_BUILD:-0}" != "1" ]; then
    "$ROOT/scripts/build-linux-desktop.sh" "$ARCH"
fi

for binary in localsend-gui-slint localsendd localsend-tray localsend-launcher localsend-tui; do
    [ -x "$RELEASE/$binary" ] || { echo "missing build output: $RELEASE/$binary" >&2; exit 1; }
done
for asset in "$DESKTOP" "$METAINFO" "$ICON"; do
    [ -f "$asset" ] || { echo "missing packaging asset: $asset" >&2; exit 1; }
done

rm -rf "$RPMROOT"
mkdir -p \
    "$STAGE/usr/bin" \
    "$STAGE/usr/share/applications" \
    "$STAGE/usr/share/icons/hicolor/scalable/apps" \
    "$STAGE/usr/share/metainfo" \
    "$RPMROOT/BUILD" \
    "$RPMROOT/BUILDROOT" \
    "$RPMROOT/RPMS" \
    "$RPMROOT/SOURCES" \
    "$RPMROOT/SPECS" \
    "$RPMROOT/SRPMS" \
    "$PACKAGES"

install -m 0755 "$RELEASE/localsend-gui-slint" "$STAGE/usr/bin/localsend"
install -m 0755 "$RELEASE/localsendd" "$STAGE/usr/bin/localsendd"
install -m 0755 "$RELEASE/localsend-tray" "$STAGE/usr/bin/localsend-tray"
install -m 0755 "$RELEASE/localsend-launcher" "$STAGE/usr/bin/localsend-launcher"
install -m 0755 "$RELEASE/localsend-tui" "$STAGE/usr/bin/localsend-tui"
install -m 0644 "$DESKTOP" "$STAGE/usr/share/applications/org.localsend.pc.desktop"
install -m 0644 "$METAINFO" "$STAGE/usr/share/metainfo/org.localsend.pc.metainfo.xml"
install -m 0644 "$ICON" "$STAGE/usr/share/icons/hicolor/scalable/apps/org.localsend.pc.svg"
install -m 0644 "$ROOT/LICENSE" "$STAGE/LICENSE"
install -m 0644 "$ROOT/THIRD_PARTY_NOTICES.md" "$STAGE/THIRD_PARTY_NOTICES.md"

cat >"$SPEC" <<EOF
Name: localsend
Version: %{localsend_version}
Release: 1%{?dist}
Summary: LocalSend desktop client for local network file sharing
License: Apache-2.0
URL: https://localsend.org
BuildArch: %{localsend_rpm_arch}
AutoReqProv: yes

%description
LocalSend is a cross-platform local network file transfer application.

%prep

%build

%install
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_datadir}/applications
mkdir -p %{buildroot}%{_datadir}/icons/hicolor/scalable/apps
mkdir -p %{buildroot}%{_datadir}/metainfo
mkdir -p %{buildroot}%{_licensedir}/localsend
install -m 0755 %{localsend_stage}/usr/bin/localsend %{buildroot}%{_bindir}/localsend
install -m 0755 %{localsend_stage}/usr/bin/localsendd %{buildroot}%{_bindir}/localsendd
install -m 0755 %{localsend_stage}/usr/bin/localsend-tray %{buildroot}%{_bindir}/localsend-tray
install -m 0755 %{localsend_stage}/usr/bin/localsend-launcher %{buildroot}%{_bindir}/localsend-launcher
install -m 0755 %{localsend_stage}/usr/bin/localsend-tui %{buildroot}%{_bindir}/localsend-tui
install -m 0644 %{localsend_stage}/usr/share/applications/org.localsend.pc.desktop %{buildroot}%{_datadir}/applications/org.localsend.pc.desktop
install -m 0644 %{localsend_stage}/usr/share/icons/hicolor/scalable/apps/org.localsend.pc.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/org.localsend.pc.svg
install -m 0644 %{localsend_stage}/usr/share/metainfo/org.localsend.pc.metainfo.xml %{buildroot}%{_datadir}/metainfo/org.localsend.pc.metainfo.xml
install -m 0644 %{localsend_stage}/LICENSE %{buildroot}%{_licensedir}/localsend/LICENSE
install -m 0644 %{localsend_stage}/THIRD_PARTY_NOTICES.md %{buildroot}%{_licensedir}/localsend/THIRD_PARTY_NOTICES.md

%files
%{_bindir}/localsend
%{_bindir}/localsendd
%{_bindir}/localsend-tray
%{_bindir}/localsend-launcher
%{_bindir}/localsend-tui
%{_datadir}/applications/org.localsend.pc.desktop
%{_datadir}/icons/hicolor/scalable/apps/org.localsend.pc.svg
%{_datadir}/metainfo/org.localsend.pc.metainfo.xml
%{_licensedir}/localsend/LICENSE
%{_licensedir}/localsend/THIRD_PARTY_NOTICES.md

%changelog
* $(LC_ALL=C date '+%a %b %d %Y') LocalSend Contributors <noreply@localhost> - ${RPM_VERSION}-1
- LocalSend ${VERSION}
EOF

rpmbuild -bb \
    --define "_topdir $RPMROOT" \
    --define "localsend_version $RPM_VERSION" \
    --define "localsend_stage $STAGE" \
    --define "localsend_rpm_arch $RPM_ARCH" \
    --define "debug_package %{nil}" \
    "$SPEC"

BUILT=$(find "$RPMROOT/RPMS" -type f -name '*.rpm' -print -quit)
[ -n "$BUILT" ] || { echo 'rpmbuild produced no rpm' >&2; exit 1; }
cp "$BUILT" "$OUTPUT"
rpm -qpi "$OUTPUT" >/dev/null
rpm -qpl "$OUTPUT" >/dev/null
printf '%s\n' "$OUTPUT"

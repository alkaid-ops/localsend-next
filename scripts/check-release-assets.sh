#!/bin/sh
set -eu

ROOT=$(cd "$(dirname "$0")/.." && pwd)
DIST=${1:-"$ROOT/dist"}
VERSION=${PACKAGE_VERSION:-}
VERSION=${VERSION#v}

if [ ! -d "$DIST" ]; then
    printf 'release asset directory not found: %s\n' "$DIST" >&2
    exit 1
fi

# Every release asset is LocalSend-<version>-<platform>-<arch>.<ext>, with one
# spelling per platform and architecture. Anything else fails the release.
expected_names() {
    version=$1
    for platform_arch in \
        linux-x86_64 linux-aarch64 \
        windows-x86_64 windows-aarch64 \
        macos-x86_64 macos-aarch64
    do
        case "$platform_arch" in
            linux-*)
                for ext in AppImage deb rpm; do
                    printf 'LocalSend-%s-%s.%s\n' "$version" "$platform_arch" "$ext"
                done
                ;;
            windows-*) printf 'LocalSend-%s-%s.zip\n' "$version" "$platform_arch" ;;
            macos-*) printf 'LocalSend-%s-%s.dmg\n' "$version" "$platform_arch" ;;
        esac
    done
}

tmp_expected=$(mktemp)
tmp_actual=$(mktemp)
trap 'rm -f "$tmp_expected" "$tmp_actual"' EXIT INT TERM HUP

if [ -n "$VERSION" ]; then
    expected_names "$VERSION" | sort >"$tmp_expected"
else
    # Without a known version, still require the shared shape and extension set.
    find "$DIST" -maxdepth 1 -type f -exec basename {} \; \
        | grep -E '\.(AppImage|deb|rpm|zip|dmg)$' | sort >"$tmp_actual"
    bad=$(grep -Ev '^LocalSend-[0-9A-Za-z][0-9A-Za-z._-]*-(linux|windows|macos)-(x86_64|aarch64)\.(AppImage|deb|rpm|zip|dmg)$' "$tmp_actual" || true)
    if [ -n "$bad" ]; then
        printf 'release assets with inconsistent names:\n%s\n' "$bad" >&2
        exit 1
    fi
    printf 'verified %s release assets in %s\n' "$(wc -l <"$tmp_actual" | tr -d ' ')" "$DIST" >&2
    tmp_actual=
fi

if [ -n "$tmp_actual" ]; then
    find "$DIST" -maxdepth 1 -type f -exec basename {} \; \
        | grep -E '\.(AppImage|deb|rpm|zip|dmg)$' | sort >"$tmp_actual"

    missing=$(comm -23 "$tmp_expected" "$tmp_actual")
    unexpected=$(comm -13 "$tmp_expected" "$tmp_actual")
    if [ -n "$missing" ] || [ -n "$unexpected" ]; then
        [ -z "$missing" ] || printf 'missing release assets:\n%s\n' "$missing" >&2
        [ -z "$unexpected" ] || printf 'unexpected release assets:\n%s\n' "$unexpected" >&2
        exit 1
    fi
fi

if command -v sha256sum >/dev/null 2>&1; then
    hash_file() {
        sha256sum "$1" | awk '{print $1}'
    }
else
    command -v shasum >/dev/null 2>&1 || { echo 'sha256sum or shasum is required' >&2; exit 1; }
    hash_file() {
        LC_ALL=C shasum -a 256 "$1" | awk '{print $1}'
    }
fi

find "$DIST" -maxdepth 1 -type f \
    \( -name '*.AppImage' -o -name '*.deb' -o -name '*.rpm' -o -name '*.zip' -o -name '*.dmg' \) \
    | sort | while IFS= read -r file; do
        printf '%s  %s\n' "$(hash_file "$file")" "$(basename "$file")"
    done

printf 'verified %s release assets in %s\n' \
    "$(find "$DIST" -maxdepth 1 -type f \( -name '*.AppImage' -o -name '*.deb' -o -name '*.rpm' -o -name '*.zip' -o -name '*.dmg' \) | wc -l | tr -d ' ')" \
    "$DIST" >&2

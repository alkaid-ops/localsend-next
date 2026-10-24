#!/bin/sh
set -eu

ROOT=$(cd "$(dirname "$0")/.." && pwd)
DIST=${1:-"$ROOT/dist"}

if [ ! -d "$DIST" ]; then
    printf 'release asset directory not found: %s\n' "$DIST" >&2
    exit 1
fi

check_count() {
    pattern=$1
    expected=$2
    actual=$(find "$DIST" -maxdepth 1 -type f -name "$pattern" | wc -l | tr -d ' ')
    if [ "$actual" -ne "$expected" ]; then
        printf 'expected %s %s files, found %s\n' "$expected" "$pattern" "$actual" >&2
        find "$DIST" -maxdepth 1 -type f -exec basename {} \; | sort >&2
        exit 1
    fi
}

check_count '*.AppImage' 2
check_count '*.deb' 2
check_count '*.rpm' 2
check_count '*.zip' 2
check_count '*.dmg' 2

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

printf 'verified 10 release assets in %s\n' "$DIST" >&2

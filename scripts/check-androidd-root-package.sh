#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    printf 'usage: %s <module.zip>\n' "$0" >&2
    exit 64
fi

ZIP=$1
ROOT=$(unset CDPATH; cd -- "$(dirname "$0")/.." && pwd)

if [ ! -f "$ZIP" ]; then
    printf 'module zip not found: %s\n' "$ZIP" >&2
    exit 1
fi

LIST=$(unzip -Z1 "$ZIP")
for file in module.prop post-fs-data.sh service.sh uninstall.sh config/settings.json bin/localsend-androidd; do
    if ! printf '%s\n' "$LIST" | grep -qx "$file"; then
        printf 'module zip is missing %s\n' "$file" >&2
        exit 1
    fi
done

if printf '%s\n' "$LIST" | grep -q 'system/etc/init\|system/bin'; then
    printf 'MVP module must not install an unverified init/system service\n' >&2
    exit 1
fi

if ! command -v zipinfo >/dev/null 2>&1; then
    printf 'zipinfo is required for executable-bit checks\n' >&2
    exit 1
fi
for executable in post-fs-data.sh service.sh uninstall.sh bin/localsend-androidd; do
    mode=$(zipinfo -l "$ZIP" | awk -v name="$executable" '$NF == name {print $1; exit}')
    case "$mode" in
        -rwx*) ;;
        *)
            printf 'module entry is not executable: %s (%s)\n' "$executable" "${mode:-missing}" >&2
            exit 1
            ;;
    esac
done

sh -n "$ROOT/packaging/android-root/post-fs-data.sh"
sh -n "$ROOT/packaging/android-root/service.sh"
sh -n "$ROOT/packaging/android-root/uninstall.sh"

printf 'PASS: module structure and shell syntax are valid\n'


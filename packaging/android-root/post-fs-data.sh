#!/system/bin/sh
set -eu

MODDIR=${0%/*}
RUNTIME=/data/adb/localsend-next
CONFIG="$RUNTIME/config"
LOGS="$RUNTIME/logs"

umask 077
mkdir -p "$CONFIG" "$LOGS"

if [ ! -f "$CONFIG/settings.json" ]; then
    cp "$MODDIR/config/settings.json" "$CONFIG/settings.json"
fi

chmod 0755 "$MODDIR/service.sh" "$MODDIR/uninstall.sh" "$MODDIR/bin/localsend-androidd" 2>/dev/null || true


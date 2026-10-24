#!/system/bin/sh
set -u

RUNTIME=/data/adb/localsend-next
PIDFILE="$RUNTIME/localsend-androidd.pid"

if [ -f "$PIDFILE" ]; then
    pid=$(cat "$PIDFILE" 2>/dev/null || true)
    if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then
        kill -TERM "$pid" 2>/dev/null || true
        count=0
        while kill -0 "$pid" 2>/dev/null && [ "$count" -lt 50 ]; do
            sleep 0.1
            count=$((count + 1))
        done
        kill -KILL "$pid" 2>/dev/null || true
    fi
fi

rm -f "$PIDFILE" "$RUNTIME/localsend-androidd.lock"


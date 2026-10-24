#!/bin/sh
set -eu

ROOT=$(unset CDPATH; cd -- "$(dirname "$0")/.." && pwd)
TIMESTAMP=$(date '+%Y%m%d-%H%M%S')
OUT="$ROOT/target/acceptance/android-root/$TIMESTAMP"
ADB=${ADB:-adb}
MODULE_ZIP=${MODULE_ZIP:-$ROOT/target/release/packages/localsend-next-root-aarch64.zip}
SOAK_MINUTES=${SOAK_MINUTES:-30}
DO_REBOOT=${DO_REBOOT:-0}
RUNTIME=/data/adb/localsend-next

mkdir -p "$OUT"

write_blocked() {
    reason=$1
    cat > "$OUT/summary.json" <<EOF
{
  "status": "BLOCKED",
  "reason": "$reason",
  "timestamp": "$TIMESTAMP",
  "runtime_verified": false
}
EOF
    printf 'BLOCKED: %s\n' "$reason" >&2
    printf 'summary: %s\n' "$OUT/summary.json"
    exit 2
}

if ! command -v "$ADB" >/dev/null 2>&1; then
    write_blocked "adb is not installed"
fi

DEVICES=$($ADB devices | awk 'NR > 1 && $2 == "device" {print $1}')
if [ -z "$DEVICES" ]; then
    write_blocked "no connected Android device"
fi
if [ "$(printf '%s\n' "$DEVICES" | wc -l | tr -d ' ')" -ne 1 ]; then
    write_blocked "exactly one Android device is required"
fi

DEVICE=$DEVICES
ADB_CMD="$ADB -s $DEVICE"
ROOT_ID=$($ADB_CMD shell 'su -c id' 2>/dev/null || true)
if ! printf '%s' "$ROOT_ID" | grep -q 'uid=0'; then
    write_blocked "root shell is unavailable"
fi
if [ ! -f "$MODULE_ZIP" ]; then
    write_blocked "module zip does not exist: $MODULE_ZIP"
fi

$ADB_CMD push "$MODULE_ZIP" /data/local/tmp/localsend-next-root.zip >/dev/null
if $ADB_CMD shell 'command -v magisk' >/dev/null 2>&1; then
    $ADB_CMD shell 'su -c "magisk --install-module /data/local/tmp/localsend-next-root.zip"' >/dev/null
elif $ADB_CMD shell 'command -v ksud' >/dev/null 2>&1; then
    $ADB_CMD shell 'su -c "ksud module install /data/local/tmp/localsend-next-root.zip"' >/dev/null
else
    write_blocked "neither Magisk nor KernelSU was detected"
fi

if [ "$DO_REBOOT" = "1" ]; then
    $ADB_CMD reboot
    $ADB_CMD wait-for-device
    count=0
    while [ "$($ADB_CMD shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" != "1" ]; do
        count=$((count + 1))
        if [ "$count" -gt 120 ]; then
            write_blocked "device did not finish booting"
        fi
        sleep 1
    done
fi

PID=$($ADB_CMD shell "cat $RUNTIME/localsend-androidd.pid 2>/dev/null" | tr -d '\r' || true)
if [ -z "$PID" ] || ! $ADB_CMD shell "kill -0 $PID" >/dev/null 2>&1; then
    write_blocked "localsend-androidd is not running"
fi

FIRST_PID=$PID
$ADB_CMD shell "kill -9 $FIRST_PID" >/dev/null 2>&1 || true
sleep 6
SECOND_PID=$($ADB_CMD shell "cat $RUNTIME/localsend-androidd.pid 2>/dev/null" | tr -d '\r' || true)
if [ -z "$SECOND_PID" ] || [ "$SECOND_PID" = "$FIRST_PID" ]; then
    write_blocked "watchdog did not replace the killed daemon"
fi

$ADB_CMD shell "touch $RUNTIME/disabled"
sleep 7
if $ADB_CMD shell "cat $RUNTIME/localsend-androidd.pid 2>/dev/null" | tr -d '\r' | grep -q .; then
    write_blocked "disabled marker did not stop the daemon"
fi
$ADB_CMD shell "rm -f $RUNTIME/disabled"
sleep 7
THIRD_PID=$($ADB_CMD shell "cat $RUNTIME/localsend-androidd.pid 2>/dev/null" | tr -d '\r' || true)
if [ -z "$THIRD_PID" ]; then
    write_blocked "daemon did not restart after removing disabled marker"
fi

$ADB_CMD shell "dumpsys notification --noredact" > "$OUT/notifications.txt" 2>/dev/null || true
$ADB_CMD shell "cat $RUNTIME/logs/supervisor.log" > "$OUT/supervisor.log" 2>/dev/null || true
$ADB_CMD shell "cat $RUNTIME/logs/localsend-androidd.log" > "$OUT/localsend-androidd.log" 2>/dev/null || true

sleep $((SOAK_MINUTES * 60))
FINAL_PID=$($ADB_CMD shell "cat $RUNTIME/localsend-androidd.pid 2>/dev/null" | tr -d '\r' || true)
if [ -z "$FINAL_PID" ] || ! $ADB_CMD shell "kill -0 $FINAL_PID" >/dev/null 2>&1; then
    write_blocked "daemon did not survive the soak period"
fi

cat > "$OUT/summary.json" <<EOF
{
  "status": "MANUAL_REQUIRED",
  "timestamp": "$TIMESTAMP",
  "runtime_verified": true,
  "device": "$DEVICE",
  "initial_pid": "$FIRST_PID",
  "restarted_pid": "$SECOND_PID",
  "final_pid": "$FINAL_PID",
  "soak_minutes": $SOAK_MINUTES,
  "manual_checks": [
    "official LocalSend discovers the device or manual IP works",
    "text notification appears once and has sound",
    "file notification appears once",
    "file SHA-256, mtime and relative path match",
    "screen-off receive succeeds after 10 minutes",
    "no persistent foreground-service notification exists"
  ]
}
EOF

printf 'MANUAL_REQUIRED: automated process checks passed; complete official-client checks\n'
printf 'summary: %s\n' "$OUT/summary.json"
exit 3


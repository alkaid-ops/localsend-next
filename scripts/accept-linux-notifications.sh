#!/bin/sh
set -eu

ROOT=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
TRAY=${1:-"$ROOT/target/release/localsend-tray"}
DAEMON=${2:-"$ROOT/target/release/localsendd"}

case "$(uname -s)" in
    Linux) ;;
    *) echo 'This acceptance script must run natively on Linux.' >&2; exit 1 ;;
esac

for executable in "$TRAY" "$DAEMON"; do
    [ -x "$executable" ] || { echo "missing executable: $executable" >&2; exit 1; }
done
command -v dbus-monitor >/dev/null 2>&1 || { echo 'dbus-monitor is required' >&2; exit 1; }
[ -n "${DBUS_SESSION_BUS_ADDRESS:-}" ] || { echo 'DBUS_SESSION_BUS_ADDRESS is not set' >&2; exit 1; }
pgrep -x localsend-tray >/dev/null 2>&1 || { echo 'localsend-tray is not running' >&2; exit 1; }
pgrep -x localsendd >/dev/null 2>&1 || { echo 'localsendd is not running' >&2; exit 1; }

LOG=$(mktemp)
cleanup() {
    if [ -n "${monitor_pid:-}" ]; then kill "$monitor_pid" 2>/dev/null || true; fi
    rm -f "$LOG"
}
trap cleanup EXIT INT TERM

dbus-monitor --session "interface='org.freedesktop.Notifications'" >"$LOG" 2>&1 &
monitor_pid=$!

printf '%s\n' \
    'Send one text message and one file from LocalSend 1.18.2 or another peer.' \
    'Verify that both notifications appear, play a sound when the desktop supports it,' \
    'and that clicking each notification opens the message dialog or receive folder.'
printf '%s' 'Type PASS only after every check succeeded: '
read -r answer
[ "$answer" = "PASS" ] || { echo 'manual notification acceptance was not confirmed' >&2; exit 1; }

grep -q 'member=Notify' "$LOG" || { echo 'no org.freedesktop.Notifications.Notify call was observed' >&2; exit 1; }
grep -q 'member=ActionInvoked' "$LOG" || { echo 'no ActionInvoked signal was observed' >&2; exit 1; }

echo 'result: passed'

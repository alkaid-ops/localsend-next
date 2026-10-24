#!/system/bin/sh
set -u

MODDIR=${0%/*}
RUNTIME=/data/adb/localsend-next
CONFIG="$RUNTIME/config"
LOGS="$RUNTIME/logs"
PIDFILE="$RUNTIME/localsend-androidd.pid"
DISABLED="$RUNTIME/disabled"
SUPERVISOR_LOG="$LOGS/supervisor.log"
BIN="$MODDIR/bin/localsend-androidd"

umask 077
mkdir -p "$CONFIG" "$LOGS"
chmod 0755 "$BIN" 2>/dev/null || true

log() {
    printf '%s %s\n' "$(date '+%Y-%m-%d %H:%M:%S')" "$*" >> "$SUPERVISOR_LOG"
}

process_is_daemon() {
    pid=$1
    [ -n "$pid" ] || return 1
    kill -0 "$pid" 2>/dev/null || return 1
    [ -r "/proc/$pid/cmdline" ] || return 1
    tr '\000' ' ' < "/proc/$pid/cmdline" | grep -q 'localsend-androidd'
}

read_pid() {
    [ -f "$PIDFILE" ] && cat "$PIDFILE" 2>/dev/null || true
}

stop_daemon() {
    pid=$(read_pid)
    if process_is_daemon "$pid"; then
        log "stopping daemon pid=$pid"
        kill -TERM "$pid" 2>/dev/null || true
        count=0
        while process_is_daemon "$pid" && [ "$count" -lt 50 ]; do
            sleep 0.1
            count=$((count + 1))
        done
        if process_is_daemon "$pid"; then
            kill -KILL "$pid" 2>/dev/null || true
        fi
    fi
    rm -f "$PIDFILE"
}

start_daemon() {
    log "starting daemon"
    "$BIN" --runtime-dir "$RUNTIME" >> "$SUPERVISOR_LOG" 2>&1 &
    pid=$!
    printf '%s\n' "$pid" > "$PIDFILE"
    if [ -w "/proc/$pid/oom_score_adj" ]; then
        printf '%s\n' '-900' > "/proc/$pid/oom_score_adj" 2>/dev/null || true
    fi
}

log "supervisor started"
until [ "$(getprop sys.boot_completed 2>/dev/null)" = "1" ]; do
    sleep 2
done
log "boot completed"

backoff=1
while true; do
    if [ -f "$DISABLED" ]; then
        stop_daemon
        backoff=1
        sleep 5
        continue
    fi

    pid=$(read_pid)
    if process_is_daemon "$pid"; then
        sleep 5
        continue
    fi

    rm -f "$PIDFILE"
    start_daemon
    sleep 5

    pid=$(read_pid)
    if process_is_daemon "$pid"; then
        backoff=1
    else
        log "daemon exited; restarting in ${backoff}s"
        sleep "$backoff"
        if [ "$backoff" -lt 30 ]; then
            backoff=$((backoff * 2))
        fi
    fi
done


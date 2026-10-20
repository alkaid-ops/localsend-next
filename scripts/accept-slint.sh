#!/bin/zsh
set -euo pipefail

ROOT=$(cd "$(dirname "$0")/.." && pwd)
APP_DIR="$ROOT/apps/localsend-gui-slint"
GOLDEN="$APP_DIR/reference/slint"
STAMP=$(date +%Y%m%d-%H%M%S)
OUT="$ROOT/target/acceptance/slint/$STAMP"
SCREENS="$OUT/screens"
mkdir -p "$OUT" "$SCREENS"

cd "$ROOT"
cargo build -p localsendd -p localsend-gui-slint | tee "$OUT/build.log"

for run in 1 2 3; do
    LOCALSEND_ACCEPTANCE=1 cargo test -p localsend-gui-slint -- --test-threads=1 --nocapture \
        >"$OUT/run-$run.log" 2>&1
    cat "$OUT/run-$run.log"
done

compare() {
    local output="$1"
    local golden="$2"
    cmp "$output" "$golden" >/dev/null || {
        printf 'golden mismatch: %s != %s\n' "$output" "$golden" >&2
        exit 1
    }
}

for page in receive send settings; do
    for dimension in 800x600 1024x768 1440x900 700x700; do
        width=${dimension%x*}
        height=${dimension#*x}
        for theme in light dark; do
            name="$page-default-$dimension-$theme.png"
            cargo run -q -p localsend-gui-slint --example render_reference -- \
                --headless --page "$page" --state default --width "$width" --height "$height" \
                --theme "$theme" --output "$SCREENS/$name"
            compare "$SCREENS/$name" "$GOLDEN/$name"
        done
    done
done

for state_page in receive:request receive:transferring send:selected send:multiple; do
    page=${state_page%%:*}
    state=${state_page#*:}
    for theme in light dark; do
        name="$page-$state-1024x768-$theme.png"
        cargo run -q -p localsend-gui-slint --example render_reference -- \
            --headless --page "$page" --state "$state" --width 1024 --height 768 \
            --theme "$theme" --output "$SCREENS/$name"
        compare "$SCREENS/$name" "$GOLDEN/$name"
    done
done

cd "$APP_DIR"
./package-macos.sh | tee "$OUT/package.log"
APP="$ROOT/target/release/bundle/macos/LocalSend.app"
codesign --verify --deep --strict "$APP" | tee "$OUT/codesign.log"

SMOKE=$(mktemp -d /tmp/localsend-next-slint-smoke.XXXXXX)
CFG="$SMOKE/config"
mkdir -p "$CFG" "$SMOKE/received"
printf '%s\n' '{"alias":"Slint Acceptance","port":0,"protocol":"http","destination_dir":"'$SMOKE'/received","auto_save":"off","start_server_on_launch":true,"revision":0}' > "$CFG/settings.json"
LOCALSEND_CONFIG_DIR="$CFG" LOCALSEND_RPC_SOCKET="$SMOKE/daemon.sock" \
    "$APP/Contents/MacOS/localsendd" >"$OUT/daemon.log" 2>&1 &
DAEMON_PID=$!
GUI_PID=""
cleanup() {
    [[ -n "$GUI_PID" ]] && kill "$GUI_PID" 2>/dev/null || true
    kill "$DAEMON_PID" 2>/dev/null || true
    [[ -n "$GUI_PID" ]] && wait "$GUI_PID" 2>/dev/null || true
    wait "$DAEMON_PID" 2>/dev/null || true
}
trap cleanup EXIT
for _ in {1..100}; do
    [[ -S "$SMOKE/daemon.sock" ]] && break
    sleep 0.05
done

LOCALSEND_CONFIG_DIR="$CFG" LOCALSEND_RPC_SOCKET="$SMOKE/daemon.sock" LOCALSEND_GUI_TAB=receive \
    "$APP/Contents/MacOS/LocalSend" --window >"$OUT/gui.log" 2>&1 &
GUI_PID=$!
sleep 3
kill -0 "$GUI_PID"

screencapture -x "$OUT/app-smoke.png"
AX_RESULT="$OUT/ax-smoke.log"
set +e
SWIFTC=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/swiftc
if [[ ! -x "$SWIFTC" ]]; then
    SWIFTC=$(xcrun --find swiftc 2>/dev/null || true)
fi
if [[ -z "$SWIFTC" || ! -x "$SWIFTC" ]]; then
    echo 'BLOCKED: no compatible Swift compiler for AX automation' >"$AX_RESULT"
    AX_CODE=2
else
    SDK=$(xcrun --sdk macosx --show-sdk-path)
    "$SWIFTC" -sdk "$SDK" "$ROOT/scripts/macos-ax-smoke.swift" -o "$OUT/ax-smoke" >"$AX_RESULT" 2>&1
    if [[ $? -eq 0 ]]; then
        "$OUT/ax-smoke" "$GUI_PID" >>"$AX_RESULT" 2>&1
        AX_CODE=$?
    else
        AX_CODE=2
    fi
fi
set -e
cat "$AX_RESULT"
if [[ $AX_CODE -eq 2 ]]; then
    printf '{"suite":"slint","result":"blocked","reason":"AX toolchain or macOS Accessibility permission unavailable","artifacts":"%s"}\n' "$OUT" > "$OUT/summary.json"
    exit 2
elif [[ $AX_CODE -ne 0 ]]; then
    printf '{"suite":"slint","result":"failed","stage":"ax-smoke","artifacts":"%s"}\n' "$OUT" > "$OUT/summary.json"
    exit 1
fi

kill "$GUI_PID"
wait "$GUI_PID" 2>/dev/null || true
kill -0 "$DAEMON_PID"

LOCALSEND_CONFIG_DIR="$CFG" LOCALSEND_RPC_SOCKET="$SMOKE/daemon.sock" \
    cargo run -q -p localsendd --example rpc_probe -- shutdown
wait "$DAEMON_PID" 2>/dev/null || true
printf '{"suite":"slint","result":"passed","runs":3,"goldens":32,"ax":"passed","artifacts":"%s"}\n' "$OUT" | tee "$OUT/summary.json"

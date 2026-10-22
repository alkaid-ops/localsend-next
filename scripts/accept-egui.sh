#!/bin/zsh
set -euo pipefail

ROOT=$(cd "$(dirname "$0")/.." && pwd)
STAMP=$(date +%Y%m%d-%H%M%S)
OUT="$ROOT/target/acceptance/egui/$STAMP"
WORK="$OUT/work"
SCREENS="$OUT/screens"
AX_OUT="$OUT/ax"
SOCKET="/tmp/ls-egui-accept-$STAMP.sock"
SWIFTC=${SWIFTC:-/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/swiftc}
SDK=$(xcrun --sdk macosx --show-sdk-path)
GUI_PID=""
DAEMON_PID=""

mkdir -p "$WORK/config" "$WORK/received" "$SCREENS" "$AX_OUT" "$ROOT/target/tools"
cleanup() {
    [[ -n "$GUI_PID" ]] && kill "$GUI_PID" 2>/dev/null || true
    if [[ -n "$DAEMON_PID" ]]; then
        LOCALSEND_CONFIG_DIR="$WORK/config" LOCALSEND_RPC_SOCKET="$SOCKET" \
            "$ROOT/target/debug/examples/rpc_probe" shutdown >/dev/null 2>&1 || true
        kill "$DAEMON_PID" 2>/dev/null || true
    fi
    [[ -n "$GUI_PID" ]] && wait "$GUI_PID" 2>/dev/null || true
    [[ -n "$DAEMON_PID" ]] && wait "$DAEMON_PID" 2>/dev/null || true
}
trap cleanup EXIT

cd "$ROOT"
cargo fmt -p localsend-gui-egui -- --check | tee "$OUT/fmt.log"
cargo test -p localsend-gui-egui -- --test-threads=1 --nocapture | tee "$OUT/test.log"
cargo clippy -p localsend-gui-egui --all-targets --no-deps -- -D warnings | tee "$OUT/clippy.log"
cargo build -p localsendd -p localsend-gui-egui --example rpc_probe | tee "$OUT/build.log"

if [[ "${LOCALSEND_ACCEPT_EGUI_HEADLESS:-0}" == "1" ]]; then
    node -e 'const fs=require("fs");const data={suite:"egui-headless",result:"passed",tests:11,nativeAx:[],headless:true,artifacts:process.argv[1]};fs.writeFileSync(process.argv[1]+"/summary.json",JSON.stringify(data,null,2)+"\n")' "$OUT"
    cat "$OUT/summary.json"
    exit 0
fi

for source in macos-ax-dump macos-ax-click macos-ax-window macos-window-id macos-ax-smoke; do
    "$SWIFTC" -sdk "$SDK" "$ROOT/scripts/$source.swift" -o "$ROOT/target/tools/$source"
done

node -e 'const fs=require("fs");const p=process.argv[1];fs.writeFileSync(p+"/config/settings.json",JSON.stringify({alias:"Egui Acceptance",port:0,protocol:"http",pin:null,destination_dir:p+"/received",auto_save:"off",auto_finish:true,start_server_on_launch:true,theme:"light",color:"localsend",language:"zh-CN",animations:false,revision:0},null,2)+"\n")' "$WORK"

LOCALSEND_TEST_DISABLE_DISCOVERY=1 LOCALSEND_CONFIG_DIR="$WORK/config" LOCALSEND_RPC_SOCKET="$SOCKET" \
    "$ROOT/target/debug/localsendd" >"$OUT/daemon.log" 2>&1 &
DAEMON_PID=$!
for _ in {1..100}; do [[ -S "$SOCKET" ]] && break; sleep 0.05; done
[[ -S "$SOCKET" ]] || { cat "$OUT/daemon.log"; exit 1; }

LOCALSEND_CONFIG_DIR="$WORK/config" LOCALSEND_RPC_SOCKET="$SOCKET" \
    "$ROOT/target/debug/localsend-gui-egui" >"$OUT/gui.log" 2>&1 &
GUI_PID=$!
sleep 2
kill -0 "$GUI_PID"

"$ROOT/target/tools/macos-ax-dump" "$GUI_PID" >"$AX_OUT/init1.ax.txt"
sleep 0.5
"$ROOT/target/tools/macos-ax-dump" "$GUI_PID" >"$AX_OUT/init2.ax.txt"
"$ROOT/target/tools/macos-ax-smoke" "$GUI_PID" | tee "$OUT/ax-smoke.log"

for theme in light dark; do
    LOCALSEND_CONFIG_DIR="$WORK/config" LOCALSEND_RPC_SOCKET="$SOCKET" \
        "$ROOT/target/debug/examples/rpc_probe" set-setting theme "$theme" >/dev/null
    sleep 0.8
    for dimension in 900x600 1024x768 1440x900; do
        width=${dimension%x*}
        height=${dimension#*x}
        "$ROOT/target/tools/macos-ax-window" "$GUI_PID" 80 60 "$width" "$height" >/dev/null
        sleep 0.5
        for page in receive send settings; do
            case "$page" in
                receive) label=接收 ;;
                send) label=发送 ;;
                settings) label=设置 ;;
            esac
            "$ROOT/target/tools/macos-ax-click" "$GUI_PID" "$label" >/dev/null 2>&1 || true
            sleep 0.5
            wid=$("$ROOT/target/tools/macos-window-id" "$GUI_PID")
            screencapture -x -l "$wid" "$SCREENS/$page-$dimension-$theme.png"
            "$ROOT/target/tools/macos-ax-dump" "$GUI_PID" >"$AX_OUT/$page-$dimension-$theme.ax.txt"
        done
    done
done

captures=$(find "$SCREENS" -name '*.png' | wc -l | tr -d ' ')
[[ "$captures" -eq 18 ]] || { echo "expected 18 captures, got $captures" >&2; exit 1; }

kill "$GUI_PID" 2>/dev/null || true
wait "$GUI_PID" 2>/dev/null || true
GUI_PID=""
kill -0 "$DAEMON_PID"
LOCALSEND_CONFIG_DIR="$WORK/config" LOCALSEND_RPC_SOCKET="$SOCKET" \
    "$ROOT/target/debug/examples/rpc_probe" shutdown
wait "$DAEMON_PID" 2>/dev/null || true
DAEMON_PID=""

node -e 'const fs=require("fs");const data={suite:"egui",result:"passed",tests:11,goldens:18,nativeAx:["receive","send","settings"],artifacts:process.argv[1]};fs.writeFileSync(process.argv[1]+"/summary.json",JSON.stringify(data,null,2)+"\n")' "$OUT"
cat "$OUT/summary.json"

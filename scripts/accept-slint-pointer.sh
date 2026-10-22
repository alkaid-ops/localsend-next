#!/bin/zsh
set -euo pipefail

ROOT=$(cd "$(dirname "$0")/.." && pwd)
STAMP=$(date +%Y%m%d-%H%M%S)
OUT="$ROOT/target/acceptance/slint-pointer/$STAMP"
WORK=$(mktemp -d /tmp/localsend-slint-pointer.XXXXXX)
FIXTURE_PID=""
GUI_PID=""
RESULT="failed"
DIAGNOSTICS=0

mkdir -p "$OUT"

cleanup() {
    [[ -n "$GUI_PID" ]] && kill "$GUI_PID" 2>/dev/null || true
    touch "$WORK/stop" 2>/dev/null || true
    if [[ -f "$WORK/ready.json" ]]; then
        DAEMON_PID=$(python3 - "$WORK/ready.json" <<'PY'
import json, sys
print(json.load(open(sys.argv[1])).get("daemon_pid", 0))
PY
        )
        [[ "$DAEMON_PID" != 0 ]] && kill "$DAEMON_PID" 2>/dev/null || true
    fi
    [[ -n "$FIXTURE_PID" ]] && kill "$FIXTURE_PID" 2>/dev/null || true
    sleep 0.5
    [[ -n "$FIXTURE_PID" ]] && kill -KILL "$FIXTURE_PID" 2>/dev/null || true
    [[ -n "$GUI_PID" ]] && kill -KILL "$GUI_PID" 2>/dev/null || true
    [[ -n "$FIXTURE_PID" ]] && wait "$FIXTURE_PID" 2>/dev/null || true
    [[ -n "$GUI_PID" ]] && wait "$GUI_PID" 2>/dev/null || true
}

on_exit() {
    local code=$?
    if [[ $code -ne 0 && $DIAGNOSTICS -eq 0 ]]; then
        DIAGNOSTICS=1
        screencapture -x "$OUT/failure.png" 2>/dev/null || true
        cp "$WORK/status.json" "$OUT/status.json" 2>/dev/null || true
        cp "$WORK/ready.json" "$OUT/ready.json" 2>/dev/null || true
    fi
    cleanup
    exit $code
}

trap on_exit EXIT

cd "$ROOT"
cargo build -p localsendd -p localsend-gui-slint --locked | tee "$OUT/build.log"
cargo build -p localsend-gui-slint --example native_e2e_fixture --locked \
    | tee -a "$OUT/build.log"
cargo build -p localsendd --example rpc_probe --locked | tee -a "$OUT/build.log"

SWIFTC=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/swiftc
if [[ ! -x "$SWIFTC" ]]; then
    SWIFTC=$(xcrun --find swiftc 2>/dev/null || true)
fi
if [[ -z "$SWIFTC" || ! -x "$SWIFTC" ]]; then
    echo 'FAIL: Swift toolchain unavailable' | tee -a "$OUT/ax.log"
    exit 1
fi
SDK=$(xcrun --sdk macosx --show-sdk-path)
"$SWIFTC" -sdk "$SDK" scripts/macos-cgevent-e2e.swift -o "$OUT/macos-cgevent-e2e" \
    >"$OUT/swift-build.log" 2>&1

"$ROOT/target/debug/examples/native_e2e_fixture" "$WORK" >"$OUT/fixture.log" 2>&1 &
FIXTURE_PID=$!
for _ in {1..100}; do
    [[ -f "$WORK/ready.json" ]] && break
    sleep 0.1
done
if [[ ! -f "$WORK/ready.json" ]]; then
    echo 'FAIL: deterministic fixture did not become ready' | tee -a "$OUT/ax.log"
    exit 1
fi

ENDPOINT=$(python3 - "$WORK/ready.json" <<'PY'
import json, sys
print(json.load(open(sys.argv[1]))["endpoint"])
PY
)
CONFIG=$(python3 - "$WORK/ready.json" <<'PY'
import json, sys
print(json.load(open(sys.argv[1]))["config_dir"])
PY
)
FINGERPRINT=$(python3 - "$WORK/ready.json" <<'PY'
import json, sys
print(json.load(open(sys.argv[1]))["peer_fingerprint"])
PY
)

printf 'physical pointer file\n' > "$WORK/sample.txt"
mkdir -p "$WORK/folder/sub"
printf 'folder alpha\n' > "$WORK/folder/alpha.txt"
printf 'folder beta\n' > "$WORK/folder/sub/beta.txt"

TEXT_MESSAGE="physical text $STAMP"
CLIPBOARD_MESSAGE="physical clipboard $STAMP"

start_gui() {
    local scenario=$1
    LOCALSEND_CONFIG_DIR="$CONFIG" LOCALSEND_RPC_SOCKET="$ENDPOINT" \
    LOCALSEND_ACCEPTANCE=1 LOCALSEND_E2E_FILE_PATH="$WORK/sample.txt" \
    LOCALSEND_E2E_FOLDER_PATH="$WORK/folder" LOCALSEND_GUI_WIDTH=1024 \
    LOCALSEND_GUI_HEIGHT=768 LOCALSEND_GUI_TAB=send \
        "$ROOT/target/debug/localsend-gui-slint" --window >"$OUT/gui-$scenario.log" 2>&1 &
    GUI_PID=$!
    for _ in {1..100}; do
        kill -0 "$GUI_PID" 2>/dev/null || break
        sleep 0.1
    done
    kill -0 "$GUI_PID" 2>/dev/null || {
        echo "FAIL: GUI exited before $scenario pointer test" | tee -a "$OUT/ax.log"
        exit 1
    }
}

stop_gui() {
    [[ -n "$GUI_PID" ]] && kill "$GUI_PID" 2>/dev/null || true
    [[ -n "$GUI_PID" ]] && wait "$GUI_PID" 2>/dev/null || true
    GUI_PID=""
    sleep 0.3
}

assert_no_active_transfers() {
    local scenario=$1
    local snapshot
    local active
    for _ in {1..30}; do
        snapshot=$(LOCALSEND_CONFIG_DIR="$CONFIG" LOCALSEND_RPC_SOCKET="$ENDPOINT" \
            "$ROOT/target/debug/examples/rpc_probe" snapshot 2>>"$OUT/ax.log") || return 1
        active=$(python3 - "$snapshot" <<'PY'
import json, sys
print(len(json.loads(sys.argv[1]).get("active_transfers", [])))
PY
)
        [[ "$active" == "0" ]] && return 0
        sleep 0.1
    done
    echo "FAIL: $scenario left $active active transfer(s) in daemon" | tee -a "$OUT/ax.log"
    return 1
}

scenario_index=0
for scenario in file folder text; do
    scenario_index=$((scenario_index + 1))
    start_gui "$scenario"
    set +e
    "$OUT/macos-cgevent-e2e" "$GUI_PID" deterministic-peer "$FINGERPRINT" \
        "$TEXT_MESSAGE" "$CLIPBOARD_MESSAGE" "$WORK/status.json" \
        "$OUT/pointer-$scenario.json" "$scenario" "$scenario_index" | tee -a "$OUT/ax.log"
    AX_CODE=${pipestatus[1]}
    set -e
    stop_gui
    if [[ "$AX_CODE" -ne 0 ]]; then
        echo "FAIL: physical pointer driver failed for $scenario" | tee -a "$OUT/ax.log"
        exit 1
    fi
    assert_no_active_transfers "$scenario"
    sleep 0.3
done

python3 - "$WORK/status.json" "$OUT" "$WORK" "$TEXT_MESSAGE" "$CLIPBOARD_MESSAGE" <<'PY'
import hashlib
import json
import pathlib
import sys
import time

status_path = pathlib.Path(sys.argv[1])
output_dir = pathlib.Path(sys.argv[2])
root = pathlib.Path(sys.argv[3])
text_message = sys.argv[4]
clipboard_message = sys.argv[5]
deadline = time.monotonic() + 25
while True:
    status = json.loads(status_path.read_text())
    peer = status["peer"]
    if len(peer["requests"]) >= 3 and len(peer["received_files"]) >= 4:
        break
    assert time.monotonic() < deadline, status
    time.sleep(0.2)

pointers = [json.loads(path.read_text()) for path in sorted(output_dir.glob("pointer-*.json"))]
assert len(pointers) == 3, pointers
for pointer in pointers:
    assert pointer["executed"] is True, pointer
    assert pointer["physical_pointer"] is True, pointer
    assert pointer["clicks"], pointer
    for click in pointer["clicks"]:
        assert click["window_before"] == click["window_after"], click
        assert click["scale_before"] == click["scale_after"], click

assert not peer["errors"], peer["errors"]
assert len(peer["requests"]) == 3, peer["requests"]
assert [sorted(request["file_names"]) for request in peer["requests"]] == [
    ["sample.txt"],
    ["folder/alpha.txt", "folder/sub/beta.txt"],
    ["message.txt"],
], peer["requests"]
assert peer["requests"][2]["previews"] == [text_message], peer["requests"][2]

received = {
    item["file_name"]: item
    for item in peer["received_files"]
    if item["file_name"] != "message.txt"
}
assert set(received) == {"sample.txt", "folder/alpha.txt", "folder/sub/beta.txt"}, received

def digest(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()

for source, remote in [
    (root / "sample.txt", received["sample.txt"]),
    (root / "folder/alpha.txt", received["folder/alpha.txt"]),
    (root / "folder/sub/beta.txt", received["folder/sub/beta.txt"]),
]:
    assert digest(source) == digest(remote["path"]), (source, remote)
PY

touch "$WORK/stop"
sleep 0.5
if kill -0 "$FIXTURE_PID" 2>/dev/null; then
    echo 'FAIL: deterministic fixture did not stop cleanly' | tee -a "$OUT/cleanup.log"
    exit 1
fi
FIXTURE_PID=""

python3 - "$OUT" "$WORK/status.json" "$OUT/summary.json" <<'PY'
import json
import pathlib
import sys

output_dir = pathlib.Path(sys.argv[1])
status = json.loads(pathlib.Path(sys.argv[2]).read_text())
pointers = [json.loads(path.read_text()) for path in sorted(output_dir.glob("pointer-*.json"))]
summary = {
    "suite": "slint-physical-pointer",
    "result": "passed",
    "executed": True,
    "physical_pointer": True,
    "clicks": [click for pointer in pointers for click in pointer["clicks"]],
    "requests": len(status["peer"]["requests"]),
    "files": len(status["peer"]["received_files"]),
    "processes_cleaned": True,
}
pathlib.Path(sys.argv[3]).write_text(json.dumps(summary, indent=2, ensure_ascii=False) + "\n")
print(json.dumps(summary, indent=2, ensure_ascii=False))
PY

RESULT="passed"
echo "PASS: $OUT"

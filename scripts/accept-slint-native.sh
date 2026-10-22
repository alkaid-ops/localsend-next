#!/bin/zsh
set -euo pipefail

ROOT=$(cd "$(dirname "$0")/.." && pwd)
STAMP=$(date +%Y%m%d-%H%M%S)
OUT="$ROOT/target/acceptance/slint-native/$STAMP"
WORK=$(mktemp -d /tmp/localsend-slint-native.XXXXXX)
FIXTURE_PID=""
GUI_PID=""

mkdir -p "$OUT" "$WORK/folder/sub"
printf 'native ax file e2e\n' > "$WORK/sample.txt"
printf 'folder alpha\n' > "$WORK/folder/alpha.txt"
printf 'folder beta\n' > "$WORK/folder/sub/beta.txt"

cleanup() {
    [[ -n "$GUI_PID" ]] && kill "$GUI_PID" 2>/dev/null || true
    touch "$WORK/stop" 2>/dev/null || true
    if [[ -f "$WORK/ready.json" ]]; then
        DAEMON_PID=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1])).get("daemon_pid", 0))' "$WORK/ready.json" 2>/dev/null || echo 0)
        if [[ "$DAEMON_PID" != 0 ]]; then kill "$DAEMON_PID" 2>/dev/null || true; fi
    fi
    for _ in {1..40}; do
        [[ -z "$FIXTURE_PID" ]] && break
        kill -0 "$FIXTURE_PID" 2>/dev/null || break
        sleep 0.1
    done
    [[ -n "$FIXTURE_PID" ]] && kill -KILL "$FIXTURE_PID" 2>/dev/null || true
    [[ -n "$FIXTURE_PID" ]] && wait "$FIXTURE_PID" 2>/dev/null || true
}
trap cleanup EXIT

cd "$ROOT"
cargo build -p localsendd | tee "$OUT/build-daemon.log"
cargo build -p localsend-gui-slint | tee "$OUT/build-gui.log"
cargo build -p localsend-gui-slint --example native_e2e_fixture \
    | tee "$OUT/build-fixture.log"

SWIFTC=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/swiftc
if [[ ! -x "$SWIFTC" ]]; then
    SWIFTC=$(xcrun --find swiftc 2>/dev/null || true)
fi
if [[ -z "$SWIFTC" || ! -x "$SWIFTC" ]]; then
    printf '{"suite":"slint-native","result":"blocked","reason":"Swift toolchain unavailable","artifacts":"%s"}\n' "$OUT" \
        | tee "$OUT/summary.json"
    exit 2
fi
SDK=$(xcrun --sdk macosx --show-sdk-path)
"$SWIFTC" -sdk "$SDK" scripts/macos-ax-e2e.swift -o "$OUT/macos-ax-e2e" \
    >"$OUT/swift-build.log" 2>&1

"$ROOT/target/debug/examples/native_e2e_fixture" "$WORK" \
    >"$OUT/fixture.log" 2>&1 &
FIXTURE_PID=$!
for _ in {1..100}; do
    [[ -f "$WORK/ready.json" ]] && break
    sleep 0.1
done
[[ -f "$WORK/ready.json" ]] || {
    cat "$OUT/fixture.log" >&2
    exit 1
}

ENDPOINT=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["endpoint"])' "$WORK/ready.json")
CONFIG=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["config_dir"])' "$WORK/ready.json")

LOCALSEND_CONFIG_DIR="$CONFIG" LOCALSEND_RPC_SOCKET="$ENDPOINT" \
LOCALSEND_ACCEPTANCE=1 LOCALSEND_E2E_FILE_PATH="$WORK/sample.txt" \
LOCALSEND_E2E_FOLDER_PATH="$WORK/folder" LOCALSEND_GUI_WIDTH=1024 \
LOCALSEND_GUI_HEIGHT=768 \
    "$ROOT/target/debug/localsend-gui-slint" --window \
    >"$OUT/gui.log" 2>&1 &
GUI_PID=$!

for _ in {1..100}; do
    kill -0 "$GUI_PID" 2>/dev/null || break
    sleep 0.1
done
kill -0 "$GUI_PID" 2>/dev/null || {
    cat "$OUT/gui.log" >&2
    exit 1
}

TEXT_MESSAGE="AX text e2e $STAMP"
CLIPBOARD_MESSAGE="AX clipboard e2e $STAMP"
set +e
"$OUT/macos-ax-e2e" "$GUI_PID" deterministic-peer "$TEXT_MESSAGE" "$CLIPBOARD_MESSAGE" \
    | tee "$OUT/ax.log"
AX_CODE=${pipestatus[1]}
set -e
if [[ "$AX_CODE" -eq 2 ]]; then
    printf '{"suite":"slint-native","result":"blocked","reason":"Accessibility permission unavailable","artifacts":"%s"}\n' "$OUT" \
        | tee "$OUT/summary.json"
    exit 2
elif [[ "$AX_CODE" -ne 0 ]]; then
    printf '{"suite":"slint-native","result":"failed","stage":"ax","artifacts":"%s"}\n' "$OUT" \
        | tee "$OUT/summary.json"
    exit 1
fi

sleep 1
python3 - "$WORK/status.json" "$WORK" "$TEXT_MESSAGE" "$CLIPBOARD_MESSAGE" "$OUT/summary.json" <<'PY'
import hashlib
import json
import pathlib
import sys
import time

status_path, root, text_message, clipboard_message, summary_path = map(pathlib.Path, sys.argv[1:])
deadline = time.monotonic() + 20
while True:
    status = json.loads(status_path.read_text())
    peer = status["peer"]
    requests = peer["requests"]
    if len(requests) >= 4 and len(peer["received_files"]) >= 5:
        break
    assert time.monotonic() < deadline, status
    time.sleep(0.2)

assert not peer["errors"], peer["errors"]
assert len(requests) == 4, requests
assert [sorted(request["file_names"]) for request in requests] == [
    ["sample.txt"],
    ["folder/alpha.txt", "folder/sub/beta.txt"],
    ["message.txt"],
    ["message.txt"],
], requests
assert requests[2]["previews"] == [str(text_message)], requests[2]
assert requests[3]["previews"] == [str(clipboard_message)], requests[3]
received = {
    item["file_name"]: item
    for item in peer["received_files"]
    if item["file_name"] != "message.txt"
}
assert set(received) == {"sample.txt", "folder/alpha.txt", "folder/sub/beta.txt"}, received
messages = sorted(
    pathlib.Path(item["path"]).read_text()
    for item in peer["received_files"]
    if item["file_name"] == "message.txt"
)
assert messages == sorted([str(text_message), str(clipboard_message)]), messages

def digest(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()

for source, remote in [
    (root / "sample.txt", received["sample.txt"]),
    (root / "folder/alpha.txt", received["folder/alpha.txt"]),
    (root / "folder/sub/beta.txt", received["folder/sub/beta.txt"]),
]:
    assert digest(source) == digest(remote["path"]), (source, remote)

summary = {
    "suite": "slint-native",
    "result": "passed",
    "driver": "macOS Accessibility semantic tree (AXUIElement)",
    "scenarios": ["file", "folder", "text", "clipboard"],
    "requests": len(requests),
    "files": len(received),
    "text_preview": str(text_message),
    "clipboard_preview": str(clipboard_message),
    "artifacts": str(summary_path.parent),
}
summary_path.write_text(json.dumps(summary, ensure_ascii=False, indent=2) + "\n")
print(json.dumps(summary, ensure_ascii=False, indent=2))
PY

#!/bin/zsh
set -euo pipefail

ROOT=$(cd "$(dirname "$0")/.." && pwd)
STAMP=$(date +%Y%m%d-%H%M%S)
OUT="$ROOT/target/acceptance/slint-receive-settings/$STAMP"
WORK=$(mktemp -d /tmp/localsend-slint-receive.XXXXXX)
FIXTURE_PID=""
GUI_PID=""

mkdir -p "$OUT"
printf 'partial alpha\n' > "$WORK/partial-a.txt"
printf 'partial beta\n' > "$WORK/partial-b.txt"
printf 'reject this\n' > "$WORK/reject-me.txt"
printf 'wrong pin\n' > "$WORK/wrong-pin.txt"
printf 'auto saved\n' > "$WORK/auto-pin.txt"
printf 'manual accepted\n' > "$WORK/manual-pin.txt"

cleanup() {
    [[ "${LOCALSEND_KEEP_E2E:-0}" == "1" ]] && return
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
    printf '{"suite":"slint-receive-settings","result":"blocked","reason":"Swift toolchain unavailable","artifacts":"%s"}\n' "$OUT" \
        | tee "$OUT/summary.json"
    exit 2
fi
SDK=$(xcrun --sdk macosx --show-sdk-path)
"$SWIFTC" -sdk "$SDK" scripts/macos-ax-receive-settings-e2e.swift -o "$OUT/macos-ax-receive-settings-e2e" \
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
LOCALSEND_ACCEPTANCE=1 LOCALSEND_GUI_WIDTH=1200 LOCALSEND_GUI_HEIGHT=900 \
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

set +e
"$OUT/macos-ax-receive-settings-e2e" "$GUI_PID" "$WORK/actions" "$WORK/status.json" 2468 \
    "receive text e2e $STAMP" | tee "$OUT/ax.log"
AX_CODE=${pipestatus[1]}
set -e
if [[ "$AX_CODE" -eq 2 ]]; then
    printf '{"suite":"slint-receive-settings","result":"blocked","reason":"Accessibility permission unavailable","artifacts":"%s"}\n' "$OUT" \
        | tee "$OUT/summary.json"
    exit 2
elif [[ "$AX_CODE" -ne 0 ]]; then
    printf '{"suite":"slint-receive-settings","result":"failed","stage":"ax","artifacts":"%s"}\n' "$OUT" \
        | tee "$OUT/summary.json"
    exit 1
fi

sleep 1
python3 - "$WORK/status.json" "$WORK" "receive text e2e $STAMP" "$OUT/summary.json" <<'PY'
import hashlib
import json
import pathlib
import sys
import time

status_path, root, message, summary_path = map(pathlib.Path, sys.argv[1:])
deadline = time.monotonic() + 30
while True:
    status = json.loads(status_path.read_text())
    results = status["incoming_results"]
    if len(results) >= 6:
        break
    assert time.monotonic() < deadline, status
    time.sleep(0.2)

assert [result["kind"] for result in results] == [
    "send_files",
    "send_files",
    "send_text",
    "send_files",
    "send_files",
    "send_files",
], results

partial, rejected, text_result, quick_saved, wrong_pin, manual_pin = results
assert partial["ok"] and len(partial["accepted_file_ids"]) == 1 and len(partial["refused_file_ids"]) == 1, partial
assert rejected["ok"] and not rejected["accepted_file_ids"] and len(rejected["refused_file_ids"]) == 1, rejected
assert text_result["ok"], text_result
assert quick_saved["ok"] and len(quick_saved["accepted_file_ids"]) == 1, quick_saved
assert not wrong_pin["ok"] and "401" in (wrong_pin["error"] or ""), wrong_pin
assert manual_pin["ok"] and len(manual_pin["accepted_file_ids"]) == 1, manual_pin

settings = status["settings"]
assert settings["pin"] == "2468", settings
assert settings["auto_finish"] is True, settings
assert settings["auto_save"] == "off", settings

destination = pathlib.Path(json.loads((root / "ready.json").read_text())["destination"])
assert (destination / "partial-a.txt").exists()
assert not (destination / "partial-b.txt").exists()
assert not (destination / "reject-me.txt").exists()
assert not (destination / "wrong-pin.txt").exists()
assert (destination / "auto-pin.txt").exists()
assert (destination / "manual-pin.txt").exists()

def digest(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()

for name in ["partial-a.txt", "auto-pin.txt", "manual-pin.txt"]:
    assert digest(root / name) == digest(destination / name), name

history = status["history"]
assert any(entry.get("message") == str(message) for entry in history), history

summary = {
    "suite": "slint-receive-settings",
    "result": "passed",
    "driver": "macOS Accessibility semantic tree (AXUIElement)",
    "scenarios": [
        "partial_accept",
        "reject",
        "incoming_text",
        "settings_quick_save",
        "settings_auto_finish",
        "settings_pin",
        "wrong_pin",
        "pin_auto_receive",
        "pin_manual_accept",
    ],
    "incoming_actions": len(results),
    "history_entries": len(history),
    "artifacts": str(summary_path.parent),
}
summary_path.write_text(json.dumps(summary, ensure_ascii=False, indent=2) + "\n")
print(json.dumps(summary, ensure_ascii=False, indent=2))
PY

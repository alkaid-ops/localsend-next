#!/bin/zsh
set -euo pipefail

ROOT=$(cd "$(dirname "$0")/.." && pwd)
STAMP=$(date +%Y%m%d-%H%M%S)
OUT="$ROOT/target/acceptance/sender-kill/$STAMP"
WORK=$(mktemp -d /tmp/localsend-sender-kill.XXXXXX)
CONFIG="$WORK/config"
RECEIVED="$WORK/received"
SOCKET="$WORK/daemon.sock"
DAEMON_PID=""
SENDER_PID=""

mkdir -p "$OUT" "$CONFIG" "$RECEIVED"

cleanup() {
    [[ -n "$SENDER_PID" ]] && kill -KILL "$SENDER_PID" 2>/dev/null || true
    [[ -n "$DAEMON_PID" ]] && kill "$DAEMON_PID" 2>/dev/null || true
    [[ -n "$SENDER_PID" ]] && wait "$SENDER_PID" 2>/dev/null || true
    [[ -n "$DAEMON_PID" ]] && wait "$DAEMON_PID" 2>/dev/null || true
}
trap cleanup EXIT

cd "$ROOT"
cargo build -q -p localsendd --example rpc_probe | tee "$OUT/build-rpc.log"
cargo build -q -p localsendd | tee "$OUT/build-daemon.log"

PORT=$(python3 - <<'PY'
import socket
s = socket.socket()
s.bind(("127.0.0.1", 0))
print(s.getsockname()[1])
s.close()
PY
)

printf '%s\n' '{"alias":"Sender Kill Receiver","port":'"$PORT"',"protocol":"http","destination_dir":"'"$RECEIVED"'","auto_save":"on","start_server_on_launch":true,"revision":0}' > "$CONFIG/settings.json"

LOCALSEND_CONFIG_DIR="$CONFIG" LOCALSEND_RPC_SOCKET="$SOCKET" \
    "$ROOT/target/debug/localsendd" >"$OUT/daemon.log" 2>&1 &
DAEMON_PID=$!

for _ in {1..100}; do
    [[ -S "$SOCKET" ]] && break
    sleep 0.05
done
[[ -S "$SOCKET" ]] || { cat "$OUT/daemon.log" >&2; exit 1; }

python3 - "$PORT" <<'PY'
import socket
import sys
import time

port = int(sys.argv[1])
deadline = time.monotonic() + 10
while time.monotonic() < deadline:
    try:
        with socket.create_connection(("127.0.0.1", port), timeout=0.2):
            raise SystemExit(0)
    except OSError:
        time.sleep(0.05)
raise SystemExit("daemon HTTP server did not start")
PY

mkfile -n 32m "$WORK/large.bin"
printf 'after kill\n' > "$WORK/small.txt"

slow_sender() {
    exec python3 - "$PORT" "$1" "$2" "$3" <<'PY' >"$4" 2>&1
import hashlib
import http.client
import json
import pathlib
import sys
import time
import uuid

port = int(sys.argv[1])
path = pathlib.Path(sys.argv[2])
rate = float(sys.argv[3])
marker = pathlib.Path(sys.argv[4])
file_id = str(uuid.uuid4())
size = path.stat().st_size
payload = {
    "info": {
        "alias": "independent-sender",
        "version": "2.2",
        "deviceModel": "testkit",
        "deviceType": "desktop",
        "fingerprint": hashlib.sha256(str(uuid.uuid4()).encode()).hexdigest().upper(),
        "port": 0,
        "protocol": "http",
        "hasWebInterface": False,
    },
    "files": {
        file_id: {
            "id": file_id,
            "fileName": path.name,
            "size": size,
            "fileType": "application/octet-stream",
            "sha256": None,
            "preview": None,
            "metadata": None,
        }
    },
}

conn = http.client.HTTPConnection("127.0.0.1", port, timeout=10)
body = json.dumps(payload).encode()
conn.request(
    "POST",
    "/api/localsend/v2/prepare-upload",
    body=body,
    headers={"Content-Type": "application/json", "Content-Length": str(len(body))},
)
response = conn.getresponse()
raw = response.read()
if response.status != 200:
    raise SystemExit(f"prepare failed {response.status}: {raw[:200]!r}")
prepared = json.loads(raw)
session_id = prepared["sessionId"]
token = prepared["files"][file_id]

conn = http.client.HTTPConnection("127.0.0.1", port, timeout=120)
target = (
    "/api/localsend/v2/upload?sessionId=" + session_id
    + "&fileId=" + file_id
    + "&token=" + token
)
conn.putrequest("POST", target)
conn.putheader("Content-Length", str(size))
conn.endheaders()
marker.write_text("started")
with path.open("rb") as source:
    sent = 0
    while chunk := source.read(64 * 1024):
        conn.send(chunk)
        sent += len(chunk)
        if rate > 0:
            time.sleep(len(chunk) / rate)
response = conn.getresponse()
response.read()
if response.status != 200:
    raise SystemExit(f"upload failed {response.status}")
print(json.dumps({"file_id": file_id, "size": sent}))
PY
}

slow_sender "$WORK/large.bin" 1048576 "$WORK/upload-started" "$OUT/slow-sender.log" &
SENDER_PID=$!
for _ in {1..100}; do
    [[ -f "$WORK/upload-started" ]] && break
    sleep 0.05
done
[[ -f "$WORK/upload-started" ]] || { cat "$OUT/slow-sender.log" >&2; exit 1; }
sleep 0.5
kill -KILL "$SENDER_PID"
wait "$SENDER_PID" 2>/dev/null || true
SENDER_PID=""
sleep 5

LOCALSEND_CONFIG_DIR="$CONFIG" LOCALSEND_RPC_SOCKET="$SOCKET" \
    "$ROOT/target/debug/examples/rpc_probe" > "$OUT/after-kill.json"

if find "$RECEIVED" -type f | grep -q .; then
    printf 'partial file remained after sender SIGKILL\n' >&2
    find "$RECEIVED" -type f >&2
    exit 1
fi

slow_sender "$WORK/small.txt" 0 "$WORK/second-started" "$OUT/second-sender.log" &
SENDER_PID=$!
wait "$SENDER_PID"
SENDER_PID=""
[[ -f "$RECEIVED/small.txt" ]] || { printf 'second transfer did not complete\n' >&2; exit 1; }

printf '{"suite":"sender-kill","result":"passed","port":%s,"artifacts":"%s"}\n' "$PORT" "$OUT" \
    | tee "$OUT/summary.json"


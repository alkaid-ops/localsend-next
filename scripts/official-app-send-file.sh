#!/bin/zsh
set -euo pipefail

if [[ $# -lt 6 || $# -gt 7 ]]; then
    echo "usage: official-app-send-file.sh PID ALIAS FILE_OR_DIRECTORY RECEIVE_DIR OUTPUT_DIR TIMEOUT_SECONDS [PIN]" >&2
    exit 64
fi

PID=$1
ALIAS=$2
SOURCE=$(cd "$(dirname "$3")" && pwd)/$(basename "$3")
RECEIVE_DIR=$4
OUTPUT_DIR=$5
TIMEOUT_SECONDS=$6
PIN=${7:-}
ROOT=$(cd "$(dirname "$0")/.." && pwd)
SWIFTC=${SWIFTC:-/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/swiftc}
SDK=$(xcrun --sdk macosx --show-sdk-path)

mkdir -p "$RECEIVE_DIR" "$OUTPUT_DIR"
"$SWIFTC" -sdk "$SDK" "$ROOT/scripts/macos-ax-dump.swift" -o "$OUTPUT_DIR/ax-dump"
"$SWIFTC" -sdk "$SDK" "$ROOT/scripts/macos-ax-click.swift" -o "$OUTPUT_DIR/ax-click"
"$SWIFTC" -sdk "$SDK" "$ROOT/scripts/macos-ax-press-button.swift" -o "$OUTPUT_DIR/press-button"
"$SWIFTC" -sdk "$SDK" "$ROOT/scripts/macos-ax-press-empty.swift" -o "$OUTPUT_DIR/press-empty"
"$SWIFTC" -sdk "$SDK" "$ROOT/scripts/macos-ax-press-at.swift" -o "$OUTPUT_DIR/press-at"
"$SWIFTC" -sdk "$SDK" "$ROOT/scripts/macos-ax-scroll.swift" -o "$OUTPUT_DIR/ax-scroll"
"$SWIFTC" -sdk "$SDK" "$ROOT/scripts/macos-ax-text.swift" -o "$OUTPUT_DIR/ax-text"
"$SWIFTC" -sdk "$SDK" "$ROOT/scripts/macos-ax-window.swift" -o "$OUTPUT_DIR/ax-window"
"$SWIFTC" -sdk "$SDK" "$ROOT/scripts/macos-window-id.swift" -o "$OUTPUT_DIR/window-id"
"$SWIFTC" -sdk "$SDK" "$ROOT/scripts/macos-click.swift" -o "$OUTPUT_DIR/click"
"$SWIFTC" -sdk "$SDK" "$ROOT/scripts/macos-ocr.swift" -o "$OUTPUT_DIR/ocr"

TOKEN=$RANDOM
if [[ -d "$SOURCE" ]]; then
    KIND=folder
    BASENAME="ls-e2e-$TOKEN"
    OCR_FILTER="$TOKEN"
else
    KIND=file
    EXTENSION=${SOURCE##*.}
    [[ "$EXTENSION" == "$SOURCE" ]] && EXTENSION=bin
    BASENAME="ls-e2e-$TOKEN.$EXTENSION"
    OCR_FILTER="$TOKEN.$EXTENSION"
fi
DOWNLOADS="$HOME/Downloads/$BASENAME"
if [[ "$KIND" == folder ]]; then
    cp -cR "$SOURCE" "$DOWNLOADS" 2>/dev/null || cp -R "$SOURCE" "$DOWNLOADS"
else
    cp -c "$SOURCE" "$DOWNLOADS" 2>/dev/null || cp "$SOURCE" "$DOWNLOADS"
fi
cleanup() {
    [[ -d "$DOWNLOADS" ]] && rm -rf "$DOWNLOADS" 2>/dev/null || true
    [[ -f "$DOWNLOADS" ]] && unlink "$DOWNLOADS" 2>/dev/null || true
}
trap cleanup EXIT

open -a /Applications/LocalSendy.app
sleep 0.6
"$OUTPUT_DIR/ax-window" "$PID" 590 25 900 600 >/dev/null 2>&1 || true
"$OUTPUT_DIR/ax-click" "$PID" '发送' >/dev/null
sleep 0.5

"$OUTPUT_DIR/ax-dump" "$PID" > "$OUTPUT_DIR/before.ax.txt"
if rg -q '文件：' "$OUTPUT_DIR/before.ax.txt"; then
    "$OUTPUT_DIR/press-empty" "$PID" '选择' >/dev/null || true
    sleep 0.4
fi

FOLDER_LABEL='文件'
[[ "$KIND" == folder ]] && FOLDER_LABEL='文件夹'
"$OUTPUT_DIR/press-button" "$PID" "$FOLDER_LABEL" >/dev/null
sleep 0.8
"$OUTPUT_DIR/ax-dump" "$PID" > "$OUTPUT_DIR/picker.ax.txt"
read -r FX FY <<<"$(python3 - "$OUTPUT_DIR/picker.ax.txt" "$OCR_FILTER" <<'PY'
import re, sys
token = sys.argv[2]
pattern = re.compile(
    r'AX(?:TextField|StaticText) (\d+),(\d+) (\d+)x(\d+).*' + re.escape(token),
    re.IGNORECASE,
)
for line in open(sys.argv[1], errors='replace'):
    match = pattern.search(line)
    if not match:
        continue
    x, y, width, height = map(int, match.groups())
    if width > 20 and height > 8:
        print(f'{x + width / 2:.1f} {y + height / 2:.1f}')
        break
PY
)"
if [[ -z "$FX" ]]; then
    screencapture -x "$OUTPUT_DIR/picker.png"
    LINE=$("$OUTPUT_DIR/ocr" "$OUTPUT_DIR/picker.png" "$OCR_FILTER" | head -1)
    [[ -n "$LINE" ]] || { echo "fixture not visible in file picker AX/OCR" >&2; exit 1; }
    read -r FX FY <<<"$(printf '%s\n' "$LINE" | awk -F'logical ' '{split($2,a," "); printf "%.1f %.1f", a[1]+a[3]/2, a[2]+a[4]/2}')"
fi
open -a /Applications/LocalSendy.app
sleep 0.3
"$OUTPUT_DIR/click" "$FX" "$FY"
sleep 0.35
open -a /Applications/LocalSendy.app
sleep 0.25
"$OUTPUT_DIR/press-button" "$PID" '打开' >/dev/null
sleep 0.9

"$OUTPUT_DIR/ax-dump" "$PID" > "$OUTPUT_DIR/selected.ax.txt"
rg -q '文件：|文件夹：' "$OUTPUT_DIR/selected.ax.txt" || { echo "selection did not appear" >&2; exit 1; }

# Return to the top, force single-recipient mode, then search downward for
# the selected target. The official app retains its scroll position and send
# mode across runs, which otherwise makes the target row disappear.
for _ in {1..8}; do
    "$OUTPUT_DIR/ax-scroll" "$PID" 1500 >/dev/null
    sleep 0.15
done
"$OUTPUT_DIR/ax-dump" "$PID" > "$OUTPUT_DIR/window.ax.txt"
GEAR=$(python3 - "$OUTPUT_DIR/window.ax.txt" <<'PY'
import re,sys
for line in open(sys.argv[1]):
    match=re.search(r'AXWindow (\d+),(\d+) (\d+)x(\d+)',line)
    if match:
        x,y,w,h=map(int,match.groups())
        print(f'{x + w * 0.586:.1f} {y + h * 0.58:.1f}')
        break
PY
)
if [[ -n "$GEAR" ]]; then
    "$OUTPUT_DIR/click" ${=GEAR}
    sleep 0.5
    "$OUTPUT_DIR/ax-dump" "$PID" > "$OUTPUT_DIR/send-mode.ax.txt"
    if rg -q '一个接收者' "$OUTPUT_DIR/send-mode.ax.txt"; then
        "$OUTPUT_DIR/press-button" "$PID" '一个接收者' >/dev/null
        sleep 0.4
    fi
fi

TARGET=""
for _ in {1..12}; do
    "$OUTPUT_DIR/ax-dump" "$PID" > "$OUTPUT_DIR/current.ax.txt"
    TARGET=$(python3 - "$OUTPUT_DIR/current.ax.txt" "$ALIAS" <<'PY'
import re,sys
pattern=re.compile(r'AXGroup (\d+),(\d+) (\d+)x(\d+).*' + re.escape(sys.argv[2]))
for line in open(sys.argv[1]):
    match=pattern.search(line)
    if not match:
        continue
    x,y,w,h=map(int,match.groups())
    if h>20 and 50<=y<=600:
        print(f'{x+w/2:.1f} {y+h/2:.1f}')
        break
PY
)
    [[ -n "$TARGET" ]] && break
    "$OUTPUT_DIR/ax-scroll" "$PID" -600 >/dev/null
    sleep 0.35
done
[[ -n "$TARGET" ]] || { echo "target not visible: $ALIAS" >&2; exit 1; }
read -r TX TY <<<"$TARGET"

find "$RECEIVE_DIR" -type f -delete
START=$(python3 -c 'import time; print(time.time_ns())')
open -a /Applications/LocalSendy.app
sleep 0.25
"$OUTPUT_DIR/click" "$TX" "$TY"
if [[ -n "$PIN" ]]; then
    PIN_VISIBLE=false
    for _ in {1..30}; do
        "$OUTPUT_DIR/ax-dump" "$PID" > "$OUTPUT_DIR/pin-dialog.ax.txt"
        if rg -q '输入 PIN' "$OUTPUT_DIR/pin-dialog.ax.txt"; then
            PIN_VISIBLE=true
            break
        fi
        sleep 0.2
    done
    [[ "$PIN_VISIBLE" == true ]] || { echo "PIN dialog did not appear" >&2; exit 1; }
    "$OUTPUT_DIR/ax-text" "$PID" "keys:$PIN" >/dev/null
    "$OUTPUT_DIR/press-button" "$PID" '确认' >/dev/null
    sleep 0.5
fi
EXPECTED_FILES=$(find "$SOURCE" -type f | wc -l | tr -d ' ')
EXPECTED_SIZE=$(find "$SOURCE" -type f -exec stat -f %z {} + | awk '{sum += $1} END {print sum + 0}')
FINISHED=""
LAST_TREE=""
STABLE=0
for index in $(seq 1 $((TIMEOUT_SECONDS * 50))); do
    CURRENT_FILES=$(find "$RECEIVE_DIR" -type f ! -name '*.part' | wc -l | tr -d ' ')
    CURRENT_SIZE=$(find "$RECEIVE_DIR" -type f ! -name '*.part' -exec stat -f %z {} + 2>/dev/null | awk '{sum += $1} END {print sum + 0}')
    CURRENT_TREE="$CURRENT_FILES:$CURRENT_SIZE"
    if [[ "$CURRENT_FILES" -eq "$EXPECTED_FILES" && "$CURRENT_SIZE" -eq "$EXPECTED_SIZE" ]]; then
        if [[ "$CURRENT_TREE" == "$LAST_TREE" ]]; then
            STABLE=$((STABLE + 1))
        else
            STABLE=0
        fi
        if [[ "$STABLE" -ge 2 ]]; then
            FINISHED=complete
            break
        fi
    else
        STABLE=0
    fi
    LAST_TREE=$CURRENT_TREE
    if (( index == 5 || index == 25 || index == 50 || index == 100 || index == 200 )); then
        WID=$("$OUTPUT_DIR/window-id" "$PID")
        screencapture -x -l "$WID" "$OUTPUT_DIR/progress-$index.png"
    fi
    sleep 0.02
done
END=$(python3 -c 'import time; print(time.time_ns())')
WID=$("$OUTPUT_DIR/window-id" "$PID")
screencapture -x -l "$WID" "$OUTPUT_DIR/finished.png"

python3 - "$SOURCE" "$FINISHED" "$START" "$END" "$OUTPUT_DIR/result.json" "$RECEIVE_DIR" <<'PY'
import hashlib,json,pathlib,sys
source=pathlib.Path(sys.argv[1]); complete=bool(sys.argv[2]); receive_root=pathlib.Path(sys.argv[6])
start,end=int(sys.argv[3]),int(sys.argv[4])
def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()
source_files = [source] if source.is_file() else sorted(p for p in source.rglob('*') if p.is_file())
received_files = sorted(p for p in receive_root.rglob('*') if p.is_file() and not p.name.endswith('.part'))
def rel(path):
    return path.name if source.is_file() else path.relative_to(source).as_posix()
source_hashes = {rel(p): digest(p) for p in source_files}
received_hashes = {}
for p in received_files:
    if source.is_file():
        received_hashes[p.name] = digest(p)
    else:
        parts = p.relative_to(receive_root).parts
        # The harness selects a randomly named copy in Downloads, so compare
        # the tree below its selected root rather than that temporary root name.
        received_hashes['/'.join(parts[1:])] = digest(p)
result={
    "source": str(source),
    "kind": "folder" if source.is_dir() else "file",
    "received_root": str(receive_root),
    "source_size": sum(p.stat().st_size for p in source_files),
    "received_size": sum(p.stat().st_size for p in received_files),
    "source_sha256": source_hashes,
    "received_sha256": received_hashes,
    "elapsed_ms": round((end-start)/1e6,2),
    "status": "PASS" if complete and source_hashes == received_hashes else "FAIL",
}
pathlib.Path(sys.argv[5]).write_text(json.dumps(result,ensure_ascii=False,indent=2)+"\n")
print(json.dumps(result,ensure_ascii=False,indent=2))
PY

#!/bin/zsh
set -euo pipefail

ROOT=$(cd "$(dirname "$0")/.." && pwd)
APP_DIR="$ROOT/apps/localsend-gui-slint"
GOLDEN="$APP_DIR/reference/slint"
UPDATE=0
OUT=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --update-goldens) UPDATE=1; shift ;;
        --output) OUT=$2; shift 2 ;;
        *) echo "usage: check-slint-goldens.sh [--update-goldens] [--output DIR]" >&2; exit 64 ;;
    esac
done

if [[ -z "$OUT" ]]; then
    STAMP=$(date +%Y%m%d-%H%M%S)
    OUT="$ROOT/target/acceptance/slint-goldens/$STAMP"
fi
mkdir -p "$OUT"

cd "$ROOT"
cargo build -p localsend-gui-slint --example render_reference --locked | tee "$OUT/build.log"

render() {
    local output=$1
    shift
    cargo run -q -p localsend-gui-slint --example render_reference -- \
        --headless --output "$output" "$@"
}

check_one() {
    local name=$1
    shift
    if [[ $UPDATE -eq 1 ]]; then
        render "$GOLDEN/$name" "$@"
    else
        render "$OUT/$name" "$@"
        cmp "$OUT/$name" "$GOLDEN/$name" >/dev/null || {
            echo "golden mismatch: $name" >&2
            exit 1
        }
    fi
}

for page in receive send settings; do
    for dimension in 800x600 1024x768 1440x900 700x700; do
        width=${dimension%x*}
        height=${dimension#*x}
        for theme in light dark; do
            name="$page-default-$dimension-$theme.png"
            check_one "$name" --page "$page" --state default --width "$width" --height "$height" --theme "$theme"
        done
    done
done

for state_page in receive:request receive:transferring receive:completion receive:web-share send:selected send:text send:multiple send:transferring send:completed; do
    page=${state_page%%:*}
    state=${state_page#*:}
    for theme in light dark; do
        name="$page-$state-1024x768-$theme.png"
        check_one "$name" --page "$page" --state "$state" --width 1024 --height 768 --theme "$theme"
    done
done

if [[ $UPDATE -eq 1 ]]; then
    MODE=updated
else
    MODE=passed
fi
printf '{"suite":"slint-goldens","result":"%s","artifacts":"%s"}\n' "$MODE" "$OUT" | tee "$OUT/summary.json"

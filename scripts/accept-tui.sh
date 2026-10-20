#!/bin/zsh
set -euo pipefail

ROOT=$(cd "$(dirname "$0")/.." && pwd)
STAMP=$(date +%Y%m%d-%H%M%S)
OUT="$ROOT/target/acceptance/tui/$STAMP"
mkdir -p "$OUT"

cd "$ROOT"
cargo build -p localsendd -p localsend-tui | tee "$OUT/build.log"
cargo test -p localsend-service --test multicast_discovery -- --nocapture \
    | tee "$OUT/multicast-discovery.log"

for run in 1 2 3; do
    LOCALSEND_ACCEPTANCE=1 cargo test -p localsend-tui -- --test-threads=1 --nocapture \
        >"$OUT/run-$run.log" 2>&1
    cat "$OUT/run-$run.log"
done

printf '{"suite":"tui","result":"passed","runs":3,"artifacts":"%s"}\n' "$OUT" | tee "$OUT/summary.json"

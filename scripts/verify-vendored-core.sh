#!/bin/sh
set -eu
export LC_ALL=C

ROOT=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
CORE="$ROOT/crates/localsend-official-core"

cd "$CORE"
shasum -a 256 -c SHA256SUMS

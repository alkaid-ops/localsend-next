#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
APP_DIR=$(dirname "$SCRIPT_DIR")
OUTPUT_DIR="$APP_DIR/reference/slint"

mkdir -p "$OUTPUT_DIR"
cd "$APP_DIR"

for page in receive send settings; do
    for dimension in 800x600 1024x768 1440x900 700x700; do
        width=${dimension%x*}
        height=${dimension#*x}
        for theme in light dark; do
            cargo run --quiet --example render_reference -- \
                --headless \
                --page "$page" \
                --state default \
                --width "$width" \
                --height "$height" \
                --theme "$theme" \
                --output "$OUTPUT_DIR/$page-default-$dimension-$theme.png"
        done
    done
done

for state_page in receive:request receive:transferring send:selected send:multiple; do
    page=${state_page%%:*}
    state=${state_page#*:}
    for theme in light dark; do
        cargo run --quiet --example render_reference -- \
            --headless \
            --page "$page" \
            --state "$state" \
            --width 1024 \
            --height 768 \
            --theme "$theme" \
            --output "$OUTPUT_DIR/$page-$state-1024x768-$theme.png"
    done
done

# LocalSend desktop OpenDesign reference

This directory preserves the OpenDesign prototype used to refactor the Slint
desktop UI. The production implementation lives in `ui/`; this prototype is a
visual reference and is not shipped in the application bundle.

Source project: `localsend-next-three-pages-redesign`

Coverage:

- Receive, Send, and Settings at desktop and compact widths
- Light and dark color roles
- Receive request, transfer, and completion states
- Send selection, device, multi-select, transfer, and completion states
- Device details and the shared dialog shell family

The Slint implementation keeps the existing RPC, daemon lifecycle, settings,
clipboard, and transfer behavior. Visual changes must not require protocol or
persistence changes.

## Visual verification policy

Do not repeatedly inspect or paste full screenshots during implementation.
Use the prototype source as the design authority:

1. Treat `design-spec.md` and CSS custom properties as the token source.
2. Read landmark geometry and computed styles from the HTML source.
3. Run the Slint testing renderer at the same viewport and state.
4. Compare dimensions, colors, and a small set of named landmark boxes with an
   automated script.
5. Inspect only failed crops when an automated threshold is exceeded.

Full-image visual review is reserved for intentional redesigns and final
release validation. Pixel-perfect equality is not expected across HTML and
Slint renderers.

## Automated comparison commands

Render one Slint state with accessibility landmarks:

```sh
cargo run -p localsend-gui-slint --example render_reference -- \
  --headless --metrics --page send --state default \
  --width 1024 --height 768 --theme light --output /tmp/slint-send.png
```

Compare it against a browser screenshot without loading the image into the
model context:

```sh
cargo run -p localsend-gui-slint --example compare_reference -- \
  --expected /tmp/open-design-send.png --actual /tmp/slint-send.png \
  --tolerance 4 --tile-size 64 --top-tiles 20 \
  --diff /tmp/send-diff.png
```

Extract browser geometry and computed styles without sending a screenshot to
the model:

```sh
node reference/open-design/extract_metrics.mjs \
  --page send --state default --width 1024 --height 768 --theme light \
  --screenshot /tmp/open-design-send.png \
  > /tmp/open-design-send.metrics.json
```

`extract_metrics.mjs` launches the installed headless Chromium through the
DevTools protocol, disables motion, applies the requested OpenDesign state,
clips the screenshot to the 1024×768 product window, and emits DOM frames plus
font, color, spacing, radius and child-control metrics. Use this JSON as the
primary comparison surface. The comparator emits the global changed
percentage, mean/max channel delta and the largest changed 64px tiles as JSON.
Inspect only a failed crop or the diff image when the numeric report identifies
a specific region.

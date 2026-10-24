# localsend-next

PC-focused LocalSend-compatible client built around the official LocalSend Rust
core. A separate root-only Android receive MVP shares the same service layer.

```text
Slint / TUI / tray
          |
   localsend-rpc
          |
     localsendd
          |
 official LocalSend core
```

The formal GUI is Slint with the FemtoVG renderer. TUI and tray are first-class
PC frontends. Flutter and Iced are intentionally out of scope.

## Binaries

- `localsendd`: network service and canonical application state
- `localsend-tray`: daemon and GUI lifecycle owner
- `localsend-tui`: terminal frontend
- `localsend-gui-slint`: formal GUI frontend
- `localsend-androidd`: Android root-only headless receiver MVP

See `docs/desktop-pc.md` for operation, `docs/verification.zh-CN.md` for the
current test evidence, `docs/acceptance-automation.zh-CN.md` for the local
acceptance gates, `docs/android-root-mvp.zh-CN.md` for the Android root module,
and `VENDORED_CORE.md` for upstream provenance.

```sh
./scripts/accept-tui.sh
./scripts/accept-slint.sh
```

## macOS test build

`apps/localsend-gui-slint/package-macos.sh` builds an arm64 ad-hoc signed test app at
`target/release/bundle/macos/LocalSend.app`. It bundles the Slint GUI, tray, and
daemon. This is not a Developer ID signed or notarized distribution artifact.

The `localsend-gui-egui`, `localsend-gui-fltk`, `localsend-gui-iced`, and
`localsend-gui-tauri` directories are experimental ports. They are intentionally
excluded from the root Cargo workspace until their RPC integration is complete.

## Android root receiver MVP

`apps/localsend-androidd` is a headless receive-only daemon for rooted Android
devices. It has no UI, no Foreground Service, and no persistent notification.
The Magisk/KernelSU module supervisor starts it after boot and restarts it when
it exits.

```sh
ANDROID_NDK_ROOT=/path/to/android-ndk ./scripts/package-androidd-root.sh
./scripts/check-androidd-root-package.sh \
  target/release/packages/localsend-next-root-aarch64.zip
./scripts/verify-androidd-root.sh
```

The verification script returns `BLOCKED` when no rooted device is connected.

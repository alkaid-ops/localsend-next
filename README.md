# localsend-next

PC-focused LocalSend-compatible client built around the official LocalSend Rust
core and a separately supervised daemon.

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
- `localsend-slint`: GUI frontend

See `docs/desktop-pc.md` for operation, `docs/verification.zh-CN.md` for the
current test evidence, and `VENDORED_CORE.md` for upstream provenance.

## macOS test build

`apps/localsend-slint/package-macos.sh` builds an arm64 ad-hoc signed test app at
`target/release/bundle/macos/LocalSend.app`. It bundles the Slint GUI, tray, and
daemon. This is not a Developer ID signed or notarized distribution artifact.

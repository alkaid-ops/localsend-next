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

See `docs/desktop-pc.md` for operation and `VENDORED_CORE.md` for the upstream
core provenance.

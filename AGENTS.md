# localsend-next repository rules

- The product is a PC-only LocalSend-compatible client.
- The only formal GUI is `apps/localsend-gui-slint` using Slint winit and FemtoVG.
- The egui, FLTK, Iced, and Tauri directories are experimental and excluded from
  the root workspace until migrated to `localsend-rpc`.
- Runtime architecture is frontends -> `localsend-rpc` -> `localsendd` ->
  vendored official LocalSend core.
- Frontends must not depend on the vendored core and must not spawn `localsendd`.
- `localsend-tray` owns daemon and GUI process lifecycle.
- The daemon owns settings, identity, favorites, history, transfers, and TLS
  identity persistence.
- Keep `crates/localsend-official-core` byte-for-byte identical to the recorded
  upstream export except for its generated `SHA256SUMS` file.
- Do not copy generated output, local databases, app bundles, `target/`, `tmp/`,
  or graphify output into the repository.

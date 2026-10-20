# Migration provenance

The reusable desktop code imported into this repository came from the local
`localsend-rs` working tree on 2026-10-20.

- Upstream repository: <https://github.com/CrossCopy/localsend-rs>
- Base commit: `23d6055`
- Source state: tracked commit plus the user's uncommitted working-tree changes

The import was selective. It includes the RPC client/framing/transport, platform
single-instance code, tray, three-page TUI, Slint UI and view model, lifecycle
tests, packaging assets, and documentation. It intentionally excludes the
CrossCopy protocol implementation, old orchestration service, Flutter app,
Iced experiments, generated output, local locks, and build directories.

The old implementation remains available in the source working tree as a
read-only migration reference. This repository is now the intended location for
new integration work.

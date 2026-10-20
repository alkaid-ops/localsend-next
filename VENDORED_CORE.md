# Vendored LocalSend Core

`crates/localsend-official-core` is an unmodified export of the official
LocalSend Rust core.

- Repository: <https://github.com/localsend/localsend>
- Tag: `v1.18.2`
- Commit: `af0416be50770a97760f7070684bc667b759a15c`
- Exported path: `packages/core`
- Export command: `git archive v1.18.2 packages/core`
- Export date: 2026-10-20
- Upstream license: Apache-2.0

The package name remains `localsend`. The new workspace consumes it through the
`localsend-core` dependency alias.

Do not edit vendored files directly. Upgrade by exporting a new tagged commit,
regenerating `SHA256SUMS`, updating this file, and running the upstream tests and
the localsend-next integration suite.

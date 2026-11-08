<div align="center">
    <img src="docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<!-- locale-switcher -->
<div align="center">

**English** | [العربية](docs/ar/README.md) | [Azərbaycanca](docs/az/README.md) | [Беларуская](docs/be/README.md) | [Български](docs/bg/README.md) | [বাংলা](docs/bn/README.md) | [Català](docs/ca/README.md) | [Čeština](docs/cs/README.md) | [Dansk](docs/da/README.md) | [Deutsch](docs/de/README.md) | [Ελληνικά](docs/el/README.md) | [English (India)](docs/en-IN/README.md) | [Español](docs/es-ES/README.md) | [Eesti](docs/et/README.md) | [Euskara](docs/eu/README.md) | [فارسی](docs/fa/README.md) | [Suomi](docs/fi/README.md) | [Filipino](docs/fil-PH/README.md) | [Français](docs/fr/README.md) | [Gaeilge](docs/ga/README.md) | [Galego](docs/gl/README.md) | [ગુજરાતી](docs/gu/README.md) | [עברית](docs/he/README.md) | [हिन्दी](docs/hi/README.md) | [Magyar](docs/hu/README.md) | [Հայերեն](docs/hy/README.md) | [Bahasa Indonesia](docs/id/README.md) | [Italiano](docs/it/README.md) | [日本語](docs/ja/README.md) | [ខ្មែរ](docs/km/README.md) | [한국어](docs/ko/README.md) | [Кыргызча](docs/ky/README.md) | [ລາວ](docs/lo/README.md) | [മലയാളം](docs/ml/README.md) | [Монгол](docs/mn/README.md) | [Bahasa Melayu](docs/ms/README.md) | [नेपाली](docs/ne/README.md) | [Nederlands](docs/nl/README.md) | [Polski](docs/pl/README.md) | [Português (Brasil)](docs/pt-BR/README.md) | [Português (Portugal)](docs/pt-PT/README.md) | [Română](docs/ro/README.md) | [Русский](docs/ru/README.md) | [සිංහල](docs/si/README.md) | [Slovenčina](docs/sk/README.md) | [Slovenščina](docs/sl/README.md) | [Српски](docs/sr/README.md) | [Српски (ћирилица)](docs/sr-Cyrl/README.md) | [Svenska](docs/sv/README.md) | [தமிழ்](docs/ta/README.md) | [ไทย](docs/th/README.md) | [Türkçe](docs/tr/README.md) | [Українська](docs/uk/README.md) | [اردو](docs/ur/README.md) | [Oʻzbekcha](docs/uz/README.md) | [Tiếng Việt](docs/vi/README.md) | [简体中文](docs/zh-CN/README.md) | [繁體中文（香港）](docs/zh-HK/README.md) | [繁體中文（台灣）](docs/zh-TW/README.md)

</div>

localsend-next is a LocalSend-compatible desktop client written in Rust. It speaks the official protocol, so it discovers and transfers files with the official LocalSend app in both directions, while running as a native desktop program instead of a Flutter app.

The network side is the official LocalSend Rust core, vendored unchanged. Everything above it — daemon, RPC layer, tray, terminal UI and the Slint GUI — is built around that core.

## Demo

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="docs/images/readme-receive-light.png">
</picture>

| Send | Settings |
| --- | --- |
| ![Send page with the file, folder, text and clipboard sources and nearby devices](docs/images/readme-send-light.png) | ![Settings page with theme, language, network and transfer options](docs/images/readme-settings-light.png) |

## Features

- Discovery and transfer against official LocalSend clients, on the vendored upstream core rather than a re-implementation
- One canonical state owner: the daemon holds settings, identity, favourites, history, transfers and the TLS identity, so every frontend sees the same thing
- GUI, terminal UI and tray as independent frontends over the same local RPC; the tray owns daemon and GUI process lifecycle
- Realtime per-file progress, history with open and reveal actions, favourites, manual device entry, and Web Share receive links
- 57 interface languages matching upstream LocalSend's list, switchable at runtime or following the system locale
- Native desktop integration: notifications from the tray only, launch at login, minimise to tray, window placement persistence, and a macOS window that draws into its title bar
- Packaging for macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) and Windows (ZIP), for both x86_64 and aarch64

## Architecture

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

The daemon is the only process that talks to the network or writes application state. Frontends never depend on the vendored core and never spawn the daemon themselves; the tray does. File bytes do not travel over RPC — the daemon resolves paths, hashes content and records history.

| Binary | Role |
| --- | --- |
| `localsendd` | Network service and canonical application state |
| `localsend-tray` | Daemon and GUI lifecycle owner, notifications |
| `localsend-gui-slint` | The formal GUI, Slint with the FemtoVG renderer |
| `localsend-tui` | Terminal frontend |
| `localsend-launcher` | Explicit launcher for environments without a tray |
| `localsend-androidd` | Standalone root-only Android receive daemon (MVP) |

The `localsend-gui-egui`, `-fltk`, `-iced` and `-tauri` directories are experimental ports and stay outside the root workspace until their RPC integration is complete.

## Download

Release assets follow one naming scheme — `LocalSend-<version>-<platform>-<arch>.<ext>`, with platforms `linux` / `windows` / `macos` and architectures `x86_64` / `aarch64`:

| Platform | Asset |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Any Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` covers every asset in a release. The macOS build produced by CI is ad-hoc signed, not Developer ID signed or notarised.

## Quick start

### macOS

```bash
open LocalSend-<version>-macos-<arch>.dmg
# drag LocalSend.app into Applications
```

### Debian / Ubuntu

```bash
sudo dpkg -i LocalSend-<version>-linux-<arch>.deb
```

### Fedora / RHEL

```bash
sudo rpm -i LocalSend-<version>-linux-<arch>.rpm
```

### AppImage

```bash
chmod +x LocalSend-<version>-linux-<arch>.AppImage
./LocalSend-<version>-linux-<arch>.AppImage
```

Linux packages install a desktop entry that launches `localsend-tray`, which owns the daemon lifecycle. On desktops without a status bar the tray keeps the GUI open instead; `LocalSend --launcher` selects that mode explicitly from the AppImage.

### Windows

Unzip and run `LocalSend.exe`. The archive holds the GUI, the daemon, the tray and the launcher side by side; keep them together.

## Behaviour worth knowing

- **Quit is one operation.** Quitting from the tray terminates the GUI recorded in the endpoint-scoped `gui.pid`, stops the owned daemon within a bounded timeout, and exits without leaving a process or a launchd keep-alive job behind.
- **Notifications come from the tray.** The GUI never posts its own notifications, so each platform has a single delivery path.
- **Window state is per endpoint.** Placement is stored beside the daemon configuration for that endpoint rather than in a shared global location, so two instances do not fight over window geometry.
- **The daemon is authoritative.** Settings changed in the GUI are submitted to the daemon and echoed back through a snapshot; a frontend keeps no private copy that could drift.

## Languages

The interface ships in 57 locales, matching upstream LocalSend's list, with Simplified Chinese as the msgid language. Pick one under **Settings → Language**, or leave it on "Follow system" to use the OS locale.

| | |
| --- | --- |
| Bundled catalogs | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UI source strings | `@tr("...")` in `apps/localsend-gui-slint/ui` |
| English catalog | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust messages) |
| Rust-side messages | `crates/localsend-i18n` (generated `lookup_generated.rs`) |
| Upstream mirror | `translations/upstream/<locale>.json` |

Regenerate the catalogs after changing any UI string:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` is committed, so the generated catalogs are reproducible offline. Correct an entry there and rerun the generator to fix a translation.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Building from source

Requirements: Rust `1.97.1` (pinned by `rust-toolchain.toml`) and, for Linux builds, the GTK3 and AppIndicator development headers.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Platform packages:

```bash
# macOS (on a matching host)
cd apps/localsend-gui-slint && PACKAGE_VERSION=0.1.3 ./package-macos.sh

# Linux
./scripts/package-linux.sh x86_64      # deb + tarball
./scripts/package-rpm.sh x86_64        # rpm
./scripts/package-appimage.sh x86_64   # AppImage

# Windows (PowerShell)
./scripts/package-windows.ps1 -Architecture x64
```

Acceptance scripts drive the real binaries instead of mocked state:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Documentation

| Document | Contents |
| --- | --- |
| [`docs/desktop-pc.md`](docs/desktop-pc.md) | Operation, packaging and platform behaviour |
| [`docs/verification.zh-CN.md`](docs/verification.zh-CN.md) | Current test evidence and known unverified areas |
| [`docs/acceptance-automation.zh-CN.md`](docs/acceptance-automation.zh-CN.md) | Local acceptance gates |
| [`docs/agent-regression-playbook.zh-CN.md`](docs/agent-regression-playbook.zh-CN.md) | Recurring regressions and the evidence trail expected before changing behaviour |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](docs/HANDOFF_CLAUDE.zh-CN.md) | Visual convergence state against the OpenDesign prototype |
| [`docs/android-root-mvp.zh-CN.md`](docs/android-root-mvp.zh-CN.md) | The Android root module and its limits |
| [`VENDORED_CORE.md`](VENDORED_CORE.md) | Upstream provenance of the vendored core |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](docs/i18n-overrides.json).

Some documents are written in Chinese; they record verification runs and conventions in the words they were agreed in.

## Scope

The primary product is the PC client. Android support is a root-only, headless receive daemon until explicitly promoted beyond that scope — it has no GUI, starts no foreground service and posts no persistent notification. `crates/localsend-official-core` is kept byte-for-byte identical to its recorded upstream export, apart from its generated `SHA256SUMS` file.

## Credits

The protocol, the wire format and the Rust core come from [localsend/localsend](https://github.com/localsend/localsend); this project builds a desktop client on that work rather than re-implementing the protocol. Thanks to the LocalSend authors and contributors.

The interface follows a design prototype developed for this project; its visual reference lives in [`apps/localsend-gui-slint/reference/open-design/`](apps/localsend-gui-slint/reference/open-design/).

## Contributing

- **Bugs and suggestions**: open an [issue](https://github.com/alkaid-ops/localsend-next/issues) with the version, platform, reproduction steps and what you saw. A log or screenshot makes it much faster to act on.
- **Code**: pull requests are welcome. Changes that touch the daemon/frontend boundary, the vendored core, or the visual contract should be discussed first — the constraints are recorded in [`AGENTS.md`](AGENTS.md).
- **Translations**: the catalogs are generated from the committed machine-translation cache, so corrections there are the easiest kind of contribution to review.

## License

[Apache-2.0](LICENSE). Third-party components and their licenses are listed in [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

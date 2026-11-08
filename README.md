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

## Languages

Matches upstream LocalSend's language list: 57 locales, including English and the
Chinese variants, with Simplified Chinese as the catalog's msgid language. The settings page lists them under **Language**; "Follow
system" uses the OS locale, and every other choice is applied immediately.

| | |
| --- | --- |
| Bundled catalogs | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UI source strings | `@tr("...")` in `apps/localsend-gui-slint/ui` |
| English catalog | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust messages) |
| Rust-side messages | `crates/localsend-i18n` (generated `lookup_generated.rs`) |
| Upstream mirror | `translations/upstream/<locale>.json` |

Regenerate after changing any UI string:

```sh
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs   # fills the machine-translation cache
node scripts/update-slint-translations.mjs # writes the .po files and Rust table
```

`locale-machine-cache.json` is committed, so the generated catalogs are
reproducible without network access; edit an entry there to correct a
translation and rerun the generator.

<details>
<summary>Bundled locales</summary>

| Locale | Language |
| --- | --- |
| `ar` | العربية |
| `az` | Azərbaycanca |
| `be` | Беларуская |
| `bg` | Български |
| `bn` | বাংলা |
| `ca` | Català |
| `cs` | Čeština |
| `da` | Dansk |
| `de` | Deutsch |
| `el` | Ελληνικά |
| `en-IN` | English (India) |
| `es-ES` | Español |
| `et` | Eesti |
| `eu` | Euskara |
| `fa` | فارسی |
| `fi` | Suomi |
| `fil-PH` | Filipino |
| `fr` | Français |
| `ga` | Gaeilge |
| `gl` | Galego |
| `gu` | ગુજરાતી |
| `he` | עברית |
| `hi` | हिन्दी |
| `hu` | Magyar |
| `hy` | Հայերեն |
| `id` | Bahasa Indonesia |
| `it` | Italiano |
| `ja` | 日本語 |
| `km` | ខ្មែរ |
| `ko` | 한국어 |
| `ky` | Кыргызча |
| `lo` | ລາວ |
| `ml` | മലയാളം |
| `mn` | Монгол |
| `ms` | Bahasa Melayu |
| `ne` | नेपाली |
| `nl` | Nederlands |
| `pl` | Polski |
| `pt-BR` | Português (Brasil) |
| `pt-PT` | Português (Portugal) |
| `ro` | Română |
| `ru` | Русский |
| `si` | සිංහල |
| `sk` | Slovenčina |
| `sl` | Slovenščina |
| `sr` | Српски |
| `sr-Cyrl` | Српски (ћирилица) |
| `sv` | Svenska |
| `ta` | தமிழ் |
| `th` | ไทย |
| `tr` | Türkçe |
| `uk` | Українська |
| `ur` | اردو |
| `uz` | Oʻzbekcha |
| `vi` | Tiếng Việt |
| `zh-HK` | 繁體中文（香港） |
| `zh-TW` | 繁體中文（台灣） |

</details>

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

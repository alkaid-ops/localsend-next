<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | **Filipino** | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [简体中文](../../docs/zh-CN/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



Ang localsend-next ay isang LocalSend-compatible na desktop client na nakasulat sa Rust. Sinasalita nito ang opisyal na protocol, kaya natutuklasan at naglilipat ito ng mga file gamit ang opisyal na LocalSend app sa parehong direksyon, habang tumatakbo bilang isang native na desktop program sa halip na isang Flutter app.

Ang bahagi ng network ay ang opisyal na LocalSend Rust core, na hindi binago. Lahat ng nasa itaas nito — daemon, RPC layer, tray, terminal UI at ang Slint GUI — ay binuo sa paligid ng core na iyon.

## Demo

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Ipadala | Mga setting |
| --- | --- |
| ![Ipadala ang page na may file, folder, text at clipboard na pinagmumulan at mga kalapit na device](../../docs/images/readme-send-light.png) | ![Pahina ng mga setting na may mga opsyon sa tema, wika, network at paglipat](../../docs/images/readme-settings-light.png) |

## Mga tampok

- Pagtuklas at paglilipat laban sa mga opisyal na LocalSend na kliyente, sa naibentang upstream core sa halip na isang muling pagpapatupad
- Isang canonical na may-ari ng estado: ang daemon ay nagtataglay ng mga setting, pagkakakilanlan, mga paborito, kasaysayan, mga paglilipat at ang TLS pagkakakilanlan, kaya ang bawat frontend ay nakikita ang parehong bagay
- GUI, terminal UI at tray bilang mga independiyenteng frontend sa parehong lokal na RPC; ang tray ay nagmamay-ari ng daemon at GUI process lifecycle
- Realtime per-file progress, history na may bukas at naghahayag na mga aksyon, paborito, manu-manong pagpasok ng device, at Web Share ay tumatanggap ng mga link
- 57 mga wika ng interface na tumutugma sa upstream na listahan ng LocalSend, naililipat sa runtime o sumusunod sa locale ng system
- Native desktop integration: mga notification mula sa tray lang, ilunsad sa pag-login, i-minimize sa tray, pagtitiyaga sa paglalagay ng window, at isang macOS na window na kumukuha sa title bar nito
- Packaging para sa macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) at Windows (ZIP), para sa parehong x86_64 at aarch64

## Arkitektura

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Ang daemon ay ang tanging proseso na nakikipag-usap sa network o nagsusulat ng estado ng aplikasyon. Ang mga frontend ay hindi kailanman umaasa sa ibinebentang core at hindi kailanman nagbubunga ng daemon mismo; ginagawa ng tray. Ang mga byte ng file ay hindi dumadaan sa RPC — nire-resolve ng daemon ang mga landas, nagha-hash ng content at nagtatala ng kasaysayan.

| Binary | Tungkulin |
| --- | --- |
| `localsendd` | Serbisyo sa network at estado ng canonical application |
| `localsend-tray` | Daemon at GUI may-ari ng lifecycle, mga notification |
| `localsend-gui-slint` | Ang pormal na GUI, Slint na may FemtoVG renderer |
| `localsend-tui` | Terminal frontend |
| `localsend-launcher` | Tahasang launcher para sa mga environment na walang tray |
| `localsend-androidd` | Standalone root-only Android makatanggap ng daemon (MVP) |

Ang `localsend-gui-egui`, `-fltk`, `-iced` at `-tauri` na mga direktoryo ay mga pang-eksperimentong port at manatili sa labas ng root workspace hanggang sa makumpleto ang kanilang RPC integration.

## I-download

Ang mga asset ng paglabas ay sumusunod sa isang scheme ng pagpapangalan — `LocalSend-<version>-<platform>-<arch>.<ext>`, na may mga platform `linux` / `windows` / `macos` at mga arkitektura `x86_64` / `aarch64`:

| Plataporma | Asset |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| anumang Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

Sinasaklaw ng `SHA256SUMS.txt` ang bawat asset sa isang release. Ang macOS build na ginawa ng CI ay ad-hoc signed, hindi Developer ID na nilagdaan o notarized.

## Mabilis na pagsisimula

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

Ang Linux package ay nag-i-install ng desktop entry na naglulunsad ng `localsend-tray`, na nagmamay-ari ng daemon lifecycle. Sa mga desktop na walang status bar, pinananatiling bukas ng tray ang GUI; Tahasang pinipili ng `LocalSend --launcher` ang mode na iyon mula sa AppImage.

### Windows

I-unzip at patakbuhin ang `LocalSend.exe`. Hawak ng archive ang GUI, ang daemon, ang tray at ang launcher na magkatabi; panatilihin silang magkasama.

## Pag-uugali na dapat malaman

- **Ang pag-quit ay isang operasyon.** Ang pag-quit mula sa tray ay magwawakas sa GUI na naitala sa endpoint-scoped `gui.pid`, hihinto ang pagmamay-ari na daemon sa loob ng limitadong timeout, at lalabas nang hindi umaalis sa isang proseso o isang launchd keep-alive na trabaho.
- **Nanggagaling ang mga notification sa tray.** Ang GUI ay hindi kailanman nagpo-post ng sarili nitong mga notification, kaya ang bawat platform ay may iisang daanan ng paghahatid.
- **Ang window state ay bawat endpoint.** Ang placement ay nakaimbak sa tabi ng configuration ng daemon para sa endpoint na iyon sa halip na sa isang nakabahaging pandaigdigang lokasyon, kaya dalawang instance ay hindi nag-aaway sa window geometry.
- **Ang daemon ay makapangyarihan.** Ang mga setting na binago sa GUI ay isinumite sa daemon at ibinabalik sa pamamagitan ng isang snapshot; ang isang frontend ay hindi nagpapanatili ng pribadong kopya na maaaring maanod.

## Mga wika

Nagpapadala ang interface sa 57 lokal, tumutugma sa upstream na listahan ng LocalSend, na may Simplified Chinese bilang msgid na wika. Pumili ng isa sa ilalim ng **Mga Setting → Wika**, o iwanan ito sa "Sundan ang system" upang magamit ang lokal na OS.

| | |
| --- | --- |
| Mga naka-bundle na katalogo | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| Mga string ng pinagmulan ng UI | `@tr("...")` sa `apps/localsend-gui-slint/ui` |
| English catalog | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust mga mensahe) |
| Mga mensaheng may kalawang | `crates/localsend-i18n` (binuo `lookup_generated.rs`) |
| Upstream na salamin | `translations/upstream/<locale>.json` |

Buuin muli ang mga katalogo pagkatapos baguhin ang anumang string ng UI:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

Ang `locale-machine-cache.json` ay nakatuon, kaya ang mga nabuong katalogo ay maaaring kopyahin offline. Iwasto ang isang entry doon at muling patakbuhin ang generator upang ayusin ang isang pagsasalin.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Gusali mula sa pinagmulan

Mga Kinakailangan: Rust `1.97.1` (naka-pin ni `rust-toolchain.toml`) at, para sa Linux build, ang GTK3 at AppIndicator na mga development header.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Mga pakete ng platform:

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

Ang mga script ng pagtanggap ay nagtutulak ng mga tunay na binary sa halip na mapanukso na estado:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Dokumentasyon

| Dokumento | Mga nilalaman |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Operasyon, packaging at pag-uugali ng platform |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Kasalukuyang ebidensya ng pagsubok at mga kilalang lugar na hindi na-verify |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Mga lokal na gate ng pagtanggap |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Ang mga umuulit na regression at ang ebidensiya ay inaasahan bago baguhin ang gawi |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Visual convergence state laban sa OpenDesign prototype |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Ang Android root module at ang mga limitasyon nito |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Upstream provenance ng ibinebentang core |

Ang README na ito ay na-publish sa lahat ng 57 interface na wika — pumili ng isa mula sa switcher sa itaas. Ang mga pagsasalin ay nabuo mula sa `README.md` ni `scripts/translate-readme.mjs`; papasok ang mga pagwawasto [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Ang ilang mga dokumento ay nakasulat sa Chinese; itinatala nila ang mga pagpapatakbo ng pagpapatunay at mga kombensiyon sa mga salitang napagkasunduan nila.

## Saklaw

Ang pangunahing produkto ay ang PC client. Ang Android na suporta ay isang root-only, walang ulo na tumanggap ng daemon hanggang sa tahasang i-promote na lampas sa saklaw na iyon — wala itong GUI, walang sinisimulang serbisyo sa foreground at walang patuloy na pag-post ng notification. Ang `crates/localsend-official-core` ay pinananatiling byte-for-byte na kapareho ng naitala nitong upstream export, bukod sa nabuo nitong `SHA256SUMS` file.

## Mga kredito

Ang protocol, ang wire format at ang Rust core ay nagmula sa [localsend/localsend](https://github.com/localsend/localsend); ang proyektong ito ay bumubuo ng isang desktop client sa gawaing iyon sa halip na muling ipatupad ang protocol. Salamat sa LocalSend mga may-akda at nag-ambag.

Ang interface ay sumusunod sa isang disenyong prototype na binuo para sa proyektong ito; nasa [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/) ang visual reference nito.

## Nag-aambag

- **Mga bug at suhestyon**: magbukas ng [isyu](https://github.com/alkaid-ops/localsend-next/issues) na may bersyon, platform, mga hakbang sa pagpaparami at kung ano ang iyong nakita. Ang isang log o screenshot ay ginagawang mas mabilis na kumilos.
- **Code**: tinatanggap ang mga pull request. Dapat munang talakayin ang mga pagbabagong nakakaapekto sa hangganan ng daemon/frontend, sa ibinebentang core, o sa visual na kontrata — ang mga hadlang ay itinatala sa [`AGENTS.md`](../../AGENTS.md).
- **Mga Pagsasalin**: ang mga katalogo ay nabuo mula sa nakatalagang machine-translation cache, kaya ang mga pagwawasto ay ang pinakamadaling uri ng kontribusyon na susuriin.

## Lisensya

[Apache-2.0](../../LICENSE). Ang mga bahagi ng third-party at ang kanilang mga lisensya ay nakalista sa [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | **Slovenščina** | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next je namizni odjemalec, združljiv z LocalSend, napisan v Rust. Govori uradni protokol, zato odkriva in prenaša datoteke z uradno aplikacijo LocalSend v obe smeri, medtem ko deluje kot izvorni namizni program namesto aplikacije Flutter.

Omrežna stran je uradno jedro LocalSend Rust, prodajalec je nespremenjen. Vse nad njim – demon, RPC sloj, pladenj, terminalski uporabniški vmesnik in Slint GUI — je zgrajeno okoli tega jedra.

## Demo

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Pošlji | nastavitve |
| --- | --- |
| ![Pošlji stran z viri datoteke, mape, besedila in odložišča ter napravami v bližini](../../docs/images/readme-send-light.png) | ![Stran z nastavitvami s temo, jezikom, omrežjem in možnostmi prenosa](../../docs/images/readme-settings-light.png) |

## Funkcije

- Odkrivanje in prenos proti uradnim odjemalcem LocalSend, na jedru navzgor, namesto ponovne implementacije
- En lastnik kanoničnega stanja: demon hrani nastavitve, identiteto, priljubljene, zgodovino, prenose in identiteto TLS, tako da vsak vmesnik vidi isto
- GUI, terminalski uporabniški vmesnik in pladenj kot neodvisni vmesniki prek istega lokalnega RPC; pladenj ima v lasti demon in GUI življenjski cikel procesa
- Napredek v realnem času za posamezno datoteko, zgodovina z dejanji odpiranja in razkrivanja, priljubljene, ročni vnos naprave in povezave za prejemanje v spletni skupni rabi
- 57 jezikov vmesnika, ki se ujemajo s seznamom upstream LocalSend, ki jih je mogoče preklapljati med izvajanjem ali v skladu s sistemskim jezikom
- Izvorna integracija namizja: obvestila samo iz pladnja, zagon ob prijavi, pomanjšanje na pladenj, obstojnost postavitve okna in okno macOS, ki se prikaže v naslovni vrstici
- Pakiranje za macOS (DMG/@ZIP), Linux (deb, rpm, AppImage, tarball) in Windows (ZIP), za x86_64 in aarch64

## Arhitektura

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Demon je edini proces, ki komunicira z omrežjem ali piše stanje aplikacije. Frontendi niso nikoli odvisni od prodajnega jedra in nikoli sami ne ustvarijo demona; pladenj naredi. Datotečni bajti ne potujejo čez RPC — demon razreši poti, razprši vsebino in zapiše zgodovino.

| Binarno | Vloga |
| --- | --- |
| `localsendd` | Omrežna storitev in stanje kanonične aplikacije |
| `localsend-tray` | Daemon in lastnik življenjskega cikla GUI, obvestila |
| `localsend-gui-slint` | Formalni GUI, Slint z upodabljalnikom FemtoVG |
| `localsend-tui` | Čelni del terminala |
| `localsend-launcher` | Eksplicitni zaganjalnik za okolja brez pladnja |
| `localsend-androidd` | Samostojni root-samo Android sprejemni demon (MVP) |

Imeniki `localsend-gui-egui`, `-fltk`, `-iced` in `-tauri` so poskusna vrata in ostanejo zunaj korenskega delovnega prostora, dokler njihova integracija RPC ni končana.

## Prenos

Sredstva izdaje sledijo eni shemi poimenovanja — `LocalSend-<version>-<platform>-<arch>.<ext>`, s platformami `linux` / `windows` / `macos` in arhitekturami `x86_64` / `aarch64`:

| Platforma | Sredstvo |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Kateri koli Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` zajema vsako sredstvo v izdaji. Zgradba macOS, ki jo izdela CI, je ad hoc podpisana, ni podpisana ali notarsko overjena z ID-jem razvijalca.

## Hitri začetek

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

Paketi Linux namestijo namizni vnos, ki zažene `localsend-tray`, ki ima v lasti življenjski cikel demona. Na namizjih brez statusne vrstice pladenj namesto tega drži GUI odprt; `LocalSend --launcher` ta način izbere izrecno iz AppImage.

### Windows

Razpakirajte in zaženite `LocalSend.exe`. Arhiv vsebuje GUI, demona, pladenj in zaganjalnik drug ob drugem; drži jih skupaj.

## Vedenje, ki ga je vredno poznati

- **Izhod je ena operacija.** Izhod iz pladnja prekine GUI, zabeležen v `gui.pid` v obsegu končne točke, ustavi lastni demon v omejeni časovni omejitvi in ​​zapre, ne da bi pustil proces ali zagnano opravilo ohranjanja pri življenju.
- **Obvestila prihajajo iz pladnja.** GUI nikoli ne objavi lastnih obvestil, zato ima vsaka platforma eno samo pot dostave.
- **Stanje okna je na končno točko.** Umestitev je shranjena poleg konfiguracije demona za to končno točko in ne na skupni globalni lokaciji, tako da se dva primerka ne prepirata glede geometrije okna.
- **Daemon je verodostojen.** Nastavitve, spremenjene v GUI, so poslane demonu in odmevajo nazaj prek posnetka; čelni del ne hrani nobene zasebne kopije, ki bi lahko odnesla.

## Jeziki

Vmesnik je na voljo v 57 jezikih, ki se ujemajo s seznamom LocalSend, s poenostavljeno kitajščino kot jezikom sporočil. Izberite eno pod **Nastavitve → Jezik** ali jo pustite na "Sledi sistemu", če želite uporabiti jezik OS.

| | |
| --- | --- |
| Paketni katalogi | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| Izvorni nizi uporabniškega vmesnika | `@tr("...")` v `apps/localsend-gui-slint/ui` |
| angleški katalog | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust sporočila) |
| Rust-side sporočila | `crates/localsend-i18n` (generirano `lookup_generated.rs`) |
| Gorvodno ogledalo | `translations/upstream/<locale>.json` |

Ponovno ustvarite kataloge po spremembi katerega koli niza uporabniškega vmesnika:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` je predan, zato je ustvarjene kataloge mogoče reproducirati brez povezave. Tam popravite vnos in znova zaženite generator, da popravite prevod.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Gradnja iz vira

Zahteve: Rust `1.97.1` (pripel `rust-toolchain.toml`) in za gradnje Linux razvojni glavi GTK3 in AppIndicator.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Paketi platforme:

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

Sprejemni skripti poganjajo prave binarne datoteke namesto zasmehovanega stanja:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Dokumentacija

| Dokument | Vsebina |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Delovanje, embalaža in obnašanje platforme |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Trenutni testni dokazi in znana nepreverjena področja |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Lokalna sprejemna vrata |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Ponavljajoče se regresije in pričakovana sled dokazov pred spremembo vedenja |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Stanje vizualne konvergence glede na prototip OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Korenski modul Android in njegove omejitve |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Provencenca prodajnega jedra |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Nekateri dokumenti so napisani v kitajščini; beležijo poteke preverjanja in konvencije z besedami, s katerimi so bili dogovorjeni.

## Obseg

Primarni izdelek je PC odjemalec. Podpora za Android je samo korenski, brezglavi sprejemni demon, dokler ni izrecno povišan izven tega obsega — nima GUI, ne zažene nobene storitve v ospredju in ne objavi nobenega trajnega obvestila. `crates/localsend-official-core` je bajt za bajtom enak svojemu posnetemu izvozu navzgor, razen ustvarjene datoteke `SHA256SUMS`.

## Krediti

Protokol, format žice in Rust jedro prihajajo iz [localsend/localsend](https://github.com/localsend/localsend); ta projekt zgradi namizni odjemalec na tem delu namesto ponovne implementacije protokola. Hvala LocalSend avtorjem in sodelavcem.

Vmesnik sledi prototipu oblikovanja, razvitem za ta projekt; njegova vizualna referenca živi v [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Prispevam

- **Napake in predlogi**: odprite [težavo](https://github.com/alkaid-ops/localsend-next/issues) z različico, platformo, koraki reprodukcije in tem, kar ste videli. Dnevnik ali posnetek zaslona omogoča veliko hitrejše ukrepanje.
- **Koda**: zahteve po vleki so dobrodošle. Najprej je treba razpravljati o spremembah, ki se dotikajo meje daemon/frontend, prodajnega jedra ali vizualne pogodbe – omejitve so zapisane v [`AGENTS.md`](../../AGENTS.md).
- **Prevodi**: katalogi so ustvarjeni iz predanega predpomnilnika strojnega prevajanja, zato so popravki najlažja vrsta prispevka za pregled.

## Licenca

[Apache-2.0](../../LICENSE). Komponente tretjih oseb in njihove licence so navedene v [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

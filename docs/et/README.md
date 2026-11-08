<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | **Eesti** | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next on LocalSendiga ühilduv töölauaklient, mis on kirjutatud Rust. See räägib ametlikku protokolli, seega avastab ja edastab faile ametliku LocalSend rakendusega mõlemas suunas, töötades samal ajal oma töölauaprogrammina Flutteri rakenduse asemel.

Võrgupool on ametlik LocalSend Rust tuum, mida tarnitakse muutmata kujul. Kõik selle kohal olev – deemon, RPC kiht, salv, terminali kasutajaliides ja Slint GUI – on üles ehitatud selle tuuma ümber.

## Demo

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Saada | Seaded |
| --- | --- |
| ![Saada leht koos faili, kausta, teksti ja lõikelaua allikate ning lähedalasuvate seadmetega](../../docs/images/readme-send-light.png) | ![Seadete leht teema, keele, võrgu ja edastusvalikutega](../../docs/images/readme-settings-light.png) |

## Funktsioonid

- Avastamine ja ülekandmine ametlike LocalSend klientide vastu tarnitud ülesvoolu tuumal, mitte uuesti juurutamisel
- Üks kanooniline olekuomanik: deemon hoiab sätteid, identiteeti, lemmikuid, ajalugu, ülekandeid ja TLS identiteeti, nii et iga kasutajaliides näeb sama asja
- GUI, terminali kasutajaliides ja salv sama kohaliku RPC sõltumatute esiservadena; salv omab deemonit ja GUI protsessi elutsüklit
- Reaalajas failipõhine edenemine, ajalugu koos avamis- ja avaldamistoimingutega, lemmikud, käsitsi seadme sisestamine ja veebijagamise vastuvõtmise lingid
- 57 liidese keelt, mis vastavad ülesvoolu LocalSend loendile, vahetatavad käitusajal või süsteemi lokaadi järgi
- Algne töölaua integreerimine: märguanded ainult salvest, käivitamine sisselogimisel, minimeerimine salve, akna paigutuse püsivus ja macOS aken, mis tõmbab selle tiitliribale
- Pakend macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) ja Windows (ZIP), nii x86_64 kui ka aarch64 jaoks

## Arhitektuur

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Deemon on ainus protsess, mis räägib võrguga või kirjutab rakenduse olekut. Esiliidesed ei sõltu kunagi tarnitavast tuumast ega tekita kunagi deemonit ise; salv teeb. Failibaidid ei liigu üle RPC — deemon lahendab teed, räsib sisu ja salvestab ajaloo.

| Binaarne | Roll |
| --- | --- |
| `localsendd` | Võrguteenuse ja kanoonilise rakenduse olek |
| `localsend-tray` | Deemon ja GUI elutsükli omanik, teatised |
| `localsend-gui-slint` | Ametlik GUI, Slint koos FemtoVG renderdajaga |
| `localsend-tui` | Terminali esiosa |
| `localsend-launcher` | Selge käivitaja ilma salveta keskkondade jaoks |
| `localsend-androidd` | Eraldiseisev root-ainult Android deemon (MVP) |

Kataloogid `localsend-gui-egui`, `-fltk`, `-iced` ja `-tauri` on eksperimentaalsed pordid ja jäävad juurtööruumist väljapoole, kuni nende RPC integreerimine on lõppenud.

## Laadi alla

Väljalaskevarad järgivad ühte nimetamisskeemi — `LocalSend-<version>-<platform>-<arch>.<ext>`, platvormidega `linux` / `windows` / `macos` ja arhitektuuriga `x86_64` / `aarch64`:

| Platvorm | Vara |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Igasugune Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` hõlmab kõiki versiooni varasid. CI toodetud macOS järg on ad hoc allkirjastatud, mitte arendaja ID allkirjastatud ega notariaalselt kinnitatud.

## Kiire algus

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

Linux paketid installivad töölauakirje, mis käivitab `localsend-tray`, mis omab deemoni elutsüklit. Ilma olekuribata lauaarvutitel hoiab salv selle asemel avatuna GUI; `LocalSend --launcher` valib selle režiimi otse AppImage.

### Windows

Pakkige lahti ja käivitage `LocalSend.exe`. Arhiiv hoiab kõrvuti GUI, deemonit, salve ja käivitajat; hoia neid koos.

## Teadmist väärt käitumine

- **Väljumine on üks toiming.** Salvest väljumine lõpetab GUI, mis on salvestatud otspunkti ulatusega `gui.pid`, peatab omanduses oleva deemoni piiratud ajalõpu jooksul ja väljub protsessi või käivitatud elushoidmise tööd maha jätmata.
- **Teavitused tulevad salvest.** GUI ei postita kunagi oma teatisi, seega on igal platvormil üks tarnetee.
- **Akna olek on lõpp-punkti kohta.** Paigutus salvestatakse selle lõpp-punkti deemoni konfiguratsiooni kõrvale, mitte jagatud globaalsesse asukohta, nii et kaks eksemplari ei võitle akna geomeetria pärast.
- **Deemon on autoriteetne.** GUI muudetud sätted edastatakse deemonile ja kajatakse tagasi läbi hetktõmmise; frontend ei säilita privaatset koopiat, mis võiks triivida.

## Keeled

Liides tarnitakse 57 lokaadis, mis vastavad ülesvoolu LocalSend loendile ja msgid keeleks on lihtsustatud hiina keel. Valige üks jaotisest **Seaded → Keel** või jätke see OS-i lokaadi kasutamiseks valikule "Jälgi süsteemi".

| | |
| --- | --- |
| Komplekteeritud kataloogid | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UI allika stringid | `@tr("...")` in `apps/localsend-gui-slint/ui` |
| Inglise kataloog | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust sõnumid) |
| Roostepoolsed teated | `crates/localsend-i18n` (loodud `lookup_generated.rs`) |
| Vastuvoolu peegel | `translations/upstream/<locale>.json` |

Pärast mis tahes kasutajaliidese stringi muutmist genereerige kataloogid uuesti:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` on pühendunud, nii et loodud kataloogid on võrguühenduseta reprodutseeritavad. Parandage seal kirje ja käivitage generaator uuesti tõlke parandamiseks.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Ehitamine allikast

Nõuded: Rust `1.97.1` (kinnitatud `rust-toolchain.toml`) ja Linux järkude puhul GTK3 ja AppIndicator arenduspäised.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Platvormi paketid:

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

Aktsepteerimisskriptid juhivad pilatud oleku asemel tegelikke binaarfaile:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Dokumentatsioon

| Dokument | Sisu |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Kasutamine, pakendamine ja platvormi käitumine |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Praegused katsetõendid ja teadaolevad kontrollimata alad |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Kohalikud vastuvõtuväravad |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Enne käitumise muutmist oodatavad korduvad regressioonid ja tõendite jälg |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Visuaalne konvergentsi olek võrreldes OpenDesigni prototüübiga |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Android juurmoodul ja selle piirangud |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Müüja tuuma päritolu ülesvoolu |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Mõned dokumendid on kirjutatud hiina keeles; nad salvestavad kontrollkäigud ja kokkulepped sõnadega, milles nad kokku lepiti.

## Ulatus

Peamine toode on arvutiklient. Android tugi on ainult root-peadeta vastuvõtudeemon, kuni seda ei reklaamita sõnaselgelt kaugemale – sellel puudub GUI, see ei käivita esiplaani teenust ega postita püsivaid teatisi. `crates/localsend-official-core` hoitakse bait-baidi kaupa identne selle salvestatud ülesvoolu ekspordiga, välja arvatud selle genereeritud `SHA256SUMS` fail.

## Krediidid

Protokoll, juhtmevorming ja Rust tuum pärinevad [localsend/localsend](https://github.com/localsend/localsend); see projekt loob sellele tööle töölauakliendi, mitte ei rakenda protokolli uuesti. Aitäh LocalSend autoritele ja kaasautoritele.

Liides järgib selle projekti jaoks välja töötatud disaini prototüüpi; selle visuaalne viide asub asukohas [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Panustamine

- **Vead ja soovitused**: avage [probleem](https://github.com/alkaid-ops/localsend-next/issues) versiooni, platvormi, paljundamise sammude ja nähtuga. Logi või ekraanipilt muudab tegutsemise palju kiiremaks.
- **Kood**: tõmbamistaotlused on teretulnud. Muudatusi, mis puudutavad daemon/frontend piiri, tarnija tuuma või visuaalset lepingut, tuleks kõigepealt arutada – piirangud salvestatakse kaustas [`AGENTS.md`](../../AGENTS.md).
- **Tõlked**: kataloogid luuakse masintõlke vahemälust, nii et seal tehtud parandused on kõige lihtsamad ülevaatamiseks.

## Litsents

[Apache-2.0](../../LICENSE). Kolmandate osapoolte komponendid ja nende litsentsid on loetletud saidil [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

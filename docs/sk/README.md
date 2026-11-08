<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | **Slovenčina** | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [简体中文](../../docs/zh-CN/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next je desktopový klient kompatibilný s LocalSend napísaný v Rust. Hovorí oficiálnym protokolom, takže objavuje a prenáša súbory s oficiálnou aplikáciou LocalSend v oboch smeroch, pričom beží ako natívny desktopový program namiesto aplikácie Flutter.

Sieťová strana je oficiálne jadro LocalSend Rust dodávané nezmenené. Všetko nad ním – démon, vrstva RPC, zásobník, používateľské rozhranie terminálu a Slint GUI – je postavené okolo tohto jadra.

## Demo

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Odoslať | Nastavenia |
| --- | --- |
| ![Pošlite stránku so súborom, priečinkom, textom a zdrojmi schránky a zariadeniami v okolí](../../docs/images/readme-send-light.png) | ![Stránka nastavení s témou, jazykom, sieťou a možnosťami prenosu](../../docs/images/readme-settings-light.png) |

## Funkcie

- Zistenie a prenos proti oficiálnym LocalSend klientom, na dodávanom upstream jadre, nie opätovná implementácia
- Jeden kanonický vlastník stavu: démon má nastavenia, identitu, obľúbené položky, históriu, prevody a identitu TLS, takže každé rozhranie vidí to isté
- GUI, terminálové používateľské rozhranie a zásobník ako nezávislé frontendy cez rovnaké lokálne RPC; zásobník vlastní démona a životný cyklus procesu GUI
- Postup pri jednotlivých súboroch v reálnom čase, história s akciami otvárania a odhaľovania, obľúbené položky, manuálne zadávanie zariadení a zdieľanie na webe prijímajú odkazy
- 57 jazykov rozhrania zodpovedajúcich upstream zoznamu LocalSend, prepínateľné za behu alebo podľa miestneho nastavenia systému
- Natívna integrácia pracovnej plochy: upozornenia iba z panela, spustenie pri prihlásení, minimalizácia do panela, pretrvávanie umiestnenia okna a okno macOS, ktoré sa vtiahne do záhlavia
- Balenie pre macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) a Windows (ZIP), pre x86_64 aj aarch64

## Architektúra

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Démon je jediný proces, ktorý komunikuje so sieťou alebo zapisuje stav aplikácie. Frontendy nikdy nezávisia od dodávaného jadra a nikdy netvoria samotného démona; zásobník robí. Bajty súboru nepresahujú RPC — démon rieši cesty, hashuje obsah a zaznamenáva históriu.

| Binárne | Role |
| --- | --- |
| `localsendd` | Sieťová služba a stav kanonickej aplikácie |
| `localsend-tray` | Démon a vlastník životného cyklu GUI, upozornenia |
| `localsend-gui-slint` | Formálne GUI, Slint s rendererom FemtoVG |
| `localsend-tui` | Terminálové rozhranie |
| `localsend-launcher` | Explicitný spúšťač pre prostredia bez zásobníka |
| `localsend-androidd` | Samostatný root-iba Android prijímací démon (MVP) |

Adresáre `localsend-gui-egui`, `-fltk`, `-iced` a `-tauri` sú experimentálne porty a zostávajú mimo koreňového pracovného priestoru, kým sa nedokončí ich integrácia RPC.

## Stiahnuť

Aktíva vydania sa riadia jednou schémou pomenovania — `LocalSend-<version>-<platform>-<arch>.<ext>`, s platformami `linux` / `windows` / `macos` a architektúrami `x86_64` / `aarch64`:

| Platforma | aktíva |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Akékoľvek Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` pokrýva každé aktívum vo vydaní. Zostava macOS vytvorená spoločnosťou CI je podpísaná ad-hoc, nie je podpísaná ani notársky overená ID vývojára.

## Rýchly štart

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

Balíky Linux nainštalujú položku na ploche, ktorá spustí `localsend-tray`, ktorá vlastní životný cyklus démona. Na stolných počítačoch bez stavového riadku má zásobník otvorený GUI; `LocalSend --launcher` vyberá tento režim explicitne z AppImage.

### Windows

Rozbaľte a spustite `LocalSend.exe`. Archív obsahuje GUI, démona, zásobník a spúšťač vedľa seba; držať ich spolu.

## Správanie, ktoré stojí za to poznať

- **Ukončenie je jedna operácia.** Ukončením zo zásobníka sa ukončí GUI zaznamenaný v `gui.pid` s rozsahom koncového bodu, zastaví sa vlastnený démon v časovom limite a skončí sa bez zanechania procesu alebo spustenej udržiavacej úlohy.
- **Upozornenia prichádzajú zo zásobníka.** GUI nikdy nezverejňuje svoje vlastné upozornenia, takže každá platforma má jednu cestu doručenia.
- **Stav okna je pre každý koncový bod.** Umiestnenie je uložené vedľa konfigurácie démona pre tento koncový bod a nie v zdieľanom globálnom umiestnení, takže dve inštancie sa nehádajú o geometriu okna.
- **Démon je autoritatívny.** Nastavenia zmenené v GUI sa odoslali démonovi a odrazili sa späť cez snímku; frontend neuchováva žiadnu súkromnú kópiu, ktorá by sa mohla unášať.

## Jazyky

Rozhranie sa dodáva v 57 lokalitách, ktoré zodpovedajú predchádzajúcemu zoznamu LocalSend, s jazykom msgstr v zjednodušenej čínštine. Vyberte jeden v časti **Nastavenia → Jazyk** alebo ho nechajte na „Sledovať systém“, ak chcete použiť miestne nastavenie operačného systému.

| | |
| --- | --- |
| Balené katalógy | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| Zdrojové reťazce používateľského rozhrania | `@tr("...")` v `apps/localsend-gui-slint/ui` |
| Anglický katalóg | `translations/locale-en.json` (používateľské rozhranie), `translations/locale-en-runtime.json` (Rust správy) |
| Správy na strane hrdze | `crates/localsend-i18n` (vygenerované `lookup_generated.rs`) |
| Zrkadlo proti prúdu | `translations/upstream/<locale>.json` |

Obnovte katalógy po zmene ľubovoľného reťazca používateľského rozhrania:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` je potvrdené, takže vygenerované katalógy sú reprodukovateľné offline. Opravte tam položku a znova spustite generátor, aby ste opravili preklad.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Budovanie od zdroja

Požiadavky: Rust `1.97.1` (pripnuté `rust-toolchain.toml`) a pre zostavy Linux vývojové hlavičky GTK3 a AppIndicator.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Balíky platformy:

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

Akceptačné skripty riadia skutočné binárne súbory namiesto zosmiešňovaného stavu:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Dokumentácia

| Dokument | Obsah |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Prevádzka, balenie a správanie platformy |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Aktuálne testovacie dôkazy a známe neoverené oblasti |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Miestne akceptačné brány |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Opakujúce sa regresie a dôkazová stopa očakávaná pred zmenou správania |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Stav vizuálnej konvergencie oproti prototypu OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Koreňový modul Android a jeho limity |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Pôvodný pôvod dodávaného jadra |

Tento README je publikovaný vo všetkých 57 jazykoch rozhrania – vyberte si jeden z prepínača v hornej časti. Preklady sú generované z `README.md` od `scripts/translate-readme.mjs`; opravy idú do [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Niektoré dokumenty sú napísané v čínštine; zaznamenávajú overovacie jazdy a konvencie v slovách, v ktorých boli dohodnuté.

## Rozsah

Primárnym produktom je PC klient. Podpora Android je iba root-a bezhlavým prijímacím démonom, kým nie je explicitne povýšená mimo tento rozsah – nemá GUI, nespúšťa žiadnu službu na popredí a neposiela žiadne trvalé upozornenia. `crates/localsend-official-core` je udržiavaný bajt po bajte identický s jeho zaznamenaným upstream exportom, okrem jeho vygenerovaného súboru `SHA256SUMS`.

## Kredity

Protokol, formát drôtu a jadro Rust pochádza z [localsend/localsend](https://github.com/localsend/localsend); tento projekt stavia desktopového klienta na tejto práci a nie na re-implementácii protokolu. Ďakujeme autorom a prispievateľom LocalSend.

Rozhranie sa riadi dizajnovým prototypom vyvinutým pre tento projekt; jeho vizuálna referencia žije v [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Prispievanie

- **Chyby a návrhy**: otvorte [vydanie](https://github.com/alkaid-ops/localsend-next/issues) s verziou, platformou, krokmi reprodukcie a tým, čo ste videli. Vďaka záznamu alebo snímke obrazovky je oveľa rýchlejšia akcia.
- **Kód**: žiadosti o stiahnutie sú vítané. Zmeny, ktoré sa dotýkajú hranice daemon/frontend, dodávaného jadra alebo vizuálnej zmluvy, by sa mali prediskutovať najskôr – obmedzenia sú zaznamenané v [`AGENTS.md`](../../AGENTS.md).
- **Preklady**: katalógy sa generujú z vyrovnávacej pamäte strojového prekladu, takže opravy sú najjednoduchším druhom príspevku na kontrolu.

## Licencia

[Apache-2.0](../../LICENSE). Komponenty tretích strán a ich licencie sú uvedené v [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

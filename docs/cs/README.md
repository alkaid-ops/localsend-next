<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | **Čeština** | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next je desktopový klient kompatibilní s LocalSend napsaný v Rust. Mluví oficiálním protokolem, takže objevuje a přenáší soubory s oficiální aplikací LocalSend oběma směry, přičemž běží jako nativní desktopový program namísto aplikace Flutter.

Síťová strana je oficiální jádro LocalSend Rust, které se dodává beze změny. Vše nad ním – démon, vrstva RPC, zásobník, uživatelské rozhraní terminálu a Slint GUI – je postaveno kolem tohoto jádra.

## Demo

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Poslat | Nastavení |
| --- | --- |
| ![Odeslat stránku se souborem, složkou, textem a zdroji schránky a blízkými zařízeními](../../docs/images/readme-send-light.png) | ![Stránka nastavení s tématem, jazykem, sítí a možnostmi přenosu](../../docs/images/readme-settings-light.png) |

## Funkce

- Zjišťování a přenos proti oficiálním klientům LocalSend na dodávaném upstreamovém jádru spíše než reimplementace
- Jeden kanonický vlastník stavu: démon uchovává nastavení, identitu, oblíbené položky, historii, přenosy a identitu TLS, takže každý frontend vidí totéž
- GUI, uživatelské rozhraní terminálu a zásobník jako nezávislá rozhraní přes stejné místní RPC; zásobník vlastní démona a GUI životní cyklus procesu
- Průběh jednotlivých souborů v reálném čase, historie s akcemi otevírání a odhalování, oblíbená místa, ruční zadávání zařízení a sdílení na webu přijímají odkazy
- 57 jazyků rozhraní odpovídajících upstream seznamu LocalSend, přepínatelné za běhu nebo podle národního prostředí systému
- Nativní integrace plochy: oznámení pouze ze zásobníku, spouštění při přihlášení, minimalizace do zásobníku, trvalé umístění okna a okno macOS, které se vtahuje do záhlaví
- Balení pro macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) a Windows (ZIP), pro x86_64 i aarch64

## Architektura

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Démon je jediný proces, který komunikuje se sítí nebo zapisuje stav aplikace. Frontendy nikdy nezávisí na dodávaném jádru a nikdy nevytvářejí samotného démona; zásobník ano. Bajty souboru nepřesahují RPC — démon řeší cesty, hashuje obsah a zaznamenává historii.

| Binární | Role |
| --- | --- |
| `localsendd` | Síťová služba a stav kanonické aplikace |
| `localsend-tray` | Démon a vlastník životního cyklu GUI, upozornění |
| `localsend-gui-slint` | Formální GUI, Slint s rendererem FemtoVG |
| `localsend-tui` | Terminálové rozhraní |
| `localsend-launcher` | Explicitní spouštěč pro prostředí bez zásobníku |
| `localsend-androidd` | Samostatný root-only Android přijímací démon (MVP) |

Adresáře `localsend-gui-egui`, `-fltk`, `-iced` a `-tauri` jsou experimentální porty a zůstávají mimo kořenový pracovní prostor, dokud nebude dokončena jejich integrace RPC.

## Stáhnout

Aktiva vydání se řídí jedním schématem pojmenování — `LocalSend-<version>-<platform>-<arch>.<ext>`, s platformami `linux` / `windows` / `macos` a architekturami `x86_64` / `aarch64`:

| Platforma | Aktivum |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Jakékoli Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` pokrývá každé aktivum ve vydání. Sestavení macOS vytvořené CI je podepsáno ad-hoc, nikoli ID vývojáře podepsané nebo notářsky ověřené.

## Rychlý start

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

Balíčky Linux nainstalují položku na ploše, která spustí `localsend-tray`, která vlastní životní cyklus démona. Na stolních počítačích bez stavového řádku zásobník ponechává místo toho otevřený GUI; `LocalSend --launcher` vybere tento režim explicitně z AppImage.

### Windows

Rozbalte a spusťte `LocalSend.exe`. Archiv obsahuje GUI, démona, zásobník a spouštěč vedle sebe; držet je pohromadě.

## Chování, které stojí za to znát

- **Ukončit je jedna operace.** Ukončení ze zásobníku ukončí GUI zaznamenané v `gui.pid` s rozsahem koncového bodu, zastaví vlastněného démona v omezeném časovém limitu a ukončí se bez zanechání procesu nebo spuštěné udržovací úlohy.
- **Oznámení přicházejí ze zásobníku.** GUI nikdy nezveřejňuje svá vlastní oznámení, takže každá platforma má jedinou cestu doručení.
- **Stav okna je pro každý koncový bod.** Umístění je uloženo vedle konfigurace démona pro tento koncový bod, nikoli ve sdíleném globálním umístění, takže dvě instance spolu nebojují o geometrii okna.
- **Démon je autoritativní.** Nastavení změněná v GUI jsou odeslána démonovi a odezvou zpět prostřednictvím snímku; frontend neuchovává žádnou soukromou kopii, která by se mohla unášet.

## Jazyky

Rozhraní se dodává v 57 lokalitách, odpovídajících upstreamu LocalSend seznamu, se zjednodušenou čínštinou jako jazykem msgstr. Vyberte jeden v **Nastavení → Jazyk** nebo jej ponechte na „Sledovat systém“, chcete-li použít národní prostředí OS.

| | |
| --- | --- |
| Sbalené katalogy | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| Zdrojové řetězce uživatelského rozhraní | `@tr("...")` v `apps/localsend-gui-slint/ui` |
| Anglický katalog | `translations/locale-en.json` (uživatelské rozhraní), `translations/locale-en-runtime.json` (Rust zprávy) |
| Rust-side zprávy | `crates/localsend-i18n` (vygenerováno `lookup_generated.rs`) |
| Přední zrcadlo | `translations/upstream/<locale>.json` |

Po změně jakéhokoli řetězce uživatelského rozhraní znovu vygenerujte katalogy:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` je potvrzeno, takže vygenerované katalogy jsou reprodukovatelné offline. Opravte tam položku a znovu spusťte generátor, abyste opravili překlad.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Stavba ze zdroje

Požadavky: Rust `1.97.1` (připnuto `rust-toolchain.toml`) a pro sestavení Linux vývojové hlavičky GTK3 a AppIndicator.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Platformové balíčky:

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

Akceptační skripty řídí skutečné binární soubory namísto zesměšňovaného stavu:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Dokumentace

| Dokument | Obsah |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Provoz, balení a chování platformy |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Aktuální testovací důkazy a známé neověřené oblasti |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Místní přejímací brány |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Opakující se regrese a stopa důkazů očekávané před změnou chování |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Stav vizuální konvergence proti prototypu OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Kořenový modul Android a jeho limity |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Upstream provenience prodávaného jádra |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Některé dokumenty jsou psány v čínštině; zaznamenávají ověřovací běhy a konvence ve slovech, ve kterých byly dohodnuty.

## Rozsah

Primárním produktem je PC klient. Podpora Android je root-only, bezhlavý přijímací démon, dokud nebude výslovně povýšen mimo tento rozsah – nemá GUI, nespouští žádnou službu na popředí a nevysílá žádná trvalá upozornění. `crates/localsend-official-core` je udržován bajt po bajtu identický s jeho zaznamenaným upstream exportem, kromě jeho vygenerovaného souboru `SHA256SUMS`.

## Kredity

Protokol, formát drátu a jádro Rust pocházejí z [localsend/localsend](https://github.com/localsend/localsend); tento projekt staví desktopového klienta na této práci spíše než znovu implementuje protokol. Děkujeme autorům a přispěvatelům LocalSend.

Rozhraní odpovídá prototypu návrhu vyvinutému pro tento projekt; jeho vizuální reference žije v [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Přispívání

- **Chyby a návrhy**: otevřete [problém](https://github.com/alkaid-ops/localsend-next/issues) s verzí, platformou, kroky reprodukce a tím, co jste viděli. Protokol nebo snímek obrazovky umožňuje mnohem rychleji jednat.
- **Kód**: žádosti o stažení jsou vítány. Změny, které se dotýkají hranice daemon/frontend, dodávaného jádra nebo vizuální smlouvy, by měly být prodiskutovány jako první — omezení jsou zaznamenána v [`AGENTS.md`](../../AGENTS.md).
- **Překlady**: katalogy jsou generovány z vyrovnávací paměti pro strojové překlady, takže opravy jsou tím nejjednodušším druhem příspěvku ke kontrole.

## Licence

[Apache-2.0](../../LICENSE). Komponenty třetích stran a jejich licence jsou uvedeny v [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

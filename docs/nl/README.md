<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | **Nederlands** | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [简体中文](../../docs/zh-CN/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next is een LocalSend-compatibele desktopclient geschreven in Rust. Het spreekt het officiële protocol, dus het ontdekt en draagt ​​bestanden over met de officiële LocalSend-app in beide richtingen, terwijl het draait als een native desktopprogramma in plaats van een Flutter-app.

De netwerkzijde is de officiële LocalSend Rust kern, onveranderd verkocht. Alles daarboven – daemon, RPC laag, tray, terminal UI en de Slint GUI – is rond die kern gebouwd.

## Demo

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Versturen | Instellingen |
| --- | --- |
| ![Verzend pagina met de bestands-, map-, tekst- en klembordbronnen en apparaten in de buurt](../../docs/images/readme-send-light.png) | ![Instellingenpagina met thema, taal, netwerk en overdrachtsopties](../../docs/images/readme-settings-light.png) |

## Functies

- Ontdekking en overdracht tegen officiële LocalSend-clients, op de upstream-kern van de leverancier in plaats van een herimplementatie
- Eén canonieke staatseigenaar: de daemon bevat instellingen, identiteit, favorieten, geschiedenis, overdrachten en de TLS identiteit, zodat elke frontend hetzelfde ziet
- GUI, terminal-UI en lade als onafhankelijke frontends via dezelfde lokale RPC; de lade is eigenaar van de daemon en GUI proceslevenscyclus
- Realtime voortgang per bestand, geschiedenis met open- en onthullingsacties, favorieten, handmatige apparaatinvoer en Web Share-ontvangstlinks
- 57 interfacetalen die overeenkomen met de upstream LocalSend-lijst, omschakelbaar tijdens runtime of volgens de systeemlandinstellingen
- Native desktop-integratie: alleen meldingen vanuit de lade, starten bij inloggen, minimaliseren naar lade, persistentie van vensterplaatsing en een macOS-venster dat in de titelbalk terechtkomt
- Verpakking voor macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) en Windows (ZIP), voor zowel x86_64 als aarch64

## Architectuur

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

De daemon is het enige proces dat met het netwerk praat of de applicatiestatus schrijft. Frontends zijn nooit afhankelijk van de kern van de leverancier en brengen nooit de daemon zelf voort; het dienblad wel. Bestandsbytes gaan niet over RPC — de daemon lost paden op, hashes inhoud en registreert de geschiedenis.

| Binair | Rol |
| --- | --- |
| `localsendd` | Netwerkservice en canonieke applicatiestatus |
| `localsend-tray` | Daemon en GUI levenscycluseigenaar, meldingen |
| `localsend-gui-slint` | De formele GUI, Slint met de FemtoVG renderer |
| `localsend-tui` | Terminal-frontend |
| `localsend-launcher` | Expliciete launcher voor omgevingen zonder lade |
| `localsend-androidd` | Standalone root-only Android ontvangt daemon (MVP) |

De mappen `localsend-gui-egui`, `-fltk`, `-iced` en `-tauri` zijn experimentele poorten en blijven buiten de root-werkruimte totdat hun RPC-integratie voltooid is.

## Downloaden

Release-items volgen één naamgevingsschema — `LocalSend-<version>-<platform>-<arch>.<ext>`, met platforms `linux` / `windows` / `macos` en architecturen `x86_64` / `aarch64`:

| Platform | Bezit |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Elke Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` dekt elk item in een release. De macOS-build geproduceerd door CI is ad-hoc ondertekend, niet ondertekend of notarieel door de ontwikkelaars-ID.

## Snel beginnen

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

Linux-pakketten installeren een bureaubladitem dat `localsend-tray` start, dat eigenaar is van de daemon-levenscyclus. Op desktops zonder statusbalk houdt de lade de GUI open; `LocalSend --launcher` selecteert die modus expliciet uit AppImage.

### Windows

Pak het uit en voer `LocalSend.exe` uit. Het archief bevat de GUI, de daemon, de lade en de launcher naast elkaar; houd ze bij elkaar.

## Gedrag dat de moeite waard is om te weten

- **Afsluiten is één handeling.** Als u de lade verlaat, wordt de GUI beëindigd die is opgenomen in de eindpuntgerichte `gui.pid`, stopt de eigen daemon binnen een beperkte time-out en wordt afgesloten zonder een proces of een gestarte keep-alive-taak achter te laten.
- **Meldingen komen uit de lade.** De GUI plaatst nooit zijn eigen meldingen, dus elk platform heeft één bezorgpad.
- **Vensterstatus geldt per eindpunt.** Plaatsing wordt opgeslagen naast de daemonconfiguratie voor dat eindpunt in plaats van op een gedeelde globale locatie, zodat twee instanties geen ruzie maken over de venstergeometrie.
- **De daemon is gezaghebbend.** Instellingen die in GUI worden gewijzigd, worden verzonden naar de daemon en teruggestuurd via een momentopname; een frontend bewaart geen privékopie die zou kunnen wegdrijven.

## Talen

De interface wordt geleverd in 57 talen, overeenkomend met de stroomopwaartse lijst van LocalSend, met Vereenvoudigd Chinees als de msgid-taal. Kies er een onder **Instellingen → Taal**, of laat deze op "Systeem volgen" staan ​​om de landinstelling van het besturingssysteem te gebruiken.

| | |
| --- | --- |
| Gebundelde catalogi | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UI-brontekenreeksen | `@tr("...")` in `apps/localsend-gui-slint/ui` |
| Engelse catalogus | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust berichten) |
| Rust-side berichten | `crates/localsend-i18n` (gegenereerd `lookup_generated.rs`) |
| Stroomopwaartse spiegel | `translations/upstream/<locale>.json` |

Genereer de catalogi opnieuw nadat u een UI-tekenreeks hebt gewijzigd:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` is vastgelegd, dus de gegenereerde catalogi zijn offline reproduceerbaar. Corrigeer daar een invoer en voer de generator opnieuw uit om een ​​vertaling te corrigeren.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Bouwen vanuit de bron

Vereisten: Rust `1.97.1` (vastgezet door `rust-toolchain.toml`) en, voor Linux builds, de GTK3 en AppIndicator ontwikkelingsheaders.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Platformpakketten:

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

Acceptatiescripts sturen de echte binaire bestanden aan in plaats van de bespotte staat:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Documentatie

| Document | Inhoud |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Bediening, verpakking en platformgedrag |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Actueel testbewijs en bekende niet-geverifieerde gebieden |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Lokale acceptatiepoorten |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Terugkerende regressies en het verwachte bewijstraject voordat gedrag wordt veranderd |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Visuele convergentiestatus ten opzichte van het OpenDesign-prototype |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | De Android rootmodule en zijn limieten |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Stroomopwaartse herkomst van de verkochte kern |

Deze README is gepubliceerd in alle 57 interfacetalen – kies er één uit de switcher bovenaan. De vertalingen worden gegenereerd vanuit `README.md` door `scripts/translate-readme.mjs`; correcties gaan naar [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Sommige documenten zijn in het Chinees geschreven; ze leggen verificatieruns en conventies vast in de woorden waarin ze zijn overeengekomen.

## Bereik

Het primaire product is de pc-client. Android-ondersteuning is een root-only, headless ontvangst-daemon totdat deze expliciet buiten dat bereik wordt gepromoot - het heeft geen GUI, start geen voorgrondservice en plaatst geen permanente melding. `crates/localsend-official-core` wordt byte-voor-byte identiek gehouden aan de opgenomen upstream-export, afgezien van het gegenereerde `SHA256SUMS`-bestand.

## Dankbetuiging

Het protocol, het draadformaat en de Rust kern komen van [localsend/localsend](https://github.com/localsend/localsend); dit project bouwt een desktopclient op dat werk in plaats van het protocol opnieuw te implementeren. Met dank aan de LocalSend auteurs en bijdragers.

De interface volgt een ontwerpprototype dat voor dit project is ontwikkeld; de visuele referentie bevindt zich in [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Bijdragen

- **Bugs en suggesties**: open een [probleem](https://github.com/alkaid-ops/localsend-next/issues) met de versie, het platform, de reproductiestappen en wat je hebt gezien. Een log of screenshot maakt het veel sneller om actie te ondernemen.
- **Code**: pull-verzoeken zijn welkom. Veranderingen die de daemon/frontend-grens, de door de leverancier geleverde kern of het visuele contract raken, moeten eerst worden besproken; de beperkingen worden vastgelegd in [`AGENTS.md`](../../AGENTS.md).
- **Vertalingen**: de catalogi worden gegenereerd op basis van de vastgelegde cache voor automatische vertalingen, dus correcties daar zijn de gemakkelijkste bijdrage om te beoordelen.

## Licentie

[Apache-2.0](../../LICENSE). Componenten van derden en hun licenties worden vermeld in [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | **Svenska** | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next är en LocalSend-kompatibel skrivbordsklient skriven i Rust. Den talar det officiella protokollet, så den upptäcker och överför filer med den officiella LocalSend-appen i båda riktningarna, samtidigt som den körs som ett inbyggt skrivbordsprogram istället för en Flutter-app.

Nätverkssidan är den officiella LocalSend Rust kärnan, säljs oförändrad. Allt ovanför det – demon, RPC lager, bricka, terminalgränssnitt och Slint GUI – är byggt runt den kärnan.

## Demo

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Skicka | Inställningar |
| --- | --- |
| ![Skicka sida med filen, mappen, texten och urklippskällorna och närliggande enheter](../../docs/images/readme-send-light.png) | ![Inställningssida med tema, språk, nätverk och överföringsalternativ](../../docs/images/readme-settings-light.png) |

## Funktioner

- Upptäckt och överföring mot officiella LocalSend-klienter, på den levererade uppströmskärnan snarare än en omimplementering
- En kanonisk tillståndsägare: demonen har inställningar, identitet, favoriter, historik, överföringar och TLS-identiteten, så varje frontend ser samma sak
- GUI, terminalgränssnitt och magasin som oberoende gränssnitt över samma lokala RPC; facket äger daemon och GUI process livscykel
- Realtidsförlopp per fil, historik med öppna och avslöjande åtgärder, favoriter, manuell enhetsinmatning och webbdelning mottagningslänkar
- 57 gränssnittsspråk som matchar uppströms LocalSends lista, kan växlas under körning eller efter systemspråk
- Inbyggd skrivbordsintegration: aviseringar endast från facket, startas vid inloggning, minimera till fack, fönsterplacering ihållande och ett macOS-fönster som drar in i namnlisten
- Förpackningar för macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) och Windows (ZIP), för både x86_64 och aarch64

## Arkitektur

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Demonen är den enda processen som pratar med nätverket eller skriver applikationstillstånd. Gränssnitt är aldrig beroende av den levererade kärnan och skapar aldrig demonen själva; brickan gör det. Filbytes går inte över RPC — demonen löser sökvägar, hashar innehåll och registrerar historik.

| Binär | Roll |
| --- | --- |
| `localsendd` | Nätverkstjänst och kanoniskt applikationstillstånd |
| `localsend-tray` | Daemon och GUI livscykelägare, meddelanden |
| `localsend-gui-slint` | Den formella GUI, Slint med FemtoVG renderaren |
| `localsend-tui` | Terminal frontend |
| `localsend-launcher` | Explicit startprogram för miljöer utan bricka |
| `localsend-androidd` | Fristående root-only Android motta demon (MVP) |

Katalogen `localsend-gui-egui`, `-fltk`, `-iced` och `-tauri` är experimentportar och stannar utanför rotarbetsytan tills deras RPC-integrering är klar.

## Ladda ner

Frisläppstillgångar följer ett namnschema — `LocalSend-<version>-<platform>-<arch>.<ext>`, med plattformar `linux` / `windows` / `macos` och arkitekturer `x86_64` / `aarch64`:

| Plattform | Tillgång |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Alla Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` täcker varje tillgång i en release. The macOS build produced by CI is ad-hoc signed, not Developer ID signed or notarised.

## Snabbstart

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

Linux-paket installerar en skrivbordspost som startar `localsend-tray`, som äger demonens livscykel. På stationära datorer utan statusfält håller facket GUI öppet istället; `LocalSend --launcher` väljer det läget uttryckligen från AppImage.

### Windows

Packa upp och kör `LocalSend.exe`. Arkivet håller GUI, demonen, facket och startprogrammet sida vid sida; hålla dem tillsammans.

## Beteende värt att känna till

- **Avsluta är en operation.** Att avsluta från facket avslutar GUI som registrerats i den slutpunktsomfattade `gui.pid`, stoppar den ägda demonen inom en begränsad timeout och avslutas utan att lämna en process eller ett startat keep-alive-jobb bakom sig.
- **Meddelanden kommer från facket.** GUI publicerar aldrig sina egna aviseringar, så varje plattform har en enda leveransväg.
- **Fönstertillstånd är per ändpunkt.** Placering lagras bredvid demonkonfigurationen för den ändpunkten snarare än på en delad global plats, så två instanser slåss inte om fönstergeometrin.
- **Demonen är auktoritativ.** Inställningar som ändrats i GUI skickas till demonen och ekas tillbaka genom en ögonblicksbild; en frontend behåller ingen privat kopia som kan glida.

## Språk

Gränssnittet levereras i 57 lokaler, matchande uppströms LocalSends lista, med förenklad kinesiska som msgid-språk. Välj en under **Inställningar → Språk**, eller lämna den på "Följ system" för att använda OS-lokalen.

| | |
| --- | --- |
| Samlade kataloger | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UI-källsträngar | `@tr("...")` i `apps/localsend-gui-slint/ui` |
| Engelsk katalog | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust meddelanden) |
| Meddelanden på rostsidan | `crates/localsend-i18n` (genererad `lookup_generated.rs`) |
| Uppströms spegel | `translations/upstream/<locale>.json` |

Återskapa katalogerna efter att ha ändrat en gränssnittssträng:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` är engagerad, så de genererade katalogerna är reproducerbara offline. Rätta en post där och kör generatorn igen för att fixa en översättning.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Bygga från källan

Krav: Rust `1.97.1` (fäst av `rust-toolchain.toml`) och, för Linux builds, GTK3 och AppIndicator utvecklingshuvudena.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Plattformspaket:

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

Acceptansskript driver de riktiga binärfilerna istället för det hånade tillståndet:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Dokumentation

| Dokumentera | Innehåll |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Drift, paketering och plattformsbeteende |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Aktuella testbevis och kända overifierade områden |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Lokala mottagningsportar |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Återkommande regressioner och bevisspåret som förväntas innan beteendet ändras |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Visuell konvergenstillstånd mot OpenDesign-prototypen |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Android rotmodulen och dess begränsningar |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Uppströms härkomst av den levererade kärnan |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Vissa dokument är skrivna på kinesiska; de registrerar verifieringskörningar och konventioner med de ord som de överenskoms i.

## Omfattning

Den primära produkten är PC-klienten. Android support är en root-enbart, huvudlös mottagningsdemon tills den uttryckligen marknadsförs utanför detta räckvidd – den har ingen GUI, startar ingen förgrundstjänst och skickar inga ihållande meddelanden. `crates/localsend-official-core` hålls byte-för-byte identisk med dess inspelade uppströmsexport, förutom den genererade `SHA256SUMS`-filen.

## Krediter

Protokollet, trådformatet och Rust kärnan kommer från [localsend/localsend](https://github.com/localsend/localsend); det här projektet bygger en skrivbordsklient på det arbetet snarare än att implementera om protokollet. Tack till LocalSend författare och bidragsgivare.

Gränssnittet följer en designprototyp som utvecklats för detta projekt; dess visuella referens bor i [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Bidrar

- **Buggar och förslag**: öppna ett [problem](https://github.com/alkaid-ops/localsend-next/issues) med version, plattform, reproduktionssteg och vad du såg. En logg eller skärmdump gör det mycket snabbare att agera på.
- **Kod**: pull-förfrågningar är välkomna. Ändringar som rör daemon/frontend-gränsen, leverantörens kärna eller det visuella kontraktet bör diskuteras först – begränsningarna registreras i [`AGENTS.md`](../../AGENTS.md).
- **Översättningar**: katalogerna genereras från den översatta maskinöversättningscachen, så korrigeringar där är den enklaste typen av bidrag att granska.

## Licens

[Apache-2.0](../../LICENSE). Tredjepartskomponenter och deras licenser listas i [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

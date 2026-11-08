<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | **Dansk** | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [简体中文](../../docs/zh-CN/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next er en LocalSend-kompatibel desktop-klient skrevet i Rust. Den taler den officielle protokol, så den opdager og overfører filer med den officielle LocalSend app i begge retninger, mens den kører som et indbygget skrivebordsprogram i stedet for en Flutter-app.

Netværkssiden er den officielle LocalSend Rust kerne, sælges uændret. Alt over det - daemon, RPC lag, bakke, terminal UI og Slint GUI - er bygget op omkring den kerne.

## Demo

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Sende | Indstillinger |
| --- | --- |
| ![Send side med fil-, mappe-, tekst- og udklipsholderkilder og enheder i nærheden](../../docs/images/readme-send-light.png) | ![Indstillingsside med tema, sprog, netværk og overførselsmuligheder](../../docs/images/readme-settings-light.png) |

## Funktioner

- Opdagelse og overførsel mod officielle LocalSend-klienter, på den forhandlede upstream-kerne snarere end en re-implementering
- Én kanonisk statsejer: dæmonen har indstillinger, identitet, favoritter, historie, overførsler og TLS-identiteten, så alle frontend ser det samme
- GUI, terminal UI og bakke som uafhængige frontends over den samme lokale RPC; bakken ejer daemon og GUI proces livscyklus
- Fremskridt i realtid pr. fil, historik med åbne og åbenbare handlinger, favoritter, manuel enhedsindtastning og webdeling modtagelseslinks
- 57 grænsefladesprog, der matcher opstrøms LocalSends liste, kan skiftes under kørsel eller følger systemets lokalitet
- Native desktop integration: meddelelser kun fra bakken, start ved login, minimer til bakke, vinduesplacering persistens og et macOS vindue, der trækker ind i dets titellinje
- Emballage til macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) og Windows (ZIP), til både x86_64 og aarch64

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

Dæmonen er den eneste proces, der taler til netværket eller skriver applikationstilstand. Frontends afhænger aldrig af den sælgende kerne og afføder aldrig selve dæmonen; bakken gør. Filbytes bevæger sig ikke over RPC — dæmonen løser stier, hashes indhold og registrerer historik.

| Binær | Rolle |
| --- | --- |
| `localsendd` | Netværkstjeneste og kanonisk applikationstilstand |
| `localsend-tray` | Daemon og GUI livscyklusejer, meddelelser |
| `localsend-gui-slint` | Den formelle GUI, Slint med FemtoVG rendereren |
| `localsend-tui` | Terminal frontend |
| `localsend-launcher` | Eksplicit launcher til miljøer uden en bakke |
| `localsend-androidd` | Standalone root-only Android modtage dæmon (MVP) |

Mappen `localsend-gui-egui`, `-fltk`, `-iced` og `-tauri` er eksperimentelle porte og forbliver uden for rodarbejdsområdet, indtil deres RPC-integration er fuldført.

## Download

Frigivelsesaktiver følger ét navngivningsskema — `LocalSend-<version>-<platform>-<arch>.<ext>`, med platforme `linux` / `windows` / `macos` og arkitekturer `x86_64` / `aarch64`:

| Platform | Aktiv |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Enhver Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` dækker hvert aktiv i en udgivelse. macOS buildet produceret af CI er ad-hoc signeret, ikke udvikler-id signeret eller notariseret.

## Hurtig start

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

Linux-pakker installerer en desktop-indgang, der starter `localsend-tray`, som ejer dæmonens livscyklus. På skriveborde uden en statuslinje holder bakken GUI åben i stedet for; `LocalSend --launcher` vælger denne tilstand eksplicit fra AppImage.

### Windows

Udpak og kør `LocalSend.exe`. Arkivet rummer GUI, dæmonen, bakken og launcheren side om side; holde dem sammen.

## Adfærd værd at kende

- **Afslut er én operation.** Afslutning fra bakken afslutter GUI, der er optaget i det endpoint-scoped `gui.pid`, stopper den ejede dæmon inden for en begrænset timeout og afsluttes uden at efterlade en proces eller et lanceret keep-alive-job.
- **Meddelelser kommer fra bakken.** GUI sender aldrig sine egne notifikationer, så hver platform har en enkelt leveringssti.
- **Vinduetilstand er pr. slutpunkt.** Placering gemmes ved siden af ​​dæmonkonfigurationen for det pågældende slutpunkt i stedet for på en delt global placering, så to forekomster kæmper ikke om vinduesgeometrien.
- **Dæmonen er autoritativ.** Indstillinger, der er ændret i GUI, sendes til dæmonen og gentages gennem et snapshot; en frontend beholder ingen privat kopi, der kan glide.

## Sprog

Grænsefladen leveres i 57 lokaliteter, der matcher opstrøms LocalSends liste, med forenklet kinesisk som msgid-sproget. Vælg en under **Indstillinger → Sprog**, eller lad den stå på "Følg system" for at bruge OS-lokaliteten.

| | |
| --- | --- |
| Medfølgende kataloger | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UI-kildestrenge | `@tr("...")` i `apps/localsend-gui-slint/ui` |
| Engelsk katalog | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust beskeder) |
| Beskeder på rustsiden | `crates/localsend-i18n` (genereret `lookup_generated.rs`) |
| Opstrøms spejl | `translations/upstream/<locale>.json` |

Gendan katalogerne efter at have ændret enhver UI-streng:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` er forpligtet, så de genererede kataloger kan reproduceres offline. Ret en indtastning der, og kør generatoren igen for at rette en oversættelse.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Byg fra kilden

Krav: Rust `1.97.1` (fastgjort af `rust-toolchain.toml`) og, for Linux builds, GTK3 og AppIndicator udviklingsheaderne.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Platformpakker:

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

Acceptscripts driver de rigtige binære filer i stedet for hånet tilstand:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Dokumentation

| Dokument | Indhold |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Drift, pakning og platformadfærd |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Aktuel testbevis og kendte ubekræftede områder |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Lokale acceptporte |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Tilbagevendende regressioner og det forventede evidensspor før adfærdsændring |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Visuel konvergenstilstand mod OpenDesign-prototypen |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Android rodmodulet og dets grænser |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Opstrøms herkomst af den sælgende kerne |

Denne README er udgivet på alle 57 grænsefladesprog - vælg et fra switcheren øverst. Oversættelserne er genereret fra `README.md` af `scripts/translate-readme.mjs`; rettelser gå i [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Nogle dokumenter er skrevet på kinesisk; de registrerer verifikationskørsler og konventioner i de ord, de blev aftalt i.

## Omfang

Det primære produkt er pc-klienten. Android support er en root-only, hovedløs modtagelsesdæmon, indtil den eksplicit promoveres ud over dette omfang – den har ingen GUI, starter ingen forgrundstjeneste og sender ingen vedvarende notifikationer. `crates/localsend-official-core` holdes byte-for-byte identisk med dens registrerede upstream-eksport, bortset fra dens genererede `SHA256SUMS`-fil.

## Kreditter

Protokollen, ledningsformatet og Rust kernen kommer fra [localsend/localsend](https://github.com/localsend/localsend); dette projekt bygger en desktop-klient på det arbejde i stedet for at genimplementere protokollen. Tak til LocalSend forfattere og bidragydere.

Interfacet følger en designprototype udviklet til dette projekt; dens visuelle reference bor i [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Bidrager

- **Bugs og forslag**: Åbn et [problem](https://github.com/alkaid-ops/localsend-next/issues) med version, platform, reproduktionstrin og hvad du så. En log eller et skærmbillede gør det meget hurtigere at handle på.
- **Kode**: pull-anmodninger er velkomne. Ændringer, der berører daemon/frontend-grænsen, den sælgende kerne eller den visuelle kontrakt, bør diskuteres først - begrænsningerne er registreret i [`AGENTS.md`](../../AGENTS.md).
- **Oversættelser**: katalogerne er genereret fra den forpligtede maskinoversættelsescache, så rettelser der er den nemmeste form for bidrag at gennemgå.

## Licens

[Apache-2.0](../../LICENSE). Tredjepartskomponenter og deres licenser er angivet i [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

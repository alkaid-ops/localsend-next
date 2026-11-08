<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | **Català** | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next és un client d'escriptori compatible amb LocalSend escrit en Rust. Parla el protocol oficial, de manera que descobreix i transfereix fitxers amb l'aplicació oficial LocalSend en ambdues direccions, mentre s'executa com un programa d'escriptori natiu en lloc d'una aplicació Flutter.

El costat de la xarxa és el nucli oficial LocalSend Rust, que es distribueix sense canvis. Tot el que hi ha a sobre (dimoni, capa RPC, safata, interfície d'usuari del terminal i Slint GUI) es construeix al voltant d'aquest nucli.

## Demostració

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Enviar | Configuració |
| --- | --- |
| ![Envia la pàgina amb el fitxer, la carpeta, el text i les fonts del porta-retalls i els dispositius propers](../../docs/images/readme-send-light.png) | ![Pàgina de configuració amb opcions de tema, idioma, xarxa i transferència](../../docs/images/readme-settings-light.png) |

## Funcions

- Descobriment i transferència contra clients LocalSend oficials, al nucli amunt del proveïdor en lloc d'una reimplementació
- Un propietari d'estat canònic: el dimoni conté la configuració, la identitat, els preferits, l'historial, les transferències i la identitat TLS, de manera que cada interfície veu el mateix
- GUI, interfície d'usuari de terminal i safata com a interfície independent sobre el mateix RPC local; la safata té el dimoni i el cicle de vida del procés GUI
- Progrés per fitxer en temps real, historial amb accions obertes i revelades, favorits, entrada manual del dispositiu i enllaços de recepció de Web Share
- 57 idiomes d'interfície que coincideixen amb la llista de LocalSend, canviables en temps d'execució o seguint la configuració regional del sistema
- Integració nativa d'escriptori: notificacions només des de la safata, llançament en iniciar sessió, minimitzar a la safata, persistència de la col·locació de finestres i una finestra macOS que s'inclou a la seva barra de títol
- Embalatge per a macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) i Windows (ZIP), tant per a x86_64 com per a aarch64

## Arquitectura

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

El dimoni és l'únic procés que parla amb la xarxa o escriu l'estat de l'aplicació. Els frontends mai depenen del nucli del proveïdor i mai generen el dimoni; la safata ho fa. Els bytes del fitxer no viatgen per RPC: el dimoni resol camins, hash contingut i enregistra l'historial.

| Binari | Rol |
| --- | --- |
| `localsendd` | Servei de xarxa i estat de l'aplicació canònica |
| `localsend-tray` | Daemon i GUI propietari del cicle de vida, notificacions |
| `localsend-gui-slint` | El GUI formal, Slint amb el renderitzador FemtoVG |
| `localsend-tui` | Interfície del terminal |
| `localsend-launcher` | Llançador explícit per a entorns sense safata |
| `localsend-androidd` | Dimoni de recepció (MVP) autònom només Android |

Els directoris `localsend-gui-egui`, `-fltk`, `-iced` i `-tauri` són ports experimentals i romanen fora de l'espai de treball arrel fins que es completa la seva integració RPC.

## Descarregar

Els actius de llançament segueixen un esquema de nomenclatura: `LocalSend-<version>-<platform>-<arch>.<ext>`, amb plataformes `linux` / `windows` / `macos` i arquitectures `x86_64` / `aarch64`:

| Plataforma | actiu |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Qualsevol Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` cobreix tots els recursos d'una versió. La compilació macOS produïda per CI està signada ad-hoc, no amb l'identificador de desenvolupador ni signat per notari.

## Inici ràpid

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

Els paquets Linux instal·len una entrada d'escriptori que llança `localsend-tray`, que és propietari del cicle de vida del dimoni. En els escriptoris sense barra d'estat, la safata manté el GUI obert; `LocalSend --launcher` selecciona aquest mode explícitament a AppImage.

### Windows

Descomprimiu i executeu `LocalSend.exe`. L'arxiu conté el GUI, el dimoni, la safata i el llançador junts; mantenir-los junts.

## Comportament que val la pena conèixer

- **Surt és una operació.** Sortir des de la safata finalitza el GUI registrat al `gui.pid` amb l'àmbit del punt final, atura el dimoni de propietat dins d'un temps d'espera limitat i surt sense deixar enrere un procés o un treball de manteniment d'un launchd.
- **Les notificacions provenen de la safata.** El GUI mai publica les seves pròpies notificacions, de manera que cada plataforma té una única ruta de lliurament.
- **L'estat de la finestra és per punt final.** La ubicació s'emmagatzema al costat de la configuració del dimoni per a aquest punt final en lloc d'una ubicació global compartida, de manera que dues instàncies no es barallen per la geometria de la finestra.
- **El dimoni és autoritzat.** Els paràmetres canviats a GUI s'envien al dimoni i es reprodueixen a través d'una instantània; una interfície no guarda cap còpia privada que pugui derivar.

## Idiomes

La interfície s'envia en 57 configuracions regionals, que coincideixen amb la llista de LocalSend aigües amunt, amb el xinès simplificat com a idioma msgid. Trieu-ne un a **Configuració → Idioma**, o deixeu-lo a "Seguir el sistema" per utilitzar la configuració regional del sistema operatiu.

| | |
| --- | --- |
| Catàlegs agrupats | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| cadenes d'origen de la IU | `@tr("...")` a `apps/localsend-gui-slint/ui` |
| Catàleg en anglès | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust missatges) |
| Missatges del costat de l'òxid | `crates/localsend-i18n` (generat `lookup_generated.rs`) |
| Mirall aigües amunt | `translations/upstream/<locale>.json` |

Regenera els catàlegs després de canviar qualsevol cadena d'IU:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` està compromès, de manera que els catàlegs generats són reproduïbles fora de línia. Corregiu una entrada allà i torneu a executar el generador per arreglar una traducció.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Construcció des de la font

Requisits: Rust `1.97.1` (fixat per `rust-toolchain.toml`) i, per a les compilacions Linux, les capçaleres de desenvolupament GTK3 i AppIndicator.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Paquets de plataforma:

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

Els scripts d'acceptació condueixen els binaris reals en lloc de l'estat burlat:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Documentació

| Document | Continguts |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Funcionament, embalatge i comportament de la plataforma |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Evidència de la prova actual i àrees conegudes no verificades |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Portes d'acceptació locals |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Regressions recurrents i el rastre de l'evidència esperat abans de canviar el comportament |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Estat de convergència visual amb el prototip d'OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | El mòdul arrel Android i els seus límits |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Procedència aigües amunt del nucli distribuït |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Alguns documents estan escrits en xinès; registren les execucions de verificació i les convencions amb les paraules en què es van acordar.

## Abast

El producte principal és el client de PC. El suport de Android és un dimoni de recepció sense cap, només arrel, fins que no es promociona explícitament més enllà d'aquest àmbit: no té GUI, no inicia cap servei en primer pla i no publica cap notificació persistent. `crates/localsend-official-core` es manté byte per byte idèntic a la seva exportació amunt gravada, a part del fitxer `SHA256SUMS` generat.

## Crèdits

El protocol, el format de cable i el nucli Rust provenen de [localsend/localsend](https://github.com/localsend/localsend); aquest projecte crea un client d'escriptori sobre aquest treball en lloc de tornar a implementar el protocol. Gràcies als LocalSend autors i col·laboradors.

La interfície segueix un prototip de disseny desenvolupat per a aquest projecte; la seva referència visual viu a [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Contribuint

- **Errores i suggeriments**: obriu un [problema] (https://github.com/alkaid-ops/localsend-next/issues) amb la versió, la plataforma, els passos de reproducció i el que heu vist. Un registre o una captura de pantalla fa que sigui molt més ràpid actuar.
- **Codi**: les sol·licituds d'extracció són benvingudes. Els canvis que toquen el límit daemon/frontend, el nucli del proveïdor o el contracte visual s'han de discutir primer; les restriccions es registren a [`AGENTS.md`](../../AGENTS.md).
- **Traducció**: els catàlegs es generen a partir de la memòria cau de traducció automàtica compromesa, de manera que les correccions són el tipus de contribució més fàcil de revisar.

## llicència

[Apache-2.0](../../LICENSE). Els components de tercers i les seves llicències es mostren a [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

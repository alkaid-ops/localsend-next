<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | **Galego** | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [简体中文](../../docs/zh-CN/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next é un cliente de escritorio compatible con LocalSend escrito en Rust. Fala o protocolo oficial, polo que descobre e transfire ficheiros coa aplicación oficial LocalSend en ambas direccións, mentres se executa como un programa de escritorio nativo en lugar dunha aplicación Flutter.

O lado da rede é o núcleo oficial LocalSend Rust, que se vende sen cambios. Todo o que está por encima (demonio, capa RPC, bandexa, interface de usuario do terminal e Slint GUI) está construído arredor dese núcleo.

## Demostración

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Enviar | Configuración |
| --- | --- |
| ![Enviar páxina co ficheiro, cartafol, fontes de texto e portapapeis e dispositivos próximos](../../docs/images/readme-send-light.png) | ![Páxina de configuración con opcións de tema, idioma, rede e transferencia](../../docs/images/readme-settings-light.png) |

## Funcións

- Descubrimento e transferencia contra clientes LocalSend oficiais, no núcleo ascendente do provedor en lugar dunha reimplementación
- Un propietario de estado canónico: o daemon contén a configuración, a identidade, os favoritos, o historial, as transferencias e a identidade TLS, polo que cada frontend ve o mesmo
- GUI, interface de usuario de terminal e bandexa como frontends independentes sobre o mesmo RPC local; a bandexa posúe o daemon e o ciclo de vida do proceso GUI
- Progreso por ficheiro en tempo real, historial con accións abertas e de revelación, favoritos, entrada manual do dispositivo e ligazóns de recepción Web Share
- 57 idiomas de interface coincidentes coa lista de LocalSend, conmutables en tempo de execución ou seguindo a configuración rexional do sistema
- Integración nativa do escritorio: notificacións só desde a bandexa, inicio ao iniciar sesión, minimizar na bandexa, persistencia da colocación da fiestra e unha ventá macOS que se incorpora á súa barra de título
- Embalaxe para macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) e Windows (ZIP), tanto para x86_64 como para aarch64

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

O daemon é o único proceso que fala coa rede ou escribe o estado da aplicación. As interfaces nunca dependen do núcleo proveído e nunca xeran o propio daemon; a bandexa fai. Os bytes do ficheiro non viaxan por RPC — o daemon resolve camiños, hash contido e rexistra o historial.

| Binario | Papel |
| --- | --- |
| `localsendd` | Servizo de rede e estado da aplicación canónica |
| `localsend-tray` | Daemon e GUI propietario do ciclo de vida, notificacións |
| `localsend-gui-slint` | O GUI formal, Slint co renderizador FemtoVG |
| `localsend-tui` | Interface terminal |
| `localsend-launcher` | Lanzador explícito para ambientes sen bandexa |
| `localsend-androidd` | Daemon de recepción (MVP) Android de root autónomo |

Os directorios `localsend-gui-egui`, `-fltk`, `-iced` e `-tauri` son portos experimentais e permanecen fóra do espazo de traballo raíz ata que se complete a súa integración RPC.

## Descargar

Os recursos de lanzamento seguen un esquema de nomenclatura: `LocalSend-<version>-<platform>-<arch>.<ext>`, con plataformas `linux` / `windows` / `macos` e arquitecturas `x86_64` / `aarch64`:

| Plataforma | Activo |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Calquera Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` cobre todos os recursos dunha versión. A compilación macOS producida por CI está asinada ad-hoc, non o ID de programador asinado nin notariado.

## Inicio rápido

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

Os paquetes Linux instalan unha entrada de escritorio que inicia `localsend-tray`, que posúe o ciclo de vida do daemon. Nos escritorios sen barra de estado, a bandexa mantén aberta a GUI; `LocalSend --launcher` selecciona ese modo explícitamente desde AppImage.

### Windows

Descomprimir e executar `LocalSend.exe`. O arquivo contén o GUI, o daemon, a bandexa e o lanzador un ao carón; mantelos xuntos.

## Comportamento que paga a pena coñecer

- **Saír é unha operación.** Ao saír da bandexa finaliza o GUI rexistrado no `gui.pid` con alcance do punto final, detén o daemon que posúe dentro dun tempo de espera limitado e sae sen deixar atrás un proceso ou un traballo de mantemento do launchd.
- **As notificacións veñen da bandexa.** O GUI nunca publica as súas propias notificacións, polo que cada plataforma ten unha única ruta de entrega.
- **O estado da xanela é por punto final.** A colocación gárdase xunto á configuración do daemon para ese punto final e non nunha localización global compartida, polo que dúas instancias non pelexan pola xeometría da xanela.
- **O daemon ten autoridade.** As configuracións modificadas en GUI envíanse ao daemon e repítense a través dunha instantánea; un frontend non garda ningunha copia privada que poida derivar.

## Linguas

A interface envíase en 57 locais, coincidindo coa lista de LocalSend, co chinés simplificado como lingua msgid. Escolla un en **Configuración → Idioma** ou déixao en "Seguir sistema" para usar a configuración rexional do SO.

| | |
| --- | --- |
| Catálogos agrupados | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| Cadenas de orixe da IU | `@tr("...")` en `apps/localsend-gui-slint/ui` |
| Catálogo inglés | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust mensaxes) |
| Mensaxes do lado do ferruxe | `crates/localsend-i18n` (xerado `lookup_generated.rs`) |
| Espello augas arriba | `translations/upstream/<locale>.json` |

Rexenera os catálogos despois de cambiar calquera cadea de IU:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` está comprometido, polo que os catálogos xerados son reproducibles fóra de liña. Corrixe unha entrada alí e volve executar o xerador para corrixir unha tradución.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Construír dende a orixe

Requisitos: Rust `1.97.1` (fixado por `rust-toolchain.toml`) e, para as compilacións Linux, as cabeceiras de desenvolvemento GTK3 e AppIndicator.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Paquetes de plataforma:

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

Os scripts de aceptación impulsan os binarios reais en lugar do estado burlado:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Documentación

| Documento | Contidos |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Funcionamento, embalaxe e comportamento da plataforma |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Evidencia das probas actuais e áreas coñecidas sen verificar |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Portas de aceptación locais |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Regresións recorrentes e o rastro de evidencia esperado antes de cambiar o comportamento |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Estado de converxencia visual contra o prototipo de OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | O módulo raíz Android e os seus límites |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Procedencia upstream do núcleo provedor |

Este README publícase nos 57 idiomas da interface. Escolla un no selector da parte superior. As traducións son xeradas desde `README.md` por `scripts/translate-readme.mjs`; as correccións van en [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Algúns documentos están escritos en chinés; rexistran as operacións de verificación e as convencións nas palabras nas que se acordaron.

## Alcance

O produto principal é o cliente de PC. A compatibilidade de Android é un daemon de recepción sen cabeza só para root ata que se promova explícitamente fóra dese ámbito: non ten GUI, non inicia servizo en primeiro plano e non publica notificacións persistentes. `crates/localsend-official-core` mantense byte por byte idéntico á súa exportación ascendente gravada, ademais do ficheiro `SHA256SUMS` xerado.

## Créditos

O protocolo, o formato de cable e o núcleo Rust proveñen de [localsend/localsend](https://github.com/localsend/localsend); este proxecto constrúe un cliente de escritorio sobre ese traballo en lugar de volver a implementar o protocolo. Grazas aos LocalSend autores e colaboradores.

A interface segue un prototipo de deseño desenvolvido para este proxecto; a súa referencia visual vive en [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Contribuíndo

- **Errores e suxestións**: abre un [problema] (https://github.com/alkaid-ops/localsend-next/issues) coa versión, plataforma, pasos de reprodución e o que viu. Un rexistro ou unha captura de pantalla fai que sexa moito máis rápido actuar.
- **Código**: as solicitudes de extracción son benvidas. Os cambios que tocan o límite daemon/frontend, o núcleo do provedor ou o contrato visual deben ser discutidos primeiro; as restricións rexístranse en [`AGENTS.md`](../../AGENTS.md).
- **Traducións**: os catálogos xéranse a partir da caché de tradución automática comprometida, polo que as correccións son o tipo de contribución máis fácil de revisar.

## Licenza

[Apache-2.0] (LICENSE). Os compoñentes de terceiros e as súas licenzas están listados en [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

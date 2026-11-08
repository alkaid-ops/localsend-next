<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | **Español** | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next es un cliente de escritorio compatible con LocalSend escrito en Rust. Habla el protocolo oficial, por lo que descubre y transfiere archivos con la aplicación oficial LocalSend en ambas direcciones, mientras se ejecuta como un programa de escritorio nativo en lugar de una aplicación Flutter.

El lado de la red es el núcleo oficial LocalSend Rust, suministrado sin cambios. Todo lo que está encima (demonio, capa RPC, bandeja, interfaz de usuario del terminal y Slint GUI) está construido alrededor de ese núcleo.

## Manifestación

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Enviar | Ajustes |
| --- | --- |
| ![Enviar página con las fuentes del archivo, carpeta, texto y portapapeles y dispositivos cercanos](../../docs/images/readme-send-light.png) | ![Página de configuración con tema, idioma, red y opciones de transferencia](../../docs/images/readme-settings-light.png) |

## Funciones

- Descubrimiento y transferencia contra clientes oficiales LocalSend, en el núcleo ascendente suministrado en lugar de una reimplementación
- Un propietario de estado canónico: el demonio contiene configuraciones, identidad, favoritos, historial, transferencias y la identidad TLS, por lo que cada interfaz ve lo mismo
- GUI, interfaz de usuario del terminal y bandeja como interfaces independientes sobre el mismo RPC local; la bandeja posee el demonio y el ciclo de vida del proceso GUI
- Progreso por archivo en tiempo real, historial con acciones de apertura y revelación, favoritos, entrada manual del dispositivo y enlaces de recepción de Web Share
- 57 idiomas de interfaz que coinciden con la lista de LocalSend, conmutables en tiempo de ejecución o siguiendo la configuración regional del sistema
- Integración nativa con el escritorio: notificaciones solo desde la bandeja, inicio al iniciar sesión, minimizar en la bandeja, persistencia de la ubicación de la ventana y una ventana macOS que se dibuja en su barra de título.
- Embalaje para macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) y Windows (ZIP), tanto para x86_64 como para aarch64.

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

El demonio es el único proceso que se comunica con la red o escribe el estado de la aplicación. Las interfaces nunca dependen del núcleo suministrado y nunca generan el demonio por sí mismas; la bandeja lo hace. Los bytes de archivo no viajan a través de RPC: el demonio resuelve rutas, codifica el contenido y registra el historial.

| Binario | Role |
| --- | --- |
| `localsendd` | Estado del servicio de red y de la aplicación canónica |
| `localsend-tray` | Daemon y propietario del ciclo de vida GUI, notificaciones |
| `localsend-gui-slint` | El GUI, Slint formal con el renderizador FemtoVG |
| `localsend-tui` | Interfaz de terminal |
| `localsend-launcher` | Lanzador explícito para entornos sin bandeja |
| `localsend-androidd` | Demonio de recepción Android independiente solo para root (MVP) |

Los directorios `localsend-gui-egui`, `-fltk`, `-iced` y `-tauri` son puertos experimentales y permanecen fuera del espacio de trabajo raíz hasta que se complete su integración RPC.

## Descargar

Los activos de versión siguen un esquema de nomenclatura: `LocalSend-<version>-<platform>-<arch>.<ext>`, con plataformas `linux` / `windows` / `macos` y arquitecturas `x86_64` / `aarch64`:

| Plataforma | Activo |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Cualquier Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` cubre todos los activos de una versión. La compilación macOS producida por CI está firmada ad-hoc, no firmada ni certificada por un ID de desarrollador.

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

Los paquetes Linux instalan una entrada de escritorio que inicia `localsend-tray`, que posee el ciclo de vida del demonio. En los escritorios sin barra de estado, la bandeja mantiene abierto GUI; `LocalSend --launcher` selecciona ese modo explícitamente del AppImage.

### Windows

Descomprima y ejecute `LocalSend.exe`. El archivo contiene el GUI, el demonio, la bandeja y el lanzador uno al lado del otro; mantenerlos juntos.

## Comportamiento que conviene conocer

- **Salir es una sola operación.** Salir desde la bandeja finaliza el GUI registrado en el `gui.pid` con ámbito de punto final, detiene el demonio de propiedad dentro de un tiempo de espera limitado y sale sin dejar atrás un proceso o un trabajo de mantenimiento de actividad iniciado.
- **Las notificaciones provienen de la bandeja.** El GUI nunca publica sus propias notificaciones, por lo que cada plataforma tiene una única ruta de entrega.
- **El estado de la ventana es por punto final.** La ubicación se almacena junto a la configuración del demonio para ese punto final en lugar de en una ubicación global compartida, por lo que dos instancias no pelean por la geometría de la ventana.
- **El demonio tiene autoridad.** Las configuraciones cambiadas en GUI se envían al demonio y se repiten a través de una instantánea; una interfaz no guarda ninguna copia privada que pueda desviarse.

## Idiomas

La interfaz se entrega en 57 configuraciones regionales, que coinciden con la lista ascendente de LocalSend, con el chino simplificado como idioma msgstr. Elija uno en **Configuración → Idioma**, o déjelo en "Seguir sistema" para usar la configuración regional del sistema operativo.

| | |
| --- | --- |
| Catálogos incluidos | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| cadenas de origen de la interfaz de usuario | `@tr("...")` en `apps/localsend-gui-slint/ui` |
| catalogo en ingles | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust mensajes) |
| Mensajes del lado oxidado | `crates/localsend-i18n` (generado `lookup_generated.rs`) |
| espejo aguas arriba | `translations/upstream/<locale>.json` |

Regenere los catálogos después de cambiar cualquier cadena de UI:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` está comprometido, por lo que los catálogos generados se pueden reproducir sin conexión. Corrija una entrada allí y vuelva a ejecutar el generador para corregir una traducción.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Construyendo desde la fuente

Requisitos: Rust `1.97.1` (fijado por `rust-toolchain.toml`) y, para compilaciones Linux, los encabezados de desarrollo GTK3 y AppIndicator.

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

Los scripts de aceptación controlan los binarios reales en lugar del estado simulado:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Documentación

| Documento | Contenido |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Operación, empaquetado y comportamiento de la plataforma. |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Evidencia de prueba actual y áreas conocidas no verificadas |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Puertas de aceptación locales |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Regresiones recurrentes y el rastro de evidencia esperado antes de cambiar el comportamiento |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Estado de convergencia visual frente al prototipo OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | El módulo raíz Android y sus límites |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Procedencia ascendente del núcleo suministrado |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Algunos documentos están escritos en chino; registran ejecuciones de verificación y convenciones en las palabras en las que se acordaron.

## Alcance

El producto principal es el cliente de PC. El soporte Android es un demonio de recepción sin cabeza y exclusivo para root hasta que se promueva explícitamente más allá de ese alcance: no tiene GUI, no inicia ningún servicio en primer plano y no publica notificaciones persistentes. `crates/localsend-official-core` se mantiene byte por byte idéntico a su exportación ascendente registrada, aparte de su archivo `SHA256SUMS` generado.

## Créditos

El protocolo, el formato de cable y el núcleo Rust provienen de [localsend/localsend](https://github.com/localsend/localsend); este proyecto construye un cliente de escritorio sobre ese trabajo en lugar de volver a implementar el protocolo. Gracias a los LocalSend autores y contribuyentes.

La interfaz sigue un prototipo de diseño desarrollado para este proyecto; su referencia visual vive en [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Cómo contribuir

- **Errores y sugerencias**: abre un [problema](https://github.com/alkaid-ops/localsend-next/issues) con la versión, plataforma, pasos de reproducción y lo que viste. Un registro o una captura de pantalla hace que actuar sea mucho más rápido.
- **Código**: las solicitudes de extracción son bienvenidas. Los cambios que tocan el límite daemon/frontend, el núcleo suministrado o el contrato visual deben discutirse primero; las restricciones se registran en [`AGENTS.md`](../../AGENTS.md).
- **Traducciones**: los catálogos se generan a partir del caché de traducción automática comprometido, por lo que las correcciones son el tipo de contribución más fácil de revisar.

## Licencia

[Apache-2.0](../../LICENSE). Los componentes de terceros y sus licencias se enumeran en [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

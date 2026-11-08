<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | **Deutsch** | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next ist ein LocalSend-kompatibler Desktop-Client, der in Rust geschrieben ist. Es spricht das offizielle Protokoll und erkennt und überträgt Dateien mit der offiziellen LocalSend-App in beide Richtungen, während es als natives Desktop-Programm statt als Flutter-App ausgeführt wird.

Die Netzwerkseite ist der offizielle LocalSend Rust Kern, der unverändert verkauft wird. Alles darüber – Daemon, RPC Layer, Tray, Terminal-UI und Slint GUI – ist um diesen Kern herum aufgebaut.

## Demo

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Schicken | Einstellungen |
| --- | --- |
| ![Seite mit den Datei-, Ordner-, Text- und Zwischenablagequellen und Geräten in der Nähe senden](../../docs/images/readme-send-light.png) | ![Einstellungsseite mit Thema, Sprache, Netzwerk und Übertragungsoptionen](../../docs/images/readme-settings-light.png) |

## Funktionen

- Erkennung und Übertragung gegen offizielle LocalSend-Clients auf dem bereitgestellten Upstream-Kern statt einer Neuimplementierung
- Ein kanonischer Statuseigentümer: Der Daemon enthält Einstellungen, Identität, Favoriten, Verlauf, Übertragungen und die TLS-Identität, sodass jedes Frontend dasselbe sieht
- GUI, Terminal-UI und Tray als unabhängige Frontends über dasselbe lokale RPC; Das Tray besitzt einen Daemon und einen GUI Prozesslebenszyklus
- Fortschritt pro Datei in Echtzeit, Verlauf mit Öffnungs- und Offenlegungsaktionen, Favoriten, manuelle Geräteeingabe und Web Share-Empfangslinks
- 57 Schnittstellensprachen, die mit der Liste der Upstream-Benutzer von LocalSend übereinstimmen, zur Laufzeit umschaltbar oder je nach Systemgebietsschema
- Native Desktop-Integration: Benachrichtigungen nur aus der Taskleiste, Start beim Anmelden, Minimieren in die Taskleiste, Persistenz der Fensterplatzierung und ein macOS-Fenster, das in die Titelleiste eingezeichnet wird
- Verpackung für macOS (DMG/ZIP), Linux (deb, rpm, AppImage, Tarball) und Windows (ZIP), sowohl für x86_64 als auch für aarch64

## Architektur

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Der Daemon ist der einzige Prozess, der mit dem Netzwerk kommuniziert oder den Anwendungsstatus schreibt. Frontends sind niemals vom Kern des Anbieters abhängig und erzeugen niemals den Daemon selbst; Das Fach tut es. Dateibytes werden nicht über RPC übertragen – der Daemon löst Pfade auf, hasht Inhalte und zeichnet den Verlauf auf.

| Binär | Rolle |
| --- | --- |
| `localsendd` | Netzwerkdienst und kanonischer Anwendungsstatus |
| `localsend-tray` | Daemon und GUI Lebenszyklusbesitzer, Benachrichtigungen |
| `localsend-gui-slint` | Das formale GUI, Slint mit dem FemtoVG-Renderer |
| `localsend-tui` | Terminal-Frontend |
| `localsend-launcher` | Expliziter Launcher für Umgebungen ohne Fach |
| `localsend-androidd` | Eigenständiger Root-Only-Android-Empfangsdaemon (MVP) |

Die Verzeichnisse `localsend-gui-egui`, `-fltk`, `-iced` und `-tauri` sind experimentelle Ports und bleiben außerhalb des Root-Arbeitsbereichs, bis ihre RPC-Integration abgeschlossen ist.

## Herunterladen

Release-Assets folgen einem Namensschema – `LocalSend-<version>-<platform>-<arch>.<ext>`, mit Plattformen `linux` / `windows` / `macos` und Architekturen `x86_64` / `aarch64`:

| Plattform | Vermögenswert |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Irgendein Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` deckt jedes Asset in einer Version ab. Der von CI erstellte macOS-Build ist ad-hoc signiert, nicht mit der Entwickler-ID signiert oder notariell beglaubigt.

## Schnellstart

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

Linux-Pakete installieren einen Desktop-Eintrag, der `localsend-tray` startet, das den Daemon-Lebenszyklus besitzt. Auf Desktops ohne Statusleiste bleibt in der Taskleiste stattdessen das GUI geöffnet; `LocalSend --launcher` wählt diesen Modus explizit aus AppImage aus.

### Windows

Entpacken und `LocalSend.exe` ausführen. Das Archiv enthält das GUI, den Daemon, die Taskleiste und den Launcher nebeneinander; Halte sie zusammen.

## Wissenswertes

- **Beenden ist ein Vorgang.** Das Beenden aus der Taskleiste beendet die GUI, die im endpunktbezogenen `gui.pid` aufgezeichnet wurde, stoppt den eigenen Daemon innerhalb einer begrenzten Zeitüberschreitung und wird beendet, ohne einen Prozess oder einen gestarteten Keep-Alive-Job zurückzulassen.
- **Benachrichtigungen kommen aus der Taskleiste.** Das GUI veröffentlicht nie seine eigenen Benachrichtigungen, daher hat jede Plattform einen einzigen Zustellungspfad.
- **Der Fensterstatus gilt pro Endpunkt.** Die Platzierung wird neben der Daemon-Konfiguration für diesen Endpunkt und nicht an einem gemeinsam genutzten globalen Speicherort gespeichert, sodass sich zwei Instanzen nicht um die Fenstergeometrie streiten.
- **Der Daemon ist autorisierend.** In GUI geänderte Einstellungen werden an den Daemon übermittelt und über einen Snapshot zurückgesendet; Ein Frontend behält keine private Kopie, die driften könnte.

## Sprachen

Die Schnittstelle wird in 57 Gebietsschemata ausgeliefert, die der Liste der Upstream-Benutzer von LocalSend entsprechen, mit vereinfachtem Chinesisch als Msgid-Sprache. Wählen Sie unter **Einstellungen → Sprache** eine aus oder belassen Sie sie auf „System folgen“, um das Gebietsschema des Betriebssystems zu verwenden.

| | |
| --- | --- |
| Gebündelte Kataloge | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UI-Quellzeichenfolgen | `@tr("...")` in `apps/localsend-gui-slint/ui` |
| Englischer Katalog | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust Nachrichten) |
| Rust-seitige Nachrichten | `crates/localsend-i18n` (generiert `lookup_generated.rs`) |
| Vorgelagerter Spiegel | `translations/upstream/<locale>.json` |

Generieren Sie die Kataloge neu, nachdem Sie eine UI-Zeichenfolge geändert haben:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` ist festgeschrieben, sodass die generierten Kataloge offline reproduzierbar sind. Korrigieren Sie dort einen Eintrag und führen Sie den Generator erneut aus, um eine Übersetzung zu korrigieren.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Bauen aus der Quelle

Anforderungen: Rust `1.97.1` (gepinnt von `rust-toolchain.toml`) und für Linux Builds die Entwicklungsheader GTK3 und AppIndicator.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Plattformpakete:

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

Akzeptanzskripte steuern die echten Binärdateien anstelle des simulierten Status:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Dokumentation

| Dokumentieren | Inhalt |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Betrieb, Verpackung und Plattformverhalten |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Aktuelle Testergebnisse und bekannte, nicht überprüfte Bereiche |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Lokale Akzeptanztore |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Wiederkehrende Regressionen und die erwartete Beweisspur vor einer Verhaltensänderung |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Visueller Konvergenzstatus gegenüber dem OpenDesign-Prototyp |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Das Android Root-Modul und seine Grenzen |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Upstream-Herkunft des verkauften Kerns |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Einige Dokumente sind auf Chinesisch verfasst; Sie zeichnen Verifizierungsläufe und Konventionen in den vereinbarten Worten auf.

## Umfang

Das Hauptprodukt ist der PC-Client. Android support ist ein reiner Root-Empfangsdämon ohne Kopf, bis er explizit über diesen Bereich hinaus befördert wird – er hat kein GUI, startet keinen Vordergrunddienst und sendet keine dauerhafte Benachrichtigung. `crates/localsend-official-core` bleibt Byte für Byte identisch mit seinem aufgezeichneten Upstream-Export, abgesehen von der generierten `SHA256SUMS`-Datei.

## Danksagung

Das Protokoll, das Wire-Format und der Rust-Kern stammen von [localsend/localsend](https://github.com/localsend/localsend); Dieses Projekt baut auf dieser Arbeit einen Desktop-Client auf, anstatt das Protokoll neu zu implementieren. Vielen Dank an die LocalSend Autoren und Mitwirkenden.

Die Schnittstelle folgt einem Design-Prototyp, der für dieses Projekt entwickelt wurde; seine visuelle Referenz lebt in [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Mitwirken

- **Fehler und Vorschläge**: Öffnen Sie ein [Problem](https://github.com/alkaid-ops/localsend-next/issues) mit der Version, der Plattform, den Reproduktionsschritten und dem, was Sie gesehen haben. Durch ein Protokoll oder einen Screenshot können Sie viel schneller reagieren.
- **Code**: Pull-Requests sind willkommen. Änderungen, die die daemon/frontend-Grenze, den verkauften Kern oder den visuellen Vertrag berühren, sollten zuerst besprochen werden – die Einschränkungen werden in [`AGENTS.md`](../../AGENTS.md) aufgezeichnet.
- **Übersetzungen**: Die Kataloge werden aus dem dedizierten Cache für maschinelle Übersetzungen generiert, sodass Korrekturen dort die am einfachsten zu überprüfende Beitragsart darstellen.

## Lizenz

[Apache-2.0](../../LICENSE). Komponenten von Drittanbietern und deren Lizenzen sind in [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md) aufgeführt.

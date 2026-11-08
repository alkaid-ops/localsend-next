<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | **Italiano** | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next è un client desktop compatibile con LocalSend scritto in Rust. Parla il protocollo ufficiale, quindi rileva e trasferisce file con l'app LocalSend ufficiale in entrambe le direzioni, mentre viene eseguito come programma desktop nativo anziché come app Flutter.

Il lato rete è il core ufficiale LocalSend Rust, venduto invariato. Tutto ciò che sta sopra di esso (daemon, livello RPC, vassoio, interfaccia utente del terminale e Slint GUI) è costruito attorno a quel nucleo.

## Dimostrazione

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Inviare | Impostazioni |
| --- | --- |
| ![Invia pagina con origini file, cartelle, testo e appunti e dispositivi vicini](../../docs/images/readme-send-light.png) | ![Pagina delle impostazioni con tema, lingua, rete e opzioni di trasferimento](../../docs/images/readme-settings-light.png) |

## Funzionalità

- Scoperta e trasferimento contro client ufficiali LocalSend, sul core upstream fornito anziché su una reimplementazione
- Un proprietario di stato canonico: il demone contiene impostazioni, identità, preferiti, cronologia, trasferimenti e l'identità TLS, quindi ogni frontend vede la stessa cosa
- GUI, interfaccia utente terminale e vassoio come frontend indipendenti sullo stesso locale RPC; il vassoio possiede il demone e il ciclo di vita del processo GUI
- Avanzamento per file in tempo reale, cronologia con azioni di apertura e rivelazione, preferiti, immissione manuale del dispositivo e collegamenti di ricezione Web Share
- 57 lingue di interfaccia corrispondenti all'elenco originale di LocalSend, commutabili in fase di esecuzione o in base alle impostazioni locali del sistema
- Integrazione desktop nativa: notifiche solo dal vassoio, avvio all'accesso, riduzione a icona nel vassoio, persistenza del posizionamento della finestra e una finestra macOS che si inserisce nella barra del titolo
- Packaging per macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) e Windows (ZIP), sia per x86_64 che aarch64

## Architettura

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Il demone è l'unico processo che comunica con la rete o scrive lo stato dell'applicazione. I frontend non dipendono mai dal core fornito e non generano mai il demone stesso; il vassoio lo fa. I byte dei file non viaggiano su RPC: il demone risolve i percorsi, esegue l'hashing del contenuto e registra la cronologia.

| Binario | Ruolo |
| --- | --- |
| `localsendd` | Servizio di rete e stato dell'applicazione canonica |
| `localsend-tray` | Daemon e proprietario del ciclo di vita GUI, notifiche |
| `localsend-gui-slint` | Il formale GUI, Slint con il renderer FemtoVG |
| `localsend-tui` | Front-end del terminale |
| `localsend-launcher` | Launcher esplicito per ambienti senza vassoio |
| `localsend-androidd` | Demone di ricezione standalone solo root Android (MVP) |

Le directory `localsend-gui-egui`, `-fltk`, `-iced` e `-tauri` sono porte sperimentali e rimangono all'esterno dell'area di lavoro root fino al completamento dell'integrazione RPC.

## Scaricamento

Le risorse di rilascio seguono uno schema di denominazione: `LocalSend-<version>-<platform>-<arch>.<ext>`, con piattaforme `linux` / `windows` / `macos` e architetture `x86_64` / `aarch64`:

| Piattaforma | Risorsa |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Qualsiasi Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` copre ogni risorsa in una versione. La build macOS prodotta da CI è firmata ad hoc, non firmata o autenticata dall'ID sviluppatore.

## Avvio rapido

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

I pacchetti Linux installano una voce desktop che avvia `localsend-tray`, che possiede il ciclo di vita del demone. Sui desktop senza barra di stato, il vassoio mantiene invece aperto GUI; `LocalSend --launcher` seleziona esplicitamente quella modalità da AppImage.

### Windows

Decomprimi ed esegui `LocalSend.exe`. L'archivio contiene GUI, il demone, il vassoio e il launcher uno accanto all'altro; tenerli insieme.

## Comportamento che vale la pena conoscere

- **L'uscita è un'operazione.** L'uscita dal vassoio termina GUI registrato nell'ambito `gui.pid` con ambito endpoint, arresta il daemon di proprietà entro un timeout limitato ed esce senza lasciare dietro di sé un processo o un processo keep-alive di avvio.
- **Le notifiche provengono dal vassoio.** GUI non pubblica mai le proprie notifiche, quindi ogni piattaforma ha un unico percorso di consegna.
- **Lo stato della finestra è per endpoint.** Il posizionamento viene archiviato accanto alla configurazione del daemon per quell'endpoint anziché in una posizione globale condivisa, quindi due istanze non combattono per la geometria della finestra.
- **Il demone è autorevole.** Le impostazioni modificate in GUI vengono inviate al demone e riprodotte tramite uno snapshot; un frontend non conserva alcuna copia privata che potrebbe andare alla deriva.

## Lingue

L'interfaccia viene fornita in 57 lingue, corrispondenti all'elenco upstream di LocalSend, con il cinese semplificato come lingua msgid. Scegline uno in **Impostazioni → Lingua** o lascialo su "Segui il sistema" per utilizzare le impostazioni internazionali del sistema operativo.

| | |
| --- | --- |
| Cataloghi in bundle | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| Stringhe di origine dell'interfaccia utente | `@tr("...")` in `apps/localsend-gui-slint/ui` |
| Catalogo inglese | `translations/locale-en.json` (interfaccia utente), `translations/locale-en-runtime.json` (Rust messaggi) |
| Messaggi dal lato ruggine | `crates/localsend-i18n` (generato `lookup_generated.rs`) |
| Specchio a monte | `translations/upstream/<locale>.json` |

Rigenera i cataloghi dopo aver modificato qualsiasi stringa dell'interfaccia utente:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` è impegnato, quindi i cataloghi generati sono riproducibili offline. Correggi una voce lì ed esegui nuovamente il generatore per correggere una traduzione.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Costruire dalla fonte

Requisiti: Rust `1.97.1` (appuntato da `rust-toolchain.toml`) e, per le build Linux, le intestazioni di sviluppo GTK3 e AppIndicator.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Pacchetti piattaforma:

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

Gli script di accettazione guidano i binari reali invece dello stato deriso:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Documentazione

| Documento | Contenuto |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Funzionamento, packaging e comportamento della piattaforma |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Prove di test attuali e aree note non verificate |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Cancelli di accettazione locali |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Regressioni ricorrenti e tracce di prove attese prima di cambiare comportamento |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Stato di convergenza visiva rispetto al prototipo OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Il modulo root Android e i suoi limiti |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Provenienza a monte del nucleo venduto |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Alcuni documenti sono scritti in cinese; registrano le verifiche e le convenzioni nelle parole con cui sono state concordate.

## Ambito

Il prodotto principale è il client PC. Il supporto Android è un demone di ricezione headless, solo root, finché non viene promosso esplicitamente oltre tale ambito: non ha GUI, non avvia alcun servizio in primo piano e non pubblica notifiche persistenti. `crates/localsend-official-core` viene mantenuto identico byte per byte all'esportazione upstream registrata, a parte il file `SHA256SUMS` generato.

## Ringraziamenti

Il protocollo, il formato wire e il core Rust provengono da [localsend/localsend](https://github.com/localsend/localsend); questo progetto crea un client desktop su quel lavoro anziché reimplementare il protocollo. Grazie agli autori e ai contributori LocalSend.

L'interfaccia segue un prototipo di design sviluppato per questo progetto; il suo riferimento visivo risiede in [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Contribuire

- **Bug e suggerimenti**: apri un [problema](https://github.com/alkaid-ops/localsend-next/issues) con la versione, la piattaforma, i passaggi di riproduzione e ciò che hai visto. Un registro o uno screenshot rendono molto più veloce l'azione.
- **Codice**: le richieste di pull sono benvenute. Le modifiche che toccano il confine daemon/frontend, il nucleo del fornitore o il contratto visivo dovrebbero essere discusse per prime: i vincoli sono registrati in [`AGENTS.md`](../../AGENTS.md).
- **Traduzioni**: i cataloghi vengono generati dalla cache di traduzione automatica impegnata, quindi le correzioni rappresentano il tipo di contributo più semplice da rivedere.

## Licenza

[Apache-2.0](../../LICENSE). I componenti di terze parti e le relative licenze sono elencati in [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

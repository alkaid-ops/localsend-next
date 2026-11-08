<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | **Română** | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next este un client desktop compatibil cu LocalSend, scris în Rust. Vorbește protocolul oficial, așa că descoperă și transferă fișiere cu aplicația oficială LocalSend în ambele direcții, în timp ce rulează ca un program desktop nativ în loc de o aplicație Flutter.

Partea de rețea este nucleul oficial LocalSend Rust, vândut neschimbat. Tot ce este deasupra lui - demon, RPC layer, tavă, terminal UI și Slint GUI - este construit în jurul acestui nucleu.

## Demo

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Trimite | Setări |
| --- | --- |
| ![Trimite pagina cu fișierul, folderul, sursele de text și clipboard și dispozitivele din apropiere](../../docs/images/readme-send-light.png) | ![Pagina de setări cu opțiuni de temă, limbă, rețea și transfer](../../docs/images/readme-settings-light.png) |

## Funcționalități

- Descoperire și transfer împotriva clienților oficiali LocalSend, pe nucleul din amonte vândut, mai degrabă decât o reimplementare
- Un proprietar de stat canonic: demonul deține setări, identitate, favorite, istoric, transferuri și identitatea TLS, astfel încât fiecare frontend vede același lucru
- GUI, terminal UI și tray ca interfață independentă pe același RPC local; tava deține daemon și ciclul de viață al procesului GUI
- Progres în timp real per fișier, istoric cu acțiuni de deschidere și dezvăluire, favorite, introducere manuală a dispozitivului și Web Share primi link-uri
- 57 de limbi de interfață care se potrivesc în amonte listei lui LocalSend, care pot fi comutate la rulare sau urmând localitatea sistemului
- Integrare desktop nativ: notificări numai din tavă, lansare la conectare, minimizare în tavă, persistență de plasare a ferestrelor și o fereastră macOS care se atrage în bara de titlu
- Ambalare pentru macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) și Windows (ZIP), atât pentru x86_64, cât și pentru aarch64

## Arhitectură

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Daemonul este singurul proces care vorbește cu rețeaua sau scrie starea aplicației. Frontend-urile nu depind niciodată de nucleul vândut și nu generează niciodată demonul în sine; tava face. Octeții de fișier nu se deplasează peste RPC — demonul rezolvă căile, hashează conținutul și înregistrează istoricul.

| Binar | Rol |
| --- | --- |
| `localsendd` | Serviciul de rețea și starea aplicației canonice |
| `localsend-tray` | Daemon și GUI proprietarul ciclului de viață, notificări |
| `localsend-gui-slint` | Formal GUI, Slint cu rendererul FemtoVG |
| `localsend-tui` | Frontend terminal |
| `localsend-launcher` | Lansator explicit pentru medii fără tavă |
| `localsend-androidd` | Daemon de primire (MVP) autonom Android |

Directoarele `localsend-gui-egui`, `-fltk`, `-iced` și `-tauri` sunt porturi experimentale și rămân în afara spațiului de lucru rădăcină până când integrarea lor RPC este completă.

## Descărcați

Elementele de lansare urmează o singură schemă de denumire — `LocalSend-<version>-<platform>-<arch>.<ext>`, cu platforme `linux` / `windows` / `macos` și arhitecturi `x86_64` / `aarch64`:

| Platformă | Atu |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Orice Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` acoperă fiecare material dintr-o versiune. Compilarea macOS produsă de CI este semnată ad-hoc, nu ID-ul dezvoltatorului semnat sau legalizat.

## Pornire rapidă

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

Pachetele Linux instalează o intrare pe desktop care lansează `localsend-tray`, care deține ciclul de viață al demonului. Pe desktop-urile fără bară de stare, tava menține GUI deschisă; `LocalSend --launcher` selectează acel mod explicit din AppImage.

### Windows

Dezarhivați și rulați `LocalSend.exe`. Arhiva deține GUI, demonul, tava și lansatorul unul lângă celălalt; tine-le impreuna.

## Comportament care merită cunoscut

- **Ieșire este o singură operație.** Ieșirea din tavă termină GUI înregistrată în domeniul de aplicare `gui.pid`, oprește demonul deținut într-un interval de timp limitat și iese fără a lăsa în urmă un proces sau o sarcină de menținere în viață lansată.
- **Notificările vin din tavă.** GUI nu postează niciodată propriile notificări, astfel încât fiecare platformă are o singură cale de livrare.
- **Starea ferestrei este pentru fiecare punct final.** Plasarea este stocată lângă configurația demonului pentru acel punct final, mai degrabă decât într-o locație globală partajată, astfel încât două instanțe nu se luptă pentru geometria ferestrei.
- **Daemonul este autoritar.** Setările modificate în GUI sunt trimise demonului și ecou înapoi printr-un instantaneu; un frontend nu păstrează nicio copie privată care ar putea deriva.

## Limbi

Interfața este livrată în 57 de localități, care se potrivesc cu lista din amonte LocalSend, cu chineza simplificată ca limbă msgid. Alegeți unul din **Setări → Limbă** sau lăsați-l pe „Urmărește sistemul” pentru a utiliza localitatea sistemului de operare.

| | |
| --- | --- |
| Cataloage la pachet | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| șiruri sursă UI | `@tr("...")` în `apps/localsend-gui-slint/ui` |
| catalog englezesc | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust mesaje) |
| Mesaje din partea ruginii | `crates/localsend-i18n` (generat `lookup_generated.rs`) |
| Oglinda din amonte | `translations/upstream/<locale>.json` |

Regenerați cataloagele după modificarea oricărui șir de interfață de utilizare:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` este comis, astfel încât cataloagele generate sunt reproductibile offline. Corectați o intrare de acolo și rulați din nou generatorul pentru a remedia o traducere.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Construire de la sursă

Cerințe: Rust `1.97.1` (fixat de `rust-toolchain.toml`) și, pentru versiunile Linux, anteturile de dezvoltare GTK3 și AppIndicator.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Pachete platforme:

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

Scripturile de acceptare conduc binarele reale în loc de starea batjocorită:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Documentare

| Document | Cuprins |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Funcționarea, ambalarea și comportamentul platformei |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Dovezi de testare curente și zone cunoscute neverificate |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Porți locale de acceptare |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Regresiile recurente și traseul dovezilor așteptate înainte de schimbarea comportamentului |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Starea de convergență vizuală față de prototipul OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Modulul rădăcină Android și limitele acestuia |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Proveniența în amonte a nucleului vândut |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Unele documente sunt scrise în limba chineză; înregistrează cursurile de verificare și convențiile în cuvintele în care au fost convenite.

## Domeniu

Produsul principal este clientul PC. Suportul Android este un daemon de primire fără cap doar pentru rădăcină până când este promovat în mod explicit dincolo de acest domeniu - nu are GUI, nu pornește niciun serviciu în prim-plan și nu postează nicio notificare persistentă. `crates/localsend-official-core` este păstrat octet pentru octet identic cu exportul său înregistrat în amonte, în afară de fișierul `SHA256SUMS` generat.

## Credite

Protocolul, formatul firului și nucleul Rust provin de la [localsend/localsend](https://github.com/localsend/localsend); acest proiect construiește un client desktop pe acea lucrare, mai degrabă decât să reimplementeze protocolul. Mulțumim autorilor și colaboratorilor LocalSend.

Interfața urmează un prototip de design dezvoltat pentru acest proiect; referința sa vizuală locuiește în [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Contribuind

- **Erori și sugestii**: deschideți o [problemă](https://github.com/alkaid-ops/localsend-next/issues) cu versiunea, platforma, pașii de reproducere și ceea ce ați văzut. Un jurnal sau o captură de ecran face mult mai rapid să acționezi.
- **Cod**: solicitările de tragere sunt binevenite. Schimbările care ating limita daemon/frontend, nucleul vândut sau contractul vizual ar trebui discutate mai întâi — constrângerile sunt înregistrate în [`AGENTS.md`](../../AGENTS.md).
- **Traduceri**: cataloagele sunt generate din memoria cache de traducere automată angajată, astfel încât corecțiile sunt cel mai ușor tip de contribuție de revizuit.

## Licenţă

[Apache-2.0](../../LICENSE). Componentele terților și licențele acestora sunt enumerate în [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

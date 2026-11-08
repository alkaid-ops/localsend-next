<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | **ខ្មែរ** | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [简体中文](../../docs/zh-CN/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next គឺជាម៉ាស៊ីនភ្ញៀវផ្ទៃតុដែលឆបគ្នានឹង LocalSend ដែលសរសេរក្នុង Rust ។ វានិយាយពិធីសារផ្លូវការ ដូច្នេះវាស្វែងរក និងផ្ទេរឯកសារជាមួយកម្មវិធី LocalSend ផ្លូវការក្នុងទិសដៅទាំងពីរ ខណៈពេលដែលដំណើរការជាកម្មវិធីកុំព្យូទ័រដើមជំនួសឱ្យកម្មវិធី Flutter ។

ផ្នែកបណ្តាញគឺជាស្នូល LocalSend Rust ផ្លូវការ អ្នកលក់មិនផ្លាស់ប្តូរ។ អ្វីគ្រប់យ៉ាងនៅពីលើវា — ដេមិន, RPC ស្រទាប់, ថាស, terminal UI និង Slint GUI — ត្រូវបានបង្កើតឡើងជុំវិញស្នូលនោះ។

## ការបង្ហាញ

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| ផ្ញើ | ការកំណត់ |
| --- | --- |
| ![ផ្ញើទំព័រជាមួយឯកសារ ថតឯកសារ អត្ថបទ និងប្រភពក្ដារតម្បៀតខ្ទាស់ និងឧបករណ៍នៅក្បែរ](../../docs/images/readme-send-light.png) | ![ទំព័រការកំណត់ជាមួយប្រធានបទ ភាសា បណ្តាញ និងជម្រើសផ្ទេរ](../../docs/images/readme-settings-light.png) |

## លក្ខណៈពិសេស

- ការរកឃើញ និងការផ្ទេរប្រាក់ប្រឆាំងនឹងអតិថិជន LocalSend ផ្លូវការ នៅលើបណ្តាញស្នូលដែលបានលក់ជាជាងការអនុវត្តន៍ឡើងវិញ
- ម្ចាស់រដ្ឋ Canonical មួយ៖ ដេមិនកាន់ការកំណត់ អត្តសញ្ញាណ ចំណូលចិត្ត ប្រវត្តិ ការផ្ទេរ និងអត្តសញ្ញាណ TLS ដូច្នេះគ្រប់ផ្នែកខាងមុខមើលឃើញរឿងដូចគ្នា
- GUI, terminal UI និង tray ជាផ្នែកខាងមុខឯករាជ្យលើមូលដ្ឋានដូចគ្នា RPC; ថាសមានដេមិន និង GUI ដំណើរការវដ្តជីវិត
- ដំណើរការតាមពេលវេលាជាក់ស្តែងក្នុងមួយឯកសារ ប្រវត្តិជាមួយនឹងការបើក និងបង្ហាញសកម្មភាព ចំណូលចិត្ត ធាតុចូលឧបករណ៍ដោយដៃ និងការចែករំលែកបណ្តាញទទួលតំណ
- 57 ភាសាចំណុចប្រទាក់ដែលត្រូវគ្នានឹងបញ្ជីរបស់ LocalSend អាចប្តូរបាននៅពេលដំណើរការ ឬតាមមូលដ្ឋានប្រព័ន្ធ
- ការរួមបញ្ចូលផ្ទៃតុដើម៖ ការជូនដំណឹងពីថាសតែប៉ុណ្ណោះ បើកដំណើរការនៅពេលចូល បង្រួមអប្បបរមាទៅថាស ភាពស្ថិតស្ថេរនៃការដាក់បង្អួច និងបង្អួច macOS ដែលអូសចូលទៅក្នុងរបារចំណងជើងរបស់វា
- ការវេចខ្ចប់សម្រាប់ macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) និង Windows (ZIP) សម្រាប់ទាំង x86_64 និង aarch64

## ស្ថាបត្យកម្ម

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

ដេមិនគឺជាដំណើរការតែមួយគត់ដែលនិយាយទៅកាន់បណ្តាញ ឬសរសេរស្ថានភាពកម្មវិធី។ Frontends មិនដែលពឹងផ្អែកលើស្នូលដែលបានលក់ ហើយមិនដែលបង្កើតដេមិនដោយខ្លួនឯងទេ។ ថាសធ្វើ។ បៃឯកសារមិនធ្វើដំណើរលើស RPC — ដេមិនដោះស្រាយផ្លូវ មាតិកា សញ្ញា និងប្រវត្តិកត់ត្រា។

| គោលពីរ | តួនាទី |
| --- | --- |
| `localsendd` | សេវាបណ្តាញ និងស្ថានភាពកម្មវិធី Canonical |
| `localsend-tray` | Daemon និង GUI ម្ចាស់វដ្តជីវិត ការជូនដំណឹង |
| `localsend-gui-slint` | ផ្លូវការ GUI, Slint ជាមួយអ្នកបង្ហាញ FemtoVG |
| `localsend-tui` | ផ្នែកខាងមុខស្ថានីយ |
| `localsend-launcher` | កម្មវិធីបើកដំណើរការច្បាស់លាស់សម្រាប់បរិស្ថានដោយគ្មានថាស |
| `localsend-androidd` | ជា root-only Android ទទួលដេមិន (MVP) |

ថត `localsend-gui-egui`, `-fltk`, `-iced` និង `-tauri` ថតគឺជាច្រកពិសោធន៍ ហើយនៅក្រៅកន្លែងធ្វើការជា root រហូតដល់ការរួមបញ្ចូល RPC របស់ពួកគេត្រូវបានបញ្ចប់។

## ទាញយក

បញ្ចេញទ្រព្យសកម្មតាមគ្រោងការណ៍ដាក់ឈ្មោះមួយ — `LocalSend-<version>-<platform>-<arch>.<ext>` ជាមួយនឹងវេទិកា `linux` / `windows` / `macos` និងស្ថាបត្យកម្ម `x86_64` / `aarch64`:

| វេទិកា | ទ្រព្យសកម្ម |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| ណាមួយ Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` គ្របដណ្តប់រាល់ទ្រព្យសម្បត្តិនៅក្នុងការចេញផ្សាយ។ ការស្ថាបនា macOS ផលិតដោយ CI ត្រូវបានចុះហត្ថលេខាដោយ ad-hoc មិនមែនលេខសម្គាល់អ្នកអភិវឌ្ឍន៍ដែលបានចុះហត្ថលេខា ឬ notarised ទេ។

## ការចាប់ផ្តើមរហ័ស

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

កញ្ចប់ Linux ដំឡើងធាតុផ្ទៃតុដែលបើកដំណើរការ `localsend-tray` ដែលគ្រប់គ្រងវដ្តជីវិតដេមិន។ នៅលើផ្ទៃតុដោយគ្មានរបារស្ថានភាព ថាសរក្សា GUI បើកជំនួសវិញ។ `LocalSend --launcher` ជ្រើសរើសរបៀបនោះយ៉ាងច្បាស់ពី AppImage ។

### Windows

ពន្លាហើយដំណើរការ `LocalSend.exe` ។ បណ្ណសារផ្ទុក GUI ដេមិន ថាស និងកម្មវិធីបើកដំណើរការនៅសងខាង។ រក្សាពួកវាជាមួយគ្នា។

## អាកប្បកិរិយាគួរដឹង

- **ការចាកចេញគឺជាប្រតិបត្តិការមួយ។** ការឈប់ចេញពីថាសបញ្ចប់ GUI ដែលបានកត់ត្រានៅក្នុង endpoint-scoped `gui.pid` បញ្ឈប់ដេមិនដែលមានកម្មសិទ្ធិក្នុងរយៈពេលកំណត់មួយ ហើយចេញដោយមិនបន្សល់ទុកនូវដំណើរការ ឬការងាររក្សាជីវិតដែលបានចាប់ផ្តើមនៅពីក្រោយ។
- **ការជូនដំណឹងបានមកពីថាស។** GUI មិនដែលបង្ហោះការជូនដំណឹងផ្ទាល់ខ្លួនទេ ដូច្នេះវេទិកានីមួយៗមានផ្លូវចែកចាយតែមួយ។
- **ស្ថានភាព Window គឺក្នុងមួយចំណុចបញ្ចប់។** ការដាក់ត្រូវបានរក្សាទុកនៅក្បែរការកំណត់រចនាសម្ព័ន្ធដេមិនសម្រាប់ចំណុចបញ្ចប់នោះ ជាជាងនៅក្នុងទីតាំងសកលដែលបានចែករំលែក ដូច្នេះឧទាហរណ៍ពីរមិនប្រឆាំងនឹងធរណីមាត្របង្អួចទេ។
- **ដេមិនគឺអនុញ្ញាត។** ការកំណត់ដែលបានផ្លាស់ប្តូរនៅក្នុង GUI ត្រូវបានដាក់ជូនទៅដេមិន ហើយបានបន្ទរត្រឡប់មកវិញតាមរយៈរូបថតមួយសន្លឹក។ ផ្នែកខាងមុខមិនរក្សាទុកច្បាប់ចម្លងឯកជនដែលអាចរសាត់បានឡើយ។

## ភាសា

ចំណុចប្រទាក់ដឹកជញ្ជូនក្នុង 57 មូលដ្ឋានដែលត្រូវគ្នានឹងបញ្ជីឈ្មោះ LocalSend ដោយមានភាសាចិនសាមញ្ញជាភាសា msgid ។ ជ្រើសរើសមួយនៅក្រោម **ការកំណត់ → ភាសា** ឬទុកវានៅលើ "Follow system" ដើម្បីប្រើ OS locale។

| | |
| --- | --- |
| កាតាឡុកជាកញ្ចប់ | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| ខ្សែអក្សរប្រភព UI | `@tr("...")` ក្នុង `apps/localsend-gui-slint/ui` |
| កាតាឡុកភាសាអង់គ្លេស | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust សារ) |
| សារដែលច្រេះ | `crates/localsend-i18n` (បង្កើត `lookup_generated.rs`) |
| កញ្ចក់ខាងលើ | `translations/upstream/<locale>.json` |

បង្កើតកាតាឡុកឡើងវិញបន្ទាប់ពីផ្លាស់ប្តូរខ្សែអក្សរ UI ណាមួយ៖

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` ត្រូវបានប្តេជ្ញាចិត្ត ដូច្នេះកាតាឡុកដែលបានបង្កើតគឺអាចផលិតឡើងវិញដោយគ្មានអ៊ីនធឺណិត។ កែធាតុនៅទីនោះ ហើយដំណើរការម៉ាស៊ីនភ្លើងឡើងវិញ ដើម្បីជួសជុលការបកប្រែ។

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## ការសាងសង់ពីប្រភព

តម្រូវការ៖ Rust `1.97.1` (ខ្ទាស់ដោយ `rust-toolchain.toml`) និងសម្រាប់ Linux ស្ថាបនា ក្បាលក្បាលអភិវឌ្ឍន៍ GTK3 និង AppIndicator ។

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

កញ្ចប់វេទិកា៖

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

ស្គ្រីប​ទទួល​បាន​ជំរុញ​គោល​ពីរ​ពិត​ជា​ជំនួស​ឱ្យ​ស្ថានភាព​ចំអក៖

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## ឯកសារ

| ឯកសារ | មាតិកា |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | ប្រតិបត្តិការ ការវេចខ្ចប់ និងឥរិយាបថវេទិកា |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | ភស្តុតាងសាកល្បងបច្ចុប្បន្ន និងតំបន់ដែលមិនបានបញ្ជាក់ |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | ច្រកទទួលក្នុងស្រុក |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | ការតំរែតំរង់ឡើងវិញ និងភស្តុតាងដែលរំពឹងទុកមុនពេលផ្លាស់ប្តូរអាកប្បកិរិយា |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | ស្ថានភាពនៃការបញ្ចូលគ្នាដែលមើលឃើញប្រឆាំងនឹងគំរូដើម OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | ម៉ូឌុលឫស Android និងដែនកំណត់របស់វា។ |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | ការបញ្ជាក់ខាងលើនៃស្នូលអ្នកលក់ |

README នេះ​ត្រូវ​បាន​បោះពុម្ព​ជា​ភាសា​ចំណុច​ប្រទាក់​ទាំង 57 — ជ្រើស​រើស​មួយ​ពី​ឧបករណ៍​ប្ដូរ​នៅ​ខាង​លើ។ ការបកប្រែត្រូវបានបង្កើតចេញពី `README.md` ដោយ `scripts/translate-readme.mjs`; ការកែតម្រូវចូល [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json)។

ឯកសារមួយចំនួនត្រូវបានសរសេរជាភាសាចិន; ពួកគេកត់ត្រាដំណើរការផ្ទៀងផ្ទាត់ និងអនុសញ្ញានៅក្នុងពាក្យដែលពួកគេត្រូវបានយល់ព្រម។

## វិសាលភាព

ផលិតផលចម្បងគឺម៉ាស៊ីនភ្ញៀវកុំព្យូទ័រ។ Android ការគាំទ្រគឺសម្រាប់តែ root ប៉ុណ្ណោះ ដេមិនទទួល headless រហូតដល់ត្រូវបានផ្សព្វផ្សាយយ៉ាងច្បាស់លាស់លើសពីវិសាលភាពនោះ — វាមិនមាន GUI ចាប់ផ្តើមគ្មានសេវាកម្មនៅខាងមុខ ហើយប្រកាសមិនមានការជូនដំណឹងបន្ត។ `crates/localsend-official-core` ត្រូវបានរក្សាទុកជា byte-for-byte ដូចគ្នាទៅនឹងការនាំចេញខាងលើដែលបានកត់ត្រាទុក ក្រៅពីឯកសារ `SHA256SUMS` ដែលបានបង្កើតរបស់វា។

## ឥណទាន

ពិធីការ ទម្រង់ខ្សែ និងស្នូល Rust មកពី [localsend/localsend](https://github.com/localsend/localsend); គម្រោងនេះបង្កើតកម្មវិធីកុំព្យូទ័រលើតុលើការងារនោះ ជាជាងអនុវត្តពិធីការឡើងវិញ។ សូមអរគុណដល់ LocalSend អ្នកនិពន្ធ និងអ្នករួមចំណែក។

ចំណុចប្រទាក់ធ្វើតាមគំរូរចនាដែលត្រូវបានបង្កើតឡើងសម្រាប់គម្រោងនេះ; ឯកសារយោងដែលមើលឃើញរបស់វារស់នៅក្នុង [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/)។

## ការរួមចំណែក

- **កំហុស និងការផ្ដល់យោបល់**៖ បើក [បញ្ហា](https://github.com/alkaid-ops/localsend-next/issues) ជាមួយនឹងកំណែ វេទិកា ជំហានបន្តពូជ និងអ្វីដែលអ្នកបានឃើញ។ កំណត់ហេតុ ឬរូបថតអេក្រង់ធ្វើឱ្យដំណើរការលឿនជាងមុន។
- ** លេខកូដ **៖ សំណើរទាញត្រូវបានស្វាគមន៍។ ការផ្លាស់ប្តូរដែលប៉ះព្រំដែន daemon/frontend ស្នូលអ្នកលក់ ឬកិច្ចសន្យាដែលមើលឃើញគួរតែត្រូវបានពិភាក្សាជាមុនសិន — ឧបសគ្គត្រូវបានកត់ត្រានៅក្នុង [`AGENTS.md`](../../AGENTS.md) ។
- **ការបកប្រែ**៖ កាតាឡុកត្រូវបានបង្កើតចេញពីឃ្លាំងសម្ងាត់ការបកប្រែដោយម៉ាស៊ីន ដែលបានប្តេជ្ញាចិត្ត ដូច្នេះការកែតម្រូវគឺជាការរួមចំណែកដ៏ងាយស្រួលបំផុតក្នុងការត្រួតពិនិត្យ។

## អាជ្ញាប័ណ្ណ

[Apache-2.0](../../LICENSE) ។ សមាសធាតុភាគីទីបី និងអាជ្ញាប័ណ្ណរបស់ពួកគេត្រូវបានរាយក្នុង [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md)។

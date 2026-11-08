<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | **മലയാളം** | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [简体中文](../../docs/zh-CN/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next എന്നത് Rust എന്നതിൽ എഴുതിയ ലോക്കൽസെൻഡ്-അനുയോജ്യമായ ഡെസ്ക്ടോപ്പ് ക്ലയൻ്റാണ്. ഇത് ഔദ്യോഗിക പ്രോട്ടോക്കോൾ സംസാരിക്കുന്നു, അതിനാൽ ഫ്ലട്ടർ ആപ്പിന് പകരം ഒരു നേറ്റീവ് ഡെസ്‌ക്‌ടോപ്പ് പ്രോഗ്രാമായി പ്രവർത്തിക്കുമ്പോൾ, ഇത് രണ്ട് ദിശകളിലേക്കും ഔദ്യോഗിക LocalSend ആപ്പ് ഉപയോഗിച്ച് ഫയലുകൾ കണ്ടെത്തുകയും കൈമാറുകയും ചെയ്യുന്നു.

നെറ്റ്‌വർക്ക് സൈഡ് ഔദ്യോഗിക LocalSend Rust കോർ ആണ്, വെണ്ടർ മാറ്റില്ല. അതിന് മുകളിലുള്ള എല്ലാം - ഡെമൺ, RPC ലെയർ, ട്രേ, ടെർമിനൽ യുഐ, Slint GUI - എന്നിവ ആ കാമ്പിനെ ചുറ്റിപ്പറ്റിയാണ് നിർമ്മിച്ചിരിക്കുന്നത്.

## ഡെമോ

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| അയക്കുക | ക്രമീകരണങ്ങൾ |
| --- | --- |
| ![ഫയൽ, ഫോൾഡർ, ടെക്‌സ്‌റ്റ്, ക്ലിപ്പ്ബോർഡ് ഉറവിടങ്ങളും സമീപത്തുള്ള ഉപകരണങ്ങളും സഹിതം പേജ് അയയ്‌ക്കുക](../../docs/images/readme-send-light.png) | ![തീം, ഭാഷ, നെറ്റ്‌വർക്ക്, ട്രാൻസ്ഫർ ഓപ്ഷനുകൾ എന്നിവയുള്ള ക്രമീകരണ പേജ്](../../docs/images/readme-settings-light.png) |

## സവിശേഷതകൾ

- വീണ്ടും നടപ്പിലാക്കുന്നതിനുപകരം, വെണ്ടർ ചെയ്ത അപ്‌സ്ട്രീം കോറിൽ, ഔദ്യോഗിക LocalSend ക്ലയൻ്റുകൾക്കെതിരെ കണ്ടെത്തലും കൈമാറ്റവും
- ഒരു കാനോനിക്കൽ സ്റ്റേറ്റ് ഉടമ: ഡെമൺ ക്രമീകരണങ്ങൾ, ഐഡൻ്റിറ്റി, പ്രിയങ്കരങ്ങൾ, ചരിത്രം, കൈമാറ്റങ്ങൾ, TLS ഐഡൻ്റിറ്റി എന്നിവ കൈവശം വയ്ക്കുന്നു, അതിനാൽ എല്ലാ മുൻഭാഗങ്ങളും ഒരേ കാര്യം കാണുന്നു
- GUI, ടെർമിനൽ യുഐയും ട്രേയും ഒരേ ലോക്കൽ RPCക്ക് മുകളിൽ സ്വതന്ത്ര ഫ്രണ്ട്എൻഡുകളായി; ട്രേയ്ക്ക് ഡെമണും GUI പ്രോസസ് ലൈഫ് സൈക്കിളും ഉണ്ട്
- ഓരോ ഫയലിനും തത്സമയ പുരോഗതി, തുറന്നതും വെളിപ്പെടുത്തുന്നതുമായ പ്രവർത്തനങ്ങളുള്ള ചരിത്രം, പ്രിയങ്കരങ്ങൾ, സ്വമേധയാലുള്ള ഉപകരണ എൻട്രി, വെബ് ഷെയർ സ്വീകരിക്കുന്ന ലിങ്കുകൾ
- 57 ഇൻ്റർഫേസ് ഭാഷകൾ അപ്‌സ്ട്രീം LocalSend യുടെ ലിസ്‌റ്റുമായി പൊരുത്തപ്പെടുന്നു, റൺടൈമിൽ മാറാവുന്നതോ സിസ്റ്റം ലോക്കൽ പിന്തുടരുന്നതോ
- നേറ്റീവ് ഡെസ്‌ക്‌ടോപ്പ് സംയോജനം: ട്രേയിൽ നിന്നുള്ള അറിയിപ്പുകൾ മാത്രം, ലോഗിൻ ചെയ്യുമ്പോൾ ലോഞ്ച് ചെയ്യുക, ട്രേയിലേക്ക് ചെറുതാക്കുക, വിൻഡോ പ്ലേസ്‌മെൻ്റ് പെർസിസ്റ്റൻസ്, കൂടാതെ അതിൻ്റെ ടൈറ്റിൽ ബാറിലേക്ക് ആകർഷിക്കുന്ന macOS വിൻഡോ
- macOS (DMG/ZIP), Linux (deb, rpm, AppImage, ടാർബോൾ), Windows (ZIP), x86_64, aarch64 എന്നിവയ്‌ക്കായുള്ള പാക്കേജിംഗ്

## വാസ്തുവിദ്യ

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

നെറ്റ്‌വർക്കുമായി സംസാരിക്കുന്ന അല്ലെങ്കിൽ ആപ്ലിക്കേഷൻ നില എഴുതുന്ന ഒരേയൊരു പ്രക്രിയ ഡെമൺ ആണ്. ഫ്രണ്ടെൻഡുകൾ ഒരിക്കലും വെണ്ടർ ചെയ്ത കാമ്പിനെ ആശ്രയിക്കുന്നില്ല, മാത്രമല്ല ഡെമണിനെ തന്നെ മുളപ്പിക്കുകയും ചെയ്യുന്നില്ല; ട്രേ ചെയ്യുന്നു. ഫയൽ ബൈറ്റുകൾ RPC എന്നതിലൂടെ സഞ്ചരിക്കില്ല — ഡെമൺ പാതകളും ഹാഷുകളും ഉള്ളടക്കവും റെക്കോർഡ് ചരിത്രവും പരിഹരിക്കുന്നു.

| ബൈനറി | പങ്ക് |
| --- | --- |
| `localsendd` | നെറ്റ്‌വർക്ക് സേവനവും കാനോനിക്കൽ ആപ്ലിക്കേഷൻ നിലയും |
| `localsend-tray` | ഡെമൺ, GUI ലൈഫ് സൈക്കിൾ ഉടമ, അറിയിപ്പുകൾ |
| `localsend-gui-slint` | FemtoVG റെൻഡറർക്കൊപ്പം ഔപചാരികമായ GUI, Slint |
| `localsend-tui` | ടെർമിനൽ മുൻഭാഗം |
| `localsend-launcher` | ട്രേ ഇല്ലാത്ത പരിതസ്ഥിതികൾക്കുള്ള വ്യക്തമായ ലോഞ്ചർ |
| `localsend-androidd` | ഒറ്റമൂലി-മാത്രം Android ഡെമൺ സ്വീകരിക്കുക (MVP) |

`localsend-gui-egui`, `-fltk`, `-iced`, `-tauri` ഡയറക്‌ടറികൾ പരീക്ഷണാത്മക പോർട്ടുകളാണ് കൂടാതെ അവയുടെ RPC സംയോജനം പൂർത്തിയാകുന്നതുവരെ റൂട്ട് വർക്ക്‌സ്‌പെയ്‌സിന് പുറത്ത് തുടരും.

## ഡൗൺലോഡ് ചെയ്യുക

റിലീസ് അസറ്റുകൾ ഒരു പേരിടൽ സ്കീം പിന്തുടരുന്നു — `LocalSend-<version>-<platform>-<arch>.<ext>`, പ്ലാറ്റ്‌ഫോമുകൾ `linux` / `windows` / `macos`, ആർക്കിടെക്ചറുകൾ `x86_64` / `aarch64`:

| പ്ലാറ്റ്ഫോം | അസറ്റ് |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| ഏതെങ്കിലും Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` ഒരു റിലീസിലെ എല്ലാ അസറ്റുകളും ഉൾക്കൊള്ളുന്നു. CI നിർമ്മിച്ച macOS ബിൽഡ് അഡ്-ഹോക്ക് സൈൻ ചെയ്തതാണ്, ഡെവലപ്പർ ഐഡി ഒപ്പിട്ടതോ നോട്ടറൈസ് ചെയ്തതോ അല്ല.

## പെട്ടെന്നുള്ള തുടക്കം

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

Linux പാക്കേജുകൾ ഡെമൺ ലൈഫ് സൈക്കിളിൻ്റെ ഉടമസ്ഥതയിലുള്ള `localsend-tray` ലോഞ്ച് ചെയ്യുന്ന ഒരു ഡെസ്ക്ടോപ്പ് എൻട്രി ഇൻസ്റ്റാൾ ചെയ്യുന്നു. സ്റ്റാറ്റസ് ബാർ ഇല്ലാത്ത ഡെസ്ക്ടോപ്പുകളിൽ ട്രേ പകരം GUI തുറന്ന് സൂക്ഷിക്കുന്നു; `LocalSend --launcher`, AppImage എന്നതിൽ നിന്ന് ആ മോഡ് വ്യക്തമായി തിരഞ്ഞെടുക്കുന്നു.

### Windows

അൺസിപ്പ് ചെയ്ത് `LocalSend.exe` റൺ ചെയ്യുക. ആർക്കൈവ് GUI, ഡെമൺ, ട്രേ, ലോഞ്ചർ എന്നിവ വശങ്ങളിലായി സൂക്ഷിക്കുന്നു; അവരെ ഒന്നിച്ചു നിർത്തുക.

## അറിഞ്ഞിരിക്കേണ്ട പെരുമാറ്റം

- **ക്വിറ്റ് എന്നത് ഒരു ഓപ്പറേഷൻ ആണ്.** ട്രേയിൽ നിന്ന് പുറത്തുകടക്കുന്നത് എൻഡ്‌പോയിൻ്റ്-സ്കോപ്പിലുള്ള `gui.pid` എന്നതിൽ രേഖപ്പെടുത്തിയിരിക്കുന്ന GUI അവസാനിപ്പിക്കുന്നു, ഒരു നിശ്ചിത സമയപരിധിക്കുള്ളിൽ ഉടമസ്ഥതയിലുള്ള ഡെമണിനെ നിർത്തുന്നു, ഒരു പ്രക്രിയയോ ലോഞ്ച് ചെയ്ത കീപ്പ്-ലൈവ് ജോലിയോ അവശേഷിപ്പിക്കാതെ പുറത്തുകടക്കുന്നു.
- **അറിയിപ്പുകൾ ട്രേയിൽ നിന്നാണ് വരുന്നത്.** GUI ഒരിക്കലും അതിൻ്റേതായ അറിയിപ്പുകൾ പോസ്റ്റുചെയ്യുന്നില്ല, അതിനാൽ ഓരോ പ്ലാറ്റ്‌ഫോമിനും ഒരൊറ്റ ഡെലിവറി പാതയുണ്ട്.
- **വിൻഡോ സ്‌റ്റേറ്റ് ഓരോ എൻഡ്‌പോയിൻ്റിനും ആണ്.** പ്ലെയ്‌സ്‌മെൻ്റ് ആ എൻഡ്‌പോയിൻ്റിനായുള്ള ഡെമൺ കോൺഫിഗറേഷനോട് ചേർന്ന് ഒരു പങ്കിട്ട ഗ്ലോബൽ ലൊക്കേഷനിൽ സൂക്ഷിക്കുന്നു, അതിനാൽ രണ്ട് സന്ദർഭങ്ങൾ വിൻഡോ ജ്യാമിതിയുമായി ബന്ധപ്പെട്ട് പോരാടുന്നില്ല.
- **ഡെമൺ ആധികാരികമാണ്.** GUI എന്നതിൽ മാറ്റം വരുത്തിയ ക്രമീകരണങ്ങൾ ഡെമണിന് സമർപ്പിക്കുകയും ഒരു സ്നാപ്പ്ഷോട്ടിലൂടെ പ്രതിധ്വനിക്കുകയും ചെയ്യുന്നു; ഒരു ഫ്രണ്ട്എൻഡ് ഡ്രിഫ്റ്റ് ചെയ്യാൻ കഴിയുന്ന ഒരു സ്വകാര്യ പകർപ്പും സൂക്ഷിക്കുന്നില്ല.

## ഭാഷകൾ

57 ലൊക്കേലുകളിൽ ഇൻ്റർഫേസ് ഷിപ്പ് ചെയ്യുന്നു, അപ്‌സ്ട്രീം LocalSend യുടെ ലിസ്റ്റുമായി പൊരുത്തപ്പെടുന്നു, ലളിതമാക്കിയ ചൈനീസ് msgstr ഭാഷയായി. **ക്രമീകരണങ്ങൾ → ഭാഷ** എന്നതിന് കീഴിൽ ഒന്ന് തിരഞ്ഞെടുക്കുക, അല്ലെങ്കിൽ OS ലൊക്കേൽ ഉപയോഗിക്കുന്നതിന് "സിസ്റ്റം പിന്തുടരുക" എന്നതിൽ വിടുക.

| | |
| --- | --- |
| ബണ്ടിൽ ചെയ്ത കാറ്റലോഗുകൾ | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UI ഉറവിട സ്ട്രിംഗുകൾ | `apps/localsend-gui-slint/ui` എന്നതിൽ `@tr("...")` |
| ഇംഗ്ലീഷ് കാറ്റലോഗ് | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust സന്ദേശങ്ങൾ) |
| റസ്റ്റ് സൈഡ് സന്ദേശങ്ങൾ | `crates/localsend-i18n` (ജനറേറ്റ് ചെയ്തത് `lookup_generated.rs`) |
| അപ്സ്ട്രീം മിറർ | `translations/upstream/<locale>.json` |

ഏതെങ്കിലും UI സ്ട്രിംഗ് മാറ്റിയതിന് ശേഷം കാറ്റലോഗുകൾ പുനഃസൃഷ്ടിക്കുക:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` പ്രതിജ്ഞാബദ്ധമാണ്, അതിനാൽ ജനറേറ്റ് ചെയ്ത കാറ്റലോഗുകൾ ഓഫ്‌ലൈനിൽ പുനർനിർമ്മിക്കാവുന്നതാണ്. വിവർത്തനം ശരിയാക്കാൻ അവിടെ ഒരു എൻട്രി ശരിയാക്കി ജനറേറ്റർ വീണ്ടും പ്രവർത്തിപ്പിക്കുക.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## ഉറവിടത്തിൽ നിന്നുള്ള കെട്ടിടം

ആവശ്യകതകൾ: Rust `1.97.1` (`rust-toolchain.toml` പിൻ ചെയ്‌തത്) കൂടാതെ, Linux ബിൽഡുകൾക്ക്, GTK3, AppIndicator വികസന തലക്കെട്ടുകൾ.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

പ്ലാറ്റ്ഫോം പാക്കേജുകൾ:

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

സ്വീകാര്യത സ്ക്രിപ്റ്റുകൾ പരിഹസിച്ച അവസ്ഥയ്ക്ക് പകരം യഥാർത്ഥ ബൈനറികളെ നയിക്കുന്നു:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## ഡോക്യുമെൻ്റേഷൻ

| പ്രമാണം | ഉള്ളടക്കം |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | ഓപ്പറേഷൻ, പാക്കേജിംഗ്, പ്ലാറ്റ്ഫോം പെരുമാറ്റം |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | നിലവിലെ ടെസ്റ്റ് തെളിവുകളും അറിയപ്പെടുന്ന സ്ഥിരീകരിക്കാത്ത പ്രദേശങ്ങളും |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | പ്രാദേശിക സ്വീകാര്യത കവാടങ്ങൾ |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | സ്വഭാവം മാറ്റുന്നതിന് മുമ്പ് പ്രതീക്ഷിക്കുന്ന ആവർത്തിച്ചുള്ള റിഗ്രഷനുകളും തെളിവുകളുടെ പാതയും |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | ഓപ്പൺഡിസൈൻ പ്രോട്ടോടൈപ്പിനെതിരായ വിഷ്വൽ കൺവേർജൻസ് അവസ്ഥ |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Android റൂട്ട് മൊഡ്യൂളും അതിൻ്റെ പരിധികളും |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | വെണ്ടർഡ് കോറിൻ്റെ അപ്‌സ്ട്രീം പ്രൊവെനൻസ് |

ഈ README എല്ലാ 57 ഇൻ്റർഫേസ് ഭാഷകളിലും പ്രസിദ്ധീകരിച്ചിട്ടുണ്ട് - മുകളിലുള്ള സ്വിച്ചറിൽ നിന്ന് ഒന്ന് തിരഞ്ഞെടുക്കുക. `README.md` എന്നതിൽ നിന്ന് `scripts/translate-readme.mjs` മുഖേനയാണ് വിവർത്തനങ്ങൾ സൃഷ്ടിച്ചത്; തിരുത്തലുകൾ [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json) എന്നതിൽ പോകുന്നു.

ചില പ്രമാണങ്ങൾ ചൈനീസ് ഭാഷയിൽ എഴുതിയിരിക്കുന്നു; അവർ സമ്മതിച്ച വാക്കുകളിൽ സ്ഥിരീകരണ പ്രവർത്തനങ്ങളും കൺവെൻഷനുകളും രേഖപ്പെടുത്തുന്നു.

## വ്യാപ്തി

പ്രാഥമിക ഉൽപ്പന്നം പിസി ക്ലയൻ്റാണ്. Android പിന്തുണ റൂട്ട്-ഒൺലി ആണ്, ഹെഡ്‌ലെസ് റിസീവ് ഡെമൺ ആ പരിധിക്കപ്പുറം വ്യക്തമായി പ്രമോട്ടുചെയ്യുന്നത് വരെ - ഇതിന് GUI ഇല്ല, ഫോർഗ്രൗണ്ട് സേവനമൊന്നും ആരംഭിക്കുന്നില്ല, സ്ഥിരമായ അറിയിപ്പൊന്നും പോസ്റ്റ് ചെയ്യുന്നില്ല. `crates/localsend-official-core` എന്നത് അതിൻ്റെ റെക്കോർഡ് ചെയ്ത അപ്‌സ്‌ട്രീം എക്‌സ്‌പോർട്ടിന് സമാനമായി ബൈറ്റ്-ഫോർ-ബൈറ്റ് സൂക്ഷിക്കുന്നു, അത് സൃഷ്ടിച്ച `SHA256SUMS` ഫയലിന് പുറമെ.

## ക്രെഡിറ്റുകൾ

പ്രോട്ടോക്കോളും വയർ ഫോർമാറ്റും Rust കോറും വരുന്നത് [localsend/localsend](https://github.com/localsend/localsend); ഈ പ്രോജക്റ്റ് പ്രോട്ടോക്കോൾ വീണ്ടും നടപ്പിലാക്കുന്നതിനുപകരം ആ ജോലിയിൽ ഒരു ഡെസ്ക്ടോപ്പ് ക്ലയൻ്റ് നിർമ്മിക്കുന്നു. LocalSend രചയിതാക്കൾക്കും സംഭാവകർക്കും നന്ദി.

ഇൻ്റർഫേസ് ഈ പ്രോജക്റ്റിനായി വികസിപ്പിച്ച ഒരു ഡിസൈൻ പ്രോട്ടോടൈപ്പ് പിന്തുടരുന്നു; അതിൻ്റെ വിഷ്വൽ റഫറൻസ് [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/) എന്നതിൽ ജീവിക്കുന്നു.

## സംഭാവന ചെയ്യുന്നു

- **ബഗുകളും നിർദ്ദേശങ്ങളും**: പതിപ്പ്, പ്ലാറ്റ്‌ഫോം, പുനർനിർമ്മാണ ഘട്ടങ്ങൾ, നിങ്ങൾ കണ്ടത് എന്നിവ ഉപയോഗിച്ച് ഒരു [പ്രശ്നം](https://github.com/alkaid-ops/localsend-next/issues) തുറക്കുക. ഒരു ലോഗ് അല്ലെങ്കിൽ സ്ക്രീൻഷോട്ട് അത് പ്രവർത്തിക്കുന്നത് വളരെ വേഗത്തിലാക്കുന്നു.
- **കോഡ്**: പുൾ അഭ്യർത്ഥനകൾ സ്വാഗതം ചെയ്യുന്നു. daemon/frontend അതിർത്തി, വെണ്ടർഡ് കോർ അല്ലെങ്കിൽ വിഷ്വൽ കരാറിനെ സ്പർശിക്കുന്ന മാറ്റങ്ങൾ ആദ്യം ചർച്ച ചെയ്യണം - നിയന്ത്രണങ്ങൾ [`AGENTS.md`](../../AGENTS.md) എന്നതിൽ രേഖപ്പെടുത്തിയിട്ടുണ്ട്.
- **വിവർത്തനങ്ങൾ**: കമ്മിറ്റ്ഡ് മെഷീൻ-ട്രാൻസ്ലേഷൻ കാഷെയിൽ നിന്നാണ് കാറ്റലോഗുകൾ ജനറേറ്റ് ചെയ്യുന്നത്, അതിനാൽ തിരുത്തലുകൾ അവലോകനം ചെയ്യുന്നതിനുള്ള ഏറ്റവും എളുപ്പമുള്ള സംഭാവനയാണ്.

## ലൈസൻസ്

[അപ്പാച്ചെ-2.0](../../LICENSE). മൂന്നാം കക്ഷി ഘടകങ്ങളും അവയുടെ ലൈസൻസുകളും [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md) എന്നതിൽ ലിസ്‌റ്റ് ചെയ്‌തിരിക്കുന്നു.

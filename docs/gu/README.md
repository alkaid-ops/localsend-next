<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | **ગુજરાતી** | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [简体中文](../../docs/zh-CN/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next એ લોકલસેન્ડ-સુસંગત ડેસ્કટોપ ક્લાયંટ છે જે Rust માં લખાયેલ છે. તે અધિકૃત પ્રોટોકોલ બોલે છે, તેથી તે ફ્લટર એપ્લિકેશનને બદલે મૂળ ડેસ્કટોપ પ્રોગ્રામ તરીકે ચલાવતી વખતે, બંને દિશામાં સત્તાવાર LocalSend એપ્લિકેશન સાથે ફાઇલોને શોધે છે અને સ્થાનાંતરિત કરે છે.

નેટવર્ક બાજુ અધિકૃત LocalSend Rust કોર છે, જે અપરિવર્તિત છે. તેની ઉપરની દરેક વસ્તુ — ડિમન, RPC લેયર, ટ્રે, ટર્મિનલ UI અને Slint GUI — તે કોરની આસપાસ બનેલ છે.

## ડેમો

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| મોકલો | સેટિંગ્સ |
| --- | --- |
| ![ફાઇલ, ફોલ્ડર, ટેક્સ્ટ અને ક્લિપબોર્ડ સ્ત્રોતો અને નજીકના ઉપકરણો સાથે પૃષ્ઠ મોકલો](../../docs/images/readme-send-light.png) | ![થીમ, ભાષા, નેટવર્ક અને ટ્રાન્સફર વિકલ્પો સાથેનું સેટિંગ્સ પૃષ્ઠ](../../docs/images/readme-settings-light.png) |

## વિશેષતાઓ

- ફરીથી અમલીકરણને બદલે વેન્ડર અપસ્ટ્રીમ કોર પર, સત્તાવાર LocalSend ગ્રાહકો સામે શોધ અને ટ્રાન્સફર
- એક પ્રમાણભૂત રાજ્ય માલિક: ડિમન સેટિંગ્સ, ઓળખ, મનપસંદ, ઇતિહાસ, સ્થાનાંતરણ અને TLS ઓળખ ધરાવે છે, તેથી દરેક ફ્રન્ટએન્ડ સમાન વસ્તુ જુએ છે
- GUI, ટર્મિનલ UI અને ટ્રે સમાન સ્થાનિક પર સ્વતંત્ર ફ્રન્ટએન્ડ તરીકે RPC; ટ્રે ડિમન અને GUI પ્રક્રિયા જીવનચક્ર ધરાવે છે
- રીઅલટાઇમ પ્રતિ-ફાઇલ પ્રગતિ, ખુલ્લી અને જાહેર ક્રિયાઓ સાથેનો ઇતિહાસ, મનપસંદ, મેન્યુઅલ ઉપકરણ એન્ટ્રી અને વેબ શેર લિંક્સ પ્રાપ્ત કરે છે
- અપસ્ટ્રીમ LocalSendની સૂચિ સાથે મેળ ખાતી 57 ઇન્ટરફેસ ભાષાઓ, રનટાઇમ પર અથવા સિસ્ટમ લોકેલને અનુસરીને સ્વિચ કરી શકાય છે
- મૂળ ડેસ્કટોપ એકીકરણ: ફક્ત ટ્રેમાંથી સૂચનાઓ, લોગિન પર લોંચ કરો, ટ્રેમાં નાનું કરો, વિન્ડો પ્લેસમેન્ટ દ્રઢતા, અને macOS વિન્ડો જે તેના શીર્ષક બારમાં ખેંચે છે
- macOS (DMG/ZIP), Linux (deb, rpm, AppImage, ટારબોલ) અને Windows (ZIP), x86_64 અને aarch64 બંને માટે પેકેજિંગ

## આર્કિટેક્ચર

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

ડિમન એ એકમાત્ર પ્રક્રિયા છે જે નેટવર્ક સાથે વાત કરે છે અથવા એપ્લિકેશન સ્ટેટ લખે છે. ફ્રન્ટેન્ડ્સ ક્યારેય વેન્ડર્ડ કોર પર આધાર રાખતા નથી અને ક્યારેય ડિમનને જાતે જ પેદા કરતા નથી; ટ્રે કરે છે. ફાઇલ બાઇટ્સ RPC થી વધુ મુસાફરી કરતા નથી — ડિમન પાથને ઉકેલે છે, સામગ્રીને હેશ કરે છે અને ઇતિહાસ રેકોર્ડ કરે છે.

| દ્વિસંગી | ભૂમિકા |
| --- | --- |
| `localsendd` | નેટવર્ક સેવા અને પ્રમાણભૂત એપ્લિકેશન સ્થિતિ |
| `localsend-tray` | ડિમન અને GUI જીવનચક્રના માલિક, સૂચનાઓ |
| `localsend-gui-slint` | FemtoVG રેન્ડરર સાથે ઔપચારિક GUI, Slint |
| `localsend-tui` | ટર્મિનલ ફ્રન્ટએન્ડ |
| `localsend-launcher` | ટ્રે વિના પર્યાવરણ માટે સ્પષ્ટ લૉન્ચર |
| `localsend-androidd` | સ્ટેન્ડઅલોન રૂટ-ઓન્લી Android પ્રાપ્ત ડિમન (MVP) |

`localsend-gui-egui`, `-fltk`, `-iced` અને `-tauri` ડિરેક્ટરીઓ પ્રાયોગિક પોર્ટ છે અને જ્યાં સુધી તેમનું RPC એકીકરણ પૂર્ણ ન થાય ત્યાં સુધી રૂટ વર્કસ્પેસની બહાર રહે છે.

## ડાઉનલોડ કરો

અસ્કયામતો એક નામકરણ યોજનાને અનુસરે છે — `LocalSend-<version>-<platform>-<arch>.<ext>`, પ્લેટફોર્મ `linux` / `windows` / `macos` અને આર્કિટેક્ચર `x86_64` / `aarch64`:

| પ્લેટફોર્મ | એસેટ |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| કોઈપણ Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` રિલીઝમાં દરેક સંપત્તિને આવરી લે છે. CI દ્વારા ઉત્પાદિત macOS બિલ્ડ એડ-હૉક હસ્તાક્ષરિત છે, વિકાસકર્તા ID હસ્તાક્ષરિત અથવા નોટરાઇઝ્ડ નથી.

## ઝડપી શરૂઆત

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

Linux પેકેજો ડેસ્કટોપ એન્ટ્રી ઇન્સ્ટોલ કરે છે જે `localsend-tray` લોન્ચ કરે છે, જે ડિમન જીવનચક્રની માલિકી ધરાવે છે. સ્ટેટસ બાર વગરના ડેસ્કટોપ પર ટ્રે તેના બદલે GUIને ખુલ્લી રાખે છે; `LocalSend --launcher` તે મોડને AppImageમાંથી સ્પષ્ટપણે પસંદ કરે છે.

### Windows

અનઝિપ કરો અને ચલાવો `LocalSend.exe`. આર્કાઇવમાં GUI, ડિમન, ટ્રે અને લૉન્ચર બાજુમાં છે; તેમને સાથે રાખો.

## જાણવા જેવું વર્તન

- **છોડો એ એક ઑપરેશન છે.** ટ્રેમાંથી બહાર નીકળવું એ એન્ડપોઇન્ટ-સ્કોપ્ડ `gui.pid`માં રેકોર્ડ થયેલ GUIને સમાપ્ત કરે છે, માલિકીના ડિમનને બાઉન્ડેડ સમયસમાપ્તિમાં રોકે છે, અને કોઈ પ્રક્રિયાને છોડ્યા વિના અથવા ચાલુ રાખવા-જીવંત કાર્યને પાછળ રાખ્યા વિના બહાર નીકળી જાય છે.
- **સૂચનાઓ ટ્રેમાંથી આવે છે.** GUI ક્યારેય તેની પોતાની સૂચનાઓ પોસ્ટ કરતું નથી, તેથી દરેક પ્લેટફોર્મનો એક જ વિતરણ માર્ગ હોય છે.
- **વિન્ડો સ્ટેટ પ્રતિ એન્ડપોઇન્ટ છે.** પ્લેસમેન્ટ શેર કરેલ વૈશ્વિક સ્થાનને બદલે તે એન્ડપોઇન્ટ માટે ડિમન રૂપરેખાંકનની બાજુમાં સંગ્રહિત થાય છે, તેથી બે ઉદાહરણો વિન્ડો ભૂમિતિ પર લડતા નથી.
- **ડિમન અધિકૃત છે.** GUI માં બદલાયેલ સેટિંગ્સ ડિમનને સબમિટ કરવામાં આવે છે અને સ્નેપશોટ દ્વારા પાછા પડઘો પાડે છે; ફ્રન્ટએન્ડ એવી કોઈ ખાનગી નકલ રાખતું નથી જે ડ્રિફ્ટ થઈ શકે.

## ભાષાઓ

ઈન્ટરફેસ 57 લોકેલમાં મોકલે છે, જે અપસ્ટ્રીમ LocalSendની યાદી સાથે મેળ ખાય છે, જેમાં msgstr ભાષા તરીકે સરળ ચાઈનીઝ છે. **સેટિંગ્સ → ભાષા** હેઠળ એક પસંદ કરો અથવા OS લોકેલનો ઉપયોગ કરવા માટે તેને "ફૉલો સિસ્ટમ" પર છોડી દો.

| | |
| --- | --- |
| બંડલ કરેલ કેટલોગ | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UI સ્ત્રોત શબ્દમાળાઓ | `@tr("...")` `apps/localsend-gui-slint/ui` માં |
| અંગ્રેજી કેટલોગ | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust સંદેશાઓ) |
| રસ્ટ-સાઇડ સંદેશાઓ | `crates/localsend-i18n` (જનરેટ કરેલ `lookup_generated.rs`) |
| અપસ્ટ્રીમ મિરર | `translations/upstream/<locale>.json` |

કોઈપણ UI સ્ટ્રિંગ બદલ્યા પછી કેટલોગ ફરીથી બનાવો:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` પ્રતિબદ્ધ છે, તેથી જનરેટ કરેલ કેટલોગ ઑફલાઇન પુનઃઉત્પાદન કરી શકાય છે. ત્યાં એક એન્ટ્રીને ઠીક કરો અને અનુવાદને ઠીક કરવા માટે જનરેટરને ફરીથી ચલાવો.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## સ્ત્રોતમાંથી મકાન

જરૂરીયાતો: Rust `1.97.1` (`rust-toolchain.toml` દ્વારા પિન કરેલ) અને Linux બિલ્ડ માટે, GTK3 અને AppIndicator વિકાસ હેડરો.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

પ્લેટફોર્મ પેકેજો:

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

સ્વીકૃતિ સ્ક્રિપ્ટો ઉપહાસની સ્થિતિને બદલે વાસ્તવિક દ્વિસંગી ચલાવે છે:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## દસ્તાવેજીકરણ

| દસ્તાવેજ | સામગ્રી |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | ઓપરેશન, પેકેજિંગ અને પ્લેટફોર્મ વર્તન |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | વર્તમાન પરીક્ષણ પુરાવા અને જાણીતા વણચકાસાયેલ વિસ્તારો |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | સ્થાનિક સ્વીકૃતિ દરવાજા |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | વર્તણૂક બદલતા પહેલા પુનરાવર્તિત રીગ્રેસન અને પુરાવા ટ્રેઇલ અપેક્ષિત છે |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | OpenDesign પ્રોટોટાઇપ સામે વિઝ્યુઅલ કન્વર્જન્સ સ્ટેટ |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Android રૂટ મોડ્યુલ અને તેની મર્યાદા |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | વેન્ડર્ડ કોરનો અપસ્ટ્રીમ ઉદ્ભવ |

આ README તમામ 57 ઈન્ટરફેસ ભાષાઓમાં પ્રકાશિત થયેલ છે - ટોચ પરના સ્વિચરમાંથી એક પસંદ કરો. અનુવાદો `README.md` થી `scripts/translate-readme.mjs` દ્વારા જનરેટ કરવામાં આવ્યા છે; સુધારાઓ [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json) માં જાય છે.

કેટલાક દસ્તાવેજો ચાઇનીઝમાં લખાયેલા છે; તેઓ જે શબ્દોમાં સંમત થયા હતા તે શબ્દોમાં તેઓ ચકાસણી રન અને સંમેલનો રેકોર્ડ કરે છે.

## અવકાશ

પ્રાથમિક ઉત્પાદન પીસી ક્લાયન્ટ છે. Android આધાર એ ફક્ત રૂટ છે, જ્યાં સુધી સ્પષ્ટપણે તે અવકાશની બહાર પ્રમોટ ન થાય ત્યાં સુધી હેડલેસ રીસીવ ડિમન છે — તેમાં કોઈ GUI નથી, કોઈ ફોરગ્રાઉન્ડ સેવા શરૂ થતી નથી અને કોઈ સતત સૂચના પોસ્ટ કરતી નથી. `crates/localsend-official-core` તેની જનરેટ કરેલ `SHA256SUMS` ફાઈલ સિવાય, તેના રેકોર્ડ કરેલ અપસ્ટ્રીમ નિકાસ માટે બાઈટ-ફોર-બાઈટ સમાન રાખવામાં આવે છે.

## શ્રેય

પ્રોટોકોલ, વાયર ફોર્મેટ અને Rust કોર [localsend/localsend](https://github.com/localsend/localsend); આ પ્રોજેક્ટ પ્રોટોકોલને ફરીથી અમલમાં મૂકવાને બદલે તે કાર્ય પર ડેસ્કટોપ ક્લાયંટ બનાવે છે. LocalSend લેખકો અને સહયોગીઓનો આભાર.

ઇન્ટરફેસ આ પ્રોજેક્ટ માટે વિકસિત ડિઝાઇન પ્રોટોટાઇપને અનુસરે છે; તેનો દ્રશ્ય સંદર્ભ [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/) માં રહે છે.

## ફાળો આપી રહ્યા છે

- **બગ્સ અને સૂચનો**: વર્ઝન, પ્લેટફોર્મ, રિપ્રોડક્શન સ્ટેપ્સ અને તમે જે જોયું તે સાથે એક [સમસ્યા](https://github.com/alkaid-ops/localsend-next/issues) ખોલો. લોગ અથવા સ્ક્રીનશૉટ તેના પર કાર્ય કરવાનું વધુ ઝડપી બનાવે છે.
- **કોડ**: પુલ વિનંતીઓનું સ્વાગત છે. ફેરફારો કે જે daemon/frontend સીમા, વિક્રેતા કોર અથવા વિઝ્યુઅલ કોન્ટ્રાક્ટને સ્પર્શે છે તેની પ્રથમ ચર્ચા થવી જોઈએ — અવરોધો [`AGENTS.md`](../../AGENTS.md) માં રેકોર્ડ કરવામાં આવ્યા છે.
- **અનુવાદ**: કૅટલોગ પ્રતિબદ્ધ મશીન-અનુવાદ કૅશમાંથી જનરેટ કરવામાં આવે છે, તેથી સુધારાઓ સમીક્ષા માટે સૌથી સરળ પ્રકારનું યોગદાન છે.

## લાઇસન્સ

[Apache-2.0](../../LICENSE). તૃતીય-પક્ષના ઘટકો અને તેમના લાઇસન્સ [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md) માં સૂચિબદ્ધ છે.

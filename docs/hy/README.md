<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | **Հայերեն** | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next-ը LocalSend-ի հետ համատեղելի աշխատասեղանի հաճախորդ է, որը գրված է Rust-ով: Այն խոսում է պաշտոնական արձանագրության մասին, ուստի այն հայտնաբերում և փոխանցում է ֆայլերը պաշտոնական LocalSend հավելվածով երկու ուղղություններով, մինչդեռ աշխատում է որպես հայրենի աշխատասեղանի ծրագիր Flutter հավելվածի փոխարեն:

Ցանցի կողմը պաշտոնական LocalSend Rust միջուկն է, որը վաճառվում է անփոփոխ: Դրա վերևում գտնվող ամեն ինչ՝ daemon, RPC շերտ, սկուտեղ, տերմինալի միջերես և Slint GUI, կառուցված է այդ միջուկի շուրջ:

## Դեմո

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Ուղարկել | Կարգավորումներ |
| --- | --- |
| [Ուղարկել էջը ֆայլի, թղթապանակի, տեքստի և սեղմատախտակի աղբյուրներով և մոտակա սարքերով](../../docs/images/readme-send-light.png) | ![Կարգավորումների էջ՝ թեմայով, լեզվով, ցանցով և փոխանցման ընտրանքներով](../../docs/images/readme-settings-light.png) |

## Առանձնահատկություններ

- Բացահայտում և փոխանցում պաշտոնական LocalSend հաճախորդների դեմ, վաճառվող վերին հոսքի միջուկում, այլ ոչ թե վերագործարկում
- Մեկ կանոնական պետության սեփականատեր. դեյմոնը ունի կարգավորումներ, ինքնություն, ֆավորիտներ, պատմություն, փոխանցումներ և TLS ինքնությունը, այնպես որ յուրաքանչյուր ճակատը տեսնում է նույնը:
- GUI, տերմինալային միջերես և սկուտեղ որպես անկախ ճակատներ նույն տեղական RPC-ի վրա; սկուտեղին պատկանում է daemon-ը և GUI գործընթացի կյանքի ցիկլը
- Իրական ժամանակում յուրաքանչյուր ֆայլի առաջընթացը, պատմությունը բաց և բացահայտող գործողություններով, ֆավորիտներ, սարքի ձեռքով մուտքագրում և Web Share ստանալու հղումներ
- 57 ինտերֆեյսի լեզուներ, որոնք համապատասխանում են LocalSend-ի ցանկին, որոնք կարող են փոխարկվել գործարկման ժամանակ կամ հետևել համակարգի տեղորոշմանը
- Մայրենի աշխատասեղանի ինտեգրում. ծանուցումներ միայն սկուտեղից, գործարկում մուտք գործելու ժամանակ, նվազագույնի հասցնել սկուտեղ, պատուհանի տեղադրման կայունություն և macOS պատուհան, որը ձգվում է իր վերնագրի տողում:
- Փաթեթավորում macOS (@4@@/ZIP), Linux (deb, rpm, AppImage, tarball) և Windows (ZIP) և x86_64 և aarch64 համարների համար

## Ճարտարապետություն

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Դեյմոնը միակ գործընթացն է, որը խոսում է ցանցի հետ կամ գրում է հավելվածի վիճակը: Frontend-ները երբեք կախված չեն վաճառվող միջուկից և երբեք չեն ծնում դեյմոնն իրենք: սկուտեղն անում է: Ֆայլի բայթերը չեն անցնում RPC-ի վրայով — դեյմոնը լուծում է ուղիները, հեշավորում է բովանդակությունը և գրանցում պատմությունը:

| Երկուական | Դեր |
| --- | --- |
| `localsendd` | Ցանցային ծառայության և կանոնական կիրառման վիճակը |
| `localsend-tray` | Daemon և GUI կյանքի ցիկլի սեփականատեր, ծանուցումներ |
| `localsend-gui-slint` | Պաշտոնական GUI, Slint FemtoVG մատուցողով |
| `localsend-tui` | Տերմինալի ճակատ |
| `localsend-launcher` | Բացահայտ գործարկիչ առանց սկուտեղի միջավայրերի համար |
| `localsend-androidd` | Անկախ միայն արմատային Android ստացեք դաեմոն (MVP) |

`localsend-gui-egui`, `-fltk`, `-iced` և `-tauri` դիրեկտորիաները փորձնական նավահանգիստներ են և մնում են արմատային աշխատանքային տարածքից դուրս, մինչև դրանց RPC ինտեգրումն ավարտվի:

## Բեռնել

Թողարկման ակտիվները հետևում են անվանման մեկ սխեմայի՝ `LocalSend-<version>-<platform>-<arch>.<ext>`, `linux` / `windows` / `macos` և ճարտարապետություններով `x86_64` / `aarch64`:

| Հարթակ | Ակտիվ |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Ցանկացած Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` ընդգրկում է թողարկման յուրաքանչյուր ակտիվ: CI-ի կողմից արտադրված macOS կառուցվածքը ժամանակավոր ստորագրված է, այլ ոչ թե մշակողի ID-ն ստորագրված կամ նոտարական վավերացված:

## Արագ մեկնարկ

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

Linux փաթեթները տեղադրում են աշխատասեղանի մուտքագրում, որը գործարկում է `localsend-tray`, որին պատկանում է դեյմոնի կյանքի ցիկլը: Առանց կարգավիճակի գծի աշխատասեղանների վրա սկուտեղը բաց է պահում GUI-ը: `LocalSend --launcher` ընտրում է այդ ռեժիմը բացահայտորեն AppImage-ից:

### Windows

Unzip և գործարկեք `LocalSend.exe`: Արխիվում պահվում են GUI-ը, դեյմոնը, սկուտեղը և գործարկիչը կողք կողքի; պահեք դրանք միասին:

## Վարքագիծ, որն արժե իմանալ

- **Դուրս գալը մեկ գործողություն է:** Սկուտեղից դուրս գալը դադարեցնում է GUI-ը, որը գրանցված է վերջնակետի շրջանակով `gui.pid`-ում, դադարեցնում է սեփական դեյմոնը սահմանափակ ժամանակի ընթացքում և դուրս է գալիս առանց որևէ գործընթաց կամ մեկնարկած պահելու գործը թողնելու:
- **Ծանուցումները գալիս են սկուտեղից:** GUI-ը երբեք չի հրապարակում իր սեփական ծանուցումները, ուստի յուրաքանչյուր հարթակ ունի առաքման մեկ ուղի:
- **Պատուհանի վիճակը յուրաքանչյուր վերջնակետի համար է:** Տեղադրումը պահվում է դեյմոնի կազմաձևման կողքին այդ վերջնակետի համար, այլ ոչ թե ընդհանուր գլոբալ վայրում, այնպես որ երկու օրինակներ չեն պայքարում պատուհանի երկրաչափության շուրջ:
- **Դեյմոնը հեղինակավոր է:** GUI-ում փոխված կարգավորումները ներկայացվում են դեյմոնին և արձագանքվում են լուսանկարի միջոցով; Frontend-ը չի պահում մասնավոր պատճենը, որը կարող է շեղվել:

## Լեզուներ

Ինտերֆեյսը առաքվում է 57 տեղանքով՝ համապատասխանելով վերին հոսքի LocalSend ցուցակին, պարզեցված չինարենով որպես msgid լեզու: Ընտրեք մեկը **Կարգավորումներ → Լեզուն** կամ թողեք այն «Հետևեք համակարգին»՝ ՕՀ-ի տեղայնությունն օգտագործելու համար:

| | |
| --- | --- |
| Միավորված կատալոգներ | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UI աղբյուրի տողեր | `@tr("...")` `apps/localsend-gui-slint/ui`-ում |
| Անգլերեն կատալոգ | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust հաղորդագրություններ) |
| Ժանգոտող հաղորդագրություններ | `crates/localsend-i18n` (ստեղծվել է `lookup_generated.rs`) |
| Վերին հոսքի հայելի | `translations/upstream/<locale>.json` |

Վերականգնեք կատալոգները UI-ի ցանկացած տող փոխելուց հետո.

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` պարտավորված է, այնպես որ ստեղծված կատալոգները վերարտադրելի են անցանց: Ուղղեք այնտեղ գրառումը և նորից գործարկեք գեներատորը՝ թարգմանությունը շտկելու համար:

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Կառուցում աղբյուրից

Պահանջներ.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Պլատֆորմի փաթեթներ.

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

Ընդունման սցենարները մղում են իրական երկուականները՝ ծաղրված վիճակի փոխարեն.

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Փաստաթղթեր

| Փաստաթուղթ | Բովանդակություն |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Գործողություն, փաթեթավորում և հարթակի վարքագիծ |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Ընթացիկ փորձարկման ապացույցներ և հայտնի չստուգված տարածքներ |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Տեղական ընդունման դարպասներ |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Կրկնվող ռեգրեսիաները և վարքագիծը փոխելուց առաջ սպասվող ապացույցների հետքը |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Տեսողական կոնվերգենցիայի վիճակ OpenDesign նախատիպի դեմ |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Android արմատային մոդուլը և դրա սահմանները |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Վաճառվող միջուկի վերին հոսքի ծագումը |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Որոշ փաստաթղթեր գրված են չինարենով. նրանք արձանագրում են ստուգման վազքներն ու կոնվենցիաները այն բառերով, որոնց հետ համաձայնեցվել են:

## Շրջանակ

Առաջնային արտադրանքը համակարգչի հաճախորդն է: Android աջակցությունը միայն արմատային, առանց գլխի ստացման դեյմոն է, քանի դեռ բացահայտորեն չի առաջխաղացվել այդ շրջանակից. այն չունի GUI, չի սկսում որևէ առաջին պլանի ծառայություն և չի հրապարակում մշտական ​​ծանուցում: `crates/localsend-official-core`-ը պահվում է բայթ առ բայթ նույնական, ինչ գրանցված արտահանումը, բացի ստեղծված `SHA256SUMS` ֆայլից:

## Վարկեր

Արձանագրությունը, մետաղալարերի ձևաչափը և Rust միջուկը գալիս են [localsend/localsend](https://github.com/localsend/localsend); այս նախագիծը կառուցում է աշխատասեղանի հաճախորդ այդ աշխատանքի վրա, այլ ոչ թե նորից կիրագործի արձանագրությունը: Շնորհակալություն LocalSend հեղինակներին և ներդրողներին:

Ինտերֆեյսը հետևում է այս նախագծի համար մշակված դիզայնի նախատիպին. դրա տեսողական հղումը ապրում է [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/):

## Նպաստելով

- **Սխալներ և առաջարկներ**. բացեք [թողարկում](https://github.com/alkaid-ops/localsend-next/issues) տարբերակը, հարթակը, վերարտադրման քայլերը և տեսածը: Գրանցամատյանը կամ սքրինշոթը շատ ավելի արագ են դարձնում գործողությունները:
- **Կոդ **. ձգման հարցումները ողջունելի են: Փոփոխությունները, որոնք դիպչում են daemon/frontend սահմանին, վաճառվող միջուկին կամ տեսողական պայմանագրին, պետք է նախ քննարկվեն. սահմանափակումները գրանցված են [`AGENTS.md`](../../AGENTS.md-ում):
- **Թարգմանություններ**. կատալոգները ստեղծվում են մեքենայական-թարգմանական քեշից, ուստի այնտեղ ուղղումները վերանայման ամենահեշտ ձևն են:

## Լիցենզիա

[Apache-2.0](../../LICENSE): Երրորդ կողմի բաղադրիչները և դրանց լիցենզիաները թվարկված են [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md):

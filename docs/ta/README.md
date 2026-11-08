<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | **தமிழ்** | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [简体中文](../../docs/zh-CN/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next என்பது Rust இல் எழுதப்பட்ட LocalSend-இணக்கமான டெஸ்க்டாப் கிளையன்ட் ஆகும். இது அதிகாரப்பூர்வ நெறிமுறையைப் பேசுகிறது, எனவே இது Flutter பயன்பாட்டிற்குப் பதிலாக சொந்த டெஸ்க்டாப் நிரலாக இயங்கும் போது, ​​இரு திசைகளிலும் அதிகாரப்பூர்வ LocalSend பயன்பாட்டைக் கொண்டு கோப்புகளைக் கண்டறிந்து மாற்றுகிறது.

நெட்வொர்க் பக்கமானது அதிகாரப்பூர்வமான LocalSend Rust மையமானது, விற்பனையானது மாறாமல் உள்ளது. அதற்கு மேலே உள்ள அனைத்தும் — டீமான், RPC லேயர், ட்ரே, டெர்மினல் UI மற்றும் Slint GUI — அந்த மையத்தைச் சுற்றி கட்டமைக்கப்பட்டுள்ளது.

## டெமோ

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| அனுப்பு | அமைப்புகள் |
| --- | --- |
| ![கோப்பு, கோப்புறை, உரை மற்றும் கிளிப்போர்டு மூலங்கள் மற்றும் அருகிலுள்ள சாதனங்களுடன் பக்கத்தை அனுப்பவும்](../../docs/images/readme-send-light.png) | ![தீம், மொழி, நெட்வொர்க் மற்றும் பரிமாற்ற விருப்பங்கள் கொண்ட அமைப்புகள் பக்கம்](../../docs/images/readme-settings-light.png) |

## அம்சங்கள்

- அதிகாரப்பூர்வமான LocalSend வாடிக்கையாளர்களுக்கு எதிராக, விற்பனை செய்யப்பட்ட அப்ஸ்ட்ரீம் மையத்தில் மீண்டும் செயல்படுத்துவதற்குப் பதிலாக கண்டறிதல் மற்றும் பரிமாற்றம்
- ஒரு நியமன மாநில உரிமையாளர்: டீமான் அமைப்புகள், அடையாளம், பிடித்தவை, வரலாறு, இடமாற்றங்கள் மற்றும் TLS அடையாளத்தை வைத்திருக்கிறது, எனவே ஒவ்வொரு முன்பக்கமும் ஒரே விஷயத்தைப் பார்க்கிறது
- GUI, டெர்மினல் UI மற்றும் ட்ரே ஆகியவை அதே உள்ளூர் RPC மீது சுயாதீன முன்முனைகளாக; தட்டில் டீமான் மற்றும் GUI செயல்முறை வாழ்க்கை சுழற்சி உள்ளது
- ஒவ்வொரு கோப்பிற்கும் நிகழ்நேர முன்னேற்றம், திறந்த மற்றும் வெளிப்படுத்தும் செயல்களுடன் கூடிய வரலாறு, பிடித்தவை, கைமுறை சாதன நுழைவு மற்றும் இணையப் பகிர்வு இணைப்புகளைப் பெறுதல்
- 57 இடைமுக மொழிகள் அப்ஸ்ட்ரீம் LocalSend இன் பட்டியலுக்குப் பொருந்தும், இயக்க நேரத்திலோ அல்லது கணினி மொழியைப் பின்பற்றியோ மாறலாம்
- நேட்டிவ் டெஸ்க்டாப் ஒருங்கிணைப்பு: தட்டில் இருந்து மட்டும் அறிவிப்புகள், உள்நுழைவின் போது தொடங்குதல், தட்டுக்கு சிறிதாக்குதல், சாளரம் வைக்கும் நிலைத்தன்மை மற்றும் macOS சாளரம் அதன் தலைப்புப் பட்டியில் ஈர்க்கும்
- macOS க்கான பேக்கேஜிங்

## கட்டிடக்கலை

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

நெட்வொர்க்குடன் பேசும் அல்லது பயன்பாட்டு நிலையை எழுதும் ஒரே செயல்முறை டெமான் ஆகும். முன்னணிகள் ஒருபோதும் விற்பனை செய்யப்பட்ட மையத்தை சார்ந்து இருப்பதில்லை மற்றும் டீமனை ஒருபோதும் உருவாக்காது; தட்டு செய்கிறது. கோப்பு பைட்டுகள் RPCக்கு மேல் பயணிக்காது — டீமான் பாதைகள், ஹாஷ் உள்ளடக்கம் மற்றும் பதிவுகள் வரலாற்றைத் தீர்க்கிறது.

| பைனரி | பங்கு |
| --- | --- |
| `localsendd` | நெட்வொர்க் சேவை மற்றும் நியமன பயன்பாட்டு நிலை |
| `localsend-tray` | டீமான் மற்றும் GUI வாழ்க்கைச் சுழற்சி உரிமையாளர், அறிவிப்புகள் |
| `localsend-gui-slint` | FemtoVG ரெண்டரருடன் முறையான GUI, Slint |
| `localsend-tui` | முனையத்தின் முன்பகுதி |
| `localsend-launcher` | தட்டு இல்லாத சூழல்களுக்கான வெளிப்படையான துவக்கி |
| `localsend-androidd` | தனித்த ரூட் மட்டும் Android ரிசீவ் டீமான் (எம்விபி) |

`localsend-gui-egui`, `-fltk`, `-iced` மற்றும் `-tauri` கோப்பகங்கள் சோதனை போர்ட்கள் மற்றும் அவற்றின் RPC ஒருங்கிணைப்பு முடியும் வரை ரூட் பணியிடத்திற்கு வெளியே இருக்கும்.

## பதிவிறக்கவும்

சொத்துக்களை வெளியிடுவது ஒரு பெயரிடும் திட்டத்தைப் பின்பற்றுகிறது — `LocalSend-<version>-<platform>-<arch>.<ext>`, தளங்களுடன் `linux` / `windows` / `macos` மற்றும் கட்டமைப்புகள் `x86_64` / `aarch64`:

| மேடை | சொத்து |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| ஏதேனும் Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` ஒரு வெளியீட்டில் உள்ள ஒவ்வொரு சொத்தையும் உள்ளடக்கியது. CI ஆல் தயாரிக்கப்பட்ட macOS உருவாக்கம் தற்காலிகமாக கையொப்பமிடப்பட்டது, டெவலப்பர் ஐடி கையொப்பமிடப்பட்ட அல்லது நோட்டரிஸ் செய்யப்படவில்லை.

## விரைவான தொடக்கம்

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

Linux பேக்கேஜ்கள் டெஸ்க்டாப் உள்ளீட்டை நிறுவும், அது `localsend-tray`, டீமான் லைஃப்சைக்கிளுக்கு சொந்தமானது. நிலைப் பட்டி இல்லாத டெஸ்க்டாப்புகளில், தட்டு GUI ஐத் திறந்து வைத்திருக்கும்; `LocalSend --launcher` அந்த பயன்முறையை AppImage இலிருந்து வெளிப்படையாகத் தேர்ந்தெடுக்கிறது.

### Windows

அன்ஜிப் செய்து `LocalSend.exe` இயக்கவும். காப்பகம் GUI, டீமான், தட்டு மற்றும் லாஞ்சர் ஆகியவற்றை அருகருகே வைத்திருக்கிறது; அவற்றை ஒன்றாக வைத்திருங்கள்.

## தெரிந்து கொள்ள வேண்டிய நடத்தை

- ** வெளியேறுவது ஒரு செயல்பாடு.** ட்ரேயில் இருந்து வெளியேறுவது, எண்ட்பாயிண்ட்-ஸ்கோப்பில் `gui.pid` இல் பதிவுசெய்யப்பட்ட GUI ஐ நிறுத்துகிறது, வரையறுக்கப்பட்ட காலக்கெடுவுக்குள் சொந்தமான டீமானை நிறுத்துகிறது, மேலும் ஒரு செயல்முறையை அல்லது தொடங்கப்பட்ட கீப்-ஆலைவ் வேலையை விட்டுவிடாமல் வெளியேறுகிறது.
- **அறிவிப்புகள் தட்டில் இருந்து வருகின்றன.** GUI அதன் சொந்த அறிவிப்புகளை வெளியிடுவதில்லை, எனவே ஒவ்வொரு இயங்குதளத்திற்கும் ஒரு டெலிவரி பாதை உள்ளது.
- **ஒரு இறுதிப்புள்ளிக்கு சாளர நிலை.** பகிரப்பட்ட உலகளாவிய இருப்பிடத்தை விட அந்த இறுதிப்புள்ளிக்கான டீமான் உள்ளமைவுக்கு அருகில் இடம் சேமிக்கப்படுகிறது, எனவே இரண்டு நிகழ்வுகள் சாளர வடிவவியலில் சண்டையிடாது.
- **டீமான் அதிகாரப்பூர்வமானது.** GUI இல் மாற்றப்பட்ட அமைப்புகள் டீமானுக்குச் சமர்ப்பிக்கப்பட்டு, ஸ்னாப்ஷாட் மூலம் எதிரொலிக்கும்; ஒரு முன்பக்கம் நகர்ந்து செல்லக்கூடிய தனிப்பட்ட நகலை வைத்திருக்காது.

## மொழிகள்

57 லோகேல்களில் இடைமுகம் அனுப்பப்படுகிறது, அப்ஸ்ட்ரீம் LocalSend இன் பட்டியலுக்கு பொருந்தும், msgstr மொழியாக எளிமைப்படுத்தப்பட்ட சீனம். **அமைப்புகள் → மொழி** என்பதன் கீழ் ஒன்றைத் தேர்ந்தெடுக்கவும் அல்லது OS லோகேலைப் பயன்படுத்த, "ஃபாலோ சிஸ்டம்" என்பதில் விடவும்.

| | |
| --- | --- |
| தொகுக்கப்பட்ட பட்டியல்கள் | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UI மூல சரங்கள் | `apps/localsend-gui-slint/ui` இல் `@tr("...")` |
| ஆங்கில அட்டவணை | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust செய்திகள்) |
| துருப்பிடித்த செய்திகள் | `crates/localsend-i18n` (உருவாக்கப்பட்டது `lookup_generated.rs`) |
| அப்ஸ்ட்ரீம் கண்ணாடி | `translations/upstream/<locale>.json` |

எந்த UI சரத்தையும் மாற்றிய பின் பட்டியல்களை மீண்டும் உருவாக்கவும்:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` உறுதியளிக்கப்பட்டுள்ளது, எனவே உருவாக்கப்பட்ட பட்டியல்கள் ஆஃப்லைனில் மீண்டும் உருவாக்கப்படும். ஒரு உள்ளீட்டைச் சரிசெய்து, மொழிபெயர்ப்பைச் சரிசெய்ய ஜெனரேட்டரை மீண்டும் இயக்கவும்.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## மூலத்திலிருந்து கட்டிடம்

தேவைகள்: Rust `1.97.1` (`rust-toolchain.toml` ஆல் பின் செய்யப்பட்டது) மற்றும் Linux பில்ட்களுக்கு, GTK3 மற்றும் AppIndicator மேம்பாட்டு தலைப்புகள்.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

பிளாட்ஃபார்ம் தொகுப்புகள்:

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

ஏற்றுக்கொள்ளும் ஸ்கிரிப்டுகள் கேலி நிலைக்குப் பதிலாக உண்மையான பைனரிகளை இயக்குகின்றன:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## ஆவணப்படுத்தல்

| ஆவணம் | உள்ளடக்கம் |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | செயல்பாடு, பேக்கேஜிங் மற்றும் இயங்குதள நடத்தை |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | தற்போதைய சோதனை சான்றுகள் மற்றும் அறியப்படாத சரிபார்க்கப்படாத பகுதிகள் |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | உள்ளூர் ஏற்றுக்கொள்ளும் வாயில்கள் |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | தொடர்ச்சியான பின்னடைவுகள் மற்றும் நடத்தையை மாற்றுவதற்கு முன் எதிர்பார்க்கப்படும் சான்றுகள் |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | OpenDesign முன்மாதிரிக்கு எதிரான காட்சி ஒருங்கிணைப்பு நிலை |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Android ரூட் தொகுதி மற்றும் அதன் வரம்புகள் |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | விற்பனை செய்யப்பட்ட மையத்தின் அப்ஸ்ட்ரீம் ஆதாரம் |

இந்த README அனைத்து 57 இடைமுக மொழிகளிலும் வெளியிடப்பட்டுள்ளது - மேலே உள்ள ஸ்விட்ச்சரில் இருந்து ஒன்றைத் தேர்ந்தெடுக்கவும். மொழிபெயர்ப்புகள் `README.md` இலிருந்து `scripts/translate-readme.mjs` மூலம் உருவாக்கப்படுகின்றன; திருத்தங்கள் [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json) இல் செல்கின்றன.

சில ஆவணங்கள் சீன மொழியில் எழுதப்பட்டுள்ளன; அவர்கள் ஒப்புக்கொண்ட வார்த்தைகளில் சரிபார்ப்பு ஓட்டங்களையும் மரபுகளையும் பதிவு செய்கிறார்கள்.

## நோக்கம்

முதன்மை தயாரிப்பு பிசி கிளையன்ட் ஆகும். Android ஆதரவு என்பது ரூட்-மட்டுமே, ஹெட்லெஸ் ரிசீவ் டீமான் அந்த எல்லைக்கு அப்பால் வெளிப்படையாக விளம்பரப்படுத்தப்படும் வரை — அதற்கு GUI இல்லை, எந்த முன்புற சேவையையும் தொடங்காது மற்றும் நிலையான அறிவிப்பை வெளியிடாது. `crates/localsend-official-core` ஆனது, உருவாக்கப்பட்ட `SHA256SUMS` கோப்பைத் தவிர, அதன் பதிவுசெய்யப்பட்ட அப்ஸ்ட்ரீம் ஏற்றுமதிக்கு பைட்டுக்கு-பைட்டுக்கு ஒரே மாதிரியாக வைக்கப்படுகிறது.

## கடன்கள்

நெறிமுறை, கம்பி வடிவம் மற்றும் Rust மையமானது [localsend/localsend](https://github.com/localsend/localsend) இலிருந்து வருகிறது; இந்த திட்டம் நெறிமுறையை மீண்டும் செயல்படுத்துவதை விட அந்த வேலையில் டெஸ்க்டாப் கிளையண்டை உருவாக்குகிறது. LocalSend ஆசிரியர்கள் மற்றும் பங்களிப்பாளர்களுக்கு நன்றி.

இந்த திட்டத்திற்காக உருவாக்கப்பட்ட வடிவமைப்பு முன்மாதிரியை இடைமுகம் பின்பற்றுகிறது; அதன் காட்சி குறிப்பு [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/) இல் உள்ளது.

## பங்களிக்கிறது

- **பிழைகள் மற்றும் பரிந்துரைகள்**: பதிப்பு, இயங்குதளம், மறுஉருவாக்கம் படிகள் மற்றும் நீங்கள் பார்த்தவற்றுடன் [சிக்கல்](https://github.com/alkaid-ops/localsend-next/issues) திறக்கவும். ஒரு பதிவு அல்லது ஸ்கிரீன்ஷாட் செயல்படுவதை மிக வேகமாக செய்கிறது.
- **குறியீடு**: இழுக்கும் கோரிக்கைகள் வரவேற்கப்படுகின்றன. daemon/frontend எல்லையைத் தொடும் மாற்றங்கள், விற்பனை செய்யப்பட்ட மைய அல்லது காட்சி ஒப்பந்தம் முதலில் விவாதிக்கப்பட வேண்டும் - கட்டுப்பாடுகள் [`AGENTS.md`](../../AGENTS.md) இல் பதிவு செய்யப்பட்டுள்ளன.
- **மொழிபெயர்ப்புகள்**: உறுதியான இயந்திர-மொழிபெயர்ப்பு தற்காலிக சேமிப்பிலிருந்து பட்டியல்கள் உருவாக்கப்படுகின்றன, எனவே திருத்தங்கள் மதிப்பாய்வு செய்ய எளிதான பங்களிப்பாகும்.

## உரிமம்

[அப்பாச்சி-2.0](../../LICENSE). மூன்றாம் தரப்பு கூறுகளும் அவற்றின் உரிமங்களும் [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md) இல் பட்டியலிடப்பட்டுள்ளன.

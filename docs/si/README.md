<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | **සිංහල** | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next යනු Rust හි ලියා ඇති LocalSend-අනුකූල ඩෙස්ක්ටොප් සේවාදායකයෙකි. එය නිල ප්‍රොටෝකෝලය කථා කරයි, එබැවින් එය Flutter යෙදුමක් වෙනුවට ස්වදේශීය ඩෙස්ක්ටොප් වැඩසටහනක් ලෙස ක්‍රියාත්මක වන අතරම, දෙපැත්තටම නිල LocalSend යෙදුම සමඟ ගොනු සොයා ගෙන මාරු කරයි.

ජාල පැත්ත නිල LocalSend Rust හරය, නොවෙනස්ව විකුණනු ලැබේ. ඊට ඉහළින් ඇති සියල්ල - ඩීමන්, RPC ස්ථරය, තැටි, පර්යන්ත UI සහ Slint GUI - එම හරය වටා ගොඩනගා ඇත.

## Demo

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| යවන්න | සැකසීම් |
| --- | --- |
| ![ගොනුව, ෆෝල්ඩරය, පෙළ සහ ක්ලිප්බෝඩ් මූලාශ්‍ර සහ අවට උපාංග සමඟ පිටුව යවන්න](../../docs/images/readme-send-light.png) | ![තේමාව, භාෂාව, ජාල සහ මාරු විකල්ප සහිත සැකසීම් පිටුව](../../docs/images/readme-settings-light.png) |

## විශේෂාංග

- නිල LocalSend සේවාදායකයින්ට එරෙහිව සොයා ගැනීම සහ මාරු කිරීම, නැවත ක්‍රියාත්මක කිරීමකට වඩා අලෙවි කරන ලද උඩු ප්‍රවාහයේ හරය මත
- එක් කැනොනිකල් රාජ්‍ය හිමිකරුවෙක්: ඩීමන් සතුව සිටුවම්, අනන්‍යතාවය, ප්‍රියතමයන්, ඉතිහාසය, මාරුවීම් සහ TLS අනන්‍යතාවය ඇත, එබැවින් සෑම ඉදිරිපෙළම එකම දේ දකියි
- GUI, ටර්මිනල් UI සහ තැටිය එකම දේශීය RPC හරහා ස්වාධීන ඉදිරිපස ලෙස; තැටියට ඩේමන් සහ GUI ක්‍රියාවලි ජීවන චක්‍රය හිමිවේ
- එක් ගොනුවකට තත්‍ය කාලීන ප්‍රගතිය, විවෘත සහ හෙළි කරන ක්‍රියාවන් සහිත ඉතිහාසය, ප්‍රියතමයන්, අතින් උපාංග ඇතුළත් කිරීම සහ වෙබ් බෙදාගැනීම සබැඳි ලබා ගැනීම
- අතුරුමුහුණත් භාෂා 57ක් upstream LocalSend හි ලැයිස්තුවට ගැලපේ, ධාවන වේලාවේදී හෝ පද්ධති පෙදෙසිය අනුගමනය කිරීමෙන් මාරු කළ හැකිය
- ස්වදේශීය ඩෙස්ක්ටොප් ඒකාබද්ධ කිරීම: තැටියෙන් පමණක් දැනුම්දීම්, පුරනය වීමේදී දියත් කිරීම, තැටියට අවම කිරීම, කවුළු ස්ථානගත කිරීමේ පැවැත්ම, සහ එහි මාතෘකා තීරුව තුළට ඇද ගන්නා macOS කවුළුවක්
- macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) සහ Windows (ZIP), x86_64 සහ aarch64 යන දෙකටම ඇසුරුම් කිරීම

## ගෘහ නිර්මාණ ශිල්පය

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

ජාලය සමඟ කතා කරන හෝ යෙදුම් තත්ත්වය ලියන එකම ක්‍රියාවලිය ඩීමන් ය. පෙරමුනු කිසිවිටෙකත් වෙළෙන්දාගේ හරය මත රඳා නොපවතින අතර කිසිවිටෙක ඩීමන්ව බිහි නොකරයි; තැටිය කරයි. ගොනු බයිට් RPC හරහා ගමන් නොකරයි - ඩීමන් මාර්ග, හැෂ් අන්තර්ගතය සහ වාර්තා ඉතිහාසය විසඳයි.

| ද්විමය | භූමිකාව |
| --- | --- |
| `localsendd` | ජාල සේවාව සහ කැනොනිකල් යෙදුම් තත්ත්වය |
| `localsend-tray` | Daemon සහ GUI ජීවන චක්‍ර හිමිකරු, දැනුම්දීම් |
| `localsend-gui-slint` | FemtoVG විදැහුම්කරු සමඟ විධිමත් GUI, Slint |
| `localsend-tui` | පර්යන්ත ඉදිරිපස |
| `localsend-launcher` | තැටියක් නොමැති පරිසරයන් සඳහා පැහැදිලි දියත් කිරීම |
| `localsend-androidd` | ස්වාධීන මූල-පමණක් Android ඩේමන් ලබා ගන්න (MVP) |

`localsend-gui-egui`, `-fltk`, `-iced` සහ `-tauri` නාමාවලි පර්යේෂණාත්මක වරායන් වන අතර ඒවායේ RPC අනුකලනය සම්පූර්ණ වන තෙක් මූල වැඩබිමෙන් පිටත පවතී.

## බාගන්න

වත්කම් මුදා හැරීම එක් නම් කිරීමේ ක්‍රමයක් අනුගමනය කරයි — `LocalSend-<version>-<platform>-<arch>.<ext>`, වේදිකා `linux` / `windows` / `macos` සහ ගෘහ නිර්මාණ ශිල්පය `x86_64` / `aarch64`:

| වේදිකාව | වත්කම |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| ඕනෑම Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` නිකුතුවක සෑම වත්කමක්ම ආවරණය කරයි. CI විසින් නිෂ්පාදනය කරන ලද macOS ගොඩනැගීම තාවකාලිකව අත්සන් කර ඇත, සංවර්ධක හැඳුනුම්පත අත්සන් කර හෝ නොතාරිස් කර නැත.

## ඉක්මන් ආරම්භය

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

Linux පැකේජ ඩීමන් ජීවන චක්‍රය හිමි `localsend-tray` දියත් කරන ඩෙස්ක්ටොප් ප්‍රවේශයක් ස්ථාපනය කරයි. තත්ත්‍ව තීරුවක් නොමැති ඩෙස්ක්ටොප් වලදී, තැටිය ඒ වෙනුවට GUI විවෘතව තබයි; `LocalSend --launcher` එම මාදිලිය පැහැදිලිවම AppImage වෙතින් තෝරා ගනී.

### Windows

Zip ඉවත් කර `LocalSend.exe` ධාවනය කරන්න. සංරක්ෂිතය GUI, ඩීමන්, තැටිය සහ දියත් කිරීම පැත්තකින් තබා ඇත; ඔවුන් එකට තබා ගන්න.

## දැනගත යුතු හැසිරීම

- **Quit යනු එක් මෙහෙයුමකි.** බන්දේසියෙන් ඉවත් වීම අන්ත ලක්ෂ්‍යයේ `gui.pid` හි සටහන් කර ඇති GUI අවසන් කරයි, සීමා සහිත කාල සීමාවක් තුළ හිමිකාර ඩීමන් නවත්වයි, සහ ක්‍රියාවලියක් හෝ දියත් කළ පණපිටින් තබා ගැනීමේ කාර්යයක් ඉතිරි නොකර පිටවීම.
- **දැනුම්දීම් තැටියෙන් පැමිණේ.** GUI කිසි විටෙක තමන්ගේම දැනුම්දීම් පළ නොකරයි, එබැවින් සෑම වේදිකාවකටම තනි බෙදාහැරීමේ මාර්ගයක් ඇත.
- **කවුළු තත්ත්වය අන්ත ලක්ෂ්‍යයකට වේ.** ස්ථානගත කිරීම බෙදාගත් ගෝලීය ස්ථානයකට වඩා එම අන්ත ලක්ෂ්‍යය සඳහා ඩීමන් වින්‍යාසය අසල ගබඩා කර ඇත, එබැවින් අවස්ථා දෙකක් කවුළු ජ්‍යාමිතිය සම්බන්ධයෙන් සටන් නොකරයි.
- **Demon බලයලත් වේ.** GUI හි වෙනස් කරන ලද සැකසීම් ඩීමන් වෙත ඉදිරිපත් කර ස්නැප්ෂොට් එකක් හරහා ප්‍රතිරාවය කරයි; ඉදිරිපස ප්ලාවනය විය හැකි පුද්ගලික පිටපතක් තබා නොගනී.

## භාෂා

අතුරුමුහුණත ස්ථාන 57 කින් නැව්ගත කරයි, upstream LocalSend හි ලැයිස්තුවට ගැලපේ, msgstr භාෂාව ලෙස සරල කළ චීන. **සැකසීම් → භාෂාව** යටතේ එකක් තෝරන්න, නැතහොත් OS පෙදෙසිය භාවිතා කිරීමට එය "පද්ධතිය අනුගමනය කරන්න" මත තබන්න.

| | |
| --- | --- |
| බණ්ඩල් කළ නාමාවලි | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UI මූලාශ්‍ර තන්තු | `apps/localsend-gui-slint/ui` හි `@tr("...")` |
| ඉංග්රීසි නාමාවලිය | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust පණිවිඩ) |
| මලකඩ පැත්තේ පණිවිඩ | `crates/localsend-i18n` (උත්පාදනය `lookup_generated.rs`) |
| උඩුගත කැඩපත | `translations/upstream/<locale>.json` |

ඕනෑම UI තන්තුවක් වෙනස් කිරීමෙන් පසු නාමාවලි නැවත උත්පාදනය කරන්න:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` කැපවී ඇත, එබැවින් ජනනය කරන ලද නාමාවලි නොබැඳිව ප්‍රතිනිෂ්පාදනය කළ හැකිය. පරිවර්තනයක් නිවැරදි කිරීමට එහි ඇතුළත් කිරීමක් නිවැරදි කර උත්පාදක යන්ත්රය නැවත ධාවනය කරන්න.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## මූලාශ්රයෙන් ගොඩනැගීම

අවශ්‍යතා: Rust `1.97.1` (`rust-toolchain.toml` විසින් පින් කර ඇත) සහ, Linux ගොඩනැගීම් සඳහා, GTK3 සහ AppIndicator සංවර්ධන ශීර්ෂයන්.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

වේදිකා පැකේජ:

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

පිළිගැනීමේ ස්ක්‍රිප්ට් සමච්චල් කරන ලද තත්වය වෙනුවට සැබෑ ද්විමය ධාවනය කරයි:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## ලේඛනගත කිරීම

| ලේඛනය | අන්තර්ගතය |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | මෙහෙයුම, ඇසුරුම්කරණය සහ වේදිකා හැසිරීම |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | වත්මන් පරීක්ෂණ සාක්ෂි සහ දන්නා තහවුරු නොකළ ප්‍රදේශ |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | දේශීය පිළිගැනීමේ දොරටු |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | පුනරාවර්තන ප්‍රතිගාමීවීම් සහ හැසිරීම වෙනස් කිරීමට පෙර අපේක්ෂා කරන සාක්ෂි මංපෙත් |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | OpenDesign මූලාකෘතියට එරෙහිව දෘශ්‍ය අභිසාරී තත්ත්වය |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Android මූල මොඩියුලය සහ එහි සීමාවන් |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | අලෙවි කරන ලද හරයේ ඉහළ ප්‍රවාහය |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

සමහර ලේඛන චීන භාෂාවෙන් ලියා ඇත; ඔවුන් එකඟ වූ වචන වලින් සත්‍යාපන ධාවන සහ සම්මුතීන් වාර්තා කරයි.

## විෂය පථය

මූලික නිෂ්පාදනය PC සේවාදායකයා වේ. Android සහාය යනු මුලට පමණක් වන අතර, එම විෂය පථයෙන් ඔබ්බට පැහැදිලිව ප්‍රවර්ධනය කරන තුරු හිස රහිත ඩේමන් ලබා ගැනීමකි - එයට GUI නැත, පෙරබිම් සේවාවක් ආරම්භ නොකරන අතර ස්ථිර දැනුම්දීමක් පළ නොකරයි. `crates/localsend-official-core` එහි උත්පාදනය කරන ලද `SHA256SUMS` ගොනුවට අමතරව, එහි වාර්තාගත උඩුගත අපනයනයට සමානව බයිට් සඳහා බයිට් තබා ඇත.

## ණය

ප්‍රොටෝකෝලය, වයර් ආකෘතිය සහ Rust හරය පැමිණෙන්නේ [localsend/localsend](https://github.com/localsend/localsend); මෙම ව්‍යාපෘතිය ප්‍රොටෝකෝලය නැවත ක්‍රියාත්මක කරනවාට වඩා එම කාර්යය මත ඩෙස්ක්ටොප් සේවාදායකයකු ගොඩනඟයි. LocalSend කතුවරුන්ට සහ දායකයින්ට ස්තූතියි.

අතුරුමුහුණත මෙම ව්‍යාපෘතිය සඳහා සංවර්ධනය කරන ලද සැලසුම් මූලාකෘතියක් අනුගමනය කරයි; එහි දෘශ්‍ය යොමුව [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/) හි ජීවත් වේ.

## දායක වෙනවා

- **දෝෂ සහ යෝජනා**: අනුවාදය, වේදිකාව, ප්‍රතිනිෂ්පාදන පියවර සහ ඔබ දුටු දේ සමඟ [නිකුතුවක්](https://github.com/alkaid-ops/localsend-next/issues) විවෘත කරන්න. ලොගයක් හෝ තිර රුවක් එය ක්‍රියා කිරීම වඩා වේගවත් කරයි.
- **කේතය**: අදින්න ඉල්ලීම් සාදරයෙන් පිළිගනිමු. daemon/frontend සීමාව, විකුණුම් හරය හෝ දෘශ්‍ය කොන්ත්‍රාත්තුව ස්පර්ශ කරන වෙනස්කම් පළමුව සාකච්ඡා කළ යුතුය - සීමාවන් [`AGENTS.md`](../../AGENTS.md) හි සටහන් වේ.
- **පරිවර්තන**: නාමාවලි උත්පාදනය කරනු ලබන්නේ කැපවූ යන්ත්‍ර පරිවර්තන හැඹිලියෙනි, එබැවින් එහි නිවැරදි කිරීම් සමාලෝචනය කිරීමට පහසුම ආකාරයේ දායකත්වය වේ.

## බලපත්රය

[Apache-2.0](../../LICENSE). තෙවන පාර්ශවීය සංරචක සහ ඒවායේ බලපත්‍ර [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md) හි ලැයිස්තුගත කර ඇත.

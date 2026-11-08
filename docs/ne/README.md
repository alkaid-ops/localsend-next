<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | **नेपाली** | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next Rust मा लेखिएको स्थानीय सेन्ड-कम्प्याटिबल डेस्कटप क्लाइन्ट हो। यसले आधिकारिक प्रोटोकल बोल्छ, त्यसैले यसले फ्लुटर एपको सट्टा नेटिभ डेस्कटप प्रोग्रामको रूपमा चलिरहेको बेला, दुबै दिशामा आधिकारिक LocalSend एपसँग फाइलहरू पत्ता लगाउँछ र स्थानान्तरण गर्छ।

नेटवर्क पक्ष आधिकारिक LocalSend Rust कोर हो, अपरिवर्तित विक्रेता। यसको माथिका सबै कुरा — डेमन, RPC तह, ट्रे, टर्मिनल UI र Slint GUI — त्यो कोर वरिपरि बनाइएको छ।

## डेमो

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| पठाउनुहोस् | सेटिङहरू |
| --- | --- |
| ![फाइल, फोल्डर, पाठ र क्लिपबोर्ड स्रोतहरू र नजिकका उपकरणहरू सहित पृष्ठ पठाउनुहोस्] (docs/images/readme-send-light.png) | ![विषयवस्तु, भाषा, नेटवर्क र स्थानान्तरण विकल्पहरूको साथ सेटिङ पृष्ठ](../../docs/images/readme-settings-light.png) |

## विशेषताहरू

- आधिकारिक LocalSend ग्राहकहरू विरुद्ध खोज र स्थानान्तरण, पुन: कार्यान्वयनको सट्टा विक्रेता अपस्ट्रीम कोरमा
- एउटा क्यानोनिकल राज्य मालिक: डेमनले सेटिङहरू, पहिचान, मनपर्नेहरू, इतिहास, स्थानान्तरणहरू र TLS पहिचान राख्छ, त्यसैले प्रत्येक फ्रन्टएन्डले एउटै कुरा देख्छ।
- GUI, टर्मिनल UI र ट्रे उही स्थानीय RPC मा स्वतन्त्र फ्रन्टएन्डको रूपमा; ट्रेमा डेमन र GUI प्रक्रिया जीवनचक्रको स्वामित्व छ
- रियलटाइम प्रति-फाइल प्रगति, खुला र प्रकट कार्यहरू सहितको इतिहास, मनपर्नेहरू, म्यानुअल उपकरण प्रविष्टि, र वेब साझेदारीले लिङ्कहरू प्राप्त गर्दछ।
- अपस्ट्रीम LocalSend को सूचीसँग मिल्ने ५७ इन्टरफेस भाषाहरू, रनटाइममा स्विच गर्न मिल्ने वा प्रणाली लोकेल पछ्याउँदै
- नेटिभ डेस्कटप एकीकरण: ट्रेबाट मात्र सूचनाहरू, लगइनमा सुरू गर्नुहोस्, ट्रेमा न्यूनतम गर्नुहोस्, विन्डो प्लेसमेन्ट दृढता, र macOS विन्डो जुन यसको शीर्षक पट्टीमा तानिन्छ।
- macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) र Windows (ZIP), दुबै x86_64 र aarch64 को लागि प्याकेजिङ

## वास्तुकला

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

डेमन एक मात्र प्रक्रिया हो जसले सञ्जालसँग कुरा गर्छ वा आवेदन अवस्था लेख्छ। फ्रन्टेन्डहरू कहिले पनि विक्रेता कोरमा भर पर्दैनन् र कहिले पनि डेमन आफैंमा स्पोन गर्दैनन्; ट्रे गर्छ। फाइल बाइटहरू RPC मा यात्रा गर्दैन — डेमनले मार्गहरू समाधान गर्दछ, सामग्री ह्यास गर्दछ र इतिहास रेकर्ड गर्दछ।

| बाइनरी | भूमिका |
| --- | --- |
| `localsendd` | नेटवर्क सेवा र क्यानोनिकल आवेदन स्थिति |
| `localsend-tray` | डेमन र GUI जीवनचक्र मालिक, सूचनाहरू |
| `localsend-gui-slint` | FemtoVG रेन्डररसँग औपचारिक GUI, Slint |
| `localsend-tui` | टर्मिनल फ्रन्टएन्ड |
| `localsend-launcher` | ट्रे बिना वातावरणको लागि स्पष्ट लन्चर |
| `localsend-androidd` | स्ट्यान्डअलोन रूट-मात्र Android डेमन प्राप्त गर्नुहोस् (MVP) |

`localsend-gui-egui`, `-fltk`, `-iced` र `-tauri` निर्देशिकाहरू प्रयोगात्मक पोर्टहरू हुन् र तिनीहरूको RPC एकीकरण पूरा नभएसम्म रूट कार्यस्थान बाहिर रहन्छन्।

## डाउनलोड गर्नुहोस्

रिलिज एसेटहरू एउटा नामकरण योजना पछ्याउनुहोस् — `LocalSend-<version>-<platform>-<arch>.<ext>`, प्लेटफर्महरू `linux` / `windows` / `macos` र वास्तुकलाहरू `x86_64` / `aarch64`:

| प्लेटफर्म | सम्पत्ति |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| कुनै पनि Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` विज्ञप्तिमा प्रत्येक सम्पत्ति कभर गर्दछ। CI द्वारा उत्पादित macOS निर्माण तदर्थ हस्ताक्षर गरिएको छ, विकासकर्ता आईडी हस्ताक्षरित वा नोटराइज गरिएको छैन।

## द्रुत सुरुवात

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

Linux प्याकेजहरूले डेस्कटप प्रविष्टि स्थापना गर्दछ जुन `localsend-tray` सुरु हुन्छ, जसले डेमन जीवनचक्रको स्वामित्व लिन्छ। डेस्कटपहरूमा स्टेटस बार बिना ट्रेले GUI खुला राख्छ; `LocalSend --launcher` ले AppImage बाट स्पष्ट रूपमा त्यो मोड चयन गर्छ।

### Windows

अनजिप गर्नुहोस् र `LocalSend.exe` चलाउनुहोस्। अभिलेखले GUI, डेमन, ट्रे र लन्चरलाई छेउमा राख्छ; तिनीहरूलाई सँगै राख्नुहोस्।

## जान्न लायक व्यवहार

- **छोड्नु एउटा अपरेशन हो।** ट्रेबाट बाहिर निस्कँदा GUI एन्डपोइन्ट-स्कोपमा रेकर्ड गरिएको `gui.pid` बन्द हुन्छ, स्वामित्वको डेमोनलाई सीमाबद्ध टाइमआउट भित्र रोक्छ, र प्रक्रिया वा प्रक्षेपण किप-लाइभ काम पछाडि नछोडिकन बाहिर निस्कन्छ।
- **सूचनाहरू ट्रेबाट आउँछन्।** GUI ले कहिले पनि आफ्नै सूचनाहरू पोस्ट गर्दैन, त्यसैले प्रत्येक प्लेटफर्ममा एउटै डेलिभरी मार्ग हुन्छ।
- **सञ्झ्याल स्थिति प्रति अन्तिम बिन्दु हो।** प्लेसमेन्ट साझा विश्वव्यापी स्थानमा भन्दा पनि अन्तिम बिन्दुको लागि डेमन कन्फिगरेसनको छेउमा भण्डार गरिएको छ, त्यसैले दुईवटा उदाहरणहरू सञ्झ्याल ज्यामितिमा लड्दैनन्।
- **डेमन आधिकारिक छ।** GUI मा परिवर्तन गरिएका सेटिङ्हरू डेमनमा पेश गरिएका छन् र स्न्यापसट मार्फत फिर्ता प्रतिध्वनित छन्; फ्रन्टएन्डले कुनै पनि निजी प्रतिलिपि राख्दैन जुन बहाउन सक्छ।

## भाषाहरू

इन्टरफेसले 57 लोकेलहरूमा पठाउँछ, अपस्ट्रीम LocalSend को सूचीसँग मेल खान्छ, सरलीकृत चिनियाँ msgstr भाषाको रूपमा। **सेटिङहरू → भाषा** अन्तर्गत एउटा छान्नुहोस्, वा ओएस लोकेल प्रयोग गर्न "प्रणाली पछ्याउनुहोस्" मा छोड्नुहोस्।

| | |
| --- | --- |
| बन्डल गरिएका क्याटलगहरू | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UI स्रोत स्ट्रिङ | `@tr("...")` `apps/localsend-gui-slint/ui` मा |
| अंग्रेजी सूची | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust सन्देशहरू) |
| रस्ट-साइड सन्देशहरू | `crates/localsend-i18n` (उत्पन्न `lookup_generated.rs`) |
| अपस्ट्रीम मिरर | `translations/upstream/<locale>.json` |

कुनै पनि UI स्ट्रिङ परिवर्तन गरेपछि क्याटलगहरू पुन: उत्पन्न गर्नुहोस्:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` प्रतिबद्ध छ, त्यसैले उत्पन्न गरिएका क्याटलगहरू अफलाइन पुन: उत्पादन योग्य छन्। त्यहाँ एउटा प्रविष्टि सही गर्नुहोस् र अनुवाद ठीक गर्न जेनेरेटर पुन: चलाउनुहोस्।

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## स्रोतबाट निर्माण

आवश्यकताहरू: Rust `1.97.1` (`rust-toolchain.toml` द्वारा पिन गरिएको) र, Linux निर्माणहरूका लागि, GTK3 र AppIndicator विकास हेडरहरू।

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

प्लेटफर्म प्याकेजहरू:

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

स्वीकृति लिपिहरूले ठट्टा गरिएको अवस्थाको सट्टा वास्तविक बाइनरीहरू ड्राइभ गर्दछ:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## कागजात

| कागजात | सामग्री |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | सञ्चालन, प्याकेजिङ्ग र प्लेटफर्म व्यवहार |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | हालको परीक्षण प्रमाण र ज्ञात अप्रमाणित क्षेत्रहरू |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | स्थानीय स्वीकृति गेटहरू |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | आवर्ती प्रतिगमन र व्यवहार परिवर्तन गर्नु अघि अपेक्षित प्रमाण ट्रेल |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | OpenDesign प्रोटोटाइप विरुद्ध भिजुअल अभिसरण अवस्था |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Android रूट मोड्युल र यसको सीमा |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | विक्रेता कोर को अपस्ट्रीम प्रोभेन्सन |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

केही कागजातहरू चिनियाँ भाषामा लेखिएका छन्; तिनीहरू प्रमाणीकरण रनहरू र अधिवेशनहरू रेकर्ड गर्छन् जुन शब्दहरूमा तिनीहरू सहमत थिए।

## स्कोप

प्राथमिक उत्पादन पीसी ग्राहक हो। Android समर्थन एक रूट-मात्र हो, हेडलेस रिसिभ डेमोन जबसम्म स्पष्ट रूपमा त्यो दायरा बाहिर प्रवर्द्धन गर्दैन — यसमा कुनै GUI छैन, कुनै अग्रभूमि सेवा सुरु गर्दैन र कुनै निरन्तर सूचना पोस्ट गर्दैन। `crates/localsend-official-core` यसको रेकर्ड गरिएको अपस्ट्रिम एक्सपोर्टमा बाइट-फर-बाइट समान राखिएको छ, यसको उत्पन्न `SHA256SUMS` फाइल बाहेक।

## क्रेडिटहरू

प्रोटोकल, तार ढाँचा र Rust कोर [localsend/localsend](https://github.com/localsend/localsend); यो परियोजनाले प्रोटोकल पुन: कार्यान्वयन गर्नुको सट्टा त्यो काममा डेस्कटप क्लाइन्ट निर्माण गर्दछ। LocalSend लेखक र योगदानकर्ताहरूलाई धन्यवाद।

इन्टरफेसले यस परियोजनाको लागि विकास गरिएको डिजाइन प्रोटोटाइप पछ्याउँछ; यसको दृश्य सन्दर्भ [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/) मा रहन्छ।

## योगदान गर्दै

- **बगहरू र सुझावहरू**: संस्करण, प्लेटफर्म, प्रजनन चरणहरू र तपाईंले के देख्नुभयो भनेर एउटा [समस्या](https://github.com/alkaid-ops/localsend-next/issues) खोल्नुहोस्। लग वा स्क्रिनसटले कार्य गर्न धेरै छिटो बनाउँछ।
- **कोड**: पुल अनुरोधहरू स्वागत छ। daemon/frontend सीमा, विक्रेता कोर, वा भिजुअल अनुबंधलाई छुने परिवर्तनहरू पहिले छलफल गरिनुपर्छ — अवरोधहरू [`AGENTS.md`](../../AGENTS.md) मा रेकर्ड गरिएका छन्।
- **अनुवाद**: क्याटलगहरू प्रतिबद्ध मेसिन-अनुवाद क्यासबाट उत्पन्न हुन्छन्, त्यसैले त्यहाँ सुधारहरू समीक्षा गर्नको लागि सबैभन्दा सजिलो प्रकारको योगदान हो।

## इजाजतपत्र

[Apache-2.0] (LICENSE)। तेस्रो-पक्ष कम्पोनेन्टहरू र तिनीहरूका इजाजतपत्रहरू [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md) मा सूचीबद्ध छन्।

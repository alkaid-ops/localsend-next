<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | **हिन्दी** | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next एक लोकलसेंड-संगत डेस्कटॉप क्लाइंट है जो Rust में लिखा गया है। यह आधिकारिक प्रोटोकॉल बोलता है, इसलिए यह फ़्लटर ऐप के बजाय मूल डेस्कटॉप प्रोग्राम के रूप में चलते हुए, दोनों दिशाओं में आधिकारिक LocalSend ऐप के साथ फ़ाइलों को खोजता है और स्थानांतरित करता है।

नेटवर्क पक्ष आधिकारिक LocalSend Rust कोर है, अपरिवर्तित रखा गया है। इसके ऊपर सब कुछ - डेमॉन, RPC परत, ट्रे, टर्मिनल यूआई और Slint GUI - उस कोर के आसपास बनाया गया है।

## डेमो

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| भेजना | सेटिंग्स |
| --- | --- |
| ![फ़ाइल, फ़ोल्डर, टेक्स्ट और क्लिपबोर्ड स्रोतों और आस-पास के उपकरणों के साथ पेज भेजें] (docs/images/readme-send-light.png) | ![विषय, भाषा, नेटवर्क और स्थानांतरण विकल्पों के साथ सेटिंग पृष्ठ] (docs/images/readme-settings-light.png) |

## विशेषताएँ

- पुन: कार्यान्वयन के बजाय वेन्डर्ड अपस्ट्रीम कोर पर आधिकारिक LocalSend क्लाइंट के विरुद्ध खोज और स्थानांतरण
- एक विहित राज्य स्वामी: डेमॉन सेटिंग्स, पहचान, पसंदीदा, इतिहास, स्थानांतरण और TLS पहचान रखता है, इसलिए प्रत्येक फ्रंटएंड एक ही चीज़ देखता है
- GUI, टर्मिनल यूआई और ट्रे एक ही स्थानीय RPC पर स्वतंत्र फ्रंटेंड के रूप में; ट्रे डेमॉन और GUI प्रक्रिया जीवनचक्र का स्वामी है
- रीयलटाइम प्रति-फ़ाइल प्रगति, खुले और प्रकट कार्यों के साथ इतिहास, पसंदीदा, मैन्युअल डिवाइस प्रविष्टि और वेब शेयर लिंक प्राप्त करते हैं
- अपस्ट्रीम LocalSend की सूची से मेल खाने वाली 57 इंटरफ़ेस भाषाएँ, रनटाइम पर या सिस्टम लोकेल का अनुसरण करते हुए स्विच करने योग्य
- मूल डेस्कटॉप एकीकरण: केवल ट्रे से सूचनाएं, लॉगिन पर लॉन्च, ट्रे में न्यूनतम, विंडो प्लेसमेंट दृढ़ता, और एक macOS विंडो जो इसके शीर्षक बार में आती है
- macOS (DMG/ZIP), Linux (डेब, आरपीएम, AppImage, टारबॉल) और Windows (ZIP) के लिए पैकेजिंग, x86_64 और aarch64 दोनों के लिए

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

डेमॉन एकमात्र प्रक्रिया है जो नेटवर्क से बात करती है या एप्लिकेशन स्थिति लिखती है। फ़्रंटएंड कभी भी वेन्डर्ड कोर पर निर्भर नहीं होते हैं और कभी भी डेमॉन को स्वयं उत्पन्न नहीं करते हैं; ट्रे करता है. फ़ाइल बाइट्स RPC पर यात्रा नहीं करते - डेमॉन पथों को हल करता है, सामग्री को हैश करता है और इतिहास रिकॉर्ड करता है।

| द्विआधारी | भूमिका |
| --- | --- |
| `localsendd` | नेटवर्क सेवा और विहित अनुप्रयोग स्थिति |
| `localsend-tray` | डेमॉन और GUI जीवनचक्र स्वामी, सूचनाएं |
| `localsend-gui-slint` | FemtoVG रेंडरर के साथ औपचारिक GUI, Slint |
| `localsend-tui` | टर्मिनल फ़्रंटएंड |
| `localsend-launcher` | ट्रे के बिना वातावरण के लिए स्पष्ट लांचर |
| `localsend-androidd` | स्टैंडअलोन रूट-केवल Android डेमॉन प्राप्त करें (एमवीपी) |

`localsend-gui-egui`, `-fltk`, `-iced` और `-tauri` निर्देशिकाएं प्रायोगिक पोर्ट हैं और उनका RPC एकीकरण पूरा होने तक रूट कार्यक्षेत्र से बाहर रहती हैं।

## डाउनलोड करना

रिलीज़ परिसंपत्तियाँ एक नामकरण योजना का पालन करती हैं - `LocalSend-<version>-<platform>-<arch>.<ext>`, प्लेटफ़ॉर्म `linux` / `windows` / `macos` और आर्किटेक्चर `x86_64` / `aarch64` के साथ:

| प्लैटफ़ॉर्म | संपत्ति |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| कोई Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` एक रिलीज़ में प्रत्येक संपत्ति को कवर करता है। सीआई द्वारा निर्मित macOS बिल्ड तदर्थ हस्ताक्षरित है, डेवलपर आईडी हस्ताक्षरित या नोटरीकृत नहीं है।

## त्वरित शुरुआत

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

Linux पैकेज एक डेस्कटॉप प्रविष्टि स्थापित करते हैं जो `localsend-tray` लॉन्च करती है, जो डेमॉन जीवनचक्र का मालिक है। बिना स्टेटस बार वाले डेस्कटॉप पर ट्रे इसके बजाय GUI को खुला रखती है; `LocalSend --launcher` AppImage से स्पष्ट रूप से उस मोड का चयन करता है।

### Windows

अनज़िप करें और `LocalSend.exe` चलाएँ। संग्रह में GUI, डेमॉन, ट्रे और लॉन्चर एक साथ हैं; उन्हें एक साथ रखो.

## जानने लायक व्यवहार

- **छोड़ना एक ऑपरेशन है।** ट्रे से बाहर निकलने से एंडपॉइंट-स्कोप्ड `gui.pid` में दर्ज GUI समाप्त हो जाता है, एक सीमित टाइमआउट के भीतर स्वामित्व वाले डेमॉन को रोक देता है, और किसी प्रक्रिया या लॉन्चड कीप-अलाइव जॉब को पीछे छोड़े बिना बाहर निकल जाता है।
- **अधिसूचनाएं ट्रे से आती हैं।** GUI कभी भी अपनी सूचनाएं पोस्ट नहीं करता है, इसलिए प्रत्येक प्लेटफ़ॉर्म पर एक ही डिलीवरी पथ होता है।
- **विंडो स्थिति प्रति समापन बिंदु है।** प्लेसमेंट को साझा वैश्विक स्थान के बजाय उस समापन बिंदु के लिए डेमॉन कॉन्फ़िगरेशन के बगल में संग्रहीत किया जाता है, इसलिए दो उदाहरण विंडो ज्यामिति पर नहीं लड़ते हैं।
- **डेमन आधिकारिक है।** GUI में बदली गई सेटिंग्स डेमॉन को सबमिट की जाती हैं और एक स्नैपशॉट के माध्यम से वापस प्रतिध्वनित की जाती हैं; फ्रंटएंड कोई निजी प्रति नहीं रखता जो बह सके।

## बोली

इंटरफ़ेस 57 स्थानों में शिप होता है, जो अपस्ट्रीम LocalSend की सूची से मेल खाता है, जिसमें सरलीकृत चीनी msgstr भाषा है। **सेटिंग्स → भाषा** के अंतर्गत एक चुनें, या ओएस लोकेल का उपयोग करने के लिए इसे "फ़ॉलो सिस्टम" पर छोड़ दें।

| | |
| --- | --- |
| बंडल कैटलॉग | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| यूआई स्रोत स्ट्रिंग्स | `@tr("...")` में `apps/localsend-gui-slint/ui` |
| अंग्रेजी कैटलॉग | `translations/locale-en.json` (यूआई), `translations/locale-en-runtime.json` (Rust संदेश) |
| जंग-रोधी संदेश | `crates/localsend-i18n` (उत्पन्न `lookup_generated.rs`) |
| अपस्ट्रीम दर्पण | `translations/upstream/<locale>.json` |

किसी भी यूआई स्ट्रिंग को बदलने के बाद कैटलॉग को पुन: उत्पन्न करें:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` प्रतिबद्ध है, इसलिए उत्पन्न कैटलॉग ऑफ़लाइन प्रतिलिपि प्रस्तुत करने योग्य हैं। वहां एक प्रविष्टि को ठीक करें और अनुवाद को ठीक करने के लिए जनरेटर को फिर से चलाएं।

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## स्रोत से निर्माण

आवश्यकताएँ: Rust `1.97.1` (`rust-toolchain.toml` द्वारा पिन किया गया) और, Linux बिल्ड के लिए, GTK3 और AppIndicator डेवलपमेंट हेडर।

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

प्लेटफ़ॉर्म पैकेज:

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

स्वीकृति स्क्रिप्ट नकली स्थिति के बजाय वास्तविक बायनेरिज़ चलाती हैं:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## प्रलेखन

| दस्तावेज़ | अंतर्वस्तु |
| --- | --- |
| [`docs/desktop-pc.md`]docs/desktop-pc.md) | संचालन, पैकेजिंग और प्लेटफ़ॉर्म व्यवहार |
| [`docs/verification.zh-CN.md`]docs/verification.zh-CN.md) | वर्तमान परीक्षण साक्ष्य और ज्ञात असत्यापित क्षेत्र |
| [`docs/acceptance-automation.zh-CN.md`]docs/acceptance-automation.zh-CN.md) | स्थानीय स्वीकृति द्वार |
| [`docs/agent-regression-playbook.zh-CN.md`]docs/agent-regression-playbook.zh-CN.md) | आवर्ती प्रतिगमन और व्यवहार बदलने से पहले अपेक्षित साक्ष्य निशान |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`]docs/HANDOFF_CLAUDE.zh-CN.md) | ओपनडिज़ाइन प्रोटोटाइप के विरुद्ध दृश्य अभिसरण स्थिति |
| [`docs/android-root-mvp.zh-CN.md`]docs/android-root-mvp.zh-CN.md) | Android रूट मॉड्यूल और इसकी सीमाएँ |
| [`VENDORED_CORE.md`]VENDORED_CORE.md) | वेन्डर्ड कोर का अपस्ट्रीम उद्गम |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

कुछ दस्तावेज़ चीनी भाषा में लिखे गए हैं; वे सत्यापन रन और सम्मेलनों को उन शब्दों में रिकॉर्ड करते हैं जिन पर वे सहमत थे।

## दायरा

प्राथमिक उत्पाद पीसी क्लाइंट है. Android समर्थन एक रूट-ओनली, हेडलेस रिसीव डेमॉन है जब तक कि उस दायरे से परे स्पष्ट रूप से प्रचारित नहीं किया जाता है - इसमें कोई GUI नहीं है, कोई अग्रभूमि सेवा शुरू नहीं होती है और कोई लगातार अधिसूचना पोस्ट नहीं करता है। `crates/localsend-official-core` को इसके जेनरेट किए गए `SHA256SUMS` फ़ाइल के अलावा, इसके रिकॉर्ड किए गए अपस्ट्रीम निर्यात के समान बाइट-फॉर-बाइट रखा जाता है।

## क्रेडिट

प्रोटोकॉल, वायर प्रारूप और Rust कोर [localsend/localsend]https://github.com/localsend/localsend) से आते हैं; यह प्रोजेक्ट प्रोटोकॉल को दोबारा लागू करने के बजाय उस कार्य पर एक डेस्कटॉप क्लाइंट बनाता है। LocalSend लेखकों और योगदानकर्ताओं को धन्यवाद।

इंटरफ़ेस इस प्रोजेक्ट के लिए विकसित डिज़ाइन प्रोटोटाइप का अनुसरण करता है; इसका दृश्य संदर्भ [`apps/localsend-gui-slint/reference/open-design/`] (apps/localsend-gui-slint/reference/open-design/) में रहता है।

## योगदान

- **बग और सुझाव**: संस्करण, प्लेटफ़ॉर्म, पुनरुत्पादन चरणों और आपने जो देखा उसके साथ एक [अंक] (https://github.com/alkaid-ops/localsend-next/issues) खोलें। एक लॉग या स्क्रीनशॉट पर कार्रवाई करना बहुत तेज़ हो जाता है।
- **कोड**: पुल अनुरोधों का स्वागत है। जो परिवर्तन daemon/frontend सीमा, विक्रेता कोर, या दृश्य अनुबंध को छूते हैं, उन पर पहले चर्चा की जानी चाहिए - बाधाएं [`AGENTS.md`] (AGENTS.md) में दर्ज की जाती हैं।
- **अनुवाद**: कैटलॉग प्रतिबद्ध मशीन-अनुवाद कैश से उत्पन्न होते हैं, इसलिए वहां सुधार समीक्षा के लिए सबसे आसान प्रकार का योगदान है।

## लाइसेंस

[अपाचे-2.0] (LICENSE). तृतीय-पक्ष घटक और उनके लाइसेंस [THIRD_PARTY_NOTICES.md] (THIRD_PARTY_NOTICES.md) में सूचीबद्ध हैं।

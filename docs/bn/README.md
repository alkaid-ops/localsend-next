<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | **বাংলা** | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next হল একটি লোকালসেন্ড-সামঞ্জস্যপূর্ণ ডেস্কটপ ক্লায়েন্ট যা Rust এ লেখা। এটি অফিসিয়াল প্রোটোকলের সাথে কথা বলে, তাই এটি একটি ফ্লাটার অ্যাপের পরিবর্তে একটি নেটিভ ডেস্কটপ প্রোগ্রাম হিসাবে চলার সময় উভয় দিকেই অফিসিয়াল LocalSend অ্যাপের সাথে ফাইলগুলি আবিষ্কার করে এবং স্থানান্তর করে।

নেটওয়ার্ক সাইড হল অফিসিয়াল LocalSend Rust কোর, বিক্রেতা অপরিবর্তিত। এর উপরের সবকিছু — ডেমন, RPC লেয়ার, ট্রে, টার্মিনাল UI এবং Slint GUI — সেই কোরের চারপাশে তৈরি করা হয়েছে।

## ডেমো

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| পাঠান | সেটিংস |
| --- | --- |
| ![ফাইল, ফোল্ডার, পাঠ্য এবং ক্লিপবোর্ড উত্স এবং কাছাকাছি ডিভাইস সহ পৃষ্ঠা পাঠান](../../docs/images/readme-send-light.png) | ![থিম, ভাষা, নেটওয়ার্ক এবং স্থানান্তর বিকল্প সহ সেটিংস পৃষ্ঠা](../../docs/images/readme-settings-light.png) |

## বৈশিষ্ট্য

- অফিসিয়াল LocalSend ক্লায়েন্টদের বিরুদ্ধে আবিষ্কার এবং স্থানান্তর, পুনরায় বাস্তবায়নের পরিবর্তে বিক্রেতা আপস্ট্রিম কোরে
- একটি ক্যানোনিকাল রাষ্ট্র মালিক: ডেমন সেটিংস, পরিচয়, প্রিয়, ইতিহাস, স্থানান্তর এবং TLS পরিচয় ধারণ করে, তাই প্রতিটি ফ্রন্টএন্ড একই জিনিস দেখে
- GUI, টার্মিনাল UI এবং ট্রে একই স্থানীয় RPC এর উপর স্বাধীন ফ্রন্টএন্ড হিসাবে; ট্রেটি ডেমন এবং GUI প্রক্রিয়া জীবনচক্রের মালিক
- রিয়েলটাইম প্রতি-ফাইল অগ্রগতি, খোলা এবং প্রকাশ ক্রিয়া সহ ইতিহাস, পছন্দসই, ম্যানুয়াল ডিভাইস এন্ট্রি এবং ওয়েব শেয়ার লিঙ্কগুলি গ্রহণ করে
- আপস্ট্রিম LocalSend এর তালিকার সাথে মিলে 57টি ইন্টারফেস ভাষা, রানটাইমে পরিবর্তন করা যায় বা সিস্টেম লোকেল অনুসরণ করা যায়
- নেটিভ ডেস্কটপ ইন্টিগ্রেশন: শুধুমাত্র ট্রে থেকে বিজ্ঞপ্তি, লগইন এ লঞ্চ করা, ট্রেতে মিনিমাইজ করা, উইন্ডো প্লেসমেন্টের অধ্যবসায়, এবং একটি macOS উইন্ডো যা এর শিরোনাম বারে আসে
- macOS (DMG/ZIP), Linux (deb, rpm, AppImage, টারবল) এবং Windows (ZIP), x86_64 এবং aarch64 উভয়ের জন্য প্যাকেজিং

## স্থাপত্য

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

ডেমন হল একমাত্র প্রক্রিয়া যা নেটওয়ার্কের সাথে কথা বলে বা অ্যাপ্লিকেশন স্টেট লেখে। ফ্রন্টেন্ড কখনই বিক্রেতা কোরের উপর নির্ভর করে না এবং কখনই ডেমন নিজেরাই জন্মায় না; ট্রে করে। ফাইল বাইট RPC এর উপর ভ্রমণ করে না — ডেমন পাথ সমাধান করে, বিষয়বস্তু হ্যাশ করে এবং ইতিহাস রেকর্ড করে।

| বাইনারি | ভূমিকা |
| --- | --- |
| `localsendd` | নেটওয়ার্ক পরিষেবা এবং ক্যানোনিকাল অ্যাপ্লিকেশন অবস্থা |
| `localsend-tray` | ডেমন এবং GUI জীবনচক্রের মালিক, বিজ্ঞপ্তি |
| `localsend-gui-slint` | FemtoVG রেন্ডারারের সাথে আনুষ্ঠানিক GUI, Slint |
| `localsend-tui` | টার্মিনাল ফ্রন্টএন্ড |
| `localsend-launcher` | একটি ট্রে ছাড়া পরিবেশের জন্য স্পষ্ট লঞ্চার |
| `localsend-androidd` | শুধুমাত্র স্বতন্ত্র রুট Android ডেমন গ্রহণ (MVP) |

`localsend-gui-egui`, `-fltk`, `-iced` এবং `-tauri` ডিরেক্টরিগুলি পরীক্ষামূলক পোর্ট এবং তাদের RPC ইন্টিগ্রেশন সম্পূর্ণ না হওয়া পর্যন্ত রুট ওয়ার্কস্পেসের বাইরে থাকে।

## ডাউনলোড করুন

রিলিজ সম্পদ এক নামকরণ স্কিম অনুসরণ করে — `LocalSend-<version>-<platform>-<arch>.<ext>`, প্ল্যাটফর্ম `linux` / `windows` / `macos` এবং আর্কিটেকচার `x86_64` / `aarch64`:

| প্ল্যাটফর্ম | সম্পদ |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| যেকোনো Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` একটি রিলিজে প্রতিটি সম্পদ কভার করে। CI দ্বারা উত্পাদিত macOS বিল্ডটি অ্যাড-হক স্বাক্ষরিত, বিকাশকারী আইডি স্বাক্ষরিত বা নোটারাইজড নয়।

## দ্রুত শুরু

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

Linux প্যাকেজগুলি একটি ডেস্কটপ এন্ট্রি ইনস্টল করে যা `localsend-tray` চালু করে, যা ডেমন লাইফসাইকেলের মালিক। স্ট্যাটাস বার ছাড়া ডেস্কটপে ট্রে এর পরিবর্তে GUI খোলা রাখে; `LocalSend --launcher` AppImage থেকে স্পষ্টভাবে সেই মোডটি নির্বাচন করে।

### Windows

আনজিপ করুন এবং `LocalSend.exe` চালান। সংরক্ষণাগারে GUI, ডেমন, ট্রে এবং লঞ্চার পাশাপাশি রয়েছে; তাদের একসাথে রাখুন।

## আচরণ জানার মতো

- **প্রস্থান হল একটি অপারেশন।** ট্রে থেকে প্রস্থান করা GUI এন্ডপয়েন্ট-স্কোপড `gui.pid`-এ রেকর্ড করা বন্ধ করে দেয়, একটি সীমাবদ্ধ সময়সীমার মধ্যে মালিকানাধীন ডেমনকে থামিয়ে দেয় এবং কোনো প্রক্রিয়া বা চালু রাখা-জীবিত কাজকে পিছনে না রেখে প্রস্থান করে।
- **বিজ্ঞপ্তিগুলি ট্রে থেকে আসে৷** GUI কখনই নিজস্ব বিজ্ঞপ্তি পোস্ট করে না, তাই প্রতিটি প্ল্যাটফর্মের একটি একক বিতরণ পথ রয়েছে৷
- **উইন্ডো স্টেট প্রতি এন্ডপয়েন্ট।** প্লেসমেন্ট শেয়ার করা গ্লোবাল লোকেশনের পরিবর্তে সেই এন্ডপয়েন্টের জন্য ডেমন কনফিগারেশনের পাশে সংরক্ষিত থাকে, তাই দুটি ঘটনা উইন্ডো জ্যামিতির সাথে লড়াই করে না।
- **ডেমনটি প্রামাণিক।** GUI-এ পরিবর্তিত সেটিংস ডেমনে জমা দেওয়া হয় এবং একটি স্ন্যাপশটের মাধ্যমে প্রতিধ্বনিত হয়; একটি ফ্রন্টএন্ড কোন ব্যক্তিগত কপি রাখে না যা প্রবাহিত হতে পারে।

## ভাষা

ইন্টারফেসটি 57টি লোকেলে পাঠানো হয়, আপস্ট্রিম LocalSend-এর তালিকার সাথে মিলে যায়, msgstr ভাষা হিসাবে সরলীকৃত চীনা সহ। **সেটিংস → ভাষা**-এর অধীনে একটি বেছে নিন, অথবা OS লোকেল ব্যবহার করতে "ফলো সিস্টেম" এ ছেড়ে দিন।

| | |
| --- | --- |
| বান্ডিল ক্যাটালগ | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UI সোর্স স্ট্রিং | `@tr("...")` `apps/localsend-gui-slint/ui` |
| ইংরেজি ক্যাটালগ | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust বার্তা) |
| মরিচা-সাইড বার্তা | `crates/localsend-i18n` (জেনারেট করা হয়েছে `lookup_generated.rs`) |
| আপস্ট্রিম আয়না | `translations/upstream/<locale>.json` |

কোনো UI স্ট্রিং পরিবর্তন করার পরে ক্যাটালগ পুনরুত্পাদন করুন:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` প্রতিশ্রুতিবদ্ধ, তাই উত্পন্ন ক্যাটালগগুলি অফলাইনে পুনরুত্পাদনযোগ্য। সেখানে একটি এন্ট্রি সংশোধন করুন এবং একটি অনুবাদ ঠিক করতে জেনারেটরটি পুনরায় চালান৷

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## উৎস থেকে বিল্ডিং

প্রয়োজনীয়তা: Rust `1.97.1` (`rust-toolchain.toml` দ্বারা পিন করা) এবং, Linux বিল্ডের জন্য, GTK3 এবং AppIndicator উন্নয়ন শিরোনাম।

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

প্ল্যাটফর্ম প্যাকেজ:

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

গ্রহণযোগ্যতা স্ক্রিপ্টগুলি উপহাস করা অবস্থার পরিবর্তে বাস্তব বাইনারিগুলি চালায়:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## ডকুমেন্টেশন

| দলিল | বিষয়বস্তু |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | অপারেশন, প্যাকেজিং এবং প্ল্যাটফর্ম আচরণ |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | বর্তমান পরীক্ষার প্রমাণ এবং পরিচিত অযাচাইকৃত এলাকা |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | স্থানীয় স্বীকৃতির গেট |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | আচরণ পরিবর্তন করার আগে প্রত্যাশিত পুনরাবৃত্তিমূলক রিগ্রেশন এবং প্রমাণ ট্রেল |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | OpenDesign প্রোটোটাইপের বিপরীতে ভিজ্যুয়াল কনভারজেন্স অবস্থা |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Android রুট মডিউল এবং এর সীমা |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | বিক্রেতা কোর আপস্ট্রিম উদ্ভব |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

কিছু নথি চীনা ভাষায় লেখা হয়; তারা যাচাইকরণ রান এবং কনভেনশন রেকর্ড করে যে শব্দে তারা সম্মত হয়েছিল।

## ব্যাপ্তি

প্রাথমিক পণ্য হল পিসি ক্লায়েন্ট। Android সমর্থন হল একটি রুট-শুধুমাত্র, হেডলেস রিসিভ ডেমন যতক্ষণ না স্পষ্টভাবে সেই সুযোগের বাইরে প্রচার করা হয় — এতে কোন GUI নেই, কোন ফোরগ্রাউন্ড পরিষেবা শুরু হয় না এবং কোন স্থায়ী বিজ্ঞপ্তি পোস্ট করা হয় না। `crates/localsend-official-core` এটির তৈরি করা `SHA256SUMS` ফাইলটি ছাড়াও রেকর্ড করা আপস্ট্রিম এক্সপোর্টের সাথে বাইট-ফর-বাইট অভিন্ন রাখা হয়।

## ক্রেডিট

প্রোটোকল, তারের বিন্যাস এবং Rust কোর আসে [localsend/localsend](https://github.com/localsend/localsend); এই প্রকল্পটি প্রোটোকল পুনরায় প্রয়োগ করার পরিবর্তে সেই কাজের উপর একটি ডেস্কটপ ক্লায়েন্ট তৈরি করে। LocalSend লেখক এবং অবদানকারীদের ধন্যবাদ।

ইন্টারফেস এই প্রকল্পের জন্য বিকশিত একটি নকশা প্রোটোটাইপ অনুসরণ করে; এর ভিজ্যুয়াল রেফারেন্স [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/) এ থাকে।

## অবদান

- **বাগ এবং পরামর্শ**: সংস্করণ, প্ল্যাটফর্ম, প্রজনন পদক্ষেপ এবং আপনি যা দেখেছেন তার সাথে একটি [ইস্যু](https://github.com/alkaid-ops/localsend-next/issues) খুলুন। একটি লগ বা স্ক্রিনশট কাজ করা অনেক দ্রুত করে তোলে।
- **কোড**: টান অনুরোধ স্বাগত জানাই. যে পরিবর্তনগুলি daemon/frontend সীমানা, বিক্রেতা মূল, বা ভিজ্যুয়াল চুক্তিকে স্পর্শ করে তা প্রথমে আলোচনা করা উচিত — সীমাবদ্ধতাগুলি [`AGENTS.md`](../../AGENTS.md) এ রেকর্ড করা হয়েছে৷
- **অনুবাদ**: প্রতিশ্রুতিবদ্ধ মেশিন-অনুবাদ ক্যাশে থেকে ক্যাটালগ তৈরি করা হয়, তাই সংশোধনগুলি পর্যালোচনার জন্য সবচেয়ে সহজ ধরনের অবদান।

## লাইসেন্স

[Apache-2.0](../../LICENSE)। তৃতীয় পক্ষের উপাদান এবং তাদের লাইসেন্সগুলি [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md) এ তালিকাভুক্ত করা হয়েছে।

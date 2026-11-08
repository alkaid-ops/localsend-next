<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | **اردو** | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next ایک لوکل سینڈ سے مطابقت رکھنے والا ڈیسک ٹاپ کلائنٹ ہے جو Rust میں لکھا گیا ہے۔ یہ آفیشل پروٹوکول بولتا ہے، اس لیے یہ Flutter ایپ کے بجائے مقامی ڈیسک ٹاپ پروگرام کے طور پر چلتے ہوئے، دونوں سمتوں میں آفیشل LocalSend ایپ کے ساتھ فائلوں کو دریافت اور منتقل کرتا ہے۔

نیٹ ورک سائیڈ آفیشل LocalSend Rust کور ہے، جس میں کوئی تبدیلی نہیں کی گئی ہے۔ اس کے اوپر کی ہر چیز — ڈیمون، RPC پرت، ٹرے، ٹرمینل UI اور Slint GUI — اس کور کے گرد بنی ہے۔

## ڈیمو

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| بھیجیں۔ | ترتیبات |
| --- | --- |
| ![فائل، فولڈر، ٹیکسٹ اور کلپ بورڈ کے ذرائع اور قریبی آلات کے ساتھ صفحہ بھیجیں](../../docs/images/readme-send-light.png) | ![تھیم، زبان، نیٹ ورک اور منتقلی کے اختیارات کے ساتھ ترتیبات کا صفحہ](../../docs/images/readme-settings-light.png) |

## خصوصیات

- آفیشل LocalSend کلائنٹس کے خلاف دریافت اور منتقلی، دوبارہ عمل درآمد کے بجائے فروخت شدہ اپ اسٹریم کور پر
- ایک کیننیکل ریاست کا مالک: ڈیمون سیٹنگز، شناخت، پسندیدہ، تاریخ، منتقلی اور TLS شناخت رکھتا ہے، لہذا ہر فرنٹ اینڈ ایک ہی چیز دیکھتا ہے۔
- GUI، ٹرمینل UI اور ٹرے بطور آزاد فرنٹ اینڈ اسی مقامی RPC پر؛ ٹرے ڈیمون اور GUI پراسیس لائف سائیکل کی مالک ہے۔
- ریئل ٹائم فی فائل کی پیشرفت، کھلی اور ظاہری کارروائیوں کے ساتھ تاریخ، فیورٹ، دستی ڈیوائس انٹری، اور ویب شیئر کے لنکس موصول ہوتے ہیں۔
- اپ اسٹریم LocalSend کی فہرست سے مماثل 57 انٹرفیس زبانیں، رن ٹائم پر یا سسٹم لوکل کو فالو کرنے کے قابل
- مقامی ڈیسک ٹاپ انضمام: صرف ٹرے سے اطلاعات، لاگ ان پر لانچ، ٹرے کو کم سے کم کرنا، ونڈو پلیسمنٹ کا استقامت، اور ایک macOS ونڈو جو اپنے ٹائٹل بار میں کھینچتی ہے۔
- macOS (DMG/ZIP)، Linux (deb، rpm، AppImage، tarball) اور Windows (ZIP)، x86_64 اور aarch64 دونوں کے لیے پیکیجنگ

## فن تعمیر

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

ڈیمون واحد عمل ہے جو نیٹ ورک سے بات کرتا ہے یا درخواست کی حالت لکھتا ہے۔ فرنٹنڈز کبھی بھی فروخت شدہ کور پر منحصر نہیں ہوتے ہیں اور کبھی بھی خود ڈیمون نہیں بناتے ہیں۔ ٹرے کرتا ہے. فائل بائٹس RPC سے زیادہ سفر نہیں کرتے — ڈیمون راستے حل کرتا ہے، مواد کو ہیش کرتا ہے اور تاریخ کو ریکارڈ کرتا ہے۔

| بائنری | کردار |
| --- | --- |
| `localsendd` | نیٹ ورک سروس اور کیننیکل ایپلیکیشن کی حالت |
| `localsend-tray` | ڈیمون اور GUI لائف سائیکل کے مالک، اطلاعات |
| `localsend-gui-slint` | رسمی GUI، Slint FemtoVG پیش کنندہ کے ساتھ |
| `localsend-tui` | ٹرمینل فرنٹ اینڈ |
| `localsend-launcher` | بغیر ٹرے کے ماحول کے لیے واضح لانچر |
| `localsend-androidd` | اسٹینڈ لون روٹ صرف Android ڈیمون وصول کریں (MVP) |

`localsend-gui-egui`، `-fltk`، `-iced` اور `-tauri` ڈائریکٹریز تجرباتی بندرگاہیں ہیں اور ان کا RPC انضمام مکمل ہونے تک روٹ ورک اسپیس سے باہر رہتی ہیں۔

## ڈاؤن لوڈ کریں۔

ریلیز اثاثے ایک نام کی اسکیم پر عمل کریں — `LocalSend-<version>-<platform>-<arch>.<ext>`، پلیٹ فارم `linux` / `windows` / `macos` اور آرکیٹیکچرز `x86_64` / `aarch64`:

| پلیٹ فارم | اثاثہ |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`، `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`، `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`، `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`، `LocalSend-0.1.3-linux-aarch64.rpm` |
| کوئی بھی Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`، `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` ریلیز میں ہر اثاثہ کا احاطہ کرتا ہے۔ CI کی طرف سے تیار کردہ macOS تعمیر ایڈہاک پر دستخط شدہ ہے، ڈیولپر ID پر دستخط شدہ یا نوٹریز نہیں ہے۔

## فوری آغاز

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

Linux پیکیجز ایک ڈیسک ٹاپ انٹری انسٹال کرتے ہیں جو `localsend-tray` کو لانچ کرتی ہے، جو ڈیمون لائف سائیکل کا مالک ہے۔ ڈیسک ٹاپس پر اسٹیٹس بار کے بغیر ٹرے GUI کو کھلا رکھتی ہے۔ `LocalSend --launcher` اس موڈ کو واضح طور پر AppImage سے منتخب کرتا ہے۔

### Windows

ان زپ کریں اور `LocalSend.exe` چلائیں۔ آرکائیو میں GUI، ڈیمون، ٹرے اور لانچر ساتھ ساتھ رکھے ہوئے ہیں۔ انہیں ایک ساتھ رکھیں.

## جاننے کے قابل سلوک

- **چھوڑنا ایک آپریشن ہے۔** ٹرے سے نکلنے سے GUI اینڈ پوائنٹ کے دائرہ کار میں ریکارڈ شدہ `gui.pid` کو ختم کر دیا جاتا ہے، ملکیتی ڈیمون کو ایک پابند ٹائم آؤٹ کے اندر روک دیا جاتا ہے، اور بغیر کسی عمل کو چھوڑ کر باہر نکل جاتا ہے۔
- **اطلاعات ٹرے سے آتی ہیں۔** GUI کبھی بھی اپنی اطلاعات پوسٹ نہیں کرتا، اس لیے ہر پلیٹ فارم کا ایک ہی ترسیل کا راستہ ہوتا ہے۔
- **ونڈو اسٹیٹ فی اینڈ پوائنٹ ہے۔** پلیسمنٹ اس اینڈ پوائنٹ کے لیے ڈیمون کنفیگریشن کے پاس اسٹور کی جاتی ہے نہ کہ کسی مشترکہ عالمی مقام پر، اس لیے دو مثالیں ونڈو جیومیٹری پر نہیں لڑتی ہیں۔
- **ڈیمون مستند ہے۔** GUI میں تبدیل شدہ سیٹنگز ڈیمون کو جمع کرائی جاتی ہیں اور اسنیپ شاٹ کے ذریعے دوبارہ گونجتی ہیں۔ ایک فرنٹ اینڈ کوئی نجی کاپی نہیں رکھتا ہے جو بڑھے

## زبانیں

انٹرفیس 57 لوکیلز میں بھیجتا ہے، اپ اسٹریم LocalSend کی فہرست سے مماثل ہے، آسان چینی کے ساتھ msgstr زبان۔ **ترتیبات → زبان** کے تحت ایک کو منتخب کریں، یا OS لوکیل استعمال کرنے کے لیے اسے "سسٹم کی پیروی کریں" پر چھوڑ دیں۔

| | |
| --- | --- |
| بنڈل کیٹلاگ | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UI سورس سٹرنگز | `@tr("...")` `apps/localsend-gui-slint/ui` میں |
| انگریزی کیٹلاگ | `translations/locale-en.json` (UI)، `translations/locale-en-runtime.json` (Rust پیغامات) |
| زنگ آلود پیغامات | `crates/localsend-i18n` (جنریٹڈ `lookup_generated.rs`) |
| اپ اسٹریم آئینہ | `translations/upstream/<locale>.json` |

کسی بھی UI سٹرنگ کو تبدیل کرنے کے بعد کیٹلاگ کو دوبارہ تخلیق کریں:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` پرعزم ہے، اس لیے تیار کردہ کیٹلاگ آف لائن دوبارہ پیش کیے جا سکتے ہیں۔ وہاں ایک اندراج کو درست کریں اور ترجمہ کو ٹھیک کرنے کے لیے جنریٹر کو دوبارہ چلائیں۔

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## ماخذ سے عمارت

تقاضے: Rust `1.97.1` (`rust-toolchain.toml` کے ذریعے پن کیا گیا) اور Linux بلڈز کے لیے، GTK3 اور AppIndicator ڈویلپمنٹ ہیڈرز۔

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

پلیٹ فارم پیکجز:

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

قبولیت کے اسکرپٹس مضحکہ خیز حالت کے بجائے اصلی بائنریز چلاتے ہیں:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## دستاویزی

| دستاویز | مشمولات |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | آپریشن، پیکیجنگ اور پلیٹ فارم کا رویہ |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | موجودہ ٹیسٹ شواہد اور معلوم غیر تصدیق شدہ علاقے |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | مقامی قبولیت کے دروازے |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | رویے کو تبدیل کرنے سے پہلے بار بار آنے والے ریگریشنز اور شواہد کی ٹریل متوقع ہے۔ |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | OpenDesign پروٹو ٹائپ کے خلاف بصری کنورجنسی حالت |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Android روٹ ماڈیول اور اس کی حدود |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | وینڈرڈ کور کا اپ اسٹریم پرووننس |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

کچھ دستاویزات چینی زبان میں لکھی گئی ہیں۔ وہ تصدیقی رنز اور کنونشن کو ان الفاظ میں ریکارڈ کرتے ہیں جن میں ان سے اتفاق کیا گیا تھا۔

## دائرہ کار

بنیادی پروڈکٹ پی سی کلائنٹ ہے۔ Android سپورٹ صرف روٹ ہے، ہیڈ لیس ڈیمون وصول کرتا ہے جب تک کہ اس دائرہ کار سے باہر واضح طور پر فروغ نہ دیا جائے — اس میں کوئی GUI نہیں ہے، کوئی پیش منظر سروس شروع نہیں کرتا ہے اور کوئی مستقل اطلاع پوسٹ نہیں کرتا ہے۔ `crates/localsend-official-core` کو اس کی تیار کردہ `SHA256SUMS` فائل کے علاوہ اس کے ریکارڈ شدہ اپ اسٹریم ایکسپورٹ سے بائٹ کے لیے بائٹ یکساں رکھا گیا ہے۔

## کریڈٹس

پروٹوکول، وائر فارمیٹ اور Rust کور [localsend/localsend](https://github.com/localsend/localsend) سے آتے ہیں؛ یہ پروجیکٹ پروٹوکول کو دوبارہ لاگو کرنے کے بجائے اس کام پر ایک ڈیسک ٹاپ کلائنٹ بناتا ہے۔ LocalSend مصنفین اور تعاون کرنے والوں کا شکریہ۔

انٹرفیس اس پروجیکٹ کے لیے تیار کردہ ڈیزائن پروٹو ٹائپ کی پیروی کرتا ہے۔ اس کا بصری حوالہ [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/) میں رہتا ہے۔

## تعاون کرنا

- **بگز اور تجاویز**: ورژن، پلیٹ فارم، تولیدی مراحل اور آپ نے جو کچھ دیکھا اس کے ساتھ ایک [مسئلہ](https://github.com/alkaid-ops/localsend-next/issues) کھولیں۔ ایک لاگ یا اسکرین شاٹ اس پر عمل کرنا بہت تیز بناتا ہے۔
- **کوڈ**: پل کی درخواستیں خوش آئند ہیں۔ وہ تبدیلیاں جو daemon/frontend باؤنڈری کو چھوتی ہیں، وینڈرڈ کور، یا بصری معاہدے پر پہلے بحث کی جانی چاہیے — رکاوٹیں [`AGENTS.md`](../../AGENTS.md) میں درج ہیں۔
- **ترجمے**: کیٹلاگ کمٹڈ مشین ٹرانسلیشن کیشے سے تیار کیے جاتے ہیں، اس لیے جائزے کے لیے سب سے آسان قسم کی تصحیحیں ہیں۔

## لائسنس

[Apache-2.0](../../LICENSE)۔ فریق ثالث کے اجزاء اور ان کے لائسنس [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md) میں درج ہیں۔

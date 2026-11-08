<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | **فارسی** | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next یک سرویس گیرنده دسکتاپ سازگار با LocalSend است که با Rust نوشته شده است. این پروتکل رسمی صحبت می کند، بنابراین فایل ها را با برنامه رسمی LocalSend در هر دو جهت کشف و انتقال می دهد، در حالی که به عنوان یک برنامه دسکتاپ بومی به جای برنامه Flutter اجرا می شود.

طرف شبکه هسته رسمی LocalSend Rust است که بدون تغییر عرضه می‌شود. همه چیز بالای آن - دیمون، لایه RPC، سینی، رابط کاربری ترمینال و Slint GUI - حول آن هسته ساخته شده است.

## نسخه ی نمایشی

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| ارسال کنید | تنظیمات |
| --- | --- |
| ![ارسال صفحه با فایل، پوشه، متن و منابع کلیپ بورد و دستگاه های نزدیک](../../docs/images/readme-send-light.png) | ![صفحه تنظیمات با تم، زبان، شبکه و گزینه‌های انتقال](../../docs/images/readme-settings-light.png) |

## ویژگی‌ها

- کشف و انتقال در برابر مشتریان رسمی LocalSend، در هسته بالادستی عرضه شده به جای پیاده سازی مجدد
- مالک یک ایالت متعارف: دیمون تنظیمات، هویت، موارد دلخواه، تاریخچه، نقل و انتقالات و هویت TLS را در اختیار دارد، بنابراین هر فرانت اند یک چیز را می بیند.
- GUI، رابط کاربری ترمینال و سینی به عنوان فرانت‌اندهای مستقل روی همان RPC محلی؛ سینی مالک daemon و چرخه عمر فرآیند GUI است
- پیشرفت بیدرنگ هر فایل، تاریخچه با اعمال باز و آشکار، موارد دلخواه، ورود دستی دستگاه، و پیوندهای دریافت اشتراک وب
- 57 زبان رابط منطبق با فهرست بالادست LocalSend، قابل تعویض در زمان اجرا یا دنبال کردن منطقه سیستم
- یکپارچه‌سازی دسک‌تاپ بومی: اعلان‌ها فقط از سینی، راه‌اندازی در هنگام ورود، کوچک‌سازی در سینی، ماندگاری قرار دادن پنجره، و یک پنجره macOS که به نوار عنوان آن کشیده می‌شود.
- بسته بندی برای macOS (@4@@/ZIP)، Linux (deb، rpm، AppImage، tarball) و Windows (ZIP)، برای هر دو x86_64 و aarch64

## معماری

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

دیمون تنها فرآیندی است که با شبکه صحبت می کند یا وضعیت برنامه را می نویسد. فرانت‌اندها هرگز به هسته عرضه‌شده وابسته نیستند و هرگز خود دیمون را تولید نمی‌کنند. سینی انجام می دهد. بایت های فایل از RPC عبور نمی کنند - دیمون مسیرها را حل می کند، محتوا را هش می کند و تاریخچه را ثبت می کند.

| باینری | نقش |
| --- | --- |
| `localsendd` | سرویس شبکه و وضعیت برنامه متعارف |
| `localsend-tray` | مالک چرخه عمر Daemon و GUI، اعلان‌ها |
| `localsend-gui-slint` | GUI، Slint رسمی با رندر FemtoVG |
| `localsend-tui` | جلوی پایانه |
| `localsend-launcher` | لانچر صریح برای محیط های بدون سینی |
| `localsend-androidd` | Root-only مستقل Android دریافت دیمون (MVP) |

دایرکتوری های `localsend-gui-egui`، `-fltk`، `-iced` و `-tauri` پورت های آزمایشی هستند و تا زمانی که ادغام RPC آنها کامل شود، خارج از فضای کاری root باقی می مانند.

## دانلود کنید

دارایی های انتشار از یک طرح نامگذاری پیروی می کنند — `LocalSend-<version>-<platform>-<arch>.<ext>`، با پلتفرم های `linux` / `windows` / `macos` و معماری `x86_64` / `aarch64`:

| پلت فرم | دارایی |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`، `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`، `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`، `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`، `LocalSend-0.1.3-linux-aarch64.rpm` |
| هر Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`، `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` هر دارایی را در یک نسخه پوشش می‌دهد. ساخت macOS تولید شده توسط CI دارای امضای ad-hoc است، نه شناسه توسعه‌دهنده امضا یا محضری.

## شروع سریع

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

بسته‌های Linux یک ورودی دسک‌تاپ را نصب می‌کنند که `localsend-tray` را راه‌اندازی می‌کند، که مالک چرخه حیات دیمون است. در رایانه‌های رومیزی بدون نوار وضعیت، سینی GUI را باز نگه می‌دارد. `LocalSend --launcher` آن حالت را به صراحت از AppImage انتخاب می‌کند.

### Windows

از حالت فشرده خارج کنید و `LocalSend.exe` را اجرا کنید. آرشیو GUI، دیمون، سینی و پرتابگر را در کنار یکدیگر نگه می‌دارد. آنها را با هم نگه دارید

## رفتاری که ارزش دانستن دارد

- **خروج یک عملیات است.** خروج از سینی GUI ضبط شده در `gui.pid` با محدوده پایانی را خاتمه می‌دهد، دیمون متعلق به خود را در یک بازه زمانی محدود متوقف می‌کند، و بدون پشت سر گذاشتن یک فرآیند یا یک کار حفظ زنده راه‌اندازی خارج می‌شود.
- **اعلان‌ها از سینی می‌آیند.** GUI هرگز اعلان‌های خود را پست نمی‌کند، بنابراین هر پلتفرم یک مسیر تحویل دارد.
- **وضعیت پنجره در هر نقطه پایانی است.** جایگذاری در کنار پیکربندی دیمون برای آن نقطه پایانی ذخیره می شود تا در یک مکان مشترک مشترک، بنابراین دو نمونه بر سر هندسه پنجره با هم مبارزه نمی کنند.
- **دیمون معتبر است.** تنظیمات تغییر یافته در GUI به دیمون ارسال می‌شوند و از طریق یک عکس فوری بازتاب داده می‌شوند. یک frontend هیچ کپی خصوصی را نگه نمی‌دارد که بتواند جابجا شود.

## زبان ها

رابط در 57 منطقه ارسال می شود، مطابق با فهرست بالادست LocalSend، با زبان چینی ساده شده به عنوان زبان پیام. یکی را در قسمت **تنظیمات → زبان** انتخاب کنید، یا آن را روی "دنبال کردن سیستم" بگذارید تا از محلی OS استفاده کنید.

| | |
| --- | --- |
| کاتالوگ های همراه | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| رشته های منبع UI | `@tr("...")` در `apps/localsend-gui-slint/ui` |
| کاتالوگ انگلیسی | `translations/locale-en.json` (UI)، `translations/locale-en-runtime.json` (Rust پیام‌ها) |
| پیام های طرف زنگ | `crates/localsend-i18n` (تولید شده `lookup_generated.rs`) |
| آینه بالادست | `translations/upstream/<locale>.json` |

پس از تغییر هر رشته رابط کاربری، کاتالوگ ها را بازسازی کنید:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` متعهد است، بنابراین کاتالوگ های تولید شده به صورت آفلاین قابل تکرار هستند. یک ورودی را در آنجا تصحیح کنید و مولد را مجدداً اجرا کنید تا یک ترجمه درست شود.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## ساخت از مبدا

الزامات: Rust `1.97.1` (پین شده توسط `rust-toolchain.toml`) و برای ساخت‌های Linux، هدرهای توسعه GTK3 و AppIndicator.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

بسته های پلت فرم:

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

اسکریپت های پذیرش به جای حالت مسخره شده، باینری های واقعی را هدایت می کنند:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## مستندات

| سند | مطالب |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | عملکرد، بسته بندی و رفتار پلت فرم |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | شواهد آزمایش فعلی و مناطق تأیید نشده شناخته شده |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | دروازه های پذیرش محلی |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | رگرسیون های مکرر و دنباله شواهد مورد انتظار قبل از تغییر رفتار |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | حالت همگرایی بصری در برابر نمونه اولیه OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | ماژول ریشه Android و محدودیت های آن |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | منشأ بالادستی هسته فروشنده |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

برخی از اسناد به زبان چینی نوشته شده است. آنها مراحل و قراردادهای راستی آزمایی را با کلماتی که در آنها توافق شده است ثبت می کنند.

## دامنه

محصول اصلی مشتری PC است. پشتیبانی Android یک شبح دریافت بدون سر و ریشه است تا زمانی که صراحتاً فراتر از این محدوده تبلیغ شود - هیچ GUI ندارد، هیچ سرویس پیش‌زمینه راه‌اندازی نمی‌کند و هیچ اعلان دائمی ارسال نمی‌کند. `crates/localsend-official-core` به غیر از فایل `SHA256SUMS` تولید شده‌اش، بایت به بایت مشابه صادرات بالادستی ثبت شده‌اش نگهداری می‌شود.

## اعتبارات

پروتکل، قالب سیم و هسته Rust از [localsend/localsend](https://github.com/localsend/localsend); این پروژه به جای پیاده سازی مجدد پروتکل، یک کلاینت دسکتاپ روی آن کار می سازد. با تشکر از LocalSend نویسندگان و مشارکت کنندگان.

رابط از یک نمونه اولیه طراحی توسعه یافته برای این پروژه پیروی می کند. مرجع بصری آن در [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/) زندگی می‌کند.

## کمک کردن

- **اشکالات و پیشنهادات**: یک [مسئله](https://github.com/alkaid-ops/localsend-next/issues) را با نسخه، پلتفرم، مراحل تولید مثل و آنچه دیدید باز کنید. گزارش یا اسکرین شات باعث می‌شود که سریع‌تر عمل کنید.
- **کد**: از درخواست های کشش استقبال می شود. تغییراتی که مرز daemon/frontend، هسته فروشنده، یا قرارداد بصری را لمس می‌کنند باید ابتدا مورد بحث قرار گیرند — محدودیت‌ها در [`AGENTS.md`](../../AGENTS.md) ثبت می‌شوند.
- **ترجمه**: کاتالوگ ها از حافظه نهان مترجم ماشینی متعهد تولید می شوند، بنابراین اصلاحات در آنجا ساده ترین نوع کمک برای بررسی است.

## مجوز

[Apache-2.0](../../LICENSE). اجزای شخص ثالث و مجوزهای آنها در [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md) فهرست شده است.

<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | **العربية** | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [简体中文](../../docs/zh-CN/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next هو عميل سطح مكتب متوافق مع LocalSend مكتوب باللغة Rust. إنه يتحدث البروتوكول الرسمي، لذا فهو يكتشف الملفات وينقلها باستخدام تطبيق LocalSend الرسمي في كلا الاتجاهين، أثناء تشغيله كبرنامج سطح مكتب أصلي بدلاً من تطبيق Flutter.

جانب الشبكة هو النواة الرسمية LocalSend Rust، والتي يتم بيعها دون تغيير. كل شيء فوقه - البرنامج الخفي، وطبقة RPC، والدرج، وواجهة المستخدم الطرفية، وSlint GUI - مبني حول هذا المركز.

## تجريبي

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| يرسل | إعدادات |
| --- | --- |
| ![أرسل صفحة تحتوي على مصادر الملف والمجلد والنص والحافظة والأجهزة القريبة](../../docs/images/readme-send-light.png) | ![صفحة الإعدادات مع الموضوع واللغة والشبكة وخيارات النقل](../../docs/images/readme-settings-light.png) |

## الميزات

- الاكتشاف والنقل مقابل عملاء LocalSend الرسميين، على النواة الأولية الموردة بدلاً من إعادة التنفيذ
- مالك حالة أساسي واحد: يحتفظ البرنامج الخفي بالإعدادات والهوية والمفضلات والتاريخ وعمليات النقل وهوية TLS، بحيث ترى كل واجهة أمامية نفس الشيء
- GUI، واجهة المستخدم الطرفية والدرج كواجهات أمامية مستقلة على نفس RPC المحلي؛ يمتلك الدرج البرنامج الخفي وGUI دورة حياة العملية
- التقدم في الوقت الفعلي لكل ملف، والتاريخ مع إجراءات الفتح والكشف، والمفضلات، والإدخال اليدوي للجهاز، وروابط تلقي مشاركة الويب
- 57 لغة واجهة تتطابق مع قائمة LocalSend الأولية، قابلة للتبديل في وقت التشغيل أو اتباع لغة النظام
- التكامل الأصلي لسطح المكتب: الإشعارات من الدرج فقط، والتشغيل عند تسجيل الدخول، والتصغير إلى الدرج، واستمرار وضع النافذة، ونافذة macOS يتم رسمها في شريط العنوان الخاص بها
- التعبئة والتغليف لـ macOS (DMG/ZIP)، Linux (deb، rpm، AppImage، كرة القطران) وWindows (ZIP)، لكل من x86_64 وaarch64

## بنيان

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

البرنامج الخفي هو العملية الوحيدة التي تتحدث إلى الشبكة أو تكتب حالة التطبيق. لا تعتمد الواجهات الأمامية أبدًا على النواة المُباعة ولا تنتج البرنامج الخفي أبدًا؛ الدرج يفعل. لا تنتقل وحدات بايت الملف عبر RPC — يقوم البرنامج الخفي بتحليل المسارات وتجزئة المحتوى وسجلات التاريخ.

| ثنائي | دور |
| --- | --- |
| `localsendd` | خدمة الشبكة وحالة التطبيق الأساسية |
| `localsend-tray` | Daemon وGUI مالك دورة الحياة، الإخطارات |
| `localsend-gui-slint` | الشكل الرسمي GUI، Slint مع العارض FemtoVG |
| `localsend-tui` | الواجهة الطرفية |
| `localsend-launcher` | قاذفة صريحة للبيئات التي لا تحتوي على صينية |
| `localsend-androidd` | جذر مستقل فقط Android تلقي البرنامج الخفي (MVP) |

تعد الدلائل `localsend-gui-egui` و`-fltk` و`-iced` و`-tauri` منافذ تجريبية وتظل خارج مساحة العمل الجذرية حتى يكتمل تكاملها RPC.

## تحميل

تتبع أصول الإصدار نظام تسمية واحد — `LocalSend-<version>-<platform>-<arch>.<ext>`، مع الأنظمة الأساسية `linux` / `windows` / `macos` والبنيات `x86_64` / `aarch64`:

| منصة | أصل |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`، `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`، `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`، `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`، `LocalSend-0.1.3-linux-aarch64.rpm` |
| أي Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`، `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` يغطي كل الأصول في الإصدار. الإصدار macOS الذي تم إنتاجه بواسطة CI هو موقع خاص، وليس معرف المطور موقعًا أو موثقًا.

## بداية سريعة

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

تقوم حزم Linux بتثبيت إدخال سطح المكتب الذي يقوم بتشغيل `localsend-tray`، والذي يمتلك دورة حياة البرنامج الخفي. على أجهزة سطح المكتب التي لا تحتوي على شريط حالة، يُبقي الدرج GUI مفتوحًا بدلاً من ذلك؛ `LocalSend --launcher` يحدد هذا الوضع بشكل صريح من AppImage.

### Windows

قم بفك الضغط وتشغيل `LocalSend.exe`. يحتوي الأرشيف على GUI والبرنامج الخفي والدرج والمشغل جنبًا إلى جنب؛ احتفظ بهم معًا.

## سلوك يستحق المعرفة

- **الإنهاء هو عملية واحدة.** يؤدي الخروج من الدرج إلى إنهاء GUI المسجل في نطاق نقطة النهاية `gui.pid`، وإيقاف البرنامج الخفي المملوك خلال مهلة محدودة، والخروج دون ترك عملية أو مهمة استمرار التشغيل التي تم إطلاقها.
- **تأتي الإشعارات من الدرج.** لا ينشر GUI أبدًا إشعاراته الخاصة، لذلك يكون لكل نظام أساسي مسار تسليم واحد.
- **حالة النافذة هي لكل نقطة نهاية.** يتم تخزين الموضع بجانب التكوين الخفي لنقطة النهاية هذه بدلاً من تخزينه في موقع عالمي مشترك، لذلك لا يتعارض مثيلان حول هندسة النافذة.
- **البرنامج الخفي موثوق.** يتم إرسال الإعدادات التي تم تغييرها في GUI إلى البرنامج الخفي ويتم تكرارها مرة أخرى من خلال لقطة؛ لا تحتفظ الواجهة الأمامية بنسخة خاصة يمكن أن تنجرف.

## اللغات

يتم شحن الواجهة في 57 لغة، مطابقة لقائمة LocalSend الأولية، مع اللغة الصينية المبسطة كلغة msgid. اختر واحدًا ضمن **الإعدادات → اللغة**، أو اتركه على "متابعة النظام" لاستخدام لغة نظام التشغيل.

| | |
| --- | --- |
| الكتالوجات المجمعة | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| سلاسل مصدر واجهة المستخدم | `@tr("...")` في `apps/localsend-gui-slint/ui` |
| كتالوج إنجليزي | `translations/locale-en.json` (واجهة المستخدم)، `translations/locale-en-runtime.json` (Rust الرسائل) |
| رسائل الصدأ الجانب | `crates/localsend-i18n` (تم إنشاؤه `lookup_generated.rs`) |
| مرآة المنبع | `translations/upstream/<locale>.json` |

قم بإعادة إنشاء الكتالوجات بعد تغيير أي سلسلة لواجهة المستخدم:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` ملتزم، لذلك يمكن إعادة إنتاج الكتالوجات التي تم إنشاؤها دون الاتصال بالإنترنت. قم بتصحيح الإدخال هناك وأعد تشغيل المولد لإصلاح الترجمة.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## البناء من المصدر

المتطلبات: Rust `1.97.1` (مثبت بواسطة `rust-toolchain.toml`) وبالنسبة للإصدارات Linux، رؤوس التطوير GTK3 وAppIndicator.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

حزم المنصة:

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

تعمل البرامج النصية للقبول على تشغيل الثنائيات الحقيقية بدلاً من الحالة الساخرة:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## التوثيق

| وثيقة | محتويات |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | التشغيل والتعبئة وسلوك المنصة |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | أدلة الاختبار الحالية والمناطق المعروفة التي لم يتم التحقق منها |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | بوابات القبول المحلية |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | الانحدارات المتكررة ومسار الأدلة المتوقع قبل تغيير السلوك |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | حالة التقارب البصري مقابل نموذج OpenDesign الأولي |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | وحدة الجذر Android وحدودها |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | مصدر المنبع من الأساسية بائعة |

تم نشر ملف README هذا بجميع لغات الواجهة البالغ عددها 57 لغة - اختر واحدة من أداة التحويل الموجودة في الأعلى. يتم إنشاء الترجمات من `README.md` بواسطة `scripts/translate-readme.mjs`؛ تدخل التصحيحات [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

بعض الوثائق مكتوبة باللغة الصينية. يقومون بتسجيل عمليات التحقق والاصطلاحات بالكلمات التي تم الاتفاق عليها.

## النطاق

المنتج الأساسي هو عميل الكمبيوتر الشخصي. الدعم Android عبارة عن برنامج خفي للاستقبال بدون رأس فقط، حتى يتم ترقيته بشكل صريح خارج هذا النطاق - لا يحتوي على GUI، ولا يبدأ أي خدمة مقدمة ولا ينشر أي إشعار مستمر. `crates/localsend-official-core` يتم الاحتفاظ بالبايت مقابل البايت بشكل مطابق للتصدير المسجل، بصرف النظر عن الملف `SHA256SUMS` الذي تم إنشاؤه.

## الاعتمادات

البروتوكول وتنسيق السلك والنواة Rust تأتي من [localsend/localsend](https://github.com/localsend/localsend)؛ يقوم هذا المشروع بإنشاء عميل سطح مكتب على هذا العمل بدلاً من إعادة تنفيذ البروتوكول. شكرًا للمؤلفين والمساهمين LocalSend.

تتبع الواجهة نموذجًا أوليًا للتصميم تم تطويره لهذا المشروع؛ مرجعها المرئي موجود في [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## المساهمة

- **الأخطاء والاقتراحات**: افتح [الإصدار](https://github.com/alkaid-ops/localsend-next/issues) بالإصدار والنظام الأساسي وخطوات إعادة الإنتاج وما شاهدته. السجل أو لقطة الشاشة تجعل العمل عليها أسرع بكثير.
- **الكود**: نرحب بطلبات السحب. يجب مناقشة التغييرات التي تمس حدود daemon/frontend أو النواة الموردة أو العقد المرئي أولاً - يتم تسجيل القيود في [`AGENTS.md`](../../AGENTS.md).
- **الترجمات**: يتم إنشاء الكتالوجات من ذاكرة التخزين المؤقت المخصصة للترجمة الآلية، لذا فإن التصحيحات هناك هي أسهل أنواع المساهمة للمراجعة.

## رخصة

[أباتشي-2.0](../../LICENSE). يتم إدراج مكونات الطرف الثالث وتراخيصها في [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

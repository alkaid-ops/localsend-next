<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | **עברית** | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next הוא לקוח שולחני תואם LocalSend שנכתב ב-Rust. הוא מדבר בפרוטוקול הרשמי, כך שהוא מגלה ומעביר קבצים עם האפליקציה הרשמית LocalSend בשני הכיוונים, תוך כדי הפעלה כתוכנית שולחן עבודה מקורית במקום אפליקציית Flutter.

צד הרשת הוא הליבה הרשמית LocalSend Rust, הנמכרת ללא שינוי. כל מה שמעליו - daemon, RPC שכבת, מגש, ממשק משתמש מסוף וה-Slint GUI - בנוי סביב הליבה הזו.

## הַדגָמָה

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| לִשְׁלוֹחַ | הגדרות |
| --- | --- |
| ![שלח דף עם מקורות הקובץ, התיקיה, הטקסט והלוח והמכשירים הסמוכים](../../docs/images/readme-send-light.png) | ![דף הגדרות עם ערכת נושא, שפה, רשת ואפשרויות העברה](../../docs/images/readme-settings-light.png) |

## תכונות

- גילוי והעברה מול לקוחות LocalSend רשמיים, בליבה במעלה הזרם המוכר במקום יישום מחדש
- בעל מדינה קנוני אחד: הדמון מחזיק בהגדרות, זהות, מועדפים, היסטוריה, העברות וזהות TLS, כך שכל חזית רואה את אותו הדבר
- GUI, ממשק משתמש ומגש מסוף כחזיתות עצמאיות על אותו RPC מקומי; המגש הוא הבעלים של daemon ו-GUI מחזור החיים של התהליך
- התקדמות בזמן אמת לכל קובץ, היסטוריה עם פעולות פתיחה וחשיפה, מועדפים, הזנת מכשיר ידנית וקישורי קבלת שיתוף אינטרנט
- 57 שפות ממשק תואמות את הרשימה של LocalSend במעלה הזרם, ניתנות להחלפה בזמן ריצה או לפי אזור המערכת
- אינטגרציה מקורית של שולחן העבודה: התראות מהמגש בלבד, הפעלה בעת הכניסה, צמצום למגש, התמדה במיקום החלון וחלון macOS שנמשך אל שורת הכותרת שלו
- אריזה עבור macOS (DMG/ZIP), Linux (דב, rpm, AppImage, tarball) ו-Windows (ZIP), גם עבור x86_64 ו-aarch64

## אַדְרִיכָלוּת

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

הדמון הוא התהליך היחיד שמדבר לרשת או כותב מצב יישום. ממשקי קצה לעולם אינם תלויים בליבה המוכרים ואינם מולידים את הדמון בעצמם; המגש עושה זאת. בתים של קבצים לא עוברים מעל RPC - ​​הדמון פותר נתיבים, גיבוב תוכן ומתעד היסטוריה.

| בינארי | תַפְקִיד |
| --- | --- |
| `localsendd` | שירות רשת ומצב יישום קנוני |
| `localsend-tray` | Daemon ו-GUI בעלים של מחזור החיים, התראות |
| `localsend-gui-slint` | הפורמלי GUI, Slint עם המעבד FemtoVG |
| `localsend-tui` | חזית מסוף |
| `localsend-launcher` | משגר מפורש עבור סביבות ללא מגש |
| `localsend-androidd` | קבלה עצמאית של דמון Android בשורש בלבד (MVP) |

הספריות `localsend-gui-egui`, `-fltk`, `-iced` ו-`-tauri` הן יציאות ניסיוניות ונשארות מחוץ לסביבת העבודה השורשית עד להשלמת השילוב שלהן עם RPC.

## הורד

נכסי שחרור פועלים לפי סכימת שמות אחת - `LocalSend-<version>-<platform>-<arch>.<ext>`, עם פלטפורמות `linux` / `windows` / `macos` וארכיטקטורות `x86_64` / `aarch64`:

| פּלַטפוֹרמָה | נֶכֶס |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| כל Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` מכסה כל נכס במהדורה. ה-build macOS שהופק על ידי CI חתום אד-הוק, לא זיהוי מפתח חתום או נוטריוני.

## התחלה מהירה

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

חבילות Linux מתקינות ערך בשולחן העבודה שמפעיל את `localsend-tray`, שהוא הבעלים של מחזור החיים של הדמון. במחשבים שולחניים ללא שורת מצב, המגש שומר את ה-GUI פתוח במקום זאת; `LocalSend --launcher` בוחר במצב זה במפורש מה-AppImage.

### Windows

פתח את ה-zip והפעל את `LocalSend.exe`. הארכיון מחזיק את GUI, הדמון, המגש והמשגר ​​זה לצד זה; לשמור אותם ביחד.

## התנהגות שכדאי להכיר

- **היציאה היא פעולה אחת.** יציאה מהמגש מפסיקה את ה-GUI שנרשם ב-`gui.pid` בטווח נקודת הקצה, עוצרת את הדמון שבבעלות תוך פרק זמן מוגבל, ויוצאת מבלי להשאיר מאחור תהליך או משימת שמירה על החיים שהושקה.
- **ההתראות מגיעות מהמגש.** ה-GUI לעולם לא מפרסם הודעות משלו, כך שלכל פלטפורמה יש נתיב מסירה יחיד.
- **מצב החלון הוא לכל נקודת קצה.** המיקום מאוחסן ליד תצורת הדמון עבור אותה נקודת קצה ולא במיקום גלובלי משותף, כך ששני מופעים אינם נלחמים על גיאומטריית החלון.
- **הדמון הוא סמכותי.** ההגדרות המשתנות ב-GUI נשלחות לדמון והדהוד בחזרה דרך תמונת מצב; חזית קצה לא שומר שום עותק פרטי שעלול להיסחף.

## שפות

הממשק נשלח ב-57 אזורים, תואמים לרשימה של LocalSend במעלה הזרם, עם סינית פשוטה כשפת ה-msgid. בחר אחד תחת **הגדרות ← שפה**, או השאר אותו ב"עקוב אחרי המערכת" כדי להשתמש באזור מערכת ההפעלה.

| | |
| --- | --- |
| קטלוגים מצורפים | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| מחרוזות מקור של ממשק משתמש | `@tr("...")` ב-`apps/localsend-gui-slint/ui` |
| קטלוג אנגלי | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust הודעות) |
| הודעות צד חלודה | `crates/localsend-i18n` (נוצר `lookup_generated.rs`) |
| מראה במעלה הזרם | `translations/upstream/<locale>.json` |

צור מחדש את הקטלוגים לאחר שינוי מחרוזת ממשק משתמש כלשהי:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` מחויב, כך שהקטלוגים שנוצרו ניתנים לשחזור במצב לא מקוון. תקן שם ערך והפעל מחדש את המחולל כדי לתקן תרגום.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## בנייה ממקור

דרישות: Rust `1.97.1` (מוצמד על ידי `rust-toolchain.toml`) ולגבי Linux build, כותרות הפיתוח GTK3 ו-AppIndicator.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

חבילות פלטפורמה:

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

סקריפטים של קבלה מניעים את הקבצים הבינאריים האמיתיים במקום מצב לעג:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## תיעוד

| מִסְמָך | תוֹכֶן |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | תפעול, אריזה והתנהגות פלטפורמה |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | עדויות בדיקה עדכניות ואזורים לא מאומתים ידועים |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | שערי קבלה מקומיים |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | רגרסיות חוזרות ושובל העדויות הצפוי לפני שינוי התנהגות |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | מצב התכנסות חזותית מול אב הטיפוס של OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | מודול השורש Android ומגבלותיו |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | מקורו במעלה הזרם של הליבה הנמכרת |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

חלק מהמסמכים כתובים בסינית; הם מתעדים ריצות אימות ומוסכמות במילים שהן הסכימו בהן.

## היקף

המוצר העיקרי הוא לקוח המחשב. התמיכה ב-Android היא דמון קבלה ללא ראש בלבד עד לקידום מפורש מעבר לטווח זה - אין לו GUI, אין לו שירות קדמה ואינו מפרסם הודעה מתמשכת. `crates/localsend-official-core` נשמר בייט-עבור-בייט זהה לייצוא המוקלט שלו במעלה הזרם, מלבד הקובץ `SHA256SUMS` שנוצר.

## קרדיטים

הפרוטוקול, פורמט החוט והליבת Rust מגיעים מ-[localsend/localsend](https://github.com/localsend/localsend); הפרויקט הזה בונה לקוח שולחני על העבודה הזו במקום ליישם מחדש את הפרוטוקול. תודה לכותבים ולתורמים LocalSend.

הממשק עוקב אחר אב טיפוס עיצובי שפותח עבור פרויקט זה; ההתייחסות החזותית שלו חיה ב-[`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## תורם

- **באגים והצעות**: פתחו [בעיה](https://github.com/alkaid-ops/localsend-next/issues) עם הגרסה, הפלטפורמה, שלבי השחזור ומה שראיתם. יומן או צילום מסך עושים את זה הרבה יותר מהיר לפעול.
- **קוד**: בקשות משיכה יתקבלו בברכה. יש לדון תחילה בשינויים הנוגעים בגבול daemon/frontend, הליבה של הספק או בחוזה החזותי - האילוצים נרשמים ב-[`AGENTS.md`](../../AGENTS.md).
- **תרגום**: הקטלוגים נוצרים ממטמון תרגום המכונה המחויב, כך שתיקונים שם הם התרומה הקלה ביותר לבדיקה.

## רִשָׁיוֹן

[Apache-2.0](../../LICENSE). רכיבי צד שלישי ורישיונותיהם מפורטים ב-[THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

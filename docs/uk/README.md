<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | **Українська** | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next — сумісний із LocalSend настільний клієнт, написаний мовою Rust. Він розмовляє офіційним протоколом, тому виявляє та передає файли за допомогою офіційної програми LocalSend в обох напрямках, водночас працює як рідна програма для робочого столу замість програми Flutter.

Мережева сторона — це офіційне ядро ​​LocalSend Rust, постачальник якого не змінився. Усе, що над ним — демон, RPC рівень, трей, інтерфейс терміналу та Slint GUI — побудовано навколо цього ядра.

## Демо

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Надіслати | Налаштування |
| --- | --- |
| ![Надіслати сторінку з джерелами файлу, папки, тексту та буфера обміну та пристроями поблизу](../../docs/images/readme-send-light.png) | ![Сторінка налаштувань із темою, мовою, мережею та параметрами передачі](../../docs/images/readme-settings-light.png) |

## Можливості

- Виявлення та передача на офіційних LocalSend клієнтах, на вищому ядрі постачальника, а не повторне впровадження
- Один канонічний державний власник: демон зберігає налаштування, ідентифікаційні дані, вибране, історію, перекази та ідентифікаційні дані TLS, тому кожен зовнішній інтерфейс бачить те саме
- GUI, інтерфейс терміналу та трей як незалежні інтерфейси над тим самим локальним RPC; трей володіє демоном і життєвим циклом процесу GUI
- Прогрес у реальному часі для кожного файлу, історія з відкриттям і розкриттям дій, вибране, введення пристрою вручну та посилання на отримання Web Share
- 57 мов інтерфейсу, які відповідають попередньому списку LocalSend, перемикаються під час виконання або відповідно до локалі системи
- Вбудована інтеграція з настільним комп’ютером: сповіщення лише з трею, запуск під час входу, згортання в трей, постійне розміщення вікна та вікно macOS, яке відображатиметься в рядку заголовка
- Упаковка для macOS (DMG/@ZIP), Linux (deb, rpm, AppImage, tarball) і Windows (ZIP), для x86_64 і aarch64

## Архітектура

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Демон — це єдиний процес, який спілкується з мережею або записує стан програми. Інтерфейси ніколи не залежать від ядра постачальника і ніколи не породжують самих демонов; лоток робить. Файлові байти не переміщуються через RPC — демон розпізнає шляхи, хешує вміст і записує історію.

| Двійковий | роль |
| --- | --- |
| `localsendd` | Мережевий сервіс і стан канонічної програми |
| `localsend-tray` | Демон і власник життєвого циклу GUI, сповіщення |
| `localsend-gui-slint` | Формальний GUI, Slint із FemtoVG рендерером |
| `localsend-tui` | Інтерфейс терміналу |
| `localsend-launcher` | Явний запуск для середовищ без трея |
| `localsend-androidd` | Автономний демон отримання тільки для root Android (MVP) |

Каталоги `localsend-gui-egui`, `-fltk`, `-iced` і `-tauri` є експериментальними портами і залишаються за межами кореневої робочої області, доки не завершиться їхня інтеграція RPC.

## Завантажити

Активи випуску мають одну схему імен — `LocalSend-<version>-<platform>-<arch>.<ext>`, з платформами `linux` / `windows` / `macos` та архітектурами `x86_64` / `aarch64`:

| Платформа | Актив |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Будь-який Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` охоплює всі активи у випуску. Збірка macOS, створена CI, підписана спеціально, а не ідентифікатором розробника або нотаріально завіреною.

## Швидкий старт

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

Пакунки Linux встановлюють запис на робочому столі, який запускає `localsend-tray`, який володіє життєвим циклом демона. На настільних комп’ютерах без рядка стану трей тримає відкритим GUI; `LocalSend --launcher` вибирає цей режим явно з AppImage.

### Windows

Розпакуйте та запустіть `LocalSend.exe`. Архів містить GUI, демон, трей і панель запуску поруч; тримати їх разом.

## Поведінка, яку варто знати

- **Вийти — це одна операція.** Вихід із трея припиняє роботу GUI, записану в області дії кінцевої точки `gui.pid`, зупиняє належний демон протягом обмеженого часу очікування та завершує роботу, не залишаючи процесу чи завдання launchd keep-alive.
- **Сповіщення надходять із трею.** GUI ніколи не публікує власні сповіщення, тому кожна платформа має єдиний шлях доставки.
- **Стан вікна залежить від кінцевої точки.** Розташування зберігається поруч із конфігурацією демона для цієї кінцевої точки, а не в спільному глобальному розташуванні, тому два екземпляри не борються за геометрію вікна.
- **Демон є повноважним.** Параметри, змінені в GUI, надсилаються демону та повертаються через знімок; інтерфейс не зберігає жодної приватної копії, яка може дрейфувати.

## Мови

Інтерфейс поставляється в 57 локалях, що відповідають попередньому списку LocalSend, зі спрощеною китайською як мовою msgid. Виберіть один у меню **Налаштування → Мова** або залиште його на «Відстежувати систему», щоб використовувати мову ОС.

| | |
| --- | --- |
| Збірні каталоги | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| Вихідні рядки інтерфейсу користувача | `@tr("...")` в `apps/localsend-gui-slint/ui` |
| Англійський каталог | `translations/locale-en.json` (інтерфейс користувача), `translations/locale-en-runtime.json` (Rust повідомлення) |
| Повідомлення зі сторони Rust | `crates/localsend-i18n` (згенеровано `lookup_generated.rs`) |
| Верхнє дзеркало | `translations/upstream/<locale>.json` |

Відновіть каталоги після зміни будь-якого рядка інтерфейсу користувача:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` закріплено, тому згенеровані каталоги можна відтворювати в автономному режимі. Виправте там запис і повторно запустіть генератор, щоб виправити переклад.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Будівництво з джерела

Вимоги: Rust `1.97.1` (закріплено `rust-toolchain.toml`) і, для збірок Linux, заголовки розробки GTK3 і AppIndicator.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Пакети платформи:

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

Сценарії прийняття керують справжніми двійковими файлами замість імітованого стану:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Документація

| документ | Зміст |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Робота, упаковка та поведінка платформи |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Поточні докази випробувань і відомі неперевірені області |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Місцеві приймальні ворота |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Повторювані регресії та слід очікуваних доказів перед зміною поведінки |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Стан візуальної конвергенції щодо прототипу OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Кореневий модуль Android і його обмеження |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Вихідне походження ядра постачальника |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Деякі документи написані китайською мовою; вони записують верифікаційні цикли та домовленості словами, якими вони були погоджені.

## Обсяг

Основним продуктом є ПК-клієнт. Підтримка Android — це демон безголового отримання лише для кореневого доступу, доки його явно не буде виведено за межі цього діапазону — він не має GUI, не запускає жодної служби переднього плану та не публікує постійних сповіщень. `crates/localsend-official-core` зберігається побайтно ідентичним до записаного вихідного експорту, окрім згенерованого файлу `SHA256SUMS`.

## Кредити

Протокол, формат дроту та ядро ​​Rust походять від [localsend/localsend](https://github.com/localsend/localsend); цей проект будує настільний клієнт на основі цієї роботи, а не повторно впроваджує протокол. Дякуємо LocalSend авторам і учасникам.

Інтерфейс відповідає прототипу дизайну, розробленому для цього проекту; його візуальне посилання знаходиться в [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Сприяння

- **Помилки та пропозиції**: відкрийте [проблему](https://github.com/alkaid-ops/localsend-next/issues) із версією, платформою, кроками відтворення та тим, що ви бачили. Журнал або знімок екрана значно пришвидшують дії.
- **Код**: вітаються запити на отримання. Зміни, які торкаються межі daemon/frontend, ядра постачальника або візуального контракту, слід обговорити спочатку — обмеження записуються в [`AGENTS.md`](../../AGENTS.md).
- **Переклади**: каталоги створюються з кешу машинного перекладу, тому виправлення є найпростішим видом внеску для перегляду.

## Ліцензія

[Apache-2.0](../../LICENSE). Сторонні компоненти та їхні ліцензії перераховані в [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

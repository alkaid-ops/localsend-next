<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | **Български** | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next е съвместим с LocalSend настолен клиент, написан на Rust. Говори официалния протокол, така че открива и прехвърля файлове с официалното приложение LocalSend в двете посоки, като същевременно работи като естествена настолна програма вместо приложение Flutter.

Страната на мрежата е официалното LocalSend Rust ядро, продавано непроменено. Всичко над него — демон, RPC слой, трей, терминален потребителски интерфейс и Slint GUI — е изградено около това ядро.

## Демо

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Изпратете | Настройки |
| --- | --- |
| ![Изпращане на страница с източници на файл, папка, текст и клипборд и близки устройства](../../docs/images/readme-send-light.png) | ![Страница с настройки с опции за тема, език, мрежа и прехвърляне](../../docs/images/readme-settings-light.png) |

## Функции

- Откриване и прехвърляне срещу официални LocalSend клиенти, на доставено ядро ​​нагоре по веригата, вместо повторно внедряване
- Един каноничен собственик на състояние: демонът държи настройки, идентичност, любими, история, трансфери и TLS идентичност, така че всеки интерфейс вижда едно и също нещо
- GUI, терминален потребителски интерфейс и трей като независими интерфейси над един и същ локален RPC; тавата притежава демон и GUI жизнения цикъл на процеса
- Напредък в реално време за всеки файл, история с действия за отваряне и разкриване, любими, ръчно въвеждане на устройство и връзки за получаване на уеб споделяне
- 57 интерфейсни езика, съответстващи на списъка на LocalSend нагоре по веригата, превключваеми по време на изпълнение или следвайки системния локал
- Вградена интеграция на работния плот: известия само от лентата, стартиране при влизане, минимизиране в област, постоянство на разположението на прозореца и macOS прозорец, който се изтегля в заглавната лента
- Опаковка за macOS (DMG/@ZIP), Linux (deb, rpm, AppImage, tarball) и Windows (ZIP), както за x86_64, така и за aarch64

## Архитектура

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Демонът е единственият процес, който говори с мрежата или записва състоянието на приложението. Frontends никога не зависят от ядрото на доставчика и никога не създават самия демон; тавата прави. Файловите байтове не преминават през RPC — демонът разрешава пътища, хеширва съдържание и записва хронология.

| Двоичен | Роля |
| --- | --- |
| `localsendd` | Мрежова услуга и състояние на канонично приложение |
| `localsend-tray` | Демон и GUI собственик на жизнения цикъл, известия |
| `localsend-gui-slint` | Официалният GUI, Slint с FemtoVG рендър |
| `localsend-tui` | Терминален интерфейс |
| `localsend-launcher` | Явен стартер за среди без тава |
| `localsend-androidd` | Самостоятелен root-само Android демон за получаване (MVP) |

Директориите `localsend-gui-egui`, `-fltk`, `-iced` и `-tauri` са експериментални портове и остават извън основното работно пространство, докато тяхната RPC интеграция не завърши.

## Изтегляне

Активите за издаване следват една схема за именуване — `LocalSend-<version>-<platform>-<arch>.<ext>`, с платформи `linux` / `windows` / `macos` и архитектури `x86_64` / `aarch64`:

| Платформа | Актив |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Всеки Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` обхваща всеки актив в издание. Компилацията macOS, произведена от CI, е подписана ad hoc, не е подписан или нотариално заверен с ID на програмиста.

## Бърз старт

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

Linux пакетите инсталират десктоп запис, който стартира `localsend-tray`, който притежава жизнения цикъл на демона. На настолни компютри без лента на състоянието тавата държи GUI отворен вместо това; `LocalSend --launcher` избира този режим изрично от AppImage.

### Windows

Разархивирайте и стартирайте `LocalSend.exe`. Архивът съдържа GUI, демона, трея и стартовия панел един до друг; дръжте ги заедно.

## Поведение, което си струва да знаете

- **Изходът е една операция.** Излизането от трея прекратява GUI, записан в обхвата на крайната точка `gui.pid`, спира притежавания демон в рамките на ограничено време за изчакване и излиза, без да оставя процес или стартирана задача за поддържане на активност.
- **Известията идват от панела.** GUI никога не публикува свои собствени известия, така че всяка платформа има един единствен път за доставка.
- **Състоянието на прозореца е за крайна точка.** Разположението се съхранява до конфигурацията на демон за тази крайна точка, а не в споделено глобално местоположение, така че два екземпляра не се борят за геометрията на прозореца.
- **Демонът е авторитетен.** Настройките, променени в GUI, се предават на демона и се повтарят обратно чрез моментна снимка; фронтендът не пази лично копие, което може да се промени.

## Езици

Интерфейсът се доставя в 57 локализации, съответстващи на списъка на LocalSend нагоре, с опростен китайски като език на msgid. Изберете един от **Настройки → Език** или го оставете на „Следване на системата“, за да използвате локала на ОС.

| | |
| --- | --- |
| Пакетни каталози | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| Изходни низове на потребителския интерфейс | `@tr("...")` в `apps/localsend-gui-slint/ui` |
| Английски каталог | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust съобщения) |
| Съобщения от страна на ръждата | `crates/localsend-i18n` (генериран `lookup_generated.rs`) |
| Огледало нагоре по течението | `translations/upstream/<locale>.json` |

Повторно генериране на каталозите след промяна на който и да е низ от потребителския интерфейс:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` е ангажиран, така че генерираните каталози могат да се възпроизвеждат офлайн. Коригирайте запис там и стартирайте отново генератора, за да коригирате превода.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Изграждане от източник

Изисквания: Rust `1.97.1` (закрепено от `rust-toolchain.toml`) и, за Linux компилации, GTK3 и AppIndicator заглавки за разработка.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Платформени пакети:

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

Скриптовете за приемане управляват истинските двоични файлове вместо подигравателно състояние:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Документация

| Документ | Съдържание |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Работа, опаковка и поведение на платформата |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Текущи доказателства от тестове и известни непроверени области |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Местни врати за приемане |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Повтарящи се регресии и следите от доказателства, очаквани преди промяна на поведението |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Състояние на визуална конвергенция спрямо прототипа на OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Основният модул Android и неговите ограничения |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Произход нагоре по веригата на доставеното ядро |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Някои документи са написани на китайски; те записват проверки и конвенции с думите, с които са договорени.

## Обхват

Основният продукт е компютърният клиент. Android поддръжката е демон за получаване само на root, без глава, докато изрично не бъде повишен извън този обхват — няма GUI, не стартира услуга на преден план и не публикува постоянно известие. `crates/localsend-official-core` се поддържа байт по байт идентичен със записания експорт нагоре, с изключение на генерирания `SHA256SUMS` файл.

## Кредити

Протоколът, форматът на кабела и Rust ядрото идват от [localsend/localsend](https://github.com/localsend/localsend); този проект изгражда десктоп клиент върху тази работа, вместо повторно внедряване на протокола. Благодаря на LocalSend авторите и сътрудниците.

Интерфейсът следва прототип на дизайн, разработен за този проект; неговата визуална препратка живее в [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Принос

- **Грешки и предложения**: отворете [проблем](https://github.com/alkaid-ops/localsend-next/issues) с версията, платформата, стъпките за възпроизвеждане и това, което сте видели. Дневник или екранна снимка прави действието много по-бързо.
- **Код**: заявките за изтегляне са добре дошли. Промените, които засягат границата daemon/frontend, ядрото на доставчика или визуалния договор, трябва да бъдат обсъдени първо — ограниченията се записват в [`AGENTS.md`](../../AGENTS.md).
- **Преводи**: каталозите се генерират от ангажирания кеш за машинен превод, така че корекциите там са най-лесният вид принос за преглед.

## Лиценз

[Apache-2.0](../../LICENSE). Компонентите на трети страни и техните лицензи са изброени в [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

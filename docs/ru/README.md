<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | **Русский** | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next — это настольный клиент, совместимый с LocalSend, написанный на Rust. Он использует официальный протокол, поэтому обнаруживает и передает файлы с помощью официального приложения LocalSend в обоих направлениях, работая при этом как собственная настольная программа, а не как приложение Flutter.

Сетевая часть — это официальное ядро ​​LocalSend Rust, поставляемое без изменений. Все, что находится над ним — демон, слой RPC, лоток, пользовательский интерфейс терминала и Slint GUI — построено вокруг этого ядра.

## Демо

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Отправлять | Настройки |
| --- | --- |
| ![Отправить страницу с исходными файлами, папками, текстом и буфером обмена, а также устройствами поблизости](../../docs/images/readme-send-light.png) | ![Страница настроек с темой, языком, сетью и параметрами передачи](../../docs/images/readme-settings-light.png) |

## Возможности

- Обнаружение и передача официальным клиентам LocalSend на поставляемом вышестоящем ядре, а не на повторной реализации.
- Один канонический владелец состояния: демон хранит настройки, идентификаторы, избранное, историю, передачи и идентификатор TLS, поэтому каждый интерфейс видит одно и то же.
- GUI, пользовательский интерфейс терминала и трей как независимые интерфейсы над одним и тем же локальным RPC; лоток владеет демоном и жизненным циклом процесса GUI
- Прогресс каждого файла в реальном времени, история с действиями по открытию и показу, избранное, ручной ввод устройства и ссылки на получение веб-ресурса.
- 57 языков интерфейса, соответствующих списку исходного LocalSend, переключаемых во время выполнения или в соответствии с языковым стандартом системы.
- Встроенная интеграция с рабочим столом: уведомления только из панели задач, запуск при входе в систему, свертывание в область уведомлений, сохранение размещения окон и окно macOS, которое отображается в строке заголовка.
- Упаковка для macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) и Windows (ZIP), как для x86_64, так и aarch64

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

Демон — единственный процесс, который общается с сетью или записывает состояние приложения. Интерфейсы никогда не зависят от ядра поставщика и никогда не порождают демона; лоток делает. Байты файла не перемещаются по RPC — демон разрешает пути, хеширует содержимое и записывает историю.

| Двоичный | Роль |
| --- | --- |
| `localsendd` | Состояние сетевой службы и канонического приложения |
| `localsend-tray` | Демон и владелец жизненного цикла GUI, уведомления |
| `localsend-gui-slint` | Формальные GUI, Slint с рендерером FemtoVG |
| `localsend-tui` | Интерфейс терминала |
| `localsend-launcher` | Явный лаунчер для сред без трея |
| `localsend-androidd` | Автономный демон Android получения только для root (MVP) |

Каталоги `localsend-gui-egui`, `-fltk`, `-iced` и `-tauri` являются экспериментальными портами и остаются за пределами корневого рабочего пространства до завершения их интеграции RPC.

## Скачать

Ресурсы выпуска следуют одной схеме именования — `LocalSend-<version>-<platform>-<arch>.<ext>`, с платформами `linux` / `windows` / `macos` и архитектурами `x86_64` / `aarch64`:

| Платформа | Объект |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Любой Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` охватывает все ресурсы релиза. Сборка macOS, созданная CI, подписывается специально, а не подписывается или нотариально заверяется идентификатором разработчика.

## Быстрый старт

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

Пакеты Linux устанавливают запись на рабочем столе, которая запускает `localsend-tray`, которому принадлежит жизненный цикл демона. На настольных компьютерах без строки состояния в трее вместо этого остается открытым GUI; `LocalSend --launcher` выбирает этот режим явно из AppImage.

### Windows

Разархивируйте и запустите `LocalSend.exe`. В архиве рядом лежат GUI, демон, трей и лаунчер; держите их вместе.

## Особенности поведения

- **Выход — это одна операция.** Выход из лотка завершает GUI, записанный в `gui.pid` в области конечной точки, останавливает принадлежащий демон в течение ограниченного времени ожидания и завершает работу, не оставляя процесс или задание поддержки активности запуска.
- **Уведомления приходят из трея.** GUI никогда не публикует собственные уведомления, поэтому каждая платформа имеет один путь доставки.
- **Состояние окна определяется для каждой конечной точки.** Размещение хранится рядом с конфигурацией демона для этой конечной точки, а не в общем глобальном расположении, поэтому два экземпляра не конфликтуют из-за геометрии окна.
- **Демон является авторитетным.** Настройки, измененные в GUI, передаются демону и возвращаются обратно через снимок; во внешнем интерфейсе нет частной копии, которая могла бы ускользнуть.

## Языки

Интерфейс поставляется в 57 языковых стандартах, соответствующих списку исходной версии LocalSend, с упрощенным китайским языком в качестве языка msgid. Выберите один из вариантов в разделе **Настройки → Язык** или оставьте значение «Следовать за системой», чтобы использовать языковой стандарт ОС.

| | |
| --- | --- |
| Комплектные каталоги | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| Исходные строки пользовательского интерфейса | `@tr("...")` в `apps/localsend-gui-slint/ui` |
| Английский каталог | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust сообщения) |
| Сообщения на стороне Rust | `crates/localsend-i18n` (генерируется `lookup_generated.rs`) |
| Восходящее зеркало | `translations/upstream/<locale>.json` |

Восстановите каталоги после изменения любой строки пользовательского интерфейса:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` зафиксирован, поэтому созданные каталоги можно воспроизводить в автономном режиме. Исправьте там запись и перезапустите генератор, чтобы исправить перевод.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Сборка из исходников

Требования: Rust `1.97.1` (закреплено `rust-toolchain.toml`) и для сборок Linux заголовки разработки GTK3 и AppIndicator.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Пакеты платформы:

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

Скрипты приемки управляют настоящими двоичными файлами, а не макетируемым состоянием:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Документация

| Документ | Содержание |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Эксплуатация, упаковка и поведение платформы |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Текущие данные испытаний и известные непроверенные области |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Местные приемные ворота |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Повторяющиеся регрессии и ожидаемые доказательства перед изменением поведения |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Состояние визуальной конвергенции относительно прототипа OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Корневой модуль Android и его ограничения |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Источник исходного ядра поставщика |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Некоторые документы написаны на китайском языке; они записывают прогоны проверок и соглашения в тех словах, в которых они были согласованы.

## Область применения

Основной продукт — клиент для ПК. Поддержка Android — это демон безголового получения только с root-доступом, пока он не будет явно продвинут за пределы этой области — он не имеет GUI, не запускает службы переднего плана и не отправляет постоянных уведомлений. `crates/localsend-official-core` сохраняется побайтово идентично записанному восходящему экспорту, за исключением сгенерированного файла `SHA256SUMS`.

## Благодарности

Протокол, формат проводной связи и ядро ​​Rust взяты из [localsend/localsend](https://github.com/localsend/localsend); этот проект создает настольный клиент на основе этой работы, а не на повторной реализации протокола. Спасибо LocalSend авторам и участникам.

Интерфейс соответствует прототипу, разработанному для этого проекта; его визуальная ссылка находится в [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Участие в проекте

- **Ошибки и предложения**: откройте [проблему](https://github.com/alkaid-ops/localsend-next/issues) с указанием версии, платформы, этапов воспроизведения и того, что вы увидели. Журнал или снимок экрана значительно ускоряют работу.
- **Код**: запросы на включение приветствуются. Изменения, затрагивающие границу daemon/frontend, ядро ​​поставщика или визуальный контракт, должны обсуждаться в первую очередь — ограничения записываются в [`AGENTS.md`](../../AGENTS.md).
- **Переводы**: каталоги создаются из выделенного кэша машинного перевода, поэтому исправления в них легче всего проверить.

## Лицензия

[Apache-2.0](../../LICENSE). Сторонние компоненты и их лицензии перечислены в [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

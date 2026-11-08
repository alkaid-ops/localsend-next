<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | **Српски** | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [简体中文](../../docs/zh-CN/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next је десктоп клијент компатибилан са ЛоцалСендом написан на Rust. Говори званичним протоколом, тако да открива и преноси датотеке са званичном LocalSend апликацијом у оба смера, док ради као изворни десктоп програм уместо Флуттер апликације.

Мрежна страна је званично LocalSend Rust језгро, испоручено непромењено. Све изнад њега — демон, слој RPC, траи, терминал УИ и Slint GUI — изграђено је око тог језгра.

## Демо

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Пошаљи | Подешавања |
| --- | --- |
| ![Пошаљи страницу са фајлом, фолдером, текстом и изворима међуспремника и оближњим уређајима](../../docs/images/readme-send-light.png) | ![Страница са подешавањима са темом, језиком, мрежом и опцијама преноса](../../docs/images/readme-settings-light.png) |

## Функције

- Откривање и трансфер против званичних LocalSend клијената, на испорученом језгру узводно, а не поновна имплементација
- Један канонски власник државе: демон држи подешавања, идентитет, фаворите, историју, трансфере и TLS идентитет, тако да сваки фронтенд види исту ствар
- GUI, корисничко сучеље терминала и траи као независни фронтендови преко истог локалног RPC; треј поседује демон и GUI животни циклус процеса
- Напредак по фајлу у реалном времену, историја са радњама отварања и откривања, фаворити, ручни унос уређаја и Веб дељење линкови за примање
- 57 језика интерфејса који се поклапају са горњом LocalSend листом, који се могу мењати током рада или пратећи локализацију система
- Интеграција матичне радне површине: обавештења само са траке, покретање при пријављивању, минимизирање у траку, постојаност постављања прозора и прозор macOS који се увлачи у своју насловну траку
- Паковање за macOS (DMG/@@ZIP), Linux (деб, рпм, AppImage, тарбалл) и Windows (ZIP), и за x86_64 и aarch64

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

Демон је једини процес који разговара са мрежом или пише стање апликације. Фронтендови никада не зависе од испорученог језгра и никада не покрећу сами демона; послужавник ради. Бајтови датотеке не путују преко RPC — демон решава путање, хешује садржај и бележи историју.

| Бинарно | Улога |
| --- | --- |
| `localsendd` | Мрежни сервис и канонско стање апликације |
| `localsend-tray` | Власник животног циклуса Даемон и GUI, обавештења |
| `localsend-gui-slint` | Формални GUI, Slint са FemtoVG приказивачем |
| `localsend-tui` | Терминал фронтенд |
| `localsend-launcher` | Експлицитни покретач за окружења без лежишта |
| `localsend-androidd` | Самостални роот само Android примајући демон (МВП) |

Директоријуми `localsend-gui-egui`, `-fltk`, `-iced` и `-tauri` су експериментални портови и остају изван основног радног простора док се њихова RPC интеграција не заврши.

## Преузми

Средства издања прате једну шему именовања — `LocalSend-<version>-<platform>-<arch>.<ext>`, са платформама `linux` / `windows` / `macos` и архитектурама `x86_64` / `aarch64`:

| Платформа | Ассет |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Било који Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` покрива свако средство у издању. macOS буилд који производи ЦИ је ад хоц потписан, а не ИД програмера потписан или оверен.

## Брзи почетак

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

Linux пакети инсталирају унос на радној површини који покреће `localsend-tray`, који поседује животни циклус демона. На рачунарима без статусне траке, трака уместо тога држи отвореним GUI; `LocalSend --launcher` експлицитно бира тај режим из AppImage.

### Windows

Распакујте и покрените `LocalSend.exe`. Архива садржи GUI, демон, ладицу и покретач један поред другог; држати их заједно.

## Понашање вредно познавања

- **Прекини је једна операција.** Излазак из траке завршава GUI снимљен у `gui.pid` са опсегом крајње тачке, зауставља демон у власништву унутар ограниченог временског ограничења и излази без остављања процеса или покренутог задатка одржавања.
- **Обавештења долазе из траке.** GUI никада не објављује сопствена обавештења, тако да свака платформа има једну путању испоруке.
- **Стање прозора је по крајњој тачки.** Положај се чува поред конфигурације демона за ту крајњу тачку, а не на дељеној глобалној локацији, тако да се две инстанце не боре око геометрије прозора.
- **Демон је ауторитативан.** Подешавања измењена у GUI се шаљу демону и враћају се кроз снимак; фронтенд не чува приватну копију која би могла да се креће.

## Језици

Интерфејс се испоручује на 57 локалитета, који одговарају упстреам LocalSend листи, са поједностављеним кинеским као мсгстр језиком. Изаберите један под **Подешавања → Језик** или га оставите на „Прати систем“ да бисте користили локализацију ОС-а.

| | |
| --- | --- |
| Каталози у пакету | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| Изворни стрингови корисничког интерфејса | `@tr("...")` у `apps/localsend-gui-slint/ui` |
| енглески каталог | `translations/locale-en.json` (УИ), `translations/locale-en-runtime.json` (Rust поруке) |
| Поруке на страни хрђе | `crates/localsend-i18n` (генерисано `lookup_generated.rs`) |
| Узводно огледало | `translations/upstream/<locale>.json` |

Поново генеришите каталоге након промене било ког корисничког низа:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` је посвећен, тако да се генерисани каталози могу репродуковати ван мреже. Исправите унос и поново покрените генератор да поправите превод.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Зграда из извора

Захтеви: Rust `1.97.1` (закачено са `rust-toolchain.toml`) и, за Linux градње, развојна заглавља GTK3 и AppIndicator.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Пакети платформе:

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

Скрипте за прихватање покрећу праве бинарне датотеке уместо исмејаног стања:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Документација

| Документ | Садржај |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Рад, паковање и понашање платформе |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Тренутни докази испитивања и позната непроверена подручја |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Локалне прихватне капије |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Понављајуће регресије и траг доказа који се очекује пре промене понашања |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Стање визуелне конвергенције у односу на ОпенДесигн прототип |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Android роот модул и његова ограничења |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Узводно порекло продаваног језгра |

Овај РЕАДМЕ је објављен на свих 57 језика интерфејса — изаберите један са прекидача на врху. Преводи су генерисани од `README.md` од стране `scripts/translate-readme.mjs`; исправке иду у [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Неки документи су написани на кинеском; они бележе верификационе поступке и конвенције речима у којима су се договорили.

## Обим

Примарни производ је ПЦ клијент. Подршка Android је само роот, безглави демон примања док се експлицитно не унапреди изван тог опсега — нема GUI, не покреће услугу у првом плану и не објављује стална обавештења. `crates/localsend-official-core` се чува бајт по бајт идентичан његовом снимљеном узводном извозу, осим генерисане датотеке `SHA256SUMS`.

## Цредитс

Протокол, формат жице и Rust језгро долазе од [localsend/localsend](https://github.com/localsend/localsend); овај пројекат гради десктоп клијент на том послу уместо да поново имплементира протокол. Хвала LocalSend ауторима и сарадницима.

Интерфејс прати прототип дизајна развијен за овај пројекат; његова визуелна референца живи у [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Допринос

- **Грешке и предлози**: отворите [проблем](https://github.com/alkaid-ops/localsend-next/issues) са верзијом, платформом, корацима репродукције и оним што сте видели. Евиденција или снимак екрана чини много бржим поступање по њима.
- **Шифра**: захтеви за повлачење су добродошли. Прво треба разговарати о променама које додирују границу daemon/frontend, језгро добављача или визуелни уговор — ограничења су забележена у [`AGENTS.md`](../../AGENTS.md).
- **Преводи**: каталози се генеришу из предатог кеша машинског превода, тако да су исправке најлакша врста доприноса за преглед.

## Лиценца

[Апацхе-2.0](../../LICENSE). Компоненте трећих страна и њихове лиценце су наведене у [ТХИРД_ПАРТИ_НОТИЦЕС.мд](../../THIRD_PARTY_NOTICES.md).

<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | **Беларуская** | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [简体中文](../../docs/zh-CN/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next - ​​сумяшчальны з LocalSend настольны кліент, напісаны на Rust. Ён размаўляе на афіцыйным пратаколе, таму выяўляе і перадае файлы з дапамогай афіцыйнага прыкладання LocalSend у абодвух напрамках, пры гэтым працуе як родная праграма для працоўнага стала замест праграмы Flutter.

Сеткавы бок - гэта афіцыйнае ядро ​​LocalSend Rust, пастаўшчык якога не змяніўся. Усё, што над ім — дэман, RPC слой, трэй, інтэрфейс тэрмінала і Slint GUI — пабудавана вакол гэтага ядра.

## Дэманстрацыя

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Адправіць | Налады |
| --- | --- |
| ![Адправіць старонку з крыніцамі файла, папкі, тэксту і буфера абмену і прыладамі паблізу](../../docs/images/readme-send-light.png) | ![Старонка налад з тэмай, мовай, сеткай і параметрамі перадачы](../../docs/images/readme-settings-light.png) |

## Асаблівасці

- Выяўленне і перадача супраць афіцыйных кліентаў LocalSend, на пастаўшчыком вышэйшага ядра, а не паўторная рэалізацыя
- Адзін кананічны дзяржаўны ўладальнік: дэман захоўвае налады, ідэнтыфікацыйныя дадзеныя, абранае, гісторыю, перадачы і ідэнтыфікацыйныя дадзеныя TLS, таму кожны інтэрфейс бачыць адно і тое ж
- GUI, карыстальніцкі інтэрфейс тэрмінала і трэй як незалежныя інтэрфейсы праз адзін і той жа лакальны RPC; трэй валодае дэманам і жыццёвым цыклам працэсу GUI
- Прагрэс кожнага файла ў рэальным часе, гісторыя з дзеяннямі адкрыцця і выяўлення, абранае, увод прылады ўручную і спасылкі для атрымання Web Share
- 57 моў інтэрфейсу, якія адпавядаюць спісу LocalSend, якія можна пераключаць падчас выканання або згодна з лакальлю сістэмы
- Уласная інтэграцыя працоўнага стала: апавяшчэнні толькі з трэя, запуск пры ўваходзе ў сістэму, згортванне ў трэй, пастаяннае размяшчэнне вокнаў і акно macOS, якое малюецца ў радку загалоўка
- Упакоўка для macOS (DMG/@ZIP), Linux (deb, rpm, AppImage, tarball) і Windows (ZIP), для x86_64 і aarch64

## Архітэктура

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Дэман - адзіны працэс, які размаўляе з сеткай або запісвае стан прыкладання. Фронтэнды ніколі не залежаць ад ядра пастаўшчыка і ніколі не спараджаюць дэмана самі; латок робіць. Байты файла не перамяшчаюцца праз RPC — дэман вызначае шляхі, хэшуе змесціва і запісвае гісторыю.

| Двайковы | Роля |
| --- | --- |
| `localsendd` | Сеткавы сэрвіс і стан кананічнага прыкладання |
| `localsend-tray` | Дэман і ўладальнік жыццёвага цыкла GUI, апавяшчэнні |
| `localsend-gui-slint` | Фармальны GUI, Slint з FemtoVG рэндэрырам |
| `localsend-tui` | Інтэрфейс тэрмінала |
| `localsend-launcher` | Яўная праграма запуску для асяроддзяў без трэя |
| `localsend-androidd` | Аўтаномны дэман атрымання толькі для root Android (MVP) |

Каталогі `localsend-gui-egui`, `-fltk`, `-iced` і `-tauri` з'яўляюцца эксперыментальнымі партамі і застаюцца па-за каранёвай працоўнай прасторай да завяршэння іх інтэграцыі RPC.

## Спампаваць

Актывы выпуску прытрымліваюцца адной схемы наймення — `LocalSend-<version>-<platform>-<arch>.<ext>` з платформамі `linux` / `windows` / `macos` і архітэктурамі `x86_64` / `aarch64`:

| Платформа | актыў |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Любы Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` ахоплівае ўсе актывы ў выпуску. Зборка macOS, створаная CI, падпісана спецыяльна, а не падпісана ідэнтыфікатарам распрацоўшчыка або натарыяльна заверана.

## Хуткі старт

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

Пакеты Linux усталёўваюць запіс працоўнага стала, які запускае `localsend-tray`, які валодае жыццёвым цыклам дэмана. На працоўных сталах без радка стану ў трэі застаюцца адкрытымі GUI; `LocalSend --launcher` выбірае гэты рэжым відавочна з AppImage.

### Windows

Распакуйце і запусціце `LocalSend.exe`. Архіў змяшчае GUI, дэман, латок і праграму запуску побач; трымаць іх разам.

## Паводзіны, якія варта ведаць

- **Выхад — гэта адна аперацыя.** Выхад з трэя спыняе GUI, запісаны ў вобласці канцавога пункта `gui.pid`, спыняе належны дэман на працягу абмежаванага часу чакання і выходзіць, не пакідаючы працэс або заданне launchd keep-alive.
- **Апавяшчэнні прыходзяць з трэя.** GUI ніколі не публікуе ўласныя апавяшчэнні, таму кожная платформа мае адзіны шлях дастаўкі.
- **Стан акна залежыць ад канчатковай кропкі.** Размяшчэнне захоўваецца побач з канфігурацыяй дэмана для гэтай канчатковай кропкі, а не ў агульным глабальным месцы, таму два асобнікі не змагаюцца за геаметрыю акна.
- **Дэман з'яўляецца аўтарытэтным.** Налады, змененыя ў GUI, перадаюцца дэману і паўтараюцца праз здымак; інтэрфейс не захоўвае прыватнай копіі, якая магла б змяніцца.

## Мовы

Інтэрфейс пастаўляецца ў 57 мовах, якія супадаюць са спісам LocalSend, са спрошчанай кітайскай мовай у якасці мовы паведамленняў. Выберыце адзін у раздзеле **Настройкі → Мова** або пакіньце яго ў «Сачыць за сістэмай», каб выкарыстоўваць лакаль АС.

| | |
| --- | --- |
| Зборныя каталогі | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| Зыходныя радкі карыстацкага інтэрфейсу | `@tr("...")` у `apps/localsend-gui-slint/ui` |
| Ангельскі каталог | `translations/locale-en.json` (інтэрфейс), `translations/locale-en-runtime.json` (Rust паведамленняў) |
| Паведамленні з боку Rust | `crates/localsend-i18n` (генеруецца `lookup_generated.rs`) |
| Уверх па плыні люстэрка | `translations/upstream/<locale>.json` |

Аднавіце каталогі пасля змены любога радка інтэрфейсу:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` фіксуецца, таму створаныя каталогі можна прайграваць у аўтаномным рэжыме. Выпраўце там запіс і перазапусціце генератар, каб выправіць пераклад.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Будаўніцтва з крыніцы

Патрабаванні: Rust `1.97.1` (прымацавана `rust-toolchain.toml`) і, для зборак Linux, загалоўкі распрацоўкі GTK3 і AppIndicator.

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

Скрыпты прыняцця кіруюць сапраўднымі двайковымі файламі замест імітаванага стану:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Дакументацыя

| Дакумент | Змест |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Праца, упакоўка і паводзіны платформы |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Бягучыя доказы выпрабаванняў і вядомыя неправераныя вобласці |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Лакальныя прыёмныя вароты |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Перыядычныя рэгрэсіі і доказы, якія чакаюцца да змены паводзін |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Стан візуальнай канвергенцыі ў параўнанні з прататыпам OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Каранёвы модуль Android і яго абмежаванні |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Вышэйшае паходжанне ядра пастаўшчыка |

Гэта README апублікавана на ўсіх 57 мовах інтэрфейсу — выберыце адну з пераключальніка ўверсе. Пераклады ствараюцца з `README.md` `scripts/translate-readme.mjs`; выпраўленні ўносяцца ў [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Некаторыя дакументы напісаны на кітайскай мове; яны запісваюць праверкі і канвенцыі словамі, якімі яны былі ўзгоднены.

## Вобласць прымянення

Асноўным прадуктам з'яўляецца кліент для ПК. Падтрымка Android з'яўляецца толькі каранёвым, бязгалоўным дэманам атрымання, пакуль яўна не будзе выведзена за межы гэтага аб'ёму — у яго няма GUI, не запускаецца першапланавы сэрвіс і не публікуюцца пастаянныя апавяшчэнні. `crates/localsend-official-core` захоўваецца байт за байтам, ідэнтычны запісанаму экспарту ўверх, акрамя створанага файла `SHA256SUMS`.

## Крэдыты

Пратакол, фармат правадоў і ядро ​​Rust паходзяць з [localsend/localsend](https://github.com/localsend/localsend); гэты праект стварае настольны кліент на гэтай працы, а не паўторна ўкараняючы пратакол. Дзякуй LocalSend аўтарам і ўдзельнікам.

Інтэрфейс адпавядае прататыпу дызайну, распрацаванаму для гэтага праекта; яго візуальная спасылка знаходзіцца ў [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Уклад

- **Памылкі і прапановы**: адкрыйце [праблему](https://github.com/alkaid-ops/localsend-next/issues) з версіяй, платформай, этапамі прайгравання і тым, што вы бачылі. Часопіс або скрыншот дазваляюць дзейнічаць нашмат хутчэй.
- **Код**: вітаюцца запыты на выцягванне. Змены, якія закранаюць мяжу daemon/frontend, ядро ​​пастаўшчыка або візуальны кантракт, варта абмеркаваць спачатку — абмежаванні запісваюцца ў [`AGENTS.md`](../../AGENTS.md).
- **Пераклады**: каталогі ствараюцца з захаванага кэша машыннага перакладу, так што выпраўленні там прасцей за ўсё праглядаць.

## Ліцэнзія

[Apache-2.0](../../LICENSE). Кампаненты іншых вытворцаў і іх ліцэнзіі пералічаны ў [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

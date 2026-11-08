<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | **Монгол** | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [简体中文](../../docs/zh-CN/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next нь Rust дээр бичигдсэн LocalSend-тэй нийцтэй ширээний клиент юм. Энэ нь албан ёсны протоколоор ярьдаг тул албан ёсны LocalSend программыг ашиглан файлуудыг хоёр чиглэлд илрүүлж, дамжуулдаг бөгөөд Flutter програмын оронд суурин компьютерийн программ хэлбэрээр ажилладаг.

Сүлжээний тал нь албан ёсны LocalSend Rust цөм бөгөөд өөрчлөгдөөгүй. Дээр дурдсан бүх зүйл - дэмон, RPC давхарга, тавиур, терминалын интерфейс ба Slint GUI - энэ цөмийг тойрон бүтээгдсэн.

## Демо

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Илгээх | Тохиргоо |
| --- | --- |
| ![Файл, хавтас, текст болон санах ойн эх сурвалж болон ойролцоох төхөөрөмжүүдийн хамт хуудас илгээх](../../docs/images/readme-send-light.png) | ![Загвар, хэл, сүлжээ, дамжуулах сонголт бүхий тохиргооны хуудас](../../docs/images/readme-settings-light.png) |

## Онцлогууд

- Дахин хэрэгжүүлэхээс илүүтэйгээр борлуулсан дээд талын үндсэн дээр албан ёсны LocalSend үйлчлүүлэгчдийн эсрэг илрүүлж, шилжүүлэх
- Нэг каноник муж эзэмшигч: демон нь тохиргоо, таних тэмдэг, дуртай зүйлс, түүх, шилжүүлэг болон TLS таниулбарыг агуулж байдаг тул урд тал бүр ижил зүйлийг хардаг.
- GUI, терминалын UI болон тавиур нь ижил орон нутгийн RPC дээр бие даасан урд талдаа; тавиур нь демон ба GUI процессын амьдралын мөчлөгийг эзэмшдэг
- Файл бүрийн бодит цагийн явц, нээлттэй болон илчлэх үйлдлүүдтэй түүх, дуртай зүйлс, төхөөрөмжөөр гараар оруулах, Вэб хуваалцах холбоосууд
- Дээд талын LocalSend-ийн жагсаалтад тохирох 57 интерфэйсийн хэл, ажиллах үед эсвэл системийн локал тохиргоонд шилжих боломжтой
- Төрөлхийн ширээний интеграцчилал: зөвхөн тавиураас мэдэгдэл хүлээн авах, нэвтрэх үед эхлүүлэх, тавиур руу багасгах, цонхны байршлын тогтвортой байдал, гарчгийн мөрөнд зурсан macOS цонх
- macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) болон Windows (ZIP), x86_64 болон aarch64 хоёрын баглаа боодол

## Архитектур

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Демон нь сүлжээтэй харьцдаг эсвэл програмын төлөвийг бичдэг цорын ганц процесс юм. Frontends хэзээ ч худалдагч цөмөөс хамаардаггүй бөгөөд хэзээ ч демоныг өөрөө төрүүлдэггүй; тавиур хийдэг. Файлын байт нь RPC дээгүүр дамждаггүй — дэмон нь замуудыг шийдэж, контентыг хэш болгож, түүхийг бичдэг.

| Хоёртын | Үүрэг |
| --- | --- |
| `localsendd` | Сүлжээний үйлчилгээ ба каноник хэрэглээний төлөв |
| `localsend-tray` | Демон болон GUI амьдралын мөчлөг эзэмшигч, мэдэгдэл |
| `localsend-gui-slint` | FemtoVG дүрслэгчтэй албан ёсны GUI, Slint |
| `localsend-tui` | Терминалын урд хэсэг |
| `localsend-launcher` | Тавиургүй орчинд зориулсан тодорхой эхлүүлэгч |
| `localsend-androidd` | Зөвхөн бие даасан root Android дэмон хүлээн авах (MVP) |

`localsend-gui-egui`, `-fltk`, `-iced` болон `-tauri` лавлахууд нь туршилтын портууд бөгөөд тэдгээрийн RPC интеграци дуусах хүртэл үндсэн ажлын талбараас гадуур байдаг.

## Татаж авах

Хөрөнгийг гаргах нэг нэршлийн схемийг дагаж мөрддөг - `LocalSend-<version>-<platform>-<arch>.<ext>`, платформууд `linux` / `windows` / `macos` болон архитектурууд `x86_64` / `aarch64`:

| Платформ | Хөрөнгө |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Дурын Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` нь хувилбарын бүх хөрөнгийг хамарна. CI-ийн үйлдвэрлэсэн macOS бүтэц нь түр гарын үсэг зурсан бөгөөд Хөгжүүлэгчийн ID гарын үсэг эсвэл нотариатаар баталгаажуулаагүй.

## Түргэн эхлэл

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

Linux багцууд нь демоны амьдралын мөчлөгийг эзэмшдэг `localsend-tray`-г эхлүүлдэг ширээний бичлэгийг суулгадаг. Статус талбаргүй ширээний компьютер дээр тавиур нь GUI-г нээлттэй байлгадаг; `LocalSend --launcher` нь AppImage-аас тухайн горимыг шууд сонгоно.

### Windows

Залгаад `LocalSend.exe` ажиллуулна уу. Архив нь GUI, дэмон, тавиур болон эхлүүлэгчийг зэрэгцүүлэн хадгалдаг; тэднийг хамт байлга.

## Мэдэх үнэ цэнэтэй зан байдал

- **Гарах нь нэг үйлдэл юм.** Тавиураас гарах нь төгсгөлийн цэгийн хамрах хүрээтэй `gui.pid`-д бичигдсэн GUI-г дуусгаж, эзэмшиж буй демоныг хязгаарлагдмал хугацааны дотор зогсоож, процесс эсвэл эхлүүлсэн амьд байлгах ажлыг орхихгүйгээр гарна.
- **Мэдэгдэл нь тавиураас ирдэг.** GUI хэзээ ч өөрийн мэдэгдлээ нийтэлдэггүй тул платформ бүр нэг хүргэх замтай.
- **Цонхны төлөв нь төгсгөлийн цэг тус бүрээр байна.** Байршлуулалт нь дэлхий нийтийн хуваалцсан байршилд бус тухайн төгсгөлийн цэгийн дэмон тохиргооны хажууд хадгалагддаг тул хоёр тохиолдол нь цонхны геометрийн талаар маргалддаггүй.
- **Демон нь эрх мэдэлтэй.** GUI-д өөрчлөгдсөн тохиргоог демонд илгээж, агшин зуурын агшинд дахин цуурайтуулна; frontend нь дрейф болох хувийн хуулбарыг хадгалдаггүй.

## Хэлнүүд

Интерфэйс нь msgid хэлээр Хялбаршуулсан Хятад хэлтэй, дээд талын LocalSend жагсаалтад нийцэх 57 хэлээр илгээгдэнэ. **Тохиргоо → Хэл** хэсгээс нэгийг нь сонгох эсвэл үйлдлийн системийн хэлийг ашиглахын тулд "Системийг дагах" хэсэгт үлдээгээрэй.

| | |
| --- | --- |
| Багцлагдсан каталогууд | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UI эх сурвалжийн мөрүүд | `@tr("...")`-д `apps/localsend-gui-slint/ui` |
| Англи хэлний каталог | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust мессежүүд) |
| Зэвэрсэн талын мессежүүд | `crates/localsend-i18n` (үүсгэсэн `lookup_generated.rs`) |
| Дээд талын толь | `translations/upstream/<locale>.json` |

Аливаа UI мөрийг өөрчилсний дараа каталогийг сэргээнэ үү:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` үүрэг хүлээсэн тул үүсгэсэн каталогийг офлайнаар хуулбарлах боломжтой. Орчуулгыг засахын тулд тэнд оруулгыг засаад генераторыг дахин ажиллуулна уу.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Эх сурвалжаас барилга барих

Тавигдах шаардлага: Rust `1.97.1` (`rust-toolchain.toml` бэхэлсэн) ба Linux бүтээцийн хувьд GTK3 болон AppIndicator хөгжүүлэлтийн толгой хэсэг.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Платформ багцууд:

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

Хүлээн авах скрипт нь элэглэсэн төлөвийн оронд жинхэнэ хоёртын файлуудыг удирддаг:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Баримт бичиг

| Баримт бичиг | Агуулга |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Үйл ажиллагаа, сав баглаа боодол, платформын зан байдал |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Одоогийн туршилтын нотлох баримтууд болон мэдэгдэж байгаа баталгаажаагүй хэсгүүд |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Орон нутгийн хүлээн авах хаалга |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Давтагдах регресс ба зан төлөвийг өөрчлөхөөс өмнө хүлээгдэж буй нотлох баримтууд |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | OpenDesign прототипийн эсрэг харааны нэгдэх төлөв |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Android root модуль ба түүний хязгаар |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Борлуулагчийн үндсэн эх үүсвэрийн эх үүсвэр |

Энэхүү README нь 57 интерфэйсийн бүх хэл дээр хэвлэгдсэн бөгөөд дээд талд байгаа шилжүүлэгчээс нэгийг нь сонго. Орчуулгууд нь `README.md`-аас `scripts/translate-readme.mjs`; засварууд [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json) дотор орно.

Зарим баримт бичгийг хятад хэл дээр бичсэн; Тэд баталгаажуулалтын ажил болон конвенцуудыг тохиролцсон үгээр тэмдэглэдэг.

## Хамрах хүрээ

Үндсэн бүтээгдэхүүн нь PC-ийн үйлчлүүлэгч юм. Android дэмжлэг нь зөвхөн root-д зориулагдсан, толгойгүй хүлээн авах дэмон бөгөөд энэ хүрээнээс хэтрүүлэн сурталчлах хүртэл бөгөөд үүнд GUI байхгүй, урд талын үйлчилгээ эхлэхгүй бөгөөд байнгын мэдэгдэл нийтлэхгүй. `crates/localsend-official-core` нь үүсгэсэн `SHA256SUMS` файлаас гадна бүртгэгдсэн дээд экспорттойгоо байт тутамд ижил хадгалагдана.

## Зээл

Протокол, утасны формат болон Rust цөм нь [localsend/localsend](https://github.com/localsend/localsend); Энэ төсөл нь протоколыг дахин хэрэгжүүлэхээс илүүтэйгээр тухайн ажил дээр ширээний клиентийг бий болгодог. LocalSend зохиогчид болон хувь нэмэр оруулагчдад баярлалаа.

Интерфэйс нь энэ төсөлд зориулж боловсруулсан дизайны прототипийг дагаж мөрддөг; түүний харааны лавлагаа [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/) дотор амьдардаг.

## Хувь нэмэр оруулж байна

- **Алдаа болон зөвлөмж**: хувилбар, платформ, хуулбарлах алхамууд болон үзсэн зүйлийнхээ хамт [асуудал](https://github.com/alkaid-ops/localsend-next/issues) нээнэ үү. Бүртгэл эсвэл дэлгэцийн агшин нь үйлдлийг илүү хурдан болгодог.
- **Код**: татах хүсэлтийг хүлээн авах боломжтой. daemon/frontend хил хязгаар, нийлүүлэгчийн үндсэн хэсэг эсвэл харааны гэрээнд хүрсэн өөрчлөлтүүдийг эхлээд хэлэлцэх ёстой — хязгаарлалтыг [`AGENTS.md`](../../AGENTS.md)-д тэмдэглэнэ.
- **Орчуулга**: Каталогууд нь автомат орчуулгын кэшээс үүсгэгддэг тул засварлах нь хамгийн хялбар хувь нэмэр оруулдаг.

## Лиценз

[Apache-2.0](../../LICENSE). Гуравдагч талын бүрэлдэхүүн хэсгүүд болон тэдгээрийн лицензүүдийг [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md)-д жагсаасан болно.

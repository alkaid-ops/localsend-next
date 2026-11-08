<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | **Oʻzbekcha** | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next bu Rust bilan yozilgan LocalSend-ga mos ish stoli mijozidir. U rasmiy protokolda gapiradi, shuning uchun u Flutter ilovasi oʻrniga mahalliy ish stoli dasturi sifatida ishlayotgan vaqtda rasmiy LocalSend ilovasi bilan fayllarni har ikki yoʻnalishda ham topadi va uzatadi.

Tarmoq tomoni rasmiy LocalSend Rust yadrosi boʻlib, sotuvchi oʻzgarmagan. Uning ustidagi hamma narsa - daemon, RPC qatlam, laganda, terminal interfeysi va Slint GUI - bu yadro atrofida qurilgan.

## Namoyish

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Yuborish | Sozlamalar |
| --- | --- |
| ![Fayl, jild, matn va vaqtinchalik xotira manbalari hamda yaqin-atrofdagi qurilmalar bilan sahifani yuborish](../../docs/images/readme-send-light.png) | ![Mavzu, til, tarmoq va uzatish opsiyalari bilan sozlamalar sahifasi](../../docs/images/readme-settings-light.png) |

## Xususiyatlar

- Rasmiy LocalSend mijozlariga qarshi kashf qilish va uzatish, qayta amalga oshirish emas, balki sotiladigan yuqori oqim yadrosida
- Bitta kanonik davlat egasi: demon sozlamalar, identifikator, sevimlilar, tarix, transferlar va TLS identifikatoriga ega, shuning uchun har bir frontend bir xil narsani ko'radi.
- GUI, terminal UI va bir xil mahalliy RPC ustida mustaqil frontendlar sifatida laganda; laganda daemon va GUI jarayonning hayot aylanishiga egalik qiladi
- Haqiqiy vaqtda har bir faylda ishlash, ochiq va oshkora amallar tarixi, sevimlilar, qurilmani qo‘lda kiritish va veb almashish havolalari
- Yuqori oqim LocalSend roʻyxatiga mos keladigan 57 ta interfeys tili, ular ish vaqtida almashtirilishi yoki tizim tiliga muvofiq
- Mahalliy ish stoli integratsiyasi: faqat lagandadan bildirishnomalar, tizimga kirishda ishga tushirish, laganda minimallashtirish, oynani joylashtirish barqarorligi va uning sarlavha satriga chizilgan macOS oynasi
- macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) va Windows (ZIP), x86_64 va aarch64 uchun qadoqlash

## Arxitektura

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Demon tarmoq bilan gaplashadigan yoki dastur holatini yozadigan yagona jarayondir. Frontendlar hech qachon sotuvchi yadroga bog'liq emas va hech qachon demonni o'zlari ishlab chiqarmaydi; laganda qiladi. Fayl baytlari RPC boʻylab oʻtmaydi — demon yoʻllarni hal qiladi, tarkibni xeshlaydi va tarixni yozib oladi.

| Ikkilik | Rol |
| --- | --- |
| `localsendd` | Tarmoq xizmati va kanonik dastur holati |
| `localsend-tray` | Daemon va GUI hayot aylanishi egasi, bildirishnomalar |
| `localsend-gui-slint` | Rasmiy GUI, Slint FemtoVG renderer bilan |
| `localsend-tui` | Terminal old tomoni |
| `localsend-launcher` | Tovoqsiz muhitlar uchun aniq ishga tushirgich |
| `localsend-androidd` | Faqat mustaqil root uchun Android daemonni qabul qilish (MVP) |

`localsend-gui-egui`, `-fltk`, `-iced` va `-tauri` kataloglari eksperimental portlar boʻlib, RPC integratsiyasi tugaguniga qadar ildiz ish maydonidan tashqarida qoladi.

## Yuklab olish

Chiqarish aktivlari bitta nomlash sxemasiga muvofiq - `LocalSend-<version>-<platform>-<arch>.<ext>`, platformalar `linux` / `windows` / `macos` va arxitekturalari `x86_64` / `aarch64`:

| Platforma | Aktiv |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Har qanday Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` relizdagi barcha aktivlarni qamrab oladi. CI tomonidan ishlab chiqarilgan macOS tuzilmasi ishlab chiquvchi identifikatori imzolangan yoki notarial tasdiqlangan emas, balki vaqtinchalik imzolangan.

## Tez boshlash

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

Linux paketlari ish stoli yozuvini o'rnatadi, u `localsend-tray` ni ishga tushiradi, u xizmat muddatini o'z ichiga oladi. Holat paneli bo'lmagan ish stollarida laganda GUI o'rniga ochiq qoladi; `LocalSend --launcher` ushbu rejimni AppImage dan aniq tanlaydi.

### Windows

Zipni oching va `LocalSend.exe` ni ishga tushiring. Arxivda GUI, daemon, laganda va ishga tushirish moslamasi yonma-yon joylashgan; ularni birga saqlang.

## Xulq-atvorni bilishga arziydi

- **Chiqish - bu bitta operatsiya.** Tovoqdan chiqish `gui.pid` soʻnggi nuqtada qayd etilgan GUI ni tugatadi, cheklangan vaqt oraligʻida tegishli demonni toʻxtatadi va jarayon yoki ishga tushirilgan jonli saqlash vazifasini qoldirmasdan chiqadi.
- **Bildirishnomalar lagandadan keladi.** GUI hech qachon o‘z bildirishnomalarini joylashtirmaydi, shuning uchun har bir platformada bitta yetkazib berish yo‘li mavjud.
- **Oyna holati har bir so‘nggi nuqtaga to‘g‘ri keladi.** Joylashuv umumiy global joylashuvda emas, balki ushbu so‘nggi nuqta uchun demon konfiguratsiyasi yonida saqlanadi, shuning uchun ikkita misol oyna geometriyasi ustida kurashmaydi.
- **Demon vakolatli.** GUI da oʻzgartirilgan sozlamalar xizmat dasturiga yuboriladi va oniy tasvir orqali aks ettiriladi; frontend drift mumkin bo'lgan shaxsiy nusxani saqlamaydi.

## Tillar

Interfeys yuqori oqim LocalSend roʻyxatiga mos keladigan 57 ta tilda yetkazib beriladi, msgid tili sifatida soddalashtirilgan xitoycha. **Sozlamalar → Til** ostidan birini tanlang yoki OT tilidan foydalanish uchun uni “Tizimga rioya qilish”da qoldiring.

| | |
| --- | --- |
| Birlashtirilgan kataloglar | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UI manba satrlari | `@tr("...")`, `apps/localsend-gui-slint/ui` |
| Ingliz katalogi | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust xabarlar) |
| Zang tomonidagi xabarlar | `crates/localsend-i18n` (hosil qilingan `lookup_generated.rs`) |
| Yuqori oqim oynasi | `translations/upstream/<locale>.json` |

Har qanday UI qatorini o'zgartirgandan so'ng kataloglarni qayta yarating:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` belgilangan, shuning uchun yaratilgan kataloglar oflayn rejimda takrorlanishi mumkin. U yerdagi yozuvni tuzating va tarjimani tuzatish uchun generatorni qayta ishga tushiring.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Manbadan qurilish

Talablar: Rust `1.97.1` (`rust-toolchain.toml` tomonidan mahkamlangan) va Linux tuzilmalari uchun GTK3 va AppIndicator ishlab chiqish sarlavhalari.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Platforma paketlari:

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

Qabul qilish skriptlari masxara qilingan holat o'rniga haqiqiy ikkilik fayllarni boshqaradi:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Hujjatlar

| Hujjat | Tarkib |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Operatsion, qadoqlash va platforma harakati |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Joriy sinov dalillari va ma'lum tasdiqlanmagan joylar |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Mahalliy qabul qilish eshiklari |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Takroriy regressiyalar va xatti-harakatlarni o'zgartirishdan oldin kutilgan dalillar izi |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | OpenDesign prototipiga nisbatan vizual konvergentsiya holati |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Android ildiz moduli va uning chegaralari |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Sotuvchi yadroning yuqori oqimidagi kelib chiqishi |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Ba'zi hujjatlar xitoy tilida yozilgan; ular kelishilgan so'zlarda tekshirish yugurishlari va konventsiyalarini qayd qiladilar.

## Qo'llash doirasi

Asosiy mahsulot kompyuter mijozidir. Android qo'llab-quvvatlash faqat ildizga ega, boshsiz qabul qilish dasturi bo'lib, ushbu doiradan tashqarida aniq targ'ib qilinmaguncha - unda GUI yo'q, oldingi xizmatni ishga tushirmaydi va doimiy bildirishnoma joylashtirmaydi. `crates/localsend-official-core` yaratilgan `SHA256SUMS` faylidan tashqari, yozib olingan yuqori oqim eksporti bilan bir xil bayt-bayt saqlanadi.

## Kreditlar

Protokol, sim formati va Rust yadrosi [localsend/localsend](https://github.com/localsend/localsend) dan keladi; bu loyiha protokolni qayta amalga oshirishdan ko'ra o'sha ish ustida ish stoli mijozini quradi. LocalSend mualliflari va hissa qo'shuvchilarga rahmat.

Interfeys ushbu loyiha uchun ishlab chiqilgan dizayn prototipiga amal qiladi; uning vizual havolasi [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/) da saqlanadi.

## Hissa

- **Xatolar va takliflar**: versiya, platforma, ko‘paytirish bosqichlari va ko‘rgan narsangiz bilan [muammo](https://github.com/alkaid-ops/localsend-next/issues) oching. Jurnal yoki skrinshot uni tezroq bajarishga yordam beradi.
- **Kod**: tortish so'rovlari qabul qilinadi. daemon/frontend chegarasiga, sotuvchi yadroga yoki vizual shartnomaga tegadigan o'zgarishlar birinchi navbatda muhokama qilinishi kerak - cheklovlar [`AGENTS.md`](../../AGENTS.md) da qayd etiladi.
- **Tarjimalar**: kataloglar avtomatik tarjima keshidan yaratiladi, shuning uchun tuzatishlar ko'rib chiqishning eng oson turi hisoblanadi.

## Litsenziya

[Apache-2.0](../../LICENSE). Uchinchi tomon komponentlari va ularning litsenziyalari [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md) da keltirilgan.

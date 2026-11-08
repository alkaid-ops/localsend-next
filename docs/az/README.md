<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | **Azərbaycanca** | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next, Rust ilə yazılmış LocalSend-ə uyğun iş masası müştərisidir. O, rəsmi protokolda danışır, buna görə də Flutter proqramı əvəzinə yerli iş masası proqramı kimi işləyərkən rəsmi LocalSend proqramı ilə faylları hər iki istiqamətdə aşkar edir və ötürür.

Şəbəkə tərəfi rəsmi LocalSend Rust nüvəsidir, dəyişməz olaraq satılır. Üstündəki hər şey - daemon, RPC qat, tray, terminal UI və Slint GUI - bu nüvə ətrafında qurulmuşdur.

## Demo

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Göndər | Parametrlər |
| --- | --- |
| ![Fayl, qovluq, mətn və mübadilə buferi mənbələri və yaxınlıqdakı cihazlarla səhifə göndərin](../../docs/images/readme-send-light.png) | ![Mövzu, dil, şəbəkə və köçürmə seçimləri ilə parametrlər səhifəsi](../../docs/images/readme-settings-light.png) |

## Xüsusiyyətlər

- Rəsmi LocalSend müştərilərinə qarşı kəşf və köçürmə, təkrar həyata keçirməkdənsə, satıcının yuxarı axını əsas üzərində
- Bir kanonik dövlət sahibi: demon parametrləri, şəxsiyyəti, sevimliləri, tarixçəni, köçürmələri və TLS şəxsiyyətini saxlayır, buna görə də hər bir frontend eyni şeyi görür.
- GUI, terminal UI və eyni yerli RPC üzərində müstəqil cəbhələr kimi tray; tray daemon və GUI prosesin həyat dövrünə sahibdir
- Fayl başına real vaxtda irəliləyiş, açıq və aşkar hərəkətlərlə tarixçə, sevimlilər, əl ilə cihaz daxiletmə və Veb Paylaşım qəbulu bağlantıları
- LocalSend siyahısına uyğun gələn 57 interfeys dili, iş vaxtında dəyişdirilə bilər və ya sistem dilini izləyir
- Doğma masaüstü inteqrasiyası: yalnız nimçədən bildirişlər, giriş zamanı işə salın, nimçəni minimuma endir, pəncərə yerləşdirmə davamlılığı və başlıq çubuğuna daxil olan macOS pəncərəsi
- macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) və Windows (ZIP), x86_64 və aarch64 üçün qablaşdırma

## Memarlıq

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Demon şəbəkə ilə danışan və ya proqram vəziyyətini yazan yeganə prosesdir. Frontendlər heç vaxt təchiz edilmiş nüvədən asılı deyil və heç vaxt demonu özləri yaratmırlar; tepsi edir. Fayl baytları RPC üzərindən keçmir — daemon yolları həll edir, məzmunu hash edir və tarixçəni qeyd edir.

| İkili | Rol |
| --- | --- |
| `localsendd` | Şəbəkə xidməti və kanonik tətbiq vəziyyəti |
| `localsend-tray` | Daemon və GUI həyat dövrü sahibi, bildirişlər |
| `localsend-gui-slint` | Rəsmi GUI, Slint FemtoVG rendereri ilə |
| `localsend-tui` | Terminal cəbhəsi |
| `localsend-launcher` | Tablası olmayan mühitlər üçün açıq başlatma |
| `localsend-androidd` | Yalnız müstəqil kök üçün Android daemon qəbul edir (MVP) |

`localsend-gui-egui`, `-fltk`, `-iced` və `-tauri` kataloqları eksperimental portlardır və RPC inteqrasiyası tamamlanana qədər kök iş yerindən kənarda qalırlar.

## Yüklə

Buraxılış aktivləri bir adlandırma sxeminə uyğundur - `LocalSend-<version>-<platform>-<arch>.<ext>`, platformalar `linux` / `windows` / `macos` və arxitekturalar `x86_64` / `aarch64`:

| Platforma | Aktiv |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| İstənilən Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` buraxılışdakı hər bir aktivi əhatə edir. CI tərəfindən istehsal edilən macOS quruluşu Tərtibatçı ID-si imzalanmış və ya notarial qaydada təsdiq edilmiş deyil, ad-hoc imzalanmışdır.

## Tez başlanğıc

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

Linux paketləri demonun həyat dövrünə sahib olan `localsend-tray`-ı işə salan masaüstü girişi quraşdırır. Vəziyyət çubuğu olmayan masaüstlərində nimçə GUI işarəsini açıq saxlayır; `LocalSend --launcher` həmin rejimi açıq şəkildə AppImage-dan seçir.

### Windows

Açın və `LocalSend.exe` işə salın. Arxiv GUI, daemon, tray və başlatma qurğusunu yan-yana saxlayır; onları bir yerdə saxlayın.

## Bilməyə dəyər davranış

- **Çıxmaq bir əməliyyatdır.** Tabladan çıxmaq son nöqtə əhatəli `gui.pid`-də qeydə alınan GUI-a son qoyur, məhdud zaman aşımı ərzində sahib dəmonu dayandırır və prosesi və ya işə salınmış canlı saxlama işini arxada qoymadan çıxır.
- **Bildirişlər lövhədən gəlir.** GUI heç vaxt öz bildirişlərini göndərmir, ona görə də hər platformanın tək çatdırılma yolu var.
- **Pəncərə vəziyyəti son nöqtəyə görədir.** Yerləşdirmə paylaşılan qlobal məkanda deyil, həmin son nöqtə üçün demon konfiqurasiyasının yanında saxlanılır, ona görə də iki instansiya pəncərə həndəsəsi üzərində mübarizə aparmır.
- **Daemon səlahiyyətlidir.** GUI-da dəyişdirilmiş parametrlər demona təqdim edilir və snapshot vasitəsilə əks-səda verilir; frontend sürüşə biləcək heç bir şəxsi nüsxə saxlamır.

## Dillər

İnterfeys msgid dili kimi Sadələşdirilmiş Çin dili ilə yuxarı axın LocalSend siyahısına uyğun gələn 57 dildə göndərilir. **Parametrlər → Dil** altından birini seçin və ya ƏS dilini istifadə etmək üçün onu "Sistemi izlə"də buraxın.

| | |
| --- | --- |
| Birləşdirilmiş kataloqlar | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UI mənbə sətirləri | `@tr("...")`, `apps/localsend-gui-slint/ui` |
| İngilis dili kataloqu | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust mesajlar) |
| Pas tərəfi mesajlar | `crates/localsend-i18n` (generasiya `lookup_generated.rs`) |
| Yuxarı güzgü | `translations/upstream/<locale>.json` |

İstənilən UI sətrini dəyişdirdikdən sonra kataloqları bərpa edin:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` sadiqdir, ona görə də yaradılan kataloqlar oflayn rejimdə təkrarlana bilər. Oradakı girişi düzəldin və tərcüməni düzəltmək üçün generatoru yenidən işə salın.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Mənbədən bina

Tələblər: Rust `1.97.1` (`rust-toolchain.toml` ilə bərkidilir) və Linux qurmaları üçün GTK3 və AppIndicator inkişaf başlıqları.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Platforma paketləri:

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

Qəbul skriptləri istehza edilmiş vəziyyət əvəzinə həqiqi ikili faylları idarə edir:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Sənədləşdirmə

| Sənəd | İçindəkilər |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Əməliyyat, qablaşdırma və platforma davranışı |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Cari sınaq sübutları və məlum yoxlanılmamış sahələr |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Yerli qəbul qapıları |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Davranışı dəyişdirməzdən əvvəl gözlənilən təkrarlanan reqressiyalar və sübut izi |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | OpenDesign prototipinə qarşı vizual yaxınlaşma vəziyyəti |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Android kök modulu və onun məhdudiyyətləri |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Satıcı nüvənin yuxarı mənşəyi |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Bəzi sənədlər Çin dilində yazılmışdır; onlar razılaşdıqları sözlərlə yoxlama qaçışlarını və konvensiyaları qeyd edirlər.

## Əhatə dairəsi

Əsas məhsul PC müştərisidir. Android dəstəyi, bu əhatə dairəsindən kənarda açıq şəkildə təbliğ olunana qədər yalnız köklü, başsız qəbul demonudur - onun GUI yoxdur, ön planda xidmətə başlamır və davamlı bildiriş göndərmir. `crates/localsend-official-core` yaradılan `SHA256SUMS` faylından başqa, qeyd edilmiş yuxarı axın ixracı ilə bayt-bayt eyni saxlanılır.

## Kreditlər

Protokol, naqil formatı və Rust nüvəsi [localsend/localsend](https://github.com/localsend/localsend); bu layihə protokolu yenidən həyata keçirməkdənsə, həmin iş üzərində masaüstü müştəri qurur. LocalSend müəlliflərinə və töhfə verənlərə təşəkkür edirik.

İnterfeys bu layihə üçün hazırlanmış dizayn prototipini izləyir; onun vizual istinadı [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/) içərisində yaşayır.

## Töhfə vermək

- **Baqlar və təkliflər**: versiya, platforma, reproduksiya addımları və gördüklərinizlə [məsələ](https://github.com/alkaid-ops/localsend-next/issues) açın. Jurnal və ya ekran görüntüsü onun üzərində işləməyi daha sürətli edir.
- **Kod**: çəkmə sorğuları qəbul edilir. daemon/frontend sərhədinə, tədarük edilən əsasa və ya vizual müqaviləyə toxunan dəyişikliklər əvvəlcə müzakirə edilməlidir — məhdudiyyətlər [`AGENTS.md`](../../AGENTS.md) ilə qeyd olunur.
- **Tərcümələr**: kataloqlar maşın-tərcümə keşindən yaradılır, ona görə də orada düzəlişlər nəzərdən keçirmək üçün ən asan töhfədir.

## Lisenziya

[Apache-2.0](../../LICENSE). Üçüncü tərəf komponentləri və onların lisenziyaları [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md) saytında verilmişdir.

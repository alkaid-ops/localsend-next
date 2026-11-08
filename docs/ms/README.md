<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | **Bahasa Melayu** | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [简体中文](../../docs/zh-CN/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next ialah klien desktop serasi LocalSend yang ditulis dalam Rust. Ia bercakap protokol rasmi, jadi ia menemui dan memindahkan fail dengan apl LocalSend rasmi di kedua-dua arah, sambil berjalan sebagai program desktop asli dan bukannya apl Flutter.

Bahagian rangkaian ialah teras LocalSend Rust rasmi, dijual tidak berubah. Segala-galanya di atasnya — daemon, RPC lapisan, dulang, UI terminal dan Slint GUI — dibina di sekeliling teras itu.

## Demo

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Hantar | tetapan |
| --- | --- |
| ![Hantar halaman dengan fail, folder, teks dan sumber papan keratan serta peranti berdekatan](../../docs/images/readme-send-light.png) | ![Halaman tetapan dengan pilihan tema, bahasa, rangkaian dan pemindahan](../../docs/images/readme-settings-light.png) |

## Ciri

- Penemuan dan pemindahan terhadap pelanggan LocalSend rasmi, pada teras huluan yang dijual dan bukannya pelaksanaan semula
- Satu pemilik keadaan kanonik: daemon memegang tetapan, identiti, kegemaran, sejarah, pemindahan dan identiti TLS, jadi setiap bahagian hadapan melihat perkara yang sama
- GUI, UI terminal dan dulang sebagai hujung hadapan bebas di atas RPC setempat yang sama; dulang memiliki daemon dan GUI kitaran hayat proses
- Kemajuan setiap fail masa nyata, sejarah dengan tindakan terbuka dan dedahkan, kegemaran, kemasukan peranti manual dan Perkongsian Web menerima pautan
- 57 bahasa antara muka yang sepadan dengan senarai huluan LocalSend, boleh ditukar pada masa jalan atau mengikut tempat sistem
- Penyepaduan desktop asli: pemberitahuan daripada dulang sahaja, pelancaran semasa log masuk, minimumkan kepada dulang, kegigihan peletakan tetingkap dan tetingkap macOS yang menarik ke dalam bar tajuknya
- Pembungkusan untuk macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) dan Windows (ZIP), untuk kedua-dua x86_64 dan aarch64

## Seni bina

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Daemon adalah satu-satunya proses yang bercakap dengan rangkaian atau menulis keadaan aplikasi. Frontend tidak pernah bergantung pada teras yang dijual dan tidak pernah melahirkan daemon itu sendiri; dulang tidak. Bait fail tidak bergerak melebihi RPC — daemon menyelesaikan laluan, mencincang kandungan dan merekodkan sejarah.

| binari | Peranan |
| --- | --- |
| `localsendd` | Perkhidmatan rangkaian dan keadaan aplikasi kanonik |
| `localsend-tray` | Daemon dan GUI pemilik kitaran hayat, pemberitahuan |
| `localsend-gui-slint` | GUI rasmi, Slint dengan pemapar FemtoVG |
| `localsend-tui` | Bahagian hadapan terminal |
| `localsend-launcher` | Pelancar eksplisit untuk persekitaran tanpa dulang |
| `localsend-androidd` | Akar kendiri sahaja Android terima daemon (MVP) |

Direktori `localsend-gui-egui`, `-fltk`, `-iced` dan `-tauri` ialah port percubaan dan kekal di luar ruang kerja akar sehingga penyepaduan RPC mereka selesai.

## Muat turun

Aset keluaran mengikut satu skim penamaan — `LocalSend-<version>-<platform>-<arch>.<ext>`, dengan platform `linux` / `windows` / `macos` dan seni bina `x86_64` / `aarch64`:

| Platform | Aset |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Mana-mana Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` meliputi setiap aset dalam keluaran. Binaan macOS yang dihasilkan oleh CI ditandatangani secara ad-hoc, bukan ID Pembangun yang ditandatangani atau dinotari.

## Permulaan cepat

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

Linux pakej memasang entri desktop yang melancarkan `localsend-tray`, yang memiliki kitaran hayat daemon. Pada desktop tanpa bar status, dulang menyimpan GUI terbuka sebaliknya; `LocalSend --launcher` memilih mod itu secara eksplisit daripada AppImage.

### Windows

Nyahzip dan jalankan `LocalSend.exe`. Arkib memegang GUI, daemon, dulang dan pelancar bersebelahan; kekalkan mereka bersama.

## Tingkah laku yang patut diketahui

- **Berhenti adalah satu operasi.** Berhenti dari dulang menamatkan GUI yang direkodkan dalam skop titik akhir `gui.pid`, menghentikan daemon yang dimiliki dalam tempoh tamat terhad dan keluar tanpa meninggalkan proses atau kerja yang terus hidup di belakang.
- **Pemberitahuan datang daripada dulang.** GUI tidak pernah menyiarkan pemberitahuannya sendiri, jadi setiap platform mempunyai satu laluan penghantaran.
- **Keadaan tetingkap adalah bagi setiap titik akhir.** Peletakan disimpan di sebelah konfigurasi daemon untuk titik akhir itu dan bukannya di lokasi global yang dikongsi, jadi dua kejadian tidak bertarung dalam geometri tetingkap.
- **Daemon adalah berwibawa.** Tetapan diubah dalam GUI diserahkan kepada daemon dan digema semula melalui syot kilat; bahagian hadapan tidak menyimpan salinan peribadi yang boleh hanyut.

## Bahasa

Antara muka dihantar dalam 57 tempat, sepadan dengan senarai hulu LocalSend, dengan Bahasa Cina Ringkas sebagai bahasa msgid. Pilih satu di bawah **Tetapan → Bahasa**, atau biarkan pada "Ikuti sistem" untuk menggunakan tempat OS.

| | |
| --- | --- |
| Katalog yang digabungkan | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| Rentetan sumber UI | `@tr("...")` dalam `apps/localsend-gui-slint/ui` |
| katalog Inggeris | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust mesej) |
| Mesej sebelah karat | `crates/localsend-i18n` (dijana `lookup_generated.rs`) |
| Cermin hulu | `translations/upstream/<locale>.json` |

Jana semula katalog selepas menukar mana-mana rentetan UI:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` komited, jadi katalog yang dijana boleh diterbitkan semula di luar talian. Betulkan entri di sana dan jalankan semula penjana untuk membetulkan terjemahan.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Membina daripada sumber

Keperluan: Rust `1.97.1` (disemat oleh `rust-toolchain.toml`) dan, untuk binaan Linux, pengepala pembangunan GTK3 dan AppIndicator.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Pakej platform:

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

Skrip penerimaan memacu binari sebenar dan bukannya keadaan mengejek:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Dokumentasi

| Dokumen | kandungan |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Operasi, pembungkusan dan tingkah laku platform |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Bukti ujian semasa dan kawasan tidak disahkan yang diketahui |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Pintu penerimaan tempatan |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Regresi berulang dan jejak bukti yang dijangkakan sebelum mengubah tingkah laku |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Keadaan penumpuan visual terhadap prototaip OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Modul akar Android dan hadnya |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Asal huluan teras yang dijual |

README ini diterbitkan dalam semua 57 bahasa antara muka — pilih satu daripada penukar di bahagian atas. Terjemahan dijana daripada `README.md` oleh `scripts/translate-readme.mjs`; pembetulan masuk [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Beberapa dokumen ditulis dalam bahasa Cina; mereka merekodkan larian pengesahan dan konvensyen dalam perkataan yang mereka telah bersetuju.

## Skop

Produk utama ialah pelanggan PC. Sokongan Android ialah daemon terima tanpa kepala tanpa kepala sahaja sehingga dipromosikan secara eksplisit di luar skop itu — ia tidak mempunyai GUI, tidak memulakan perkhidmatan latar depan dan tidak menyiarkan pemberitahuan berterusan. `crates/localsend-official-core` disimpan bait-untuk-bait yang sama dengan eksport huluannya yang direkodkan, selain daripada fail `SHA256SUMS` yang dijananya.

## Kredit

Protokol, format wayar dan teras Rust berasal daripada [localsend/localsend](https://github.com/localsend/localsend); projek ini membina klien desktop pada kerja itu dan bukannya melaksanakan semula protokol. Terima kasih kepada LocalSend pengarang dan penyumbang.

Antara muka mengikuti prototaip reka bentuk yang dibangunkan untuk projek ini; rujukan visualnya hidup dalam [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Menyumbang

- **Pepijat dan cadangan**: buka [isu](https://github.com/alkaid-ops/localsend-next/issues) dengan versi, platform, langkah pembiakan dan perkara yang anda lihat. Log atau tangkapan skrin menjadikannya lebih pantas untuk bertindak.
- **Kod**: permintaan tarik adalah dialu-alukan. Perubahan yang menyentuh sempadan daemon/frontend, teras vendor atau kontrak visual harus dibincangkan terlebih dahulu — kekangan direkodkan dalam [`AGENTS.md`](../../AGENTS.md).
- **Terjemahan**: katalog dijana daripada cache terjemahan mesin yang komited, jadi pembetulan terdapat jenis sumbangan yang paling mudah untuk disemak.

## Lesen

[Apache-2.0](../../LICENSE). Komponen pihak ketiga dan lesennya disenaraikan dalam [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

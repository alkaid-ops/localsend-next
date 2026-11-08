<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | **Bahasa Indonesia** | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [简体中文](../../docs/zh-CN/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next adalah klien desktop yang kompatibel dengan LocalSend yang ditulis dalam Rust. Ia menggunakan protokol resmi, sehingga ia menemukan dan mentransfer file dengan aplikasi resmi LocalSend di kedua arah, sambil dijalankan sebagai program desktop asli, bukan aplikasi Flutter.

Sisi jaringan adalah inti resmi LocalSend Rust, yang dijual tidak berubah. Segala sesuatu di atasnya — daemon, lapisan RPC, baki, UI terminal, dan Slint GUI — dibangun berdasarkan inti tersebut.

## Demo

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| Mengirim | Pengaturan |
| --- | --- |
| ![Kirim halaman dengan sumber file, folder, teks dan clipboard serta perangkat terdekat](../../docs/images/readme-send-light.png) | ![Halaman pengaturan dengan opsi tema, bahasa, jaringan dan transfer](../../docs/images/readme-settings-light.png) |

## Fitur

- Penemuan dan transfer terhadap klien resmi LocalSend, pada inti hulu yang divendorkan, bukan implementasi ulang
- Satu pemilik status kanonik: daemon menyimpan pengaturan, identitas, favorit, riwayat, transfer, dan identitas TLS, sehingga setiap frontend melihat hal yang sama
- GUI, UI terminal dan baki sebagai frontend independen pada RPC lokal yang sama; baki memiliki daemon dan siklus hidup proses GUI
- Kemajuan per file secara real-time, riwayat dengan tindakan membuka dan mengungkapkan, favorit, entri perangkat manual, dan tautan penerimaan Berbagi Web
- 57 bahasa antarmuka yang cocok dengan daftar upstream LocalSend, dapat dialihkan saat runtime atau mengikuti lokal sistem
- Integrasi desktop asli: notifikasi hanya dari baki, diluncurkan saat login, perkecil ke baki, persistensi penempatan jendela, dan jendela macOS yang masuk ke dalam bilah judulnya
- Kemasan untuk macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) dan Windows (ZIP), untuk x86_64 dan aarch64

## Arsitektur

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

Daemon adalah satu-satunya proses yang berkomunikasi dengan jaringan atau menulis status aplikasi. Frontend tidak pernah bergantung pada inti yang divendorkan dan tidak pernah memunculkan daemon itu sendiri; nampan itu bisa. Byte file tidak berpindah ke RPC — daemon menyelesaikan jalur, melakukan hashing konten, dan mencatat riwayat.

| Biner | Peran |
| --- | --- |
| `localsendd` | Layanan jaringan dan status aplikasi kanonik |
| `localsend-tray` | Daemon dan pemilik siklus hidup GUI, notifikasi |
| `localsend-gui-slint` | GUI formal, Slint dengan penyaji FemtoVG |
| `localsend-tui` | Bagian depan terminal |
| `localsend-launcher` | Peluncur eksplisit untuk lingkungan tanpa baki |
| `localsend-androidd` | Hanya root mandiri Android yang menerima daemon (MVP) |

Direktori `localsend-gui-egui`, `-fltk`, `-iced` dan `-tauri` adalah port eksperimental dan tetap berada di luar ruang kerja root hingga integrasi RPC selesai.

## Unduh

Aset rilis mengikuti satu skema penamaan — `LocalSend-<version>-<platform>-<arch>.<ext>`, dengan platform `linux` / `windows` / `macos` dan arsitektur `x86_64` / `aarch64`:

| Platform | Aset |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Setiap Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` mencakup setiap aset dalam rilis. macOS build yang dihasilkan oleh CI ditandatangani secara ad-hoc, bukan ID Pengembang yang ditandatangani atau disahkan oleh notaris.

## Mulai cepat

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

Paket Linux menginstal entri desktop yang meluncurkan `localsend-tray`, yang memiliki siklus hidup daemon. Pada desktop tanpa bilah status, baki tetap membiarkan GUI tetap terbuka; `LocalSend --launcher` memilih mode tersebut secara eksplisit dari AppImage.

### Windows

Buka zip dan jalankan `LocalSend.exe`. Arsip tersebut menampung GUI, daemon, baki, dan peluncur secara berdampingan; jaga agar mereka tetap bersama.

## Perilaku yang perlu diketahui

- **Keluar adalah satu operasi.** Keluar dari baki akan menghentikan GUI yang direkam dalam cakupan titik akhir `gui.pid`, menghentikan daemon yang dimiliki dalam batas waktu yang ditentukan, dan keluar tanpa meninggalkan proses atau tugas peluncuran yang tetap hidup.
- **Pemberitahuan datang dari baki.** GUI tidak pernah memposting pemberitahuannya sendiri, sehingga setiap platform memiliki satu jalur pengiriman.
- **Status jendela adalah per titik akhir.** Penempatan disimpan di samping konfigurasi daemon untuk titik akhir tersebut, bukan di lokasi global bersama, sehingga dua instance tidak berebut geometri jendela.
- **Daemon bersifat otoritatif.** Setelan yang diubah di GUI dikirimkan ke daemon dan digaungkan kembali melalui snapshot; frontend tidak menyimpan salinan pribadi yang dapat hilang.

## Bahasa

Antarmuka dikirimkan dalam 57 lokal, cocok dengan daftar hulu LocalSend, dengan bahasa Cina Sederhana sebagai bahasa msgstr. Pilih satu di **Pengaturan → Bahasa**, atau biarkan di "Ikuti sistem" untuk menggunakan lokal OS.

| | |
| --- | --- |
| Katalog yang dibundel | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| String sumber UI | `@tr("...")` di `apps/localsend-gui-slint/ui` |
| Katalog bahasa Inggris | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust pesan) |
| Pesan sisi karat | `crates/localsend-i18n` (dihasilkan `lookup_generated.rs`) |
| Cermin hulu | `translations/upstream/<locale>.json` |

Buat ulang katalog setelah mengubah string UI apa pun:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` berkomitmen, sehingga katalog yang dihasilkan dapat direproduksi secara offline. Perbaiki entri di sana dan jalankan kembali generator untuk memperbaiki terjemahan.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## Membangun dari sumber

Persyaratan: Rust `1.97.1` (disematkan oleh `rust-toolchain.toml`) dan, untuk build Linux, header pengembangan GTK3 dan AppIndicator.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

Paket platform:

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

Skrip penerimaan menggerakkan binari sebenarnya alih-alih keadaan yang diolok-olok:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## Dokumentasi

| Dokumen | Isi |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | Perilaku pengoperasian, pengemasan, dan platform |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | Bukti pengujian terkini dan area yang diketahui belum diverifikasi |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | Gerbang penerimaan lokal |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | Regresi berulang dan jejak bukti diharapkan sebelum perubahan perilaku |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | Status konvergensi visual terhadap prototipe OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Modul root Android dan batasannya |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | Asal hulu dari inti yang dijajakan |

README ini diterbitkan dalam 57 bahasa antarmuka — pilih salah satu dari pengalih di atas. Terjemahan dihasilkan dari `README.md` oleh `scripts/translate-readme.mjs`; koreksi masuk [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

Beberapa dokumen ditulis dalam bahasa Mandarin; mereka mencatat proses verifikasi dan konvensi dalam kata-kata yang disepakati.

## Cakupan

Produk utamanya adalah klien PC. Dukungan Android adalah daemon penerimaan tanpa kepala yang hanya dapat di-root hingga dipromosikan secara eksplisit di luar cakupan tersebut — tidak memiliki GUI, tidak memulai layanan latar depan, dan tidak memposting pemberitahuan terus-menerus. `crates/localsend-official-core` disimpan byte demi byte identik dengan ekspor hulu yang tercatat, selain dari file `SHA256SUMS` yang dihasilkannya.

## Kredit

Protokol, format kabel, dan inti Rust berasal dari [localsend/localsend](https://github.com/localsend/localsend); proyek ini membangun klien desktop berdasarkan pekerjaan itu daripada mengimplementasikan ulang protokol. Terima kasih kepada LocalSend penulis dan kontributor.

Antarmuka mengikuti prototipe desain yang dikembangkan untuk proyek ini; referensi visualnya ada di [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## Berkontribusi

- **Bug dan saran**: buka [masalah](https://github.com/alkaid-ops/localsend-next/issues) dengan versi, platform, langkah reproduksi, dan apa yang Anda lihat. Log atau tangkapan layar membuatnya lebih cepat untuk ditindaklanjuti.
- **Kode**: permintaan penarikan dipersilakan. Perubahan yang menyentuh batasan daemon/frontend, inti yang divendorkan, atau kontrak visual harus didiskusikan terlebih dahulu — batasannya dicatat dalam [`AGENTS.md`](../../AGENTS.md).
- **Terjemahan**: katalog dihasilkan dari cache terjemahan mesin yang dikomit, jadi koreksi di sana merupakan jenis kontribusi yang paling mudah untuk ditinjau.

## Lisensi

[Apache-2.0](../../LICENSE). Komponen pihak ketiga dan lisensinya tercantum di [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

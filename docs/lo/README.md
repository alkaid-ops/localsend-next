<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | **ລາວ** | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next ເປັນລູກຄ້າເດັສທັອບທີ່ເຂົ້າກັນໄດ້ກັບ LocalSend ທີ່ຂຽນດ້ວຍ Rust. ມັນເວົ້າພິທີການຢ່າງເປັນທາງການ, ສະນັ້ນມັນຄົ້ນພົບແລະໂອນໄຟລ໌ກັບ app LocalSend ຢ່າງເປັນທາງການໃນທັງສອງທິດທາງ, ໃນຂະນະທີ່ແລ່ນເປັນໂຄງການ desktop ພື້ນເມືອງແທນທີ່ຈະເປັນ app Flutter.

ດ້ານເຄືອຂ່າຍແມ່ນຫຼັກ LocalSend Rust, ຜູ້ຂາຍບໍ່ປ່ຽນແປງ. ທຸກຢ່າງທີ່ຢູ່ເໜືອມັນ — daemon, RPC layer, tray, terminal UI ແລະ Slint GUI — ແມ່ນ​ສ້າງ​ຂຶ້ນ​ຢູ່​ອ້ອມ​ຂ້າງ​ຫຼັກ​.

## ສາທິດ

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| ສົ່ງ | ການຕັ້ງຄ່າ |
| --- | --- |
| ![ສົ່ງ​ຫນ້າ​ທີ່​ມີ​ໄຟລ​໌​, ໂຟນ​ເດີ​, ຂໍ້​ຄວາມ​ແລະ​ແຫຼ່ງ clipboard ແລະ​ອຸ​ປະ​ກອນ​ທີ່​ໃກ້​ຄຽງ​](../../docs/images/readme-send-light.png​) | ![ໜ້າ​ການ​ຕັ້ງ​ຄ່າ​ທີ່​ມີ​ຫົວ​ຂໍ້, ພາ​ສາ, ເຄືອ​ຂ່າຍ ແລະ​ທາງ​ເລືອກ​ການ​ໂອນ](../../docs/images/readme-settings-light.png) |

## ຄຸນສົມບັດ

- ການຄົ້ນພົບແລະໂອນຕໍ່ກັບລູກຄ້າ LocalSend ຢ່າງເປັນທາງການ, ຢູ່ໃນຫຼັກແຫຼ່ງທີ່ຂາຍແລ້ວແທນທີ່ຈະເປັນການປະຕິບັດຄືນໃຫມ່
- ຫນຶ່ງໃນເຈົ້າຂອງລັດ canonical: daemon ຖືການຕັ້ງຄ່າ, ຕົວຕົນ, favorite, ປະຫວັດສາດ, ການໂອນແລະ TLS ຕົວຕົນ, ດັ່ງນັ້ນທຸກ frontend ເຫັນສິ່ງດຽວກັນ.
- GUI, terminal UI ແລະ tray ເປັນ frontends ເອກະລາດໃນໄລຍະດຽວກັນກັບທ້ອງຖິ່ນ RPC; ຖາດເປັນເຈົ້າຂອງ daemon ແລະ GUI process lifecycle
- ຄວາມຄືບໜ້າຕາມເວລາຈິງຕໍ່ໄຟລ໌, ປະຫວັດເປີດ ແລະເປີດເຜີຍການກະທຳ, ລາຍການທີ່ມັກ, ການເຂົ້າອຸປະກອນດ້ວຍຕົນເອງ, ແລະ Web Share ໄດ້ຮັບລິ້ງ
- 57 ພາສາໃນການໂຕ້ຕອບທີ່ກົງກັບລາຍຊື່ຂອງ LocalSend, ສາມາດສະຫຼັບໄດ້ໃນເວລາແລ່ນ ຫຼືຕິດຕາມສະຖານທີ່ຂອງລະບົບ.
- ການປະສົມປະສານຂອງເດັສທັອບເດີມ: ການແຈ້ງເຕືອນຈາກຖາດເທົ່ານັ້ນ, ເປີດຕົວໃນເວລາເຂົ້າສູ່ລະບົບ, ຫຍໍ້ລົງໃສ່ຖາດ, ການຈັດວາງໜ້າຕ່າງຄົງທີ່, ແລະໜ້າຈໍ macOS ທີ່ແຕ້ມໃສ່ແຖບຫົວຂໍ້ຂອງມັນ.
- ການຫຸ້ມຫໍ່ສໍາລັບ macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) ແລະ Windows (ZIP), ສໍາລັບທັງສອງ x86_64 ແລະ aarch64

## ສະຖາປັດຕະຍະກໍາ

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

daemon ແມ່ນຂະບວນການດຽວທີ່ສົນທະນາກັບເຄືອຂ່າຍຫຼືຂຽນສະຖານະຂອງແອັບພລິເຄຊັນ. Frontends ບໍ່ເຄີຍຂຶ້ນກັບຫຼັກຜູ້ຂາຍແລະບໍ່ເຄີຍ spawn daemon ດ້ວຍຕົນເອງ; ຖາດເຮັດ. ໄບຕ໌ຂອງໄຟລ໌ບໍ່ເດີນທາງເກີນ RPC — daemon ແກ້ໄຂເສັ້ນທາງ, hashes ເນື້ອຫາ ແລະບັນທຶກປະຫວັດ.

| ຄູ່ | ບົດບາດ |
| --- | --- |
| `localsendd` | ບໍລິການເຄືອຂ່າຍ ແລະສະຖານະແອັບພລິເຄຊັນ canonical |
| `localsend-tray` | Daemon ແລະ GUI ເຈົ້າຂອງວົງຈອນຊີວິດ, ການແຈ້ງເຕືອນ |
| `localsend-gui-slint` | ຢ່າງເປັນທາງການ GUI, Slint ກັບຕົວສະແດງ FemtoVG |
| `localsend-tui` | ດ້ານໜ້າຂອງສະຖານີ |
| `localsend-launcher` | launcher ຊັດເຈນສໍາລັບສະພາບແວດລ້ອມທີ່ບໍ່ມີຖາດ |
| `localsend-androidd` | ຮາກຜູ້ດຽວເທົ່ານັ້ນ Android ຮັບ daemon (MVP) |

ໄດເຣັກທໍຣີ `localsend-gui-egui`, `-fltk`, `-iced` ແລະ `-tauri` ໄດເຣັກທໍຣີເປັນພອດທົດລອງ ແລະຢູ່ຂ້າງນອກພື້ນທີ່ເຮັດວຽກຂອງຮາກຈົນກວ່າການລວມ RPC ຂອງພວກເຂົາສຳເລັດ.

## ດາວໂຫຼດ

ປ່ອຍຊັບສິນປະຕິບັດຕາມໂຄງການການຕັ້ງຊື່ — `LocalSend-<version>-<platform>-<arch>.<ext>`, ມີເວທີ `linux` / `windows` / `macos` ແລະສະຖາປັດຕະຍະກໍາ `x86_64` / `aarch64`:

| ເວທີ | ຊັບສິນ |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| ໃດ Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` ກວມເອົາທຸກຊັບສິນໃນການປ່ອຍ. ການກໍ່ສ້າງ macOS ທີ່ຜະລິດໂດຍ CI ແມ່ນເຊັນແບບສະເພາະ, ບໍ່ແມ່ນຜູ້ພັດທະນາ ID ໄດ້ເຊັນ ຫຼື notarised.

## ເລີ່ມໄວ

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

ແພັກເກດ Linux ຕິດຕັ້ງລາຍການ desktop ທີ່ເປີດ `localsend-tray`, ເຊິ່ງເປັນເຈົ້າຂອງ daemon lifecycle. ໃນເດັສທັອບທີ່ບໍ່ມີແຖບສະຖານະ ຖາດຈະເປີດ GUI ແທນ; `LocalSend --launcher` ເລືອກໂໝດນັ້ນຢ່າງຈະແຈ້ງຈາກ AppImage.

### Windows

Unzip ແລະດໍາເນີນການ `LocalSend.exe`. ຮວບຮວມຖື GUI, daemon, tray ແລະ launcher ຂ້າງຄຽງ; ໃຫ້ພວກເຂົາຮ່ວມກັນ.

## ພຶດຕິກໍາທີ່ຄວນຮູ້

- **ການອອກແມ່ນໜຶ່ງຄຳສັ່ງ.** ການເຊົາຈາກຖາດຈະຢຸດ GUI ທີ່ບັນທຶກໄວ້ໃນຂອບເຂດຈຸດສິ້ນສຸດ `gui.pid`, ຢຸດ daemon ທີ່ເປັນເຈົ້າຂອງພາຍໃນເວລາໝົດເວລາ, ແລະອອກໂດຍບໍ່ປ່ອຍໃຫ້ຂະບວນການ ຫຼືວຽກທີ່ເປີດໄວ້ຢູ່ເບື້ອງຫຼັງ.
- **ການແຈ້ງເຕືອນມາຈາກຖາດ.** GUI ບໍ່ເຄີຍປະກາດການແຈ້ງເຕືອນຂອງຕົນເອງ, ສະນັ້ນແຕ່ລະແພລດຟອມມີເສັ້ນທາງການຈັດສົ່ງອັນດຽວ.
- **ສະຖານະຂອງປ່ອງຢ້ຽມແມ່ນຕໍ່ຈຸດສິ້ນສຸດ.** ການຈັດວາງແມ່ນຖືກເກັບໄວ້ຂ້າງການຕັ້ງຄ່າ daemon ສໍາລັບຈຸດສິ້ນສຸດນັ້ນ ແທນທີ່ຈະຢູ່ໃນສະຖານທີ່ທົ່ວໂລກທີ່ໃຊ້ຮ່ວມກັນ, ດັ່ງນັ້ນສອງຕົວຢ່າງຈະບໍ່ຕໍ່ສູ້ກັບເລຂາຄະນິດຂອງປ່ອງຢ້ຽມ.
- **daemon ແມ່ນ authoritative.** ການ​ຕັ້ງ​ຄ່າ​ການ​ປ່ຽນ​ແປງ​ໃນ GUI ຖືກ​ສົ່ງ​ກັບ daemon ແລະ echoed ກັບ​ຄືນ​ໄປ​ບ່ອນ​ໂດຍ​ຜ່ານ​ການ​ຖ່າຍ​ຮູບ​; frontend ຮັກສາບໍ່ມີສໍາເນົາສ່ວນຕົວທີ່ສາມາດເລື່ອນໄດ້.

## ພາສາ

ອິນເຕີເຟດສົ່ງຢູ່ໃນ 57 ທ້ອງຖິ່ນ, ກົງກັບລາຍຊື່ຂອງ LocalSend, ໂດຍມີພາສາຈີນຕົວຫຍໍ້ເປັນພາສາ msgid. ເລືອກອັນໜຶ່ງພາຍໃຕ້ **ການຕັ້ງຄ່າ → ພາສາ**, ຫຼືປ່ອຍໃຫ້ມັນຢູ່ໃນ "ຕິດຕາມລະບົບ" ເພື່ອໃຊ້ OS locale.

| | |
| --- | --- |
| ແຄດຕາລັອກລວມ | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| ສະຕຣິງແຫຼ່ງ UI | `@tr("...")` ໃນ `apps/localsend-gui-slint/ui` |
| ລາຍການພາສາອັງກິດ | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust ຂໍ້ຄວາມ) |
| ຂໍ້ຄວາມຂ້າງ rust | `crates/localsend-i18n` (ສ້າງ `lookup_generated.rs`) |
| ກະຈົກ Upstream | `translations/upstream/<locale>.json` |

ສ້າງລາຍການຄືນໃໝ່ຫຼັງຈາກປ່ຽນສະຕຣິງ UI:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` ມີຄວາມຕັ້ງໃຈ, ດັ່ງນັ້ນລາຍການທີ່ສ້າງຂຶ້ນແມ່ນສາມາດຜະລິດແບບອອບໄລນ໌ໄດ້. ແກ້ໄຂການເຂົ້າຢູ່ທີ່ນັ້ນ ແລະແລ່ນເຄື່ອງກໍາເນີດຄືນໃໝ່ເພື່ອແກ້ໄຂການແປ.

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## ການກໍ່ສ້າງຈາກແຫຼ່ງ

ຄວາມຕ້ອງການ: Rust `1.97.1` (ປັກໝຸດໂດຍ `rust-toolchain.toml`) ແລະ, ສໍາລັບ Linux ກໍ່ສ້າງ, ຫົວຂໍ້ GTK3 ແລະ AppIndicator ການພັດທະນາ.

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

ຊຸດແພລະຕະຟອມ:

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

ສະຄຣິບທີ່ຍອມຮັບໄດ້ຂັບ binaries ທີ່ແທ້ຈິງແທນທີ່ຈະເປັນສະຖານະ mocked:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## ເອກະສານ

| ເອກະສານ | ເນື້ອໃນ |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | ການດໍາເນີນງານ, ການຫຸ້ມຫໍ່ແລະພຶດຕິກໍາເວທີ |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | ຫຼັກຖານການທົດສອບໃນປະຈຸບັນແລະພື້ນທີ່ທີ່ບໍ່ໄດ້ຮັບການຢັ້ງຢືນທີ່ຮູ້ຈັກ |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | ປະຕູຮັບທ້ອງຖິ່ນ |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | ການຖົດຖອຍຄືນໃຫມ່ແລະເສັ້ນທາງຫຼັກຖານທີ່ຄາດໄວ້ກ່ອນທີ່ຈະປ່ຽນແປງພຶດຕິກໍາ |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | ສະພາບລວມຂອງພາບທຽບກັບຕົ້ນແບບ OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | ໂມດູນຮາກ Android ແລະຂໍ້ຈໍາກັດຂອງມັນ |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | ການພິສູດທາງນ້ໍາຂອງຫຼັກຜູ້ຂາຍ |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

ບາງເອກະສານຂຽນເປັນພາສາຈີນ; ພວກ​ເຂົາ​ເຈົ້າ​ບັນ​ທຶກ​ການ​ດໍາ​ເນີນ​ການ​ຢັ້ງ​ຢືນ​ແລະ​ສົນ​ທິ​ສັນ​ຍາ​ໃນ​ຄໍາ​ສັບ​ຕ່າງໆ​ທີ່​ເຂົາ​ເຈົ້າ​ໄດ້​ຮັບ​ການ​ຕົກ​ລົງ​ໃນ.

## ຂອບເຂດ

ຜະລິດຕະພັນຕົ້ນຕໍແມ່ນລູກຄ້າ PC. Android ສະຫນັບສະຫນູນແມ່ນເປັນຮາກເທົ່ານັ້ນ, headless ຮັບ daemon ຈົນກ່ວາການສົ່ງເສີມຢ່າງຊັດເຈນເກີນຂອບເຂດທີ່ - ມັນບໍ່ມີ GUI, ເລີ່ມຕົ້ນບໍ່ມີການບໍລິການ foreground ແລະການໂພດບໍ່ມີການແຈ້ງເຕືອນຢ່າງຕໍ່ເນື່ອງ. `crates/localsend-official-core` ຖືກເກັບຮັກສາ byte-for-byte ຄືກັນກັບການສົ່ງອອກຕົ້ນທາງທີ່ບັນທຶກໄວ້, ນອກຈາກໄຟລ໌ `SHA256SUMS` ທີ່ສ້າງຂຶ້ນຂອງມັນ.

## ສິນເຊື່ອ

ໂປຣໂຕຄໍ, ຮູບແບບສາຍ ແລະ Rust ຫຼັກມາຈາກ [localsend/localsend](https://github.com/localsend/localsend); ໂຄງ​ການ​ນີ້​ສ້າງ​ລູກ​ຄ້າ desktop ໃນ​ການ​ເຮັດ​ວຽກ​ນັ້ນ​ແທນ​ທີ່​ຈະ​ເປັນ​ການ​ປະ​ຕິ​ບັດ​ອະ​ນຸ​ສັນ​ຍາ​ຄືນ​ໃຫມ່​. ຂໍຂອບໃຈກັບ LocalSend ຜູ້ຂຽນແລະຜູ້ປະກອບສ່ວນ.

ການໂຕ້ຕອບປະຕິບັດຕາມຕົ້ນແບບການອອກແບບທີ່ພັດທະນາສໍາລັບໂຄງການນີ້; ການອ້າງອີງທາງສາຍຕາຂອງມັນອາໄສຢູ່ໃນ [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/).

## ປະກອບສ່ວນ

- **ຂໍ້ບົກຜ່ອງ ແລະຄຳແນະນຳ**: ເປີດ [ບັນຫາ](https://github.com/alkaid-ops/localsend-next/issues) ດ້ວຍເວີຊັນ, ເວທີ, ຂັ້ນຕອນການສືບພັນ ແລະສິ່ງທີ່ທ່ານເຫັນ. ບັນທຶກ ຫຼືພາບໜ້າຈໍເຮັດໃຫ້ມັນໄວຂຶ້ນຫຼາຍໃນການດຳເນີນການ.
- ** ລະຫັດ **: ການຮ້ອງຂໍດຶງແມ່ນຍິນດີຕ້ອນຮັບ. ການປ່ຽນແປງທີ່ແຕະຕ້ອງຂອບເຂດ daemon/frontend, ຫຼັກການຂາຍ, ຫຼືສັນຍາພາບຄວນໄດ້ຮັບການປຶກສາຫາລືກ່ອນ - ຂໍ້ຈໍາກັດແມ່ນຖືກບັນທຶກໄວ້ໃນ [`AGENTS.md`](../../AGENTS.md).
- **ການແປ**: ລາຍການແມ່ນສ້າງຂຶ້ນຈາກຖານຄວາມຈຳການແປຂອງເຄື່ອງຈັກ, ສະນັ້ນ ການແກ້ໄຂແມ່ນເປັນການປະກອບສ່ວນທີ່ງ່າຍທີ່ສຸດໃນການກວດສອບ.

## ໃບອະນຸຍາດ

[Apache-2.0](../../LICENSE). ອົງປະກອບພາກສ່ວນທີສາມ ແລະໃບອະນຸຍາດຂອງພວກມັນຖືກລະບຸໄວ້ໃນ [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

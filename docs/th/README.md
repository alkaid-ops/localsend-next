<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | **ไทย** | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [简体中文](../../docs/zh-CN/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | [繁體中文（台灣）](../../docs/zh-TW/README.md)

</div>



localsend-next เป็นไคลเอนต์เดสก์ท็อปที่รองรับ LocalSend ซึ่งเขียนด้วย Rust พูดโปรโตคอลอย่างเป็นทางการ ดังนั้นจึงค้นพบและถ่ายโอนไฟล์ด้วยแอป LocalSend อย่างเป็นทางการในทั้งสองทิศทาง ในขณะที่ทำงานเป็นโปรแกรมเดสก์ท็อปดั้งเดิมแทนที่จะเป็นแอป Flutter

ฝั่งเครือข่ายเป็นคอร์อย่างเป็นทางการ LocalSend Rust ซึ่งจำหน่ายไม่เปลี่ยนแปลง ทุกสิ่งที่อยู่เหนือมัน — daemon, เลเยอร์ RPC, ถาด, UI เทอร์มินัล และ Slint GUI — ถูกสร้างขึ้นรอบๆ แกนหลักนั้น

## สาธิต

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| ส่ง | การตั้งค่า |
| --- | --- |
| ![ส่งเพจพร้อมไฟล์ โฟลเดอร์ ข้อความ และแหล่งที่มาของคลิปบอร์ด และอุปกรณ์ใกล้เคียง](../../docs/images/readme-send-light.png) | ![หน้าการตั้งค่าพร้อมธีม ภาษา เครือข่าย และตัวเลือกการถ่ายโอน](../../docs/images/readme-settings-light.png) |

## คุณสมบัติ

- ค้นหาและถ่ายโอนข้อมูลกับไคลเอนต์ LocalSend อย่างเป็นทางการบนคอร์อัพสตรีมที่จำหน่าย แทนที่จะนำไปปรับใช้ใหม่
- เจ้าของสถานะตามรูปแบบบัญญัติหนึ่งคน: daemon เก็บการตั้งค่า ข้อมูลประจำตัว รายการโปรด ประวัติ การถ่ายโอน และข้อมูลระบุตัวตน TLS ดังนั้นทุกส่วนหน้าจึงมองเห็นสิ่งเดียวกัน
- GUI, UI เทอร์มินัลและถาดเป็นส่วนหน้าอิสระเหนือ RPC ในเครื่องเดียวกัน ถาดเป็นเจ้าของ daemon และ GUI วงจรชีวิตกระบวนการ
- ความคืบหน้าต่อไฟล์แบบเรียลไทม์ ประวัติการเปิดและเปิดเผย รายการโปรด การป้อนข้อมูลอุปกรณ์ด้วยตนเอง และลิงก์การรับ Web Share
- ภาษาอินเทอร์เฟซ 57 ภาษาที่ตรงกับรายการอัปสตรีม LocalSend สามารถสลับได้ในขณะรันไทม์หรือตามภาษาของระบบ
- การรวมเดสก์ท็อปแบบเนทีฟ: การแจ้งเตือนจากถาดเท่านั้น เปิดเมื่อเข้าสู่ระบบ ย่อขนาดลงในถาด การคงอยู่ของตำแหน่งหน้าต่าง และหน้าต่าง macOS ที่ดึงเข้าไปในแถบชื่อเรื่อง
- บรรจุภัณฑ์สำหรับ macOS (DMG/ZIP), Linux (deb, rpm, AppImage, tarball) และ Windows (ZIP) สำหรับทั้ง x86_64 และ aarch64

## สถาปัตยกรรม

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

daemon เป็นกระบวนการเดียวที่สื่อสารกับเครือข่ายหรือเขียนสถานะแอปพลิเคชัน ส่วนหน้าไม่เคยขึ้นอยู่กับคอร์ที่จำหน่ายและไม่เคยวางไข่ daemon ด้วยตัวมันเอง ถาดทำ ไบต์ของไฟล์จะไม่เดินทางผ่าน RPC — daemon จะแก้ไขเส้นทาง แฮชเนื้อหา และประวัติการบันทึก

| ไบนารี่ | บทบาท |
| --- | --- |
| `localsendd` | บริการเครือข่ายและสถานะแอปพลิเคชันมาตรฐาน |
| `localsend-tray` | เจ้าของวงจรชีวิต Daemon และ GUI การแจ้งเตือน |
| `localsend-gui-slint` | GUI, Slint อย่างเป็นทางการพร้อมตัวเรนเดอร์ FemtoVG |
| `localsend-tui` | ส่วนหน้าของเทอร์มินัล |
| `localsend-launcher` | ตัวเรียกใช้ที่ชัดเจนสำหรับสภาพแวดล้อมที่ไม่มีถาด |
| `localsend-androidd` | รูทแบบสแตนด์อโลนเท่านั้น Android รับ daemon (MVP) |

ไดเร็กทอรี `localsend-gui-egui`, `-fltk`, `-iced` และ `-tauri` เป็นพอร์ตทดลองและจะอยู่นอกเวิร์กสเปซรูทจนกว่าการรวม RPC จะเสร็จสมบูรณ์

## ดาวน์โหลด

เนื้อหาที่เผยแพร่เป็นไปตามรูปแบบการตั้งชื่อเดียว — `LocalSend-<version>-<platform>-<arch>.<ext>` โดยมีแพลตฟอร์ม `linux` / `windows` / `macos` และสถาปัตยกรรม `x86_64` / `aarch64`:

| แพลตฟอร์ม | สินทรัพย์ |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`, `LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`, `LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`, `LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`, `LocalSend-0.1.3-linux-aarch64.rpm` |
| Linux ก็ได้ | `LocalSend-0.1.3-linux-x86_64.AppImage`, `LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` ครอบคลุมทุกเนื้อหาในการเผยแพร่ การสร้าง macOS ที่สร้างโดย CI นั้นมีการลงนามเฉพาะกิจ ไม่ใช่ ID นักพัฒนาที่ลงนามหรือรับรอง

## เริ่มต้นอย่างรวดเร็ว

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

แพ็คเกจ Linux ติดตั้งรายการเดสก์ท็อปที่เรียกใช้ `localsend-tray` ซึ่งเป็นเจ้าของวงจรการใช้งาน daemon บนเดสก์ท็อปที่ไม่มีแถบสถานะ ถาดจะเปิด GUI ไว้แทน `LocalSend --launcher` เลือกโหมดนั้นอย่างชัดเจนจาก AppImage

### Windows

แตกไฟล์แล้วรัน `LocalSend.exe` ไฟล์เก็บถาวรจะเก็บ GUI, daemon, ถาดและตัวเรียกใช้งานไว้เคียงข้างกัน เก็บไว้ด้วยกัน

## พฤติกรรมที่ควรรู้

- **การออกเป็นการดำเนินการเดียว** การออกจากถาดจะยุติ GUI ที่บันทึกไว้ในขอบเขตปลายทาง `gui.pid` หยุด daemon ที่เป็นเจ้าของภายในระยะหมดเวลาที่มีขอบเขต และออกโดยไม่ทิ้งกระบวนการหรืองาน Keep-alive ที่เปิดใช้ไว้เบื้องหลัง
- **การแจ้งเตือนมาจากถาด** GUI ไม่เคยโพสต์การแจ้งเตือนของตนเอง ดังนั้นแต่ละแพลตฟอร์มจึงมีเส้นทางการจัดส่งเดียว
- **สถานะของหน้าต่างเป็นไปตามตำแหน่งข้อมูล** ตำแหน่งจะถูกเก็บไว้ข้างการกำหนดค่า daemon สำหรับตำแหน่งข้อมูลนั้น แทนที่จะอยู่ในตำแหน่งส่วนกลางที่ใช้ร่วมกัน ดังนั้น 2 อินสแตนซ์จึงไม่ทะเลาะกันเรื่องเรขาคณิตของหน้าต่าง
- **เดมอนนั้นเชื่อถือได้** การตั้งค่าที่เปลี่ยนแปลงใน GUI จะถูกส่งไปยังดีมอนและสะท้อนกลับผ่านสแน็ปช็อต ส่วนหน้าไม่เก็บสำเนาส่วนตัวที่สามารถลอยได้

## ภาษา

อินเทอร์เฟซจัดส่งใน 57 ภาษา ซึ่งตรงกับรายการอัปสตรีม LocalSend โดยมีภาษาจีนตัวย่อเป็นภาษา msgid เลือกอันใดอันหนึ่งภายใต้ **การตั้งค่า → ภาษา** หรือปล่อยไว้ที่ "ติดตามระบบ" เพื่อใช้ภาษาของระบบปฏิบัติการ

| | |
| --- | --- |
| แคตตาล็อกที่รวมกลุ่ม | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| สตริงแหล่งที่มาของ UI | `@tr("...")` ใน `apps/localsend-gui-slint/ui` |
| แคตตาล็อกภาษาอังกฤษ | `translations/locale-en.json` (UI), `translations/locale-en-runtime.json` (Rust ข้อความ) |
| ข้อความด้านสนิม | `crates/localsend-i18n` (สร้าง `lookup_generated.rs`) |
| กระจกต้นน้ำ | `translations/upstream/<locale>.json` |

สร้างแค็ตตาล็อกใหม่หลังจากเปลี่ยนสตริง UI ใด ๆ:

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` มุ่งมั่น ดังนั้นแคตตาล็อกที่สร้างขึ้นจึงสามารถทำซ้ำแบบออฟไลน์ได้ แก้ไขรายการที่นั่นและรันตัวสร้างอีกครั้งเพื่อแก้ไขการแปล

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## การสร้างจากแหล่งที่มา

ข้อกำหนด: Rust `1.97.1` (ปักหมุดโดย `rust-toolchain.toml`) และสำหรับ Linux บิลด์ ส่วนหัว GTK3 และ AppIndicator การพัฒนา

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

แพ็คเกจแพลตฟอร์ม:

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

สคริปต์การยอมรับขับเคลื่อนไบนารีจริงแทนที่จะเป็นสถานะที่เยาะเย้ย:

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## เอกสารประกอบ

| เอกสาร | สารบัญ |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | พฤติกรรมการทำงาน บรรจุภัณฑ์ และแพลตฟอร์ม |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | หลักฐานการทดสอบในปัจจุบันและพื้นที่ที่ทราบที่ยังไม่ได้รับการยืนยัน |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | ประตูการยอมรับในท้องถิ่น |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | การถดถอยที่เกิดซ้ำและหลักฐานที่คาดไว้ก่อนที่จะเปลี่ยนพฤติกรรม |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | สถานะการบรรจบกันของภาพกับต้นแบบ OpenDesign |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | โมดูลรูท Android และขีดจำกัด |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | แหล่งที่มาต้นน้ำของแกนที่จำหน่าย |

README นี้เผยแพร่ในภาษาอินเทอร์เฟซทั้งหมด 57 ภาษา — เลือกหนึ่งภาษาจากตัวสลับที่ด้านบน การแปลถูกสร้างขึ้นจาก `README.md` โดย `scripts/translate-readme.mjs`; แก้ไขให้ถูกต้องใน [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json)

เอกสารบางฉบับเขียนเป็นภาษาจีน พวกเขาบันทึกการดำเนินการตรวจสอบและแบบแผนตามคำที่พวกเขาตกลงกันไว้

## ขอบเขต

ผลิตภัณฑ์หลักคือไคลเอนต์พีซี การสนับสนุน Android เป็น daemon รับแบบ headless เฉพาะรูทเท่านั้น จนกว่าจะได้รับการเลื่อนตำแหน่งอย่างชัดเจนเกินขอบเขตนั้น — ไม่มี GUI ไม่เริ่มบริการเบื้องหน้า และไม่มีโพสต์การแจ้งเตือนถาวร `crates/localsend-official-core` ถูกเก็บแบบไบต์ต่อไบต์เหมือนกับการส่งออกอัปสตรีมที่บันทึกไว้ นอกเหนือจากไฟล์ `SHA256SUMS` ที่สร้างขึ้น

## เครดิต

โปรโตคอล รูปแบบการส่งสัญญาณ และคอร์ Rust มาจาก [localsend/localsend](https://github.com/localsend/localsend); โปรเจ็กต์นี้สร้างไคลเอ็นต์เดสก์ท็อปในงานนั้นแทนที่จะนำโปรโตคอลไปใช้ใหม่ ขอขอบคุณผู้เขียนและผู้มีส่วนร่วม LocalSend

อินเทอร์เฟซเป็นไปตามต้นแบบการออกแบบที่พัฒนาขึ้นสำหรับโปรเจ็กต์นี้ การอ้างอิงภาพอยู่ใน [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/)

## มีส่วนร่วม

- **ข้อบกพร่องและข้อเสนอแนะ**: เปิด [ปัญหา](https://github.com/alkaid-ops/localsend-next/issues) พร้อมเวอร์ชัน แพลตฟอร์ม ขั้นตอนการทำซ้ำ และสิ่งที่คุณเห็น บันทึกหรือภาพหน้าจอช่วยให้ดำเนินการได้รวดเร็วยิ่งขึ้น
- **รหัส**: ยินดีรับคำขอดึง การเปลี่ยนแปลงที่กระทบกับขอบเขต daemon/frontend, แกนหลักที่จำหน่าย หรือสัญญาเชิงภาพควรได้รับการพูดคุยก่อน — ข้อจำกัดจะถูกบันทึกไว้ใน [`AGENTS.md`](../../AGENTS.md)
- **การแปล**: แคตตาล็อกสร้างขึ้นจากแคชการแปลด้วยเครื่องที่คอมมิต ดังนั้นการแก้ไขจึงเป็นวิธีที่ง่ายที่สุดในการตรวจสอบ

## ใบอนุญาต

[Apache-2.0](../../LICENSE) ส่วนประกอบของบุคคลที่สามและใบอนุญาตแสดงอยู่ใน [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md)

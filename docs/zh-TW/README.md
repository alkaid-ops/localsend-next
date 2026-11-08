<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../../docs/ar/README.md) | [Azərbaycanca](../../docs/az/README.md) | [Беларуская](../../docs/be/README.md) | [Български](../../docs/bg/README.md) | [বাংলা](../../docs/bn/README.md) | [Català](../../docs/ca/README.md) | [Čeština](../../docs/cs/README.md) | [Dansk](../../docs/da/README.md) | [Deutsch](../../docs/de/README.md) | [Ελληνικά](../../docs/el/README.md) | [English (India)](../../docs/en-IN/README.md) | [Español](../../docs/es-ES/README.md) | [Eesti](../../docs/et/README.md) | [Euskara](../../docs/eu/README.md) | [فارسی](../../docs/fa/README.md) | [Suomi](../../docs/fi/README.md) | [Filipino](../../docs/fil-PH/README.md) | [Français](../../docs/fr/README.md) | [Gaeilge](../../docs/ga/README.md) | [Galego](../../docs/gl/README.md) | [ગુજરાતી](../../docs/gu/README.md) | [עברית](../../docs/he/README.md) | [हिन्दी](../../docs/hi/README.md) | [Magyar](../../docs/hu/README.md) | [Հայերեն](../../docs/hy/README.md) | [Bahasa Indonesia](../../docs/id/README.md) | [Italiano](../../docs/it/README.md) | [日本語](../../docs/ja/README.md) | [ខ្មែរ](../../docs/km/README.md) | [한국어](../../docs/ko/README.md) | [Кыргызча](../../docs/ky/README.md) | [ລາວ](../../docs/lo/README.md) | [മലയാളം](../../docs/ml/README.md) | [Монгол](../../docs/mn/README.md) | [Bahasa Melayu](../../docs/ms/README.md) | [नेपाली](../../docs/ne/README.md) | [Nederlands](../../docs/nl/README.md) | [Polski](../../docs/pl/README.md) | [Português (Brasil)](../../docs/pt-BR/README.md) | [Português (Portugal)](../../docs/pt-PT/README.md) | [Română](../../docs/ro/README.md) | [Русский](../../docs/ru/README.md) | [සිංහල](../../docs/si/README.md) | [Slovenčina](../../docs/sk/README.md) | [Slovenščina](../../docs/sl/README.md) | [Српски](../../docs/sr/README.md) | [Српски (ћирилица)](../../docs/sr-Cyrl/README.md) | [Svenska](../../docs/sv/README.md) | [தமிழ்](../../docs/ta/README.md) | [ไทย](../../docs/th/README.md) | [Türkçe](../../docs/tr/README.md) | [Українська](../../docs/uk/README.md) | [اردو](../../docs/ur/README.md) | [Oʻzbekcha](../../docs/uz/README.md) | [Tiếng Việt](../../docs/vi/README.md) | [繁體中文（香港）](../../docs/zh-HK/README.md) | **繁體中文（台灣）**

</div>



localsend-next 是用Rust 編寫的與LocalSend 相容的桌面用戶端。它使用官方協議，因此它可以使用官方 LocalSend 應用程式在兩個方向上發現和傳輸文件，同時作為本機桌面程式而不是 Flutter 應用程式運行。

網路側是官方LocalSendRust核心，銷售量不變。它上面的所有東西——守護進程、RPC層、托盤、終端用戶界面和SlintGUI——都是圍繞著這個核心構建的。

## 示範

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="Receive page showing a discoverable device, the rotating sun indicator and the receive-via-link action" src="../../docs/images/readme-receive-light.png">
</picture>

| 傳送 | 設定 |
| --- | --- |
| ![發送包含文件、資料夾、文字和剪貼簿來源以及附近設備的頁面](../../docs/images/readme-send-light.png) | ![帶有主題、語言、網路和傳輸選項的設定頁](../../docs/images/readme-settings-light.png) |

## 功能

- 在供應的上游核心上針對官方 LocalSend 客戶端進行發現和傳輸，而不是重新實現
- 一個規範的狀態擁有者：守護程式保存設定、身份、收藏夾、歷史記錄、傳輸和 TLS 身份，因此每個前端都會看到相同的內容
- GUI，終端 UI 和托盤作為同一本地 RPC 上的獨立前端；托盤擁有守護進程和GUI進程生命週期
- 每個文件的即時進度、開啟和顯示操作的歷史記錄、收藏夾、手動設備輸入和 Web 共享接收鏈接
- 57 種介面語言與上游 LocalSend 清單匹配，可在運行時或遵循系統區域設定進行切換
- 本機桌面整合：僅來自托盤的通知、登入時啟動、最小化到托盤、視窗放置持久性以及繪製到標題列的 macOS 視窗
- macOS (DMG/ZIP)、Linux (deb、rpm、AppImage、tarball) 和 Windows (ZIP) 的包裝，適用於 x86_64 和aarch64

## 架構

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

守護程式是唯一與網路通訊或寫入應用程式狀態的進程。前端從不依賴供應商的核心，也從不產生守護程式本身；托盤可以。檔案位元組不會透過 RPC 傳輸 - 守護程式解析路徑、雜湊內容並記錄歷史記錄。

| 二進位 | 角色 |
| --- | --- |
| `localsendd` | 網路服務和規範的應用程式狀態 |
| `localsend-tray` | 守護程式和 GUI 生命週期擁有者、通知 |
| `localsend-gui-slint` | 正式的 GUI、Slint 和 FemtoVG 渲染器 |
| `localsend-tui` | 終端前端 |
| `localsend-launcher` | 適用於沒有托盤的環境的明確啟動器 |
| `localsend-androidd` | 獨立的僅限 root Android 接收守護程式 (MVP) |

`localsend-gui-egui`、`-fltk`、`-iced` 和 `-tauri` 目錄是實驗端口，在 RPC 整合完成之前一直保留在根工作區之外。

## 下載

發布資產遵循一種命名方案 - `LocalSend-<version>-<platform>-<arch>.<ext>`，平台為 `linux` / `windows` / `macos` 和架構 `x86_64` / `aarch64`：

| 平台 | 資產 |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`、`LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`、`LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`、`LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`、`LocalSend-0.1.3-linux-aarch64.rpm` |
| 任意Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`、`LocalSend-0.1.3-linux-aarch64.AppImage` |

`SHA256SUMS.txt` 涵蓋了版本中的所有資產。 CI 產生的 macOS 建置是暫時簽署的，而不是開發人員 ID 簽章或公證的。

## 快速啟動

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

Linux 軟體包安裝一個啟動 `localsend-tray` 的桌面條目，該條目擁有守護程式生命週期。在沒有狀態列的桌面上，托盤使 GUI 保持打開狀態；`LocalSend --launcher` 從 AppImage 中明確選擇該模式。

### Windows

解壓縮並運行`LocalSend.exe`。存檔並排保存GUI、守護進程、托盤和啟動器；讓他們在一起。

## 值得了解的行為

- **退出是一項操作。 ** 從托盤退出會終止端點範圍`gui.pid`中記錄的GUI，在有限的超時時間內停止擁有的守護進程，並退出而不留下進程或已啟動的保持活動作業。
- **通知來自托盤。 ** GUI 從不發布自己的通知，因此每個平台都有單一的傳遞路徑。
- **視窗狀態是每個端點的。 ** 放置儲存在該端點的守護程式配置旁邊，而不是儲存在共用的全域位置中，因此兩個實例不會爭奪視窗幾何形狀。
- **守護程式是權威的。 ** GUI 中更改的設定將提交給守護程序並透過快照回顯；前端不保留可能漂移的私人副本。

## 語言

此介面提供 57 種語言環境，與上游 LocalSend 的清單匹配，並使用簡體中文作為 msgid 語言。在 **設定 → 語言** 下選擇一項，或將其保留在「遵循系統」上以使用作業系統區域設定。

| | |
| --- | --- |
| 捆綁目錄 | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| UI 來源字串 | `@tr("...")`在`apps/localsend-gui-slint/ui` |
| 英文目錄 | `translations/locale-en.json`（使用者介面）、`translations/locale-en-runtime.json`（Rust 訊息） |
| Rust 端訊息 | `crates/localsend-i18n`（生成`lookup_generated.rs`） |
| 上游鏡像 | `translations/upstream/<locale>.json` |

更改任何 UI 字串後重新產生目錄：

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # fills the machine-translation cache
node scripts/update-slint-translations.mjs  # writes the .po files and the Rust table
```

`locale-machine-cache.json` 已提交，因此產生的目錄可離線重現。修正那裡的條目並重新運行生成器以修復翻譯。

<details>
<summary>Bundled locales</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## 從原始碼建置

要求：Rust `1.97.1`（由`rust-toolchain.toml`固定），​​對於Linux構建，GTK3和AppIndicator開發標頭。

```bash
cargo build --release            # workspace binaries
cargo test --locked              # per-crate tests
cargo clippy --all-targets --no-deps -- -D warnings
```

平台包：

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

接受腳本驅動真正的二進位檔案而不是模擬狀態：

```bash
./scripts/accept-slint.sh   # GUI journeys against a live daemon
./scripts/accept-tui.sh     # terminal frontend
```

## 文件

| 文件 | 內容 |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | 營運、包裝和平台行為 |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | 目前的測試證據和已知的未經驗證的區域 |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | 當地受理門 |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | 反覆出現的回歸和改變行為之前預期的證據線索 |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | 與 OpenDesign 原型的視覺融合狀態 |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Android根模組及其限制 |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | 供應核心的上游來源 |

This README is published in all 57 interface languages — pick one from the switcher at the top. The translations are generated from `README.md` by `scripts/translate-readme.mjs`; corrections go in [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json).

有些文件是用中文寫的；他們以商定的措詞記錄驗證運行和約定。

## 適用範圍

主要產品是PC客戶端。 Android 支援是一個僅限 root 的無頭接收守護進程，直到明確提升到超出該範圍為止 - 它沒有 GUI，不啟動前台服務，也不發布持久通知。除了產生的 `SHA256SUMS` 檔案之外，`crates/localsend-official-core` 與其記錄的上游匯出逐位元組保持相同。

## 致謝

協定、有線格式和Rust核心來自[localsend/localsend](https://github.com/localsend/localsend)；該專案在此基礎上建置了一個桌面用戶端，而不是重新實作協定。感謝LocalSend 作者和貢獻者。

此介面遵循為本專案開發的設計原型；它的視覺參考位於[`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/)。

## 參與貢獻

- **錯誤與建議**：開啟一個[問題](https://github.com/alkaid-ops/localsend-next/issues)，其中包含版本、平台、複製步驟和您所看到的內容。日誌或螢幕截圖可以讓您更快採取行動。
- **代碼**：歡迎請求請求。應先討論觸及daemon/frontend邊界、供應核心或視覺契約的變更－約束記錄在[`AGENTS.md`](../../AGENTS.md)。
- **翻譯**：目錄是從提交的機器翻譯快取生成的，因此更正是最容易審查的貢獻。

## 執照

[Apache-2.0](../../LICENSE)。第三方元件及其授權列在 [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md) 中。

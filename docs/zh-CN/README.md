<div align="center">
    <img src="../../docs/images/logo.png" alt="localsend-next logo" width="128" height="128">

<h1>localsend-next</h1>

[![Latest release](https://img.shields.io/github/v/release/alkaid-ops/localsend-next?style=flat-square)](https://github.com/alkaid-ops/localsend-next/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](../../LICENSE)
[![Release packages](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml/badge.svg)](https://github.com/alkaid-ops/localsend-next/actions/workflows/release.yml)

</div>

<div align="center">

[English](../../README.md) | [العربية](../ar/README.md) | [Azərbaycanca](../az/README.md) | [Беларуская](../be/README.md) | [Български](../bg/README.md) | [বাংলা](../bn/README.md) | [Català](../ca/README.md) | [Čeština](../cs/README.md) | [Dansk](../da/README.md) | [Deutsch](../de/README.md) | [Ελληνικά](../el/README.md) | [English (India)](../en-IN/README.md) | [Español](../es-ES/README.md) | [Eesti](../et/README.md) | [Euskara](../eu/README.md) | [فارسی](../fa/README.md) | [Suomi](../fi/README.md) | [Filipino](../fil-PH/README.md) | [Français](../fr/README.md) | [Gaeilge](../ga/README.md) | [Galego](../gl/README.md) | [ગુજરાતી](../gu/README.md) | [עברית](../he/README.md) | [हिन्दी](../hi/README.md) | [Magyar](../hu/README.md) | [Հայերեն](../hy/README.md) | [Bahasa Indonesia](../id/README.md) | [Italiano](../it/README.md) | [日本語](../ja/README.md) | [ខ្មែរ](../km/README.md) | [한국어](../ko/README.md) | [Кыргызча](../ky/README.md) | [ລາວ](../lo/README.md) | [മലയാളം](../ml/README.md) | [Монгол](../mn/README.md) | [Bahasa Melayu](../ms/README.md) | [नेपाली](../ne/README.md) | [Nederlands](../nl/README.md) | [Polski](../pl/README.md) | [Português (Brasil)](../pt-BR/README.md) | [Português (Portugal)](../pt-PT/README.md) | [Română](../ro/README.md) | [Русский](../ru/README.md) | [සිංහල](../si/README.md) | [Slovenčina](../sk/README.md) | [Slovenščina](../sl/README.md) | [Српски](../sr/README.md) | [Српски (ћирилица)](../sr-Cyrl/README.md) | [Svenska](../sv/README.md) | [தமிழ்](../ta/README.md) | [ไทย](../th/README.md) | [Türkçe](../tr/README.md) | [Українська](../uk/README.md) | [اردو](../ur/README.md) | [Oʻzbekcha](../uz/README.md) | [Tiếng Việt](../vi/README.md) | [繁體中文（香港）](../zh-HK/README.md) | [繁體中文（台灣）](../zh-TW/README.md)

</div>

localsend-next 是一个用 Rust 编写的 LocalSend 兼容桌面客户端。它使用官方协议，因此能与官方 LocalSend 应用双向发现和传输文件，同时以原生桌面程序的形式运行，而不是 Flutter 应用。

网络层直接使用官方 LocalSend 的 Rust 核心，原样引入、不做改动。核心之上的一切——守护进程、RPC 层、托盘、终端界面和 Slint GUI——都围绕这个核心构建。

## 演示

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../docs/images/readme-receive-dark.png">
  <img alt="接收页，显示可发现的设备、旋转的太阳指示器和通过链接接收按钮" src="../../docs/images/readme-receive-light.png">
</picture>

| 发送 | 设置 |
| --- | --- |
| ![发送页，包含文件、文件夹、文本、剪贴板四种入口和附近设备列表](../../docs/images/readme-send-light.png) | ![设置页，包含主题、语言、网络和传输选项](../../docs/images/readme-settings-light.png) |

## 功能

- 与官方 LocalSend 客户端互相发现和传输，底层是原样引入的上游核心，而不是重新实现的协议
- 单一状态权威：守护进程持有设置、身份、收藏、历史、传输和 TLS 身份，所有前端看到的是同一份状态
- GUI、终端界面和托盘是同一套本地 RPC 之上的独立前端；托盘负责守护进程与 GUI 的进程生命周期
- 实时逐文件进度、可打开与定位的历史记录、收藏设备、手动添加设备，以及 Web Share 接收链接
- 57 个界面语言，与上游 LocalSend 的语言列表一致，可运行时切换或跟随系统
- 原生桌面集成：通知只由托盘发出、登录时启动、关闭最小化到托盘、窗口位置记忆，以及内容绘制进标题栏的 macOS 窗口
- 覆盖 macOS（DMG/ZIP）、Linux（deb、rpm、AppImage、tar 包）和 Windows（ZIP），x86_64 与 aarch64 各有一套

## 架构

```text
      Slint GUI · TUI · tray
                │
      localsend-rpc (local socket)
                │
          localsendd (daemon)
                │
   official LocalSend core (vendored)
```

守护进程是唯一与网络通信、唯一写入应用状态的进程。前端不依赖原样引入的核心，也不会自行启动守护进程——那是托盘的工作。文件字节不经过 RPC：路径解析、内容哈希和历史记录都在守护进程内完成。

| 可执行文件 | 职责 |
| --- | --- |
| `localsendd` | 网络服务与应用状态的唯一权威 |
| `localsend-tray` | 守护进程与 GUI 的生命周期、系统通知 |
| `localsend-gui-slint` | 正式 GUI，Slint + FemtoVG 渲染器 |
| `localsend-tui` | 终端前端 |
| `localsend-launcher` | 在没有托盘的环境里显式启动 |
| `localsend-androidd` | 独立的 root-only Android 接收守护进程（MVP） |

`localsend-gui-egui`、`-fltk`、`-iced`、`-tauri` 是实验性移植，在 RPC 集成完成前不在根工作区内。

## 下载

发布产物的命名规则统一为 `LocalSend-<版本>-<平台>-<架构>.<扩展名>`，平台取 `linux` / `windows` / `macos`，架构取 `x86_64` / `aarch64`：

| 平台 | 产物 |
| --- | --- |
| macOS | `LocalSend-0.1.3-macos-aarch64.dmg`、`LocalSend-0.1.3-macos-x86_64.dmg` |
| Windows | `LocalSend-0.1.3-windows-x86_64.zip`、`LocalSend-0.1.3-windows-aarch64.zip` |
| Debian / Ubuntu | `LocalSend-0.1.3-linux-x86_64.deb`、`LocalSend-0.1.3-linux-aarch64.deb` |
| Fedora / RHEL | `LocalSend-0.1.3-linux-x86_64.rpm`、`LocalSend-0.1.3-linux-aarch64.rpm` |
| 任意 Linux | `LocalSend-0.1.3-linux-x86_64.AppImage`、`LocalSend-0.1.3-linux-aarch64.AppImage` |

每次发布都附带 `SHA256SUMS.txt`，覆盖全部产物。CI 产出的 macOS 版本使用 ad-hoc 签名，不是 Developer ID 签名，也没有公证。

## 快速开始

### macOS

```bash
open LocalSend-<version>-macos-<arch>.dmg
# 把 LocalSend.app 拖进"应用程序"
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

Linux 安装包会注册一个启动 `localsend-tray` 的桌面入口，由托盘管理守护进程生命周期。在没有状态栏的环境里，托盘会改为保持 GUI 打开；AppImage 下可用 `LocalSend --launcher` 显式选择这一模式。

### Windows

解压后运行 `LocalSend.exe`。压缩包内 GUI、守护进程、托盘和启动器并排放置，请保持它们在同一目录。

## 值得了解的行为

- **退出是一个整体操作。** 从托盘退出会终止记录在端点级 `gui.pid` 中的 GUI，在限定超时内停掉它拥有的守护进程，然后退出，不留下残余进程或 launchd 保活任务。
- **通知只由托盘发出。** GUI 从不自行发送通知，因此每个平台只有一条投递路径。
- **窗口状态按端点保存。** 位置信息与对应端点的守护进程配置放在一起，而不是放在全局共享位置，这样两个实例不会争抢窗口几何。
- **守护进程是权威。** GUI 中改动的设置会提交给守护进程，再通过快照回传；前端不会保留可能与真实状态漂移的私有副本。

## 多语言

界面提供 57 个语言，与上游 LocalSend 的列表一致，以简体中文作为词条源语言（msgid）。在**设置 → 语言**中选择，或保持"跟随系统"以使用系统语言。

| | |
| --- | --- |
| 打包的词条 | `apps/localsend-gui-slint/translations/<locale>/LC_MESSAGES/localsend-gui-slint.po` |
| 界面源字符串 | `apps/localsend-gui-slint/ui` 中的 `@tr("...")` |
| 英文词条 | `translations/locale-en.json`（界面）、`translations/locale-en-runtime.json`（Rust 消息） |
| Rust 侧消息 | `crates/localsend-i18n`（生成的 `lookup_generated.rs`） |
| 上游镜像 | `translations/upstream/<locale>.json` |

修改任何界面字符串后重新生成词条：

```bash
node scripts/update-slint-translations.mjs --collect > /tmp/collect.json
node scripts/translate-slint-locales.mjs    # 填充机器翻译缓存
node scripts/update-slint-translations.mjs  # 写出 .po 文件和 Rust 表
```

`locale-machine-cache.json` 已入库，因此生成的词条可离线复现。要修正某条翻译，改缓存里对应条目后重跑生成器即可。

<details>
<summary>已打包的语言</summary>

`ar` `az` `be` `bg` `bn` `ca` `cs` `da` `de` `el` `en-IN` `en-US` `es-ES` `et` `eu` `fa` `fi` `fil-PH` `fr` `ga` `gl` `gu` `he` `hi` `hu` `hy` `id` `it` `ja` `km` `ko` `ky` `lo` `ml` `mn` `ms` `ne` `nl` `pl` `pt-BR` `pt-PT` `ro` `ru` `si` `sk` `sl` `sr` `sr-Cyrl` `sv` `ta` `th` `tr` `uk` `ur` `uz` `vi` `zh-CN` `zh-HK` `zh-TW`

</details>

## 从源码构建

需要 Rust `1.97.1`（由 `rust-toolchain.toml` 固定）；构建 Linux 版本还需要 GTK3 和 AppIndicator 的开发头文件。

```bash
cargo build --release            # 工作区二进制
cargo test --locked              # 各 crate 的测试
cargo clippy --all-targets --no-deps -- -D warnings
```

平台打包：

```bash
# macOS（需在对应架构的主机上）
cd apps/localsend-gui-slint && PACKAGE_VERSION=0.1.3 ./package-macos.sh

# Linux
./scripts/package-linux.sh x86_64      # deb + tar 包
./scripts/package-rpm.sh x86_64        # rpm
./scripts/package-appimage.sh x86_64   # AppImage

# Windows（PowerShell）
./scripts/package-windows.ps1 -Architecture x64
```

验收脚本直接驱动真实二进制，而不是模拟状态：

```bash
./scripts/accept-slint.sh   # 针对实时守护进程的 GUI 流程
./scripts/accept-tui.sh     # 终端前端
```

## 文档

| 文档 | 内容 |
| --- | --- |
| [`docs/desktop-pc.md`](../../docs/desktop-pc.md) | 运行、打包与平台行为 |
| [`docs/verification.zh-CN.md`](../../docs/verification.zh-CN.md) | 当前测试证据与已知未验证项 |
| [`docs/acceptance-automation.zh-CN.md`](../../docs/acceptance-automation.zh-CN.md) | 本地验收门禁 |
| [`docs/agent-regression-playbook.zh-CN.md`](../../docs/agent-regression-playbook.zh-CN.md) | 反复出现的回归，以及改动行为前应留下的证据链 |
| [`docs/HANDOFF_CLAUDE.zh-CN.md`](../../docs/HANDOFF_CLAUDE.zh-CN.md) | 相对 OpenDesign 原型的视觉收敛状态 |
| [`docs/android-root-mvp.zh-CN.md`](../../docs/android-root-mvp.zh-CN.md) | Android root 模块及其边界 |
| [`VENDORED_CORE.md`](../../VENDORED_CORE.md) | 原样引入核心的上游出处 |

本 README 以全部 57 个界面语言发布，可在顶部的语言切换中选择。译文由 `scripts/translate-readme.mjs` 从 `README.md` 生成；修正写在 [`docs/i18n-overrides.json`](../../docs/i18n-overrides.json) 中。

部分文档使用中文撰写；它们以当初商定的表述记录验证过程与约定。

## 范围

主要产品是 PC 客户端。Android 支持是一个 root-only、无界面的接收守护进程，在明确提升范围之前：它没有 GUI，不启动前台服务，也不创建常驻通知。`crates/localsend-official-core` 与记录在案的上游导出版本逐字节一致，仅其生成的 `SHA256SUMS` 文件除外。

## 致谢

协议、线格式与 Rust 核心来自 [localsend/localsend](https://github.com/localsend/localsend)；本项目在这份工作之上构建桌面客户端，而不是重新实现协议。感谢 LocalSend 的作者与贡献者。

界面遵循为本项目开发的设计原型，其视觉参考位于 [`apps/localsend-gui-slint/reference/open-design/`](../../apps/localsend-gui-slint/reference/open-design/)。

## 参与贡献

- **问题与建议**：在本仓库的 [Issues](https://github.com/alkaid-ops/localsend-next/issues) 提交，附上版本、平台、复现步骤和你看到的现象。日志或截图能显著加快处理。
- **代码**：欢迎 pull request。涉及守护进程/前端边界、原样引入的核心或视觉契约的改动，建议先讨论——相关约束记录在 [`AGENTS.md`](../../AGENTS.md)。
- **翻译**：词条由已入库的机器翻译缓存生成，直接修正缓存是最容易审查的贡献方式。

## 许可

[Apache-2.0](../../LICENSE)。第三方组件及其许可见 [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md)。

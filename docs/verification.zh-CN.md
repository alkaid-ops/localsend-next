# 迁移验证记录

日期：2026-10-20

## Vendored core

- 来源：官方 `localsend/localsend` tag `v1.18.2`
- commit：`af0416be50770a97760f7070684bc667b759a15c`
- `./scripts/verify-vendored-core.sh`：61 个文件哈希全部匹配

## 自建 workspace

通过的测试：

```text
cargo test --workspace --exclude localsend --no-fail-fast
localsend-platform: 1 passed
localsend-rpc: 3 passed
localsend-service: 1 loopback integration passed
localsend-tui: 2 passed
localsend-tray: 1 passed
localsend-gui-slint: 9 passed
localsendd: 1 lifecycle integration passed
```

`cargo check --workspace` 通过。

自有代码 clippy 通过：

```sh
cargo clippy --workspace --exclude localsend --all-targets --no-deps -- -D warnings
```

注意：官方 core 在 Rust 1.97/Clippy 1.97 下自身有若干历史 lint，不能在不修改 vendored 源码的前提下通过 `-D warnings`。

## 官方 core 测试

- 78 个 lib tests：通过
- `accept_resilience`、`discovery`、`hash`、`internal_server`、`multicast`、`v2_server`、`v2_tls_pinning`、`v2_web_send`：通过
- `event_backpressure::register_keeps_answering_when_events_are_not_consumed`：通过
- `event_backpressure::subnet_scan_finishes_when_events_are_not_consumed`：macOS 稳定失败

失败信息：

```text
the scan must return every confirmed device, got 1 for a channel of 16
```

该测试假设 `127.0.0.x` 的每个地址都会被扫描器视为独立可确认设备；当前 macOS
网络栈下只得到 1 个结果。vendored 源码保持未修改，该上游平台差异记录为已知限制。

## macOS release

- FemtoVG release 构建成功。
- App：`target/release/bundle/macos/LocalSend.app`
- ZIP：`target/release/bundle/macos/LocalSend-macos-arm64.zip`
- GUI、`localsendd`、`localsend-tray` 均为 arm64。
- `codesign --verify --deep --strict` 通过。
- 使用隔离配置启动打包后的 GUI，Receive 页面正常显示本地 alias、端口和服务器运行状态。
- 结束 GUI 进程后，独立 daemon 继续存活。
- 打包 tray 在 daemon 不存在时成功创建 owned `localsendd`；tray 退出后 daemon、socket 和 runtime manifest 均自动清理。

## 全流程自动化门禁

- `./scripts/accept-tui.sh`：3 次连续运行通过；真实 PTY 全流程、TestBackend 三尺寸渲染和官方 core peer 全部通过。
- `./scripts/accept-slint.sh`：3 次连续运行通过；Slint accessible tree、真实 daemon worker、32 张 golden、FemtoVG release、codesign 和 AXUIElement 页面切换全部通过。
- 自动验收的具体覆盖与产物路径见 `docs/acceptance-automation.zh-CN.md`。

## Discovery 兼容修复

- LocalSend v1.18.2 固定使用 UDP `53317` 和 IPv4 组播 `224.0.0.167`。
- 修复前 daemon 在 HTTP 端口非 0 时把 HTTP 端口误当作组播端口，导致原版客户端无法发现。
- 现在组播端口始终为 `53317`；配置了旧地址 `239.255.88.61` 时同时加入该组和官方默认组。
- `multicast_discovery` 集成测试使用两个不同 HTTP 端口、一个旧组和一个默认组，双向发现通过。
- 实测 HTTP 监听 `53532` 时，daemon 仍持有 IPv4 `53317` 的旧组/官方组 socket 和 IPv6 `53317` socket；不再出现 `53532/UDP`。

## 官方客户端互操作冒烟

使用本机已安装的官方 LocalSend 1.18.2 补测：

- 官方客户端随机监听 `127.0.0.1:53532`，协议为 HTTP。
- 通过 RPC `AddManualDevice` 可以成功读取官方 `/api/localsend/v2/info`，设备进入 Slint 发送页。
- Slint 正确显示官方设备 alias、HTTP、macOS 型号和 `127.0.0.1` 地址。
- 从 daemon 发送文本到官方客户端时，官方客户端返回 `401` PIN challenge；daemon 将发送标记为 `failed`，证明请求已到达官方协议端点。
- 未输入官方客户端 PIN，因此本轮没有执行成功传输、拒绝和取消的完整官方 UI 矩阵。

## 实验 GUI 目录

当前应用目录包含：

- `apps/localsend-gui-slint`：正式 GUI，仍在 root workspace 和发布链路中。
- `apps/localsend-gui-egui`
- `apps/localsend-gui-fltk`
- `apps/localsend-gui-iced`
- `apps/localsend-gui-tauri`

四个实验 GUI 已从 root workspace 排除，避免未完成实现影响正式构建和后续 CI。它们已经改为依赖
`localsend-rpc`，不再引用旧的 `localsend-rs` 项目。

独立 `cargo check --manifest-path ...` 结果：

- egui：通过
- Iced：通过
- Tauri：通过
- FLTK：通过；首次 bundled SDK 下载因 HTTP/2 中断失败，使用本地代理重试后通过

这些检查只证明原型可以编译，不代表功能、视觉或跨平台验收完成。

## Android Root Native Boot Daemon MVP

日期：2026-10-23

- `apps/localsend-androidd`：11 个单元测试通过。
- `cargo check --workspace --locked`：通过。
- `cargo clippy -p localsend-androidd -p localsend-service --all-targets --locked --no-deps -- -D warnings`：通过。
- 模块 shell 语法、目录结构和禁用开关静态检查通过。
- Android NDK r30 arm64 release 交叉编译：通过。
- 模块 ZIP：`target/release/packages/localsend-next-root-aarch64.zip`，4.8 MiB，
  SHA-256 `47a9438ea19eb0e97929212bf40a71ef99049bd38dd5b49e85b20cfdcbb1fcbf`。
- native binary：ELF64 AArch64，15 MiB，SHA-256
  `d2eda8b422c08331a0b587e6c7b0543ef4d9722b07990f66d72ce3a23c4be273`。
- 模块目录、shell 语法和无 `system/bin`/`init.rc` 结构检查：通过。
- 真机运行时验收：`BLOCKED`，当前没有连接 Android 设备。

阻断产物记录在 `target/acceptance/android-root/`。在真实 root 设备上完成重启、
常驻、通知、灭屏、LocalSend 互通和 watchdog 测试前，不得将该 MVP 标记为通过。

## Linux deb / AppImage 真机验收

日期：2026-10-24

测试机：antiX / Debian GNU/Linux 13，x86_64，icewm，X11 `DISPLAY=:0.0`。

产物：

- deb：`localsend_0.1.0_amd64.deb`，SHA-256
  `e953a92872235504df62e8de68c9fa90e9b0e4307b6c44c90ad74321657d4c28`
- AppImage：`LocalSend-0.1.0-linux-x86_64.AppImage`，SHA-256
  `95b9e31189dcf486e29acebad76a35769bd8b4cb5354f3ecef22014f32c66d66`

通过的场景：

- deb 安装成功，桌面入口为 `localsend-tray`；tray 启动 owned `localsendd`，
  IPv4/IPv6 TCP 和 UDP `53317` 均正常监听。
- GTK3 文件选择器在未安装/未运行 xdg-desktop-portal、zenity 的环境中正常打开，
  并能选中文件。
- deb GUI 向确定性 peer 发送文件成功，源文件与接收文件 SHA-256 一致。
- deb GUI 发送文本消息成功，peer 收到正确文本 preview。
- 确定性 peer 向 deb 发送文件和文本成功；文件落盘哈希一致，文本以历史消息记录。
- 接收请求页面正常显示 sender、文件、拒绝/接受所选/全部接受操作。
- GUI 关闭后 tray 和 daemon 继续运行并保持端口监听。
- 真实 tray 菜单“打开主界面”可启动 GUI；菜单“退出”后 GUI、tray、daemon 和端口全部清理。
- AppImage 可直接启动 tray 和 owned daemon，GUI、GTK3 文件选择器、文件发送、
  文件/文本接收、历史记录和 tray 退出清理均正常。

验收截图和 peer 状态保存在 `target/acceptance/linux-antiX-20261024/`。

同一轮真机测试发现并修复：

- 安装版 GUI 命令为 `/usr/bin/localsend --window`，原进程识别遗漏该形式，导致
  tray 退出后 GUI 残留。
- Linux GUI/tray 的 `rfd` 默认 portal 后端在 antiX 上不可用，已改为 GTK3。
- `.txt` 普通文件曾被按 MIME 类型误报为“文本已发送”，现仅 preview 非空的文本消息
  使用文本文案。

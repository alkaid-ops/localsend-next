# localsend-next PC 桌面架构

## 进程边界

```text
localsend-gui-slint / localsend-tui / localsend-tray
                       |
                localsend-rpc
                       |
                  localsendd
                       |
        crates/localsend-official-core
```

- `localsendd` 是唯一持有官方 LocalSend core 的进程。
- `localsend-rpc` 提供 Unix socket / Windows named pipe、长度前缀 JSON、RPC v2、snapshot 和事件流。
- `localsend-service` 将官方 `DiscoveryHandle`、HTTP server/client、TLS identity、Web Share 和 WebRTC 映射为稳定的桌面协议。
- Slint、TUI 和 tray 只通过 RPC 工作，不直接依赖 vendored core。

## 生命周期

- tray 先取得单实例锁，再连接 daemon；daemon 不存在时由 tray 以 owner lease 模式启动。
- Linux 无 AppIndicator 时，桌面入口使用 `localsend-launcher`：launcher 是生命周期所有者，启动 daemon 和 GUI，并在 GUI 退出后清理自己创建的 daemon。
- `localsend-launcher --background` 只启动持久 daemon，然后在 Linux 登录自启动场景中立即退出；GUI 设置页可关闭该后台服务。
- Slint GUI 是纯前端。GUI 关闭不会停止 daemon，GUI 也不自行拉起 daemon。
- 直接运行 `localsend-tui` 只连接已有 daemon；daemon 未运行时显示错误并重试。
- 直接运行 `localsendd` 创建 Persistent daemon，不跟随 tray 或 GUI 退出。
- tray 退出时取消活动传输、请求 owned daemon 退出并等待 IPC endpoint 释放。
- `Open LocalSend` 只打开 GUI；`Show status` 只刷新托盘状态，不创建窗口；`Quit` 不打开 GUI。

## 数据目录

默认沿用旧 daemon 的 `org.LocalSend.localsend-rs` 目录，迁移不会重建身份、设置或历史：

- `settings.json`
- `identity.pem`
- `favorites.json`
- `history.jsonl`
- `runtime.json`

测试可用 `LOCALSEND_CONFIG_DIR` 和 `LOCALSEND_RPC_SOCKET` 完全隔离。

## 前端

正式 GUI 是 `apps/localsend-gui-slint`，release 固定使用 `winit-femtovg-wgpu`。macOS 上 wgpu 使用 Metal 后端。接收、发送、设置三页保持原 LocalSend 结构。

`localsend-gui-egui`、`localsend-gui-fltk`、`localsend-gui-iced` 和
`localsend-gui-tauri` 保留为实验目录，暂不进入 workspace 和发布链路。

TUI 主要键位：

- Receive: `j/k`、`Space`、`y`、`Y`、`n`、`x`、`PgUp/PgDn`、`Enter`。
- Send: `r` 刷新、`m` 手动 IP、`f` 文件浏览器、`t` 文本、`F` 收藏、`e` 编辑收藏。
- Settings: `j/k`、`Enter`、`s` 启停服务器、`h` 切换 HTTP/HTTPS、`a` 切换 auto-save。
- Global: `1/2/3` 切页、`?` 帮助、`q`/`Ctrl+C` 退出前端。

## 构建与运行

```sh
cargo check --workspace
cargo run -p localsendd
cargo run -p localsend-tui
cargo run -p localsend-tray
cargo run -p localsend-gui-slint --no-default-features --features backend-femtovg-wgpu
```

全流程本地验收：

```sh
./scripts/accept-tui.sh
./scripts/accept-slint.sh
```

macOS 打包：

```sh
cd apps/localsend-gui-slint
./package-macos.sh
codesign --verify --deep --strict ../../target/release/bundle/macos/LocalSend.app
```

Windows 绿色包在 Windows 原生主机的 PowerShell 中生成：

```powershell
./scripts/package-windows.ps1 -Architecture x64
./scripts/package-windows.ps1 -Architecture arm64
```

Linux tar/deb 在对应架构的 Linux 主机中生成：

```sh
./scripts/package-linux.sh x86_64
./scripts/package-linux.sh aarch64
```

所有发布物统一输出到 `target/release/packages/`。Linux 无托盘环境从桌面菜单启动 `localsend-launcher`；需要 StatusNotifier/AppIndicator 时可单独运行 `localsend-tray`。GNOME 用户如果没有托盘图标，需要安装并启用 AppIndicator 扩展。

macOS 包内的 tray 可执行文件名为 `LocalSendTray`，GUI 为 `LocalSend`。Windows GUI、tray 和 launcher 使用 GUI subsystem；daemon 子进程使用无窗口创建标志。开机自启动在 Windows 指向 `localsend-tray.exe --minimized`，在 Linux 指向 `localsend-launcher --background`。

## 当前范围

- 仅 PC；不迁移 Flutter、Iced 和 CrossCopy 自建协议栈。
- 文件字节不经过 RPC；daemon 负责路径展开、接收目录、哈希与历史。
- 官方 core 目录保持上游 `v1.18.2` 字节一致，升级流程见 `VENDORED_CORE.md`。
- Windows/Linux 本轮只完成代码和打包脚本；macOS 编译通过不代表 Windows/Linux 已做运行验收。

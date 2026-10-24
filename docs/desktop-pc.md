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
- Linux 桌面入口默认启动 `localsend-tray`；tray 是生命周期所有者，负责启动 owned daemon、打开 GUI、重启和退出。
- Linux 托盘图标无法创建时，tray 不退出，而是打开 GUI 并继续持有 owned daemon；GUI 关闭后再清理 daemon。没有托盘运行环境时仍可显式使用 `localsend-launcher`，AppImage 可通过 `--launcher` 选择该模式。
- Linux 的 GUI 和 tray 文件/消息对话框固定使用 GTK3 后端，不依赖 xdg-desktop-portal 或 zenity。
- Slint GUI 是纯前端。GUI 关闭不会停止 daemon，GUI 也不自行拉起 daemon。
- 直接运行 `localsend-tui` 只连接已有 daemon；daemon 未运行时显示错误并重试。
- 直接运行 `localsendd` 创建 Persistent daemon，不跟随 tray 或 GUI 退出。
- tray 退出时取消活动传输、请求 owned daemon 退出并等待 IPC endpoint 释放。
- tray 菜单只保留“打开主界面”“重启”“退出”。打开主界面不启动第二个 GUI；退出不创建 GUI。

## 系统通知

- tray 是系统通知的唯一发送者，GUI 不重复发送通知。
- macOS 使用 `UNUserNotificationCenter` + 系统默认声音。
- Windows/Linux 使用 `notify-rust`。Windows Toast 使用固定 AUMID
  `org.localsend.pc.prototype`、开始菜单快捷方式和默认提示音；Linux 使用
  `org.freedesktop.Notifications`，提示音 hint 由桌面通知服务决定是否播放。
- 文本通知来自 `TextReceived`；文件通知只来自非消息类型的成功
  `HistoryAppended`。消息历史不会重复通知。
- 点击通知通过 `GuiAction` 路由。GUI 未运行时先写入持久 action queue，再启动 GUI。
- Linux/Windows 的本地 GUI 控制通道分别是 Unix socket 和 named pipe；Windows 在
  named pipe 不可用时回退到 action queue。

通知真机冒烟：

```powershell
./scripts/accept-windows-notifications.ps1
```

```sh
./scripts/accept-linux-notifications.sh
```

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

所有发布物统一输出到 `target/release/packages/`。Linux 默认从桌面菜单启动 `localsend-tray`；没有状态栏图标的桌面环境会进入 GUI 守护 fallback，也可显式运行 `localsend-launcher`，AppImage 使用 `--launcher`。GNOME 用户如果需要图标菜单，仍需安装并启用 AppIndicator 扩展。

macOS 包内的 tray 可执行文件名为 `LocalSendTray`，GUI 为 `LocalSend`。Windows GUI、tray 和 launcher 使用 GUI subsystem；daemon 子进程使用无窗口创建标志。开机自启动在 Windows 指向 `localsend-tray.exe --minimized`，在 Linux 指向 `localsend-tray --minimized`。

## 当前范围

- 仅 PC；不迁移 Flutter、Iced 和 CrossCopy 自建协议栈。
- 文件字节不经过 RPC；daemon 负责路径展开、接收目录、哈希与历史。
- 官方 core 目录保持上游 `v1.18.2` 字节一致，升级流程见 `VENDORED_CORE.md`。
- Windows/Linux 通知、named pipe 和打包代码可进行静态检查；没有对应真机结果时，
  对应平台仍标记为 `UNVERIFIED`。

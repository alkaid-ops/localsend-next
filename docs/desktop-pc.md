# localsend-next PC 桌面架构

## 进程边界

```text
localsend-slint / localsend-tui / localsend-tray
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

正式 GUI 是 `apps/localsend-slint`，release 固定使用 `winit-femtovg`。接收、发送、设置三页保持原 LocalSend 结构。

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
cargo run -p localsend-slint --no-default-features --features backend-femtovg
```

macOS 打包：

```sh
cd apps/localsend-slint
./package-macos.sh
codesign --verify --deep --strict ../../target/release/bundle/macos/LocalSend.app
```

## 当前范围

- 仅 PC；不迁移 Flutter、Iced 和 CrossCopy 自建协议栈。
- 文件字节不经过 RPC；daemon 负责路径展开、接收目录、哈希与历史。
- 官方 core 目录保持上游 `v1.18.2` 字节一致，升级流程见 `VENDORED_CORE.md`。

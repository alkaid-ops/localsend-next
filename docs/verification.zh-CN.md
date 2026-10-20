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

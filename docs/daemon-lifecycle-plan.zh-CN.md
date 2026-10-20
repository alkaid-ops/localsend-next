# localsend-next 进程生命周期

## 规则

1. `localsend-tray` 是 GUI 和 owned daemon 的启动者。
2. `localsend-slint` 先取得自己的单实例锁，但只连接 daemon，不 spawn daemon。
3. `localsendd` 先取得 instance lock，再绑定 RPC endpoint；第二个 daemon 必须干净退出。
4. owned daemon 使用 `--require-owner`，在 3 秒内没有 owner 或 owner 断开后优雅退出。
5. 显式启动的 daemon 没有 owner lease，只能由显式 Shutdown 停止。
6. tray `Quit` 不创建 GUI 窗口；它先取消活动传输，再停止 owned daemon。
7. GUI 关闭只退出 GUI 进程，tray 与 daemon 继续运行。

## IPC

- macOS/Linux: Unix domain socket，权限 0600。
- Windows: named pipe。
- RPC: v2，4 字节大端长度前缀 JSON，单帧上限 1 MiB。
- 连接建立后先收完整 snapshot，再接收带递增序号的事件。
- 文件内容不经过 IPC，只传路径、元数据和进度。

## Runtime Manifest

daemon 绑定 endpoint 后写入 `runtime.json`，包含 RPC 版本、endpoint、PID、instance ID、生命周期、配置目录和启动时间。前端按显式 endpoint、环境变量、manifest、平台默认值的顺序解析 endpoint。

## 验收命令

```sh
cargo test -p localsendd --test daemon_lifecycle
cargo test -p localsend-service --test service_smoke
cargo check --workspace
```

当前自动化覆盖 owner loss、owner-start timeout、Persistent shutdown、instance lock、runtime manifest 和 service loopback 收发。

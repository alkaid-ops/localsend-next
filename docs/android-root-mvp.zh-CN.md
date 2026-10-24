# Android Root Native Boot Daemon MVP

## 目标

在已 root 的 arm64 Android 设备上运行一个无 UI 的 LocalSend 接收端。它不使用
Android Foreground Service，不显示常驻通知，只发送文本和文件接收事件通知。

```text
Magisk / KernelSU supervisor
            |
  localsend-androidd
            |
   localsend-service
            |
 official LocalSend core
```

## 运行目录

- 根目录：`/data/adb/localsend-next`
- 配置：`/data/adb/localsend-next/config/settings.json`
- 日志：`/data/adb/localsend-next/logs/`
- PID：`/data/adb/localsend-next/localsend-androidd.pid`
- 临时禁用：`/data/adb/localsend-next/disabled`
- 接收目录：`/storage/emulated/0/Download/LocalSend`

不存在 `disabled` 文件时，supervisor 会启动并 watchdog 重启 daemon。创建该文件后，
daemon 自行收到 Shutdown，并停止接收；删除文件后 supervisor 自动恢复。

## 行为

- 默认 HTTP、端口 `53317`、auto-save 开启。
- `auto_save=on` 会在无 UI 情况下自动接受同网段请求。生产使用前应配置 PIN，或仅在
  可信局域网中启用模块。
- 文本只由 `TextReceived` 触发一次通知；对应的消息历史不会重复通知。
- 文件由成功接收的非消息 `HistoryAppended` 触发通知，并尝试发出媒体扫描广播。
- 通知使用 `/system/bin/cmd notification post`，不包含常驻服务通知。
- 日志不记录消息正文，单文件超过 10 MiB 时保留一个备份。

## 构建与检查

需要 Android NDK，默认 API 29、目标 `aarch64-linux-android`：

```sh
export ANDROID_NDK_ROOT=/path/to/android-ndk
./scripts/package-androidd-root.sh
./scripts/check-androidd-root-package.sh \
  target/release/packages/localsend-next-root-aarch64.zip
```

模块 ZIP 只包含 supervisor、配置和 native binary，不包含 APK、Foreground Service、
`system/etc/init` 或 `system/bin`。这样不会在未验证的 Android 启动阶段注入 init 配置。

## 真机验收

```sh
DO_REBOOT=1 SOAK_MINUTES=30 ./scripts/verify-androidd-root.sh
```

需要检查：

1. 重启后 PID 存在，进程为 `localsend-androidd`。
2. 没有常驻服务通知。
3. 官方 LocalSend 可以发现设备；若 discovery 失败，再测试手动 IP。
4. 文本和文件通知各出现一次，文件哈希、mtime、相对目录正确。
5. 灭屏 10 分钟后仍能接收。
6. `kill -9` 后 watchdog 在 5 秒内恢复。
7. 禁用开关、恢复、模块卸载清理都正常。

没有连接设备时脚本必须输出 `status: BLOCKED`，不得报告通过。

## 已知限制

- 只支持 root、arm64。
- 不包含 UI、发送端控制、Web Share、WebRTC 或通知点击回主界面。
- 不是真正的 `init.rc` 系统服务，也不提供自定义 SELinux domain。
- Android 的 Wi-Fi 组播过滤、Doze 和 OEM 省电策略需要真机验证。
- `cmd notification post` 的通知声音、可见性和点击行为必须由真机确认。


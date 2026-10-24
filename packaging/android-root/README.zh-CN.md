# LocalSend Next Android Root MVP

该模块在 Magisk 或 KernelSU 环境中启动一个无 UI 的 Android 原生
`localsend-androidd` 接收进程。它不使用 Android Foreground Service，也不会创建
常驻服务通知。

## 安装

1. 将 `localsend-next-root-aarch64.zip` 作为 Magisk/KernelSU 模块安装。
2. 重启设备。
3. 等待 `sys.boot_completed=1`，检查 `/data/adb/localsend-next/localsend-androidd.pid`。
4. 日志位于 `/data/adb/localsend-next/logs/`。

## 配置

- 配置文件：`/data/adb/localsend-next/config/settings.json`
- 接收目录：`/storage/emulated/0/Download/LocalSend`
- 临时禁用：`touch /data/adb/localsend-next/disabled`
- 恢复接收：`rm /data/adb/localsend-next/disabled`

修改 `settings.json` 后执行以下命令重启接收进程：

```sh
kill "$(cat /data/adb/localsend-next/localsend-androidd.pid)"
```

supervisor 会在 5 秒内重新启动它。

默认 `auto_save=on` 表示无 UI 情况下自动接受局域网内所有传输请求。若设备处于不可信
网络，必须在 `settings.json` 中设置 `pin`，或停止模块并改为有确认界面的方案。

## MVP 限制

- 不支持 Send、Web Share、WebRTC 或 UI。
- 通知依赖 `/system/bin/cmd notification post`，必须在真机确认通知可见和声音正常。
- 不包含真正的 init `.rc` 服务或自定义 SELinux domain；由模块 supervisor 启动 native daemon。
- 后台 multicast 在灭屏和不同 OEM 上的行为仍需要真机验证。


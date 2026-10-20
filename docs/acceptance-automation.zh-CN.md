# TUI 与 Slint GUI 自动化验收

## 本地门禁

```sh
./scripts/accept-tui.sh
./scripts/accept-slint.sh
```

两个脚本均不依赖 GitHub Actions，不创建 CI workflow。产物写入
`target/acceptance/<suite>/<timestamp>/`。

## TUI

`accept-tui.sh` 会：

1. 构建 `localsendd` 和 `localsend-tui`。
2. 连续运行三次测试，禁用测试重试。
3. 使用真实 PTY 驱动 `localsend-tui`，通过 vt100 解析最终终端画面。
4. 使用 Ratatui `TestBackend` 验证 60x20、80x24、120x40 三页布局。
5. 使用 vendored 官方 core 构建确定性 peer，验证发送和接收协议。

当前自动化覆盖：

- 三页切换、最小尺寸布局和 TUI 退出后 daemon 存活。
- 接收全部文件、落盘字节、历史记录。
- 手动 IP、文件发送、Unicode 文本发送。
- PIN 错误提示和重试成功。
- alias 设置提交与 daemon snapshot 持久化。

失败产物包含终端画面和 daemon snapshot。脚本只有在三次运行全部通过后才输出
`result: passed`。

## Slint GUI

`accept-slint.sh` 会：

1. 运行三次单元、语义和 worker 集成测试。
2. 通过 Slint testing backend 查询 accessible tree，执行无障碍默认动作和设置输入。
3. 使用真实 daemon 与官方 core peer 验证发送、接收和设置持久化。
4. 生成 Receive/Send/Settings 的 32 张软件渲染截图，并与 golden 做逐字节比较。
5. 构建 FemtoVG release App，执行 deep strict 签名验证。
6. 启动真实 App，使用 macOS AXUIElement 切换发送和设置页，不使用 AppleScript。
7. 关闭 GUI 后确认 daemon 仍存活，再由 RPC 关闭 daemon。

AX 自动化优先使用 Xcode 6.0.3 工具链；缺少兼容 Swift 编译器或 Accessibility
权限时，脚本返回 `blocked`，不会伪造通过。

## 当前结果

- TUI：3 次连续运行通过，4 条真实 PTY 全流程通过。
- Slint：3 次连续语义/worker 测试通过，32 张 golden 完全一致。
- 真实 App：native AX navigation、release 打包、codesign 和 daemon 生命周期通过。
- vendored 官方 core 未修改。

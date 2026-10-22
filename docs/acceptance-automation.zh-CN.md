# TUI 与 Slint GUI 自动化验收

## 本地门禁

```sh
./scripts/accept-tui.sh
./scripts/accept-slint.sh
./scripts/accept-slint-native.sh
./scripts/accept-slint-receive-settings.sh
./scripts/accept-egui.sh
```

需要继续使用桌面、不希望自动化抢焦点时，可只运行 egui 的无窗口门禁：

```sh
LOCALSEND_ACCEPT_EGUI_HEADLESS=1 ./scripts/accept-egui.sh
```

该模式仍运行 11 条 `egui_kittest` 与官方 core peer 测试、格式检查和 clippy，
但不启动真实窗口，也不执行 macOS AX 截图。默认完整模式保留原生 AX、18 张截图
和进程生命周期验收，适合在机器空闲时运行。

两个脚本均不依赖 GitHub Actions，不创建 CI workflow。产物写入
`target/acceptance/<suite>/<timestamp>/`。

## TUI

`accept-tui.sh` 会：

1. 构建 `localsendd` 和 `localsend-tui`。
2. 连续运行三次测试，禁用测试重试。
3. 使用真实 PTY 驱动 `localsend-tui`，通过 vt100 解析最终终端画面。
4. 使用 Ratatui `TestBackend` 验证 60x20、80x24、120x40 三页布局。
5. 使用 vendored 官方 core 构建确定性 peer，验证发送和接收协议。
6. 使用两个不同 HTTP 端口和不同 IPv4 组播组的 daemon 验证 fixed-port 发现兼容性。

当前自动化覆盖：

- 三页切换、最小尺寸布局和 TUI 退出后 daemon 存活。
- daemon 被终止后进入 reconnecting，重启后恢复 snapshot。
- 组播 UDP 固定使用 `53317`，不跟随 HTTP 端口；自定义旧组同时兼容官方
  `224.0.0.167`。
- 接收全部文件、部分接受、拒绝、落盘字节和历史记录。
- 手动 IP、文件/目录发送、目录相对名称、Unicode 文本发送。
- 收藏添加和收藏别名编辑。
- PIN 错误提示和重试成功。
- 慢速发送取消并保持 Cancelled 状态。
- alias、HTTP/HTTPS、auto-save 设置提交与 daemon snapshot 持久化。
- 设置 revision 冲突在 daemon 层被拒绝。

失败产物包含终端画面和 daemon snapshot。脚本只有在三次运行全部通过后才输出
`result: passed`。

除专门的组播测试外，`DaemonFixture` 默认设置
`LOCALSEND_TEST_DISABLE_DISCOVERY=1`，测试 daemon 不会再绑定真实 UDP 53317
或向局域网公告测试身份。需要验证组播的测试必须显式使用
`DaemonFixture::start_with_discovery`，避免 E2E 设备出现在官方客户端列表中。
真实 LocalSend 组播组测试默认忽略，只有在本机没有生产实例且显式设置
`LOCALSEND_RUN_REAL_MULTICAST_TEST=1` 时才运行。

### 原生语义 E2E

`accept-slint-native.sh` 使用 macOS AXUIElement 语义树驱动真实 Slint 窗口，不使用
截图坐标点击。脚本按 accessible role 和 label 查找“发送”“文件”“文件夹”“文本”
“剪贴板”、设备项和对话框按钮，并执行 `AXPress` 或写入输入值。

每次运行启动隔离的 `localsendd`、确定性官方 core peer 和独立 GUI 进程，覆盖：

- 本地文件按钮 → 目标设备 → 文件落盘和 SHA-256 一致。
- 文件夹按钮 → 目标设备 → 两个嵌套文件及相对名称。
- 文本对话框输入 → 确定 → 目标设备 → `message.txt` preview 一致。
- 系统剪贴板 → 剪贴板按钮 → 目标设备 → `message.txt` preview 一致。
- 四个传输全部 finished，peer 无错误，会话不残留。

文件选择器在 `LOCALSEND_ACCEPTANCE=1` 时可通过 `LOCALSEND_E2E_FILE_PATH` 和
`LOCALSEND_E2E_FOLDER_PATH` 注入固定路径，避免自动化依赖 NSOpenPanel 的本地化
按钮；普通启动不会读取这些变量。测试产物和 `summary.json` 写入
`target/acceptance/slint-native/<timestamp>/`。

### 接收与设置原生 E2E

`accept-slint-receive-settings.sh` 继续使用 AXUIElement 语义树，不依赖截图或坐标，
覆盖接收页和设置页的真实交互：

- 双文件请求默认全选，取消一个文件后“接受所选”，验证只落盘被选文件。
- “拒绝”不产生文件；入站文本显示消息弹窗并可关闭。
- 设置页切换“快速保存”“自动完成传输任务”，设置 PIN。
- 无 PIN 请求返回 401；正确 PIN 的快速保存不弹请求；关闭快速保存后正确 PIN
  显示请求，点击“全部接受”完成。
- 自动完成由 service 集成测试验证：完成后等待 3 秒从活动传输移除。

测试产物写入 `target/acceptance/slint-receive-settings/<timestamp>/`。

## Slint GUI

`accept-slint.sh` 会：

1. 运行三次单元、语义和 worker 集成测试。
2. 通过 Slint testing backend 查询 accessible tree，执行无障碍默认动作和设置输入。
3. 使用真实 daemon 与官方 core peer 验证发送、接收、部分接受、拒绝、取消、PIN
   重试、Web Share 接收页和设置持久化。
4. 生成 Receive/Send/Settings 的 34 张软件渲染截图，并与 golden 做逐字节比较。
5. 构建 FemtoVG release App，执行 deep strict 签名验证。
6. 启动真实 App，使用 macOS AXUIElement 切换发送和设置页，不使用 AppleScript。
7. 关闭 GUI 后确认 daemon 仍存活，再由 RPC 关闭 daemon。

AX 自动化优先使用 Xcode 6.0.3 工具链；缺少兼容 Swift 编译器或 Accessibility
权限时，脚本返回 `blocked`，不会伪造通过。

Slint 翻译由 `scripts/update-slint-translations.mjs` 从 `@tr(...)` 自动生成
`translations/{en-US,zh-CN}/LC_MESSAGES/localsend-gui-slint.po`。新增或修改静态
文案后必须重新运行该脚本；`tests/language.rs` 会在独立进程验证中文、英文和
动态传输状态切换。默认中文的 38 张无窗口 golden 已与既有基线逐字节一致；最近证据：
`target/acceptance/slint-translation/20261021-165504/summary.txt`。英文布局矩阵生成
Receive/Send/Settings × 700/800/1024/1440 × Light/Dark 共 24 张，最近证据：
`target/acceptance/slint-language/20261021-170230/summary.txt`。

原生文件面板通过 `with_native_file_dialog_cleanup` 包裹。守卫只回收面板打开后新增的
`openAndSavePanelService` / `QuickLookUIService` PID，绝不按名称全局 kill。设备卡点击
回归由 `tests/send_tile_click.rs` 覆盖，包括卡片中心和标题文字区域。相关证据：
`target/acceptance/gui-fixes/20261021-175850/summary.txt`。真实鼠标打开并取消一次
文件面板后没有新增 XPC PID，设备卡点击会出现选中边框和 Snackbar；最近证据：
`target/acceptance/gui-fixes/20261021-181311/manual-mouse/summary.txt`。

## 当前结果

- TUI：3 次连续运行通过，8 条真实 PTY 全流程通过。
- Slint：3 次连续语义/worker 测试通过，34 张 golden 完全一致。
- Slint native AX：文件、文件夹、文本、剪贴板四条真实 GUI 操作链通过，3 个接收
  文件哈希一致，4 个 transfer 均为 finished。
- 发送和接收传输卡片均显示真实百分比进度条、状态和取消操作。
- Slint 接收/设置 AX：部分接受、拒绝、文本、快速保存、自动完成和 PIN 全链路通过。
- 真实 App：native AX navigation、release 打包、codesign 和 daemon 生命周期通过。
- vendored 官方 core 未修改。

## egui GUI

`accept-egui.sh` 会：

1. 运行 `cargo fmt --check`、11 条 egui 测试和 clippy。
2. 启动隔离 daemon 和真实 egui 窗口，连续触发两次 AX 初始化。
3. 使用 macOS AX 导航 Receive/Send/Settings。
4. 生成 Light/Dark × 900x600/1024x768/1440x900 共 18 张窗口截图。
5. 关闭 GUI 后确认 daemon 存活，再通过 RPC 关闭 daemon。

完整门禁产物：`target/acceptance/egui/20261021-151251/summary.json`；最新无窗口
11 条测试：`target/acceptance/egui/20261021-152429/summary.json`。egui release
二进制为 `target/release/localsend-gui-egui`，release 原生 AX smoke 另存于
`target/acceptance/egui/release-smoke-localized/20261021-151638/`。

真实互测只记录已完成的代表性链路：egui → 官方 App 文件（含 PIN 重试，SHA-256 一致），
官方 App → egui 文本（历史与消息弹框），官方 App → egui 嵌套文件夹（双方 SHA-256
一致）。后台 AX 还覆盖了 egui → 官方 App 的 PIN 重试后拒绝与待确认请求取消：

```sh
cargo test -p localsend-gui-egui egui_official_app_background_rejects_headless_send -- \
  --ignored --nocapture --test-threads=1
cargo test -p localsend-gui-egui egui_official_app_background_cancels_pending_send -- \
  --ignored --nocapture --test-threads=1
```

两条用例只调用 `target/tools/macos-ax-background` 的 AXPress，不激活官方 App、不移动
鼠标。最近证据：`target/acceptance/egui/official-interop/1792569495-background-reject/`
和 `.../1792569618-background-cancel/`。官方 App → egui 的 PIN 输入仍为 `UNVERIFIED`：
Flutter 文本框不接受后台 AX 写值、`CGEvent.postToPid` 按键或本地菜单 `Paste`，三种方式
都无法更新 controller，daemon 返回 `401 Invalid PIN`；需要机器空闲时用前台真实键盘输入，
不能由 headless service 测试替代标记为真实 App PASS。

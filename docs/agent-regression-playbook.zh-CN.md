# localsend-next Agent 历史回归手册

更新时间：2026-10-23

这份文档整理本项目历史对话中反复出现的故障、误判和返工点。它不替代当前代码、
测试或 `AGENTS.md`，而是给后续 agent 一份“先检查什么、不要重复什么”的排查手册。
遇到与当前代码冲突的内容时，以当前代码和可复现证据为准，并同步修正本文档。

## 1. 开始工作前必须确认

1. 先读根目录 `AGENTS.md` 和本文档。
2. 运行 `git status --short`。工作树可能包含用户或其他 agent 的未提交修改，禁止用
   reset、checkout 或批量覆盖“清理”。
3. 先确认进程边界：GUI、tray、daemon 是三个独立角色，不是三个可互换入口。
4. 先定位故障层级：Slint 事件 -> `UiCommand` -> RPC -> daemon -> 官方 core。
   不要只看最终界面，也不要靠猜测跨层修代码。
5. 优先使用无头测试、RPC fixture、日志和状态数据。只有视觉或真实窗口链路问题才做
   截图；不要把截图点击作为通用调试手段。
6. 完成一个根因修复后，必须留下能防止同类问题复发的测试或静态门禁。

## 2. 不可破坏的架构与生命周期

- `localsend-gui-slint` 只渲染 UI 并通过 `localsend-rpc` 通信，不启动 daemon。
- `localsendd` 是 LocalSend core、设置、传输、历史和身份的唯一权威。
- `localsend-tray` 只负责打开 GUI、重启、退出和系统通知，不维护设备、历史、设置或
  传输状态，不读取完整 snapshot。
- tray 菜单只保留“打开主界面”“重启”“退出”，不增加服务器启停、状态展示或
  “打开接收文件夹”；接收目录由通知动作打开。
- 正式 GUI 固定 Slint + winit + FemtoVG；当前 release 使用
  `backend-femtovg-wgpu`，其他 GUI 目录是实验代码，不进入正式链路。
- tray 先取得自身单实例锁，再连接或启动 owned daemon；daemon 同时持有自己的锁。
- 关闭 GUI 窗口就是退出 GUI 进程。tray 和 daemon 继续运行。
- tray `Quit` 是整个实例的退出操作：关闭 GUI、停止 owned daemon、退出 tray，
  不创建新 GUI，不留下 launchd keep-alive、socket、PID 文件或孤儿进程。
- GUI 和 TUI 在 daemon 不存在时显示断线并重试，不能自行拉起 daemon。
- 外部启动的 daemon 不归当前 tray 所有；tray 退出不能停止外部 daemon。
- vendored official core 保持原样。禁止为了格式、lint 或临时测试直接修改它。

## 3. 高频反复问题

速查顺序：

| 用户看到的症状 | 第一层检查 | 最终防复发措施 |
| --- | --- | --- |
| 官方客户端发现不到设备 | UDP `53317`、官方组播组、实际 HTTP port | 隔离 discovery 测试和真实组播兼容测试 |
| 多个 daemon 或 Quit 后重生 | 单实例锁、owner、启动方式 | 精确 PID 管理，禁止 `launchctl submit`/`killall` |
| tray 打开后无窗口 | control socket、`gui.pid`、窗口 alpha | `open -n --window`、单实例锁和真实窗口验收 |
| Quit 时又弹出 GUI | Quit 是否走错 open-main 路径 | 两条退出路径统一清理 GUI/daemon/tray |
| 点设备没有反应 | `TouchArea` 命中、`UiCommand`、RPC | pointer 注入 + 真实 CGEvent + peer 哈希 |
| 发送很慢、卡住或假失败 | 事件 seq、snapshot、terminal event | 单次 snapshot + 有序事件 reducer |
| 文本/文件通知缺失或重复 | tray bundle、权限、notification stream | tray 独占通知，消息历史去重 |
| 设置保存无反馈或被覆盖 | `expected_revision`、保存响应 | revision 冲突刷新并二次确认 |
| 文件面板留下 XPC/QuickLook | 面板前后 PID 基线 | 只清理本次新增 PID，禁止按名称全局杀 |
| 视觉仍不像原版 | token、布局 landmark、字号/圆角 | headless 断言 + 少量固定 golden |
| 测试绿了但真机仍坏 | 是否只测 mock/AX/脚本退出码 | 真实输入 -> RPC -> peer -> 哈希全链路证据 |

### 3.1 官方客户端发现不到本机

历史症状：

- 官方 LocalSend 和本项目互相看不到。
- 修正后过一段时间又复发。
- HTTP 端口变化后，误以为组播端口也应该跟着变化。
- `port == 0` 被误认为“不可能”或错误配置。

真实规则：

- LocalSend discovery 固定使用 UDP `53317`，不跟随 HTTP 监听端口。
- 官方 IPv4 组播组是 `224.0.0.167`。
- 自定义旧组 `239.255.88.61` 只能作为兼容组，必须同时保留官方默认组。
- HTTP `port=0` 是合法的系统分配随机端口；announcement 中要公布实际 HTTP 端口，
  不能把 HTTP 端口写进 multicast socket 端口。
- 测试 daemon 默认必须禁用真实 discovery，否则会出现在用户的官方客户端设备列表中。

排查顺序：

1. 检查 daemon 是否绑定 IPv4/IPv6 UDP `53317`。
2. 检查 multicast group 是否包含官方组。
3. 检查 announcement payload 的实际 HTTP port。
4. 检查接口白名单/黑名单是否排除了当前网卡。
5. 再检查 HTTP server 是否可达。

禁止：

- 只看 GUI 设备列表判断 discovery 是否工作。
- 用“端口不是 53317 所以组播端口也不对”这种推断。
- 让普通 E2E 测试加入真实组播组。

### 3.2 多个 daemon、孤儿进程和假重启

历史症状：

- 同时出现多个 `localsendd`。
- tray 图标消失后又自己出现。
- 点了多次 Quit 才真正退出。
- 重启后 id 或状态没有变化，实际旧实例仍在。
- daemon 退出时间过长，用户再次启动 App，形成新旧实例竞争。

真实根因：

- 单实例判断不完整，或只检查进程名子串。
- tray、GUI、launcher 都以为自己有权启动/停止 daemon。
- `launchctl submit` 等启动方式带 keep-alive，进程退出后被系统重新拉起。
- 使用全局 `killall` 或宽泛信号，既杀不到正确实例，也可能影响其他应用。
- 优雅退出没有 bounded timeout，或在退出未完成时又启动新实例。

必须做到：

- daemon、tray、GUI 各有 endpoint-scoped 单实例锁。
- 进程只能按精确 PID、可执行路径、endpoint 和 ownership 操作。
- tray 只终止自己创建的 owned daemon；外部 daemon 保持不动。
- Quit 使用有限等待；超时只终止已确认属于本实例的子进程。
- 测试 fixture 记录子进程 PID，在成功、失败、abort 和中断路径都清理。
- 每轮 E2E 后检查是否残留 `target/debug/localsendd`、fixture、GUI 或 control socket。

禁止：

- `launchctl submit` 启动 LocalSend App 或 tray。
- `killall LocalSend`、按 “LocalSend” 子串杀进程。
- 为了让测试结束而无条件 `pkill`。
- 在没有等待旧进程退出时启动同名新实例。

### 3.3 tray 打开主界面后无窗口、闪一下或透明窗口

历史症状：

- Dock 中有程序但看不到窗口。
- 窗口一闪而过，进程仍在。
- tray 菜单显示正常，但“打开主界面”无效。
- 第二次点击后才出现多个 GUI 或没有复用已有窗口。

真实根因：

- macOS 直接 spawn GUI 可能产生 `alpha=0` 或非前台窗口。
- LaunchServices `open -n -a ... --args --window` 的显示行为不同。
- 已有 GUI 时又启动新进程，只靠单实例锁退出，没有先通过 control socket 唤起。
- 半失效 GUI 仍有 PID，但 control socket 已不可用。

当前正确方向：

1. 先向 GUI control socket 发送 `ShowMain`。
2. 失败时读取并验证 `gui.pid`。
3. PID 文件很新，使用 `open -n -a <bundle> --args --window`，让 GUI 单实例锁最终去重。
4. PID 已存在且超过约 2 秒仍无 socket，才判定失效并终止该精确 PID。
5. 不保留固定 15 次 sleep 或全局 launch-in-flight 布尔锁。

验收必须同时检查：

- 只有一个 GUI PID。
- 窗口 `alpha=1`、onscreen，且不是最小化隐藏状态。
- 已有 GUI 时再次打开复用原 PID。
- 关闭 GUI 后进程、`gui.pid`、control socket 消失。
- tray 和 daemon 仍存活。

不要用 AppleScript 代替 tray 的真实菜单/生命周期路径。需要查窗口属性时使用
CoreGraphics/AppKit 等只读 API；需要真实点击时使用 AX 定位、CGEvent 输入。

### 3.4 Quit 又弹出 GUI，或图标消失后又重启

历史根因：

- Quit 路径错误复用了“打开主界面”逻辑。
- 通过 launchd/keep-alive 启动，tray 退出后被重新拉起。
- GUI 没有被终止，只是 tray 图标消失。
- owned daemon 未停止，runtime manifest 仍指向存活实例。

固定门禁：

- 在 tray 菜单和 `--quit` 两条路径都验证：不创建 GUI。
- Quit 后 GUI、tray、owned daemon、socket、PID 文件和 action listener 全部消失。
- 等待至少数秒确认没有自动重启。
- 若 daemon 是外部所有，只退出 tray/GUI，不停止 daemon。

### 3.5 GUI 点击设备没有反应，但 AX 操作能发送

历史症状：

- 文件已选，点击设备卡没有任何 RPC 命令。
- 只有自动化按 accessible action 或点击某个发送按钮才工作。
- 设备卡中心、标题文字、右侧按钮命中行为不一致。

真实根因：

- Slint `TouchArea` 覆盖顺序或范围错误。
- 业务逻辑只在发送按钮上，没有接入整张设备卡。
- AX 语义动作成功不等于真实 pointer 事件命中成功。
- 设备列表来自陈旧 snapshot，点击的是已失效 fingerprint。

修复原则：

- 设备卡标题、IP、图标和空白区域都应触发同一设备动作。
- 右侧详情、收藏等独立按钮不能被父级 TouchArea 抢走。
- 有文件就发 `SendFiles`，有文本就发 `SendText`，无内容时给出明确提示。
- 点击后立即更新选中态/Snackbar，并确认 `UiCommand` 已发出。
- 禁止自动选择第一台设备；发送目标必须由用户当前选择决定。

至少覆盖三层测试：

1. Testing Backend pointer 注入，断言精确 `UiCommand`。
2. 真实 daemon worker 流程，断言 RPC 和文件字节。
3. macOS 原生 pointer E2E，AX 只定位 frame，CGEvent 发 mouse move/press/release。

### 3.6 文本或文件发送很慢、无反应，或发送成功却显示失败

历史症状：

- 点击后 5 秒才发送，官方客户端不到 1 秒。
- 文本和文件都卡住。
- peer 已收到文件，GUI 仍显示失败。
- 发送失败后卡在发送页。

排查顺序：

1. 看真实 pointer 是否产生 `UiCommand`。
2. 看 worker 是否发出对应 RPC 命令。
3. 看 daemon 是否创建 transfer 并收到 peer 响应。
4. 看 event `seq` 是否连续，`TransferFinished`/`TransferFailed` 是否被本地 reducer 应用。
5. 最后才检查 Slint 状态和文案映射。

已知坑：

- 旧 worker 在每条事件后重新拉 snapshot，造成延迟和竞态。
- 测试 helper 只消费 `WorkerEvent::Snapshot`，丢掉了 `DeviceUpsert` 等增量事件。
- 把协议完成事件和前端成功文案绑定错，导致真实成功后显示失败。
- 文件选择完成不代表已发送；点击目标设备时仍要检查 pending content。
- 慢速取消必须让传输进入 `Cancelled`，不能只关掉当前详情页。

正确状态模型：

- 首连、重连、`seq` 缺口、broadcast `Lagged` 时拉一次完整 snapshot。
- 正常状态更新使用有序事件，不按 100ms 或固定周期轮询 `GetSnapshot`。
- 聚合进度单调不减，并 clamp 到总字节数。
- terminal 状态之后才允许 auto-finish；移除 transfer 只广播一次 Snapshot。
- 传输结束页必须有真实返回和取消路径，失败不能把 UI 锁死。

### 3.7 通知缺失、重复、点开错误页面

历史症状：

- 文本接收成功但没有系统通知。
- 文本通知重复，消息历史又通知一次。
- 点通知默认打开脚本编辑器。
- GUI 和 tray 同时运行后通知重复。
- 新版本通知无法路由，因为 bundle identity 变化。

当前规则：

- 系统通知只由 `localsend-tray` 发送。
- tray 使用 notification-only、non-interactive 连接，不请求 snapshot。
- 文本只看 `TextReceived`；成功文件只看非消息类型的 `HistoryAppended`。
- 消息对应的 `HistoryAppended` 不能再通知一次。
- PIN、拒绝、失败、进度不产生成功文件通知。
- 通知点击通过持久 action queue + GUI control socket 路由。
- GUI 未运行时由 tray 启动，GUI 启动后消费队列。
- 文本和文件通知使用 macOS 系统默认提示音；通知内容必须显式设置
  `UNNotificationSound::defaultSound()`，前台展示选项也必须包含 `Sound`。
- macOS tray helper 必须使用自己的 bundle id 和 `TrayInfo.plist`；不要把 tray 可执行文件
  直接塞回 GUI 的 `Contents/MacOS`。

隐私与日志：

- 不记录消息正文。
- 文本事件日志最多记录 sender、字符数和 action 类型。
- 不创建 `localsend-tray-events.log` 之类旁路诊断日志；统一进入 `localsend-tray.log`。

### 3.8 设置页没有保存反馈、字段不生效或覆盖别人修改

历史症状：

- alias 修改后点保存像没反应。
- GUI 保留了旧 alias，daemon 没重启或 server alias 未更新。
- 主题被写成“亮度”，自动完成、quick-save 等字段缺失。
- TUI/GUI 同时改设置时静默覆盖。

规则：

- `UpdateSettings` 必须携带 snapshot 中的 `expected_revision`。
- revision 冲突必须显示错误并重新拉 snapshot，让用户确认后再提交。
- alias、port、protocol、PIN、receive directory、auto-save、discovery、multicast 和接口过滤
  由 daemon 负责持久化和必要的 server restart。
- GUI 只编辑和提交设置，不复制 daemon 的 restart 逻辑。
- UI 标签必须和实际 DTO 字段一致，例如 Theme 不能映射成 Brightness。
- 设置页修改后要有明确保存成功/失败反馈。

### 3.9 文件选择器关闭后留下 XPC/QuickLook 进程

历史症状：

- `Open and Save Panel Service (LocalSend)` 和 `QuickLookUIService` 持续存在。
- 关闭 GUI 后仍有本次选择产生的服务。

规则：

- 保留系统原生 `rfd::FileDialog`，不要自写文件选择器。
- 打开前记录同名服务 PID 基线；返回、取消、报错后只清理本次新增 PID。
- 先 SIGTERM，有限等待后再对同一批 PID SIGKILL。
- 禁止 `killall` 或按名称杀启动前已存在的共享服务。
- 正常返回、取消、错误和关闭 GUI 都要走同一个 guard 的清理路径。

### 3.10 视觉“差不多”但用户仍认为不像原版

历史症状：

- 页面结构、字号、留白、圆角、按钮顺序和 Flutter 原版明显不同。
- 为了观察差异反复截图、点击，消耗大量 token 和时间。
- 功能可用但组件形态仍不对。

正确做法：

- 从 Flutter 原版提取 token：颜色、字号、spacing、radius、控件尺寸。
- 先实现布局和组件层级，再做少量固定视口截图对照。
- 日常回归优先使用 accessible-id、Testing Backend 和语义断言。
- 只在视觉变更或发布门禁跑 golden，不在每次小修复中全量截图。
- 截图失败只作诊断，不作为“功能已经通过”的证据。

对当前产品尤其注意：

- Receive 页以官方空闲/请求/传输状态为准。
- 文本消息要显示消息阅读界面和复制/关闭，不要只显示 `message.txt`。
- Send 页先选内容并可随后点击设备发送，设备列表要实时刷新。
- Settings 使用 Section/Entry 布局、较短控件宽度和明确版本/关于信息。
- 完成页必须有倒计时、真实返回和取消；不能是假按钮。

### 3.11 自动化测试给了错误信心

历史问题：

- AX 控件可调用不代表真实鼠标能点。
- mock handler 有命令不代表 Slint 事件真的进入 handler。
- 测试只检查 GUI 显示，没验证 peer 文件 SHA-256。
- 测试用生产 `53317` 或真实 Downloads，受用户环境干扰。
- 完整测试中有 `#[ignore]`，却整体报告“全部通过”。
- 测试失败后用“再跑一次”掩盖竞态。
- worker 测试把增量事件丢掉，随后把等待超时误判为初始 snapshot 失败。

门禁原则：

- 单元测试验证 reducer、view model、revision、seq、gap 和 recovery。
- worker 集成测试使用真实 daemon、隔离 endpoint/config/destination 和确定性 peer。
- UI 测试断言精确 `UiCommand`，包括次数、目标 fingerprint、路径和文本。
- 原生 E2E 同时证明：输入实际执行、RPC 到达、peer 收到数据、哈希匹配、资源清理完成。
- 不自动重试。失败、skip、未实际执行、peer 未收到数据或残留进程都不能算通过。
- 产物必须记录 timestamp、日志、最终 snapshot、事件序号、文件哈希和清理报告。

### 3.12 首次打开 GUI 出现 `No such file or directory (os error 2)`

历史症状：

- 第一次双击 App 时底部弹出原始 `ENOENT`，随后界面又能正常使用。
- 第二次打开通常不复现。
- 进程启动顺序中 GUI 早于 owned daemon。

真实根因：

- `LocalSend` 主进程在 macOS 首次启动时先创建 tray，同时继续创建 GUI 和 worker。
- tray 此时才在后台启动 `localsendd` 并等待 Unix socket。
- GUI worker 的第一次连接发生在 socket 创建前，得到 `ENOENT`。
- 旧代码把该 I/O 错误原文放进 `WorkerEvent::Connection`，GUI 对任何断线事件都显示
  Snackbar。

修复约束：

- 连接期间的 `std::io::Error` 是预期的可重试状态，不直接展示给用户。
- 初次连接显示“正在连接后台服务”，断线后显示“正在重连”，只更新状态。
- 协议、版本或非 I/O 错误才显示 Snackbar。
- 首次连接前 50 次重试使用短间隔，之后退回 1 秒间隔。
- 原始错误可以写日志供诊断，但不得进入用户可见的 Snackbar。

## 4. 推荐排障顺序

### GUI 看起来没反应

1. 进程是否存活，窗口是 onscreen/alpha 正常还是只有 Dock 图标。
2. GUI 单实例锁和 `gui.pid` 是否有效。
3. control socket 是否存在，`ShowMain` 是否可达。
4. 真实 pointer 是否命中目标 `TouchArea`。
5. handler 是否发出正确 `UiCommand`。
6. worker 是否发出 RPC 命令，daemon 是否创建 transfer。

### 传输失败/卡住

1. 目标 fingerprint 是否来自当前 snapshot。
2. peer 是否可达，HTTP/HTTPS 和证书 pin 是否正确。
3. daemon 日志是否有 prepare/upload/response 错误。
4. event seq 是否连续，terminal event 是否应用。
5. 是否把成功 history 与失败 transfer 混用。
6. 检查聚合 bytes 是否回退、越界或永不终态。

### 进程退出异常

1. 确认当前实例的 endpoint、owner PID、tray PID、GUI PID 和 daemon PID。
2. 检查是否由 `launchctl submit` 或 keep-alive 启动。
3. 检查 Quit 是否错误进入 open-main 路径。
4. 检查 owned daemon 的 bounded shutdown。
5. 检查 runtime manifest、PID 文件、socket 和 action queue 是否清理。
6. 等待数秒确认没有自动重启。

### 通知异常

1. 确认 tray helper bundle id 与 `TrayInfo.plist` 正确。
2. 确认系统通知权限已授予 tray helper，不是只授予 GUI。
3. 确认 notification-only 连接没有收到 snapshot 或进度事件。
4. 确认文本和文件通知来源事件不同且不重复。
5. 检查 action queue 和 GUI control socket 路由。

## 5. 常用检查命令

```sh
git status --short

pgrep -fl '/Applications/LocalSend.app|localsendd|LocalSendTray' || true
pgrep -fl 'target/debug/localsendd|native_e2e_fixture|worker_flow' || true

find "$HOME/Library/Application Support/org.LocalSend.localsend-rs" \
  -maxdepth 2 \( -name 'gui.pid' -o -name 'gui-control.sock' -o -name 'runtime.json' \) -print

cargo check --workspace --locked
cargo test -p localsend-gui-slint --test worker_flow --locked
cargo clippy -p localsend-platform -p localsend-rpc -p localsend-tray \
  -p localsend-gui-slint -p localsend-service \
  --all-targets --locked --no-deps -- -D warnings
```

macOS App 安装后检查：

```sh
codesign --verify --deep --strict --verbose=2 /Applications/LocalSend.app
find /Applications/LocalSend.app/Contents -maxdepth 5 -type f -perm -111 -print | sort
```

不要运行 `cargo fmt --all` 修改 vendored core。当前官方 core 的原始格式与 Rust 1.97
rustfmt 不完全一致，全局检查可能失败。使用受影响 crate 检查：

```sh
cargo fmt -p localsend-gui-slint -p localsend-tray -p localsend-platform \
  -p localsend-rpc -p localsend-service -p localsendd -- --check
```

## 6. 禁止的省事做法

- 为了让界面“看起来有反应”而加假按钮、假进度或假完成状态。
- 在 GUI/TUI 内调用 `connect_or_spawn` 或直接启动 daemon。
- tray 读取完整 snapshot 来推断语言、设备或传输状态。
- 每条事件后重新拉 snapshot，或定时轮询 snapshot。
- 用全局进程名 kill 清理测试或生产进程。
- 修改 vendored core 来消除格式或 lint 失败。
- 用 AppleScript 驱动 tray 菜单或代替真实 GUI 输入链路。
- 只凭截图肉眼判断问题，或每一步都用截图代替日志和断言。
- 把 AX `AXPress` 成功、脚本退出码 0 或测试 skip 当作端到端通过。
- 为通过测试加入自动重试、宽泛超时或无上限 sleep。
- 在测试中使用真实 `53317`、真实 Downloads 或用户正在运行的 daemon，除非测试明确
  是发布前的官方互操作/组播实验。

## 7. 修改完成的最低交付

一个修复至少应包含：

1. 明确写出观察到的现象和复现路径。
2. 说明故障位于 UI、worker、RPC、daemon、core 或生命周期中的哪一层。
3. 有对应回归测试，或说明无法自动化的具体原因和人工证据。
4. 相关 crate 的 check/test/clippy 结果。
5. 如果涉及 App 发布：FemtoVG release、签名、安装位置、进程和窗口状态。
6. 没有新增孤儿进程、socket、PID 文件、诊断日志或测试设备可见于局域网。
7. 没有顺手格式化或修改 vendored official core。

## 8. 关键代码入口

- Slint UI 和事件接线：`apps/localsend-gui-slint/`
- GUI worker/状态：`apps/localsend-gui-slint/src/daemon_worker.rs`
- GUI 动作与 worker 集成测试：`apps/localsend-gui-slint/tests/worker_flow.rs`
- tray 生命周期和通知：`apps/localsend-tray/src/main.rs`
- tray 系统通知实现：`apps/localsend-tray/src/notifications.rs`
- GUI action/control socket：`crates/localsend-platform/src/gui_actions.rs`
- GUI/tray PID 管理：`crates/localsend-platform/src/gui_process.rs`
- daemon RPC server：`apps/localsendd/src/main.rs`
- RPC 类型和客户端：`crates/localsend-rpc/src/`
- 前端事件物化状态：`crates/localsend-rpc/src/snapshot_store.rs`
- 官方 core adapter：`crates/localsend-service/src/service.rs`
- 官方 core vendoring provenance：`VENDORED_CORE.md`、`THIRD_PARTY_NOTICES.md` 和
  `crates/localsend-official-core/SHA256SUMS`
- macOS 打包：`apps/localsend-gui-slint/package-macos.sh`

最后一条原则：先证明事实，再改代码。这个项目过去最耗时的返工，几乎都来自把
“界面现象”“AX 成功”“脚本退出码”“进程名相同”误当成真实原因。

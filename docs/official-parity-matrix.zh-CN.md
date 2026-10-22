# 官方 LocalSend 1.18.2 对齐矩阵

状态定义：

- `PASS`：有自动化测试或真实官方 App 证据。
- `FAIL`：已复现功能差异。
- `UNVERIFIED`：尚未用规定方式验证。
- `BLOCKED_EXTERNAL`：外部设备或权限当前不可用，代码侧不能替代验证。

## Phase 0：官方基线

| 项目 | 当前状态 | 证据 |
| --- | --- | --- |
| 官方 App 版本 `/Applications/LocalSendy.app` = 1.18.2 | PASS | 本机 App bundle 与官方源码 tag |
| 官方源码控件行为核对 | PARTIAL | 已核对设置标签、文本协议、发现超时、接收完成弹框和接收自动保存；其余弹框焦点/关闭规则待记录 |
| 900x600 三页基线 | PASS | `target/acceptance/official-parity/20261021-013855/baseline/official-{receive,send,settings}-900x600.png` 及 AX |
| 1024x768 / 1440x900 基线 | PASS | 同目录 `official-*-1024x768.png`、`official-*-1440x900.png` 及 AX |
| Light 主题三页基线 | PASS | `official-receive-light.png`、`official-send-light.png`、`official-settings-light.png` |
| 接收请求/文本/传输/完成/历史/Web Share/PIN 弹框 | PARTIAL | 已采集请求、文本、PIN、传输进度、完成弹框、历史、信息和 Web Share；失败/断网/取消异常态仍待采集 |
| 历史与网络信息页面 | PASS | `official-history-page.png/.ax.txt`、`official-network-info.png/.ax.txt` |
| Web Share 页面控件基线 | PASS | `official-web-share-page.png/.ax.txt`：URL、复制/打开/分享、加密、自动接受、PIN |
| 发送选择/设备/收藏/手动 IP/单选多选/PIN/成功失败 | PARTIAL | 已有发送和手动 IP 证据；视觉/交互矩阵待采集 |
| 设置完整分组、默认值、条件显示 | PARTIAL | 标签已对齐；逐项值和行为仍待记录 |

下一项：完成 Phase 0 的官方基线，随后按 `S1` 做双向功能矩阵；不要重复已有 Slint 综合门禁。

## Phase 1：文本消息

| 项目 | 当前状态 | 证据 |
| --- | --- | --- |
| 发送端创建 `text/plain`、真实 UTF-8 size、preview、可选 SHA-256 | PASS | `crates/localsend-service/tests/text_send.rs` |
| prepare 成功后上传真实文本字节 | PASS | `text_send` 测试验证接收文件内容与字节长度 |
| 接收当前官方协议并弹出文本消息 | PASS | `crates/localsend-service/tests/text_receive.rs` |
| 接收消息不落永久文件 | PASS | `text_receive` 测试验证接收目录为空 |
| 兼容旧 preview-only 消息 | PASS | `SenderPeer::send_legacy_preview_text` + `text_receive.rs` |
| 官方 App 接收 Slint 服务端文本并显示正文 | PASS | 2026-10-21 本机官方 App，`ui_seen=0.476s`，AX 显示 `new-text-protocol-check` |
| Slint GUI 按钮路径发送文本到官方 App | PASS | 安装版点击文本弹框与设备后 `ui_seen=0.733s`，官方 AX 显示正文 `5` |
| `#111` 文本 UI 延迟 | BLOCKED_EXTERNAL | TCP/HTTPS 已恢复可达，RPC `latency-text` 三次均 `refresh_ms=0 send_ms=0`；文件被远端 `409 Blocked by another session` 拒绝，需等远端会话释放后才能测真实出现/落盘时间 |
| GUI 完成后明确显示成功 | PASS | daemon worker 在 finished 后发布“文本已发送给 <设备>” |
| 发现超时默认值与官方一致 | PASS | 官方 UI=`500 ms`；RPC/DTO 默认 `500`，旧 `discovery_timeout_secs` 配置迁移并保留兼容测试 |

## Phase 2：速度

| 场景 | 官方 | 本项目 | 状态 |
| --- | --- | --- | --- |
| 同连接 `RefreshDiscovery → SendText` 命令延迟 | 未测 | `refresh_ms=0`, `send_ms=1` | PASS（本机官方 App） |
| 点击发送到官方 App 出现文本消息 | `<1s` 目标 | `0.476s`（RPC） / `0.733s`（Slint GUI） | PASS（本机） |
| 1 KiB 官方 App → 本项目（点击到落盘） | 未测 | `183.29 ms`，SHA-256 一致 | PASS（本机回环） |
| 10 MiB 官方会话 → 本项目 | `102.46 / 105.96 / 107.27 ms`（HTTP session） | `100 / 96 / 101 ms`（RPC→完成） | PASS（本机回环，SHA-256 一致） |
| 256 MiB 官方会话 → 本项目 | `2032.70 / 2035.44 / 2024.32 ms`（HTTP session） | `1989 / 2001 / 1996 ms`（RPC→完成） | PASS（本机回环，SHA-256 一致） |
| 2 GiB + 1 byte 本项目 → 官方 App | 未测 | `1368 ms`，约 `1496.63 MiB/s`，SHA-256 `b8030a8a…` 一致 | PASS（本机回环，稀疏 fixture） |
| 官方 App 点击到最终落盘 | 10 MiB `546.79 / 572.66 / 583.25 ms`；256 MiB `2818.47 / 2771.85 / 2775.31 ms` | 同左（实测） | PASS（含 Flutter UI/hash/finalize 口径） |
| HTTP/HTTPS 与跨机文件首字节时间 | UNVERIFIED | UNVERIFIED | 待采集局域网/第二台设备 |
| 速度汇总 JSON | PASS | `target/acceptance/official-parity/20261021-013855/interop/speed-summary.json` |

## Phase 3：Slint

| 项目 | 当前状态 | 证据 |
| --- | --- | --- |
| 接收文件请求、逐文件选择、接受/拒绝 | PASS（合成 peer） | `scripts/accept-slint-receive-settings.sh` 历史通过 |
| 接收文本弹框正文、复制、关闭 | PASS（本机官方 App） | `ours-text-applescript.png` |
| 发送文件/目录/文本/剪贴板 | PASS | `target/acceptance/slint-native/20261021-003633`；文本已按真实字节上传 |
| 发送文本给官方 App | PASS | Slint GUI 输入 `5` → 官方 AX 显示正文，`ui_seen=0.733s` |
| PIN 双向正确/错误 | PASS（真实官方 App） | `target/acceptance/official-parity/20261021-111000/interop/official-settings-pin/result.json`：官方→项目正确 PIN 收到文本、错误 PIN 显示 `PIN 无效`；项目→官方正确 PIN 收到文本、错误 PIN 记录 `401 Invalid PIN` |
| 官方 App 拒绝 | PASS（真实官方 App） | 官方请求页点击“拒绝”，发送端 daemon 得到 `403 Rejected`：`snapshot-after-reject.json` |
| 官方 App 取消 | PASS（真实官方 App） | 512 MiB 传输接受后由发送端取消，官方显示错误/100%，daemon transfer 状态为 `cancelled`：`cancel-result.png`、`snapshot-after-cancel.json` |
| 接收/部分接受/PIN/自动保存/自动完成/设置标签 | PASS | `target/acceptance/slint-receive-settings/20261021-050623` |
| 发送文本完成提示 | PASS | `worker_sends_text_bytes_and_reports_completion` |
| 完整 Slint 门禁 | PASS | `target/acceptance/slint/20261021-115536/summary.json`：3 runs、38 goldens、native AX、package、codesign、smoke（最新源码，单一完整运行） |
| 接收完成“打开文件”弹框 | PASS | 新增 Slint dialog；`accept-slint-receive-settings.sh` 验证出现、文件名和 5 秒自动关闭 |
| 自动保存解释弹框 | PASS | 开启自动保存后显示官方同文风险说明；`target/acceptance/slint-receive-settings/20261021-050623` |
| Web Share 加密/自动接受/PIN 控件 | PASS（Slint + worker） | `worker_starts_web_receive_page` 覆盖重启后 auto-accept；无障碍测试覆盖加密开关、返回停止分享 |
| Web Share 页面布局 | PASS | 按官方独立页面重构为返回箭头、居中标题、580px 内容区和链接卡片；新增 2 张 golden |
| Safari Web Share 上传/下载 | PASS | `target/acceptance/official-parity/20261021-013855/web-share/result.json`：Safari 实际上传与下载各 1 KiB，源/落盘 SHA-256 均为 `5f70bf18…`；`StopWebShare` 后 `/info` 恢复 `download=false` |
| Web Share 生命周期归属 | PASS（行为定义） | 浏览器标签关闭后服务仍保持启用，符合 daemon/frontend 持有分享生命周期的模型；官方 core 无浏览器页面关闭回调，只有前端 `StopWebShare` 才复位 |
| 官方 App 接收请求页对照 | PASS | `target/acceptance/official-parity/20261021-013855/interop/slint-to-official/official-request-dialog.png` |
| Receive 空闲页移除额外设备号/调试状态行 | PASS | 已按官方截图移除 `#80` 与底部状态行；34 张 golden 已更新 |
| 设置完整功能与官方标签 | PASS（macOS）/UNVERIFIED（Windows/Linux） | 标签、alias/port/HTTPS/PIN、接收目录、auto-save、校验和、网络组播/接口过滤/发现超时均已接线；`network_settings_restart` 验证重启和持久化；Slint bundled gettext 支持中英文运行时切换，`tests/language.rs` 覆盖三页、弹框、状态和设置当前值，38 张默认中文 golden 逐字节一致，24 张英文响应式截图已人工检查 |
| 进度、失败、历史、托盘生命周期 | PASS | Slint worker 覆盖 outgoing `403 Rejected` 失败提示与慢速取消；官方 App PIN/拒绝/取消已复验；安装版 tray→GUI→owned daemon→quit 生命周期通过 |
| Slint 设备卡真实鼠标点击 | PASS | `send_tile_click` 在卡片中心和标题文字区域分别发送 pointer press/release，均触发 `send-to-device`；真实鼠标点击出现选中边框和 Snackbar：`target/acceptance/gui-fixes/20261021-181311/manual-mouse/` |
| macOS 文件面板服务清收 | PASS | `macos_dialog_services` 只回收本次面板新增 PID，先 `SIGTERM`、超时后 `SIGKILL`；真实打开/取消面板后新增 PID 为 0，旧会话 `22435/22436` 已清理，其他应用更早的 `433/61427/61428` 保留 |
| 发送 AX（文件/目录/文本/剪贴板） | PASS | `target/acceptance/slint-native/20261021-013248/summary.json` |
| 接收/设置/PIN AX | PASS | `target/acceptance/slint-receive-settings/20261021-013344/summary.json` |

## Phase 4：边界

| 项目 | 状态 |
| --- | --- |
| 多文件、目录、Unicode、重名、mtime、SHA-256、符号链接 | PASS（服务集成） | `send_boundaries.rs` |
| 本项目 → 官方 App 嵌套目录 | PASS | `interop/directory-061205/result.json`：保留 `alpha.txt` 与 `sub/beta.txt`，哈希一致 |
| 官方 App → 本项目 嵌套目录 | PASS | `target/acceptance/official-parity/20261021-111000/interop/official-to-slint-directory/result.json`：官方 App 文件夹选择器发送 2 个文件，接收端保留 `alpha.txt` 与 `sub/beta.txt`，SHA-256 一致，耗时 `640.62 ms` |
| >2 GiB 流式传输 | PASS | `large_file_streaming.rs`：2048 MiB，2.21s |
| 独立 sender 部分上传后 SIGKILL | PASS | `scripts/accept-sender-kill.sh`：无残片，下一笔传输成功 |
| 取消、断网 | PARTIAL | 已有取消测试；断网场景仍需官方 App 复验 |

## Phase 5：发布

| 项目 | 状态 | 证据 |
| --- | --- | --- |
| FemtoVG release + codesign | PASS | `target/release/bundle/macos/LocalSend.app`，deep strict codesign |
| 安装版启动、tray、owned daemon | PASS | `target/acceptance/official-parity/20261021-121500/installed-final/` |
| 最新 release 构建、打包与安装 | PASS | `target/acceptance/official-parity/20261021-160559/release-final/summary.txt`：FemtoVG App、zip、deep strict codesign；`/Applications/LocalSend.app` 的 GUI/daemon SHA-256 与 release 一致 |
| 安装版 tray→GUI→daemon→quit 生命周期 | PASS | 从 tray 打开 GUI，native AX 导航三页；tray quit 后 GUI/daemon/tray 全部退出，无残留进程 |
| 安装版官方 App 文本/文件双向矩阵 | PARTIAL | 代表链路已通过，完整矩阵仍需机器空闲时补测 |
| 最新源码重新安装到 `/Applications/LocalSend.app` | PASS | `ditto` 覆盖安装后 `codesign --verify --deep --strict` 通过；未自动启动，避免抢占当前桌面 |
| 安装版启动冒烟 | PASS | `target/acceptance/official-parity/20261021-013855/installed-app-final.png` |
| 安装版 → 官方 App 文本 | PASS | `installed-app-to-official-text.png`，官方弹出消息正文并可复制/关闭 |
| 手动添加设备不被 discovery TTL 降级 | PASS | `installed-snapshot-with-manual-source.json`：官方设备经 discovery 更新后仍为 `source=manual` |

## Phase 6：egui

| 项目 | 当前状态 | 证据 |
| --- | --- | --- |
| Workspace 正式成员 | PASS | `apps/localsend-gui-egui` |
| 后台 RPC worker、断线重连、事件合并 | PASS | worker 与真实 daemon 集成测试 |
| 三页 UI、语义导航和控制流程 | PASS | `target/acceptance/egui/20261021-151251/summary.json`：9 tests、18 screenshots、AX；`target/acceptance/egui/20261021-152429/summary.json`：11 headless tests |
| 文本和文件 daemon 发送 | PASS | `egui_worker_sends_text_and_file_through_daemon` |
| 接收全部、部分接受和拒绝 | PASS | `egui_partial_accept_and_reject_are_driven_through_ui` |
| PIN 弹框与重试 | PASS | `egui_pin_dialog_retries_and_sends_file` |
| 设置控件、保存和 revision 冲突 | PASS | `egui_controls_cover_send_modes_webshare_and_settings`、`egui_snapshot_revision_conflict_keeps_unsaved_draft` |
| 设置完整语义效果 | PASS（macOS）/UNVERIFIED（Windows/Linux） | 中英文三页、弹窗、状态和设置选项值已本地化；窗口位置接入 eframe persistence，macOS/Linux/Windows 登录启动实现已接入但仅本机编译验收 |
| egui 发送端 headless 拒绝 | PASS | `egui_sender_surfaces_headless_peer_rejection`：官方 core peer 拒绝后 egui 显示 Failed，peer accepted 为空 |
| egui 发送端 headless 慢速取消 | PASS | `egui_sender_cancels_headless_slow_transfer_through_control`：通过 egui“取消”控件把 transfer 置为 Cancelled |
| Web Share 控件和 daemon 启动 | PASS | `egui_controls_cover_send_modes_webshare_and_settings` |
| 收藏新增、取消和重命名 | PASS | `egui_favorite_can_be_added_and_renamed_through_controls` |
| 原生 AccessKit 三页导航 | PASS | `target/acceptance/egui/20261021-151251/ax-smoke.log` |
| Light/Dark × 900/1024/1440 截图矩阵 | PASS | `target/acceptance/egui/20261021-151251/screens/`：18 张 |
| release 二进制 | PASS | `target/release/localsend-gui-egui`，Mach-O arm64；`target/acceptance/egui/release-smoke-localized/20261021-151638/summary.json` |
| egui → 官方 App 文件与 PIN 重试 | PASS | `target/acceptance/egui/official-interop/20261021-123657/summary.json`；双方 SHA-256 一致 |
| 官方 App → egui 文本消息 | PASS | 同上；历史正文和“文本消息”弹框均可见 |
| 官方 App → egui 文件夹 | PASS | `target/acceptance/egui/official-interop/20261021-123657/egui-folder-headless/official-to-egui-directory-final/result.json`：两个嵌套文件、21 B、双方 SHA-256 一致，682.51 ms |
| egui → 官方 App：PIN 重试后拒绝 | PASS（后台 AX） | `target/acceptance/egui/official-interop/1792569495-background-reject/summary.txt`：egui 输入官方 PIN 后官方请求页拒绝，egui 收到 `403 Rejected`，官方无落盘 |
| egui → 官方 App：待确认请求取消 | PASS（后台 AX） | `target/acceptance/egui/official-interop/1792569618-background-cancel/summary.txt`：egui 取消后 transfer 为 Cancelled，官方请求页关闭且无落盘 |
| 官方 App → egui PIN 输入 | UNVERIFIED | 官方 Flutter App 的 PIN 文本框拒绝 AX 写值、`CGEvent.postToPid` 按键和后台菜单 `Paste`；三者均不更新 controller，daemon 返回 `401 Invalid PIN`。需要前台真实键盘输入，按用户要求不在占用桌面时执行 |
| GUI 生命周期 | PASS（Slint 代码路径）/PASS（egui） | Slint 关闭窗口不再发送 `ShutdownDaemon`；egui 退出不持有 daemon，新增 GUI 单实例锁和 pid 归属，进程关闭不停止 daemon |

## 本地代码门禁（2026-10-21）

| 门禁 | 状态 | 证据 |
| --- | --- | --- |
| 最新 Slint 综合门禁 | PASS | `target/acceptance/slint/20261021-115536/summary.json`：3 runs、38 goldens、native AX、package、codesign、smoke |
| 项目代码测试（排除 vendored core） | PASS | `cargo test --workspace --exclude localsend` |
| Service 完整测试 | PASS | `cargo test -p localsend-service -- --test-threads=1`：文本、HTTPS、校验和、路径、网络重启、2 GiB 流式等非 ignored 场景通过 |
| 速度探针状态判定 | PASS | `rpc_probe benchmark-file` 现在只接受 `TransferStatus::Finished`；`Failed/Cancelled` 返回非零并输出真实 error，避免把失败样本计为 PASS |
| Clippy（项目 crate，`--no-deps -D warnings`） | PASS | rpc/service/testkit/slint/d/egui |
| vendored core 哈希 | PASS | `shasum -a 256 -c SHA256SUMS` |
| vendored core `event_backpressure::subnet_scan_finishes_when_events_are_not_consumed` | FAIL（环境限制） | 本机 macOS 上稳定只发现 1 个 127.0.0.x；未修改 vendored core |
| vendored core 其余测试 | PASS | `cargo test -p localsend --features full -- --skip subnet_scan_finishes_when_events_are_not_consumed` |


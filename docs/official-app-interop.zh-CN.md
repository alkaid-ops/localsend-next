# 官方 LocalSend App 双向联调记录

## 2026-10-21 官方 1.18.2 基线、速度与弹框复验

环境：macOS 本机回环，官方 `/Applications/LocalSendy.app` 1.18.2，本项目 daemon `Official Baseline Receiver`（HTTP，端口 `50313`）。全部证据位于：

`target/acceptance/official-parity/20261021-013855/`

### 基线

- 已采集官方 Receive、Send、Settings 三个页面在 `900x600`、`1024x768`、`1440x900` 下的 PNG 与 AX 语义树，位于 `baseline/`。
- 已额外采集 Light 主题三页：`official-receive-light.png`、`official-send-light.png`、`official-settings-light.png`；主题已恢复为“跟随系统”。
- 官方 Settings 在 `900x600` 下为左侧 `256px` NavigationRail、右侧 `644px` 内容区；通用分组之外的设置需要滚动。官方“搜索设备超时”默认显示 `500`，源码常量为 `500 ms`。
- 官方接收 256 MiB 文件时已捕获真实进度页与完成弹框：`interop/slint-to-official/progress-1950.png`、`progress-2300.png`。完成弹框包含“打开”“在文件管理器中显示”“关闭 (5)”以及 5 秒倒计时。
- 官方接收请求页已通过本项目 daemon 的 1 KiB 传输截取：`interop/slint-to-official/official-request-dialog.png`。布局为居中发送者、平台标签、请求文案、验证/设置、拒绝与接受。未在 60 秒内接受时，官方会显示“发送者取消了请求”，与当前发送准备超时方向一致。

### 双向速度

原始样本和汇总：`interop/speed-summary.json`。

| 尺寸 | 官方 App → 本项目协议会话 | 本项目 → 官方 App（RPC 到完成） | 结论 |
| --- | --- | --- | --- |
| 1 KiB | 点击到落盘 `183.29 ms` | `11 / 8 / 6 ms` | 小文件开销不同，均完成且哈希一致 |
| 10 MiB | `102.46 / 105.96 / 107.27 ms` | `100 / 96 / 101 ms` | 本项目中位耗时约为官方 `94.4%` |
| 256 MiB | `2032.70 / 2035.44 / 2024.32 ms` | `1989 / 2001 / 1996 ms` | 本项目中位耗时约为官方 `98.2%`，约 `128.26 MiB/s` |
| 2 GiB + 1 B | 未测 | `1368 ms`，约 `1496.63 MiB/s` | PASS（本项目 → 官方 App；SHA-256 一致） |

说明：官方会话时间来自 daemon 的 `Upload session created -> finished`；本项目时间为 `rpc_probe` 从发送命令到完成事件。官方“点击到最终落盘”另外包含 Flutter UI、哈希与 finalize，10 MiB 为 `546.79 / 572.66 / 583.25 ms`，256 MiB 为 `2818.47 / 2771.85 / 2775.31 ms`。

### 本轮修复

1. `discovery_timeout_secs=10` 改为官方一致的 `discovery_timeout_ms=500`；旧配置自动从秒迁移为毫秒，并有缺省值/迁移/新字段优先测试。
2. Slint 接收完成后新增官方风格“打开文件”弹框：打开、在文件管理器中显示、关闭；`auto_finish=true` 时 5 秒倒计时，关闭时保持。
3. 手动添加设备成功后保持 `source=manual`，不再被后续 discovery 更新降级为 120 秒 TTL 的 `source=discovery`。
4. 新增官方 App 文件发送 harness：`scripts/official-app-send-file.sh`，通过 AX + Vision OCR 选择真实文件并验证最终大小/SHA-256。
5. 新增官方 App 接收请求截图和 Slint 完成弹框 AX 用例；Slint 现有进度卡、请求页和完成弹框可与官方截图逐项对照。

### Safari Web Share 实测

使用独立 daemon 端口 `58852`、配置目录和接收目录，在真实 Safari 中完成：

- `/` 上传页：选文件 `web-share-upload-1k.bin`，落盘 1024 B，源文件与接收文件 SHA-256 均为 `5f70bf18a086007016e948b04aed3b82103a36bea41755b6cddfaf10ace3c6ef`。
- `mode=Share` 下载页：点击 1 KiB 文件链接并在 Safari“允许下载”后落盘，源文件与下载文件 SHA-256 一致。
- 浏览器标签关闭后 `/info` 仍为 `download=true`，这与官方 core 的设计一致：分享生命周期由持有 daemon 的前端控制，不由浏览器标签控制。执行 `StopWebShare` 后 `/info` 立即恢复 `download=false`。
- 证据：`target/acceptance/official-parity/20261021-013855/web-share/result.json`、`safari-upload-complete.png`、`safari-download-complete.png`、对应 `.ax.txt`。

### 官方 App 文件夹发送实测

通过官方 App 的真实“文件夹”选择器和 AX 点击完成官方 App → 本项目隔离 daemon 的嵌套目录传输。首次实测发现接收端使用 `sanitize_path` 只保留最后一段，`sub/beta.txt` 被扁平化为 `beta.txt`；已改为逐段清洗并保留安全相对路径。

- 修复后的接收树：`ls-e2e-<token>/alpha.txt`、`ls-e2e-<token>/sub/beta.txt`。
- 两个文件 SHA-256 均与源文件一致，传输耗时 `640.62 ms`。
- `../../escape.txt` 等目录穿越片段只落入接收根目录，不能逃逸 `destination_dir`；新增 `crates/localsend-service/tests/receive_paths.rs`。
- 证据：`target/acceptance/official-parity/20261021-111000/interop/official-to-slint-directory/result.json`、`finished.png`、`selected.ax.txt`。

### PIN、拒绝与取消实测

使用官方 App 1.18.2 的真实 PIN 弹框和请求页，双向完成：

- 官方 App → 本项目：测试 PIN `2468` 时收到文本正文 `official pin text`；输入 `0000` 时官方显示 `PIN 无效`，项目历史不新增消息。
- 本项目 → 官方 App：在官方设置中开启 PIN `2468` 后，正确 PIN 的文本显示在官方消息页；错误 PIN 在 daemon 状态中记录 `401; Invalid PIN`。
- 官方 App 接收项目文件请求时点击“拒绝”，项目发送端记录 `403; Rejected`。
- 官方 App 接受 512 MiB 文件后，由发送端取消，官方显示完成但错误，项目 transfer 状态为 `cancelled`。
- 证据：`target/acceptance/official-parity/20261021-111000/interop/official-settings-pin/result.json`、`pin-text-complete.ax.txt`、`wrong-pin-result.ax.txt`、`snapshot-after-project-wrong-pin.json`、`snapshot-after-reject.json`、`cancel-result.png`。

### Slint 最终门禁与安装版

- 单一完整运行：`target/acceptance/slint/20261021-115536/summary.json`，3 次测试、38 张 golden、native 文件/目录/文本/剪贴板、接收设置/PIN、FemtoVG 打包、签名和 smoke 全部通过。
- 安装版 `/Applications/LocalSend.app` 的 GUI SHA-256 为 `238fc06d…`，daemon 为 `2f2c4003…`，与 release 包逐字节一致。
- 从安装版 tray 打开 GUI，AX 成功切换发送和设置页；tray quit 后 tray、GUI、owned daemon 全部退出。
- 证据：`target/acceptance/official-parity/20261021-121500/installed-final/summary.txt`、`installed-hashes.txt`。

### egui 真实性互测

- egui → 官方 App：通过真实 egui 文件和设备按钮发送，官方返回 401 后在 egui PIN 弹框输入 `2468`，文件最终落盘；双方 SHA-256 均为 `cc2b9599…`。
- 官方 App → egui：官方发送文本 `egui official reverse message 123`，egui daemon 历史记录收到正文，egui 窗口显示可复制的“文本消息”弹框。
- 证据：`target/acceptance/egui/official-interop/20261021-123657/summary.json`、`egui-to-official/hashes.txt`、`egui-https-reverse/after-official-send2.ax.txt`。
- 官方 App → egui 文件夹：官方目录选择器选中两个文件后由 egui daemon 自动接收；`alpha.txt`、`sub/beta.txt` 保留相对路径，双方 SHA-256 一致，耗时 `682.51 ms`。
- 文件夹证据：`target/acceptance/egui/official-interop/20261021-123657/egui-folder-headless/official-to-egui-directory-final/result.json`。

### 仍未覆盖

- 官方 App 的 Light 主题三页、历史/Web Share/失败态弹框对照。
- 局域网第二台设备或真实 `#111` 的跨机首字节与吞吐。
- 2 GiB 官方 App 双向实测；当前仅有 service 层 `2 GiB + 1 byte` 流式测试。
- 最新源码安装版 `/Applications/LocalSend.app` 复验，以及取消/拒绝/PIN 全矩阵截图化。

日期：2026-10-20  
官方参考版本：LocalSend 1.18.2（`/Applications/LocalSendy.app`）  
本项目：`localsend-next`，Slint + FemtoVG，官方 core + 自有 daemon  
隔离实例：daemon 端口 `53532`，配置目录 `/tmp/ls-official-slint-20261020-215501/config`，接收目录 `/tmp/ls-official-slint-20261020-215501/received`

## 验收方法

- 不使用 AppleScript 操作托盘。窗口和按钮通过 macOS AX 语义树定位，按钮通过 `AXPress` 触发。
- Flutter 文本框的 `AXValue` 写入只改变可访问性值，不会可靠更新 Flutter controller；文本用例改用聚焦后的真实键盘输入，再通过 AX 点击“确认”和“接受”。
- 每个阶段保存官方窗口和 Slint 窗口截图，并单独计算接收文件 SHA-256。
- 测试实例与用户日常 daemon 使用不同配置目录、端口和 PID；未复用真实下载目录。

## 双向矩阵

| 场景 | 结果 | 证据 |
| --- | --- | --- |
| 官方 App → Slint：单文件 | PASS | `localsend-official-to-slint-215936.txt`，26 B，SHA-256 `d4be83b5279b8e75a389d923af3ee6cf5cde85a345e666640da885dc94157621` |
| 官方 App → Slint：文本消息 | PASS | 官方键盘输入后，Slint 弹出“文本消息”对话框，正文为 `applescript-message-20261020`，可复制、关闭；历史记录保留同一正文 |
| 官方 App → Slint：256 MiB 文件 | PASS | SHA-256 `a6d72ac7690f53be6ae46ba88506bd97302a093f7108472bd9efc3cefda06484`，双方一致；Slint 进度卡显示 100%，自动完成后清空活动传输 |
| Slint → 官方 App：单文件 | PASS | 官方入站页显示发送方 `Slint Official Test` 和 `macOS`；接受后文件哈希 `cc7bf62cd00d63d26c11c36272c6192b41e1648e74ffc115b418600f031edaa5` 与原文件一致 |
| 手动 IP 添加官方 App | PASS | 通过 `AddManualDevice 127.0.0.1:53317` 探测到官方实例，随后完成发送与接收；GUI 对话框现在提交后立即关闭 |
| 安装后的 `/Applications/LocalSend.app` 接收官方文件 | PASS | release 包启动后由 tray 拉起 GUI/owned daemon；`localsend-final-smoke.txt` 哈希 `7486b3a3a28555ae8686ad278eca3c1aa891e9b83855ee6f6c6ebe0cc2e93730`，发送端与接收端一致 |

截图、AX dump 和连续进度帧位于：

`target/acceptance/official-interop/20261020-215501/`

重点文件：

- `official-big-waiting.png`：官方等待接收确认。
- `ours-big-request.png`：Slint 接收请求。
- `seq/ours-012.png`：Slint 接收进度/完成卡片。
- `ours-text-applescript.png`：非空文本消息对话框。
- `official-incoming-fixed.png`：修复后的发送方别名和平台标识。
- `ours-settings-aligned.png`、`official-settings.png`：设置页对照。

## 本轮发现并修复

1. 发送侧 `RegisterDto.alias` 原本硬编码为 `localsend-next`，官方 App 会把我们显示成错误设备名。现在使用 daemon 的设备别名，平台标识使用 `macOS`/`Windows`/`Linux`。
2. “发送文件时创建校验和”和“接收文件时验证校验和”原来没有持久化字段，也没有协议行为。现在发送端按文件流式计算 SHA-256，接收端按设置启用 core 校验。
3. 校验失败时官方 core 会先落盘再返回 422，原实现会留下不可信文件。现在接收失败会删除该目标文件。
4. 文件选择从单选改为 `pick_files()`；验收注入使用 `LOCALSEND_E2E_FILE_PATHS`（换行分隔）并保留旧的单路径变量。
5. 手动添加设备提交后对话框不关闭。现在提交后清空目标并立即关闭；失败仍由 daemon Notice 显示。
6. 动画开关没有作用。接收页 logo 现在只在动画启用且服务器运行时旋转；关闭后固定到 0°。
7. 组播、网络接口和发现超时修改后不会重新配置发现链路。现在这些 revision-safe 设置会触发 server/discovery 重启。
8. 发现设备 TTL 过短，官方 App 不持续广播时会从列表消失。TTL 提高到至少 120 秒。
9. 设置页标签按官方 1.18.2 对齐：自动保存、收藏夹自动保存、启用 PIN 密码、保存目录、搜索设备超时、加密、多播、发送校验和、接收校验和。
10. 切换导航时清除上一页面残留的传输错误，避免错误文本永久挂在“设置”页顶部。
11. 文本/文件选择后的 `RefreshDiscovery` 原来会同步等待完整组播 announce，随后排队的 `SendText`/`SendFiles` 会被阻塞数秒。现在发现刷新在后台任务执行，发送命令立即进入协议阶段。

## 仍存在的差异和边界

- 语言选项会持久化，但当前 Slint 文案仍以中文为主，尚未完成全量本地化。
- 256 MiB 本机回环传输很快，连续帧能稳定捕获请求前、等待、100% 完成和自动清理；中间的 1%–99% 页面需要在跨机或限速链路补测。
- 本轮只验证 macOS 13+。Windows/Linux 的原生文件选择器、托盘和路径规则未在真实设备上验收。
- `localsend-rs` 与官方的 v3/WebRTC 能力不在本轮 PC 核心验收范围。

## 门禁结果

- `cargo check --workspace`：PASS。
- `cargo test -p localsend-service --test checksum_settings --test verify_checksums_setting --test https_send --test auto_save_finish`：PASS。
- `cargo test -p localsend-gui-slint -- --test-threads=1`：PASS。
- `cargo clippy -p localsend-rpc -p localsend-service -p localsend-testkit -p localsend-gui-slint --all-targets --no-deps -- -D warnings`：PASS。
- 完整 workspace clippy 会进入 vendored 官方 core 并报告其上已有的 32 个 lint；按仓库约束不修改 vendored core。
- `codesign --verify --deep --strict /Applications/LocalSend.app`：PASS（ad-hoc，本机测试包）；tray、GUI、owned daemon 均从安装包启动。
- 同一 RPC 连接先发送 `RefreshDiscovery` 再发送 `SendText` 的本地延迟探针：`refresh_ms=0 send_ms=0`，确认发现刷新不再堵塞发送队列。
- 对局域网测试设备（`192.0.2.111`，文档中的 `#111`）执行同一延迟探针：`refresh_ms=0 send_ms=0`，包含进程启动的 CLI 总耗时 `0.42s`。


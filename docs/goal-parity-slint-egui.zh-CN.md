# Goal：以官方 LocalSend 为基准完成 Slint，再完成 egui

## Goal 目标

把 `localsend-next` 做到与官方 LocalSend 1.18.2 在 PC 三页（接收、发送、设置）上功能一致、传输速度可接受、弹框和传输体验一致。执行顺序固定：

1. 先完成 Slint + FemtoVG。
2. Slint 全部门禁通过并安装验收后，再完成 egui。
3. 任何阶段都不能用“自动化测试通过”代替真实官方 App 互测。
4. 不修改 `crates/localsend-official-core`。

官方参考：

- 官方 App：`/Applications/LocalSendy.app`
- 官方源码：同级 `localsend` 工作树
- 本项目：当前 `localsend-next` 仓库
- 正式 GUI：`apps/localsend-gui-slint`
- 后续 GUI：`apps/localsend-gui-egui`

## 不可违反的执行规则

- 每个功能点必须先复现官方行为，再写失败测试，再改实现。
- 真实 App 验收使用隔离配置、隔离端口和隔离接收目录；结束后清理进程和测试文件。
- macOS 原生自动化优先使用 AX 语义树。禁止用截图坐标猜按钮。
- Flutter 文本框的 `AXValue` 写入不会可靠更新 controller；官方 App 文本输入使用聚焦后的真实键盘输入。
- 每次测试必须保存：命令输出、AX dump、截图、transfer 状态、文件哈希、daemon/进程清理报告。
- 速度问题必须测量时间线，不能通过增加延迟、sleep 或放宽断言掩盖。
- 每完成一个 Phase，更新 `docs/official-parity-matrix.zh-CN.md`，并创建一个可运行状态。
- 同一问题连续 3 次修复失败时，停止该分支，记录复现步骤、日志和阻塞原因，不继续猜测。

## Goal Runner 执行契约

本文件供低思考度模型在 goal 模式逐轮执行。每轮必须遵守以下顺序，不得并行跳到后面的 Phase。

1. 读取 `docs/official-parity-matrix.zh-CN.md`，选择第一个 `FAIL` 或 `UNVERIFIED`，不要重跑已标为 `PASS` 且证据仍有效的场景。
2. 先记录官方 1.18.2 同场景行为，再写一个能失败的自动化或 AX 用例；无法自动化的步骤必须明确标记并保存人工证据。
3. 只改达成该项所需的最小范围；禁止改 vendored core，禁止用 sleep、放宽阈值或删除断言掩盖问题。
4. 运行该场景的定向测试；通过后再跑一次受影响的 suite，不重复执行无关全套门禁。
5. 用 `/Applications/LocalSendy.app` 完成真实 App 互测，保存截图、AX dump、时间线、哈希和进程清理报告。
6. 更新矩阵中的状态、官方行为、当前结果和证据路径；未拿到真实 App 证据时只能保持 `UNVERIFIED`。
7. 创建一个可运行 commit。若同一问题连续三次失败，记录 `BLOCKED_EXTERNAL` 或技术阻塞，不继续猜。

统一证据目录：

```text
target/acceptance/official-parity/<timestamp>/
  baseline/                 # 官方 App 截图、AX dump、设置值
  slint/                    # 当前 Slint 同场景证据
  benchmark/                # timeline.json、throughput.json
  interop/                  # 双向 hash、transfer、截图
  processes.txt             # 测试前后进程和清理结果
  summary.json              # 机器可读 PASS/FAIL/UNVERIFIED
```

现有可复用门禁：

- `./scripts/accept-slint.sh`：单元/worker、3 轮、34 张 golden、原生 AX、package、codesign、GUI smoke。
- `./scripts/accept-slint-native.sh`：文件、目录、文本、剪贴板 AX 路径。
- `./scripts/accept-slint-receive-settings.sh`：部分接受、拒绝、文本、自动保存、自动完成、PIN。
- `./scripts/accept-egui.sh`：egui 11 条自动化、clippy、原生 AX、18 张视觉截图和清理；设置 `LOCALSEND_ACCEPT_EGUI_HEADLESS=1` 时只跑无窗口门禁。
- `./scripts/accept-sender-kill.sh`：独立 sender 中断、残片清理和下笔传输恢复。

## 当前证据快照（2026-10-21）

以下是已完成基线，除非相关源码再次变化，不要重复执行整套：

| 项目 | 当前状态 | 证据 |
| --- | --- | --- |
| 最新 Slint 综合门禁 | PASS | `target/acceptance/slint/20261021-115536/summary.json` |
| 最新发送 AX 矩阵 | PASS | `target/acceptance/slint-native/20261021-013248/summary.json` |
| 最新接收/设置/PIN AX | PASS | `target/acceptance/slint-receive-settings/20261021-013344/summary.json` |
| 文本协议、旧协议兼容、文本临时文件清理 | PASS | `crates/localsend-service/tests/text_send.rs`、`text_receive.rs` |
| 本机官方 App 文本出现延迟 | PASS | RPC `0.476s`，Slint GUI `0.733s` |
| 独立 sender 中断 | PASS | `target/acceptance/sender-kill/20261021-005520` |
| >2 GiB 流式传输 | PASS | `crates/localsend-service/tests/large_file_streaming.rs` |
| 2 GiB + 1 byte 本项目 → 官方 App | PASS | `interop/speed-summary.json`：1368 ms、SHA-256 一致 |
| `#111` 局域网复测 | BLOCKED_EXTERNAL | `https://192.0.2.111:53317/api/localsend/v2/info` 可达，RPC 文本命令 `0 ms`；文件返回 `409 Blocked by another session`，不能计为速度 PASS |
| egui 自动化与视觉门禁 | PASS | `target/acceptance/egui/20261021-151251/summary.json`：9 tests、18 screenshots、AX smoke；`target/acceptance/egui/20261021-152429/summary.json`：11 tests headless |
| egui release 二进制 | PASS | `target/release/localsend-gui-egui`；`target/acceptance/egui/release-smoke-localized/20261021-151638/summary.json` |
| egui 与官方 App 代表性双向互测 | PARTIAL | egui→官方文件（PIN 重试）、拒绝和待确认请求取消均通过后台 AX；官方→egui 文本和嵌套文件夹哈希一致；官方→egui PIN 输入仍受 Flutter 前台键盘限制 |
| GUI 生命周期修正 | PASS | Slint 关闭前端不再停止 daemon；egui 退出不停止 daemon，并新增 GUI 单实例锁和 pid 归属 |
| 网络设置映射与重启 | PASS | `settings_tests` 覆盖组播/过滤/超时映射；`network_settings_restart` 验证真实 server 停止/重启、revision 增加和设置持久化 |
| Slint 发送失败与取消状态 | PASS | `worker_surfaces_rejected_outgoing_transfer` 验证 `403 Rejected` 失败事件/UI notice；`worker_cancels_slow_outgoing_transfer` 验证 Cancelled 状态 |
| Slint 中英文运行时切换 | PASS | bundled gettext + `@tr` 覆盖三页、弹框、传输状态、历史和设置当前值；`tests/language.rs` 验证中文/英文切换及动态 `Waiting` 状态 |
| Slint 设备点击与面板服务回收 | PASS | 设备卡标题区域真实 pointer 测试和真鼠标 Snackbar/选中态通过；原生面板守卫按 PID 差集 TERM/KILL，真实打开/取消面板后新增 PID 为 0；旧 `22435/22436` 已清理，共享历史服务保留。证据：`target/acceptance/gui-fixes/20261021-181311/manual-mouse/` |
| 最新 release 打包与安装 | PASS | `target/acceptance/official-parity/20261021-160559/release-final/summary.txt`：FemtoVG App、deep strict codesign、安装哈希与 release 一致 |
| 官方 App 三页基线（900/1024/1440） | PASS | `target/acceptance/official-parity/20261021-013855/baseline/` |
| 官方 App Light 主题三页基线 | PASS | 同目录 `official-*-light.png` |
| 官方 App 接收进度与完成弹框 | PASS | `target/acceptance/official-parity/20261021-013855/interop/slint-to-official/progress-1950.png`、`progress-2300.png` |
| 官方 App 接收请求弹框 | PASS | `target/acceptance/official-parity/20261021-013855/interop/slint-to-official/official-request-dialog.png` |
| 官方历史、网络信息、Web Share 基线 | PASS | `baseline/official-history-page.png`、`official-network-info.png`、`official-web-share-page.png` |
| 官方 App 双向文件速度（10/256 MiB） | PASS | `target/acceptance/official-parity/20261021-013855/interop/speed-summary.json` |
| Slint 接收完成弹框与 5 秒自动关闭 | PASS | `target/acceptance/slint-receive-settings/20261021-050623/summary.json` |
| 安装版 `/Applications/LocalSend.app` | PASS | 最新 release、tray/GUI/owned daemon 启动、codesign 通过；`installed-app-final.png` |

仍未完成、且决定 Slint 是否能收口的事项：

- 官方 App 真实双向完整矩阵：文本、单文件、多文件、目录、部分接受、拒绝、取消、PIN、HTTPS。
- 官方 App 与 Slint 的同状态截图、弹框尺寸/位置/按钮顺序/关闭行为对照。
- 2 GiB 的官方 App 对照吞吐；1 KiB/10 MiB/256 MiB 已完成。
- 每个设置项的真实行为，而不只是标签和持久化。
- 安装版 `/Applications/LocalSend.app` 在最新源码上的官方 App 双向验收。
- 取消、断网、对端退出、失败提示和下次传输恢复的官方 App 场景证据。

`docs/official-parity-matrix.zh-CN.md` 是唯一进度看板。规划文字不能替代其中的证据。

## 固定任务队列

Goal 模式只能按以下编号向前推进；前一项未 PASS，不得开始后一项。

| 编号 | 任务 | 完成证据 |
| --- | --- | --- |
| S0 | 官方 1.18.2 三页和全部弹框基线 | baseline 截图、AX、控件行为表 |
| S1 | 官方 App ↔ Slint 功能矩阵 | 双向文本/文件/目录/PIN/拒绝/取消哈希和截图 |
| S2 | 速度 harness 与 4 种尺寸/2 种协议 | timeline.json、throughput.json、三次 p95 |
| S3 | Slint 弹框和视觉差异修正 | 同状态截图、AX 位置、对比结论 |
| S4 | 设置项逐项真实行为 | 每项 UI → daemon → 可见效果证据 |
| S5 | 异常路径 | 取消、断网、sender kill、daemon kill、恢复 |
| S6 | Slint release + 安装版验收 | codesign、tray、owned daemon、官方 App 双向 |
| E0 | egui 视觉基线 | egui/Slint/官方三页与弹框截图 |
| E1 | egui 接收页 | kittest + 官方 App 入站矩阵 |
| E2 | egui 发送页 | kittest + 官方 App 出站矩阵 |
| E3 | egui 设置页 | 全设置同步和 revision 冲突 |
| E4 | egui 生命周期 | tray/daemon/GUI 冒烟与清理 |
| E5 | egui 官方 App 矩阵 | 与 S1 相同的双向矩阵 |
| E6 | egui 视觉/响应式 | 5 尺寸、Light/Dark、长文本 |
| E7 | egui 性能与 native smoke | 复用 S2 阈值 |

下一次 continuation 从矩阵中第一个未通过项开始。`S0` 的主页面基线已完成，还剩历史/Web Share/失败态弹框；随后继续 `S1` 双向功能矩阵，不要重复三页默认截图。

## Phase 0：冻结基线

目标：明确官方行为、当前差异和可重复测试环境。

任务：

- 以 900x600 为主尺寸，1024x768、1440x900 为补充尺寸，采集官方 App：
  - 接收空闲、文件请求、部分选择、文本消息、传输中、完成、失败、历史、信息、Web Share、PIN。
  - 发送未选择、文件已选、附近设备、收藏、手动 IP、单选/多选、PIN、进度、失败、成功。
  - 设置完整分组和每个控件默认值。
- 从官方源码核对每个控件的实际行为，不只依据截图。
- 生成 `target/acceptance/official-parity/<stamp>/baseline/`。
- 建立 `docs/official-parity-matrix.zh-CN.md`，列：功能、官方行为、当前结果、证据、状态。
- 确认 Slint 当前 release 能由 tray 启动 GUI 和 owned daemon。

Phase 0 门禁：

- 能列出官方三页所有可见控件及其行为。
- 每个当前差异都有官方截图或源码依据。

## Phase 1：文本协议与延迟回归保护

协议实现已完成，保留本 Phase 作为回归门槛，不再重写发送协议。

每个后续提交若改动 service 文本路径，只运行：

```sh
cargo test -p localsend-service --test text_send --test text_receive
cargo test -p localsend-gui-slint worker_sends_text_bytes_and_reports_completion
```

必须继续成立：

- `text/plain`、真实 UTF-8 `size`、`preview`、按设置生成 SHA-256。
- prepare 后上传真实文本字节；旧 preview-only 消息仍兼容。
- 接收消息后弹出正文，临时文件删除，历史保留正文。
- 本机官方 App 从 Slint 点击设备到出现请求 `<1s`，连续 3 次 p95 `<1.2s`。
- 官方 App 发送端显示完成，不出现重复设备或停在“等待接收”。

只有出现失败证据时才修改实现；没有新失败时禁止重复优化文本路径。

## Phase 2：传输速度与时间线

目标：文本不慢于 1 秒；文件首字节和吞吐量达到官方同环境结果的可接受范围。

测量点：

- UI 点击设备时间。
- daemon 收到 `SendText/SendFiles` 时间。
- prepare-upload 发出/返回时间。
- 目标出现请求时间。
- upload 首字节、50%、完成时间。
- 每秒字节数和总耗时。

测试矩阵：

- 文本：1 字节、1 KiB、64 KiB。
- 文件：1 KiB、10 MiB、256 MiB、2 GiB。
- 协议：HTTP、HTTPS。
- 场景：本机官方 App、局域网 `#111`、另一台 PC。

验收阈值：

- 文本从点击设备到对端出现请求：目标 `<1s`；连续 3 次 p95 `<1.2s`。
- 文件 prepare 返回：p95 不超过官方同场景 1.5 倍。
- 文件吞吐：不低于官方同场景 80%。
- 大文件传输期间 UI 每 250ms 至少刷新一次进度，不能假死。
- 任何速度修复都不能牺牲校验和、取消、失败清理或进度准确性。

禁止：

- 通过缩短 discovery timeout 掩盖阻塞。
- 用固定 sleep 让测试看起来通过。
- 只测 loopback 就宣布性能达标。

### Phase 2 执行顺序

1. 新建 `scripts/bench-official-app.sh`：同一台 Mac 上启动隔离 daemon，并确认官方 App `53317` 与本项目测试端口都可访问。
2. 新建 `scripts/bench_transfer_probe.py`：生成确定内容的 1 KiB、10 MiB、256 MiB、2 GiB fixture；测试后删除，2 GiB 使用稀疏文件避免占盘。
3. 对每个尺寸分别测：
   - 官方 App → Slint daemon（官方作为 sender）。
   - Slint → 官方 App（Slint 作为 sender）。
   - HTTP 与 HTTPS 各至少一轮。
4. 每轮至少 3 次，输出原始样本、中位数、p95、首字节时间、吞吐 MiB/s 和双方 SHA-256。
5. 本机 loopback 只用于性能回归；速度结论必须另有局域网设备或第二台机器的证据。
6. 若 `<1 KiB` 或 `10 MiB` 小于官方 80%，先查同步 discovery、重复 identity、UI 线程阻塞和 HTTP/1.1 连接复用，再改代码。

Phase 2 产物必须包含：

```json
{
  "scenario": "slint_to_official",
  "protocol": "https",
  "size_bytes": 268435456,
  "samples": [{"prepare_ms": 0, "first_byte_ms": 0, "total_ms": 0, "mib_per_s": 0}],
  "official_reference": {"median_mib_per_s": 0},
  "ratio_to_official": 0,
  "sha256_match": true,
  "status": "PASS|FAIL|UNVERIFIED"
}
```

Phase 2 门禁：文本 p95、文件 prepare p95、吞吐比例、进度刷新和哈希全部 PASS；否则不得进入 Slint 发布收口。

## Phase 3：Slint 三页与弹框完整对齐

### 接收页

- 空闲状态：logo、alias、设备号、服务器状态、历史/信息按钮、通过链接按钮。
- 文件请求：文件列表、逐文件勾选、拒绝、接受所选、全部接受。
- 文本请求：参考官方的消息弹框，显示发送者、完整正文、复制、关闭。
- 传输中：进度、速度、取消。
- 完成/失败：明确结果，自动完成设置生效。
- 历史：文本全文、文件路径、打开文件和清空历史。

### 发送页

- 未选择文件时的四个大按钮。
- 已选择文件卡片、数量、大小、移除/清除。
- 附近设备、收藏、手动 IP、扫描、发送模式。
- 单选立即发送；多选明确确认。
- 文本和剪贴板发送。
- PIN 输入、错误重试、取消。
- 发送进度、accepted/refused/skipped、完成和失败。
- 完成后保留/清除选择的规则必须与官方 App 实测一致。

### 设置页

- 分组、顺序、标签与官方 1.18.2 对齐。
- 所有开关必须有真实 daemon 行为，不允许只保存 UI 状态。
- 主题、颜色、语言、窗口位置、托盘、开机启动、动画。
- 自动保存、收藏夹自动保存、PIN、保存目录、自动完成、历史、接收校验和。
- Web Share 自动接收、发送校验和。
- 设备名、端口、网络、搜索超时、加密、多播。
- revision 冲突可见；更新后状态 5 秒内同步。

视觉与交互门禁：

- 900x600 下与官方同状态截图对比。
- 700x700、1024x768、1440x900 无重叠、截断或按钮越界。
- 弹框宽度不超出窗口，长文本可滚动/复制。
- 所有发送、接收流程必须有明确的“等待、进行、成功、失败”状态。

自动化门禁：

- 使用 AX 语义树驱动 Slint 控件；只允许初始化树时做一次定位，不反复网格扫描。
- 文本输入使用真实键盘注入。
- 真实官方 App 文件、文件夹、文本、剪贴板、拒绝、取消、PIN、部分接受全部有证据。

### Slint 视觉与弹框逐项门禁

主窗口固定 `900x600`；补充 `700x700`、`1024x768`、`1440x900`。官方 App 和 Slint 必须处于同一页面、同一主题、同一业务状态，再保存：

- `official.png`
- `slint.png`
- `official.ax.txt`
- `slint.ax.txt`
- `comparison.json`

必须逐项对照：

| 场景 | 核对内容 |
| --- | --- |
| 接收空闲 | logo、alias、ID、服务器状态、历史和“通过链接”按钮的顺序与位置 |
| 文件请求 | 发送者、文件列表、复选框、拒绝/接受所选/全部接受 |
| 文本消息 | 发送者、完整正文、复制、关闭、换行和长文本滚动 |
| 传输中 | 文件名、方向、百分比、速度、取消按钮、完成后的消失时机 |
| PIN | 标题、输入框、确定/取消顺序、错误文案、重试后保留原内容 |
| 手动 IP | 输入框、确认/取消、关闭时机、非法地址错误 |
| 发送选择 | 四类 BigButton、已选择卡片、数量/大小、编辑/添加/清除 |
| 设备区 | 附近/收藏分组、单选/多选、收藏标记、HTTP/HTTPS badge、空状态 |
| 设置 | 分组、顺序、标签、当前值、开关反馈、保存/失败提示 |
| 历史 | 消息全文、文件路径、空状态、打开文件/目录、清空历史 |
| Web Share | 分享 URL、复制、停止、自动接收、浏览器关闭后的状态复位 |

自动比对只负责发现明显回归：

- 窗口截图尺寸必须完全一致。
- 关键控件 AX 位置偏差不超过 `8px`；正文/标题字号偏差不超过 `2px`。
- 主色采样每通道偏差不超过 `8`；圆角偏差不超过 `2px`。
- 语义树中的主要标签、控件顺序和按钮状态必须一致。
- 字体抗锯齿、平台字体字形差异不作为失败，但布局留白和层级必须一致。

弹框行为门禁：

- 打开后焦点进入首个输入框或主操作，Esc/取消可关闭。
- 弹框不得超出窗口，长正文和长文件名可滚动。
- 成功/失败后弹框关闭规则与官方一致，不残留旧错误。
- 传输请求由第一个有效响应决定，其他前端显示已处理而不是再次操作。

## Phase 4：文件、目录与异常路径

- 多文件与文件夹混合选择。
- Unicode、长文件名、重名文件、空目录、嵌套目录。
- 不跟随符号链接。
- mtime、SHA-256、目录相对路径。
- 大文件 >2 GiB 流式传输。
- 发送端 SIGKILL、daemon SIGKILL、断网、取消、对端拒绝。
- 失败无残片；下一次传输不受污染。
- 官方 App 与本项目双向各跑一轮。

## Phase 5：Slint 发布验收

- `RUSTUP_TOOLCHAIN=stable cargo test --features desktop`。
- `cargo clippy -p ... --all-targets --no-deps -- -D warnings`。
- release 固定 FemtoVG。
- `package-macos.sh` 生成 App，校验内嵌 `localsendd`、`localsend-tray`、GUI。
- 安装 `/Applications/LocalSend.app`。
- Finder 启动、托盘启动、关闭窗口、托盘退出、owned daemon 清理。
- 安装版与官方 App 完成文本、文件、目录、PIN、拒绝、取消双向验收。

Phase 5 门禁：

- 不允许用 debug binary 代替安装版验收。
- 所有验收截图和命令输出写入 `target/acceptance/official-parity/<stamp>/slint-final/`。

只有 `docs/official-parity-matrix.zh-CN.md` 的 Slint 部分全部 PASS，才允许开始 Phase 6。egui 开发可以与 Slint 源码阅读并行，但不得用 egui 的绿测结果推进 Slint 状态。

## Phase 6：egui 功能移植

前提：Phase 0–5 全部通过。Slint 不再有阻断性功能差异后才开始。

架构要求：

- `localsend-gui-egui` 只依赖 `localsend-rpc`、`localsend-platform` 和共享纯 UI 状态模型。
- 禁止复制 discovery、client、server、TLS、协议和持久化逻辑。
- daemon 生命周期仍由 tray 持有；egui 不自行 spawn daemon。
- 三个页面和所有关键弹框必须存在。

建议实现顺序：

1. worker + snapshot/event reducer。
2. 接收页与请求/文本/历史弹框。
3. 发送页、文件选择、附近设备、收藏、手动 IP、PIN。
4. 设置页全部功能。
5. 进度、取消、失败、完成反馈。
6. 响应式布局与主题。

egui 测试：

- `egui_kittest` 覆盖三页、事件 reducer、文件选择后的发送路径、设置 revision 冲突。
- 原生 App 用 AX 做启动、三页切换和至少一轮真实收发。
- 与官方 App 完成文本、单文件、多文件、目录、PIN、拒绝、取消矩阵。
- 吞吐量和文本延迟复用 Phase 2 阈值。

Phase 6 门禁：

- egui 功能矩阵与 Slint 一致。
- 不存在独立的网络/daemon 实现。
- 安装版和 tray 生命周期正常。
- Slint 仍是正式发布 GUI；egui 通过后才决定是否进入正式发布链路。

### Phase 6 低思考度执行顺序

1. **E0 视觉基线**：在 `900x600` 下截取 Slint 与官方 App 的接收、发送、设置、文本消息、文件请求、PIN、手动 IP、历史、传输中状态；保存到 `reference/egui/`。先修布局和信息层级，不碰网络层。
2. **E1 接收页**：请求逐文件勾选、全部/所选接受、拒绝、文本消息、进度/速度、取消、历史、完成/失败、通过链接。用 `egui_kittest` 驱动控件，禁止直接调用私有 handler 冒充 UI 测试。
3. **E2 发送页**：文件/目录/文本/剪贴板、先选择内容再刷新设备、附近+收藏+手动 IP、单选/多选、PIN 重试、accepted/refused/skipped、进度/取消、成功后清理规则。
4. **E3 设置页**：与 Slint 同一 `DesktopSettings` 和 revision；每个控件保存后 5 秒内反映到 daemon/TUI/官方 App 可见状态。语言、主题、颜色、动画、校验和、网络过滤不得只改本地变量。
5. **E4 生命周期**：GUI 不 spawn daemon；tray 持有 daemon；关闭窗口到托盘、tray 退出停止 daemon、GUI 断线重连、daemon 重启后 snapshot 恢复。
6. **E5 官方 App 矩阵**：复用 Slint 的脚本和 fixture，逐项完成文本/文件/目录/PIN/拒绝/取消；只替换 GUI driver，不复制协议测试。
7. **E6 视觉/响应式**：三页在 700x700、900x600、1024x768、1440x900、Light/Dark 下无重叠和截断；与 Slint 同状态截图并排审查。
8. **E7 性能与发布**：egui 文本延迟和文件吞吐必须达到 Slint/官方阈值；生成可启动 native binary，完成一次真实 App 收发与进程清理。

egui 每阶段命令：

```sh
cargo fmt -p localsend-gui-egui -- --check
cargo check -p localsend-gui-egui
cargo test -p localsend-gui-egui -- --nocapture
cargo clippy -p localsend-gui-egui --all-targets --no-deps -- -D warnings
```

Phase 6 门禁：

- 所有三个页面的所有核心操作均可通过 UI 语义或真实键盘完成。
- 与官方 App 双向矩阵、速度阈值、截图矩阵全部 PASS。
- 断线、daemon 重启、tray 退出后无残留进程或 fixture。

## Goal 执行循环

每次 continuation 只执行一个最小未完成项：

1. 从矩阵选择第一个 `FAIL` 或 `UNVERIFIED`。
2. 复现官方行为并保存证据。
3. 写失败测试。
4. 改最小实现。
5. 跑定向测试和 clippy。
6. 用官方 App 执行对应真实流程。
7. 保存截图、AX、transfer、哈希和进程清理结果。
8. 更新矩阵，提交一个可运行状态。
9. 继续下一个未完成项。

每轮结束时必须回答：

- 这一轮改变了什么行为？
- 官方 App 同场景表现是什么？
- 证据路径是什么？
- 是否还有绕过、临时 sleep、放宽断言或人工操作缺口？

## 最终 Definition of Done

- Slint：三页及全部核心弹框功能与官方一致，真实 App 双向矩阵通过。
- 文本到 `#111` 小于 1 秒，三次连续发送稳定。
- 文件吞吐不低于官方同环境 80%，进度和取消准确。
- 所有设置项都有真实效果。
- 无残留 daemon、GUI、tray、fixture、测试文件。
- release App 在 macOS 安装并完成真实收发。
- egui 使用同一 daemon/RPC，功能矩阵通过，不复制协议栈。
- 完整证据目录可审计，未通过项不能被标记为 PASS。


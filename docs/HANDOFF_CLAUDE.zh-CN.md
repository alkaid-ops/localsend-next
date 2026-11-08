# localsend-next Slint 视觉还原交接文档（Claude）

更新时间：2026-11-01  
仓库：`/Users/alkaidhe/Documents/project/localsend-project/localsend-next`  
视觉真源：OpenDesign 原型
`/Users/alkaidhe/Library/Application Support/Open Design/namespaces/release-stable/data/projects/localsend-next-three-pages-redesign/index.html`
（仓库内镜像：`apps/localsend-gui-slint/reference/open-design/`）

## 1. 当前结论

- 三页默认态已收敛到 0.79%–7.33%（1024×768 为 1.26 / 3.69 / 4.10）。
- 9 个弹窗按 `dialog-panel` 内部区域计分，收敛到 3.5%–8.0%（发送模式弹窗已从设计移除）。
- 剩余差异绝大多数是字体栅格化（Chromium 亚像素 vs Slint 灰度抗锯齿）和 1px 基线，
  **不要宣称 100% 完成**；第 7 节记录已补齐的每文件进度链路。
- 工作树有意保持 dirty，未提交、未打 tag、未推送。
- 正式 GUI 仍是 Slint + FemtoVG，打包使用 `backend-femtovg-wgpu`。

## 2. 不可破坏的约束

1. 不要修改或回退 vendored official-core 用户改动：
   `crates/localsend-official-core/examples/stress_send.rs`、
   `crates/localsend-official-core/tests/discovery.rs`。
2. 不要整体重写 `ui/`。先读当前实现，再按具体 landmark 做局部修改。
3. 不要用整页截图作为日常主循环；截图只用于最终 golden、失败局部 crop、
   无法用 DOM/accessibility 指标解释的差异。
4. GUI/tray/daemon 边界不要改：tray 管生命周期/通知，GUI 只管 UI 与 RPC，
   daemon 是状态唯一权威。
5. 不要为了让测试变绿而删除已有功能或把真实状态替换成假数据。

## 3. 工具链（本项目已验证）

### 3.1 Slint 官方 AI 插件（1.18.1，与本仓库版本一致）

- 插件：`slint@slint`，技能 `slint:slint`；参考文件在
  `~/.claude/plugins/cache/slint/slint/1.18.1/skills/slint/`。
- 项目根 `.mcp.json` 注册了两个 MCP：
  - `slint-docs`（https://docs.slint.dev/mcp）：`search` / `fetch`，索引 1.18.1。
    用于确认 `line-height-factor`、`letter-spacing`、`cross-axis-alignment` 等精确语义。
  - `slint-app`（http://localhost:9315/mcp）：运行中 App 的嵌入式 MCP。
- 嵌入式 MCP 启动方式（不改 `Cargo.toml`）：
  ```sh
  SLINT_EMIT_DEBUG_INFO=1 SLINT_MCP_PORT=9315 \
    cargo run -p localsend-gui-slint --features slint/mcp
  ```
  工具：`list_windows`、`get_element_tree`、`find_elements_by_id`、
  `take_screenshot`、`click_element`、`dispatch_key_event` 等。
  App 需要 daemon；用隔离开的 `localsendd` 或已安装实例的 endpoint，避免加入真实组播。

### 3.2 确定性渲染与对比（主判据）

```sh
# 原型 DOM/样式 + 截图（需要本地静态服务器）
cd apps/localsend-gui-slint/reference/open-design && python3 -m http.server 4173 --bind 127.0.0.1 &
node apps/localsend-gui-slint/reference/open-design/extract_metrics.mjs \
  --page send --state default --width 1024 --height 768 --theme light \
  --screenshot /tmp/od-send.png > /tmp/od-send.json

# Slint 无头渲染 + accessibility 元素框
cargo run -q -p localsend-gui-slint --example render_reference -- \
  --headless --metrics --page send --state default \
  --width 1024 --height 768 --theme light --output /tmp/slint-send.png

# 像素比较（新增 --crop，弹窗按 panel 区域计分）
cargo run -q -p localsend-gui-slint --example compare_reference -- \
  --expected /tmp/od-send.png --actual /tmp/slint-send.png \
  --tolerance 4 --tile-size 32 --top-tiles 10 --crop 192,135,640,498
```

- 提取器在截图前会 blur 当前焦点并清除 tooltip，参考图只包含静态设计状态。
- 修改 `ui/` 后先 `find apps/localsend-gui-slint/ui -type f -exec touch {} +` 再构建。

## 4. 当前视觉矩阵

产物在 `target/acceptance/slint-visual-current/{defaults,states,dialogs}/report.tsv`。

### 默认页面（Light；Dark 相差 < 0.03pp）

| 页面 | 800×600 | 1024×768 | 1440×900 | 700×700 |
| --- | ---: | ---: | ---: | ---: |
| Receive | 2.980 | 1.264 | 0.788 | 2.169 |
| Send | 6.088 | 3.593 | 2.201 | 4.550 |
| Settings | 7.312 | 4.101 | 3.031 | 5.856 |

### 状态页（1024×768 Light）

| 状态 | changed% | 状态 | changed% |
| --- | ---: | --- | ---: |
| receive request | 3.993 | send selected | 4.458 |
| receive transferring | 2.228 | send dense | 5.367 |
| receive completed | 2.052 | send multi | 6.156 |
| send transferring | 5.136 | send completed | 5.279 |

### 弹窗 panel crop（1024×768 Light）

| 弹窗 | changed% | 弹窗 | changed% |
| --- | ---: | --- | ---: |
| PIN | 3.499 | Web Share | 6.019 |
| Manual | 3.961 | Info | 6.158 |
| Text | 5.188 | Auto-save help | 6.615 |
| Favorites | 5.186 | History | 7.973 |
| Troubleshoot | 5.670 | Send mode | 8.617 |

## 5. 本轮关键修复（按影响排序）

1. **字体权重的真实语义**：CSS `<strong>` 默认 700，不是 600。
   最初把全部 600 改成 700 反而让 device name 等变粗；最终规则：
   - `<strong>` 派生标题（历史/收藏/故障排除列表行、单选卡、汇总数字、Web Share 标题）→ `700`
   - CSS 显式 `600`（device name、section title、按钮、link-button、big-button）→ `600`
   - 550 / 610 / 620 / 650 保持不变（实测与 Chromium 一致）
   这一项把 PIN 6.7→3.5、Manual 6.6→4.0、Troubleshoot 6.9→5.7。
2. **line-height 映射**：Slint 自然行高约 1.37×字号，CSS 1.55/1.6 需要
   `line-height-factor ≈ 1.17`（14px 正文/12px 标签）与 `1.13/1.06`（13/11px 卡片）。
3. DialogShell 内容左右内缩 23px（补偿 CSS 1px 边框）；副标题间距 1px；
   footer 按钮间距 8px、文本按钮用文本样式、标签基线上移 2px。
4. 弹窗逐项：历史行文案顺序与动作列 8px 内边距、收藏行 60px、故障排除改为标准
   list-row（badge 右对齐、重新扫描为链接按钮）、单选卡 65px + 原生风格圆点、
   PIN/Manual/Web Share 输入框 34px/outline 边框、Auto-save 段落行高与间距。
5. **进度条填充**：`Rectangle` 未设 `x`/`y` 时会被父级居中，send/receive 两条进度条
   都从中间开始；补 `x: 0; y: 0;`。
6. Send 工作区：文件行右侧状态 pill、去掉设计中不存在的 header 图标
   （`send-progress-advanced` 移到完成面板的“查看详情”）、取消按钮文案“取消发送”、
   进度面板间距 12/16。
7. Receive 请求页：卡片行改用 `width: 100%`（`parent.width` 不扣布局 padding，
   导致卡片右移 30px）、隐藏滚动条、复选框改为 16px 自绘（选中填充 primary + 勾）。
8. Settings 紧凑断点：分组间距 22px（容器高 <680）、header 文本 +1px。
9. 参考图卫生：提取器 blur 焦点/隐藏 tooltip，弹窗基线不再被 focus ring 污染。

## 5.1 macOS App 图标

- 设计源：`apps/localsend-gui-slint/packaging/macos/app-icon.svg`（品牌青绿渐变 + 白色光轮）。
- 打包母版：`packaging/macos/app-icon.png`（1024，已烘焙投影）。
  macOS 规范：1024 画布、824×824 内容区、圆角 185；内容区与系统 App（如 Notes）一致
  （solid bbox 100..923），投影 bbox 约 65..959。
- 不要在打包脚本里用 `qlmanage` 栅格化 SVG：QuickLook 会铺不透明白底；
  用 `sips -s format png`（保 alpha）。sips 不会渲染 SVG 滤镜，所以投影烘焙在 PNG 母版里，
  改图形后需要重新导出母版。

## 5.2 2026-11-01 原型更新落地

原型本轮更新（`localsend-next-three-pages-redesign`，2026-11-01）：

- 发送模式从设备工具条图标 + 弹窗改为页头 `单选/多选` 两段控件；`send-mode` 弹窗与 `i-tune`
  按钮已删除（DialogLayer / app.rs / render_reference 同步移除）。
- 多选模式的“已选择 N 台设备 / 发送到所选设备”条改为吸附在滚动区底部（半透明表面 + 顶部分隔线），
  设备列表从其下方滑过；按钮在未选设备时禁用（按设备数，而不是文件数）。
- 发送工作区固定一屏：头部与底部摘要/进度卡不滚动，只有中间文件列表独立滚动；发送完成面板
  宽度改为 100%，统计卡保持三列（窄屏 16/10px、内边距 9/10）。
- 选择区四个入口始终单行四等分；≤700px 时按钮 62px、图标 20px、标签 12px 单行。
- 导航图标动效（仅当前页）：接收三条弧线由内向外依次点亮（2.4s），发送纸飞机对角冲出淡出
  （1.8s），设置齿轮 8s 一圈。实现为 `NavMotionIcon`：60ms `Timer` 驱动 `tick`，用
  `Math.mod` 算相位。注意两点：`animation-tick()` 在本项目里不会驱动重绘（不要再用）；
  同一个容器里的多个 `Path` 只会画出其中一个，三条弧线要用单弧 SVG
  （`arc-near/mid/far.svg`）+ `Image` 叠加。受 `animations-enabled` 控制；`freeze-motion`
  （仅渲染器）用于静态截图。
- 设置图标换成 Lucide 齿轮；发送文件行新增 `is-image`，完成行副标题改为“已送达”。
- 新增参考页 `reference/open-design/nav-motion-options.html`（R1/S3/G1 方案来源）。
- 导航 rail 的 HorizontalLayout 之前漏了 `cross-axis-alignment: center`，图标比文字高约 14px；
  已修正（动画图标同样受用）。窄屏底部导航的 stacked VerticalLayout 同样漏了横向居中，
  图标会偏左，也已补上。多弧 `Path` 在 Slint 里只画一条，改为三条单弧 SVG 图片。
- 设置页“服务器状态”行：状态圆点（固定 7px 的 Rectangle）和文字在同一个
  HorizontalLayout 里，但布局漏写 `cross-axis-alignment`，交叉轴默认 start/拉伸——
  文字被拉满行高并垂直居中，圆点却贴在行顶部，看起来圆点浮在“在线”上方、与右侧
  图标按钮不在一条线。补上内外两层 `cross-axis-alignment: center`，并按 CSS
  `.server-actions` 的右对齐改为 `alignment: end`。
- 打包装箱脚本会在签名后 `touch` App 与 Contents，避免原地覆盖安装后 Finder/Dock 继续显示
  旧的图标缓存（icns 本身是正确的青绿渐变）。
- appimagetool 2026-10 之后的连续构建不再接受 SVG `.DirIcon`（直接报错不产出 AppImage）。
  `scripts/package-appimage.sh` 现在在 AppDir 根目录放 256px PNG
  （`packaging/linux/app-icon-256.png`），由 appimagetool 生成 `.DirIcon`；可缩放 SVG 仍保留给菜单。
- 禁用态按钮按 CSS 补上 `surface-container` 底色（含 outlined）；工作区在 ≤700px 改为
  18px 内边距占满宽度、标题基线上移 3px、完成面板 182px/统计行 55px/间距 12px，
  700×700 下 transferring 6.76%、completed 8.31%。

## 6. 逐弹窗建议（按当前分数）

1. Send mode 8.62：结构已对齐，剩余是两卡片大段 11px 文本的栅格化差异，收益有限。
2. History 7.97：行内 1px 基线（lib 文本行高），继续调收益 <0.5pp。
3. Auto-save 6.62 / Info 6.16 / Web Share 6.02：单行/段落文字抗锯齿为主。
4. Troubleshoot 5.67 / Favorites 5.19 / Text 5.19：已接近下限。
5. PIN 3.50 / Manual 3.96：可作为其他弹窗的参考实现。

不要只优化整窗 changed%；弹窗一律用 `--crop <dialog panel 框>` 计分。

## 7. 每文件进度（本轮已补齐）

- `FileView` 新增 `bytes_done`（serde 默认 0，向后兼容），`ServiceEvent::TransferProgress`
  携带 `file_id`；service 在发送/接收进度、文件完成和终态时写回每个文件的字节数，
  `SnapshotStore` 只更新事件点名的文件并做单调钳制。
- GUI `TransferFileItem` 增加 `status/detail/progress-percent/completed/in-progress`，
  发送工作区文件行按真实数据显示百分比 / “待处理” / “已完成”。
- 回归测试：`snapshot_store::transfer_progress_updates_only_the_named_file`、worker_flow
  的真实 daemon 限速传输用例。原型里“完成态仍显示 58%”是演示数据，App 按真实终态显示 100%。

## 8. 验证与打包

代码门禁（本轮全部通过）：

```sh
cargo fmt -p localsend-gui-slint -- --check
cargo test -p localsend-gui-slint --locked
cargo clippy -p localsend-gui-slint --all-targets --locked --no-deps -- -D warnings
```

打包：

```sh
cd apps/localsend-gui-slint
PACKAGE_VERSION=0.1.1 BUILD_NUMBER=18 ./package-macos.sh
```

安装与校验：

```sh
ditto target/release/bundle/macos/LocalSend.app /Applications/LocalSend.app
codesign --verify --deep --strict --verbose=2 /Applications/LocalSend.app
open -n -a /Applications/LocalSend.app
```

窗口检查：GUI 窗口 `alpha=1`、`onscreen=1`、`layer=0`。退出用：

```sh
/Applications/LocalSend.app/Contents/Helpers/LocalSendTray.app/Contents/MacOS/LocalSendTray --quit
```

本轮验收记录（2026-10-25）：

- 版本 `0.1.1`，build `18`；DMG
  `target/release/packages/LocalSend-0.1.1-macos-arm64.dmg`，ZIP 同名 `.zip`。
- `codesign --verify --deep --strict` 通过；安装到 `/Applications/LocalSend.app`。
- 启动后窗口 866×528，`alpha=1`、`onscreen=1`、`layer=0`；
  FemtoVG 实机截图与无头参考渲染一致（Receive 页）。
- 全屏截图用 `screencapture -x`（Retina 2x，按窗口 bounds ×2 裁剪）；
  `screencapture -l <window-id>` 在本机不可用。

## 9. 判断是否完成的标准

- 默认三页 × 800/1024/1440/700 × Light/Dark 矩阵稳定在各自当前水平或更好。
- Receive/Send 全部关键状态结构无差异。
- 所有弹窗 panel crop 无结构性差异（当前达标），残余差异可归因于字体栅格化。
- 真实 App 人工检查关键页面与弹窗。
- 最终重新打包、签名、安装并记录命令输出与产物路径。

## 10. 多语言（2026-11-08）

目标：对齐官方 LocalSend 的全部语言，界面、托盘、通知一致。

- 语言清单与官方 `app/assets/i18n` 相同，共 57 个 locale（镜像在
  `apps/localsend-gui-slint/translations/upstream/`），简中为 msgid 源语言。
- 译文来自三处，优先级从高到低：
  1. `locale-en.json` / `locale-en-runtime.json`（人工英文基线）；
  2. 官方对应词条（按 zh-CN 归一化匹配）；
  3. `locale-machine-cache.json`（机器翻译，已入库，可逐条人工修正）。
- Rust 侧长落在 `crates/localsend-i18n`：`translate()` 查整串，
  `t()` / `t!` 宏先翻译模板再填占位符，保证语序正确的插值消息。
- 界面数据契约不再使用译文：`direction` 取 `send`/`receive`，
  `status` 取 `waiting`/`transferring`/`finished`/`failed`/`cancelled`，
  由 `UiText.transfer-status` / `transfer-direction` 在渲染时翻译。
- 语言名（日本語、简体中文等）永远按自身文字显示，不参与机翻。
- 生成流程：`--collect` → `translate-slint-locales.mjs` → 生成器；
  生成器同时产出 59 份 `.po` 与 `lookup_generated.rs`。
- 回归测试：`tests/language.rs` 会遍历全部 locale 验证可切换且不回退中文，
  并校验每份目录没有空 msgstr；`localsend-i18n` 覆盖占位符与 locale 解析。

已知取舍：官方没有的定制文案为机器翻译初稿，措辞可在 cache 中修订后重跑生成器。

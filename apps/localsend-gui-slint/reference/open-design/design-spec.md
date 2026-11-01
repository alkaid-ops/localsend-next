# LocalSend Next 桌面原型规范

## 目标与范围

本原型重构 Receive、Send、Settings 三个主页面，并把设备详情与历史、网络信息、手动连接、收藏、PIN、发送文本、故障排除、传输完成、Web Share、快速保存说明等关键流程统一到同一套 DialogShell。默认中文，主设计尺寸为 1024 × 768，并可切换到 800 × 600、700 × 700、1440 × 900 评审视口。

原型只模拟文件选择、设备发现和传输状态，不接入真实网络、系统托盘、文件系统或持久化存储。未实现能力明确显示为 disabled，不伪装为可执行操作。

## 视觉方向

界面采用克制的 Material 3 桌面工具语言，而不是营销页或 Flutter 组件复刻。信息层级由 surface、排版、间距与对比度建立；卡片只用于请求、选择摘要、设置分组和独立传输工作区，不在 card 内继续嵌套 card。品牌青绿色只承担主操作、选中状态、连接状态和少量身份标记。

品牌字体使用 `Avenir Next / PingFang SC / Noto Sans CJK SC` 组合，中文标题保持正字距，CJK 标题行高不低于 1.3。功能图标统一为 1.8px 描边的 Material/Lucide 风格内联 SVG，不混用填充图标或 emoji。

## 设计 Token

### 色彩角色

| Token | Light | Dark | 用途 |
| --- | --- | --- | --- |
| `surface` | `#F8FBFA` | `#121916` | 主内容面 |
| `surface-container` | `#EEF3F1` | `#19221F` | 分组、输入底色、次级 surface |
| `surface-container-high` | `#E6EDEB` | `#202B27` | 悬浮、选中背景的底层 |
| `on-surface` | `#16211F` | `#E3ECE8` | 主文本 |
| `on-surface-variant` | `#50615D` | `#B3C1BC` | 辅助文本、元数据 |
| `outline` | `#BCCBC7` | `#3E4D48` | 常规边框 |
| `primary` | `#00796B` | `#61D8C8` | 主操作、品牌、焦点 |
| `on-primary` | `#FFFFFF` | `#00372F` | 主操作前景 |
| `primary-container` | `#C6EEE7` | `#164C45` | 导航选中、文件缩略图 |
| `error` | `#BA1A1A` | `#FFB4AB` | 错误和破坏性状态 |
| `success` | `#006B45` | `#69D6A3` | 完成、在线、已验证 |

Light 背景不是纯白，使用轻微青灰底色区分窗口与内容；Dark 不使用纯黑。正文与背景的配色以 4.5:1 对比度为下限，状态从不只依赖颜色，始终配合文字或图标。

### 字体与节奏

| 角色 | 字号 / 行高 | 用法 |
| --- | --- | --- |
| Receive alias | 48 / 1.12 | 中央设备身份 |
| 页面标题 | 24–26 / 1.25 | 主页面标题 |
| 区块标题 | 18–20 / 1.35 | 分组标题 |
| 正文 | 14–16 / 1.55 | 说明、设置标签 |
| 辅助 | 11–13 / 1.45 | 元数据、badge、帮助说明 |

字距以 0 为默认；小号大写标签使用 `0.06em–0.08em` 正字距。间距只使用 `4 / 8 / 12 / 16 / 24 / 32 / 40`。卡片圆角 12px，输入框 6px，按钮 10px。阴影只用于请求卡、Dialog 和 Toast。

### 动效

- 交互反馈：160ms。
- 状态与 Dialog 进入：240ms，ease-out。
- 退出时长约为进入的 65%，使用 ease-in。
- 只有 transform 和 opacity 参与界面动效；进度条宽度是数据反馈而非装饰。
- 只有当前所在页面的导航图标运行动效，其余两个保持静止；方案编号为接收 R1、发送 S3、设置 G1。接收沿用普通 WiFi 造型，三条弧线由内向外依次点亮（2.4s，不加雷达圆环）；发送纸飞机按 1.8s 沿对角直线加速冲出并淡出，呈现飞行动线而非原位抖动；设置齿轮 8s 匀速整圈旋转。设备详情页归属发送，因此该页仍由发送图标动效标记当前位置。
- `prefers-reduced-motion` 下关闭旋转、位移和过渡。

## 布局与响应式

- 应用窗口外框只负责尺寸与容器查询，内部 `app-shell` 负责 NavigationRail、主内容和底部导航的网格切换，避免容器查询修改自身导致错位。
- 1024px 及以上：248px NavigationRail，内容区独立滚动。
- 800–1023px：72px 图标 rail，保留 tooltip 与可见焦点。
- 小于 700px：72px rail 切换为底部导航；700px 本身仍使用图标 rail，以严格对应“窄于 700px”的规则。底部导航项使用等分伸缩，宽度不足 360px 时仍保持三个入口完整可见。
- 容器高度低于 680px：收紧缩略顶栏、接收状态区、Settings 分组间距和传输工作区，页面内容继续在内部滚动。
- 宽度低于 560px：Settings 标签与控件改为纵向排列；低于 480px 时设备行、请求文件行和操作按钮允许换行。
- Settings 内容最大宽度 600px，垂直起始内边距 40px，桌面端设置标签与右侧控件保持稳定双列。
- Settings 内联输入文字一律左对齐；超时使用 72px、端口使用 88px 紧凑字段，Multicast 与网段保持 110px，其余文本字段填满右侧控件，避免短数值占用过长输入框，并保证“已保存”状态不挤出控件。
- Send 页“选择”区与“附近设备”区保持正常文档流和 24px 区块间距；选择卡、设备标题与工具条不通过偏移或负外边距叠放。
- 选择区的文件/文件夹/文本/剪贴板四个入口在任意宽度下始终单行四等分；窄于 700px 时压缩为 62px 高度、20px 图标与 12px 单行标签，图标与文字均不换行、不折行成两排。
- 多选模式的“已选择 N 台设备 / 发送到所选设备”条吸附在滚动区底部，设备列表再长也无需滚到底部即可发送；吸附时使用半透明表面与顶部分隔线，设备行从其下方滑过，滚动到底后回到正常文档流。
- 评审视口使用应用容器尺寸模拟，不缩放字体，不通过 transform 伪造响应式结果。
- Send 页面使用完整高度 Flex 布局，设备列表、内容卡和空状态均在页面内部滚动，不会被窗口裁切。
- 发送工作区（传输中与发送完成）整体固定为一屏：头部与底部摘要/进度卡不滚动，只有中间的文件列表可独立滚动；文件列表吸收剩余高度，摘要卡与操作固定在底部贴近导航，不把空白留在页面下半段；发送完成的统计卡在窄屏仍保持三列，宽度与文件列表一致，避免因纵向堆叠或过窄留出侧边空白。
- Snackbar 锚定应用内容区；评审控制收起时缩为 44px 图标入口，正方形与纵向视口不再压住底部导航。

## 组件与状态

| 组件 | 关键变体 | 必须覆盖的状态 |
| --- | --- | --- |
| NavigationRail | 248px / 72px / 底部导航 | default / hover / focus / selected |
| Topbar Action | 圆形图标按钮 | default / hover / focus / tooltip / disabled |
| Primary / Outlined / Text Button | 主次与紧凑尺寸 | default / hover / focus / pressed / disabled |
| SettingsPanel | 通用、接收、网络、其他 | enabled / disabled / saved / error |
| Switch | 开 / 关 | default / hover / focus / disabled |
| Segmented Control | 主题切换 | selected / unselected / focus |
| Inline Field | alias、port、timeout、filter | idle / focus / invalid / saved |
| Request Card | 单文件与多文件 | partial selection / all selected / none selected |
| Progress Workspace | 发送与接收 | active / cancelling / complete |
| Device Row | 单选与多选 | default / hover / selected / favorite |
| DialogShell | 11 类关键弹窗 | opening / focus / validation error / close / focus return |
| Snackbar | 操作结果 | success / info / error / auto dismiss |
| Empty State | 无设备 | explanatory / refresh action |

## 交互约束

1. 三页导航通过本地状态切换，不依赖页面跳转或 hash。
2. 接收请求的 checkbox 会实时更新“接受所选”可用状态与数量。
3. 发送模式在页头以“单选 / 多选”两段切换呈现，不再占用设备工具条图标；单选模式点击设备行立即进入发送工作区，多选模式点击行切换选中状态，再由显式主按钮发送。
4. Dialog 使用原生 `dialog.showModal()`；Esc、关闭按钮和底部取消均可关闭，关闭后焦点回到原触发控件。
5. PIN、手动 IP 和文本输入只在用户提交后展示错误；错误信息解释原因与修正方式。
6. 设置中的内联输入支持 Enter 与失焦提交，保存状态短暂显示；非法端口、超时和空值不提交。
7. Web Share、打开文件夹、复制日志等系统能力在原型中以可理解的模拟反馈呈现。
8. 评审演示控制位于产品窗口之外，可折叠；它不属于 LocalSend 产品界面。

## 场景覆盖

评审面板提供 1024 × 768、700 × 700、800 × 600、1440 × 900 四种视口，Light/Dark 两套主题，以及以下状态：

- Receive：在线、离线、接收请求、传输中、完成。
- Send：未选择、已选择、空设备、多设备、多选、传输中、完成。
- Settings：主题、颜色、语言、窗口位置、启动、托盘、动画、快速保存、自动完成、PIN、接收目录、历史、Checksum、服务器启停与重启、设备身份、端口、HTTPS、网络接口、发现超时、Multicast、网络筛选、关于、日志、版本和版权。
- 子页面：设备详情作为完整页面，包含返回、设备信息、收藏、验证与日志。
- DialogShell：历史、网络信息、手动连接、收藏、PIN、发送文本、故障排除、传输完成、Web Share、快速保存说明。

状态覆盖图见 `state-coverage.svg`。

## 文件与运行

- `index.html`：原型入口。
- `styles.css`：语义 token、布局、组件状态与响应式规则。
- `app.js`：页面路由、状态机、演示数据、DialogShell 与交互反馈。
- `state-coverage.svg`：评审用状态覆盖总览图。

直接在浏览器打开 `index.html` 即可运行，无需安装依赖或启动构建服务。

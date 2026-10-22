import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const appDir = path.join(root, "apps/localsend-gui-slint");
const uiDir = path.join(appDir, "ui");
const translations = {
  "IP 地址：": "IP address: ",
  "PIN 值": "PIN value",
  "主题": "Theme",
  "使用条款": "Terms of Use",
  "保存": "Save",
  "保存到历史记录": "Save to history",
  "保存目录": "Save folder",
  "全部接受": "Accept all",
  "全部接口": "All interfaces",
  "关于 LocalSend": "About LocalSend",
  "关闭": "Close",
  "关闭 (": "Close (",
  "关闭时最小化到系统托盘": "Minimize to tray when closed",
  "其他": "Other",
  "刷新设备": "Refresh devices",
  "剪贴板": "Clipboard",
  "加密": "Encryption",
  "动画效果": "Animations",
  "单选": "Single",
  "单选模式：点击设备后立即发送。": "Single-recipient mode: send immediately after selecting a device.",
  "发送": "Send",
  "发送文件时创建校验和": "Create checksums when sending files",
  "发送文本": "Send text",
  "发送文本内容": "Message",
  "发送模式": "Send mode",
  "取消": "Cancel",
  "启用 PIN 密码": "Require PIN",
  "在文件管理器中显示": "Show in file manager",
  "在浏览器中打开链接：": "Open link in browser:",
  "复制": "Copy",
  "多播": "Multicast",
  "多选": "Multiple",
  "多选模式：选择设备后按发送按钮。": "Multiple-recipient mode: select devices, then press Send.",
  "已选择 ": "Selected ",
  "您是否要打开接收的文件？": "Do you want to open the received file?",
  "手动添加设备": "Add device manually",
  "打开": "Open",
  "打开接收目录": "Open receive folder",
  "打开文件": "Open file",
  "打开防火墙设置": "Open firewall settings",
  "拒绝": "Decline",
  "接受所选": "Accept selected",
  "接收": "Receive",
  "接收历史": "History",
  "接收文件时验证校验和": "Verify checksums when receiving files",
  "搜索设备超时": "Discovery timeout",
  "支持 LocalSend": "Support LocalSend",
  "收藏": "Favorite",
  "收藏设备": "Favorite devices",
  "收藏设备可用于自动保存规则。": "Favorite devices can be used for auto-save rules.",
  "故障排除": "Troubleshooting",
  "文件": "File",
  "文件夹": "Folder",
  "文本": "Text",
  "文本消息": "Text message",
  "服务器": "Server",
  "正在查找设备": "Looking for devices",
  "正在连接 localsendd...": "Connecting to localsendd...",
  "添加": "Add",
  "添加文件": "Add files",
  "添加文件夹": "Add folder",
  "清空历史": "Clear history",
  "清除所选项": "Clear selection",
  "版本与更新日志": "Version and changelog",
  "状态：": "Status:",
  "登录系统后自动启动程序": "Launch automatically after login",
  "目标设备要求 PIN 才会接收传输。": "The target device requires a PIN before accepting transfers.",
  "确定": "OK",
  "移除": "Remove",
  "移除所选项": "Remove selected",
  "端口": "Port",
  "端口：": "Port:",
  "编辑设置": "Edit setting",
  "网络": "Network",
  "网络信息": "Network info",
  "自动保存": "Auto save",
  "自动保存来自“收藏夹(白名单)”设备的文件": "Auto-save files from favorite (allowlist) devices",
  "自动完成传输任务": "Auto finish transfers",
  "自动接受所有文件传输请求。请注意，这会让此网络中的所有人都可以向你发送文件。": "Automatically accept all file transfer requests. Anyone on this network can send you files.",
  "自动接受请求": "Auto accept requests",
  "设备不可见：确认设备连接到同一 Wi-Fi 或局域网，并检查防火墙权限。": "Device not visible: make sure both devices are on the same Wi-Fi or LAN, and check firewall permissions.",
  "设备名称": "Device name",
  "设备名称：": "Device name:",
  "设备型号": "Device model",
  "设备类型": "Device type",
  "设备需要连接到同一网络": "Devices must be on the same network",
  "设置": "Settings",
  "设置值": "Setting value",
  "语言": "Language",
  "跟随系统": "System",
  "输入接收方 PIN": "Enter receiver PIN",
  "返回": "Back",
  "还没有接收记录": "No received items yet",
  "连接失败：检查端口、HTTPS、PIN 和目标设备的网络隔离设置。": "Connection failed: check the port, HTTPS, PIN, and target network isolation settings.",
  "退出时保存窗口位置": "Save window position on exit",
  "选择": "Select",
  "通用": "General",
  "通过链接分享：自动同意接收请求": "Share via link: automatically accept receive requests",
  "通过链接接收": "Receive via link",
  "重试": "Retry",
  "附近的设备": "Nearby devices",
  "隐私政策": "Privacy policy",
  "颜色": "Color",
  "高级设置": "Advanced settings",
  "等待中": "Waiting",
  "传输中": "Transferring",
  "已完成": "Completed",
  "失败": "Failed",
  "已取消": "Cancelled",
};

function slintFiles(directory) {
  return fs.readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const fullPath = path.join(directory, entry.name);
    return entry.isDirectory() ? slintFiles(fullPath) : entry.name.endsWith(".slint") ? [fullPath] : [];
  });
}

const msgids = new Set();
for (const file of slintFiles(uiDir)) {
  const source = fs.readFileSync(file, "utf8");
  for (const match of source.matchAll(/@tr\("([^"]*)"\)/g)) {
    msgids.add(match[1]);
  }
}

const missing = [...msgids].filter((msgid) => !(msgid in translations));
if (missing.length > 0) {
  throw new Error(`Missing Slint translations:\n${missing.map((item) => `- ${item}`).join("\n")}`);
}

function poString(value) {
  return value.replaceAll("\\", "\\\\").replaceAll('"', '\\"').replaceAll("\n", "\\n");
}

function poFile(language, lookup) {
  const entries = [...msgids].sort().map((msgid) => `msgid "${poString(msgid)}"\nmsgstr "${poString(lookup(msgid))}"\n`);
  return `msgid ""\nmsgstr ""\n"Content-Type: text/plain; charset=UTF-8\\n"\n"Language: ${language}\\n"\n\n${entries.join("\n")}`;
}

for (const [language, lookup] of [
  ["en-US", (msgid) => translations[msgid]],
  ["zh-CN", (msgid) => msgid],
]) {
  const output = path.join(appDir, "translations", language, "LC_MESSAGES", "localsend-gui-slint.po");
  fs.mkdirSync(path.dirname(output), { recursive: true });
  fs.writeFileSync(output, poFile(language, lookup));
}

console.log(`updated ${msgids.size} Slint translations`);

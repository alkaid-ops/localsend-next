#!/usr/bin/env node

import { spawn } from "node:child_process";
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { existsSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const DEFAULT_CHROME =
  "/Users/alkaidhe/Library/Caches/ms-playwright/chromium_headless_shell-1243/chrome-headless-shell-mac-arm64/chrome-headless-shell";
const DEFAULT_URL = "http://127.0.0.1:4173/";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(name);
  return index >= 0 && index + 1 < args.length ? args[index + 1] : fallback;
};

const page = option("--page", "receive");
const state = option("--state", "default");
const width = Number(option("--width", "1024"));
const height = Number(option("--height", "768"));
const theme = option("--theme", "light");
const url = option("--url", DEFAULT_URL);
const chromePath = process.env.CHROME_HEADLESS_SHELL || option("--chrome", DEFAULT_CHROME);
const screenshotPath = option("--screenshot", null);

if (!Number.isInteger(width) || !Number.isInteger(height) || width < 320 || height < 240) {
  throw new Error("--width and --height must be integers of at least 320x240");
}
if (!["receive", "send", "settings"].includes(page)) {
  throw new Error("--page must be receive, send, or settings");
}
if (!["light", "dark"].includes(theme)) {
  throw new Error("--theme must be light or dark");
}
if (!existsSync(chromePath)) {
  throw new Error(
    `Chromium headless shell not found at ${chromePath}; set CHROME_HEADLESS_SHELL`,
  );
}

async function main() {
  const profile = await mkdtemp(join(tmpdir(), "localsend-od-metrics-"));
  const chrome = spawn(
    chromePath,
    [
      "--headless=new",
      "--disable-gpu",
      "--hide-scrollbars",
      "--no-first-run",
      "--no-default-browser-check",
      "--remote-debugging-port=0",
      `--user-data-dir=${profile}`,
      "about:blank",
    ],
    { stdio: ["ignore", "ignore", "pipe"] },
  );

  let stderr = "";
  chrome.stderr.setEncoding("utf8");
  chrome.stderr.on("data", (chunk) => {
    stderr += chunk;
  });

  try {
    const activePortFile = join(profile, "DevToolsActivePort");
    const deadline = Date.now() + 10_000;
    while (!existsSync(activePortFile)) {
      if (chrome.exitCode !== null) {
        throw new Error(`Chromium exited early (${chrome.exitCode}): ${stderr}`);
      }
      if (Date.now() > deadline) {
        throw new Error(`Timed out waiting for Chromium DevTools: ${stderr}`);
      }
      await sleep(50);
    }

    const [port] = (await readFile(activePortFile, "utf8")).trim().split("\n");
    const target = await fetch(
      `http://127.0.0.1:${port}/json/new?${encodeURIComponent(url)}`,
      { method: "PUT" },
    ).then((response) => response.json());
    const session = new CdpSession(target.webSocketDebuggerUrl);
    await session.open();

    try {
      await session.send("Page.enable");
      await session.send("Runtime.enable");
      await session.send("Emulation.setDeviceMetricsOverride", {
        width: width + 48,
        height: height + 48,
        deviceScaleFactor: 1,
        mobile: false,
      });
      await waitForReady(session);
      await session.evaluate(seedExpression({ page, state, width, height, theme }));
      await sleep(400);
      // Dialog content is captured in its neutral state: focus rings and
      // tooltips are interactive artifacts, not part of the static design.
      await session.evaluate(
        `(() => { const active = document.activeElement; if (active && typeof active.blur === "function") active.blur(); else if (active) active.removeAttribute("data-tip"); })()`,
      );
      await sleep(80);
      if (screenshotPath) {
        const origin = await session.evaluate(
          `(() => { const r = document.querySelector("#appWindow").getBoundingClientRect(); return { x: r.x, y: r.y, width: r.width, height: r.height }; })()`,
        );
        const screenshot = await session.send("Page.captureScreenshot", {
          format: "png",
          fromSurface: true,
          captureBeyondViewport: false,
          clip: { ...origin, scale: 1 },
        });
        await writeFile(screenshotPath, Buffer.from(screenshot.data, "base64"));
      }
      const metrics = await session.evaluate(metricsExpression());
      process.stdout.write(`${JSON.stringify(metrics, null, 2)}\n`);
    } finally {
      session.close();
    }
  } finally {
    chrome.kill("SIGTERM");
    await Promise.race([onceExit(chrome), sleep(2_000)]);
    if (chrome.exitCode === null) chrome.kill("SIGKILL");
    await rm(profile, { recursive: true, force: true });
  }
}

function seedExpression({ page, state, width, height, theme }) {
  return `(() => {
    if (typeof applyTheme !== "function" || typeof applyViewport !== "function" || typeof setNav !== "function") {
      throw new Error("OpenDesign prototype helpers are unavailable");
    }
    applyTheme(${JSON.stringify(theme)}, false);
    applyViewport(${JSON.stringify(`${width}x${height}`)});
    setNav(${JSON.stringify(page)});
    if (${JSON.stringify(page)} === "receive" && typeof setReceiveScene === "function") {
      const receiveScenes = ["online", "offline", "request", "transferring", "completed"];
      setReceiveScene(receiveScenes.includes(${JSON.stringify(state)}) ? ${JSON.stringify(state)} : "online", false);
    }
    if (${JSON.stringify(page)} === "send") {
      state.sendScene = ${JSON.stringify(state)};
      state.selection = [];
      state.sendMode = "single";
      if (${JSON.stringify(state)} === "selected" || ${JSON.stringify(state)} === "dense" || ${JSON.stringify(state)} === "multi" || ${JSON.stringify(state)} === "transferring") {
        state.selection = state.defaultSelection.map((item) => ({ ...item }));
      }
      if (${JSON.stringify(state)} === "multi") state.sendMode = "multi";
      renderSelection();
      if (typeof setSendMode === "function") setSendMode(state.sendMode, false);
      if (${JSON.stringify(state)} === "transferring") openSendWorkspace("Flutter Reference", false);
      if (${JSON.stringify(state)} === "completed") {
        state.selection = state.defaultSelection.map((item) => ({ ...item }));
        renderSelection();
        openSendWorkspace("Flutter Reference", false);
        finishSendWorkspace();
      }
    }
    const noMotion = document.createElement("style");
    noMotion.textContent = "*,*::before,*::after{animation:none!important;transition:none!important}";
    document.head.appendChild(noMotion);
    document.querySelector(".stage").style.padding = "24px";
    document.querySelector("#reviewToggle")?.setAttribute("hidden", "");
    document.querySelector("#reviewDock")?.setAttribute("hidden", "");
    const dialogState = ${JSON.stringify(state)};
    const dialogs = {
      history: typeof historyDialog === "function" ? historyDialog : null,
      info: typeof infoDialog === "function" ? infoDialog : null,
      manual: typeof manualIpDialog === "function" ? manualIpDialog : null,
      favorites: typeof favoritesDialog === "function" ? favoritesDialog : null,
      "send-mode": typeof sendModeDialog === "function" ? sendModeDialog : null,
      pin: typeof pinDialog === "function" ? pinDialog : null,
      "text-dialog": typeof openTextDialog === "function" ? openTextDialog : null,
      troubleshoot: typeof troubleshootDialog === "function" ? troubleshootDialog : null,
      "auto-save-help": typeof autoSaveDialog === "function" ? autoSaveDialog : null,
      "web-share": typeof webShareDialog === "function" ? webShareDialog : null,
      about: typeof aboutDialog === "function" ? aboutDialog : null,
      logs: typeof logsDialog === "function" ? logsDialog : null,
      "edit-selection": typeof editSelectionDialog === "function" ? editSelectionDialog : null,
      "transfer-complete": typeof transferCompleteDialog === "function" ? transferCompleteDialog : null,
    };
    dialogs[dialogState]?.();
    return true;
  })()`;
}

function metricsExpression() {
  return `(() => {
    const origin = document.querySelector("#appWindow").getBoundingClientRect();
    const selectors = [
      ["app-window", "#appWindow"],
      ["app-shell", ".app-shell"],
      ["nav", ".nav-rail"],
      ["brand", ".brand"],
      ["brand-mark", ".brand-mark"],
      ["brand-word", ".brand-word"],
      ["nav-receive", '[data-nav="receive"]'],
      ["nav-send", '[data-nav="send"]'],
      ["nav-settings", '[data-nav="settings"]'],
      ["app-main", ".app-main"],
      ["page-visible", '.page:not([hidden])'],
      ["topbar", '.page:not([hidden]) .topbar'],
      ["page-title", '.page:not([hidden]) .topbar h1, .page:not([hidden]) .settings-header h1'],
      ["receive-layout", ".receive-layout"],
      ["receive-stage", ".receive-stage"],
      ["receive-state-visible", ".receive-state:not([hidden])"],
      ["receive-ring", ".receive-state:not([hidden]) .receive-ring"],
      ["receive-complete-mark", ".receive-state:not([hidden]) .complete-mark"],
      ["receive-complete-svg", ".receive-state:not([hidden]) .complete-mark svg"],
      ["receive-eyebrow", ".receive-state:not([hidden]) .eyebrow"],
      ["receive-alias", ".receive-state:not([hidden]) .alias-title"],
      ["receive-network", ".receive-state:not([hidden]) .network-id"],
      ["receive-status", ".receive-state:not([hidden]) .status-copy"],
      ["receive-action", ".receive-state:not([hidden]) .receive-action"],
      ["request-wrap", "#requestWrap:not([hidden])"],
      ["request-card", "#requestWrap:not([hidden]) .request-card"],
      ["transfer-panel", ".receive-state:not([hidden]) .transfer-panel"],
      ["completion-panel", ".receive-state:not([hidden]) .completion-panel"],
      ["send-browse", "#sendBrowse:not([hidden])"],
      ["send-body", "#sendBrowse:not([hidden]) .page-body"],
      ["send-picker", "#sendPicker:not([hidden])"],
      ["pick-grid", "#sendPicker:not([hidden]) .pick-grid"],
      ["big-button", "#sendPicker:not([hidden]) .big-button"],
      ["selection-card", "#sendSelection:not([hidden])"],
      ["device-section", "#sendBrowse:not([hidden]) .device-section"],
      ["section-row", "#sendBrowse:not([hidden]) .section-row"],
      ["device-list", "#sendBrowse:not([hidden]) .device-list"],
      ["device-row", "#sendBrowse:not([hidden]) .device-row"],
      ["empty-device", "#sendBrowse:not([hidden]) .empty-device"],
      ["send-workspace", "#sendWorkspace:not([hidden])"],
      ["workspace-head", "#sendWorkspace:not([hidden]) .workspace-head"],
      ["workspace-files", "#sendWorkspace:not([hidden]) .workspace-files"],
      ["workspace-file", "#sendWorkspace:not([hidden]) .workspace-file"],
      ["send-progress", "#sendProgressPanel:not([hidden])"],
      ["send-complete", "#sendCompletePanel:not([hidden])"],
      ["settings-scroll", ".settings-page .page-scroll"],
      ["settings-wrap", ".settings-wrap"],
      ["settings-header", ".settings-header"],
      ["settings-group", ".settings-group"],
      ["settings-panel", ".settings-panel"],
      ["settings-row", ".settings-row"],
      ["settings-label", ".settings-row .settings-label"],
      ["settings-control", ".settings-row .settings-control"],
      ["dialog", "#dialogShell[open]"],
      ["dialog-header", "#dialogShell[open] .dialog-header"],
      ["dialog-body", "#dialogShell[open] .dialog-body"],
      ["dialog-footer", "#dialogShell[open] .dialog-footer"],
      ["toast", "#toast.show"],
    ];
    const elements = [];
    for (const [id, selector] of selectors) {
      const node = document.querySelector(selector);
      if (!node) continue;
      const rect = node.getBoundingClientRect();
      const style = getComputedStyle(node);
      elements.push({
        id,
        selector,
        className: node.className,
        text: (node.textContent || "").trim().replace(/\\s+/g, " ").slice(0, 160),
        x: round(rect.left - origin.left),
        y: round(rect.top - origin.top),
        width: round(rect.width),
        height: round(rect.height),
        display: style.display,
        color: style.color,
        backgroundColor: style.backgroundColor,
        borderColor: style.borderColor,
        borderRadius: style.borderRadius,
        fontFamily: style.fontFamily,
        fontSize: style.fontSize,
        fontWeight: style.fontWeight,
        lineHeight: style.lineHeight,
        letterSpacing: style.letterSpacing,
        padding: style.padding,
        gap: style.gap,
      });
    }
    addRepeated("#sendBrowse:not([hidden]) .big-button", "send-big-button", elements, origin);
    addRepeated("#sendBrowse:not([hidden]) .device-row", "device-row", elements, origin);
    addRepeated("#sendWorkspace:not([hidden]) .workspace-file", "workspace-file", elements, origin);
    addRepeated("#sendWorkspace:not([hidden]) .transfer-head", "send-transfer-head", elements, origin);
    addRepeated("#sendWorkspace:not([hidden]) .transfer-actions > *", "send-transfer-action", elements, origin);
    addRepeated("#sendWorkspace:not([hidden]) .progress-track", "send-progress-track", elements, origin);
    addRepeated("#sendWorkspace:not([hidden]) .completion-summary", "send-completion-summary", elements, origin);
    addRepeated("#sendWorkspace:not([hidden]) .summary-cell", "send-completion-cell", elements, origin);
    addRepeated("#sendWorkspace:not([hidden]) .completion-panel .request-actions > *", "send-completion-action", elements, origin);
    addRepeated(".receive-state:not([hidden]) .completion-panel .completion-summary", "receive-completion-summary", elements, origin);
    addRepeated(".receive-state:not([hidden]) .completion-panel .request-actions > *", "receive-completion-action", elements, origin);
    addRepeated(".settings-group", "settings-group", elements, origin);
    addRepeated(".settings-row", "settings-row", elements, origin);
    addRepeated("#dialogShell[open] .dialog-body > *", "dialog-body-child", elements, origin);
    addRepeated("#dialogShell[open] .dialog-footer > *", "dialog-footer-child", elements, origin);
    addRepeated("#dialogShell[open] .dialog-body textarea", "dialog-textarea", elements, origin);
    addRepeated("#dialogShell[open] .dialog-body input", "dialog-input", elements, origin);
    addRepeated("#dialogShell[open] .dialog-body label", "dialog-label", elements, origin);
    addRepeated("#dialogShell[open] .dialog-body .dialog-note", "dialog-note", elements, origin);
    addRepeated("#dialogShell[open] .dialog-body .list-panel", "dialog-list-panel", elements, origin);
    addRepeated("#dialogShell[open] .dialog-body .list-row", "dialog-list-row", elements, origin);
    addRepeated("#dialogShell[open] .dialog-body .field-stack", "dialog-field-stack", elements, origin);
    addRepeated("#dialogShell[open] .dialog-body .table", "dialog-table", elements, origin);
    return {
      viewport: { width: window.innerWidth, height: window.innerHeight },
      appWindow: {
        x: round(origin.left),
        y: round(origin.top),
        width: round(origin.width),
        height: round(origin.height),
      },
      theme: document.documentElement.dataset.theme,
      elements,
    };

    function round(value) {
      return Math.round(value * 100) / 100;
    }

    function addRepeated(selector, idPrefix, output, base) {
      document.querySelectorAll(selector).forEach((node, index) => {
        if (node.closest("[hidden]")) return;
        const rect = node.getBoundingClientRect();
        const style = getComputedStyle(node);
        const label = node.querySelector(".settings-label, .device-name, .alias-title");
        const control = node.querySelector(".settings-control, .device-actions");
        const children = control
          ? [...control.children].map((child) => {
              const childRect = child.getBoundingClientRect();
              return {
                tag: child.tagName.toLowerCase(),
                text: (child.textContent || "").trim().replace(/\\s+/g, " ").slice(0, 80),
                x: round(childRect.left - base.left),
                y: round(childRect.top - base.top),
                width: round(childRect.width),
                height: round(childRect.height),
              };
            })
          : [];
        output.push({
          id: idPrefix + "-" + index,
          selector,
          className: node.className,
          text: (node.textContent || "").trim().replace(/\\s+/g, " ").slice(0, 220),
          labelText: (label?.textContent || "").trim().replace(/\\s+/g, " ").slice(0, 160),
          controlText: (control?.textContent || "").trim().replace(/\\s+/g, " ").slice(0, 160),
          x: round(rect.left - base.left),
          y: round(rect.top - base.top),
          width: round(rect.width),
          height: round(rect.height),
          display: style.display,
          color: style.color,
          backgroundColor: style.backgroundColor,
          borderColor: style.borderColor,
          borderRadius: style.borderRadius,
          fontFamily: style.fontFamily,
          fontSize: style.fontSize,
          fontWeight: style.fontWeight,
          lineHeight: style.lineHeight,
          letterSpacing: style.letterSpacing,
          padding: style.padding,
          gap: style.gap,
          children,
        });
      });
    }
  })()`;
}

async function waitForReady(session) {
  const deadline = Date.now() + 10_000;
  while (Date.now() < deadline) {
    const ready = await session.evaluate("document.readyState === 'complete'");
    if (ready) return;
    await sleep(50);
  }
  throw new Error("Timed out waiting for OpenDesign page load");
}

class CdpSession {
  constructor(url) {
    this.url = url;
    this.nextId = 1;
    this.pending = new Map();
    this.socket = null;
  }

  async open() {
    this.socket = new WebSocket(this.url);
    this.socket.addEventListener("message", (event) => {
      const message = JSON.parse(event.data);
      if (!message.id) return;
      const pending = this.pending.get(message.id);
      if (!pending) return;
      this.pending.delete(message.id);
      if (message.error) pending.reject(new Error(message.error.message));
      else pending.resolve(message.result);
    });
    await new Promise((resolve, reject) => {
      this.socket.addEventListener("open", resolve, { once: true });
      this.socket.addEventListener("error", reject, { once: true });
    });
  }

  send(method, params = {}) {
    const id = this.nextId++;
    return new Promise((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
      this.socket.send(JSON.stringify({ id, method, params }));
    });
  }

  async evaluate(expression) {
    const response = await this.send("Runtime.evaluate", {
      expression,
      awaitPromise: true,
      returnByValue: true,
      userGesture: true,
    });
    if (response.exceptionDetails) {
      throw new Error(response.exceptionDetails.exception?.description || "Evaluation failed");
    }
    return response.result.value;
  }

  close() {
    this.socket?.close();
    this.socket = null;
  }
}

function onceExit(child) {
  if (child.exitCode !== null) return Promise.resolve();
  return new Promise((resolve) => child.once("exit", resolve));
}

function sleep(milliseconds) {
  return new Promise((resolve) => setTimeout(resolve, milliseconds));
}

await main();

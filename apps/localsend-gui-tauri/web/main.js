const invoke = window.__TAURI__.core.invoke;
const listen = window.__TAURI__.event.listen;

let snapshot = null;
let selectedFingerprint = null;

function setTab(name) {
  document.querySelectorAll(".tab").forEach((node) => node.classList.toggle("active", node.id === name));
  document.querySelectorAll("nav button").forEach((node) => node.classList.toggle("active", node.dataset.tab === name));
}

document.querySelectorAll("nav button").forEach((button) => {
  button.addEventListener("click", () => setTab(button.dataset.tab));
});

async function refresh() {
  snapshot = await invoke("get_snapshot");
  if (!snapshot) return;
  document.querySelector("#daemon-state").textContent = `daemon ${snapshot.daemon_pid}`;
  document.querySelector("#server-state").textContent = `服务器：${snapshot.server_running ? `运行中（端口 ${snapshot.server_port}）` : "已停止"}${snapshot.last_error ? `，错误：${snapshot.last_error}` : ""}`;
  document.querySelector("#alias").value = snapshot.settings.alias;
  document.querySelector("#port").value = snapshot.settings.port;
  document.querySelector("#pin").value = snapshot.settings.pin ?? "";
  document.querySelector("#destination").value = snapshot.settings.destination_dir ?? "";
  document.querySelector("#auto-accept").checked = snapshot.settings.auto_accept;
  document.querySelector("#https").checked = snapshot.settings.protocol === "https";
  renderDevices();
  renderTransfers();
  renderHistory();
}

function renderDevices() {
  const list = document.querySelector("#devices");
  list.replaceChildren();
  snapshot.devices.filter((device) => device.device.ip !== "0.0.0.0").forEach((device) => {
    const item = document.createElement("li");
    const button = document.createElement("button");
    button.textContent = `${device.device.alias} · ${device.device.ip ?? "?"}`;
    button.className = device.device.fingerprint === selectedFingerprint ? "selected" : "";
    button.addEventListener("click", () => {
      selectedFingerprint = device.device.fingerprint;
      renderDevices();
    });
    item.appendChild(button);
    list.appendChild(item);
  });
}

function renderTransfers() {
  document.querySelector("#transfers").textContent = snapshot.active_transfers.length
    ? snapshot.active_transfers.map((transfer) => `${transfer.direction} ${transfer.peer_alias}: ${transfer.bytes_done}/${transfer.bytes_total}`).join("\n")
    : "当前没有活动传输";
}

function renderHistory() {
  const list = document.querySelector("#history");
  list.replaceChildren();
  snapshot.history.slice(-10).reverse().forEach((entry) => {
    const item = document.createElement("li");
    item.textContent = `${entry.direction} · ${entry.peer_alias} · ${entry.file_name}`;
    list.appendChild(item);
  });
}

async function sendCommand(command, statusTarget) {
  try {
    await invoke("command", { command });
    if (statusTarget) statusTarget.textContent = "已提交";
    await refresh();
  } catch (error) {
    if (statusTarget) statusTarget.textContent = String(error);
  }
}

document.querySelector("#refresh").addEventListener("click", () => sendCommand({ command: "refresh_discovery" }));
document.querySelector("#add-manual").addEventListener("click", () => sendCommand({
  command: "add_manual_device",
  target: document.querySelector("#manual-target").value,
}));
document.querySelector("#send-file").addEventListener("click", () => sendCommand({
  command: "send_files",
  target_fingerprint: selectedFingerprint,
  target: null,
  files: [{
    path: document.querySelector("#send-path").value,
    file_id: null,
    sha256: null,
    metadata: null,
  }],
  pin: document.querySelector("#send-pin").value || null,
}, document.querySelector("#send-status")));
document.querySelector("#send-text-button").addEventListener("click", () => sendCommand({
  command: "send_text",
  target_fingerprint: selectedFingerprint,
  target: null,
  text: document.querySelector("#send-text").value,
  pin: document.querySelector("#send-pin").value || null,
}, document.querySelector("#send-status")));
document.querySelector("#save-settings").addEventListener("click", () => {
  const settings = structuredClone(snapshot.settings);
  settings.alias = document.querySelector("#alias").value;
  settings.port = Number(document.querySelector("#port").value);
  settings.pin = document.querySelector("#pin").value || null;
  settings.destination_dir = document.querySelector("#destination").value || null;
  settings.auto_accept = document.querySelector("#auto-accept").checked;
  settings.protocol = document.querySelector("#https").checked ? "https" : "http";
  sendCommand({ command: "update_settings", expected_revision: snapshot.settings.revision, settings }, document.querySelector("#settings-status"));
});

listen("service-event", async () => {
  await refresh();
});
refresh();

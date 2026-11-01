const $ = (selector, root = document) => root.querySelector(selector);
const $$ = (selector, root = document) => [...root.querySelectorAll(selector)];

const els = {
  appWindow: $('#appWindow'),
  reviewToggle: $('#reviewToggle'),
  reviewDock: $('#reviewDock'),
  dialog: $('#dialogShell'),
  dialogTitle: $('#dialogTitle'),
  dialogSubtitle: $('#dialogSubtitle'),
  dialogBody: $('#dialogBody'),
  dialogFooter: $('#dialogFooter'),
  toast: $('#toast'),
  toastText: $('#toastText'),
};

const state = {
  nav: 'receive',
  lastPage: 'receive',
  themePreference: 'light',
  currentTheme: 'light',
  viewport: '1024x768',
  receiveScene: 'online',
  sendScene: 'default',
  sendMode: 'single',
  receiveAccepted: false,
  sendStartedAt: 0,
  lastSendTarget: 'Flutter Reference',
  lastSendSummary: null,
  sendTimer: null,
  receiveTimer: null,
  toastTimer: null,
  dialogFocus: null,
  selectedDevices: new Set(),
  selection: [],
  defaultSelection: [
    { name: '产品视觉规范.pdf', meta: '4.0 MiB · PDF', icon: 'i-file' },
    { name: 'receive-default.png', meta: '2.4 MiB · PNG', icon: 'i-image' },
    { name: 'send-selected.png', meta: '2.3 MiB · PNG', icon: 'i-image' },
  ],
  devices: [],
  favorites: new Set(['Flutter Reference', 'iPhone 15 Pro']),
};

const standardDevices = [
  { name: 'Flutter Reference', type: 'laptop', meta: 'macOS 15', protocol: 'HTTPS', model: 'MacBook Pro' },
  { name: 'iPhone 15 Pro', type: 'phone', meta: 'iOS 18.2', protocol: 'HTTPS', model: 'iPhone 15 Pro' },
  { name: 'Studio Mac', type: 'monitor', meta: 'macOS 15', protocol: 'HTTP', model: 'Mac Studio' },
  { name: 'Pixel 9', type: 'phone', meta: 'Android 16', protocol: 'HTTPS', model: 'Pixel 9' },
];

const denseDevices = [
  ...standardDevices,
  { name: 'Design Server', type: 'monitor', meta: 'Linux', protocol: 'HTTPS', model: 'Framework' },
  { name: '客厅 iPad', type: 'phone', meta: 'iPadOS 19', protocol: 'HTTPS', model: 'iPad Air' },
  { name: 'QA Windows', type: 'laptop', meta: 'Windows 11', protocol: 'HTTPS', model: 'ThinkPad' },
  { name: 'Build Agent', type: 'monitor', meta: 'Ubuntu 26.04', protocol: 'HTTP', model: 'NUC' },
];

function icon(id, size = 20) {
  return `<svg class="od-icon" style="--od-icon-size:${size}px" aria-hidden="true"><use href="#${id}"></use></svg>`;
}

function parseSize(meta) {
  const match = /([\d.]+)\s*(B|KiB|MiB|GiB)/.exec(meta || '');
  if (!match) return 0;
  const unit = { B: 1, KiB: 1024, MiB: 1024 ** 2, GiB: 1024 ** 3 }[match[2]];
  return Number.parseFloat(match[1]) * unit;
}

function formatSize(bytes) {
  if (bytes >= 1024 ** 3) return `${(bytes / 1024 ** 3).toFixed(1)} GiB`;
  if (bytes >= 1024 ** 2) return `${(bytes / 1024 ** 2).toFixed(1)} MiB`;
  if (bytes >= 1024) return `${(bytes / 1024).toFixed(1)} KiB`;
  return `${Math.round(bytes)} B`;
}

function showToast(message, tone = 'default') {
  clearTimeout(state.toastTimer);
  els.toastText.textContent = message;
  const use = $('use', els.toast);
  if (use) use.setAttribute('href', tone === 'error' ? '#i-alert' : tone === 'info' ? '#i-info' : '#i-check');
  els.toast.classList.add('show');
  state.toastTimer = setTimeout(() => els.toast.classList.remove('show'), 2600);
}

function resolveTheme(preference) {
  if (preference === 'system') {
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
  }
  return preference;
}

function applyTheme(preference, announce = false) {
  state.themePreference = preference;
  state.currentTheme = resolveTheme(preference);
  document.documentElement.dataset.theme = state.currentTheme;
  $$('[data-demo-theme]').forEach((button) => {
    button.classList.toggle('active', button.dataset.demoTheme === state.currentTheme);
  });
  $$('[data-theme-set]').forEach((button) => {
    button.classList.toggle('active', button.dataset.themeSet === preference);
  });
  if (announce) showToast(`已切换为${state.currentTheme === 'dark' ? '深色' : '浅色'}主题`);
}

function applyViewport(value) {
  const [width, height] = value.split('x').map(Number);
  state.viewport = value;
  els.appWindow.style.setProperty('--demo-w', `${width}px`);
  els.appWindow.style.setProperty('--demo-h', `${height}px`);
  $$('[data-viewport]').forEach((button) => button.classList.toggle('active', button.dataset.viewport === value));
}

function setNav(page) {
  state.nav = page;
  if (page !== 'device') state.lastPage = page;
  $$('.page').forEach((view) => {
    view.hidden = view.dataset.pageView !== page;
  });
  $('#page-device').hidden = page !== 'device';
  $$('[data-nav]').forEach((button) => {
    const active = button.dataset.nav === (page === 'device' ? 'send' : page);
    button.classList.toggle('active', active);
    if (active) button.setAttribute('aria-current', 'page');
    else button.removeAttribute('aria-current');
  });
  if (page === 'send') renderDevices();
}

function setReceiveScene(scene, announce = false) {
  state.receiveScene = scene;
  if (scene === 'request') state.receiveAccepted = false;
  if (scene === 'online' || scene === 'offline') {
    state.receiveAccepted = false;
    clearInterval(state.receiveTimer);
  }
  if (scene === 'online') {
    const copy = $('#receiveStatusCopy');
    if (copy) copy.textContent = '接收服务已就绪，附近设备可以在同一网络中发现你。';
  }
  $('#requestWrap').hidden = scene !== 'request';
  $('#receiveOnline').hidden = !['online', 'request'].includes(scene);
  $('#receiveOffline').hidden = scene !== 'offline';
  $('#receiveTransferring').hidden = scene !== 'transferring';
  $('#receiveCompleted').hidden = scene !== 'completed';
  updateRequestSelection();
  $$('[data-receive-scene]').forEach((button) => {
    button.classList.toggle('active', button.dataset.receiveScene === scene);
  });
  if (scene === 'transferring') {
    $('#receiveProgress').style.width = '64%';
    if (state.receiveAccepted) startReceiveCompletion();
  }
  if (announce) showToast(`接收状态：${sceneLabel(scene)}`);
}

function sceneLabel(scene) {
  return ({ online: '在线', offline: '离线', request: '接收请求', transferring: '传输中', completed: '已完成' })[scene] || scene;
}

function updateRequestSelection() {
  const boxes = $$('[data-request-file]');
  const checked = boxes.filter((box) => box.checked).length;
  const count = $('#selectedRequestCount');
  if (count) count.textContent = String(checked);
  const accept = $('#acceptSelected');
  if (accept) accept.disabled = checked === 0;
}

function startReceiveCompletion() {
  clearInterval(state.receiveTimer);
  const bar = $('#receiveProgress');
  let value = 64;
  state.receiveTimer = setInterval(() => {
    value = Math.min(100, value + 6);
    if (bar) bar.style.width = `${value}%`;
    if (value >= 100) {
      clearInterval(state.receiveTimer);
      setTimeout(() => setReceiveScene('completed'), 450);
    }
  }, 380);
}

function renderDevices() {
  const list = $('#deviceList');
  const empty = $('#emptyDevices');
  const emptyScene = state.sendScene === 'empty';
  const dense = state.sendScene === 'dense' || state.sendScene === 'multi';
  state.devices = emptyScene ? [] : dense ? denseDevices : standardDevices;
  empty.hidden = !emptyScene;
  list.hidden = emptyScene;
  if (emptyScene) {
    $('#multiSendFooter').hidden = true;
    return;
  }

  list.innerHTML = state.devices.map((device) => {
    const selected = state.selectedDevices.has(device.name);
    const favorite = state.favorites.has(device.name);
    return `
      <div class="device-row send-target ${state.sendMode === 'multi' ? 'multi' : ''} ${selected ? 'selected' : ''}" data-device-row="${device.name}" role="button" tabindex="0" aria-label="${state.sendMode === 'multi' ? '选择' : '发送到'} ${device.name}">
        ${state.sendMode === 'multi' ? `<input class="device-check" type="checkbox" ${selected ? 'checked' : ''} data-select-device="${device.name}" aria-label="选择 ${device.name}">` : ''}
        <div class="device-avatar">${icon(`i-${device.type}`, 21)}</div>
        <div style="min-width:0">
          <span class="device-name truncate">${device.name}</span>
          <div class="device-meta"><span class="badge ${favorite ? 'success' : ''}">${favorite ? '收藏' : device.protocol}</span><span class="badge">${device.model}</span><span class="badge">${device.meta}</span></div>
        </div>
        <div class="device-actions">
          <button class="icon-btn" type="button" data-fav-device="${device.name}" data-tip="${favorite ? '取消收藏' : '收藏'}" aria-label="${favorite ? '取消收藏' : '收藏'} ${device.name}">${icon('i-heart', 18)}</button>
          <button class="icon-btn" type="button" data-info-device="${device.name}" data-tip="设备详情" aria-label="查看 ${device.name} 的设备详情">${icon('i-info', 18)}</button>
        </div>
      </div>`;
  }).join('');

  $('#nearbyCopy').textContent = dense ? `发现 ${state.devices.length} 台设备；点击设备行即可发送。` : '同一网络下的设备会自动出现。';
  updateMultiFooter();
}

function updateMultiFooter() {
  const footer = $('#multiSendFooter');
  const visible = state.sendMode === 'multi' && state.sendScene !== 'empty';
  footer.hidden = !visible;
  const button = $('#sendSelectedDevices');
  const count = state.selectedDevices.size;
  $('#multiSelectionCopy').textContent = `已选择 ${count} 台设备`;
  button.disabled = count === 0;
}

function setSendMode(mode, announce = false) {
  state.sendMode = mode;
  state.selectedDevices.clear();
  $$('[data-send-mode]').forEach((button) => {
    const active = button.dataset.sendMode === mode;
    button.classList.toggle('active', active);
    button.setAttribute('aria-pressed', String(active));
  });
  renderDevices();
  if (announce) showToast(mode === 'multi' ? '已切换为多选发送' : '已切换为单选发送');
}

function selectionSize() {
  return state.selection.length === 3 ? '共 8.7 MiB' : state.selection.length === 1 ? state.selection[0].meta.split(' · ')[0] : `共 ${state.selection.length} 个项目`;
}

function renderSelection() {
  const hasSelection = state.selection.length > 0;
  $('#sendPicker').hidden = hasSelection;
  $('#sendSelection').hidden = !hasSelection;
  if (!hasSelection) return;

  $('#selectionTitle').textContent = `已选择 ${state.selection.length} 个项目`;
  $('#selectionSize').textContent = selectionSize();
  $('#selectionSummary').textContent = state.selection.map((item) => item.name).join('、');
  $('.thumb-rail').innerHTML = state.selection.map((item) => `
    <div class="file-thumb">
      <span class="thumb-icon">${icon(item.icon || 'i-file', 24)}</span>
      <span class="thumb-label" title="${item.name}">${item.name}</span>
    </div>`).join('');
}

function mockSelection(kind) {
  if (kind === 'text') {
    openTextDialog();
    return;
  }
  if (kind === 'clipboard') {
    state.selection = [{ name: '剪贴板文本.txt', meta: '284 B · 文本', icon: 'i-text' }];
    renderSelection();
    showToast('已从剪贴板创建文本内容');
    return;
  }
  state.selection = state.defaultSelection.map((item) => ({ ...item }));
  renderSelection();
  showToast(kind === 'folder' ? '已模拟选择文件夹中的 3 个项目' : '已模拟选择 3 个文件');
}

function openSendWorkspace(deviceName = 'Flutter Reference', autoComplete = false) {
  if (!state.selection.length) {
    showToast('请先选择要发送的内容', 'info');
    return;
  }
  setNav('send');
  state.sendStartedAt = Date.now();
  state.lastSendTarget = deviceName;
  $('#sendBrowse').hidden = true;
  $('#sendWorkspace').hidden = false;
  $('#sendWorkspaceTitle').textContent = '正在发送文件';
  $('#sendWorkspaceCopy').textContent = `发送到 ${deviceName}`;
  $('#sendProgressPanel').hidden = false;
  $('#sendCompletePanel').hidden = true;
  const totalBytes = state.selection.reduce((sum, item) => sum + parseSize(item.meta), 0);
  const doneCount = Math.max(0, Math.min(state.selection.length - 1, Math.floor(state.selection.length * 0.58)));
  $('#sendProgress').style.width = '58%';
  $('#sendProgressStat').textContent = `已完成 ${doneCount} / ${state.selection.length} 个文件 · ${formatSize(totalBytes * 0.58)} / ${formatSize(totalBytes)}`;
  $('#sendWorkspaceFiles').innerHTML = state.selection.map((item, index) => {
    const done = index < doneCount;
    const active = index === doneCount;
    return `
    <div class="workspace-file">
      <span class="thumb-icon">${icon(item.icon || 'i-file', 22)}</span>
      <div><strong class="truncate">${item.name}</strong><span>${done ? '已完成' : active ? '正在传输' : '等待中'} · ${item.meta.split(' · ')[0]}</span></div>
      <span class="badge ${done || active ? 'success' : ''}">${done ? '100%' : active ? '58%' : '待处理'}</span>
    </div>`;
  }).join('');
  clearInterval(state.sendTimer);
  if (autoComplete) {
    state.sendTimer = setTimeout(() => finishSendWorkspace(), 3800);
  }
}

function finishSendWorkspace() {
  clearInterval(state.sendTimer);
  const items = state.selection;
  const totalBytes = items.reduce((sum, item) => sum + parseSize(item.meta), 0);
  const seconds = Math.max(0.1, (Date.now() - (state.sendStartedAt || Date.now())) / 1000);
  const summary = { count: items.length, size: formatSize(totalBytes), time: `${seconds.toFixed(1)} s` };
  state.lastSendSummary = summary;

  $('#sendWorkspaceFiles').innerHTML = items.map((item) => `
    <div class="workspace-file completed">
      <span class="thumb-icon">${icon(item.icon || 'i-file', 22)}</span>
      <div><strong class="truncate">${item.name}</strong><span>已送达 · ${item.meta.split(' · ')[0]}</span></div>
      <span class="badge success">已完成</span>
    </div>`).join('');

  $('#sendCompleteCount').textContent = summary.count;
  $('#sendCompleteSize').textContent = summary.size;
  $('#sendCompleteTime').textContent = summary.time;
  $('#sendCompleteTarget').textContent = `文件已发送到「${state.lastSendTarget}」。`;

  $('#sendProgressPanel').hidden = true;
  $('#sendCompletePanel').hidden = false;
  $('#sendWorkspaceTitle').textContent = '发送完成';
  $('#sendWorkspaceCopy').textContent = '文件已成功送达';
  showToast('发送完成');
}

function closeSendWorkspace() {
  clearInterval(state.sendTimer);
  $('#sendWorkspace').hidden = true;
  $('#sendBrowse').hidden = false;
  setNav('send');
}

function openDeviceDetails(name) {
  const device = state.devices.find((item) => item.name === name) || standardDevices[0];
  $('#deviceDetailTitle').textContent = '设备详情';
  $('#detailDeviceIcon').innerHTML = icon(`i-${device.type}`, 30);
  $('.details-hero h1').textContent = device.name;
  $('.details-hero p').textContent = `${device.protocol} · ${device.meta} · ${device.model}`;
  const values = $$('.detail-row span:last-child');
  if (values[0]) values[0].textContent = `LocalSend v2.1 · ${device.protocol}`;
  if (values[1]) values[1].textContent = `${device.model} · 14-inch`;
  if (values[2]) values[2].textContent = device.name === 'Pixel 9' ? '192.168.1.57:53317' : '192.168.1.42:53317';
  const heart = $('#detailFavorite');
  heart.dataset.deviceName = device.name;
  heart.classList.toggle('active', state.favorites.has(device.name));
  setNav('device');
  $('#verifyStatus').textContent = '证书有效，连接可信';
  $('#verifyStatus').style.color = 'var(--success)';
}

function closeDialog() {
  if (els.dialog.open) els.dialog.close();
}

function openDialog({ title, subtitle = '', body = '', footer = '', width = '560px', onOpen }) {
  state.dialogFocus = document.activeElement;
  els.dialog.style.setProperty('--dialog-width', width);
  els.dialogTitle.textContent = title;
  els.dialogSubtitle.textContent = subtitle;
  els.dialogSubtitle.hidden = !subtitle;
  els.dialogBody.innerHTML = body;
  els.dialogFooter.innerHTML = footer;
  els.dialog.showModal();
  if (typeof onOpen === 'function') onOpen();
  requestAnimationFrame(() => {
    const preferred = $('#dialogBody input:not([type="hidden"]), #dialogBody textarea, #dialogBody select, #dialogBody button, #dialogFooter .button');
    if (preferred) preferred.focus();
  });
}

function historyDialog() {
  openDialog({
    title: '历史记录', subtitle: '搜索消息、文件名或设备', width: '640px',
    body: `
      <div class="dialog-content-grid">
        <label class="search-field">${icon('i-search', 18)}<input id="historySearch" type="search" placeholder="搜索历史" aria-label="搜索历史记录"></label>
        <div class="list-panel" id="historyList">
          <div class="list-row" data-history-row="iphone 产品评审 截图"><div class="list-row-main"><strong>来自 iPhone 15 Pro · 3 个文件</strong><span>今天 10:24 · Downloads/LocalSend/产品评审说明.pdf</span><span>今天 10:24 · 界面截图-2026-10-24.png、组件清单.md</span></div><button class="link-button" type="button" data-open-file>打开文件</button></div>
          <div class="list-row" data-history-row="flutter reference 视觉规范"><div class="list-row-main"><strong>发送到 Flutter Reference · 产品视觉规范.pdf</strong><span>昨天 18:42 · 4.0 MiB · 已完成</span></div><button class="link-button" type="button" data-open-file>打开文件</button></div>
          <div class="list-row" data-history-row="文本 会议 摘要"><div class="list-row-main"><strong>来自 Pixel 9 · 文本消息</strong><span>本次评审需覆盖 14 个关键状态，并确认 Light/Dark、窄窗口和传输异常恢复。</span><span>10 月 22 日 15:08 · 全文已展示</span></div><button class="link-button" type="button" data-copy-history>复制</button></div>
        </div>
        <div class="dialog-note">历史记录只保存在当前设备，不会同步到云服务。</div>
      </div>`,
    footer: `<button class="button text danger-text footer-left" type="button" id="clearHistory">清空历史</button><button class="button text" type="button" data-dialog-close>关闭</button>`,
    onOpen() {
      $('#historySearch').addEventListener('input', (event) => {
        const query = event.target.value.trim().toLowerCase();
        $$('[data-history-row]').forEach((row) => {
          row.hidden = !row.dataset.historyRow.includes(query);
        });
      });
      $('#clearHistory').addEventListener('click', () => {
        $('#historyList').innerHTML = '<div class="empty-device" style="padding:24px"><span class="empty-icon">' + icon('i-history', 28) + '</span><strong>暂无历史记录</strong><p>完成一次接收或发送后，记录会显示在这里。</p></div>';
        showToast('历史记录已清空');
      });
      $$('[data-open-file]').forEach((button) => button.addEventListener('click', () => showToast('已模拟在文件管理器中打开')));
      $$('[data-copy-history]').forEach((button) => button.addEventListener('click', () => showToast('消息全文已复制')));
    },
  });
}

function infoDialog() {
  openDialog({
    title: '网络信息', subtitle: '当前接收服务的连接信息', width: '520px',
    body: `<table class="table"><tbody><tr><th>Alias</th><td>Flutter Reference</td></tr><tr><th>IP 地址</th><td class="mono">192.168.1.42</td></tr><tr><th>端口</th><td class="mono">53317</td></tr><tr><th>协议</th><td>HTTPS · LocalSend v2.1</td></tr><tr><th>网络</th><td>Wi‑Fi · en0</td></tr><tr><th>指纹</th><td class="mono">E4:7A:21:9C:88:BD</td></tr></tbody></table>`,
    footer: '<button class="button text" type="button" data-dialog-close>关闭</button>',
  });
}

function manualIpDialog() {
  openDialog({
    title: '手动连接', subtitle: '输入同一网络中设备的 IP 地址', width: '480px',
    body: `<div class="field-stack"><label for="manualIp">IP 地址或主机名</label><input class="input mono" id="manualIp" value="192.168.1.42" autocomplete="off"><span class="field-error" id="manualIpError">请输入有效的 IPv4 地址，例如 192.168.1.42。</span></div>`,
    footer: '<button class="button text" type="button" data-dialog-close>取消</button><button class="button primary" type="button" id="manualConnect">连接</button>',
    onOpen() {
      $('#manualConnect').addEventListener('click', () => {
        const value = $('#manualIp').value.trim();
        const valid = /^(\d{1,3}\.){3}\d{1,3}$/.test(value) && value.split('.').every((part) => Number(part) <= 255);
        $('#manualIpError').classList.toggle('show', !valid);
        $('#manualIp').setAttribute('aria-invalid', String(!valid));
        if (valid) {
          closeDialog();
          showToast(`正在连接 ${value}`);
        }
      });
      $('#manualIp').addEventListener('keydown', (event) => {
        if (event.key === 'Enter') $('#manualConnect').click();
      });
    },
  });
}

function favoritesDialog() {
  const items = [...state.favorites].map((name) => `<div class="list-row"><div class="list-row-main"><strong>${name}</strong><span>已验证 · 快速保存已开启</span></div><button class="icon-btn" type="button" data-remove-favorite="${name}" data-tip="移除收藏" aria-label="移除 ${name}">${icon('i-trash', 17)}</button></div>`).join('');
  openDialog({
    title: '收藏设备', subtitle: '收藏设备可使用快速保存与快捷发送', width: '520px',
    body: `<div class="list-panel" id="favoriteList">${items}</div>`,
    footer: '<button class="button text" type="button" data-dialog-close>关闭</button>',
    onOpen() {
      $$('[data-remove-favorite]').forEach((button) => button.addEventListener('click', () => {
        state.favorites.delete(button.dataset.removeFavorite);
        button.closest('.list-row').remove();
        renderDevices();
        showToast('已从收藏中移除');
      }));
    },
  });
}

function pinDialog() {
  let confirmed = false;
  const restoreOnClose = () => {
    if (!confirmed) $('#pinSetting').checked = false;
  };
  els.dialog.addEventListener('close', restoreOnClose, { once: true });
  openDialog({
    title: '设置 PIN', subtitle: '接收文件前，发送方需要输入 4 位 PIN', width: '460px',
    body: `<div class="field-stack"><label for="pinInput">PIN 密码</label><input class="input mono" id="pinInput" type="password" inputmode="numeric" maxlength="4" autocomplete="one-time-code" aria-describedby="pinError"><span class="field-error" id="pinError">PIN 不正确，请重新输入。演示 PIN 为 4268。</span></div>`,
    footer: '<button class="button text" type="button" data-dialog-close>取消</button><button class="button primary" type="button" id="confirmPin">确认</button>',
    onOpen() {
      $('#confirmPin').addEventListener('click', () => {
        const value = $('#pinInput').value.trim();
        const invalid = !/^\d{4}$/.test(value) || value !== '4268';
        $('#pinError').classList.toggle('show', invalid);
        $('#pinInput').setAttribute('aria-invalid', String(invalid));
        if (!invalid) {
          confirmed = true;
          closeDialog();
          $('#pinSetting').checked = true;
          showToast('PIN 验证已开启');
        }
      });
      $('#pinInput').addEventListener('keydown', (event) => {
        if (event.key === 'Enter') $('#confirmPin').click();
      });
    },
  });
}

function openTextDialog() {
  const sample = '这是通过 LocalSend Next 发送的示例文本。它适用于快速分享地址、会议摘要或短链接。';
  openDialog({
    title: '发送文本', subtitle: '文本会作为 .txt 文件发送到所选设备', width: '600px',
    body: `<div class="dialog-content-grid"><div class="field-stack"><label for="sendTextInput">文本内容</label><textarea class="textarea" id="sendTextInput" maxlength="10000">${sample}</textarea><span class="char-count" id="textCharCount">${sample.length} / 10000</span></div><span class="field-error" id="textError">请输入要发送的文本。</span></div>`,
    footer: '<button class="button text" type="button" data-dialog-close>取消</button><button class="button primary" type="button" id="confirmSendText">确定</button>',
    onOpen() {
      const input = $('#sendTextInput');
      input.addEventListener('input', () => {
        $('#textCharCount').textContent = `${input.value.length} / 10000`;
      });
      $('#confirmSendText').addEventListener('click', () => {
        const value = input.value.trim();
        const invalid = value.length === 0;
        $('#textError').classList.toggle('show', invalid);
        input.setAttribute('aria-invalid', String(invalid));
        if (!invalid) {
          state.selection = [{ name: '剪贴板文本.txt', meta: `${new Blob([value]).size} B · 文本`, icon: 'i-text' }];
          renderSelection();
          closeDialog();
          showToast('文本已准备好发送');
        }
      });
    },
  });
}

function troubleshootDialog() {
  openDialog({
    title: '故障排除', subtitle: '按顺序检查设备发现和连接', width: '600px',
    body: `<div class="list-panel"><div class="list-row"><div class="list-row-main"><strong>网络连接</strong><span>Wi‑Fi · en0 · 192.168.1.42</span></div><span class="badge success">正常</span></div><div class="list-row"><div class="list-row-main"><strong>本地端口</strong><span>TCP 53317 正在监听</span></div><span class="badge success">正常</span></div><div class="list-row"><div class="list-row-main"><strong>Multicast 发现</strong><span>最近一次响应：刚刚</span></div><span class="badge success">正常</span></div><div class="list-row"><div class="list-row-main"><strong>另一台设备未出现？</strong><span>确认双方处于同一子网，并允许防火墙访问 LocalSend。</span></div><button class="link-button" type="button" data-dialog-close>重新扫描</button></div></div>`,
    footer: '<button class="button text" type="button" data-dialog-close>关闭</button><button class="button primary" type="button" id="copyDiagnostics">复制诊断信息</button>',
    onOpen() { $('#copyDiagnostics').addEventListener('click', () => showToast('诊断信息已复制')); },
  });
}

function transferCompleteDialog() {
  const summary = state.lastSendSummary || { count: 0, size: '0 B', time: '0.0 s' };
  openDialog({
    title: '传输完成', subtitle: '所有文件均已通过 Checksum 校验', width: '520px',
    body: `<div class="completion-summary"><div class="summary-cell"><strong>${summary.count}</strong><span>已发送</span></div><div class="summary-cell"><strong>${summary.size}</strong><span>总大小</span></div><div class="summary-cell"><strong>${summary.time}</strong><span>传输用时</span></div></div><p class="dialog-note" style="margin-top:16px">目标设备：${state.lastSendTarget} · HTTPS · 192.168.1.42</p>`,
    footer: '<button class="button text" type="button" data-dialog-close>关闭</button><button class="button primary" type="button" id="dialogOpenFolder">打开目标文件夹</button>',
    onOpen() { $('#dialogOpenFolder').addEventListener('click', () => showToast('已模拟打开目标文件夹')); },
  });
}

function webShareDialog() {
  openDialog({
    title: '通过链接接收', subtitle: '浏览器可在同一网络内访问此地址', width: '560px',
    body: `<div class="dialog-content-grid"><div class="field-stack"><label for="receiveLink">接收链接</label><input class="input mono" id="receiveLink" value="https://localsend.local/receive?pin=4821" readonly></div><div class="list-row"><div class="list-row-main"><strong>Web Share 已验证</strong><span>访问者需要通过 PIN 验证后才能上传文件。</span></div><span class="badge success">已就绪</span></div></div>`,
    footer: '<button class="button text" type="button" data-dialog-close>关闭</button><button class="button primary" type="button" id="copyReceiveLink">复制链接</button>',
    onOpen() {
      $('#copyReceiveLink').addEventListener('click', async () => {
        const value = $('#receiveLink').value;
        try { await navigator.clipboard.writeText(value); } catch { $('#receiveLink').select(); }
        showToast('接收链接已复制');
      });
    },
  });
}

function autoSaveDialog() {
  openDialog({
    title: '快速保存说明', subtitle: '可信设备可以跳过每次接收确认', width: '540px',
    body: `<p style="margin-top:0">「收藏设备」只会自动接受来自已验证收藏设备的请求。文件保存到当前接收目录，并继续显示在历史记录中。</p><p class="dialog-note">若设备指纹发生变化，LocalSend 会重新要求确认。你可以随时在收藏设备中取消信任。</p>`,
    footer: '<button class="button text" type="button" data-dialog-close>关闭</button><button class="button primary" type="button" id="openFavoritesFromAutosave">管理收藏设备</button>',
    onOpen() { $('#openFavoritesFromAutosave').addEventListener('click', () => { closeDialog(); setTimeout(favoritesDialog, 80); }); },
  });
}

function aboutDialog() {
  openDialog({
    title: '关于 LocalSend Next', width: '500px',
    body: `<div class="dialog-content-grid"><p style="margin:0">LocalSend 是一个开源、跨平台的本地文件传输工具。本原型用于桌面端三页重构评审，并作为 Slint 视觉规范参考。</p><table class="table"><tbody><tr><th>版本</th><td>0.9.0 · 原型评审版</td></tr><tr><th>许可证</th><td>MIT</td></tr><tr><th>默认语言</th><td>简体中文</td></tr><tr><th>设计基准</th><td>1024 × 768</td></tr></tbody></table></div>`,
    footer: '<button class="button primary" type="button" data-dialog-close>关闭</button>',
  });
}

function logsDialog() {
  openDialog({
    title: '会话日志', subtitle: '最近 120 行 · 已隐藏敏感字段', width: '680px',
    body: '<div class="log-panel">10:26:18  server      listening on 0.0.0.0:53317 (HTTPS)\n10:26:18  discovery   multicast enabled · 224.0.0.167:53317\n10:26:19  peer        found "Flutter Reference" at 192.168.1.42\n10:26:19  handshake   certificate valid · protocol v2.1\n10:26:20  session     ready\n10:26:22  request     incoming files=3 bytes=7340032\n10:26:23  receive     current=界面截图-2026-10-24.png\n10:26:24  checksum    sha256 pending</div>',
    footer: '<button class="button text" type="button" id="copyLogs">复制日志</button><button class="button primary" type="button" data-dialog-close>关闭</button>',
    onOpen() { $('#copyLogs').addEventListener('click', () => showToast('日志已复制')); },
  });
}

function editSelectionDialog() {
  openDialog({
    title: '编辑发送内容', subtitle: '移除不需要的项目后继续', width: '580px',
    body: `<div class="list-panel" id="editSelectionList">${state.selection.map((item) => `<div class="list-row" data-selection-item="${item.name}"><div class="list-row-main"><strong>${item.name}</strong><span>${item.meta}</span></div><button class="icon-btn" type="button" data-remove-selection="${item.name}" data-tip="移除" aria-label="移除 ${item.name}">${icon('i-trash', 17)}</button></div>`).join('')}</div>`,
    footer: '<button class="button text" type="button" data-dialog-close>完成</button>',
    onOpen() {
      $$('[data-remove-selection]').forEach((button) => button.addEventListener('click', () => {
        state.selection = state.selection.filter((item) => item.name !== button.dataset.removeSelection);
        button.closest('.list-row').remove();
        renderSelection();
        showToast('已移除所选项目');
      }));
    },
  });
}

function bindCommonEvents() {
  $$('[data-nav]').forEach((button) => button.addEventListener('click', () => {
    if (button.dataset.nav === 'send') {
      $('#sendBrowse').hidden = false;
      $('#sendWorkspace').hidden = true;
    }
    setNav(button.dataset.nav);
  }));

  $('#historyBtn').addEventListener('click', historyDialog);
  $('#networkInfoBtn').addEventListener('click', infoDialog);
  $('#receiveLinkBtn').addEventListener('click', webShareDialog);
  $('#retryReceive').addEventListener('click', () => { setReceiveScene('online'); showToast('接收服务已重新启动'); });
  $('#declineRequest').addEventListener('click', () => { clearInterval(state.receiveTimer); setReceiveScene('online'); showToast('已拒绝本次接收请求'); });
  $('#acceptAll').addEventListener('click', () => {
    $$('[data-request-file]').forEach((box) => { box.checked = true; });
    updateRequestSelection();
    state.receiveAccepted = true;
    setReceiveScene('transferring');
    showToast('已开始接收文件');
  });
  $('#acceptSelected').addEventListener('click', () => {
    state.receiveAccepted = true;
    setReceiveScene('transferring');
    showToast(`正在接收 ${$$('[data-request-file]').filter((box) => box.checked).length} 个文件`);
  });
  $$('[data-request-file]').forEach((box) => box.addEventListener('change', updateRequestSelection));
  $('#cancelReceive').addEventListener('click', () => { clearInterval(state.receiveTimer); setReceiveScene('online'); showToast('接收已取消'); });
  $('#openReceiveFolder').addEventListener('click', () => showToast('已模拟打开接收文件夹'));
  $$('[data-close-completion]').forEach((button) => button.addEventListener('click', () => setReceiveScene('online')));

  $$('[data-pick]').forEach((button) => button.addEventListener('click', () => mockSelection(button.dataset.pick)));
  $('#filePicker').addEventListener('change', (event) => {
    const files = [...event.target.files];
    if (!files.length) return;
    state.selection = files.map((file) => ({ name: file.name, meta: `${Math.max(1, Math.round(file.size / 1024))} KiB · ${file.type || '文件'}`, icon: file.type.startsWith('image/') ? 'i-image' : 'i-file' }));
    renderSelection();
    showToast(`已选择 ${files.length} 个文件`);
  });
  $('#clearSelection').addEventListener('click', () => { state.selection = []; renderSelection(); showToast('已清除选择'); });
  $$('[data-send-edit]').forEach((button) => button.addEventListener('click', editSelectionDialog));
  $('#refreshDevices').addEventListener('click', () => { showToast('正在扫描附近设备', 'info'); setTimeout(() => showToast('扫描完成'), 700); });
  $('#emptyRefresh').addEventListener('click', () => { showToast('正在重新扫描', 'info'); setTimeout(() => showToast('仍未发现设备，请检查网络'), 700); });
  $('#manualIpBtn').addEventListener('click', manualIpDialog);
  $('#favoritesBtn').addEventListener('click', favoritesDialog);
  $$('[data-send-mode]').forEach((button) => button.addEventListener('click', () => setSendMode(button.dataset.sendMode, true)));
  $('#sendSelectedDevices').addEventListener('click', () => {
    if (state.selectedDevices.size) openSendWorkspace([...state.selectedDevices].join('、'), true);
  });

  $('#deviceList').addEventListener('click', (event) => {
    const info = event.target.closest('[data-info-device]');
    if (info) { event.stopPropagation(); openDeviceDetails(info.dataset.infoDevice); return; }
    const fav = event.target.closest('[data-fav-device]');
    if (fav) {
      event.stopPropagation();
      const name = fav.dataset.favDevice;
      if (state.favorites.has(name)) state.favorites.delete(name); else state.favorites.add(name);
      renderDevices();
      showToast(state.favorites.has(name) ? '已收藏设备' : '已取消收藏');
      return;
    }
    const row = event.target.closest('[data-device-row]');
    if (!row) return;
    const name = row.dataset.deviceRow;
    if (state.sendMode === 'multi') {
      if (state.selectedDevices.has(name)) state.selectedDevices.delete(name); else state.selectedDevices.add(name);
      renderDevices();
    } else {
      openSendWorkspace(name, true);
    }
  });
  $('#deviceList').addEventListener('keydown', (event) => {
    if (event.target.closest('button, input')) return;
    const row = event.target.closest('[data-device-row]');
    if (row && (event.key === 'Enter' || event.key === ' ')) {
      event.preventDefault();
      const name = row.dataset.deviceRow;
      if (state.sendMode === 'multi') {
        if (state.selectedDevices.has(name)) state.selectedDevices.delete(name); else state.selectedDevices.add(name);
        renderDevices();
      } else openSendWorkspace(name, true);
    }
  });
  $('#sendWorkspaceBack').addEventListener('click', closeSendWorkspace);
  $('#cancelSend').addEventListener('click', () => { clearInterval(state.sendTimer); closeSendWorkspace(); showToast('发送已取消'); });
  $('#completeSendNow').addEventListener('click', finishSendWorkspace);
  $('#sendAgain').addEventListener('click', () => openSendWorkspace(state.lastSendTarget, true));
  $('#sendDetailDialog').addEventListener('click', transferCompleteDialog);
  $('#closeSendComplete').addEventListener('click', closeSendWorkspace);

  $('#deviceBackButton').addEventListener('click', () => setNav('send'));
  $('#detailFavorite').addEventListener('click', () => {
    const name = $('#detailFavorite').dataset.deviceName || 'Flutter Reference';
    if (state.favorites.has(name)) state.favorites.delete(name); else state.favorites.add(name);
    $('#detailFavorite').classList.toggle('active', state.favorites.has(name));
    showToast(state.favorites.has(name) ? '已收藏设备' : '已取消收藏');
  });
  $('#verifyDevice').addEventListener('click', () => { $('#verifyStatus').textContent = '正在验证证书…'; $('#verifyStatus').style.color = 'var(--on-surface-variant)'; setTimeout(() => { $('#verifyStatus').textContent = '证书有效，连接可信'; $('#verifyStatus').style.color = 'var(--success)'; showToast('设备验证通过'); }, 650); });
  $('#clearDeviceLog').addEventListener('click', () => { $('.log-panel').textContent = '暂无新的会话日志'; showToast('设备日志已清空'); });
  $('#sendToDetailDevice').addEventListener('click', () => { closeDialog(); setNav('send'); if (state.selection.length) openSendWorkspace($('#detailFavorite').dataset.deviceName || 'Flutter Reference', true); else { mockSelection('file'); setTimeout(() => openSendWorkspace($('#detailFavorite').dataset.deviceName || 'Flutter Reference', true), 120); } });

  $$('[data-theme-set]').forEach((button) => button.addEventListener('click', () => applyTheme(button.dataset.themeSet, true)));
  $$('[data-inline-setting]').forEach((input) => {
    const initial = input.value;
    const save = () => {
      const kind = input.dataset.inlineSetting;
      const value = input.value.trim();
      let valid = value.length > 0;
      let message = '值不能为空';
      if (kind === 'port') { valid = /^\d+$/.test(value) && Number(value) >= 1024 && Number(value) <= 65535; message = '端口需在 1024–65535 之间'; }
      if (kind === 'timeout') { valid = /^\d+$/.test(value) && Number(value) >= 0; message = '超时时间需为非负整数'; }
      if (kind === 'multicast') { valid = /^(\d{1,3}\.){3}\d{1,3}$/.test(value) && value.split('.').every((part) => Number(part) <= 255); message = 'Multicast 地址需为有效的 IPv4 地址'; }
      input.setAttribute('aria-invalid', String(!valid));
      if (!valid) { input.value = initial; showToast(message, 'error'); return; }
      const status = input.parentElement.querySelector('.save-status');
      if (status) { status.classList.add('show'); setTimeout(() => status.classList.remove('show'), 1400); }
      showToast('设置已保存');
    };
    input.addEventListener('keydown', (event) => { if (event.key === 'Enter') { event.preventDefault(); save(); input.blur(); } });
    input.addEventListener('blur', save);
  });
  $$('.settings-panel input[type="checkbox"]:not(#pinSetting)').forEach((checkbox) => checkbox.addEventListener('change', () => showToast(checkbox.checked ? '设置已开启' : '设置已关闭')));
  $('#pinSetting').addEventListener('change', (event) => {
    if (event.target.checked) pinDialog();
  });
  $('#receiveFolderBtn').addEventListener('click', () => showToast('已模拟打开接收目录'));
  $('#autosaveInfoBtn').addEventListener('click', autoSaveDialog);
  $('#settingsHistoryBtn').addEventListener('click', historyDialog);
  $('#troubleshootBtn').addEventListener('click', troubleshootDialog);
  $('#logsBtn').addEventListener('click', logsDialog);
  $('#aboutBtn').addEventListener('click', aboutDialog);
  $('#restartServer').addEventListener('click', () => {
    $('#serverSettingStatus').textContent = '正在重启接收服务…';
    const status = $('.server-status');
    status.textContent = '重启中';
    status.style.color = 'var(--warning)';
    setTimeout(() => {
      $('#serverSettingStatus').textContent = '接收服务正在运行';
      status.textContent = '在线'; status.style.color = 'var(--success)';
      showToast('接收服务已重启');
    }, 900);
  });
  $('#toggleServer').addEventListener('click', () => {
    const stopped = $('#toggleServer').textContent === '停止';
    $('#toggleServer').textContent = stopped ? '启动' : '停止';
    $('#serverSettingStatus').textContent = stopped ? '接收服务已停止' : '接收服务正在运行';
    const status = $('.server-status'); status.textContent = stopped ? '离线' : '在线'; status.style.color = stopped ? 'var(--on-surface-variant)' : 'var(--success)';
    showToast(stopped ? '接收服务已停止' : '接收服务已启动');
  });

  $('#reviewToggle').addEventListener('click', () => {
    const opening = els.reviewDock.hidden;
    els.reviewDock.hidden = !opening;
    els.reviewToggle.setAttribute('aria-expanded', String(opening));
  });
  $('#closeReview').addEventListener('click', () => { els.reviewDock.hidden = true; els.reviewToggle.setAttribute('aria-expanded', 'false'); });
  $$('[data-viewport]').forEach((button) => button.addEventListener('click', () => applyViewport(button.dataset.viewport)));
  $$('[data-demo-theme]').forEach((button) => button.addEventListener('click', () => applyTheme(button.dataset.demoTheme)));
  $$('[data-receive-scene]').forEach((button) => button.addEventListener('click', () => { setNav('receive'); state.receiveAccepted = false; setReceiveScene(button.dataset.receiveScene, true); }));
  $$('[data-send-scene]').forEach((button) => button.addEventListener('click', () => {
    const scene = button.dataset.sendScene;
    state.sendScene = scene;
    setNav('send');
    $('#sendBrowse').hidden = false;
    $('#sendWorkspace').hidden = true;
    if (scene === 'selected') { state.selection = state.defaultSelection.map((item) => ({ ...item })); state.sendMode = 'single'; }
    if (scene === 'default') { state.selection = []; state.sendMode = 'single'; }
    if (scene === 'empty') { state.selection = []; state.sendMode = 'single'; }
    if (scene === 'dense' || scene === 'multi') { state.selection = state.defaultSelection.map((item) => ({ ...item })); state.sendMode = scene === 'multi' ? 'multi' : 'single'; }
    if (scene === 'transferring') {
      state.selection = state.defaultSelection.map((item) => ({ ...item }));
      renderSelection();
      openSendWorkspace('Flutter Reference', false);
    } else {
      renderSelection();
      setSendMode(state.sendMode);
    }
    $$('[data-send-scene]').forEach((item) => item.classList.toggle('active', item.dataset.sendScene === scene));
    showToast(`发送状态：${({ default: '未选择', selected: '已选择', empty: '空设备', dense: '多设备', multi: '多选', transferring: '传输中' })[scene]}`);
  }));
  $('#resetDemo').addEventListener('click', resetDemo);
  $('#dialogClose').addEventListener('click', closeDialog);
  els.dialogFooter.addEventListener('click', (event) => {
    if (event.target.closest('[data-dialog-close]')) closeDialog();
  });
  els.dialog.addEventListener('close', () => {
    if (state.dialogFocus && document.contains(state.dialogFocus)) state.dialogFocus.focus();
  });
  document.addEventListener('keydown', (event) => {
    if (event.key === 'Escape' && els.dialog.open) closeDialog();
  });
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
    if (state.themePreference === 'system') applyTheme('system');
  });
}

function resetDemo() {
  clearInterval(state.sendTimer);
  clearInterval(state.receiveTimer);
  state.selectedDevices.clear();
  state.selection = [];
  state.sendMode = 'single';
  state.sendScene = 'default';
  state.receiveAccepted = false;
  applyViewport('1024x768');
  applyTheme('light');
  setNav('receive');
  setReceiveScene('online');
  setSendMode('single');
  renderSelection();
  $$('[data-receive-scene]').forEach((button) => button.classList.toggle('active', button.dataset.receiveScene === 'online'));
  $$('[data-send-scene]').forEach((button) => button.classList.toggle('active', button.dataset.sendScene === 'default'));
  showToast('评审场景已恢复默认');
}

function init() {
  bindCommonEvents();
  applyTheme('light');
  applyViewport('1024x768');
  setNav('receive');
  setReceiveScene('online');
  setSendMode('single');
  renderSelection();
  renderDevices();
}

init();

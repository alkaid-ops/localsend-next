mod worker;

use eframe::egui::{
    self, Align, Context, FontData, FontDefinitions, FontFamily, Layout, RichText, ScrollArea,
    ViewportBuilder,
};
use localsend_rpc::{
    AutoSaveMode, DesktopSettings, Protocol, ReceiverDecision, ServiceSnapshot, TransferDirection,
    TransferStatus,
};
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use worker::{DaemonWorker, UiCommand, WorkerEvent};

thread_local! {
    static ENGLISH_UI: Cell<bool> = const { Cell::new(false) };
}

fn tr(chinese: &'static str, english: &'static str) -> &'static str {
    if ENGLISH_UI.with(Cell::get) {
        english
    } else {
        chinese
    }
}

fn localized(chinese: impl FnOnce() -> String, english: impl FnOnce() -> String) -> String {
    if ENGLISH_UI.with(Cell::get) {
        english()
    } else {
        chinese()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tab {
    Receive,
    Send,
    Settings,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SendMode {
    Single,
    Multiple,
}

#[derive(Clone, Debug)]
struct WebShareApproval {
    ip: String,
    session_id: String,
    user_agent: Option<String>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct WindowGeometry {
    position: [f32; 2],
    size: [f32; 2],
}

struct DesktopApp {
    worker: DaemonWorker,
    snapshot: Option<ServiceSnapshot>,
    tab: Tab,
    message: String,
    selected_paths: Vec<PathBuf>,
    pending_text: Option<String>,
    selected_device: Option<String>,
    send_mode: SendMode,
    selected_devices: HashSet<String>,
    selected_request_id: Option<String>,
    selected_request_files: HashSet<String>,
    text_dialog: bool,
    text_input: String,
    manual_dialog: bool,
    manual_input: String,
    pin_dialog: bool,
    pin_transfer_id: String,
    pin_input: String,
    history_dialog: bool,
    network_dialog: bool,
    message_dialog: Option<(String, String)>,
    favorite_edit: Option<(String, String)>,
    web_share_dialog: bool,
    web_share_pin_enabled: bool,
    web_share_pin_input: String,
    web_share_auto_accept: bool,
    web_share_request: Option<WebShareApproval>,
    web_share_progress: Option<(u64, u64)>,
    settings_draft: Option<DesktopSettings>,
    settings_dirty: bool,
    settings_conflict: bool,
    settings_last_edit: Option<Instant>,
    system_dark: bool,
    launch_on_login_applied: Option<bool>,
    restored_window_geometry: Option<WindowGeometry>,
    window_geometry_restored: bool,
    last_window_geometry: Option<WindowGeometry>,
}

impl DesktopApp {
    fn new(context: &eframe::CreationContext<'_>) -> Self {
        install_cjk_font(&context.egui_ctx);
        context.egui_ctx.set_visuals(egui::Visuals::dark());
        let restored_window_geometry = context
            .storage
            .and_then(|storage| eframe::get_value::<WindowGeometry>(storage, "window_geometry"));
        Self::with_worker_and_geometry(DaemonWorker::spawn(), restored_window_geometry)
    }

    #[cfg(test)]
    fn with_worker(worker: DaemonWorker) -> Self {
        Self::with_worker_and_geometry(worker, None)
    }

    fn with_worker_and_geometry(
        worker: DaemonWorker,
        restored_window_geometry: Option<WindowGeometry>,
    ) -> Self {
        Self {
            worker,
            snapshot: None,
            tab: Tab::Receive,
            message: tr("正在连接 localsendd…", "Connecting to localsendd…").to_string(),
            selected_paths: Vec::new(),
            pending_text: None,
            selected_device: None,
            send_mode: SendMode::Single,
            selected_devices: HashSet::new(),
            selected_request_id: None,
            selected_request_files: HashSet::new(),
            text_dialog: false,
            text_input: String::new(),
            manual_dialog: false,
            manual_input: String::new(),
            pin_dialog: false,
            pin_transfer_id: String::new(),
            pin_input: String::new(),
            history_dialog: false,
            network_dialog: false,
            message_dialog: None,
            favorite_edit: None,
            web_share_dialog: false,
            web_share_pin_enabled: false,
            web_share_pin_input: String::new(),
            web_share_auto_accept: false,
            web_share_request: None,
            web_share_progress: None,
            settings_draft: None,
            settings_dirty: false,
            settings_conflict: false,
            settings_last_edit: None,
            system_dark: true,
            launch_on_login_applied: None,
            restored_window_geometry,
            window_geometry_restored: false,
            last_window_geometry: None,
        }
    }

    fn dispatch(&self, command: UiCommand) {
        let _ = self.worker.commands.send(command);
    }

    fn poll_events(&mut self) {
        while let Ok(event) = self.worker.events.try_recv() {
            match event {
                WorkerEvent::Snapshot(snapshot) => {
                    self.apply_snapshot(*snapshot);
                }
                WorkerEvent::Notice(message) => self.message = message,
                WorkerEvent::Connection { connected, message } => {
                    if !connected {
                        self.message = message;
                    } else {
                        self.message =
                            tr("已连接 localsendd", "Connected to localsendd").to_string();
                    }
                }
                WorkerEvent::PinRequired { transfer_id } => {
                    self.pin_transfer_id = transfer_id;
                    self.pin_input.clear();
                    self.pin_dialog = true;
                }
                WorkerEvent::TransferCreated { transfer_id, .. } => {
                    let short_id = &transfer_id[..8.min(transfer_id.len())];
                    self.message = localized(
                        || format!("已创建传输 {short_id}"),
                        || format!("Transfer {short_id} created"),
                    );
                }
                WorkerEvent::TextReceived { sender_alias, text } => {
                    self.message_dialog = Some((sender_alias, text));
                }
                WorkerEvent::WebShareRequested {
                    ip,
                    session_id,
                    user_agent,
                } => {
                    self.message = localized(
                        || format!("浏览器 {ip} 请求上传文件"),
                        || format!("Browser {ip} requested a file upload"),
                    );
                    self.web_share_request = Some(WebShareApproval {
                        ip,
                        session_id,
                        user_agent,
                    });
                }
                WorkerEvent::WebShareProgress {
                    session_id,
                    bytes,
                    total,
                } => {
                    if self
                        .web_share_request
                        .as_ref()
                        .is_some_and(|request| request.session_id == session_id)
                    {
                        self.web_share_progress = Some((bytes, total));
                    }
                }
                WorkerEvent::WebShareDone { session_id } => {
                    if self
                        .web_share_request
                        .as_ref()
                        .is_some_and(|request| request.session_id == session_id)
                    {
                        self.web_share_request = None;
                        self.web_share_progress = None;
                    }
                }
            }
        }
    }

    fn apply_snapshot(&mut self, snapshot: ServiceSnapshot) {
        ENGLISH_UI.with(|language| {
            language.set(snapshot.settings.language.starts_with("en"));
        });
        let launch_on_login = snapshot.settings.launch_on_login;
        if self.launch_on_login_applied != Some(launch_on_login) {
            if !cfg!(test)
                && let Err(error) = localsend_platform::apply_launch_on_login(launch_on_login)
            {
                self.message = localized(
                    || format!("更新开机启动失败：{error}"),
                    || format!("Failed to update launch on login: {error}"),
                );
            }
            self.launch_on_login_applied = Some(launch_on_login);
        }
        let request_id = snapshot
            .incoming_requests
            .first()
            .map(|request| request.request_id.clone());
        if request_id != self.selected_request_id {
            self.selected_request_id = request_id;
            self.selected_request_files = snapshot
                .incoming_requests
                .first()
                .map(|request| request.files.iter().map(|file| file.id.clone()).collect())
                .unwrap_or_default();
        }
        if self.settings_dirty {
            if self
                .settings_draft
                .as_ref()
                .is_some_and(|draft| draft.revision != snapshot.settings.revision)
            {
                self.settings_conflict = true;
                self.message = tr(
                    "设置已被其他前端修改；请刷新后重试",
                    "Settings were changed by another frontend; reload and try again",
                )
                .to_string();
            }
        } else {
            self.settings_draft = Some(snapshot.settings.clone());
            self.settings_conflict = false;
        }
        self.snapshot = Some(snapshot);
    }

    fn choose_files(&mut self) {
        let paths = pick_files_for_e2e();
        if !paths.is_empty() {
            self.selected_paths.extend(paths);
            self.selected_paths.sort();
            self.selected_paths.dedup();
            self.pending_text = None;
            self.dispatch(UiCommand::RefreshDiscovery);
        }
    }

    fn choose_folder(&mut self) {
        if let Some(path) = pick_folder_for_e2e() {
            if !self.selected_paths.contains(&path) {
                self.selected_paths.push(path);
            }
            self.pending_text = None;
            self.dispatch(UiCommand::RefreshDiscovery);
        }
    }

    fn send_to_device(&mut self, fingerprint: String) {
        self.selected_device = Some(fingerprint.clone());
        if !self.selected_paths.is_empty() {
            self.dispatch(UiCommand::SendFiles {
                fingerprint,
                paths: self.selected_paths.clone(),
            });
            self.message = tr("正在连接目标设备", "Connecting to target device").to_string();
        } else if let Some(text) = self.pending_text.take() {
            self.dispatch(UiCommand::SendText { fingerprint, text });
            self.message = tr("正在发送文本", "Sending text").to_string();
        } else {
            self.message = tr(
                "请先选择文件、文件夹、文本或剪贴板",
                "Select a file, folder, text, or clipboard content first",
            )
            .to_string();
        }
    }

    fn send_to_selected_devices(&mut self) {
        let fingerprints = self.selected_devices.iter().cloned().collect::<Vec<_>>();
        if fingerprints.is_empty() {
            self.message = tr("请先选择至少一个设备", "Select at least one device").to_string();
            return;
        }
        if self.selected_paths.is_empty() && self.pending_text.is_none() {
            self.message = tr(
                "请先选择文件、文件夹、文本或剪贴板",
                "Select a file, folder, text, or clipboard content first",
            )
            .to_string();
            return;
        }
        for fingerprint in &fingerprints {
            if !self.selected_paths.is_empty() {
                self.dispatch(UiCommand::SendFiles {
                    fingerprint: fingerprint.clone(),
                    paths: self.selected_paths.clone(),
                });
            } else if let Some(text) = self.pending_text.clone() {
                self.dispatch(UiCommand::SendText {
                    fingerprint: fingerprint.clone(),
                    text,
                });
            }
        }
        self.selected_device = fingerprints.first().cloned();
        self.selected_devices.clear();
        self.message = localized(
            || format!("正在发送到 {} 个设备", fingerprints.len()),
            || format!("Sending to {} devices", fingerprints.len()),
        );
    }

    fn settings_mut(&mut self) -> Option<&mut DesktopSettings> {
        if self.settings_draft.is_none() {
            self.settings_draft = self
                .snapshot
                .as_ref()
                .map(|snapshot| snapshot.settings.clone());
        }
        self.settings_draft.as_mut()
    }

    fn mark_settings_edited(&mut self) {
        self.settings_dirty = true;
        self.settings_last_edit = Some(Instant::now());
    }

    fn flush_settings(&mut self, force: bool) {
        if !self.settings_dirty
            || (!force
                && self
                    .settings_last_edit
                    .is_some_and(|at| at.elapsed() < Duration::from_millis(600)))
        {
            return;
        }
        let Some(snapshot) = &self.snapshot else {
            return;
        };
        let Some(mut settings) = self.settings_draft.clone() else {
            return;
        };
        if self.settings_conflict || settings.revision != snapshot.settings.revision {
            self.settings_conflict = true;
            self.message = tr(
                "设置 revision 冲突；请刷新设置后重试",
                "Settings revision conflict; reload and try again",
            )
            .to_string();
            return;
        }
        settings.revision = snapshot.settings.revision;
        self.dispatch(UiCommand::UpdateSettings {
            expected_revision: snapshot.settings.revision,
            settings: Box::new(settings),
        });
        self.settings_dirty = false;
        self.message = tr("设置已提交", "Settings submitted").to_string();
    }

    fn reload_settings(&mut self) {
        self.settings_draft = self
            .snapshot
            .as_ref()
            .map(|snapshot| snapshot.settings.clone());
        self.settings_dirty = false;
        self.settings_conflict = false;
        self.message = tr("设置已刷新", "Settings reloaded").to_string();
    }

    fn respond(&mut self, decision: ReceiverDecision) {
        let Some(snapshot) = &self.snapshot else {
            return;
        };
        let Some(request) = snapshot.incoming_requests.first() else {
            return;
        };
        self.dispatch(UiCommand::RespondTransfer {
            request_id: request.request_id.clone(),
            decision,
        });
        self.selected_request_files.clear();
        self.selected_request_id = None;
    }

    fn accept_selected(&mut self) {
        let Some(snapshot) = &self.snapshot else {
            return;
        };
        let Some(request) = snapshot.incoming_requests.first() else {
            return;
        };
        if self.selected_request_files.is_empty() {
            self.message = tr("请至少选择一个文件", "Select at least one file").to_string();
            return;
        }
        self.dispatch(UiCommand::RespondTransfer {
            request_id: request.request_id.clone(),
            decision: ReceiverDecision::AcceptFiles {
                file_ids: self.selected_request_files.iter().cloned().collect(),
            },
        });
        self.selected_request_files.clear();
        self.selected_request_id = None;
    }

    fn draw_navigation(&mut self, root: &mut egui::Ui) {
        egui::Panel::left("navigation")
            .exact_size(210.0)
            .resizable(false)
            .show(root, |ui| {
                ui.add_space(20.0);
                ui.heading(RichText::new("LocalSend").size(30.0));
                ui.add_space(28.0);
                for (tab, label) in [
                    (Tab::Receive, tr("接收", "Receive")),
                    (Tab::Send, tr("发送", "Send")),
                    (Tab::Settings, tr("设置", "Settings")),
                ] {
                    if ui
                        .add_sized(
                            [180.0, 48.0],
                            egui::Button::selectable(
                                self.tab == tab,
                                RichText::new(label).color(if self.tab == tab {
                                    egui::Color32::WHITE
                                } else {
                                    ui.visuals().text_color()
                                }),
                            ),
                        )
                        .clicked()
                    {
                        self.tab = tab;
                        self.message.clear();
                        if tab == Tab::Send {
                            self.dispatch(UiCommand::RefreshDiscovery);
                        }
                    }
                }
                ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                    ui.add_space(16.0);
                    ui.label(RichText::new(&self.message).small());
                    let running = self
                        .snapshot
                        .as_ref()
                        .is_some_and(|snapshot| snapshot.server_running);
                    ui.label(if running {
                        tr("服务器运行中", "Server running")
                    } else {
                        tr("服务器已停止", "Server stopped")
                    });
                });
            });
    }

    fn draw_receive(&mut self, ui: &mut egui::Ui) {
        let Some(snapshot) = self.snapshot.clone() else {
            ui.spinner();
            return;
        };
        let receive_activity = !snapshot.incoming_requests.is_empty()
            || snapshot
                .active_transfers
                .iter()
                .any(|transfer| transfer.direction == TransferDirection::Receive);
        if !receive_activity && self.web_share_request.is_none() {
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), 32.0),
                Layout::right_to_left(Align::Center),
                |ui| {
                    if ui.button(tr("网络信息", "Network info")).clicked() {
                        self.network_dialog = true;
                    }
                    if ui
                        .button(format!(
                            "{} ({})",
                            tr("历史记录", "History"),
                            snapshot.history.len()
                        ))
                        .clicked()
                    {
                        self.history_dialog = true;
                    }
                },
            );
            ui.add_space(68.0);
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("LocalSend").size(44.0).strong());
                ui.add_space(18.0);
                ui.label(RichText::new(&snapshot.settings.alias).size(38.0));
                if let Some(local) = snapshot
                    .devices
                    .iter()
                    .find(|device| device.source == "local")
                {
                    ui.label(
                        RichText::new(device_visual_id(&local.device.ip))
                            .size(22.0)
                            .color(ui.visuals().weak_text_color()),
                    );
                }
                ui.add_space(10.0);
                let status = if snapshot.server_running {
                    format!(
                        "{} · {}",
                        tr("在线", "Online"),
                        snapshot.server_port.unwrap_or(snapshot.settings.port)
                    )
                } else {
                    tr("服务器已停止", "Server stopped").to_string()
                };
                ui.label(status);
                ui.add_space(18.0);
                if ui
                    .add_sized(
                        [180.0, 44.0],
                        egui::Button::new(tr("通过链接接收", "Receive via link")),
                    )
                    .clicked()
                {
                    self.web_share_pin_enabled = snapshot.settings.pin.is_some();
                    self.web_share_pin_input = snapshot.settings.pin.clone().unwrap_or_default();
                    self.web_share_auto_accept = snapshot.settings.auto_accept;
                    self.web_share_dialog = true;
                }
                if let Some(local) = snapshot
                    .devices
                    .iter()
                    .find(|device| device.source == "local" && device.device.download)
                {
                    ui.add_space(12.0);
                    let url = local_ip_url(local, snapshot.server_port, snapshot.settings.protocol);
                    ui.label(&url);
                    ui.horizontal(|ui| {
                        if ui.button(tr("复制链接", "Copy link")).clicked() {
                            ui.ctx().copy_text(url.clone());
                        }
                        if ui.button(tr("停止分享", "Stop sharing")).clicked() {
                            self.dispatch(UiCommand::StopWebShare);
                        }
                    });
                }
            });
            return;
        }
        ui.heading(tr("接收", "Receive"));
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            ui.label(format!(
                "{} · {}",
                snapshot.settings.alias,
                snapshot.server_port.unwrap_or(snapshot.settings.port)
            ));
            if ui
                .button(if snapshot.server_running {
                    tr("停止服务器", "Stop server")
                } else {
                    tr("启动服务器", "Start server")
                })
                .clicked()
            {
                self.dispatch(UiCommand::ToggleServer);
            }
            if ui.button(tr("通过链接接收", "Receive via link")).clicked() {
                self.web_share_pin_enabled = snapshot.settings.pin.is_some();
                self.web_share_pin_input = snapshot.settings.pin.clone().unwrap_or_default();
                self.web_share_auto_accept = snapshot.settings.auto_accept;
                self.web_share_dialog = true;
            }
            if ui.button(tr("网络信息", "Network info")).clicked() {
                self.network_dialog = true;
            }
        });
        if let Some(local) = snapshot
            .devices
            .iter()
            .find(|device| device.source == "local" && device.device.download)
        {
            ui.horizontal(|ui| {
                let url = local_ip_url(local, snapshot.server_port, snapshot.settings.protocol);
                ui.label(format!("{}: {url}", tr("分享链接", "Share link")));
                if ui.button(tr("复制链接", "Copy link")).clicked() {
                    ui.ctx().copy_text(url.clone());
                }
                if ui.button(tr("打开链接", "Open link")).clicked() {
                    ui.ctx().open_url(egui::OpenUrl::new_tab(url.clone()));
                }
                if ui.button(tr("停止分享", "Stop sharing")).clicked() {
                    self.dispatch(UiCommand::StopWebShare);
                }
            });
        }
        if let Some(request) = self.web_share_request.clone() {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.heading(tr("浏览器请求上传", "Browser upload request"));
                ui.label(format!(
                    "{} · {}",
                    request.ip,
                    request
                        .user_agent
                        .as_deref()
                        .unwrap_or(tr("未知浏览器", "Unknown browser"))
                ));
                if let Some((bytes, total)) = self.web_share_progress {
                    let progress = if total == 0 {
                        0.0
                    } else {
                        bytes as f32 / total as f32
                    };
                    ui.add(egui::ProgressBar::new(progress).show_percentage());
                }
                ui.horizontal(|ui| {
                    if ui.button(tr("拒绝上传", "Reject upload")).clicked() {
                        self.dispatch(UiCommand::RespondWebShare {
                            session_id: request.session_id.clone(),
                            accepted: false,
                        });
                        self.web_share_request = None;
                    }
                    if ui.button(tr("接受上传", "Accept upload")).clicked() {
                        self.dispatch(UiCommand::RespondWebShare {
                            session_id: request.session_id.clone(),
                            accepted: true,
                        });
                    }
                });
            });
            ui.add_space(8.0);
        }
        ui.add_space(12.0);

        if let Some(request) = snapshot.incoming_requests.first() {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.heading(format!(
                    "{} {}",
                    request.sender.alias,
                    tr("想发送文件", "wants to send files")
                ));
                for file in &request.files {
                    let mut selected = self.selected_request_files.contains(&file.id);
                    if ui
                        .checkbox(
                            &mut selected,
                            format!("{} · {}", file.file_name, human_size(file.size)),
                        )
                        .changed()
                    {
                        if selected {
                            self.selected_request_files.insert(file.id.clone());
                        } else {
                            self.selected_request_files.remove(&file.id);
                        }
                    }
                }
                ui.horizontal(|ui| {
                    if ui.button(tr("拒绝", "Decline")).clicked() {
                        self.respond(ReceiverDecision::Decline);
                    }
                    if ui
                        .add_enabled(
                            !self.selected_request_files.is_empty(),
                            egui::Button::new(tr("接受所选", "Accept selected")),
                        )
                        .clicked()
                    {
                        self.accept_selected();
                    }
                    if ui.button(tr("全部接受", "Accept all")).clicked() {
                        self.selected_request_files.clear();
                        self.respond(ReceiverDecision::AcceptAll);
                    }
                });
            });
            ui.add_space(10.0);
        }

        ScrollArea::vertical().max_height(320.0).show(ui, |ui| {
            for transfer in snapshot
                .active_transfers
                .iter()
                .filter(|transfer| transfer.direction == TransferDirection::Receive)
            {
                transfer_card(ui, transfer, &self.worker);
            }
        });

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            if ui
                .button(format!(
                    "{} ({})",
                    tr("历史记录", "History"),
                    snapshot.history.len()
                ))
                .clicked()
            {
                self.history_dialog = true;
            }
            if ui
                .button(tr("打开接收目录", "Open receive folder"))
                .clicked()
            {
                let path = snapshot
                    .settings
                    .destination_dir
                    .clone()
                    .unwrap_or_else(default_download_dir);
                let _ = open_path(&path);
            }
        });
    }

    fn draw_send(&mut self, ui: &mut egui::Ui) {
        ui.heading(tr("选择", "Select"));
        ui.add_space(10.0);
        ui.horizontal_wrapped(|ui| {
            if ui
                .add_sized([110.0, 48.0], egui::Button::new(tr("文件", "File")))
                .clicked()
            {
                self.choose_files();
            }
            if ui
                .add_sized([110.0, 48.0], egui::Button::new(tr("文件夹", "Folder")))
                .clicked()
            {
                self.choose_folder();
            }
            if ui
                .add_sized([110.0, 48.0], egui::Button::new(tr("文本", "Text")))
                .clicked()
            {
                self.text_dialog = true;
            }
            if ui
                .add_sized([110.0, 48.0], egui::Button::new(tr("剪贴板", "Clipboard")))
                .clicked()
            {
                match arboard::Clipboard::new().and_then(|mut clipboard| clipboard.get_text()) {
                    Ok(text) if !text.trim().is_empty() => {
                        self.pending_text = Some(text);
                        self.selected_paths.clear();
                        self.dispatch(UiCommand::RefreshDiscovery);
                    }
                    _ => {
                        self.message = tr("剪贴板中没有文本", "No text in clipboard").to_string();
                    }
                }
            }
        });
        ui.add_space(10.0);
        if !self.selected_paths.is_empty() {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "{} {}",
                        tr("已选择", "Selected"),
                        self.selected_paths.len()
                    ));
                    if ui.small_button(tr("清除", "Clear")).clicked() {
                        self.selected_paths.clear();
                    }
                });
                for path in self.selected_paths.iter().take(8) {
                    ui.label(path.display().to_string());
                }
            });
        } else if let Some(text) = &self.pending_text {
            ui.label(format!("{}: {text}", tr("待发送文本", "Pending text")));
        }
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            ui.heading(tr("设备", "Devices"));
            if ui.button(tr("刷新", "Refresh")).clicked() {
                self.dispatch(UiCommand::RefreshDiscovery);
            }
            if ui.button(tr("手动 IP", "Manual IP")).clicked() {
                self.manual_dialog = true;
            }
            ui.separator();
            ui.selectable_value(&mut self.send_mode, SendMode::Single, tr("单选", "Single"));
            ui.selectable_value(
                &mut self.send_mode,
                SendMode::Multiple,
                tr("多选", "Multiple"),
            );
        });

        let Some(snapshot) = self.snapshot.clone() else {
            return;
        };
        let favorite_fingerprints = snapshot
            .favorites
            .iter()
            .map(|favorite| favorite.device.fingerprint.clone())
            .collect::<HashSet<_>>();

        if !snapshot.favorites.is_empty() {
            ui.label(RichText::new(tr("收藏夹", "Favorites")).strong());
            ScrollArea::vertical()
                .id_salt("favorite_devices")
                .max_height(170.0)
                .show(ui, |ui| {
                    for favorite in &snapshot.favorites {
                        self.draw_device_row(ui, &favorite.device, &favorite.alias, true, true);
                    }
                });
            ui.add_space(8.0);
        }

        ui.label(RichText::new(tr("附近的设备", "Nearby devices")).strong());
        ScrollArea::vertical()
            .id_salt("nearby_devices")
            .max_height(260.0)
            .show(ui, |ui| {
                for device in snapshot
                    .devices
                    .iter()
                    .filter(|device| device.source != "local")
                {
                    let favorite = favorite_fingerprints.contains(&device.device.fingerprint);
                    self.draw_device_row(ui, &device.device, &device.device.alias, favorite, false);
                }
            });
        if self.send_mode == SendMode::Multiple {
            ui.add_space(6.0);
            if ui
                .button(format!(
                    "{} ({})",
                    tr("发送到所选", "Send to selected"),
                    self.selected_devices.len()
                ))
                .clicked()
            {
                self.send_to_selected_devices();
            }
        }
        ui.add_space(8.0);
        ScrollArea::vertical().max_height(220.0).show(ui, |ui| {
            for transfer in snapshot
                .active_transfers
                .iter()
                .filter(|transfer| transfer.direction == TransferDirection::Send)
            {
                transfer_card(ui, transfer, &self.worker);
            }
        });
    }

    fn draw_device_row(
        &mut self,
        ui: &mut egui::Ui,
        device: &localsend_rpc::DeviceInfo,
        display_alias: &str,
        favorite: bool,
        favorite_section: bool,
    ) {
        let is_selected = if self.send_mode == SendMode::Single {
            self.selected_device.as_deref() == Some(device.fingerprint.as_str())
        } else {
            self.selected_devices.contains(&device.fingerprint)
        };
        egui::Frame::group(ui.style())
            .fill(if is_selected {
                ui.visuals().selection.bg_fill
            } else {
                ui.visuals().faint_bg_color
            })
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new(display_alias).size(18.0));
                        if display_alias != device.alias {
                            ui.label(format!("{}: {}", tr("原名", "Original name"), device.alias));
                        }
                        ui.label(format!(
                            "{} · {}",
                            device.ip.as_deref().unwrap_or("?"),
                            if device.protocol == Protocol::Https {
                                "HTTPS"
                            } else {
                                "HTTP"
                            }
                        ));
                    });
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        match self.send_mode {
                            SendMode::Single => {
                                if ui.button(tr("发送", "Send")).clicked() {
                                    self.send_to_device(device.fingerprint.clone());
                                }
                            }
                            SendMode::Multiple => {
                                let label = if is_selected {
                                    tr("取消选择", "Deselect")
                                } else {
                                    tr("选择", "Select")
                                };
                                if ui.button(label).clicked() {
                                    if is_selected {
                                        self.selected_devices.remove(&device.fingerprint);
                                    } else {
                                        self.selected_devices.insert(device.fingerprint.clone());
                                    }
                                }
                            }
                        }
                        if favorite_section {
                            if ui.button(tr("重命名", "Rename")).clicked() {
                                self.favorite_edit =
                                    Some((device.fingerprint.clone(), display_alias.to_string()));
                            }
                            if ui.button(tr("移除收藏", "Remove favorite")).clicked() {
                                self.dispatch(UiCommand::ToggleFavorite(
                                    device.fingerprint.clone(),
                                ));
                            }
                        } else {
                            if ui
                                .button(if favorite {
                                    tr("取消收藏", "Remove favorite")
                                } else {
                                    tr("收藏", "Favorite")
                                })
                                .clicked()
                            {
                                self.dispatch(UiCommand::ToggleFavorite(
                                    device.fingerprint.clone(),
                                ));
                            }
                        }
                    });
                });
            });
    }

    fn draw_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading(tr("设置选项", "Settings"));
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            if ui.button(tr("保存设置", "Save settings")).clicked() {
                self.flush_settings(true);
            }
            if ui.button(tr("刷新设置", "Reload settings")).clicked() {
                self.reload_settings();
            }
            if self.settings_conflict {
                ui.colored_label(
                    ui.visuals().error_fg_color,
                    tr("检测到 revision 冲突", "Settings revision conflict"),
                );
            } else if self.settings_dirty {
                ui.label(tr("有未保存的修改", "Unsaved changes"));
            }
        });
        ui.add_space(8.0);
        if self.settings_mut().is_none() {
            ui.spinner();
            return;
        }
        let draft = self.settings_draft.as_mut().unwrap();
        let mut changed = false;
        let mut restart_server = false;

        egui::Grid::new("settings_grid")
            .num_columns(2)
            .spacing([28.0, 14.0])
            .show(ui, |ui| {
                let label = ui.label(tr("设备名称", "Device name"));
                changed |= ui
                    .text_edit_singleline(&mut draft.alias)
                    .labelled_by(label.id)
                    .changed();
                ui.end_row();

                ui.label(tr("端口", "Port"));
                changed |= ui
                    .add(egui::DragValue::new(&mut draft.port).range(1..=65535))
                    .changed();
                ui.end_row();

                let label = ui.label(tr("HTTPS 加密", "HTTPS encryption"));
                let mut https = draft.protocol == Protocol::Https;
                if ui
                    .add(egui::Checkbox::without_text(&mut https))
                    .labelled_by(label.id)
                    .changed()
                {
                    draft.protocol = if https {
                        Protocol::Https
                    } else {
                        Protocol::Http
                    };
                    changed = true;
                }
                ui.end_row();

                let label = ui.label(tr("PIN", "PIN"));
                let mut pin = draft.pin.clone().unwrap_or_default();
                if ui
                    .text_edit_singleline(&mut pin)
                    .labelled_by(label.id)
                    .changed()
                {
                    draft.pin = (!pin.trim().is_empty()).then_some(pin);
                    changed = true;
                }
                ui.end_row();

                let label = ui.label(tr("接收目录", "Receive folder"));
                let mut destination = draft
                    .destination_dir
                    .as_ref()
                    .map(|path| path.display().to_string())
                    .unwrap_or_default();
                if ui
                    .text_edit_singleline(&mut destination)
                    .labelled_by(label.id)
                    .changed()
                {
                    draft.destination_dir =
                        (!destination.trim().is_empty()).then(|| PathBuf::from(destination));
                    changed = true;
                }
                ui.end_row();

                let label = ui.label(tr("自动保存", "Auto save"));
                let mut all = draft.auto_save == AutoSaveMode::On;
                if ui
                    .add(egui::Checkbox::without_text(&mut all))
                    .labelled_by(label.id)
                    .changed()
                {
                    draft.auto_save = if all {
                        AutoSaveMode::On
                    } else if draft.auto_save_from_favorites {
                        AutoSaveMode::Favorites
                    } else {
                        AutoSaveMode::Off
                    };
                    changed = true;
                }
                ui.end_row();

                let label = ui.label(tr("收藏夹自动保存", "Auto save from favorites"));
                if ui
                    .add(egui::Checkbox::without_text(
                        &mut draft.auto_save_from_favorites,
                    ))
                    .labelled_by(label.id)
                    .changed()
                {
                    draft.auto_save = if draft.auto_save == AutoSaveMode::On {
                        AutoSaveMode::On
                    } else if draft.auto_save_from_favorites {
                        AutoSaveMode::Favorites
                    } else {
                        AutoSaveMode::Off
                    };
                    changed = true;
                }
                ui.end_row();

                let label = ui.label(tr("自动完成传输任务", "Auto finish transfers"));
                changed |= ui
                    .add(egui::Checkbox::without_text(&mut draft.auto_finish))
                    .labelled_by(label.id)
                    .changed();
                ui.end_row();

                let label = ui.label(tr("保存到历史记录", "Save history"));
                changed |= ui
                    .add(egui::Checkbox::without_text(&mut draft.save_to_history))
                    .labelled_by(label.id)
                    .changed();
                ui.end_row();

                ui.label(tr("历史记录上限", "History limit"));
                changed |= ui
                    .add(egui::DragValue::new(&mut draft.history_limit).range(1..=100_000))
                    .changed();
                ui.end_row();

                let label = ui.label(tr("发送时创建校验和", "Create checksums when sending"));
                changed |= ui
                    .add(egui::Checkbox::without_text(&mut draft.create_checksums))
                    .labelled_by(label.id)
                    .changed();
                ui.end_row();

                let label = ui.label(tr("接收时验证校验和", "Verify checksums when receiving"));
                changed |= ui
                    .add(egui::Checkbox::without_text(&mut draft.verify_checksums))
                    .labelled_by(label.id)
                    .changed();
                ui.end_row();

                ui.label(tr("搜索设备超时（毫秒）", "Discovery timeout (ms)"));
                changed |= ui
                    .add(egui::DragValue::new(&mut draft.discovery_timeout_ms).range(1..=60_000))
                    .changed();
                ui.end_row();

                let label = ui.label(tr("通过链接分享时自动接收", "Auto accept Web Share"));
                changed |= ui
                    .add(egui::Checkbox::without_text(&mut draft.auto_accept))
                    .labelled_by(label.id)
                    .changed();
                ui.end_row();

                let label = ui.label(tr("退出时保存窗口位置", "Save window position"));
                changed |= ui
                    .add(egui::Checkbox::without_text(
                        &mut draft.save_window_position,
                    ))
                    .labelled_by(label.id)
                    .changed();
                ui.end_row();

                let label = ui.label(tr("关闭时最小化到托盘", "Close to tray"));
                changed |= ui
                    .add(egui::Checkbox::without_text(&mut draft.minimize_to_tray))
                    .labelled_by(label.id)
                    .changed();
                ui.end_row();

                let label = ui.label(tr("登录后自动启动", "Launch on login"));
                changed |= ui
                    .add(egui::Checkbox::without_text(&mut draft.launch_on_login))
                    .labelled_by(label.id)
                    .changed();
                ui.end_row();

                let label = ui.label(tr("动画效果", "Animations"));
                changed |= ui
                    .add(egui::Checkbox::without_text(&mut draft.animations))
                    .labelled_by(label.id)
                    .changed();
                ui.end_row();

                let label = ui.label(tr("启动时开启服务器", "Start server on launch"));
                changed |= ui
                    .add(egui::Checkbox::without_text(
                        &mut draft.start_server_on_launch,
                    ))
                    .labelled_by(label.id)
                    .changed();
                ui.end_row();

                let label = ui.label(tr("多播地址", "Multicast address"));
                let mut multicast = draft.multicast_group.clone().unwrap_or_default();
                if ui
                    .text_edit_singleline(&mut multicast)
                    .labelled_by(label.id)
                    .changed()
                {
                    draft.multicast_group = (!multicast.trim().is_empty()).then_some(multicast);
                    changed = true;
                }
                ui.end_row();

                let label = ui.label(tr(
                    "网络白名单（逗号分隔）",
                    "Interface allowlist (comma separated)",
                ));
                let mut whitelist = draft
                    .interface_whitelist
                    .as_ref()
                    .map(|items| items.join(","))
                    .unwrap_or_default();
                if ui
                    .text_edit_singleline(&mut whitelist)
                    .labelled_by(label.id)
                    .changed()
                {
                    draft.interface_whitelist = parse_filter_list(&whitelist);
                    changed = true;
                }
                ui.end_row();

                let label = ui.label(tr(
                    "网络黑名单（逗号分隔）",
                    "Interface blocklist (comma separated)",
                ));
                let mut blacklist = draft
                    .interface_blacklist
                    .as_ref()
                    .map(|items| items.join(","))
                    .unwrap_or_default();
                if ui
                    .text_edit_singleline(&mut blacklist)
                    .labelled_by(label.id)
                    .changed()
                {
                    draft.interface_blacklist = parse_filter_list(&blacklist);
                    changed = true;
                }
                ui.end_row();

                ui.label(tr("服务器", "Server"));
                ui.horizontal(|ui| {
                    if ui.button(tr("重启", "Restart")).clicked() {
                        restart_server = true;
                    }
                });
                ui.end_row();

                ui.label(tr("主题", "Theme"));
                egui::ComboBox::from_id_salt("theme")
                    .selected_text(theme_label(&draft.theme))
                    .show_ui(ui, |ui| {
                        for value in ["system", "light", "dark"] {
                            changed |= ui
                                .selectable_value(
                                    &mut draft.theme,
                                    value.to_string(),
                                    theme_label(value),
                                )
                                .changed();
                        }
                    });
                ui.end_row();

                ui.label(tr("颜色", "Color"));
                egui::ComboBox::from_id_salt("color")
                    .selected_text(color_label(&draft.color))
                    .show_ui(ui, |ui| {
                        for value in ["localsend", "blue", "green"] {
                            changed |= ui
                                .selectable_value(
                                    &mut draft.color,
                                    value.to_string(),
                                    color_label(value),
                                )
                                .changed();
                        }
                    });
                ui.end_row();

                ui.label(tr("语言", "Language"));
                egui::ComboBox::from_id_salt("language")
                    .selected_text(language_label(&draft.language))
                    .show_ui(ui, |ui| {
                        for value in ["system", "zh-CN", "en-US"] {
                            changed |= ui
                                .selectable_value(
                                    &mut draft.language,
                                    value.to_string(),
                                    language_label(value),
                                )
                                .changed();
                        }
                    });
                ui.end_row();
            });

        if changed {
            self.mark_settings_edited();
        }
        if restart_server {
            self.dispatch(UiCommand::RestartServer);
        }
    }

    fn tick(&mut self) {
        self.poll_events();
        self.flush_settings(false);
    }

    fn sync_window_geometry(&mut self, ctx: &Context) {
        let viewport = ctx.input(|input| input.viewport().clone());
        if let (Some(outer), Some(inner)) = (viewport.outer_rect, viewport.inner_rect) {
            self.last_window_geometry = Some(WindowGeometry {
                position: [outer.min.x, outer.min.y],
                size: [inner.width(), inner.height()],
            });
        }
        if self.window_geometry_restored
            || !self
                .snapshot
                .as_ref()
                .is_some_and(|snapshot| snapshot.settings.save_window_position)
        {
            return;
        }
        self.window_geometry_restored = true;
        let Some(geometry) = self.restored_window_geometry else {
            return;
        };
        if geometry.size[0] >= 320.0 && geometry.size[1] >= 240.0 {
            ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(egui::pos2(
                geometry.position[0],
                geometry.position[1],
            )));
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
                geometry.size[0],
                geometry.size[1],
            )));
        }
    }

    fn apply_theme(&self, ctx: &Context) {
        let Some(settings) = self.snapshot.as_ref().map(|snapshot| &snapshot.settings) else {
            return;
        };
        let dark = match settings.theme.as_str() {
            "dark" => true,
            "light" => false,
            _ => self.system_dark,
        };
        let mut visuals = if dark {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };
        visuals.selection.bg_fill = match settings.color.as_str() {
            "blue" => egui::Color32::from_rgb(36, 122, 181),
            "green" => egui::Color32::from_rgb(44, 145, 102),
            _ => egui::Color32::from_rgb(0, 117, 111),
        };
        ctx.set_visuals(visuals);
        let theme = if dark {
            egui::Theme::Dark
        } else {
            egui::Theme::Light
        };
        let mut style = (*ctx.style_of(theme)).clone();
        style.animation_time = if settings.animations { 0.15 } else { 0.0 };
        ctx.set_style_of(theme, style);
    }

    fn render(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        self.sync_window_geometry(&ctx);
        self.apply_theme(&ctx);
        self.draw_navigation(ui);
        egui::CentralPanel::default().show(ui, |ui| match self.tab {
            Tab::Receive => self.draw_receive(ui),
            Tab::Send => self.draw_send(ui),
            Tab::Settings => self.draw_settings(ui),
        });
        self.draw_dialogs(&ctx);
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

impl eframe::App for DesktopApp {
    fn logic(&mut self, _ctx: &Context, _frame: &mut eframe::Frame) {
        self.tick();
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.render(ui);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if self
            .snapshot
            .as_ref()
            .is_some_and(|snapshot| snapshot.settings.save_window_position)
            && let Some(geometry) = self.last_window_geometry
        {
            eframe::set_value(storage, "window_geometry", &geometry);
        }
    }
}

impl DesktopApp {
    fn draw_dialogs(&mut self, ctx: &Context) {
        if self.text_dialog {
            let mut open = self.text_dialog;
            let mut cancel = false;
            let mut submit = false;
            egui::Window::new(tr("发送文本", "Send text"))
                .collapsible(false)
                .resizable(false)
                .open(&mut open)
                .show(ctx, |ui| {
                    let label = ui.label(tr("消息", "Message"));
                    ui.text_edit_multiline(&mut self.text_input)
                        .labelled_by(label.id);
                    ui.horizontal(|ui| {
                        if ui.button(tr("取消", "Cancel")).clicked() {
                            cancel = true;
                        }
                        if ui.button(tr("确定", "OK")).clicked()
                            && !self.text_input.trim().is_empty()
                        {
                            submit = true;
                        }
                    });
                });
            if submit {
                self.pending_text = Some(std::mem::take(&mut self.text_input));
                self.selected_paths.clear();
                self.dispatch(UiCommand::RefreshDiscovery);
            }
            self.text_dialog = open && !cancel && !submit;
        }

        if self.manual_dialog {
            let mut open = self.manual_dialog;
            let mut cancel = false;
            let mut submit = false;
            egui::Window::new(tr("手动添加设备", "Add device manually"))
                .collapsible(false)
                .resizable(false)
                .open(&mut open)
                .show(ctx, |ui| {
                    let label = ui.label(tr("设备地址或主机名", "Device address or hostname"));
                    ui.text_edit_singleline(&mut self.manual_input)
                        .labelled_by(label.id);
                    ui.horizontal(|ui| {
                        if ui.button(tr("取消", "Cancel")).clicked() {
                            cancel = true;
                        }
                        if ui.button(tr("添加", "Add")).clicked()
                            && !self.manual_input.trim().is_empty()
                        {
                            submit = true;
                        }
                    });
                });
            if submit {
                let target = std::mem::take(&mut self.manual_input);
                self.dispatch(UiCommand::AddManual(target));
            }
            self.manual_dialog = open && !cancel && !submit;
        }

        if self.pin_dialog {
            let mut open = self.pin_dialog;
            let mut pin_input = std::mem::take(&mut self.pin_input);
            let mut retry = false;
            egui::Window::new(tr("需要 PIN", "PIN required"))
                .collapsible(false)
                .resizable(false)
                .open(&mut open)
                .show(ctx, |ui| {
                    let label = ui.label(tr("PIN 密码", "PIN"));
                    ui.add(egui::TextEdit::singleline(&mut pin_input).password(true))
                        .labelled_by(label.id);
                    if ui.button(tr("重试", "Retry")).clicked() {
                        retry = true;
                    }
                });
            if retry {
                self.dispatch(UiCommand::RetryPin {
                    transfer_id: self.pin_transfer_id.clone(),
                    pin: pin_input.clone(),
                });
            }
            self.pin_dialog = open && !retry;
            if self.pin_dialog {
                self.pin_input = pin_input;
            }
        }

        if let Some((sender, text)) = self.message_dialog.clone() {
            let mut open = true;
            egui::Window::new(tr("文本消息", "Text message"))
                .collapsible(false)
                .resizable(true)
                .open(&mut open)
                .show(ctx, |ui| {
                    ui.label(sender);
                    let mut text_value = text.clone();
                    ui.add(egui::TextEdit::multiline(&mut text_value).interactive(false));
                    if ui.button(tr("复制", "Copy")).clicked() {
                        ctx.copy_text(text.clone());
                    }
                    if ui.button(tr("关闭", "Close")).clicked() {
                        self.message_dialog = None;
                    }
                });
            if !open {
                self.message_dialog = None;
            }
        }

        if self.history_dialog {
            let mut open = self.history_dialog;
            egui::Window::new(tr("历史记录", "History"))
                .open(&mut open)
                .default_size([520.0, 420.0])
                .show(ctx, |ui| {
                    if ui.button(tr("清空历史", "Clear history")).clicked() {
                        self.dispatch(UiCommand::ClearHistory);
                    }
                    ScrollArea::vertical().show(ui, |ui| {
                        if let Some(snapshot) = &self.snapshot {
                            for entry in snapshot.history.iter().rev() {
                                ui.label(format!(
                                    "{} · {} · {}",
                                    entry.peer_alias,
                                    entry.direction_label(),
                                    entry.file_name
                                ));
                                if let Some(message) = &entry.message {
                                    ui.label(message);
                                }
                                if ui
                                    .button(tr("打开/定位", "Open / show in folder"))
                                    .clicked()
                                    && let Some(path) = &entry.path
                                {
                                    let _ = open_path(path);
                                }
                                ui.separator();
                            }
                        }
                    });
                });
            self.history_dialog = open;
        }

        if self.web_share_dialog {
            let mut open = self.web_share_dialog;
            let mut start = false;
            let mut stop = false;
            let mut close_clicked = false;
            let local = self.snapshot.as_ref().and_then(|snapshot| {
                snapshot
                    .devices
                    .iter()
                    .find(|device| device.source == "local" && device.device.download)
                    .cloned()
            });
            egui::Window::new(tr("通过链接接收", "Receive via link"))
                .collapsible(false)
                .resizable(false)
                .open(&mut open)
                .show(ctx, |ui| {
                    let running = local.is_some();
                    if let Some(local) = &local {
                        let port = self
                            .snapshot
                            .as_ref()
                            .and_then(|snapshot| snapshot.server_port)
                            .unwrap_or(local.device.port);
                        let protocol = self
                            .snapshot
                            .as_ref()
                            .and_then(|snapshot| snapshot.server_protocol)
                            .unwrap_or(local.device.protocol);
                        let url = local_ip_url(local, Some(port), protocol);
                        ui.label(tr("分享链接", "Share link"));
                        let mut shown_url = url.clone();
                        ui.add(egui::TextEdit::singleline(&mut shown_url).interactive(false));
                        ui.horizontal(|ui| {
                            if ui.button(tr("复制链接", "Copy link")).clicked() {
                                ui.ctx().copy_text(url.clone());
                            }
                            if ui.button(tr("在浏览器打开", "Open in browser")).clicked() {
                                ui.ctx().open_url(egui::OpenUrl::new_tab(url.clone()));
                            }
                        });
                    } else {
                        ui.label(tr(
                            "浏览器可以通过链接上传文件到这台设备。",
                            "A browser can upload files to this device through the link.",
                        ));
                        ui.checkbox(
                            &mut self.web_share_pin_enabled,
                            tr("启用 PIN 密码", "Require a PIN"),
                        );
                        if self.web_share_pin_enabled {
                            let label = ui.label(tr("PIN 密码", "PIN"));
                            ui.add(
                                egui::TextEdit::singleline(&mut self.web_share_pin_input)
                                    .password(true),
                            )
                            .labelled_by(label.id);
                        }
                        ui.checkbox(
                            &mut self.web_share_auto_accept,
                            tr("自动接受上传", "Auto accept uploads"),
                        );
                    }
                    if let Some(request) = &self.web_share_request {
                        ui.separator();
                        ui.label(localized(
                            || format!("请求：{}", request.ip),
                            || format!("Request: {}", request.ip),
                        ));
                        if let Some((bytes, total)) = self.web_share_progress {
                            let progress = if total == 0 {
                                0.0
                            } else {
                                bytes as f32 / total as f32
                            };
                            ui.add(egui::ProgressBar::new(progress).show_percentage());
                        }
                        ui.horizontal(|ui| {
                            if ui.button(tr("拒绝", "Decline")).clicked() {
                                self.dispatch(UiCommand::RespondWebShare {
                                    session_id: request.session_id.clone(),
                                    accepted: false,
                                });
                            }
                            if ui.button(tr("接受", "Accept")).clicked() {
                                self.dispatch(UiCommand::RespondWebShare {
                                    session_id: request.session_id.clone(),
                                    accepted: true,
                                });
                            }
                        });
                    }
                    ui.separator();
                    ui.horizontal(|ui| {
                        if running {
                            if ui.button(tr("停止分享", "Stop sharing")).clicked() {
                                stop = true;
                            }
                        } else if ui.button(tr("开始分享", "Start sharing")).clicked() {
                            if self.web_share_pin_enabled
                                && self.web_share_pin_input.trim().is_empty()
                            {
                                self.message = tr(
                                    "启用 PIN 时必须填写 PIN",
                                    "Enter a PIN when PIN protection is enabled",
                                )
                                .to_string();
                            } else {
                                start = true;
                            }
                        }
                        if ui.button(tr("关闭", "Close")).clicked() {
                            close_clicked = true;
                        }
                    });
                });
            if start {
                self.dispatch(UiCommand::StartWebShare {
                    pin: self
                        .web_share_pin_enabled
                        .then(|| self.web_share_pin_input.trim().to_string()),
                    auto_accept: self.web_share_auto_accept,
                });
                self.message = tr("正在启动 Web Share", "Starting Web Share").to_string();
            }
            if stop {
                self.dispatch(UiCommand::StopWebShare);
                self.web_share_request = None;
                self.web_share_progress = None;
            }
            self.web_share_dialog = open && !close_clicked;
        }

        if self.network_dialog {
            let mut open = self.network_dialog;
            let mut close_clicked = false;
            egui::Window::new(tr("网络信息", "Network info"))
                .collapsible(false)
                .resizable(false)
                .open(&mut open)
                .show(ctx, |ui| {
                    if let Some(snapshot) = &self.snapshot {
                        let local = snapshot
                            .devices
                            .iter()
                            .find(|device| device.source == "local");
                        let alias = snapshot
                            .server_alias
                            .as_deref()
                            .unwrap_or(&snapshot.settings.alias);
                        let port = snapshot.server_port.unwrap_or(snapshot.settings.port);
                        let protocol = snapshot
                            .server_protocol
                            .unwrap_or(snapshot.settings.protocol);
                        let ip = local
                            .and_then(|device| device.device.ip.as_deref())
                            .unwrap_or(tr("未知", "Unknown"));
                        let fingerprint = local
                            .map(|device| device.device.fingerprint.as_str())
                            .unwrap_or(tr("未知", "Unknown"));
                        ui.label(localized(
                            || format!("设备名称：{alias}"),
                            || format!("Device name: {alias}"),
                        ));
                        ui.label(localized(
                            || format!("端口：{port}"),
                            || format!("Port: {port}"),
                        ));
                        ui.label(localized(
                            || format!("协议：{protocol}"),
                            || format!("Protocol: {protocol}"),
                        ));
                        ui.label(localized(|| format!("IP：{ip}"), || format!("IP: {ip}")));
                        ui.label(localized(
                            || format!("指纹：{fingerprint}"),
                            || format!("Fingerprint: {fingerprint}"),
                        ));
                    }
                    if ui.button(tr("关闭", "Close")).clicked() {
                        close_clicked = true;
                    }
                });
            self.network_dialog = open && !close_clicked;
        }

        if let Some((fingerprint, alias)) = self.favorite_edit.clone() {
            let mut open = true;
            let mut edited = alias;
            let mut save = false;
            let mut cancel = false;
            egui::Window::new(tr("编辑收藏别名", "Edit favorite alias"))
                .collapsible(false)
                .resizable(false)
                .open(&mut open)
                .show(ctx, |ui| {
                    let label = ui.label(tr("收藏名称", "Favorite name"));
                    ui.text_edit_singleline(&mut edited).labelled_by(label.id);
                    ui.horizontal(|ui| {
                        if ui.button(tr("取消", "Cancel")).clicked() {
                            cancel = true;
                        }
                        if ui.button(tr("保存", "Save")).clicked() && !edited.trim().is_empty() {
                            save = true;
                        }
                    });
                });
            if save {
                self.dispatch(UiCommand::RenameFavorite {
                    fingerprint: fingerprint.clone(),
                    alias: edited.trim().to_string(),
                });
                open = false;
            }
            self.favorite_edit = (open && !cancel).then_some((fingerprint, edited));
        }
    }
}

trait HistoryDirectionLabel {
    fn direction_label(&self) -> &'static str;
}

impl HistoryDirectionLabel for localsend_rpc::HistoryEntry {
    fn direction_label(&self) -> &'static str {
        match self.direction {
            TransferDirection::Send => tr("发送", "Sent"),
            TransferDirection::Receive => tr("接收", "Received"),
        }
    }
}

fn transfer_card(ui: &mut egui::Ui, transfer: &localsend_rpc::TransferView, worker: &DaemonWorker) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new(&transfer.peer_alias).strong());
                let files = transfer
                    .files
                    .iter()
                    .map(|file| file.file_name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                ui.label(format!(
                    "{files} · {} / {}",
                    human_size(transfer.bytes_done),
                    human_size(transfer.bytes_total)
                ));
                let progress = if transfer.bytes_total == 0 {
                    0.0
                } else {
                    transfer.bytes_done as f32 / transfer.bytes_total as f32
                };
                ui.add(egui::ProgressBar::new(progress).show_percentage());
                ui.label(status_label(transfer.status));
                if let Some(error) = &transfer.error {
                    ui.colored_label(ui.visuals().error_fg_color, error);
                }
            });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if matches!(
                    transfer.status,
                    TransferStatus::Waiting | TransferStatus::Transferring
                ) && ui.button(tr("取消", "Cancel")).clicked()
                {
                    let _ = worker
                        .commands
                        .send(UiCommand::CancelTransfer(transfer.transfer_id.clone()));
                }
            });
        });
    });
    ui.add_space(6.0);
}

fn status_label(status: TransferStatus) -> &'static str {
    match status {
        TransferStatus::Waiting => tr("等待中", "Waiting"),
        TransferStatus::Transferring => tr("传输中", "Transferring"),
        TransferStatus::Finished => tr("已完成", "Completed"),
        TransferStatus::Failed => tr("失败", "Failed"),
        TransferStatus::Cancelled => tr("已取消", "Cancelled"),
    }
}

fn theme_label(value: &str) -> &'static str {
    match value {
        "light" => tr("浅色", "Light"),
        "dark" => tr("深色", "Dark"),
        _ => tr("跟随系统", "System"),
    }
}

fn color_label(value: &str) -> &'static str {
    match value {
        "blue" => tr("蓝色", "Blue"),
        "green" => tr("绿色", "Green"),
        _ => tr("LocalSend 青绿", "LocalSend teal"),
    }
}

fn language_label(value: &str) -> &'static str {
    match value {
        "zh-CN" => tr("简体中文", "Simplified Chinese"),
        "en-US" => tr("English", "English"),
        _ => tr("跟随系统", "System"),
    }
}

fn human_size(bytes: u64) -> String {
    if bytes >= 1_073_741_824 {
        format!("{:.1} GiB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
        format!("{:.1} MiB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1_024 {
        format!("{:.1} KiB", bytes as f64 / 1_024.0)
    } else {
        format!("{bytes} B")
    }
}

fn local_ip_url(
    local: &localsend_rpc::DeviceView,
    server_port: Option<u16>,
    protocol: Protocol,
) -> String {
    let ip = local.device.ip.as_deref().unwrap_or("127.0.0.1");
    let port = server_port.unwrap_or(local.device.port);
    let scheme = if protocol == Protocol::Https {
        "https"
    } else {
        "http"
    };
    if ip.contains(':') {
        format!("{scheme}://[{ip}]:{port}/")
    } else {
        format!("{scheme}://{ip}:{port}/")
    }
}

fn device_visual_id(ip: &Option<String>) -> String {
    ip.as_deref()
        .and_then(|value| value.split('.').next_back())
        .filter(|value| !value.is_empty())
        .map(|value| format!("#{value}"))
        .unwrap_or_else(|| "#Offline".to_string())
}

fn parse_filter_list(value: &str) -> Option<Vec<String>> {
    let values = value
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    (!values.is_empty()).then_some(values)
}

fn default_download_dir() -> PathBuf {
    directories::UserDirs::new()
        .and_then(|dirs| dirs.download_dir().map(Path::to_path_buf))
        .unwrap_or_else(|| std::env::temp_dir().join("localsend-downloads"))
}

fn open_path(path: &Path) -> std::io::Result<()> {
    #[cfg(target_os = "macos")]
    let mut command = {
        let mut command = Command::new("open");
        command.arg(path);
        command
    };
    #[cfg(target_os = "windows")]
    let mut command = {
        let mut command = Command::new("explorer");
        command.arg(path);
        command
    };
    #[cfg(all(unix, not(target_os = "macos")))]
    let mut command = {
        let mut command = Command::new("xdg-open");
        command.arg(path);
        command
    };
    command.status().map(|_| ())
}

fn pick_files_for_e2e() -> Vec<PathBuf> {
    if std::env::var("LOCALSEND_ACCEPTANCE").ok().as_deref() == Some("1") {
        return acceptance_file_choices("LOCALSEND_E2E_FILE_PATHS", "LOCALSEND_E2E_FILE_PATH");
    }
    localsend_platform::with_native_file_dialog_cleanup(|| rfd::FileDialog::new().pick_files())
        .unwrap_or_default()
}

fn pick_folder_for_e2e() -> Option<PathBuf> {
    if std::env::var("LOCALSEND_ACCEPTANCE").ok().as_deref() == Some("1") {
        return acceptance_file_choices("LOCALSEND_E2E_FOLDER_PATH", "LOCALSEND_E2E_FOLDER_PATH")
            .into_iter()
            .next();
    }
    localsend_platform::with_native_file_dialog_cleanup(|| rfd::FileDialog::new().pick_folder())
}

fn acceptance_file_choices(primary: &str, fallback: &str) -> Vec<PathBuf> {
    static CONSUMED: OnceLock<std::sync::Mutex<HashSet<String>>> = OnceLock::new();
    let mut consumed = CONSUMED
        .get_or_init(|| std::sync::Mutex::new(HashSet::new()))
        .lock()
        .expect("acceptance choices mutex poisoned");
    if !consumed.insert(primary.to_string()) {
        return Vec::new();
    }
    let raw = std::env::var_os(primary)
        .or_else(|| std::env::var_os(fallback))
        .unwrap_or_default();
    raw.to_string_lossy()
        .lines()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .filter(|path| path.exists())
        .collect()
}

fn install_cjk_font(context: &Context) {
    let candidates = [
        "/System/Library/Fonts/PingFang.ttc",
        "/System/Library/Fonts/Hiragino Sans GB.ttc",
        r"C:\Windows\Fonts\msyh.ttc",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
    ];
    for candidate in candidates {
        let Ok(bytes) = fs::read(candidate) else {
            continue;
        };
        let mut fonts = FontDefinitions::default();
        fonts
            .font_data
            .insert("cjk".to_string(), FontData::from_owned(bytes).into());
        for family in [FontFamily::Proportional, FontFamily::Monospace] {
            fonts
                .families
                .entry(family)
                .or_default()
                .insert(0, "cjk".to_string());
        }
        context.set_fonts(fonts);
        return;
    }
}

fn main() -> anyhow::Result<()> {
    let identity = localsend_rpc::transport::endpoint_path()
        .to_string_lossy()
        .into_owned();
    let lock_name = localsend_platform::scoped_lock_name("localsend-gui-egui", &identity);
    let Some(_instance_lock) = localsend_platform::InstanceLock::try_acquire(&lock_name)? else {
        eprintln!("LocalSend egui GUI is already running");
        return Ok(());
    };
    let _gui_pid = localsend_platform::GuiPidGuard::acquire()?;
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title("LocalSend PC")
            .with_inner_size([980.0, 700.0])
            .with_min_inner_size([760.0, 520.0]),
        ..Default::default()
    };
    eframe::run_native(
        "LocalSend PC",
        options,
        Box::new(|context| Ok(Box::new(DesktopApp::new(context)))),
    )
    .map_err(|error| anyhow::anyhow!(error.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{DaemonWorker, DesktopApp, SendMode, Tab, install_cjk_font};
    use eframe::egui::accesskit::Role;
    use egui_kittest::{Harness, kittest::Queryable};
    use localsend_testkit::{
        DaemonFixture, PeerAcceptance, PeerFile, ProtocolType, ReceiverPeer, SenderPeer,
        daemon_binary_for_tests, default_daemon_settings,
    };
    use std::time::Duration;

    #[test]
    fn egui_controls_cover_send_modes_webshare_and_settings() -> anyhow::Result<()> {
        let runtime = tokio::runtime::Runtime::new()?;
        let mut daemon = runtime.block_on(DaemonFixture::start(
            &daemon_binary_for_tests()?,
            default_daemon_settings(),
        ))?;
        let worker = DaemonWorker::spawn_with_endpoint(daemon.endpoint().to_string_lossy());
        let app = DesktopApp::with_worker(worker);
        let mut harness = Harness::builder()
            .allow_missing_glyphs()
            .with_context_setup(install_cjk_font)
            .build_ui_state(
                |ui, app: &mut DesktopApp| {
                    app.tick();
                    app.render(ui);
                },
                app,
            );
        for _ in 0..20 {
            harness.run_steps(1);
            std::thread::sleep(Duration::from_millis(50));
        }

        harness.get_by_label("发送").click();
        harness.run_steps(2);
        harness.get_by_label("多选").click();
        harness.run_steps(2);
        assert_eq!(harness.state().send_mode, SendMode::Multiple);
        assert!(harness.query_by_label("发送到所选 (0)").is_some());

        harness.get_by_label("文本").click();
        harness.run_steps(2);
        assert!(harness.query_by_label("发送文本").is_some());
        harness.get_by_label("取消").click();
        harness.run_steps(2);

        harness.get_by_label("接收").click();
        harness.run_steps(2);
        assert_eq!(harness.state().tab, Tab::Receive);
        harness
            .get_by_role_and_label(Role::Button, "通过链接接收")
            .click();
        harness.run_steps(2);
        // AccessKit click synthesis does not always release inside a centered
        // layout in kittest; seed the modal and continue testing its controls.
        harness.state_mut().web_share_dialog = true;
        harness.run_steps(2);
        assert!(harness.query_by_label("开始分享").is_some());
        assert!(harness.query_by_label("启用 PIN 密码").is_some());
        assert!(harness.query_by_label("自动接受上传").is_some());
        harness.get_by_label("开始分享").click();
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        loop {
            harness.run_steps(1);
            let running = harness.state().snapshot.as_ref().is_some_and(|snapshot| {
                snapshot
                    .devices
                    .iter()
                    .any(|device| device.source == "local" && device.device.download)
            });
            if running {
                break;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "web share did not start"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
        harness.get_by_label("关闭").click();
        harness.run_steps(2);

        harness.get_by_label("网络信息").click();
        harness.run_steps(2);
        assert!(harness.query_by_label_contains("指纹：").is_some());
        harness.get_by_label("关闭").click();
        harness.run_steps(2);

        harness.get_by_label("设置").click();
        harness.run_steps(2);
        for label in [
            "保存设置",
            "刷新设置",
            "语言",
            "历史记录上限",
            "启动时开启服务器",
        ] {
            assert!(harness.query_by_label(label).is_some(), "missing {label}");
        }

        drop(harness);
        runtime.block_on(daemon.shutdown())?;
        Ok(())
    }

    #[test]
    fn egui_english_language_updates_primary_surfaces_and_dialogs() -> anyhow::Result<()> {
        let runtime = tokio::runtime::Runtime::new()?;
        let mut settings = default_daemon_settings();
        settings.language = "en-US".to_string();
        let mut daemon =
            runtime.block_on(DaemonFixture::start(&daemon_binary_for_tests()?, settings))?;
        let worker = DaemonWorker::spawn_with_endpoint(daemon.endpoint().to_string_lossy());
        let app = DesktopApp::with_worker(worker);
        let mut harness = Harness::builder()
            .allow_missing_glyphs()
            .with_context_setup(install_cjk_font)
            .build_ui_state(
                |ui, app: &mut DesktopApp| {
                    app.tick();
                    app.render(ui);
                },
                app,
            );
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        while harness.state().snapshot.is_none() {
            harness.run_steps(1);
            assert!(
                std::time::Instant::now() < deadline,
                "English snapshot timed out"
            );
            std::thread::sleep(Duration::from_millis(50));
        }

        harness.get_by_label("Send").click();
        harness.run_steps(2);
        for label in [
            "Select",
            "File",
            "Folder",
            "Text",
            "Clipboard",
            "Devices",
            "Manual IP",
            "Single",
            "Multiple",
        ] {
            assert!(harness.query_by_label(label).is_some(), "missing {label}");
        }
        harness.get_by_label("Text").click();
        harness.run_steps(2);
        assert!(harness.query_by_label("Send text").is_some());
        assert!(harness.query_by_label("Message").is_some());
        harness.get_by_label("Cancel").click();
        harness.run_steps(2);

        harness.get_by_label("Receive").click();
        harness.run_steps(2);
        harness
            .get_by_role_and_label(Role::Button, "Receive via link")
            .click();
        harness.state_mut().web_share_dialog = true;
        harness.run_steps(2);
        for label in ["Start sharing", "Require a PIN", "Auto accept uploads"] {
            assert!(harness.query_by_label(label).is_some(), "missing {label}");
        }
        harness.get_by_label("Close").click();
        harness.run_steps(2);

        harness.get_by_label("Settings").click();
        harness.run_steps(2);
        for label in [
            "Save settings",
            "Reload settings",
            "Device name",
            "Theme",
            "Language",
            "Close to tray",
        ] {
            assert!(harness.query_by_label(label).is_some(), "missing {label}");
        }

        drop(harness);
        runtime.block_on(daemon.shutdown())?;
        Ok(())
    }

    #[test]
    fn egui_snapshot_revision_conflict_keeps_unsaved_draft() -> anyhow::Result<()> {
        let runtime = tokio::runtime::Runtime::new()?;
        let mut daemon = runtime.block_on(DaemonFixture::start(
            &daemon_binary_for_tests()?,
            default_daemon_settings(),
        ))?;
        let worker = DaemonWorker::spawn_with_endpoint(daemon.endpoint().to_string_lossy());
        let mut app = DesktopApp::with_worker(worker);
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        while app.snapshot.is_none() {
            app.tick();
            assert!(
                std::time::Instant::now() < deadline,
                "snapshot did not arrive"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
        let original = app.snapshot.clone().expect("snapshot");
        let mut draft = original.settings.clone();
        draft.alias = "unsaved alias".to_string();
        app.settings_draft = Some(draft);
        app.settings_dirty = true;

        let mut changed = original;
        changed.settings.revision = changed.settings.revision.saturating_add(1);
        changed.settings.alias = "other frontend alias".to_string();
        app.apply_snapshot(changed);

        assert!(app.settings_conflict);
        assert_eq!(
            app.settings_draft
                .as_ref()
                .map(|settings| settings.alias.as_str()),
            Some("unsaved alias")
        );
        runtime.block_on(daemon.shutdown())?;
        Ok(())
    }

    #[test]
    fn egui_favorite_can_be_added_and_renamed_through_controls() -> anyhow::Result<()> {
        let runtime = tokio::runtime::Runtime::new()?;
        let mut daemon = runtime.block_on(DaemonFixture::start(
            &daemon_binary_for_tests()?,
            default_daemon_settings(),
        ))?;
        let peer = runtime.block_on(ReceiverPeer::start(
            ProtocolType::Http,
            PeerAcceptance::All,
            None,
            None,
        ))?;
        let worker = DaemonWorker::spawn_with_endpoint(daemon.endpoint().to_string_lossy());
        let mut app = DesktopApp::with_worker(worker);
        app.dispatch(super::UiCommand::AddManual(format!(
            "127.0.0.1:{}",
            peer.port()
        )));
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        while app.snapshot.as_ref().is_none_or(|snapshot| {
            !snapshot
                .devices
                .iter()
                .any(|device| device.device.port == peer.port())
        }) {
            app.tick();
            assert!(
                std::time::Instant::now() < deadline,
                "peer discovery timed out"
            );
            std::thread::sleep(Duration::from_millis(50));
        }

        let mut harness = Harness::builder()
            .allow_missing_glyphs()
            .with_context_setup(install_cjk_font)
            .build_ui_state(
                |ui, app: &mut DesktopApp| {
                    app.tick();
                    app.render(ui);
                },
                app,
            );
        harness.run_steps(2);
        harness.get_by_label("发送").click();
        harness.run_steps(3);
        harness.get_by_label("收藏").click();
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        while harness
            .state()
            .snapshot
            .as_ref()
            .is_none_or(|snapshot| snapshot.favorites.is_empty())
        {
            harness.run_steps(1);
            assert!(
                std::time::Instant::now() < deadline,
                "favorite was not added"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
        harness.run_steps(3);
        assert!(harness.query_by_label("收藏夹").is_some());
        harness.get_by_label("重命名").click();
        harness.run_steps(2);
        if let Some((_, alias)) = harness.state_mut().favorite_edit.as_mut() {
            alias.clear();
        }
        harness.run_steps(1);
        let input = harness.get_by_label("收藏名称");
        input.focus();
        input.type_text("Desk favorite");
        harness.run_steps(2);
        harness.get_by_label("保存").click();
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        while harness.state().snapshot.as_ref().is_none_or(|snapshot| {
            snapshot.favorites.first().map(|fav| fav.alias.as_str()) != Some("Desk favorite")
        }) {
            harness.run_steps(1);
            assert!(
                std::time::Instant::now() < deadline,
                "favorite alias was not updated"
            );
            std::thread::sleep(Duration::from_millis(50));
        }

        drop(harness);
        runtime.block_on(peer.shutdown());
        runtime.block_on(daemon.shutdown())?;
        Ok(())
    }

    #[test]
    fn three_page_shell_and_settings_are_semantically_reachable() -> anyhow::Result<()> {
        let runtime = tokio::runtime::Runtime::new()?;
        let mut daemon = runtime.block_on(DaemonFixture::start(
            &daemon_binary_for_tests()?,
            default_daemon_settings(),
        ))?;
        let worker = DaemonWorker::spawn_with_endpoint(daemon.endpoint().to_string_lossy());
        let app = DesktopApp::with_worker(worker);
        let mut harness = Harness::builder()
            .allow_missing_glyphs()
            .with_context_setup(install_cjk_font)
            .build_ui_state(
                |ui, app: &mut DesktopApp| {
                    app.tick();
                    app.render(ui);
                },
                app,
            );
        std::thread::sleep(std::time::Duration::from_millis(300));
        harness.run_steps(5);

        harness.get_by_label("发送").click();
        harness.run_steps(3);
        assert!(harness.query_by_label("文件").is_some());
        assert!(harness.query_by_label("文件夹").is_some());
        assert!(harness.query_by_label("文本").is_some());

        harness.get_by_label("设置").click();
        harness.run_steps(3);
        assert!(harness.query_by_label("设备名称").is_some());
        assert!(harness.query_by_label("发送时创建校验和").is_some());
        assert!(harness.query_by_label("接收时验证校验和").is_some());

        drop(harness);
        runtime.block_on(daemon.shutdown())?;
        Ok(())
    }

    #[test]
    fn egui_worker_sends_text_and_file_through_daemon() -> anyhow::Result<()> {
        let runtime = tokio::runtime::Runtime::new()?;
        let mut daemon = runtime.block_on(DaemonFixture::start(
            &daemon_binary_for_tests()?,
            default_daemon_settings(),
        ))?;
        let peer = runtime.block_on(ReceiverPeer::start(
            ProtocolType::Http,
            PeerAcceptance::All,
            None,
            None,
        ))?;
        let worker = DaemonWorker::spawn_with_endpoint(daemon.endpoint().to_string_lossy());
        let mut app = DesktopApp::with_worker(worker);

        app.dispatch(super::UiCommand::AddManual(format!(
            "127.0.0.1:{}",
            peer.port()
        )));
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        let fingerprint = loop {
            app.tick();
            if let Some(fingerprint) = app.snapshot.as_ref().and_then(|snapshot| {
                snapshot
                    .devices
                    .iter()
                    .find(|device| device.device.port == peer.port())
                    .map(|device| device.device.fingerprint.clone())
            }) {
                break fingerprint;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "peer discovery timed out"
            );
            std::thread::sleep(Duration::from_millis(50));
        };

        let text = "egui daemon text";
        app.pending_text = Some(text.to_string());
        app.selected_paths.clear();
        app.send_to_device(fingerprint.clone());
        let received = runtime.block_on(peer.wait_for_files(1, Duration::from_secs(10)))?;
        assert_eq!(
            std::fs::read(received[0].path.as_ref().unwrap())?,
            text.as_bytes()
        );

        let source_dir = tempfile::TempDir::new()?;
        let source = source_dir.path().join("egui-file.txt");
        std::fs::write(&source, b"egui file bytes")?;
        app.selected_paths = vec![source];
        app.pending_text = None;
        app.send_to_device(fingerprint);
        let received = runtime.block_on(peer.wait_for_files(2, Duration::from_secs(10)))?;
        let file = received
            .iter()
            .find(|file| file.file_name == "egui-file.txt")
            .expect("received file");
        assert_eq!(
            std::fs::read(file.path.as_ref().unwrap())?,
            b"egui file bytes"
        );

        runtime.block_on(peer.shutdown());
        runtime.block_on(daemon.shutdown())?;
        Ok(())
    }

    #[test]
    fn egui_sender_surfaces_headless_peer_rejection() -> anyhow::Result<()> {
        let runtime = tokio::runtime::Runtime::new()?;
        let mut daemon = runtime.block_on(DaemonFixture::start(
            &daemon_binary_for_tests()?,
            default_daemon_settings(),
        ))?;
        let peer = runtime.block_on(ReceiverPeer::start(
            ProtocolType::Http,
            PeerAcceptance::Reject,
            None,
            None,
        ))?;
        let worker = DaemonWorker::spawn_with_endpoint(daemon.endpoint().to_string_lossy());
        let mut app = DesktopApp::with_worker(worker);
        app.dispatch(super::UiCommand::AddManual(format!(
            "127.0.0.1:{}",
            peer.port()
        )));
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        let fingerprint = loop {
            app.tick();
            if let Some(fingerprint) = app.snapshot.as_ref().and_then(|snapshot| {
                snapshot
                    .devices
                    .iter()
                    .find(|device| device.device.port == peer.port())
                    .map(|device| device.device.fingerprint.clone())
            }) {
                break fingerprint;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "reject peer discovery timed out"
            );
            std::thread::sleep(Duration::from_millis(50));
        };

        let source_dir = tempfile::TempDir::new()?;
        let source = source_dir.path().join("egui-rejected.txt");
        std::fs::write(&source, b"must not be accepted")?;
        app.selected_paths = vec![source];
        app.pending_text = None;
        app.send_to_device(fingerprint);
        let deadline = std::time::Instant::now() + Duration::from_secs(15);
        while !app.message.contains("传输失败") {
            app.tick();
            assert!(
                std::time::Instant::now() < deadline,
                "egui did not surface peer rejection"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
        app.tick();
        let snapshot = app.snapshot.as_ref().expect("snapshot after rejection");
        let transfer = snapshot
            .active_transfers
            .iter()
            .find(|transfer| transfer.peer_alias == "deterministic-peer")
            .expect("rejected transfer state");
        assert_eq!(transfer.status, super::TransferStatus::Failed);
        assert!(transfer.error.is_some());

        let status = runtime.block_on(peer.status());
        assert_eq!(status.requests.len(), 1);
        assert!(status.requests[0].accepted_file_ids.is_empty());
        assert!(status.received_files.is_empty());

        runtime.block_on(peer.shutdown());
        runtime.block_on(daemon.shutdown())?;
        Ok(())
    }

    #[cfg(target_os = "macos")]
    #[test]
    #[ignore = "requires a running official LocalSend app and Accessibility permission"]
    fn egui_official_app_background_rejects_headless_send() -> anyhow::Result<()> {
        let pid = std::env::var("LOCALSEND_OFFICIAL_PID").unwrap_or_else(|_| "2629".to_string());
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .expect("workspace root")
            .to_path_buf();
        let helper = root.join("target/tools/macos-ax-background");
        anyhow::ensure!(
            helper.exists(),
            "background AX helper is missing: {}",
            helper.display()
        );
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        let evidence = root
            .join("target/acceptance/egui/official-interop")
            .join(format!("{stamp}-background-reject"));
        std::fs::create_dir_all(&evidence)?;

        let _ = ax_background(&helper, &pid, &["press", "关闭", "AXButton"]);
        let _ = ax_background(&helper, &pid, &["press", "取消", "AXButton"]);
        let _ = ax_background(&helper, &pid, &["press", "接收", "AXStaticText"]);

        let runtime = tokio::runtime::Runtime::new()?;
        let mut daemon = runtime.block_on(DaemonFixture::start(
            &daemon_binary_for_tests()?,
            default_daemon_settings(),
        ))?;
        let worker = DaemonWorker::spawn_with_endpoint(daemon.endpoint().to_string_lossy());
        let mut app = DesktopApp::with_worker(worker);
        app.dispatch(super::UiCommand::AddManual("127.0.0.1:53317".to_string()));
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        let fingerprint = loop {
            app.tick();
            if let Some(fingerprint) = app.snapshot.as_ref().and_then(|snapshot| {
                snapshot
                    .devices
                    .iter()
                    .find(|device| device.device.port == 53317)
                    .map(|device| device.device.fingerprint.clone())
            }) {
                break fingerprint;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "official app manual peer timed out"
            );
            std::thread::sleep(Duration::from_millis(50));
        };

        let source_dir = tempfile::TempDir::new()?;
        let source = source_dir
            .path()
            .join(format!("official-background-reject-{stamp}.bin"));
        std::fs::write(&source, vec![0x42; 4096])?;
        app.selected_paths = vec![source.clone()];
        app.pending_text = None;
        app.selected_device = Some(fingerprint.clone());
        app.tab = Tab::Send;
        let mut harness = Harness::builder()
            .allow_missing_glyphs()
            .with_context_setup(install_cjk_font)
            .build_ui_state(
                |ui, app: &mut DesktopApp| {
                    app.tick();
                    app.render(ui);
                },
                app,
            );
        harness.run_steps(3);
        // Device selection and the send button are covered by the deterministic
        // peer suite. This case isolates the official-app request interaction.
        harness.state_mut().send_to_device(fingerprint);

        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        while !harness.state().pin_dialog {
            harness.run_steps(1);
            assert!(
                std::time::Instant::now() < deadline,
                "official app receive PIN dialog did not appear in egui"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
        harness.state_mut().pin_input = "2468".to_string();
        harness.run_steps(1);
        harness.get_by_role_and_label(Role::Button, "重试").click();
        harness.run_steps(2);

        let deadline = std::time::Instant::now() + Duration::from_secs(20);
        let mut rejected = false;
        while std::time::Instant::now() < deadline {
            harness.run_steps(1);
            let output = ax_background(&helper, &pid, &["dump", "拒绝", "AXButton"])?;
            std::fs::write(evidence.join("official-request.ax.txt"), &output)?;
            if output.contains("拒绝") {
                ax_background(&helper, &pid, &["press", "拒绝", "AXButton"])?;
                rejected = true;
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        anyhow::ensure!(rejected, "official app reject button did not appear");

        let deadline = std::time::Instant::now() + Duration::from_secs(15);
        loop {
            harness.run_steps(1);
            if harness.state().message.contains("传输失败") {
                break;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "egui did not surface the official rejection"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
        let downloads = directories::UserDirs::new()
            .and_then(|dirs| dirs.download_dir().map(std::path::Path::to_path_buf))
            .unwrap_or_else(|| std::path::PathBuf::from("/tmp"));
        assert!(
            !downloads
                .join(source.file_name().expect("source name"))
                .exists()
        );
        std::fs::write(
            evidence.join("summary.txt"),
            format!(
                "scenario=egui_official_app_background_reject\nresult=passed\n\
                 egui_message={}\nsource={}\nofficial_received=false\n",
                harness.state().message,
                source.display()
            ),
        )?;

        drop(harness);
        runtime.block_on(daemon.shutdown())?;
        Ok(())
    }

    #[cfg(target_os = "macos")]
    #[test]
    #[ignore = "requires a running official LocalSend app and Accessibility permission"]
    fn egui_official_app_background_cancels_pending_send() -> anyhow::Result<()> {
        let pid = std::env::var("LOCALSEND_OFFICIAL_PID").unwrap_or_else(|_| "2629".to_string());
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .expect("workspace root")
            .to_path_buf();
        let helper = root.join("target/tools/macos-ax-background");
        anyhow::ensure!(
            helper.exists(),
            "background AX helper is missing: {}",
            helper.display()
        );
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        let evidence = root
            .join("target/acceptance/egui/official-interop")
            .join(format!("{stamp}-background-cancel"));
        std::fs::create_dir_all(&evidence)?;

        let _ = ax_background(&helper, &pid, &["press", "关闭", "AXButton"]);
        let _ = ax_background(&helper, &pid, &["press", "取消", "AXButton"]);
        let _ = ax_background(&helper, &pid, &["press", "接收", "AXStaticText"]);

        let runtime = tokio::runtime::Runtime::new()?;
        let mut daemon = runtime.block_on(DaemonFixture::start(
            &daemon_binary_for_tests()?,
            default_daemon_settings(),
        ))?;
        let worker = DaemonWorker::spawn_with_endpoint(daemon.endpoint().to_string_lossy());
        let mut app = DesktopApp::with_worker(worker);
        app.dispatch(super::UiCommand::AddManual("127.0.0.1:53317".to_string()));
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        let fingerprint = loop {
            app.tick();
            if let Some(fingerprint) = app.snapshot.as_ref().and_then(|snapshot| {
                snapshot
                    .devices
                    .iter()
                    .find(|device| device.device.port == 53317)
                    .map(|device| device.device.fingerprint.clone())
            }) {
                break fingerprint;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "official app manual peer timed out"
            );
            std::thread::sleep(Duration::from_millis(50));
        };

        let source_dir = tempfile::TempDir::new()?;
        let source = source_dir
            .path()
            .join(format!("official-background-cancel-{stamp}.bin"));
        std::fs::write(&source, vec![0x43; 4096])?;
        app.selected_paths = vec![source.clone()];
        app.pending_text = None;
        app.selected_device = Some(fingerprint.clone());
        app.tab = Tab::Send;
        let mut harness = Harness::builder()
            .allow_missing_glyphs()
            .with_context_setup(install_cjk_font)
            .build_ui_state(
                |ui, app: &mut DesktopApp| {
                    app.tick();
                    app.render(ui);
                },
                app,
            );
        harness.run_steps(3);
        harness.state_mut().send_to_device(fingerprint);

        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        while !harness.state().pin_dialog {
            harness.run_steps(1);
            assert!(
                std::time::Instant::now() < deadline,
                "official app receive PIN dialog did not appear in egui"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
        harness.state_mut().pin_input = "2468".to_string();
        harness.run_steps(1);
        harness.get_by_role_and_label(Role::Button, "重试").click();
        harness.run_steps(2);

        let deadline = std::time::Instant::now() + Duration::from_secs(20);
        let mut request_seen = false;
        while std::time::Instant::now() < deadline {
            harness.run_steps(1);
            let output = ax_background(&helper, &pid, &["dump", "拒绝", "AXButton"])?;
            std::fs::write(evidence.join("official-request.ax.txt"), &output)?;
            if output.contains("拒绝") {
                request_seen = true;
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        anyhow::ensure!(request_seen, "official app request did not appear");
        harness.get_by_role_and_label(Role::Button, "取消").click();

        let deadline = std::time::Instant::now() + Duration::from_secs(15);
        loop {
            harness.run_steps(1);
            if harness.state().snapshot.as_ref().is_some_and(|snapshot| {
                snapshot.active_transfers.iter().any(|transfer| {
                    transfer.peer_alias == "Test macbook"
                        && transfer.status == super::TransferStatus::Cancelled
                })
            }) {
                break;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "egui did not mark the pending official transfer cancelled"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        let mut request_closed = false;
        while std::time::Instant::now() < deadline {
            let output = ax_background(&helper, &pid, &["dump", "拒绝", "AXButton"])?;
            std::fs::write(evidence.join("official-after-cancel.ax.txt"), &output)?;
            if !output.contains("拒绝") {
                request_closed = true;
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        anyhow::ensure!(
            request_closed,
            "official app request stayed open after cancel"
        );
        let downloads = directories::UserDirs::new()
            .and_then(|dirs| dirs.download_dir().map(std::path::Path::to_path_buf))
            .unwrap_or_else(|| std::path::PathBuf::from("/tmp"));
        assert!(
            !downloads
                .join(source.file_name().expect("source name"))
                .exists()
        );
        std::fs::write(
            evidence.join("summary.txt"),
            format!(
                "scenario=egui_official_app_background_cancel\nresult=passed\n\
                 source={}\nofficial_received=false\nrequest_closed=true\n",
                source.display()
            ),
        )?;

        drop(harness);
        runtime.block_on(daemon.shutdown())?;
        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn ax_background(
        helper: &std::path::Path,
        pid: &str,
        arguments: &[&str],
    ) -> anyhow::Result<String> {
        let output = std::process::Command::new(helper)
            .arg(pid)
            .args(arguments)
            .output()?;
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        if !output.status.success() {
            anyhow::bail!(
                "background AX failed: status={}; stdout={}; stderr={}",
                output.status,
                stdout,
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(stdout)
    }

    #[test]
    fn egui_sender_cancels_headless_slow_transfer_through_control() -> anyhow::Result<()> {
        let runtime = tokio::runtime::Runtime::new()?;
        let mut daemon = runtime.block_on(DaemonFixture::start(
            &daemon_binary_for_tests()?,
            default_daemon_settings(),
        ))?;
        let peer = runtime.block_on(ReceiverPeer::start(
            ProtocolType::Http,
            PeerAcceptance::All,
            None,
            Some(64 * 1024),
        ))?;
        let worker = DaemonWorker::spawn_with_endpoint(daemon.endpoint().to_string_lossy());
        let mut app = DesktopApp::with_worker(worker);
        app.dispatch(super::UiCommand::AddManual(format!(
            "127.0.0.1:{}",
            peer.port()
        )));
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        let fingerprint = loop {
            app.tick();
            if let Some(fingerprint) = app.snapshot.as_ref().and_then(|snapshot| {
                snapshot
                    .devices
                    .iter()
                    .find(|device| device.device.port == peer.port())
                    .map(|device| device.device.fingerprint.clone())
            }) {
                break fingerprint;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "slow peer discovery timed out"
            );
            std::thread::sleep(Duration::from_millis(50));
        };

        let source_dir = tempfile::TempDir::new()?;
        let source = source_dir.path().join("egui-slow-cancel.bin");
        std::fs::write(&source, vec![0x5a; 1024 * 1024])?;
        app.selected_paths = vec![source];
        app.pending_text = None;
        app.tab = Tab::Send;
        app.send_to_device(fingerprint);
        let deadline = std::time::Instant::now() + Duration::from_secs(15);
        while app.snapshot.as_ref().is_none_or(|snapshot| {
            !snapshot.active_transfers.iter().any(|transfer| {
                transfer.peer_alias == "deterministic-peer"
                    && transfer.status == super::TransferStatus::Transferring
            })
        }) {
            app.tick();
            assert!(
                std::time::Instant::now() < deadline,
                "slow transfer did not become active"
            );
            std::thread::sleep(Duration::from_millis(50));
        }

        let mut harness = Harness::builder()
            .allow_missing_glyphs()
            .with_context_setup(install_cjk_font)
            .build_ui_state(
                |ui, app: &mut DesktopApp| {
                    app.tick();
                    app.render(ui);
                },
                app,
            );
        harness.run_steps(2);
        harness.get_by_role_and_label(Role::Button, "取消").click();
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        loop {
            harness.run_steps(1);
            if harness.state().snapshot.as_ref().is_some_and(|snapshot| {
                snapshot.active_transfers.iter().any(|transfer| {
                    transfer.peer_alias == "deterministic-peer"
                        && transfer.status == super::TransferStatus::Cancelled
                })
            }) {
                break;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "egui cancel control did not mark the transfer cancelled"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
        let status = runtime.block_on(peer.status());
        assert_eq!(status.requests.len(), 1);
        assert!(
            status
                .received_files
                .iter()
                .all(|file| file.size > 0 && file.path.as_ref().is_none_or(|path| path.exists()))
        );

        drop(harness);
        runtime.block_on(peer.shutdown());
        runtime.block_on(daemon.shutdown())?;
        Ok(())
    }

    #[test]
    fn egui_receive_reducer_accepts_incoming_file() -> anyhow::Result<()> {
        let runtime = tokio::runtime::Runtime::new()?;
        let mut daemon = runtime.block_on(DaemonFixture::start(
            &daemon_binary_for_tests()?,
            default_daemon_settings(),
        ))?;
        let worker = DaemonWorker::spawn_with_endpoint(daemon.endpoint().to_string_lossy());
        let mut app = DesktopApp::with_worker(worker);
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        let port = loop {
            app.tick();
            if let Some(port) = app
                .snapshot
                .as_ref()
                .filter(|snapshot| snapshot.server_running)
                .and_then(|snapshot| snapshot.server_port)
            {
                break port;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "server startup timed out"
            );
            std::thread::sleep(Duration::from_millis(50));
        };

        let source_dir = tempfile::TempDir::new()?;
        let source = source_dir.path().join("egui-incoming.txt");
        std::fs::write(&source, b"incoming through egui")?;
        let sender = runtime.spawn(SenderPeer::send_files(
            "127.0.0.1",
            port,
            ProtocolType::Http,
            None,
            vec![PeerFile {
                path: source,
                remote_name: "egui-incoming.txt".to_string(),
            }],
        ));
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        loop {
            app.tick();
            if app
                .snapshot
                .as_ref()
                .is_some_and(|snapshot| !snapshot.incoming_requests.is_empty())
            {
                break;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "incoming request timed out"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
        app.respond(crate::ReceiverDecision::AcceptAll);
        runtime.block_on(sender)??;
        let destination = daemon.destination().join("egui-incoming.txt");
        assert_eq!(std::fs::read(destination)?, b"incoming through egui");

        runtime.block_on(daemon.shutdown())?;
        Ok(())
    }

    #[test]
    fn egui_partial_accept_and_reject_are_driven_through_ui() -> anyhow::Result<()> {
        let runtime = tokio::runtime::Runtime::new()?;
        let mut daemon = runtime.block_on(DaemonFixture::start(
            &daemon_binary_for_tests()?,
            default_daemon_settings(),
        ))?;
        let worker = DaemonWorker::spawn_with_endpoint(daemon.endpoint().to_string_lossy());
        let app = DesktopApp::with_worker(worker);
        let mut harness = Harness::builder()
            .allow_missing_glyphs()
            .with_context_setup(install_cjk_font)
            .build_ui_state(
                |ui, app: &mut DesktopApp| {
                    app.tick();
                    app.render(ui);
                },
                app,
            );
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        let port = loop {
            harness.run_steps(1);
            if let Some(port) = harness
                .state()
                .snapshot
                .as_ref()
                .filter(|snapshot| snapshot.server_running)
                .and_then(|snapshot| snapshot.server_port)
            {
                break port;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "server startup timed out"
            );
            std::thread::sleep(Duration::from_millis(50));
        };

        let source = tempfile::TempDir::new()?;
        let alpha = source.path().join("partial-a.txt");
        let beta = source.path().join("partial-b.txt");
        std::fs::write(&alpha, b"alpha")?;
        std::fs::write(&beta, b"beta")?;
        let sender = runtime.spawn(SenderPeer::send_files(
            "127.0.0.1",
            port,
            ProtocolType::Http,
            None,
            vec![
                PeerFile {
                    path: alpha,
                    remote_name: "partial-a.txt".to_string(),
                },
                PeerFile {
                    path: beta,
                    remote_name: "partial-b.txt".to_string(),
                },
            ],
        ));
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        while harness
            .state()
            .snapshot
            .as_ref()
            .is_none_or(|snapshot| snapshot.incoming_requests.is_empty())
        {
            harness.run_steps(1);
            assert!(
                std::time::Instant::now() < deadline,
                "partial request timed out"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
        harness
            .get_by_role_and_label(Role::CheckBox, "partial-b.txt · 4 B")
            .click();
        harness.run_steps(2);
        harness
            .get_by_role_and_label(Role::Button, "接受所选")
            .click();
        harness.run_steps(2);
        runtime.block_on(sender)??;
        assert_eq!(
            std::fs::read(daemon.destination().join("partial-a.txt"))?,
            b"alpha"
        );
        assert!(!daemon.destination().join("partial-b.txt").exists());

        let reject = source.path().join("reject.txt");
        std::fs::write(&reject, b"reject")?;
        let rejected = runtime.spawn(SenderPeer::send_files(
            "127.0.0.1",
            port,
            ProtocolType::Http,
            None,
            vec![PeerFile {
                path: reject,
                remote_name: "reject.txt".to_string(),
            }],
        ));
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        while harness
            .state()
            .snapshot
            .as_ref()
            .is_none_or(|snapshot| snapshot.incoming_requests.is_empty())
        {
            harness.run_steps(1);
            assert!(
                std::time::Instant::now() < deadline,
                "reject request timed out"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
        harness.get_by_role_and_label(Role::Button, "拒绝").click();
        harness.run_steps(2);
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        while harness
            .state()
            .snapshot
            .as_ref()
            .is_some_and(|snapshot| !snapshot.incoming_requests.is_empty())
        {
            harness.run_steps(1);
            assert!(
                std::time::Instant::now() < deadline,
                "reject request did not close"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
        // SenderPeer waits on its own 20s client timeout for a 403. The UI
        // contract here is that rejection closes the request locally; the
        // official App matrix separately asserts the wire-level 403.
        std::thread::sleep(Duration::from_millis(500));
        rejected.abort();
        let _ = runtime.block_on(rejected);
        assert!(!daemon.destination().join("reject.txt").exists());

        drop(harness);
        runtime.block_on(daemon.shutdown())?;
        Ok(())
    }

    #[test]
    fn egui_pin_dialog_retries_and_sends_file() -> anyhow::Result<()> {
        let runtime = tokio::runtime::Runtime::new()?;
        let mut daemon = runtime.block_on(DaemonFixture::start(
            &daemon_binary_for_tests()?,
            default_daemon_settings(),
        ))?;
        let peer = runtime.block_on(ReceiverPeer::start(
            ProtocolType::Http,
            PeerAcceptance::All,
            Some("2468".to_string()),
            None,
        ))?;
        let worker = DaemonWorker::spawn_with_endpoint(daemon.endpoint().to_string_lossy());
        let mut app = DesktopApp::with_worker(worker);
        app.dispatch(super::UiCommand::AddManual(format!(
            "127.0.0.1:{}",
            peer.port()
        )));
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        let fingerprint = loop {
            app.tick();
            if let Some(fingerprint) = app.snapshot.as_ref().and_then(|snapshot| {
                snapshot
                    .devices
                    .iter()
                    .find(|device| device.device.port == peer.port())
                    .map(|device| device.device.fingerprint.clone())
            }) {
                break fingerprint;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "PIN peer discovery timed out"
            );
            std::thread::sleep(Duration::from_millis(50));
        };

        let source = tempfile::TempDir::new()?;
        let outgoing = source.path().join("egui-pin-retry.txt");
        std::fs::write(&outgoing, b"egui pin retry")?;
        app.selected_paths = vec![outgoing];
        app.pending_text = None;
        app.send_to_device(fingerprint);
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        while !app.pin_dialog {
            app.tick();
            assert!(
                std::time::Instant::now() < deadline,
                "PIN dialog did not open"
            );
            std::thread::sleep(Duration::from_millis(50));
        }

        let mut harness = Harness::builder()
            .allow_missing_glyphs()
            .with_context_setup(install_cjk_font)
            .build_ui_state(
                |ui, app: &mut DesktopApp| {
                    app.tick();
                    app.render(ui);
                },
                app,
            );
        harness.run_steps(2);
        assert!(harness.query_by_label("需要 PIN").is_some());
        harness.state_mut().pin_input = "2468".to_string();
        harness.run_steps(1);
        harness.get_by_role_and_label(Role::Button, "重试").click();
        harness.run_steps(2);
        let received = runtime.block_on(peer.wait_for_files(1, Duration::from_secs(15)))?;
        assert_eq!(received[0].file_name, "egui-pin-retry.txt");
        assert_eq!(
            std::fs::read(received[0].path.as_ref().expect("received path"))?,
            b"egui pin retry"
        );

        drop(harness);
        runtime.block_on(peer.shutdown());
        runtime.block_on(daemon.shutdown())?;
        Ok(())
    }
}

use localsend_gui_slint::daemon_worker::{DaemonWorker, UiCommand, WorkerEvent};
use localsend_gui_slint::view_model;
use localsend_gui_slint::*;
use localsend_platform::{InstanceLock, local_ipv4_addresses, scoped_lock_name};
use localsend_rpc::transport;
use localsend_rpc::{
    AutoSaveMode, DEFAULT_MULTICAST_ADDRESS, DesktopSettings, Protocol, ReceiverDecision,
    ServiceSnapshot,
};
use serde::{Deserialize, Serialize};
use slint::{CloseRequestResponse, ComponentHandle, Model, ModelRc, VecModel, Weak};
use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::rc::Rc;
use std::sync::OnceLock;
use std::time::Duration;

static SYSTEM_DARK_THEME: OnceLock<bool> = OnceLock::new();

#[derive(Clone)]
struct SelectionState {
    paths: Rc<RefCell<Vec<PathBuf>>>,
    devices: Rc<RefCell<Vec<String>>>,
    selected_target: Rc<RefCell<Option<String>>>,
    request_files: Rc<RefCell<std::collections::HashSet<String>>>,
    send_mode: Rc<Cell<i32>>,
    last_history_id: Rc<RefCell<Option<String>>>,
}

impl SelectionState {
    fn new() -> Self {
        Self {
            paths: Rc::new(RefCell::new(Vec::new())),
            devices: Rc::new(RefCell::new(Vec::new())),
            selected_target: Rc::new(RefCell::new(None)),
            request_files: Rc::new(RefCell::new(std::collections::HashSet::new())),
            send_mode: Rc::new(Cell::new(0)),
            last_history_id: Rc::new(RefCell::new(None)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
struct WindowPlacement {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    maximized: bool,
}

impl WindowPlacement {
    fn is_valid(self) -> bool {
        (600..=4096).contains(&self.width)
            && (500..=4096).contains(&self.height)
            && (-100_000..=100_000).contains(&self.x)
            && (-100_000..=100_000).contains(&self.y)
    }

    fn capture(ui: &AppWindow) -> Self {
        let position = ui.window().position();
        let size = ui.window().size();
        Self {
            x: position.x,
            y: position.y,
            width: size.width,
            height: size.height,
            maximized: ui.window().is_maximized(),
        }
    }

    fn apply(self, ui: &AppWindow) {
        ui.window().set_maximized(false);
        ui.window()
            .set_size(slint::PhysicalSize::new(self.width, self.height));
        ui.window()
            .set_position(slint::PhysicalPosition::new(self.x, self.y));
        ui.window().set_maximized(self.maximized);
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
struct WindowState {
    always_on_top: bool,
    placement: Option<WindowPlacement>,
}

impl WindowState {
    fn read(path: &Path) -> Option<Self> {
        let mut state: Self = serde_json::from_slice(&fs::read(path).ok()?).ok()?;
        if state
            .placement
            .is_some_and(|placement| !placement.is_valid())
        {
            state.placement = None;
        }
        Some(state)
    }
}

struct WindowPersistence {
    path: Option<PathBuf>,
    enabled: bool,
    restored: bool,
    test_size_override: bool,
    state: WindowState,
    last_saved: Option<WindowState>,
    last_write: std::time::Instant,
}

impl WindowPersistence {
    fn new(test_size_override: bool) -> Self {
        let path = directories::ProjectDirs::from("org", "LocalSend", "localsend-rs")
            .map(|dirs| dirs.config_dir().join("window.json"));
        let state = path
            .as_deref()
            .and_then(WindowState::read)
            .unwrap_or_default();
        Self {
            path,
            enabled: false,
            restored: false,
            test_size_override,
            state,
            last_saved: Some(state),
            last_write: std::time::Instant::now(),
        }
    }

    fn always_on_top(&self) -> bool {
        self.state.always_on_top
    }

    fn set_always_on_top(&mut self, ui: &AppWindow, value: bool) {
        self.state.always_on_top = value;
        ui.set_settings_always_on_top(value);
        self.persist_if_changed(ui, true);
    }

    fn apply_setting(&mut self, ui: &AppWindow, enabled: bool) {
        self.enabled = enabled;
        if !enabled || self.restored {
            return;
        }
        self.restored = true;
        if self.test_size_override {
            return;
        }
        if let Some(placement) = self.state.placement {
            placement.apply(ui);
        }
    }

    fn persist_if_changed(&mut self, ui: &AppWindow, force: bool) {
        if !force && self.last_write.elapsed() < Duration::from_millis(500) {
            return;
        }
        if self.enabled {
            self.state.placement = Some(WindowPlacement::capture(ui));
        }
        if self.last_saved == Some(self.state) {
            return;
        }
        let Some(path) = self.path.as_deref() else {
            return;
        };
        let Some(parent) = path.parent() else {
            return;
        };
        if fs::create_dir_all(parent).is_err() {
            return;
        }
        let Ok(bytes) = serde_json::to_vec(&self.state) else {
            return;
        };
        if fs::write(path, bytes).is_ok() {
            self.last_saved = Some(self.state);
            self.last_write = std::time::Instant::now();
        }
    }
}

fn main() -> anyhow::Result<()> {
    let arguments: Vec<String> = std::env::args().collect();
    if !arguments.iter().any(|argument| argument == "--window") {
        launch_tray_window(arguments.iter().any(|argument| argument == "--minimized"))?;
        return Ok(());
    }
    let gui_lock_name = scoped_lock_name("localsend-gui", "gui");
    let Some(_gui_lock) = InstanceLock::try_acquire(&gui_lock_name)? else {
        eprintln!("LocalSend GUI is already running");
        return Ok(());
    };
    let worker = DaemonWorker::spawn();
    let commands = worker.commands;
    let worker_events = worker.events;
    let latest_snapshot = Rc::new(RefCell::new(None::<ServiceSnapshot>));
    let launch_setting = Rc::new(Cell::new(None::<bool>));
    let ui = AppWindow::new()?;
    if let Ok(tab) = std::env::var("LOCALSEND_GUI_TAB")
        && matches!(tab.as_str(), "receive" | "send" | "settings")
    {
        ui.set_selected_tab(tab.into());
    }
    let test_size_override = set_initial_size(&ui);
    ui.set_settings_terms_supported(cfg!(target_os = "macos"));
    open_initial_dialog(&ui);
    let weak = ui.as_weak();
    let window_persistence = Rc::new(RefCell::new(WindowPersistence::new(test_size_override)));
    ui.set_settings_always_on_top(window_persistence.borrow().always_on_top());
    if std::env::args().any(|argument| argument == "--minimized") {
        ui.window().set_minimized(true);
    }
    ui.window().on_close_requested({
        let weak = weak.clone();
        let commands = commands.clone();
        let latest_snapshot = latest_snapshot.clone();
        let window_persistence = window_persistence.clone();
        move || {
            if let Some(ui) = weak.upgrade() {
                let save_window_position = latest_snapshot
                    .borrow()
                    .as_ref()
                    .is_some_and(|snapshot| snapshot.settings.save_window_position);
                let mut persistence = window_persistence.borrow_mut();
                persistence.apply_setting(&ui, save_window_position);
                persistence.persist_if_changed(&ui, true);
            }
            let minimize_to_tray = latest_snapshot
                .borrow()
                .as_ref()
                .is_some_and(|snapshot| snapshot.settings.minimize_to_tray);
            if minimize_to_tray {
                let _ = slint::quit_event_loop();
                return CloseRequestResponse::HideWindow;
            }
            if latest_snapshot
                .borrow()
                .as_ref()
                .is_some_and(|snapshot| !snapshot.active_transfers.is_empty())
                && rfd::MessageDialog::new()
                    .set_title("Quit LocalSend")
                    .set_description("Active transfers will be cancelled. Quit LocalSend?")
                    .set_buttons(rfd::MessageButtons::YesNo)
                    .show()
                    != rfd::MessageDialogResult::Yes
            {
                return CloseRequestResponse::KeepWindowShown;
            }
            let _ = commands.send(UiCommand::ShutdownDaemon);
            let _ = slint::quit_event_loop();
            CloseRequestResponse::HideWindow
        }
    });

    let selection = SelectionState::new();

    ui.on_refresh({
        let commands = commands.clone();
        move || dispatch_command(&commands, UiCommand::RefreshDiscovery)
    });

    ui.on_toggle_server({
        let commands = commands.clone();
        move || dispatch_command(&commands, UiCommand::ToggleServer)
    });

    ui.on_set_auto_save({
        let commands = commands.clone();
        move |index| {
            let mode = match index {
                2 => AutoSaveMode::On,
                1 => AutoSaveMode::Favorites,
                _ => AutoSaveMode::Off,
            };
            dispatch_command(&commands, UiCommand::SetAutoSave(mode));
        }
    });

    ui.on_choose_file({
        let weak = weak.clone();
        let selection = selection.clone();
        move || {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                selection.paths.borrow_mut().push(path);
                update_selection(&weak, &selection);
            }
        }
    });

    ui.on_choose_folder({
        let weak = weak.clone();
        let selection = selection.clone();
        move || {
            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                selection.paths.borrow_mut().push(path);
                update_selection(&weak, &selection);
            }
        }
    });

    ui.on_clear_selection({
        let weak = weak.clone();
        let selection = selection.clone();
        move || {
            selection.paths.borrow_mut().clear();
            selection.devices.borrow_mut().clear();
            update_selection(&weak, &selection);
        }
    });

    ui.on_remove_selected({
        let weak = weak.clone();
        let selection = selection.clone();
        move |path| {
            let path = PathBuf::from(path.to_string());
            selection
                .paths
                .borrow_mut()
                .retain(|selected| selected != &path);
            update_selection(&weak, &selection);
        }
    });

    ui.on_send_text({
        let weak = weak.clone();
        let commands = commands.clone();
        let selection = selection.clone();
        move |text| {
            if text.trim().is_empty() {
                return;
            }
            if let Some(fingerprint) = selection.selected_target.borrow().clone() {
                dispatch_command(
                    &commands,
                    UiCommand::SendText {
                        fingerprint,
                        text: text.to_string(),
                    },
                );
            } else {
                update_message(&weak, "请先在设备列表中选择发送目标");
            }
            let _ = &selection;
        }
    });

    ui.on_send_clipboard({
        let weak = weak.clone();
        let commands = commands.clone();
        let selection = selection.clone();
        move || {
            let Ok(mut clipboard) = arboard::Clipboard::new() else {
                update_message(&weak, "无法访问剪贴板");
                return;
            };
            let Ok(text) = clipboard.get_text() else {
                update_message(&weak, "剪贴板中没有文本");
                return;
            };
            if text.trim().is_empty() {
                update_message(&weak, "剪贴板中没有文本");
                return;
            }
            if let Some(fingerprint) = selection.selected_target.borrow().clone() {
                dispatch_command(&commands, UiCommand::SendText { fingerprint, text });
            } else {
                update_message(&weak, "请先在设备列表中选择发送目标");
            }
        }
    });

    ui.on_add_manual({
        let weak = weak.clone();
        let commands = commands.clone();
        move |target| {
            dispatch_command(&commands, UiCommand::AddManual(target.to_string()));
            update_message(&weak, "正在添加设备");
        }
    });

    ui.on_toggle_favorite({
        let commands = commands.clone();
        move |fingerprint| {
            let fingerprint = fingerprint.to_string();
            dispatch_command(&commands, UiCommand::ToggleFavorite(fingerprint));
        }
    });

    ui.on_send_to_device({
        let weak = weak.clone();
        let commands = commands.clone();
        let selection = selection.clone();
        move |fingerprint| {
            let fingerprint = fingerprint.to_string();
            *selection.selected_target.borrow_mut() = Some(fingerprint.clone());
            if selection.send_mode.get() == 1 {
                let mut devices = selection.devices.borrow_mut();
                if let Some(index) = devices.iter().position(|item| item == &fingerprint) {
                    devices.remove(index);
                } else {
                    devices.push(fingerprint);
                }
                drop(devices);
                return;
            }
            if selection.paths.borrow().is_empty() {
                update_message(&weak, "已选择设备，请先选择文件或发送文本");
                return;
            }
            send_selected_paths_to(&weak, &commands, &selection, &fingerprint);
        }
    });

    ui.on_send_multi({
        let weak = weak.clone();
        let commands = commands.clone();
        let selection = selection.clone();
        move || {
            let devices = selection.devices.borrow().clone();
            if devices.is_empty() {
                update_message(&weak, "请先选择至少一个设备");
                return;
            }
            for fingerprint in &devices {
                send_selected_paths_to(&weak, &commands, &selection, fingerprint);
            }
            selection.devices.borrow_mut().clear();
            {
                if let Some(ui) = weak.upgrade() {
                    ui.set_send_mode_dialog_open(false);
                }
            }
        }
    });

    ui.on_set_send_mode({
        let weak = weak.clone();
        let selection = selection.clone();
        move |mode| {
            selection.send_mode.set(mode);
            selection.devices.borrow_mut().clear();
            if let Some(ui) = weak.upgrade() {
                ui.set_send_mode(mode);
            }
        }
    });

    ui.on_clear_history({
        let weak = weak.clone();
        let commands = commands.clone();
        move || {
            dispatch_command(&commands, UiCommand::ClearHistory);
            update_message(&weak, "正在清空历史");
        }
    });

    ui.on_dismiss_message_received({
        let weak = weak.clone();
        move || {
            if let Some(ui) = weak.upgrade() {
                ui.set_message_received_open(false);
            }
        }
    });

    ui.on_copy_message_received({
        let weak = weak.clone();
        move || {
            let Some(ui) = weak.upgrade() else {
                return;
            };
            let text = ui.get_message_received_text().to_string();
            match arboard::Clipboard::new().and_then(|mut clipboard| clipboard.set_text(text)) {
                Ok(()) => update_message(&weak, "消息已复制"),
                Err(error) => update_message(&weak, &format!("复制消息失败：{error}")),
            }
        }
    });

    ui.on_open_destination({
        let weak = weak.clone();
        let latest_snapshot = latest_snapshot.clone();
        move || {
            let Some(snapshot) = latest_snapshot.borrow().as_ref().cloned() else {
                return;
            };
            let destination = snapshot
                .settings
                .destination_dir
                .unwrap_or_else(default_download_dir);
            if let Err(error) = open_path(&destination) {
                update_message(&weak, &error);
            }
        }
    });

    ui.on_open_troubleshoot_firewall({
        let weak = weak.clone();
        move || {
            if let Err(error) = open_firewall_settings() {
                update_message(&weak, &error);
            }
        }
    });

    ui.on_cycle_setting({
        let weak = weak.clone();
        let commands = commands.clone();
        let latest_snapshot = latest_snapshot.clone();
        move |name| {
            let Some(snapshot) = latest_snapshot.borrow().as_ref().cloned() else {
                update_message(&weak, "等待 localsendd 连接");
                return;
            };
            let revision = snapshot.settings.revision;
            let mut settings = snapshot.settings;
            match name.as_str() {
                "theme" => {
                    settings.theme = match settings.theme.as_str() {
                        "system" => "dark".to_string(),
                        "dark" => "light".to_string(),
                        _ => "system".to_string(),
                    };
                }
                "color" => {
                    settings.color = match settings.color.as_str() {
                        "localsend" => "blue".to_string(),
                        "blue" => "green".to_string(),
                        _ => "localsend".to_string(),
                    };
                }
                "language" => {
                    settings.language = match settings.language.as_str() {
                        "system" => "zh-CN".to_string(),
                        "zh-CN" => "en".to_string(),
                        _ => "system".to_string(),
                    };
                }
                _ => return,
            }
            dispatch_settings(&weak, &commands, revision, settings);
        }
    });

    ui.on_set_toggle({
        let weak = weak.clone();
        let commands = commands.clone();
        let latest_snapshot = latest_snapshot.clone();
        let window_persistence = window_persistence.clone();
        move |name, value| {
            if name == "advanced" {
                if let Some(ui) = weak.upgrade() {
                    ui.set_settings_advanced(value);
                }
                return;
            }
            if name == "always_on_top" {
                if let Some(ui) = weak.upgrade() {
                    window_persistence
                        .borrow_mut()
                        .set_always_on_top(&ui, value);
                }
                return;
            }
            let Some(snapshot) = latest_snapshot.borrow().as_ref().cloned() else {
                update_message(&weak, "等待 localsendd 连接");
                return;
            };
            let revision = snapshot.settings.revision;
            let mut settings = snapshot.settings;
            match name.as_str() {
                "save_window_position" => settings.save_window_position = value,
                "minimize_to_tray" => settings.minimize_to_tray = value,
                "launch_on_login" => settings.launch_on_login = value,
                "animations" => settings.animations = value,
                "auto_finish" => settings.auto_finish = value,
                "save_to_history" => settings.save_to_history = value,
                "https" => {
                    settings.protocol = if value {
                        Protocol::Https
                    } else {
                        Protocol::Http
                    }
                }
                _ => {}
            }
            dispatch_settings(&weak, &commands, revision, settings);
        }
    });

    ui.on_respond_transfer({
        let weak = weak.clone();
        let commands = commands.clone();
        move |request_id, selected_ids, accept| {
            let decision = if !accept {
                ReceiverDecision::Decline
            } else if selected_ids.row_count() == 0 {
                ReceiverDecision::AcceptAll
            } else {
                ReceiverDecision::AcceptFiles {
                    file_ids: selected_ids
                        .iter()
                        .map(|file_id| file_id.to_string())
                        .collect(),
                }
            };
            dispatch_command(
                &commands,
                UiCommand::RespondTransfer {
                    request_id: request_id.to_string(),
                    decision,
                },
            );
            update_message(
                &weak,
                if accept {
                    "正在接受传输"
                } else {
                    "正在拒绝传输"
                },
            );
        }
    });

    ui.on_toggle_request_file({
        let selection = selection.clone();
        let latest_snapshot = latest_snapshot.clone();
        let weak = weak.clone();
        move |file_id| {
            let file_id = file_id.to_string();
            let mut selected = selection.request_files.borrow_mut();
            if !selected.remove(&file_id) {
                selected.insert(file_id);
            }
            drop(selected);
            if let Some(snapshot) = latest_snapshot.borrow().as_ref() {
                update_snapshot(&weak, snapshot, &selection);
            }
        }
    });

    ui.on_cancel_transfer({
        let weak = weak.clone();
        let commands = commands.clone();
        move |transfer_id| {
            dispatch_command(
                &commands,
                UiCommand::CancelTransfer(transfer_id.to_string()),
            );
            update_message(&weak, "正在取消传输");
        }
    });

    ui.on_retry_pin({
        let commands = commands.clone();
        move |transfer_id, pin| {
            dispatch_command(
                &commands,
                UiCommand::RetryPin {
                    transfer_id: transfer_id.to_string(),
                    pin: pin.to_string(),
                },
            );
        }
    });

    ui.on_dismiss_pin({
        let weak = weak.clone();
        move || {
            if let Some(ui) = weak.upgrade() {
                ui.set_pin_dialog_open(false);
                ui.set_pin_value("".into());
            }
        }
    });

    ui.on_open_web_share({
        let weak = weak.clone();
        move || {
            if let Some(ui) = weak.upgrade() {
                ui.set_web_share_dialog_open(true);
            }
        }
    });

    ui.on_start_web_share({
        let weak = weak.clone();
        let commands = commands.clone();
        move || {
            dispatch_command(&commands, UiCommand::StartWebReceive);
            update_message(&weak, "正在启动浏览器接收页面");
        }
    });

    ui.on_stop_web_share({
        let weak = weak.clone();
        let commands = commands.clone();
        move || {
            dispatch_command(&commands, UiCommand::StopWebShare);
            update_message(&weak, "正在关闭浏览器接收页面");
        }
    });

    ui.on_copy_web_share_url({
        let weak = weak.clone();
        let latest_snapshot = latest_snapshot.clone();
        move || {
            let url = latest_snapshot
                .borrow()
                .as_ref()
                .and_then(web_receive_url)
                .unwrap_or_default();
            if url.is_empty() {
                update_message(&weak, "当前没有可复制的接收链接");
                return;
            }
            match arboard::Clipboard::new().and_then(|mut clipboard| clipboard.set_text(url)) {
                Ok(()) => update_message(&weak, "接收链接已复制"),
                Err(error) => update_message(&weak, &format!("无法复制链接：{error}")),
            }
        }
    });

    ui.on_show_device_details({
        let weak = weak.clone();
        let latest_snapshot = latest_snapshot.clone();
        move |fingerprint| {
            let fingerprint = fingerprint.to_string();
            let Some(snapshot) = latest_snapshot.borrow().as_ref().cloned() else {
                return;
            };
            if let Some(device) = snapshot
                .devices
                .iter()
                .find(|device| device.device.fingerprint == fingerprint)
                && let Some(ui) = weak.upgrade()
            {
                ui.set_info_alias(device.device.alias.clone().into());
                ui.set_info_ip(
                    device
                        .device
                        .ip
                        .clone()
                        .unwrap_or_else(|| "未知".into())
                        .into(),
                );
                ui.set_info_port(device.device.port.to_string().into());
                ui.set_info_dialog_open(true);
            }
        }
    });

    ui.on_open_history_path({
        let weak = weak.clone();
        move |path| {
            if let Err(error) = open_path(&PathBuf::from(path.to_string())) {
                update_message(&weak, &format!("无法打开文件：{error}"));
            }
        }
    });

    ui.on_pick_destination({
        let weak = weak.clone();
        let latest_snapshot = latest_snapshot.clone();
        let commands = commands.clone();
        move || {
            let Some(directory) = rfd::FileDialog::new().pick_folder() else {
                return;
            };
            let Some(snapshot) = latest_snapshot.borrow().as_ref().cloned() else {
                update_message(&weak, "等待 localsendd 连接");
                return;
            };
            let mut settings = snapshot.settings.clone();
            settings.destination_dir = Some(directory);
            dispatch_settings(&weak, &commands, settings.revision, settings);
        }
    });

    ui.on_restart_server({
        let weak = weak.clone();
        let commands = commands.clone();
        move || {
            dispatch_command(&commands, UiCommand::RestartServer);
            update_message(&weak, "正在重启服务器");
        }
    });

    ui.on_edit_setting({
        let weak = weak.clone();
        let latest_snapshot = latest_snapshot.clone();
        let commands = commands.clone();
        move |key| {
            let key = key.to_string();
            let Some(snapshot) = latest_snapshot.borrow().as_ref().cloned() else {
                update_message(&weak, "等待 localsendd 连接");
                return;
            };
            if key == "pin-disable" {
                let mut settings = snapshot.settings.clone();
                settings.pin = None;
                dispatch_settings(&weak, &commands, settings.revision, settings);
                return;
            }
            let value = match key.as_str() {
                "alias" => snapshot.settings.alias.clone(),
                "port" => snapshot.settings.port.to_string(),
                "pin-enable" => String::new(),
                "pin" => snapshot.settings.pin.clone().unwrap_or_default(),
                "destination" => snapshot
                    .settings
                    .destination_dir
                    .as_ref()
                    .map_or_else(|| "-".to_string(), |path| path.display().to_string()),
                "discovery-timeout" => snapshot.settings.discovery_timeout_secs.to_string(),
                "multicast-group" => snapshot
                    .settings
                    .multicast_group
                    .clone()
                    .unwrap_or_default(),
                "network-filter" => format_network_filter(&snapshot.settings),
                _ => {
                    update_message(&weak, "此项由系统自动检测或当前平台不支持编辑");
                    return;
                }
            };
            if let Some(ui) = weak.upgrade() {
                ui.set_setting_edit_key(key.into());
                ui.set_setting_edit_value(value.into());
                ui.set_setting_dialog_open(true);
            }
        }
    });

    ui.on_cancel_setting({
        let weak = weak.clone();
        move || {
            if let Some(ui) = weak.upgrade() {
                ui.set_setting_dialog_open(false);
                ui.set_setting_edit_key("".into());
                ui.set_setting_edit_value("".into());
            }
        }
    });

    ui.on_submit_setting({
        let weak = weak.clone();
        let latest_snapshot = latest_snapshot.clone();
        let commands = commands.clone();
        move |key, value| {
            let Some(snapshot) = latest_snapshot.borrow().as_ref().cloned() else {
                update_message(&weak, "等待 localsendd 连接");
                return;
            };
            let key = key.to_string();
            let value = value.to_string();
            let mut settings = snapshot.settings.clone();
            let error = match key.as_str() {
                "alias" => {
                    settings.alias = value;
                    None
                }
                "port" => match value.parse::<u16>() {
                    Ok(port) if port > 0 => {
                        settings.port = port;
                        None
                    }
                    _ => Some("端口必须是 1 到 65535 之间的数字"),
                },
                "pin" | "pin-enable" => {
                    if value.trim().is_empty() {
                        Some("PIN 不能为空")
                    } else {
                        settings.pin = Some(value);
                        None
                    }
                }
                "destination" => {
                    settings.destination_dir = if value == "-" {
                        None
                    } else {
                        Some(PathBuf::from(value))
                    };
                    None
                }
                "discovery-timeout" => match value.parse::<u64>() {
                    Ok(timeout) if timeout > 0 => {
                        settings.discovery_timeout_secs = timeout;
                        None
                    }
                    _ => Some("发现超时必须是正整数"),
                },
                "multicast-group" => {
                    if value.trim().is_empty() || value == DEFAULT_MULTICAST_ADDRESS {
                        settings.multicast_group = None;
                        None
                    } else if value.parse::<std::net::Ipv4Addr>().is_ok() {
                        settings.multicast_group = Some(value);
                        None
                    } else {
                        Some("组播地址必须是有效 IPv4 地址")
                    }
                }
                "network-filter" => {
                    parse_network_filter(&value, &mut settings);
                    None
                }
                _ => Some("此设置不能通过文本输入修改"),
            };
            if let Some(error) = error {
                update_message(&weak, error);
                return;
            }
            dispatch_settings(&weak, &commands, settings.revision, settings);
            if let Some(ui) = weak.upgrade() {
                ui.set_setting_dialog_open(false);
                ui.set_setting_edit_key("".into());
                ui.set_setting_edit_value("".into());
            }
        }
    });

    ui.on_open_about({
        let weak = weak.clone();
        move || {
            if let Err(error) = open_url("https://localsend.org") {
                update_message(&weak, &error);
            }
        }
    });

    ui.on_open_support({
        let weak = weak.clone();
        move || {
            if let Err(error) = open_url("https://github.com/sponsors/Tienisto") {
                update_message(&weak, &error);
            }
        }
    });

    ui.on_open_privacy({
        let weak = weak.clone();
        move || {
            if let Err(error) = open_url("https://localsend.org/privacy") {
                update_message(&weak, &error);
            }
        }
    });

    ui.on_open_terms({
        let weak = weak.clone();
        move || {
            if let Err(error) =
                open_url("https://www.apple.com/legal/internet-services/itunes/dev/stdeula/")
            {
                update_message(&weak, &error);
            }
        }
    });

    ui.on_open_changelog({
        let weak = weak.clone();
        move || {
            if let Err(error) = open_url("https://github.com/localsend/localsend/releases") {
                update_message(&weak, &error);
            }
        }
    });

    let poll_timer = Box::leak(Box::new(slint::Timer::default()));
    poll_timer.start(slint::TimerMode::Repeated, Duration::from_millis(50), {
        let weak = weak.clone();
        let latest_snapshot = latest_snapshot.clone();
        let launch_setting = launch_setting.clone();
        let window_persistence = window_persistence.clone();
        let selection = selection.clone();
        let events = Rc::new(RefCell::new(worker_events));
        let events_for_timer = events.clone();
        move || {
            loop {
                let event = events_for_timer.borrow().try_recv();
                let Ok(event) = event else { break };
                match event {
                    WorkerEvent::Snapshot(snapshot) => {
                        if let Some(ui) = weak.upgrade() {
                            window_persistence
                                .borrow_mut()
                                .apply_setting(&ui, snapshot.settings.save_window_position);
                        }
                        update_snapshot(&weak, &snapshot, &selection);
                        let launch_on_login = snapshot.settings.launch_on_login;
                        if launch_setting.get() != Some(launch_on_login) {
                            if let Err(error) = apply_launch_on_login(launch_on_login) {
                                update_message(&weak, &error);
                            }
                            launch_setting.set(Some(launch_on_login));
                        }
                        *latest_snapshot.borrow_mut() = Some(*snapshot);
                    }
                    WorkerEvent::Notice(message) => update_message(&weak, &message),
                    WorkerEvent::TransferCreated { transfer_id, files } => {
                        update_message(
                            &weak,
                            &format!(
                                "已创建传输 {}",
                                transfer_id.chars().take(8).collect::<String>()
                            ),
                        );
                        if files {
                            selection.paths.borrow_mut().clear();
                            selection.devices.borrow_mut().clear();
                            update_selection(&weak, &selection);
                        }
                    }
                    WorkerEvent::Connection { connected, message } => {
                        if let Some(ui) = weak.upgrade() {
                            ui.set_server_status(message.clone().into());
                        }
                        if !connected {
                            update_message(&weak, &message);
                        }
                    }
                    WorkerEvent::TextReceived { sender_alias, text } => {
                        if let Some(ui) = weak.upgrade() {
                            ui.set_message_received_sender(sender_alias.clone().into());
                            ui.set_message_received_text(text.clone().into());
                            ui.set_message_received_open(true);
                            ui.set_last_received_title("收到新消息".into());
                            ui.set_last_received_body(format!("{sender_alias}：{text}").into());
                            ui.set_last_received_visible(true);
                        }
                        show_notification("收到新消息", &format!("{sender_alias}：{text}"));
                    }
                    WorkerEvent::PinRequired { transfer_id } => {
                        if let Some(ui) = weak.upgrade() {
                            ui.set_pin_transfer_id(transfer_id.into());
                            ui.set_pin_value("".into());
                            ui.set_pin_dialog_open(true);
                        }
                    }
                }
            }
            if let Some(ui) = weak.upgrade() {
                window_persistence
                    .borrow_mut()
                    .persist_if_changed(&ui, false);
            }
        }
    });

    if std::env::args().any(|argument| argument == "--minimized")
        && let Some(snapshot) = latest_snapshot.borrow().as_ref()
    {
        ui.global::<ThemeTokens>()
            .set_dark(resolve_dark_theme(&snapshot.settings.theme));
    }
    ui.run()?;
    Ok(())
}

fn set_initial_size(ui: &AppWindow) -> bool {
    let width = std::env::var("LOCALSEND_GUI_WIDTH")
        .ok()
        .and_then(|value| value.parse::<f32>().ok());
    let height = std::env::var("LOCALSEND_GUI_HEIGHT")
        .ok()
        .and_then(|value| value.parse::<f32>().ok());
    if let (Some(width), Some(height)) = (width, height) {
        ui.window().set_size(slint::LogicalSize::new(width, height));
        true
    } else {
        false
    }
}

fn open_initial_dialog(ui: &AppWindow) {
    let Ok(dialog) = std::env::var("LOCALSEND_GUI_DIALOG") else {
        return;
    };
    match dialog.as_str() {
        "history" => ui.set_history_dialog_open(true),
        "info" => ui.set_info_dialog_open(true),
        "manual" => ui.set_manual_dialog_open(true),
        "favorites" => ui.set_favorites_dialog_open(true),
        "send-mode" => ui.set_send_mode_dialog_open(true),
        "troubleshoot" => ui.set_troubleshoot_dialog_open(true),
        "text" => ui.set_text_dialog_open(true),
        _ => {}
    }
}

fn update_selection(weak: &Weak<AppWindow>, selection: &SelectionState) {
    let Some(ui) = weak.upgrade() else {
        return;
    };
    let paths = selection.paths.borrow();
    let mut file_total = 0_u64;
    let files = paths
        .iter()
        .map(|path| {
            let metadata = std::fs::symlink_metadata(path).ok();
            let is_directory = metadata.as_ref().is_some_and(|metadata| metadata.is_dir());
            if !is_directory {
                file_total = file_total
                    .saturating_add(metadata.as_ref().map_or(0, |metadata| metadata.len()));
            }
            SelectedFileItem {
                path: path.to_string_lossy().to_string().into(),
                name: path
                    .file_name()
                    .map(|name| name.to_string_lossy().to_string())
                    .unwrap_or_else(|| path.display().to_string())
                    .into(),
                is_directory,
                size: if is_directory {
                    "文件夹".to_string().into()
                } else {
                    view_model::format_bytes(metadata.as_ref().map_or(0, |metadata| metadata.len()))
                        .into()
                },
            }
        })
        .collect::<Vec<_>>();
    ui.set_selected_files(ModelRc::new(VecModel::from(files)));
    ui.set_has_selection(!paths.is_empty());
    ui.set_selected_file_count(paths.len().min(i32::MAX as usize) as i32);
    ui.set_selected_total_size(view_model::format_bytes(file_total).into());
    ui.set_selected_files_text(
        paths
            .iter()
            .take(3)
            .map(|path| {
                path.file_name()
                    .map(|name| name.to_string_lossy().to_string())
                    .unwrap_or_else(|| path.display().to_string())
            })
            .collect::<Vec<_>>()
            .join(" · ")
            .into(),
    );
}

fn send_selected_paths_to(
    weak: &Weak<AppWindow>,
    commands: &tokio::sync::mpsc::UnboundedSender<UiCommand>,
    selection: &SelectionState,
    fingerprint: &str,
) {
    let paths = selection.paths.borrow().clone();
    if paths.is_empty() {
        update_message(weak, "请先选择文件或文件夹");
        return;
    }
    if commands
        .send(UiCommand::SendFiles {
            fingerprint: fingerprint.to_string(),
            paths,
        })
        .is_ok()
    {
        update_message(weak, "已加入发送队列");
    } else {
        update_message(weak, "daemon worker 已关闭");
    }
}

fn dispatch_command(commands: &tokio::sync::mpsc::UnboundedSender<UiCommand>, command: UiCommand) {
    let _ = commands.send(command);
}

fn dispatch_settings(
    weak: &Weak<AppWindow>,
    commands: &tokio::sync::mpsc::UnboundedSender<UiCommand>,
    expected_revision: u64,
    settings: DesktopSettings,
) {
    if commands
        .send(UiCommand::UpdateSettings {
            expected_revision,
            settings: Box::new(settings),
        })
        .is_ok()
    {
        update_message(weak, "设置已提交");
    } else {
        update_message(weak, "daemon worker 已关闭");
    }
}

fn update_message(weak: &Weak<AppWindow>, message: &str) {
    if let Some(ui) = weak.upgrade() {
        ui.set_message(message.into());
    }
}

fn update_snapshot(weak: &Weak<AppWindow>, snapshot: &ServiceSnapshot, selection: &SelectionState) {
    let Some(ui) = weak.upgrade() else {
        return;
    };

    ui.global::<ThemeTokens>()
        .set_dark(resolve_dark_theme(&snapshot.settings.theme));
    ui.global::<ThemeTokens>()
        .set_accent_index(match snapshot.settings.color.as_str() {
            "blue" => 1,
            "green" => 2,
            _ => 0,
        });

    ui.set_device_alias(snapshot.settings.alias.clone().into());
    ui.set_device_id(local_ip_visual_ids(snapshot).into());
    ui.set_auto_save_index(match snapshot.settings.auto_save {
        AutoSaveMode::Off => 0,
        AutoSaveMode::Favorites => 1,
        AutoSaveMode::On => 2,
    });
    ui.set_settings_auto_finish(snapshot.settings.auto_finish);
    ui.set_settings_save_to_history(snapshot.settings.save_to_history);
    ui.set_device_count(
        snapshot
            .devices
            .iter()
            .filter(|device| device.source != "local")
            .count()
            .min(i32::MAX as usize) as i32,
    );
    ui.set_active_transfer_count(snapshot.active_transfers.len().min(i32::MAX as usize) as i32);
    ui.set_history_count(snapshot.history.len().min(i32::MAX as usize) as i32);
    ui.set_server_running(snapshot.server_running);
    ui.set_settings_theme(theme_label(&snapshot.settings.theme).into());
    ui.set_settings_color(color_label(&snapshot.settings.color).into());
    ui.set_settings_language(language_label(&snapshot.settings.language).into());
    ui.set_save_window_position(snapshot.settings.save_window_position);
    ui.set_minimize_to_tray(snapshot.settings.minimize_to_tray);
    ui.set_launch_on_login(snapshot.settings.launch_on_login);
    ui.set_animations_enabled(snapshot.settings.animations);
    let local_device = snapshot
        .devices
        .iter()
        .find(|device| device.source == "local");
    ui.set_settings_device_type(
        local_device
            .and_then(|device| device.device.device_type)
            .map(|device_type| format!("{device_type:?}"))
            .unwrap_or_else(|| "Desktop".to_string())
            .into(),
    );
    ui.set_settings_device_model(
        local_device
            .and_then(|device| device.device.device_model.clone())
            .unwrap_or_default()
            .into(),
    );
    ui.set_settings_discovery_timeout(snapshot.settings.discovery_timeout_secs.to_string().into());
    ui.set_settings_multicast_group(
        snapshot
            .settings
            .multicast_group
            .clone()
            .unwrap_or_else(|| DEFAULT_MULTICAST_ADDRESS.to_string())
            .into(),
    );
    ui.set_settings_network_filter(format_network_filter(&snapshot.settings).into());
    ui.set_settings_encryption(snapshot.settings.protocol == Protocol::Https);
    ui.set_settings_require_pin(snapshot.settings.pin.is_some());
    ui.set_web_share_running(local_device.is_some_and(|device| device.device.download));
    let web_share_url = local_device
        .and_then(|device| device.device.ip.as_deref())
        .and_then(|ip| {
            let port = snapshot.server_port?;
            let scheme = if snapshot.settings.protocol == Protocol::Https {
                "https"
            } else {
                "http"
            };
            if ip.contains(':') {
                Some(format!("{scheme}://[{ip}]:{port}/"))
            } else {
                Some(format!("{scheme}://{ip}:{port}/"))
            }
        })
        .unwrap_or_default();
    ui.set_web_share_url(web_share_url.into());
    ui.set_destination_dir(
        snapshot
            .settings
            .destination_dir
            .as_deref()
            .map(|path| path.display().to_string())
            .unwrap_or_default()
            .into(),
    );
    ui.set_info_alias(snapshot.settings.alias.clone().into());
    ui.set_info_ip(local_ip_summary(snapshot).into());
    ui.set_info_port(
        snapshot
            .server_port
            .unwrap_or(snapshot.settings.port)
            .to_string()
            .into(),
    );
    let target_alias = selection
        .selected_target
        .borrow()
        .as_deref()
        .and_then(|fingerprint| {
            snapshot
                .devices
                .iter()
                .find(|device| device.device.fingerprint == fingerprint)
                .map(|device| device.device.alias.clone())
                .or_else(|| {
                    snapshot
                        .favorites
                        .iter()
                        .find(|favorite| favorite.device.fingerprint == fingerprint)
                        .map(|favorite| favorite.alias.clone())
                })
        })
        .unwrap_or_else(|| "请选择目标设备".to_string());
    ui.set_selected_target_label(target_alias.into());
    if let Some(request) = snapshot.incoming_requests.first() {
        if selection.request_files.borrow().is_empty() {
            selection
                .request_files
                .borrow_mut()
                .extend(request.files.iter().map(|file| file.id.to_string()));
        }
        ui.set_incoming_request_count(1);
        ui.set_incoming_request_id(request.request_id.clone().into());
        ui.set_incoming_request_title(format!("{} 想发送文件", request.sender.alias).into());
        let rows = view_model::request_files(request, &selection.request_files.borrow());
        let selected = rows
            .iter()
            .filter(|file| file.selected)
            .map(|file| file.file_id.clone().into())
            .collect::<Vec<_>>();
        ui.set_selected_request_ids(ModelRc::new(VecModel::from(selected)));
        ui.set_incoming_files(ModelRc::new(VecModel::from(
            rows.into_iter()
                .map(|file| RequestFileItem {
                    file_id: file.file_id.into(),
                    name: file.name.into(),
                    size: file.size.into(),
                    selected: file.selected,
                })
                .collect::<Vec<_>>(),
        )));
        ui.set_incoming_request_body(format!("{} 个文件", request.files.len()).into());
    } else {
        ui.set_incoming_request_count(0);
        ui.set_incoming_request_id("".into());
        ui.set_incoming_request_title("".into());
        ui.set_incoming_request_body("".into());
        ui.set_incoming_files(ModelRc::new(VecModel::from(Vec::<RequestFileItem>::new())));
        ui.set_selected_request_ids(ModelRc::new(VecModel::from(
            Vec::<slint::SharedString>::new(),
        )));
        selection.request_files.borrow_mut().clear();
    }
    ui.set_transfer_items(ModelRc::new(VecModel::from(
        view_model::transfers(snapshot)
            .into_iter()
            .map(|transfer| TransferItem {
                transfer_id: transfer.transfer_id.into(),
                peer: transfer.peer.into(),
                summary: transfer.summary.into(),
                status: transfer.status.into(),
                direction: transfer.direction.into(),
                progress_percent: transfer.progress_percent,
                file_count: transfer.file_count,
            })
            .collect::<Vec<_>>(),
    )));
    ui.set_active_transfer_summary(
        snapshot
            .active_transfers
            .first()
            .map(|transfer| {
                format!(
                    "{} · {} / {}",
                    transfer.peer_alias,
                    format_bytes(transfer.bytes_done),
                    format_bytes(transfer.bytes_total)
                )
            })
            .unwrap_or_default()
            .into(),
    );
    if let Some(entry) = snapshot.history.last() {
        let previous = selection.last_history_id.borrow().clone();
        if previous.as_deref() != Some(entry.id.as_str()) {
            let is_new = previous.is_some();
            let (title, body) = if let Some(message) = entry.message.as_deref() {
                (
                    "收到新消息".to_string(),
                    format!("{}：{}", entry.peer_alias, message),
                )
            } else {
                (
                    "收到新文件".to_string(),
                    format!(
                        "{} · {} · {}",
                        entry.peer_alias,
                        entry.file_name,
                        format_bytes(entry.size)
                    ),
                )
            };
            if is_new && entry.message.is_none() {
                ui.set_last_received_title(title.clone().into());
                ui.set_last_received_body(body.clone().into());
                ui.set_last_received_visible(true);
                show_notification(&title, &body);
            }
            *selection.last_history_id.borrow_mut() = Some(entry.id.clone());
        }
    }
    ui.set_server_status(
        format!(
            "{} · 端口 {} · 传输 {} · 历史 {}{}",
            if snapshot.server_running {
                "服务器运行中"
            } else {
                "服务器已停止"
            },
            snapshot.server_port.unwrap_or(snapshot.settings.port),
            snapshot.active_transfers.len(),
            snapshot.history.len(),
            snapshot
                .last_error
                .as_deref()
                .map(|error| format!(" · {error}"))
                .unwrap_or_default()
        )
        .into(),
    );
    ui.set_devices(build_devices(snapshot, selection));
    ui.set_favorites(build_favorites(snapshot));
    ui.set_history_items(build_history(snapshot));
    ui.set_devices_text(devices_text(snapshot).into());
    update_selection(weak, selection);
}

fn resolve_dark_theme(theme: &str) -> bool {
    match theme {
        "dark" | "oled" => true,
        "light" | "yaru" => false,
        _ => system_prefers_dark(),
    }
}

fn system_prefers_dark() -> bool {
    *SYSTEM_DARK_THEME.get_or_init(system_prefers_dark_uncached)
}

fn system_prefers_dark_uncached() -> bool {
    #[cfg(target_os = "macos")]
    {
        Command::new("defaults")
            .args(["read", "-g", "AppleInterfaceStyle"])
            .output()
            .ok()
            .is_some_and(|output| String::from_utf8_lossy(&output.stdout).trim() == "Dark")
    }
    #[cfg(target_os = "windows")]
    {
        return Command::new("reg")
            .args([
                "query",
                r"HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
                "/v",
                "AppsUseLightTheme",
            ])
            .output()
            .ok()
            .is_some_and(|output| String::from_utf8_lossy(&output.stdout).contains("0x0"));
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        std::env::var("GTK_THEME")
            .ok()
            .is_some_and(|theme| theme.to_lowercase().contains("dark"))
    }
}

fn build_devices(snapshot: &ServiceSnapshot, selection: &SelectionState) -> ModelRc<DeviceItem> {
    let mut selected: HashSet<String> = selection.devices.borrow().iter().cloned().collect();
    if let Some(target) = selection.selected_target.borrow().as_ref() {
        selected.insert(target.clone());
    }
    let items: Vec<DeviceItem> = view_model::devices(snapshot, &selected)
        .into_iter()
        .map(|device| DeviceItem {
            alias: device.alias.into(),
            ip: device.ip.into(),
            fingerprint: device.fingerprint.into(),
            protocol: device.protocol.into(),
            model: device.model.into(),
            device_type: device.device_type.into(),
            favorite: device.favorite,
            selected: device.selected,
        })
        .collect();
    ModelRc::new(VecModel::from(items))
}

fn build_favorites(snapshot: &ServiceSnapshot) -> ModelRc<FavoriteItem> {
    let items: Vec<FavoriteItem> = view_model::favorites(snapshot)
        .into_iter()
        .map(|favorite| FavoriteItem {
            alias: favorite.alias.into(),
            fingerprint: favorite.fingerprint.into(),
        })
        .collect();
    ModelRc::new(VecModel::from(items))
}

fn build_history(snapshot: &ServiceSnapshot) -> ModelRc<HistoryItem> {
    let items: Vec<HistoryItem> = view_model::history(snapshot)
        .into_iter()
        .take(50)
        .map(|entry| HistoryItem {
            id: entry.id.into(),
            name: entry.name.into(),
            subtitle: entry.subtitle.into(),
            path: entry.path.into(),
            message: entry.message.into(),
            is_message: entry.is_message,
        })
        .collect();
    ModelRc::new(VecModel::from(items))
}

fn devices_text(snapshot: &ServiceSnapshot) -> String {
    snapshot
        .devices
        .iter()
        .filter(|device| device.source != "local")
        .map(|device| {
            format!(
                "{} · {}",
                device.device.alias,
                device.device.ip.as_deref().unwrap_or("?")
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn local_ip_summary(snapshot: &ServiceSnapshot) -> String {
    let addresses: Vec<String> = snapshot
        .devices
        .iter()
        .filter(|device| device.source == "local")
        .filter_map(|device| device.device.ip.clone())
        .collect();
    if addresses.is_empty() {
        "未知".to_string()
    } else {
        addresses.join(" ")
    }
}

fn local_ip_visual_ids(_snapshot: &ServiceSnapshot) -> String {
    visual_ids_from_ips(local_ipv4_addresses().unwrap_or_default())
}

fn visual_ids_from_ips(ips: Vec<std::net::Ipv4Addr>) -> String {
    let mut ids: Vec<_> = ips
        .into_iter()
        .map(|ip| ip.octets()[3])
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    ids.sort_unstable();
    if ids.is_empty() {
        "Offline".to_string()
    } else {
        ids.iter()
            .map(|id| format!("#{id}"))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn web_receive_url(snapshot: &ServiceSnapshot) -> Option<String> {
    if !snapshot.server_running {
        return None;
    }
    let ip = snapshot
        .devices
        .iter()
        .find(|device| device.source == "local")?
        .device
        .ip
        .as_deref()?;
    let port = snapshot.server_port?;
    let scheme = if snapshot.settings.protocol == Protocol::Https {
        "https"
    } else {
        "http"
    };
    Some(if ip.contains(':') {
        format!("{scheme}://[{ip}]:{port}/")
    } else {
        format!("{scheme}://{ip}:{port}/")
    })
}

fn format_network_filter(settings: &DesktopSettings) -> String {
    let allow = settings
        .interface_whitelist
        .as_ref()
        .map(|items| items.join(","))
        .unwrap_or_default();
    let deny = settings
        .interface_blacklist
        .as_ref()
        .map(|items| items.join(","))
        .unwrap_or_default();
    if allow.is_empty() && deny.is_empty() {
        "全部接口".to_string()
    } else {
        format!("allow={allow}; deny={deny}")
    }
}

fn parse_network_filter(value: &str, settings: &mut DesktopSettings) {
    settings.interface_whitelist = None;
    settings.interface_blacklist = None;
    for part in value
        .split(';')
        .map(str::trim)
        .filter(|part| !part.is_empty())
    {
        if let Some(patterns) = part.strip_prefix("allow=") {
            settings.interface_whitelist = parse_patterns(patterns);
        } else if let Some(patterns) = part.strip_prefix("deny=") {
            settings.interface_blacklist = parse_patterns(patterns);
        }
    }
}

fn parse_patterns(value: &str) -> Option<Vec<String>> {
    let patterns: Vec<String> = value
        .split([',', ' '])
        .map(str::trim)
        .filter(|pattern| !pattern.is_empty())
        .map(ToOwned::to_owned)
        .collect();
    (!patterns.is_empty()).then_some(patterns)
}

fn open_url(url: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    let mut command = Command::new("open");
    #[cfg(target_os = "windows")]
    let mut command = Command::new("rundll32");
    #[cfg(target_os = "windows")]
    command.args(["url.dll,FileProtocolHandler", url]);
    #[cfg(all(unix, not(target_os = "macos")))]
    let mut command = Command::new("xdg-open");
    #[cfg(any(target_os = "macos", all(unix, not(target_os = "macos"))))]
    command.arg(url);
    command
        .spawn()
        .map(|_| ())
        .map_err(|error| error.to_string())
}

fn default_download_dir() -> PathBuf {
    directories::UserDirs::new()
        .and_then(|dirs| dirs.download_dir().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."))
}

fn open_path(path: &PathBuf) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    let mut command = {
        let mut command = std::process::Command::new("open");
        command.arg(path);
        command
    };
    #[cfg(target_os = "windows")]
    let mut command = {
        let mut command = std::process::Command::new("explorer");
        command.arg(path);
        command
    };
    #[cfg(all(unix, not(target_os = "macos")))]
    let mut command = {
        let mut command = std::process::Command::new("xdg-open");
        command.arg(path);
        command
    };
    command
        .spawn()
        .map(|_| ())
        .map_err(|error| error.to_string())
}

fn open_firewall_settings() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Firewall")
            .spawn()
            .map(|_| ())
            .map_err(|error| error.to_string())
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("wf")
            .spawn()
            .map(|_| ())
            .map_err(|error| error.to_string())
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        Err("请在系统设置中检查防火墙规则".to_string())
    }
}

fn launch_tray_window(start_minimized: bool) -> anyhow::Result<()> {
    spawn_tray_process(&[])?;
    let mut args = vec!["--open-main".to_string()];
    if start_minimized {
        args.push("--minimized".to_string());
    }
    spawn_tray_process(&args)
}

fn spawn_tray_process(extra_args: &[String]) -> anyhow::Result<()> {
    let current = std::env::current_exe()?;
    let binary = if cfg!(windows) {
        "localsend-tray.exe"
    } else {
        "localsend-tray"
    };
    let executable = current
        .parent()
        .map(|parent| parent.join(binary))
        .into_iter()
        .chain(std::iter::once(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("target/debug")
                .join(binary),
        ))
        .find(|path| path.exists())
        .ok_or_else(|| anyhow::anyhow!("localsend-tray helper was not found next to the GUI"))?;

    let mut command = Command::new(executable);
    if let Some(endpoint) = transport::configured_endpoint() {
        command.arg("--endpoint").arg(endpoint);
    }
    command.args(extra_args);
    command
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        unsafe {
            command.pre_exec(|| {
                let _ = libc::setsid();
                Ok(())
            });
        }
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    command.spawn()?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn apply_launch_on_login(enabled: bool) -> Result<(), String> {
    const LABEL: &str = "org.localsend.pc.prototype";

    let home = std::env::var_os("HOME").ok_or_else(|| "HOME is not set".to_string())?;
    let launch_agents = PathBuf::from(home).join("Library/LaunchAgents");
    let plist_path = launch_agents.join(format!("{LABEL}.plist"));
    let uid = current_uid();
    let gui_target = format!("gui/{uid}");

    if !enabled {
        let _ = Command::new("launchctl")
            .args(["bootout", &format!("{gui_target}/{LABEL}")])
            .status();
        if plist_path.exists() {
            fs::remove_file(&plist_path).map_err(|error| error.to_string())?;
        }
        return Ok(());
    }

    fs::create_dir_all(&launch_agents).map_err(|error| error.to_string())?;
    let (program, mut arguments) = launch_program_and_arguments();
    arguments.push("--minimized".to_string());
    let mut program_arguments = String::new();
    for argument in std::iter::once(&program).chain(arguments.iter()) {
        program_arguments.push_str(&format!(
            "        <string>{}</string>\n",
            xml_escape(argument)
        ));
    }
    let plist = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>{LABEL}</string>
    <key>ProgramArguments</key>
    <array>
{program_arguments}    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <false/>
    <key>ProcessType</key>
    <string>Interactive</string>
</dict>
</plist>
"#
    );
    fs::write(&plist_path, plist).map_err(|error| error.to_string())?;

    let plist = plist_path.to_string_lossy().to_string();
    let status = Command::new("launchctl")
        .args(["bootstrap", &gui_target, &plist])
        .status()
        .map_err(|error| error.to_string())?;
    if !status.success() {
        let _ = Command::new("launchctl")
            .args(["load", "-w", &plist])
            .status();
    }
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn apply_launch_on_login(_enabled: bool) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "macos")]
fn launch_program_and_arguments() -> (String, Vec<String>) {
    let current = std::env::current_exe().ok();
    let bundle = current
        .as_ref()
        .and_then(|path| {
            path.ancestors().find(|ancestor| {
                ancestor
                    .extension()
                    .is_some_and(|extension| extension == "app")
            })
        })
        .map(|path| path.to_path_buf());
    if let Some(bundle) = bundle {
        (
            "/usr/bin/open".to_string(),
            vec![
                "-a".to_string(),
                bundle.to_string_lossy().to_string(),
                "--args".to_string(),
            ],
        )
    } else {
        (
            current
                .map(|path| path.to_string_lossy().to_string())
                .unwrap_or_else(|| "LocalSend".to_string()),
            Vec::new(),
        )
    }
}

#[cfg(target_os = "macos")]
fn current_uid() -> String {
    Command::new("id")
        .arg("-u")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|uid| uid.trim().to_string())
        .filter(|uid| !uid.is_empty())
        .unwrap_or_else(|| "501".to_string())
}

#[cfg(target_os = "macos")]
fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn theme_label(theme: &str) -> &'static str {
    match theme {
        "dark" => "深色",
        "light" => "浅色",
        _ => "跟随系统",
    }
}

fn color_label(color: &str) -> &'static str {
    match color {
        "blue" => "蓝色",
        "green" => "绿色",
        _ => "LocalSend",
    }
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit + 1 < UNITS.len() {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{bytes} {}", UNITS[unit])
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

fn show_notification(title: &str, body: &str) {
    #[cfg(target_os = "macos")]
    {
        let script = format!(
            "display notification \"{}\" with title \"{}\"",
            applescript_escape(body),
            applescript_escape(title)
        );
        let _ = Command::new("osascript").args(["-e", &script]).spawn();
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (title, body);
    }
}

#[cfg(target_os = "macos")]
fn applescript_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn language_label(language: &str) -> &'static str {
    match language {
        "en" | "en-US" => "English",
        "zh" | "zh-CN" => "简体中文",
        _ => "跟随系统",
    }
}

#[cfg(test)]
mod tests {
    use super::{WindowPlacement, WindowState, visual_ids_from_ips};
    use i_slint_backend_testing::{AccessibleRole, ElementQuery};
    use localsend_gui_slint::AppWindow;
    use slint::ComponentHandle;
    use std::fs;

    #[test]
    fn window_state_rejects_unreasonable_dimensions_and_coordinates() {
        let state = WindowPlacement {
            x: 0,
            y: 0,
            width: 600,
            height: 500,
            maximized: false,
        };
        assert!(state.is_valid());
        assert!(
            !WindowPlacement {
                width: 599,
                ..state
            }
            .is_valid()
        );
        assert!(
            !WindowPlacement {
                height: 499,
                ..state
            }
            .is_valid()
        );
        assert!(
            !WindowPlacement {
                x: 100_001,
                ..state
            }
            .is_valid()
        );
    }

    #[test]
    fn visual_ids_use_real_ipv4_last_octets() {
        let ips = vec![
            "192.168.31.80".parse().unwrap(),
            "10.0.0.7".parse().unwrap(),
            "192.168.31.80".parse().unwrap(),
        ];
        assert_eq!(visual_ids_from_ips(ips), "#7 #80");
        assert_eq!(visual_ids_from_ips(Vec::new()), "Offline");
    }

    #[test]
    fn window_state_round_trips_through_its_json_file() {
        let state = WindowState {
            always_on_top: true,
            placement: Some(WindowPlacement {
                x: 120,
                y: 80,
                width: 1440,
                height: 900,
                maximized: true,
            }),
        };
        let path = std::env::temp_dir().join(format!(
            "localsend-gui-window-state-{}.json",
            std::process::id()
        ));
        fs::write(&path, serde_json::to_vec(&state).unwrap()).unwrap();
        assert_eq!(WindowState::read(&path), Some(state));
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn window_state_keeps_local_window_preference_when_placement_is_invalid() {
        let state = WindowState {
            always_on_top: true,
            placement: Some(WindowPlacement {
                x: 0,
                y: 0,
                width: 400,
                height: 900,
                maximized: false,
            }),
        };
        let path = std::env::temp_dir().join(format!(
            "localsend-gui-window-state-invalid-{}.json",
            std::process::id()
        ));
        fs::write(&path, serde_json::to_vec(&state).unwrap()).unwrap();
        assert_eq!(
            WindowState::read(&path),
            Some(WindowState {
                always_on_top: true,
                placement: None,
            })
        );
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn accessible_navigation_and_primary_actions_are_semantic() {
        i_slint_backend_testing::init_integration_test_with_mock_time();
        let ui = AppWindow::new().expect("create window");
        ui.window()
            .set_size(slint::LogicalSize::new(1200.0, 1800.0));
        let choose_file = std::rc::Rc::new(std::cell::Cell::new(false));
        let toggles = std::rc::Rc::new(std::cell::RefCell::new(Vec::<(String, bool)>::new()));
        let submitted = std::rc::Rc::new(std::cell::RefCell::new(Vec::<(String, String)>::new()));
        ui.on_choose_file({
            let choose_file = choose_file.clone();
            move || choose_file.set(true)
        });
        ui.on_set_toggle({
            let toggles = toggles.clone();
            move |name, value| toggles.borrow_mut().push((name.to_string(), value))
        });
        ui.on_submit_setting({
            let submitted = submitted.clone();
            move |name, value| {
                submitted
                    .borrow_mut()
                    .push((name.to_string(), value.to_string()))
            }
        });
        ui.set_selected_tab("receive".into());
        ui.show().expect("show window");
        i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_millis(50));

        let send = ElementQuery::from_root(&ui)
            .match_accessible_role(AccessibleRole::Button)
            .match_predicate(|element| {
                element
                    .accessible_label()
                    .is_some_and(|label| label == "发送")
            })
            .find_first()
            .expect("send navigation button");
        send.invoke_accessible_default_action();
        assert_eq!(ui.get_selected_tab(), "send");
        i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_millis(50));

        let file = ElementQuery::from_root(&ui)
            .match_accessible_role(AccessibleRole::Button)
            .match_predicate(|element| {
                element
                    .accessible_label()
                    .is_some_and(|label| label == "文件")
            })
            .find_first()
            .expect("file button");
        file.invoke_accessible_default_action();
        assert!(choose_file.get());

        let settings = ElementQuery::from_root(&ui)
            .match_accessible_role(AccessibleRole::Button)
            .match_predicate(|element| {
                element
                    .accessible_label()
                    .is_some_and(|label| label == "设置")
            })
            .find_first()
            .expect("settings navigation button");
        settings.invoke_accessible_default_action();
        assert_eq!(ui.get_selected_tab(), "settings");
        i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_millis(50));

        let animation_switch = ElementQuery::from_root(&ui)
            .match_accessible_role(AccessibleRole::Switch)
            .match_predicate(|element| {
                element
                    .accessible_label()
                    .is_some_and(|label| label == "动画效果")
            })
            .find_first()
            .expect("animation switch");
        animation_switch.invoke_accessible_default_action();
        assert!(
            toggles
                .borrow()
                .iter()
                .any(|(name, _)| name == "animations")
        );

        let alias = ElementQuery::from_root(&ui)
            .match_accessible_role(AccessibleRole::TextInput)
            .match_predicate(|element| {
                element
                    .accessible_label()
                    .is_some_and(|label| label == "设备名称")
            })
            .find_first()
            .expect("alias input");
        alias.set_accessible_value("Accessible PC");
        assert!(
            submitted
                .borrow()
                .iter()
                .any(|(name, value)| name == "alias" && value == "Accessible PC")
        );
    }
}

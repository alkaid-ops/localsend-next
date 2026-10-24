use crate::clipboard_payload::{self, ClipboardPayload};
use crate::daemon_worker::{DaemonWorker, UiCommand, WorkerEvent};
use crate::view_model;
use crate::*;
use localsend_platform::{
    GuiAction, GuiControlServer, GuiPidGuard, InstanceLock, activate_current_application_window,
    apply_launch_on_login, local_ipv4_addresses, scoped_lock_name, send_gui_control_action,
    take_queued_gui_actions, with_native_file_dialog_cleanup,
};
use localsend_rpc::snapshot_store::{EventApply, SnapshotStore};
use localsend_rpc::transport;
use localsend_rpc::{
    AutoSaveMode, DEFAULT_MULTICAST_ADDRESS, DesktopSettings, Protocol, ReceiverDecision,
    ServiceSnapshot,
};
use serde::{Deserialize, Serialize};
use slint::{CloseRequestResponse, ComponentHandle, Model, ModelRc, VecModel, Weak};
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::rc::Rc;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

static SYSTEM_DARK_THEME: OnceLock<bool> = OnceLock::new();

thread_local! {
    static ACTIVE_LANGUAGE: RefCell<Option<&'static str>> = const { RefCell::new(None) };
}

#[derive(Clone)]
struct SelectionState {
    paths: Rc<RefCell<Vec<PathBuf>>>,
    devices: Rc<RefCell<Vec<String>>>,
    selected_target: Rc<RefCell<Option<String>>>,
    pending_text: Rc<RefCell<Option<String>>>,
    request_files: Rc<RefCell<std::collections::HashSet<String>>>,
    dismissed_transfers: Rc<RefCell<HashSet<String>>>,
    send_mode: Rc<Cell<i32>>,
    last_history_id: Rc<RefCell<Option<String>>>,
    last_send_at: Rc<RefCell<HashMap<String, Instant>>>,
}

impl SelectionState {
    fn new() -> Self {
        Self {
            paths: Rc::new(RefCell::new(Vec::new())),
            devices: Rc::new(RefCell::new(Vec::new())),
            selected_target: Rc::new(RefCell::new(None)),
            pending_text: Rc::new(RefCell::new(None)),
            request_files: Rc::new(RefCell::new(std::collections::HashSet::new())),
            dismissed_transfers: Rc::new(RefCell::new(HashSet::new())),
            send_mode: Rc::new(Cell::new(0)),
            last_history_id: Rc::new(RefCell::new(None)),
            last_send_at: Rc::new(RefCell::new(HashMap::new())),
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

    fn disabled(test_size_override: bool) -> Self {
        Self {
            path: None,
            enabled: false,
            restored: true,
            test_size_override,
            state: WindowState::default(),
            last_saved: Some(WindowState::default()),
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

#[derive(Debug, Clone)]
pub struct AppOptions {
    pub initial_tab: Option<String>,
    pub minimized: bool,
    pub restore_window: bool,
    pub initial_files: Vec<PathBuf>,
    pub initial_text: Option<String>,
}

impl AppOptions {
    pub fn from_process() -> Self {
        Self {
            initial_tab: std::env::var("LOCALSEND_GUI_TAB")
                .ok()
                .filter(|tab| matches!(tab.as_str(), "receive" | "send" | "settings")),
            minimized: std::env::args().any(|argument| argument == "--minimized"),
            restore_window: true,
            initial_files: Vec::new(),
            initial_text: None,
        }
    }

    pub fn for_test(initial_tab: &str) -> Self {
        Self {
            initial_tab: Some(initial_tab.to_string()),
            minimized: false,
            restore_window: false,
            initial_files: Vec::new(),
            initial_text: None,
        }
    }
}

pub fn run_main() -> anyhow::Result<()> {
    let arguments: Vec<String> = std::env::args().collect();
    if !arguments.iter().any(|argument| argument == "--window") {
        launch_tray_window(arguments.iter().any(|argument| argument == "--minimized"))?;
        #[cfg(target_os = "linux")]
        return Ok(());
    }
    let gui_identity = transport::endpoint_path().to_string_lossy().into_owned();
    let gui_lock_name = scoped_lock_name("localsend-gui", &gui_identity);
    let Some(_gui_lock) = InstanceLock::try_acquire(&gui_lock_name)? else {
        let _ = send_gui_control_action(&GuiAction::ShowMain);
        return Ok(());
    };
    let _gui_pid = GuiPidGuard::acquire()?;
    clipboard_payload::cleanup_old_images();
    let worker = DaemonWorker::spawn();
    let ui = build_app(worker.commands, worker.events, AppOptions::from_process())?;
    ui.run()?;
    Ok(())
}

pub fn build_app(
    commands: tokio::sync::mpsc::UnboundedSender<UiCommand>,
    worker_events: std::sync::mpsc::Receiver<WorkerEvent>,
    options: AppOptions,
) -> anyhow::Result<AppWindow> {
    let latest_snapshot = Rc::new(RefCell::new(None::<ServiceSnapshot>));
    let local_state = Rc::new(RefCell::new(None::<SnapshotStore>));
    let launch_setting = Rc::new(Cell::new(None::<bool>));
    let (gui_action_tx, gui_action_rx) = std::sync::mpsc::channel();
    let control_server = if options.restore_window {
        let server = GuiControlServer::start(gui_action_tx.clone())?;
        for action in take_queued_gui_actions()? {
            let _ = gui_action_tx.send(action);
        }
        Some(server)
    } else {
        None
    };
    let ui = AppWindow::new()?;
    ui.set_app_version(env!("CARGO_PKG_VERSION").into());
    if let Some(tab) = options.initial_tab.as_deref() {
        ui.set_selected_tab(tab.into());
    }
    let test_size_override = set_initial_size(&ui);
    ui.set_settings_terms_supported(cfg!(target_os = "macos"));
    ui.set_settings_daemon_shutdown_supported(cfg!(target_os = "linux"));
    open_initial_dialog(&ui);
    let weak = ui.as_weak();
    let window_persistence = Rc::new(RefCell::new(if options.restore_window {
        WindowPersistence::new(test_size_override)
    } else {
        WindowPersistence::disabled(test_size_override)
    }));
    ui.set_settings_always_on_top(window_persistence.borrow().always_on_top());
    if options.minimized {
        ui.window().set_minimized(true);
    }
    ui.window().on_close_requested({
        let weak = weak.clone();
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
            // GUI exit is independent from the tray-owned daemon lifecycle.
            let _ = slint::quit_event_loop();
            CloseRequestResponse::HideWindow
        }
    });

    let selection = SelectionState::new();
    add_selected_paths(&selection, options.initial_files.clone());
    *selection.pending_text.borrow_mut() = options.initial_text.clone();
    update_selection(&weak, &selection);

    ui.on_refresh({
        let commands = commands.clone();
        move || dispatch_command(&commands, UiCommand::RefreshDiscovery)
    });

    ui.on_toggle_server({
        let commands = commands.clone();
        move || dispatch_command(&commands, UiCommand::ToggleServer)
    });

    ui.on_shutdown_daemon({
        let weak = weak.clone();
        let commands = commands.clone();
        move || {
            dispatch_command(&commands, UiCommand::ShutdownDaemon);
            update_message(&weak, "正在关闭后台服务");
        }
    });

    ui.on_set_auto_save({
        let weak = weak.clone();
        let commands = commands.clone();
        move |all, favorites| {
            dispatch_command(&commands, UiCommand::SetAutoSave { all, favorites });
            if all && let Some(ui) = weak.upgrade() {
                ui.set_auto_save_help_dialog_open(true);
            }
        }
    });

    ui.on_close_auto_save_help({
        let weak = weak.clone();
        move || {
            if let Some(ui) = weak.upgrade() {
                ui.set_auto_save_help_dialog_open(false);
            }
        }
    });

    ui.on_choose_file({
        let weak = weak.clone();
        let commands = commands.clone();
        let selection = selection.clone();
        move || {
            let paths = pick_files_for_e2e();
            if !paths.is_empty() {
                add_selected_paths(&selection, paths);
                *selection.pending_text.borrow_mut() = None;
                update_selection(&weak, &selection);
                dispatch_command(&commands, UiCommand::RefreshDiscovery);
                update_message(&weak, "已选择文件，正在刷新附近设备");
            }
        }
    });

    ui.on_choose_folder({
        let weak = weak.clone();
        let commands = commands.clone();
        let selection = selection.clone();
        move || {
            if let Some(path) = pick_folder_for_e2e() {
                add_selected_paths(&selection, vec![path]);
                *selection.pending_text.borrow_mut() = None;
                update_selection(&weak, &selection);
                dispatch_command(&commands, UiCommand::RefreshDiscovery);
                update_message(&weak, "已选择文件夹，正在刷新附近设备");
            }
        }
    });

    ui.on_edit_selection({
        let weak = weak.clone();
        let selection = selection.clone();
        move || {
            let paths = pick_files_for_e2e();
            if paths.is_empty() {
                return;
            }
            selection.paths.borrow_mut().clear();
            *selection.pending_text.borrow_mut() = None;
            add_selected_paths(&selection, paths);
            update_selection(&weak, &selection);
        }
    });

    ui.on_clear_selection({
        let weak = weak.clone();
        let selection = selection.clone();
        move || {
            selection.paths.borrow_mut().clear();
            selection.devices.borrow_mut().clear();
            *selection.pending_text.borrow_mut() = None;
            update_selection(&weak, &selection);
        }
    });

    ui.on_remove_selected({
        let weak = weak.clone();
        let selection = selection.clone();
        move |path| {
            let path = PathBuf::from(path.to_string());
            if path.as_os_str().is_empty() {
                *selection.pending_text.borrow_mut() = None;
                update_selection(&weak, &selection);
                return;
            }
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
            *selection.pending_text.borrow_mut() = Some(text.to_string());
            selection.paths.borrow_mut().clear();
            selection.devices.borrow_mut().clear();
            *selection.selected_target.borrow_mut() = None;
            update_selection(&weak, &selection);
            dispatch_command(&commands, UiCommand::RefreshDiscovery);
            if let Some(ui) = weak.upgrade() {
                ui.set_text_dialog_open(false);
                ui.set_text_message("".into());
            }
            update_message(&weak, "文本已准备好，请选择附近设备发送");
        }
    });

    ui.on_send_clipboard({
        let weak = weak.clone();
        let commands = commands.clone();
        let selection = selection.clone();
        move || match clipboard_payload::read_clipboard() {
            Ok(ClipboardPayload::Text(text)) => {
                *selection.pending_text.borrow_mut() = Some(text);
                selection.paths.borrow_mut().clear();
                selection.devices.borrow_mut().clear();
                *selection.selected_target.borrow_mut() = None;
                update_selection(&weak, &selection);
                dispatch_command(&commands, UiCommand::RefreshDiscovery);
                update_message(
                    &weak,
                    if ui_is_english() {
                        "Clipboard text ready; select a nearby device to send"
                    } else {
                        "剪贴板文本已准备好，请选择附近设备发送"
                    },
                );
            }
            Ok(ClipboardPayload::Files(paths)) => {
                if paths.is_empty() {
                    update_message(
                        &weak,
                        if ui_is_english() {
                            "No file, folder, image, or text in clipboard"
                        } else {
                            "剪贴板中没有可发送的文件、文件夹、图片或文本"
                        },
                    );
                    return;
                }
                *selection.pending_text.borrow_mut() = None;
                selection.paths.borrow_mut().clear();
                add_selected_paths(&selection, paths.clone());
                selection.devices.borrow_mut().clear();
                *selection.selected_target.borrow_mut() = None;
                update_selection(&weak, &selection);
                dispatch_command(&commands, UiCommand::RefreshDiscovery);
                update_message(
                    &weak,
                    &if ui_is_english() {
                        format!(
                            "{} clipboard item(s) ready; refreshing nearby devices",
                            paths.len()
                        )
                    } else {
                        format!(
                            "剪贴板中有 {} 项内容，已准备发送并刷新附近设备",
                            paths.len()
                        )
                    },
                );
            }
            Err(error) => {
                update_message(&weak, error.user_message(ui_is_english()));
            }
        }
    });

    ui.on_add_manual({
        let weak = weak.clone();
        let commands = commands.clone();
        move |target| {
            if target.trim().is_empty() {
                update_message(&weak, "请输入设备 IP 地址或主机名");
                return;
            }
            dispatch_command(&commands, UiCommand::AddManual(target.to_string()));
            if let Some(ui) = weak.upgrade() {
                ui.set_manual_target("".into());
                ui.set_manual_dialog_open(false);
            }
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

    ui.on_show_device_details({
        let weak = weak.clone();
        let latest_snapshot = latest_snapshot.clone();
        move |fingerprint| {
            let Some(ui) = weak.upgrade() else {
                return;
            };
            let fingerprint = fingerprint.to_string();
            let snapshot = latest_snapshot.borrow();
            let live = snapshot.as_ref().and_then(|snapshot| {
                snapshot
                    .devices
                    .iter()
                    .find(|device| device.device.fingerprint == fingerprint)
                    .map(|device| {
                        let is_favorite = snapshot
                            .favorites
                            .iter()
                            .any(|favorite| favorite.device.fingerprint == fingerprint);
                        (
                            device.device.alias.clone(),
                            device.device.version.clone(),
                            device.device.ip.clone().unwrap_or_default(),
                            device.device.port,
                            device.device.protocol.to_string(),
                            is_favorite,
                            device.source.clone(),
                        )
                    })
            });
            let device = if let Some(live) = live {
                live
            } else {
                let Some(device) = ui
                    .get_devices()
                    .iter()
                    .find(|device| device.fingerprint.as_str() == fingerprint)
                else {
                    update_message(&weak, "设备已离线，请刷新设备列表");
                    return;
                };
                (
                    device.alias.to_string(),
                    String::new(),
                    device.ip.to_string(),
                    0,
                    device.protocol.to_string(),
                    device.favorite,
                    "snapshot".to_string(),
                )
            };
            ui.set_device_details_fingerprint(fingerprint.into());
            ui.set_device_details_alias(device.0.clone().into());
            ui.set_device_details_version(device.1.clone().into());
            ui.set_device_details_protocol(device.4.clone().to_uppercase().into());
            let address = if device.3 == 0 {
                device.2.clone()
            } else {
                format!("{}:{}", device.2, device.3)
            };
            ui.set_device_details_address(address.into());
            ui.set_device_details_favorite(device.5);
            let log = format!(
                "{} · {} · {} ({})",
                if ui_is_english() {
                    "Recent discovery"
                } else {
                    "最近发现"
                },
                device.6,
                device.4,
                if device.2.is_empty() {
                    "unknown"
                } else {
                    &device.2
                }
            );
            ui.set_device_details_logs(ModelRc::new(VecModel::from(vec![crate::DeviceLogItem {
                text: log.into(),
            }])));
            ui.set_device_details_open(true);
        }
    });

    ui.on_close_device_details({
        let weak = weak.clone();
        move || {
            if let Some(ui) = weak.upgrade() {
                ui.set_device_details_open(false);
            }
        }
    });

    ui.on_toggle_device_details_favorite({
        let weak = weak.clone();
        let commands = commands.clone();
        move |fingerprint| {
            let Some(ui) = weak.upgrade() else {
                return;
            };
            let favorite = !ui.get_device_details_favorite();
            ui.set_device_details_favorite(favorite);
            dispatch_command(
                &commands,
                UiCommand::ToggleFavorite(fingerprint.to_string()),
            );
        }
    });

    ui.on_verify_device_details({
        let weak = weak.clone();
        let commands = commands.clone();
        move |fingerprint| {
            if let Some(ui) = weak.upgrade() {
                ui.set_device_details_logs(ModelRc::new(VecModel::from(vec![
                    crate::DeviceLogItem {
                        text: "正在刷新设备连接…".into(),
                    },
                ])));
            }
            dispatch_command(&commands, UiCommand::VerifyDevice(fingerprint.to_string()));
        }
    });

    ui.on_send_to_device({
        let weak = weak.clone();
        let commands = commands.clone();
        let selection = selection.clone();
        let latest_snapshot = latest_snapshot.clone();
        move |fingerprint| {
            let fingerprint = fingerprint.to_string();
            #[cfg(debug_assertions)]
            eprintln!("localsend-gui: device callback {fingerprint}");
            *selection.selected_target.borrow_mut() = Some(fingerprint.clone());
            if selection.send_mode.get() == 1 {
                let mut devices = selection.devices.borrow_mut();
                if let Some(index) = devices.iter().position(|item| item == &fingerprint) {
                    devices.remove(index);
                } else {
                    devices.push(fingerprint);
                }
                drop(devices);
                if let Some(snapshot) = latest_snapshot.borrow().clone() {
                    update_snapshot(&weak, &snapshot, &selection);
                }
                let count = selection.devices.borrow().len();
                update_message(
                    &weak,
                    &if ui_is_english() {
                        format!("{count} devices selected; press Send")
                    } else {
                        format!("已选择 {count} 台设备；点击发送")
                    },
                );
                return;
            }
            if let Some(snapshot) = latest_snapshot.borrow().clone() {
                update_snapshot(&weak, &snapshot, &selection);
            }
            if selection.paths.borrow().is_empty() {
                let text = selection.pending_text.borrow_mut().take();
                if let Some(text) = text {
                    dispatch_command(&commands, UiCommand::SendText { fingerprint, text });
                    update_message(&weak, "正在发送文本，等待接收确认");
                } else {
                    update_message(&weak, "请先选择文件、文件夹、文本或剪贴板，再选择设备发送");
                }
            } else {
                send_selected_paths_to(&weak, &commands, &selection, &fingerprint);
            }
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
                Ok(()) => {
                    ui.set_message_received_open(false);
                    update_message(&weak, "消息已复制");
                }
                Err(error) => update_message(&weak, &format!("复制消息失败：{error}")),
            }
        }
    });

    ui.on_open_completion_file({
        let weak = weak.clone();
        move || {
            let Some(ui) = weak.upgrade() else {
                return;
            };
            let path = PathBuf::from(ui.get_completion_file_path().to_string());
            if let Err(error) = open_path(&path) {
                update_message(&weak, &format!("无法打开文件：{error}"));
            }
        }
    });

    ui.on_reveal_completion_file({
        let weak = weak.clone();
        move || {
            let Some(ui) = weak.upgrade() else {
                return;
            };
            let path = PathBuf::from(ui.get_completion_file_path().to_string());
            if let Err(error) = reveal_path(&path) {
                update_message(&weak, &format!("无法定位文件：{error}"));
            }
        }
    });

    ui.on_close_completion({
        let weak = weak.clone();
        move || {
            if let Some(ui) = weak.upgrade() {
                ui.set_completion_dialog_open(false);
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
            let web_share_running = snapshot
                .devices
                .iter()
                .find(|device| device.source == "local")
                .is_some_and(|device| device.device.download);
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
            if web_share_running && matches!(name.as_str(), "https" | "send_auto_accept") {
                let _ = commands.send(UiCommand::RestartWebReceive);
            }
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
                "launch_on_login" => settings.launch_on_login = value,
                "animations" => settings.animations = value,
                "auto_finish" => settings.auto_finish = value,
                "send_auto_accept" => settings.auto_accept = value,
                "create_checksums" => settings.create_checksums = value,
                "verify_checksums" => settings.verify_checksums = value,
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
        let selection = selection.clone();
        let latest_snapshot = latest_snapshot.clone();
        move |transfer_id| {
            let transfer_id = transfer_id.to_string();
            dispatch_command(&commands, UiCommand::CancelTransfer(transfer_id.clone()));
            // Dismiss the progress surface immediately. The daemon still owns
            // the cancellation and reports the terminal result to every client.
            selection
                .dismissed_transfers
                .borrow_mut()
                .insert(transfer_id);
            if let Some(snapshot) = latest_snapshot.borrow().as_ref() {
                update_snapshot(&weak, snapshot, &selection);
            }
            update_message(&weak, "正在取消传输");
        }
    });

    ui.on_dismiss_transfer({
        let weak = weak.clone();
        let commands = commands.clone();
        let selection = selection.clone();
        let latest_snapshot = latest_snapshot.clone();
        move |transfer_id| {
            let transfer_id = transfer_id.to_string();
            dispatch_command(&commands, UiCommand::DismissTransfer(transfer_id.clone()));
            selection
                .dismissed_transfers
                .borrow_mut()
                .insert(transfer_id);
            if let Some(snapshot) = latest_snapshot.borrow().as_ref() {
                update_snapshot(&weak, snapshot, &selection);
            }
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
        let commands = commands.clone();
        move || {
            dispatch_command(&commands, UiCommand::StartWebReceive);
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

    ui.on_open_web_share_url({
        let weak = weak.clone();
        let latest_snapshot = latest_snapshot.clone();
        move || {
            let Some(snapshot) = latest_snapshot.borrow().as_ref().cloned() else {
                return;
            };
            let Some(url) = web_receive_url(&snapshot) else {
                update_message(&weak, "Web Share 链接不可用");
                return;
            };
            if let Err(error) = open_url(&url) {
                update_message(&weak, &format!("无法打开浏览器：{error}"));
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
            let Some(directory) =
                with_native_file_dialog_cleanup(|| rfd::FileDialog::new().pick_folder())
            else {
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
                let web_share_running = snapshot
                    .devices
                    .iter()
                    .find(|device| device.source == "local")
                    .is_some_and(|device| device.device.download);
                dispatch_settings(&weak, &commands, settings.revision, settings);
                if web_share_running {
                    let _ = commands.send(UiCommand::RestartWebReceive);
                }
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
                "discovery-timeout" => snapshot.settings.discovery_timeout_ms.to_string(),
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
                        settings.discovery_timeout_ms = timeout;
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
            if matches!(key.as_str(), "pin" | "pin-enable" | "pin-disable")
                && snapshot
                    .devices
                    .iter()
                    .find(|device| device.source == "local")
                    .is_some_and(|device| device.device.download)
            {
                let _ = commands.send(UiCommand::RestartWebReceive);
            }
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
        let local_state = local_state.clone();
        let launch_setting = launch_setting.clone();
        let window_persistence = window_persistence.clone();
        let selection = selection.clone();
        let events = Rc::new(RefCell::new(worker_events));
        let events_for_timer = events.clone();
        let gui_actions = Rc::new(RefCell::new(gui_action_rx));
        let gui_actions_for_timer = gui_actions.clone();
        let _control_server = control_server;
        #[cfg(not(unix))]
        let mut next_queue_scan = Instant::now();
        move || {
            let _ = &_control_server;
            #[cfg(not(unix))]
            if Instant::now() >= next_queue_scan {
                if let Ok(actions) = take_queued_gui_actions() {
                    for action in actions {
                        handle_gui_action(&weak, action);
                    }
                }
                next_queue_scan = Instant::now() + Duration::from_millis(250);
            }
            while let Ok(action) = gui_actions_for_timer.borrow().try_recv() {
                handle_gui_action(&weak, action);
            }
            loop {
                let event = events_for_timer.borrow().try_recv();
                let Ok(event) = event else { break };
                match event {
                    WorkerEvent::Snapshot(snapshot) => {
                        local_state
                            .borrow_mut()
                            .replace(SnapshotStore::new((*snapshot).clone()));
                        publish_ui_snapshot(
                            &weak,
                            *snapshot,
                            &latest_snapshot,
                            &selection,
                            &launch_setting,
                            &window_persistence,
                        );
                    }
                    WorkerEvent::DaemonEvent(event) => {
                        let snapshot = {
                            let mut state = local_state.borrow_mut();
                            state.as_mut().and_then(|state| match state.apply(&event) {
                                EventApply::Applied => Some(state.snapshot().clone()),
                                EventApply::Duplicate | EventApply::Gap => None,
                            })
                        };
                        if let Some(snapshot) = snapshot {
                            publish_ui_snapshot(
                                &weak,
                                snapshot,
                                &latest_snapshot,
                                &selection,
                                &launch_setting,
                                &window_persistence,
                            );
                        }
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
                        *selection.pending_text.borrow_mut() = None;
                        let _ = files;
                    }
                    WorkerEvent::Connection {
                        connected,
                        message,
                        show_notice,
                    } => {
                        if let Some(ui) = weak.upgrade() {
                            ui.set_server_status(message.clone().into());
                        }
                        if !connected && show_notice {
                            update_message(&weak, &message);
                        }
                    }
                    WorkerEvent::TextReceived {
                        sender_alias,
                        sender_device_model,
                        text,
                    } => {
                        let title = if ui_is_english() {
                            "New message received"
                        } else {
                            "收到新消息"
                        };
                        let body = if ui_is_english() {
                            format!("{sender_alias}: {text}")
                        } else {
                            format!("{sender_alias}：{text}")
                        };
                        let _device = sender_device_model.unwrap_or_default();
                        if let Some(ui) = weak.upgrade() {
                            ui.set_last_received_title(title.into());
                            ui.set_last_received_body(body.clone().into());
                            ui.set_last_received_visible(true);
                        }
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

    if options.minimized
        && let Some(snapshot) = latest_snapshot.borrow().as_ref()
    {
        ui.global::<ThemeTokens>()
            .set_dark(resolve_dark_theme(&snapshot.settings.theme));
    }
    Ok(ui)
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

fn pick_files_for_e2e() -> Vec<PathBuf> {
    if std::env::var("LOCALSEND_ACCEPTANCE").ok().as_deref() == Some("1") {
        return acceptance_file_choices("LOCALSEND_E2E_FILE_PATHS", "LOCALSEND_E2E_FILE_PATH");
    }
    with_native_file_dialog_cleanup(|| rfd::FileDialog::new().pick_files()).unwrap_or_default()
}

fn pick_folder_for_e2e() -> Option<PathBuf> {
    if std::env::var("LOCALSEND_ACCEPTANCE").ok().as_deref() == Some("1") {
        return acceptance_file_choice("LOCALSEND_E2E_FOLDER_PATH");
    }
    with_native_file_dialog_cleanup(|| rfd::FileDialog::new().pick_folder())
}

fn acceptance_file_choice(variable: &str) -> Option<PathBuf> {
    acceptance_file_choices(variable, variable)
        .into_iter()
        .next()
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

fn add_selected_paths(selection: &SelectionState, paths: Vec<PathBuf>) {
    let mut selected = selection.paths.borrow_mut();
    for path in paths {
        if !selected.contains(&path) {
            selected.push(path);
        }
    }
}

fn is_image_path(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "png" | "jpg" | "jpeg" | "gif" | "webp" | "bmp" | "tif" | "tiff" | "heic" | "heif"
            )
        })
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
    let pending_text = selection.pending_text.borrow();
    let mut file_total = 0_u64;
    let mut files = paths
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
                is_text: false,
                is_image: !is_directory && is_image_path(path),
                size: if is_directory {
                    if ui_is_english() {
                        "Folder".to_string().into()
                    } else {
                        "文件夹".to_string().into()
                    }
                } else {
                    view_model::format_bytes(metadata.as_ref().map_or(0, |metadata| metadata.len()))
                        .into()
                },
            }
        })
        .collect::<Vec<_>>();
    if files.is_empty()
        && let Some(text) = pending_text.as_ref()
    {
        file_total = text.len() as u64;
        files.push(SelectedFileItem {
            path: "".into(),
            name: if ui_is_english() {
                "Text message".into()
            } else {
                "文本消息".into()
            },
            is_directory: false,
            is_text: true,
            is_image: false,
            size: view_model::format_bytes(file_total).into(),
        });
    }
    let item_count = files.len();
    ui.set_selected_files(ModelRc::new(VecModel::from(files)));
    ui.set_has_selection(item_count > 0);
    ui.set_selected_file_count(item_count.min(i32::MAX as usize) as i32);
    ui.set_selected_total_size(view_model::format_bytes(file_total).into());
    ui.set_selected_files_text(
        if item_count == 0 {
            String::new()
        } else if !paths.is_empty() {
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
        } else if ui_is_english() {
            "Text message".to_string()
        } else {
            "文本消息".to_string()
        }
        .into(),
    );
}

fn publish_ui_snapshot(
    weak: &Weak<AppWindow>,
    snapshot: ServiceSnapshot,
    latest_snapshot: &Rc<RefCell<Option<ServiceSnapshot>>>,
    selection: &SelectionState,
    launch_setting: &Cell<Option<bool>>,
    window_persistence: &Rc<RefCell<WindowPersistence>>,
) {
    if let Some(ui) = weak.upgrade() {
        window_persistence
            .borrow_mut()
            .apply_setting(&ui, snapshot.settings.save_window_position);
    }
    update_snapshot(weak, &snapshot, selection);
    let launch_on_login = snapshot.settings.launch_on_login;
    if launch_setting.get() != Some(launch_on_login) {
        if let Err(error) = apply_launch_on_login(launch_on_login) {
            update_message(weak, &error);
        }
        launch_setting.set(Some(launch_on_login));
    }
    *latest_snapshot.borrow_mut() = Some(snapshot);
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
    let mut last_send_at = selection.last_send_at.borrow_mut();
    if last_send_at
        .get(fingerprint)
        .is_some_and(|sent_at| sent_at.elapsed() < Duration::from_millis(750))
    {
        update_message(weak, "发送请求已提交，请等待对端响应");
        return;
    }
    last_send_at.insert(fingerprint.to_string(), Instant::now());
    drop(last_send_at);
    if commands
        .send(UiCommand::SendFiles {
            fingerprint: fingerprint.to_string(),
            paths,
        })
        .is_ok()
    {
        update_message(weak, "正在连接对端，等待接收确认");
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
    let message = localize_message(message);
    if let Some(ui) = weak.upgrade() {
        ui.set_message(message.into());
    }
}

fn localize_message(message: &str) -> String {
    if !ui_is_english() {
        return message.to_string();
    }
    for (prefix, replacement) in [
        ("复制消息失败：", "Copy failed: "),
        ("无法打开文件：", "Cannot open file: "),
        ("无法定位文件：", "Cannot reveal file: "),
        ("无法复制链接：", "Cannot copy link: "),
        ("无法打开浏览器：", "Cannot open browser: "),
        ("已创建传输 ", "Transfer created "),
        ("传输失败：", "Transfer failed: "),
        ("文本已发送给 ", "Text sent to "),
        ("文件已发送给 ", "File sent to "),
    ] {
        if let Some(rest) = message.strip_prefix(prefix) {
            return format!("{replacement}{rest}");
        }
    }
    match message {
        "已选择文件，正在刷新附近设备" => "File selected; refreshing nearby devices",
        "已选择文件夹，正在刷新附近设备" => {
            "Folder selected; refreshing nearby devices"
        }
        "文本已准备好，请选择附近设备发送" => {
            "Text ready; select a nearby device to send"
        }
        "无法访问剪贴板" => "Cannot access the clipboard",
        "剪贴板中没有文本" => "No text in clipboard",
        "剪贴板文本已准备好，请选择附近设备发送" => {
            "Clipboard text ready; select a nearby device to send"
        }
        "请输入设备 IP 地址或主机名" => "Enter a device IP address or hostname",
        "正在添加设备" => "Adding device",
        "正在发送文本，等待接收确认" => {
            "Sending text; waiting for receiver confirmation"
        }
        "请先选择文件、文件夹、文本或剪贴板，再选择设备发送" => {
            "Select a file, folder, text, or clipboard content before choosing a device"
        }
        "请先选择至少一个设备" => "Select at least one device",
        "正在清空历史" => "Clearing history",
        "消息已复制" => "Message copied",
        "等待 localsendd 连接" => "Waiting for localsendd connection",
        "正在接受传输" => "Accepting transfer",
        "正在拒绝传输" => "Declining transfer",
        "正在取消传输" => "Cancelling transfer",
        "正在启动浏览器接收页面" => "Starting browser receive page",
        "正在关闭浏览器接收页面" => "Stopping browser receive page",
        "当前没有可复制的接收链接" => "No receive link is available to copy",
        "接收链接已复制" => "Receive link copied",
        "Web Share 链接不可用" => "Web Share link is unavailable",
        "正在重启服务器" => "Restarting server",
        "此项由系统自动检测或当前平台不支持编辑" => {
            "This value is detected by the system or cannot be edited on this platform"
        }
        "端口必须是 1 到 65535 之间的数字" => "Port must be a number from 1 to 65535",
        "PIN 不能为空" => "PIN cannot be empty",
        "发现超时必须是正整数" => "Discovery timeout must be a positive integer",
        "组播地址必须是有效 IPv4 地址" => {
            "Multicast address must be a valid IPv4 address"
        }
        "此设置不能通过文本输入修改" => {
            "This setting cannot be changed through text input"
        }
        "请先选择文件或文件夹" => "Select a file or folder first",
        "发送请求已提交，请等待对端响应" => {
            "Send request submitted; waiting for the receiver"
        }
        "正在连接对端，等待接收确认" => {
            "Connecting to the receiver; waiting for confirmation"
        }
        "daemon worker 已关闭" => "Daemon worker is closed",
        "设置已提交" => "Settings submitted",
        "收到新消息" => "New message received",
        "收到新文件" => "New file received",
        "服务器运行中" => "Server running",
        "服务器已停止" => "Server stopped",
        "请在系统设置中检查防火墙规则" => "Check firewall rules in system settings",
        _ => return message.to_string(),
    }
    .to_string()
}

fn update_snapshot(weak: &Weak<AppWindow>, snapshot: &ServiceSnapshot, selection: &SelectionState) {
    let Some(ui) = weak.upgrade() else {
        return;
    };
    apply_interface_language(&snapshot.settings.language);
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
    ui.set_settings_auto_save_all(snapshot.settings.auto_save == AutoSaveMode::On);
    ui.set_settings_auto_save_favorites(snapshot.settings.auto_save_from_favorites);
    ui.set_settings_auto_finish(snapshot.settings.auto_finish);
    ui.set_settings_send_auto_accept(snapshot.settings.auto_accept);
    ui.set_settings_create_checksums(snapshot.settings.create_checksums);
    ui.set_settings_verify_checksums(snapshot.settings.verify_checksums);
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
    ui.set_settings_discovery_timeout(snapshot.settings.discovery_timeout_ms.to_string().into());
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
    ui.set_settings_pin_enabled(snapshot.settings.pin.is_some());
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
        .unwrap_or_else(|| {
            if ui_is_english() {
                "Select a target device".to_string()
            } else {
                "请选择目标设备".to_string()
            }
        });
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
        ui.set_incoming_request_title(
            if ui_is_english() {
                format!("{} wants to send files", request.sender.alias)
            } else {
                format!("{} 想发送文件", request.sender.alias)
            }
            .into(),
        );
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
        ui.set_incoming_request_body(
            if ui_is_english() {
                format!("{} files", request.files.len())
            } else {
                format!("{} 个文件", request.files.len())
            }
            .into(),
        );
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
    let dismissed = selection.dismissed_transfers.borrow();
    let visible_transfers = view_model::transfers_with_language(snapshot, ui_is_english())
        .into_iter()
        .filter(|transfer| !dismissed.contains(transfer.transfer_id.as_str()))
        .collect::<Vec<_>>();
    let visible_send_ids = visible_transfers
        .iter()
        .map(|transfer| transfer.transfer_id.as_str())
        .collect::<HashSet<_>>();
    let sending_active = snapshot.active_transfers.iter().any(|transfer| {
        transfer.direction == localsend_rpc::TransferDirection::Send
            && visible_send_ids.contains(transfer.transfer_id.as_str())
    });
    ui.set_transfer_items(ModelRc::new(VecModel::from(
        visible_transfers
            .iter()
            .map(|transfer| TransferItem {
                transfer_id: transfer.transfer_id.clone().into(),
                peer: transfer.peer.clone().into(),
                summary: transfer.summary.clone().into(),
                status: transfer.status.clone().into(),
                direction: transfer.direction.clone().into(),
                terminal: transfer.terminal,
                progress_percent: transfer.progress_percent,
                file_count: transfer.file_count,
                completion_countdown: transfer.completion_countdown,
                files: ModelRc::new(VecModel::from(
                    transfer
                        .files
                        .iter()
                        .map(|file| TransferFileItem {
                            name: file.name.clone().into(),
                            size: file.size.clone().into(),
                            is_text: file.is_text,
                        })
                        .collect::<Vec<_>>(),
                )),
            })
            .collect::<Vec<_>>(),
    )));
    ui.set_sending_active(sending_active);
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
                    if ui_is_english() {
                        "New message received".to_string()
                    } else {
                        "收到新消息".to_string()
                    },
                    if ui_is_english() {
                        format!("{}: {}", entry.peer_alias, message)
                    } else {
                        format!("{}：{}", entry.peer_alias, message)
                    },
                )
            } else {
                (
                    if ui_is_english() {
                        "New file received".to_string()
                    } else {
                        "收到新文件".to_string()
                    },
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
                if entry.direction == localsend_rpc::TransferDirection::Receive
                    && let Some(path) = entry.path.as_ref()
                {
                    ui.set_completion_file_name(entry.file_name.clone().into());
                    ui.set_completion_file_path(path.to_string_lossy().into_owned().into());
                    ui.set_completion_countdown(if snapshot.settings.auto_finish { 5 } else { 0 });
                    ui.set_completion_dialog_open(true);
                    ui.set_last_received_visible(false);
                }
            }
            *selection.last_history_id.borrow_mut() = Some(entry.id.clone());
        }
    }
    let english = ui_is_english();
    let running = if snapshot.server_running {
        if english {
            "Server running"
        } else {
            "服务器运行中"
        }
    } else if english {
        "Server stopped"
    } else {
        "服务器已停止"
    };
    let status = if english {
        format!(
            "{running} · port {} · transfers {} · history {}{}",
            snapshot.server_port.unwrap_or(snapshot.settings.port),
            snapshot.active_transfers.len(),
            snapshot.history.len(),
            snapshot
                .last_error
                .as_deref()
                .map(|error| format!(" · {error}"))
                .unwrap_or_default()
        )
    } else {
        format!(
            "{running} · 端口 {} · 传输 {} · 历史 {}{}",
            snapshot.server_port.unwrap_or(snapshot.settings.port),
            snapshot.active_transfers.len(),
            snapshot.history.len(),
            snapshot
                .last_error
                .as_deref()
                .map(|error| format!(" · {error}"))
                .unwrap_or_default()
        )
    };
    ui.set_server_status(status.into());
    ui.set_devices(build_devices(snapshot, selection));
    ui.set_favorites(build_favorites(snapshot));
    ui.set_history_items(build_history(snapshot));
    ui.set_devices_text(devices_text(snapshot).into());
    update_selection(weak, selection);
}

fn apply_interface_language(language: &str) {
    let locale = if language.starts_with("en") {
        "en-US"
    } else {
        "zh-CN"
    };
    let changed = ACTIVE_LANGUAGE.with(|active| {
        let mut active = active.borrow_mut();
        if active.as_deref() == Some(locale) {
            false
        } else {
            *active = Some(locale);
            true
        }
    });
    if changed && let Err(error) = slint::select_bundled_translation(locale) {
        eprintln!("failed to select Slint translation {locale}: {error}");
    }
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
    let items: Vec<HistoryItem> = view_model::history_with_language(snapshot, ui_is_english())
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
        if ui_is_english() {
            "Unknown".to_string()
        } else {
            "未知".to_string()
        }
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
        if ui_is_english() {
            "All interfaces".to_string()
        } else {
            "全部接口".to_string()
        }
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

fn reveal_path(path: &PathBuf) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    let mut command = {
        let mut command = std::process::Command::new("open");
        command.arg("-R").arg(path);
        command
    };
    #[cfg(target_os = "windows")]
    let mut command = {
        let mut command = std::process::Command::new("explorer");
        command.arg(format!("/select,{}", path.display()));
        command
    };
    #[cfg(all(unix, not(target_os = "macos")))]
    let mut command = {
        let mut command = std::process::Command::new("xdg-open");
        command.arg(path.parent().unwrap_or(path));
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
        std::process::Command::new("control.exe")
            .args(["/name", "Microsoft.WindowsFirewall"])
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
    let _ = start_minimized;
    spawn_tray_process(&[])
}

fn spawn_tray_process(extra_args: &[String]) -> anyhow::Result<()> {
    let current = std::env::current_exe()?;
    let binary = if cfg!(windows) {
        "localsend-tray.exe"
    } else if cfg!(target_os = "macos") {
        "LocalSendTray"
    } else {
        "localsend-tray"
    };
    let mut candidates = Vec::new();
    #[cfg(target_os = "macos")]
    if let Some(contents) = current.parent().and_then(std::path::Path::parent) {
        candidates.push(
            contents
                .join("Helpers/LocalSendTray.app/Contents/MacOS")
                .join(binary),
        );
    }
    if let Some(parent) = current.parent() {
        candidates.push(parent.join(binary));
    }
    candidates.push(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("target/debug")
            .join(binary),
    );
    let executable = candidates
        .into_iter()
        .find(|path| path.exists())
        .ok_or_else(|| anyhow::anyhow!("tray helper was not found next to the GUI"))?;

    #[cfg(target_os = "macos")]
    if let Some(helper_app) = executable
        .ancestors()
        .find(|path| path.extension().is_some_and(|extension| extension == "app"))
    {
        let mut command = Command::new("/usr/bin/open");
        command.arg("-n").arg("-a").arg(helper_app).arg("--args");
        if let Some(endpoint) = transport::configured_endpoint() {
            command.arg("--endpoint").arg(endpoint);
        }
        command.args(extra_args);
        command.spawn()?;
        return Ok(());
    }

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

fn theme_label(theme: &str) -> &'static str {
    if ui_is_english() {
        match theme {
            "dark" => "Dark",
            "light" => "Light",
            _ => "System",
        }
    } else {
        match theme {
            "dark" => "深色",
            "light" => "浅色",
            _ => "跟随系统",
        }
    }
}

fn color_label(color: &str) -> &'static str {
    if ui_is_english() {
        match color {
            "blue" => "Blue",
            "green" => "Green",
            _ => "LocalSend",
        }
    } else {
        match color {
            "blue" => "蓝色",
            "green" => "绿色",
            _ => "LocalSend",
        }
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

fn handle_gui_action(weak: &Weak<AppWindow>, action: GuiAction) {
    match action {
        GuiAction::ShowMain => {
            present_main_window(weak);
        }
        GuiAction::OpenMessage {
            sender,
            device,
            text,
        } => {
            let Some(ui) = weak.upgrade() else {
                return;
            };
            present_main_window(weak);
            ui.set_selected_tab("receive".into());
            ui.set_last_received_visible(false);
            ui.set_message_received_sender(sender.into());
            ui.set_message_received_device(device.into());
            ui.set_message_received_text(text.into());
            ui.set_message_received_open(true);
        }
        GuiAction::OpenReceiveFolder { path } => {
            if let Err(error) = open_path(&path) {
                update_message(weak, &error);
            }
        }
    }
}

fn present_main_window(weak: &Weak<AppWindow>) {
    let Some(ui) = weak.upgrade() else {
        return;
    };
    let _ = ui.show();
    ui.window().set_minimized(false);
    activate_current_application_window();

    let weak = ui.as_weak();
    slint::Timer::single_shot(Duration::from_millis(150), move || {
        if let Some(ui) = weak.upgrade() {
            let _ = ui.show();
            ui.window().set_minimized(false);
            activate_current_application_window();
        }
    });
}

fn language_label(language: &str) -> &'static str {
    if ui_is_english() {
        match language {
            "en" | "en-US" => "English",
            "zh" | "zh-CN" => "Simplified Chinese",
            _ => "System",
        }
    } else {
        match language {
            "en" | "en-US" => "English",
            "zh" | "zh-CN" => "简体中文",
            _ => "跟随系统",
        }
    }
}

fn ui_is_english() -> bool {
    ACTIVE_LANGUAGE.with(|active| active.borrow().as_deref() == Some("en-US"))
}

#[cfg(test)]
mod tests {
    use super::{
        ACTIVE_LANGUAGE, SelectionState, WindowPlacement, WindowState, acceptance_file_choices,
        add_selected_paths, color_label, language_label, theme_label, visual_ids_from_ips,
    };
    use crate::AppWindow;
    use i_slint_backend_testing::{AccessibleRole, ElementQuery};
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
    fn settings_value_labels_follow_interface_language() {
        ACTIVE_LANGUAGE.with(|active| *active.borrow_mut() = Some("en-US"));
        assert_eq!(theme_label("dark"), "Dark");
        assert_eq!(theme_label("light"), "Light");
        assert_eq!(color_label("blue"), "Blue");
        assert_eq!(language_label("zh-CN"), "Simplified Chinese");
        ACTIVE_LANGUAGE.with(|active| *active.borrow_mut() = Some("zh-CN"));
        assert_eq!(theme_label("dark"), "深色");
        assert_eq!(language_label("zh-CN"), "简体中文");
        ACTIVE_LANGUAGE.with(|active| *active.borrow_mut() = None);
    }

    #[test]
    fn visual_ids_use_real_ipv4_last_octets() {
        let ips = vec![
            "192.0.2.80".parse().unwrap(),
            "10.0.0.7".parse().unwrap(),
            "192.0.2.80".parse().unwrap(),
        ];
        assert_eq!(visual_ids_from_ips(ips), "#7 #80");
        assert_eq!(visual_ids_from_ips(Vec::new()), "Offline");
    }

    #[test]
    fn acceptance_file_picker_accepts_multiple_newline_separated_paths() {
        let root =
            std::env::temp_dir().join(format!("localsend-gui-multi-file-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let first = root.join("first.txt");
        let second = root.join("second.txt");
        fs::write(&first, b"first").unwrap();
        fs::write(&second, b"second").unwrap();
        let variable = format!("LOCALSEND_E2E_FILE_PATHS_TEST_{}", std::process::id());
        unsafe {
            std::env::set_var(
                &variable,
                format!("{}\n{}\n", first.display(), second.display()),
            );
        }
        let paths = acceptance_file_choices(&variable, &variable);
        unsafe {
            std::env::remove_var(&variable);
        }
        assert_eq!(paths, vec![first, second]);

        let selection = SelectionState::new();
        add_selected_paths(&selection, paths.clone());
        add_selected_paths(&selection, paths.clone());
        assert_eq!(*selection.paths.borrow(), paths);
        fs::remove_dir_all(root).unwrap();
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
        let completion_revealed = std::rc::Rc::new(std::cell::Cell::new(false));
        let web_share_stopped = std::rc::Rc::new(std::cell::Cell::new(false));
        let toggles = std::rc::Rc::new(std::cell::RefCell::new(Vec::<(String, bool)>::new()));
        let edit_settings = std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
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
        ui.on_edit_setting({
            let edit_settings = edit_settings.clone();
            move |key| edit_settings.borrow_mut().push(key.to_string())
        });
        ui.on_reveal_completion_file({
            let completion_revealed = completion_revealed.clone();
            move || completion_revealed.set(true)
        });
        ui.on_stop_web_share({
            let web_share_stopped = web_share_stopped.clone();
            move || web_share_stopped.set(true)
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

        let create_checksums = ElementQuery::from_root(&ui)
            .match_accessible_role(AccessibleRole::Switch)
            .match_predicate(|element| {
                element
                    .accessible_label()
                    .is_some_and(|label| label == "发送文件时创建校验和")
            })
            .find_first()
            .expect("create-checksums switch");
        create_checksums.invoke_accessible_default_action();
        assert!(
            toggles
                .borrow()
                .iter()
                .any(|(name, _)| name == "create_checksums")
        );

        let verify_checksums = ElementQuery::from_root(&ui)
            .match_accessible_role(AccessibleRole::Switch)
            .match_predicate(|element| {
                element
                    .accessible_label()
                    .is_some_and(|label| label == "接收文件时验证校验和")
            })
            .find_first()
            .expect("verify-checksums switch");
        verify_checksums.invoke_accessible_default_action();
        assert!(
            toggles
                .borrow()
                .iter()
                .any(|(name, _)| name == "verify_checksums")
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

        let discovery_timeout = ElementQuery::from_root(&ui)
            .match_accessible_role(AccessibleRole::TextInput)
            .match_predicate(|element| {
                element
                    .accessible_label()
                    .is_some_and(|label| label == "搜索设备超时")
            })
            .find_first()
            .expect("discovery timeout input");
        assert_eq!(discovery_timeout.accessible_value().as_deref(), Some("500"));

        ui.set_completion_dialog_open(true);
        ui.set_completion_file_name("received.bin".into());
        ui.set_completion_file_path("received.bin".into());
        ui.set_completion_countdown(0);
        i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_millis(50));
        let reveal = ElementQuery::from_root(&ui)
            .match_accessible_role(AccessibleRole::Button)
            .match_predicate(|element| {
                element
                    .accessible_label()
                    .is_some_and(|label| label == "在文件管理器中显示")
            })
            .find_first()
            .expect("reveal completion button");
        reveal.invoke_accessible_default_action();
        assert!(completion_revealed.get());

        ui.set_completion_dialog_open(false);
        ui.set_web_share_dialog_open(true);
        ui.set_settings_encryption(true);
        i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_millis(50));
        let web_share_encryption = ElementQuery::from_root(&ui)
            .match_accessible_role(AccessibleRole::Checkbox)
            .match_predicate(|element| {
                element
                    .accessible_label()
                    .is_some_and(|label| label == "加密")
            })
            .find_first()
            .expect("web share encryption checkbox");
        web_share_encryption.invoke_accessible_default_action();
        assert!(
            toggles
                .borrow()
                .iter()
                .any(|(name, value)| name == "https" && !value)
        );
        let web_share_back = ElementQuery::from_root(&ui)
            .match_accessible_role(AccessibleRole::Button)
            .match_predicate(|element| {
                element
                    .accessible_label()
                    .is_some_and(|label| label == "返回")
            })
            .find_first()
            .expect("web share back button");
        let web_share_pin = ElementQuery::from_root(&ui)
            .match_accessible_role(AccessibleRole::Checkbox)
            .match_predicate(|element| {
                element
                    .accessible_label()
                    .is_some_and(|label| label == "启用 PIN 密码")
            })
            .find_first()
            .expect("web share pin checkbox");
        web_share_pin.invoke_accessible_default_action();
        assert!(edit_settings.borrow().iter().any(|key| key == "pin-enable"));
        web_share_back.invoke_accessible_default_action();
        assert!(web_share_stopped.get());
    }
}

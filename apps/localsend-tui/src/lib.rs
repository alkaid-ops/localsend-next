use anyhow::Context;
use crossterm::event::DisableBracketedPaste;
use crossterm::event::EnableBracketedPaste;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use localsend_rpc::{
    AutoSaveMode, CommandResult, DaemonClient, DesktopSettings, DeviceInfo, DeviceView, Protocol,
    ReceiverDecision, SendFileRequest, SequencedEvent, ServiceCommand, ServiceErrorKind,
    ServiceEvent, ServiceSnapshot, TransferDirection, TransferStatus,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Tabs, Wrap};
use ratatui::{Frame, Terminal};
use std::collections::{HashMap, HashSet};
use std::io::Stdout;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// Startup-only overrides used by the compatibility `localsend-rs tui`
/// command. The desktop TUI binary starts with no overrides and uses the
/// daemon's persisted settings.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TuiOptions {
    pub alias: Option<String>,
    pub port: Option<u16>,
    pub protocol: Option<Protocol>,
    /// `None` keeps the current PIN; `Some(None)` clears it.
    pub pin: Option<Option<String>>,
    pub auto_accept: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Receive,
    Send,
    Settings,
}

#[derive(Debug, Clone)]
enum InputMode {
    ManualTarget,
    SendPath,
    SendText,
    SendPin,
    FileFilter,
    EditAlias,
    EditPort,
    EditPin,
    EditDestination,
    EditDiscoveryTimeout,
    EditMulticastGroup,
    EditWhitelist,
    EditBlacklist,
    EditFavoriteAlias,
}

#[derive(Debug, Clone)]
struct FileEntry {
    path: PathBuf,
    name: String,
    is_dir: bool,
    selected: bool,
}

#[derive(Debug)]
struct FileBrowser {
    current_dir: PathBuf,
    entries: Vec<FileEntry>,
    cursor: usize,
    selected: HashSet<PathBuf>,
    filter: String,
    show_hidden: bool,
    error: Option<String>,
}

impl FileBrowser {
    fn new(start: PathBuf) -> Self {
        let current_dir = if start.is_dir() {
            start
        } else {
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        };
        let mut browser = Self {
            current_dir,
            entries: Vec::new(),
            cursor: 0,
            selected: HashSet::new(),
            filter: String::new(),
            show_hidden: false,
            error: None,
        };
        browser.reload();
        browser
    }

    fn reload(&mut self) {
        self.entries.clear();
        self.error = None;
        let filter = self.filter.to_lowercase();
        let read_dir = match std::fs::read_dir(&self.current_dir) {
            Ok(entries) => entries,
            Err(error) => {
                self.error = Some(error.to_string());
                self.cursor = 0;
                return;
            }
        };
        for entry in read_dir.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if !self.show_hidden && name.starts_with('.') {
                continue;
            }
            if !filter.is_empty() && !name.to_lowercase().contains(&filter) {
                continue;
            }
            let path = entry.path();
            let metadata = match std::fs::symlink_metadata(&path) {
                Ok(metadata) => metadata,
                Err(_) => continue,
            };
            // Symlinks are deliberately visible but never traversed or sent.
            if metadata.file_type().is_symlink() {
                continue;
            }
            let is_dir = metadata.is_dir();
            self.entries.push(FileEntry {
                selected: self.selected.contains(&path),
                path,
                name,
                is_dir,
            });
        }
        self.entries.sort_by(|a, b| {
            b.is_dir
                .cmp(&a.is_dir)
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });
        self.cursor = self.cursor.min(self.entries.len().saturating_sub(1));
    }

    fn move_cursor(&mut self, delta: isize) {
        if self.entries.is_empty() {
            return;
        }
        let next = self.cursor as isize + delta;
        self.cursor = next.rem_euclid(self.entries.len() as isize) as usize;
    }

    fn enter(&mut self) -> bool {
        let Some(entry) = self.entries.get(self.cursor) else {
            return false;
        };
        if !entry.is_dir {
            return false;
        }
        self.current_dir = entry.path.clone();
        self.cursor = 0;
        self.reload();
        true
    }

    fn parent(&mut self) -> bool {
        let Some(parent) = self.current_dir.parent() else {
            return false;
        };
        self.current_dir = parent.to_path_buf();
        self.cursor = 0;
        self.reload();
        true
    }

    fn toggle(&mut self) {
        let Some(entry) = self.entries.get_mut(self.cursor) else {
            return;
        };
        entry.selected = !entry.selected;
        if entry.selected {
            self.selected.insert(entry.path.clone());
        } else {
            self.selected.remove(&entry.path);
        }
    }

    fn toggle_all(&mut self) {
        let all = self.entries.iter().all(|entry| entry.selected);
        for entry in &mut self.entries {
            entry.selected = !all;
            if entry.selected {
                self.selected.insert(entry.path.clone());
            } else {
                self.selected.remove(&entry.path);
            }
        }
    }

    fn selected_paths(&self) -> Vec<PathBuf> {
        let mut paths: Vec<_> = self.selected.iter().cloned().collect();
        paths.sort();
        paths
    }
}

#[derive(Debug, Clone)]
enum PendingSend {
    Files {
        target: DeviceInfo,
        paths: Vec<PathBuf>,
    },
    Text {
        target: DeviceInfo,
        text: String,
    },
}

struct App {
    client: DaemonClient,
    snapshot: ServiceSnapshot,
    events: tokio::sync::broadcast::Receiver<SequencedEvent>,
    tab: Tab,
    selected_device: usize,
    selected_setting: usize,
    input_mode: Option<InputMode>,
    input: String,
    status: String,
    should_quit: bool,
    connected: bool,
    reconnect_at: Instant,
    request_cursor: usize,
    request_selected: HashSet<String>,
    history_cursor: usize,
    selected_transfer: usize,
    file_browser: Option<FileBrowser>,
    pending_send: Option<(String, PendingSend)>,
    retry_send: Option<PendingSend>,
    show_help: bool,
    file_status: HashMap<String, HashMap<String, String>>,
    skipped_paths: Vec<String>,
}

impl App {
    fn new(client: DaemonClient, snapshot: ServiceSnapshot) -> Self {
        let events = client.subscribe();
        Self {
            client,
            snapshot,
            events,
            tab: Tab::Receive,
            selected_device: 0,
            selected_setting: 0,
            input_mode: None,
            input: String::new(),
            status: "Ready".to_string(),
            should_quit: false,
            connected: true,
            reconnect_at: Instant::now(),
            request_cursor: 0,
            request_selected: HashSet::new(),
            history_cursor: 0,
            selected_transfer: 0,
            file_browser: None,
            pending_send: None,
            retry_send: None,
            show_help: false,
            file_status: HashMap::new(),
            skipped_paths: Vec::new(),
        }
    }

    async fn refresh(&mut self) {
        if !self.connected && Instant::now() >= self.reconnect_at {
            match DaemonClient::connect_interactive().await {
                Ok(client) => {
                    if let Some(snapshot) = client.snapshot() {
                        self.client = client;
                        self.snapshot = snapshot;
                        self.events = self.client.subscribe();
                        self.connected = true;
                        self.status = "Reconnected to localsendd".to_string();
                    }
                }
                Err(error) => {
                    self.reconnect_at = Instant::now() + Duration::from_secs(1);
                    self.status = format!("Waiting for localsendd: {error}");
                }
            }
        }

        let mut changed = false;
        while let Ok(event) = self.events.try_recv() {
            changed = true;
            match event.event {
                ServiceEvent::Error { message } => self.status = message,
                ServiceEvent::DaemonStopping => {
                    self.connected = false;
                    self.reconnect_at = Instant::now() + Duration::from_secs(1);
                    self.status = "Daemon stopped; reconnect pending".to_string();
                }
                ServiceEvent::TransferProgress {
                    transfer_id,
                    file_id: Some(file_id),
                    file_name,
                    ..
                } => {
                    self.file_status.entry(transfer_id).or_default().insert(
                        file_id.to_string(),
                        format!("sending {}", file_name.unwrap_or_default()),
                    );
                }
                ServiceEvent::SendOfferAnswered {
                    transfer_id,
                    accepted_file_ids,
                    refused_file_ids,
                } => {
                    let statuses = self.file_status.entry(transfer_id).or_default();
                    for file_id in accepted_file_ids {
                        statuses.insert(file_id.to_string(), "accepted".to_string());
                    }
                    for file_id in refused_file_ids {
                        statuses.insert(file_id.to_string(), "skipped".to_string());
                    }
                }
                ServiceEvent::TransferPrepared { transfer, skipped } => {
                    self.skipped_paths = skipped
                        .iter()
                        .map(|entry| format!("{}: {}", entry.path.display(), entry.reason))
                        .collect();
                    self.status = format!(
                        "Prepared {} file(s) for {}; skipped {}",
                        transfer.files.len(),
                        transfer.peer_alias,
                        skipped.len()
                    );
                }
                ServiceEvent::TransferFinished { transfer } => {
                    if self
                        .pending_send
                        .as_ref()
                        .is_some_and(|(id, _)| id == &transfer.transfer_id)
                        && transfer.status == TransferStatus::Finished
                    {
                        self.pending_send = None;
                    }
                    self.status =
                        format!("Transfer {}: {:?}", transfer.transfer_id, transfer.status);
                }
                ServiceEvent::TransferFailed {
                    transfer_id,
                    file_id,
                    error_kind,
                    error,
                } => {
                    self.status = error.clone();
                    if let Some(file_id) = file_id {
                        self.file_status
                            .entry(transfer_id.clone())
                            .or_default()
                            .insert(file_id.to_string(), "failed".to_string());
                    }
                    if matches!(
                        error_kind,
                        Some(ServiceErrorKind::InvalidPin | ServiceErrorKind::PinRequired)
                    ) && let Some((id, pending)) = self.pending_send.take()
                        && id == transfer_id
                    {
                        self.retry_send = Some(pending);
                        self.input.clear();
                        self.input_mode = Some(InputMode::SendPin);
                        self.status = "Receiver requires a PIN".to_string();
                    } else if self
                        .pending_send
                        .as_ref()
                        .is_some_and(|(id, _)| id == &transfer_id)
                    {
                        self.pending_send = None;
                    }
                }
                ServiceEvent::WebShareRequested { ip, session_id, .. } => {
                    self.status = format!("Web Share request from {ip} ({session_id})");
                }
                _ => {}
            }
        }
        if self.connected && self.client.is_connection_closed() {
            self.connected = false;
            self.reconnect_at = Instant::now() + Duration::from_secs(1);
            self.status = "Daemon stopped; reconnect pending".to_string();
            changed = true;
        }
        if changed {
            match self.client.command(ServiceCommand::GetSnapshot).await {
                Ok(CommandResult::Snapshot { snapshot }) => self.snapshot = snapshot,
                Ok(_) => {}
                Err(error) => {
                    self.connected = false;
                    self.reconnect_at = Instant::now() + Duration::from_secs(1);
                    self.status = format!("Refresh failed: {error}");
                }
            }
        }
        self.sync_request_selection();
        self.selected_transfer = self
            .selected_transfer
            .min(self.snapshot.active_transfers.len().saturating_sub(1));
        self.history_cursor = self
            .history_cursor
            .min(self.snapshot.history.len().saturating_sub(1));
    }

    fn sync_request_selection(&mut self) {
        let Some(request) = self.snapshot.incoming_requests.first() else {
            self.request_selected.clear();
            self.request_cursor = 0;
            return;
        };
        let valid: HashSet<String> = request
            .files
            .iter()
            .map(|file| file.id.to_string())
            .collect();
        self.request_selected.retain(|id| valid.contains(id));
        if self.request_selected.is_empty() {
            self.request_selected.extend(valid);
        }
        self.request_cursor = self
            .request_cursor
            .min(request.files.len().saturating_sub(1));
    }

    fn visible_devices(&self) -> Vec<DeviceView> {
        let mut devices: Vec<_> = self
            .snapshot
            .devices
            .iter()
            .filter(|device| device.source != "local")
            .filter(|device| device.device.ip.as_deref() != Some("0.0.0.0"))
            .cloned()
            .collect();
        for favorite in &self.snapshot.favorites {
            if !devices
                .iter()
                .any(|device| device.device.fingerprint == favorite.device.fingerprint)
            {
                devices.push(DeviceView {
                    device: favorite.device.clone(),
                    last_seen_ms: favorite.added_at_ms,
                    source: "favorite".to_string(),
                });
            }
        }
        devices
    }

    fn selected_device(&self) -> Option<DeviceView> {
        self.visible_devices().get(self.selected_device).cloned()
    }

    fn set_tab(&mut self, tab: Tab) {
        self.tab = tab;
        self.selected_device = self
            .selected_device
            .min(self.visible_devices().len().saturating_sub(1));
        self.selected_setting = 0;
        self.input_mode = None;
    }

    async fn command(&mut self, command: ServiceCommand, success: &str) {
        match self.client.command(command).await {
            Ok(result) => {
                self.status = success.to_string();
                if let CommandResult::Snapshot { snapshot } = result {
                    self.snapshot = snapshot;
                } else if let Ok(CommandResult::Snapshot { snapshot }) =
                    self.client.command(ServiceCommand::GetSnapshot).await
                {
                    self.snapshot = snapshot;
                }
            }
            Err(error) => {
                self.connected = false;
                self.reconnect_at = Instant::now() + Duration::from_secs(1);
                self.status = error;
            }
        }
    }

    async fn submit_input(&mut self) {
        let Some(mode) = self.input_mode.take() else {
            return;
        };
        let value = self.input.trim().to_string();
        if value.is_empty() && !matches!(mode, InputMode::FileFilter) {
            self.status = "Input is empty".to_string();
            return;
        }
        match mode {
            InputMode::ManualTarget => {
                self.command(
                    ServiceCommand::AddManualDevice { target: value },
                    "Manual device added",
                )
                .await;
            }
            InputMode::SendPath => {
                let Some(device) = self.selected_device() else {
                    self.status = "Select a device first".to_string();
                    return;
                };
                self.start_file_send(device.device.clone(), vec![PathBuf::from(value)], None)
                    .await;
            }
            InputMode::SendText => {
                let Some(device) = self.selected_device() else {
                    self.status = "Select a device first".to_string();
                    return;
                };
                self.start_text_send(device.device.clone(), value, None)
                    .await;
            }
            InputMode::SendPin => {
                let Some(pending) = self.retry_send.take() else {
                    self.status = "No send is waiting for a PIN".to_string();
                    return;
                };
                match pending {
                    PendingSend::Files { target, paths } => {
                        self.start_file_send(target, paths, Some(value)).await;
                    }
                    PendingSend::Text { target, text } => {
                        self.start_text_send(target, text, Some(value)).await;
                    }
                }
            }
            InputMode::FileFilter => {
                if let Some(browser) = self.file_browser.as_mut() {
                    browser.filter = value;
                    browser.reload();
                }
            }
            InputMode::EditAlias => {
                self.update_settings(|settings| settings.alias = value)
                    .await
            }
            InputMode::EditPort => match value.parse::<u16>() {
                Ok(port) if port > 0 => {
                    self.update_settings(|settings| settings.port = port).await;
                }
                Err(_) => self.status = "Port must be a number".to_string(),
                Ok(_) => self.status = "Port must be between 1 and 65535".to_string(),
            },
            InputMode::EditPin => {
                let pin = if value == "-" { None } else { Some(value) };
                self.update_settings(|settings| settings.pin = pin).await;
            }
            InputMode::EditDestination => {
                self.update_settings(|settings| {
                    settings.destination_dir = if value == "-" {
                        None
                    } else {
                        Some(PathBuf::from(value))
                    }
                })
                .await;
            }
            InputMode::EditDiscoveryTimeout => match value.parse::<u64>() {
                Ok(timeout) if timeout > 0 => {
                    self.update_settings(|settings| settings.discovery_timeout_ms = timeout)
                        .await
                }
                _ => self.status = "Discovery timeout must be a positive integer".to_string(),
            },
            InputMode::EditMulticastGroup => {
                if value.parse::<std::net::Ipv4Addr>().is_ok() {
                    self.update_settings(|settings| settings.multicast_group = Some(value))
                        .await;
                } else {
                    self.status = "Multicast group must be an IPv4 address".to_string();
                }
            }
            InputMode::EditWhitelist => {
                let patterns = parse_patterns(&value);
                self.update_settings(|settings| settings.interface_whitelist = patterns)
                    .await;
            }
            InputMode::EditBlacklist => {
                let patterns = parse_patterns(&value);
                self.update_settings(|settings| settings.interface_blacklist = patterns)
                    .await;
            }
            InputMode::EditFavoriteAlias => {
                let Some(device) = self.selected_device() else {
                    self.status = "Select a device first".to_string();
                    return;
                };
                self.command(
                    ServiceCommand::AddFavorite {
                        fingerprint: device.device.fingerprint.clone(),
                        alias: Some(value),
                        device: Some(device.device),
                    },
                    "Favorite alias saved",
                )
                .await;
            }
        }
    }

    async fn start_file_send(
        &mut self,
        target: DeviceInfo,
        paths: Vec<PathBuf>,
        pin: Option<String>,
    ) {
        if paths.is_empty() {
            self.status = "Select at least one file or directory".to_string();
            return;
        }
        let pending = PendingSend::Files {
            target: target.clone(),
            paths: paths.clone(),
        };
        match self
            .client
            .command(ServiceCommand::SendFiles {
                target_fingerprint: Some(target.fingerprint),
                target: None,
                files: paths
                    .into_iter()
                    .map(|path| SendFileRequest {
                        path,
                        file_id: None,
                        sha256: None,
                        metadata: None,
                    })
                    .collect(),
                pin,
            })
            .await
        {
            Ok(CommandResult::Transfer { transfer }) => {
                self.pending_send = Some((transfer.transfer_id, pending));
                self.status = "File send started".to_string();
            }
            Ok(_) => self.status = "Daemon did not return a transfer".to_string(),
            Err(error) => self.status = error,
        }
    }

    async fn start_text_send(&mut self, target: DeviceInfo, text: String, pin: Option<String>) {
        if text.is_empty() {
            self.status = "Text is empty".to_string();
            return;
        }
        let pending = PendingSend::Text {
            target: target.clone(),
            text: text.clone(),
        };
        match self
            .client
            .command(ServiceCommand::SendText {
                target_fingerprint: Some(target.fingerprint),
                target: None,
                text,
                pin,
            })
            .await
        {
            Ok(CommandResult::Transfer { transfer }) => {
                self.pending_send = Some((transfer.transfer_id, pending));
                self.status = "Text send started".to_string();
            }
            Ok(_) => self.status = "Daemon did not return a transfer".to_string(),
            Err(error) => self.status = error,
        }
    }

    async fn update_settings(&mut self, update: impl FnOnce(&mut DesktopSettings)) {
        let mut settings = self.snapshot.settings.clone();
        update(&mut settings);
        self.command(
            ServiceCommand::UpdateSettings {
                expected_revision: self.snapshot.settings.revision,
                settings,
            },
            "Settings saved",
        )
        .await;
    }

    async fn handle_key(&mut self, key: KeyEvent) {
        if self.input_mode.is_some() {
            if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                self.should_quit = true;
                return;
            }
            match key.code {
                KeyCode::Esc => {
                    self.input_mode = None;
                    self.input.clear();
                }
                KeyCode::Enter => self.submit_input().await,
                KeyCode::Backspace => {
                    self.input.pop();
                }
                KeyCode::Char(character) => self.input.push(character),
                _ => {}
            }
            return;
        }

        if self.file_browser.is_some() {
            self.handle_file_browser_key(key).await;
            return;
        }

        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.should_quit = true;
            return;
        }
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('q') {
            return;
        }

        match key.code {
            KeyCode::Char('1') => self.set_tab(Tab::Receive),
            KeyCode::Char('2') => self.set_tab(Tab::Send),
            KeyCode::Char('3') => self.set_tab(Tab::Settings),
            KeyCode::Tab => {
                self.set_tab(match self.tab {
                    Tab::Receive => Tab::Send,
                    Tab::Send => Tab::Settings,
                    Tab::Settings => Tab::Receive,
                });
            }
            KeyCode::BackTab => {
                self.set_tab(match self.tab {
                    Tab::Receive => Tab::Settings,
                    Tab::Send => Tab::Receive,
                    Tab::Settings => Tab::Send,
                });
            }
            KeyCode::Char('?') => self.show_help = !self.show_help,
            KeyCode::Esc => self.show_help = false,
            KeyCode::Char('r') => {
                self.command(ServiceCommand::RefreshDiscovery, "Discovery refreshed")
                    .await;
            }
            KeyCode::Char('s') if self.tab == Tab::Settings => {
                let command = if self.snapshot.server_running {
                    ServiceCommand::StopServer
                } else {
                    ServiceCommand::StartServer
                };
                self.command(command, "Server state updated").await;
            }
            KeyCode::Char('h') if self.tab == Tab::Settings => {
                self.update_settings(|settings| {
                    settings.protocol = if matches!(settings.protocol, Protocol::Https) {
                        Protocol::Http
                    } else {
                        Protocol::Https
                    };
                })
                .await;
            }
            KeyCode::Char('a') if self.tab == Tab::Settings => {
                self.update_settings(|settings| {
                    settings.auto_save = match settings.auto_save {
                        AutoSaveMode::Off => AutoSaveMode::Favorites,
                        AutoSaveMode::Favorites => AutoSaveMode::On,
                        AutoSaveMode::On => AutoSaveMode::Off,
                    };
                })
                .await;
            }
            KeyCode::Char('m') => {
                self.input.clear();
                self.input_mode = Some(InputMode::ManualTarget);
            }
            KeyCode::Char('f') if self.tab == Tab::Send => {
                let start = std::env::var_os("HOME")
                    .or_else(|| std::env::var_os("USERPROFILE"))
                    .map(PathBuf::from)
                    .unwrap_or_else(|| PathBuf::from("."));
                self.file_browser = Some(FileBrowser::new(start));
            }
            KeyCode::Char('t') if self.tab == Tab::Send => {
                self.input.clear();
                self.input_mode = Some(InputMode::SendText);
            }
            KeyCode::Char('l')
                if self.tab == Tab::Send && key.modifiers.contains(KeyModifiers::CONTROL) =>
            {
                self.input.clear();
                self.input_mode = Some(InputMode::SendPath);
            }
            KeyCode::Char('F') if self.tab == Tab::Send => {
                self.toggle_selected_favorite().await;
            }
            KeyCode::Char('e') if self.tab == Tab::Send => {
                if self.selected_device().is_some() {
                    self.input.clear();
                    self.input_mode = Some(InputMode::EditFavoriteAlias);
                }
            }
            KeyCode::Char(' ') if self.tab == Tab::Receive => self.toggle_request_file(),
            KeyCode::Char('y') if self.tab == Tab::Receive => self.accept_selected_request().await,
            KeyCode::Char('Y') if self.tab == Tab::Receive => {
                if let Some(request) = self.snapshot.incoming_requests.first() {
                    self.command(
                        ServiceCommand::RespondTransfer {
                            request_id: request.request_id.clone(),
                            decision: ReceiverDecision::AcceptAll,
                        },
                        "Transfer accepted",
                    )
                    .await;
                }
            }
            KeyCode::Char('n') if self.tab == Tab::Receive => self.decline_request().await,
            KeyCode::Char('x') if matches!(self.tab, Tab::Receive | Tab::Send) => {
                self.cancel_selected_transfer().await;
            }
            KeyCode::Char('c') if self.tab == Tab::Receive => self.clear_history().await,
            KeyCode::PageUp | KeyCode::Char('[') => {
                self.history_cursor = self.history_cursor.saturating_sub(1);
            }
            KeyCode::PageDown | KeyCode::Char(']') => {
                let max = self.snapshot.history.len().saturating_sub(1);
                self.history_cursor = (self.history_cursor + 1).min(max);
            }
            KeyCode::Enter if self.tab == Tab::Receive => self.reveal_selected_history(),
            KeyCode::Up | KeyCode::Char('k') => match self.tab {
                Tab::Send => {
                    self.selected_device = self.selected_device.saturating_sub(1);
                }
                Tab::Settings => {
                    self.selected_setting = self.selected_setting.saturating_sub(1);
                }
                Tab::Receive => {
                    self.request_cursor = self.request_cursor.saturating_sub(1);
                }
            },
            KeyCode::Down | KeyCode::Char('j') => match self.tab {
                Tab::Send => {
                    let max = self.visible_devices().len().saturating_sub(1);
                    self.selected_device = (self.selected_device + 1).min(max);
                }
                Tab::Settings => {
                    self.selected_setting = (self.selected_setting + 1).min(9);
                }
                Tab::Receive => {
                    let max = self
                        .snapshot
                        .incoming_requests
                        .first()
                        .map(|request| request.files.len().saturating_sub(1))
                        .unwrap_or(0);
                    self.request_cursor = (self.request_cursor + 1).min(max);
                }
            },
            KeyCode::Enter if self.tab == Tab::Settings => {
                match self.selected_setting {
                    2 => {
                        self.update_settings(|settings| {
                            settings.protocol = match settings.protocol {
                                Protocol::Https => Protocol::Http,
                                Protocol::Http => Protocol::Https,
                            };
                        })
                        .await;
                        return;
                    }
                    5 => {
                        self.update_settings(|settings| {
                            settings.auto_save = match settings.auto_save {
                                AutoSaveMode::Off => AutoSaveMode::Favorites,
                                AutoSaveMode::Favorites => AutoSaveMode::On,
                                AutoSaveMode::On => AutoSaveMode::Off,
                            };
                        })
                        .await;
                        return;
                    }
                    _ => {}
                }
                self.input.clear();
                self.input_mode = Some(match self.selected_setting {
                    0 => InputMode::EditAlias,
                    1 => InputMode::EditPort,
                    3 => InputMode::EditPin,
                    4 => InputMode::EditDestination,
                    6 => InputMode::EditDiscoveryTimeout,
                    7 => InputMode::EditMulticastGroup,
                    8 => InputMode::EditWhitelist,
                    _ => InputMode::EditBlacklist,
                });
            }
            _ => {}
        }
    }

    fn toggle_request_file(&mut self) {
        let Some(request) = self.snapshot.incoming_requests.first() else {
            return;
        };
        let Some(file) = request.files.get(self.request_cursor) else {
            return;
        };
        let id = file.id.to_string();
        if !self.request_selected.remove(&id) {
            self.request_selected.insert(id);
        }
    }

    async fn accept_selected_request(&mut self) {
        let Some(request) = self.snapshot.incoming_requests.first() else {
            self.status = "No pending request".to_string();
            return;
        };
        if self.request_selected.is_empty() {
            self.status = "Select at least one file".to_string();
            return;
        }
        let file_ids = self.request_selected.iter().cloned().collect();
        self.command(
            ServiceCommand::RespondTransfer {
                request_id: request.request_id.clone(),
                decision: ReceiverDecision::AcceptFiles { file_ids },
            },
            "Selected files accepted",
        )
        .await;
    }

    async fn decline_request(&mut self) {
        let Some(request) = self.snapshot.incoming_requests.first() else {
            self.status = "No pending request".to_string();
            return;
        };
        self.command(
            ServiceCommand::RespondTransfer {
                request_id: request.request_id.clone(),
                decision: ReceiverDecision::Decline,
            },
            "Transfer declined",
        )
        .await;
    }

    async fn cancel_selected_transfer(&mut self) {
        let transfer = self
            .snapshot
            .active_transfers
            .get(self.selected_transfer)
            .or_else(|| self.snapshot.active_transfers.first());
        let Some(transfer) = transfer else {
            self.status = "No active transfer".to_string();
            return;
        };
        self.command(
            ServiceCommand::CancelTransfer {
                transfer_id: transfer.transfer_id.clone(),
            },
            "Transfer cancellation requested",
        )
        .await;
    }

    async fn clear_history(&mut self) {
        self.command(ServiceCommand::ClearHistory, "History cleared")
            .await;
    }

    fn reveal_selected_history(&mut self) {
        let Some(path) = self
            .snapshot
            .history
            .iter()
            .rev()
            .nth(self.history_cursor)
            .and_then(|entry| entry.path.as_deref())
        else {
            self.status = "Selected history item has no file path".to_string();
            return;
        };
        match reveal_path(path) {
            Ok(()) => self.status = "Revealed in file manager".to_string(),
            Err(error) => self.status = format!("Could not reveal file: {error}"),
        }
    }

    async fn toggle_selected_favorite(&mut self) {
        let Some(device) = self.selected_device() else {
            self.status = "Select a device first".to_string();
            return;
        };
        let existing = self
            .snapshot
            .favorites
            .iter()
            .find(|favorite| favorite.device.fingerprint == device.device.fingerprint);
        if existing.is_some() {
            self.command(
                ServiceCommand::RemoveFavorite {
                    fingerprint: device.device.fingerprint,
                },
                "Favorite removed",
            )
            .await;
        } else {
            self.command(
                ServiceCommand::AddFavorite {
                    fingerprint: device.device.fingerprint.clone(),
                    alias: None,
                    device: Some(device.device),
                },
                "Favorite added",
            )
            .await;
        }
    }

    async fn handle_file_browser_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('s') {
            let Some(device) = self.selected_device() else {
                self.status = "Select a device first".to_string();
                return;
            };
            let paths = self
                .file_browser
                .as_ref()
                .map(FileBrowser::selected_paths)
                .unwrap_or_default();
            if paths.is_empty() {
                self.status = "Select at least one file or directory".to_string();
                return;
            }
            self.file_browser = None;
            self.start_file_send(device.device.clone(), paths, None)
                .await;
            return;
        }

        let Some(browser) = self.file_browser.as_mut() else {
            return;
        };
        match key.code {
            KeyCode::Esc => self.file_browser = None,
            KeyCode::Up | KeyCode::Char('k') => browser.move_cursor(-1),
            KeyCode::Down | KeyCode::Char('j') => browser.move_cursor(1),
            KeyCode::Home | KeyCode::Char('g') => browser.cursor = 0,
            KeyCode::End | KeyCode::Char('G') => {
                browser.cursor = browser.entries.len().saturating_sub(1)
            }
            KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                browser.show_hidden = !browser.show_hidden;
                browser.reload();
            }
            KeyCode::Backspace | KeyCode::Left | KeyCode::Char('h') => {
                browser.parent();
            }
            KeyCode::Right | KeyCode::Char('l') => {
                browser.enter();
            }
            KeyCode::Enter => {
                if !browser.enter() {
                    browser.toggle();
                }
            }
            KeyCode::Char(' ') => browser.toggle(),
            KeyCode::Char('a') => browser.toggle_all(),
            KeyCode::Char('/') => {
                self.input.clear();
                self.input_mode = Some(InputMode::FileFilter);
            }
            _ => {}
        }
    }
}

fn parse_patterns(value: &str) -> Option<Vec<String>> {
    let patterns: Vec<String> = value
        .split([',', ';', ' ', '\n'])
        .map(str::trim)
        .filter(|pattern| !pattern.is_empty())
        .map(ToOwned::to_owned)
        .collect();
    (!patterns.is_empty()).then_some(patterns)
}

fn reveal_path(path: &Path) -> std::io::Result<()> {
    use std::process::Command;

    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg("-R").arg(path).spawn()?;
    }
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(format!("/select,{}", path.display()))
            .spawn()?;
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Command::new("xdg-open")
            .arg(path.parent().unwrap_or(path))
            .spawn()?;
    }
    Ok(())
}

pub async fn run_tui(options: TuiOptions) -> anyhow::Result<()> {
    let client = DaemonClient::connect_interactive()
        .await
        .context("connecting to localsendd; start localsend-tray or localsendd first")?;
    let mut snapshot = client
        .snapshot()
        .context("daemon did not provide a snapshot")?;
    if options != TuiOptions::default() {
        let server_running = snapshot.server_running;
        let mut settings = snapshot.settings.clone();
        if let Some(alias) = options.alias {
            settings.alias = alias;
        }
        if let Some(port) = options.port {
            settings.port = port;
        }
        if let Some(protocol) = options.protocol {
            settings.protocol = protocol;
        }
        if let Some(pin) = options.pin {
            settings.pin = pin;
        }
        if let Some(auto_accept) = options.auto_accept {
            settings.auto_accept = auto_accept;
        }
        client
            .command(ServiceCommand::UpdateSettings {
                expected_revision: snapshot.settings.revision,
                settings,
            })
            .await
            .map_err(anyhow::Error::msg)
            .context("applying TUI startup options")?;
        if server_running {
            client
                .command(ServiceCommand::RestartServer)
                .await
                .map_err(anyhow::Error::msg)
                .context("restarting localsendd with TUI startup options")?;
        }
        snapshot = match client
            .command(ServiceCommand::GetSnapshot)
            .await
            .map_err(anyhow::Error::msg)?
        {
            CommandResult::Snapshot { snapshot } => snapshot,
            _ => anyhow::bail!("daemon did not return a snapshot after TUI options"),
        };
    }
    let mut app = App::new(client, snapshot);
    let mut terminal = init_terminal()?;
    let result = run_ui(&mut terminal, &mut app).await;
    restore_terminal(&mut terminal)?;
    result
}

fn init_terminal() -> anyhow::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableBracketedPaste)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend).map_err(Into::into)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> anyhow::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        DisableBracketedPaste,
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;
    Ok(())
}

async fn run_ui(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
) -> anyhow::Result<()> {
    loop {
        app.refresh().await;
        terminal.draw(|frame| draw(frame, app))?;
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    if app.should_quit
                        || (key.code == KeyCode::Char('q') && app.input_mode.is_none())
                    {
                        break;
                    }
                    app.handle_key(key).await;
                }
                Event::Paste(text) => {
                    if app.input_mode.is_some() {
                        app.input.push_str(&text);
                    } else {
                        app.status = "Open a text or path input before pasting".to_string();
                    }
                }
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
    }
    Ok(())
}

fn draw(frame: &mut Frame, app: &App) {
    let terminal_area = frame.area();
    if terminal_area.width < 60 || terminal_area.height < 20 {
        frame.render_widget(
            Paragraph::new(format!(
                "Terminal too small\n{}x{}\nminimum 60x20",
                terminal_area.width, terminal_area.height
            ))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("LocalSend TUI"),
            ),
            terminal_area,
        );
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
            Constraint::Length(2),
        ])
        .split(frame.area());
    let titles = ["1 Receive", "2 Send", "3 Settings"];
    let selected = match app.tab {
        Tab::Receive => 0,
        Tab::Send => 1,
        Tab::Settings => 2,
    };
    frame.render_widget(
        Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("LocalSend PC"))
            .select(selected)
            .highlight_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        chunks[0],
    );

    match app.tab {
        Tab::Receive => draw_receive(frame, app, chunks[1]),
        Tab::Send => draw_send(frame, app, chunks[1]),
        Tab::Settings => draw_settings(frame, app, chunks[1]),
    }

    let status = format!(
        "daemon {} | {} | server {}:{} | {}",
        app.snapshot.daemon_pid,
        if app.connected {
            "connected"
        } else {
            "reconnecting"
        },
        if app.snapshot.server_running {
            "running"
        } else {
            "stopped"
        },
        app.snapshot.server_port.unwrap_or(0),
        app.status
    );
    frame.render_widget(
        Paragraph::new(status).block(Block::default().borders(Borders::ALL).title("Status")),
        chunks[2],
    );
    frame.render_widget(
        Paragraph::new(match app.tab {
            Tab::Receive => {
                "j/k file | Space toggle | y/Y accept | n decline | PgUp/PgDn history | Enter open"
            }
            Tab::Send => {
                "j/k device | m manual IP | f browser | Ctrl+L path | t text | F favorite | e alias"
            }
            Tab::Settings => {
                "j/k setting | Enter edit | s server | h HTTPS | a auto-save | ? help | q quit"
            }
        })
        .style(Style::default().fg(Color::DarkGray)),
        chunks[3],
    );

    if let Some(mode) = &app.input_mode {
        draw_input(frame, mode, &app.input);
    }
    if app.show_help {
        draw_help(frame);
    }
    if let Some(browser) = &app.file_browser {
        let target = app
            .selected_device()
            .map(|device| device.device.alias)
            .unwrap_or_else(|| "<select a Send-page device first>".to_string());
        draw_file_browser(frame, browser, &target);
    }
}

fn draw_receive(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Min(4),
            Constraint::Percentage(30),
        ])
        .split(area);
    let request_text = if let Some(request) = app.snapshot.incoming_requests.first() {
        let mut lines = vec![Line::from(format!(
            "From: {} ({})",
            request.sender.alias, request.sender.fingerprint
        ))];
        for (index, file) in request.files.iter().enumerate() {
            let selected = app.request_selected.contains(&file.id.to_string());
            let marker = if index == app.request_cursor {
                ">"
            } else {
                " "
            };
            lines.push(Line::from(format!(
                "{marker} [{}] {} ({} bytes)",
                if selected { "x" } else { " " },
                file.file_name,
                file.size
            )));
        }
        Text::from(lines)
    } else {
        Text::from(vec![Line::from("No pending transfer request.")])
    };
    frame.render_widget(
        Paragraph::new(request_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Incoming Request"),
            )
            .wrap(Wrap { trim: true }),
        chunks[0],
    );
    let transfers: Vec<ListItem> = app
        .snapshot
        .active_transfers
        .iter()
        .enumerate()
        .map(|(index, transfer)| {
            let percent = transfer
                .bytes_done
                .saturating_mul(100)
                .checked_div(transfer.bytes_total)
                .unwrap_or(0);
            let files = app
                .file_status
                .get(&transfer.transfer_id)
                .map(|states| states.values().cloned().collect::<Vec<_>>().join(", "))
                .unwrap_or_default();
            ListItem::new(format!(
                "{} {} -> {} | {:?} | {}% | {}",
                if index == app.selected_transfer {
                    ">"
                } else {
                    " "
                },
                match transfer.direction {
                    TransferDirection::Send => "send",
                    TransferDirection::Receive => "receive",
                },
                transfer.peer_alias,
                transfer.status,
                percent,
                files
            ))
        })
        .collect();
    frame.render_widget(
        List::new(transfers).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Active Transfers"),
        ),
        chunks[1],
    );
    let reversed: Vec<_> = app.snapshot.history.iter().rev().collect();
    let start = app.history_cursor.saturating_sub(4);
    let mut history_lines = Vec::new();
    for (index, entry) in reversed.iter().enumerate().skip(start).take(8) {
        let kind = if entry.message.is_some() {
            "message"
        } else {
            "file"
        };
        history_lines.push(Line::from(format!(
            "{} {kind} | {} | {} | {:?}",
            if index == app.history_cursor {
                ">"
            } else {
                " "
            },
            entry.peer_alias,
            entry.file_name,
            entry.status
        )));
    }
    if let Some(entry) = reversed.get(app.history_cursor) {
        history_lines.push(Line::from(""));
        if let Some(message) = &entry.message {
            history_lines.push(Line::from(format!("Message: {message}")));
        } else if let Some(path) = &entry.path {
            history_lines.push(Line::from(format!("Path: {}", path.display())));
        }
    }
    frame.render_widget(
        Paragraph::new(history_lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Recent History (PgUp/PgDn)"),
            )
            .wrap(Wrap { trim: false }),
        chunks[2],
    );
}

fn draw_send(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = if area.width >= 100 {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(48), Constraint::Percentage(52)])
            .split(area)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
            .split(area)
    };
    let devices = app.visible_devices();
    let items: Vec<ListItem> = devices
        .iter()
        .enumerate()
        .map(|(index, device)| {
            let marker = if index == app.selected_device {
                ">"
            } else {
                " "
            };
            ListItem::new(format!(
                "{marker} {} | {} | {}",
                device.device.alias,
                device.device.ip.as_deref().unwrap_or("?"),
                device.device.fingerprint
            ))
        })
        .collect();
    frame.render_widget(
        List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Nearby Devices"),
            )
            .highlight_style(Style::default().fg(Color::Cyan)),
        chunks[0],
    );
    let selected = app
        .selected_device()
        .map(|device| format!("Selected: {}", device.device.alias))
        .unwrap_or_else(|| "Select a device with j/k".to_string());
    let transfer_text = app
        .snapshot
        .active_transfers
        .iter()
        .filter(|transfer| transfer.direction == TransferDirection::Send)
        .map(|transfer| {
            let percent = transfer
                .bytes_done
                .saturating_mul(100)
                .checked_div(transfer.bytes_total)
                .unwrap_or(0);
            format!(
                "{}: {}% | {:?} | {} files",
                transfer.peer_alias,
                percent,
                transfer.status,
                transfer.files.len()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let mut detail = vec![
        Line::from(selected),
        Line::from(""),
        Line::from("Favorites:"),
    ];
    if app.snapshot.favorites.is_empty() {
        detail.push(Line::from("  none"));
    } else {
        for favorite in &app.snapshot.favorites {
            detail.push(Line::from(format!(
                "  {} | {}",
                favorite.alias,
                favorite.device.ip.as_deref().unwrap_or("?")
            )));
        }
    }
    detail.push(Line::from(""));
    detail.push(Line::from(transfer_text));
    if !app.skipped_paths.is_empty() {
        detail.push(Line::from(""));
        detail.push(Line::from("Skipped paths:"));
        for skipped in app.skipped_paths.iter().take(5) {
            detail.push(Line::from(format!("  {skipped}")));
        }
    }
    frame.render_widget(
        Paragraph::new(detail)
            .block(Block::default().borders(Borders::ALL).title("Send"))
            .wrap(Wrap { trim: true }),
        chunks[1],
    );
}

fn draw_settings(frame: &mut Frame, app: &App, area: Rect) {
    let settings = &app.snapshot.settings;
    let rows = [
        format!("Alias: {}", settings.alias),
        format!("Port: {}", settings.port),
        format!(
            "Protocol: {}",
            match settings.protocol {
                Protocol::Https => "HTTPS",
                Protocol::Http => "HTTP",
            }
        ),
        format!("PIN: {}", settings.pin.as_deref().unwrap_or("<none>")),
        format!(
            "Destination: {}",
            settings
                .destination_dir
                .as_ref()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| "<system downloads>".to_string())
        ),
        format!("Auto-save: {:?}", settings.auto_save),
        format!("Discovery timeout: {} ms", settings.discovery_timeout_ms),
        format!(
            "Multicast group: {}",
            settings.multicast_group.as_deref().unwrap_or("<default>")
        ),
        format!(
            "Interface whitelist: {}",
            settings
                .interface_whitelist
                .as_ref()
                .map(|items| items.join(","))
                .unwrap_or_else(|| "<none>".to_string())
        ),
        format!(
            "Interface blacklist: {}",
            settings
                .interface_blacklist
                .as_ref()
                .map(|items| items.join(","))
                .unwrap_or_else(|| "<none>".to_string())
        ),
    ];
    let items: Vec<ListItem> = rows
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let marker = if index == app.selected_setting {
                ">"
            } else {
                " "
            };
            ListItem::new(format!("{marker} {row}"))
        })
        .collect();
    frame.render_widget(
        List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Settings"))
            .highlight_style(Style::default().fg(Color::Cyan)),
        area,
    );
}

fn draw_input(frame: &mut Frame, mode: &InputMode, input: &str) {
    let title = match mode {
        InputMode::ManualTarget => "Manual IP / host:port",
        InputMode::SendPath => "File or directory path",
        InputMode::SendText => "Text message (Enter submits)",
        InputMode::SendPin => "Receiver PIN",
        InputMode::FileFilter => "File browser filter (empty clears)",
        InputMode::EditAlias => "Alias",
        InputMode::EditPort => "Port",
        InputMode::EditPin => "PIN (use - to clear)",
        InputMode::EditDestination => "Receive directory (use - for default)",
        InputMode::EditDiscoveryTimeout => "Discovery timeout milliseconds",
        InputMode::EditMulticastGroup => "Multicast group (IPv4)",
        InputMode::EditWhitelist => "Interface whitelist (comma separated)",
        InputMode::EditBlacklist => "Interface blacklist (comma separated)",
        InputMode::EditFavoriteAlias => "Favorite alias",
    };
    let area = centered_rect(60, 12, frame.area());
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(input)
            .block(Block::default().borders(Borders::ALL).title(title))
            .wrap(Wrap { trim: false }),
        area,
    );
    frame.set_cursor_position((area.x + 1 + input.len() as u16, area.y + 1));
}

fn draw_help(frame: &mut Frame) {
    let area = centered_rect(70, 16, frame.area());
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(vec![
            Line::from("1/2/3   Receive / Send / Settings"),
            Line::from("Tab     Next page"),
            Line::from("j/k     Move selection"),
            Line::from("r       Refresh discovery"),
            Line::from("m       Add a manual host:port"),
            Line::from("f       Open the file browser"),
            Line::from("t       Send a text message"),
            Line::from("y/Y/n   Accept selected / all / decline"),
            Line::from("x       Cancel the selected active transfer"),
            Line::from("q       Quit the TUI; localsendd keeps running"),
            Line::from("?       Close this help"),
        ])
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .wrap(Wrap { trim: true }),
        area,
    );
}

fn draw_file_browser(frame: &mut Frame, browser: &FileBrowser, target: &str) {
    let area = centered_rect(90, frame.area().height.saturating_sub(4), frame.area());
    frame.render_widget(Clear, area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(4),
            Constraint::Length(2),
        ])
        .split(area);
    let header = format!(
        "Target: {target} | {} | filter={} | hidden={} | selected={}",
        browser.current_dir.display(),
        if browser.filter.is_empty() {
            "<none>"
        } else {
            &browser.filter
        },
        browser.show_hidden,
        browser.selected.len()
    );
    frame.render_widget(
        Paragraph::new(header).block(Block::default().borders(Borders::ALL).title("File Browser")),
        chunks[0],
    );
    let items: Vec<ListItem> = browser
        .entries
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            ListItem::new(format!(
                "{} [{}] {}{}",
                if index == browser.cursor { ">" } else { " " },
                if entry.selected { "x" } else { " " },
                entry.name,
                if entry.is_dir { "/" } else { "" }
            ))
        })
        .collect();
    frame.render_widget(
        List::new(items)
            .block(
                Block::default().borders(Borders::ALL).title(
                    browser
                        .error
                        .as_deref()
                        .unwrap_or("Enter open/toggle | s send"),
                ),
            )
            .highlight_style(Style::default().fg(Color::Cyan)),
        chunks[1],
    );
    frame.render_widget(
        Paragraph::new(
            "j/k move | Enter open/toggle | Space select | a all | / filter | Ctrl+H hidden | s SEND selected | Esc close",
        )
        .style(Style::default().fg(Color::DarkGray)),
        chunks[2],
    );
}

fn centered_rect(percent_x: u16, height: u16, area: Rect) -> Rect {
    let width = area.width * percent_x / 100;
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width.max(20), height.min(area.height))
}

#[cfg(test)]
mod tests {
    use super::*;
    use localsend_testkit::{DaemonFixture, daemon_binary_for_tests, default_daemon_settings};
    use ratatui::backend::TestBackend;

    #[test]
    fn file_browser_hides_dotfiles_until_requested_and_selects_paths() {
        let root = tempfile::tempdir().expect("temp dir");
        std::fs::create_dir(root.path().join("folder")).expect("create folder");
        std::fs::write(root.path().join("visible.txt"), b"visible").expect("write visible");
        std::fs::write(root.path().join(".hidden"), b"hidden").expect("write hidden");

        let mut browser = FileBrowser::new(root.path().to_path_buf());
        assert!(browser.entries.iter().any(|entry| entry.name == "folder"));
        assert!(!browser.entries.iter().any(|entry| entry.name == ".hidden"));

        browser.show_hidden = true;
        browser.reload();
        assert!(browser.entries.iter().any(|entry| entry.name == ".hidden"));

        browser.cursor = browser
            .entries
            .iter()
            .position(|entry| entry.name == "visible.txt")
            .expect("visible file");
        browser.toggle();
        assert_eq!(
            browser.selected_paths(),
            vec![root.path().join("visible.txt")]
        );
    }

    #[test]
    fn interface_patterns_clear_when_empty() {
        assert_eq!(parse_patterns(""), None);
        assert_eq!(
            parse_patterns("192.168.*, 10.0.*; !lo"),
            Some(vec![
                "192.168.*".to_string(),
                "10.0.*".to_string(),
                "!lo".to_string()
            ])
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn all_pages_render_at_supported_terminal_sizes() -> anyhow::Result<()> {
        if daemon_binary_for_tests().is_err() && std::env::var_os("LOCALSEND_ACCEPTANCE").is_none()
        {
            return Ok(());
        }
        let mut daemon =
            DaemonFixture::start(&daemon_binary_for_tests()?, default_daemon_settings()).await?;
        let client = daemon.connect_interactive().await?;
        let snapshot = client.snapshot().expect("snapshot");
        let mut app = App::new(client, snapshot);
        for (width, height) in [(60, 20), (80, 24), (120, 40)] {
            for tab in [Tab::Receive, Tab::Send, Tab::Settings] {
                app.tab = tab;
                let backend = TestBackend::new(width, height);
                let mut terminal = Terminal::new(backend)?;
                terminal.draw(|frame| draw(frame, &app))?;
                let content = terminal
                    .backend()
                    .buffer()
                    .content
                    .iter()
                    .map(|cell| cell.symbol())
                    .collect::<String>();
                assert!(!content.trim().is_empty());
                assert!(
                    content.contains("LocalSend PC")
                        || content.contains("Receive")
                        || content.contains("Send")
                );
            }
        }
        daemon.shutdown().await?;
        Ok(())
    }
}

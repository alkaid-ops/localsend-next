use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub const RPC_VERSION: u32 = 2;
pub const DEFAULT_HTTP_PORT: u16 = 53317;
pub const DEFAULT_MULTICAST_ADDRESS: &str = "224.0.0.167";
pub type FileId = String;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Http,
    #[default]
    Https,
}

impl Protocol {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Http => "http",
            Self::Https => "https",
        }
    }
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    Mobile,
    #[default]
    Desktop,
    Web,
    Headless,
    Server,
}

fn default_port() -> u16 {
    DEFAULT_HTTP_PORT
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub alias: String,
    pub version: String,
    #[serde(rename = "deviceModel", default)]
    pub device_model: Option<String>,
    #[serde(rename = "deviceType", default)]
    pub device_type: Option<DeviceType>,
    pub fingerprint: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default)]
    pub protocol: Protocol,
    #[serde(default)]
    pub download: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileMetadataDetails {
    pub modified: Option<String>,
    pub accessed: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileMetadata {
    pub id: FileId,
    #[serde(rename = "fileName")]
    pub file_name: String,
    pub size: u64,
    #[serde(rename = "fileType")]
    pub file_type: String,
    pub sha256: Option<String>,
    pub preview: Option<String>,
    pub metadata: Option<FileMetadataDetails>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DaemonLifecycle {
    Persistent,
    Owned,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum AutoSaveMode {
    #[default]
    Off,
    Favorites,
    On,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct DesktopSettings {
    pub alias: String,
    pub port: u16,
    pub protocol: Protocol,
    pub pin: Option<String>,
    pub destination_dir: Option<PathBuf>,
    pub auto_accept: bool,
    pub auto_save: AutoSaveMode,
    pub auto_finish: bool,
    pub save_to_history: bool,
    pub start_server_on_launch: bool,
    pub discovery_timeout_secs: u64,
    /// Multicast group the GUI asked for. `None` keeps LocalSend's default.
    pub multicast_group: Option<String>,
    /// IP patterns (`*` matches within one octet) of the interfaces to use and
    /// to avoid, mirroring the official app's interface filter.
    pub interface_whitelist: Option<Vec<String>>,
    pub interface_blacklist: Option<Vec<String>>,
    pub theme: String,
    pub color: String,
    pub language: String,
    pub save_window_position: bool,
    pub minimize_to_tray: bool,
    pub launch_on_login: bool,
    pub animations: bool,
    pub history_limit: usize,
    pub revision: u64,
}

impl Default for DesktopSettings {
    fn default() -> Self {
        Self {
            alias: default_alias(),
            port: DEFAULT_HTTP_PORT,
            protocol: Protocol::Https,
            pin: None,
            destination_dir: None,
            auto_accept: false,
            auto_save: AutoSaveMode::Off,
            auto_finish: true,
            save_to_history: true,
            start_server_on_launch: true,
            discovery_timeout_secs: 10,
            multicast_group: None,
            interface_whitelist: None,
            interface_blacklist: None,
            theme: "system".to_string(),
            color: "localsend".to_string(),
            language: "system".to_string(),
            save_window_position: true,
            minimize_to_tray: true,
            launch_on_login: false,
            animations: true,
            history_limit: 500,
            revision: 0,
        }
    }
}

fn default_alias() -> String {
    std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "LocalSend PC".to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeviceView {
    pub device: DeviceInfo,
    pub last_seen_ms: u64,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FavoriteView {
    pub device: DeviceInfo,
    pub alias: String,
    pub added_at_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileView {
    pub id: FileId,
    pub file_name: String,
    pub size: u64,
    pub file_type: String,
    pub sha256: Option<String>,
    pub preview: Option<String>,
}

impl FileView {
    pub fn from_metadata(metadata: &FileMetadata) -> Self {
        Self {
            id: metadata.id.clone(),
            file_name: metadata.file_name.clone(),
            size: metadata.size,
            file_type: metadata.file_type.clone(),
            sha256: metadata.sha256.clone(),
            preview: metadata.preview.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TransferDirection {
    Send,
    Receive,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TransferStatus {
    Waiting,
    Transferring,
    Finished,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ServiceErrorKind {
    InvalidPin,
    PinRequired,
    Rejected,
    NotFound,
    Io,
    Protocol,
    Internal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransferRequestView {
    pub request_id: String,
    pub sender: DeviceInfo,
    pub files: Vec<FileView>,
    pub received_at_ms: u64,
}

/// One file a frontend asked the daemon to send.
///
/// The caller owns the file id and the checksum it computed, so the receiver
/// verifies exactly what the UI promised instead of a digest the daemon made
/// up on its own.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendFileRequest {
    pub path: PathBuf,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<FileMetadataDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkippedPathView {
    pub path: PathBuf,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WebShareMode {
    Share,
    Receive,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransferView {
    pub transfer_id: String,
    pub direction: TransferDirection,
    pub peer_alias: String,
    pub peer_fingerprint: String,
    pub files: Vec<FileView>,
    pub status: TransferStatus,
    pub bytes_done: u64,
    pub bytes_total: u64,
    pub started_at_ms: u64,
    pub ended_at_ms: Option<u64>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HistoryEntry {
    pub id: String,
    /// The protocol file id the entry was created for, when the entry belongs
    /// to a single file. Older history files predate this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    pub direction: TransferDirection,
    pub peer_alias: String,
    pub file_name: String,
    pub path: Option<PathBuf>,
    pub size: u64,
    pub timestamp_ms: u64,
    pub status: TransferStatus,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServiceSnapshot {
    pub daemon_pid: u32,
    pub daemon_instance_id: String,
    pub revision: u64,
    pub last_error: Option<String>,
    pub server_running: bool,
    pub server_alias: Option<String>,
    pub server_port: Option<u16>,
    pub server_protocol: Option<Protocol>,
    pub devices: Vec<DeviceView>,
    pub incoming_requests: Vec<TransferRequestView>,
    pub active_transfers: Vec<TransferView>,
    pub history: Vec<HistoryEntry>,
    pub favorites: Vec<FavoriteView>,
    pub settings: DesktopSettings,
    pub last_event_seq: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ReceiverDecision {
    AcceptAll,
    AcceptFiles { file_ids: Vec<FileId> },
    Decline,
    Refuse { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum ServiceCommand {
    GetSnapshot,
    StartServer,
    StopServer,
    RestartServer,
    /// Tells the daemon whether an interactive frontend answers transfer
    /// requests itself. Purely a runtime policy: it is not persisted, so a
    /// daemon started without a frontend keeps honouring `auto_save`.
    SetRequestPolicy {
        frontend_answers_requests: bool,
    },
    StartDiscovery,
    StopDiscovery,
    RefreshDiscovery,
    AddManualDevice {
        target: String,
    },
    RemoveDevice {
        fingerprint: String,
    },
    AddFavorite {
        fingerprint: String,
        alias: Option<String>,
        /// The device to remember, when the caller knows it and this daemon
        /// has not seen it yet (a frontend restoring its own list after a
        /// restart, for example).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        device: Option<DeviceInfo>,
    },
    RemoveFavorite {
        fingerprint: String,
    },
    SendFiles {
        target_fingerprint: Option<String>,
        target: Option<DeviceInfo>,
        files: Vec<SendFileRequest>,
        pin: Option<String>,
    },
    SendText {
        target_fingerprint: Option<String>,
        target: Option<DeviceInfo>,
        text: String,
        pin: Option<String>,
    },
    StartWebShare {
        files: Vec<SendFileRequest>,
        pin: Option<String>,
        mode: WebShareMode,
        auto_accept: bool,
        #[serde(default)]
        i18n: std::collections::HashMap<String, String>,
    },
    StopWebShare,
    RespondWebShare {
        session_id: String,
        accepted: bool,
    },
    RespondTransfer {
        request_id: String,
        decision: ReceiverDecision,
    },
    CancelTransfer {
        transfer_id: String,
    },
    UpdateSettings {
        expected_revision: u64,
        settings: DesktopSettings,
    },
    ClearHistory,
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "result", rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum CommandResult {
    Ok,
    Snapshot { snapshot: ServiceSnapshot },
    Device { device: DeviceView },
    Favorite { favorite: FavoriteView },
    Transfer { transfer: TransferView },
    History { entries: Vec<HistoryEntry> },
    Settings { settings: DesktopSettings },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "event", rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum ServiceEvent {
    Snapshot {
        snapshot: ServiceSnapshot,
    },
    ServerState {
        running: bool,
        alias: Option<String>,
        port: Option<u16>,
        protocol: Option<Protocol>,
    },
    DeviceUpsert {
        device: DeviceView,
    },
    DeviceRemoved {
        fingerprint: String,
    },
    FavoriteUpsert {
        favorite: FavoriteView,
    },
    FavoriteRemoved {
        fingerprint: String,
    },
    TransferRequested {
        request: TransferRequestView,
    },
    TextReceived {
        sender_alias: String,
        text: String,
    },
    WebShareRequested {
        ip: String,
        session_id: String,
        user_agent: Option<String>,
    },
    WebShareProgress {
        session_id: String,
        file_id: FileId,
        bytes: u64,
        total: u64,
    },
    WebShareDone {
        session_id: String,
    },
    /// The receiver answered a send offer: which files it will take and which
    /// it refused. Progress for the accepted ones arrives as TransferProgress
    /// with the file id filled in.
    SendOfferAnswered {
        transfer_id: String,
        accepted_file_ids: Vec<FileId>,
        refused_file_ids: Vec<FileId>,
    },
    TransferPrepared {
        transfer: TransferView,
        skipped: Vec<SkippedPathView>,
    },
    TransferProgress {
        transfer_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        file_id: Option<FileId>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        file_name: Option<String>,
        bytes_done: u64,
        bytes_total: u64,
    },
    TransferFinished {
        transfer: TransferView,
    },
    TransferFailed {
        transfer_id: String,
        /// The file that failed, when the failure belongs to one file rather
        /// than to the whole transfer.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        file_id: Option<FileId>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        error_kind: Option<ServiceErrorKind>,
        error: String,
    },
    HistoryAppended {
        entry: HistoryEntry,
    },
    SettingsChanged {
        settings: DesktopSettings,
    },
    Error {
        message: String,
    },
    DaemonStopping,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SequencedEvent {
    pub seq: u64,
    pub event: ServiceEvent,
}

#[derive(Debug)]
pub enum InternalEvent {
    Server,
    Device(DeviceInfo, String),
    /// The receiver answered the offer: which files it will take and which it
    /// refused, so a frontend can mark the refused ones as skipped instead of
    /// waiting for progress that will never come.
    SendStarted {
        transfer_id: String,
        accepted_file_ids: Vec<FileId>,
        refused_file_ids: Vec<FileId>,
    },
    SendProgress {
        transfer_id: String,
        file_id: Option<FileId>,
        file_name: Option<String>,
        bytes_done: u64,
        bytes_total: u64,
    },
    SendFinished {
        transfer_id: String,
    },
    SendFailed {
        transfer_id: String,
        file_id: Option<FileId>,
        error_kind: Option<ServiceErrorKind>,
        error: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum RpcClientMessage {
    Hello {
        version: u32,
        client_name: String,
        resume_from: Option<u64>,
        #[serde(default)]
        interactive: bool,
        #[serde(default)]
        lifecycle_owner: bool,
        #[serde(default)]
        client_pid: u32,
    },
    Command {
        id: u64,
        command: ServiceCommand,
    },
    Ping {
        id: u64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RpcServerMessage {
    Hello {
        version: u32,
        daemon_pid: u32,
        instance_id: String,
        lifecycle: DaemonLifecycle,
        owner_pid: Option<u32>,
    },
    Snapshot {
        snapshot: ServiceSnapshot,
        last_event_seq: u64,
    },
    Response {
        id: u64,
        result: Result<CommandResult, String>,
    },
    Event {
        event: SequencedEvent,
    },
    Error {
        message: String,
    },
}

pub fn unix_ms_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or_default()
}

pub fn file_views(files: &HashMap<FileId, FileMetadata>) -> Vec<FileView> {
    files.values().map(FileView::from_metadata).collect()
}

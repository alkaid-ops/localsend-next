use anyhow::Context;
use bytes::Bytes;
use localsend_core::crypto::cert::{fingerprint_from_cert_der, generate_self_signed};
use localsend_core::crypto::hash::{HashError, sha256_file_content, sha256_hex};
use localsend_core::discovery::{
    DeviceIdentity, DiscoveredDevice, DiscoveryConfig, DiscoveryEvent, DiscoveryHandle,
    HttpChannel, StatefulDevice,
};
use localsend_core::http::client::LsHttpClient;
use localsend_core::http::client::LsHttpClientVersion;
use localsend_core::http::client::v2::LsHttpClientV2;
use localsend_core::http::dto::{PrepareUploadRequestDto, RegisterDto};
use localsend_core::http::server::common::save::FileUploadTarget;
use localsend_core::http::server::v2::{
    PrepareUploadDecisionV2, ServerEventV2, SessionEndReasonV2,
};
use localsend_core::http::server::web::{
    WebConfig, WebI18n, WebPages, WebSendConfig, WebSendEvent,
};
use localsend_core::http::server::{ServerConfigV2, ServerHandle, TlsConfig, start_with_port};
use localsend_core::http::state::ClientInfo;
use localsend_core::model::discovery::{DeviceType, PROTOCOL_VERSION_V2, ProtocolType};
use localsend_core::model::transfer::{FileContent, FileDto, FileMetadata};
use localsend_core::multicast::{
    DEFAULT_MULTICAST_GROUP, DEFAULT_MULTICAST_GROUP_V6, DEFAULT_PORT as DEFAULT_MULTICAST_PORT,
    MulticastDevice,
};
use localsend_core::util::filename;
use localsend_core::util::interface::InterfaceFilter;
use localsend_rpc::{
    AutoSaveMode, CommandResult, DesktopSettings, DeviceInfo, DeviceView, FavoriteView,
    FileMetadataDetails, FileView, HistoryEntry, Protocol, ReceiverDecision, SendFileRequest,
    SequencedEvent, ServiceCommand, ServiceErrorKind, ServiceEvent, ServiceSnapshot,
    SkippedPathView, TransferDirection, TransferRequestView, TransferStatus, TransferView,
    WebShareMode, unix_ms_now,
};
use parking_lot::RwLock;
use std::collections::{HashMap, HashSet};
use std::net::Ipv4Addr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::{broadcast, mpsc, oneshot, watch};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

const EVENT_BUFFER: usize = 1024;
const SERVER_STOP_TIMEOUT: Duration = Duration::from_secs(2);
const DISCOVERY_STOP_TIMEOUT: Duration = Duration::from_secs(1);
const SEND_PREPARE_TIMEOUT: Duration = Duration::from_secs(60);
const AUTO_FINISH_DELAY: Duration = Duration::from_secs(3);

#[derive(Debug, Clone)]
pub struct ServicePaths {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub settings_file: PathBuf,
    pub history_file: PathBuf,
    pub favorites_file: PathBuf,
    pub identity_file: PathBuf,
}

pub fn service_paths() -> anyhow::Result<ServicePaths> {
    let base = std::env::var_os("LOCALSEND_CONFIG_DIR")
        .map(PathBuf::from)
        .or_else(|| {
            directories::ProjectDirs::from("org", "LocalSend", "localsend-rs")
                .map(|dirs| dirs.config_dir().to_path_buf())
        })
        .unwrap_or_else(|| std::env::temp_dir().join("localsend-rs"));
    std::fs::create_dir_all(&base).with_context(|| format!("creating {}", base.display()))?;
    Ok(ServicePaths {
        data_dir: base.join("data"),
        settings_file: base.join("settings.json"),
        history_file: base.join("history.jsonl"),
        favorites_file: base.join("favorites.json"),
        identity_file: base.join("identity.pem"),
        config_dir: base,
    })
}

fn multicast_groups_for(settings: &DesktopSettings) -> Vec<Ipv4Addr> {
    let configured = settings
        .multicast_group
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .and_then(|value| value.parse().ok())
        .unwrap_or(DEFAULT_MULTICAST_GROUP);
    if configured == DEFAULT_MULTICAST_GROUP {
        vec![configured]
    } else {
        vec![configured, DEFAULT_MULTICAST_GROUP]
    }
}

fn interface_filter_for(settings: &DesktopSettings) -> InterfaceFilter {
    InterfaceFilter {
        whitelist: settings.interface_whitelist.clone(),
        blacklist: settings.interface_blacklist.clone(),
    }
}

fn discovery_timeout_for(settings: &DesktopSettings) -> Duration {
    Duration::from_millis(settings.discovery_timeout_ms.max(1))
}

#[derive(Clone)]
pub struct ServiceHandle {
    tx: mpsc::UnboundedSender<CommandEnvelope>,
    snapshot: Arc<RwLock<ServiceSnapshot>>,
    events: broadcast::Sender<SequencedEvent>,
    shutdown: watch::Receiver<bool>,
    interactive: Arc<AtomicUsize>,
}

impl ServiceHandle {
    pub async fn spawn() -> anyhow::Result<Self> {
        let paths = service_paths()?;
        Self::spawn_with_paths(paths).await
    }

    pub async fn spawn_with_paths(paths: ServicePaths) -> anyhow::Result<Self> {
        std::fs::create_dir_all(&paths.config_dir)
            .with_context(|| format!("creating {}", paths.config_dir.display()))?;
        std::fs::create_dir_all(&paths.data_dir)
            .with_context(|| format!("creating {}", paths.data_dir.display()))?;
        let settings = load_settings(&paths.settings_file);
        let identity = Identity::load_or_generate(&paths.identity_file, &settings.alias)?;
        let history = load_history(&paths.history_file, settings.history_limit);
        let favorites = load_json(&paths.favorites_file).unwrap_or_default();
        let snapshot = ServiceSnapshot {
            daemon_pid: std::process::id(),
            daemon_instance_id: Uuid::new_v4().to_string(),
            revision: 0,
            last_error: None,
            server_running: false,
            server_alias: None,
            server_port: None,
            server_protocol: None,
            devices: Vec::new(),
            incoming_requests: Vec::new(),
            active_transfers: Vec::new(),
            history,
            favorites,
            settings: settings.clone(),
            last_event_seq: 0,
        };
        let snapshot = Arc::new(RwLock::new(snapshot));
        let (events, _) = broadcast::channel(EVENT_BUFFER);
        let (shutdown_tx, shutdown_rx) = watch::channel(false);
        let (tx, rx) = mpsc::unbounded_channel();
        let interactive = Arc::new(AtomicUsize::new(0));
        let handle = Self {
            tx,
            snapshot: snapshot.clone(),
            events: events.clone(),
            shutdown: shutdown_rx,
            interactive: interactive.clone(),
        };
        let task = ServiceTask::new(paths, settings, identity, snapshot, events, shutdown_tx, rx);
        tokio::spawn(task.run());
        Ok(handle)
    }

    pub fn snapshot(&self) -> ServiceSnapshot {
        self.snapshot.read().clone()
    }

    pub fn subscribe(&self) -> broadcast::Receiver<SequencedEvent> {
        self.events.subscribe()
    }

    pub fn subscribe_shutdown(&self) -> watch::Receiver<bool> {
        self.shutdown.clone()
    }

    pub fn interactive_connected(&self) -> usize {
        self.interactive.fetch_add(1, Ordering::AcqRel) + 1
    }

    pub fn interactive_disconnected(&self) -> usize {
        self.interactive
            .fetch_sub(1, Ordering::AcqRel)
            .saturating_sub(1)
    }

    pub async fn command(&self, command: ServiceCommand) -> Result<CommandResult, String> {
        let (reply, response) = oneshot::channel();
        self.tx
            .send(CommandEnvelope { command, reply })
            .map_err(|_| "service stopped".to_string())?;
        response.await.map_err(|_| "service stopped".to_string())?
    }
}

struct CommandEnvelope {
    command: ServiceCommand,
    reply: oneshot::Sender<Result<CommandResult, String>>,
}

enum InternalEvent {
    SendOffer {
        transfer_id: String,
        accepted: Vec<String>,
        refused: Vec<String>,
    },
    SendProgress {
        transfer_id: String,
        file_id: String,
        file_name: String,
        bytes_done: u64,
        bytes_total: u64,
    },
    SendFinished {
        transfer_id: String,
    },
    SendFailed {
        transfer_id: String,
        file_id: Option<String>,
        kind: ServiceErrorKind,
        error: String,
    },
    ReceiveProgress {
        transfer_id: String,
        file_id: String,
        file_name: String,
        bytes_done: u64,
        bytes_total: u64,
    },
    ReceiveFileDone {
        transfer_id: String,
        file_id: String,
        file_name: String,
        path: PathBuf,
        size: u64,
        alias: String,
        device_model: Option<String>,
        message: Option<String>,
        result: Result<(), String>,
    },
    FinalizeSessionUploads {
        transfer_id: String,
        status: TransferStatus,
    },
    AutoFinishTransfer {
        transfer_id: String,
    },
}

struct RunningServer {
    handle: ServerHandle,
    stop_tx: Option<oneshot::Sender<()>>,
}

struct RunningDiscovery {
    handle: Arc<DiscoveryHandle>,
    stop_tx: Option<oneshot::Sender<()>>,
}

struct PendingReceive {
    sender: DeviceInfo,
    decision_tx: oneshot::Sender<PrepareUploadDecisionV2>,
}

struct Identity {
    cert_pem: String,
    key_pem: String,
    fingerprint: String,
}

impl Identity {
    fn load_or_generate(path: &Path, _alias: &str) -> anyhow::Result<Self> {
        match std::fs::read_to_string(path) {
            Ok(contents) => Self::from_pem(&contents),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                let generated = generate_self_signed()?;
                let identity = Self {
                    cert_pem: generated.certificate_pem,
                    key_pem: generated.private_key_pem,
                    fingerprint: generated.fingerprint,
                };
                write_private_file(
                    path,
                    format!("{}{}", identity.cert_pem, identity.key_pem).as_bytes(),
                )?;
                Ok(identity)
            }
            Err(error) => Err(error.into()),
        }
    }

    fn from_pem(contents: &str) -> anyhow::Result<Self> {
        let blocks = pem::parse_many(contents)?;
        let certificate = blocks
            .iter()
            .find(|block| block.tag() == "CERTIFICATE")
            .context("identity.pem has no certificate")?;
        let private_key = blocks
            .iter()
            .find(|block| block.tag().ends_with("PRIVATE KEY"))
            .context("identity.pem has no private key")?;
        Ok(Self {
            cert_pem: pem::encode(certificate),
            key_pem: pem::encode(private_key),
            fingerprint: fingerprint_from_cert_der(certificate.contents()),
        })
    }

    fn device_identity(&self) -> DeviceIdentity {
        DeviceIdentity {
            cert_pem: self.cert_pem.clone(),
            private_key_pem: self.key_pem.clone(),
        }
    }
}

struct ServiceTask {
    paths: ServicePaths,
    settings: DesktopSettings,
    identity: Identity,
    snapshot: Arc<RwLock<ServiceSnapshot>>,
    events: broadcast::Sender<SequencedEvent>,
    shutdown_tx: watch::Sender<bool>,
    command_rx: mpsc::UnboundedReceiver<CommandEnvelope>,
    server_tx: mpsc::Sender<ServerEventV2>,
    server_rx: mpsc::Receiver<ServerEventV2>,
    discovery_tx: mpsc::Sender<DiscoveryEvent>,
    discovery_rx: mpsc::Receiver<DiscoveryEvent>,
    web_tx: mpsc::Sender<WebSendEvent>,
    web_rx: mpsc::Receiver<WebSendEvent>,
    internal_tx: mpsc::UnboundedSender<InternalEvent>,
    internal_rx: mpsc::UnboundedReceiver<InternalEvent>,
    frontend_answers_requests: bool,
    server: Option<RunningServer>,
    discoveries: Vec<RunningDiscovery>,
    pending_receives: HashMap<String, PendingReceive>,
    receive_peers: HashMap<String, DeviceInfo>,
    web_decisions: HashMap<String, oneshot::Sender<bool>>,
    cancellations: HashMap<String, CancellationToken>,
    file_progress: HashMap<(String, String), u64>,
    pending_upload_paths: HashMap<(String, String), PathBuf>,
    web_share: Option<WebShareRuntime>,
    web_auto_accept: bool,
    http_clients: HashMap<HttpClientKey, Arc<LsHttpClient>>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct HttpClientKey {
    https: bool,
    v3: bool,
    fingerprint: Option<String>,
}

impl HttpClientKey {
    fn for_target(target: &DeviceInfo) -> Self {
        let https = target.protocol == Protocol::Https;
        Self {
            https,
            v3: target.version.starts_with('3'),
            fingerprint: https.then(|| target.fingerprint.clone()),
        }
    }
}

struct WebShareRuntime {
    mode: WebShareMode,
    files: HashMap<String, FileDto>,
    paths: HashMap<String, PathBuf>,
}

impl ServiceTask {
    #[allow(clippy::too_many_arguments)]
    fn new(
        paths: ServicePaths,
        settings: DesktopSettings,
        identity: Identity,
        snapshot: Arc<RwLock<ServiceSnapshot>>,
        events: broadcast::Sender<SequencedEvent>,
        shutdown_tx: watch::Sender<bool>,
        command_rx: mpsc::UnboundedReceiver<CommandEnvelope>,
    ) -> Self {
        let (server_tx, server_rx) = mpsc::channel(128);
        let (discovery_tx, discovery_rx) = mpsc::channel(128);
        let (web_tx, web_rx) = mpsc::channel(64);
        let (internal_tx, internal_rx) = mpsc::unbounded_channel();
        Self {
            paths,
            settings,
            identity,
            snapshot,
            events,
            shutdown_tx,
            command_rx,
            server_tx,
            server_rx,
            discovery_tx,
            discovery_rx,
            web_tx,
            web_rx,
            internal_tx,
            internal_rx,
            frontend_answers_requests: false,
            server: None,
            discoveries: Vec::new(),
            pending_receives: HashMap::new(),
            receive_peers: HashMap::new(),
            web_decisions: HashMap::new(),
            cancellations: HashMap::new(),
            file_progress: HashMap::new(),
            pending_upload_paths: HashMap::new(),
            web_share: None,
            web_auto_accept: false,
            http_clients: HashMap::new(),
        }
    }

    async fn run(mut self) {
        if self.settings.start_server_on_launch
            && let Err(error) = self.start_server().await
        {
            self.set_error(format!("starting server: {error:#}"));
        }
        let mut discovery_expiry = tokio::time::interval(Duration::from_secs(5));
        discovery_expiry.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            tokio::select! {
                envelope = self.command_rx.recv() => {
                    let Some(CommandEnvelope { command, reply }) = envelope else { break };
                    let shutdown = matches!(command, ServiceCommand::Shutdown);
                    let result = self.handle_command(command).await;
                    let _ = reply.send(result);
                    if shutdown {
                        break;
                    }
                }
                Some(event) = self.server_rx.recv() => self.handle_server_event(event).await,
                Some(event) = self.discovery_rx.recv() => self.handle_discovery_event(event),
                Some(event) = self.web_rx.recv() => self.handle_web_event(event),
                Some(event) = self.internal_rx.recv() => self.handle_internal_event(event),
                _ = discovery_expiry.tick() => self.expire_discovered_devices(),
            }
        }
        self.shutdown_tx.send_replace(true);
        self.stop_network().await;
    }

    async fn handle_command(&mut self, command: ServiceCommand) -> Result<CommandResult, String> {
        match command {
            ServiceCommand::GetSnapshot => Ok(CommandResult::Snapshot {
                snapshot: self.snapshot(),
            }),
            ServiceCommand::StartServer => {
                self.start_server()
                    .await
                    .map_err(|error| error.to_string())?;
                Ok(CommandResult::Ok)
            }
            ServiceCommand::StopServer => {
                self.stop_server().await;
                Ok(CommandResult::Ok)
            }
            ServiceCommand::RestartServer => {
                self.stop_server().await;
                self.start_server()
                    .await
                    .map_err(|error| error.to_string())?;
                Ok(CommandResult::Ok)
            }
            ServiceCommand::SetRequestPolicy {
                frontend_answers_requests,
            } => {
                self.frontend_answers_requests = frontend_answers_requests;
                Ok(CommandResult::Ok)
            }
            ServiceCommand::StartDiscovery | ServiceCommand::RefreshDiscovery => {
                self.refresh_discovery().await;
                Ok(CommandResult::Ok)
            }
            ServiceCommand::StopDiscovery => {
                self.stop_discovery().await;
                Ok(CommandResult::Ok)
            }
            ServiceCommand::AddManualDevice { target } => {
                self.add_manual_device(&target).await?;
                Ok(CommandResult::Ok)
            }
            ServiceCommand::RemoveDevice { fingerprint } => {
                self.snapshot
                    .write()
                    .devices
                    .retain(|device| device.device.fingerprint != fingerprint);
                self.emit(ServiceEvent::DeviceRemoved { fingerprint });
                Ok(CommandResult::Ok)
            }
            ServiceCommand::AddFavorite {
                fingerprint,
                alias,
                device,
            } => self.add_favorite(fingerprint, alias, device),
            ServiceCommand::RemoveFavorite { fingerprint } => {
                let mut snapshot = self.snapshot.write();
                snapshot
                    .favorites
                    .retain(|favorite| favorite.device.fingerprint != fingerprint);
                let favorites = snapshot.favorites.clone();
                drop(snapshot);
                save_json(&self.paths.favorites_file, &favorites)
                    .map_err(|error| error.to_string())?;
                self.emit(ServiceEvent::FavoriteRemoved { fingerprint });
                Ok(CommandResult::Ok)
            }
            ServiceCommand::SendFiles {
                target_fingerprint,
                target,
                files,
                pin,
            } => {
                let transfer = self.start_send_files(target_fingerprint, target, files, pin)?;
                Ok(CommandResult::Transfer { transfer })
            }
            ServiceCommand::SendText {
                target_fingerprint,
                target,
                text,
                pin,
            } => {
                if text.trim().is_empty() {
                    return Err("text message cannot be empty".to_string());
                }
                let transfer = self.start_send_text(target_fingerprint, target, text, pin)?;
                Ok(CommandResult::Transfer { transfer })
            }
            ServiceCommand::StartWebShare {
                files,
                pin,
                mode,
                auto_accept,
                i18n,
            } => {
                self.start_web_share(files, pin, mode, auto_accept, i18n)
                    .await?;
                Ok(CommandResult::Ok)
            }
            ServiceCommand::StopWebShare => {
                self.web_share = None;
                self.web_auto_accept = false;
                self.web_decisions.clear();
                if self.server.is_some() {
                    self.stop_server().await;
                    self.start_server()
                        .await
                        .map_err(|error| error.to_string())?;
                }
                Ok(CommandResult::Ok)
            }
            ServiceCommand::RespondWebShare {
                session_id,
                accepted,
            } => {
                if let Some(tx) = self.web_decisions.remove(&session_id) {
                    let _ = tx.send(accepted);
                }
                Ok(CommandResult::Ok)
            }
            ServiceCommand::RespondTransfer {
                request_id,
                decision,
            } => self.respond_transfer(&request_id, decision),
            ServiceCommand::CancelTransfer { transfer_id } => {
                if let Some(token) = self.cancellations.remove(&transfer_id) {
                    token.cancel();
                }
                if let Some(server) = &self.server {
                    let _ = server.handle.cancel_v2_session(&transfer_id).await;
                }
                self.set_transfer_status(&transfer_id, TransferStatus::Cancelled, None);
                Ok(CommandResult::Ok)
            }
            ServiceCommand::DismissTransfer { transfer_id } => {
                self.cancellations.remove(&transfer_id);
                self.snapshot
                    .write()
                    .active_transfers
                    .retain(|transfer| transfer.transfer_id != transfer_id);
                self.file_progress
                    .retain(|(session_id, _), _| session_id != &transfer_id);
                self.emit(ServiceEvent::Snapshot {
                    snapshot: self.snapshot(),
                });
                Ok(CommandResult::Ok)
            }
            ServiceCommand::UpdateSettings {
                expected_revision,
                settings,
            } => self.update_settings(expected_revision, settings).await,
            ServiceCommand::ClearHistory => {
                let _ = std::fs::remove_file(&self.paths.history_file);
                self.snapshot.write().history.clear();
                self.emit(ServiceEvent::Snapshot {
                    snapshot: self.snapshot(),
                });
                Ok(CommandResult::Ok)
            }
            ServiceCommand::Shutdown => {
                self.emit(ServiceEvent::DaemonStopping);
                Ok(CommandResult::Ok)
            }
        }
    }

    fn snapshot(&self) -> ServiceSnapshot {
        self.snapshot.read().clone()
    }

    fn emit(&self, mut event: ServiceEvent) -> u64 {
        let mut snapshot = self.snapshot.write();
        snapshot.last_event_seq = snapshot.last_event_seq.saturating_add(1);
        snapshot.revision = snapshot.revision.saturating_add(1);
        let seq = snapshot.last_event_seq;
        if let ServiceEvent::Snapshot {
            snapshot: event_snapshot,
        } = &mut event
        {
            *event_snapshot = snapshot.clone();
        }
        drop(snapshot);
        let _ = self.events.send(SequencedEvent { seq, event });
        seq
    }

    fn set_error(&self, message: String) {
        tracing::warn!("{message}");
        self.snapshot.write().last_error = Some(message.clone());
        self.emit(ServiceEvent::Error { message });
    }

    async fn start_server(&mut self) -> anyhow::Result<()> {
        self.stop_server().await;
        let protocol = protocol_type(self.settings.protocol);
        let tls = if self.settings.protocol == Protocol::Https {
            Some(TlsConfig {
                cert: self.identity.cert_pem.clone(),
                private_key: self.identity.key_pem.clone(),
            })
        } else {
            None
        };
        let (stop_tx, stop_rx) = oneshot::channel();
        let web_send = self.web_share.as_ref().and_then(|web| match web.mode {
            WebShareMode::Share => Some(WebSendConfig {
                files: web.files.clone(),
                pin: self.settings.pin.clone(),
                event_tx: self.web_tx.clone(),
            }),
            WebShareMode::Receive => None,
        });
        let web = WebConfig {
            send: web_send,
            upload: true,
            i18n: WebI18n::default(),
            pages: WebPages::default(),
        };
        let info = ClientInfo {
            alias: self.settings.alias.clone(),
            version: PROTOCOL_VERSION_V2.to_string(),
            device_model: Some(default_device_model()),
            device_type: Some(DeviceType::Desktop),
            token: self.identity.fingerprint.clone(),
        };
        let config = ServerConfigV2 {
            pin: self.settings.pin.clone(),
            verify_checksums: self.settings.verify_checksums,
            event_tx: self.server_tx.clone(),
        };
        let handle = start_with_port(
            self.settings.port,
            tls,
            info,
            None,
            Some(config),
            Some(web),
            stop_rx,
        )
        .await?;
        let port = handle.port();
        let local_ip = handle
            .local_addresses()
            .first()
            .map(|address| address.ip().to_string())
            .or_else(|| Some("127.0.0.1".to_string()));
        self.server = Some(RunningServer {
            handle,
            stop_tx: Some(stop_tx),
        });
        {
            let mut snapshot = self.snapshot.write();
            snapshot.server_running = true;
            snapshot.server_alias = Some(self.settings.alias.clone());
            snapshot.server_port = Some(port);
            snapshot.server_protocol = Some(self.settings.protocol);
        }
        self.emit(ServiceEvent::ServerState {
            running: true,
            alias: Some(self.settings.alias.clone()),
            port: Some(port),
            protocol: Some(self.settings.protocol),
        });
        let local_device = DeviceView {
            device: DeviceInfo {
                alias: self.settings.alias.clone(),
                version: PROTOCOL_VERSION_V2.to_string(),
                device_model: Some(default_device_model()),
                device_type: Some(localsend_rpc::DeviceType::Desktop),
                fingerprint: self.identity.fingerprint.clone(),
                port,
                protocol: self.settings.protocol,
                download: self.web_share.is_some(),
                ip: local_ip,
            },
            last_seen_ms: unix_ms_now(),
            source: "local".to_string(),
        };
        {
            let mut snapshot = self.snapshot.write();
            snapshot.devices.retain(|device| device.source != "local");
            snapshot.devices.push(local_device.clone());
        }
        self.emit(ServiceEvent::DeviceUpsert {
            device: local_device,
        });
        self.start_discovery(protocol).await?;
        Ok(())
    }

    async fn stop_server(&mut self) {
        if let Some(mut server) = self.server.take() {
            if let Some(stop_tx) = server.stop_tx.take() {
                let _ = stop_tx.send(());
            }
            let _ = tokio::time::timeout(SERVER_STOP_TIMEOUT, server.handle.wait_stopped()).await;
        }
        self.stop_discovery().await;
        {
            let mut snapshot = self.snapshot.write();
            snapshot.server_running = false;
            snapshot.server_port = None;
            let removed = snapshot
                .devices
                .iter()
                .filter(|device| device.source == "local")
                .map(|device| device.device.fingerprint.clone())
                .collect::<Vec<_>>();
            snapshot.devices.retain(|device| device.source != "local");
            drop(snapshot);
            for fingerprint in removed {
                self.emit(ServiceEvent::DeviceRemoved { fingerprint });
            }
        }
        self.emit(ServiceEvent::ServerState {
            running: false,
            alias: None,
            port: None,
            protocol: None,
        });
    }

    async fn start_discovery(&mut self, protocol: ProtocolType) -> anyhow::Result<()> {
        self.stop_discovery().await;
        if std::env::var_os("LOCALSEND_TEST_DISABLE_DISCOVERY").is_some() {
            return Ok(());
        }
        let server_port = self
            .server
            .as_ref()
            .map(|server| server.handle.port())
            .unwrap_or(self.settings.port);
        let groups = multicast_groups_for(&self.settings);
        let interface_filter = interface_filter_for(&self.settings);
        let timeout = discovery_timeout_for(&self.settings);
        for (index, group) in groups.into_iter().enumerate() {
            let (stop_tx, stop_rx) = oneshot::channel();
            let config = DiscoveryConfig {
                group,
                group_v6: (index == 0).then_some(DEFAULT_MULTICAST_GROUP_V6),
                // LocalSend always discovers on 53317. The advertised HTTP
                // port below may be different and is carried in the payload.
                port: DEFAULT_MULTICAST_PORT,
                interface_filter: interface_filter.clone(),
                device: MulticastDevice {
                    alias: self.settings.alias.clone(),
                    version: PROTOCOL_VERSION_V2.to_string(),
                    device_model: Some(default_device_model()),
                    device_type: Some(DeviceType::Desktop),
                    fingerprint: self.identity.fingerprint.clone(),
                    port: server_port,
                    protocol,
                    download: self.web_share.is_some(),
                },
                identity: self.identity.device_identity(),
                timeout,
                event_tx: Some(self.discovery_tx.clone()),
            };
            let handle = Arc::new(localsend_core::discovery::start(config, stop_rx).await);
            let announcing = handle.clone();
            tokio::spawn(async move {
                announcing.announce().await;
            });
            self.discoveries.push(RunningDiscovery {
                handle,
                stop_tx: Some(stop_tx),
            });
        }
        Ok(())
    }

    async fn stop_discovery(&mut self) {
        for mut discovery in std::mem::take(&mut self.discoveries) {
            if let Some(stop_tx) = discovery.stop_tx.take() {
                let _ = stop_tx.send(());
            }
            let _ =
                tokio::time::timeout(DISCOVERY_STOP_TIMEOUT, discovery.handle.wait_stopped()).await;
        }
    }

    async fn refresh_discovery(&mut self) {
        let mut started_now = false;
        if self.discoveries.is_empty() {
            let protocol = protocol_type(self.settings.protocol);
            if let Err(error) = self.start_discovery(protocol).await {
                self.set_error(format!("starting discovery: {error:#}"));
                return;
            }
            started_now = true;
        }
        let handles = self
            .discoveries
            .iter()
            .map(|discovery| discovery.handle.clone())
            .collect::<Vec<_>>();
        if !started_now {
            for handle in &handles {
                let handle = handle.clone();
                tokio::spawn(async move {
                    handle.announce().await;
                });
            }
        }
        let channels = self
            .snapshot()
            .favorites
            .into_iter()
            .filter_map(|favorite| device_to_http_channel(&favorite.device))
            .collect::<Vec<_>>();
        if let Some(handle) = handles.first().cloned() {
            tokio::spawn(async move {
                let _ = handle.discover_known_http_channels(channels).await;
            });
        }
    }

    async fn add_manual_device(&mut self, target: &str) -> Result<(), String> {
        let (host, port) = parse_target(target, self.settings.port)?;
        if std::env::var_os("LOCALSEND_TEST_DISABLE_DISCOVERY").is_some() {
            return self.discover_manual_device(&host, port).await;
        }
        if self.discoveries.is_empty() {
            self.start_discovery(protocol_type(self.settings.protocol))
                .await
                .map_err(|error| error.to_string())?;
        }
        let discovery = self
            .discoveries
            .first()
            .ok_or_else(|| "discovery is not running".to_string())?;
        let mut last_error = None;
        for protocol in [
            protocol_type(self.settings.protocol),
            ProtocolType::Http,
            ProtocolType::Https,
        ] {
            match discovery.handle.discover(&host, port, protocol).await {
                Ok(Some(device)) => {
                    self.upsert_discovered_device(&device);
                    return Ok(());
                }
                Ok(None) => return Ok(()),
                Err(error) => last_error = Some(error.to_string()),
            }
        }
        Err(last_error.unwrap_or_else(|| "device did not answer".to_string()))
    }

    async fn discover_manual_device(&mut self, host: &str, port: u16) -> Result<(), String> {
        let server_port = self
            .server
            .as_ref()
            .map(|server| server.handle.port())
            .unwrap_or(self.settings.port);
        let mut last_error = None;
        for protocol in [
            protocol_type(self.settings.protocol),
            ProtocolType::Http,
            ProtocolType::Https,
        ] {
            let client = match LsHttpClientV2::try_new(
                &self.identity.key_pem,
                &self.identity.cert_pem,
                None,
                Some(Duration::from_millis(
                    self.settings.discovery_timeout_ms.max(1),
                )),
            ) {
                Ok(client) => client,
                Err(error) => {
                    last_error = Some(error.to_string());
                    continue;
                }
            };
            let register = localsend_core::http::dto_v2::RegisterDtoV2 {
                alias: self.settings.alias.clone(),
                version: PROTOCOL_VERSION_V2.to_string(),
                device_model: Some(default_device_model()),
                device_type: Some(DeviceType::Desktop),
                fingerprint: self.identity.fingerprint.clone(),
                port: server_port,
                protocol,
                download: false,
            };
            match client.register(protocol, host, port, register).await {
                Ok(response) => {
                    let fingerprint = if protocol == ProtocolType::Https {
                        match response.cert_fingerprint {
                            Some(fingerprint) => fingerprint,
                            None => {
                                last_error = Some("HTTPS register returned no certificate".into());
                                continue;
                            }
                        }
                    } else {
                        response.body.fingerprint
                    };
                    let device = DiscoveredDevice {
                        alias: response.body.alias,
                        version: response.body.version,
                        device_model: response.body.device_model,
                        device_type: response.body.device_type,
                        fingerprint,
                        channel: localsend_core::discovery::DeviceChannel::Http(HttpChannel {
                            host: host.to_string(),
                            port,
                            protocol,
                        }),
                        download: response.body.download,
                    };
                    let mut view = discovered_device_view(&device);
                    view.source = "manual".to_string();
                    self.snapshot
                        .write()
                        .devices
                        .retain(|item| item.device.fingerprint != view.device.fingerprint);
                    self.snapshot.write().devices.push(view.clone());
                    self.emit(ServiceEvent::DeviceUpsert { device: view });
                    return Ok(());
                }
                Err(error) => last_error = Some(error.to_string()),
            }
        }
        Err(last_error.unwrap_or_else(|| "device did not answer".to_string()))
    }

    fn add_favorite(
        &mut self,
        fingerprint: String,
        alias: Option<String>,
        device: Option<DeviceInfo>,
    ) -> Result<CommandResult, String> {
        let device = device
            .or_else(|| {
                self.snapshot
                    .read()
                    .devices
                    .iter()
                    .find(|view| view.device.fingerprint == fingerprint)
                    .map(|view| view.device.clone())
            })
            .or_else(|| {
                self.snapshot
                    .read()
                    .favorites
                    .iter()
                    .find(|favorite| favorite.device.fingerprint == fingerprint)
                    .map(|favorite| favorite.device.clone())
            })
            .ok_or_else(|| format!("unknown device fingerprint: {fingerprint}"))?;
        let favorite = FavoriteView {
            alias: alias.unwrap_or_else(|| device.alias.clone()),
            device,
            added_at_ms: unix_ms_now(),
        };
        let mut snapshot = self.snapshot.write();
        snapshot
            .favorites
            .retain(|item| item.device.fingerprint != fingerprint);
        snapshot.favorites.push(favorite.clone());
        let favorites = snapshot.favorites.clone();
        drop(snapshot);
        save_json(&self.paths.favorites_file, &favorites).map_err(|error| error.to_string())?;
        self.emit(ServiceEvent::FavoriteUpsert {
            favorite: favorite.clone(),
        });
        Ok(CommandResult::Favorite { favorite })
    }

    fn respond_transfer(
        &mut self,
        request_id: &str,
        decision: ReceiverDecision,
    ) -> Result<CommandResult, String> {
        let pending = self
            .pending_receives
            .remove(request_id)
            .ok_or_else(|| "request is no longer pending".to_string())?;
        let request = {
            let mut snapshot = self.snapshot.write();
            let index = snapshot
                .incoming_requests
                .iter()
                .position(|request| request.request_id == request_id);
            index.map(|index| snapshot.incoming_requests.remove(index))
        };
        let Some(request) = request else {
            return Err("request is no longer pending".to_string());
        };
        let (decision, accepted, refused) = match decision {
            ReceiverDecision::AcceptAll => (
                PrepareUploadDecisionV2::Accept(
                    request.files.iter().map(|file| file.id.clone()).collect(),
                ),
                request
                    .files
                    .iter()
                    .map(|file| file.id.clone())
                    .collect::<HashSet<_>>(),
                HashSet::new(),
            ),
            ReceiverDecision::AcceptFiles { file_ids } => {
                let accepted: HashSet<_> = file_ids.into_iter().collect();
                let refused = request
                    .files
                    .iter()
                    .filter(|file| !accepted.contains(&file.id))
                    .map(|file| file.id.clone())
                    .collect();
                (
                    PrepareUploadDecisionV2::Accept(accepted.clone()),
                    accepted,
                    refused,
                )
            }
            ReceiverDecision::Decline | ReceiverDecision::Refuse { .. } => (
                PrepareUploadDecisionV2::Decline,
                HashSet::new(),
                request.files.iter().map(|file| file.id.clone()).collect(),
            ),
        };
        let accepted_vec = accepted.iter().cloned().collect::<Vec<_>>();
        let refused_vec = refused.iter().cloned().collect::<Vec<_>>();
        let _ = pending.decision_tx.send(decision);
        if !accepted.is_empty() {
            let transfer = TransferView {
                transfer_id: request.request_id.clone(),
                direction: TransferDirection::Receive,
                peer_alias: pending.sender.alias.clone(),
                peer_fingerprint: pending.sender.fingerprint.clone(),
                files: request
                    .files
                    .iter()
                    .filter(|file| accepted.contains(&file.id))
                    .cloned()
                    .collect(),
                status: TransferStatus::Waiting,
                bytes_done: 0,
                bytes_total: request
                    .files
                    .iter()
                    .filter(|file| accepted.contains(&file.id))
                    .map(|file| file.size)
                    .sum(),
                started_at_ms: unix_ms_now(),
                ended_at_ms: None,
                error: None,
            };
            self.snapshot
                .write()
                .active_transfers
                .push(transfer.clone());
            self.emit(ServiceEvent::TransferPrepared {
                transfer,
                skipped: Vec::new(),
            });
            self.emit(ServiceEvent::SendOfferAnswered {
                transfer_id: request.request_id,
                accepted_file_ids: accepted_vec,
                refused_file_ids: refused_vec,
            });
        }
        Ok(CommandResult::Ok)
    }

    fn start_send_files(
        &mut self,
        target_fingerprint: Option<String>,
        target: Option<DeviceInfo>,
        requests: Vec<SendFileRequest>,
        pin: Option<String>,
    ) -> Result<TransferView, String> {
        let target = self.resolve_target(target_fingerprint.as_deref(), target)?;
        let (items, skipped) = collect_send_items(requests);
        if items.is_empty() {
            return Err("no readable files selected".to_string());
        }
        let client = self.http_client_for(&target)?;
        let transfer_id = Uuid::new_v4().to_string();
        let total: u64 = items.iter().map(|item| item.file.size).sum();
        let transfer = TransferView {
            transfer_id: transfer_id.clone(),
            direction: TransferDirection::Send,
            peer_alias: target.alias.clone(),
            peer_fingerprint: target.fingerprint.clone(),
            files: items.iter().map(|item| file_view(&item.file)).collect(),
            status: TransferStatus::Waiting,
            bytes_done: 0,
            bytes_total: total,
            started_at_ms: unix_ms_now(),
            ended_at_ms: None,
            error: None,
        };
        self.snapshot
            .write()
            .active_transfers
            .push(transfer.clone());
        self.emit(ServiceEvent::TransferPrepared {
            transfer: transfer.clone(),
            skipped,
        });
        let cancel = CancellationToken::new();
        self.cancellations
            .insert(transfer_id.clone(), cancel.clone());
        let own_alias = self.settings.alias.clone();
        tokio::spawn(run_send_files(
            client,
            self.identity.fingerprint.clone(),
            own_alias,
            target,
            items,
            self.settings.create_checksums,
            pin,
            transfer_id,
            cancel,
            self.internal_tx.clone(),
        ));
        Ok(transfer)
    }

    fn start_send_text(
        &mut self,
        target_fingerprint: Option<String>,
        target: Option<DeviceInfo>,
        text: String,
        pin: Option<String>,
    ) -> Result<TransferView, String> {
        let target = self.resolve_target(target_fingerprint.as_deref(), target)?;
        let client = self.http_client_for(&target)?;
        let transfer_id = Uuid::new_v4().to_string();
        let file_id = Uuid::new_v4().to_string();
        let text_size = text.len() as u64;
        let text_sha256 = self
            .settings
            .create_checksums
            .then(|| sha256_hex(text.as_bytes()));
        let file = FileView {
            id: file_id.clone(),
            file_name: "message.txt".to_string(),
            size: text_size,
            file_type: "text/plain".to_string(),
            sha256: text_sha256.clone(),
            preview: Some(text.clone()),
        };
        let transfer = TransferView {
            transfer_id: transfer_id.clone(),
            direction: TransferDirection::Send,
            peer_alias: target.alias.clone(),
            peer_fingerprint: target.fingerprint.clone(),
            files: vec![file],
            status: TransferStatus::Waiting,
            bytes_done: 0,
            bytes_total: text_size,
            started_at_ms: unix_ms_now(),
            ended_at_ms: None,
            error: None,
        };
        self.snapshot
            .write()
            .active_transfers
            .push(transfer.clone());
        self.emit(ServiceEvent::TransferPrepared {
            transfer: transfer.clone(),
            skipped: Vec::new(),
        });
        let cancel = CancellationToken::new();
        self.cancellations
            .insert(transfer_id.clone(), cancel.clone());
        let own_alias = self.settings.alias.clone();
        tokio::spawn(run_send_text(
            client,
            self.identity.fingerprint.clone(),
            own_alias,
            target,
            text,
            file_id,
            text_sha256,
            pin,
            transfer_id,
            cancel,
            self.internal_tx.clone(),
        ));
        Ok(transfer)
    }

    fn http_client_for(&mut self, target: &DeviceInfo) -> Result<Arc<LsHttpClient>, String> {
        let key = HttpClientKey::for_target(target);
        if let Some(client) = self.http_clients.get(&key) {
            tracing::debug!(
                alias = %target.alias,
                protocol = ?target.protocol,
                v3 = key.v3,
                "reusing outbound HTTP client"
            );
            return Ok(client.clone());
        }

        let started = Instant::now();
        let version = if key.v3 {
            LsHttpClientVersion::V3
        } else {
            LsHttpClientVersion::V2
        };
        let client = LsHttpClient::new(
            &self.identity.key_pem,
            &self.identity.cert_pem,
            version,
            key.fingerprint.clone(),
            None,
        )
        .map_err(|error| error.to_string())?;
        let client = Arc::new(client);
        self.http_clients.insert(key.clone(), client.clone());
        tracing::info!(
            alias = %target.alias,
            host = target.ip.as_deref().unwrap_or("unknown"),
            port = target.port,
            protocol = ?target.protocol,
            v3 = key.v3,
            build_ms = started.elapsed().as_millis(),
            "created outbound HTTP client"
        );
        Ok(client)
    }

    fn resolve_target(
        &self,
        fingerprint: Option<&str>,
        target: Option<DeviceInfo>,
    ) -> Result<DeviceInfo, String> {
        if let Some(target) = target {
            return Ok(target);
        }
        let fingerprint = fingerprint.ok_or_else(|| "send target is missing".to_string())?;
        self.snapshot
            .read()
            .devices
            .iter()
            .find(|view| view.device.fingerprint == fingerprint)
            .map(|view| view.device.clone())
            .ok_or_else(|| format!("target device is not discovered: {fingerprint}"))
    }

    async fn start_web_share(
        &mut self,
        requests: Vec<SendFileRequest>,
        _pin: Option<String>,
        mode: WebShareMode,
        auto_accept: bool,
        _i18n: HashMap<String, String>,
    ) -> Result<(), String> {
        let (items, _skipped) = collect_send_items(requests);
        self.web_share = Some(WebShareRuntime {
            mode,
            files: items
                .iter()
                .map(|item| (item.file.id.clone(), item.file.clone()))
                .collect(),
            paths: items
                .iter()
                .map(|item| (item.file.id.clone(), item.path.clone()))
                .collect(),
        });
        self.web_auto_accept = auto_accept;
        if self.server.is_some() {
            self.stop_server().await;
            self.start_server()
                .await
                .map_err(|error| error.to_string())?;
        }
        Ok(())
    }

    async fn update_settings(
        &mut self,
        expected_revision: u64,
        mut settings: DesktopSettings,
    ) -> Result<CommandResult, String> {
        if expected_revision != self.settings.revision {
            return Err(format!(
                "settings revision conflict: expected {expected_revision}, current {}",
                self.settings.revision
            ));
        }
        settings.revision = self.settings.revision.saturating_add(1);
        save_json(&self.paths.settings_file, &settings).map_err(|error| error.to_string())?;
        let restart = settings.alias != self.settings.alias
            || settings.port != self.settings.port
            || settings.protocol != self.settings.protocol
            || settings.pin != self.settings.pin
            || settings.verify_checksums != self.settings.verify_checksums
            || settings.discovery_timeout_ms != self.settings.discovery_timeout_ms
            || settings.multicast_group != self.settings.multicast_group
            || settings.interface_whitelist != self.settings.interface_whitelist
            || settings.interface_blacklist != self.settings.interface_blacklist;
        self.settings = settings.clone();
        self.snapshot.write().settings = settings.clone();
        self.emit(ServiceEvent::SettingsChanged {
            settings: settings.clone(),
        });
        if restart && self.server.is_some() {
            self.stop_server().await;
            self.start_server()
                .await
                .map_err(|error| error.to_string())?;
        }
        Ok(CommandResult::Settings { settings })
    }

    async fn handle_server_event(&mut self, event: ServerEventV2) {
        match event {
            ServerEventV2::Register { ip, info } => {
                let host = ip.to_string();
                self.upsert_registered_device(host, info).await;
            }
            ServerEventV2::PrepareUpload {
                session_id,
                ip,
                info,
                cert_fingerprint,
                files,
                decision_tx,
            } => {
                let sender = DeviceInfo {
                    alias: info.alias.clone(),
                    version: info.version.clone(),
                    device_model: info.device_model.clone(),
                    device_type: map_device_type(info.device_type.as_ref()),
                    fingerprint: cert_fingerprint.unwrap_or(info.fingerprint),
                    port: info.port,
                    protocol: map_protocol(info.protocol),
                    download: info.download,
                    ip: Some(ip.to_string()),
                };
                self.receive_peers
                    .insert(session_id.clone(), sender.clone());
                let file_views = files.values().map(file_view).collect::<Vec<_>>();
                if let Some(message) = message_text(&files)
                    && files.values().all(|file| file.size == 0)
                {
                    let _ = decision_tx.send(PrepareUploadDecisionV2::Accept(HashSet::new()));
                    self.emit(ServiceEvent::TextReceived {
                        sender_alias: sender.alias.clone(),
                        sender_device_model: sender.device_model.clone(),
                        text: message.clone(),
                    });
                    let entry = HistoryEntry {
                        id: Uuid::new_v4().to_string(),
                        file_id: files.keys().next().cloned(),
                        direction: TransferDirection::Receive,
                        peer_alias: sender.alias,
                        file_name: "message.txt".to_string(),
                        path: None,
                        size: 0,
                        timestamp_ms: unix_ms_now(),
                        status: TransferStatus::Finished,
                        message: Some(message),
                    };
                    if self.settings.save_to_history {
                        self.record_history(entry.clone());
                    }
                    self.emit(ServiceEvent::HistoryAppended { entry });
                    self.receive_peers.remove(&session_id);
                    return;
                }
                let message_request = message_text(&files).is_some();
                let auto_accept = message_request
                    || self.settings.auto_save == AutoSaveMode::On
                    || (self.settings.auto_save == AutoSaveMode::Favorites
                        && self
                            .snapshot
                            .read()
                            .favorites
                            .iter()
                            .any(|favorite| favorite.device.fingerprint == sender.fingerprint));
                let request = TransferRequestView {
                    request_id: session_id.clone(),
                    sender: sender.clone(),
                    files: file_views.clone(),
                    received_at_ms: unix_ms_now(),
                };
                if auto_accept {
                    let accepted: HashSet<_> = files.keys().cloned().collect();
                    let _ = decision_tx.send(PrepareUploadDecisionV2::Accept(accepted.clone()));
                    if !accepted.is_empty() {
                        let transfer = TransferView {
                            transfer_id: session_id,
                            direction: TransferDirection::Receive,
                            peer_alias: sender.alias.clone(),
                            peer_fingerprint: sender.fingerprint.clone(),
                            files: file_views,
                            status: TransferStatus::Transferring,
                            bytes_done: 0,
                            bytes_total: files.values().map(|file| file.size).sum(),
                            started_at_ms: unix_ms_now(),
                            ended_at_ms: None,
                            error: None,
                        };
                        self.snapshot
                            .write()
                            .active_transfers
                            .push(transfer.clone());
                        self.emit(ServiceEvent::TransferPrepared {
                            transfer,
                            skipped: Vec::new(),
                        });
                    }
                } else {
                    self.snapshot
                        .write()
                        .incoming_requests
                        .push(request.clone());
                    self.emit(ServiceEvent::TransferRequested { request });
                    self.pending_receives.insert(
                        session_id,
                        PendingReceive {
                            sender,
                            decision_tx,
                        },
                    );
                }
            }
            ServerEventV2::FileUpload {
                session_id,
                file_id,
                file,
                target_tx,
            } => {
                self.handle_file_upload(session_id, file_id, file, target_tx)
                    .await
            }
            ServerEventV2::SessionEnd { session_id, reason } => {
                let interrupted = self
                    .pending_upload_paths
                    .keys()
                    .any(|(id, _)| id == &session_id);
                let status = match (reason, interrupted) {
                    (_, true) => TransferStatus::Failed,
                    (SessionEndReasonV2::Finished, false) => TransferStatus::Finished,
                    (SessionEndReasonV2::Cancelled, false) => TransferStatus::Cancelled,
                };
                if interrupted {
                    self.schedule_session_finalize(
                        &session_id,
                        match reason {
                            SessionEndReasonV2::Finished => TransferStatus::Finished,
                            SessionEndReasonV2::Cancelled => TransferStatus::Cancelled,
                        },
                    );
                } else {
                    self.set_transfer_status(&session_id, status, None);
                }
                self.receive_peers.remove(&session_id);
            }
            ServerEventV2::PrepareUploadAborted { session_id } => {
                self.pending_receives.remove(&session_id);
                self.receive_peers.remove(&session_id);
                self.snapshot
                    .write()
                    .incoming_requests
                    .retain(|request| request.request_id != session_id);
                if self
                    .pending_upload_paths
                    .keys()
                    .any(|(id, _)| id == &session_id)
                {
                    self.schedule_session_finalize(&session_id, TransferStatus::Cancelled);
                }
            }
            ServerEventV2::CancelReceived { session_id, .. } => {
                if let Some(token) = self.cancellations.remove(&session_id) {
                    token.cancel();
                }
            }
            ServerEventV2::ListenerFailed { error } => {
                self.set_error(format!("server listener failed: {error}"));
                self.snapshot.write().server_running = false;
            }
        }
    }

    async fn handle_file_upload(
        &mut self,
        session_id: String,
        file_id: String,
        file: FileDto,
        target_tx: oneshot::Sender<FileUploadTarget>,
    ) {
        let message = (file.file_type == "text" || file.file_type.starts_with("text/"))
            .then(|| file.preview.clone())
            .flatten();
        let destination = if message.is_some() {
            self.paths.data_dir.join("messages")
        } else {
            self.settings
                .destination_dir
                .clone()
                .unwrap_or_else(default_download_dir)
        };
        if let Err(error) = std::fs::create_dir_all(&destination) {
            let _ = target_tx.send(FileUploadTarget::Path {
                path: destination.join("invalid"),
                result_tx: oneshot::channel().0,
                progress_tx: None,
            });
            self.set_error(format!("creating receive directory: {error}"));
            return;
        }
        let path = unique_path(&destination, &file.file_name);
        if let Some(parent) = path.parent()
            && let Err(error) = std::fs::create_dir_all(parent)
        {
            let _ = target_tx.send(FileUploadTarget::Path {
                path: destination.join("invalid"),
                result_tx: oneshot::channel().0,
                progress_tx: None,
            });
            self.set_error(format!("creating receive subdirectory: {error}"));
            return;
        }
        self.pending_upload_paths
            .insert((session_id.clone(), file_id.clone()), path.clone());
        let (progress_tx, mut progress_rx) = mpsc::channel(32);
        let (result_tx, result_rx) = oneshot::channel();
        if target_tx
            .send(FileUploadTarget::Path {
                path: path.clone(),
                result_tx,
                progress_tx: Some(progress_tx),
            })
            .is_err()
        {
            self.set_error("receiver dropped upload target".to_string());
            return;
        }
        let (alias, device_model) = self
            .receive_peers
            .get(&session_id)
            .map(|sender| (sender.alias.clone(), sender.device_model.clone()))
            .unwrap_or_else(|| {
                let alias = self
                    .snapshot
                    .read()
                    .active_transfers
                    .iter()
                    .find(|transfer| transfer.transfer_id == session_id)
                    .map(|transfer| transfer.peer_alias.clone())
                    .unwrap_or_else(|| "Peer".to_string());
                (alias, None)
            });
        let progress_events = self.internal_tx.clone();
        let progress_transfer = session_id.clone();
        let progress_file_id = file_id.clone();
        let progress_name = file.file_name.clone();
        tokio::spawn(async move {
            while let Some(bytes) = progress_rx.recv().await {
                let _ = progress_events.send(InternalEvent::ReceiveProgress {
                    transfer_id: progress_transfer.clone(),
                    file_id: progress_file_id.clone(),
                    file_name: progress_name.clone(),
                    bytes_done: bytes,
                    bytes_total: file.size,
                });
            }
        });
        let result_events = self.internal_tx.clone();
        let result_path = path.clone();
        let result_size = file.size;
        let result_name = file.file_name.clone();
        tokio::spawn(async move {
            let result = result_rx
                .await
                .unwrap_or_else(|_| Err("upload ended without a result".to_string()));
            let _ = result_events.send(InternalEvent::ReceiveFileDone {
                transfer_id: session_id,
                file_id,
                file_name: result_name,
                path: result_path,
                size: result_size,
                alias,
                device_model,
                message,
                result,
            });
        });
    }

    fn handle_discovery_event(&mut self, event: DiscoveryEvent) {
        match event {
            DiscoveryEvent::Discovered { device } | DiscoveryEvent::Updated { device } => {
                let mut snapshot = self.snapshot.write();
                let mut view = discovered_device_view(&device);
                if snapshot.devices.iter().any(|item| {
                    item.device.fingerprint == view.device.fingerprint && item.source == "manual"
                }) {
                    view.source = "manual".to_string();
                }
                snapshot
                    .devices
                    .retain(|item| item.device.fingerprint != view.device.fingerprint);
                snapshot.devices.push(view.clone());
                drop(snapshot);
                self.emit(ServiceEvent::DeviceUpsert { device: view });
            }
            DiscoveryEvent::MulticastFailed => {
                self.set_error("multicast discovery stopped".to_string());
            }
        }
    }

    fn expire_discovered_devices(&self) {
        let ttl_ms = self
            .settings
            .discovery_timeout_ms
            .saturating_mul(30)
            .max(120_000);
        let now = unix_ms_now();
        let removed = {
            let mut snapshot = self.snapshot.write();
            let mut removed = Vec::new();
            snapshot.devices.retain(|device| {
                let expired = device.source == "discovery"
                    && now.saturating_sub(device.last_seen_ms) > ttl_ms;
                if expired {
                    removed.push(device.device.fingerprint.clone());
                }
                !expired
            });
            removed
        };
        for fingerprint in removed {
            self.emit(ServiceEvent::DeviceRemoved { fingerprint });
        }
    }

    fn handle_web_event(&mut self, event: WebSendEvent) {
        match event {
            WebSendEvent::PrepareDownload {
                ip,
                session_id,
                user_agent,
                decision_tx,
            } => {
                if self.web_auto_accept {
                    let _ = decision_tx.send(true);
                    return;
                }
                self.web_decisions.insert(session_id.clone(), decision_tx);
                self.emit(ServiceEvent::WebShareRequested {
                    ip: ip.to_string(),
                    session_id,
                    user_agent,
                });
            }
            WebSendEvent::FileDownload {
                session_id,
                file_id,
                file,
                content_tx,
            } => {
                if let Some(web) = &self.web_share
                    && let Some(path) = web.paths.get(&file_id)
                {
                    let _ = content_tx.send(FileContent::Path(path.clone()));
                    self.emit(ServiceEvent::WebShareProgress {
                        session_id,
                        file_id,
                        bytes: 0,
                        total: file.size,
                    });
                }
            }
        }
    }

    fn handle_internal_event(&mut self, event: InternalEvent) {
        match event {
            InternalEvent::SendOffer {
                transfer_id,
                accepted,
                refused,
            } => {
                self.set_transfer_status(&transfer_id, TransferStatus::Transferring, None);
                self.emit(ServiceEvent::SendOfferAnswered {
                    transfer_id,
                    accepted_file_ids: accepted,
                    refused_file_ids: refused,
                });
            }
            InternalEvent::SendProgress {
                transfer_id,
                file_id,
                file_name,
                bytes_done,
                bytes_total,
            } => {
                let total_done = record_file_progress(
                    &mut self.file_progress,
                    &transfer_id,
                    &file_id,
                    bytes_done,
                );
                let totals = {
                    let mut snapshot = self.snapshot.write();
                    snapshot
                        .active_transfers
                        .iter_mut()
                        .find(|transfer| transfer.transfer_id == transfer_id)
                        .map(|transfer| {
                            transfer.bytes_done = total_done.min(transfer.bytes_total);
                            transfer.status = TransferStatus::Transferring;
                            (transfer.bytes_done, transfer.bytes_total)
                        })
                };
                self.emit(ServiceEvent::TransferProgress {
                    transfer_id,
                    file_id: Some(file_id),
                    file_name: Some(file_name),
                    bytes_done,
                    bytes_total,
                    transfer_bytes_done: totals.map(|(done, _)| done),
                    transfer_bytes_total: totals.map(|(_, total)| total),
                });
            }
            InternalEvent::SendFinished { transfer_id } => {
                self.cancellations.remove(&transfer_id);
                self.file_progress.retain(|(id, _), _| id != &transfer_id);
                self.set_transfer_status(&transfer_id, TransferStatus::Finished, None);
            }
            InternalEvent::SendFailed {
                transfer_id,
                file_id,
                kind,
                error,
            } => {
                self.cancellations.remove(&transfer_id);
                self.file_progress.retain(|(id, _), _| id != &transfer_id);
                if self
                    .snapshot
                    .read()
                    .active_transfers
                    .iter()
                    .any(|transfer| {
                        transfer.transfer_id == transfer_id
                            && transfer.status == TransferStatus::Cancelled
                    })
                {
                    return;
                }
                self.set_transfer_status(&transfer_id, TransferStatus::Failed, Some(error.clone()));
                self.emit(ServiceEvent::TransferFailed {
                    transfer_id,
                    file_id,
                    error_kind: Some(kind),
                    error,
                });
            }
            InternalEvent::ReceiveProgress {
                transfer_id,
                file_id,
                file_name,
                bytes_done,
                bytes_total,
            } => {
                let total_done = record_file_progress(
                    &mut self.file_progress,
                    &transfer_id,
                    &file_id,
                    bytes_done,
                );
                let totals = {
                    let mut snapshot = self.snapshot.write();
                    snapshot
                        .active_transfers
                        .iter_mut()
                        .find(|transfer| transfer.transfer_id == transfer_id)
                        .map(|transfer| {
                            transfer.bytes_done = total_done.min(transfer.bytes_total);
                            transfer.status = TransferStatus::Transferring;
                            (transfer.bytes_done, transfer.bytes_total)
                        })
                };
                self.emit(ServiceEvent::TransferProgress {
                    transfer_id,
                    file_id: Some(file_id),
                    file_name: Some(file_name),
                    bytes_done,
                    bytes_total,
                    transfer_bytes_done: totals.map(|(done, _)| done),
                    transfer_bytes_total: totals.map(|(_, total)| total),
                });
            }
            InternalEvent::ReceiveFileDone {
                transfer_id,
                file_id,
                file_name,
                path,
                size,
                alias,
                device_model,
                message,
                result,
            } => {
                self.pending_upload_paths
                    .remove(&(transfer_id.clone(), file_id.clone()));
                self.file_progress
                    .remove(&(transfer_id.clone(), file_id.clone()));
                let status = if result.is_ok() {
                    TransferStatus::Finished
                } else {
                    TransferStatus::Failed
                };
                let failure = result.as_ref().err().cloned();
                if let Err(error) = &result {
                    // The official core validates the hash after writing the
                    // target. Remove that unverified file before reporting the
                    // failed receive so it cannot be mistaken for a valid one.
                    let _ = std::fs::remove_file(&path);
                    self.set_error(format!("receiving {file_name}: {error}"));
                }
                let successful = result.is_ok();
                let is_message = message.is_some();
                if successful && is_message {
                    let _ = std::fs::remove_file(&path);
                }
                let entry = HistoryEntry {
                    id: Uuid::new_v4().to_string(),
                    file_id: Some(file_id.clone()),
                    direction: TransferDirection::Receive,
                    peer_alias: alias.clone(),
                    file_name,
                    path: (successful && !is_message).then_some(path),
                    size,
                    timestamp_ms: unix_ms_now(),
                    status,
                    message: message.clone(),
                };
                if self.settings.save_to_history {
                    self.record_history(entry.clone());
                }
                self.emit(ServiceEvent::HistoryAppended { entry });
                if successful && let Some(text) = message {
                    self.emit(ServiceEvent::TextReceived {
                        sender_alias: alias,
                        sender_device_model: device_model,
                        text,
                    });
                }
                if let Some(error) = failure {
                    self.emit(ServiceEvent::TransferFailed {
                        transfer_id,
                        file_id: Some(file_id),
                        error_kind: Some(ServiceErrorKind::Io),
                        error,
                    });
                }
            }
            InternalEvent::FinalizeSessionUploads {
                transfer_id,
                status,
            } => {
                let keys = self
                    .pending_upload_paths
                    .keys()
                    .filter(|(session_id, _)| session_id == &transfer_id)
                    .cloned()
                    .collect::<Vec<_>>();
                let mut removed_paths = 0_usize;
                for key in keys {
                    if let Some(path) = self.pending_upload_paths.remove(&key) {
                        let _ = std::fs::remove_file(path);
                        removed_paths += 1;
                    }
                }
                if removed_paths > 0 {
                    self.set_transfer_status(
                        &transfer_id,
                        TransferStatus::Failed,
                        Some("upload interrupted".to_string()),
                    );
                    self.emit(ServiceEvent::TransferFailed {
                        transfer_id,
                        file_id: None,
                        error_kind: Some(ServiceErrorKind::Io),
                        error: "upload interrupted".to_string(),
                    });
                } else {
                    self.set_transfer_status(&transfer_id, status, None);
                }
            }
            InternalEvent::AutoFinishTransfer { transfer_id } => {
                let removed = {
                    let mut snapshot = self.snapshot.write();
                    let before = snapshot.active_transfers.len();
                    snapshot.active_transfers.retain(|transfer| {
                        !(transfer.transfer_id == transfer_id
                            && matches!(
                                transfer.status,
                                TransferStatus::Finished
                                    | TransferStatus::Failed
                                    | TransferStatus::Cancelled
                            ))
                    });
                    snapshot.active_transfers.len() != before
                };
                if removed {
                    self.emit(ServiceEvent::Snapshot {
                        snapshot: self.snapshot(),
                    });
                }
            }
        }
    }

    fn set_transfer_status(
        &self,
        transfer_id: &str,
        status: TransferStatus,
        error: Option<String>,
    ) {
        let terminal = matches!(
            status,
            TransferStatus::Finished | TransferStatus::Failed | TransferStatus::Cancelled
        );
        let transfer = {
            let mut snapshot = self.snapshot.write();
            let Some(transfer) = snapshot
                .active_transfers
                .iter_mut()
                .find(|transfer| transfer.transfer_id == transfer_id)
            else {
                return;
            };
            if transfer.status == TransferStatus::Failed && status == TransferStatus::Finished {
                return;
            }
            transfer.status = status;
            transfer.error = error;
            if terminal {
                transfer.ended_at_ms = Some(unix_ms_now());
                if transfer.bytes_done == 0 && status == TransferStatus::Finished {
                    transfer.bytes_done = transfer.bytes_total;
                }
            }
            transfer.clone()
        };
        if !terminal {
            return;
        }
        self.emit(ServiceEvent::TransferFinished { transfer });
        if terminal && self.settings.auto_finish {
            let transfer_id = transfer_id.to_string();
            let events = self.internal_tx.clone();
            tokio::spawn(async move {
                tokio::time::sleep(AUTO_FINISH_DELAY).await;
                let _ = events.send(InternalEvent::AutoFinishTransfer { transfer_id });
            });
        }
    }

    fn schedule_session_finalize(&self, transfer_id: &str, status: TransferStatus) {
        let transfer_id = transfer_id.to_string();
        let events = self.internal_tx.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let _ = events.send(InternalEvent::FinalizeSessionUploads {
                transfer_id,
                status,
            });
        });
    }

    fn record_history(&self, entry: HistoryEntry) {
        {
            let mut snapshot = self.snapshot.write();
            snapshot.history.push(entry.clone());
            let limit = self.settings.history_limit.max(1);
            if snapshot.history.len() > limit {
                let excess = snapshot.history.len() - limit;
                snapshot.history.drain(0..excess);
            }
        }
        if let Err(error) = append_history(&self.paths.history_file, &entry) {
            tracing::warn!("could not append history: {error:#}");
        }
    }

    async fn upsert_registered_device(
        &mut self,
        host: String,
        info: localsend_core::http::dto_v2::RegisterDtoV2,
    ) {
        let device = DiscoveredDevice {
            alias: info.alias,
            version: info.version,
            device_model: info.device_model,
            device_type: info.device_type,
            fingerprint: info.fingerprint,
            channel: localsend_core::discovery::DeviceChannel::Http(HttpChannel {
                host,
                port: info.port,
                protocol: info.protocol,
            }),
            download: info.download,
        };
        let fallback_view = discovered_device_view(&device);
        if !self.discoveries.is_empty() {
            for discovery in &self.discoveries {
                discovery.handle.add_device(device.clone()).await;
            }
            return;
        }
        self.snapshot
            .write()
            .devices
            .retain(|item| item.device.fingerprint != fallback_view.device.fingerprint);
        self.snapshot.write().devices.push(fallback_view.clone());
        self.emit(ServiceEvent::DeviceUpsert {
            device: fallback_view,
        });
    }

    fn upsert_discovered_device(&mut self, device: &StatefulDevice) {
        let mut view = stateful_device_view(device);
        // A successful manual address lookup is user intent and must not be
        // downgraded to a short-lived discovery record.
        view.source = "manual".to_string();
        self.snapshot
            .write()
            .devices
            .retain(|item| item.device.fingerprint != view.device.fingerprint);
        self.snapshot.write().devices.push(view.clone());
        self.emit(ServiceEvent::DeviceUpsert { device: view });
    }

    async fn stop_network(&mut self) {
        self.stop_server().await;
        self.stop_discovery().await;
    }
}

fn protocol_type(protocol: Protocol) -> ProtocolType {
    match protocol {
        Protocol::Http => ProtocolType::Http,
        Protocol::Https => ProtocolType::Https,
    }
}

fn map_protocol(protocol: ProtocolType) -> Protocol {
    match protocol {
        ProtocolType::Http => Protocol::Http,
        ProtocolType::Https => Protocol::Https,
    }
}

fn map_device_type(device_type: Option<&DeviceType>) -> Option<localsend_rpc::DeviceType> {
    device_type.map(|device_type| match device_type {
        DeviceType::Mobile => localsend_rpc::DeviceType::Mobile,
        DeviceType::Desktop => localsend_rpc::DeviceType::Desktop,
        DeviceType::Web => localsend_rpc::DeviceType::Web,
        DeviceType::Headless => localsend_rpc::DeviceType::Headless,
        DeviceType::Server => localsend_rpc::DeviceType::Server,
    })
}

fn discovered_device_view(device: &DiscoveredDevice) -> DeviceView {
    let http = device.http();
    DeviceView {
        device: DeviceInfo {
            alias: device.alias.clone(),
            version: device.version.clone(),
            device_model: device.device_model.clone(),
            device_type: map_device_type(device.device_type.as_ref()),
            fingerprint: device.fingerprint.clone(),
            port: http.map(|http| http.port).unwrap_or(53317),
            protocol: http
                .map(|http| map_protocol(http.protocol))
                .unwrap_or(Protocol::Https),
            download: device.download,
            ip: http.map(|http| http.host.clone()),
        },
        last_seen_ms: unix_ms_now(),
        source: "discovery".to_string(),
    }
}

fn stateful_device_view(device: &StatefulDevice) -> DeviceView {
    discovered_device_view(&device.device)
}

fn file_view(file: &FileDto) -> FileView {
    FileView {
        id: file.id.clone(),
        file_name: file.file_name.clone(),
        size: file.size,
        file_type: file.file_type.clone(),
        sha256: file.sha256.clone(),
        preview: file.preview.clone(),
    }
}

fn message_text(files: &HashMap<String, FileDto>) -> Option<String> {
    if files.len() != 1 {
        return None;
    }
    let file = files.values().next()?;
    if file.file_type != "text" && !file.file_type.starts_with("text/") {
        return None;
    }
    file.preview.clone()
}

fn device_to_http_channel(device: &DeviceInfo) -> Option<HttpChannel> {
    Some(HttpChannel {
        host: device.ip.clone()?,
        port: device.port,
        protocol: protocol_type(device.protocol),
    })
}

fn parse_target(target: &str, default_port: u16) -> Result<(String, u16), String> {
    let target = target.trim();
    if target.is_empty() {
        return Err("target address is empty".to_string());
    }
    if let Ok(address) = target.parse::<std::net::SocketAddr>() {
        return Ok((address.ip().to_string(), address.port()));
    }
    if let Some((host, port)) = target.rsplit_once(':')
        && let Ok(port) = port.parse()
    {
        return Ok((host.trim_matches(['[', ']']).to_string(), port));
    }
    Ok((target.to_string(), default_port))
}

struct SendItem {
    file: FileDto,
    path: PathBuf,
}

fn collect_send_items(requests: Vec<SendFileRequest>) -> (Vec<SendItem>, Vec<SkippedPathView>) {
    let mut items = Vec::new();
    let mut skipped = Vec::new();
    for request in requests {
        let metadata = match std::fs::symlink_metadata(&request.path) {
            Ok(metadata) => metadata,
            Err(error) => {
                skipped.push(SkippedPathView {
                    path: request.path,
                    reason: error.to_string(),
                });
                continue;
            }
        };
        if metadata.file_type().is_symlink() {
            skipped.push(SkippedPathView {
                path: request.path,
                reason: "symbolic links are not followed".to_string(),
            });
            continue;
        }
        if metadata.is_file() {
            let file_name = request
                .path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| "unnamed".to_string());
            items.push(SendItem {
                file: file_dto_from_request(
                    &request,
                    file_name,
                    metadata.len(),
                    FileMetadata::from_fs_metadata(&metadata),
                ),
                path: request.path,
            });
        } else if metadata.is_dir() {
            collect_directory(&request, &request.path, &mut items, &mut skipped);
        } else {
            skipped.push(SkippedPathView {
                path: request.path,
                reason: "not a regular file or directory".to_string(),
            });
        }
    }
    (items, skipped)
}

fn collect_directory(
    request: &SendFileRequest,
    root: &Path,
    items: &mut Vec<SendItem>,
    skipped: &mut Vec<SkippedPathView>,
) {
    let mut stack = vec![root.to_path_buf()];
    while let Some(directory) = stack.pop() {
        let entries = match std::fs::read_dir(&directory) {
            Ok(entries) => entries,
            Err(error) => {
                skipped.push(SkippedPathView {
                    path: directory,
                    reason: error.to_string(),
                });
                continue;
            }
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let metadata = match std::fs::symlink_metadata(&path) {
                Ok(metadata) => metadata,
                Err(error) => {
                    skipped.push(SkippedPathView {
                        path,
                        reason: error.to_string(),
                    });
                    continue;
                }
            };
            if metadata.file_type().is_symlink() {
                skipped.push(SkippedPathView {
                    path,
                    reason: "symbolic links are not followed".to_string(),
                });
            } else if metadata.is_dir() {
                stack.push(path);
            } else if metadata.is_file() {
                let relative = path.strip_prefix(root).unwrap_or(&path);
                let root_name = root
                    .file_name()
                    .map(|name| name.to_string_lossy())
                    .unwrap_or_default();
                let name = format!(
                    "{root_name}/{}",
                    relative.to_string_lossy().replace('\\', "/")
                );
                items.push(SendItem {
                    file: file_dto_from_request(
                        request,
                        name,
                        metadata.len(),
                        FileMetadata::from_fs_metadata(&metadata),
                    ),
                    path,
                });
            }
        }
    }
}

fn file_dto_from_request(
    request: &SendFileRequest,
    file_name: String,
    size: u64,
    metadata: Option<FileMetadata>,
) -> FileDto {
    FileDto {
        id: request
            .file_id
            .clone()
            .unwrap_or_else(|| Uuid::new_v4().to_string()),
        file_name,
        size,
        file_type: mime_guess::from_path(&request.path)
            .first_or_octet_stream()
            .to_string(),
        sha256: request.sha256.clone(),
        preview: None,
        metadata: request.metadata.clone().map(map_metadata).or(metadata),
    }
}

fn map_metadata(metadata: FileMetadataDetails) -> FileMetadata {
    FileMetadata {
        modified: metadata.modified,
        accessed: metadata.accessed,
    }
}

#[allow(clippy::too_many_arguments)]
async fn run_send_files(
    client: Arc<LsHttpClient>,
    own_fingerprint: String,
    own_alias: String,
    target: DeviceInfo,
    mut items: Vec<SendItem>,
    create_checksums: bool,
    pin: Option<String>,
    transfer_id: String,
    cancel: CancellationToken,
    events: mpsc::UnboundedSender<InternalEvent>,
) {
    let total_started = Instant::now();
    tracing::info!(
        transfer_id,
        alias = %target.alias,
        host = target.ip.as_deref().unwrap_or("unknown"),
        port = target.port,
        protocol = ?target.protocol,
        file_count = items.len(),
        "outbound file send started"
    );
    if create_checksums {
        let checksum_started = Instant::now();
        for item in &mut items {
            if item.file.sha256.is_some() {
                continue;
            }
            match sha256_file_content(FileContent::Path(item.path.clone()), &cancel, |_| {}).await {
                Ok(checksum) => item.file.sha256 = Some(checksum),
                Err(error) => {
                    let (kind, message) = match error {
                        HashError::Cancelled => (
                            ServiceErrorKind::Rejected,
                            "checksum calculation cancelled".to_string(),
                        ),
                        HashError::Io(error) => (
                            ServiceErrorKind::Io,
                            format!("failed to checksum {}: {error}", item.path.display()),
                        ),
                    };
                    let _ = events.send(InternalEvent::SendFailed {
                        transfer_id,
                        file_id: Some(item.file.id.clone()),
                        kind,
                        error: message,
                    });
                    return;
                }
            }
        }
        tracing::info!(
            transfer_id,
            checksum_ms = checksum_started.elapsed().as_millis(),
            "outbound file checksums ready"
        );
    }
    let files: HashMap<String, FileDto> = items
        .iter()
        .map(|item| (item.file.id.clone(), item.file.clone()))
        .collect();
    let register = RegisterDto {
        alias: own_alias,
        version: PROTOCOL_VERSION_V2.to_string(),
        device_model: Some(default_device_model()),
        device_type: Some(DeviceType::Desktop),
        token: own_fingerprint,
        port: 0,
        protocol: protocol_type(target.protocol),
        has_web_interface: false,
    };
    let payload = PrepareUploadRequestDto {
        info: register,
        files: files.clone(),
    };
    let Some(host) = target.ip.as_deref() else {
        let _ = events.send(InternalEvent::SendFailed {
            transfer_id,
            file_id: None,
            kind: ServiceErrorKind::Protocol,
            error: "target has no known IP address".to_string(),
        });
        return;
    };
    let prepare_started = Instant::now();
    let prepared = match tokio::time::timeout(
        SEND_PREPARE_TIMEOUT,
        client.prepare_upload(
            protocol_type(target.protocol),
            host,
            target.port,
            // Fingerprint pinning is enforced during the TLS handshake. This
            // parameter is an expected PEM public key, not a certificate
            // fingerprint, and must remain empty unless the caller knows it.
            None,
            payload,
            pin.as_deref(),
            cancel.clone(),
        ),
    )
    .await
    {
        Ok(Ok(prepared)) => prepared,
        Ok(Err(error)) => {
            let (kind, message) = classify_send_error(&error);
            let _ = events.send(InternalEvent::SendFailed {
                transfer_id,
                file_id: None,
                kind,
                error: message,
            });
            return;
        }
        Err(_) => {
            let _ = events.send(InternalEvent::SendFailed {
                transfer_id,
                file_id: None,
                kind: ServiceErrorKind::Protocol,
                error: "receiver did not answer the send request within 60 seconds".to_string(),
            });
            return;
        }
    };
    let prepare_ms = prepare_started.elapsed().as_millis();
    if prepare_ms >= 1_000 {
        tracing::warn!(
            transfer_id,
            alias = %target.alias,
            prepare_ms,
            "receiver took over one second to answer the file offer"
        );
    }
    tracing::info!(transfer_id, prepare_ms, "outbound file offer answered");
    let Some(response) = prepared.response else {
        let _ = events.send(InternalEvent::SendFinished { transfer_id });
        return;
    };
    let accepted = response.files.keys().cloned().collect::<HashSet<_>>();
    let refused = files
        .keys()
        .filter(|file_id| !accepted.contains(*file_id))
        .cloned()
        .collect();
    let _ = events.send(InternalEvent::SendOffer {
        transfer_id: transfer_id.clone(),
        accepted: accepted.iter().cloned().collect(),
        refused,
    });
    let upload_started = Instant::now();
    for item in items {
        let Some(token) = response.files.get(&item.file.id).cloned() else {
            continue;
        };
        let progress_events = events.clone();
        let progress_transfer = transfer_id.clone();
        let file_id = item.file.id.clone();
        let file_name = item.file.file_name.clone();
        let file_size = item.file.size;
        let progress = move |bytes: u64| {
            let _ = progress_events.send(InternalEvent::SendProgress {
                transfer_id: progress_transfer.clone(),
                file_id: file_id.clone(),
                file_name: file_name.clone(),
                bytes_done: bytes,
                bytes_total: file_size,
            });
        };
        if let Err(error) = client
            .upload(
                protocol_type(target.protocol),
                host,
                target.port,
                None,
                &response.session_id,
                &item.file.id,
                &token,
                FileContent::Path(item.path),
                progress,
                cancel.clone(),
            )
            .await
        {
            let (kind, message) = classify_send_error(&error);
            let _ = events.send(InternalEvent::SendFailed {
                transfer_id,
                file_id: Some(item.file.id),
                kind,
                error: message,
            });
            return;
        }
    }
    let upload_ms = upload_started.elapsed().as_millis();
    let total_ms = total_started.elapsed().as_millis();
    if upload_ms >= 1_000 {
        tracing::warn!(
            transfer_id,
            alias = %target.alias,
            prepare_ms,
            upload_ms,
            total_ms,
            "outbound file upload took over one second"
        );
    }
    tracing::info!(
        transfer_id,
        prepare_ms,
        upload_ms,
        total_ms,
        "outbound file send finished"
    );
    let _ = events.send(InternalEvent::SendFinished { transfer_id });
}

#[allow(clippy::too_many_arguments)]
async fn run_send_text(
    client: Arc<LsHttpClient>,
    own_fingerprint: String,
    own_alias: String,
    target: DeviceInfo,
    text: String,
    file_id: String,
    file_sha256: Option<String>,
    pin: Option<String>,
    transfer_id: String,
    cancel: CancellationToken,
    events: mpsc::UnboundedSender<InternalEvent>,
) {
    let total_started = Instant::now();
    tracing::info!(
        transfer_id,
        alias = %target.alias,
        host = target.ip.as_deref().unwrap_or("unknown"),
        port = target.port,
        protocol = ?target.protocol,
        bytes = text.len(),
        "outbound text send started"
    );
    let bytes = Bytes::from(text.clone().into_bytes());
    let file_size = bytes.len() as u64;
    let file = FileDto {
        id: file_id.clone(),
        file_name: "message.txt".to_string(),
        size: file_size,
        file_type: "text/plain".to_string(),
        sha256: file_sha256,
        preview: Some(text),
        metadata: None,
    };
    let payload = PrepareUploadRequestDto {
        info: RegisterDto {
            alias: own_alias,
            version: PROTOCOL_VERSION_V2.to_string(),
            device_model: Some(default_device_model()),
            device_type: Some(DeviceType::Desktop),
            token: own_fingerprint,
            port: 0,
            protocol: protocol_type(target.protocol),
            has_web_interface: false,
        },
        files: HashMap::from([(file_id.clone(), file)]),
    };
    let Some(host) = target.ip.as_deref() else {
        let _ = events.send(InternalEvent::SendFailed {
            transfer_id,
            file_id: None,
            kind: ServiceErrorKind::Protocol,
            error: "target has no known IP address".to_string(),
        });
        return;
    };
    let prepare_started = Instant::now();
    let prepared = match tokio::time::timeout(
        SEND_PREPARE_TIMEOUT,
        client.prepare_upload(
            protocol_type(target.protocol),
            host,
            target.port,
            None,
            payload,
            pin.as_deref(),
            cancel.clone(),
        ),
    )
    .await
    {
        Ok(Ok(prepared)) => prepared,
        Ok(Err(error)) => {
            let (kind, message) = classify_send_error(&error);
            let _ = events.send(InternalEvent::SendFailed {
                transfer_id,
                file_id: None,
                kind,
                error: message,
            });
            return;
        }
        Err(_) => {
            let _ = events.send(InternalEvent::SendFailed {
                transfer_id,
                file_id: None,
                kind: ServiceErrorKind::Protocol,
                error: "receiver did not answer the send request within 60 seconds".to_string(),
            });
            return;
        }
    };
    let prepare_ms = prepare_started.elapsed().as_millis();
    if prepare_ms >= 1_000 {
        tracing::warn!(
            transfer_id,
            alias = %target.alias,
            prepare_ms,
            "receiver took over one second to answer the text offer"
        );
    }
    tracing::info!(transfer_id, prepare_ms, "outbound text offer answered");
    if prepared.status_code == 204 || prepared.response.is_none() {
        tracing::info!(
            transfer_id,
            status_code = prepared.status_code,
            "receiver accepted text without a file upload"
        );
        let _ = events.send(InternalEvent::SendFinished { transfer_id });
        return;
    }
    let Some(response) = prepared.response else {
        unreachable!("a non-204 prepare response must contain a response body");
    };
    let Some(token) = response.files.get(&file_id).cloned() else {
        let _ = events.send(InternalEvent::SendFailed {
            transfer_id,
            file_id: Some(file_id),
            kind: ServiceErrorKind::Rejected,
            error: "receiver did not accept the message file".to_string(),
        });
        return;
    };
    let (content_tx, content_rx) = mpsc::channel(1);
    if content_tx.send(bytes).await.is_err() {
        let _ = events.send(InternalEvent::SendFailed {
            transfer_id,
            file_id: Some(file_id),
            kind: ServiceErrorKind::Internal,
            error: "message content stream closed".to_string(),
        });
        return;
    }
    drop(content_tx);
    let progress_events = events.clone();
    let progress_transfer = transfer_id.clone();
    let progress_file = file_id.clone();
    let progress = move |done: u64| {
        let _ = progress_events.send(InternalEvent::SendProgress {
            transfer_id: progress_transfer.clone(),
            file_id: progress_file.clone(),
            file_name: "message.txt".to_string(),
            bytes_done: done,
            bytes_total: file_size,
        });
    };
    let upload_started = Instant::now();
    if let Err(error) = client
        .upload(
            protocol_type(target.protocol),
            host,
            target.port,
            None,
            &response.session_id,
            &file_id,
            &token,
            FileContent::Stream(content_rx),
            progress,
            cancel,
        )
        .await
    {
        let (kind, message) = classify_send_error(&error);
        let _ = events.send(InternalEvent::SendFailed {
            transfer_id,
            file_id: Some(file_id),
            kind,
            error: message,
        });
        return;
    }
    let upload_ms = upload_started.elapsed().as_millis();
    let total_ms = total_started.elapsed().as_millis();
    if upload_ms >= 1_000 {
        tracing::warn!(
            transfer_id,
            alias = %target.alias,
            prepare_ms,
            upload_ms,
            total_ms,
            "outbound text upload took over one second"
        );
    }
    tracing::info!(
        transfer_id,
        prepare_ms,
        upload_ms,
        total_ms,
        "outbound text send finished"
    );
    let _ = events.send(InternalEvent::SendFinished { transfer_id });
}

fn classify_send_error(
    error: &localsend_core::http::client::ClientError,
) -> (ServiceErrorKind, String) {
    match error {
        localsend_core::http::client::ClientError::StatusCode(status) => match status.status {
            401 => (ServiceErrorKind::InvalidPin, status.to_string()),
            403 => (ServiceErrorKind::Rejected, status.to_string()),
            _ => (ServiceErrorKind::Protocol, status.to_string()),
        },
        localsend_core::http::client::ClientError::Cancelled => {
            (ServiceErrorKind::Rejected, "transfer cancelled".to_string())
        }
        localsend_core::http::client::ClientError::Io(_) => {
            (ServiceErrorKind::Io, error.to_string())
        }
        _ => (ServiceErrorKind::Internal, error.to_string()),
    }
}

fn default_download_dir() -> PathBuf {
    directories::UserDirs::new()
        .and_then(|dirs| dirs.download_dir().map(Path::to_path_buf))
        .unwrap_or_else(|| std::env::temp_dir().join("localsend-downloads"))
}

fn default_device_model() -> String {
    match std::env::consts::OS {
        "macos" => "macOS".to_string(),
        "windows" => "Windows".to_string(),
        "linux" => "Linux".to_string(),
        other => other.to_string(),
    }
}

fn unique_path(dir: &Path, file_name: &str) -> PathBuf {
    let relative = sanitized_relative_path(file_name);
    let parent = relative.parent().unwrap_or_else(|| Path::new(""));
    let name = relative
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "untitled".to_string());
    let target_dir = dir.join(parent);
    let candidate = target_dir.join(&name);
    if !candidate.exists() {
        return candidate;
    }
    let (stem, extension) = match name.rsplit_once('.') {
        Some((stem, extension)) if !stem.is_empty() => (stem, format!(".{extension}")),
        _ => (name.as_str(), String::new()),
    };
    (1..)
        .map(|index| target_dir.join(format!("{stem} ({index}){extension}")))
        .find(|candidate| !candidate.exists())
        .unwrap()
}

fn sanitized_relative_path(file_name: &str) -> PathBuf {
    let mut relative = PathBuf::new();
    for segment in file_name.split(['/', '\\']) {
        if segment.is_empty() || matches!(segment, "." | "..") {
            continue;
        }
        relative.push(filename::sanitize(segment, filename::Rules::Universal));
    }
    if relative.as_os_str().is_empty() {
        relative.push("untitled");
    }
    relative
}

fn load_settings(path: &Path) -> DesktopSettings {
    let Ok(bytes) = std::fs::read(path) else {
        return DesktopSettings::default();
    };
    let Ok(mut value) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
        return DesktopSettings::default();
    };
    if let Some(object) = value.as_object_mut()
        && !object.contains_key("discovery_timeout_ms")
        && let Some(seconds) = object
            .get("discovery_timeout_secs")
            .and_then(|value| value.as_u64())
    {
        object.insert(
            "discovery_timeout_ms".to_string(),
            serde_json::Value::from(seconds.saturating_mul(1_000)),
        );
    }
    if let Some(object) = value.as_object_mut() {
        // `discovery_timeout_secs` is only an input alias. Removing it avoids a
        // duplicate-field error when both the legacy and current keys exist.
        object.remove("discovery_timeout_secs");
    }
    serde_json::from_value(value).unwrap_or_default()
}

fn load_json<T: serde::de::DeserializeOwned>(path: &Path) -> Option<T> {
    serde_json::from_slice(&std::fs::read(path).ok()?).ok()
}

fn save_json<T: serde::Serialize>(path: &Path, value: &T) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let temporary = path.with_extension(format!("tmp-{}", std::process::id()));
    std::fs::write(&temporary, serde_json::to_vec_pretty(value)?)?;
    std::fs::rename(temporary, path)?;
    Ok(())
}

fn load_history(path: &Path, limit: usize) -> Vec<HistoryEntry> {
    let Ok(contents) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    let mut history = contents
        .lines()
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect::<Vec<_>>();
    if history.len() > limit {
        history.drain(0..history.len() - limit);
    }
    history
}

fn append_history(path: &Path, entry: &HistoryEntry) -> anyhow::Result<()> {
    use std::io::Write;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    serde_json::to_writer(&mut file, entry)?;
    file.write_all(b"\n")?;
    Ok(())
}

fn record_file_progress(
    progress: &mut HashMap<(String, String), u64>,
    transfer_id: &str,
    file_id: &str,
    bytes_done: u64,
) -> u64 {
    progress
        .entry((transfer_id.to_string(), file_id.to_string()))
        .and_modify(|current| *current = (*current).max(bytes_done))
        .or_insert(bytes_done);
    progress
        .iter()
        .filter(|((id, _), _)| id == transfer_id)
        .map(|(_, bytes)| *bytes)
        .sum()
}

fn write_private_file(path: &Path, contents: &[u8]) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    #[cfg(unix)]
    {
        use std::io::Write;
        use std::os::unix::fs::OpenOptionsExt;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(path)?;
        file.write_all(contents)?;
    }
    #[cfg(not(unix))]
    std::fs::write(path, contents)?;
    Ok(())
}

#[cfg(test)]
mod settings_tests {
    use super::{
        DEFAULT_MULTICAST_GROUP, DesktopSettings, discovery_timeout_for, interface_filter_for,
        load_settings, multicast_groups_for,
    };
    use std::time::Duration;

    #[test]
    fn legacy_discovery_timeout_seconds_is_migrated_to_milliseconds() {
        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("settings.json");
        std::fs::write(&path, r#"{"alias":"legacy","discovery_timeout_secs":10}"#)
            .expect("write settings");

        let settings = load_settings(&path);
        assert_eq!(settings.alias, "legacy");
        assert_eq!(settings.discovery_timeout_ms, 10_000);
    }

    #[test]
    fn new_discovery_timeout_takes_precedence_over_legacy_key() {
        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("settings.json");
        std::fs::write(
            &path,
            r#"{"discovery_timeout_ms":750,"discovery_timeout_secs":10}"#,
        )
        .expect("write settings");

        let settings = load_settings(&path);
        assert_eq!(settings.discovery_timeout_ms, 750);
    }

    #[test]
    fn missing_settings_file_uses_official_500ms_timeout() {
        let directory = tempfile::tempdir().expect("tempdir");
        let settings = load_settings(&directory.path().join("missing.json"));
        assert_eq!(settings.discovery_timeout_ms, 500);
    }

    #[test]
    fn custom_multicast_group_keeps_official_group_as_compatibility_fallback() {
        let settings = DesktopSettings {
            multicast_group: Some("239.255.88.61".to_string()),
            ..DesktopSettings::default()
        };
        assert_eq!(
            multicast_groups_for(&settings),
            vec!["239.255.88.61".parse().unwrap(), DEFAULT_MULTICAST_GROUP]
        );
    }

    #[test]
    fn invalid_or_empty_multicast_group_uses_official_default() {
        for value in [
            None,
            Some(String::new()),
            Some("not-an-address".to_string()),
        ] {
            let settings = DesktopSettings {
                multicast_group: value,
                ..DesktopSettings::default()
            };
            assert_eq!(
                multicast_groups_for(&settings),
                vec![DEFAULT_MULTICAST_GROUP]
            );
        }
    }

    #[test]
    fn interface_filters_and_timeout_are_copied_into_discovery_configuration() {
        let settings = DesktopSettings {
            discovery_timeout_ms: 0,
            interface_whitelist: Some(vec!["en0".to_string()]),
            interface_blacklist: Some(vec!["utun*".to_string()]),
            ..DesktopSettings::default()
        };
        let filter = interface_filter_for(&settings);
        assert_eq!(filter.whitelist, Some(vec!["en0".to_string()]));
        assert_eq!(filter.blacklist, Some(vec!["utun*".to_string()]));
        assert_eq!(discovery_timeout_for(&settings), Duration::from_millis(1));
    }
}

#[cfg(test)]
mod progress_tests {
    use super::record_file_progress;
    use std::collections::HashMap;

    #[test]
    fn aggregate_file_progress_is_monotonic_across_files() {
        let mut progress = HashMap::new();
        assert_eq!(
            record_file_progress(&mut progress, "transfer", "first", 40),
            40
        );
        assert_eq!(
            record_file_progress(&mut progress, "transfer", "second", 30),
            70
        );
        assert_eq!(
            record_file_progress(&mut progress, "transfer", "first", 25),
            70
        );
        assert_eq!(
            record_file_progress(&mut progress, "transfer", "first", 60),
            90
        );
        assert_eq!(
            record_file_progress(&mut progress, "other", "first", 10),
            10
        );
        assert_eq!(
            record_file_progress(&mut progress, "transfer", "second", 30),
            90
        );
    }
}

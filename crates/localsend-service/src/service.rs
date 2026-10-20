use anyhow::Context;
use localsend_core::crypto::cert::{fingerprint_from_cert_der, generate_self_signed};
use localsend_core::discovery::{
    DEFAULT_DISCOVERY_TIMEOUT, DeviceIdentity, DiscoveredDevice, DiscoveryConfig, DiscoveryEvent,
    DiscoveryHandle, HttpChannel, StatefulDevice,
};
use localsend_core::http::client::LsHttpClient;
use localsend_core::http::client::LsHttpClientVersion;
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
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::sync::{broadcast, mpsc, oneshot, watch};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

const EVENT_BUFFER: usize = 1024;
const SERVER_STOP_TIMEOUT: Duration = Duration::from_secs(2);
const DISCOVERY_STOP_TIMEOUT: Duration = Duration::from_secs(1);

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
        result: Result<(), String>,
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
    discovery: Option<RunningDiscovery>,
    pending_receives: HashMap<String, PendingReceive>,
    web_decisions: HashMap<String, oneshot::Sender<bool>>,
    cancellations: HashMap<String, CancellationToken>,
    file_progress: HashMap<(String, String), u64>,
    web_share: Option<WebShareRuntime>,
    web_auto_accept: bool,
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
            discovery: None,
            pending_receives: HashMap::new(),
            web_decisions: HashMap::new(),
            cancellations: HashMap::new(),
            file_progress: HashMap::new(),
            web_share: None,
            web_auto_accept: false,
        }
    }

    async fn run(mut self) {
        if self.settings.start_server_on_launch {
            if let Err(error) = self.start_server().await {
                self.set_error(format!("starting server: {error:#}"));
            }
        }
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

    fn emit(&self, event: ServiceEvent) -> u64 {
        let mut snapshot = self.snapshot.write();
        snapshot.last_event_seq = snapshot.last_event_seq.saturating_add(1);
        snapshot.revision = snapshot.revision.saturating_add(1);
        let seq = snapshot.last_event_seq;
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
            device_model: Some("localsend-next".to_string()),
            device_type: Some(DeviceType::Desktop),
            token: self.identity.fingerprint.clone(),
        };
        let config = ServerConfigV2 {
            pin: self.settings.pin.clone(),
            verify_checksums: true,
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
                device_model: Some("localsend-next".to_string()),
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
        let server_port = self
            .server
            .as_ref()
            .map(|server| server.handle.port())
            .unwrap_or(self.settings.port);
        let group = self
            .settings
            .multicast_group
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .map(str::parse)
            .transpose()
            .unwrap_or(Some(DEFAULT_MULTICAST_GROUP));
        let (stop_tx, stop_rx) = oneshot::channel();
        let config = DiscoveryConfig {
            group: group.unwrap_or(DEFAULT_MULTICAST_GROUP),
            group_v6: Some(DEFAULT_MULTICAST_GROUP_V6),
            port: if self.settings.port == 0 {
                DEFAULT_MULTICAST_PORT
            } else {
                self.settings.port
            },
            interface_filter: InterfaceFilter {
                whitelist: self.settings.interface_whitelist.clone(),
                blacklist: self.settings.interface_blacklist.clone(),
            },
            device: MulticastDevice {
                alias: self.settings.alias.clone(),
                version: PROTOCOL_VERSION_V2.to_string(),
                device_model: Some("localsend-next".to_string()),
                device_type: Some(DeviceType::Desktop),
                fingerprint: self.identity.fingerprint.clone(),
                port: server_port,
                protocol,
                download: self.web_share.is_some(),
            },
            identity: self.identity.device_identity(),
            timeout: DEFAULT_DISCOVERY_TIMEOUT,
            event_tx: Some(self.discovery_tx.clone()),
        };
        let handle = Arc::new(localsend_core::discovery::start(config, stop_rx).await);
        let announcing = handle.clone();
        tokio::spawn(async move {
            announcing.announce().await;
        });
        self.discovery = Some(RunningDiscovery {
            handle,
            stop_tx: Some(stop_tx),
        });
        Ok(())
    }

    async fn stop_discovery(&mut self) {
        if let Some(mut discovery) = self.discovery.take() {
            if let Some(stop_tx) = discovery.stop_tx.take() {
                let _ = stop_tx.send(());
            }
            let _ =
                tokio::time::timeout(DISCOVERY_STOP_TIMEOUT, discovery.handle.wait_stopped()).await;
        }
    }

    async fn refresh_discovery(&mut self) {
        if self.discovery.is_none() {
            let protocol = protocol_type(self.settings.protocol);
            if let Err(error) = self.start_discovery(protocol).await {
                self.set_error(format!("starting discovery: {error:#}"));
                return;
            }
        }
        if let Some(discovery) = &self.discovery {
            discovery.handle.announce().await;
            let channels = self
                .snapshot()
                .favorites
                .into_iter()
                .filter_map(|favorite| device_to_http_channel(&favorite.device))
                .collect();
            let _ = discovery
                .handle
                .discover_known_http_channels(channels)
                .await;
        }
    }

    async fn add_manual_device(&mut self, target: &str) -> Result<(), String> {
        let (host, port) = parse_target(target, self.settings.port)?;
        if self.discovery.is_none() {
            self.start_discovery(protocol_type(self.settings.protocol))
                .await
                .map_err(|error| error.to_string())?;
        }
        let discovery = self
            .discovery
            .as_ref()
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
        tokio::spawn(run_send_files(
            self.identity.cert_pem.clone(),
            self.identity.key_pem.clone(),
            self.identity.fingerprint.clone(),
            target,
            items,
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
        let transfer_id = Uuid::new_v4().to_string();
        let file = FileView {
            id: Uuid::new_v4().to_string(),
            file_name: "message.txt".to_string(),
            size: 0,
            file_type: "text".to_string(),
            sha256: None,
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
            bytes_total: 0,
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
        tokio::spawn(run_send_text(
            self.identity.cert_pem.clone(),
            self.identity.key_pem.clone(),
            self.identity.fingerprint.clone(),
            target,
            text,
            pin,
            transfer_id,
            cancel,
            self.internal_tx.clone(),
        ));
        Ok(transfer)
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
            || settings.pin != self.settings.pin;
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
                let file_views = files.values().map(file_view).collect::<Vec<_>>();
                if let Some(message) = message_text(&files) {
                    let _ = decision_tx.send(PrepareUploadDecisionV2::Accept(HashSet::new()));
                    self.emit(ServiceEvent::TextReceived {
                        sender_alias: sender.alias.clone(),
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
                    return;
                }
                let auto_accept = !self.frontend_answers_requests
                    && (self.settings.auto_save == AutoSaveMode::On
                        || (self.settings.auto_save == AutoSaveMode::Favorites
                            && self.snapshot.read().favorites.iter().any(|favorite| {
                                favorite.device.fingerprint == sender.fingerprint
                            })));
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
                let status = match reason {
                    SessionEndReasonV2::Finished => TransferStatus::Finished,
                    SessionEndReasonV2::Cancelled => TransferStatus::Cancelled,
                };
                self.set_transfer_status(&session_id, status, None);
            }
            ServerEventV2::PrepareUploadAborted { session_id } => {
                self.pending_receives.remove(&session_id);
                self.snapshot
                    .write()
                    .incoming_requests
                    .retain(|request| request.request_id != session_id);
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
        let destination = self
            .settings
            .destination_dir
            .clone()
            .unwrap_or_else(default_download_dir);
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
        let alias = self
            .snapshot
            .read()
            .active_transfers
            .iter()
            .find(|transfer| transfer.transfer_id == session_id)
            .map(|transfer| transfer.peer_alias.clone())
            .unwrap_or_else(|| "Peer".to_string());
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
                result,
            });
        });
    }

    fn handle_discovery_event(&mut self, event: DiscoveryEvent) {
        match event {
            DiscoveryEvent::Discovered { device } | DiscoveryEvent::Updated { device } => {
                let mut snapshot = self.snapshot.write();
                let view = discovered_device_view(&device);
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
                if let Some(web) = &self.web_share {
                    if let Some(path) = web.paths.get(&file_id) {
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
                self.file_progress
                    .insert((transfer_id.clone(), file_id.clone()), bytes_done);
                let total_done = self
                    .file_progress
                    .iter()
                    .filter(|((id, _), _)| id == &transfer_id)
                    .map(|(_, bytes)| *bytes)
                    .sum::<u64>();
                if let Some(transfer) = self
                    .snapshot
                    .write()
                    .active_transfers
                    .iter_mut()
                    .find(|transfer| transfer.transfer_id == transfer_id)
                {
                    transfer.bytes_done = total_done.min(transfer.bytes_total);
                    transfer.status = TransferStatus::Transferring;
                }
                self.emit(ServiceEvent::TransferProgress {
                    transfer_id,
                    file_id: Some(file_id),
                    file_name: Some(file_name),
                    bytes_done,
                    bytes_total,
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
                self.file_progress
                    .insert((transfer_id.clone(), file_id.clone()), bytes_done);
                let total_done = self
                    .file_progress
                    .iter()
                    .filter(|((id, _), _)| id == &transfer_id)
                    .map(|(_, bytes)| *bytes)
                    .sum::<u64>();
                if let Some(transfer) = self
                    .snapshot
                    .write()
                    .active_transfers
                    .iter_mut()
                    .find(|transfer| transfer.transfer_id == transfer_id)
                {
                    transfer.bytes_done = total_done.min(transfer.bytes_total);
                    transfer.status = TransferStatus::Transferring;
                }
                self.emit(ServiceEvent::TransferProgress {
                    transfer_id,
                    file_id: Some(file_id),
                    file_name: Some(file_name),
                    bytes_done,
                    bytes_total,
                });
            }
            InternalEvent::ReceiveFileDone {
                transfer_id,
                file_id,
                file_name,
                path,
                size,
                alias,
                result,
            } => {
                self.file_progress
                    .remove(&(transfer_id.clone(), file_id.clone()));
                let status = if result.is_ok() {
                    TransferStatus::Finished
                } else {
                    TransferStatus::Failed
                };
                let failure = result.as_ref().err().cloned();
                if let Err(error) = &result {
                    self.set_error(format!("receiving {file_name}: {error}"));
                }
                let entry = HistoryEntry {
                    id: Uuid::new_v4().to_string(),
                    file_id: Some(file_id.clone()),
                    direction: TransferDirection::Receive,
                    peer_alias: alias,
                    file_name,
                    path: result.as_ref().ok().map(|_| path),
                    size,
                    timestamp_ms: unix_ms_now(),
                    status,
                    message: None,
                };
                if self.settings.save_to_history {
                    self.record_history(entry.clone());
                }
                self.emit(ServiceEvent::HistoryAppended { entry });
                if let Some(error) = failure {
                    self.emit(ServiceEvent::TransferFailed {
                        transfer_id,
                        file_id: Some(file_id),
                        error_kind: Some(ServiceErrorKind::Io),
                        error,
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
        let transfer = {
            let mut snapshot = self.snapshot.write();
            let Some(transfer) = snapshot
                .active_transfers
                .iter_mut()
                .find(|transfer| transfer.transfer_id == transfer_id)
            else {
                return;
            };
            transfer.status = status;
            transfer.error = error;
            if matches!(
                status,
                TransferStatus::Finished | TransferStatus::Failed | TransferStatus::Cancelled
            ) {
                transfer.ended_at_ms = Some(unix_ms_now());
                if transfer.bytes_done == 0 && status == TransferStatus::Finished {
                    transfer.bytes_done = transfer.bytes_total;
                }
            }
            transfer.clone()
        };
        self.emit(ServiceEvent::TransferFinished { transfer });
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
        if let Some(discovery) = &self.discovery {
            discovery.handle.add_device(device).await;
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
        let view = stateful_device_view(device);
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
    if let Some((host, port)) = target.rsplit_once(':') {
        if let Ok(port) = port.parse() {
            return Ok((host.trim_matches(['[', ']']).to_string(), port));
        }
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
    cert_pem: String,
    key_pem: String,
    own_fingerprint: String,
    target: DeviceInfo,
    items: Vec<SendItem>,
    pin: Option<String>,
    transfer_id: String,
    cancel: CancellationToken,
    events: mpsc::UnboundedSender<InternalEvent>,
) {
    let client_version = if target.version.starts_with('3') {
        LsHttpClientVersion::V3
    } else {
        LsHttpClientVersion::V2
    };
    let expected = (target.protocol == Protocol::Https).then(|| target.fingerprint.clone());
    let client =
        match LsHttpClient::new(&key_pem, &cert_pem, client_version, expected.clone(), None) {
            Ok(client) => client,
            Err(error) => {
                let _ = events.send(InternalEvent::SendFailed {
                    transfer_id,
                    file_id: None,
                    kind: ServiceErrorKind::Internal,
                    error: error.to_string(),
                });
                return;
            }
        };
    let files: HashMap<String, FileDto> = items
        .iter()
        .map(|item| (item.file.id.clone(), item.file.clone()))
        .collect();
    let register = RegisterDto {
        alias: "localsend-next".to_string(),
        version: PROTOCOL_VERSION_V2.to_string(),
        device_model: Some("localsend-next".to_string()),
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
    let prepared = match client
        .prepare_upload(
            protocol_type(target.protocol),
            host,
            target.port,
            expected.clone(),
            payload,
            pin.as_deref(),
            cancel.clone(),
        )
        .await
    {
        Ok(prepared) => prepared,
        Err(error) => {
            let (kind, message) = classify_send_error(&error);
            let _ = events.send(InternalEvent::SendFailed {
                transfer_id,
                file_id: None,
                kind,
                error: message,
            });
            return;
        }
    };
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
                expected.clone(),
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
    let _ = events.send(InternalEvent::SendFinished { transfer_id });
}

#[allow(clippy::too_many_arguments)]
async fn run_send_text(
    cert_pem: String,
    key_pem: String,
    own_fingerprint: String,
    target: DeviceInfo,
    text: String,
    pin: Option<String>,
    transfer_id: String,
    cancel: CancellationToken,
    events: mpsc::UnboundedSender<InternalEvent>,
) {
    let client_version = if target.version.starts_with('3') {
        LsHttpClientVersion::V3
    } else {
        LsHttpClientVersion::V2
    };
    let expected = (target.protocol == Protocol::Https).then(|| target.fingerprint.clone());
    let Ok(client) = LsHttpClient::new(&key_pem, &cert_pem, client_version, expected.clone(), None)
    else {
        let _ = events.send(InternalEvent::SendFailed {
            transfer_id,
            file_id: None,
            kind: ServiceErrorKind::Internal,
            error: "failed to create HTTP client".to_string(),
        });
        return;
    };
    let file_id = Uuid::new_v4().to_string();
    let file = FileDto {
        id: file_id.clone(),
        file_name: "message.txt".to_string(),
        size: 0,
        file_type: "text".to_string(),
        sha256: None,
        preview: Some(text),
        metadata: None,
    };
    let payload = PrepareUploadRequestDto {
        info: RegisterDto {
            alias: "localsend-next".to_string(),
            version: PROTOCOL_VERSION_V2.to_string(),
            device_model: Some("localsend-next".to_string()),
            device_type: Some(DeviceType::Desktop),
            token: own_fingerprint,
            port: 0,
            protocol: protocol_type(target.protocol),
            has_web_interface: false,
        },
        files: HashMap::from([(file_id, file)]),
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
    match client
        .prepare_upload(
            protocol_type(target.protocol),
            host,
            target.port,
            expected,
            payload,
            pin.as_deref(),
            cancel,
        )
        .await
    {
        Ok(_) => {
            let _ = events.send(InternalEvent::SendFinished { transfer_id });
        }
        Err(error) => {
            let (kind, message) = classify_send_error(&error);
            let _ = events.send(InternalEvent::SendFailed {
                transfer_id,
                file_id: None,
                kind,
                error: message,
            });
        }
    }
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

fn unique_path(dir: &Path, file_name: &str) -> PathBuf {
    let name = filename::sanitize_path(file_name, filename::Rules::current());
    let candidate = dir.join(&name);
    if !candidate.exists() {
        return candidate;
    }
    let (stem, extension) = match name.rsplit_once('.') {
        Some((stem, extension)) if !stem.is_empty() => (stem, format!(".{extension}")),
        _ => (name.as_str(), String::new()),
    };
    (1..)
        .map(|index| dir.join(format!("{stem} ({index}){extension}")))
        .find(|candidate| !candidate.exists())
        .unwrap()
}

fn load_settings(path: &Path) -> DesktopSettings {
    load_json(path).unwrap_or_default()
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

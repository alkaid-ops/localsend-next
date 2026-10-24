use localsend_rpc::snapshot_store::{EventApply, SnapshotStore};
use localsend_rpc::{
    AutoSaveMode, CommandResult, DaemonClient, DesktopSettings, DeviceInfo, ReceiverDecision,
    SendFileRequest, SequencedEvent, ServiceCommand, ServiceErrorKind, ServiceEvent,
    ServiceSnapshot, TransferDirection, TransferStatus, TransferView, WebShareMode,
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc as std_mpsc;
use std::thread;
use tokio::sync::mpsc;
use tokio::time::Duration;

#[derive(Debug)]
pub enum UiCommand {
    RefreshDiscovery,
    ToggleServer,
    RestartServer,
    SetAutoSave {
        all: bool,
        favorites: bool,
    },
    StartWebReceive,
    RestartWebReceive,
    StopWebShare,
    AddManual(String),
    VerifyDevice(String),
    ToggleFavorite(String),
    SendFiles {
        fingerprint: String,
        paths: Vec<PathBuf>,
    },
    SendText {
        fingerprint: String,
        text: String,
    },
    RetryPin {
        transfer_id: String,
        pin: String,
    },
    RespondTransfer {
        request_id: String,
        decision: ReceiverDecision,
    },
    CancelTransfer(String),
    DismissTransfer(String),
    ClearHistory,
    UpdateSettings {
        expected_revision: u64,
        settings: Box<DesktopSettings>,
    },
    ShutdownDaemon,
}

#[derive(Debug, Clone)]
pub enum WorkerEvent {
    Snapshot(Box<ServiceSnapshot>),
    DaemonEvent(Box<SequencedEvent>),
    Notice(String),
    Connection {
        connected: bool,
        message: String,
        show_notice: bool,
    },
    PinRequired {
        transfer_id: String,
    },
    TransferCreated {
        transfer_id: String,
        files: bool,
    },
    TextReceived {
        sender_alias: String,
        sender_device_model: Option<String>,
        text: String,
    },
}

pub struct DaemonWorker {
    pub commands: mpsc::UnboundedSender<UiCommand>,
    pub events: std_mpsc::Receiver<WorkerEvent>,
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

impl DaemonWorker {
    pub fn spawn() -> Self {
        Self::spawn_inner(None)
    }

    pub fn spawn_with_endpoint(endpoint: impl Into<String>) -> Self {
        Self::spawn_inner(Some(endpoint.into()))
    }

    fn spawn_inner(endpoint: Option<String>) -> Self {
        let (commands, command_rx) = mpsc::unbounded_channel();
        let (event_tx, events) = std_mpsc::channel();
        thread::Builder::new()
            .name("localsend-gui-slint-daemon-worker".to_string())
            .spawn(move || {
                let runtime = match tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                {
                    Ok(runtime) => runtime,
                    Err(error) => {
                        let _ = event_tx.send(WorkerEvent::Connection {
                            connected: false,
                            message: format!("Cannot start daemon worker: {error}"),
                            show_notice: true,
                        });
                        return;
                    }
                };
                runtime.block_on(run_worker(endpoint, command_rx, event_tx));
            })
            .expect("spawn daemon worker");
        Self { commands, events }
    }
}

async fn run_worker(
    endpoint: Option<String>,
    mut commands: mpsc::UnboundedReceiver<UiCommand>,
    events_out: std_mpsc::Sender<WorkerEvent>,
) {
    let mut pending_sends = HashMap::<String, PendingSend>::new();
    let mut connection_attempts = 0_u32;
    let mut ever_connected = false;
    loop {
        let connected = match endpoint.as_deref() {
            Some(endpoint) => DaemonClient::connect_to_interactive(endpoint).await,
            None => DaemonClient::connect_interactive().await,
        };
        let client = match connected {
            Ok(client) => client,
            Err(error) => {
                connection_attempts = connection_attempts.saturating_add(1);
                if connection_attempts == 1 || connection_attempts.is_multiple_of(50) {
                    eprintln!(
                        "localsendd connection attempt {connection_attempts} failed: {error:#}"
                    );
                }
                let (message, show_notice) = connection_failure_message(&error, ever_connected);
                let _ = events_out.send(WorkerEvent::Connection {
                    connected: false,
                    message,
                    show_notice,
                });
                if commands.is_closed() {
                    return;
                }
                let retry_delay = if !ever_connected && connection_attempts < 50 {
                    Duration::from_millis(100)
                } else {
                    Duration::from_secs(1)
                };
                tokio::time::sleep(retry_delay).await;
                continue;
            }
        };
        ever_connected = true;
        connection_attempts = 0;
        let _ = events_out.send(WorkerEvent::Connection {
            connected: true,
            message: "Connected to localsendd".to_string(),
            show_notice: false,
        });
        let snapshot = match client.snapshot() {
            Some(snapshot) => snapshot,
            None => match fetch_snapshot(&client).await {
                Some(snapshot) => snapshot,
                None => {
                    let _ = events_out.send(WorkerEvent::Connection {
                        connected: false,
                        message: "Daemon did not provide an initial snapshot".to_string(),
                        show_notice: true,
                    });
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue;
                }
            },
        };
        let mut state = SnapshotStore::new(snapshot.clone());
        let _ = events_out.send(WorkerEvent::Snapshot(Box::new(snapshot)));
        let mut daemon_events = client.subscribe();
        let mut disconnected = false;

        while !disconnected {
            tokio::select! {
                command = commands.recv() => {
                    let Some(command) = command else { return; };
                    if handle_command(
                        command,
                        &client,
                        &events_out,
                        &mut pending_sends,
                        &mut state,
                    ).await {
                        return;
                    }
                }
                event = daemon_events.recv() => {
                    match event {
                        Ok(event) => {
                            match &event.event {
                                ServiceEvent::TextReceived {
                                    sender_alias,
                                    sender_device_model,
                                    text,
                                } => {
                                    let _ = events_out.send(WorkerEvent::TextReceived {
                                        sender_alias: sender_alias.clone(),
                                        sender_device_model: sender_device_model.clone(),
                                        text: text.clone(),
                                    });
                                }
                                ServiceEvent::TransferFailed {
                                    transfer_id,
                                    error_kind: Some(ServiceErrorKind::InvalidPin | ServiceErrorKind::PinRequired),
                                    ..
                                } if pending_sends.contains_key(transfer_id) => {
                                    let _ = events_out.send(WorkerEvent::PinRequired {
                                        transfer_id: transfer_id.clone(),
                                    });
                                }
                                ServiceEvent::TransferFailed {
                                    transfer_id, error, ..
                                } => {
                                    pending_sends.remove(transfer_id);
                                    let _ = events_out.send(WorkerEvent::Notice(format!(
                                        "传输失败：{error}"
                                    )));
                                }
                                ServiceEvent::TransferFinished { transfer }
                                    if transfer.status != TransferStatus::Failed =>
                                {
                                    pending_sends.remove(&transfer.transfer_id);
                                    if transfer.status == TransferStatus::Finished
                                        && transfer.direction == TransferDirection::Send
                                    {
                                        let subject = sent_transfer_subject(transfer);
                                        let _ = events_out.send(WorkerEvent::Notice(format!(
                                            "{subject}已发送给 {}",
                                            transfer.peer_alias
                                        )));
                                    }
                                }
                                _ => {}
                            }
                            match state.apply(&event) {
                                EventApply::Applied => {
                                    if let ServiceEvent::Snapshot { snapshot } = &event.event {
                                        let _ = events_out.send(WorkerEvent::Snapshot(Box::new(
                                            snapshot.clone(),
                                        )));
                                    } else {
                                        let _ = events_out.send(WorkerEvent::DaemonEvent(Box::new(event)));
                                    }
                                }
                                EventApply::Duplicate => {}
                                EventApply::Gap => {
                                    if let Some(snapshot) = fetch_snapshot(&client).await {
                                        state.replace(snapshot.clone());
                                        let _ = events_out.send(WorkerEvent::Snapshot(Box::new(snapshot)));
                                    } else {
                                        disconnected = true;
                                    }
                                }
                            }
                        }
                        Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                            if let Some(snapshot) = fetch_snapshot(&client).await {
                                state.replace(snapshot.clone());
                                let _ = events_out.send(WorkerEvent::Snapshot(Box::new(snapshot)));
                            } else {
                                disconnected = true;
                            }
                        }
                        Err(tokio::sync::broadcast::error::RecvError::Closed) => disconnected = true,
                    }
                }
            }
        }
        let _ = events_out.send(WorkerEvent::Connection {
            connected: false,
            message: "Connection to localsendd lost; reconnecting".to_string(),
            show_notice: false,
        });
        pending_sends.clear();
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

fn connection_failure_message(error: &anyhow::Error, ever_connected: bool) -> (String, bool) {
    if error.downcast_ref::<std::io::Error>().is_some() {
        let message = if ever_connected {
            "与后台服务的连接已断开，正在重连…"
        } else {
            "正在连接后台服务…"
        };
        (message.to_string(), false)
    } else {
        (
            "无法连接后台服务，请确认 localsendd 正在运行".to_string(),
            true,
        )
    }
}

async fn fetch_snapshot(client: &DaemonClient) -> Option<ServiceSnapshot> {
    match client.command(ServiceCommand::GetSnapshot).await {
        Ok(CommandResult::Snapshot { snapshot }) => Some(snapshot),
        Ok(_) | Err(_) => None,
    }
}

async fn handle_command(
    command: UiCommand,
    client: &DaemonClient,
    events: &std_mpsc::Sender<WorkerEvent>,
    pending_sends: &mut HashMap<String, PendingSend>,
    state: &mut SnapshotStore,
) -> bool {
    let result: Result<Option<(String, PendingSend)>, String> = match command {
        UiCommand::RefreshDiscovery => client
            .command(ServiceCommand::RefreshDiscovery)
            .await
            .map(|_| None),
        UiCommand::ToggleServer => {
            let running = state.snapshot().server_running;
            let command = if running {
                ServiceCommand::StopServer
            } else {
                ServiceCommand::StartServer
            };
            client.command(command).await.map(|_| None)
        }
        UiCommand::RestartServer => client
            .command(ServiceCommand::RestartServer)
            .await
            .map(|_| None),
        UiCommand::StartWebReceive => {
            let pin = state.snapshot().settings.pin.clone();
            let auto_accept = state.snapshot().settings.auto_accept;
            client
                .command(ServiceCommand::StartWebShare {
                    files: Vec::new(),
                    pin,
                    mode: WebShareMode::Receive,
                    auto_accept,
                    i18n: Default::default(),
                })
                .await
                .map(|_| None)
        }
        UiCommand::RestartWebReceive => {
            let settings = &state.snapshot().settings;
            let pin = settings.pin.clone();
            let auto_accept = settings.auto_accept;
            if let Err(error) = client.command(ServiceCommand::StopWebShare).await {
                return emit_error(events, error);
            }
            client
                .command(ServiceCommand::StartWebShare {
                    files: Vec::new(),
                    pin,
                    mode: WebShareMode::Receive,
                    auto_accept,
                    i18n: Default::default(),
                })
                .await
                .map(|_| None)
        }
        UiCommand::StopWebShare => client
            .command(ServiceCommand::StopWebShare)
            .await
            .map(|_| None),
        UiCommand::SetAutoSave { all, favorites } => {
            update_settings(client, state, move |settings| {
                settings.auto_save_from_favorites = favorites;
                settings.auto_save = if all {
                    AutoSaveMode::On
                } else if favorites {
                    AutoSaveMode::Favorites
                } else {
                    AutoSaveMode::Off
                };
            })
            .await
            .map(|_| None)
        }
        UiCommand::AddManual(target) => client
            .command(ServiceCommand::AddManualDevice { target })
            .await
            .map(|_| None),
        UiCommand::VerifyDevice(fingerprint) => {
            let result = client
                .command(ServiceCommand::RefreshDiscovery)
                .await
                .map(|_| None);
            if result.is_ok() {
                let _ = events.send(WorkerEvent::Notice(format!(
                    "已刷新设备连接：{}",
                    fingerprint.chars().take(12).collect::<String>()
                )));
            }
            result
        }
        UiCommand::ToggleFavorite(fingerprint) => {
            let snapshot = state.snapshot();
            if snapshot
                .favorites
                .iter()
                .any(|favorite| favorite.device.fingerprint == fingerprint)
            {
                client
                    .command(ServiceCommand::RemoveFavorite { fingerprint })
                    .await
                    .map(|_| None)
            } else {
                let Some(device) = snapshot
                    .devices
                    .iter()
                    .find(|device| device.device.fingerprint == fingerprint)
                    .map(|device| device.device.clone())
                else {
                    let _ = events.send(WorkerEvent::Notice(
                        "Device is offline and is not in the daemon snapshot".into(),
                    ));
                    return false;
                };
                client
                    .command(ServiceCommand::AddFavorite {
                        fingerprint,
                        alias: None,
                        device: Some(device),
                    })
                    .await
                    .map(|_| None)
            }
        }
        UiCommand::SendFiles { fingerprint, paths } => {
            let target = match resolve_target(state, &fingerprint) {
                Ok(target) => target,
                Err(error) => {
                    let _ = events.send(WorkerEvent::Notice(error));
                    return false;
                }
            };
            let pending = PendingSend::Files {
                target: target.clone(),
                paths: paths.clone(),
            };
            let files = paths
                .into_iter()
                .map(|path| SendFileRequest {
                    path,
                    file_id: None,
                    sha256: None,
                    metadata: None,
                })
                .collect();
            send_command(
                client,
                ServiceCommand::SendFiles {
                    target_fingerprint: Some(fingerprint),
                    target: Some(target),
                    files,
                    pin: None,
                },
            )
            .await
            .map(|transfer_id| Some((transfer_id, pending)))
        }
        UiCommand::SendText { fingerprint, text } => {
            let target = match resolve_target(state, &fingerprint) {
                Ok(target) => target,
                Err(error) => {
                    let _ = events.send(WorkerEvent::Notice(error));
                    return false;
                }
            };
            let pending = PendingSend::Text {
                target: target.clone(),
                text: text.clone(),
            };
            send_command(
                client,
                ServiceCommand::SendText {
                    target_fingerprint: Some(fingerprint),
                    target: Some(target),
                    text,
                    pin: None,
                },
            )
            .await
            .map(|transfer_id| Some((transfer_id, pending)))
        }
        UiCommand::RetryPin { transfer_id, pin } => {
            let Some(pending) = pending_sends.remove(&transfer_id) else {
                let _ = events.send(WorkerEvent::Notice(
                    "The pending send is no longer available".into(),
                ));
                return false;
            };
            match pending {
                PendingSend::Files { target, paths } => {
                    let files = paths
                        .iter()
                        .cloned()
                        .map(|path| SendFileRequest {
                            path,
                            file_id: None,
                            sha256: None,
                            metadata: None,
                        })
                        .collect();
                    send_command(
                        client,
                        ServiceCommand::SendFiles {
                            target_fingerprint: Some(target.fingerprint.clone()),
                            target: Some(target.clone()),
                            files,
                            pin: Some(pin),
                        },
                    )
                    .await
                    .map(|new_id| Some((new_id, PendingSend::Files { target, paths })))
                }
                PendingSend::Text { target, text } => send_command(
                    client,
                    ServiceCommand::SendText {
                        target_fingerprint: Some(target.fingerprint.clone()),
                        target: Some(target.clone()),
                        text: text.clone(),
                        pin: Some(pin),
                    },
                )
                .await
                .map(|new_id| Some((new_id, PendingSend::Text { target, text }))),
            }
        }
        UiCommand::RespondTransfer {
            request_id,
            decision,
        } => client
            .command(ServiceCommand::RespondTransfer {
                request_id,
                decision,
            })
            .await
            .map(|_| None),
        UiCommand::CancelTransfer(transfer_id) => client
            .command(ServiceCommand::CancelTransfer { transfer_id })
            .await
            .map(|_| None),
        UiCommand::DismissTransfer(transfer_id) => client
            .command(ServiceCommand::DismissTransfer { transfer_id })
            .await
            .map(|_| None),
        UiCommand::ClearHistory => client
            .command(ServiceCommand::ClearHistory)
            .await
            .map(|_| None),
        UiCommand::UpdateSettings {
            expected_revision,
            settings,
        } => client
            .command(ServiceCommand::UpdateSettings {
                expected_revision,
                settings: *settings,
            })
            .await
            .map(|_| None),
        UiCommand::ShutdownDaemon => {
            let _ = client.command(ServiceCommand::Shutdown).await;
            return true;
        }
    };

    match result {
        Ok(pending) => {
            if let Some((transfer_id, pending)) = pending {
                let files = matches!(pending, PendingSend::Files { .. });
                let _ = events.send(WorkerEvent::TransferCreated {
                    transfer_id: transfer_id.clone(),
                    files,
                });
                pending_sends.insert(transfer_id, pending);
            }
        }
        Err(error) => {
            if error.contains("revision conflict")
                && let Some(snapshot) = fetch_snapshot(client).await
            {
                state.replace(snapshot.clone());
                let _ = events.send(WorkerEvent::Snapshot(Box::new(snapshot)));
            }
            let _ = events.send(WorkerEvent::Notice(error));
        }
    }
    false
}

fn resolve_target(state: &SnapshotStore, fingerprint: &str) -> Result<DeviceInfo, String> {
    let snapshot = state.snapshot();
    snapshot
        .devices
        .iter()
        .find(|device| device.device.fingerprint == fingerprint)
        .map(|device| device.device.clone())
        .or_else(|| {
            snapshot
                .favorites
                .iter()
                .find(|favorite| favorite.device.fingerprint == fingerprint)
                .map(|favorite| favorite.device.clone())
        })
        .ok_or_else(|| {
            "target device is no longer available; refresh discovery and retry".to_string()
        })
}

async fn send_command(client: &DaemonClient, command: ServiceCommand) -> Result<String, String> {
    match client.command(command).await? {
        CommandResult::Transfer { transfer } => Ok(transfer.transfer_id),
        _ => Err("Daemon did not create a transfer".to_string()),
    }
}

async fn update_settings(
    client: &DaemonClient,
    state: &SnapshotStore,
    update: impl FnOnce(&mut DesktopSettings),
) -> Result<(), String> {
    let mut settings = state.snapshot().settings.clone();
    update(&mut settings);
    client
        .command(ServiceCommand::UpdateSettings {
            expected_revision: settings.revision,
            settings,
        })
        .await
        .map(|_| ())
}

fn emit_error(events: &std_mpsc::Sender<WorkerEvent>, error: String) -> bool {
    let _ = events.send(WorkerEvent::Notice(error));
    false
}

fn sent_transfer_subject(transfer: &TransferView) -> &'static str {
    let is_text_message = transfer.files.iter().any(|file| {
        (file.file_type == "text" || file.file_type.starts_with("text/")) && file.preview.is_some()
    });
    if is_text_message { "文本" } else { "文件" }
}

#[cfg(test)]
mod tests {
    use super::{connection_failure_message, sent_transfer_subject};
    use localsend_rpc::{FileView, TransferDirection, TransferStatus, TransferView};
    use std::io;

    #[test]
    fn missing_socket_during_startup_is_a_silent_retry_state() {
        let error = anyhow::Error::from(io::Error::from(io::ErrorKind::NotFound));
        let (message, show_notice) = connection_failure_message(&error, false);
        assert!(!show_notice);
        assert!(message.contains("正在连接"));
    }

    #[test]
    fn io_failure_after_connection_is_a_silent_reconnect_state() {
        let error = anyhow::Error::from(io::Error::from(io::ErrorKind::ConnectionReset));
        let (message, show_notice) = connection_failure_message(&error, true);
        assert!(!show_notice);
        assert!(message.contains("正在重连"));
    }

    #[test]
    fn protocol_failure_is_reported_to_the_user() {
        let error = anyhow::anyhow!("unsupported daemon protocol");
        let (_, show_notice) = connection_failure_message(&error, false);
        assert!(show_notice);
    }

    #[test]
    fn text_mime_without_preview_is_reported_as_a_file() {
        let base = TransferView {
            transfer_id: "transfer".to_string(),
            direction: TransferDirection::Send,
            peer_alias: "peer".to_string(),
            peer_fingerprint: "fingerprint".to_string(),
            files: vec![FileView {
                id: "file".to_string(),
                file_name: "notes.txt".to_string(),
                size: 5,
                file_type: "text/plain".to_string(),
                sha256: None,
                preview: None,
            }],
            status: TransferStatus::Finished,
            bytes_done: 5,
            bytes_total: 5,
            started_at_ms: 0,
            ended_at_ms: Some(1),
            error: None,
        };
        assert_eq!(sent_transfer_subject(&base), "文件");

        let mut message = base;
        message.files[0].preview = Some("hello".to_string());
        assert_eq!(sent_transfer_subject(&message), "文本");
    }
}

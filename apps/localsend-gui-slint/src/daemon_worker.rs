use localsend_rpc::{
    AutoSaveMode, CommandResult, DaemonClient, DesktopSettings, ReceiverDecision, SendFileRequest,
    ServiceCommand, ServiceErrorKind, ServiceEvent, ServiceSnapshot, TransferStatus, WebShareMode,
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc as std_mpsc;
use std::thread;
use tokio::sync::mpsc;
use tokio::time::{Duration, MissedTickBehavior, interval};

#[derive(Debug)]
pub enum UiCommand {
    RefreshDiscovery,
    ToggleServer,
    RestartServer,
    SetAutoSave(AutoSaveMode),
    StartWebReceive,
    StopWebShare,
    AddManual(String),
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
    Notice(String),
    Connection { connected: bool, message: String },
    PinRequired { transfer_id: String },
    TransferCreated { transfer_id: String, files: bool },
    TextReceived { sender_alias: String, text: String },
}

pub struct DaemonWorker {
    pub commands: mpsc::UnboundedSender<UiCommand>,
    pub events: std_mpsc::Receiver<WorkerEvent>,
}

#[derive(Debug, Clone)]
enum PendingSend {
    Files {
        fingerprint: String,
        paths: Vec<PathBuf>,
    },
    Text {
        fingerprint: String,
        text: String,
    },
}

impl DaemonWorker {
    pub fn spawn() -> Self {
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
                        });
                        return;
                    }
                };
                runtime.block_on(run_worker(command_rx, event_tx));
            })
            .expect("spawn daemon worker");
        Self { commands, events }
    }
}

async fn run_worker(
    mut commands: mpsc::UnboundedReceiver<UiCommand>,
    events_out: std_mpsc::Sender<WorkerEvent>,
) {
    let mut pending_sends = HashMap::<String, PendingSend>::new();
    loop {
        let client = match DaemonClient::connect_interactive().await {
            Ok(client) => client,
            Err(error) => {
                let _ = events_out.send(WorkerEvent::Connection {
                    connected: false,
                    message: error.to_string(),
                });
                if commands.is_closed() {
                    return;
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }
        };
        let _ = events_out.send(WorkerEvent::Connection {
            connected: true,
            message: "Connected to localsendd".to_string(),
        });
        publish_snapshot(&client, &events_out).await;
        let mut daemon_events = client.subscribe();
        let mut refresh = interval(Duration::from_millis(100));
        refresh.set_missed_tick_behavior(MissedTickBehavior::Skip);
        let mut disconnected = false;

        while !disconnected {
            tokio::select! {
                command = commands.recv() => {
                    let Some(command) = command else { return; };
                    if handle_command(command, &client, &events_out, &mut pending_sends).await {
                        return;
                    }
                }
                event = daemon_events.recv() => {
                    match event {
                        Ok(event) => {
                            match event.event {
                                ServiceEvent::TextReceived { sender_alias, text } => {
                                    let _ = events_out.send(WorkerEvent::TextReceived {
                                        sender_alias,
                                        text,
                                    });
                                }
                                ServiceEvent::TransferFailed {
                                    transfer_id,
                                    error_kind: Some(ServiceErrorKind::InvalidPin | ServiceErrorKind::PinRequired),
                                    ..
                                } if pending_sends.contains_key(&transfer_id) => {
                                    let _ = events_out.send(WorkerEvent::PinRequired { transfer_id });
                                }
                                ServiceEvent::TransferFailed { transfer_id, .. } => {
                                    pending_sends.remove(&transfer_id);
                                }
                                ServiceEvent::TransferFinished { transfer }
                                    if transfer.status != TransferStatus::Failed =>
                                {
                                    pending_sends.remove(&transfer.transfer_id);
                                }
                                _ => {}
                            }
                            if !publish_snapshot(&client, &events_out).await {
                                disconnected = true;
                            }
                        }
                        Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                            if !publish_snapshot(&client, &events_out).await {
                                disconnected = true;
                            }
                        }
                        Err(tokio::sync::broadcast::error::RecvError::Closed) => disconnected = true,
                    }
                }
                _ = refresh.tick() => {
                    if !publish_snapshot(&client, &events_out).await {
                        disconnected = true;
                    }
                }
            }
        }
        let _ = events_out.send(WorkerEvent::Connection {
            connected: false,
            message: "Connection to localsendd lost; reconnecting".to_string(),
        });
        pending_sends.clear();
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn publish_snapshot(client: &DaemonClient, events: &std_mpsc::Sender<WorkerEvent>) -> bool {
    match client.command(ServiceCommand::GetSnapshot).await {
        Ok(CommandResult::Snapshot { snapshot }) => {
            let _ = events.send(WorkerEvent::Snapshot(Box::new(snapshot)));
            true
        }
        Ok(_) => true,
        Err(error) => {
            let _ = events.send(WorkerEvent::Connection {
                connected: false,
                message: error,
            });
            false
        }
    }
}

async fn handle_command(
    command: UiCommand,
    client: &DaemonClient,
    events: &std_mpsc::Sender<WorkerEvent>,
    pending_sends: &mut HashMap<String, PendingSend>,
) -> bool {
    let result: Result<Option<(String, PendingSend)>, String> = match command {
        UiCommand::RefreshDiscovery => client
            .command(ServiceCommand::RefreshDiscovery)
            .await
            .map(|_| None),
        UiCommand::ToggleServer => {
            let snapshot = match client.command(ServiceCommand::GetSnapshot).await {
                Ok(CommandResult::Snapshot { snapshot }) => snapshot,
                Ok(_) => return emit_error(events, "Daemon did not return a snapshot".into()),
                Err(error) => return emit_error(events, error),
            };
            let running = snapshot.server_running;
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
            let snapshot = match client.command(ServiceCommand::GetSnapshot).await {
                Ok(CommandResult::Snapshot { snapshot }) => snapshot,
                Ok(_) => return emit_error(events, "Daemon did not return a snapshot".into()),
                Err(error) => return emit_error(events, error),
            };
            client
                .command(ServiceCommand::StartWebShare {
                    files: Vec::new(),
                    pin: snapshot.settings.pin,
                    mode: WebShareMode::Receive,
                    auto_accept: snapshot.settings.auto_accept,
                    i18n: Default::default(),
                })
                .await
                .map(|_| None)
        }
        UiCommand::StopWebShare => client
            .command(ServiceCommand::StopWebShare)
            .await
            .map(|_| None),
        UiCommand::SetAutoSave(mode) => update_settings(client, move |settings| {
            settings.auto_save = mode;
        })
        .await
        .map(|_| None),
        UiCommand::AddManual(target) => client
            .command(ServiceCommand::AddManualDevice { target })
            .await
            .map(|_| None),
        UiCommand::ToggleFavorite(fingerprint) => {
            let snapshot = match client.command(ServiceCommand::GetSnapshot).await {
                Ok(CommandResult::Snapshot { snapshot }) => snapshot,
                Ok(_) => {
                    let _ = events.send(WorkerEvent::Notice("Daemon returned no snapshot".into()));
                    return false;
                }
                Err(error) => return emit_error(events, error),
            };
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
            let pending = PendingSend::Files {
                fingerprint: fingerprint.clone(),
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
                    target: None,
                    files,
                    pin: None,
                },
            )
            .await
            .map(|transfer_id| Some((transfer_id, pending)))
        }
        UiCommand::SendText { fingerprint, text } => {
            let pending = PendingSend::Text {
                fingerprint: fingerprint.clone(),
                text: text.clone(),
            };
            send_command(
                client,
                ServiceCommand::SendText {
                    target_fingerprint: Some(fingerprint),
                    target: None,
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
                PendingSend::Files { fingerprint, paths } => {
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
                            target_fingerprint: Some(fingerprint.clone()),
                            target: None,
                            files,
                            pin: Some(pin),
                        },
                    )
                    .await
                    .map(|new_id| Some((new_id, PendingSend::Files { fingerprint, paths })))
                }
                PendingSend::Text { fingerprint, text } => send_command(
                    client,
                    ServiceCommand::SendText {
                        target_fingerprint: Some(fingerprint.clone()),
                        target: None,
                        text: text.clone(),
                        pin: Some(pin),
                    },
                )
                .await
                .map(|new_id| Some((new_id, PendingSend::Text { fingerprint, text }))),
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
            publish_snapshot(client, events).await;
        }
        Err(error) => {
            let _ = events.send(WorkerEvent::Notice(error));
            publish_snapshot(client, events).await;
        }
    }
    false
}

async fn send_command(client: &DaemonClient, command: ServiceCommand) -> Result<String, String> {
    match client.command(command).await? {
        CommandResult::Transfer { transfer } => Ok(transfer.transfer_id),
        _ => Err("Daemon did not create a transfer".to_string()),
    }
}

async fn update_settings(
    client: &DaemonClient,
    update: impl FnOnce(&mut DesktopSettings),
) -> Result<(), String> {
    let snapshot = match client.command(ServiceCommand::GetSnapshot).await? {
        CommandResult::Snapshot { snapshot } => snapshot,
        _ => return Err("Daemon did not return settings snapshot".to_string()),
    };
    let mut settings = snapshot.settings;
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

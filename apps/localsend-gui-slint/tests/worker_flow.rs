#![cfg(unix)]

use localsend_gui_slint::daemon_worker::{DaemonWorker, UiCommand, WorkerEvent};
use localsend_rpc::snapshot_store::{EventApply, SnapshotStore};
use localsend_rpc::{ReceiverDecision, ServiceSnapshot};
use localsend_testkit::{
    DaemonFixture, PeerAcceptance, PeerFile, ProtocolType, ReceiverPeer, SenderPeer,
    daemon_binary_for_tests, default_daemon_settings,
};
use std::time::Duration;
use tempfile::TempDir;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{sleep, timeout};

struct WorkerProbe {
    worker: DaemonWorker,
    state: Option<SnapshotStore>,
}

impl WorkerProbe {
    fn spawn_with_endpoint(endpoint: impl Into<String>) -> Self {
        Self {
            worker: DaemonWorker::spawn_with_endpoint(endpoint),
            state: None,
        }
    }

    fn next_event(&mut self, deadline: std::time::Instant) -> WorkerEvent {
        let remaining = deadline.saturating_duration_since(std::time::Instant::now());
        assert!(
            !remaining.is_zero(),
            "timed out waiting for worker event; initial snapshot received={}",
            self.state.is_some()
        );
        let event = self
            .worker
            .events
            .recv_timeout(remaining)
            .unwrap_or_else(|error| {
                panic!(
                    "worker event wait failed: {error}; initial snapshot received={}",
                    self.state.is_some()
                )
            });
        match event {
            WorkerEvent::Snapshot(snapshot) => {
                self.state = Some(SnapshotStore::new((*snapshot).clone()));
                WorkerEvent::Snapshot(snapshot)
            }
            WorkerEvent::DaemonEvent(event) => {
                let state = self
                    .state
                    .as_mut()
                    .expect("daemon event arrived before the initial snapshot");
                match state.apply(&event) {
                    EventApply::Applied => WorkerEvent::DaemonEvent(event),
                    EventApply::Duplicate => self.next_event(deadline),
                    EventApply::Gap => panic!("worker reported a daemon event sequence gap"),
                }
            }
            WorkerEvent::Connection {
                connected: false,
                message,
                show_notice: _,
            } => panic!("worker disconnected: {message}"),
            event => event,
        }
    }

    fn current_snapshot(&self) -> Option<ServiceSnapshot> {
        self.state.as_ref().map(|state| state.snapshot().clone())
    }
}

impl std::ops::Deref for WorkerProbe {
    type Target = DaemonWorker;

    fn deref(&self) -> &Self::Target {
        &self.worker
    }
}

#[test]
fn missing_daemon_socket_uses_a_silent_startup_state() {
    let root = TempDir::new().expect("temp root");
    let endpoint = root.path().join("not-created.sock");
    let worker = DaemonWorker::spawn_with_endpoint(endpoint.to_string_lossy());
    let event = worker
        .events
        .recv_timeout(Duration::from_secs(2))
        .expect("connection event");
    match event {
        WorkerEvent::Connection {
            connected: false,
            message,
            show_notice,
        } => {
            assert!(
                !show_notice,
                "startup connection retry must not show a snackbar"
            );
            assert!(!message.contains("os error"), "{message}");
            assert!(!message.contains("No such file"), "{message}");
        }
        other => panic!("expected a connection event, got {other:?}"),
    }
}

fn wait_for_snapshot(
    worker: &mut WorkerProbe,
    predicate: impl Fn(&ServiceSnapshot) -> bool,
) -> ServiceSnapshot {
    let deadline = std::time::Instant::now() + Duration::from_secs(15);
    loop {
        let _ = worker.next_event(deadline);
        if let Some(snapshot) = worker.current_snapshot()
            && predicate(&snapshot)
        {
            return snapshot;
        }
    }
}

fn wait_for_pin_required(worker: &mut WorkerProbe, deadline: std::time::Instant) -> String {
    loop {
        if let WorkerEvent::PinRequired { transfer_id } = worker.next_event(deadline) {
            return transfer_id;
        }
    }
}

fn wait_for_notice(worker: &mut WorkerProbe, deadline: std::time::Instant, needle: &str) -> String {
    loop {
        if let WorkerEvent::Notice(message) = worker.next_event(deadline)
            && message.contains(needle)
        {
            return message;
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn worker_send_receive_and_settings_flow() -> anyhow::Result<()> {
    let mut daemon =
        DaemonFixture::start(&daemon_binary_for_tests()?, default_daemon_settings()).await?;
    let peer = ReceiverPeer::start(ProtocolType::Https, PeerAcceptance::All, None, None).await?;
    let mut worker = WorkerProbe::spawn_with_endpoint(daemon.endpoint().to_string_lossy());
    let initial = wait_for_snapshot(&mut worker, |snapshot| snapshot.server_running);
    assert!(!initial.daemon_instance_id.is_empty());

    worker
        .commands
        .send(UiCommand::AddManual(PeerTarget::format(peer.port())))?;
    let with_peer = wait_for_snapshot(&mut worker, |snapshot| {
        snapshot
            .devices
            .iter()
            .any(|device| device.device.alias == "deterministic-peer")
    });
    let fingerprint = with_peer
        .devices
        .iter()
        .find(|device| device.device.alias == "deterministic-peer")
        .expect("peer device")
        .device
        .fingerprint
        .clone();

    let source = TempDir::new()?;
    let outgoing = source.path().join("worker-out.txt");
    std::fs::write(&outgoing, b"worker outgoing")?;
    worker.commands.send(UiCommand::SendFiles {
        fingerprint,
        paths: vec![outgoing],
    })?;
    peer.wait_for_files(1, Duration::from_secs(15)).await?;

    let sender = tokio::spawn(SenderPeer::send_files(
        "127.0.0.1",
        daemon.snapshot().await?.server_port.expect("daemon port"),
        ProtocolType::Http,
        None,
        vec![PeerFile {
            path: source.path().join("worker-in.txt"),
            remote_name: "worker-in.txt".to_string(),
        }],
    ));
    std::fs::write(source.path().join("worker-in.txt"), b"worker incoming")?;
    let request_snapshot = wait_for_snapshot(&mut worker, |snapshot| {
        !snapshot.incoming_requests.is_empty()
    });
    let request = request_snapshot.incoming_requests[0].clone();
    worker.commands.send(UiCommand::RespondTransfer {
        request_id: request.request_id,
        decision: ReceiverDecision::AcceptAll,
    })?;
    timeout(Duration::from_secs(20), sender)
        .await??
        .map_err(anyhow::Error::from)?;
    let received = daemon.destination().join("worker-in.txt");
    timeout(Duration::from_secs(10), async {
        while !received.exists() {
            sleep(Duration::from_millis(50)).await;
        }
    })
    .await?;
    assert_eq!(std::fs::read(received)?, b"worker incoming");

    let mut settings = daemon.snapshot().await?.settings;
    settings.alias = "worker-persisted".to_string();
    worker.commands.send(UiCommand::UpdateSettings {
        expected_revision: settings.revision,
        settings: Box::new(settings),
    })?;
    wait_for_snapshot(&mut worker, |snapshot| {
        snapshot.settings.alias == "worker-persisted"
    });

    let snapshot = wait_for_snapshot(&mut worker, |snapshot| snapshot.server_running);
    let mut settings = snapshot.settings;
    settings.create_checksums = false;
    settings.verify_checksums = false;
    worker.commands.send(UiCommand::UpdateSettings {
        expected_revision: settings.revision,
        settings: Box::new(settings),
    })?;
    wait_for_snapshot(&mut worker, |snapshot| {
        snapshot.server_running
            && !snapshot.settings.create_checksums
            && !snapshot.settings.verify_checksums
    });

    worker.commands.send(UiCommand::SetAutoSave {
        all: false,
        favorites: true,
    })?;
    wait_for_snapshot(&mut worker, |snapshot| {
        snapshot.settings.auto_save == localsend_rpc::AutoSaveMode::Favorites
            && snapshot.settings.auto_save_from_favorites
    });
    worker.commands.send(UiCommand::SetAutoSave {
        all: true,
        favorites: true,
    })?;
    wait_for_snapshot(&mut worker, |snapshot| {
        snapshot.settings.auto_save == localsend_rpc::AutoSaveMode::On
            && snapshot.settings.auto_save_from_favorites
    });
    worker.commands.send(UiCommand::SetAutoSave {
        all: false,
        favorites: false,
    })?;
    wait_for_snapshot(&mut worker, |snapshot| {
        snapshot.settings.auto_save == localsend_rpc::AutoSaveMode::Off
            && !snapshot.settings.auto_save_from_favorites
    });

    worker.commands.send(UiCommand::ShutdownDaemon)?;
    peer.shutdown().await;
    let _ = daemon.shutdown().await;
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn worker_retries_pin_and_rejects_incoming_request() -> anyhow::Result<()> {
    let mut daemon =
        DaemonFixture::start(&daemon_binary_for_tests()?, default_daemon_settings()).await?;
    let peer = ReceiverPeer::start(
        ProtocolType::Http,
        PeerAcceptance::All,
        Some("2468".to_string()),
        None,
    )
    .await?;
    let mut worker = WorkerProbe::spawn_with_endpoint(daemon.endpoint().to_string_lossy());
    wait_for_snapshot(&mut worker, |snapshot| snapshot.server_running);
    worker
        .commands
        .send(UiCommand::AddManual(format!("127.0.0.1:{}", peer.port())))?;
    let snapshot = wait_for_snapshot(&mut worker, |snapshot| {
        snapshot
            .devices
            .iter()
            .any(|device| device.device.port == peer.port())
    });
    let fingerprint = snapshot
        .devices
        .iter()
        .find(|device| device.device.port == peer.port())
        .expect("pin peer")
        .device
        .fingerprint
        .clone();
    let source = TempDir::new()?;
    let outgoing = source.path().join("pin-retry.txt");
    std::fs::write(&outgoing, b"pin retry")?;
    worker.commands.send(UiCommand::SendFiles {
        fingerprint,
        paths: vec![outgoing],
    })?;
    let transfer_id = wait_for_pin_required(
        &mut worker,
        std::time::Instant::now() + Duration::from_secs(10),
    );
    worker.commands.send(UiCommand::RetryPin {
        transfer_id,
        pin: "2468".to_string(),
    })?;
    peer.wait_for_files(1, Duration::from_secs(15)).await?;

    let incoming = source.path().join("reject.txt");
    std::fs::write(&incoming, b"reject me")?;
    let sender = tokio::spawn(SenderPeer::send_files(
        "127.0.0.1",
        daemon.snapshot().await?.server_port.expect("daemon port"),
        ProtocolType::Http,
        None,
        vec![PeerFile {
            path: incoming.clone(),
            remote_name: "reject.txt".to_string(),
        }],
    ));
    let request_snapshot = wait_for_snapshot(&mut worker, |snapshot| {
        !snapshot.incoming_requests.is_empty()
    });
    worker.commands.send(UiCommand::RespondTransfer {
        request_id: request_snapshot.incoming_requests[0].request_id.clone(),
        decision: ReceiverDecision::Decline,
    })?;
    let report = timeout(Duration::from_secs(20), sender)
        .await??
        .map_err(anyhow::Error::from)?;
    assert!(report.accepted_file_ids.is_empty());
    assert_eq!(report.refused_file_ids.len(), 1);
    assert!(!daemon.destination().join("reject.txt").exists());
    worker.commands.send(UiCommand::ShutdownDaemon)?;
    peer.shutdown().await;
    let _ = daemon.shutdown().await;
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn worker_sends_text_bytes_and_reports_completion() -> anyhow::Result<()> {
    let mut daemon =
        DaemonFixture::start(&daemon_binary_for_tests()?, default_daemon_settings()).await?;
    let peer = ReceiverPeer::start(ProtocolType::Http, PeerAcceptance::All, None, None).await?;
    let mut worker = WorkerProbe::spawn_with_endpoint(daemon.endpoint().to_string_lossy());
    wait_for_snapshot(&mut worker, |snapshot| snapshot.server_running);
    worker
        .commands
        .send(UiCommand::AddManual(PeerTarget::format(peer.port())))?;
    let snapshot = wait_for_snapshot(&mut worker, |snapshot| {
        snapshot
            .devices
            .iter()
            .any(|device| device.device.alias == "deterministic-peer")
    });
    let fingerprint = snapshot
        .devices
        .iter()
        .find(|device| device.device.alias == "deterministic-peer")
        .expect("peer device")
        .device
        .fingerprint
        .clone();

    let text = "text through daemon";
    worker.commands.send(UiCommand::SendText {
        fingerprint,
        text: text.to_string(),
    })?;
    let notice = wait_for_notice(
        &mut worker,
        std::time::Instant::now() + Duration::from_secs(15),
        "文本已发送给",
    );
    assert!(notice.contains("deterministic-peer"), "{notice}");
    let received = peer.wait_for_files(1, Duration::from_secs(5)).await?;
    assert_eq!(received[0].size, text.len() as u64);
    assert_eq!(
        std::fs::read(received[0].path.as_ref().unwrap())?,
        text.as_bytes()
    );

    worker.commands.send(UiCommand::ShutdownDaemon)?;
    peer.shutdown().await;
    let _ = daemon.shutdown().await;
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn worker_sends_clipboard_png_with_matching_bytes() -> anyhow::Result<()> {
    let mut daemon =
        DaemonFixture::start(&daemon_binary_for_tests()?, default_daemon_settings()).await?;
    let peer = ReceiverPeer::start(ProtocolType::Http, PeerAcceptance::All, None, None).await?;
    let mut worker = WorkerProbe::spawn_with_endpoint(daemon.endpoint().to_string_lossy());
    wait_for_snapshot(&mut worker, |snapshot| snapshot.server_running);
    worker
        .commands
        .send(UiCommand::AddManual(PeerTarget::format(peer.port())))?;
    let snapshot = wait_for_snapshot(&mut worker, |snapshot| {
        snapshot
            .devices
            .iter()
            .any(|device| device.device.alias == "deterministic-peer")
    });
    let fingerprint = snapshot
        .devices
        .iter()
        .find(|device| device.device.alias == "deterministic-peer")
        .expect("peer device")
        .device
        .fingerprint
        .clone();

    let source = TempDir::new()?;
    let image = source.path().join("clipboard_2026-10-24_12-34.png");
    let mut png_bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut png_bytes, 2, 1);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&[255, 0, 0, 255, 0, 255, 0, 128])?;
        writer.finish()?;
    }
    std::fs::write(&image, &png_bytes)?;

    worker.commands.send(UiCommand::SendFiles {
        fingerprint,
        paths: vec![image],
    })?;
    let received = peer.wait_for_files(1, Duration::from_secs(15)).await?;
    assert_eq!(received[0].file_name, "clipboard_2026-10-24_12-34.png");
    assert_eq!(
        std::fs::read(received[0].path.as_ref().expect("received path"))?,
        png_bytes
    );
    assert!(
        received[0]
            .sha256
            .as_deref()
            .is_some_and(|hash| hash.len() == 64)
    );

    worker.commands.send(UiCommand::ShutdownDaemon)?;
    peer.shutdown().await;
    let _ = daemon.shutdown().await;
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn worker_starts_web_receive_page() -> anyhow::Result<()> {
    let mut daemon =
        DaemonFixture::start(&daemon_binary_for_tests()?, default_daemon_settings()).await?;
    let mut worker = WorkerProbe::spawn_with_endpoint(daemon.endpoint().to_string_lossy());
    wait_for_snapshot(&mut worker, |snapshot| snapshot.server_running);
    worker.commands.send(UiCommand::StartWebReceive)?;

    let response = timeout(Duration::from_secs(10), async {
        loop {
            let Some(port) = daemon.snapshot().await?.server_port else {
                sleep(Duration::from_millis(50)).await;
                continue;
            };
            let mut stream = match tokio::net::TcpStream::connect(("127.0.0.1", port)).await {
                Ok(stream) => stream,
                Err(_) => {
                    sleep(Duration::from_millis(50)).await;
                    continue;
                }
            };
            if stream
                .write_all(b"GET / HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n")
                .await
                .is_err()
            {
                sleep(Duration::from_millis(50)).await;
                continue;
            }
            let mut response = String::new();
            if stream.read_to_string(&mut response).await.is_err() {
                sleep(Duration::from_millis(50)).await;
                continue;
            }
            if response.contains("200 OK") && response.contains("<html") {
                break Ok::<_, anyhow::Error>(response);
            }
            sleep(Duration::from_millis(50)).await;
        }
    })
    .await??;
    assert!(response.contains("LocalSend") || response.contains("upload"));

    let settings = daemon.snapshot().await?.settings;
    worker.commands.send(UiCommand::UpdateSettings {
        expected_revision: settings.revision,
        settings: Box::new(localsend_rpc::DesktopSettings {
            auto_accept: true,
            ..settings
        }),
    })?;
    worker.commands.send(UiCommand::RestartWebReceive)?;
    wait_for_snapshot(&mut worker, |snapshot| {
        snapshot.settings.auto_accept
            && snapshot
                .devices
                .iter()
                .find(|device| device.source == "local")
                .is_some_and(|device| device.device.download)
    });

    worker.commands.send(UiCommand::StopWebShare)?;
    worker.commands.send(UiCommand::ShutdownDaemon)?;
    let _ = daemon.shutdown().await;
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn worker_surfaces_rejected_outgoing_transfer() -> anyhow::Result<()> {
    let mut daemon =
        DaemonFixture::start(&daemon_binary_for_tests()?, default_daemon_settings()).await?;
    let peer = ReceiverPeer::start(ProtocolType::Http, PeerAcceptance::Reject, None, None).await?;
    let mut worker = WorkerProbe::spawn_with_endpoint(daemon.endpoint().to_string_lossy());
    wait_for_snapshot(&mut worker, |snapshot| snapshot.server_running);
    worker
        .commands
        .send(UiCommand::AddManual(format!("127.0.0.1:{}", peer.port())))?;
    let snapshot = wait_for_snapshot(&mut worker, |snapshot| {
        snapshot
            .devices
            .iter()
            .any(|device| device.device.port == peer.port())
    });
    let fingerprint = snapshot
        .devices
        .iter()
        .find(|device| device.device.port == peer.port())
        .expect("reject peer")
        .device
        .fingerprint
        .clone();
    let source = TempDir::new()?;
    let rejected = source.path().join("rejected.bin");
    std::fs::write(&rejected, vec![7_u8; 4096])?;
    worker.commands.send(UiCommand::SendFiles {
        fingerprint,
        paths: vec![rejected],
    })?;
    let notice = wait_for_notice(
        &mut worker,
        std::time::Instant::now() + Duration::from_secs(15),
        "传输失败",
    );
    assert!(notice.contains("403") || notice.contains("Rejected"));
    let snapshot = wait_for_snapshot(&mut worker, |snapshot| {
        snapshot.active_transfers.iter().any(|transfer| {
            transfer.direction == localsend_rpc::TransferDirection::Send
                && transfer.status == localsend_rpc::TransferStatus::Failed
        })
    });
    let failed = snapshot
        .active_transfers
        .iter()
        .find(|transfer| transfer.status == localsend_rpc::TransferStatus::Failed)
        .expect("failed transfer");
    assert!(failed.error.is_some());
    let peer_status = peer.status().await;
    assert_eq!(peer_status.requests.len(), 1);
    assert!(peer_status.requests[0].accepted_file_ids.is_empty());
    assert!(peer_status.received_files.is_empty());

    worker.commands.send(UiCommand::ShutdownDaemon)?;
    peer.shutdown().await;
    let _ = daemon.shutdown().await;
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn worker_cancels_slow_outgoing_transfer() -> anyhow::Result<()> {
    let mut daemon =
        DaemonFixture::start(&daemon_binary_for_tests()?, default_daemon_settings()).await?;
    let peer = ReceiverPeer::start(
        ProtocolType::Http,
        PeerAcceptance::All,
        None,
        Some(64 * 1024),
    )
    .await?;
    let mut worker = WorkerProbe::spawn_with_endpoint(daemon.endpoint().to_string_lossy());
    wait_for_snapshot(&mut worker, |snapshot| snapshot.server_running);
    worker
        .commands
        .send(UiCommand::AddManual(format!("127.0.0.1:{}", peer.port())))?;
    let snapshot = wait_for_snapshot(&mut worker, |snapshot| {
        snapshot
            .devices
            .iter()
            .any(|device| device.device.port == peer.port())
    });
    let fingerprint = snapshot
        .devices
        .iter()
        .find(|device| device.device.port == peer.port())
        .expect("slow peer")
        .device
        .fingerprint
        .clone();
    let source = TempDir::new()?;
    let large = source.path().join("slow.bin");
    std::fs::write(&large, vec![9_u8; 16 * 1024 * 1024])?;
    worker.commands.send(UiCommand::SendFiles {
        fingerprint,
        paths: vec![large],
    })?;
    let transferring = wait_for_snapshot(&mut worker, |snapshot| {
        snapshot.active_transfers.iter().any(|transfer| {
            transfer.direction == localsend_rpc::TransferDirection::Send
                && matches!(
                    transfer.status,
                    localsend_rpc::TransferStatus::Waiting
                        | localsend_rpc::TransferStatus::Transferring
                )
        })
    });
    let transfer_id = transferring
        .active_transfers
        .iter()
        .find(|transfer| transfer.direction == localsend_rpc::TransferDirection::Send)
        .expect("send transfer")
        .transfer_id
        .clone();
    worker
        .commands
        .send(UiCommand::CancelTransfer(transfer_id))?;
    wait_for_snapshot(&mut worker, |snapshot| {
        snapshot.active_transfers.iter().any(|transfer| {
            transfer.direction == localsend_rpc::TransferDirection::Send
                && transfer.status == localsend_rpc::TransferStatus::Cancelled
        })
    });
    worker.commands.send(UiCommand::ShutdownDaemon)?;
    peer.shutdown().await;
    let _ = daemon.shutdown().await;
    Ok(())
}

struct PeerTarget;

impl PeerTarget {
    fn format(port: u16) -> String {
        format!("127.0.0.1:{port}")
    }
}

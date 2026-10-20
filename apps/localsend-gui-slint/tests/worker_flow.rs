#![cfg(unix)]

use localsend_gui_slint::daemon_worker::{DaemonWorker, UiCommand, WorkerEvent};
use localsend_rpc::{ReceiverDecision, ServiceSnapshot};
use localsend_testkit::{
    DaemonFixture, PeerAcceptance, PeerFile, ProtocolType, ReceiverPeer, SenderPeer,
    daemon_binary_for_tests, default_daemon_settings,
};
use std::time::Duration;
use tempfile::TempDir;
use tokio::time::{sleep, timeout};

fn next_snapshot(worker: &DaemonWorker, deadline: std::time::Instant) -> ServiceSnapshot {
    loop {
        let remaining = deadline.saturating_duration_since(std::time::Instant::now());
        assert!(
            !remaining.is_zero(),
            "timed out waiting for worker snapshot"
        );
        match worker.events.recv_timeout(remaining).expect("worker event") {
            WorkerEvent::Snapshot(snapshot) => return *snapshot,
            WorkerEvent::Connection {
                connected: false,
                message,
            } => panic!("worker disconnected: {message}"),
            _ => {}
        }
    }
}

fn wait_for_snapshot(
    worker: &DaemonWorker,
    predicate: impl Fn(&ServiceSnapshot) -> bool,
) -> ServiceSnapshot {
    let deadline = std::time::Instant::now() + Duration::from_secs(15);
    loop {
        let snapshot = next_snapshot(worker, deadline);
        if predicate(&snapshot) {
            return snapshot;
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn worker_send_receive_and_settings_flow() -> anyhow::Result<()> {
    let mut daemon =
        DaemonFixture::start(&daemon_binary_for_tests()?, default_daemon_settings()).await?;
    let peer = ReceiverPeer::start(ProtocolType::Http, PeerAcceptance::All, None, None).await?;
    let worker = DaemonWorker::spawn_with_endpoint(daemon.endpoint().to_string_lossy());
    let initial = wait_for_snapshot(&worker, |snapshot| snapshot.server_running);
    assert!(!initial.daemon_instance_id.is_empty());

    worker
        .commands
        .send(UiCommand::AddManual(PeerTarget::format(peer.port())))?;
    let with_peer = wait_for_snapshot(&worker, |snapshot| {
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
    let request_snapshot =
        wait_for_snapshot(&worker, |snapshot| !snapshot.incoming_requests.is_empty());
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
    wait_for_snapshot(&worker, |snapshot| {
        snapshot.settings.alias == "worker-persisted"
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

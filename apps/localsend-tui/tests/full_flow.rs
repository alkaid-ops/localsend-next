use localsend_rpc::{AutoSaveMode, DesktopSettings, Protocol, ServiceCommand};
use localsend_testkit::{
    AcceptanceArtifacts, DaemonFixture, PeerAcceptance, PeerFile, ReceiverPeer, SenderPeer,
    daemon_binary_for_tests, default_daemon_settings,
};
use portable_pty::{Child, CommandBuilder, MasterPty, PtySize, native_pty_system};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::mpsc as std_mpsc;
use std::time::{Duration, Instant};
use tempfile::TempDir;
use tokio::time::{sleep, timeout};

struct TuiPty {
    writer: Box<dyn Write + Send>,
    child: Box<dyn Child + Send + Sync>,
    _master: Box<dyn MasterPty + Send>,
    parser: vt100::Parser,
    output: std_mpsc::Receiver<Vec<u8>>,
}

impl TuiPty {
    fn spawn(
        endpoint: &std::path::Path,
        config: &std::path::Path,
        rows: u16,
        cols: u16,
    ) -> anyhow::Result<Self> {
        let binary = tui_binary_for_tests()?;
        let pty_system = native_pty_system();
        let pair = pty_system.openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;
        let mut command = CommandBuilder::new(binary);
        command.env("TERM", "xterm-256color");
        command.env("LOCALSEND_RPC_SOCKET", endpoint);
        command.env("LOCALSEND_CONFIG_DIR", config);
        command.env("RUST_LOG", "warn");
        let child = pair.slave.spawn_command(command)?;
        drop(pair.slave);
        let mut reader = pair.master.try_clone_reader()?;
        let writer = pair.master.take_writer()?;
        let (tx, rx) = std_mpsc::channel();
        std::thread::spawn(move || {
            let mut buffer = [0_u8; 8192];
            while let Ok(read) = reader.read(&mut buffer) {
                if read == 0 || tx.send(buffer[..read].to_vec()).is_err() {
                    break;
                }
            }
        });
        Ok(Self {
            writer,
            child,
            _master: pair.master,
            parser: vt100::Parser::new(rows, cols, 0),
            output: rx,
        })
    }

    fn send(&mut self, text: &str) -> anyhow::Result<()> {
        self.writer.write_all(text.as_bytes())?;
        self.writer.flush()?;
        Ok(())
    }

    fn wait_for_text(&mut self, needle: &str, timeout: Duration) -> anyhow::Result<String> {
        let deadline = Instant::now() + timeout;
        loop {
            while let Ok(bytes) = self.output.try_recv() {
                self.parser.process(&bytes);
            }
            let screen = self.parser.screen().contents();
            if screen.contains(needle) {
                return Ok(screen);
            }
            anyhow::ensure!(
                Instant::now() < deadline,
                "timed out waiting for {needle:?}:\n{screen}"
            );
            std::thread::sleep(Duration::from_millis(25));
        }
    }

    fn wait_exit(&mut self, timeout: Duration) -> anyhow::Result<()> {
        let deadline = Instant::now() + timeout;
        loop {
            if self.child.try_wait()?.is_some() {
                return Ok(());
            }
            anyhow::ensure!(Instant::now() < deadline, "TUI did not exit");
            std::thread::sleep(Duration::from_millis(25));
        }
    }
}

impl Drop for TuiPty {
    fn drop(&mut self) {
        if self.child.try_wait().ok().flatten().is_none() {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

fn tui_binary_for_tests() -> anyhow::Result<PathBuf> {
    if let Some(path) = std::env::var_os("CARGO_BIN_EXE_localsend-tui") {
        return Ok(path.into());
    }
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let name = if cfg!(windows) {
        "localsend-tui.exe"
    } else {
        "localsend-tui"
    };
    let path = root.join("target/debug").join(name);
    anyhow::ensure!(
        path.is_file(),
        "localsend-tui is not built; run `cargo build -p localsend-tui` first"
    );
    Ok(path)
}

async fn fixture(settings: DesktopSettings) -> anyhow::Result<DaemonFixture> {
    DaemonFixture::start(&daemon_binary_for_tests()?, settings).await
}

#[tokio::test(flavor = "multi_thread")]
async fn tui_pages_lifecycle_and_daemon_survival() -> anyhow::Result<()> {
    let mut daemon = fixture(default_daemon_settings()).await?;
    let mut tui = TuiPty::spawn(daemon.endpoint(), daemon.config_dir(), 24, 80)?;
    tui.wait_for_text("Incoming Request", Duration::from_secs(10))?;
    tui.send("2")?;
    tui.wait_for_text("Nearby Devices", Duration::from_secs(5))?;
    tui.send("3")?;
    tui.wait_for_text("Settings", Duration::from_secs(5))?;
    tui.send("q")?;
    tui.wait_exit(Duration::from_secs(5))?;
    let snapshot = daemon.snapshot().await?;
    assert!(snapshot.server_running);
    daemon.shutdown().await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn tui_reconnects_after_daemon_restart() -> anyhow::Result<()> {
    let mut daemon = fixture(default_daemon_settings()).await?;
    let mut tui = TuiPty::spawn(daemon.endpoint(), daemon.config_dir(), 24, 80)?;
    tui.wait_for_text("Incoming Request", Duration::from_secs(10))?;
    daemon.kill()?;
    tui.wait_for_text("reconnecting", Duration::from_secs(10))?;
    daemon.restart().await?;
    tui.wait_for_text("Reconnected to localsendd", Duration::from_secs(10))?;
    tui.send("q")?;
    tui.wait_exit(Duration::from_secs(5))?;
    daemon.shutdown().await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn tui_receive_accepts_all_and_records_history() -> anyhow::Result<()> {
    let artifacts = AcceptanceArtifacts::create("tui")?;
    let mut daemon = fixture(default_daemon_settings()).await?;
    let mut tui = TuiPty::spawn(daemon.endpoint(), daemon.config_dir(), 40, 120)?;
    tui.wait_for_text("Incoming Request", Duration::from_secs(10))?;
    let source = TempDir::new()?;
    let first = source.path().join("alpha.txt");
    let second = source.path().join("beta.bin");
    std::fs::write(&first, b"alpha")?;
    std::fs::write(&second, b"beta")?;
    let port = daemon.snapshot().await?.server_port.expect("server port");
    let sender = tokio::spawn(SenderPeer::send_files(
        "127.0.0.1",
        port,
        localsend_core::model::discovery::ProtocolType::Http,
        None,
        vec![
            PeerFile {
                path: first.clone(),
                remote_name: "alpha.txt".to_string(),
            },
            PeerFile {
                path: second.clone(),
                remote_name: "beta.bin".to_string(),
            },
        ],
    ));
    tui.wait_for_text("alpha.txt", Duration::from_secs(10))?;
    tui.wait_for_text("beta.bin", Duration::from_secs(10))?;
    tui.send("Y")?;
    let report = timeout(Duration::from_secs(20), sender)
        .await??
        .map_err(anyhow::Error::from)?;
    assert_eq!(report.accepted_file_ids.len(), 2);
    tui.wait_for_text("Recent History", Duration::from_secs(10))?;
    let screen = tui.parser.screen().contents();
    artifacts.write_text("receive-screen.txt", &screen)?;
    let received = daemon.destination();
    assert_eq!(std::fs::read(received.join("alpha.txt"))?, b"alpha");
    assert_eq!(std::fs::read(received.join("beta.bin"))?, b"beta");
    let snapshot = daemon.snapshot().await?;
    artifacts.write_json("receive-snapshot.json", &snapshot)?;
    assert_eq!(snapshot.history.len(), 2);
    tui.send("q")?;
    tui.wait_exit(Duration::from_secs(5))?;
    daemon.shutdown().await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn tui_receive_supports_partial_accept_and_reject() -> anyhow::Result<()> {
    let mut daemon = fixture(default_daemon_settings()).await?;
    let mut tui = TuiPty::spawn(daemon.endpoint(), daemon.config_dir(), 40, 120)?;
    tui.wait_for_text("Incoming Request", Duration::from_secs(10))?;
    let source = TempDir::new()?;
    let first = source.path().join("one.txt");
    let second = source.path().join("two.txt");
    std::fs::write(&first, b"one")?;
    std::fs::write(&second, b"two")?;
    let port = daemon.snapshot().await?.server_port.expect("server port");
    let sender = tokio::spawn(SenderPeer::send_files(
        "127.0.0.1",
        port,
        localsend_testkit::ProtocolType::Http,
        None,
        vec![
            PeerFile {
                path: first.clone(),
                remote_name: "one.txt".to_string(),
            },
            PeerFile {
                path: second.clone(),
                remote_name: "two.txt".to_string(),
            },
        ],
    ));
    tui.wait_for_text("one.txt", Duration::from_secs(10))?;
    tui.wait_for_text("two.txt", Duration::from_secs(10))?;
    tui.send(" ")?;
    tui.send("y")?;
    let partial = timeout(Duration::from_secs(20), sender)
        .await??
        .map_err(anyhow::Error::from)?;
    assert_eq!(partial.accepted_file_ids.len(), 1);
    assert_eq!(partial.refused_file_ids.len(), 1);
    let saved_after_partial = std::fs::read_dir(daemon.destination())?.count();
    assert_eq!(saved_after_partial, 1);

    let partial_session = partial.session_id.clone().expect("partial session");
    let cancel_result = timeout(Duration::from_secs(10), async {
        loop {
            let snapshot = daemon.snapshot().await?;
            if snapshot.incoming_requests.is_empty()
                && snapshot.active_transfers.iter().any(|transfer| {
                    transfer.transfer_id == partial_session
                        && transfer.status == localsend_rpc::TransferStatus::Finished
                })
            {
                break Ok::<_, anyhow::Error>(());
            }
            sleep(Duration::from_millis(50)).await;
        }
    })
    .await;
    if cancel_result.is_err() {
        let snapshot = daemon.snapshot().await?;
        anyhow::bail!("cancel status did not arrive: {snapshot:?}");
    }
    cancel_result??;
    let reject_sender = tokio::spawn(SenderPeer::send_files(
        "127.0.0.1",
        port,
        localsend_testkit::ProtocolType::Http,
        None,
        vec![
            PeerFile {
                path: first,
                remote_name: "one.txt".to_string(),
            },
            PeerFile {
                path: second,
                remote_name: "two.txt".to_string(),
            },
        ],
    ));
    timeout(Duration::from_secs(10), async {
        loop {
            if !daemon.snapshot().await?.incoming_requests.is_empty() {
                break Ok::<_, anyhow::Error>(());
            }
            sleep(Duration::from_millis(50)).await;
        }
    })
    .await??;
    sleep(Duration::from_millis(500)).await;
    tui.send("n")?;
    let rejected = timeout(Duration::from_secs(20), reject_sender)
        .await??
        .map_err(anyhow::Error::from)?;
    assert!(rejected.accepted_file_ids.is_empty());
    assert_eq!(rejected.refused_file_ids.len(), 2);
    assert_eq!(std::fs::read_dir(daemon.destination())?.count(), 1);
    tui.send("q")?;
    tui.wait_exit(Duration::from_secs(5))?;
    daemon.shutdown().await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn tui_manual_ip_send_file_and_text() -> anyhow::Result<()> {
    let artifacts = AcceptanceArtifacts::create("tui")?;
    let peer = ReceiverPeer::start(
        localsend_core::model::discovery::ProtocolType::Http,
        PeerAcceptance::All,
        None,
        None,
    )
    .await?;
    let mut daemon = fixture(default_daemon_settings()).await?;
    let rpc = daemon.connect_interactive().await?;
    rpc.command(ServiceCommand::AddManualDevice {
        target: format!("127.0.0.1:{}", peer.port()),
    })
    .await
    .map_err(anyhow::Error::msg)?;
    let mut tui = TuiPty::spawn(daemon.endpoint(), daemon.config_dir(), 40, 120)?;
    tui.wait_for_text("Incoming Request", Duration::from_secs(10))?;
    tui.send("2")?;
    tui.wait_for_text("deterministic-peer", Duration::from_secs(10))?;
    tui.send("F")?;
    timeout(Duration::from_secs(10), async {
        loop {
            if !daemon.snapshot().await?.favorites.is_empty() {
                break Ok::<_, anyhow::Error>(());
            }
            sleep(Duration::from_millis(50)).await;
        }
    })
    .await??;
    tui.send("e")?;
    tui.wait_for_text("Favorite alias", Duration::from_secs(5))?;
    tui.send("Peer Favorite\r")?;
    timeout(Duration::from_secs(10), async {
        loop {
            if daemon
                .snapshot()
                .await?
                .favorites
                .iter()
                .any(|favorite| favorite.alias == "Peer Favorite")
            {
                break Ok::<_, anyhow::Error>(());
            }
            sleep(Duration::from_millis(50)).await;
        }
    })
    .await??;
    let source = TempDir::new()?;
    let path = source.path().join("manual.txt");
    std::fs::write(&path, b"manual-ip")?;
    tui.send("\u{c}")?;
    tui.wait_for_text("File or directory path", Duration::from_secs(5))?;
    tui.send(&format!("{}\r", path.display()))?;
    let request = peer.wait_for_request(Duration::from_secs(10)).await?;
    assert!(request.file_names.iter().any(|name| name == "manual.txt"));
    peer.wait_for_files(1, Duration::from_secs(15)).await?;
    tui.send("t")?;
    tui.wait_for_text("Text message (Enter submits)", Duration::from_secs(5))?;
    tui.send("hello from tui\r")?;
    timeout(Duration::from_secs(10), async {
        loop {
            let status = peer.status().await;
            if status.requests.len() >= 2 {
                break;
            }
            sleep(Duration::from_millis(50)).await;
        }
    })
    .await?;
    let status = peer.status().await;
    artifacts.write_json("send-peer-status.json", &status)?;
    assert!(
        status
            .requests
            .iter()
            .any(|request| request.file_names.iter().any(|name| name == "message.txt"))
    );
    tui.send("q")?;
    tui.wait_exit(Duration::from_secs(5))?;
    peer.shutdown().await;
    daemon.shutdown().await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn tui_sends_directory_preserving_relative_names() -> anyhow::Result<()> {
    let peer = ReceiverPeer::start(
        localsend_testkit::ProtocolType::Http,
        PeerAcceptance::All,
        None,
        None,
    )
    .await?;
    let mut daemon = fixture(default_daemon_settings()).await?;
    daemon
        .connect_interactive()
        .await?
        .command(ServiceCommand::AddManualDevice {
            target: format!("127.0.0.1:{}", peer.port()),
        })
        .await
        .map_err(anyhow::Error::msg)?;
    let mut tui = TuiPty::spawn(daemon.endpoint(), daemon.config_dir(), 40, 120)?;
    tui.wait_for_text("Incoming Request", Duration::from_secs(10))?;
    tui.send("2")?;
    tui.wait_for_text("deterministic-peer", Duration::from_secs(10))?;
    let source = TempDir::new()?;
    let folder = source.path().join("bundle");
    std::fs::create_dir_all(folder.join("nested"))?;
    std::fs::write(folder.join("root.txt"), b"root")?;
    std::fs::write(folder.join("nested/child.txt"), b"child")?;
    tui.send("\u{c}")?;
    tui.wait_for_text("File or directory path", Duration::from_secs(5))?;
    tui.send(&format!("{}\r", folder.display()))?;
    let request = peer.wait_for_request(Duration::from_secs(10)).await?;
    assert!(
        request
            .file_names
            .iter()
            .any(|name| name == "bundle/root.txt" || name == "bundle\\root.txt")
    );
    assert!(
        request
            .file_names
            .iter()
            .any(|name| name == "bundle/nested/child.txt" || name == "bundle\\nested\\child.txt")
    );
    peer.wait_for_files(2, Duration::from_secs(15)).await?;
    tui.send("q")?;
    tui.wait_exit(Duration::from_secs(5))?;
    peer.shutdown().await;
    daemon.shutdown().await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn tui_cancels_slow_outgoing_transfer() -> anyhow::Result<()> {
    let peer = ReceiverPeer::start(
        localsend_testkit::ProtocolType::Http,
        PeerAcceptance::All,
        None,
        Some(64 * 1024),
    )
    .await?;
    let mut daemon = fixture(default_daemon_settings()).await?;
    daemon
        .connect_interactive()
        .await?
        .command(ServiceCommand::AddManualDevice {
            target: format!("127.0.0.1:{}", peer.port()),
        })
        .await
        .map_err(anyhow::Error::msg)?;
    let mut tui = TuiPty::spawn(daemon.endpoint(), daemon.config_dir(), 40, 120)?;
    tui.wait_for_text("Incoming Request", Duration::from_secs(10))?;
    tui.send("2")?;
    tui.wait_for_text("deterministic-peer", Duration::from_secs(10))?;
    let source = TempDir::new()?;
    let large = source.path().join("slow.bin");
    std::fs::write(&large, vec![7_u8; 16 * 1024 * 1024])?;
    tui.send("\u{c}")?;
    tui.wait_for_text("File or directory path", Duration::from_secs(5))?;
    tui.send(&format!("{}\r", large.display()))?;
    timeout(Duration::from_secs(15), async {
        loop {
            let snapshot = daemon.snapshot().await?;
            if snapshot.active_transfers.iter().any(|transfer| {
                transfer.direction == localsend_rpc::TransferDirection::Send
                    && matches!(
                        transfer.status,
                        localsend_rpc::TransferStatus::Waiting
                            | localsend_rpc::TransferStatus::Transferring
                    )
            }) {
                break Ok::<_, anyhow::Error>(());
            }
            sleep(Duration::from_millis(50)).await;
        }
    })
    .await??;
    sleep(Duration::from_millis(300)).await;
    tui.send("x")?;
    let cancel_result = timeout(Duration::from_secs(10), async {
        loop {
            let snapshot = daemon.snapshot().await?;
            if snapshot.active_transfers.iter().any(|transfer| {
                transfer.direction == localsend_rpc::TransferDirection::Send
                    && transfer.status == localsend_rpc::TransferStatus::Cancelled
            }) {
                break Ok::<_, anyhow::Error>(());
            }
            sleep(Duration::from_millis(50)).await;
        }
    })
    .await;
    if cancel_result.is_err() {
        anyhow::bail!(
            "cancel status did not arrive: {:?}",
            daemon.snapshot().await?
        );
    }
    cancel_result??;
    tui.send("q")?;
    tui.wait_exit(Duration::from_secs(5))?;
    peer.shutdown().await;
    daemon.shutdown().await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn tui_pin_retry_and_settings_persist() -> anyhow::Result<()> {
    let peer = ReceiverPeer::start(
        localsend_core::model::discovery::ProtocolType::Http,
        PeerAcceptance::All,
        Some("2468".to_string()),
        None,
    )
    .await?;
    let mut settings = default_daemon_settings();
    settings.auto_save = AutoSaveMode::Off;
    let mut daemon = fixture(settings).await?;
    let rpc = daemon.connect_interactive().await?;
    rpc.command(ServiceCommand::AddManualDevice {
        target: format!("127.0.0.1:{}", peer.port()),
    })
    .await
    .map_err(anyhow::Error::msg)?;
    let mut tui = TuiPty::spawn(daemon.endpoint(), daemon.config_dir(), 40, 120)?;
    tui.wait_for_text("Incoming Request", Duration::from_secs(10))?;
    tui.send("2")?;
    tui.wait_for_text("deterministic-peer", Duration::from_secs(10))?;
    let source = TempDir::new()?;
    let path = source.path().join("pin.txt");
    std::fs::write(&path, b"pin")?;
    tui.send("\u{c}")?;
    tui.send(&format!("{}\r", path.display()))?;
    tui.wait_for_text("Receiver requires a PIN", Duration::from_secs(10))?;
    tui.send("2468\r")?;
    peer.wait_for_files(1, Duration::from_secs(15)).await?;
    tui.send("3")?;
    tui.wait_for_text("Settings", Duration::from_secs(5))?;
    tui.send("\r")?;
    tui.wait_for_text("Alias", Duration::from_secs(5))?;
    tui.send("TUI Persisted\r")?;
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if daemon.snapshot().await?.settings.alias == "TUI Persisted" {
            break;
        }
        anyhow::ensure!(Instant::now() < deadline, "settings did not persist");
        sleep(Duration::from_millis(50)).await;
    }
    tui.send("h")?;
    tui.send("a")?;
    timeout(Duration::from_secs(10), async {
        loop {
            let snapshot = daemon.snapshot().await?;
            if snapshot.settings.protocol == Protocol::Https
                && snapshot.settings.auto_save != AutoSaveMode::Off
            {
                break Ok::<_, anyhow::Error>(());
            }
            sleep(Duration::from_millis(50)).await;
        }
    })
    .await??;
    tui.send("q")?;
    tui.wait_exit(Duration::from_secs(5))?;
    peer.shutdown().await;
    daemon.shutdown().await?;
    Ok(())
}

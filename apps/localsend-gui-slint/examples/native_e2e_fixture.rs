use anyhow::Context;
use localsend_rpc::{AutoSaveMode, DaemonClient, DesktopSettings, HistoryEntry, ServiceCommand};
use localsend_testkit::{
    DaemonFixture, PeerAcceptance, PeerFile, ProtocolType, ReceiverPeer, SenderPeer,
    daemon_binary_for_tests, default_daemon_settings,
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Serialize)]
struct FixtureManifest {
    endpoint: String,
    config_dir: PathBuf,
    status_path: PathBuf,
    peer_alias: String,
    peer_fingerprint: String,
    peer_port: u16,
    daemon_pid: u32,
    daemon_port: u16,
    destination: PathBuf,
}

#[derive(Serialize)]
struct FixtureStatus {
    peer: localsend_testkit::PeerStatus,
    settings: DesktopSettings,
    history: Vec<HistoryEntry>,
    incoming_results: Vec<IncomingResult>,
}

#[derive(Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum IncomingAction {
    SendFiles {
        paths: Vec<PathBuf>,
        #[serde(default)]
        pin: Option<String>,
    },
    SendText {
        text: String,
        #[serde(default)]
        pin: Option<String>,
    },
}

#[derive(Clone, Serialize)]
struct IncomingResult {
    kind: String,
    ok: bool,
    session_id: Option<String>,
    accepted_file_ids: Vec<String>,
    refused_file_ids: Vec<String>,
    error: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let root = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .context("usage: native_e2e_fixture ROOT")?;
    std::fs::create_dir_all(&root)?;
    let stop_path = root.join("stop");
    let ready_path = root.join("ready.json");
    let status_path = root.join("status.json");
    let actions_dir = root.join("actions");
    let processed_dir = root.join("processed");
    std::fs::create_dir_all(&actions_dir)?;
    std::fs::create_dir_all(&processed_dir)?;

    let peer = ReceiverPeer::start(ProtocolType::Http, PeerAcceptance::All, None, None).await?;
    let mut settings = default_daemon_settings();
    settings.port = free_tcp_port()?;
    settings.auto_save = AutoSaveMode::Off;
    settings.auto_finish = std::env::var_os("LOCALSEND_E2E_AUTO_FINISH").is_some();
    let mut daemon = DaemonFixture::start(&daemon_binary_for_tests()?, settings).await?;
    let client = DaemonClient::connect_to_interactive(daemon.endpoint().to_string_lossy()).await?;
    client
        .command(ServiceCommand::AddManualDevice {
            target: format!("127.0.0.1:{}", peer.port()),
        })
        .await
        .map_err(anyhow::Error::msg)?;

    wait_for_peer(&daemon, peer.fingerprint()).await?;
    let initial_snapshot = daemon.snapshot().await?;
    let daemon_port = initial_snapshot.server_port.context("daemon server port")?;
    let manifest = FixtureManifest {
        endpoint: daemon.endpoint().to_string_lossy().into_owned(),
        config_dir: daemon.config_dir().to_path_buf(),
        status_path: status_path.clone(),
        peer_alias: "deterministic-peer".to_string(),
        peer_fingerprint: peer.fingerprint().to_string(),
        peer_port: peer.port(),
        daemon_pid: initial_snapshot.daemon_pid,
        daemon_port,
        destination: daemon.destination().to_path_buf(),
    };
    write_json(&ready_path, &manifest)?;

    let mut incoming_results = Vec::new();
    while !stop_path.exists() {
        let mut actions = std::fs::read_dir(&actions_dir)?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                path.extension()
                    .is_some_and(|extension| extension == "json")
            })
            .collect::<Vec<_>>();
        actions.sort();
        for action_path in actions {
            let action = match std::fs::read(&action_path)
                .map_err(anyhow::Error::from)
                .and_then(|bytes| {
                    serde_json::from_slice::<IncomingAction>(&bytes).map_err(Into::into)
                }) {
                Ok(action) => action,
                Err(error) => {
                    eprintln!(
                        "invalid incoming action {}: {error:#}",
                        action_path.display()
                    );
                    continue;
                }
            };
            let mut result = None;
            for _ in 0..10 {
                let current = run_incoming_action(action.clone(), manifest.daemon_port).await;
                let retry = !current.ok
                    && current
                        .error
                        .as_deref()
                        .is_some_and(|error| error.contains("error sending request"));
                result = Some(current);
                if !retry {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(250)).await;
            }
            let Some(result) = result else {
                eprintln!("incoming action did not run: {}", action_path.display());
                continue;
            };
            incoming_results.push(result);
            let processed_path =
                processed_dir.join(action_path.file_name().context("action file name")?);
            std::fs::rename(action_path, processed_path)?;
        }
        let status = FixtureStatus {
            peer: peer.status().await,
            settings: read_settings(daemon.config_dir())?,
            history: read_history(daemon.config_dir())?,
            incoming_results: incoming_results.clone(),
        };
        write_json(&status_path, &status)?;
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    let _ = client.command(ServiceCommand::Shutdown).await;
    let _ = daemon.shutdown().await;
    peer.shutdown().await;
    Ok(())
}

async fn run_incoming_action(action: IncomingAction, daemon_port: u16) -> IncomingResult {
    let (kind, result) = match action {
        IncomingAction::SendFiles { paths, pin } => (
            "send_files".to_string(),
            SenderPeer::send_files(
                "127.0.0.1",
                daemon_port,
                ProtocolType::Http,
                pin.as_deref(),
                paths
                    .into_iter()
                    .map(|path| PeerFile {
                        remote_name: path
                            .file_name()
                            .map(|name| name.to_string_lossy().into_owned())
                            .unwrap_or_else(|| "file.bin".to_string()),
                        path,
                    })
                    .collect(),
            )
            .await,
        ),
        IncomingAction::SendText { text, pin } => (
            "send_text".to_string(),
            SenderPeer::send_text(
                "127.0.0.1",
                daemon_port,
                ProtocolType::Http,
                pin.as_deref(),
                text,
            )
            .await,
        ),
    };
    match result {
        Ok(report) => IncomingResult {
            kind,
            ok: true,
            session_id: report.session_id,
            accepted_file_ids: report.accepted_file_ids,
            refused_file_ids: report.refused_file_ids,
            error: None,
        },
        Err(error) => IncomingResult {
            kind,
            ok: false,
            session_id: None,
            accepted_file_ids: Vec::new(),
            refused_file_ids: Vec::new(),
            error: Some(error.to_string()),
        },
    }
}

async fn wait_for_peer(daemon: &DaemonFixture, fingerprint: &str) -> anyhow::Result<()> {
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let snapshot = daemon.snapshot().await.expect("daemon snapshot");
            if snapshot
                .devices
                .iter()
                .any(|device| device.device.fingerprint == fingerprint)
            {
                return;
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    })
    .await
    .context("deterministic peer was not added")
}

fn write_json(path: &Path, value: &impl Serialize) -> anyhow::Result<()> {
    let bytes = serde_json::to_vec_pretty(value)?;
    let temporary = path.with_extension("tmp");
    std::fs::write(&temporary, bytes)?;
    std::fs::rename(temporary, path)?;
    Ok(())
}

fn free_tcp_port() -> anyhow::Result<u16> {
    let listener = std::net::TcpListener::bind(("127.0.0.1", 0))?;
    Ok(listener.local_addr()?.port())
}

fn read_settings(config_dir: &Path) -> anyhow::Result<DesktopSettings> {
    Ok(serde_json::from_slice(&std::fs::read(
        config_dir.join("settings.json"),
    )?)?)
}

fn read_history(config_dir: &Path) -> anyhow::Result<Vec<HistoryEntry>> {
    let Ok(contents) = std::fs::read_to_string(config_dir.join("history.jsonl")) else {
        return Ok(Vec::new());
    };
    Ok(contents
        .lines()
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect())
}

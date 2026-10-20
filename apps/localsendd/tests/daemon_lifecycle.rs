#![cfg(unix)]

use localsend_rpc::{DaemonClient, DaemonLifecycle, DesktopSettings, Protocol, ServiceCommand};
use std::ffi::OsString;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use tempfile::TempDir;

struct EnvRestore {
    socket: Option<OsString>,
    config: Option<OsString>,
}

impl EnvRestore {
    fn set(socket: &Path, config: &Path) -> Self {
        let previous = Self {
            socket: std::env::var_os("LOCALSEND_RPC_SOCKET"),
            config: std::env::var_os("LOCALSEND_CONFIG_DIR"),
        };
        unsafe {
            std::env::set_var("LOCALSEND_RPC_SOCKET", socket);
            std::env::set_var("LOCALSEND_CONFIG_DIR", config);
        }
        previous
    }
}

impl Drop for EnvRestore {
    fn drop(&mut self) {
        unsafe {
            if let Some(value) = self.socket.take() {
                std::env::set_var("LOCALSEND_RPC_SOCKET", value);
            } else {
                std::env::remove_var("LOCALSEND_RPC_SOCKET");
            }
            if let Some(value) = self.config.take() {
                std::env::set_var("LOCALSEND_CONFIG_DIR", value);
            } else {
                std::env::remove_var("LOCALSEND_CONFIG_DIR");
            }
        }
    }
}

fn spawn_daemon(require_owner: bool) -> Child {
    let mut command = Command::new(env!("CARGO_BIN_EXE_localsendd"));
    if require_owner {
        command.arg("--require-owner");
    }
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn localsendd")
}

async fn connect_owned() -> DaemonClient {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(8);
    loop {
        match DaemonClient::connect_owned().await {
            Ok(client) => return client,
            Err(error) if tokio::time::Instant::now() < deadline => {
                let _ = error;
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
            Err(error) => panic!("daemon did not accept owner: {error}"),
        }
    }
}

async fn connect_normal() -> DaemonClient {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(8);
    loop {
        match DaemonClient::connect().await {
            Ok(client) => return client,
            Err(error) if tokio::time::Instant::now() < deadline => {
                let _ = error;
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
            Err(error) => panic!("daemon did not become ready: {error}"),
        }
    }
}

async fn wait_for_exit(child: &mut Child, timeout: Duration) -> std::process::ExitStatus {
    let deadline = tokio::time::Instant::now() + timeout;
    loop {
        if let Some(status) = child.try_wait().expect("poll daemon") {
            return status;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "daemon did not exit in time"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn daemon_lock_owner_lease_manifest_and_persistent_mode() {
    let root = TempDir::new().expect("temp root");
    let config = root.path().join("config");
    std::fs::create_dir_all(&config).expect("create config");
    let settings = DesktopSettings {
        port: 0,
        protocol: Protocol::Http,
        start_server_on_launch: false,
        ..DesktopSettings::default()
    };
    std::fs::write(
        config.join("settings.json"),
        serde_json::to_vec(&settings).expect("serialize settings"),
    )
    .expect("write settings");
    let socket = root.path().join("daemon.sock");
    let _restore = EnvRestore::set(&socket, &config);

    let mut owner_daemon = spawn_daemon(true);
    let owner = connect_owned().await;
    assert!(owner.owns_lifecycle());
    let manifest_path = config.join("runtime.json");
    let manifest: localsend_rpc::transport::RuntimeManifest =
        serde_json::from_slice(&std::fs::read(&manifest_path).expect("runtime manifest"))
            .expect("parse runtime manifest");
    assert_eq!(manifest.endpoint, socket.to_string_lossy());
    assert_eq!(manifest.pid, owner_daemon.id());
    assert_eq!(manifest.lifecycle, DaemonLifecycle::Owned);

    let mut duplicates = (0..8).map(|_| spawn_daemon(true)).collect::<Vec<_>>();
    for duplicate in &mut duplicates {
        let status = wait_for_exit(duplicate, Duration::from_secs(2)).await;
        assert!(status.success(), "duplicate daemon should exit cleanly");
    }
    assert!(owner_daemon.try_wait().expect("poll owner").is_none());

    drop(owner);
    let status = wait_for_exit(&mut owner_daemon, Duration::from_secs(6)).await;
    assert!(
        status.success(),
        "owned daemon should exit after owner loss"
    );
    assert!(
        !manifest_path.exists(),
        "runtime manifest should be removed"
    );

    let mut persistent = spawn_daemon(false);
    let client = connect_normal().await;
    assert!(!client.owns_lifecycle());
    assert!(DaemonClient::connect_owned().await.is_err());
    client
        .command(ServiceCommand::Shutdown)
        .await
        .expect("shutdown persistent daemon");
    drop(client);
    let status = wait_for_exit(&mut persistent, Duration::from_secs(4)).await;
    assert!(status.success(), "persistent daemon should stop on request");
}

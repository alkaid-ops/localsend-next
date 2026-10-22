use localsend_rpc::{AutoSaveMode, Protocol, ServiceCommand};
use localsend_service::ServiceHandle;
use localsend_testkit::{PeerChecksum, PeerFile, ProtocolType, SenderPeer};
use std::ffi::OsString;
use std::time::Duration;
use tempfile::TempDir;
use tokio_util::sync::CancellationToken;

struct EnvRestore {
    config: Option<OsString>,
}

impl EnvRestore {
    fn set(config: &std::path::Path) -> Self {
        let previous = Self {
            config: std::env::var_os("LOCALSEND_CONFIG_DIR"),
        };
        unsafe {
            std::env::set_var("LOCALSEND_CONFIG_DIR", config);
        }
        previous
    }
}

impl Drop for EnvRestore {
    fn drop(&mut self) {
        unsafe {
            if let Some(value) = self.config.take() {
                std::env::set_var("LOCALSEND_CONFIG_DIR", value);
            } else {
                std::env::remove_var("LOCALSEND_CONFIG_DIR");
            }
        }
    }
}

async fn wait_for_server(service: &ServiceHandle) -> u16 {
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let snapshot = service.snapshot();
            if snapshot.server_running {
                return snapshot.server_port.expect("server port");
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    })
    .await
    .expect("service did not start its HTTP server")
}

#[tokio::test(flavor = "multi_thread")]
async fn verify_checksums_setting_controls_incoming_validation() {
    let root = TempDir::new().expect("temp root");
    let config = root.path().join("config");
    let destination = root.path().join("received");
    std::fs::create_dir_all(&config).expect("create config");
    std::fs::create_dir_all(&destination).expect("create destination");
    let settings = localsend_rpc::DesktopSettings {
        port: 0,
        protocol: Protocol::Http,
        destination_dir: Some(destination.clone()),
        auto_save: AutoSaveMode::On,
        verify_checksums: false,
        start_server_on_launch: true,
        ..localsend_rpc::DesktopSettings::default()
    };
    std::fs::write(
        config.join("settings.json"),
        serde_json::to_vec(&settings).expect("serialize settings"),
    )
    .expect("write settings");
    let _restore = EnvRestore::set(&config);

    let service = ServiceHandle::spawn().await.expect("spawn service");
    let port = wait_for_server(&service).await;
    let first = root.path().join("verification-disabled.txt");
    std::fs::write(&first, b"verification disabled").expect("write first file");
    SenderPeer::send_files_with_checksum(
        "127.0.0.1",
        port,
        ProtocolType::Http,
        None,
        vec![PeerFile {
            path: first,
            remote_name: "verification-disabled.txt".to_string(),
        }],
        PeerChecksum::Fixed("00".repeat(32)),
        CancellationToken::new(),
    )
    .await
    .expect("receiver should ignore checksum when verification is disabled");
    assert!(destination.join("verification-disabled.txt").exists());

    let snapshot = service.snapshot();
    let mut settings = snapshot.settings.clone();
    settings.verify_checksums = true;
    service
        .command(ServiceCommand::UpdateSettings {
            expected_revision: snapshot.settings.revision,
            settings,
        })
        .await
        .expect("enable verification");
    let port = wait_for_server(&service).await;
    let second = root.path().join("verification-enabled.txt");
    std::fs::write(&second, b"verification enabled").expect("write second file");
    let error = SenderPeer::send_files_with_checksum(
        "127.0.0.1",
        port,
        ProtocolType::Http,
        None,
        vec![PeerFile {
            path: second,
            remote_name: "verification-enabled.txt".to_string(),
        }],
        PeerChecksum::Fixed("00".repeat(32)),
        CancellationToken::new(),
    )
    .await
    .expect_err("receiver should reject a mismatched checksum");
    assert!(matches!(
        error,
        localsend_core::http::client::ClientError::StatusCode(status) if status.status == 422
    ));
    assert!(!destination.join("verification-enabled.txt").exists());

    let _ = service.command(ServiceCommand::Shutdown).await;
}

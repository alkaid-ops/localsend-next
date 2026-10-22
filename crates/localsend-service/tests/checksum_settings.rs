use localsend_rpc::{DeviceInfo, DeviceType, Protocol, SendFileRequest, ServiceCommand};
use localsend_service::ServiceHandle;
use localsend_testkit::{PeerAcceptance, ProtocolType, ReceiverPeer};
use sha2::{Digest, Sha256};
use std::ffi::OsString;
use std::time::Duration;
use tempfile::TempDir;

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

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[tokio::test(flavor = "multi_thread")]
async fn create_checksums_setting_controls_outgoing_sha256() {
    let root = TempDir::new().expect("temp root");
    let config = root.path().join("config");
    std::fs::create_dir_all(&config).expect("create config");
    let settings = localsend_rpc::DesktopSettings {
        start_server_on_launch: false,
        create_checksums: true,
        ..localsend_rpc::DesktopSettings::default()
    };
    std::fs::write(
        config.join("settings.json"),
        serde_json::to_vec(&settings).expect("serialize settings"),
    )
    .expect("write settings");
    let _restore = EnvRestore::set(&config);

    let peer = ReceiverPeer::start(ProtocolType::Http, PeerAcceptance::All, None, None)
        .await
        .expect("start peer");
    let service = ServiceHandle::spawn().await.expect("spawn service");
    let target = || DeviceInfo {
        alias: "checksum peer".to_string(),
        version: "2.2".to_string(),
        device_model: Some("test".to_string()),
        device_type: Some(DeviceType::Desktop),
        fingerprint: peer.fingerprint().to_string(),
        port: peer.port(),
        protocol: Protocol::Http,
        download: false,
        ip: Some("127.0.0.1".to_string()),
    };

    let first_bytes = b"checksum enabled";
    let first = root.path().join("checksum-enabled.txt");
    std::fs::write(&first, first_bytes).expect("write first source");
    service
        .command(ServiceCommand::SendFiles {
            target_fingerprint: Some(peer.fingerprint().to_string()),
            target: Some(target()),
            files: vec![SendFileRequest {
                path: first,
                file_id: Some("checksum-enabled".to_string()),
                sha256: None,
                metadata: None,
            }],
            pin: None,
        })
        .await
        .expect("send with checksums");
    let received = peer
        .wait_for_files(1, Duration::from_secs(15))
        .await
        .expect("peer receives checksummed file");
    assert_eq!(
        received[0].sha256.as_deref(),
        Some(sha256_hex(first_bytes).as_str())
    );
    assert_eq!(
        peer.status().await.requests[0].alias,
        settings.alias,
        "the sender must register with the daemon's configured alias"
    );

    let snapshot = service.snapshot();
    let mut settings = snapshot.settings.clone();
    settings.create_checksums = false;
    service
        .command(ServiceCommand::UpdateSettings {
            expected_revision: snapshot.settings.revision,
            settings,
        })
        .await
        .expect("disable checksums");

    let second = root.path().join("checksum-disabled.txt");
    std::fs::write(&second, b"checksum disabled").expect("write second source");
    service
        .command(ServiceCommand::SendFiles {
            target_fingerprint: Some(peer.fingerprint().to_string()),
            target: Some(target()),
            files: vec![SendFileRequest {
                path: second,
                file_id: Some("checksum-disabled".to_string()),
                sha256: None,
                metadata: None,
            }],
            pin: None,
        })
        .await
        .expect("send without checksums");
    let received = peer
        .wait_for_files(2, Duration::from_secs(15))
        .await
        .expect("peer receives unchecksummed file");
    let second = received
        .iter()
        .find(|file| file.file_id == "checksum-disabled")
        .expect("second received file");
    assert!(second.sha256.is_none());
    assert!(peer.status().await.errors.is_empty());

    let _ = service.command(ServiceCommand::Shutdown).await;
    peer.shutdown().await;
}

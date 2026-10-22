use localsend_rpc::{
    DeviceInfo, DeviceType, Protocol, SendFileRequest, ServiceCommand, ServiceEvent, TransferStatus,
};
use localsend_service::ServiceHandle;
use localsend_testkit::{PeerAcceptance, ProtocolType, ReceiverPeer};
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

#[tokio::test(flavor = "multi_thread")]
async fn https_send_pins_certificate_without_treating_fingerprint_as_public_key() {
    let root = TempDir::new().expect("temp root");
    let config = root.path().join("config");
    std::fs::create_dir_all(&config).expect("create config");
    let settings = localsend_rpc::DesktopSettings {
        port: 0,
        start_server_on_launch: false,
        ..localsend_rpc::DesktopSettings::default()
    };
    std::fs::write(
        config.join("settings.json"),
        serde_json::to_vec(&settings).expect("serialize settings"),
    )
    .expect("write settings");
    let _restore = EnvRestore::set(&config);

    let peer = ReceiverPeer::start(ProtocolType::Https, PeerAcceptance::All, None, None)
        .await
        .expect("start HTTPS peer");
    let service = ServiceHandle::spawn().await.expect("spawn service");
    let mut events = service.subscribe();
    let source = root.path().join("https-regression.txt");
    std::fs::write(&source, b"fingerprint is not a public key").expect("write source");

    service
        .command(ServiceCommand::SendFiles {
            target_fingerprint: Some(peer.fingerprint().to_string()),
            target: Some(DeviceInfo {
                alias: "https peer".to_string(),
                version: "2.2".to_string(),
                device_model: Some("test".to_string()),
                device_type: Some(DeviceType::Mobile),
                fingerprint: peer.fingerprint().to_string(),
                port: peer.port(),
                protocol: Protocol::Https,
                download: false,
                ip: Some("127.0.0.1".to_string()),
            }),
            files: vec![SendFileRequest {
                path: source,
                file_id: Some("https-regression".to_string()),
                sha256: None,
                metadata: None,
            }],
            pin: None,
        })
        .await
        .expect("send command");

    let finished = tokio::time::timeout(Duration::from_secs(20), async {
        loop {
            let event = events.recv().await.expect("service event stream").event;
            match event {
                ServiceEvent::TransferFinished { transfer }
                    if transfer.status == TransferStatus::Finished =>
                {
                    return transfer;
                }
                ServiceEvent::TransferFailed { error, .. } => {
                    panic!("HTTPS transfer failed: {error}");
                }
                _ => {}
            }
        }
    })
    .await
    .expect("HTTPS transfer timed out");
    assert_eq!(finished.status, TransferStatus::Finished);

    let received = peer
        .wait_for_files(1, Duration::from_secs(5))
        .await
        .expect("peer receives file");
    assert_eq!(
        std::fs::read(received[0].path.as_ref().expect("received path"))
            .expect("read received file"),
        b"fingerprint is not a public key"
    );
    assert!(peer.status().await.errors.is_empty());

    let _ = service.command(ServiceCommand::Shutdown).await;
    peer.shutdown().await;
}

use localsend_rpc::{DeviceInfo, DeviceType, Protocol, SendFileRequest, ServiceCommand};
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
#[ignore = "writes a >2 GiB file; run explicitly for release acceptance"]
async fn transfer_larger_than_two_gib_streams_without_buffering_in_memory() {
    let root = TempDir::new().expect("temp root");
    let config = root.path().join("config");
    std::fs::create_dir_all(&config).expect("create config");
    let settings = localsend_rpc::DesktopSettings {
        start_server_on_launch: false,
        create_checksums: false,
        ..localsend_rpc::DesktopSettings::default()
    };
    std::fs::write(
        config.join("settings.json"),
        serde_json::to_vec(&settings).expect("serialize settings"),
    )
    .expect("write settings");
    let _restore = EnvRestore::set(&config);

    let source = root.path().join("larger-than-2g.bin");
    let size = 2_u64 * 1024 * 1024 * 1024 + 1;
    let file = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&source)
        .expect("create sparse file");
    file.set_len(size).expect("set sparse file length");
    file.sync_all().expect("sync sparse file");
    drop(file);

    let peer = ReceiverPeer::start(ProtocolType::Http, PeerAcceptance::All, None, None)
        .await
        .expect("start peer");
    let service = ServiceHandle::spawn().await.expect("spawn service");
    let started = std::time::Instant::now();
    service
        .command(ServiceCommand::SendFiles {
            target_fingerprint: Some(peer.fingerprint().to_string()),
            target: Some(DeviceInfo {
                alias: "large peer".to_string(),
                version: "2.2".to_string(),
                device_model: Some("test".to_string()),
                device_type: Some(DeviceType::Desktop),
                fingerprint: peer.fingerprint().to_string(),
                port: peer.port(),
                protocol: Protocol::Http,
                download: false,
                ip: Some("127.0.0.1".to_string()),
            }),
            files: vec![SendFileRequest {
                path: source,
                file_id: None,
                sha256: None,
                metadata: None,
            }],
            pin: None,
        })
        .await
        .expect("send large file");

    let received = peer
        .wait_for_files(1, Duration::from_secs(300))
        .await
        .expect("peer receives >2 GiB file");
    assert_eq!(received[0].size, size);
    let target = received[0].path.as_ref().expect("received path");
    assert_eq!(
        std::fs::metadata(target).expect("target metadata").len(),
        size
    );
    eprintln!(
        "streamed {} MiB in {:.2}s",
        size / 1_048_576,
        started.elapsed().as_secs_f64()
    );

    let _ = service.command(ServiceCommand::Shutdown).await;
    peer.shutdown().await;
}

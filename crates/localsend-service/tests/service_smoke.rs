use localsend_rpc::{
    DeviceInfo, Protocol, SendFileRequest, ServiceCommand, ServiceEvent, TransferStatus,
};
use localsend_service::ServiceHandle;
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

async fn wait_for_server(service: &ServiceHandle) -> (u16, String) {
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let snapshot = service.snapshot();
            if snapshot.server_running {
                return (
                    snapshot.server_port.expect("server port"),
                    snapshot.settings.alias,
                );
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    })
    .await
    .expect("service did not start its HTTP server")
}

#[tokio::test(flavor = "multi_thread")]
async fn service_exposes_snapshot_and_can_send_to_its_own_receiver() {
    let root = TempDir::new().expect("temp root");
    let config = root.path().join("config");
    let destination = root.path().join("received");
    std::fs::create_dir_all(&config).expect("create config");
    std::fs::create_dir_all(&destination).expect("create destination");
    let settings = localsend_rpc::DesktopSettings {
        alias: "localsend-next test".to_string(),
        port: 0,
        protocol: Protocol::Http,
        destination_dir: Some(destination.clone()),
        auto_save: localsend_rpc::AutoSaveMode::On,
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
    let (port, alias) = wait_for_server(&service).await;
    let source = root.path().join("hello.txt");
    std::fs::write(&source, b"official core adapter").expect("write source");
    let mut events = service.subscribe();
    service
        .command(ServiceCommand::SendFiles {
            target_fingerprint: None,
            target: Some(DeviceInfo {
                alias,
                version: "2.2".to_string(),
                device_model: Some("test".to_string()),
                device_type: Some(localsend_rpc::DeviceType::Desktop),
                fingerprint: "loopback".to_string(),
                port,
                protocol: Protocol::Http,
                download: false,
                ip: Some("127.0.0.1".to_string()),
            }),
            files: vec![SendFileRequest {
                path: source,
                file_id: Some("hello-file".to_string()),
                sha256: None,
                metadata: None,
            }],
            pin: None,
        })
        .await
        .expect("send command");

    let result = tokio::time::timeout(Duration::from_secs(20), async {
        let mut send_finished = false;
        let mut receive_finished = false;
        while !(send_finished && receive_finished) {
            let event = events.recv().await.expect("service event stream").event;
            match event {
                ServiceEvent::TransferFinished { transfer }
                    if transfer.status == TransferStatus::Finished =>
                {
                    match transfer.direction {
                        localsend_rpc::TransferDirection::Send => send_finished = true,
                        localsend_rpc::TransferDirection::Receive => receive_finished = true,
                    }
                }
                ServiceEvent::TransferFailed {
                    transfer_id, error, ..
                } => panic!("transfer {transfer_id} failed: {error}"),
                _ => {}
            }
        }
    })
    .await;
    assert!(result.is_ok(), "loopback transfer did not finish");
    assert_eq!(
        std::fs::read(destination.join("hello.txt")).expect("received file"),
        b"official core adapter"
    );
    let _ = service.command(ServiceCommand::Shutdown).await;
}

use localsend_rpc::{
    AutoSaveMode, DesktopSettings, ServiceCommand, TransferDirection, TransferStatus,
};
use localsend_service::ServiceHandle;
use localsend_testkit::{PeerFile, ProtocolType, SenderPeer};
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
async fn auto_save_receives_without_frontend_and_auto_finish_clears_the_transfer() {
    let root = TempDir::new().expect("temp root");
    let config = root.path().join("config");
    let destination = root.path().join("received");
    std::fs::create_dir_all(&config).expect("create config");
    std::fs::create_dir_all(&destination).expect("create destination");
    let settings = DesktopSettings {
        port: 0,
        protocol: localsend_rpc::Protocol::Http,
        destination_dir: Some(destination.clone()),
        auto_save: AutoSaveMode::On,
        auto_finish: true,
        start_server_on_launch: true,
        ..DesktopSettings::default()
    };
    std::fs::write(
        config.join("settings.json"),
        serde_json::to_vec(&settings).expect("serialize settings"),
    )
    .expect("write settings");
    let _restore = EnvRestore::set(&config);

    let service = ServiceHandle::spawn().await.expect("spawn service");
    let port = tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let snapshot = service.snapshot();
            if snapshot.server_running
                && let Some(port) = snapshot.server_port
            {
                break port;
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    })
    .await
    .expect("server did not start");

    let source = root.path().join("auto-save.txt");
    std::fs::write(&source, b"quick save").expect("write source");
    SenderPeer::send_files(
        "127.0.0.1",
        port,
        ProtocolType::Http,
        None,
        vec![PeerFile {
            path: source,
            remote_name: "auto-save.txt".to_string(),
        }],
    )
    .await
    .expect("quick-save sender failed");

    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            if destination.join("auto-save.txt").exists() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    })
    .await
    .expect("auto-save file was not written");
    assert_eq!(
        std::fs::read(destination.join("auto-save.txt")).expect("received file"),
        b"quick save"
    );

    tokio::time::timeout(Duration::from_secs(8), async {
        loop {
            let snapshot = service.snapshot();
            let receive_finished = snapshot.active_transfers.iter().any(|transfer| {
                transfer.direction == TransferDirection::Receive
                    && transfer.status == TransferStatus::Finished
            });
            if receive_finished {
                break;
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    })
    .await
    .expect("receive transfer did not finish");

    tokio::time::sleep(Duration::from_secs(4)).await;
    assert!(
        !service
            .snapshot()
            .active_transfers
            .iter()
            .any(|transfer| transfer.direction == TransferDirection::Receive),
        "auto-finish did not clear the completed receive transfer"
    );

    let _ = service.command(ServiceCommand::Shutdown).await;
}

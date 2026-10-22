use localsend_rpc::{AutoSaveMode, Protocol, ServiceCommand};
use localsend_service::ServiceHandle;
use localsend_testkit::{PeerFile, ProtocolType, SenderPeer};
use std::ffi::OsString;
use std::path::Path;
use std::time::Duration;
use tempfile::TempDir;

struct EnvRestore {
    config: Option<OsString>,
}

impl EnvRestore {
    fn set(config: &Path) -> Self {
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
async fn incoming_relative_paths_are_preserved_without_escaping_destination() {
    let root = TempDir::new().expect("temp root");
    let config = root.path().join("config");
    let destination = root.path().join("downloads");
    std::fs::create_dir_all(&config).expect("create config");
    std::fs::create_dir_all(&destination).expect("create destination");
    let settings = localsend_rpc::DesktopSettings {
        port: 0,
        protocol: Protocol::Http,
        destination_dir: Some(destination.clone()),
        auto_save: AutoSaveMode::On,
        ..localsend_rpc::DesktopSettings::default()
    };
    std::fs::write(
        config.join("settings.json"),
        serde_json::to_vec(&settings).expect("serialize settings"),
    )
    .expect("write settings");
    let _restore = EnvRestore::set(&config);

    let alpha = root.path().join("alpha-source.txt");
    let beta = root.path().join("beta-source.txt");
    let escape = root.path().join("escape-source.txt");
    std::fs::write(&alpha, b"alpha").expect("write alpha");
    std::fs::write(&beta, b"beta").expect("write beta");
    std::fs::write(&escape, b"escape").expect("write escape fixture");

    let service = ServiceHandle::spawn().await.expect("spawn service");
    let port = wait_for_server(&service).await;
    SenderPeer::send_files(
        "127.0.0.1",
        port,
        ProtocolType::Http,
        None,
        vec![
            PeerFile {
                path: alpha,
                remote_name: "folder/alpha.txt".to_string(),
            },
            PeerFile {
                path: beta,
                remote_name: "folder/sub/beta.txt".to_string(),
            },
        ],
    )
    .await
    .expect("nested files are accepted");

    assert_eq!(
        std::fs::read(destination.join("folder/alpha.txt")).expect("nested alpha"),
        b"alpha"
    );
    assert_eq!(
        std::fs::read(destination.join("folder/sub/beta.txt")).expect("nested beta"),
        b"beta"
    );
    assert!(!destination.join("beta.txt").exists());

    SenderPeer::send_files(
        "127.0.0.1",
        port,
        ProtocolType::Http,
        None,
        vec![PeerFile {
            path: escape,
            remote_name: "../../escape.txt".to_string(),
        }],
    )
    .await
    .expect("traversal-like name is safely accepted");
    assert_eq!(
        std::fs::read(destination.join("escape.txt")).expect("sanitized escape file"),
        b"escape"
    );
    assert!(!root.path().join("escape.txt").exists());

    let _ = service.command(ServiceCommand::Shutdown).await;
}

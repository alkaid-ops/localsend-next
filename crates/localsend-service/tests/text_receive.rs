use localsend_rpc::{AutoSaveMode, Protocol, ServiceCommand, ServiceEvent};
use localsend_service::ServiceHandle;
use localsend_testkit::{ProtocolType, SenderPeer};
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
async fn current_official_text_protocol_is_received_and_not_left_on_disk() {
    let root = TempDir::new().expect("temp root");
    let config = root.path().join("config");
    let destination = root.path().join("downloads");
    std::fs::create_dir_all(&config).expect("create config");
    std::fs::create_dir_all(&destination).expect("create destination");
    let settings = localsend_rpc::DesktopSettings {
        port: 0,
        protocol: Protocol::Http,
        destination_dir: Some(destination.clone()),
        auto_save: AutoSaveMode::Off,
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
    let mut events = service.subscribe();
    let text = "官方协议文本\nsecond line";

    SenderPeer::send_text("127.0.0.1", port, ProtocolType::Http, None, text)
        .await
        .expect("sender uploads text bytes");

    let received = tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let event = events.recv().await.expect("service event stream").event;
            if let ServiceEvent::TextReceived {
                sender_alias,
                text: received,
            } = event
            {
                return (sender_alias, received);
            }
        }
    })
    .await
    .expect("text event timed out");
    assert_eq!(received.0, "deterministic-sender");
    assert_eq!(received.1, text);

    let snapshot = service.snapshot();
    let entry = snapshot.history.last().expect("message history entry");
    assert_eq!(entry.message.as_deref(), Some(text));
    assert!(entry.path.is_none());
    assert!(
        std::fs::read_dir(&destination)
            .expect("destination directory")
            .next()
            .is_none()
    );

    let legacy_text = "legacy preview message";
    SenderPeer::send_legacy_preview_text("127.0.0.1", port, ProtocolType::Http, legacy_text)
        .await
        .expect("legacy sender succeeds");
    let legacy = tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let event = events.recv().await.expect("service event stream").event;
            if let ServiceEvent::TextReceived { text, .. } = event {
                return text;
            }
        }
    })
    .await
    .expect("legacy text event timed out");
    assert_eq!(legacy, legacy_text);
    let snapshot = service.snapshot();
    assert_eq!(
        snapshot
            .history
            .last()
            .and_then(|entry| entry.message.as_deref()),
        Some(legacy_text)
    );

    let _ = service.command(ServiceCommand::Shutdown).await;
}

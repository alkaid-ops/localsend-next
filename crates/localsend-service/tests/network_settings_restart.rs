use localsend_rpc::{DesktopSettings, Protocol, ServiceCommand, ServiceEvent};
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
            std::env::set_var("LOCALSEND_TEST_DISABLE_DISCOVERY", "1");
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
            std::env::remove_var("LOCALSEND_TEST_DISABLE_DISCOVERY");
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
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    })
    .await
    .expect("server did not start")
}

#[tokio::test(flavor = "multi_thread")]
async fn network_settings_restart_server_and_persist_values() {
    let root = TempDir::new().expect("temp root");
    let config = root.path().join("config");
    std::fs::create_dir_all(&config).expect("create config");
    let settings = DesktopSettings {
        port: 0,
        protocol: Protocol::Http,
        start_server_on_launch: true,
        ..DesktopSettings::default()
    };
    std::fs::write(
        config.join("settings.json"),
        serde_json::to_vec_pretty(&settings).expect("serialize settings"),
    )
    .expect("write settings");
    let _restore = EnvRestore::set(&config);

    let service = ServiceHandle::spawn().await.expect("spawn service");
    let first_port = wait_for_server(&service).await;
    let mut events = service.subscribe();
    let snapshot = service.snapshot();
    let mut changed = snapshot.settings.clone();
    changed.multicast_group = Some("239.255.88.61".to_string());
    changed.interface_whitelist = Some(vec!["en0".to_string()]);
    changed.interface_blacklist = Some(vec!["utun*".to_string()]);
    changed.discovery_timeout_ms = 750;
    service
        .command(ServiceCommand::UpdateSettings {
            expected_revision: snapshot.settings.revision,
            settings: changed,
        })
        .await
        .expect("update network settings");

    let second_port = wait_for_server(&service).await;
    let mut saw_stop = false;
    let mut saw_start = false;
    tokio::time::timeout(Duration::from_secs(5), async {
        while !(saw_stop && saw_start) {
            if let Ok(event) = events.recv().await {
                match event.event {
                    ServiceEvent::ServerState { running: false, .. } => saw_stop = true,
                    ServiceEvent::ServerState { running: true, .. } => saw_start = true,
                    _ => {}
                }
            }
        }
    })
    .await
    .expect("server restart events");
    assert!(saw_stop && saw_start);
    assert_ne!(first_port, 0);
    assert_ne!(second_port, 0);

    let snapshot = service.snapshot();
    assert_eq!(snapshot.settings.revision, settings.revision + 1);
    assert_eq!(
        snapshot.settings.multicast_group.as_deref(),
        Some("239.255.88.61")
    );
    assert_eq!(
        snapshot.settings.interface_whitelist,
        Some(vec!["en0".to_string()])
    );
    assert_eq!(
        snapshot.settings.interface_blacklist,
        Some(vec!["utun*".to_string()])
    );
    assert_eq!(snapshot.settings.discovery_timeout_ms, 750);

    let persisted: DesktopSettings = serde_json::from_slice(
        &std::fs::read(config.join("settings.json")).expect("read persisted settings"),
    )
    .expect("parse persisted settings");
    assert_eq!(persisted.multicast_group, snapshot.settings.multicast_group);
    assert_eq!(
        persisted.interface_whitelist,
        snapshot.settings.interface_whitelist
    );
    assert_eq!(
        persisted.interface_blacklist,
        snapshot.settings.interface_blacklist
    );
    assert_eq!(
        persisted.discovery_timeout_ms,
        snapshot.settings.discovery_timeout_ms
    );

    let _ = service.command(ServiceCommand::Shutdown).await;
}

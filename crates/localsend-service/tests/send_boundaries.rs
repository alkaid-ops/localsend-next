use localsend_rpc::{
    AutoSaveMode, DeviceInfo, DeviceType, Protocol, SendFileRequest, ServiceCommand, ServiceEvent,
};
use localsend_service::ServiceHandle;
use localsend_testkit::{PeerAcceptance, ProtocolType, ReceiverPeer};
use sha2::{Digest, Sha256};
use std::ffi::OsString;
use std::fs::{FileTimes, OpenOptions};
use std::time::{Duration, SystemTime};
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

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[tokio::test(flavor = "multi_thread")]
async fn send_handles_unicode_directories_duplicates_mtime_and_symlinks() {
    let root = TempDir::new().expect("temp root");
    let config = root.path().join("config");
    std::fs::create_dir_all(&config).expect("create config");
    let settings = localsend_rpc::DesktopSettings {
        start_server_on_launch: false,
        auto_save: AutoSaveMode::Off,
        ..localsend_rpc::DesktopSettings::default()
    };
    std::fs::write(
        config.join("settings.json"),
        serde_json::to_vec(&settings).expect("serialize settings"),
    )
    .expect("write settings");
    let _restore = EnvRestore::set(&config);

    let unicode = root.path().join("说明-😀.txt");
    std::fs::write(&unicode, b"unicode content").expect("write unicode file");
    let modified = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    OpenOptions::new()
        .write(true)
        .open(&unicode)
        .expect("open unicode file")
        .set_times(FileTimes::new().set_modified(modified))
        .expect("set mtime");

    let folder = root.path().join("folder");
    std::fs::create_dir_all(folder.join("sub")).expect("create nested folder");
    std::fs::create_dir_all(folder.join("empty")).expect("create empty folder");
    std::fs::write(folder.join("alpha.txt"), b"alpha").expect("write alpha");
    std::fs::write(folder.join("sub/beta.txt"), b"beta").expect("write beta");

    let duplicate_a = root.path().join("dup-a.txt");
    let duplicate_b = root.path().join("dup-b.txt");
    std::fs::write(&duplicate_a, b"duplicate a").expect("write duplicate a");
    std::fs::write(&duplicate_b, b"duplicate b").expect("write duplicate b");

    #[cfg(unix)]
    let symlink = root.path().join("do-not-follow.txt");
    #[cfg(unix)]
    std::os::unix::fs::symlink(&duplicate_a, &symlink).expect("create symlink");

    let peer = ReceiverPeer::start(ProtocolType::Http, PeerAcceptance::All, None, None)
        .await
        .expect("start peer");
    let service = ServiceHandle::spawn().await.expect("spawn service");
    let mut events = service.subscribe();

    let mut requests = vec![
        SendFileRequest {
            path: unicode.clone(),
            file_id: None,
            sha256: None,
            metadata: None,
        },
        SendFileRequest {
            path: folder,
            file_id: None,
            sha256: None,
            metadata: None,
        },
        SendFileRequest {
            path: duplicate_a,
            file_id: None,
            sha256: None,
            metadata: None,
        },
        SendFileRequest {
            path: duplicate_b,
            file_id: None,
            sha256: None,
            metadata: None,
        },
    ];
    #[cfg(unix)]
    requests.push(SendFileRequest {
        path: symlink.clone(),
        file_id: None,
        sha256: None,
        metadata: None,
    });

    service
        .command(ServiceCommand::SendFiles {
            target_fingerprint: Some(peer.fingerprint().to_string()),
            target: Some(DeviceInfo {
                alias: "boundary peer".to_string(),
                version: "2.2".to_string(),
                device_model: Some("test".to_string()),
                device_type: Some(DeviceType::Desktop),
                fingerprint: peer.fingerprint().to_string(),
                port: peer.port(),
                protocol: Protocol::Http,
                download: false,
                ip: Some("127.0.0.1".to_string()),
            }),
            files: requests,
            pin: None,
        })
        .await
        .expect("send boundary files");

    let skipped = tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let event = events.recv().await.expect("event stream").event;
            if let ServiceEvent::TransferPrepared { skipped, .. } = event {
                return skipped;
            }
        }
    })
    .await
    .expect("prepare event timed out");
    #[cfg(unix)]
    {
        assert_eq!(skipped.len(), 1);
        assert_eq!(skipped[0].path, symlink);
    }
    #[cfg(not(unix))]
    assert!(skipped.is_empty());

    let received = peer
        .wait_for_files(5, Duration::from_secs(20))
        .await
        .expect("peer receives boundary files");
    assert_eq!(received.len(), 5);
    assert!(peer.status().await.errors.is_empty());

    let by_name = |name: &str| {
        received
            .iter()
            .filter(|file| file.file_name == name)
            .collect::<Vec<_>>()
    };
    assert_eq!(by_name("说明-😀.txt").len(), 1);
    assert_eq!(by_name("folder/alpha.txt").len(), 1);
    assert_eq!(by_name("folder/sub/beta.txt").len(), 1);
    assert_eq!(by_name("dup-a.txt").len(), 1);
    assert_eq!(by_name("dup-b.txt").len(), 1);

    let unicode_received = by_name("说明-😀.txt")[0];
    assert_eq!(
        unicode_received.sha256.as_deref(),
        Some(sha256(b"unicode content").as_str())
    );
    let received_mtime = std::fs::metadata(unicode_received.path.as_ref().unwrap())
        .expect("received metadata")
        .modified()
        .expect("received mtime");
    let delta = received_mtime
        .duration_since(modified)
        .or_else(|error| Ok::<_, std::time::SystemTimeError>(error.duration()))
        .expect("mtime delta");
    assert!(delta <= Duration::from_secs(2), "mtime delta: {delta:?}");

    let _ = service.command(ServiceCommand::Shutdown).await;
    peer.shutdown().await;
}

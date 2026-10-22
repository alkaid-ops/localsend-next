use localsend_rpc::{
    CommandResult, DeviceInfo, DeviceType, Protocol, ServiceCommand, ServiceEvent, TransferStatus,
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
async fn text_send_uploads_utf8_bytes_with_text_preview() {
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
    let text = "跨平台 text\n第二行";
    service
        .command(ServiceCommand::SendText {
            target_fingerprint: Some(peer.fingerprint().to_string()),
            target: Some(DeviceInfo {
                alias: "text peer".to_string(),
                version: "2.2".to_string(),
                device_model: Some("test".to_string()),
                device_type: Some(DeviceType::Desktop),
                fingerprint: peer.fingerprint().to_string(),
                port: peer.port(),
                protocol: Protocol::Http,
                download: false,
                ip: Some("127.0.0.1".to_string()),
            }),
            text: text.to_string(),
            pin: None,
        })
        .await
        .expect("send text command");

    let received = peer
        .wait_for_files(1, Duration::from_secs(15))
        .await
        .expect("peer receives text bytes");
    assert_eq!(received[0].file_name, "message.txt");
    assert_eq!(received[0].size, text.len() as u64);
    let path = received[0].path.as_ref().expect("received path");
    assert_eq!(std::fs::read(path).expect("read text"), text.as_bytes());

    let status = peer.status().await;
    assert_eq!(status.requests[0].previews, vec![Some(text.to_string())]);
    assert!(status.errors.is_empty());
    assert_eq!(status.received_files.len(), 1);

    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    loop {
        if service
            .snapshot()
            .active_transfers
            .iter()
            .any(|transfer| transfer.status == TransferStatus::Finished)
            || service.snapshot().active_transfers.is_empty()
        {
            break;
        }
        assert!(tokio::time::Instant::now() < deadline);
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    let _ = service.command(ServiceCommand::Shutdown).await;
    peer.shutdown().await;

    // Official LocalSend answers a message-only prepare-upload with 204 and no
    // upload token. That means the message preview was delivered, not rejected.
    let peer = ReceiverPeer::start(ProtocolType::Http, PeerAcceptance::First(0), None, None)
        .await
        .expect("start no-content peer");
    let service = ServiceHandle::spawn()
        .await
        .expect("spawn no-content service");
    let mut events = service.subscribe();
    let result = service
        .command(ServiceCommand::SendText {
            target_fingerprint: Some(peer.fingerprint().to_string()),
            target: Some(DeviceInfo {
                alias: "no-content text peer".to_string(),
                version: "2.2".to_string(),
                device_model: Some("test".to_string()),
                device_type: Some(DeviceType::Desktop),
                fingerprint: peer.fingerprint().to_string(),
                port: peer.port(),
                protocol: Protocol::Http,
                download: false,
                ip: Some("127.0.0.1".to_string()),
            }),
            text: "x".to_string(),
            pin: None,
        })
        .await
        .expect("send message-only text");
    let CommandResult::Transfer { transfer } = result else {
        panic!("send did not return a transfer");
    };
    let finished = tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let event = events.recv().await.expect("service event").event;
            if let ServiceEvent::TransferFinished { transfer: finished } = event
                && finished.transfer_id == transfer.transfer_id
            {
                return finished;
            }
        }
    })
    .await
    .expect("message-only transfer timeout");
    assert_eq!(finished.status, TransferStatus::Finished);
    assert_eq!(finished.bytes_done, 1);
    let status = peer.status().await;
    assert_eq!(status.requests.len(), 1);
    assert_eq!(status.requests[0].previews, vec![Some("x".to_string())]);
    assert!(status.received_files.is_empty());

    let _ = service.command(ServiceCommand::Shutdown).await;
    peer.shutdown().await;
}

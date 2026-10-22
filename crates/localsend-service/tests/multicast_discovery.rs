#![cfg(unix)]

use localsend_rpc::{DesktopSettings, Protocol};
use localsend_testkit::{DaemonFixture, daemon_binary_for_tests};
use std::net::TcpListener;
use std::time::Duration;
use tokio::time::{Instant, sleep};

#[tokio::test(flavor = "multi_thread")]
#[ignore = "uses real LocalSend multicast groups; run only with no production instance"]
async fn discovery_uses_fixed_port_and_official_group_compatibility() -> anyhow::Result<()> {
    let first_listener = TcpListener::bind("127.0.0.1:0")?;
    let second_listener = TcpListener::bind("127.0.0.1:0")?;
    let custom_port = first_listener.local_addr()?.port();
    let default_port = second_listener.local_addr()?.port();
    drop(first_listener);
    drop(second_listener);
    let mut custom = DaemonFixture::start_with_discovery(
        &daemon_binary_for_tests()?,
        DesktopSettings {
            alias: "custom-group-peer".to_string(),
            port: custom_port,
            protocol: Protocol::Http,
            multicast_group: Some("239.255.88.61".to_string()),
            start_server_on_launch: true,
            ..DesktopSettings::default()
        },
    )
    .await?;
    let mut default = DaemonFixture::start_with_discovery(
        &daemon_binary_for_tests()?,
        DesktopSettings {
            alias: "official-group-peer".to_string(),
            port: default_port,
            protocol: Protocol::Http,
            multicast_group: None,
            start_server_on_launch: true,
            ..DesktopSettings::default()
        },
    )
    .await?;

    let deadline = Instant::now() + Duration::from_secs(15);
    loop {
        let custom_sees_default = custom
            .snapshot()
            .await?
            .devices
            .iter()
            .any(|device| device.device.alias == "official-group-peer");
        let default_sees_custom = default
            .snapshot()
            .await?
            .devices
            .iter()
            .any(|device| device.device.alias == "custom-group-peer");
        if custom_sees_default && default_sees_custom {
            break;
        }
        anyhow::ensure!(
            Instant::now() < deadline,
            "multicast discovery did not converge"
        );
        sleep(Duration::from_millis(100)).await;
    }

    default.shutdown().await?;
    custom.shutdown().await?;
    Ok(())
}

use localsend_rpc::{DaemonClient, ServiceCommand};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut arguments = std::env::args().skip(1);
    match arguments.next().as_deref() {
        Some("add") => {
            let target = arguments
                .next()
                .ok_or_else(|| anyhow::anyhow!("usage: rpc_probe add host:port"))?;
            let client = DaemonClient::connect_interactive().await?;
            client
                .command(ServiceCommand::AddManualDevice { target })
                .await
                .map_err(anyhow::Error::msg)?;
        }
        Some("shutdown") => {
            let client = DaemonClient::connect().await?;
            client
                .command(ServiceCommand::Shutdown)
                .await
                .map_err(anyhow::Error::msg)?;
            return Ok(());
        }
        Some("text") => {
            let fingerprint = arguments
                .next()
                .ok_or_else(|| anyhow::anyhow!("usage: rpc_probe text fingerprint text"))?;
            let text = arguments
                .next()
                .ok_or_else(|| anyhow::anyhow!("usage: rpc_probe text fingerprint text"))?;
            let client = DaemonClient::connect_interactive().await?;
            let snapshot = match client
                .command(ServiceCommand::GetSnapshot)
                .await
                .map_err(anyhow::Error::msg)?
            {
                localsend_rpc::CommandResult::Snapshot { snapshot } => snapshot,
                other => anyhow::bail!("unexpected snapshot result: {other:?}"),
            };
            let target = snapshot
                .devices
                .into_iter()
                .find(|device| device.device.fingerprint == fingerprint)
                .map(|device| device.device)
                .ok_or_else(|| anyhow::anyhow!("device not found: {fingerprint}"))?;
            client
                .command(ServiceCommand::SendText {
                    target_fingerprint: Some(fingerprint),
                    target: Some(target),
                    text,
                    pin: None,
                })
                .await
                .map_err(anyhow::Error::msg)?;
            return Ok(());
        }
        Some("send") => {
            let fingerprint = arguments
                .next()
                .ok_or_else(|| anyhow::anyhow!("usage: rpc_probe send fingerprint path"))?;
            let path = arguments
                .next()
                .ok_or_else(|| anyhow::anyhow!("usage: rpc_probe send fingerprint path"))?;
            let client = DaemonClient::connect_interactive().await?;
            let snapshot = match client
                .command(ServiceCommand::GetSnapshot)
                .await
                .map_err(anyhow::Error::msg)?
            {
                localsend_rpc::CommandResult::Snapshot { snapshot } => snapshot,
                other => anyhow::bail!("unexpected snapshot result: {other:?}"),
            };
            let target = snapshot
                .devices
                .into_iter()
                .find(|device| device.device.fingerprint == fingerprint)
                .map(|device| device.device)
                .ok_or_else(|| anyhow::anyhow!("device not found: {fingerprint}"))?;
            client
                .command(ServiceCommand::SendFiles {
                    target_fingerprint: Some(fingerprint),
                    target: Some(target),
                    files: vec![localsend_rpc::SendFileRequest {
                        path: path.into(),
                        file_id: None,
                        sha256: None,
                        metadata: None,
                    }],
                    pin: None,
                })
                .await
                .map_err(anyhow::Error::msg)?;
            return Ok(());
        }
        Some(value) => anyhow::bail!("unknown command: {value}"),
        None => {}
    }
    let client = DaemonClient::connect_interactive().await?;
    let snapshot = client
        .command(ServiceCommand::GetSnapshot)
        .await
        .map_err(anyhow::Error::msg)?;
    println!("{}", serde_json::to_string_pretty(&snapshot)?);
    Ok(())
}

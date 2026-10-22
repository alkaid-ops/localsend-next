use localsend_rpc::{DaemonClient, ServiceCommand};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut arguments = std::env::args().skip(1);
    match arguments.next().as_deref() {
        Some("snapshot") => {
            let client = DaemonClient::connect().await?;
            let snapshot = match client
                .command(ServiceCommand::GetSnapshot)
                .await
                .map_err(anyhow::Error::msg)?
            {
                localsend_rpc::CommandResult::Snapshot { snapshot } => snapshot,
                other => anyhow::bail!("unexpected snapshot result: {other:?}"),
            };
            println!("{}", serde_json::to_string_pretty(&snapshot)?);
            return Ok(());
        }
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
        Some("repair-acceptance-config") => {
            let client = DaemonClient::connect_interactive().await?;
            let snapshot = match client
                .command(ServiceCommand::GetSnapshot)
                .await
                .map_err(anyhow::Error::msg)?
            {
                localsend_rpc::CommandResult::Snapshot { snapshot } => snapshot,
                other => anyhow::bail!("unexpected snapshot result: {other:?}"),
            };
            let mut settings = snapshot.settings;
            settings.destination_dir = None;
            settings.discovery_timeout_ms = 500;
            client
                .command(ServiceCommand::UpdateSettings {
                    expected_revision: settings.revision,
                    settings,
                })
                .await
                .map_err(anyhow::Error::msg)?;
            return Ok(());
        }
        Some("set-pin") => {
            let pin = arguments
                .next()
                .ok_or_else(|| anyhow::anyhow!("usage: rpc_probe set-pin PIN|none"))?;
            let client = DaemonClient::connect_interactive().await?;
            let mut settings = match client
                .command(ServiceCommand::GetSnapshot)
                .await
                .map_err(anyhow::Error::msg)?
            {
                localsend_rpc::CommandResult::Snapshot { snapshot } => snapshot.settings,
                other => anyhow::bail!("unexpected snapshot result: {other:?}"),
            };
            settings.pin = (!matches!(pin.as_str(), "none" | "off" | "")).then(|| pin.clone());
            client
                .command(ServiceCommand::UpdateSettings {
                    expected_revision: settings.revision,
                    settings,
                })
                .await
                .map_err(anyhow::Error::msg)?;
            return Ok(());
        }
        Some("cancel") => {
            let transfer_id = arguments
                .next()
                .ok_or_else(|| anyhow::anyhow!("usage: rpc_probe cancel TRANSFER_ID"))?;
            let client = DaemonClient::connect_interactive().await?;
            client
                .command(ServiceCommand::CancelTransfer { transfer_id })
                .await
                .map_err(anyhow::Error::msg)?;
            return Ok(());
        }
        Some("start-discovery") => {
            let client = DaemonClient::connect_interactive().await?;
            client
                .command(ServiceCommand::StartDiscovery)
                .await
                .map_err(anyhow::Error::msg)?;
            return Ok(());
        }
        Some("set-setting") => {
            let key = arguments
                .next()
                .ok_or_else(|| anyhow::anyhow!("usage: rpc_probe set-setting KEY VALUE"))?;
            let value = arguments
                .next()
                .ok_or_else(|| anyhow::anyhow!("usage: rpc_probe set-setting KEY VALUE"))?;
            let client = DaemonClient::connect_interactive().await?;
            let mut settings = match client
                .command(ServiceCommand::GetSnapshot)
                .await
                .map_err(anyhow::Error::msg)?
            {
                localsend_rpc::CommandResult::Snapshot { snapshot } => snapshot.settings,
                other => anyhow::bail!("unexpected snapshot result: {other:?}"),
            };
            match key.as_str() {
                "theme" => settings.theme = value,
                "color" => settings.color = value,
                "language" => settings.language = value,
                "alias" => settings.alias = value,
                other => anyhow::bail!("unsupported setting: {other}"),
            }
            client
                .command(ServiceCommand::UpdateSettings {
                    expected_revision: settings.revision,
                    settings,
                })
                .await
                .map_err(anyhow::Error::msg)?;
            return Ok(());
        }
        Some("web-receive") => {
            let client = DaemonClient::connect_interactive().await?;
            let snapshot = match client
                .command(ServiceCommand::GetSnapshot)
                .await
                .map_err(anyhow::Error::msg)?
            {
                localsend_rpc::CommandResult::Snapshot { snapshot } => snapshot,
                other => anyhow::bail!("unexpected snapshot result: {other:?}"),
            };
            client
                .command(ServiceCommand::StartWebShare {
                    files: Vec::new(),
                    pin: snapshot.settings.pin,
                    mode: localsend_rpc::WebShareMode::Receive,
                    auto_accept: snapshot.settings.auto_accept,
                    i18n: Default::default(),
                })
                .await
                .map_err(anyhow::Error::msg)?;
            return Ok(());
        }
        Some("web-share") => {
            let path = arguments
                .next()
                .ok_or_else(|| anyhow::anyhow!("usage: rpc_probe web-share path"))?;
            let client = DaemonClient::connect_interactive().await?;
            let snapshot = match client
                .command(ServiceCommand::GetSnapshot)
                .await
                .map_err(anyhow::Error::msg)?
            {
                localsend_rpc::CommandResult::Snapshot { snapshot } => snapshot,
                other => anyhow::bail!("unexpected snapshot result: {other:?}"),
            };
            client
                .command(ServiceCommand::StartWebShare {
                    files: vec![localsend_rpc::SendFileRequest {
                        path: path.into(),
                        file_id: None,
                        sha256: None,
                        metadata: None,
                    }],
                    pin: snapshot.settings.pin,
                    mode: localsend_rpc::WebShareMode::Share,
                    auto_accept: true,
                    i18n: Default::default(),
                })
                .await
                .map_err(anyhow::Error::msg)?;
            return Ok(());
        }
        Some("web-stop") => {
            let client = DaemonClient::connect().await?;
            client
                .command(ServiceCommand::StopWebShare)
                .await
                .map_err(anyhow::Error::msg)?;
            return Ok(());
        }
        Some("text") => {
            let fingerprint = arguments
                .next()
                .ok_or_else(|| anyhow::anyhow!("usage: rpc_probe text fingerprint text [pin]"))?;
            let text = arguments
                .next()
                .ok_or_else(|| anyhow::anyhow!("usage: rpc_probe text fingerprint text [pin]"))?;
            let pin = arguments.next();
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
                    pin,
                })
                .await
                .map_err(anyhow::Error::msg)?;
            return Ok(());
        }
        Some("latency-text") => {
            let fingerprint = arguments
                .next()
                .ok_or_else(|| anyhow::anyhow!("usage: rpc_probe latency-text fingerprint text"))?;
            let text = arguments
                .next()
                .ok_or_else(|| anyhow::anyhow!("usage: rpc_probe latency-text fingerprint text"))?;
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
            let mut events = client.subscribe();
            let refresh_started = std::time::Instant::now();
            client
                .command(ServiceCommand::RefreshDiscovery)
                .await
                .map_err(anyhow::Error::msg)?;
            let refresh_elapsed = refresh_started.elapsed();
            let send_started = std::time::Instant::now();
            let result = client
                .command(ServiceCommand::SendText {
                    target_fingerprint: Some(fingerprint),
                    target: Some(target),
                    text,
                    pin: None,
                })
                .await
                .map_err(anyhow::Error::msg)?;
            let localsend_rpc::CommandResult::Transfer { transfer } = result else {
                anyhow::bail!("daemon did not create a text transfer");
            };
            let transfer_id = transfer.transfer_id;
            let queued = send_started.elapsed();
            let mut prepared = None;
            let mut offer = None;
            let mut first_progress = None;
            let finished = tokio::time::timeout(std::time::Duration::from_secs(60), async {
                loop {
                    let event = events.recv().await?.event;
                    match event {
                        localsend_rpc::ServiceEvent::TransferPrepared {
                            transfer: prepared_transfer,
                            ..
                        } if prepared_transfer.transfer_id == transfer_id => {
                            prepared.get_or_insert_with(|| send_started.elapsed());
                        }
                        localsend_rpc::ServiceEvent::SendOfferAnswered {
                            transfer_id: offered_id,
                            ..
                        } if offered_id == transfer_id => {
                            offer.get_or_insert_with(|| send_started.elapsed());
                        }
                        localsend_rpc::ServiceEvent::TransferProgress {
                            transfer_id: progress_id,
                            ..
                        } if progress_id == transfer_id => {
                            first_progress.get_or_insert_with(|| send_started.elapsed());
                        }
                        localsend_rpc::ServiceEvent::TransferFinished {
                            transfer: finished_transfer,
                        } if finished_transfer.transfer_id == transfer_id => {
                            return Ok::<_, anyhow::Error>(send_started.elapsed());
                        }
                        localsend_rpc::ServiceEvent::TransferFailed {
                            transfer_id: failed_id,
                            error,
                            ..
                        } if failed_id == transfer_id => {
                            anyhow::bail!("transfer failed: {error}");
                        }
                        _ => {}
                    }
                }
            })
            .await
            .map_err(|_| anyhow::anyhow!("text send timed out"))??;
            println!(
                "refresh_ms={} queued_ms={} prepared_ms={} offer_ms={} first_progress_ms={} finished_ms={}",
                refresh_elapsed.as_millis(),
                queued.as_millis(),
                prepared.unwrap_or_default().as_millis(),
                offer.unwrap_or_default().as_millis(),
                first_progress.unwrap_or_default().as_millis(),
                finished.as_millis()
            );
            return Ok(());
        }
        Some("benchmark-file") => {
            let fingerprint = arguments.next().ok_or_else(|| {
                anyhow::anyhow!("usage: rpc_probe benchmark-file fingerprint path")
            })?;
            let path = arguments.next().ok_or_else(|| {
                anyhow::anyhow!("usage: rpc_probe benchmark-file fingerprint path")
            })?;
            let size = std::fs::metadata(&path)
                .map_err(|error| anyhow::anyhow!("cannot stat benchmark file: {error}"))?
                .len();
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
            let mut events = client.subscribe();
            let started = std::time::Instant::now();
            let result = client
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
            let localsend_rpc::CommandResult::Transfer { transfer } = result else {
                anyhow::bail!("daemon did not create a transfer");
            };
            let queued = started.elapsed();
            let transfer_id = transfer.transfer_id;
            let transfer = tokio::time::timeout(std::time::Duration::from_secs(300), async {
                loop {
                    let event = events.recv().await?.event;
                    match event {
                        localsend_rpc::ServiceEvent::TransferFinished { transfer }
                            if transfer.transfer_id == transfer_id =>
                        {
                            return Ok::<_, anyhow::Error>(transfer);
                        }
                        localsend_rpc::ServiceEvent::TransferFailed {
                            transfer_id: failed_id,
                            error,
                            ..
                        } if failed_id == transfer_id => {
                            anyhow::bail!("transfer failed: {error}");
                        }
                        _ => {}
                    }
                }
            })
            .await
            .map_err(|_| anyhow::anyhow!("benchmark timed out"))??;
            if transfer.status != localsend_rpc::TransferStatus::Finished {
                anyhow::bail!(
                    "transfer ended with {:?}: {}",
                    transfer.status,
                    transfer.error.as_deref().unwrap_or("unknown error")
                );
            }
            let elapsed = started.elapsed();
            let mib_per_second = size as f64 / 1_048_576.0 / elapsed.as_secs_f64();
            println!(
                "status={:?} size={size} queued_ms={} total_ms={} throughput_mib_s={mib_per_second:.2}",
                transfer.status,
                queued.as_millis(),
                elapsed.as_millis()
            );
            return Ok(());
        }
        Some("send") => {
            let fingerprint = arguments
                .next()
                .ok_or_else(|| anyhow::anyhow!("usage: rpc_probe send fingerprint path [pin]"))?;
            let path = arguments
                .next()
                .ok_or_else(|| anyhow::anyhow!("usage: rpc_probe send fingerprint path [pin]"))?;
            let pin = arguments.next();
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
                    pin,
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

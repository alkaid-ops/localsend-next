use localsend_rpc::{
    DeviceView, FavoriteView, HistoryEntry, ServiceSnapshot, TransferDirection, TransferStatus,
    TransferView, unix_ms_now,
};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceRow {
    pub alias: String,
    pub ip: String,
    pub fingerprint: String,
    pub protocol: String,
    pub model: String,
    pub device_type: String,
    pub favorite: bool,
    pub selected: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FavoriteRow {
    pub alias: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryRow {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    pub path: String,
    pub message: String,
    pub is_message: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestFileRow {
    pub file_id: String,
    pub name: String,
    pub size: String,
    pub selected: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferRow {
    pub transfer_id: String,
    pub peer: String,
    pub summary: String,
    pub status: String,
    pub direction: String,
    pub terminal: bool,
    pub progress_percent: i32,
    pub file_count: i32,
    pub completion_countdown: i32,
    pub files: Vec<TransferFileRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferFileRow {
    pub name: String,
    pub size: String,
    pub is_text: bool,
}

pub fn devices(
    snapshot: &ServiceSnapshot,
    selected_fingerprints: &HashSet<String>,
) -> Vec<DeviceRow> {
    let favorite_fingerprints: HashSet<&str> = snapshot
        .favorites
        .iter()
        .map(|favorite| favorite.device.fingerprint.as_str())
        .collect();
    let mut rows: Vec<_> = snapshot
        .devices
        .iter()
        .filter(|device| device.source != "local" && device.device.ip.as_deref() != Some("0.0.0.0"))
        .map(|device| device_row(device, &favorite_fingerprints, selected_fingerprints))
        .collect();
    for favorite in &snapshot.favorites {
        if !rows
            .iter()
            .any(|row| row.fingerprint == favorite.device.fingerprint)
        {
            rows.push(favorite_row_as_device(favorite, selected_fingerprints));
        }
    }
    rows.sort_by(|left, right| {
        right
            .favorite
            .cmp(&left.favorite)
            .then_with(|| left.alias.to_lowercase().cmp(&right.alias.to_lowercase()))
            .then_with(|| left.ip.cmp(&right.ip))
    });
    rows
}

fn device_row(
    view: &DeviceView,
    favorites: &HashSet<&str>,
    selected: &HashSet<String>,
) -> DeviceRow {
    let device = &view.device;
    DeviceRow {
        alias: device.alias.clone(),
        ip: device.ip.clone().unwrap_or_else(|| "?".to_string()),
        fingerprint: device.fingerprint.clone(),
        protocol: device.protocol.to_string(),
        model: device.device_model.clone().unwrap_or_default(),
        device_type: format!("{:?}", device.device_type.unwrap_or_default()),
        favorite: favorites.contains(device.fingerprint.as_str()),
        selected: selected.contains(&device.fingerprint),
    }
}

fn favorite_row_as_device(favorite: &FavoriteView, selected: &HashSet<String>) -> DeviceRow {
    DeviceRow {
        alias: favorite.alias.clone(),
        ip: favorite
            .device
            .ip
            .clone()
            .unwrap_or_else(|| "?".to_string()),
        fingerprint: favorite.device.fingerprint.clone(),
        protocol: favorite.device.protocol.to_string(),
        model: favorite.device.device_model.clone().unwrap_or_default(),
        device_type: format!("{:?}", favorite.device.device_type.unwrap_or_default()),
        favorite: true,
        selected: selected.contains(&favorite.device.fingerprint),
    }
}

pub fn favorites(snapshot: &ServiceSnapshot) -> Vec<FavoriteRow> {
    snapshot
        .favorites
        .iter()
        .map(|favorite| FavoriteRow {
            alias: favorite.alias.clone(),
            fingerprint: favorite.device.fingerprint.clone(),
        })
        .collect()
}

pub fn history(snapshot: &ServiceSnapshot) -> Vec<HistoryRow> {
    history_with_language(snapshot, false)
}

pub fn history_with_language(snapshot: &ServiceSnapshot, english: bool) -> Vec<HistoryRow> {
    snapshot
        .history
        .iter()
        .rev()
        .map(|entry| history_entry(entry, english))
        .collect()
}

fn history_entry(entry: &HistoryEntry, english: bool) -> HistoryRow {
    let is_message = entry.message.is_some();
    HistoryRow {
        id: entry.id.clone(),
        name: if is_message {
            if english {
                "Text message".to_string()
            } else {
                "文本消息".to_string()
            }
        } else {
            entry.file_name.clone()
        },
        subtitle: format!(
            "{} · {} · {} · {}",
            if entry.direction == TransferDirection::Receive {
                if english { "Received" } else { "接收" }
            } else {
                if english { "Sent" } else { "发送" }
            },
            entry.peer_alias,
            format_bytes(entry.size),
            transfer_status_text(entry.status, english)
        ),
        path: entry
            .path
            .as_deref()
            .map(|path| path.display().to_string())
            .unwrap_or_default(),
        message: entry.message.clone().unwrap_or_default(),
        is_message,
    }
}

pub fn request_files(
    request: &localsend_rpc::TransferRequestView,
    selected: &HashSet<String>,
) -> Vec<RequestFileRow> {
    request
        .files
        .iter()
        .map(|file| RequestFileRow {
            file_id: file.id.to_string(),
            name: file.file_name.clone(),
            size: format_bytes(file.size),
            selected: selected.contains(file.id.as_str()),
        })
        .collect()
}

pub fn transfers(snapshot: &ServiceSnapshot) -> Vec<TransferRow> {
    transfers_with_language(snapshot, false)
}

pub fn transfers_with_language(snapshot: &ServiceSnapshot, english: bool) -> Vec<TransferRow> {
    snapshot
        .active_transfers
        .iter()
        .map(|transfer| transfer_row(transfer, english, snapshot.settings.auto_finish))
        .collect()
}

fn transfer_row(item: &TransferView, english: bool, auto_finish: bool) -> TransferRow {
    let progress_percent = item
        .bytes_done
        .saturating_mul(100)
        .checked_div(item.bytes_total)
        .unwrap_or(0)
        .min(100) as i32;
    let completion_countdown = if auto_finish
        && matches!(
            item.status,
            TransferStatus::Finished | TransferStatus::Failed | TransferStatus::Cancelled
        )
        && let Some(ended_at) = item.ended_at_ms
    {
        let elapsed = unix_ms_now().saturating_sub(ended_at);
        let remaining_ms = 3_000_u64.saturating_sub(elapsed);
        remaining_ms.div_ceil(1_000).min(3) as i32
    } else {
        0
    };
    TransferRow {
        transfer_id: item.transfer_id.clone(),
        peer: item.peer_alias.clone(),
        summary: format!(
            "{} / {}",
            format_bytes(item.bytes_done),
            format_bytes(item.bytes_total)
        ),
        status: transfer_status_text(item.status, english).to_string(),
        direction: if item.direction == TransferDirection::Receive {
            if english {
                "Receive".to_string()
            } else {
                "接收".to_string()
            }
        } else {
            if english {
                "Send".to_string()
            } else {
                "发送".to_string()
            }
        },
        terminal: matches!(
            item.status,
            TransferStatus::Finished | TransferStatus::Failed | TransferStatus::Cancelled
        ),
        progress_percent,
        file_count: item.files.len().min(i32::MAX as usize) as i32,
        completion_countdown,
        files: item
            .files
            .iter()
            .map(|file| TransferFileRow {
                name: file.file_name.clone(),
                size: format_bytes(file.size),
                is_text: file.file_type == "text" || file.file_type.starts_with("text/"),
            })
            .collect(),
    }
}

pub fn format_bytes(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = KIB * 1024;
    const GIB: u64 = MIB * 1024;
    if bytes >= GIB {
        format!("{:.1} GiB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.1} MiB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.1} KiB", bytes as f64 / KIB as f64)
    } else {
        format!("{bytes} B")
    }
}

pub fn transfer_status_label(status: TransferStatus) -> &'static str {
    match status {
        TransferStatus::Waiting => "等待中",
        TransferStatus::Transferring => "传输中",
        TransferStatus::Finished => "已完成",
        TransferStatus::Failed => "失败",
        TransferStatus::Cancelled => "已取消",
    }
}

fn transfer_status_text(status: TransferStatus, english: bool) -> &'static str {
    if english {
        match status {
            TransferStatus::Waiting => "Waiting",
            TransferStatus::Transferring => "Transferring",
            TransferStatus::Finished => "Completed",
            TransferStatus::Failed => "Failed",
            TransferStatus::Cancelled => "Cancelled",
        }
    } else {
        transfer_status_label(status)
    }
}

#[cfg(test)]
pub fn should_show_pin_prompt(kind: Option<localsend_rpc::ServiceErrorKind>) -> bool {
    matches!(
        kind,
        Some(
            localsend_rpc::ServiceErrorKind::InvalidPin
                | localsend_rpc::ServiceErrorKind::PinRequired
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use localsend_rpc::{DesktopSettings, FileView, ServiceErrorKind, TransferRequestView};
    use localsend_rpc::{DeviceInfo, DeviceType, Protocol};

    fn device(alias: &str, fingerprint: &str, ip: Option<&str>) -> DeviceInfo {
        DeviceInfo {
            alias: alias.to_string(),
            version: "2.1".to_string(),
            device_model: Some("Test model".to_string()),
            device_type: Some(DeviceType::Desktop),
            fingerprint: fingerprint.to_string(),
            port: 53317,
            protocol: Protocol::Https,
            download: false,
            ip: ip.map(str::to_string),
        }
    }

    fn snapshot() -> ServiceSnapshot {
        ServiceSnapshot {
            daemon_pid: 1,
            daemon_instance_id: "test-instance".into(),
            revision: 0,
            last_error: None,
            server_running: false,
            server_alias: None,
            server_port: None,
            server_protocol: None,
            devices: Vec::new(),
            incoming_requests: Vec::new(),
            active_transfers: Vec::new(),
            history: Vec::new(),
            favorites: Vec::new(),
            settings: DesktopSettings::default(),
            last_event_seq: 0,
        }
    }

    #[test]
    fn progress_and_zero_total_are_safe() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1024), "1.0 KiB");
        let mut item = TransferView {
            transfer_id: "id".into(),
            direction: TransferDirection::Send,
            peer_alias: "peer".into(),
            peer_fingerprint: "fp".into(),
            files: Vec::new(),
            status: TransferStatus::Transferring,
            bytes_done: 1,
            bytes_total: 0,
            started_at_ms: 0,
            ended_at_ms: None,
            error: None,
        };
        assert_eq!(transfer_row(&item, false, true).progress_percent, 0);
        item.bytes_total = 4;
        item.bytes_done = 3;
        assert_eq!(transfer_row(&item, false, true).progress_percent, 75);
        let english = transfer_row(&item, true, true);
        assert_eq!(english.status, "Transferring");
        assert_eq!(english.direction, "Send");

        item.status = TransferStatus::Failed;
        item.ended_at_ms = Some(unix_ms_now());
        let failed = transfer_row(&item, false, true);
        assert_eq!(failed.completion_countdown, 3);
    }

    #[test]
    fn pin_prompt_only_matches_authentication_errors() {
        assert!(should_show_pin_prompt(Some(ServiceErrorKind::InvalidPin)));
        assert!(should_show_pin_prompt(Some(ServiceErrorKind::PinRequired)));
        assert!(!should_show_pin_prompt(Some(ServiceErrorKind::Rejected)));
        assert!(!should_show_pin_prompt(None));
    }

    #[test]
    fn empty_snapshot_converts_to_empty_ui_models() {
        let snapshot = snapshot();
        assert!(devices(&snapshot, &HashSet::new()).is_empty());
        assert!(favorites(&snapshot).is_empty());
        assert!(history(&snapshot).is_empty());
        assert!(transfers(&snapshot).is_empty());
    }

    #[test]
    fn device_rows_exclude_local_device_and_keep_offline_favorites() {
        let mut snapshot = snapshot();
        snapshot.devices = vec![
            DeviceView {
                device: device("Local", "local", Some("0.0.0.0")),
                last_seen_ms: 0,
                source: "local".to_string(),
            },
            DeviceView {
                device: device("Peer", "peer", Some("192.168.1.20")),
                last_seen_ms: 1,
                source: "http".to_string(),
            },
        ];
        snapshot.favorites.push(FavoriteView {
            device: device("Old peer", "offline", None),
            alias: "Saved alias".to_string(),
            added_at_ms: 0,
        });

        let selected = HashSet::from(["offline".to_string()]);
        let rows = devices(&snapshot, &selected);

        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|row| row.fingerprint != "local"));
        let offline = rows
            .iter()
            .find(|row| row.fingerprint == "offline")
            .unwrap();
        assert_eq!(offline.alias, "Saved alias");
        assert_eq!(offline.ip, "?");
        assert!(offline.favorite);
        assert!(offline.selected);
    }

    #[test]
    fn history_and_request_models_preserve_message_and_selection() {
        let mut snapshot = snapshot();
        snapshot.history.push(HistoryEntry {
            id: "history-1".to_string(),
            file_id: None,
            direction: TransferDirection::Receive,
            peer_alias: "Sender".to_string(),
            file_name: "message.txt".to_string(),
            path: None,
            size: 12,
            timestamp_ms: 1,
            status: TransferStatus::Finished,
            message: Some("hello world!".to_string()),
        });
        let history = history(&snapshot);
        assert_eq!(history[0].message, "hello world!");
        assert!(history[0].is_message);
        assert_eq!(history[0].name, "文本消息");
        let english_history = history_with_language(&snapshot, true);
        assert_eq!(english_history[0].name, "Text message");
        assert!(english_history[0].subtitle.contains("Received"));
        assert!(english_history[0].subtitle.contains("Completed"));

        let request = TransferRequestView {
            request_id: "request-1".to_string(),
            sender: device("Sender", "sender", Some("192.168.1.21")),
            files: vec![FileView {
                id: "file-1".to_string(),
                file_name: "document.pdf".to_string(),
                size: 2048,
                file_type: "application/pdf".to_string(),
                sha256: None,
                preview: None,
            }],
            received_at_ms: 1,
        };
        let selected = HashSet::from(["file-1".to_string()]);
        let files = request_files(&request, &selected);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "document.pdf");
        assert_eq!(files[0].size, "2.0 KiB");
        assert!(files[0].selected);
    }
}

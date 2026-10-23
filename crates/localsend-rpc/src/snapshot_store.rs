use crate::types::{
    DeviceView, FavoriteView, HistoryEntry, SequencedEvent, ServiceEvent, ServiceSnapshot,
    TransferRequestView, TransferStatus, TransferView,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventApply {
    Applied,
    Duplicate,
    Gap,
}

/// Frontend-side materialized state. The daemon snapshot establishes the
/// baseline; sequenced events keep it current until a gap requires recovery.
#[derive(Debug, Clone)]
pub struct SnapshotStore {
    snapshot: ServiceSnapshot,
    last_seq: u64,
}

impl SnapshotStore {
    pub fn new(snapshot: ServiceSnapshot) -> Self {
        let last_seq = snapshot.last_event_seq;
        Self { snapshot, last_seq }
    }

    pub fn snapshot(&self) -> &ServiceSnapshot {
        &self.snapshot
    }

    pub fn replace(&mut self, snapshot: ServiceSnapshot) {
        self.last_seq = snapshot.last_event_seq;
        self.snapshot = snapshot;
    }

    pub fn apply(&mut self, sequenced: &SequencedEvent) -> EventApply {
        if let ServiceEvent::Snapshot { snapshot } = &sequenced.event {
            if snapshot.last_event_seq < self.last_seq || snapshot.revision < self.snapshot.revision
            {
                return EventApply::Duplicate;
            }
            let changed = snapshot != &self.snapshot;
            self.replace(snapshot.clone());
            return if changed {
                EventApply::Applied
            } else {
                EventApply::Duplicate
            };
        }
        if sequenced.seq <= self.last_seq {
            return EventApply::Duplicate;
        }
        if sequenced.seq != self.last_seq.saturating_add(1) {
            return EventApply::Gap;
        }

        match &sequenced.event {
            ServiceEvent::Snapshot { .. } => unreachable!("snapshot events are handled above"),
            ServiceEvent::ServerState {
                running,
                alias,
                port,
                protocol,
            } => {
                self.snapshot.server_running = *running;
                self.snapshot.server_alias = alias.clone();
                self.snapshot.server_port = *port;
                self.snapshot.server_protocol = *protocol;
            }
            ServiceEvent::DeviceUpsert { device } => {
                upsert_device(&mut self.snapshot.devices, device)
            }
            ServiceEvent::DeviceRemoved { fingerprint } => self
                .snapshot
                .devices
                .retain(|item| item.device.fingerprint != *fingerprint),
            ServiceEvent::FavoriteUpsert { favorite } => {
                upsert_favorite(&mut self.snapshot.favorites, favorite)
            }
            ServiceEvent::FavoriteRemoved { fingerprint } => self
                .snapshot
                .favorites
                .retain(|item| item.device.fingerprint != *fingerprint),
            ServiceEvent::TransferRequested { request } => {
                upsert_request(&mut self.snapshot.incoming_requests, request)
            }
            ServiceEvent::TransferPrepared { transfer, .. } => {
                self.snapshot
                    .incoming_requests
                    .retain(|request| request.request_id != transfer.transfer_id);
                upsert_transfer(&mut self.snapshot.active_transfers, transfer);
            }
            ServiceEvent::TransferProgress {
                transfer_id,
                bytes_done,
                bytes_total,
                transfer_bytes_done,
                transfer_bytes_total,
                ..
            } => {
                if let Some(transfer) = self
                    .snapshot
                    .active_transfers
                    .iter_mut()
                    .find(|item| item.transfer_id == *transfer_id)
                {
                    transfer.bytes_total = transfer_bytes_total.unwrap_or(*bytes_total);
                    transfer.bytes_done = transfer
                        .bytes_done
                        .max(transfer_bytes_done.unwrap_or(*bytes_done))
                        .min(transfer.bytes_total);
                    transfer.status = TransferStatus::Transferring;
                }
            }
            ServiceEvent::TransferFinished { transfer } => {
                self.snapshot
                    .incoming_requests
                    .retain(|request| request.request_id != transfer.transfer_id);
                upsert_transfer(&mut self.snapshot.active_transfers, transfer);
            }
            ServiceEvent::TransferFailed {
                transfer_id, error, ..
            } => {
                if let Some(transfer) = self
                    .snapshot
                    .active_transfers
                    .iter_mut()
                    .find(|item| item.transfer_id == *transfer_id)
                {
                    transfer.status = TransferStatus::Failed;
                    transfer.error = Some(error.clone());
                    transfer.ended_at_ms = Some(localsend_rpc_unix_ms());
                }
                self.snapshot
                    .incoming_requests
                    .retain(|request| request.request_id != *transfer_id);
            }
            ServiceEvent::HistoryAppended { entry } => append_history(&mut self.snapshot, entry),
            ServiceEvent::SettingsChanged { settings } => {
                self.snapshot.settings = settings.clone();
            }
            ServiceEvent::Error { message } => self.snapshot.last_error = Some(message.clone()),
            ServiceEvent::TextReceived { .. }
            | ServiceEvent::WebShareRequested { .. }
            | ServiceEvent::WebShareProgress { .. }
            | ServiceEvent::WebShareDone { .. }
            | ServiceEvent::SendOfferAnswered { .. }
            | ServiceEvent::DaemonStopping => {}
        }

        self.last_seq = sequenced.seq;
        self.snapshot.last_event_seq = sequenced.seq;
        self.snapshot.revision = self.snapshot.revision.saturating_add(1);
        EventApply::Applied
    }
}

fn upsert_device(items: &mut Vec<DeviceView>, device: &DeviceView) {
    if let Some(current) = items
        .iter_mut()
        .find(|item| item.device.fingerprint == device.device.fingerprint)
    {
        *current = device.clone();
    } else {
        items.push(device.clone());
    }
}

fn upsert_favorite(items: &mut Vec<FavoriteView>, favorite: &FavoriteView) {
    if let Some(current) = items
        .iter_mut()
        .find(|item| item.device.fingerprint == favorite.device.fingerprint)
    {
        *current = favorite.clone();
    } else {
        items.push(favorite.clone());
    }
}

fn upsert_request(items: &mut Vec<TransferRequestView>, request: &TransferRequestView) {
    if let Some(current) = items
        .iter_mut()
        .find(|item| item.request_id == request.request_id)
    {
        *current = request.clone();
    } else {
        items.push(request.clone());
    }
}

fn upsert_transfer(items: &mut Vec<TransferView>, transfer: &TransferView) {
    if let Some(current) = items
        .iter_mut()
        .find(|item| item.transfer_id == transfer.transfer_id)
    {
        *current = transfer.clone();
    } else {
        items.push(transfer.clone());
    }
}

fn append_history(snapshot: &mut ServiceSnapshot, entry: &HistoryEntry) {
    if snapshot
        .history
        .iter()
        .any(|current| current.id == entry.id)
    {
        return;
    }
    snapshot.history.push(entry.clone());
    let limit = snapshot.settings.history_limit;
    if snapshot.history.len() > limit {
        let excess = snapshot.history.len() - limit;
        snapshot.history.drain(..excess);
    }
}

fn localsend_rpc_unix_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DesktopSettings, ServiceEvent, TransferDirection, TransferView};

    fn snapshot_with_transfer(transfer_id: &str, bytes_total: u64) -> ServiceSnapshot {
        ServiceSnapshot {
            daemon_pid: 1,
            daemon_instance_id: "test-daemon".to_string(),
            revision: 1,
            last_error: None,
            server_running: true,
            server_alias: Some("test".to_string()),
            server_port: Some(53317),
            server_protocol: None,
            devices: Vec::new(),
            incoming_requests: Vec::new(),
            active_transfers: vec![TransferView {
                transfer_id: transfer_id.to_string(),
                direction: TransferDirection::Send,
                peer_alias: "peer".to_string(),
                peer_fingerprint: "peer-fingerprint".to_string(),
                files: Vec::new(),
                status: TransferStatus::Waiting,
                bytes_done: 0,
                bytes_total,
                started_at_ms: 1,
                ended_at_ms: None,
                error: None,
            }],
            history: Vec::new(),
            favorites: Vec::new(),
            settings: DesktopSettings::default(),
            last_event_seq: 0,
        }
    }

    fn progress_event(seq: u64, bytes_done: u64, bytes_total: u64) -> SequencedEvent {
        SequencedEvent {
            seq,
            event: ServiceEvent::TransferProgress {
                transfer_id: "transfer-1".to_string(),
                file_id: None,
                file_name: None,
                bytes_done,
                bytes_total,
                transfer_bytes_done: Some(bytes_done),
                transfer_bytes_total: Some(bytes_total),
            },
        }
    }

    #[test]
    fn transfer_progress_is_monotonic_and_clamped_to_total() {
        let mut store = SnapshotStore::new(snapshot_with_transfer("transfer-1", 100));

        assert_eq!(
            store.apply(&progress_event(1, 40, 100)),
            EventApply::Applied
        );
        assert_eq!(
            store.apply(&progress_event(2, 25, 100)),
            EventApply::Applied
        );
        assert_eq!(
            store.apply(&progress_event(3, 130, 100)),
            EventApply::Applied
        );

        let transfer = &store.snapshot().active_transfers[0];
        assert_eq!(transfer.bytes_done, 100);
        assert_eq!(transfer.bytes_total, 100);
        assert_eq!(transfer.status, TransferStatus::Transferring);
    }
}

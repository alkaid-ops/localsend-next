use crate::framing::{read_json, write_json};
use crate::transport;
use crate::types::{
    CommandResult, DaemonLifecycle, RPC_VERSION, RpcClientMessage, RpcServerMessage,
    SequencedEvent, ServiceCommand, ServiceEvent, ServiceSnapshot, SubscriptionMode,
};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::{broadcast, mpsc, oneshot, watch};
use tokio::time::{Duration, timeout};

type PendingResponses =
    Arc<parking_lot::Mutex<HashMap<u64, oneshot::Sender<Result<CommandResult, String>>>>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DaemonConnectionInfo {
    pub daemon_pid: u32,
    pub instance_id: String,
    pub lifecycle: DaemonLifecycle,
    pub owner_pid: Option<u32>,
}

struct ConnectionShutdown {
    tx: watch::Sender<bool>,
}

impl Drop for ConnectionShutdown {
    fn drop(&mut self) {
        self.tx.send_replace(true);
    }
}

#[derive(Clone)]
pub struct DaemonClient {
    tx: mpsc::UnboundedSender<RpcClientMessage>,
    pending: PendingResponses,
    events: Arc<parking_lot::Mutex<Option<broadcast::Sender<SequencedEvent>>>>,
    first_event_receiver: Arc<parking_lot::Mutex<Option<broadcast::Receiver<SequencedEvent>>>>,
    connection_closed: watch::Receiver<bool>,
    connection_info: Arc<RwLock<Option<DaemonConnectionInfo>>>,
    _shutdown: Arc<ConnectionShutdown>,
    snapshot: Arc<RwLock<Option<ServiceSnapshot>>>,
    next_id: Arc<AtomicU64>,
}

impl DaemonClient {
    pub async fn connect() -> anyhow::Result<Self> {
        Self::connect_with_role(false, false).await
    }

    pub async fn connect_interactive() -> anyhow::Result<Self> {
        Self::connect_with_role(true, false).await
    }

    pub async fn connect_owned() -> anyhow::Result<Self> {
        Self::connect_with_role(false, true).await
    }

    pub async fn connect_owned_interactive() -> anyhow::Result<Self> {
        Self::connect_with_role(true, true).await
    }

    pub async fn connect_notifications() -> anyhow::Result<Self> {
        Self::connect_with_subscription(false, false, SubscriptionMode::Notifications).await
    }

    pub async fn connect_owned_notifications() -> anyhow::Result<Self> {
        Self::connect_with_subscription(false, true, SubscriptionMode::Notifications).await
    }

    pub async fn connect_to(endpoint: impl AsRef<str>) -> anyhow::Result<Self> {
        Self::connect_to_with_role(endpoint.as_ref(), false, false).await
    }

    pub async fn connect_to_interactive(endpoint: impl AsRef<str>) -> anyhow::Result<Self> {
        Self::connect_to_with_role(endpoint.as_ref(), true, false).await
    }

    pub async fn connect_to_owned(endpoint: impl AsRef<str>) -> anyhow::Result<Self> {
        Self::connect_to_with_role(endpoint.as_ref(), false, true).await
    }

    pub async fn connect_to_notifications(endpoint: impl AsRef<str>) -> anyhow::Result<Self> {
        Self::connect_to_with_subscription(
            endpoint.as_ref(),
            false,
            false,
            SubscriptionMode::Notifications,
        )
        .await
    }

    async fn connect_with_role(interactive: bool, lifecycle_owner: bool) -> anyhow::Result<Self> {
        Self::connect_with_subscription(interactive, lifecycle_owner, SubscriptionMode::Full).await
    }

    async fn connect_with_subscription(
        interactive: bool,
        lifecycle_owner: bool,
        subscription: SubscriptionMode,
    ) -> anyhow::Result<Self> {
        let connection = transport::connect().await?;
        Self::from_connection(connection, interactive, lifecycle_owner, subscription).await
    }

    async fn connect_to_with_role(
        endpoint: &str,
        interactive: bool,
        lifecycle_owner: bool,
    ) -> anyhow::Result<Self> {
        let connection = transport::connect_to(endpoint).await?;
        Self::from_connection(
            connection,
            interactive,
            lifecycle_owner,
            SubscriptionMode::Full,
        )
        .await
    }

    async fn connect_to_with_subscription(
        endpoint: &str,
        interactive: bool,
        lifecycle_owner: bool,
        subscription: SubscriptionMode,
    ) -> anyhow::Result<Self> {
        let connection = transport::connect_to(endpoint).await?;
        Self::from_connection(connection, interactive, lifecycle_owner, subscription).await
    }

    async fn from_connection(
        connection: transport::BoxedIo,
        interactive: bool,
        lifecycle_owner: bool,
        subscription: SubscriptionMode,
    ) -> anyhow::Result<Self> {
        let (mut reader, mut writer) = tokio::io::split(connection);
        let (tx, mut outbound) = mpsc::unbounded_channel::<RpcClientMessage>();
        let (event_sender, first_event_receiver) = broadcast::channel(1024);
        let first_event_receiver = Arc::new(parking_lot::Mutex::new(Some(first_event_receiver)));
        let events = Arc::new(parking_lot::Mutex::new(Some(event_sender)));
        let (connection_closed_tx, connection_closed) = watch::channel(false);
        let (shutdown_tx, shutdown_rx) = watch::channel(false);
        let shutdown = Arc::new(ConnectionShutdown { tx: shutdown_tx });
        let connection_info = Arc::new(RwLock::new(None));
        let snapshot = Arc::new(RwLock::new(None));
        let pending = Arc::new(parking_lot::Mutex::new(HashMap::<
            u64,
            oneshot::Sender<Result<CommandResult, String>>,
        >::new()));
        let (ready_tx, ready_rx) = oneshot::channel();
        let reader_subscription = subscription;
        let reader_snapshot = snapshot.clone();
        let reader_pending = pending.clone();
        let reader_events = events.clone();
        let reader_connection_info = connection_info.clone();
        let mut reader_shutdown = shutdown_rx.clone();

        tokio::spawn(async move {
            let mut ready_sender = Some(ready_tx);
            loop {
                let incoming = tokio::select! {
                    changed = reader_shutdown.changed() => {
                        if changed.is_err() || *reader_shutdown.borrow() {
                            None
                        } else {
                            continue;
                        }
                    }
                    incoming = read_json::<RpcServerMessage, _>(&mut reader) => Some(incoming),
                };
                let Some(incoming) = incoming else {
                    break;
                };
                match incoming {
                    Ok(Some(RpcServerMessage::Hello {
                        daemon_pid,
                        instance_id,
                        lifecycle,
                        owner_pid,
                        ..
                    })) => {
                        *reader_connection_info.write() = Some(DaemonConnectionInfo {
                            daemon_pid,
                            instance_id,
                            lifecycle,
                            owner_pid,
                        });
                        if reader_subscription == SubscriptionMode::Notifications
                            && let Some(sender) = ready_sender.take()
                        {
                            let _ = sender.send(());
                        }
                    }
                    Ok(Some(RpcServerMessage::Snapshot {
                        snapshot: current, ..
                    })) => {
                        *reader_snapshot.write() = Some(current.clone());
                        if let Some(sender) = ready_sender.take() {
                            let _ = sender.send(());
                        } else if reader_subscription == SubscriptionMode::Full
                            && let Some(sender) = reader_events.lock().as_ref()
                        {
                            let _ = sender.send(SequencedEvent {
                                seq: current.last_event_seq,
                                event: ServiceEvent::Snapshot { snapshot: current },
                            });
                        }
                    }
                    Ok(Some(RpcServerMessage::Response { id, result })) => {
                        if let Some(sender) = reader_pending.lock().remove(&id) {
                            let _ = sender.send(result);
                        }
                    }
                    Ok(Some(RpcServerMessage::Event { event })) => {
                        if let Some(sender) = reader_events.lock().as_ref() {
                            let _ = sender.send(event);
                        }
                    }
                    Ok(Some(RpcServerMessage::Error { message })) => {
                        tracing::warn!("desktop RPC error: {message}");
                    }
                    Ok(None) => break,
                    Err(error) => {
                        tracing::debug!("desktop RPC reader stopped: {error}");
                        break;
                    }
                }
            }
            drop(ready_sender.take());
            for (_, sender) in reader_pending.lock().drain() {
                let _ = sender.send(Err("daemon connection is closed".to_string()));
            }
            reader_events.lock().take();
            connection_closed_tx.send_replace(true);
        });

        let client = Self {
            tx: tx.clone(),
            pending,
            events,
            first_event_receiver,
            connection_closed,
            connection_info,
            _shutdown: shutdown.clone(),
            snapshot,
            next_id: Arc::new(AtomicU64::new(1)),
        };
        tx.send(RpcClientMessage::Hello {
            version: RPC_VERSION,
            client_name: "localsend-desktop-client".to_string(),
            resume_from: None,
            interactive,
            subscription,
            lifecycle_owner,
            client_pid: std::process::id(),
        })?;

        tokio::spawn(async move {
            let mut shutdown = shutdown_rx;
            loop {
                let message = tokio::select! {
                    changed = shutdown.changed() => {
                        if changed.is_err() || *shutdown.borrow() {
                            None
                        } else {
                            continue;
                        }
                    }
                    message = outbound.recv() => message,
                };
                let Some(message) = message else {
                    break;
                };
                if let Err(error) = write_json(&mut writer, &message).await {
                    tracing::debug!("desktop RPC writer stopped: {error}");
                    break;
                }
            }
        });

        timeout(Duration::from_secs(5), ready_rx)
            .await
            .map_err(|_| anyhow::anyhow!("timed out waiting for daemon subscription"))?
            .map_err(|_| anyhow::anyhow!("daemon closed before confirming the subscription"))?;
        Ok(client)
    }

    pub fn snapshot(&self) -> Option<ServiceSnapshot> {
        self.snapshot.read().clone()
    }

    pub fn connection_info(&self) -> Option<DaemonConnectionInfo> {
        self.connection_info.read().clone()
    }

    pub fn instance_id(&self) -> Option<String> {
        self.connection_info()
            .map(|connection| connection.instance_id)
    }

    pub fn is_connection_closed(&self) -> bool {
        *self.connection_closed.borrow()
    }

    pub fn owns_lifecycle(&self) -> bool {
        self.connection_info()
            .is_some_and(|connection| connection.owner_pid == Some(std::process::id()))
    }

    pub async fn wait_closed(&self) {
        let mut closed = self.connection_closed.clone();
        if !*closed.borrow() {
            let _ = closed.changed().await;
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<SequencedEvent> {
        if let Some(receiver) = self.first_event_receiver.lock().take() {
            return receiver;
        }
        if let Some(sender) = self.events.lock().as_ref() {
            sender.subscribe()
        } else {
            let (sender, receiver) = broadcast::channel(1);
            drop(sender);
            receiver
        }
    }

    pub async fn command(&self, command: ServiceCommand) -> Result<CommandResult, String> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let (reply, response) = oneshot::channel();
        self.pending.lock().insert(id, reply);
        self.tx
            .send(RpcClientMessage::Command { id, command })
            .map_err(|_| "daemon connection is closed".to_string())?;
        let mut connection_closed = self.connection_closed.clone();
        tokio::select! {
            response = timeout(Duration::from_secs(60), response) => {
                response
                    .map_err(|_| "daemon command timed out".to_string())?
                    .map_err(|_| "daemon stopped before answering".to_string())?
            }
            _ = async {
                if !*connection_closed.borrow() {
                    let _ = connection_closed.changed().await;
                }
            } => Err("daemon connection is closed".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framing::{read_json, write_json};
    use crate::types::{DaemonLifecycle, RpcServerMessage, ServiceSnapshot};
    use serde_json::json;
    use tokio::io::AsyncWriteExt;

    async fn mock_handshake(
        mut reader: impl tokio::io::AsyncRead + Unpin,
        mut writer: impl tokio::io::AsyncWrite + Unpin,
    ) {
        let hello = read_json::<RpcClientMessage, _>(&mut reader)
            .await
            .unwrap()
            .unwrap();
        assert!(matches!(hello, RpcClientMessage::Hello { .. }));
        write_json(
            &mut writer,
            &RpcServerMessage::Hello {
                version: RPC_VERSION,
                daemon_pid: 1,
                instance_id: "test-instance".into(),
                lifecycle: DaemonLifecycle::Persistent,
                owner_pid: None,
            },
        )
        .await
        .unwrap();
        let snapshot: ServiceSnapshot = serde_json::from_value(json!({
            "daemon_pid": 1,
            "daemon_instance_id": "test-instance",
            "revision": 0,
            "last_error": null,
            "server_running": false,
            "server_alias": null,
            "server_port": null,
            "server_protocol": null,
            "devices": [],
            "incoming_requests": [],
            "active_transfers": [],
            "history": [],
            "favorites": [],
            "settings": {},
            "last_event_seq": 0
        }))
        .unwrap();
        write_json(
            &mut writer,
            &RpcServerMessage::Snapshot {
                snapshot,
                last_event_seq: 0,
            },
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn connection_loss_closes_event_stream_and_fails_pending_commands() {
        let (client_io, server_io) = tokio::io::duplex(16 * 1024);
        let (mut server_reader, mut server_writer) = tokio::io::split(server_io);
        let server = tokio::spawn(async move {
            mock_handshake(&mut server_reader, &mut server_writer).await;
            let command = read_json::<RpcClientMessage, _>(&mut server_reader)
                .await
                .unwrap()
                .unwrap();
            assert!(matches!(command, RpcClientMessage::Command { .. }));
            server_writer.shutdown().await.unwrap();
        });

        let client = DaemonClient::from_connection(
            Box::new(client_io),
            false,
            false,
            SubscriptionMode::Full,
        )
        .await
        .unwrap();
        let mut events = client.subscribe();
        let command = tokio::time::timeout(
            Duration::from_secs(1),
            client.command(ServiceCommand::GetSnapshot),
        )
        .await
        .expect("a disconnected daemon must not leave commands waiting for the 60-second timeout");
        assert!(command.unwrap_err().contains("connection is closed"));
        assert!(matches!(
            tokio::time::timeout(Duration::from_secs(1), events.recv())
                .await
                .unwrap(),
            Err(broadcast::error::RecvError::Closed)
        ));
        server.await.unwrap();
    }
}

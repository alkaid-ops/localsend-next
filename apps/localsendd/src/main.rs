use localsend_platform::{InstanceLock, scoped_lock_name};
use localsend_rpc::framing::{read_json, write_json};
use localsend_rpc::transport::{self, BoxedIo, LocalListener};
use localsend_rpc::{
    CommandResult, DaemonLifecycle, RPC_VERSION, RpcClientMessage, RpcServerMessage, ServiceCommand,
};
use localsend_service::ServiceHandle;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex as StdMutex};
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::{Mutex, mpsc};

const OWNER_START_TIMEOUT: Duration = Duration::from_secs(3);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();
    run_daemon().await
}

#[derive(Clone)]
struct LogWriter(Arc<StdMutex<File>>);

impl Write for LogWriter {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        self.0
            .lock()
            .map_err(|_| std::io::Error::other("daemon log lock poisoned"))?
            .write(buffer)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0
            .lock()
            .map_err(|_| std::io::Error::other("daemon log lock poisoned"))?
            .flush()
    }
}

impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for LogWriter {
    type Writer = Self;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

fn init_tracing() {
    let env_filter = || {
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))
    };
    if let Some(path) = daemon_log_path() {
        match OpenOptions::new().create(true).append(true).open(&path) {
            Ok(file) => {
                tracing_subscriber::fmt()
                    .with_env_filter(env_filter())
                    .with_target(false)
                    .with_ansi(false)
                    .with_writer(LogWriter(Arc::new(StdMutex::new(file))))
                    .init();
                tracing::info!(log = %path.display(), pid = std::process::id(), "localsendd log opened");
                return;
            }
            Err(error) => eprintln!("cannot open daemon log {}: {error}", path.display()),
        }
    }
    tracing_subscriber::fmt()
        .with_env_filter(env_filter())
        .with_target(false)
        .init();
}

fn daemon_log_path() -> Option<PathBuf> {
    let paths = localsend_service::service_paths().ok()?;
    let logs = paths.config_dir.join("logs");
    std::fs::create_dir_all(&logs).ok()?;
    Some(logs.join("localsendd.log"))
}

pub async fn run_daemon() -> anyhow::Result<()> {
    let require_owner = std::env::args().any(|argument| argument == "--require-owner");
    let instance_lock_name =
        scoped_lock_name("localsendd-instance", &transport::endpoint_identity());
    let Some(_instance_lock) = InstanceLock::try_acquire(&instance_lock_name)? else {
        tracing::info!("localsendd already owns {}", instance_lock_name);
        return Ok(());
    };

    let listener = LocalListener::bind().await?;
    tracing::info!("localsendd listening at {:?}", listener.path());
    let service = ServiceHandle::spawn().await.map_err(anyhow::Error::msg)?;
    let instance_id = service.snapshot().daemon_instance_id.clone();
    let (lifecycle, owner_disconnected) = LifecycleTracker::new(require_owner, instance_id);
    let _runtime_manifest = transport::write_runtime_manifest(
        &listener.path().to_string_lossy(),
        &lifecycle.instance_id,
        lifecycle.lifecycle(),
    )?;
    lifecycle.install_owner_timeout(service.clone());
    lifecycle.install_disconnect_watch(owner_disconnected, service.clone());
    let mut shutdown = service.subscribe_shutdown();

    loop {
        let connection = tokio::select! {
            changed = shutdown.changed() => {
                if changed.is_err() || *shutdown.borrow() {
                    None
                } else {
                    continue;
                }
            }
            result = listener.accept() => match result {
                Ok(connection) => Some(connection),
                Err(error) if is_accept_interrupted(&error) => continue,
                Err(error) => return Err(error.into()),
            }
        };
        let Some(connection) = connection else {
            break;
        };
        let service = service.clone();
        let lifecycle = lifecycle.clone();
        tokio::spawn(async move {
            if let Err(error) = handle_connection(connection, service, lifecycle).await {
                tracing::debug!("desktop RPC connection ended: {error}");
            }
        });
    }
    Ok(())
}

fn is_accept_interrupted(error: &std::io::Error) -> bool {
    matches!(
        error.kind(),
        std::io::ErrorKind::Interrupted | std::io::ErrorKind::ConnectionAborted
    )
}

#[derive(Clone)]
struct LifecycleTracker {
    required_owner: bool,
    instance_id: String,
    owner_claimed: Arc<AtomicBool>,
    owner_disconnected: mpsc::UnboundedSender<()>,
}

impl LifecycleTracker {
    fn new(required_owner: bool, instance_id: String) -> (Self, mpsc::UnboundedReceiver<()>) {
        let (owner_disconnected, receiver) = mpsc::unbounded_channel();
        (
            Self {
                required_owner,
                instance_id,
                owner_claimed: Arc::new(AtomicBool::new(false)),
                owner_disconnected,
            },
            receiver,
        )
    }

    fn install_owner_timeout(&self, service: ServiceHandle) {
        if !self.required_owner {
            return;
        }
        let claimed = self.owner_claimed.clone();
        tokio::spawn(async move {
            tokio::time::sleep(OWNER_START_TIMEOUT).await;
            if !claimed.load(Ordering::Acquire) {
                tracing::warn!("owned daemon did not receive an owner connection in time");
                let _ = service.command(ServiceCommand::Shutdown).await;
            }
        });
    }

    fn install_disconnect_watch(
        &self,
        receiver: mpsc::UnboundedReceiver<()>,
        service: ServiceHandle,
    ) {
        if !self.required_owner {
            return;
        }
        tokio::spawn(async move {
            let mut receiver = receiver;
            if receiver.recv().await.is_some() {
                tracing::info!("lifecycle owner disconnected; shutting down daemon");
                let _ = service.command(ServiceCommand::Shutdown).await;
            }
        });
    }

    fn claim_owner(&self) -> Result<OwnerGuard, String> {
        if !self.required_owner {
            return Err("this daemon is persistent and does not accept an owner lease".into());
        }
        self.owner_claimed
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| "this daemon already has a lifecycle owner".to_string())?;
        Ok(OwnerGuard {
            disconnected: self.owner_disconnected.clone(),
        })
    }

    fn lifecycle(&self) -> DaemonLifecycle {
        if self.required_owner {
            DaemonLifecycle::Owned
        } else {
            DaemonLifecycle::Persistent
        }
    }
}

struct OwnerGuard {
    disconnected: mpsc::UnboundedSender<()>,
}

impl Drop for OwnerGuard {
    fn drop(&mut self) {
        let _ = self.disconnected.send(());
    }
}

async fn handle_connection(
    connection: BoxedIo,
    service: ServiceHandle,
    lifecycle: LifecycleTracker,
) -> anyhow::Result<()> {
    let (mut reader, writer) = tokio::io::split(connection);
    let writer = Arc::new(Mutex::new(writer));

    let (hello, interactive, lifecycle_owner, client_pid) =
        match read_json::<RpcClientMessage, _>(&mut reader).await? {
            Some(RpcClientMessage::Hello {
                version,
                interactive,
                lifecycle_owner,
                client_pid,
                ..
            }) => (version, interactive, lifecycle_owner, client_pid),
            Some(_) => anyhow::bail!("first RPC message must be hello"),
            None => return Ok(()),
        };

    if hello != RPC_VERSION {
        write_server_message(
            &writer,
            RpcServerMessage::Error {
                message: format!("RPC version mismatch: client {hello}, daemon {RPC_VERSION}"),
            },
        )
        .await?;
        return Ok(());
    }

    let _owner_guard = if lifecycle_owner {
        match lifecycle.claim_owner() {
            Ok(guard) => Some(guard),
            Err(message) => {
                write_server_message(&writer, RpcServerMessage::Error { message }).await?;
                return Ok(());
            }
        }
    } else {
        None
    };
    let owner_pid = _owner_guard.as_ref().map(|_| client_pid);

    let interactive_count = if interactive {
        Some(service.interactive_connected())
    } else {
        None
    };
    let _interactive_guard = interactive.then(|| InteractiveGuard {
        service: service.clone(),
    });
    if interactive_count == Some(1) {
        service
            .command(ServiceCommand::SetRequestPolicy {
                frontend_answers_requests: true,
            })
            .await
            .map_err(anyhow::Error::msg)?;
    }

    write_server_message(
        &writer,
        RpcServerMessage::Hello {
            version: RPC_VERSION,
            daemon_pid: std::process::id(),
            instance_id: lifecycle.instance_id.clone(),
            lifecycle: lifecycle.lifecycle(),
            owner_pid,
        },
    )
    .await?;

    let mut events = service.subscribe();
    let snapshot = service.snapshot();
    write_server_message(
        &writer,
        RpcServerMessage::Snapshot {
            snapshot: snapshot.clone(),
            last_event_seq: snapshot.last_event_seq,
        },
    )
    .await?;

    loop {
        tokio::select! {
            incoming = read_json::<RpcClientMessage, _>(&mut reader) => {
                match incoming? {
                    Some(RpcClientMessage::Command { id, command }) => {
                        let result = service.command(command).await;
                        write_server_message(&writer, RpcServerMessage::Response { id, result }).await?;
                    }
                    Some(RpcClientMessage::Ping { id }) => {
                        write_server_message(
                            &writer,
                            RpcServerMessage::Response {
                                id,
                                result: Ok(CommandResult::Ok),
                            },
                        ).await?;
                    }
                    Some(RpcClientMessage::Hello { .. }) => {}
                    None => break,
                }
            }
            event = events.recv() => {
                match event {
                    Ok(event) => {
                        write_server_message(&writer, RpcServerMessage::Event { event }).await?;
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(skipped)) => {
                        tracing::warn!("desktop RPC event stream lagged by {skipped} events");
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                }
            }
        }
    }

    Ok(())
}

struct InteractiveGuard {
    service: ServiceHandle,
}

impl Drop for InteractiveGuard {
    fn drop(&mut self) {
        if self.service.interactive_disconnected() != 0 {
            return;
        }
        let service = self.service.clone();
        tokio::spawn(async move {
            let _ = service
                .command(ServiceCommand::SetRequestPolicy {
                    frontend_answers_requests: false,
                })
                .await;
        });
    }
}

async fn write_server_message<W>(
    writer: &Arc<Mutex<W>>,
    message: RpcServerMessage,
) -> anyhow::Result<()>
where
    W: AsyncWrite + Unpin + Send,
{
    let mut writer = writer.lock().await;
    write_json(&mut *writer, &message).await?;
    Ok(())
}

#[allow(dead_code)]
fn _assert_async_io<T: AsyncRead + AsyncWrite + Unpin + Send>() {}

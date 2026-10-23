use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum GuiAction {
    ShowMain,
    OpenMessage {
        sender: String,
        device: String,
        text: String,
    },
    OpenReceiveFolder {
        path: PathBuf,
    },
}

pub fn queue_gui_action(action: GuiAction) -> io::Result<()> {
    with_queue_lock(|path| {
        let mut actions = read_actions(path);
        actions.push(action);
        if actions.len() > 64 {
            actions.drain(..actions.len() - 64);
        }
        write_actions(path, &actions)
    })
}

pub fn take_queued_gui_actions() -> io::Result<Vec<GuiAction>> {
    with_queue_lock(|path| {
        let actions = read_actions(path);
        match fs::remove_file(path) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
        Ok(actions)
    })
}

#[cfg(unix)]
pub fn gui_control_endpoint() -> PathBuf {
    config_dir().join("gui-control.sock")
}

#[cfg(unix)]
pub struct GuiControlServer {
    endpoint: PathBuf,
    shutdown: std::sync::Arc<std::sync::atomic::AtomicBool>,
    wake: Option<std::os::unix::net::UnixStream>,
    thread: Option<std::thread::JoinHandle<()>>,
}

#[cfg(unix)]
impl GuiControlServer {
    pub fn start(sender: std::sync::mpsc::Sender<GuiAction>) -> io::Result<Self> {
        Self::start_at(gui_control_endpoint(), sender)
    }

    fn start_at(endpoint: PathBuf, sender: std::sync::mpsc::Sender<GuiAction>) -> io::Result<Self> {
        use std::os::unix::fs::PermissionsExt;
        use std::os::unix::net::{UnixListener, UnixStream};
        use std::sync::atomic::AtomicBool;

        if let Some(parent) = endpoint.parent() {
            fs::create_dir_all(parent)?;
        }
        match fs::remove_file(&endpoint) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
        let listener = UnixListener::bind(&endpoint)?;
        listener.set_nonblocking(true)?;
        fs::set_permissions(&endpoint, fs::Permissions::from_mode(0o600))?;
        let (wake_reader, wake_writer) = UnixStream::pair()?;
        let shutdown = std::sync::Arc::new(AtomicBool::new(false));
        let thread_shutdown = shutdown.clone();
        let thread = std::thread::Builder::new()
            .name("localsend-gui-control".to_string())
            .spawn(move || {
                use std::io::Read;
                use std::os::unix::io::AsRawFd;
                use std::sync::atomic::Ordering;
                loop {
                    if thread_shutdown.load(Ordering::Acquire) {
                        break;
                    }
                    let mut descriptors = [
                        libc::pollfd {
                            fd: listener.as_raw_fd(),
                            events: libc::POLLIN,
                            revents: 0,
                        },
                        libc::pollfd {
                            fd: wake_reader.as_raw_fd(),
                            events: libc::POLLIN,
                            revents: 0,
                        },
                    ];
                    let ready = unsafe {
                        libc::poll(
                            descriptors.as_mut_ptr(),
                            descriptors.len() as libc::nfds_t,
                            -1,
                        )
                    };
                    if ready < 0 {
                        if io::Error::last_os_error().kind() == io::ErrorKind::Interrupted {
                            continue;
                        }
                        break;
                    }
                    if descriptors[1].revents != 0 {
                        break;
                    }
                    if descriptors[0].revents == 0 {
                        continue;
                    }
                    loop {
                        match listener.accept() {
                            Ok((mut stream, _)) => {
                                let _ = stream
                                    .set_read_timeout(Some(std::time::Duration::from_secs(1)));
                                let mut payload = Vec::new();
                                if (&mut stream)
                                    .take(1024 * 1024)
                                    .read_to_end(&mut payload)
                                    .is_ok()
                                    && let Ok(action) =
                                        serde_json::from_slice::<GuiAction>(&payload)
                                {
                                    let _ = sender.send(action);
                                }
                            }
                            Err(error) if error.kind() == io::ErrorKind::WouldBlock => break,
                            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                            Err(_) => return,
                        }
                    }
                }
            })?;
        Ok(Self {
            endpoint,
            shutdown,
            wake: Some(wake_writer),
            thread: Some(thread),
        })
    }
}

#[cfg(unix)]
impl Drop for GuiControlServer {
    fn drop(&mut self) {
        use std::io::Write;
        use std::sync::atomic::Ordering;
        self.shutdown.store(true, Ordering::Release);
        if let Some(mut wake) = self.wake.take() {
            let _ = wake.write_all(&[1]);
        }
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
        let _ = fs::remove_file(&self.endpoint);
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn control_server_shutdown_wakes_without_polling() {
        let root = std::path::PathBuf::from("/tmp").join(format!(
            "ls-ctl-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        std::fs::create_dir_all(&root).expect("temp root");
        for index in 0..100 {
            let endpoint = root.join(format!("control-{index}.sock"));
            let (sender, _receiver) = std::sync::mpsc::channel();
            let server =
                GuiControlServer::start_at(endpoint.clone(), sender).expect("start server");
            std::thread::sleep(Duration::from_millis(1));
            let started = Instant::now();
            drop(server);
            assert!(
                started.elapsed() < Duration::from_millis(200),
                "control server shutdown took {:?}",
                started.elapsed()
            );
            assert!(!endpoint.exists(), "control socket was not removed");
        }
        std::fs::remove_dir_all(root).expect("remove temp root");
    }
}

#[cfg(all(test, windows))]
mod windows_tests {
    use super::*;
    use std::time::{Duration, Instant};

    fn test_pipe_name(suffix: &str) -> String {
        format!(
            r"\\.\pipe\localsend-gui-test-{}-{suffix}",
            std::process::id()
        )
    }

    fn write_raw_pipe(name: &str, payload: &[u8]) {
        use tokio::io::AsyncWriteExt;
        use tokio::net::windows::named_pipe::ClientOptions;

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("runtime");
        runtime.block_on(async move {
            let mut client = ClientOptions::new().open(name).expect("open pipe");
            client.write_all(payload).await.expect("write pipe");
            client.shutdown().await.expect("close pipe");
        });
    }

    #[test]
    fn named_pipe_round_trip_and_shutdown_are_bounded() {
        let name = test_pipe_name("round-trip");
        let (sender, receiver) = std::sync::mpsc::channel();
        let server = GuiControlServer::start_at_pipe(name.clone(), sender).expect("start server");

        send_gui_control_action_at(&name, &GuiAction::ShowMain).expect("send action");
        assert!(matches!(
            receiver.recv_timeout(Duration::from_secs(1)),
            Ok(GuiAction::ShowMain)
        ));

        let started = Instant::now();
        drop(server);
        assert!(started.elapsed() < Duration::from_millis(200));
    }

    #[test]
    fn oversized_pipe_message_is_discarded_without_stopping_server() {
        let name = test_pipe_name("oversized");
        let (sender, receiver) = std::sync::mpsc::channel();
        let server = GuiControlServer::start_at_pipe(name.clone(), sender).expect("start server");

        write_raw_pipe(&name, &vec![b'x'; 1024 * 1024 + 1]);
        assert!(receiver.recv_timeout(Duration::from_millis(200)).is_err());

        send_gui_control_action_at(&name, &GuiAction::ShowMain).expect("send valid action");
        assert!(matches!(
            receiver.recv_timeout(Duration::from_secs(1)),
            Ok(GuiAction::ShowMain)
        ));
        drop(server);
    }

    #[test]
    fn concurrent_pipe_clients_are_serialized_by_the_server() {
        let name = test_pipe_name("concurrent");
        let (sender, receiver) = std::sync::mpsc::channel();
        let server = GuiControlServer::start_at_pipe(name.clone(), sender).expect("start server");

        let clients = (0..4)
            .map(|index| {
                let name = name.clone();
                std::thread::spawn(move || {
                    send_gui_control_action_at(
                        &name,
                        &GuiAction::OpenReceiveFolder {
                            path: format!("/tmp/received-{index}").into(),
                        },
                    )
                })
            })
            .collect::<Vec<_>>();
        for client in clients {
            client.join().expect("client thread").expect("send action");
        }

        let mut paths = (0..4)
            .map(|_| match receiver.recv_timeout(Duration::from_secs(2)) {
                Ok(GuiAction::OpenReceiveFolder { path }) => path,
                other => panic!("unexpected action: {other:?}"),
            })
            .collect::<Vec<_>>();
        paths.sort();
        assert_eq!(paths.len(), 4);
        drop(server);
    }
}

#[cfg(windows)]
pub struct GuiControlServer {
    shutdown: tokio::sync::watch::Sender<bool>,
    thread: Option<std::thread::JoinHandle<()>>,
}

#[cfg(windows)]
impl GuiControlServer {
    pub fn start(sender: std::sync::mpsc::Sender<GuiAction>) -> io::Result<Self> {
        Self::start_at_pipe(gui_control_pipe_name(), sender)
    }

    fn start_at_pipe(name: String, sender: std::sync::mpsc::Sender<GuiAction>) -> io::Result<Self> {
        use tokio::net::windows::named_pipe::ServerOptions;

        let (ready_tx, ready_rx) = std::sync::mpsc::sync_channel(1);
        let (shutdown, shutdown_rx) = tokio::sync::watch::channel(false);
        let thread = std::thread::Builder::new()
            .name("localsend-gui-control-pipe".to_string())
            .spawn(move || {
                let runtime = match tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                {
                    Ok(runtime) => runtime,
                    Err(error) => {
                        let _ = ready_tx.send(Err(io::Error::other(error.to_string())));
                        return;
                    }
                };
                runtime.block_on(async move {
                    let mut shutdown = shutdown_rx;
                    let mut first_instance = true;
                    loop {
                        let server = ServerOptions::new()
                            .first_pipe_instance(first_instance)
                            .create(&name);
                        first_instance = false;
                        let mut server = match server {
                            Ok(server) => server,
                            Err(error) => {
                                let _ = ready_tx.send(Err(error));
                                return;
                            }
                        };
                        let _ = ready_tx.send(Ok(()));

                        let connected = tokio::select! {
                            result = server.connect() => result,
                            _ = shutdown.changed() => break,
                        };
                        if connected.is_err() {
                            continue;
                        }

                        let read = tokio::select! {
                            result = tokio::time::timeout(
                                std::time::Duration::from_secs(2),
                                read_limited(&mut server),
                            ) => result,
                            _ = shutdown.changed() => break,
                        };
                        let Ok(Ok(payload)) = read else {
                            continue;
                        };
                        if payload.is_empty() {
                            continue;
                        }
                        if let Ok(action) = serde_json::from_slice::<GuiAction>(&payload) {
                            let _ = sender.send(action);
                        }
                    }
                });
            })?;

        match ready_rx.recv() {
            Ok(Ok(())) => Ok(Self {
                shutdown,
                thread: Some(thread),
            }),
            Ok(Err(error)) => {
                let _ = shutdown.send_replace(true);
                let _ = thread.join();
                Err(error)
            }
            Err(error) => {
                let _ = shutdown.send_replace(true);
                let _ = thread.join();
                Err(io::Error::other(error))
            }
        }
    }
}

#[cfg(windows)]
impl Drop for GuiControlServer {
    fn drop(&mut self) {
        let _ = self.shutdown.send_replace(true);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

#[cfg(all(not(unix), not(windows)))]
pub struct GuiControlServer;

#[cfg(all(not(unix), not(windows)))]
impl GuiControlServer {
    pub fn start(_sender: std::sync::mpsc::Sender<GuiAction>) -> io::Result<Self> {
        Ok(Self)
    }
}

#[cfg(unix)]
pub fn send_gui_control_action(action: &GuiAction) -> io::Result<()> {
    use std::io::Write;
    use std::os::unix::net::UnixStream;

    let mut stream = UnixStream::connect(gui_control_endpoint())?;
    let payload = serde_json::to_vec(action).map_err(io::Error::other)?;
    stream.write_all(&payload)?;
    stream.shutdown(std::net::Shutdown::Write)
}

#[cfg(windows)]
pub fn send_gui_control_action(action: &GuiAction) -> io::Result<()> {
    send_gui_control_action_at(&gui_control_pipe_name(), action)
}

#[cfg(windows)]
fn send_gui_control_action_at(name: &str, action: &GuiAction) -> io::Result<()> {
    use tokio::io::AsyncWriteExt;
    use tokio::net::windows::named_pipe::ClientOptions;

    let payload = serde_json::to_vec(action).map_err(io::Error::other)?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(io::Error::other)?;
    runtime.block_on(async move {
        let mut last_error = None;
        let mut client = None;
        for attempt in 0..10 {
            match ClientOptions::new().open(name) {
                Ok(connected) => {
                    client = Some(connected);
                    break;
                }
                Err(error) => {
                    last_error = Some(error);
                    if attempt < 9 {
                        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
                    }
                }
            }
        }
        let mut client = client.ok_or_else(|| {
            last_error.unwrap_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotConnected,
                    "could not open GUI control pipe",
                )
            })
        })?;
        client.write_all(&payload).await?;
        client.shutdown().await
    })
}

#[cfg(all(not(unix), not(windows)))]
pub fn send_gui_control_action(_action: &GuiAction) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "GUI control socket is unavailable on this platform",
    ))
}

#[cfg(windows)]
fn gui_control_pipe_name() -> String {
    use sha2::{Digest, Sha256};

    let mut identity = config_dir().to_string_lossy().into_owned();
    if let Some(endpoint) = std::env::var_os("LOCALSEND_RPC_SOCKET") {
        identity.push('|');
        identity.push_str(&endpoint.to_string_lossy());
    }
    let digest = Sha256::digest(identity.as_bytes());
    let suffix = digest[..8]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!(r"\\.\pipe\localsend-gui-{suffix}")
}

#[cfg(windows)]
async fn read_limited(
    stream: &mut tokio::net::windows::named_pipe::NamedPipeServer,
) -> io::Result<Vec<u8>> {
    use tokio::io::AsyncReadExt;

    const MAX_PAYLOAD: usize = 1024 * 1024;
    let mut payload = Vec::new();
    let mut buffer = [0_u8; 8192];
    loop {
        let read = stream.read(&mut buffer).await?;
        if read == 0 {
            break;
        }
        if payload.len().saturating_add(read) > MAX_PAYLOAD {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "GUI control message exceeds 1 MiB",
            ));
        }
        payload.extend_from_slice(&buffer[..read]);
    }
    Ok(payload)
}

fn with_queue_lock<T>(operation: impl FnOnce(&Path) -> io::Result<T>) -> io::Result<T> {
    let path = gui_action_queue_path();
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::other("GUI action queue has no parent directory"))?;
    fs::create_dir_all(parent)?;
    let lock_path = path.with_extension("lock");
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(lock_path)?;
    FileExt::lock_exclusive(&lock)?;
    let result = operation(&path);
    let _ = FileExt::unlock(&lock);
    result
}

fn read_actions(path: &Path) -> Vec<GuiAction> {
    fs::read(path)
        .ok()
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
        .unwrap_or_default()
}

fn write_actions(path: &Path, actions: &[GuiAction]) -> io::Result<()> {
    let temporary = path.with_extension(format!("tmp-{}", std::process::id()));
    let bytes = serde_json::to_vec(actions).map_err(io::Error::other)?;
    fs::write(&temporary, bytes)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&temporary, fs::Permissions::from_mode(0o600))?;
    }
    if path.exists() {
        fs::remove_file(path)?;
    }
    fs::rename(temporary, path)
}

pub fn gui_action_queue_path() -> PathBuf {
    config_dir().join("gui-actions.json")
}

fn config_dir() -> PathBuf {
    std::env::var_os("LOCALSEND_CONFIG_DIR")
        .map(PathBuf::from)
        .or_else(|| {
            directories::ProjectDirs::from("org", "LocalSend", "localsend-rs")
                .map(|dirs| dirs.config_dir().to_path_buf())
        })
        .unwrap_or_else(|| std::env::temp_dir().join("localsend-rs"))
}

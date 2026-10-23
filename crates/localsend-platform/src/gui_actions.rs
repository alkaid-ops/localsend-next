use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[cfg(not(unix))]
pub struct GuiControlServer;

#[cfg(not(unix))]
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

#[cfg(not(unix))]
pub fn send_gui_control_action(_action: &GuiAction) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "GUI control socket is unavailable on this platform",
    ))
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

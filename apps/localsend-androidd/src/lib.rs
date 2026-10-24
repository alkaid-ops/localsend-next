use anyhow::{Context, bail};
use fs2::FileExt;
use localsend_rpc::{
    AutoSaveMode, DesktopSettings, HistoryEntry, Protocol, ServiceEvent, TransferDirection,
    TransferStatus,
};
use localsend_service::{ServiceHandle, ServicePaths};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::broadcast::error::RecvError;

pub const DEFAULT_RUNTIME_DIR: &str = "/data/adb/localsend-next";
pub const DEFAULT_DESTINATION: &str = "/storage/emulated/0/Download/LocalSend";
const DEFAULT_LOG_LIMIT: u64 = 10 * 1024 * 1024;
const SERVER_START_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Debug, Clone)]
pub struct Options {
    pub runtime_dir: PathBuf,
    pub config_dir: PathBuf,
    pub destination_dir: PathBuf,
    pub log_file: PathBuf,
    pub pid_file: PathBuf,
    pub lock_file: PathBuf,
    pub disabled_file: PathBuf,
}

impl Options {
    pub fn from_env() -> anyhow::Result<Self> {
        Self::from_args(std::env::args().skip(1))
    }

    pub fn from_args(args: impl IntoIterator<Item = String>) -> anyhow::Result<Self> {
        let mut runtime_dir = PathBuf::from(DEFAULT_RUNTIME_DIR);
        let mut destination_dir = PathBuf::from(DEFAULT_DESTINATION);
        let mut args = args.into_iter();
        while let Some(argument) = args.next() {
            match argument.as_str() {
                "--runtime-dir" => {
                    runtime_dir = args
                        .next()
                        .filter(|value| !value.trim().is_empty())
                        .context("--runtime-dir requires a value")?
                        .into();
                }
                "--destination" => {
                    destination_dir = args
                        .next()
                        .filter(|value| !value.trim().is_empty())
                        .context("--destination requires a value")?
                        .into();
                }
                "-h" | "--help" => {
                    println!(
                        "localsend-androidd [--runtime-dir PATH] [--destination PATH]\n\
                         Defaults: runtime={DEFAULT_RUNTIME_DIR}, destination={DEFAULT_DESTINATION}"
                    );
                    std::process::exit(0);
                }
                other => bail!("unknown argument: {other}"),
            }
        }
        let config_dir = runtime_dir.join("config");
        let log_file = runtime_dir.join("logs/localsend-androidd.log");
        let pid_file = runtime_dir.join("localsend-androidd.pid");
        let lock_file = runtime_dir.join("localsend-androidd.lock");
        let disabled_file = runtime_dir.join("disabled");
        Ok(Self {
            runtime_dir,
            config_dir,
            destination_dir,
            log_file,
            pid_file,
            lock_file,
            disabled_file,
        })
    }

    fn service_paths(&self) -> ServicePaths {
        ServicePaths {
            config_dir: self.config_dir.clone(),
            data_dir: self.config_dir.join("data"),
            settings_file: self.config_dir.join("settings.json"),
            history_file: self.config_dir.join("history.jsonl"),
            favorites_file: self.config_dir.join("favorites.json"),
            identity_file: self.config_dir.join("identity.pem"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationEvent {
    pub kind: NotificationKind,
    pub title: String,
    pub body: String,
    pub tag: String,
    pub media_scan_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationKind {
    Text,
    File,
}

pub trait PlatformAdapter: Send + Sync {
    fn post_notification(&self, event: &NotificationEvent) -> Result<(), String>;
    fn scan_media(&self, path: &Path) -> Result<(), String>;
}

#[derive(Debug)]
struct CommandSpec {
    program: &'static str,
    args: Vec<String>,
}

pub struct AndroidCommandAdapter;

impl PlatformAdapter for AndroidCommandAdapter {
    fn post_notification(&self, event: &NotificationEvent) -> Result<(), String> {
        let spec = notification_command(event);
        run_command(&spec)
    }

    fn scan_media(&self, path: &Path) -> Result<(), String> {
        let spec = media_scan_command(path);
        run_command(&spec)
    }
}

fn notification_command(event: &NotificationEvent) -> CommandSpec {
    CommandSpec {
        program: "/system/bin/cmd",
        args: vec![
            "notification".to_string(),
            "post".to_string(),
            "-t".to_string(),
            event.title.clone(),
            event.tag.clone(),
            event.body.clone(),
        ],
    }
}

fn media_scan_command(path: &Path) -> CommandSpec {
    CommandSpec {
        program: "/system/bin/am",
        args: vec![
            "broadcast".to_string(),
            "-a".to_string(),
            "android.intent.action.MEDIA_SCANNER_SCAN_FILE".to_string(),
            "-d".to_string(),
            format!("file://{}", path.display()),
        ],
    }
}

fn run_command(spec: &CommandSpec) -> Result<(), String> {
    let output = ProcessCommand::new(spec.program)
        .args(&spec.args)
        .output()
        .map_err(|error| format!("running {}: {error}", spec.program))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "{} failed with {}: {}",
            spec.program,
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

fn default_settings(destination: &Path) -> DesktopSettings {
    DesktopSettings {
        alias: std::env::var("LOCALSEND_ANDROID_ALIAS")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "LocalSend Android".to_string()),
        port: 53317,
        protocol: Protocol::Http,
        destination_dir: Some(destination.to_path_buf()),
        auto_save: AutoSaveMode::On,
        start_server_on_launch: true,
        save_to_history: true,
        ..DesktopSettings::default()
    }
}

fn ensure_default_settings(path: &Path, destination: &Path) -> anyhow::Result<()> {
    if path.exists() {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let settings = default_settings(destination);
    let temporary = path.with_extension(format!("tmp-{}", std::process::id()));
    fs::write(&temporary, serde_json::to_vec_pretty(&settings)?)?;
    fs::rename(temporary, path)?;
    Ok(())
}

pub async fn run(options: Options) -> anyhow::Result<()> {
    fs::create_dir_all(&options.runtime_dir)
        .with_context(|| format!("creating {}", options.runtime_dir.display()))?;
    fs::create_dir_all(&options.config_dir)
        .with_context(|| format!("creating {}", options.config_dir.display()))?;
    fs::create_dir_all(&options.destination_dir)
        .with_context(|| format!("creating {}", options.destination_dir.display()))?;
    ensure_default_settings(
        &options.config_dir.join("settings.json"),
        &options.destination_dir,
    )?;

    let _runtime_guard = RuntimeGuard::acquire(&options.pid_file, &options.lock_file)?;
    if options.disabled_file.exists() {
        tracing::info!("disabled marker exists; daemon will not start");
        return Ok(());
    }

    let service = ServiceHandle::spawn_with_paths(options.service_paths())
        .await
        .map_err(anyhow::Error::msg)?;
    let events = service.subscribe();
    wait_for_server(&service).await?;
    tracing::info!(port = ?service.snapshot().server_port, "LocalSend Android receiver is running");

    let adapter: Arc<dyn PlatformAdapter> = Arc::new(AndroidCommandAdapter);
    run_event_loop(service, events, adapter, options.disabled_file).await
}

async fn wait_for_server(service: &ServiceHandle) -> anyhow::Result<()> {
    let attempts = SERVER_START_TIMEOUT.as_millis() / 100;
    for _ in 0..attempts {
        if service.snapshot().server_running {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    bail!("LocalSend HTTP server did not start within {SERVER_START_TIMEOUT:?}")
}

async fn run_event_loop(
    service: ServiceHandle,
    mut events: tokio::sync::broadcast::Receiver<localsend_rpc::SequencedEvent>,
    adapter: Arc<dyn PlatformAdapter>,
    disabled_file: PathBuf,
) -> anyhow::Result<()> {
    let mut shutdown = service.subscribe_shutdown();
    let mut disabled_check = tokio::time::interval(Duration::from_secs(1));
    disabled_check.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    let shutdown_signal = shutdown_signal();
    tokio::pin!(shutdown_signal);
    let mut shutdown_requested = false;

    loop {
        tokio::select! {
            _ = &mut shutdown_signal => {
                tracing::info!("received termination signal");
                let _ = service.command(localsend_rpc::ServiceCommand::Shutdown).await;
                break;
            }
            changed = shutdown.changed() => {
                if changed.is_err() || *shutdown.borrow() {
                    break;
                }
            }
            _ = disabled_check.tick(), if !shutdown_requested => {
                if disabled_file.exists() {
                    shutdown_requested = true;
                    tracing::info!("disabled marker detected; stopping daemon");
                    let _ = service.command(localsend_rpc::ServiceCommand::Shutdown).await;
                }
            }
            event = events.recv() => {
                match event {
                    Ok(event) => {
                        if matches!(event.event, ServiceEvent::DaemonStopping) {
                            break;
                        }
                        if let Some(notification) = notification_for_event(&event.event) {
                            dispatch_notification(adapter.clone(), notification);
                        }
                    }
                    Err(RecvError::Lagged(skipped)) => {
                        tracing::warn!(skipped, "service event stream lagged; notification may be missed");
                    }
                    Err(RecvError::Closed) => break,
                }
            }
        }
    }
    Ok(())
}

fn dispatch_notification(adapter: Arc<dyn PlatformAdapter>, notification: NotificationEvent) {
    tokio::task::spawn_blocking(move || {
        if let Err(error) = adapter.post_notification(&notification) {
            tracing::warn!(kind = ?notification.kind, %error, "notification failed");
        }
        if let Some(path) = notification.media_scan_path.as_deref()
            && let Err(error) = adapter.scan_media(path)
        {
            tracing::warn!(path = %path.display(), %error, "media scan failed");
        }
    });
}

fn notification_for_event(event: &ServiceEvent) -> Option<NotificationEvent> {
    match event {
        ServiceEvent::TextReceived {
            sender_alias, text, ..
        } => Some(NotificationEvent {
            kind: NotificationKind::Text,
            title: "收到新消息".to_string(),
            body: format!("{sender_alias}: {text}"),
            tag: "localsend-text".to_string(),
            media_scan_path: None,
        }),
        ServiceEvent::HistoryAppended { entry } => notification_for_history(entry),
        _ => None,
    }
}

fn notification_for_history(entry: &HistoryEntry) -> Option<NotificationEvent> {
    if entry.direction != TransferDirection::Receive
        || entry.status != TransferStatus::Finished
        || entry.message.is_some()
    {
        return None;
    }
    Some(NotificationEvent {
        kind: NotificationKind::File,
        title: "收到文件".to_string(),
        body: format!(
            "{} · {} · {}",
            entry.peer_alias,
            entry.file_name,
            format_bytes(entry.size)
        ),
        tag: format!("localsend-file-{}", entry.id),
        media_scan_path: entry.path.clone(),
    })
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit + 1 < UNITS.len() {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{bytes} {}", UNITS[unit])
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};

        let terminate = signal(SignalKind::terminate());
        match terminate {
            Ok(mut terminate) => {
                tokio::select! {
                    _ = tokio::signal::ctrl_c() => {}
                    _ = terminate.recv() => {}
                }
            }
            Err(_) => {
                let _ = tokio::signal::ctrl_c().await;
            }
        }
    }
    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }
}

struct RuntimeGuard {
    pid_file: PathBuf,
    lock_file: File,
    pid: u32,
}

impl RuntimeGuard {
    fn acquire(pid_file: &Path, lock_file: &Path) -> anyhow::Result<Self> {
        if let Some(parent) = pid_file.parent() {
            fs::create_dir_all(parent)?;
        }
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(lock_file)?;
        lock.try_lock_exclusive()
            .context("another localsend-androidd instance already owns the runtime lock")?;
        let pid = std::process::id();
        fs::write(pid_file, pid.to_string())?;
        Ok(Self {
            pid_file: pid_file.to_path_buf(),
            lock_file: lock,
            pid,
        })
    }
}

impl Drop for RuntimeGuard {
    fn drop(&mut self) {
        if fs::read_to_string(&self.pid_file)
            .ok()
            .and_then(|value| value.trim().parse::<u32>().ok())
            == Some(self.pid)
        {
            let _ = fs::remove_file(&self.pid_file);
        }
        let _ = self.lock_file.unlock();
    }
}

pub fn init_logging(path: &Path) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let writer = RotatingLogWriter::open(path, DEFAULT_LOG_LIMIT)?;
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_ansi(false)
        .with_writer(writer)
        .try_init();
    tracing::info!(path = %path.display(), pid = std::process::id(), "localsend-androidd log opened");
    Ok(())
}

#[derive(Clone)]
struct RotatingLogWriter(Arc<Mutex<RotatingLog>>);

impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for RotatingLogWriter {
    type Writer = Self;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

impl Write for RotatingLogWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.0
            .lock()
            .map_err(|_| io::Error::other("log lock poisoned"))?
            .write(buffer)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0
            .lock()
            .map_err(|_| io::Error::other("log lock poisoned"))?
            .flush()
    }
}

struct RotatingLog {
    path: PathBuf,
    file: File,
    written: u64,
    limit: u64,
}

impl RotatingLogWriter {
    fn open(path: &Path, limit: u64) -> io::Result<Self> {
        Ok(Self(Arc::new(Mutex::new(RotatingLog::open(path, limit)?))))
    }
}

impl RotatingLog {
    fn open(path: &Path, limit: u64) -> io::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        let written = file.metadata()?.len();
        Ok(Self {
            path: path.to_path_buf(),
            file,
            written,
            limit,
        })
    }

    fn rotate(&mut self) -> io::Result<()> {
        self.file.flush()?;
        let backup = self.path.with_extension("log.1");
        if backup.exists() {
            fs::remove_file(&backup)?;
        }
        fs::rename(&self.path, &backup)?;
        self.file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        self.written = 0;
        Ok(())
    }
}

impl Write for RotatingLog {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        if self.limit > 0 && self.written > 0 && self.written + buffer.len() as u64 > self.limit {
            self.rotate()?;
        }
        let written = self.file.write(buffer)?;
        self.written = self.written.saturating_add(written as u64);
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use localsend_rpc::ServiceEvent;
    use std::sync::Mutex;

    #[derive(Default)]
    struct RecordingAdapter {
        notifications: Mutex<Vec<NotificationEvent>>,
        scans: Mutex<Vec<PathBuf>>,
    }

    impl PlatformAdapter for RecordingAdapter {
        fn post_notification(&self, event: &NotificationEvent) -> Result<(), String> {
            self.notifications.lock().unwrap().push(event.clone());
            Ok(())
        }

        fn scan_media(&self, path: &Path) -> Result<(), String> {
            self.scans.lock().unwrap().push(path.to_path_buf());
            Ok(())
        }
    }

    fn history(message: Option<&str>, path: Option<PathBuf>) -> HistoryEntry {
        HistoryEntry {
            id: "history-1".to_string(),
            file_id: Some("file-1".to_string()),
            direction: TransferDirection::Receive,
            peer_alias: "Sender".to_string(),
            file_name: "file.txt".to_string(),
            path,
            size: 2048,
            timestamp_ms: 1,
            status: TransferStatus::Finished,
            message: message.map(ToOwned::to_owned),
        }
    }

    #[test]
    fn text_event_maps_to_one_notification() {
        let event = ServiceEvent::TextReceived {
            sender_alias: "Sender".to_string(),
            sender_device_model: None,
            text: "hello".to_string(),
        };
        let notification = notification_for_event(&event).expect("notification");
        assert_eq!(notification.kind, NotificationKind::Text);
        assert!(notification.body.contains("Sender: hello"));
        assert!(notification.media_scan_path.is_none());
    }

    #[test]
    fn message_history_is_not_notified_twice() {
        let event = ServiceEvent::HistoryAppended {
            entry: history(Some("hello"), None),
        };
        assert!(notification_for_event(&event).is_none());
    }

    #[test]
    fn received_file_maps_to_notification_and_media_scan() {
        let event = ServiceEvent::HistoryAppended {
            entry: history(None, Some(PathBuf::from("/sdcard/file.txt"))),
        };
        let notification = notification_for_event(&event).expect("notification");
        assert_eq!(notification.kind, NotificationKind::File);
        assert_eq!(
            notification.media_scan_path,
            Some(PathBuf::from("/sdcard/file.txt"))
        );
    }

    #[test]
    fn notification_command_keeps_body_as_a_single_argument() {
        let notification = NotificationEvent {
            kind: NotificationKind::Text,
            title: "title".to_string(),
            body: "line one\nline two".to_string(),
            tag: "localsend-text".to_string(),
            media_scan_path: None,
        };
        let command = notification_command(&notification);
        assert_eq!(command.program, "/system/bin/cmd");
        assert_eq!(command.args.last().unwrap(), "line one\nline two");
        assert_eq!(command.args[2], "-t");
    }

    #[tokio::test]
    async fn recording_adapter_observes_notification_and_scan() {
        let adapter = Arc::new(RecordingAdapter::default());
        dispatch_notification(
            adapter.clone(),
            NotificationEvent {
                kind: NotificationKind::File,
                title: "file".to_string(),
                body: "body".to_string(),
                tag: "tag".to_string(),
                media_scan_path: Some(PathBuf::from("/sdcard/file.txt")),
            },
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
        assert_eq!(adapter.scans.lock().unwrap().len(), 1);
        assert_eq!(adapter.notifications.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn disabled_marker_prevents_service_startup() {
        let root = tempfile::tempdir().unwrap();
        let options = Options::from_args([
            "--runtime-dir".to_string(),
            root.path().join("runtime").display().to_string(),
            "--destination".to_string(),
            root.path().join("downloads").display().to_string(),
        ])
        .unwrap();
        fs::create_dir_all(options.runtime_dir.join("config")).unwrap();
        fs::write(&options.disabled_file, b"disabled").unwrap();
        run(options.clone())
            .await
            .expect("disabled daemon exits cleanly");
        assert!(!options.pid_file.exists());
        assert!(options.config_dir.join("settings.json").exists());
    }

    #[test]
    fn defaults_enable_receive_and_auto_save() {
        let settings = default_settings(Path::new(DEFAULT_DESTINATION));
        assert!(settings.start_server_on_launch);
        assert_eq!(settings.auto_save, AutoSaveMode::On);
        assert_eq!(
            settings.destination_dir.as_deref(),
            Some(Path::new(DEFAULT_DESTINATION))
        );
    }

    #[test]
    fn options_use_android_runtime_contract() {
        let options = Options::from_args(Vec::new()).expect("default options");
        assert_eq!(options.runtime_dir, PathBuf::from(DEFAULT_RUNTIME_DIR));
        assert_eq!(
            options.config_dir,
            PathBuf::from(DEFAULT_RUNTIME_DIR).join("config")
        );
        assert_eq!(
            options.disabled_file,
            PathBuf::from(DEFAULT_RUNTIME_DIR).join("disabled")
        );
        let paths = options.service_paths();
        assert_eq!(
            paths.settings_file,
            options.config_dir.join("settings.json")
        );
        assert_eq!(paths.identity_file, options.config_dir.join("identity.pem"));
    }

    #[test]
    fn options_allow_isolated_test_directories() {
        let options = Options::from_args([
            "--runtime-dir".to_string(),
            "/tmp/localsend-root".to_string(),
            "--destination".to_string(),
            "/tmp/downloads".to_string(),
        ])
        .expect("options");
        assert_eq!(options.runtime_dir, PathBuf::from("/tmp/localsend-root"));
        assert_eq!(options.destination_dir, PathBuf::from("/tmp/downloads"));
        assert_eq!(
            options.log_file,
            PathBuf::from("/tmp/localsend-root/logs/localsend-androidd.log")
        );
    }

    #[test]
    fn rotating_log_keeps_one_backup() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("daemon.log");
        let mut log = RotatingLog::open(&path, 8).unwrap();
        log.write_all(b"12345678").unwrap();
        log.write_all(b"abc").unwrap();
        log.flush().unwrap();
        assert!(path.exists());
        assert!(path.with_extension("log.1").exists());
        assert_eq!(fs::read(path).unwrap(), b"abc");
    }

    #[test]
    fn module_default_settings_parse_with_receive_enabled() {
        let settings: DesktopSettings = serde_json::from_str(include_str!(
            "../../../packaging/android-root/config/settings.json"
        ))
        .expect("module settings");
        assert!(settings.start_server_on_launch);
        assert_eq!(settings.auto_save, AutoSaveMode::On);
        assert_eq!(
            settings.destination_dir.as_deref(),
            Some(Path::new(DEFAULT_DESTINATION))
        );
    }
}

#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod notifications;

use anyhow::Context;
use localsend_platform::{
    GuiAction, InstanceLock, TrayPidGuard, gui_pid_path, gui_process_is_running, queue_gui_action,
    scoped_lock_name, send_gui_control_action, terminate_gui_process, terminate_tray_process,
};
use localsend_rpc::{
    DaemonClient, DaemonLifecycle, ServiceCommand, ServiceEvent, TransferDirection, TransferStatus,
};
use std::process::{Command, Stdio};
use std::time::Duration;
use tray_icon::menu::{Menu, MenuEvent, MenuItem};
use tray_icon::{Icon, TrayIconBuilder};

const OPEN_APP: &str = "localsend.open-app";
const RESTART_APP: &str = "localsend.restart";
const QUIT_APP: &str = "localsend.quit";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrayLanguage {
    English,
    SimplifiedChinese,
}

impl TrayLanguage {
    fn from_settings(language: &str) -> Self {
        if language.starts_with("zh") {
            Self::SimplifiedChinese
        } else if language.starts_with("en") {
            Self::English
        } else if sys_locale::get_locale().is_some_and(|locale| locale.starts_with("zh")) {
            Self::SimplifiedChinese
        } else {
            Self::English
        }
    }

    fn open_app(self) -> &'static str {
        match self {
            Self::English => "Open main window",
            Self::SimplifiedChinese => "打开主界面",
        }
    }

    fn restart_app(self) -> &'static str {
        match self {
            Self::English => "Restart",
            Self::SimplifiedChinese => "重启",
        }
    }

    fn quit_app(self) -> &'static str {
        match self {
            Self::English => "Quit",
            Self::SimplifiedChinese => "退出",
        }
    }

    fn restart_unavailable(self) -> &'static str {
        match self {
            Self::English => "Restart unavailable",
            Self::SimplifiedChinese => "无法重启",
        }
    }

    fn externally_managed_daemon(self) -> &'static str {
        match self {
            Self::English => "LocalSend is using an externally managed daemon.",
            Self::SimplifiedChinese => "LocalSend 正在使用外部管理的后台服务。",
        }
    }
}

struct TrayMenu {
    open_app: MenuItem,
    restart_app: MenuItem,
    quit_app: MenuItem,
}

impl TrayMenu {
    fn new(language: TrayLanguage) -> anyhow::Result<(Self, Menu)> {
        let items = Self {
            open_app: MenuItem::with_id(OPEN_APP, language.open_app(), true, None),
            restart_app: MenuItem::with_id(RESTART_APP, language.restart_app(), true, None),
            quit_app: MenuItem::with_id(QUIT_APP, language.quit_app(), true, None),
        };
        let menu = Menu::with_items(&[&items.open_app, &items.restart_app, &items.quit_app])?;
        Ok((items, menu))
    }
}

pub fn run_tray() -> anyhow::Result<()> {
    if std::env::args().any(|argument| argument == "--quit") {
        return quit_instance_request();
    }
    initialize_macos_tray()?;
    let (gui_action_tx, gui_action_rx) = std::sync::mpsc::channel();
    if std::env::args().any(|argument| argument == "--open-main") {
        if std::env::args().any(|argument| argument == "--minimized") {
            return open_app(true);
        }
        route_gui_action(GuiAction::ShowMain);
        return Ok(());
    }
    let lock_name = scoped_lock_name("localsend-tray", "tray");
    let Some(_lock) = InstanceLock::try_acquire(&lock_name)? else {
        return Ok(());
    };
    let _tray_pid = TrayPidGuard::acquire()?;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()?;
    let mut client = runtime
        .block_on(connect_or_start_daemon())
        .context("starting or connecting to localsendd")?;
    if std::env::var("LOCALSEND_TRAY_DISABLED")
        .is_ok_and(|value| matches!(value.as_str(), "1" | "true" | "yes"))
    {
        eprintln!("tray disabled; localsendd is running and the TUI remains available");
        return Ok(());
    }

    let language = TrayLanguage::from_settings("system");
    notifications::init(gui_action_tx);
    let (_menu_items, menu) = TrayMenu::new(language)?;
    let icon = Icon::from_rgba(tray_icon_rgba(32), 32, 32)?;
    let builder = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("LocalSend PC")
        .with_menu_on_left_click(true);
    #[cfg(target_os = "macos")]
    let tray = builder.with_icon_templated(icon).build()?;
    #[cfg(not(target_os = "macos"))]
    let tray = builder.with_icon(icon).build().context(
        "failed to create system tray; Linux users may need GTK3 and AppIndicator support \
         (for GNOME also enable an AppIndicator extension)",
    )?;
    let _menu_keepalive = tray.clone();
    let mut notification_client = match runtime.block_on(DaemonClient::connect_notifications()) {
        Ok(client) => Some(client),
        Err(error) => {
            tracing::warn!(%error, "notification stream connection failed");
            None
        }
    };
    let mut daemon_events = notification_client
        .as_ref()
        .map(DaemonClient::subscribe)
        .unwrap_or_else(|| client.subscribe());
    tracing::info!("notification event stream active");

    loop {
        pump_macos_events();
        if client.is_connection_closed() {
            let was_lifecycle_owner = client.owns_lifecycle();
            drop(notification_client.take());
            drop(client);
            client = runtime
                .block_on(reconnect_daemon(was_lifecycle_owner))
                .context("reconnecting to localsendd")?;
            notification_client = runtime.block_on(DaemonClient::connect_notifications()).ok();
            daemon_events = notification_client
                .as_ref()
                .map(DaemonClient::subscribe)
                .unwrap_or_else(|| client.subscribe());
            continue;
        }
        loop {
            let event = match daemon_events.try_recv() {
                Ok(event) => event,
                Err(tokio::sync::broadcast::error::TryRecvError::Lagged(skipped)) => {
                    tracing::warn!("tray notification stream missed {skipped} events");
                    match runtime.block_on(DaemonClient::connect_notifications()) {
                        Ok(reconnected) => {
                            daemon_events = reconnected.subscribe();
                            notification_client = Some(reconnected);
                        }
                        Err(error) => {
                            tracing::warn!("could not reconnect tray notification stream: {error}");
                        }
                    }
                    break;
                }
                Err(tokio::sync::broadcast::error::TryRecvError::Empty) => break,
                Err(tokio::sync::broadcast::error::TryRecvError::Closed) => {
                    if !client.is_connection_closed() {
                        match runtime.block_on(DaemonClient::connect_notifications()) {
                            Ok(reconnected) => {
                                daemon_events = reconnected.subscribe();
                                notification_client = Some(reconnected);
                            }
                            Err(error) => {
                                tracing::warn!(
                                    "could not reconnect tray notification stream: {error}"
                                );
                            }
                        }
                    }
                    break;
                }
            };
            match event.event {
                ServiceEvent::TextReceived {
                    sender_alias,
                    sender_device_model,
                    text,
                } => {
                    tracing::debug!(
                        sender = %sender_alias,
                        characters = text.chars().count(),
                        "received text notification event"
                    );
                    let (title, body) = if language == TrayLanguage::English {
                        ("New message received", format!("{sender_alias}: {text}"))
                    } else {
                        ("收到新消息", format!("{sender_alias}：{text}"))
                    };
                    let action_label = if language == TrayLanguage::English {
                        "Open"
                    } else {
                        "打开"
                    };
                    notifications::show(
                        GuiAction::OpenMessage {
                            sender: sender_alias,
                            device: sender_device_model.unwrap_or_default(),
                            text,
                        },
                        title,
                        &body,
                        action_label,
                    );
                }
                ServiceEvent::HistoryAppended { entry } if should_notify_received_file(&entry) => {
                    let folder = entry
                        .path
                        .as_ref()
                        .and_then(|path| path.parent().map(std::path::Path::to_path_buf))
                        .or_else(download_directory);
                    if let Some(folder) = folder {
                        let (title, body) = if language == TrayLanguage::English {
                            (
                                "File received",
                                format!(
                                    "{} · {} · {}",
                                    entry.peer_alias,
                                    entry.file_name,
                                    format_bytes(entry.size)
                                ),
                            )
                        } else {
                            (
                                "收到文件",
                                format!(
                                    "{} · {} · {}",
                                    entry.peer_alias,
                                    entry.file_name,
                                    format_bytes(entry.size)
                                ),
                            )
                        };
                        let action_label = if language == TrayLanguage::English {
                            "Open"
                        } else {
                            "打开"
                        };
                        notifications::show(
                            GuiAction::OpenReceiveFolder { path: folder },
                            title,
                            &body,
                            action_label,
                        );
                    }
                }
                ServiceEvent::DaemonStopping => {}
                _ => {}
            }
        }
        while let Ok(action) = gui_action_rx.try_recv() {
            route_gui_action(action);
        }
        if let Ok(event) = MenuEvent::receiver().recv_timeout(Duration::from_millis(100)) {
            match event.id.as_ref() {
                OPEN_APP => route_gui_action(GuiAction::ShowMain),
                RESTART_APP => {
                    if !client.owns_lifecycle() {
                        show_message(
                            language.restart_unavailable(),
                            language.externally_managed_daemon(),
                        );
                        continue;
                    }
                    let old_instance = client.instance_id();
                    drop(notification_client.take());
                    runtime
                        .block_on(client.command(ServiceCommand::Shutdown))
                        .map_err(anyhow::Error::msg)?;
                    runtime.block_on(client.wait_closed());
                    drop(client);
                    client = runtime
                        .block_on(connect_or_start_daemon())
                        .context("restarting localsendd")?;
                    if client.instance_id() == old_instance {
                        anyhow::bail!("localsendd restart returned the previous instance");
                    }
                    notification_client =
                        runtime.block_on(DaemonClient::connect_notifications()).ok();
                    daemon_events = notification_client
                        .as_ref()
                        .map(DaemonClient::subscribe)
                        .unwrap_or_else(|| client.subscribe());
                }
                QUIT_APP => {
                    terminate_gui_process();
                    if client.owns_lifecycle() {
                        let _ = runtime.block_on(async {
                            tokio::time::timeout(
                                Duration::from_secs(5),
                                client.command(ServiceCommand::Shutdown),
                            )
                            .await
                        });
                        let _ = runtime.block_on(async {
                            tokio::time::timeout(Duration::from_secs(5), client.wait_closed()).await
                        });
                    }
                    notifications::clear_delivered();
                    break;
                }
                _ => {}
            }
        }
    }
    terminate_gui_process();
    notifications::clear_delivered();
    Ok(())
}

fn quit_instance_request() -> anyhow::Result<()> {
    terminate_gui_process();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()?;
    let Ok(client) = runtime.block_on(DaemonClient::connect_notifications()) else {
        terminate_tray_process();
        return Ok(());
    };
    let owned_daemon = client
        .connection_info()
        .is_some_and(|info| info.lifecycle == DaemonLifecycle::Owned);
    terminate_tray_process();
    if owned_daemon {
        let _ = runtime.block_on(async {
            tokio::time::timeout(Duration::from_secs(5), client.wait_closed()).await
        });
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn initialize_macos_tray() -> anyhow::Result<()> {
    use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy};
    use objc2_foundation::MainThreadMarker;

    let mtm = MainThreadMarker::new()
        .ok_or_else(|| anyhow::anyhow!("tray must be initialized on the main thread"))?;
    let app = NSApplication::sharedApplication(mtm);
    app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);
    app.finishLaunching();
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn initialize_macos_tray() -> anyhow::Result<()> {
    Ok(())
}

#[cfg(target_os = "macos")]
fn pump_macos_events() {
    use objc2_app_kit::{NSApplication, NSEventMask};
    use objc2_foundation::{MainThreadMarker, NSDate, NSDefaultRunLoopMode};

    let Some(mtm) = MainThreadMarker::new() else {
        return;
    };
    let app = NSApplication::sharedApplication(mtm);
    let until = NSDate::dateWithTimeIntervalSinceNow(0.0);
    while let Some(event) = app.nextEventMatchingMask_untilDate_inMode_dequeue(
        NSEventMask::Any,
        Some(&until),
        unsafe { NSDefaultRunLoopMode },
        true,
    ) {
        app.sendEvent(&event);
    }
}

#[cfg(not(target_os = "macos"))]
fn pump_macos_events() {}

fn show_message(title: &str, description: &str) {
    rfd::MessageDialog::new()
        .set_title(title)
        .set_description(description)
        .set_buttons(rfd::MessageButtons::Ok)
        .show();
}

fn download_directory() -> Option<std::path::PathBuf> {
    directories::UserDirs::new().and_then(|dirs| dirs.download_dir().map(Into::into))
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

fn should_notify_received_file(entry: &localsend_rpc::HistoryEntry) -> bool {
    entry.direction == TransferDirection::Receive
        && entry.status == TransferStatus::Finished
        && entry.message.is_none()
}

fn route_gui_action(action: GuiAction) {
    if send_gui_control_action(&action).is_ok() {
        return;
    }
    if gui_process_is_running() && gui_pid_is_stale() {
        tracing::warn!("GUI control socket unavailable; replacing the GUI process");
        terminate_gui_process();
    }
    if let Err(error) = queue_gui_action(action) {
        tracing::warn!("could not queue GUI action: {error}");
    }
    if let Err(error) = open_app(false) {
        show_message("LocalSend", &error.to_string());
    }
}

fn open_app(start_minimized: bool) -> anyhow::Result<()> {
    let current = std::env::current_exe()?;
    let executable = find_gui_executable(&current)
        .ok_or_else(|| anyhow::anyhow!("LocalSend GUI executable was not found"))?;

    #[cfg(target_os = "macos")]
    if let Some(bundle) = app_bundle_for(&executable) {
        let mut command = Command::new("/usr/bin/open");
        command
            .arg("-n")
            .arg("-a")
            .arg(&bundle)
            .arg("--args")
            .arg("--window");
        if start_minimized {
            command.arg("--minimized");
        }
        let output = command.output()?;
        if !output.status.success() {
            anyhow::bail!(
                "macOS could not launch the LocalSend window: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }
        tracing::info!(bundle = %bundle.display(), "LocalSend GUI launch requested");
        return Ok(());
    }

    let mut command = Command::new(&executable);
    command.arg("--window");
    if start_minimized {
        command.arg("--minimized");
    }
    command.stdin(Stdio::null());
    if let Some((stdout, stderr)) = gui_log_streams() {
        command.stdout(stdout).stderr(stderr);
    } else {
        command.stdout(Stdio::null()).stderr(Stdio::null());
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        use std::os::unix::process::CommandExt;
        unsafe {
            command.pre_exec(|| {
                let _ = libc::setsid();
                Ok(())
            });
        }
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }
    let mut child = command.spawn()?;
    let pid = child.id();
    tracing::info!(path = %executable.display(), pid, "LocalSend GUI launch requested");
    std::thread::spawn(move || match child.wait() {
        Ok(status) => tracing::warn!(pid, %status, "LocalSend GUI exited"),
        Err(error) => tracing::warn!(pid, %error, "could not wait for LocalSend GUI"),
    });
    Ok(())
}

fn gui_pid_is_stale() -> bool {
    std::fs::metadata(gui_pid_path())
        .and_then(|metadata| metadata.modified())
        .ok()
        .and_then(|modified| modified.elapsed().ok())
        .is_some_and(|age| age > Duration::from_secs(2))
}

fn gui_log_streams() -> Option<(Stdio, Stdio)> {
    let config_dir = config_dir()?;
    let log_dir = config_dir.join("logs");
    std::fs::create_dir_all(&log_dir).ok()?;
    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_dir.join("localsend-gui.log"))
        .ok()?;
    let stderr = file.try_clone().ok()?;
    Some((Stdio::from(file), Stdio::from(stderr)))
}

#[cfg(target_os = "macos")]
fn app_bundle_for(executable: &std::path::Path) -> Option<std::path::PathBuf> {
    executable
        .ancestors()
        .find(|path| path.extension().is_some_and(|extension| extension == "app"))
        .map(std::path::Path::to_path_buf)
}

fn find_gui_executable(current: &std::path::Path) -> Option<std::path::PathBuf> {
    let gui_name = if cfg!(windows) {
        "LocalSend.exe"
    } else if cfg!(target_os = "macos") {
        "LocalSend"
    } else {
        "localsend"
    };
    let dev_gui_name = if cfg!(windows) {
        "localsend-gui-slint.exe"
    } else {
        "localsend-gui-slint"
    };
    let mut candidates = Vec::new();
    #[cfg(target_os = "macos")]
    if let Some(main_app) = current
        .ancestors()
        .filter(|path| path.extension().is_some_and(|extension| extension == "app"))
        .nth(1)
    {
        candidates.push(main_app.join("Contents/MacOS").join(gui_name));
    }
    if let Some(parent) = current.parent() {
        candidates.push(parent.join(gui_name));
        candidates.push(parent.join("LocalSend"));
        candidates.push(parent.join(dev_gui_name));
    }
    candidates.push(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/debug")
            .join(dev_gui_name),
    );
    candidates.into_iter().find(|path| path.exists())
}

async fn connect_or_start_daemon() -> anyhow::Result<DaemonClient> {
    if let Ok(client) = DaemonClient::connect_notifications().await {
        return Ok(client);
    }
    let current = std::env::current_exe()?;
    let daemon_name = if cfg!(windows) {
        "localsendd.exe"
    } else {
        "localsendd"
    };
    let executable =
        find_daemon_executable(&current, daemon_name).unwrap_or_else(|| daemon_name.into());
    let mut command = Command::new(executable);
    command.arg("--require-owner").stdin(Stdio::null());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        unsafe {
            command.pre_exec(|| {
                let _ = libc::setsid();
                Ok(())
            });
        }
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    let mut child = command.spawn()?;
    for _ in 0..50 {
        if let Ok(client) = DaemonClient::connect_owned_notifications().await {
            return Ok(client);
        }
        if let Some(status) = child.try_wait()? {
            anyhow::bail!("localsendd exited while starting: {status}");
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    let _ = child.kill();
    let _ = child.wait();
    anyhow::bail!("timed out starting localsendd")
}

fn find_daemon_executable(
    current: &std::path::Path,
    daemon_name: &str,
) -> Option<std::path::PathBuf> {
    let mut candidates = Vec::new();
    #[cfg(target_os = "macos")]
    if let Some(main_app) = current
        .ancestors()
        .filter(|path| path.extension().is_some_and(|extension| extension == "app"))
        .nth(1)
    {
        candidates.push(main_app.join("Contents/MacOS").join(daemon_name));
    }
    if let Some(parent) = current.parent() {
        candidates.push(parent.join(daemon_name));
    }
    candidates.into_iter().find(|path| path.exists())
}

async fn reconnect_daemon(previously_owned: bool) -> anyhow::Result<DaemonClient> {
    for attempt in 0..100 {
        match connect_or_start_daemon().await {
            Ok(client) => {
                let still_owned_daemon = client
                    .connection_info()
                    .is_some_and(|info| info.lifecycle == DaemonLifecycle::Owned)
                    && !client.owns_lifecycle();
                if !previously_owned || !still_owned_daemon {
                    return Ok(client);
                }
                drop(client);
            }
            Err(error) => tracing::warn!("daemon reconnect attempt failed: {error}"),
        }
        if attempt < 99 {
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    }
    anyhow::bail!("timed out reconnecting to localsendd")
}

fn main() -> anyhow::Result<()> {
    localsend_platform::set_current_process_name("LocalSendTray");
    init_logging();
    run_tray()
}

fn init_logging() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    if let Some(log) = config_dir()
        .and_then(|dir| std::fs::create_dir_all(dir.join("logs")).ok().map(|_| dir))
        .and_then(|dir| {
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(dir.join("logs/localsend-tray.log"))
                .ok()
        })
        && tracing_subscriber::fmt()
            .with_env_filter(filter.clone())
            .with_target(false)
            .with_writer(log)
            .try_init()
            .is_ok()
    {
        tracing::info!(pid = std::process::id(), "localsend-tray started");
        return;
    }
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .try_init();
    tracing::info!(pid = std::process::id(), "localsend-tray started");
}

fn config_dir() -> Option<std::path::PathBuf> {
    std::env::var_os("LOCALSEND_CONFIG_DIR")
        .map(std::path::PathBuf::from)
        .or_else(|| {
            directories::ProjectDirs::from("org", "LocalSend", "localsend-rs")
                .map(|dirs| dirs.config_dir().to_path_buf())
        })
}

fn tray_icon_rgba(size: u32) -> Vec<u8> {
    let mut pixels = Vec::with_capacity((size * size * 4) as usize);
    let center = (size as f32 - 1.0) / 2.0;
    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            let distance = (dx * dx + dy * dy).sqrt();
            let outer = distance <= center * 0.9;
            let inner = distance > center * 0.52;
            let hub = distance <= center * 0.3;
            let ring = outer && inner;
            let (red, green, blue, alpha) = if !(ring || hub) {
                (0, 0, 0, 0)
            } else if cfg!(target_os = "macos") {
                (0, 0, 0, 255)
            } else {
                (15, 118, 110, 255)
            };
            pixels.extend_from_slice(&[red, green, blue, alpha]);
        }
    }
    pixels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packaged_tray_finds_sibling_gui_executable() {
        let root = tempfile::tempdir().expect("temp root");
        let macos = root.path().join("Contents/MacOS");
        std::fs::create_dir_all(&macos).expect("create macOS dir");
        let tray = macos.join("LocalSendTray");
        let gui = macos.join("LocalSend");
        std::fs::write(&tray, b"tray").expect("write tray");
        std::fs::write(&gui, b"gui").expect("write gui");
        assert_eq!(find_gui_executable(&tray), Some(gui));
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn helper_bundle_finds_gui_in_enclosing_app() {
        let root = tempfile::tempdir().expect("temp root");
        let main_app = root.path().join("LocalSend.app");
        let helper_macos = main_app.join("Contents/Helpers/LocalSendTray.app/Contents/MacOS");
        std::fs::create_dir_all(&helper_macos).expect("create helper macOS dir");
        let tray = helper_macos.join("LocalSendTray");
        let gui = main_app.join("Contents/MacOS/LocalSend");
        std::fs::create_dir_all(gui.parent().expect("gui parent")).expect("create GUI dir");
        std::fs::write(&tray, b"tray").expect("write tray");
        std::fs::write(&gui, b"gui").expect("write gui");
        assert_eq!(find_gui_executable(&tray), Some(gui));
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn helper_bundle_finds_daemon_in_enclosing_app() {
        let root = tempfile::tempdir().expect("temp root");
        let main_app = root.path().join("LocalSend.app");
        let helper_macos = main_app.join("Contents/Helpers/LocalSendTray.app/Contents/MacOS");
        std::fs::create_dir_all(&helper_macos).expect("create helper macOS dir");
        let tray = helper_macos.join("LocalSendTray");
        let daemon = main_app.join("Contents/MacOS/localsendd");
        std::fs::create_dir_all(daemon.parent().expect("daemon parent"))
            .expect("create daemon dir");
        std::fs::write(&tray, b"tray").expect("write tray");
        std::fs::write(&daemon, b"daemon").expect("write daemon");
        assert_eq!(find_daemon_executable(&tray, "localsendd"), Some(daemon));
    }

    #[test]
    fn tray_language_honors_explicit_settings() {
        assert_eq!(
            TrayLanguage::from_settings("zh-CN"),
            TrayLanguage::SimplifiedChinese
        );
        assert_eq!(TrayLanguage::from_settings("en-US"), TrayLanguage::English);
        assert_eq!(
            TrayLanguage::from_settings("zh-CN").open_app(),
            "打开主界面"
        );
        assert_eq!(TrayLanguage::from_settings("zh-CN").restart_app(), "重启");
        assert_eq!(TrayLanguage::from_settings("zh-CN").quit_app(), "退出");
    }

    #[test]
    fn only_successful_non_message_history_entries_create_file_notifications() {
        let base = localsend_rpc::HistoryEntry {
            id: "history".to_string(),
            file_id: None,
            direction: TransferDirection::Receive,
            peer_alias: "peer".to_string(),
            file_name: "file.txt".to_string(),
            path: Some("/tmp/file.txt".into()),
            size: 1,
            timestamp_ms: 0,
            status: TransferStatus::Finished,
            message: None,
        };
        assert!(should_notify_received_file(&base));
        assert!(!should_notify_received_file(&localsend_rpc::HistoryEntry {
            message: Some("hello".to_string()),
            ..base.clone()
        }));
        assert!(!should_notify_received_file(&localsend_rpc::HistoryEntry {
            status: TransferStatus::Failed,
            ..base.clone()
        }));
        assert!(!should_notify_received_file(&localsend_rpc::HistoryEntry {
            direction: TransferDirection::Send,
            ..base
        }));
    }
}

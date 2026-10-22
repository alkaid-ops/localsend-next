#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use anyhow::Context;
use localsend_platform::{
    InstanceLock, TrayPidGuard, scoped_lock_name, terminate_gui_process, terminate_tray_process,
};
use localsend_rpc::{DaemonClient, DaemonLifecycle, ServiceCommand, ServiceSnapshot};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use tray_icon::menu::{Menu, MenuEvent, MenuItem};
use tray_icon::{Icon, TrayIconBuilder};

const OPEN_APP: &str = "localsend.open-app";
const OPEN_DOWNLOADS: &str = "localsend.open-downloads";
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

    fn from_snapshot(snapshot: Option<&ServiceSnapshot>) -> Self {
        snapshot
            .map(|snapshot| Self::from_settings(&snapshot.settings.language))
            .unwrap_or_else(|| Self::from_settings("system"))
    }

    fn open_app(self) -> &'static str {
        match self {
            Self::English => "Open main window",
            Self::SimplifiedChinese => "打开主界面",
        }
    }

    fn open_downloads(self) -> &'static str {
        match self {
            Self::English => "Open receive folder",
            Self::SimplifiedChinese => "打开接收文件夹",
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

    fn active_transfers_confirm(self) -> &'static str {
        match self {
            Self::English => "Active transfers will be cancelled. Continue?",
            Self::SimplifiedChinese => "有活动传输将被取消，是否继续？",
        }
    }

    fn daemon_unavailable(self) -> &'static str {
        match self {
            Self::English => "Daemon unavailable",
            Self::SimplifiedChinese => "后台服务不可用",
        }
    }

    fn server_running(self, port: u16) -> String {
        match self {
            Self::English => format!("Server: running on port {port}"),
            Self::SimplifiedChinese => format!("服务器：运行中，端口 {port}"),
        }
    }

    fn server_stopped(self) -> &'static str {
        match self {
            Self::English => "Server: stopped",
            Self::SimplifiedChinese => "服务器：已停止",
        }
    }

    fn devices(self, count: usize) -> String {
        match self {
            Self::English => format!("Devices: {count}"),
            Self::SimplifiedChinese => format!("设备：{count}"),
        }
    }

    fn active_transfers(self, count: usize) -> String {
        match self {
            Self::English => format!("Active transfers: {count}"),
            Self::SimplifiedChinese => format!("活动传输：{count}"),
        }
    }
}

struct TrayMenu {
    open_app: MenuItem,
    open_downloads: MenuItem,
    restart_app: MenuItem,
    quit_app: MenuItem,
}

impl TrayMenu {
    fn new(language: TrayLanguage) -> anyhow::Result<(Self, Menu)> {
        let items = Self {
            open_app: MenuItem::with_id(OPEN_APP, language.open_app(), true, None),
            open_downloads: MenuItem::with_id(
                OPEN_DOWNLOADS,
                language.open_downloads(),
                true,
                None,
            ),
            restart_app: MenuItem::with_id(RESTART_APP, language.restart_app(), true, None),
            quit_app: MenuItem::with_id(QUIT_APP, language.quit_app(), true, None),
        };
        let menu = Menu::with_items(&[
            &items.open_app,
            &items.open_downloads,
            &items.restart_app,
            &items.quit_app,
        ])?;
        Ok((items, menu))
    }

    fn apply_language(&self, language: TrayLanguage) {
        self.open_app.set_text(language.open_app());
        self.open_downloads.set_text(language.open_downloads());
        self.restart_app.set_text(language.restart_app());
        self.quit_app.set_text(language.quit_app());
    }
}

pub fn run_tray() -> anyhow::Result<()> {
    if std::env::args().any(|argument| argument == "--quit") {
        return quit_instance_request();
    }
    initialize_macos_tray()?;
    if std::env::args().any(|argument| argument == "--open-main") {
        return open_app(std::env::args().any(|argument| argument == "--minimized"));
    }
    let lock_name = scoped_lock_name("localsend-tray", "tray");
    let Some(_lock) = InstanceLock::try_acquire(&lock_name)? else {
        return Ok(());
    };
    let _tray_pid = TrayPidGuard::acquire()?;
    let runtime = tokio::runtime::Builder::new_current_thread()
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

    let mut last_snapshot = client.snapshot();
    let mut language = TrayLanguage::from_snapshot(last_snapshot.as_ref());
    let (menu_items, menu) = TrayMenu::new(language)?;
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
    update_tooltip(&tray, last_snapshot.as_ref(), language);
    let mut tooltip_due = Some(Instant::now() + Duration::from_secs(1));

    loop {
        pump_macos_events();
        last_snapshot = client.snapshot();
        let current_language = TrayLanguage::from_snapshot(last_snapshot.as_ref());
        if current_language != language {
            language = current_language;
            menu_items.apply_language(language);
            update_tooltip(&tray, last_snapshot.as_ref(), language);
            tooltip_due = Some(Instant::now() + Duration::from_secs(1));
        }
        if tooltip_due.is_some_and(|due| Instant::now() >= due) {
            last_snapshot = client.snapshot();
            update_tooltip(&tray, last_snapshot.as_ref(), language);
            tooltip_due = Some(Instant::now() + Duration::from_secs(1));
        }
        if client.is_connection_closed() {
            break;
        }
        if let Ok(event) = MenuEvent::receiver().recv_timeout(Duration::from_millis(100)) {
            match event.id.as_ref() {
                OPEN_APP => open_app(false)?,
                OPEN_DOWNLOADS => {
                    if let Some(path) = receive_directory(last_snapshot.as_ref()) {
                        open_path(&path);
                    }
                }
                RESTART_APP => {
                    if !client.owns_lifecycle() {
                        show_message(
                            language.restart_unavailable(),
                            language.externally_managed_daemon(),
                        );
                        continue;
                    }
                    if !confirm_shutdown(last_snapshot.as_ref(), language.restart_app(), language) {
                        continue;
                    }
                    let old_instance = client.instance_id();
                    cancel_active_transfers(&runtime, &client);
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
                    tooltip_due = Some(Instant::now() + Duration::from_millis(300));
                }
                QUIT_APP => {
                    if !confirm_shutdown(last_snapshot.as_ref(), language.quit_app(), language) {
                        continue;
                    }
                    terminate_gui_process();
                    cancel_active_transfers(&runtime, &client);
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
                    break;
                }
                _ => {}
            }
        }
    }
    terminate_gui_process();
    Ok(())
}

fn quit_instance_request() -> anyhow::Result<()> {
    terminate_gui_process();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let Ok(client) = runtime.block_on(DaemonClient::connect()) else {
        return Ok(());
    };
    if client
        .connection_info()
        .is_some_and(|info| info.lifecycle == DaemonLifecycle::Owned)
    {
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
    terminate_tray_process();
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

fn confirm_shutdown(
    snapshot: Option<&ServiceSnapshot>,
    title: &str,
    language: TrayLanguage,
) -> bool {
    let has_transfers = snapshot.is_some_and(|snapshot| !snapshot.active_transfers.is_empty());
    if !has_transfers {
        return true;
    }
    if let Ok(value) = std::env::var("LOCALSEND_TEST_CONFIRM_QUIT") {
        return matches!(value.as_str(), "accept" | "yes" | "1");
    }
    rfd::MessageDialog::new()
        .set_title(title)
        .set_description(language.active_transfers_confirm())
        .set_buttons(rfd::MessageButtons::YesNo)
        .show()
        == rfd::MessageDialogResult::Yes
}

fn show_message(title: &str, description: &str) {
    rfd::MessageDialog::new()
        .set_title(title)
        .set_description(description)
        .set_buttons(rfd::MessageButtons::Ok)
        .show();
}

fn cancel_active_transfers(runtime: &tokio::runtime::Runtime, client: &DaemonClient) {
    let Some(snapshot) = client.snapshot() else {
        return;
    };
    for transfer in snapshot.active_transfers {
        let _ = runtime.block_on(client.command(ServiceCommand::CancelTransfer {
            transfer_id: transfer.transfer_id,
        }));
    }
}

fn update_tooltip(
    tray: &tray_icon::TrayIcon,
    snapshot: Option<&ServiceSnapshot>,
    language: TrayLanguage,
) {
    let text = snapshot
        .map(|snapshot| {
            format!(
                "LocalSend PC\n{}\n{}\n{}",
                if snapshot.server_running {
                    language.server_running(snapshot.server_port.unwrap_or_default())
                } else {
                    language.server_stopped().to_string()
                },
                language.devices(snapshot.devices.len()),
                language.active_transfers(snapshot.active_transfers.len())
            )
        })
        .unwrap_or_else(|| format!("LocalSend PC\n{}", language.daemon_unavailable()));
    let _ = tray.set_tooltip(Some(text));
}

fn receive_directory(snapshot: Option<&ServiceSnapshot>) -> Option<std::path::PathBuf> {
    snapshot
        .and_then(|snapshot| snapshot.settings.destination_dir.clone())
        .or_else(|| {
            directories::UserDirs::new().and_then(|dirs| dirs.download_dir().map(Into::into))
        })
}

fn open_app(start_minimized: bool) -> anyhow::Result<()> {
    let current = std::env::current_exe()?;
    let executable = find_gui_executable(&current)
        .ok_or_else(|| anyhow::anyhow!("LocalSend GUI executable was not found"))?;
    let mut command = Command::new(executable);
    command.arg("--window");
    if start_minimized {
        command.arg("--minimized");
    }
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
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
        command.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }
    command.spawn()?;
    Ok(())
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
    current
        .parent()
        .map(|parent| {
            [
                parent.join(gui_name),
                parent.join("LocalSend"),
                parent.join(dev_gui_name),
            ]
        })
        .into_iter()
        .flatten()
        .chain(std::iter::once(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("target/debug")
                .join(dev_gui_name),
        ))
        .find(|path| path.exists())
}

async fn connect_or_start_daemon() -> anyhow::Result<DaemonClient> {
    if let Ok(client) = DaemonClient::connect().await {
        return Ok(client);
    }
    let current = std::env::current_exe()?;
    let daemon_name = if cfg!(windows) {
        "localsendd.exe"
    } else {
        "localsendd"
    };
    let executable = current
        .parent()
        .map(|parent| parent.join(daemon_name))
        .filter(|path| path.exists())
        .unwrap_or_else(|| daemon_name.into());
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
        if let Ok(client) = DaemonClient::connect_owned_interactive().await {
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

fn main() -> anyhow::Result<()> {
    localsend_platform::set_current_process_name("LocalSendTray");
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(false)
        .init();
    run_tray()
}

fn open_path(path: &std::path::Path) {
    #[cfg(target_os = "macos")]
    let _ = Command::new("open").arg(path).spawn();
    #[cfg(target_os = "windows")]
    let _ = Command::new("explorer").arg(path).spawn();
    #[cfg(all(unix, not(target_os = "macos")))]
    let _ = Command::new("xdg-open").arg(path).spawn();
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
        assert_eq!(
            TrayLanguage::from_settings("en-US").active_transfers(2),
            "Active transfers: 2"
        );
    }
}

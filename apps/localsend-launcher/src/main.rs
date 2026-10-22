#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use anyhow::Context;
use localsend_platform::{InstanceLock, scoped_lock_name};
use localsend_rpc::transport;
use localsend_rpc::{DaemonClient, ServiceCommand};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

const START_TIMEOUT: Duration = Duration::from_secs(8);
const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(false)
        .init();

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    if std::env::args().any(|argument| argument == "--background") {
        return runtime.block_on(start_background_daemon());
    }

    let identity = transport::endpoint_identity();
    let lock_name = scoped_lock_name("localsend-launcher", &identity);
    let Some(_lock) = InstanceLock::try_acquire(&lock_name)? else {
        return Ok(());
    };

    let (mut owned_daemon, client) = runtime.block_on(connect_or_start_daemon())?;
    let gui = find_sibling_executable(&["localsend", "localsend-gui-slint", "LocalSend"])
        .context("LocalSend GUI executable was not found next to localsend-launcher")?;
    let mut gui_command = Command::new(gui);
    gui_command.arg("--window");
    if let Some(endpoint) = transport::configured_endpoint() {
        gui_command.arg("--endpoint").arg(endpoint);
    }
    configure_detached(&mut gui_command);
    gui_command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let mut gui_child = gui_command.spawn().context("starting LocalSend GUI")?;
    let gui_status = gui_child.wait().context("waiting for LocalSend GUI")?;

    if let Some(mut daemon) = owned_daemon.take() {
        let _ = runtime.block_on(async {
            tokio::time::timeout(SHUTDOWN_TIMEOUT, client.command(ServiceCommand::Shutdown)).await
        });
        let _ = runtime
            .block_on(async { tokio::time::timeout(SHUTDOWN_TIMEOUT, client.wait_closed()).await });
        wait_or_kill(&mut daemon, SHUTDOWN_TIMEOUT);
    }

    ensure_success(gui_status, "LocalSend GUI")
}

async fn start_background_daemon() -> anyhow::Result<()> {
    if DaemonClient::connect().await.is_ok() {
        return Ok(());
    }
    let mut command = daemon_command(false)?;
    let mut child = command.spawn().context("starting background localsendd")?;
    match wait_for_daemon(&mut child).await {
        Ok(()) => Ok(()),
        Err(error) => {
            let _ = child.kill();
            let _ = child.wait();
            Err(error)
        }
    }
}

async fn connect_or_start_daemon() -> anyhow::Result<(Option<Child>, DaemonClient)> {
    if let Ok(client) = DaemonClient::connect().await {
        return Ok((None, client));
    }

    let mut child = daemon_command(true)?
        .spawn()
        .context("starting owned localsendd")?;
    match wait_for_owned_client(&mut child).await {
        Ok(client) => Ok((Some(child), client)),
        Err(error) => {
            let _ = child.kill();
            let _ = child.wait();
            Err(error)
        }
    }
}

fn daemon_command(require_owner: bool) -> anyhow::Result<Command> {
    let current = std::env::current_exe()?;
    let daemon = find_sibling_executable_in(current.parent(), &["localsendd", "localsendd.exe"])
        .context("localsendd executable was not found next to localsend-launcher")?;
    let mut command = Command::new(daemon);
    if require_owner {
        command.arg("--require-owner");
    }
    if let Some(endpoint) = transport::configured_endpoint() {
        command.arg("--endpoint").arg(endpoint);
    }
    configure_detached(&mut command);
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    Ok(command)
}

async fn wait_for_daemon(child: &mut Child) -> anyhow::Result<()> {
    let deadline = Instant::now() + START_TIMEOUT;
    loop {
        if DaemonClient::connect().await.is_ok() {
            return Ok(());
        }
        if let Some(status) = child.try_wait()? {
            anyhow::bail!("localsendd exited before becoming ready: {status}");
        }
        if Instant::now() >= deadline {
            anyhow::bail!("timed out waiting for localsendd endpoint");
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn wait_for_owned_client(child: &mut Child) -> anyhow::Result<DaemonClient> {
    let deadline = Instant::now() + START_TIMEOUT;
    loop {
        match DaemonClient::connect_owned_interactive().await {
            Ok(client) => return Ok(client),
            Err(_) => {
                if let Some(status) = child.try_wait()? {
                    anyhow::bail!("localsendd exited before accepting its owner: {status}");
                }
                if Instant::now() >= deadline {
                    anyhow::bail!("timed out waiting for owned localsendd");
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }
}

fn find_sibling_executable(names: &[&str]) -> Option<PathBuf> {
    let current = std::env::current_exe().ok()?;
    find_sibling_executable_in(current.parent(), names)
}

fn find_sibling_executable_in(parent: Option<&Path>, names: &[&str]) -> Option<PathBuf> {
    let parent = parent?;
    names
        .iter()
        .map(|name| parent.join(name))
        .find(|path| path.is_file())
}

fn configure_detached(command: &mut Command) {
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
}

fn wait_or_kill(child: &mut Child, timeout: Duration) {
    let deadline = Instant::now() + timeout;
    loop {
        match child.try_wait() {
            Ok(Some(_)) => return,
            Ok(None) if Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(50));
            }
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                return;
            }
        }
    }
}

fn ensure_success(status: ExitStatus, process: &str) -> anyhow::Result<()> {
    if status.success() {
        Ok(())
    } else {
        anyhow::bail!("{process} exited with {status}")
    }
}

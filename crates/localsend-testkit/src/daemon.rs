use anyhow::Context;
use localsend_rpc::{DaemonClient, DesktopSettings, Protocol, ServiceCommand, ServiceSnapshot};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use tempfile::TempDir;
use tokio::time::{Duration, Instant};

pub struct DaemonFixture {
    root: TempDir,
    child: Child,
    daemon_binary: PathBuf,
    endpoint: PathBuf,
    config_dir: PathBuf,
    destination: PathBuf,
}

impl DaemonFixture {
    pub async fn start(daemon_binary: &Path, settings: DesktopSettings) -> anyhow::Result<Self> {
        let root = tempfile::tempdir().context("creating daemon test directory")?;
        let config_dir = root.path().join("config");
        let destination = root.path().join("received");
        std::fs::create_dir_all(&config_dir)?;
        std::fs::create_dir_all(&destination)?;
        let endpoint = root.path().join("daemon.sock");
        let mut settings = settings;
        settings.port = 0;
        settings.destination_dir = Some(destination.clone());
        settings.start_server_on_launch = true;
        std::fs::write(
            config_dir.join("settings.json"),
            serde_json::to_vec_pretty(&settings)?,
        )?;
        let child = Command::new(daemon_binary)
            .env("LOCALSEND_CONFIG_DIR", &config_dir)
            .env("LOCALSEND_RPC_SOCKET", &endpoint)
            .env("RUST_LOG", "info")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .with_context(|| format!("spawning {}", daemon_binary.display()))?;
        let fixture = Self {
            root,
            child,
            daemon_binary: daemon_binary.to_path_buf(),
            endpoint,
            config_dir,
            destination,
        };
        fixture.wait_ready().await?;
        Ok(fixture)
    }

    pub fn endpoint(&self) -> &Path {
        &self.endpoint
    }

    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }

    pub fn destination(&self) -> &Path {
        &self.destination
    }

    pub fn root(&self) -> &Path {
        self.root.path()
    }

    pub async fn connect(&self) -> anyhow::Result<DaemonClient> {
        DaemonClient::connect_to(self.endpoint.to_string_lossy()).await
    }

    pub async fn connect_interactive(&self) -> anyhow::Result<DaemonClient> {
        DaemonClient::connect_to_interactive(self.endpoint.to_string_lossy()).await
    }

    pub async fn snapshot(&self) -> anyhow::Result<ServiceSnapshot> {
        let client = self.connect().await?;
        match client.command(ServiceCommand::GetSnapshot).await {
            Ok(localsend_rpc::CommandResult::Snapshot { snapshot }) => Ok(snapshot),
            Ok(other) => anyhow::bail!("unexpected snapshot response: {other:?}"),
            Err(error) => anyhow::bail!(error),
        }
    }

    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        if let Ok(client) = self.connect().await {
            let _ = client.command(ServiceCommand::Shutdown).await;
        }
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if self.child.try_wait()?.is_some() {
                return Ok(());
            }
            if Instant::now() >= deadline {
                anyhow::bail!("daemon did not exit within five seconds");
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }

    pub fn kill(&mut self) -> anyhow::Result<()> {
        if self.child.try_wait()?.is_none() {
            self.child.kill()?;
            let _ = self.child.wait();
        }
        Ok(())
    }

    pub async fn restart(&mut self) -> anyhow::Result<()> {
        self.kill()?;
        self.child = spawn_process(&self.daemon_binary, &self.config_dir, &self.endpoint)?;
        self.wait_ready().await
    }

    async fn wait_ready(&self) -> anyhow::Result<()> {
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            match DaemonClient::connect_to(self.endpoint.to_string_lossy()).await {
                Ok(client) => {
                    if let Ok(localsend_rpc::CommandResult::Snapshot { snapshot }) =
                        client.command(ServiceCommand::GetSnapshot).await
                        && snapshot.server_running
                    {
                        return Ok(());
                    }
                }
                Err(_) if Instant::now() < deadline => {}
                Err(error) => return Err(error),
            }
            anyhow::ensure!(Instant::now() < deadline, "daemon did not become ready");
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }
}

fn spawn_process(
    daemon_binary: &Path,
    config_dir: &Path,
    endpoint: &Path,
) -> anyhow::Result<Child> {
    Command::new(daemon_binary)
        .env("LOCALSEND_CONFIG_DIR", config_dir)
        .env("LOCALSEND_RPC_SOCKET", endpoint)
        .env("RUST_LOG", "info")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("spawning {}", daemon_binary.display()))
}

impl Drop for DaemonFixture {
    fn drop(&mut self) {
        if self.child.try_wait().ok().flatten().is_none() {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

pub fn daemon_binary_for_tests() -> anyhow::Result<PathBuf> {
    if let Some(path) = std::env::var_os("CARGO_BIN_EXE_localsendd") {
        return Ok(path.into());
    }
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let name = if cfg!(windows) {
        "localsendd.exe"
    } else {
        "localsendd"
    };
    let candidate = workspace.join("target/debug").join(name);
    anyhow::ensure!(
        candidate.is_file(),
        "localsendd is not built; run `cargo build -p localsendd` before acceptance tests"
    );
    Ok(candidate)
}

pub fn default_daemon_settings() -> DesktopSettings {
    DesktopSettings {
        protocol: Protocol::Http,
        auto_save: localsend_rpc::AutoSaveMode::Off,
        ..DesktopSettings::default()
    }
}

pub fn write_settings(config_dir: &Path, settings: &DesktopSettings) -> anyhow::Result<()> {
    std::fs::write(
        config_dir.join("settings.json"),
        serde_json::to_vec_pretty(settings)?,
    )?;
    Ok(())
}

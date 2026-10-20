use crate::types::{DaemonLifecycle, RPC_VERSION};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;
use tokio::io::{AsyncRead, AsyncWrite};

pub trait LocalIo: AsyncRead + AsyncWrite + Unpin + Send {}

impl<T> LocalIo for T where T: AsyncRead + AsyncWrite + Unpin + Send {}

pub type BoxedIo = Box<dyn LocalIo>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeManifest {
    pub rpc_version: u32,
    pub endpoint: String,
    pub pid: u32,
    pub instance_id: String,
    pub lifecycle: DaemonLifecycle,
    pub config_dir: String,
    pub started_at_ms: u64,
}

pub struct RuntimeManifestGuard {
    path: PathBuf,
    instance_id: String,
}

impl Drop for RuntimeManifestGuard {
    fn drop(&mut self) {
        if let Some(manifest) = read_runtime_manifest()
            && manifest.instance_id == self.instance_id
        {
            let _ = fs::remove_file(&self.path);
        }
    }
}

pub fn endpoint_path() -> PathBuf {
    if let Some(path) = configured_endpoint() {
        return PathBuf::from(path);
    }
    if let Some(manifest) = read_runtime_manifest() {
        let path = PathBuf::from(manifest.endpoint);
        if !path.as_os_str().is_empty() {
            return path;
        }
    }
    std::env::temp_dir().join(format!("localsendd-{}.sock", user_tag()))
}

pub fn configured_endpoint() -> Option<String> {
    let mut arguments = std::env::args().skip(1);
    while let Some(argument) = arguments.next() {
        if argument == "--endpoint" {
            return arguments.next().filter(|value| !value.trim().is_empty());
        }
        if let Some(value) = argument.strip_prefix("--endpoint=") {
            return (!value.trim().is_empty()).then(|| value.to_string());
        }
    }
    std::env::var("LOCALSEND_RPC_SOCKET")
        .ok()
        .filter(|value| !value.trim().is_empty())
}

pub fn runtime_manifest_path() -> PathBuf {
    let base = std::env::var_os("LOCALSEND_CONFIG_DIR")
        .map(PathBuf::from)
        .or_else(|| {
            directories::ProjectDirs::from("org", "LocalSend", "localsend-rs")
                .map(|dirs| dirs.config_dir().to_path_buf())
        })
        .unwrap_or_else(|| std::env::temp_dir().join("localsend-rs"));
    base.join("runtime.json")
}

pub fn read_runtime_manifest() -> Option<RuntimeManifest> {
    let manifest: RuntimeManifest =
        serde_json::from_slice(&fs::read(runtime_manifest_path()).ok()?).ok()?;
    if manifest.rpc_version != RPC_VERSION || manifest.endpoint.trim().is_empty() {
        return None;
    }
    Some(manifest)
}

pub fn write_runtime_manifest(
    endpoint: &str,
    instance_id: &str,
    lifecycle: DaemonLifecycle,
) -> io::Result<RuntimeManifestGuard> {
    let path = runtime_manifest_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let manifest = RuntimeManifest {
        rpc_version: RPC_VERSION,
        endpoint: endpoint.to_string(),
        pid: std::process::id(),
        instance_id: instance_id.to_string(),
        lifecycle,
        config_dir: path
            .parent()
            .map(|parent| parent.to_string_lossy().into_owned())
            .unwrap_or_default(),
        started_at_ms: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64,
    };
    let temporary = path.with_extension(format!("tmp-{}", std::process::id()));
    fs::write(
        &temporary,
        serde_json::to_vec_pretty(&manifest).map_err(std::io::Error::other)?,
    )?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&temporary, fs::Permissions::from_mode(0o600))?;
    }
    fs::rename(&temporary, &path)?;
    Ok(RuntimeManifestGuard {
        path,
        instance_id: instance_id.to_string(),
    })
}

pub fn endpoint_identity() -> String {
    #[cfg(unix)]
    {
        endpoint_path().to_string_lossy().into_owned()
    }
    #[cfg(windows)]
    {
        endpoint_name()
    }
}

fn user_tag() -> String {
    let raw = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "default".to_string());
    raw.chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
                character
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(unix)]
pub struct LocalListener {
    inner: tokio::net::UnixListener,
    path: PathBuf,
}

#[cfg(unix)]
impl LocalListener {
    pub async fn bind() -> io::Result<Self> {
        let path = endpoint_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        if path.exists() {
            match tokio::net::UnixStream::connect(&path).await {
                Ok(_) => {
                    return Err(io::Error::new(
                        io::ErrorKind::AlreadyExists,
                        format!("daemon is already listening at {}", path.display()),
                    ));
                }
                Err(_) => {
                    let _ = std::fs::remove_file(&path);
                }
            }
        }
        let inner = tokio::net::UnixListener::bind(&path)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
        }
        Ok(Self { inner, path })
    }

    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    pub async fn accept(&self) -> io::Result<BoxedIo> {
        let (stream, _) = self.inner.accept().await?;
        Ok(Box::new(stream))
    }
}

#[cfg(unix)]
impl Drop for LocalListener {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

#[cfg(unix)]
pub async fn connect() -> io::Result<BoxedIo> {
    Ok(Box::new(
        tokio::net::UnixStream::connect(endpoint_path()).await?,
    ))
}

#[cfg(windows)]
pub struct LocalListener {
    name: String,
    pending: tokio::sync::Mutex<Option<tokio::net::windows::named_pipe::NamedPipeServer>>,
}

#[cfg(windows)]
impl LocalListener {
    pub async fn bind() -> io::Result<Self> {
        use tokio::net::windows::named_pipe::ServerOptions;

        let name = endpoint_name();
        let server = ServerOptions::new()
            .first_pipe_instance(true)
            .create(&name)?;
        Ok(Self {
            name,
            pending: tokio::sync::Mutex::new(Some(server)),
        })
    }

    pub fn path(&self) -> &str {
        &self.name
    }

    pub async fn accept(&self) -> io::Result<BoxedIo> {
        use tokio::net::windows::named_pipe::ServerOptions;

        let mut pending = self.pending.lock().await;
        let server = pending
            .take()
            .ok_or_else(|| io::Error::new(io::ErrorKind::WouldBlock, "named pipe is not ready"))?;
        server.connect().await?;
        *pending = Some(ServerOptions::new().create(&self.name)?);
        Ok(Box::new(server))
    }
}

#[cfg(windows)]
pub async fn connect() -> io::Result<BoxedIo> {
    use tokio::net::windows::named_pipe::ClientOptions;

    Ok(Box::new(ClientOptions::new().open(endpoint_name())?))
}

#[cfg(windows)]
pub fn endpoint_name() -> String {
    if let Some(path) = configured_endpoint() {
        return path;
    }
    if let Some(manifest) = read_runtime_manifest() {
        if !manifest.endpoint.trim().is_empty() {
            return manifest.endpoint;
        }
    }
    format!(r"\\.\pipe\localsendd-{}", user_tag())
}

#[cfg(windows)]
pub fn endpoint_path_for_display() -> String {
    endpoint_name()
}

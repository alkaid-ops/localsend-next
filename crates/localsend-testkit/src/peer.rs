use anyhow::Context;
use bytes::Bytes;
use localsend_core::crypto::cert::generate_self_signed;
use localsend_core::http::client::{ClientError, LsHttpClient, LsHttpClientVersion};
use localsend_core::http::dto::{PrepareUploadRequestDto, RegisterDto};
use localsend_core::http::server::common::save::FileUploadTarget;
use localsend_core::http::server::v2::{
    PrepareUploadDecisionV2, ServerEventV2, SessionEndReasonV2,
};
use localsend_core::http::server::{ServerConfigV2, TlsConfig, start_with_port};
use localsend_core::http::state::ClientInfo;
use localsend_core::model::discovery::{DeviceType, PROTOCOL_VERSION_V2, ProtocolType};
use localsend_core::model::transfer::{FileDto, FileMetadata};
use localsend_core::util::filename;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tempfile::TempDir;
use tokio::sync::{Mutex, mpsc, oneshot};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PeerFile {
    pub path: PathBuf,
    pub remote_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeerAcceptance {
    All,
    First(usize),
    Reject,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PeerStatus {
    pub requests: Vec<PeerRequest>,
    pub received_files: Vec<PeerReceivedFile>,
    pub session_end: Option<String>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PeerRequest {
    pub alias: String,
    pub session_id: String,
    pub file_names: Vec<String>,
    pub accepted_file_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PeerReceivedFile {
    pub file_id: String,
    pub file_name: String,
    pub path: Option<PathBuf>,
    pub size: u64,
    pub sha256: Option<String>,
}

pub struct ReceiverPeer {
    port: u16,
    protocol: ProtocolType,
    fingerprint: String,
    root: TempDir,
    stop_tx: Option<oneshot::Sender<()>>,
    status: Arc<Mutex<PeerStatus>>,
    handler: JoinHandle<()>,
}

impl ReceiverPeer {
    pub async fn start(
        protocol: ProtocolType,
        acceptance: PeerAcceptance,
        pin: Option<String>,
        throttle_bytes_per_second: Option<u64>,
    ) -> anyhow::Result<Self> {
        let root = tempfile::tempdir().context("creating peer directory")?;
        let cert = generate_self_signed()?;
        let identity = Arc::new(cert);
        let (event_tx, mut event_rx) = mpsc::channel(128);
        let (stop_tx, stop_rx) = oneshot::channel();
        let handle = start_with_port(
            0,
            if protocol == ProtocolType::Https {
                Some(TlsConfig {
                    cert: identity.certificate_pem.clone(),
                    private_key: identity.private_key_pem.clone(),
                })
            } else {
                None
            },
            ClientInfo {
                alias: "deterministic-peer".to_string(),
                version: PROTOCOL_VERSION_V2.to_string(),
                device_model: Some("localsend-testkit".to_string()),
                device_type: Some(DeviceType::Headless),
                token: identity.fingerprint.clone(),
            },
            None,
            Some(ServerConfigV2 {
                pin,
                verify_checksums: true,
                event_tx,
            }),
            None,
            stop_rx,
        )
        .await?;
        let port = handle.port();
        let fingerprint = identity.fingerprint.clone();
        let status = Arc::new(Mutex::new(PeerStatus::default()));
        let status_for_task = status.clone();
        let receive_dir = root.path().join("received");
        std::fs::create_dir_all(&receive_dir)?;
        let handler = tokio::spawn(async move {
            while let Some(event) = event_rx.recv().await {
                match event {
                    ServerEventV2::PrepareUpload {
                        session_id,
                        info,
                        files,
                        decision_tx,
                        ..
                    } => {
                        let file_ids = files.keys().cloned().collect::<Vec<_>>();
                        let accepted = match acceptance {
                            PeerAcceptance::All => file_ids.clone(),
                            PeerAcceptance::First(count) => {
                                file_ids.iter().take(count).cloned().collect()
                            }
                            PeerAcceptance::Reject => Vec::new(),
                        };
                        {
                            let mut status = status_for_task.lock().await;
                            status.requests.push(PeerRequest {
                                alias: info.alias,
                                session_id,
                                file_names: files
                                    .values()
                                    .map(|file| file.file_name.clone())
                                    .collect(),
                                accepted_file_ids: accepted.clone(),
                            });
                        }
                        let decision = if matches!(acceptance, PeerAcceptance::Reject) {
                            PrepareUploadDecisionV2::Decline
                        } else {
                            PrepareUploadDecisionV2::Accept(accepted.into_iter().collect())
                        };
                        let _ = decision_tx.send(decision);
                    }
                    ServerEventV2::FileUpload {
                        file_id,
                        file,
                        target_tx,
                        ..
                    } => {
                        if let Some(bytes_per_second) = throttle_bytes_per_second {
                            let (binary_tx, mut binary_rx) = mpsc::channel::<Bytes>(16);
                            let (result_tx, result_rx) = oneshot::channel();
                            if target_tx
                                .send(FileUploadTarget::Stream {
                                    binary_tx,
                                    result_rx,
                                })
                                .is_err()
                            {
                                continue;
                            }
                            let path = unique_peer_path(&receive_dir, &file.file_name);
                            let result =
                                receive_slow(&path, &mut binary_rx, bytes_per_second).await;
                            let _ = result_tx.send(result.clone());
                            let mut status = status_for_task.lock().await;
                            if let Err(error) = result {
                                status.errors.push(error);
                            } else {
                                status.received_files.push(PeerReceivedFile {
                                    file_id,
                                    file_name: file.file_name,
                                    path: Some(path),
                                    size: file.size,
                                    sha256: file.sha256,
                                });
                            }
                        } else {
                            let path = unique_peer_path(&receive_dir, &file.file_name);
                            let (result_tx, result_rx) = oneshot::channel();
                            if target_tx
                                .send(FileUploadTarget::Path {
                                    path: path.clone(),
                                    result_tx,
                                    progress_tx: None,
                                })
                                .is_err()
                            {
                                continue;
                            }
                            let result = result_rx.await.unwrap_or_else(|_| {
                                Err("receiver result channel closed".to_string())
                            });
                            let mut status = status_for_task.lock().await;
                            if let Err(error) = result {
                                status.errors.push(error);
                            } else {
                                status.received_files.push(PeerReceivedFile {
                                    file_id,
                                    file_name: file.file_name,
                                    path: Some(path),
                                    size: file.size,
                                    sha256: file.sha256,
                                });
                            }
                        }
                    }
                    ServerEventV2::SessionEnd { reason, .. } => {
                        status_for_task.lock().await.session_end = Some(match reason {
                            SessionEndReasonV2::Finished => "finished".to_string(),
                            SessionEndReasonV2::Cancelled => "cancelled".to_string(),
                        });
                    }
                    ServerEventV2::ListenerFailed { error } => {
                        status_for_task.lock().await.errors.push(error);
                    }
                    _ => {}
                }
            }
        });
        Ok(Self {
            port,
            protocol,
            fingerprint,
            root,
            stop_tx: Some(stop_tx),
            status,
            handler,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn protocol(&self) -> ProtocolType {
        self.protocol
    }

    pub fn fingerprint(&self) -> &str {
        &self.fingerprint
    }

    pub fn root(&self) -> &Path {
        self.root.path()
    }

    pub async fn status(&self) -> PeerStatus {
        self.status.lock().await.clone()
    }

    pub async fn wait_for_request(&self, timeout: Duration) -> anyhow::Result<PeerRequest> {
        wait_for(timeout, || async {
            self.status.lock().await.requests.last().cloned()
        })
        .await
    }

    pub async fn wait_for_files(
        &self,
        count: usize,
        timeout: Duration,
    ) -> anyhow::Result<Vec<PeerReceivedFile>> {
        wait_for(timeout, || async {
            let status = self.status.lock().await;
            (status.received_files.len() >= count).then(|| status.received_files.clone())
        })
        .await
    }

    pub async fn shutdown(mut self) {
        if let Some(tx) = self.stop_tx.take() {
            let _ = tx.send(());
        }
        self.handler.abort();
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SendReport {
    pub session_id: Option<String>,
    pub accepted_file_ids: Vec<String>,
    pub refused_file_ids: Vec<String>,
}

pub struct SenderPeer;

impl SenderPeer {
    pub async fn send_files(
        host: &str,
        port: u16,
        protocol: ProtocolType,
        pin: Option<&str>,
        files: Vec<PeerFile>,
    ) -> Result<SendReport, ClientError> {
        Self::send_files_with_cancel(host, port, protocol, pin, files, CancellationToken::new())
            .await
    }

    pub async fn send_files_with_cancel(
        host: &str,
        port: u16,
        protocol: ProtocolType,
        pin: Option<&str>,
        files: Vec<PeerFile>,
        cancel: CancellationToken,
    ) -> Result<SendReport, ClientError> {
        let identity = generate_self_signed()?;
        let client = LsHttpClient::new(
            &identity.private_key_pem,
            &identity.certificate_pem,
            LsHttpClientVersion::V2,
            None,
            Some(Duration::from_secs(20)),
        )?;
        let mut payload_files = HashMap::new();
        let mut paths = HashMap::new();
        for file in files {
            let metadata = std::fs::metadata(&file.path)?;
            let file_id = Uuid::new_v4().to_string();
            let sha256 = sha256_file(&file.path)?;
            payload_files.insert(
                file_id.clone(),
                FileDto {
                    id: file_id.clone(),
                    file_name: file.remote_name,
                    size: metadata.len(),
                    file_type: mime_guess::from_path(&file.path)
                        .first_or_octet_stream()
                        .to_string(),
                    sha256: Some(sha256),
                    preview: None,
                    metadata: FileMetadata::from_fs_metadata(&metadata),
                },
            );
            paths.insert(file_id, file.path);
        }
        let payload = PrepareUploadRequestDto {
            info: RegisterDto {
                alias: "deterministic-sender".to_string(),
                version: PROTOCOL_VERSION_V2.to_string(),
                device_model: Some("localsend-testkit".to_string()),
                device_type: Some(DeviceType::Headless),
                token: identity.fingerprint,
                port: 0,
                protocol,
                has_web_interface: false,
            },
            files: payload_files.clone(),
        };
        let prepared = client
            .prepare_upload(protocol, host, port, None, payload, pin, cancel.clone())
            .await?;
        let Some(response) = prepared.response else {
            return Ok(SendReport {
                session_id: None,
                accepted_file_ids: Vec::new(),
                refused_file_ids: payload_files.keys().cloned().collect(),
            });
        };
        let accepted: HashSet<_> = response.files.keys().cloned().collect();
        for (file_id, token) in &response.files {
            if let Some(path) = paths.remove(file_id) {
                client
                    .upload(
                        protocol,
                        host,
                        port,
                        None,
                        &response.session_id,
                        file_id,
                        token,
                        localsend_core::model::transfer::FileContent::Path(path),
                        |_| {},
                        cancel.clone(),
                    )
                    .await?;
            }
        }
        Ok(SendReport {
            session_id: Some(response.session_id),
            accepted_file_ids: accepted.iter().cloned().collect(),
            refused_file_ids: payload_files
                .keys()
                .filter(|id| !accepted.contains(*id))
                .cloned()
                .collect(),
        })
    }
}

async fn wait_for<T, F, Fut>(timeout: Duration, mut read: F) -> anyhow::Result<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Option<T>>,
{
    let deadline = tokio::time::Instant::now() + timeout;
    loop {
        if let Some(value) = read().await {
            return Ok(value);
        }
        anyhow::ensure!(
            tokio::time::Instant::now() < deadline,
            "condition timed out"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

fn sha256_file(path: &Path) -> anyhow::Result<String> {
    use std::io::Read;
    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

fn unique_peer_path(directory: &Path, remote_name: &str) -> PathBuf {
    let name = filename::sanitize_path(remote_name, filename::Rules::current());
    let candidate = directory.join(&name);
    if !candidate.exists() {
        return candidate;
    }
    let (stem, extension) = name
        .rsplit_once('.')
        .map(|(stem, extension)| (stem, format!(".{extension}")))
        .unwrap_or((name.as_str(), String::new()));
    (1..)
        .map(|index| directory.join(format!("{stem} ({index}){extension}")))
        .find(|path| !path.exists())
        .unwrap()
}

async fn receive_slow(
    path: &Path,
    binary_rx: &mut mpsc::Receiver<Bytes>,
    bytes_per_second: u64,
) -> Result<(), String> {
    use tokio::io::AsyncWriteExt;
    let mut file = tokio::fs::File::create(path)
        .await
        .map_err(|error| error.to_string())?;
    while let Some(bytes) = binary_rx.recv().await {
        file.write_all(&bytes)
            .await
            .map_err(|error| error.to_string())?;
        let delay = if bytes_per_second == 0 {
            Duration::ZERO
        } else {
            Duration::from_secs_f64(bytes.len() as f64 / bytes_per_second as f64)
        };
        tokio::time::sleep(delay).await;
    }
    file.flush().await.map_err(|error| error.to_string())
}

use fs2::FileExt;
use sha2::{Digest, Sha256};
use std::fs::{File, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};

/// A process-wide advisory lock that is released automatically when the
/// process exits or the value is dropped.
pub struct InstanceLock {
    file: File,
    path: PathBuf,
}

impl InstanceLock {
    pub fn try_acquire(name: &str) -> io::Result<Option<Self>> {
        let path = lock_path(name);
        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(&path)?;
        match file.try_lock_exclusive() {
            Ok(()) => Ok(Some(Self { file, path })),
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => Ok(None),
            Err(error) => Err(error),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for InstanceLock {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self.file);
    }
}

pub fn scoped_lock_name(prefix: &str, identity: &str) -> String {
    let digest = Sha256::digest(identity.as_bytes());
    let suffix = digest[..12]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{prefix}-{}-{suffix}.lock", user_tag())
}

fn lock_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(name)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scoped_names_are_stable_and_endpoint_specific() {
        let first = scoped_lock_name("localsendd-instance", "/tmp/a.sock");
        let second = scoped_lock_name("localsendd-instance", "/tmp/a.sock");
        let different = scoped_lock_name("localsendd-instance", "/tmp/b.sock");
        assert_eq!(first, second);
        assert_ne!(first, different);
    }

    #[test]
    fn instance_lock_rejects_a_second_owner_until_released() {
        let name = format!(
            "localsend-lock-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        let first = InstanceLock::try_acquire(&name)
            .unwrap()
            .expect("first lock");
        assert!(InstanceLock::try_acquire(&name).unwrap().is_none());
        drop(first);
        assert!(InstanceLock::try_acquire(&name).unwrap().is_some());
    }
}

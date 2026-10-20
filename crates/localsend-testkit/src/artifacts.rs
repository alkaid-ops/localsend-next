use anyhow::Context;
use serde::Serialize;
use std::path::{Path, PathBuf};

pub struct AcceptanceArtifacts {
    root: PathBuf,
}

impl AcceptanceArtifacts {
    pub fn create(suite: &str) -> anyhow::Result<Self> {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/acceptance")
            .join(suite)
            .join(stamp.to_string());
        std::fs::create_dir_all(&root).with_context(|| format!("creating {}", root.display()))?;
        Ok(Self { root })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn write_text(&self, name: &str, contents: &str) -> anyhow::Result<PathBuf> {
        let path = self.root.join(name);
        std::fs::write(&path, contents).with_context(|| format!("writing {}", path.display()))?;
        Ok(path)
    }

    pub fn write_json<T: Serialize>(&self, name: &str, value: &T) -> anyhow::Result<PathBuf> {
        let path = self.root.join(name);
        std::fs::write(&path, serde_json::to_vec_pretty(value)?)
            .with_context(|| format!("writing {}", path.display()))?;
        Ok(path)
    }
}

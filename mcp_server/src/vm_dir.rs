use std::fs;
use std::path::PathBuf;

// We bundle the digest at build time. User chose repo-root assets/ path.
// Adjust include_str! relative to this file's directory (src/)
// ../../assets resolves from src/ -> project root -> assets
const BUNDLED_DIGEST: &str = include_str!("../../assets/Fortnite.digest.verse");

pub fn vm_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Cannot resolve home directory")
        .join(".vm")
}

pub fn digest_path() -> PathBuf {
    vm_dir().join("Fortnite.digest.verse")
}

/// Called once at startup — idempotent, safe to call every time
pub fn ensure_vm_dir() -> anyhow::Result<()> {
    let dir = vm_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
        tracing::info!("Created ~/.vm directory at {}", dir.display());
    }

    let digest = digest_path();
    if !digest.exists() {
        fs::write(&digest, BUNDLED_DIGEST)?;
        tracing::info!("Extracted bundled digest to {}", digest.display());
    }

    Ok(())
}

/// Load digest content — always from ~/.vm
pub fn load_digest_content() -> anyhow::Result<String> {
    ensure_vm_dir()?;
    Ok(fs::read_to_string(digest_path())?)
}

use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManagedDigest {
    pub name: &'static str,
    pub bundled_content: &'static str,
}

const FORTNITE_DIGEST: ManagedDigest = ManagedDigest {
    name: "Fortnite.digest.verse",
    bundled_content: include_str!("../../assets/Fortnite.digest.verse"),
};

const UNREAL_ENGINE_DIGEST: ManagedDigest = ManagedDigest {
    name: "UnrealEngine.digest.verse",
    bundled_content: include_str!("../../assets/UnrealEngine.digest.verse"),
};

const VERSE_DIGEST: ManagedDigest = ManagedDigest {
    name: "Verse.digest.verse",
    bundled_content: include_str!("../../assets/Verse.digest.verse"),
};

const SUPPORTED_DIGESTS: [ManagedDigest; 3] = [FORTNITE_DIGEST, UNREAL_ENGINE_DIGEST, VERSE_DIGEST];

pub fn vm_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Cannot resolve home directory")
        .join(".vm")
}

pub fn supported_digests() -> &'static [ManagedDigest] {
    &SUPPORTED_DIGESTS
}

pub fn digest_path(digest: &ManagedDigest) -> PathBuf {
    vm_dir().join(digest.name)
}

/// Called once at startup — idempotent, safe to call every time
pub fn ensure_vm_dir() -> anyhow::Result<()> {
    let dir = vm_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
        tracing::info!("Created ~/.vm directory at {}", dir.display());
    }

    ensure_managed_digests()
}

pub fn ensure_managed_digests() -> anyhow::Result<()> {
    for digest in supported_digests() {
        let digest_path = digest_path(digest);
        if !digest_path.exists() {
            fs::write(&digest_path, digest.bundled_content)?;
            tracing::info!("Extracted bundled digest to {}", digest_path.display());
        }
    }

    Ok(())
}

/// Load digest content — always from ~/.vm
pub fn load_digest_content(digest: &ManagedDigest) -> anyhow::Result<String> {
    ensure_vm_dir()?;
    Ok(fs::read_to_string(digest_path(digest))?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_digests_include_expected_files() {
        let digest_names: Vec<_> = supported_digests()
            .iter()
            .map(|digest| digest.name)
            .collect();
        assert_eq!(
            digest_names,
            vec![
                "Fortnite.digest.verse",
                "UnrealEngine.digest.verse",
                "Verse.digest.verse"
            ]
        );
    }

}

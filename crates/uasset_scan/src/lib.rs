//! uasset_scan - UEFN .uasset file scanner
//!
//! This crate provides functionality for parsing UEFN ExternalActors .uasset files
//! and extracting device information including triggers, receivers, and settings.

use indexmap::IndexMap;

pub mod types;
pub mod fingerprint;
pub mod classify;

pub use types::{DeviceInfo, ScanOutput};
pub use fingerprint::Fingerprint;
pub use classify::{Classification, classify};

/// Magic number for UE5 asset files
pub const UE_MAGIC: u32 = 0x9E2A83C1;

/// Result type for scan operations
pub type Result<T> = std::result::Result<T, ScanError>;

/// Error types for scan operations
#[derive(Debug, thiserror::Error)]
pub enum ScanError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid asset magic number")]
    InvalidMagic,

    #[error("Failed to parse name map")]
    NameMapParse,

    #[error("Failed to parse asset: {0}")]
    AssetParse(String),

    #[error("Unsupported engine version")]
    UnsupportedVersion,
}

/// Scan a UEFN project directory for devices
pub fn scan_project(project_path: &std::path::Path) -> Result<ScanOutput> {
    let actors_root = project_path.join("Content").join("__ExternalActors__");

    if !actors_root.exists() {
        return Err(ScanError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("ExternalActors directory not found: {}", actors_root.display()),
        )));
    }

    let mut devices = Vec::new();
    let mut skipped = 0;
    let mut total_files = 0;

    for entry in walkdir::WalkDir::new(&actors_root)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "uasset") {
            total_files += 1;
            match parse_file(path, &actors_root) {
                Ok(Some(device)) => devices.push(device),
                Ok(None) => skipped += 1,
                Err(e) => {
                    tracing::debug!("Skipping {}: {}", path.display(), e);
                    skipped += 1;
                }
            }
        }
    }

    // Group by device type
    let mut by_type: IndexMap<String, Vec<DeviceInfo>> = IndexMap::new();
    for device in devices.clone() {
        by_type
            .entry(device.device_type.clone())
            .or_default()
            .push(device);
    }

    // Sort keys for consistent output
    by_type.sort_keys();

    Ok(ScanOutput {
        scanned_at: chrono_lite_now(),
        project_root: project_path.to_string_lossy().to_string(),
        total_files,
        total_devices: devices.len(),
        skipped,
        device_types: by_type.keys().cloned().collect(),
        by_type,
    })
}

/// Parse a single .uasset file
fn parse_file(path: &std::path::Path, _base_path: &std::path::Path) -> Result<Option<DeviceInfo>> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    if buf.len() < 64 {
        return Ok(None);
    }

    // Check magic number
    let magic = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
    if magic != UE_MAGIC {
        return Ok(None);
    }

    // TODO: Implement actual parsing using unreal_asset crate or custom parser
    // For now, return None to indicate file structure is valid but parsing not complete
    Ok(None)
}

/// Simple timestamp function (no chrono dependency needed)
fn chrono_lite_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();
    // Format as ISO 8601-like string
    let days = secs / 86400;
    let years = 1970 + days / 365;
    let remaining_days = days % 365;
    let months = remaining_days / 30 + 1;
    let day = remaining_days % 30 + 1;
    let hours = (secs % 86400) / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        years, months, day, hours, minutes, seconds
    )
}

//! uasset_scan - UEFN .uasset file scanner
//!
//! This crate provides functionality for parsing UEFN ExternalActors .uasset files
//! and extracting device information including triggers, receivers, and settings.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

// ============================================================================
// Core Types (inlined for accessibility to other modules)
// ============================================================================

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

/// Extracted device info from a single .uasset file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// Relative file path from ExternalActors root
    pub file: String,
    /// Device type (e.g., "Device_Campfire_C", "button_device")
    pub device_type: String,
    /// Display label from UEFN Details panel
    pub label: Option<String>,
    /// Trigger event names (e.g., "TriggerOnEnterRadius", "OnDisabled")
    pub triggers: Vec<String>,
    /// Receiver event names (e.g., "ReceiverLight", "EnableWhenReceiving")
    pub receivers: Vec<String>,
    /// Configured property values
    pub settings: Option<IndexMap<String, String>>,
}

impl DeviceInfo {
    /// Create a new DeviceInfo with minimal data
    pub fn new(file: String, device_type: String) -> Self {
        Self {
            file,
            device_type,
            label: None,
            triggers: Vec::new(),
            receivers: Vec::new(),
            settings: None,
        }
    }

    /// Check if this device has any Verse-accessible events
    pub fn has_events(&self) -> bool {
        !self.triggers.is_empty() || !self.receivers.is_empty()
    }
}

/// Scan output matching JS scanner format for compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanOutput {
    /// ISO 8601 timestamp of scan
    pub scanned_at: String,
    /// Root directory of scanned project
    pub project_root: String,
    /// Total .uasset files found
    pub total_files: usize,
    /// Successfully parsed devices
    pub total_devices: usize,
    /// Skipped files (non-device assets)
    pub skipped: usize,
    /// Unique device types found
    pub device_types: Vec<String>,
    /// Devices grouped by type
    pub by_type: IndexMap<String, Vec<DeviceInfo>>,
}

impl ScanOutput {
    /// Create an empty scan output
    pub fn empty(project_root: String) -> Self {
        Self {
            scanned_at: String::new(),
            project_root,
            total_files: 0,
            total_devices: 0,
            skipped: 0,
            device_types: Vec::new(),
            by_type: IndexMap::new(),
        }
    }
}

// ============================================================================
// Module Declarations
// ============================================================================

#[path = "./classify/mod.rs"]
mod classify;

#[path = "./digest/mod.rs"]
mod digest;

#[path = "./fingerprint/mod.rs"]
mod fingerprint;

#[path = "./parser/mod.rs"]
mod parser;

#[path = "./validator/mod.rs"]
mod validator;

#[path = "./wiring_validator/mod.rs"]
mod wiring_validator;

#[path = "./grapher/mod.rs"]
mod grapher;

#[path = "./template_manager/mod.rs"]
mod template_manager;

// Include the internal lib module
pub mod lib_internal;

// ============================================================================
// Public API Exports
// ============================================================================

pub use classify::{classify, Classification};
pub use digest::{
    normalize_device_name, DeviceDef, DigestIndex, Event, Method, Param, Property,
    SearchResult, SymbolKind, SymbolLocation, DigestError, DiffStats,
    ChangeKind, DigestChange, DigestDiff,
};
pub use fingerprint::Fingerprint;
pub use lib_internal::{
    DeviceInfo, ScanOutput, UE_MAGIC, ScanError, Result,
    scan_project,
    parse_file,
    chrono_lite_now,
};
pub use wiring_validator::{IssueKind, WiringIssue, WiringValidator};
pub use wiring_validator::{Severity, ValidationIssue, VerseValidator};
pub use grapher::{DeviceConnection, DeviceGrapher, GraphFormat};
pub use template_manager::{
    Template, TemplateDevice, TemplateError, TemplateManager, TemplateWire,
};

/// Scan a UEFN project directory for devices (parallel implementation)
///
/// Scans both __ExternalActors__ and __ExternalObjects__ directories for .uasset files.
pub fn scan_project(project_path: &std::path::Path) -> Result<ScanOutput> {
    use rayon::prelude::*;

    let content_root = project_path.join("Content");

    // Scan both __ExternalActors__ and __ExternalObjects__
    let scan_dirs = vec![
        content_root.join("__ExternalActors__"),
        content_root.join("__ExternalObjects__"),
    ];

    let mut all_files = Vec::new();

    for scan_dir in scan_dirs {
        if scan_dir.exists() {
            tracing::info!("Scanning directory: {}", scan_dir.display());
            let files: Vec<_> = walkdir::WalkDir::new(&scan_dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().is_some_and(|ext| ext == "uasset"))
                .map(|e| e.path().to_path_buf())
                .collect();
            tracing::info!("Found {} .uasset files in {}", files.len(), scan_dir.display());
            all_files.extend(files);
        } else {
            tracing::debug!("Directory not found: {}", scan_dir.display());
        }
    }

    if all_files.is_empty() {
        return Err(ScanError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "No __ExternalActors__ or __ExternalObjects__ directories found in {}",
                content_root.display()
            ),
        )));
    }

    let total_files = all_files.len();
    tracing::info!("Total .uasset files to scan: {}", total_files);

    // Parse in parallel using rayon
    let results: Vec<_> = all_files
        .into_par_iter()
        .map(|path| parse_file(&path, &content_root))
        .collect();

    // Include all parsed files (no skipping)
    let mut devices = Vec::new();
    let mut skipped = 0;

    for result in results {
        match result {
            Ok(Some(device)) => devices.push(device),
            Ok(None) => skipped += 1,
            Err(_) => skipped += 1,
        }
    }

    let total_devices = devices.len();

    // Group by device type
    let mut by_type: IndexMap<String, Vec<DeviceInfo>> = IndexMap::new();
    for device in devices {
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
        total_devices,
        skipped,
        device_types: by_type.keys().cloned().collect(),
        by_type,
    })
}

/// Parse a single .uasset file
fn parse_file(path: &std::path::Path, base_path: &std::path::Path) -> Result<Option<DeviceInfo>> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    // Get relative path from Content/ directory (includes __ExternalActors__ or __ExternalObjects__)
    let relative_path = path
        .strip_prefix(base_path)
        .unwrap_or(path)
        .to_string_lossy()
        .replace(r"\", "/");

    parser::parse_uasset(&buf, &relative_path)
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

//! Core device and scan types

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Magic number for UE5 asset files
pub const UE_MAGIC: u32 = 0x9E2A83C1;

/// Result type for scan operations
pub type Result<T> = std::result::Result<T, ScanError>;

/// Error types for scan operations
#[derive(Debug, Error)]
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

//! Core data types for device scanning

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

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

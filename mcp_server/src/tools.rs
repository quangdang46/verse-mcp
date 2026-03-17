//! MCP Tool definitions for Verse MCP Server
//!
//! Tools provided:
//! - scan_map_devices: Scan project for placed devices
//!
//! Note: These functions are helper implementations. The actual tool
//! execution happens in main.rs via the ServerHandler trait.

use std::path::Path;

/// Tool: Scan a UEFN project for all placed devices
#[allow(dead_code)]
pub fn scan_map_devices(project_path: &Path) -> anyhow::Result<uasset_scan::ScanOutput> {
    uasset_scan::scan_project(project_path).map_err(Into::into)
}

/// Represents an @editable field found in Verse source
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EditableField {
    /// Field name
    pub name: String,
    /// Field type
    pub type_name: String,
    /// Source file
    pub source_file: String,
    /// Line number
    pub line: usize,
    /// Default value (if any)
    pub default_value: Option<String>,
}

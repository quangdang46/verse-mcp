//! MCP Tool definitions for Verse MCP Server
//!
//! Tools provided:
//! - scan_map_devices: Scan project for placed devices
//! - get_device_props: Get device properties from digest
//! - query_digest: Search digest symbols
//! - list_editables: Parse *.verse for @editable fields
//! - scaffold_ui: Generate Verse UI code
//!
//! Note: These functions are helper implementations. The actual tool
//! execution happens in main.rs via the ServerHandler trait.

use std::path::Path;

/// Tool: Scan a UEFN project for all placed devices
#[allow(dead_code)]
pub fn scan_map_devices(project_path: &Path) -> anyhow::Result<uasset_scan::ScanOutput> {
    uasset_scan::scan_project(project_path).map_err(Into::into)
}

/// Tool: Get device properties from Fortnite.digest.verse
#[allow(dead_code)]
pub fn get_device_props<'a>(
    index: &'a uasset_scan::DigestIndex,
    device_type: &str,
) -> Option<&'a uasset_scan::DeviceDef> {
    index.get_device(device_type)
}

/// Tool: Query Fortnite.digest.verse for symbols
#[allow(dead_code)]
pub fn query_digest(
    index: &uasset_scan::DigestIndex,
    query: &str,
    search_type: &str,
) -> Vec<uasset_scan::SearchResult> {
    match search_type {
        "device" => index.search_devices(query),
        "event" => index.search_events(query),
        "method" => index.search_methods(query),
        _ => index.search_all(query),
    }
}

/// Tool: List all @editable fields in a project
#[allow(dead_code)]
pub fn list_editables(_project_path: &Path) -> anyhow::Result<Vec<EditableField>> {
    // TODO: Implement Verse file parsing
    Ok(Vec::new())
}

/// Tool: Generate Verse UI scaffolding code
#[allow(dead_code)]
pub fn scaffold_ui(_intent: &str) -> anyhow::Result<String> {
    // TODO: Implement UI scaffolding
    Ok("// UI scaffolding not yet implemented".to_string())
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

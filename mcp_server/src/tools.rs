//! MCP Tool definitions for Verse MCP Server
//!
//! Tools provided:
//! - scan_map_devices: Scan project for placed devices
//! - get_device_props: Get device properties from digest
//! - query_digest: Search digest symbols
//! - list_editables: Parse *.verse for @editable fields
//! - scaffold_ui: Generate Verse UI code
//!
//! Note: These functions are placeholders for future phases. The actual tool
//! execution happens in main.rs via the ServerHandler trait.

/// Tool: Scan a UEFN project for all placed devices
#[allow(dead_code)]
pub async fn scan_map_devices(project_path: &str) -> anyhow::Result<serde_json::Value> {
    let path = std::path::Path::new(project_path);
    let output = uasset_scan::scan_project(path)?;
    Ok(serde_json::to_value(output)?)
}

/// Tool: Get device properties from Fortnite.digest.verse
#[allow(dead_code)]
pub async fn get_device_props(_device_type: &str) -> anyhow::Result<serde_json::Value> {
    // TODO: Implement digest lookup
    Ok(serde_json::json!({
        "error": "Digest lookup not yet implemented"
    }))
}

/// Tool: Query Fortnite.digest.verse for symbols
#[allow(dead_code)]
pub async fn query_digest(_query: &str) -> anyhow::Result<serde_json::Value> {
    // TODO: Implement digest search
    Ok(serde_json::json!({
        "error": "Digest search not yet implemented"
    }))
}

/// Tool: List all @editable fields in a project
#[allow(dead_code)]
pub async fn list_editables(_project_path: &str) -> anyhow::Result<serde_json::Value> {
    // TODO: Implement Verse file parsing
    Ok(serde_json::json!({
        "error": "Verse parsing not yet implemented"
    }))
}

/// Tool: Generate Verse UI scaffolding code
#[allow(dead_code)]
pub async fn scaffold_ui(_intent: &str) -> anyhow::Result<serde_json::Value> {
    // TODO: Implement UI scaffolding
    Ok(serde_json::json!({
        "error": "UI scaffolding not yet implemented"
    }))
}

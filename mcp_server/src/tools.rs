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

/// Tool: Get device properties from the managed digest set
#[allow(dead_code)]
pub fn get_device_props<'a>(
    index: &'a uasset_scan::DigestIndex,
    device_type: &str,
) -> Option<&'a uasset_scan::DeviceDef> {
    index.get_device(device_type)
}

/// Tool: Query the managed digest set for symbols
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_digest() -> uasset_scan::DigestIndex {
        let digest = r#"
device_player_ui_device = class():
    AddWidget(Widget:widget):void
    RemoveWidget(Widget:widget):void

device_canvas_slot_device = class():
    AddWidget(Slot:canvas_slot):void
    RemoveWidget(Widget:widget):void

device_button_device = class():
    OnPressed():event():void
"#;
        uasset_scan::DigestIndex::parse(digest).unwrap()
    }

    #[test]
    fn test_query_digest_defaults_to_global_ranked_search() {
        let index = create_test_digest();

        let results = query_digest(&index, "AddWidget player ui slot", "all");
        assert!(!results.is_empty());
        assert_eq!(results[0].name, "AddWidget");
        assert_eq!(
            results[0].device.as_deref(),
            Some("device_player_ui_device")
        );
    }

    #[test]
    fn test_query_digest_supports_natural_language_query_fillers() {
        let index = create_test_digest();

        let results = query_digest(&index, "add widget to player ui slot", "all");
        assert!(!results.is_empty());
        assert_eq!(results[0].name, "AddWidget");
        assert_eq!(
            results[0].device.as_deref(),
            Some("device_player_ui_device")
        );
    }

    #[test]
    fn test_query_digest_respects_search_type() {
        let index = create_test_digest();

        let method_results = query_digest(&index, "AddWidget player ui slot", "method");
        assert!(!method_results.is_empty());
        assert_eq!(method_results[0].name, "AddWidget");

        let event_results = query_digest(&index, "pressed", "event");
        assert_eq!(event_results.len(), 1);
        assert_eq!(event_results[0].name, "OnPressed");
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

//! Graph tool handler - generate_device_graph

use std::path::PathBuf;
use std::sync::Mutex;
use std::time::SystemTime;

use rmcp::{Error, model::{CallToolRequestParam, CallToolResult, Annotated}};

use crate::handler::VerseMcpHandler;
use crate::cache::{ScanCache, get_max_mtime};
use crate::parser;

/// Handle generate_device_graph tool call
pub fn handle_generate_device_graph(
    handler: &VerseMcpHandler,
    params: &CallToolRequestParam,
) -> Result<CallToolResult, Error> {
    // Parse project_path from arguments (required)
    let scan_path = params
        .arguments
        .as_ref()
        .and_then(|args| args.get("project_path"))
        .and_then(|v| v.as_str())
        .map(PathBuf::from)
        .ok_or_else(|| Error::invalid_params("project_path is required", None))?;

    // Get format from arguments
    let format_str = params
        .arguments
        .as_ref()
        .and_then(|args| args.get("format"))
        .and_then(|v| v.as_str())
        .unwrap_or("mermaid");

    let format = match format_str {
        "dot" => crate::grapher::GraphFormat::Dot,
        _ => crate::grapher::GraphFormat::Mermaid,
    };

    // Get all devices from cache or scan
    let output = get_scan_output(handler, &scan_path)?;

    // Generate graph
    let graph = crate::grapher::DeviceGrapher::generate(&output, format);

    let result_json = serde_json::json!({
        "format": format_str,
        "graph": graph,
        "device_count": output.total_devices,
    });

    Ok(CallToolResult {
        content: vec![Annotated::text(
            serde_json::to_string_pretty(&result_json).unwrap(),
        )],
        is_error: Some(false),
    })
}

/// Get scan output from cache or perform new scan
fn get_scan_output(
    handler: &VerseMcpHandler,
    scan_path: &PathBuf,
) -> Result<crate::types::ScanOutput, Error> {
    let cache_guard = handler.cache.lock().unwrap();
    if let Some(ref cached) = *cache_guard {
        let output = cached.output.clone();
        std::mem::drop(cache_guard);
        Ok(output)
    } else {
        // Need to scan first - drop lock before scanning
        std::mem::drop(cache_guard);

        match parser::scan_project(scan_path) {
            Ok(output) => {
                // Update cache
                {
                    let mut cache_guard = handler.cache.lock().unwrap();
                    *cache_guard = Some(ScanCache {
                        max_mtime: get_max_mtime(scan_path),
                        cached_at: SystemTime::now(),
                        output: output.clone(),
                    });
                }
                Ok(output)
            }
            Err(e) => {
                let error_json = serde_json::json!({
                    "error": e.to_string(),
                    "error_type": std::any::type_name_of_val(&e)
                });
                Ok(CallToolResult {
                    content: vec![Annotated::text(
                        serde_json::to_string_pretty(&error_json).unwrap(),
                    )],
                    is_error: Some(true),
                })
            }
        }.map_err(|_| Error::internal_error("Failed to scan project".to_string(), None))
    }
}

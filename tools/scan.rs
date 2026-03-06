//! Scan tool handler - scan_map_devices

use std::path::PathBuf;
use std::sync::Mutex;
use std::time::SystemTime;

use rmcp::{Error, model::{CallToolRequestParam, CallToolResult, Annotated}};

use crate::cache::{ScanCache, get_max_mtime};
use crate::handler::VerseMcpHandler;
use crate::parser;

/// Handle scan_map_devices tool call
pub fn handle_scan_map_devices(
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

    tracing::info!("Scanning project at: {}", scan_path.display());

    // Parse force_refresh from arguments
    let force_refresh = params
        .arguments
        .as_ref()
        .and_then(|args| args.get("force_refresh"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    // Check cache with mtime-based invalidation
    let should_scan = {
        let cache_guard = handler.cache.lock().unwrap();
        if force_refresh {
            tracing::info!("Force refresh requested, bypassing cache");
            true
        } else if let Some(ref cached) = *cache_guard {
            // Check if files have changed since last scan
            let current_mtime = get_max_mtime(&scan_path);
            if current_mtime > cached.max_mtime {
                tracing::info!("Cache invalidated: files modified since last scan");
                true
            } else {
                tracing::info!("Using cached scan result");
                false
            }
        } else {
            tracing::info!("No cache found, performing fresh scan");
            true
        }
    };

    // Return cached result or perform new scan
    if !should_scan {
        let cache_guard = handler.cache.lock().unwrap();
        if let Some(ref cached) = *cache_guard {
            let json = serde_json::to_string_pretty(&cached.output)
                .map_err(|e| Error::internal_error(e.to_string(), None))?;
            return Ok(CallToolResult {
                content: vec![Annotated::text(json)],
                is_error: Some(false),
            });
        }
    }

    // Perform fresh scan
    match parser::scan_project(&scan_path) {
        Ok(output) => {
            // Update cache
            {
                let mut cache_guard = handler.cache.lock().unwrap();
                *cache_guard = Some(ScanCache {
                    max_mtime: get_max_mtime(&scan_path),
                    cached_at: SystemTime::now(),
                    output: output.clone(),
                });
            }

            let json = serde_json::to_string_pretty(&output)
                .map_err(|e| Error::internal_error(e.to_string(), None))?;
            Ok(CallToolResult {
                content: vec![Annotated::text(json)],
                is_error: Some(false),
            })
        }
        Err(e) => {
            // Return error as JSON
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
    }
}

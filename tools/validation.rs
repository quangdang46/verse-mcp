//! Validation tool handlers - validate_wiring, validate_verse

use std::path::PathBuf;

use rmcp::{Error, model::{CallToolRequestParam, CallToolResult, Annotated}};

use crate::handler::VerseMcpHandler;
use crate::wiring_validator::{VerseValidator, WiringValidator};
use crate::lib_internal;

/// Handle validate_wiring tool call
pub fn handle_validate_wiring(
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

    // Get all devices from cache or scan
    let devices = get_devices(handler, &scan_path)?;

    // Validate wiring
    let issues = WiringValidator::validate(&devices);

    let result_json = serde_json::json!({
        "total_issues": issues.len(),
        "issues": issues,
    });

    Ok(CallToolResult {
        content: vec![Annotated::text(
            serde_json::to_string_pretty(&result_json).unwrap(),
        )],
        is_error: Some(false),
    })
}

/// Handle validate_verse tool call
pub fn handle_validate_verse(
    handler: &VerseMcpHandler,
    params: &CallToolRequestParam,
) -> Result<CallToolResult, Error> {
    // Get code from arguments
    let code = params
        .arguments
        .as_ref()
        .and_then(|args| args.get("code"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if code.is_empty() {
        return Ok(CallToolResult {
            content: vec![Annotated::text(
                serde_json::to_string_pretty(&serde_json::json!({
                    "error": "code parameter is required"
                }))
                .unwrap(),
            )],
            is_error: Some(true),
        });
    }

    // Read digest index
    let digest_guard = handler.digest.read().unwrap();
    match &*digest_guard {
        Some(index) => {
            let validator = VerseValidator::new(index.clone());
            let issues = validator.validate(code);

            let result_json = serde_json::json!({
                "total_issues": issues.len(),
                "issues": issues,
            });

            Ok(CallToolResult {
                content: vec![Annotated::text(
                    serde_json::to_string_pretty(&result_json).unwrap(),
                )],
                is_error: Some(false),
            })
        }
        None => {
            Ok(CallToolResult {
                content: vec![Annotated::text(serde_json::to_string_pretty(&serde_json::json!({
                    "error": "Digest not loaded. Ensure Fortnite.digest.verse is in the project directory."
                })).unwrap())],
                is_error: Some(true),
            })
        }
    }
}

/// Get devices from cache or scan
fn get_devices(handler: &VerseMcpHandler, scan_path: &PathBuf) -> Result<Vec<crate::DeviceInfo>, Error> {
    let cache_guard = handler.cache.lock().unwrap();
    if let Some(ref cached) = *cache_guard {
        let all_devices: Vec<_> = cached.output.by_type.values().flatten().cloned().collect();
        std::mem::drop(cache_guard);
        Ok(all_devices)
    } else {
        // Need to scan first - drop lock before scanning
        std::mem::drop(cache_guard);

        match crate::lib::scan_project(scan_path) {
            Ok(output) => {
                let devices: Vec<_> = output.by_type.values().flatten().cloned().collect();
                Ok(devices)
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
        }.map_err(|_| Error::internal_error("Failed to get devices".to_string(), None))
    }
}

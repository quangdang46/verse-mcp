//! Digest query tool handlers - get_device_props, query_digest, diff_digests

use rmcp::{Error, model::{CallToolRequestParam, CallToolResult, Annotated}};

use crate::handler::VerseMcpHandler;

/// Handle get_device_props tool call
pub fn handle_get_device_props(
    handler: &VerseMcpHandler,
    params: &CallToolRequestParam,
) -> Result<CallToolResult, Error> {
    // Get device_type from arguments
    let device_type = params
        .arguments
        .as_ref()
        .and_then(|args| args.get("device_type"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if device_type.is_empty() {
        return Ok(CallToolResult {
            content: vec![Annotated::text(
                serde_json::to_string_pretty(&serde_json::json!({
                    "error": "device_type parameter is required"
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
            match index.get_device(device_type) {
                Some(device) => {
                    // Separate triggers and receivers
                    let triggers: Vec<_> = device.events.iter()
                        .filter(|e| !e.is_receiver)
                        .map(|e| e.name.clone())
                        .collect();
                    let receivers: Vec<_> = device.events.iter()
                        .filter(|e| e.is_receiver)
                        .map(|e| e.name.clone())
                        .collect();
                    let methods: Vec<_> = device.methods.iter()
                        .map(|m| m.name.clone())
                        .collect();

                    let result = serde_json::json!({
                        "name": device.name,
                        "triggers": triggers,
                        "receivers": receivers,
                        "methods": methods,
                        "events": device.events.iter().map(|e| serde_json::json!({
                            "name": e.name,
                            "params": e.params.iter().map(|p| format!("{}:{}", p.name, p.type_name)).collect::<Vec<_>>(),
                            "return_type": e.return_type,
                            "is_receiver": e.is_receiver
                        })).collect::<Vec<_>>(),
                        "method_signatures": device.methods.iter().map(|m| serde_json::json!({
                            "name": m.name,
                            "params": m.params.iter().map(|p| format!("{}:{}", p.name, p.type_name)).collect::<Vec<_>>(),
                            "return_type": m.return_type
                        })).collect::<Vec<_>>()
                    });

                    Ok(CallToolResult {
                        content: vec![Annotated::text(serde_json::to_string_pretty(&result).unwrap())],
                        is_error: Some(false),
                    })
                }
                None => {
                    Ok(CallToolResult {
                        content: vec![Annotated::text(serde_json::to_string_pretty(&serde_json::json!({
                            "error": format!("Device not found: {}", device_type),
                            "suggestion": "Try using query_digest to search for available devices"
                        })).unwrap())],
                        is_error: Some(true),
                    })
                }
            }
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

/// Handle query_digest tool call
pub fn handle_query_digest(
    handler: &VerseMcpHandler,
    params: &CallToolRequestParam,
) -> Result<CallToolResult, Error> {
    // Get query from arguments
    let query = params
        .arguments
        .as_ref()
        .and_then(|args| args.get("query"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let search_type = params
        .arguments
        .as_ref()
        .and_then(|args| args.get("search_type"))
        .and_then(|v| v.as_str())
        .unwrap_or("all");

    if query.is_empty() {
        return Ok(CallToolResult {
            content: vec![Annotated::text(
                serde_json::to_string_pretty(&serde_json::json!({
                    "error": "query parameter is required"
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
            let results = match search_type {
                "device" => index.search_devices(query),
                "event" => index.search_events(query),
                "method" => index.search_methods(query),
                _ => index.search_all(query),
            };

            let result = serde_json::json!({
                "query": query,
                "search_type": search_type,
                "total": results.len(),
                "results": results
            });

            Ok(CallToolResult {
                content: vec![Annotated::text(serde_json::to_string_pretty(&result).unwrap())],
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

/// Handle diff_digests tool call
pub fn handle_diff_digests(
    handler: &VerseMcpHandler,
    params: &CallToolRequestParam,
) -> Result<CallToolResult, Error> {
    // Get old_content and new_content from arguments
    let old_content = params
        .arguments
        .as_ref()
        .and_then(|args| args.get("old_content"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let new_content = params
        .arguments
        .as_ref()
        .and_then(|args| args.get("new_content"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if old_content.is_empty() || new_content.is_empty() {
        return Ok(CallToolResult {
            content: vec![Annotated::text(
                serde_json::to_string_pretty(&serde_json::json!({
                    "error": "old_content and new_content parameters are required"
                }))
                .unwrap(),
            )],
            is_error: Some(true),
        });
    }

    // Parse both digest contents
    let old_index = match crate::digest::DigestIndex::parse(old_content) {
        Ok(index) => index,
        Err(e) => {
            return Ok(CallToolResult {
                content: vec![Annotated::text(
                    serde_json::to_string_pretty(&serde_json::json!({
                        "error": format!("Failed to parse old digest: {}", e)
                    }))
                    .unwrap(),
                )],
                is_error: Some(true),
            });
        }
    };

    let new_index = match crate::digest::DigestIndex::parse(new_content) {
        Ok(index) => index,
        Err(e) => {
            return Ok(CallToolResult {
                content: vec![Annotated::text(
                    serde_json::to_string_pretty(&serde_json::json!({
                        "error": format!("Failed to parse new digest: {}", e)
                    }))
                    .unwrap(),
                )],
                is_error: Some(true),
            });
        }
    };

    // Perform diff
    let diff = old_index.diff(&new_index);

    let result_json = serde_json::json!({
        "breaking_changes": diff.breaking_changes,
        "additions": diff.additions,
        "stats": diff.stats,
        "summary": format!(
            "{} breaking changes, {} additions",
            diff.breaking_changes.len(),
            diff.additions.len()
        )
    });

    Ok(CallToolResult {
        content: vec![Annotated::text(
            serde_json::to_string_pretty(&result_json).unwrap(),
        )],
        is_error: Some(false),
    })
}

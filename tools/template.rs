//! Template tool handlers - list_templates, load_template, save_template, delete_template

use rmcp::{Error, model::{CallToolRequestParam, CallToolResult, Annotated}};

use crate::handler::VerseMcpHandler;

/// Handle list_templates tool call
pub fn handle_list_templates(
    handler: &VerseMcpHandler,
    _params: &CallToolRequestParam,
) -> Result<CallToolResult, Error> {
    let manager = crate::template_manager::TemplateManager::new(handler.templates_dir.clone());
    match manager.list() {
        Ok(templates) => {
            let result_json = serde_json::json!({
                "templates": templates,
                "count": templates.len(),
                "templates_dir": handler.templates_dir.display().to_string(),
            });
            Ok(CallToolResult {
                content: vec![Annotated::text(
                    serde_json::to_string_pretty(&result_json).unwrap(),
                )],
                is_error: Some(false),
            })
        }
        Err(e) => Ok(CallToolResult {
            content: vec![Annotated::text(
                serde_json::to_string_pretty(&serde_json::json!({
                    "error": e.to_string()
                }))
                .unwrap(),
            )],
            is_error: Some(true),
        }),
    }
}

/// Handle load_template tool call
pub fn handle_load_template(
    handler: &VerseMcpHandler,
    params: &CallToolRequestParam,
) -> Result<CallToolResult, Error> {
    let template_name = params
        .arguments
        .as_ref()
        .and_then(|args| args.get("name"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if template_name.is_empty() {
        return Ok(CallToolResult {
            content: vec![Annotated::text(
                serde_json::to_string_pretty(&serde_json::json!({
                    "error": "name parameter is required"
                }))
                .unwrap(),
            )],
            is_error: Some(true),
        });
    }

    let manager = crate::template_manager::TemplateManager::new(handler.templates_dir.clone());
    match manager.load(template_name) {
        Ok(template) => {
            let result_json = serde_json::to_value(&template)
                .unwrap_or_else(|_| serde_json::json!({"error": "serialization failed"}));
            Ok(CallToolResult {
                content: vec![Annotated::text(
                    serde_json::to_string_pretty(&result_json).unwrap(),
                )],
                is_error: Some(false),
            })
        }
        Err(e) => Ok(CallToolResult {
            content: vec![Annotated::text(
                serde_json::to_string_pretty(&serde_json::json!({
                    "error": e.to_string()
                }))
                .unwrap(),
            )],
            is_error: Some(true),
        }),
    }
}

/// Handle save_template tool call
pub fn handle_save_template(
    handler: &VerseMcpHandler,
    params: &CallToolRequestParam,
) -> Result<CallToolResult, Error> {
    let template_name = params
        .arguments
        .as_ref()
        .and_then(|args| args.get("name"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if template_name.is_empty() {
        return Ok(CallToolResult {
            content: vec![Annotated::text(
                serde_json::to_string_pretty(&serde_json::json!({
                    "error": "name parameter is required"
                }))
                .unwrap(),
            )],
            is_error: Some(true),
        });
    }

    let from_scan = params
        .arguments
        .as_ref()
        .and_then(|args| args.get("from_scan"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let description = params
        .arguments
        .as_ref()
        .and_then(|args| args.get("description"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let manager = crate::template_manager::TemplateManager::new(handler.templates_dir.clone());

    let template = if from_scan {
        // Create template from current scan output
        let cache_guard = handler.cache.lock().unwrap();
        match &*cache_guard {
            Some(cached) => crate::template_manager::TemplateManager::from_scan_output(
                template_name.to_string(),
                description,
                &cached.output,
            ),
            None => {
                return Ok(CallToolResult {
                    content: vec![Annotated::text(
                        serde_json::to_string_pretty(&serde_json::json!({
                            "error": "No scan data available. Run scan_map_devices first."
                        }))
                        .unwrap(),
                    )],
                    is_error: Some(true),
                });
            }
        }
    } else {
        // Parse from template_json
        let template_json = params
            .arguments
            .as_ref()
            .and_then(|args| args.get("template_json"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if template_json.is_empty() {
            return Ok(CallToolResult {
                content: vec![Annotated::text(
                    serde_json::to_string_pretty(&serde_json::json!({
                        "error": "Either from_scan=true or template_json is required"
                    }))
                    .unwrap(),
                )],
                is_error: Some(true),
            });
        }

        match serde_json::from_str::<crate::template_manager::Template>(template_json) {
            Ok(t) => t,
            Err(e) => {
                return Ok(CallToolResult {
                    content: vec![Annotated::text(
                        serde_json::to_string_pretty(&serde_json::json!({
                            "error": format!("Invalid template JSON: {}", e)
                        }))
                        .unwrap(),
                    )],
                    is_error: Some(true),
                });
            }
        }
    };

    match manager.save(&template) {
        Ok(()) => {
            let result_json = serde_json::json!({
                "saved": true,
                "name": template.name,
                "devices": template.devices.len(),
                "wiring": template.wiring.len(),
            });
            Ok(CallToolResult {
                content: vec![Annotated::text(
                    serde_json::to_string_pretty(&result_json).unwrap(),
                )],
                is_error: Some(false),
            })
        }
        Err(e) => Ok(CallToolResult {
            content: vec![Annotated::text(
                serde_json::to_string_pretty(&serde_json::json!({
                    "error": e.to_string()
                }))
                .unwrap(),
            )],
            is_error: Some(true),
        }),
    }
}

/// Handle delete_template tool call
pub fn handle_delete_template(
    handler: &VerseMcpHandler,
    params: &CallToolRequestParam,
) -> Result<CallToolResult, Error> {
    let template_name = params
        .arguments
        .as_ref()
        .and_then(|args| args.get("name"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if template_name.is_empty() {
        return Ok(CallToolResult {
            content: vec![Annotated::text(
                serde_json::to_string_pretty(&serde_json::json!({
                    "error": "name parameter is required"
                }))
                .unwrap(),
            )],
            is_error: Some(true),
        });
    }

    let manager = crate::template_manager::TemplateManager::new(handler.templates_dir.clone());
    match manager.delete(template_name) {
        Ok(()) => {
            let result_json = serde_json::json!({
                "deleted": true,
                "name": template_name,
            });
            Ok(CallToolResult {
                content: vec![Annotated::text(
                    serde_json::to_string_pretty(&result_json).unwrap(),
                )],
                is_error: Some(false),
            })
        }
        Err(e) => Ok(CallToolResult {
            content: vec![Annotated::text(
                serde_json::to_string_pretty(&serde_json::json!({
                    "error": e.to_string()
                }))
                .unwrap(),
            )],
            is_error: Some(true),
        }),
    }
}

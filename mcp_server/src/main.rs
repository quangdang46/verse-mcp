//! Verse MCP Server - MCP server for UEFN/Verse development
//!
//! Provides tools for:
//! - Scanning UEFN projects for devices
//! - Querying Fortnite.digest.verse
//! - Listing @editable fields
//! - UI scaffolding

use anyhow::Result;
use clap::Parser;
use rmcp::model::{Annotated, CallToolRequestMethod};
use rmcp::{ServerHandler, ServiceExt};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::time::SystemTime;
use tokio_util::sync::CancellationToken;
use tracing_subscriber::EnvFilter;

mod tools;
mod vm_dir;

/// CLI arguments for the MCP server
#[derive(Parser, Debug)]
#[command(name = "vm")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Verse MCP Server for UEFN/Verse development", long_about = None)]
struct Cli {
    /// Transport type: stdio or http
    #[arg(short, long, default_value = "stdio")]
    transport: String,

    /// Host for HTTP transport (ignored for stdio)
    #[arg(short, long, default_value = "127.0.0.1")]
    host: String,

    /// Port for HTTP transport (ignored for stdio)
    #[arg(short, long, default_value = "2003")]
    port: u16,
}

/// Cache entry for scan results
#[derive(Debug, Clone)]
struct ScanCache {
    /// Cached scan output
    output: uasset_scan::ScanOutput,
    /// Maximum mtime of scanned files when cache was created
    max_mtime: SystemTime,
    /// When the cache was created (for future time-based expiration)
    #[allow(dead_code)]
    cached_at: SystemTime,
}

/// Get the maximum modification time of scanned directories
///
/// Scans both __ExternalActors__ and __ExternalObjects__ for cache invalidation.
fn get_max_mtime(project_path: &std::path::Path) -> SystemTime {
    let content_root = project_path.join("Content");
    let scan_dirs = vec![
        content_root.join("__ExternalActors__"),
        content_root.join("__ExternalObjects__"),
    ];
    let mut max_mtime = SystemTime::UNIX_EPOCH;

    for scan_dir in scan_dirs {
        if let Ok(entries) = std::fs::read_dir(&scan_dir) {
            for entry in entries.flatten() {
                if entry.path().extension().is_some_and(|ext| ext == "uasset") {
                    if let Ok(metadata) = entry.metadata() {
                        if let Ok(mtime) = metadata.modified() {
                            if mtime > max_mtime {
                                max_mtime = mtime;
                            }
                        }
                    }
                }
            }
        }
    }

    max_mtime
}

/// Load digest index from file
fn load_digest() -> Option<uasset_scan::DigestIndex> {
    match vm_dir::load_digest_content() {
        Ok(content) => match uasset_scan::DigestIndex::parse(&content) {
            Ok(index) => {
                tracing::info!(
                    "Digest loaded from ~/.vm: {} devices, {} symbols",
                    index.devices.len(),
                    index.symbols.len()
                );
                Some(index)
            }
            Err(e) => {
                tracing::warn!("Failed to parse digest: {}", e);
                None
            }
        },
        Err(e) => {
            tracing::warn!("Failed to load digest from ~/.vm: {}", e);
            None
        }
    }
}

/// MCP server entry point
#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments first - this handles --help before any logging
    let cli = Cli::parse();

    // Initialize logging to stderr (NEVER stdout - MCP uses stdout for protocol)
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .with_writer(std::io::stderr)
        .init();

    // Ensure ~/.vm exists and bundled digest is extracted if missing
    if let Err(e) = vm_dir::ensure_vm_dir() {
        tracing::warn!("Could not init .vm dir: {}", e);
    }

    tracing::info!(
        "Starting Verse MCP Server with transport: {}",
        cli.transport
    );

    // Use current directory for templates (templates_dir only)
    let project_path = std::env::current_dir().unwrap_or_default();

    tracing::info!("Templates directory: {}", project_path.display());

    // Load digest index from project path (or None if not found)
    let digest_index = load_digest();

    // Create server handler (project_path is default for templates only)
    let templates_dir = project_path.join("templates");
    let handler = VerseMcpHandler {
        project_path,
        cache: Mutex::new(None),
        digest: RwLock::new(digest_index),
        templates_dir,
    };

    // Select transport mode
    match cli.transport.as_str() {
        "http" => {
            let addr = format!("{}:{}", cli.host, cli.port);
            tracing::info!("Starting HTTP server on {}", addr);

            let bind_addr: std::net::SocketAddr = addr.parse()?;
            let sse_server: rmcp::transport::SseServer =
                rmcp::transport::SseServer::serve(bind_addr).await?;
            let token: CancellationToken = sse_server.with_service(move || handler.clone());

            tracing::info!("HTTP server listening on {}", addr);
            tokio::signal::ctrl_c().await?;
            tracing::info!("Shutting down HTTP server...");
            token.cancel();
            token.cancelled().await;
        }
        _ => {
            tracing::info!("Using stdio transport");
            let transport = rmcp::transport::stdio();
            let server = handler.serve(transport).await?;
            tracing::info!("Server initialized, waiting for requests...");
            let quit_reason = server.waiting().await?;
            tracing::info!("Server shutdown: {:?}", quit_reason);
        }
    }

    Ok(())
}

/// Verse MCP Handler implementation
#[derive(Debug)]
struct VerseMcpHandler {
    project_path: PathBuf,
    /// Cache for scan results (uses Mutex for interior mutability)
    cache: Mutex<Option<ScanCache>>,
    /// Digest index for API validation
    digest: RwLock<Option<uasset_scan::DigestIndex>>,
    /// Directory for composition templates
    templates_dir: PathBuf,
}

impl Clone for VerseMcpHandler {
    fn clone(&self) -> Self {
        Self {
            project_path: self.project_path.clone(),
            cache: Mutex::new(self.cache.lock().unwrap().clone()),
            digest: RwLock::new(self.digest.read().unwrap().clone()),
            templates_dir: self.templates_dir.clone(),
        }
    }
}

impl ServerHandler for VerseMcpHandler {
    fn get_info(&self) -> rmcp::model::ServerInfo {
        rmcp::model::ServerInfo {
            protocol_version: rmcp::model::ProtocolVersion::default(),
            capabilities: rmcp::model::ServerCapabilities {
                tools: Some(rmcp::model::ToolsCapability { list_changed: None }),
                ..Default::default()
            },
            server_info: rmcp::model::Implementation {
                name: "verse-mcp".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            instructions: Some("Verse MCP Server for UEFN/Verse development. Use scan_map_devices to scan your project for devices, query_digest to search the Verse API, and get_device_props to get device properties.".to_string()),
        }
    }

    async fn list_tools(
        &self,
        _pagination: Option<rmcp::model::PaginatedRequestParamInner>,
        _context: rmcp::service::RequestContext<rmcp::service::RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, rmcp::Error> {
        // Build input schema for scan_map_devices with project_path and force_refresh parameters
        let mut scan_schema = rmcp::model::JsonObject::new();
        scan_schema.insert("type".to_string(), serde_json::json!("object"));
        scan_schema.insert(
            "properties".to_string(),
            serde_json::json!({
                "project_path": {
                    "type": "string",
                    "description": "Path to UEFN project (e.g., E:\\\\Projects\\\\Testproject or /mnt/e/Projects/Testproject)"
                },
                "force_refresh": {
                    "type": "boolean",
                    "description": "Force re-scan even if results are cached"
                }
            }),
        );
        scan_schema.insert("required".to_string(), serde_json::json!(["project_path"]));

        // Build input schema for get_device_props
        let mut device_props_schema = rmcp::model::JsonObject::new();
        device_props_schema.insert("type".to_string(), serde_json::json!("object"));
        device_props_schema.insert(
            "properties".to_string(),
            serde_json::json!({
                "device_type": {
                    "type": "string",
                    "description": "Device type name (e.g., device_campfire_device, Device_Campfire_C)"
                }
            }),
        );
        device_props_schema.insert("required".to_string(), serde_json::json!(["device_type"]));

        // Build input schema for query_digest
        let mut query_digest_schema = rmcp::model::JsonObject::new();
        query_digest_schema.insert("type".to_string(), serde_json::json!("object"));
        query_digest_schema.insert(
            "properties".to_string(),
            serde_json::json!({
                "query": {
                    "type": "string",
                    "description": "Search term (device name, event, method, etc.)"
                },
                "search_type": {
                    "type": "string",
                    "enum": ["device", "event", "method", "all"],
                    "description": "Type of symbol to search for (default: all)"
                }
            }),
        );
        query_digest_schema.insert("required".to_string(), serde_json::json!(["query"]));

        // Build input schema for validate_verse
        let mut validate_verse_schema = rmcp::model::JsonObject::new();
        validate_verse_schema.insert("type".to_string(), serde_json::json!("object"));
        validate_verse_schema.insert(
            "properties".to_string(),
            serde_json::json!({
                "code": {
                    "type": "string",
                    "description": "Verse source code to validate"
                }
            }),
        );
        validate_verse_schema.insert("required".to_string(), serde_json::json!(["code"]));

        // Build input schema for generate_device_graph
        let mut graph_schema = rmcp::model::JsonObject::new();
        graph_schema.insert("type".to_string(), serde_json::json!("object"));
        graph_schema.insert(
            "properties".to_string(),
            serde_json::json!({
                "project_path": {
                    "type": "string",
                    "description": "Path to UEFN project"
                },
                "format": {
                    "type": "string",
                    "enum": ["mermaid", "dot"],
                    "description": "Output format (default: mermaid)"
                }
            }),
        );
        graph_schema.insert("required".to_string(), serde_json::json!(["project_path"]));

        // Build input schema for diff_digests
        let mut diff_schema = rmcp::model::JsonObject::new();
        diff_schema.insert("type".to_string(), serde_json::json!("object"));
        diff_schema.insert(
            "properties".to_string(),
            serde_json::json!({
                "old_content": {
                    "type": "string",
                    "description": "Content of the old Fortnite.digest.verse file"
                },
                "new_content": {
                    "type": "string",
                    "description": "Content of the new Fortnite.digest.verse file"
                }
            }),
        );
        diff_schema.insert(
            "required".to_string(),
            serde_json::json!(["old_content", "new_content"]),
        );

        // Build input schema for list_templates (no parameters required)
        let mut list_templates_schema = rmcp::model::JsonObject::new();
        list_templates_schema.insert("type".to_string(), serde_json::json!("object"));

        // Build input schema for load_template
        let mut load_template_schema = rmcp::model::JsonObject::new();
        load_template_schema.insert("type".to_string(), serde_json::json!("object"));
        load_template_schema.insert(
            "properties".to_string(),
            serde_json::json!({
                "name": {
                    "type": "string",
                    "description": "Template name to load"
                }
            }),
        );
        load_template_schema.insert("required".to_string(), serde_json::json!(["name"]));

        // Build input schema for save_template
        let mut save_template_schema = rmcp::model::JsonObject::new();
        save_template_schema.insert("type".to_string(), serde_json::json!("object"));
        save_template_schema.insert(
            "properties".to_string(),
            serde_json::json!({
                "name": {
                    "type": "string",
                    "description": "Template name"
                },
                "description": {
                    "type": "string",
                    "description": "Template description"
                },
                "from_scan": {
                    "type": "boolean",
                    "description": "Create template from current scan output (default: false)"
                },
                "template_json": {
                    "type": "string",
                    "description": "Full template JSON (if not using from_scan)"
                }
            }),
        );
        save_template_schema.insert("required".to_string(), serde_json::json!(["name"]));

        // Build input schema for delete_template
        let mut delete_template_schema = rmcp::model::JsonObject::new();
        delete_template_schema.insert("type".to_string(), serde_json::json!("object"));
        delete_template_schema.insert(
            "properties".to_string(),
            serde_json::json!({
                "name": {
                    "type": "string",
                    "description": "Template name to delete"
                }
            }),
        );
        delete_template_schema.insert("required".to_string(), serde_json::json!(["name"]));

        Ok(rmcp::model::ListToolsResult {
            tools: vec![
                rmcp::model::Tool {
                    name: "scan_map_devices".into(),
                    description: "Scan UEFN project for all placed devices. Returns device types, triggers, receivers, and settings. Results are cached and invalidated when files change.".into(),
                    input_schema: Arc::new(scan_schema),
                },
                rmcp::model::Tool {
                    name: "get_device_props".into(),
                    description: "Get all events, methods, and properties for a device type from Fortnite.digest.verse. Handles both Verse naming (device_campfire_device) and UE naming (Device_Campfire_C).".into(),
                    input_schema: Arc::new(device_props_schema),
                },
                rmcp::model::Tool {
                    name: "query_digest".into(),
                    description: "Search Fortnite.digest.verse for device types, events, methods, or symbols. Returns matching entries with formatted signatures.".into(),
                    input_schema: Arc::new(query_digest_schema),
                },
                rmcp::model::Tool {
                    name: "validate_wiring".into(),
                    description: "Validate device wiring for issues like orphaned channels, conflicts, and missing connections.".into(),
                    input_schema: Arc::new({
                        let mut schema = rmcp::model::JsonObject::new();
                        schema.insert("type".to_string(), serde_json::json!("object"));
                        schema.insert(
                            "properties".to_string(),
                            serde_json::json!({
                                "project_path": {
                                    "type": "string",
                                    "description": "Path to UEFN project"
                                }
                            }),
                        );
                        schema.insert("required".to_string(), serde_json::json!(["project_path"]));
                        schema
                    }),
                },
                rmcp::model::Tool {
                    name: "validate_verse".into(),
                    description: "Validate Verse code against Fortnite.digest.verse to detect hallucinated API names (unknown methods, events, or device types). Returns issues with suggestions.".into(),
                    input_schema: Arc::new(validate_verse_schema),
                },
                rmcp::model::Tool {
                    name: "generate_device_graph".into(),
                    description: "Generate a diagram showing device connections in the project. Outputs Mermaid or DOT format for visualization.".into(),
                    input_schema: Arc::new(graph_schema),
                },
                rmcp::model::Tool {
                    name: "diff_digests".into(),
                    description: "Compare two Fortnite.digest.verse versions to detect breaking changes and additions. Useful for tracking API changes across Fortnite updates.".into(),
                    input_schema: Arc::new(diff_schema),
                },
                rmcp::model::Tool {
                    name: "list_templates".into(),
                    description: "List all available composition templates (saved device wiring patterns).".into(),
                    input_schema: Arc::new(list_templates_schema),
                },
                rmcp::model::Tool {
                    name: "load_template".into(),
                    description: "Load a composition template by name. Returns the template with devices, wiring, and settings.".into(),
                    input_schema: Arc::new(load_template_schema),
                },
                rmcp::model::Tool {
                    name: "save_template".into(),
                    description: "Save a composition template. Can create from current scan output or from provided JSON.".into(),
                    input_schema: Arc::new(save_template_schema),
                },
                rmcp::model::Tool {
                    name: "delete_template".into(),
                    description: "Delete a composition template by name.".into(),
                    input_schema: Arc::new(delete_template_schema),
                },
            ],
            next_cursor: None,
        })
    }

    async fn call_tool(
        &self,
        params: rmcp::model::CallToolRequestParam,
        _context: rmcp::service::RequestContext<rmcp::service::RoleServer>,
    ) -> Result<rmcp::model::CallToolResult, rmcp::Error> {
        let name = params.name.as_ref();
        match name {
            "scan_map_devices" => {
                // Parse project_path from arguments (required)
                let scan_path = params
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("project_path"))
                    .and_then(|v| v.as_str())
                    .map(PathBuf::from)
                    .ok_or_else(|| rmcp::Error::invalid_params("project_path is required", None))?;

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
                    let cache_guard = self.cache.lock().unwrap();
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
                    let cache_guard = self.cache.lock().unwrap();
                    if let Some(ref cached) = *cache_guard {
                        let json = serde_json::to_string_pretty(&cached.output)
                            .map_err(|e| rmcp::Error::internal_error(e.to_string(), None))?;
                        return Ok(rmcp::model::CallToolResult {
                            content: vec![Annotated::text(json)],
                            is_error: Some(false),
                        });
                    }
                }

                // Perform fresh scan
                match uasset_scan::scan_project(&scan_path) {
                    Ok(output) => {
                        // Update cache
                        {
                            let mut cache_guard = self.cache.lock().unwrap();
                            *cache_guard = Some(ScanCache {
                                max_mtime: get_max_mtime(&scan_path),
                                cached_at: SystemTime::now(),
                                output: output.clone(),
                            });
                        }

                        let json = serde_json::to_string_pretty(&output)
                            .map_err(|e| rmcp::Error::internal_error(e.to_string(), None))?;
                        Ok(rmcp::model::CallToolResult {
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
                        Ok(rmcp::model::CallToolResult {
                            content: vec![Annotated::text(
                                serde_json::to_string_pretty(&error_json).unwrap(),
                            )],
                            is_error: Some(true),
                        })
                    }
                }
            }
            "get_device_props" => {
                // Get device_type from arguments
                let device_type = params
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("device_type"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                if device_type.is_empty() {
                    return Ok(rmcp::model::CallToolResult {
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
                let digest_guard = self.digest.read().unwrap();
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

                                Ok(rmcp::model::CallToolResult {
                                    content: vec![Annotated::text(serde_json::to_string_pretty(&result).unwrap())],
                                    is_error: Some(false),
                                })
                            }
                            None => {
                                Ok(rmcp::model::CallToolResult {
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
                        Ok(rmcp::model::CallToolResult {
                            content: vec![Annotated::text(serde_json::to_string_pretty(&serde_json::json!({
                                "error": "Digest not loaded. Ensure Fortnite.digest.verse is in the project directory."
                            })).unwrap())],
                            is_error: Some(true),
                        })
                    }
                }
            }
            "query_digest" => {
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
                    return Ok(rmcp::model::CallToolResult {
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
                let digest_guard = self.digest.read().unwrap();
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

                        Ok(rmcp::model::CallToolResult {
                            content: vec![Annotated::text(serde_json::to_string_pretty(&result).unwrap())],
                            is_error: Some(false),
                        })
                    }
                    None => {
                        Ok(rmcp::model::CallToolResult {
                            content: vec![Annotated::text(serde_json::to_string_pretty(&serde_json::json!({
                                "error": "Digest not loaded. Ensure Fortnite.digest.verse is in the project directory."
                            })).unwrap())],
                            is_error: Some(true),
                        })
                    }
                }
            }
            "validate_wiring" => {
                // Parse project_path from arguments (required)
                let scan_path = params
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("project_path"))
                    .and_then(|v| v.as_str())
                    .map(PathBuf::from)
                    .ok_or_else(|| rmcp::Error::invalid_params("project_path is required", None))?;

                // Get all devices from cache or scan
                let devices: Vec<uasset_scan::DeviceInfo> = {
                    let cache_guard = self.cache.lock().unwrap();
                    if let Some(ref cached) = *cache_guard {
                        let all_devices: Vec<_> =
                            cached.output.by_type.values().flatten().cloned().collect();
                        all_devices
                    } else {
                        // Need to scan first - drop the lock before scanning
                        std::mem::drop(cache_guard);

                        match uasset_scan::scan_project(&scan_path) {
                            Ok(output) => {
                                let devices: Vec<_> =
                                    output.by_type.values().flatten().cloned().collect();
                                devices
                            }
                            Err(e) => {
                                let error_json = serde_json::json!({
                                    "error": e.to_string(),
                                    "error_type": std::any::type_name_of_val(&e)
                                });
                                return Ok(rmcp::model::CallToolResult {
                                    content: vec![Annotated::text(
                                        serde_json::to_string_pretty(&error_json).unwrap(),
                                    )],
                                    is_error: Some(true),
                                });
                            }
                        }
                    }
                };

                // Validate wiring
                let issues = uasset_scan::WiringValidator::validate(&devices);

                let result_json = serde_json::json!({
                    "total_issues": issues.len(),
                    "issues": issues,
                });

                Ok(rmcp::model::CallToolResult {
                    content: vec![Annotated::text(
                        serde_json::to_string_pretty(&result_json).unwrap(),
                    )],
                    is_error: Some(false),
                })
            }
            "validate_verse" => {
                // Get code from arguments
                let code = params
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("code"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                if code.is_empty() {
                    return Ok(rmcp::model::CallToolResult {
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
                let digest_guard = self.digest.read().unwrap();
                match &*digest_guard {
                    Some(index) => {
                        let validator = uasset_scan::VerseValidator::new(index.clone());
                        let issues = validator.validate(code);

                        let result_json = serde_json::json!({
                            "total_issues": issues.len(),
                            "issues": issues,
                        });

                        Ok(rmcp::model::CallToolResult {
                            content: vec![Annotated::text(
                                serde_json::to_string_pretty(&result_json).unwrap(),
                            )],
                            is_error: Some(false),
                        })
                    }
                    None => {
                        Ok(rmcp::model::CallToolResult {
                            content: vec![Annotated::text(serde_json::to_string_pretty(&serde_json::json!({
                                "error": "Digest not loaded. Ensure Fortnite.digest.verse is in the project directory."
                            })).unwrap())],
                            is_error: Some(true),
                        })
                    }
                }
            }
            "generate_device_graph" => {
                // Parse project_path from arguments (required)
                let scan_path = params
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("project_path"))
                    .and_then(|v| v.as_str())
                    .map(PathBuf::from)
                    .ok_or_else(|| rmcp::Error::invalid_params("project_path is required", None))?;

                // Get format from arguments
                let format_str = params
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("format"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("mermaid");

                let format = match format_str {
                    "dot" => uasset_scan::GraphFormat::Dot,
                    _ => uasset_scan::GraphFormat::Mermaid,
                };

                // Get all devices from cache or scan
                let output = {
                    let cache_guard = self.cache.lock().unwrap();
                    if let Some(ref cached) = *cache_guard {
                        cached.output.clone()
                    } else {
                        // Need to scan first - drop the lock before scanning
                        std::mem::drop(cache_guard);

                        match uasset_scan::scan_project(&scan_path) {
                            Ok(output) => {
                                // Update cache
                                {
                                    let mut cache_guard = self.cache.lock().unwrap();
                                    *cache_guard = Some(ScanCache {
                                        max_mtime: get_max_mtime(&scan_path),
                                        cached_at: SystemTime::now(),
                                        output: output.clone(),
                                    });
                                }
                                output
                            }
                            Err(e) => {
                                let error_json = serde_json::json!({
                                    "error": e.to_string(),
                                    "error_type": std::any::type_name_of_val(&e)
                                });
                                return Ok(rmcp::model::CallToolResult {
                                    content: vec![Annotated::text(
                                        serde_json::to_string_pretty(&error_json).unwrap(),
                                    )],
                                    is_error: Some(true),
                                });
                            }
                        }
                    }
                };

                // Generate graph
                let graph = uasset_scan::DeviceGrapher::generate(&output, format);

                let result_json = serde_json::json!({
                    "format": format_str,
                    "graph": graph,
                    "device_count": output.total_devices,
                });

                Ok(rmcp::model::CallToolResult {
                    content: vec![Annotated::text(
                        serde_json::to_string_pretty(&result_json).unwrap(),
                    )],
                    is_error: Some(false),
                })
            }
            "diff_digests" => {
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
                    return Ok(rmcp::model::CallToolResult {
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
                let old_index = match uasset_scan::DigestIndex::parse(old_content) {
                    Ok(index) => index,
                    Err(e) => {
                        return Ok(rmcp::model::CallToolResult {
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

                let new_index = match uasset_scan::DigestIndex::parse(new_content) {
                    Ok(index) => index,
                    Err(e) => {
                        return Ok(rmcp::model::CallToolResult {
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

                Ok(rmcp::model::CallToolResult {
                    content: vec![Annotated::text(
                        serde_json::to_string_pretty(&result_json).unwrap(),
                    )],
                    is_error: Some(false),
                })
            }
            "list_templates" => {
                let manager = uasset_scan::TemplateManager::new(self.templates_dir.clone());
                match manager.list() {
                    Ok(templates) => {
                        let result_json = serde_json::json!({
                            "templates": templates,
                            "count": templates.len(),
                            "templates_dir": self.templates_dir.display().to_string(),
                        });
                        Ok(rmcp::model::CallToolResult {
                            content: vec![Annotated::text(
                                serde_json::to_string_pretty(&result_json).unwrap(),
                            )],
                            is_error: Some(false),
                        })
                    }
                    Err(e) => Ok(rmcp::model::CallToolResult {
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
            "load_template" => {
                let template_name = params
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("name"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                if template_name.is_empty() {
                    return Ok(rmcp::model::CallToolResult {
                        content: vec![Annotated::text(
                            serde_json::to_string_pretty(&serde_json::json!({
                                "error": "name parameter is required"
                            }))
                            .unwrap(),
                        )],
                        is_error: Some(true),
                    });
                }

                let manager = uasset_scan::TemplateManager::new(self.templates_dir.clone());
                match manager.load(template_name) {
                    Ok(template) => {
                        let result_json = serde_json::to_value(&template).unwrap_or_else(
                            |_| serde_json::json!({"error": "serialization failed"}),
                        );
                        Ok(rmcp::model::CallToolResult {
                            content: vec![Annotated::text(
                                serde_json::to_string_pretty(&result_json).unwrap(),
                            )],
                            is_error: Some(false),
                        })
                    }
                    Err(e) => Ok(rmcp::model::CallToolResult {
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
            "save_template" => {
                let template_name = params
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("name"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                if template_name.is_empty() {
                    return Ok(rmcp::model::CallToolResult {
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

                let manager = uasset_scan::TemplateManager::new(self.templates_dir.clone());

                let template = if from_scan {
                    // Create template from current scan output
                    let cache_guard = self.cache.lock().unwrap();
                    match &*cache_guard {
                        Some(cached) => uasset_scan::TemplateManager::from_scan_output(
                            template_name.to_string(),
                            description,
                            &cached.output,
                        ),
                        None => {
                            return Ok(rmcp::model::CallToolResult {
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
                        return Ok(rmcp::model::CallToolResult {
                            content: vec![Annotated::text(
                                serde_json::to_string_pretty(&serde_json::json!({
                                    "error": "Either from_scan=true or template_json is required"
                                }))
                                .unwrap(),
                            )],
                            is_error: Some(true),
                        });
                    }

                    match serde_json::from_str::<uasset_scan::Template>(template_json) {
                        Ok(t) => t,
                        Err(e) => {
                            return Ok(rmcp::model::CallToolResult {
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
                        Ok(rmcp::model::CallToolResult {
                            content: vec![Annotated::text(
                                serde_json::to_string_pretty(&result_json).unwrap(),
                            )],
                            is_error: Some(false),
                        })
                    }
                    Err(e) => Ok(rmcp::model::CallToolResult {
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
            "delete_template" => {
                let template_name = params
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("name"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                if template_name.is_empty() {
                    return Ok(rmcp::model::CallToolResult {
                        content: vec![Annotated::text(
                            serde_json::to_string_pretty(&serde_json::json!({
                                "error": "name parameter is required"
                            }))
                            .unwrap(),
                        )],
                        is_error: Some(true),
                    });
                }

                let manager = uasset_scan::TemplateManager::new(self.templates_dir.clone());
                match manager.delete(template_name) {
                    Ok(()) => {
                        let result_json = serde_json::json!({
                            "deleted": true,
                            "name": template_name,
                        });
                        Ok(rmcp::model::CallToolResult {
                            content: vec![Annotated::text(
                                serde_json::to_string_pretty(&result_json).unwrap(),
                            )],
                            is_error: Some(false),
                        })
                    }
                    Err(e) => Ok(rmcp::model::CallToolResult {
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
            _ => Err(rmcp::Error::method_not_found::<CallToolRequestMethod>()),
        }
    }
}

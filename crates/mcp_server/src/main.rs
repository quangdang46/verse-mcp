//! Verse MCP Server - MCP server for UEFN/Verse development
//!
//! Provides tools for:
//! - Scanning UEFN projects for devices
//! - Querying Fortnite.digest.verse
//! - Listing @editable fields
//! - UI scaffolding

use anyhow::Result;
use rmcp::{ServerHandler, ServiceExt};
use rmcp::model::{Annotated, CallToolRequestMethod};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::SystemTime;
use tracing_subscriber::EnvFilter;

mod tools;

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

/// Get the maximum modification time of the ExternalActors directory
fn get_max_mtime(project_path: &PathBuf) -> SystemTime {
    let external_actors = project_path.join("Content").join("__ExternalActors__");
    let mut max_mtime = SystemTime::UNIX_EPOCH;

    if let Ok(entries) = std::fs::read_dir(&external_actors) {
        for entry in entries.flatten() {
            if entry.path().extension().map_or(false, |ext| ext == "uasset") {
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

    max_mtime
}

/// MCP server entry point
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging to stderr (NEVER stdout - MCP uses stdout for protocol)
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .with_writer(std::io::stderr)
        .init();

    tracing::info!("Starting Verse MCP Server...");

    // Get project path from environment or use current directory
    let project_path = std::env::var("VERSE_PROJECT_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());

    tracing::info!("Project path: {}", project_path.display());

    // Create server handler
    let handler = VerseMcpHandler {
        project_path,
        cache: Mutex::new(None),
    };

    // Use stdio transport (rmcp expects (stdin, stdout) tuple)
    let transport = rmcp::transport::stdio();

    // Serve over stdio and wait for server shutdown
    // The serve() method completes initialization, waiting() keeps server running
    let server = handler.serve(transport).await?;
    tracing::info!("Server initialized, waiting for requests...");

    // Keep the server running until shutdown
    let quit_reason = server.waiting().await?;
    tracing::info!("Server shutdown: {:?}", quit_reason);

    Ok(())
}

/// Verse MCP Handler implementation
#[derive(Debug)]
struct VerseMcpHandler {
    project_path: PathBuf,
    /// Cache for scan results (uses Mutex for interior mutability)
    cache: Mutex<Option<ScanCache>>,
}

impl Clone for VerseMcpHandler {
    fn clone(&self) -> Self {
        Self {
            project_path: self.project_path.clone(),
            cache: Mutex::new(self.cache.lock().unwrap().clone()),
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
            instructions: Some("Verse MCP Server for UEFN/Verse development. Use scan_map_devices to scan your project for devices.".to_string()),
        }
    }

    async fn list_tools(
        &self,
        _pagination: Option<rmcp::model::PaginatedRequestParamInner>,
        _context: rmcp::service::RequestContext<rmcp::service::RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, rmcp::Error> {
        // Build input schema for scan_map_devices with force_refresh parameter
        let mut scan_schema = rmcp::model::JsonObject::new();
        scan_schema.insert("type".to_string(), serde_json::json!("object"));
        scan_schema.insert(
            "properties".to_string(),
            serde_json::json!({
                "force_refresh": {
                    "type": "boolean",
                    "description": "Force re-scan even if results are cached"
                }
            }),
        );

        Ok(rmcp::model::ListToolsResult {
            tools: vec![
                rmcp::model::Tool {
                    name: "scan_map_devices".into(),
                    description: "Scan UEFN project for all placed devices. Returns device types, triggers, receivers, and settings. Results are cached and invalidated when files change.".into(),
                    input_schema: Arc::new(scan_schema),
                },
                rmcp::model::Tool {
                    name: "get_device_props".into(),
                    description: "Get device properties from Fortnite.digest.verse (not yet implemented)".into(),
                    input_schema: Arc::new(rmcp::model::JsonObject::new()),
                },
                rmcp::model::Tool {
                    name: "query_digest".into(),
                    description: "Search Fortnite.digest.verse for symbols (not yet implemented)".into(),
                    input_schema: Arc::new(rmcp::model::JsonObject::new()),
                },
                rmcp::model::Tool {
                    name: "validate_wiring".into(),
                    description: "Validate device wiring for issues like orphaned channels, conflicts, and missing connections.".into(),
                    input_schema: Arc::new(rmcp::model::JsonObject::new()),
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
                // Parse force_refresh from arguments
                let force_refresh = params.arguments.as_ref()
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
                        let current_mtime = get_max_mtime(&self.project_path);
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
                match uasset_scan::scan_project(&self.project_path) {
                    Ok(output) => {
                        // Update cache
                        {
                            let mut cache_guard = self.cache.lock().unwrap();
                            *cache_guard = Some(ScanCache {
                                max_mtime: get_max_mtime(&self.project_path),
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
                            content: vec![Annotated::text(serde_json::to_string_pretty(&error_json).unwrap())],
                            is_error: Some(true),
                        })
                    }
                }
            }
            "get_device_props" | "query_digest" => {
                Ok(rmcp::model::CallToolResult {
                    content: vec![Annotated::text("This tool is not yet implemented. Check back in Phase 3.".to_string())],
                    is_error: Some(false),
                })
            }
            "validate_wiring" => {
                // Get all devices from cache or scan
                let devices: Vec<uasset_scan::DeviceInfo> = {
                    let cache_guard = self.cache.lock().unwrap();
                    if let Some(ref cached) = *cache_guard {
                        let all_devices: Vec<_> = cached.output.by_type.values()
                            .flatten()
                            .cloned()
                            .collect();
                        all_devices
                    } else {
                        // Need to scan first - drop the lock before scanning
                        std::mem::drop(cache_guard);

                        match uasset_scan::scan_project(&self.project_path) {
                            Ok(output) => {
                                let devices: Vec<_> = output.by_type.values()
                                    .flatten()
                                    .cloned()
                                    .collect();
                                devices
                            }
                            Err(e) => {
                                let error_json = serde_json::json!({
                                    "error": e.to_string(),
                                    "error_type": std::any::type_name_of_val(&e)
                                });
                                return Ok(rmcp::model::CallToolResult {
                                    content: vec![Annotated::text(serde_json::to_string_pretty(&error_json).unwrap())],
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
                    content: vec![Annotated::text(serde_json::to_string_pretty(&result_json).unwrap())],
                    is_error: Some(false),
                })
            }
            _ => Err(rmcp::Error::method_not_found::<CallToolRequestMethod>()),
        }
    }
}

//! Verse MCP Server - MCP server for UEFN/Verse development
//!
//! Provides tools for:
//! - Scanning UEFN projects for devices

use anyhow::Result;
use clap::Parser;
use rmcp::model::{Annotated, CallToolRequestMethod};
use rmcp::{ServerHandler, ServiceExt};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use tokio_util::sync::CancellationToken;
use tracing_subscriber::EnvFilter;

mod tools;

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
    #[arg(long, default_value = "127.0.0.1")]
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

/// MCP server entry point
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .with_writer(std::io::stderr)
        .init();

    tracing::info!(
        "Starting Verse MCP Server with transport: {}",
        cli.transport
    );

    let handler = VerseMcpHandler {
        cache: Mutex::new(None),
    };

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
    /// Cache for scan results (uses Mutex for interior mutability)
    cache: Mutex<Option<ScanCache>>,
}

impl Clone for VerseMcpHandler {
    fn clone(&self) -> Self {
        Self {
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
            instructions: Some("Verse MCP Server for UEFN/Verse development. Use scan_map_devices to scan your project for placed devices.".to_string()),
        }
    }

    async fn list_tools(
        &self,
        _pagination: Option<rmcp::model::PaginatedRequestParamInner>,
        _context: rmcp::service::RequestContext<rmcp::service::RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, rmcp::Error> {
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

        Ok(rmcp::model::ListToolsResult {
            tools: vec![rmcp::model::Tool {
                name: "scan_map_devices".into(),
                description: "Scan UEFN project for all placed devices. Returns device types, triggers, receivers, and settings. Results are cached and invalidated when files change.".into(),
                input_schema: Arc::new(scan_schema),
            }],
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
                let scan_path = params
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("project_path"))
                    .and_then(|v| v.as_str())
                    .map(PathBuf::from)
                    .ok_or_else(|| rmcp::Error::invalid_params("project_path is required", None))?;

                tracing::info!("Scanning project at: {}", scan_path.display());

                let force_refresh = params
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("force_refresh"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                let should_scan = {
                    let cache_guard = self.cache.lock().unwrap();
                    if force_refresh {
                        tracing::info!("Force refresh requested, bypassing cache");
                        true
                    } else if let Some(ref cached) = *cache_guard {
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

                match tools::scan_map_devices(&scan_path) {
                    Ok(output) => {
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
            _ => Err(rmcp::Error::method_not_found::<CallToolRequestMethod>()),
        }
    }
}

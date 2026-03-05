//! Verse MCP Server - MCP server for UEFN/Verse development
//!
//! Provides tools for:
//! - Scanning UEFN projects for devices
//! - Querying Fortnite.digest.verse
//! - Listing @editable fields
//! - UI scaffolding

use anyhow::Result;
use rmcp::{ServerHandler, serve_server};
use rmcp::model::{Annotated, CallToolRequestMethod};
use std::path::PathBuf;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

mod tools;

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
    let handler = VerseMcpHandler { project_path };

    // Use stdio transport (rmcp expects (stdin, stdout) tuple)
    let transport = rmcp::transport::stdio();

    // Serve over stdio
    serve_server(handler, transport).await?;

    Ok(())
}

/// Verse MCP Handler implementation
#[derive(Debug, Clone)]
struct VerseMcpHandler {
    project_path: PathBuf,
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
        Ok(rmcp::model::ListToolsResult {
            tools: vec![
                rmcp::model::Tool {
                    name: "scan_map_devices".into(),
                    description: "Scan UEFN project for all placed devices. Returns device types, triggers, receivers, and settings.".into(),
                    input_schema: Arc::new(rmcp::model::JsonObject::new()),
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
                match uasset_scan::scan_project(&self.project_path) {
                    Ok(output) => {
                        let json = serde_json::to_string_pretty(&output)
                            .map_err(|e| rmcp::Error::internal_error(e.to_string(), None))?;
                        Ok(rmcp::model::CallToolResult {
                            content: vec![Annotated::text(json)],
                            is_error: Some(false),
                        })
                    }
                    Err(e) => {
                        Ok(rmcp::model::CallToolResult {
                            content: vec![Annotated::text(format!("Error: {}", e))],
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
            _ => Err(rmcp::Error::method_not_found::<CallToolRequestMethod>()),
        }
    }
}

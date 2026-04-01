//! Verse MCP Server - MCP server for UEFN/Verse development
//!
//! Provides tools for:
//! - Scanning UEFN projects for devices

use anyhow::Result;
use clap::Parser;
use grounding_engine::{
    format_query_response, format_scan_response, DocsQueryOptions, GroundingEngine,
    ScanProjectRequest,
};
use rmcp::model::{Annotated, CallToolRequestMethod, Content};
use rmcp::{ServerHandler, ServiceExt};
use std::path::PathBuf;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use tracing_subscriber::EnvFilter;

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
        grounding: GroundingEngine::default(),
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
#[derive(Debug, Clone)]
struct VerseMcpHandler {
    grounding: GroundingEngine,
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
            instructions: Some("Verse MCP Server for UEFN/Verse development. Use scan_map_devices to scan your project for placed devices. Use query-docs with a required `query` string to search the built-in documentation index.".to_string()),
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

        let mut docs_schema = rmcp::model::JsonObject::new();
        docs_schema.insert("type".to_string(), serde_json::json!("object"));
        docs_schema.insert(
            "properties".to_string(),
            serde_json::json!({
                "query": {
                    "type": "string",
                    "description": "Required. Pass a non-empty query string with the Verse/Fortnite topic to search for, for example `editable properties`, `npc behavior`, or `creative_device AND event`."
                },
                "limit": {
                    "type": "integer",
                    "description": "Maximum number of ranked results to return (clamped to 1-10)."
                },
                "offset": {
                    "type": "integer",
                    "description": "Zero-based result offset for pagination."
                },
                "fetch_source_urls": {
                    "type": "boolean",
                    "description": "When true, fetch the returned source_url pages for the selected results and include their normalized text in the response."
                },
                "max_fetches": {
                    "type": "integer",
                    "description": "Maximum number of source_url pages to fetch when fetch_source_urls=true (clamped to 0-5)."
                }
            }),
        );
        docs_schema.insert("required".to_string(), serde_json::json!(["query"]));

        let mut fetch_doc_schema = rmcp::model::JsonObject::new();
        fetch_doc_schema.insert("type".to_string(), serde_json::json!("object"));
        fetch_doc_schema.insert(
            "properties".to_string(),
            serde_json::json!({
                "url": {
                    "type": "string",
                    "description": "Required. The source URL to fetch and normalize into plain text."
                }
            }),
        );
        fetch_doc_schema.insert("required".to_string(), serde_json::json!(["url"]));

        Ok(rmcp::model::ListToolsResult {
            tools: vec![
                rmcp::model::Tool {
                    name: "scan_map_devices".into(),
                    description: "Scan UEFN project for all placed devices. Returns device types, triggers, receivers, and settings. Results are cached and invalidated when files change.".into(),
                    input_schema: Arc::new(scan_schema),
                },
                rmcp::model::Tool {
                    name: "query-docs".into(),
                    description: "Search the built-in SQLite Verse/Fortnite documentation index and return ranked results with full indexed content by default, plus optional fetched source_url page content.".into(),
                    input_schema: Arc::new(docs_schema),
                },
                rmcp::model::Tool {
                    name: "fetch-doc-source".into(),
                    description: "Fetch and normalize one documentation source URL into agent-friendly text and JSON metadata.".into(),
                    input_schema: Arc::new(fetch_doc_schema),
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
            "query-docs" => {
                let args = params.arguments.as_ref();
                let query = args
                    .and_then(|args| args.get("query"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| rmcp::Error::invalid_params("query is required; pass a non-empty `query` string argument to `query-docs`", None))?;

                let limit = args
                    .and_then(|args| args.get("limit"))
                    .and_then(|v| v.as_u64())
                    .map(|limit| limit as usize);

                let offset = args
                    .and_then(|args| args.get("offset"))
                    .and_then(|v| v.as_u64())
                    .map(|offset| offset as usize);

                let fetch_source_urls = args
                    .and_then(|args| args.get("fetch_source_urls"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                let max_fetches = args
                    .and_then(|args| args.get("max_fetches"))
                    .and_then(|v| v.as_u64())
                    .map(|value| value as usize);

                let options = DocsQueryOptions {
                    limit,
                    offset,
                    fetch_source_urls,
                    max_fetches,
                };

                match self.grounding.query_docs(query, options) {
                    Ok(output) => {
                        let summary = format_query_response(&output);
                        let structured = Content::json(&output)
                            .map_err(|e| rmcp::Error::internal_error(e.to_string(), None))?;
                        Ok(rmcp::model::CallToolResult {
                            content: vec![Annotated::text(summary), structured],
                            is_error: Some(false),
                        })
                    }
                    Err(e) => Ok(rmcp::model::CallToolResult {
                        content: vec![Annotated::text(format!(
                            "query-docs failed: {}. Fix the query input and try again. Example queries: `editable properties`, `npc behavior`, or `creative_device AND event`.",
                            e
                        ))],
                        is_error: Some(true),
                    }),
                }
            }
            "fetch-doc-source" => {
                let url = params
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("url"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        rmcp::Error::invalid_params(
                            "url is required; pass a non-empty `url` string argument to `fetch-doc-source`",
                            None,
                        )
                    })?;

                match self.grounding.fetch_doc_source(url) {
                    Ok(output) => {
                        let summary = format!(
                            "Fetched source {} with status {}.",
                            output.url, output.status
                        );
                        let structured = Content::json(&output)
                            .map_err(|e| rmcp::Error::internal_error(e.to_string(), None))?;
                        Ok(rmcp::model::CallToolResult {
                            content: vec![Annotated::text(summary), structured],
                            is_error: Some(false),
                        })
                    }
                    Err(e) => Ok(rmcp::model::CallToolResult {
                        content: vec![Annotated::text(format!(
                            "fetch-doc-source failed: {}. Pass a valid documentation URL and try again.",
                            e
                        ))],
                        is_error: Some(true),
                    }),
                }
            }
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

                let request = ScanProjectRequest {
                    project_path: scan_path,
                    force_refresh,
                };

                match self.grounding.scan_project(&request) {
                    Ok(output) => {
                        let summary = format_scan_response(&output);
                        let structured = Content::json(&output)
                            .map_err(|e| rmcp::Error::internal_error(e.to_string(), None))?;
                        Ok(rmcp::model::CallToolResult {
                            content: vec![Annotated::text(summary), structured],
                            is_error: Some(false),
                        })
                    }
                    Err(e) => {
                        let error_json = serde_json::json!({
                            "error": e.to_string(),
                            "error_type": std::any::type_name_of_val(&e)
                        });
                        let error_text = serde_json::to_string_pretty(&error_json)
                            .unwrap_or_else(|_| error_json.to_string());
                        Ok(rmcp::model::CallToolResult {
                            content: vec![Annotated::text(error_text)],
                            is_error: Some(true),
                        })
                    }
                }
            }
            _ => Err(rmcp::Error::method_not_found::<CallToolRequestMethod>()),
        }
    }
}

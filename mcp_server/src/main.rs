//! Verse MCP Server - MCP server for UEFN/Verse development
//!
//! Provides tools for:
//! - Scanning UEFN projects for devices

use anyhow::Result;
use axum::Router;
use clap::Parser;
use grounding_engine::{
    format_query_response, format_reload_metadata_response, format_scan_response, DocsQueryOptions,
    GroundingEngine, ReloadMetadataRequest, ScanProjectRequest,
};
use rmcp::model::{CallToolRequestMethod, Content};
use rmcp::transport::streamable_http_server::{
    session::local::LocalSessionManager, StreamableHttpServerConfig, StreamableHttpService,
};
use rmcp::{ServerHandler, ServiceExt};
use serde::Serialize;
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

    let handler = VerseMcpHandler::new();

    match cli.transport.as_str() {
        "http" => {
            let addr = format!("{}:{}", cli.host, cli.port);
            tracing::info!("Starting HTTP server on {}", addr);

            let bind_addr: std::net::SocketAddr = addr.parse()?;
            let shutdown = CancellationToken::new();
            let router = build_http_router(handler.clone(), shutdown.clone());
            let listener = tokio::net::TcpListener::bind(bind_addr).await?;

            tracing::info!("HTTP server listening on http://{}/mcp", addr);
            axum::serve(listener, router)
                .with_graceful_shutdown(async move {
                    let _ = tokio::signal::ctrl_c().await;
                    tracing::info!("Shutting down HTTP server...");
                    shutdown.cancel();
                    shutdown.cancelled().await;
                })
                .await?;
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

impl VerseMcpHandler {
    fn new() -> Self {
        Self {
            grounding: GroundingEngine::default(),
        }
    }
}

fn build_http_router(handler: VerseMcpHandler, shutdown: CancellationToken) -> Router {
    let config = StreamableHttpServerConfig::default()
        .with_stateful_mode(false)
        .with_json_response(true)
        .with_sse_keep_alive(None)
        .with_cancellation_token(shutdown.child_token());
    let service: StreamableHttpService<VerseMcpHandler, LocalSessionManager> =
        StreamableHttpService::new(move || Ok(handler.clone()), Default::default(), config);

    Router::new().nest_service("/mcp", service)
}

fn ok_with_structured<T: Serialize>(
    summary: impl Into<String>,
    value: &T,
) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
    let structured = serde_json::to_value(value)
        .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
    let mut result = rmcp::model::CallToolResult::structured(structured);
    result.content.insert(0, Content::text(summary.into()));
    Ok(result)
}

fn err_with_text(message: impl Into<String>) -> rmcp::model::CallToolResult {
    rmcp::model::CallToolResult::error(vec![Content::text(message.into())])
}

impl ServerHandler for VerseMcpHandler {
    fn get_info(&self) -> rmcp::model::ServerInfo {
        rmcp::model::ServerInfo::new(rmcp::model::ServerCapabilities::builder().enable_tools().build())
            .with_server_info(rmcp::model::Implementation::new(
                "verse-mcp",
                env!("CARGO_PKG_VERSION"),
            ))
            .with_instructions("Verse MCP Server for UEFN/Verse development. Use scan_map_devices to scan your project for placed devices, query-docs to search the built-in documentation index, and reload-project-metadata to drop cached project scan metadata without restarting the server.")
    }

    async fn list_tools(
        &self,
        _pagination: Option<rmcp::model::PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::service::RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, rmcp::ErrorData> {
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

        let mut reload_metadata_schema = rmcp::model::JsonObject::new();
        reload_metadata_schema.insert("type".to_string(), serde_json::json!("object"));
        reload_metadata_schema.insert(
            "properties".to_string(),
            serde_json::json!({
                "project_path": {
                    "type": "string",
                    "description": "Required. Path to the UEFN project whose cached scan metadata should be dropped."
                }
            }),
        );
        reload_metadata_schema.insert("required".to_string(), serde_json::json!(["project_path"]));

        let mut list_workflows_schema = rmcp::model::JsonObject::new();
        list_workflows_schema.insert("type".to_string(), serde_json::json!("object"));
        list_workflows_schema.insert("properties".to_string(), serde_json::json!({}));

        let mut get_workflow_schema = rmcp::model::JsonObject::new();
        get_workflow_schema.insert("type".to_string(), serde_json::json!("object"));
        get_workflow_schema.insert(
            "properties".to_string(),
            serde_json::json!({
                "name": {
                    "type": "string",
                    "description": "Required. Workflow name from list-agent-workflows."
                }
            }),
        );
        get_workflow_schema.insert("required".to_string(), serde_json::json!(["name"]));

        Ok(rmcp::model::ListToolsResult::with_all_items(vec![
            rmcp::model::Tool::new(
                "scan_map_devices",
                "Scan UEFN project for all placed devices. Returns device types, triggers, receivers, and settings. Results are cached and invalidated when files change.",
                Arc::new(scan_schema),
            ),
            rmcp::model::Tool::new(
                "query-docs",
                "Search the built-in SQLite Verse/Fortnite documentation index and return ranked results with full indexed content by default, plus optional fetched source_url page content.",
                Arc::new(docs_schema),
            ),
            rmcp::model::Tool::new(
                "fetch-doc-source",
                "Fetch and normalize one documentation source URL into agent-friendly text and JSON metadata.",
                Arc::new(fetch_doc_schema),
            ),
            rmcp::model::Tool::new(
                "reload-project-metadata",
                "Drop cached project scan metadata for one UEFN project so the next scan rebuilds state without restarting the server.",
                Arc::new(reload_metadata_schema),
            ),
            rmcp::model::Tool::new(
                "list-agent-workflows",
                "List markdown-defined Verse troubleshooting workflows available to an agent client.",
                Arc::new(list_workflows_schema),
            ),
            rmcp::model::Tool::new(
                "get-agent-workflow",
                "Load one markdown-defined Verse troubleshooting workflow by name.",
                Arc::new(get_workflow_schema),
            ),
        ]))
    }

    async fn call_tool(
        &self,
        params: rmcp::model::CallToolRequestParams,
        _context: rmcp::service::RequestContext<rmcp::service::RoleServer>,
    ) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
        let name = params.name.as_ref();
        match name {
            "query-docs" => {
                let args = params.arguments.as_ref();
                let query = args
                    .and_then(|args| args.get("query"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| rmcp::ErrorData::invalid_params("query is required; pass a non-empty `query` string argument to `query-docs`", None))?;

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
                        ok_with_structured(summary, &output)
                    }
                    Err(e) => Ok(err_with_text(format!(
                            "query-docs failed: {}. Fix the query input and try again. Example queries: `editable properties`, `npc behavior`, or `creative_device AND event`.",
                            e
                        ))),
                }
            }
            "fetch-doc-source" => {
                let url = params
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("url"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        rmcp::ErrorData::invalid_params(
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
                        ok_with_structured(summary, &output)
                    }
                    Err(e) => Ok(err_with_text(format!(
                            "fetch-doc-source failed: {}. Pass a valid documentation URL and try again.",
                            e
                        ))),
                }
            }
            "scan_map_devices" => {
                let scan_path = params
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("project_path"))
                    .and_then(|v| v.as_str())
                    .map(PathBuf::from)
                    .ok_or_else(|| rmcp::ErrorData::invalid_params("project_path is required", None))?;

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
                let policy = self.grounding.evaluate_scan_policy(&request);
                if !policy.allowed {
                    let denial = serde_json::json!({
                        "project_path": request.project_path.display().to_string(),
                        "policy": policy,
                    });
                    let denial_text = serde_json::to_string_pretty(&denial)
                        .unwrap_or_else(|_| denial.to_string());
                    return Ok(err_with_text(denial_text));
                }

                match self.grounding.scan_project(&request) {
                    Ok(output) => {
                        let summary = format_scan_response(&output);
                        ok_with_structured(summary, &output)
                    }
                    Err(e) => {
                        let error_json = serde_json::json!({
                            "error": e.to_string(),
                            "error_type": std::any::type_name_of_val(&e)
                        });
                        let error_text = serde_json::to_string_pretty(&error_json)
                            .unwrap_or_else(|_| error_json.to_string());
                        Ok(err_with_text(error_text))
                    }
                }
            }
            "reload-project-metadata" => {
                let project_path = params
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("project_path"))
                    .and_then(|v| v.as_str())
                    .map(PathBuf::from)
                    .ok_or_else(|| rmcp::ErrorData::invalid_params("project_path is required", None))?;

                let response = self
                    .grounding
                    .reload_project_metadata(&ReloadMetadataRequest { project_path });
                let summary = format_reload_metadata_response(&response);
                ok_with_structured(summary, &response)
            }
            "list-agent-workflows" => match self.grounding.list_agent_workflows() {
                Ok(response) => {
                    let summary = format!(
                        "Loaded {} agent workflow(s) from {}.",
                        response.workflows.len(),
                        response.root
                    );
                    ok_with_structured(summary, &response)
                }
                Err(e) => Ok(err_with_text(format!(
                        "list-agent-workflows failed: {}. Ensure the workflows directory is present and readable.",
                        e
                    ))),
            },
            "get-agent-workflow" => {
                let name = params
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("name"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| rmcp::ErrorData::invalid_params("name is required", None))?;

                match self.grounding.get_agent_workflow(name) {
                    Ok(response) => {
                        let summary = format!(
                            "Loaded workflow {} from {}.",
                            response.name, response.source_path
                        );
                        ok_with_structured(summary, &response)
                    }
                    Err(e) => Ok(err_with_text(format!(
                            "get-agent-workflow failed: {}. Call list-agent-workflows first to discover valid names.",
                            e
                        ))),
                }
            }
            _ => Err(rmcp::ErrorData::method_not_found::<CallToolRequestMethod>()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{build_http_router, VerseMcpHandler};
    use anyhow::Result;
    use serde_json::json;
    use tokio_util::sync::CancellationToken;

    #[tokio::test]
    async fn http_transport_exposes_tools_on_mcp_endpoint() -> Result<()> {
        let shutdown = CancellationToken::new();
        let router = build_http_router(VerseMcpHandler::new(), shutdown.clone());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let shutdown_wait = shutdown.clone();
        let server = tokio::spawn(async move {
            axum::serve(listener, router)
                .with_graceful_shutdown(async move {
                    shutdown_wait.cancelled().await;
                })
                .await
        });

        let client = reqwest::Client::new();
        let endpoint = format!("http://{addr}/mcp");

        let initialize = client
            .post(&endpoint)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(
                reqwest::header::ACCEPT,
                "application/json, text/event-stream",
            )
            .json(&json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {
                    "protocolVersion": "2025-03-26",
                    "capabilities": {},
                    "clientInfo": {
                        "name": "verse-mcp-test",
                        "version": "0.1.0"
                    }
                }
            }))
            .send()
            .await?;

        assert_eq!(initialize.status(), reqwest::StatusCode::OK);
        let initialize_json: serde_json::Value = initialize.json().await?;
        assert_eq!(initialize_json["result"]["serverInfo"]["name"], "verse-mcp");

        let list_tools = client
            .post(&endpoint)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(
                reqwest::header::ACCEPT,
                "application/json, text/event-stream",
            )
            .json(&json!({
                "jsonrpc": "2.0",
                "id": 2,
                "method": "tools/list",
                "params": {}
            }))
            .send()
            .await?;

        assert_eq!(list_tools.status(), reqwest::StatusCode::OK);
        let list_tools_json: serde_json::Value = list_tools.json().await?;
        let tools = list_tools_json["result"]["tools"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("tools/list should return a tool array"))?;
        let tool_names = tools
            .iter()
            .filter_map(|tool| tool["name"].as_str())
            .collect::<Vec<_>>();

        assert!(tool_names.contains(&"scan_map_devices"));
        assert!(tool_names.contains(&"query-docs"));
        assert!(tool_names.contains(&"fetch-doc-source"));

        shutdown.cancel();
        server.await??;
        Ok(())
    }
}

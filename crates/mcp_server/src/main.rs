//! Verse MCP Server - MCP server for UEFN/Verse development

use anyhow::Result;
use std::path::PathBuf;
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

    // Run the MCP server
    run_server(project_path).await
}

/// Run the MCP server with stdio transport
async fn run_server(project_path: PathBuf) -> Result<()> {
    use rmcp::{ServerHandler, RoleState};
    use rmcp::transport::{TokioChildProcess, ConfigureSessionExt};
    use tokio::io::{stdin, stdout};

    // Create server handler
    let handler = VerseMcpHandler { project_path };

    // Create server with handler
    let server = rmcp::Server::new(handler);

    // Serve over stdio
    let service = server
        .connect(stdin(), stdout())
        .await?;

    // Keep running
    service.waiting().await?;

    Ok(())
}

/// Verse MCP Handler implementation
#[derive(Debug, Clone)]
struct VerseMcpHandler {
    project_path: PathBuf,
}

impl ServerHandler for VerseMcpHandler {
    type Error = anyhow::Error;

    fn get_info(&self) -> rmcp::model::ServerInfo {
        rmcp::model::ServerInfo::default()
    }

    async fn list_tools(&self) -> Result<Vec<rmcp::model::Tool>, Self::Error> {
        Ok(vec![
            rmcp::model::Tool {
                name: "scan_map_devices".into(),
                description: "Scan UEFN project for all devices".into(),
                input_schema: Default::default(),
            },
        ])
    }

    async fn call_tool(
        &self,
        name: &str,
        _arguments: serde_json::Value,
    ) -> Result<Vec<rmcp::model::RawContent>, Self::Error> {
        match name {
            "scan_map_devices" => {
                let output = uasset_scan::scan_project(&self.project_path)?;
                let json = serde_json::to_string_pretty(&output)?;
                Ok(vec![rmcp::model::RawContent::text(json)])
            }
            _ => anyhow::bail!("Unknown tool: {}", name),
        }
    }
}

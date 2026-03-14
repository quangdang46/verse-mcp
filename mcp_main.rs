//! Verse MCP Server - MCP server for UEFN/Verse development
//!
//! Provides tools for:
//! - Scanning UEFN projects for devices
//! - Querying Fortnite.digest.verse
//! - Listing @editable fields
//! - UI scaffolding

use anyhow::Result;
use tracing_subscriber::EnvFilter;
use rmcp::transport;

mod cli;
mod cache;
mod digest;
mod handler;
mod tools;
mod parser;
mod grapher;
mod template_manager;
mod classify;
mod fingerprint;
mod lib;

fn load_digest(project_path: &std::path::Path) -> Option<digest::DigestIndex> {
    use crate::digest::DigestIndex;

    // Try common locations for Fortnite.digest.verse
    let digest_paths = vec![
        project_path.join("Fortnite.digest.verse"),
        project_path.join("Content").join("Fortnite.digest.verse"),
        project_path.join("..").join("Fortnite.digest.verse"),
    ];

    for digest_path in digest_paths {
        if digest_path.exists() {
            tracing::info!("Loading digest from: {}", digest_path.display());
            match std::fs::read_to_string(&digest_path) {
                Ok(content) => match DigestIndex::parse(&content) {
                    Ok(index) => {
                        tracing::info!(
                            "Digest loaded: {} devices, {} symbols",
                            index.devices.len(),
                            index.symbols.len()
                        );
                        return Some(index);
                    }
                    Err(e) => {
                        tracing::warn!("Failed to parse digest: {}", e);
                    }
                },
                Err(e) => {
                    tracing::warn!("Failed to read digest file: {}", e);
                }
            }
        }
    }

    tracing::info!("No digest file found, digest tools will return empty results");
    None
}

/// MCP server entry point
#[tokio::main]
async fn main() -> Result<()> {
    use cli::Cli;
    use rmcp::{ServerHandler, ServiceExt};
    use tracing_subscriber::fmt;

    // Parse CLI arguments first - this handles --help before any logging
    let cli = Cli::parse();

    // Initialize logging to stderr (NEVER stdout - MCP uses stdout for protocol)
    fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .with_writer(std::io::stderr)
        .init();

    tracing::info!("Starting Verse MCP Server with transport: {}", cli.transport);

    // Use current directory for templates (templates_dir only)
    let project_path = std::env::current_dir().unwrap_or_default();

    tracing::info!("Templates directory: {}", project_path.display());

    // Load digest index from project path (or None if not found)
    let digest_index = load_digest(&project_path);

    // Create server handler (project_path is default for templates only)
    let templates_dir = project_path.join("templates");
    let handler = handler::VerseMcpHandler {
        project_path,
        cache: std::sync::Mutex::new(None),
        digest: std::sync::RwLock::new(digest_index),
        templates_dir,
    };

    // Select transport mode
    match cli.transport.as_str() {
        "http" => {
            let addr = format!("{}:{}", cli.host, cli.port);
            tracing::info!("Starting HTTP server on {}", addr);

            let bind_addr: std::net::SocketAddr = addr.parse()?;
            let sse_server = SseServer::serve(bind_addr).await?;
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

//! Verse MCP Server - MCP server for UEFN/Verse development
//!
//! Provides tools for:
//! - Scanning UEFN projects for devices
//! - Querying Fortnite.digest.verse
//! - Listing @editable fields
//! - UI scaffolding

use anyhow::Result;
use tracing_subscriber::EnvFilter;

mod tools;

/// MCP server entry point
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("info"))
        .with_writer(std::io::stderr)
        .init();

    tracing::info!("Starting Verse MCP Server...");

    // TODO: Initialize MCP server with rmcp
    // For now, just indicate the server is ready
    tracing::info!("MCP server skeleton ready - implementation pending");

    Ok(())
}

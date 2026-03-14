//! CLI arguments for the Verse MCP Server

use clap::Parser;

/// CLI arguments for the MCP server
#[derive(Parser, Debug)]
#[command(name = "vm")]
#[command(about = "Verse MCP Server for UEFN/Verse development", long_about = None)]
pub struct Cli {
    /// Transport type: stdio or http
    #[arg(short, long, default_value = "stdio")]
    pub transport: String,

    /// Host for HTTP transport (ignored for stdio)
    #[arg(short, long, default_value = "127.0.0.1")]
    pub host: String,

    /// Port for HTTP transport (ignored for stdio)
    #[arg(short, long, default_value = "2003")]
    pub port: u16,
}

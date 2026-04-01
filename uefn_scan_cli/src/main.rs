//! UEFN Scanner CLI - Scan UEFN projects for device information
//!
//! Usage:
//!   verse-scan --dir /path/to/UEFN/Project
//!   verse-scan --dir /path/to/UEFN/Project --out result.json

use anyhow::Result;
use clap::Parser;
use grounding_engine::{GroundingEngine, ScanProjectRequest};
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

/// UEFN project scanner
#[derive(Parser, Debug)]
#[command(name = "verse-scan")]
#[command(about = "Scan UEFN projects for device information")]
struct Args {
    /// Path to UEFN project directory
    #[arg(short, long)]
    dir: PathBuf,

    /// Output file path (JSON)
    #[arg(short, long)]
    out: Option<PathBuf>,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging
    let filter = if args.verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();

    eprintln!("[verse-scan] Project: {}", args.dir.display());

    // Scan the project
    let engine = GroundingEngine::default();
    let response = engine.scan_project(&ScanProjectRequest {
        project_path: args.dir.clone(),
        force_refresh: false,
    })?;
    let output = response.output;

    eprintln!("[verse-scan] Files: {} .uasset", output.total_files);
    eprintln!("[verse-scan] Devices: {} parsed", output.total_devices);
    eprintln!(
        "[verse-scan] Skipped: {} (non-device assets)",
        output.skipped
    );

    let json = serde_json::to_string_pretty(&output)?;

    if let Some(out_path) = &args.out {
        std::fs::write(out_path, &json)?;
        eprintln!("[verse-scan] Saved → {}", out_path.display());
    } else {
        println!("{}", json);
    }

    Ok(())
}

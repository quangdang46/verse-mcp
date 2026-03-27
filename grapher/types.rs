//! Grapher types

use serde::{Deserialize, Serialize};

/// Output format for graph generation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GraphFormat {
    /// Mermaid diagram format (for markdown rendering)
    Mermaid,
    /// Graphviz DOT format
    Dot,
}

/// Connection between devices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConnection {
    /// Source device ID
    pub source: String,
    /// Target device ID
    pub target: String,
    /// Channel name (if applicable)
    pub channel: Option<String>,
    /// Trigger event (if applicable)
    pub trigger: Option<String>,
}

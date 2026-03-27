//! Validator types

use serde::{Deserialize, Serialize};

/// A validation issue found in Verse code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    /// Severity level
    pub severity: Severity,
    /// Issue message
    pub message: String,
    /// Line number in source (1-indexed)
    pub line: usize,
    /// Column number in source (1-indexed)
    pub column: usize,
    /// Suggested fix (if available)
    pub suggestion: Option<String>,
    /// Kind of symbol that caused the issue
    pub symbol_kind: String,
}

/// Severity level for validation issues
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Severity {
    /// Definitely wrong (unknown symbol)
    Error,
    /// Possibly wrong (suspicious pattern)
    Warning,
    /// Informational (style suggestion)
    Info,
}

/// Kind of wiring issue detected
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IssueKind {
    /// Channel has sender but no receiver
    OrphanedSender,
    /// Channel has receiver but no sender
    OrphanedReceiver,
    /// Multiple devices send on same channel
    ChannelConflict,
    /// Device has triggers but no channel configured
    UnconfiguredTrigger,
    /// Device has receivers but no channel configured
    UnconfiguredReceiver,
    /// Potential cycle detected in device chain
    PotentialCycle,
}

/// A detected wiring issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WiringIssue {
    /// Kind of issue
    pub kind: IssueKind,
    /// Channel name if applicable
    pub channel: Option<String>,
    /// Device labels involved
    pub devices: Vec<String>,
    /// Human-readable description
    pub message: String,
    /// Suggested fix
    pub suggestion: Option<String>,
}

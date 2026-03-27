//! Digest data structures

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error type for digest parsing
#[derive(Debug, Error)]
pub enum DigestError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Invalid digest format")]
    InvalidFormat,
}

/// Parsed digest index containing all device definitions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DigestIndex {
    /// Device definitions indexed by normalized name (lowercase)
    pub devices: IndexMap<String, DeviceDef>,
    /// Symbol lookup: symbol name -> locations where it's defined
    pub symbols: IndexMap<String, Vec<SymbolLocation>>,
}

/// A device definition from digest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDef {
    /// Device name as it appears in digest (e.g., "device_campfire_device")
    pub name: String,
    /// Trigger events (e.g., TriggerOnEnterRadius, OnDisabled)
    pub events: Vec<Event>,
    /// Methods callable on this device
    pub methods: Vec<Method>,
    /// Properties (rare, mostly for creative devices)
    pub properties: Vec<Property>,
}

/// An event definition (triggers and receivers)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Event name (e.g., "TriggerOnEnterRadius")
    pub name: String,
    /// Parameter signature
    pub params: Vec<Param>,
    /// Return type (usually "void" for events)
    pub return_type: String,
    /// Whether this is a receiver (starts with "Receiver" or ends with "WhenReceiving")
    pub is_receiver: bool,
}

/// A method definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    /// Method name
    pub name: String,
    /// Parameter signature
    pub params: Vec<Param>,
    /// Return type
    pub return_type: String,
}

/// A property definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    /// Property name
    pub name: String,
    /// Property type
    pub type_name: String,
}

/// A parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub type_name: String,
}

/// Location where a symbol is defined
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolLocation {
    /// Device containing this symbol
    pub device: String,
    /// Symbol kind
    pub kind: SymbolKind,
}

/// Kind of symbol
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SymbolKind {
    Device,
    Event,
    Method,
    Property,
}

/// Search result from digest query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Kind of match
    pub kind: String,
    /// Symbol name
    pub name: String,
    /// Device this belongs to (if applicable)
    pub device: Option<String>,
    /// Formatted signature
    pub signature: String,
}

/// Kind of change in a diff
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChangeKind {
    /// Symbol was added
    Added,
    /// Symbol was removed
    Removed,
    /// Symbol signature changed
    Modified,
}

/// A single change between two digest versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigestChange {
    /// Kind of change
    pub kind: ChangeKind,
    /// Symbol type (device, event, method)
    pub symbol_type: String,
    /// Symbol name
    pub name: String,
    /// Device this belongs to (if applicable)
    pub device: Option<String>,
    /// Old signature (for modified/removed)
    pub old_signature: Option<String>,
    /// New signature (for modified/added)
    pub new_signature: Option<String>,
}

/// Result of comparing two digest versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigestDiff {
    /// Breaking changes (removed/modified APIs)
    pub breaking_changes: Vec<DigestChange>,
    /// Non-breaking changes (new APIs)
    pub additions: Vec<DigestChange>,
    /// Summary stats
    pub stats: DiffStats,
}

/// Summary statistics for a diff
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiffStats {
    /// Total devices added
    pub devices_added: usize,
    /// Total devices removed
    pub devices_removed: usize,
    /// Total events added
    pub events_added: usize,
    /// Total events removed
    pub events_removed: usize,
    /// Total methods added
    pub methods_added: usize,
    /// Total methods removed
    pub methods_removed: usize,
}

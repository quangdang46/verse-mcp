//! Digest parser for Fortnite.digest.verse
//!
//! Parses the Verse digest file to extract device definitions, events, methods,
//! and properties for API validation and lookup.

use indexmap::IndexMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

/// Parsed digest index containing all device definitions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DigestIndex {
    /// Device definitions indexed by normalized name (lowercase)
    pub devices: IndexMap<String, DeviceDef>,
    /// Symbol lookup: symbol name -> locations where it's defined
    pub symbols: IndexMap<String, Vec<SymbolLocation>>,
}

/// A device definition from the digest
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

// Regex patterns for parsing (compiled once)
static DEVICE_DECL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^([a-z][a-z0-9_]*_device)\s*=\s*class\s*\(\s*\)\s*:")
        .expect("Invalid device decl regex")
});

static EVENT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^([A-Za-z][A-Za-z0-9_]*)\s*\(([^)]*)\)\s*:\s*event\s*(?:\(([^)]*)\)\s*)?:\s*(\w+)")
        .expect("Invalid event regex")
});

static METHOD_RE: LazyLock<Regex> = LazyLock::new(|| {
    // Match function-like declarations: Name(params):return_type
    // Note: Events are checked separately first, so this only matches non-event methods
    Regex::new(r"^([A-Za-z][A-Za-z0-9_]*)\s*\(([^)]*)\)\s*:\s*(\w+)").expect("Invalid method regex")
});

static PARAM_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\w+)\s*:\s*([^,\s]+)").expect("Invalid param regex"));

impl DigestIndex {
    /// Parse digest content into an index
    pub fn parse(content: &str) -> Result<Self, DigestError> {
        let mut index = DigestIndex::default();
        let mut current_device: Option<String> = None;

        for line in content.lines() {
            let trimmed = line.trim();

            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            // Check for device class declaration
            if let Some(caps) = DEVICE_DECL_RE.captures(trimmed) {
                let name = caps[1].to_string();
                current_device = Some(name.clone());
                index
                    .devices
                    .entry(name.clone())
                    .or_insert_with(|| DeviceDef {
                        name,
                        events: Vec::new(),
                        methods: Vec::new(),
                        properties: Vec::new(),
                    });
                continue;
            }

            // Must be inside a device block to parse events/methods
            let device_name = match &current_device {
                Some(name) => name.clone(),
                None => continue,
            };

            // Try to parse as event
            if let Some(caps) = EVENT_RE.captures(trimmed) {
                let name = caps[1].to_string();
                let params_str = &caps[2];
                let return_type = caps[4].to_string();

                let params = parse_params(params_str);
                let is_receiver = name.starts_with("Receiver") || name.ends_with("WhenReceiving");

                let event = Event {
                    name: name.clone(),
                    params,
                    return_type,
                    is_receiver,
                };

                if let Some(device) = index.devices.get_mut(&device_name) {
                    device.events.push(event);
                }

                // Add to symbol index
                index
                    .symbols
                    .entry(name.clone())
                    .or_default()
                    .push(SymbolLocation {
                        device: device_name.clone(),
                        kind: SymbolKind::Event,
                    });
                continue;
            }

            // Try to parse as method (not an event)
            if let Some(caps) = METHOD_RE.captures(trimmed) {
                let name = caps[1].to_string();
                let params_str = &caps[2];
                let return_type = caps[3].to_string();

                let params = parse_params(params_str);

                // Check if this is a receiver (starts with "Receiver" or ends with "WhenReceiving")
                let is_receiver = name.starts_with("Receiver") || name.ends_with("WhenReceiving");

                if is_receiver {
                    // Receivers are stored as events with is_receiver=true
                    let event = Event {
                        name: name.clone(),
                        params,
                        return_type,
                        is_receiver: true,
                    };

                    if let Some(device) = index.devices.get_mut(&device_name) {
                        device.events.push(event);
                    }

                    // Add to symbol index
                    index
                        .symbols
                        .entry(name.clone())
                        .or_default()
                        .push(SymbolLocation {
                            device: device_name.clone(),
                            kind: SymbolKind::Event,
                        });
                } else {
                    // Regular method
                    let method = Method {
                        name: name.clone(),
                        params,
                        return_type,
                    };

                    if let Some(device) = index.devices.get_mut(&device_name) {
                        device.methods.push(method);
                    }

                    // Add to symbol index
                    index
                        .symbols
                        .entry(name.clone())
                        .or_default()
                        .push(SymbolLocation {
                            device: device_name.clone(),
                            kind: SymbolKind::Method,
                        });
                }
                continue;
            }

            // Check for end of device block (dedent or next top-level declaration)
            if !line.starts_with(' ') && !line.starts_with('\t') && !trimmed.starts_with('#') {
                // This is a top-level line, check if it's a new device or end of block
                if DEVICE_DECL_RE.captures(trimmed).is_none() && !trimmed.starts_with("using") {
                    // End of device block
                    current_device = None;
                }
            }
        }

        // Add device names to symbol index
        for device_name in index.devices.keys() {
            index
                .symbols
                .entry(device_name.clone())
                .or_default()
                .push(SymbolLocation {
                    device: device_name.clone(),
                    kind: SymbolKind::Device,
                });
        }

        Ok(index)
    }

    /// Search for devices matching query
    pub fn search_devices(&self, query: &str) -> Vec<SearchResult> {
        let query_lower = query.to_lowercase();
        self.devices
            .values()
            .filter(|d| d.name.to_lowercase().contains(&query_lower))
            .map(|d| SearchResult {
                kind: "device".to_string(),
                name: d.name.clone(),
                device: None,
                signature: format_device(d),
            })
            .collect()
    }

    /// Search for events matching query
    pub fn search_events(&self, query: &str) -> Vec<SearchResult> {
        let query_lower = query.to_lowercase();
        self.devices
            .values()
            .flat_map(|d| {
                d.events.iter().filter_map(|e| {
                    if e.name.to_lowercase().contains(&query_lower) {
                        Some(SearchResult {
                            kind: if e.is_receiver { "receiver" } else { "trigger" }.to_string(),
                            name: e.name.clone(),
                            device: Some(d.name.clone()),
                            signature: format_event(e),
                        })
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    /// Search for methods matching query
    pub fn search_methods(&self, query: &str) -> Vec<SearchResult> {
        let query_lower = query.to_lowercase();
        self.devices
            .values()
            .flat_map(|d| {
                d.methods.iter().filter_map(|m| {
                    if m.name.to_lowercase().contains(&query_lower) {
                        Some(SearchResult {
                            kind: "method".to_string(),
                            name: m.name.clone(),
                            device: Some(d.name.clone()),
                            signature: format_method(m),
                        })
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    /// Search all symbols
    pub fn search_all(&self, query: &str) -> Vec<SearchResult> {
        let mut results = Vec::new();
        results.extend(self.search_devices(query));
        results.extend(self.search_events(query));
        results.extend(self.search_methods(query));
        results
    }

    /// Get device by name (handles normalization)
    pub fn get_device(&self, name: &str) -> Option<&DeviceDef> {
        let normalized = normalize_device_name(name);
        self.devices.get(&normalized)
    }

    /// Validate if a symbol exists in the digest
    pub fn symbol_exists(&self, symbol: &str) -> bool {
        self.symbols.contains_key(symbol) || self.devices.values().any(|d| d.name == symbol)
    }

    /// Compare this digest with another version
    pub fn diff(&self, other: &Self) -> DigestDiff {
        let mut breaking_changes = Vec::new();
        let mut additions = Vec::new();
        let mut stats = DiffStats::default();

        // Compare devices
        for device_name in self.devices.keys() {
            if !other.devices.contains_key(device_name) {
                // Device removed
                let device = &self.devices[device_name];
                breaking_changes.push(DigestChange {
                    kind: ChangeKind::Removed,
                    symbol_type: "device".to_string(),
                    name: device.name.clone(),
                    device: None,
                    old_signature: Some(format_device(device)),
                    new_signature: None,
                });
                stats.devices_removed += 1;
            } else {
                // Device exists in both, compare events/methods
                let old_device = &self.devices[device_name];
                let new_device = &other.devices[device_name];

                // Compare events
                let old_events: std::collections::HashMap<&str, &Event> =
                    old_device.events.iter().map(|e| (e.name.as_str(), e)).collect();
                let new_events: std::collections::HashMap<&str, &Event> =
                    new_device.events.iter().map(|e| (e.name.as_str(), e)).collect();

                for (name, event) in &old_events {
                    if !new_events.contains_key(*name) {
                        // Event removed
                        breaking_changes.push(DigestChange {
                            kind: ChangeKind::Removed,
                            symbol_type: if event.is_receiver {
                                "receiver"
                            } else {
                                "trigger"
                            }
                            .to_string(),
                            name: event.name.clone(),
                            device: Some(device_name.clone()),
                            old_signature: Some(format_event(event)),
                            new_signature: None,
                        });
                        stats.events_removed += 1;
                    } else {
                        // Event exists in both, check if modified
                        let new_event = new_events[*name];
                        if event_signature_changed(event, new_event) {
                            breaking_changes.push(DigestChange {
                                kind: ChangeKind::Modified,
                                symbol_type: if event.is_receiver {
                                    "receiver"
                                } else {
                                    "trigger"
                                }
                                .to_string(),
                                name: event.name.clone(),
                                device: Some(device_name.clone()),
                                old_signature: Some(format_event(event)),
                                new_signature: Some(format_event(new_event)),
                            });
                        }
                    }
                }

                for (name, event) in &new_events {
                    if !old_events.contains_key(*name) {
                        // Event added
                        additions.push(DigestChange {
                            kind: ChangeKind::Added,
                            symbol_type: if event.is_receiver {
                                "receiver"
                            } else {
                                "trigger"
                            }
                            .to_string(),
                            name: event.name.clone(),
                            device: Some(device_name.clone()),
                            old_signature: None,
                            new_signature: Some(format_event(event)),
                        });
                        stats.events_added += 1;
                    }
                }

                // Compare methods
                let old_methods: std::collections::HashMap<&str, &Method> =
                    old_device.methods.iter().map(|m| (m.name.as_str(), m)).collect();
                let new_methods: std::collections::HashMap<&str, &Method> =
                    new_device.methods.iter().map(|m| (m.name.as_str(), m)).collect();

                for (name, method) in &old_methods {
                    if !new_methods.contains_key(*name) {
                        // Method removed
                        breaking_changes.push(DigestChange {
                            kind: ChangeKind::Removed,
                            symbol_type: "method".to_string(),
                            name: method.name.clone(),
                            device: Some(device_name.clone()),
                            old_signature: Some(format_method(method)),
                            new_signature: None,
                        });
                        stats.methods_removed += 1;
                    } else {
                        // Method exists in both, check if modified
                        let new_method = new_methods[*name];
                        if method_signature_changed(method, new_method) {
                            breaking_changes.push(DigestChange {
                                kind: ChangeKind::Modified,
                                symbol_type: "method".to_string(),
                                name: method.name.clone(),
                                device: Some(device_name.clone()),
                                old_signature: Some(format_method(method)),
                                new_signature: Some(format_method(new_method)),
                            });
                        }
                    }
                }

                for (name, method) in &new_methods {
                    if !old_methods.contains_key(*name) {
                        // Method added
                        additions.push(DigestChange {
                            kind: ChangeKind::Added,
                            symbol_type: "method".to_string(),
                            name: method.name.clone(),
                            device: Some(device_name.clone()),
                            old_signature: None,
                            new_signature: Some(format_method(method)),
                        });
                        stats.methods_added += 1;
                    }
                }
            }
        }

        // Check for new devices
        for device_name in other.devices.keys() {
            if !self.devices.contains_key(device_name) {
                // Device added
                let device = &other.devices[device_name];
                additions.push(DigestChange {
                    kind: ChangeKind::Added,
                    symbol_type: "device".to_string(),
                    name: device.name.clone(),
                    device: None,
                    old_signature: None,
                    new_signature: Some(format_device(device)),
                });
                stats.devices_added += 1;
            }
        }

        DigestDiff {
            breaking_changes,
            additions,
            stats,
        }
    }
}

/// Default implementation for DiffStats
impl Default for DiffStats {
    fn default() -> Self {
        Self {
            devices_added: 0,
            devices_removed: 0,
            events_added: 0,
            events_removed: 0,
            methods_added: 0,
            methods_removed: 0,
        }
    }
}

/// Check if event signature changed (params or return type)
fn event_signature_changed(old: &Event, new: &Event) -> bool {
    if old.return_type != new.return_type {
        return true;
    }
    if old.params.len() != new.params.len() {
        return true;
    }
    for (o, n) in old.params.iter().zip(new.params.iter()) {
        if o.name != n.name || o.type_name != n.type_name {
            return true;
        }
    }
    false
}

/// Check if method signature changed (params or return type)
fn method_signature_changed(old: &Method, new: &Method) -> bool {
    if old.return_type != new.return_type {
        return true;
    }
    if old.params.len() != new.params.len() {
        return true;
    }
    for (o, n) in old.params.iter().zip(new.params.iter()) {
        if o.name != n.name || o.type_name != n.type_name {
            return true;
        }
    }
    false
}

/// Parse parameter string into Param structs
fn parse_params(params_str: &str) -> Vec<Param> {
    if params_str.trim().is_empty() {
        return Vec::new();
    }

    PARAM_RE
        .captures_iter(params_str)
        .map(|caps| Param {
            name: caps[1].to_string(),
            type_name: caps[2].to_string(),
        })
        .collect()
}

/// Normalize device name to canonical form
pub fn normalize_device_name(name: &str) -> String {
    let name = name.trim();

    // Handle UE naming: Device_Campfire_C -> device_campfire_device
    if name.starts_with("Device_") && name.ends_with("_C") {
        let base = &name[7..name.len() - 2];
        format!("device_{}_device", base.to_lowercase())
    } else {
        name.to_lowercase()
    }
}

/// Format device for display
fn format_device(d: &DeviceDef) -> String {
    format!(
        "{} = class() # {} events, {} methods",
        d.name,
        d.events.len(),
        d.methods.len()
    )
}

/// Format event for display
fn format_event(e: &Event) -> String {
    let params = e
        .params
        .iter()
        .map(|p| format!("{}:{}", p.name, p.type_name))
        .collect::<Vec<_>>()
        .join(", ");

    if e.is_receiver {
        format!("{}({}):void # receiver", e.name, params)
    } else {
        format!("{}({}):event:void # trigger", e.name, params)
    }
}

/// Format method for display
fn format_method(m: &Method) -> String {
    let params = m
        .params
        .iter()
        .map(|p| format!("{}:{}", p.name, p.type_name))
        .collect::<Vec<_>>()
        .join(", ");

    format!("{}({}):{}", m.name, params, m.return_type)
}

/// Error type for digest parsing
#[derive(Debug, thiserror::Error)]
pub enum DigestError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error at line {line}: {message}")]
    Parse { line: usize, message: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_device() {
        let digest = r#"
device_campfire_device = class():
    TriggerOnEnterRadius():event():void
    OnDisabled():event():void
    ReceiverAddFuel(Amount:int):void
    AddFuel(Amount:int):void
    Extinguish():void
"#;
        let index = DigestIndex::parse(digest).unwrap();

        assert!(index.devices.contains_key("device_campfire_device"));
        let device = &index.devices["device_campfire_device"];

        assert_eq!(device.events.len(), 3);
        assert_eq!(device.methods.len(), 2);

        // Check triggers vs receivers
        let triggers: Vec<_> = device.events.iter().filter(|e| !e.is_receiver).collect();
        let receivers: Vec<_> = device.events.iter().filter(|e| e.is_receiver).collect();
        assert_eq!(triggers.len(), 2);
        assert_eq!(receivers.len(), 1);
    }

    #[test]
    fn test_normalize_device_name() {
        assert_eq!(
            normalize_device_name("Device_Campfire_C"),
            "device_campfire_device"
        );
        assert_eq!(
            normalize_device_name("device_campfire_device"),
            "device_campfire_device"
        );
    }

    #[test]
    fn test_search() {
        let digest = r#"
device_campfire_device = class():
    TriggerOnEnterRadius():event():void
    AddFuel(Amount:int):void

device_button_device = class():
    InteractWithAgent():event(such as {Agent}):void
"#;
        let index = DigestIndex::parse(digest).unwrap();

        let results = index.search_devices("camp");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "device_campfire_device");

        let results = index.search_events("trigger");
        assert_eq!(results.len(), 1);

        let results = index.search_methods("add");
        assert_eq!(results.len(), 1);
    }
}

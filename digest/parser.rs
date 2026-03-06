//! Digest parsing logic

use regex::Regex;
use std::sync::LazyLock;

use super::types::{DigestIndex, DeviceDef, Event, Method, Param, SymbolLocation, SymbolKind};

use crate::digest::DigestError;

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

        for (line_num, line) in content.lines().enumerate() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::types::{ChangeKind};

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
}

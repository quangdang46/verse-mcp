//! Digest search functionality

use super::types::{DigestIndex, SearchResult, Event, Method, DeviceDef};

impl DigestIndex {
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
        let normalized = super::parser::normalize_device_name(name);
        self.devices.get(&normalized)
    }

    /// Validate if a symbol exists in digest
    pub fn symbol_exists(&self, symbol: &str) -> bool {
        self.symbols.contains_key(symbol) || self.devices.values().any(|d| d.name == symbol)
    }

    /// Compare this digest with another version
    pub fn diff(&self, other: &Self) -> super::types::DigestDiff {
        let mut breaking_changes = Vec::new();
        let mut additions = Vec::new();
        let mut stats = super::types::DiffStats::default();

        // Compare devices
        for device_name in self.devices.keys() {
            if !other.devices.contains_key(device_name) {
                // Device removed
                let device = &self.devices[device_name];
                breaking_changes.push(super::types::DigestChange {
                    kind: super::types::ChangeKind::Removed,
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
                        breaking_changes.push(super::types::DigestChange {
                            kind: super::types::ChangeKind::Removed,
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
                            breaking_changes.push(super::types::DigestChange {
                                kind: super::types::ChangeKind::Modified,
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
                        additions.push(super::types::DigestChange {
                            kind: super::types::ChangeKind::Added,
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
                        breaking_changes.push(super::types::DigestChange {
                            kind: super::types::ChangeKind::Removed,
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
                            breaking_changes.push(super::types::DigestChange {
                                kind: super::types::ChangeKind::Modified,
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
                        additions.push(super::types::DigestChange {
                            kind: super::types::ChangeKind::Added,
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
                additions.push(super::types::DigestChange {
                    kind: super::types::ChangeKind::Added,
                    symbol_type: "device".to_string(),
                    name: device.name.clone(),
                    device: None,
                    old_signature: None,
                    new_signature: Some(format_device(device)),
                });
                stats.devices_added += 1;
            }
        }

        super::types::DigestDiff {
            breaking_changes,
            additions,
            stats,
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::types::{ChangeKind};

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

    #[test]
    fn test_diff_device_removed() {
        let old = DigestIndex::parse(
            "device_campfire_device = class():\n    AddFuel(Amount:int):void\n\ndevice_button_device = class():\n    Press():void\n",
        ).unwrap();
        let new = DigestIndex::parse(
            "device_campfire_device = class():\n    AddFuel(Amount:int):void\n",
        ).unwrap();

        let diff = old.diff(&new);
        assert_eq!(diff.stats.devices_removed, 1);
        assert!(diff.breaking_changes.iter().any(|c| c.name == "device_button_device" && c.kind == ChangeKind::Removed));
    }

    #[test]
    fn test_diff_device_added() {
        let old = DigestIndex::parse(
            "device_campfire_device = class():\n    AddFuel(Amount:int):void\n",
        ).unwrap();
        let new = DigestIndex::parse(
            "device_campfire_device = class():\n    AddFuel(Amount:int):void\n\ndevice_button_device = class():\n    Press():void\n",
        ).unwrap();

        let diff = old.diff(&new);
        assert_eq!(diff.stats.devices_added, 1);
        assert!(diff.additions.iter().any(|c| c.name == "device_button_device" && c.kind == ChangeKind::Added));
    }

    #[test]
    fn test_diff_event_removed() {
        let old = DigestIndex::parse(
            "device_campfire_device = class():\n    TriggerOnEnterRadius():event():void\n    OnDisabled():event():void\n",
        ).unwrap();
        let new = DigestIndex::parse(
            "device_campfire_device = class():\n    TriggerOnEnterRadius():event():void\n",
        ).unwrap();

        let diff = old.diff(&new);
        assert_eq!(diff.stats.events_removed, 1);
        assert!(diff.breaking_changes.iter().any(|c| c.name == "OnDisabled" && c.kind == ChangeKind::Removed));
    }
}

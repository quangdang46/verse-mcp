//! Device graph generator

use std::collections::{HashMap, HashSet};

use crate::lib_internal::{DeviceInfo, ScanOutput};
use super::types::{GraphFormat, DeviceConnection};

/// Device graph generator
pub struct DeviceGrapher;

impl DeviceGrapher {
    /// Generate graph diagram from scan output
    pub fn generate(output: &ScanOutput, format: GraphFormat) -> String {
        let devices: Vec<_> = output.by_type.values().flatten().collect();
        let connections = Self::extract_connections(&devices);

        match format {
            GraphFormat::Mermaid => Self::to_mermaid(&devices, &connections),
            GraphFormat::Dot => Self::to_dot(&devices, &connections),
        }
    }

    /// Extract connections from device settings
    fn extract_connections(devices: &[&DeviceInfo]) -> Vec<DeviceConnection> {
        let mut connections = Vec::new();
        let mut channel_senders: HashMap<String, Vec<String>> = HashMap::new();
        let mut channel_receivers: HashMap<String, Vec<String>> = HashMap::new();

        // First pass: collect channel senders and receivers
        for device in devices {
            let device_id = sanitize_id(&device.file);

            if let Some(ref settings) = device.settings {
                for (key, value) in settings {
                    let key_lower = key.to_lowercase();

                    // Check for channel triggers (senders)
                    if key_lower.contains("trigger") && key_lower.contains("channel") {
                        channel_senders
                            .entry(value.clone())
                            .or_default()
                            .push(device_id.clone());
                    }

                    // Check for channel receivers
                    if key_lower.contains("receiver") && key_lower.contains("channel") {
                        channel_receivers
                            .entry(value.clone())
                            .or_default()
                            .push(device_id.clone());
                    }

                    // Check for "when receiving" on channel
                    if key_lower.contains("when") && key_lower.contains("receiving") && key_lower.contains("channel") {
                        channel_receivers
                            .entry(value.clone())
                            .or_default()
                            .push(device_id.clone());
                    }
                }
            }
        }

        // Second pass: create connections from channels
        for (channel, senders) in &channel_senders {
            if let Some(receivers) = channel_receivers.get(channel) {
                for sender in senders {
                    for receiver in receivers {
                        if sender != receiver {
                            connections.push(DeviceConnection {
                                source: sender.clone(),
                                target: receiver.clone(),
                                channel: Some(channel.clone()),
                                trigger: None,
                            });
                        }
                    }
                }
            }
        }

        connections
    }

    /// Generate Mermaid diagram
    fn to_mermaid(devices: &[&DeviceInfo], connections: &[DeviceConnection]) -> String {
        let mut lines = vec!["graph LR".to_string()];
        let mut seen_nodes: HashSet<String> = HashSet::new();

        // Node definitions
        for device in devices {
            let node_id = sanitize_id(&device.file);
            if seen_nodes.contains(&node_id) {
                continue;
            }
            seen_nodes.insert(node_id.clone());

            let label = device
                .label
                .as_deref()
                .unwrap_or(&device.device_type);
            let safe_label = escape_mermaid_label(label);
            lines.push(format!("  {}[\"{}\"]", node_id, safe_label));
        }

        // Add subgraph for device types if we have multiple of same type
        let mut type_groups: HashMap<String, Vec<String>> = HashMap::new();
        for device in devices {
            type_groups
                .entry(device.device_type.clone())
                .or_default()
                .push(sanitize_id(&device.file));
        }

        // Connections
        let mut seen_connections: HashSet<(String, String)> = HashSet::new();
        for conn in connections {
            let key = (conn.source.clone(), conn.target.clone());
            if seen_connections.contains(&key) {
                continue;
            }
            seen_connections.insert(key.clone());

            let label = match (&conn.channel, &conn.trigger) {
                (Some(ch), Some(tr)) => format!("{}:{}", ch, tr),
                (Some(ch), None) => ch.clone(),
                (None, Some(tr)) => tr.clone(),
                (None, None) => "".to_string(),
            };

            if label.is_empty() {
                lines.push(format!("  {} --> {}", conn.source, conn.target));
            } else {
                lines.push(format!("  {} -->|\"{}\"| {}", conn.source, label, conn.target));
            }
        }

        // Add styling
        lines.push("".to_string());
        lines.push("  %% Styling".to_string());
        lines.push("  classDef device fill:#e1f5fe,stroke:#01579b,stroke-width:2px".to_string());
        lines.push("  classDef trigger fill:#fff3e0,stroke:#e65100,stroke-width:2px".to_string());
        lines.push("  classDef receiver fill:#e8f5e9,stroke:#1b5e20,stroke-width:2px".to_string());

        lines.join("\n")
    }

    /// Generate Graphviz DOT diagram
    fn to_dot(devices: &[&DeviceInfo], connections: &[DeviceConnection]) -> String {
        let mut lines = vec!["digraph DeviceGraph {".to_string()];
        lines.push("  rankdir=LR;".to_string());
        lines.push("  node [shape=box, style=filled, fillcolor=\"#e1f5fe\"];".to_string());
        lines.push("  edge [color=\"#666666\"];".to_string());
        lines.push("".to_string());

        let mut seen_nodes: HashSet<String> = HashSet::new();

        // Node definitions
        for device in devices {
            let node_id = sanitize_id(&device.file);
            if seen_nodes.contains(&node_id) {
                continue;
            }
            seen_nodes.insert(node_id.clone());

            let label = device
                .label
                .as_deref()
                .unwrap_or(&device.device_type);
            lines.push(format!("  {} [label=\"{}\"];", node_id, escape_dot_label(label)));
        }

        lines.push("".to_string());

        // Connections
        let mut seen_connections: HashSet<(String, String)> = HashSet::new();
        for conn in connections {
            let key = (conn.source.clone(), conn.target.clone());
            if seen_connections.contains(&key) {
                continue;
            }
            seen_connections.insert(key.clone());

            let label = match (&conn.channel, &conn.trigger) {
                (Some(ch), Some(tr)) => format!("{}:{}", ch, tr),
                (Some(ch), None) => ch.clone(),
                (None, Some(tr)) => tr.clone(),
                (None, None) => "".to_string(),
            };

            if label.is_empty() {
                lines.push(format!("  {} -> {};", conn.source, conn.target));
            } else {
                lines.push(format!("  {} -> {} [label=\"{}\"];", conn.source, conn.target, label));
            }
        }

        lines.push("}".to_string());
        lines.join("\n")
    }
}

/// Sanitize file path to valid graph node ID
fn sanitize_id(path: &str) -> String {
    // Extract just the filename without extension
    let filename = path
        .rsplit('/')
        .next()
        .unwrap_or(path)
        .replace(".uasset", "");

    // Replace invalid characters and prefix with 'n' if starts with digit
    let sanitized: String = filename
        .chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => c,
            _ => '_',
        })
        .collect();

    if sanitized.starts_with(|c: char| c.is_ascii_digit()) {
        format!("n_{}", sanitized)
    } else {
        sanitized
    }
}

/// Escape special characters in Mermaid labels
fn escape_mermaid_label(label: &str) -> String {
    label
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

/// Escape special characters in DOT labels
fn escape_dot_label(label: &str) -> String {
    label
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    fn create_test_devices() -> Vec<DeviceInfo> {
        let mut button_settings = IndexMap::new();
        button_settings.insert("TriggerOnPressedChannel".to_string(), "game_start".to_string());

        let mut spawner_settings = IndexMap::new();
        spawner_settings.insert("EnableWhenReceivingOnChannel".to_string(), "game_start".to_string());

        vec![
            DeviceInfo {
                file: "Button1.uasset".to_string(),
                device_type: "button_device".to_string(),
                label: Some("Start Button".to_string()),
                triggers: vec!["OnPressed".to_string()],
                receivers: vec![],
                settings: Some(button_settings),
            },
            DeviceInfo {
                file: "Spawner1.uasset".to_string(),
                device_type: "player_spawner_device".to_string(),
                label: Some("Main Spawner".to_string()),
                triggers: vec![],
                receivers: vec!["EnableWhenReceiving".to_string()],
                settings: Some(spawner_settings),
            },
            DeviceInfo {
                file: "Campfire1.uasset".to_string(),
                device_type: "campfire_device".to_string(),
                label: None,
                triggers: vec!["TriggerOnEnterRadius".to_string()],
                receivers: vec![],
                settings: None,
            },
        ]
    }

    #[test]
    fn test_sanitize_id() {
        assert_eq!(sanitize_id("path/to/Button1.uasset"), "Button1");
        assert_eq!(sanitize_id("123Device.uasset"), "n_123Device");
        assert_eq!(sanitize_id("device-with-dashes.uasset"), "device_with_dashes");
    }

    #[test]
    fn test_mermaid_output() {
        let devices = create_test_devices();
        let device_refs: Vec<_> = devices.iter().collect();
        let connections = DeviceGrapher::extract_connections(&device_refs);

        let output = DeviceGrapher::to_mermaid(&device_refs, &connections);

        assert!(output.starts_with("graph LR"));
        assert!(output.contains("Button1[\"Start Button\"]"));
        assert!(output.contains("Spawner1[\"Main Spawner\"]"));
        assert!(output.contains("Campfire1[\"campfire_device\"]"));
        assert!(output.contains("Button1 -->"));
        assert!(output.contains("Spawner1"));
    }

    #[test]
    fn test_dot_output() {
        let devices = create_test_devices();
        let device_refs: Vec<_> = devices.iter().collect();
        let connections = DeviceGrapher::extract_connections(&device_refs);

        let output = DeviceGrapher::to_dot(&device_refs, &connections);

        assert!(output.starts_with("digraph DeviceGraph"));
        assert!(output.contains("rankdir=LR"));
        assert!(output.contains("Button1 [label=\"Start Button\"]"));
        assert!(output.contains("Button1 ->"));
    }

    #[test]
    fn test_extract_connections() {
        let devices = create_test_devices();
        let device_refs: Vec<_> = devices.iter().collect();
        let connections = DeviceGrapher::extract_connections(&device_refs);

        // Should find connection from Button to Spawner via game_start channel
        assert!(!connections.is_empty());

        let conn = connections.iter().find(|c| c.source == "Button1" && c.target == "Spawner1");
        assert!(conn.is_some());
        assert_eq!(conn.unwrap().channel, Some("game_start".to_string()));
    }

    #[test]
    fn test_escape_labels() {
        assert_eq!(escape_mermaid_label("Test \"Label\""), "Test \\\"Label\\\"");
        assert_eq!(escape_dot_label("Test \"Label\""), "Test \\\"Label\\\"");
    }
}

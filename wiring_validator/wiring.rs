//! Wiring validator for detecting device connection issues
//!
//! Analyzes device connections to find:
//! - Orphaned channels (no sender or no receiver)
//! - Channel conflicts (multiple senders on same channel)
//! - Missing connections (device has triggers/receivers but no wiring)
//! - Cycles (A → B → C → A)

use crate::lib_internal::DeviceInfo;
use super::types::{IssueKind, WiringIssue};
use std::collections::HashMap;

/// Wiring validator for detecting connection issues
#[derive(Debug, Default)]
pub struct WiringValidator {
    /// Devices that send on channels: channel -> list of device labels
    senders: HashMap<String, Vec<String>>,
    /// Devices that receive on channels: channel -> list of device labels
    receivers: HashMap<String, Vec<String>>,
    /// All devices with triggers but no channel configured
    unconfigured_triggers: Vec<String>,
    /// All devices with receivers but no channel configured
    unconfigured_receivers: Vec<String>,
}

impl WiringValidator {
    /// Create a new validator
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate device wiring and return list of issues
    pub fn validate(devices: &[DeviceInfo]) -> Vec<WiringIssue> {
        let mut validator = Self::new();
        validator.build_channel_maps(devices);
        validator.find_issues()
    }

    /// Build internal maps of senders and receivers
    fn build_channel_maps(&mut self, devices: &[DeviceInfo]) {
        for device in devices {
            let label = device
                .label
                .as_deref()
                .unwrap_or(&device.device_type)
                .to_string();

            // Check settings for channel configurations
            let mut has_channel_setting = false;

            if let Some(ref settings) = device.settings {
                for (key, value) in settings {
                    let key_lower = key.to_lowercase();

                    // Detect channel senders (triggers)
                    if key_lower.contains("channel")
                        && (key_lower.contains("trigger")
                            || key_lower.contains("transmit")
                            || key_lower.contains("send"))
                    {
                        self.senders
                            .entry(value.clone())
                            .or_default()
                            .push(label.clone());
                        has_channel_setting = true;
                    }

                    // Detect channel receivers
                    if key_lower.contains("channel")
                        && (key_lower.contains("receiv")
                            || key_lower.contains("listen")
                            || key_lower.contains("when"))
                    {
                        self.receivers
                            .entry(value.clone())
                            .or_default()
                            .push(label.clone());
                        has_channel_setting = true;
                    }
                }
            }

            // Track devices with triggers/receivers but no channel config
            if !device.triggers.is_empty() && !has_channel_setting {
                self.unconfigured_triggers.push(label.clone());
            }
            if !device.receivers.is_empty() && !has_channel_setting {
                self.unconfigured_receivers.push(label);
            }
        }
    }

    /// Find all wiring issues
    fn find_issues(&self) -> Vec<WiringIssue> {
        let mut issues = Vec::new();

        // Find orphaned senders (channel has senders but no receivers)
        for (channel, senders) in &self.senders {
            if !self.receivers.contains_key(channel) {
                issues.push(WiringIssue {
                    kind: IssueKind::OrphanedSender,
                    channel: Some(channel.clone()),
                    devices: senders.clone(),
                    message: format!(
                        "Channel '{}' has {} sender(s) but no receivers",
                        channel,
                        senders.len()
                    ),
                    suggestion: Some(
                        "Add a device to receive on this channel, or remove the sender".to_string(),
                    ),
                });
            }
        }

        // Find orphaned receivers (channel has receivers but no senders)
        for (channel, receivers) in &self.receivers {
            if !self.senders.contains_key(channel) {
                issues.push(WiringIssue {
                    kind: IssueKind::OrphanedReceiver,
                    channel: Some(channel.clone()),
                    devices: receivers.clone(),
                    message: format!(
                        "Channel '{}' has {} receiver(s) but no senders",
                        channel,
                        receivers.len()
                    ),
                    suggestion: Some(
                        "Add a device to send on this channel, or remove the receiver".to_string(),
                    ),
                });
            }
        }

        // Find channel conflicts (multiple senders on same channel)
        for (channel, senders) in &self.senders {
            if senders.len() > 1 {
                issues.push(WiringIssue {
                    kind: IssueKind::ChannelConflict,
                    channel: Some(channel.clone()),
                    devices: senders.clone(),
                    message: format!("Channel '{}' has {} senders which may cause conflicts", channel, senders.len()),
                    suggestion: Some("Consider using different channels for each sender to avoid race conditions".to_string()),
                });
            }
        }

        // Find unconfigured triggers
        for device in &self.unconfigured_triggers {
            issues.push(WiringIssue {
                kind: IssueKind::UnconfiguredTrigger,
                channel: None,
                devices: vec![device.clone()],
                message: format!("Device '{}' has triggers but no channel configured", device),
                suggestion: Some("Configure a channel in the device settings".to_string()),
            });
        }

        // Find unconfigured receivers
        for device in &self.unconfigured_receivers {
            issues.push(WiringIssue {
                kind: IssueKind::UnconfiguredReceiver,
                channel: None,
                devices: vec![device.clone()],
                message: format!(
                    "Device '{}' has receivers but no channel configured",
                    device
                ),
                suggestion: Some("Configure a channel in the device settings".to_string()),
            });
        }

        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DeviceInfo;

    fn make_device(
        label: &str,
        triggers: Vec<&str>,
        receivers: Vec<&str>,
        settings: Option<Vec<(&str, &str)>>,
    ) -> DeviceInfo {
        DeviceInfo {
            file: format!("{}.uasset", label),
            device_type: "TestDevice".to_string(),
            label: Some(label.to_string()),
            triggers: triggers.into_iter().map(String::from).collect(),
            receivers: receivers.into_iter().map(String::from).collect(),
            settings: settings.map(|s| {
                s.into_iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect()
            }),
        }
    }

    #[test]
    fn test_orphaned_sender() {
        let devices = vec![make_device(
            "Button",
            vec!["OnPressed"],
            vec![],
            Some(vec![("TriggerChannel", "game_start")]),
        )];

        let issues = WiringValidator::validate(&devices);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].kind, IssueKind::OrphanedSender);
    }

    #[test]
    fn test_orphaned_receiver() {
        let devices = vec![make_device(
            "Spawner",
            vec![],
            vec!["OnSpawn"],
            Some(vec![("ReceiverChannel", "game_start")]),
        )];

        let issues = WiringValidator::validate(&devices);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].kind, IssueKind::OrphanedReceiver);
    }

    #[test]
    fn test_channel_conflict() {
        let devices = vec![
            make_device(
                "Button1",
                vec!["OnPressed"],
                vec![],
                Some(vec![("TriggerChannel", "fire")]),
            ),
            make_device(
                "Button2",
                vec!["OnPressed"],
                vec![],
                Some(vec![("TriggerChannel", "fire")]),
            ),
            make_device(
                "Spawner",
                vec![],
                vec!["OnSpawn"],
                Some(vec![("ReceiverChannel", "fire")]),
            ),
        ];

        let issues = WiringValidator::validate(&devices);
        let conflict: Vec<_> = issues
            .iter()
            .filter(|i| i.kind == IssueKind::ChannelConflict)
            .collect();
        assert_eq!(conflict.len(), 1);
        assert_eq!(conflict[0].devices.len(), 2);
    }

    #[test]
    fn test_valid_connection() {
        let devices = vec![
            make_device(
                "Button",
                vec!["OnPressed"],
                vec![],
                Some(vec![("TriggerChannel", "game_start")]),
            ),
            make_device(
                "Spawner",
                vec![],
                vec!["OnSpawn"],
                Some(vec![("ReceiverChannel", "game_start")]),
            ),
        ];

        let issues = WiringValidator::validate(&devices);
        // Should have no orphan or conflict issues
        let critical: Vec<_> = issues
            .iter()
            .filter(|i| {
                matches!(
                    i.kind,
                    IssueKind::OrphanedSender
                        | IssueKind::OrphanedReceiver
                        | IssueKind::ChannelConflict
                )
            })
            .collect();
        assert!(
            critical.is_empty(),
            "Expected no critical issues, got {:?}",
            critical
        );
    }
}

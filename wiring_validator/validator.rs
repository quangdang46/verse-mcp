//! Verse code validator

use std::collections::HashMap;

use super::types::{ValidationIssue, Severity};
use super::patterns::{DEVICE_TYPE_RE, METHOD_CALL_RE, EVENT_SUBSCRIBE_RE};
use crate::digest::DigestIndex;

/// Verse code validator
pub struct VerseValidator {
    /// Digest index for validation
    pub digest: DigestIndex,
}

impl VerseValidator {
    /// Create a new validator with the given digest index
    pub fn new(digest: DigestIndex) -> Self {
        Self { digest }
    }

    /// Validate Verse code and return all issues found
    pub fn validate(&self, code: &str) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        // First pass: build variable-to-device-type mapping
        let var_types = self.extract_variable_types(code);

        // Second pass: validate device types
        for (line_num, line) in code.lines().enumerate() {
            let line_num = line_num + 1;

            // Check device type references
            for caps in DEVICE_TYPE_RE.captures_iter(line) {
                let device_type = caps[2].to_string();
                let column = caps.get(2).map(|m| m.start() + 1).unwrap_or(1);

                if !self.digest.devices.contains_key(&device_type) {
                    let suggestion = self.find_similar_device(&device_type);
                    issues.push(ValidationIssue {
                        severity: Severity::Error,
                        message: format!("Unknown device type: {}", device_type),
                        line: line_num,
                        column,
                        suggestion,
                        symbol_kind: "device_type".to_string(),
                    });
                }
            }
        }

        // Third pass: validate method calls and event subscriptions
        for (line_num, line) in code.lines().enumerate() {
            let line_num = line_num + 1;

            // Extract event subscriptions (check before method calls)
            for caps in EVENT_SUBSCRIBE_RE.captures_iter(line) {
                let var_name = caps[1].to_string();
                let event_name = caps[2].to_string();
                let column = caps.get(2).map(|m| m.start() + 1).unwrap_or(1);

                // Look up device type for this variable
                if let Some(device_type) = var_types.get(&var_name) {
                    if let Some(device_def) = self.digest.get_device(device_type) {
                        if !device_def.events.iter().any(|e| e.name == event_name) {
                            // Check if it's actually a method being subscribed
                            if device_def.methods.iter().any(|m| m.name == event_name) {
                                issues.push(ValidationIssue {
                                    severity: Severity::Warning,
                                    message: format!(
                                        "'{}' is a method, not an event. Cannot subscribe to methods.",
                                        event_name
                                    ),
                                    line: line_num,
                                    column,
                                    suggestion: Some(format!("{}.{}(...)", var_name, event_name)),
                                    symbol_kind: "method_as_event".to_string(),
                                });
                            } else {
                                let suggestion = self.find_similar_event(device_type, &event_name);
                                issues.push(ValidationIssue {
                                    severity: Severity::Error,
                                    message: format!("Unknown event: {}.{}", var_name, event_name),
                                    line: line_num,
                                    column,
                                    suggestion,
                                    symbol_kind: "event".to_string(),
                                });
                            }
                        }
                    }
                }
            }

            // Extract method calls (skip if already matched as event subscribe)
            for caps in METHOD_CALL_RE.captures_iter(line) {
                let var_name = caps[1].to_string();
                let method_name = caps[2].to_string();

                // Skip if this is an event subscribe (already captured)
                if line.contains(&format!("{}.{}.Subscribe", var_name, method_name)) {
                    continue;
                }

                let column = caps.get(2).map(|m| m.start() + 1).unwrap_or(1);

                // Look up device type for this variable
                if let Some(device_type) = var_types.get(&var_name) {
                    if let Some(device_def) = self.digest.get_device(device_type) {
                        if !device_def.methods.iter().any(|m| m.name == method_name) {
                            // Check if it's actually an event being called as method
                            if device_def.events.iter().any(|e| e.name == method_name) {
                                issues.push(ValidationIssue {
                                    severity: Severity::Warning,
                                    message: format!(
                                        "'{}' is an event, not a method. Did you mean to subscribe?",
                                        method_name
                                    ),
                                    line: line_num,
                                    column,
                                    suggestion: Some(format!("{}.{}.Subscribe(...)", var_name, method_name)),
                                    symbol_kind: "event_as_method".to_string(),
                                });
                            } else {
                                let suggestion = self.find_similar_method(device_type, &method_name);
                                issues.push(ValidationIssue {
                                    severity: Severity::Error,
                                    message: format!("Unknown method: {}.{}", var_name, method_name),
                                    line: line_num,
                                    column,
                                    suggestion,
                                    symbol_kind: "method".to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }

        issues
    }

    /// Extract variable-to-device-type mappings from code
    fn extract_variable_types(&self, code: &str) -> HashMap<String, String> {
        let mut var_types = HashMap::new();

        for line in code.lines() {
            // Match device type declarations: @editable varName:device_type_device
            for caps in DEVICE_TYPE_RE.captures_iter(line) {
                let var_name = caps[1].to_string();
                let device_type = caps[2].to_string();
                var_types.insert(var_name, device_type);
            }
        }

        var_types
    }

    /// Find similar device names using Levenshtein distance
    fn find_similar_device(&self, name: &str) -> Option<String> {
        let name_lower = name.to_lowercase();
        self.digest
            .devices
            .values()
            .map(|d| {
                let dist = super::utils::levenshtein(&d.name.to_lowercase(), &name_lower);
                (d.name.clone(), dist)
            })
            .filter(|(_, dist)| *dist <= 3)
            .min_by_key(|(_, dist)| *dist)
            .map(|(name, _)| name)
    }

    /// Find similar method names for a device
    fn find_similar_method(&self, device: &str, method: &str) -> Option<String> {
        let device_def = self.digest.get_device(device)?;
        let method_lower = method.to_lowercase();

        device_def
            .methods
            .iter()
            .map(|m| {
                let dist = super::utils::levenshtein(&m.name.to_lowercase(), &method_lower);
                (m.name.clone(), dist)
            })
            .filter(|(_, dist)| *dist <= 3)
            .min_by_key(|(_, dist)| *dist)
            .map(|(name, _)| format!("{}.{}(...)", device, name))
    }

    /// Find similar event names for a device
    fn find_similar_event(&self, device: &str, event: &str) -> Option<String> {
        let device_def = self.digest.get_device(device)?;
        let event_lower = event.to_lowercase();

        device_def
            .events
            .iter()
            .map(|e| {
                let dist = super::utils::levenshtein(&e.name.to_lowercase(), &event_lower);
                (e.name.clone(), dist)
            })
            .filter(|(_, dist)| *dist <= 3)
            .min_by_key(|(_, dist)| *dist)
            .map(|(name, _)| format!("{}.{}.Subscribe(...)", device, name))
    }
}

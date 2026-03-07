//! Verse code validator for hallucination detection
//!
//! Parses Verse source code and validates symbols against the digest index
//! to detect hallucinated API names (wrong method names, incorrect signatures,
//! non-existent events).

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::LazyLock;

use crate::digest::DigestIndex;

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

// Regex patterns for Verse code parsing
static DEVICE_TYPE_RE: LazyLock<Regex> = LazyLock::new(|| {
    // Match device type declarations: @editable varName:device_type_device
    Regex::new(r"(?:@editable\s+)?(\w+)\s*:\s*([a-z][a-z0-9_]*_device)")
        .expect("Invalid device type regex")
});

static METHOD_CALL_RE: LazyLock<Regex> = LazyLock::new(|| {
    // Match method calls: DeviceName.MethodName(
    Regex::new(r"(\w+)\.([A-Za-z][A-Za-z0-9_]*)\s*\(").expect("Invalid method call regex")
});

static EVENT_SUBSCRIBE_RE: LazyLock<Regex> = LazyLock::new(|| {
    // Match event subscriptions: DeviceName.EventName.Subscribe(
    Regex::new(r"(\w+)\.([A-Za-z][A-Za-z0-9_]*)\.Subscribe\s*\(")
        .expect("Invalid event subscribe regex")
});

/// Verse code validator
pub struct VerseValidator {
    /// Digest index for validation
    digest: DigestIndex,
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
                                let suggestion =
                                    self.find_similar_method(device_type, &method_name);
                                issues.push(ValidationIssue {
                                    severity: Severity::Error,
                                    message: format!(
                                        "Unknown method: {}.{}",
                                        var_name, method_name
                                    ),
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
                let dist = levenshtein(&d.name.to_lowercase(), &name_lower);
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
                let dist = levenshtein(&m.name.to_lowercase(), &method_lower);
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
                let dist = levenshtein(&e.name.to_lowercase(), &event_lower);
                (e.name.clone(), dist)
            })
            .filter(|(_, dist)| *dist <= 3)
            .min_by_key(|(_, dist)| *dist)
            .map(|(name, _)| format!("{}.{}.Subscribe(...)", device, name))
    }
}

/// Calculate Levenshtein distance between two strings
fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];

    for (i, row) in matrix.iter_mut().enumerate() {
        row[0] = i;
    }

    #[allow(clippy::needless_range_loop)]
    for j in 0..=b_len {
        matrix[0][j] = j;
    }

    for (i, a_char) in a_chars.iter().enumerate() {
        for (j, b_char) in b_chars.iter().enumerate() {
            let cost = if a_char == b_char { 0 } else { 1 };
            matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                .min(matrix[i + 1][j] + 1)
                .min(matrix[i][j] + cost);
        }
    }

    matrix[a_len][b_len]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::digest::DigestIndex;

    fn create_test_digest() -> DigestIndex {
        let digest = r#"
device_campfire_device = class():
    TriggerOnEnterRadius():event():void
    OnDisabled():event():void
    ReceiverAddFuel(Amount:int):void
    AddFuel(Amount:int):void
    Extinguish():void
    Light():void

device_button_device = class():
    InteractWithAgent():event(such as {Agent}):void
    OnPressed():event():void
    Disable():void
    Enable():void
"#;
        DigestIndex::parse(digest).unwrap()
    }

    #[test]
    fn test_validate_valid_code() {
        let digest = create_test_digest();
        let validator = VerseValidator::new(digest);

        let code = r#"
using { /Verse.org/Symbols }
using { /Fortnite.com/Devices }

campfire_device := class(device_campfire_device):
    @editable
    var MyCampfire:device_campfire_device = device_campfire_device{}

OnBegin<override>()<suspends>:void=
    MyCampfire.Light()
    MyCampfire.TriggerOnEnterRadius.Subscribe(HandleTrigger)

HandleTrigger(Agent:?agent):void=
    MyCampfire.AddFuel(5)
"#;

        let issues = validator.validate(code);
        // Should have no errors (all symbols are valid)
        assert!(issues.is_empty(), "Expected no issues, got: {:?}", issues);
    }

    #[test]
    fn test_detect_unknown_device_type() {
        let digest = create_test_digest();
        let validator = VerseValidator::new(digest);

        let code = r#"
@editable var MyDevice:device_nonexistent_device = device_nonexistent_device{}
"#;

        let issues = validator.validate(code);
        assert!(!issues.is_empty());
        assert!(issues[0].message.contains("Unknown device type"));
    }

    #[test]
    fn test_detect_unknown_method() {
        let digest = create_test_digest();
        let validator = VerseValidator::new(digest);

        let code = r#"
@editable var MyCampfire:device_campfire_device = device_campfire_device{}
MyCampfire.NonExistentMethod()
"#;

        let issues = validator.validate(code);
        assert!(!issues.is_empty(), "Expected issues, got: {:?}", issues);
        assert!(issues[0].message.contains("Unknown method"));
    }

    #[test]
    fn test_detect_unknown_event() {
        let digest = create_test_digest();
        let validator = VerseValidator::new(digest);

        let code = r#"
@editable var MyCampfire:device_campfire_device = device_campfire_device{}
MyCampfire.NonExistentEvent.Subscribe(HandleEvent)
"#;

        let issues = validator.validate(code);
        assert!(!issues.is_empty(), "Expected issues, got: {:?}", issues);
        assert!(issues[0].message.contains("Unknown event"));
    }

    #[test]
    fn test_suggest_similar_method() {
        let digest = create_test_digest();
        let validator = VerseValidator::new(digest);

        let code = r#"
@editable var MyCampfire:device_campfire_device = device_campfire_device{}
MyCampfire.Extenguish()
"#;

        let issues = validator.validate(code);
        assert!(!issues.is_empty(), "Expected issues, got: {:?}", issues);
        assert!(issues[0].suggestion.is_some());
        assert!(issues[0]
            .suggestion
            .as_ref()
            .unwrap()
            .contains("Extinguish"));
    }

    #[test]
    fn test_event_called_as_method() {
        let digest = create_test_digest();
        let validator = VerseValidator::new(digest);

        let code = r#"
@editable var MyCampfire:device_campfire_device = device_campfire_device{}
MyCampfire.TriggerOnEnterRadius()
"#;

        let issues = validator.validate(code);
        assert!(!issues.is_empty(), "Expected issues, got: {:?}", issues);
        assert!(issues[0].message.contains("is an event, not a method"));
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein("kitten", "sitting"), 3);
        assert_eq!(levenshtein("", "test"), 4);
        assert_eq!(levenshtein("test", ""), 4);
        assert_eq!(levenshtein("same", "same"), 0);
    }
}

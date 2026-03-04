//! Device classification from Name Map
//!
//! This module extracts device type, triggers, and receivers from the Name Map
//! by detecting _GEN_VARIABLE twins which signal Verse-accessible events.

use regex::Regex;
use std::collections::HashSet;
use std::sync::OnceLock;

/// Regex pattern for device type names (e.g., Device_Campfire_C)
static DEVICE_TYPE_RE: OnceLock<Regex> = OnceLock::new();

fn device_type_regex() -> &'static Regex {
    DEVICE_TYPE_RE.get_or_init(|| Regex::new(r"^Device_[A-Za-z0-9]+_C$").unwrap())
}

/// Trigger event name patterns (Stage A heuristic filter)
const TRIGGER_PREFIXES: &[&str] = &["On", "Trigger", "When"];

/// Receiver event name patterns (Stage A heuristic filter)
const RECEIVER_PREFIXES: &[&str] = &["Receiver"];
const RECEIVER_SUFFIXES: &[&str] = &["WhenReceiving"];

/// Component noise patterns to filter out (even if they have _GEN_VARIABLE twins)
/// Note: These only apply to names that DON'T match event patterns (On*, Trigger*, Receiver*, etc.)
const NOISE_COMPONENTS: &[&str] = &[
    "Component",
    "Mesh",
    "Collision",
    "Sphere",
    "Box",
    "Actor",
    "Visual",
    "Audio",
    "Particle",
    "Camera",
    "Widget",
    "Canvas",
];

/// Classification result from Name Map analysis
#[derive(Debug, Clone, Default)]
pub struct Classification {
    /// Device type (e.g., "Device_Campfire_C") if found
    pub device_type: Option<String>,
    /// Trigger event names (e.g., "TriggerOnEnterRadius", "OnDisabled")
    pub triggers: Vec<String>,
    /// Receiver event names (e.g., "ReceiverLight", "EnableWhenReceiving")
    pub receivers: Vec<String>,
}

impl Classification {
    /// Create a new empty classification
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any Verse events were found
    pub fn has_events(&self) -> bool {
        !self.triggers.is_empty() || !self.receivers.is_empty()
    }

    /// Get total event count
    pub fn event_count(&self) -> usize {
        self.triggers.len() + self.receivers.len()
    }
}

/// Classify a list of Name Map entries into device info
///
/// # Arguments
/// * `names` - List of Name Map entries from .uasset file
///
/// # Returns
/// Classification with device_type, triggers, and receivers
pub fn classify(names: &[String]) -> Classification {
    let gen_suffix = "_GEN_VARIABLE";
    let gen_suffix_len = gen_suffix.len();

    // Build set of names that have _GEN_VARIABLE twins
    // These are the Verse-accessible events
    let gen_set: HashSet<&str> = names
        .iter()
        .filter(|n| n.ends_with(gen_suffix))
        .filter_map(|n| n.get(..n.len() - gen_suffix_len))
        .collect();

    let mut device_type = None;
    let mut triggers = Vec::new();
    let mut receivers = Vec::new();

    for name in names {
        // Skip the _GEN_VARIABLE twins themselves
        if name.ends_with(gen_suffix) {
            continue;
        }

        // Look for device type first (only take first match)
        if device_type.is_none() && device_type_regex().is_match(name) {
            device_type = Some(name.clone());
            continue;
        }

        // Only classify names that have _GEN_VARIABLE twins (Verse events)
        if !gen_set.contains(name.as_str()) {
            continue;
        }

        // Filter out component noise (Stage A heuristic)
        if is_noise_component(name) {
            continue;
        }

        // Apply pattern validation (Stage A heuristic filter)
        if is_receiver(name) {
            receivers.push(name.clone());
        } else if is_trigger(name) {
            triggers.push(name.clone());
        }
        // Skip names that don't match trigger/receiver patterns
    }

    // Sort for consistent output
    triggers.sort();
    triggers.dedup();
    receivers.sort();
    receivers.dedup();

    Classification {
        device_type,
        triggers,
        receivers,
    }
}

/// Check if a name matches receiver patterns
fn is_receiver(name: &str) -> bool {
    RECEIVER_PREFIXES.iter().any(|prefix| name.starts_with(prefix))
        || RECEIVER_SUFFIXES.iter().any(|suffix| name.ends_with(suffix))
}

/// Check if a name matches trigger patterns
fn is_trigger(name: &str) -> bool {
    TRIGGER_PREFIXES.iter().any(|prefix| name.starts_with(prefix))
}

/// Check if a name is a component that should be filtered out
fn is_noise_component(name: &str) -> bool {
    NOISE_COMPONENTS.iter().any(|noise| name.contains(noise))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_names() -> Vec<String> {
        vec![
            // Device type
            "Device_Campfire_C".to_string(),
            "Device_Campfire_C_GEN_VARIABLE".to_string(),
            // Triggers
            "TriggerOnEnterRadius".to_string(),
            "TriggerOnEnterRadius_GEN_VARIABLE".to_string(),
            "OnDisabled".to_string(),
            "OnDisabled_GEN_VARIABLE".to_string(),
            "OnEnabled".to_string(),
            "OnEnabled_GEN_VARIABLE".to_string(),
            // Receivers
            "ReceiverLight".to_string(),
            "ReceiverLight_GEN_VARIABLE".to_string(),
            "EnableWhenReceiving".to_string(),
            "EnableWhenReceiving_GEN_VARIABLE".to_string(),
            "DisableWhenReceiving".to_string(),
            "DisableWhenReceiving_GEN_VARIABLE".to_string(),
            // Noise (has _GEN_VARIABLE but is component)
            "ToyOptionsComponent".to_string(),
            "ToyOptionsComponent_GEN_VARIABLE".to_string(),
            "ButtonMesh".to_string(),
            "ButtonMesh_GEN_VARIABLE".to_string(),
            // Script paths
            "/Script/FortniteGame".to_string(),
            "/Game/Devices".to_string(),
        ]
    }

    #[test]
    fn test_classify_device_type() {
        let names = make_names();
        let result = classify(&names);
        assert_eq!(result.device_type, Some("Device_Campfire_C".to_string()));
    }

    #[test]
    fn test_classify_triggers() {
        let names = make_names();
        let result = classify(&names);

        assert!(result.triggers.contains(&"TriggerOnEnterRadius".to_string()));
        assert!(result.triggers.contains(&"OnDisabled".to_string()));
        assert!(result.triggers.contains(&"OnEnabled".to_string()));
        // Noise should be filtered
        assert!(!result.triggers.contains(&"ToyOptionsComponent".to_string()));
        assert!(!result.triggers.contains(&"ButtonMesh".to_string()));
    }

    #[test]
    fn test_classify_receivers() {
        let names = make_names();
        let result = classify(&names);

        assert!(result.receivers.contains(&"ReceiverLight".to_string()));
        assert!(result.receivers.contains(&"EnableWhenReceiving".to_string()));
        assert!(result.receivers.contains(&"DisableWhenReceiving".to_string()));
    }

    #[test]
    fn test_classify_filters_gen_variable() {
        let names = make_names();
        let result = classify(&names);

        // _GEN_VARIABLE twins should never appear in results
        for name in &result.triggers {
            assert!(!name.ends_with("_GEN_VARIABLE"));
        }
        for name in &result.receivers {
            assert!(!name.ends_with("_GEN_VARIABLE"));
        }
    }

    #[test]
    fn test_classify_filters_noise() {
        let names = make_names();
        let result = classify(&names);

        // Component noise should be filtered
        assert!(!result.triggers.iter().any(|n| n.contains("Component")));
        assert!(!result.triggers.iter().any(|n| n.contains("Mesh")));
    }

    #[test]
    fn test_classify_empty() {
        let names: Vec<String> = vec![];
        let result = classify(&names);
        assert!(result.device_type.is_none());
        assert!(result.triggers.is_empty());
        assert!(result.receivers.is_empty());
    }

    #[test]
    fn test_is_receiver() {
        assert!(is_receiver("ReceiverLight"));
        assert!(is_receiver("EnableWhenReceiving"));
        assert!(!is_receiver("TriggerOnEnter"));
    }

    #[test]
    fn test_is_trigger() {
        assert!(is_trigger("OnDisabled"));
        assert!(is_trigger("TriggerOnEnterRadius"));
        assert!(is_trigger("WhenInteracted"));
        assert!(!is_trigger("ReceiverLight"));
    }

    #[test]
    fn test_is_noise_component() {
        assert!(is_noise_component("ToyOptionsComponent"));
        assert!(is_noise_component("ButtonMesh"));
        assert!(is_noise_component("SphereCollision"));
        assert!(!is_noise_component("TriggerOnEnter"));
    }
}

//! Device fingerprinting for unknown device classification
//!
//! Some UEFN devices don't have a Device_*_C class name in their Name Map.
//! We identify these by matching their settings keys against fingerprints.

/// Fingerprint for unknown device classification
#[derive(Debug, Clone)]
pub struct Fingerprint {
    /// Device type name to assign if fingerprint matches
    pub device_type: &'static str,
    /// Required settings keys that indicate this device type
    pub required_keys: &'static [&'static str],
    /// Minimum number of keys that must match
    pub min_matches: usize,
}

impl Fingerprint {
    /// Create a new fingerprint
    pub const fn new(
        device_type: &'static str,
        required_keys: &'static [&'static str],
        min_matches: usize,
    ) -> Self {
        Self {
            device_type,
            required_keys,
            min_matches,
        }
    }

    /// Check if the given settings keys match this fingerprint
    pub fn matches(&self, settings_keys: &[&str]) -> bool {
        let matches = self
            .required_keys
            .iter()
            .filter(|key| settings_keys.contains(key))
            .count();
        matches >= self.min_matches
    }
}

/// Built-in fingerprints for common UEFN devices
pub const FINGERPRINTS: &[Fingerprint] = &[
    // Button device - identified by interaction settings
    Fingerprint::new(
        "button_device",
        &["InteractionRadius", "InteractTime", "TriggerSound"],
        2,
    ),
    // Player spawner - identified by spawn settings
    Fingerprint::new(
        "player_spawner_device",
        &["UseAsIslandStart", "PlayerTeam"],
        2,
    ),
    // Character spawner - identified by NPC settings
    Fingerprint::new(
        "character_spawner_device",
        &["NPCCharacterDefinition", "DespawnAIsWhenDisabled"],
        2,
    ),
    // Tracker device - identified by tracking settings
    Fingerprint::new(
        "tracker_device",
        &["TrackingType", "CountToWin"],
        2,
    ),
    // Island settings - identified by game settings
    Fingerprint::new(
        "island_settings_device",
        &["MaxHealth", "SpawnLocation", "Teams"],
        2,
    ),
];

/// Classify an unknown device by matching settings against fingerprints
pub fn classify_by_fingerprint(settings_keys: &[&str]) -> Option<&'static str> {
    FINGERPRINTS
        .iter()
        .find(|fp| fp.matches(settings_keys))
        .map(|fp| fp.device_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_fingerprint() {
        let keys = vec!["InteractionRadius", "InteractTime", "TriggerSound"];
        assert_eq!(classify_by_fingerprint(&keys), Some("button_device"));
    }

    #[test]
    fn test_player_spawner_fingerprint() {
        let keys = vec!["UseAsIslandStart", "PlayerTeam", "OtherSetting"];
        assert_eq!(classify_by_fingerprint(&keys), Some("player_spawner_device"));
    }

    #[test]
    fn test_no_match() {
        let keys = vec!["UnknownSetting"];
        assert_eq!(classify_by_fingerprint(&keys), None);
    }
}

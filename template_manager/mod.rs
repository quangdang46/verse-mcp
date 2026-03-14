//! Composition templates for device wiring patterns
//!
//! Save and load common device wiring patterns as JSON templates.

use crate::lib_internal::ScanOutput;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

/// Template for a device wiring pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    /// Template name (used as filename)
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// Devices in this template
    pub devices: Vec<TemplateDevice>,
    /// Wiring connections
    pub wiring: Vec<TemplateWire>,
    /// Default settings per device label
    #[serde(default)]
    pub settings: IndexMap<String, IndexMap<String, String>>,
}

/// Device definition in a template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateDevice {
    /// Device type (e.g., "button_device")
    #[serde(rename = "type")]
    pub device_type: String,
    /// Label for this device instance
    pub label: String,
}

/// Wiring connection in a template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateWire {
    /// Source device label
    pub from: String,
    /// Event name
    pub event: String,
    /// Target device label
    pub to: String,
    /// Method to call (optional)
    pub method: Option<String>,
}

/// Template manager for saving/loading templates
pub struct TemplateManager {
    /// Directory containing templates
    templates_dir: PathBuf,
}

/// Error type for template operations
#[derive(Debug, Error)]
pub enum TemplateError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Template not found: {0}")]
    NotFound(String),

    #[error("Template already exists: {0}")]
    AlreadyExists(String),
}

type Result<T> = std::result::Result<T, TemplateError>;

impl TemplateManager {
    /// Create a new template manager
    pub fn new(templates_dir: PathBuf) -> Self {
        Self { templates_dir }
    }

    /// List all available templates
    pub fn list(&self) -> Result<Vec<String>> {
        if !self.templates_dir.exists() {
            return Ok(Vec::new());
        }

        let mut templates = Vec::new();
        let entries_iter = std::fs::read_dir(&self.templates_dir)?;
        for entry_result in entries_iter {
            match entry_result {
                Ok(entry) => {
                    let path = entry.path();
                    if path.extension().is_some_and(|ext| ext == "json") {
                        if let Some(stem) = path.file_stem() {
                            if let Some(stem_str) = stem.to_str() {
                                // Skip if it's a non-template JSON file
                                if stem_str.ends_with("_template") || stem_str == "template" {
                                    continue;
                                }
                                templates.push(stem_str.to_string());
                            }
                        }
                    }
                }
                Err(_) => continue,
            }
        }
        templates.sort();
        Ok(templates)
    }

    /// Load a template by name
    pub fn load(&self, name: &str) -> Result<Template> {
        let path = self.templates_dir.join(format!("{}.json", name));
        if !path.exists() {
            return Err(TemplateError::NotFound(name.to_string()));
        }

        let content = std::fs::read_to_string(&path)?;
        let template: Template = serde_json::from_str(&content)?;
        Ok(template)
    }

    /// Save a template
    pub fn save(&self, template: &Template) -> Result<()> {
        // Ensure directory exists
        std::fs::create_dir_all(&self.templates_dir)?;

        let path = self.templates_dir.join(format!("{}.json", template.name));
        let content = serde_json::to_string_pretty(template)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Delete a template
    pub fn delete(&self, name: &str) -> Result<()> {
        let path = self.templates_dir.join(format!("{}.json", name));
        if !path.exists() {
            return Err(TemplateError::NotFound(name.to_string()));
        }
        std::fs::remove_file(path)?;
        Ok(())
    }

    /// Create a template from current scan output
    pub fn from_scan_output(
        name: String,
        description: String,
        output: &ScanOutput,
    ) -> Template {
        let mut devices = Vec::new();
        let mut wiring = Vec::new();
        let mut settings = IndexMap::new();

        // Collect all devices
        for (device_type, device_list) in &output.by_type {
            for (idx, device) in device_list.iter().enumerate() {
                let label = device
                    .label
                    .clone()
                    .unwrap_or_else(|| format!("{}_{}", device_type, idx));

                devices.push(TemplateDevice {
                    device_type: device_type.clone(),
                    label: label.clone(),
                });

                // Copy settings
                if let Some(ref device_settings) = device.settings {
                    settings.insert(label.clone(), device_settings.clone());
                }

                // Extract wiring from triggers
                for trigger in &device.triggers {
                    // Check settings for channel
                    if let Some(ref device_settings) = device.settings {
                        for (key, value) in device_settings {
                            if key.to_lowercase().contains("trigger")
                                && key.to_lowercase().contains("channel")
                            {
                                // This is a sender - record of trigger
                                wiring.push(TemplateWire {
                                    from: label.clone(),
                                    event: trigger.clone(),
                                    to: format!("channel:{}", value),
                                    method: None,
                                });
                            }
                        }
                    }
                }
            }
        }

        Template {
            name,
            description,
            devices,
            wiring,
            settings,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_test_template() -> Template {
        let mut settings = IndexMap::new();
        let mut score_settings = IndexMap::new();
        score_settings.insert("InitialScore".to_string(), "0".to_string());
        score_settings.insert("MaxScore".to_string(), "100".to_string());
        settings.insert("ScoreManager".to_string(), score_settings);

        Template {
            name: "basic_scoring".to_string(),
            description: "Basic scoring system with HUD".to_string(),
            devices: vec![
                TemplateDevice {
                    device_type: "score_manager_device".to_string(),
                    label: "ScoreManager".to_string(),
                },
                TemplateDevice {
                    device_type: "hud_device".to_string(),
                    label: "ScoreHUD".to_string(),
                },
            ],
            wiring: vec![TemplateWire {
                from: "ScoreManager".to_string(),
                event: "OnScoreChange".to_string(),
                to: "ScoreHUD".to_string(),
                method: Some("UpdateScore".to_string()),
            }],
            settings,
        }
    }

    #[test]
    fn test_save_and_load_template() {
        let dir = tempdir().unwrap();
        let manager = TemplateManager::new(dir.path().to_path_buf());

        let template = create_test_template();
        manager.save(&template).unwrap();

        let loaded = manager.load("basic_scoring").unwrap();
        assert_eq!(loaded.name, "basic_scoring");
        assert_eq!(loaded.devices.len(), 2);
        assert_eq!(loaded.wiring.len(), 1);
    }

    #[test]
    fn test_list_templates() {
        let dir = tempdir().unwrap();
        let manager = TemplateManager::new(dir.path().to_path_buf());

        assert!(manager.list().unwrap().is_empty());

        let template = create_test_template();
        manager.save(&template).unwrap();

        let list = manager.list().unwrap();
        assert_eq!(list.len(), 1);
        assert!(list.contains(&"basic_scoring".to_string()));
    }

    #[test]
    fn test_delete_template() {
        let dir = tempdir().unwrap();
        let manager = TemplateManager::new(dir.path().to_path_buf());

        let template = create_test_template();
        manager.save(&template).unwrap();
        assert!(manager.list().unwrap().len() == 1);

        manager.delete("basic_scoring").unwrap();
        assert!(manager.list().unwrap().is_empty());
    }

    #[test]
    fn test_template_not_found() {
        let dir = tempdir().unwrap();
        let manager = TemplateManager::new(dir.path().to_path_buf());

        let result = manager.load("nonexistent");
        assert!(matches!(result, Err(TemplateError::NotFound(_))));
    }

    #[test]
    fn test_template_serialization() {
        let template = create_test_template();
        let json = serde_json::to_string(&template).unwrap();
        assert!(json.contains("basic_scoring"));
        assert!(json.contains("score_manager_device"));

        let parsed: Template = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.name, template.name);
    }
}

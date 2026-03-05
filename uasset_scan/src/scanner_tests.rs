//! Scanner validation tests

#[cfg(test)]
mod tests {
    use crate::{DeviceInfo, ScanOutput};
    use indexmap::IndexMap;

    #[test]
    fn test_scan_output_schema() {
        let output = ScanOutput::empty("/test/path".to_string());
        let json = serde_json::to_string(&output).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(parsed.is_object());
        assert!(parsed.get("scanned_at").is_some());
        assert!(parsed.get("project_root").is_some());
        assert!(parsed.get("total_files").is_some());
        assert!(parsed.get("total_devices").is_some());
        assert!(parsed.get("skipped").is_some());
        assert!(parsed.get("device_types").is_some());
        assert!(parsed.get("by_type").is_some());
    }

    #[test]
    fn test_device_info_schema() {
        let device = DeviceInfo::new("test.uasset".to_string(), "Device_Test_C".to_string());
        let json = serde_json::to_string(&device).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["file"], "test.uasset");
        assert_eq!(parsed["device_type"], "Device_Test_C");
        assert!(parsed["label"].is_null());
        assert!(parsed["triggers"].is_array());
        assert!(parsed["receivers"].is_array());
        assert!(parsed["settings"].is_null());
    }

    #[test]
    fn test_device_info_with_data() {
        let mut device =
            DeviceInfo::new("test.uasset".to_string(), "Device_Campfire_C".to_string());
        device.label = Some("Test Campfire".to_string());
        device.triggers = vec!["OnEnabled".to_string(), "OnDisabled".to_string()];
        device.receivers = vec!["ReceiverLight".to_string()];
        let mut settings = IndexMap::new();
        settings.insert("StartLit".to_string(), "False".to_string());
        device.settings = Some(settings);

        let json = serde_json::to_string(&device).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["label"], "Test Campfire");
        assert_eq!(parsed["triggers"].as_array().unwrap().len(), 2);
        assert_eq!(parsed["receivers"].as_array().unwrap().len(), 1);
        assert!(parsed["settings"]["StartLit"].is_string());
    }
}

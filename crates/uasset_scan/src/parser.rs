//! UE5 binary parser for .uasset files
//!
//! Based on reverse-engineered UE5 format from scan-verse.js POC.

use crate::{DeviceInfo, ScanError, UE_MAGIC};
use std::io::Read;

/// FString from UE binary format
#[derive(Debug, Clone)]
pub struct FString {
    pub value: String,
}

impl FString {
    /// Read FString from buffer at offset
    /// Returns (string, next_offset)
    pub fn read(buf: &[u8], offset: usize) -> Option<(Self, usize)> {
        if offset + 4 > buf.len() {
            return None;
        }

        let len = i32::from_le_bytes([buf[offset], buf[offset + 1], buf[offset + 2], buf[offset + 3]]);

        if len == 0 {
            return Some((FString { value: String::new() }, offset + 4));
        }

        if len > 0 {
            // ASCII string
            let end = offset + 4 + len as usize;
            if end > buf.len() || buf[end - 1] != 0 {
                return None;
            }
            // Validate ASCII
            for i in (offset + 4)..(end - 1) {
                if buf[i] < 0x20 || buf[i] > 0x7e {
                    return None;
                }
            }
            let s = std::str::from_utf8(&buf[offset + 4..end - 1]).ok()?.to_string();
            Some((FString { value: s }, end))
        } else {
            // UTF-16 string (negative length)
            let char_count = (-len) as usize;
            let end = offset + 4 + char_count * 2;
            if end > buf.len() {
                return None;
            }
            let s = String::from_utf16_lossy(
                &buf[offset + 4..end]
                    .chunks_exact(2)
                    .map(|c| u16::from_le_bytes([c[0], c[1]]))
                    .collect::<Vec<_>>(),
            );
            Some((FString { value: s }, end))
        }
    }
}

/// Name Map entries from UE asset
pub struct NameMap {
    pub names: Vec<String>,
    pub end_offset: usize,
}

impl NameMap {
    /// Read Name Map by probing for /Script/ or /Game/ prefix
    pub fn read(buf: &[u8]) -> Option<Self> {
        // Probe bytes 100-2000 for Name Map start
        for probe in 100..std::cmp::min(buf.len(), 2000) {
            let (s, _) = FString::read(buf, probe)?;
            if !s.value.starts_with("/Script/") && !s.value.starts_with("/Game/") {
                continue;
            }

            // Verify run of valid strings
            let mut offset = probe;
            let mut run = 0;
            let mut names = Vec::new();

            while offset < buf.len().saturating_sub(8) && run < 10 {
                let (fs, next) = FString::read(buf, offset)?;
                if fs.value.is_empty() {
                    break;
                }
                run += 1;
                offset = next + 4; // Skip hash
            }

            if run < 5 {
                continue;
            }

            // Read all names
            offset = probe;
            while offset < buf.len().saturating_sub(8) {
                let (fs, next) = FString::read(buf, offset)?;
                if fs.value.is_empty() {
                    break;
                }
                names.push(fs.value);
                offset = next + 4;
            }

            return Some(NameMap {
                names,
                end_offset: offset,
            });
        }
        None
    }
}

/// Extract ActorLabel from binary
pub fn extract_label(buf: &[u8]) -> Option<String> {
    // Search for "ActorLabel" pattern
    let needle = b"\x0b\x00\x00\x00ActorLabel\x00";
    for i in 0..buf.len().saturating_sub(needle.len() + 8) {
        if buf[i..i + needle.len()] == *needle {
            let (fs, _) = FString::read(buf, i + needle.len())?;
            // Validate label format
            if fs.value.len() <= 40 && fs.value.chars().all(|c| c.is_alphanumeric() || c == ' ' || c == '_' || c == '-') {
                return Some(fs.value);
            }
        }
    }
    None
}

/// Extract property settings from PropertyOverrideData
pub fn extract_settings(buf: &[u8], scan_start: usize) -> indexmap::IndexMap<String, String> {
    let mut result = indexmap::IndexMap::new();

    for i in scan_start..buf.len().saturating_sub(60) {
        let (key, key_end) = match FString::read(buf, i) {
            Some((fs, end)) if !fs.value.is_empty() => (fs.value, end),
            _ => continue,
        };

        // Key must be PascalCase, letters only, 4-50 chars
        if !key.chars().all(|c| c.is_ascii_alphabetic()) || key.len() < 4 || key.len() > 50 {
            continue;
        }
        if !key.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
            continue;
        }

        // Value is at 25 bytes after key end
        let val_offset = key_end + 25;
        let (value, _) = match FString::read(buf, val_offset) {
            Some((fs, end)) => (fs.value, end),
            None => continue,
        };

        // Validate value format
        let valid = value.parse::<f64>().is_ok()
            || value == "True" || value == "False"
            || matches!(value.as_str(), "Always" | "Never" | "No Effect" | "Yes" | "No")
            || value.starts_with('(')
            || (value.len() > 1 && value.len() <= 30 && value.chars().all(|c| c.is_alphanumeric() || c == ' ' || c == '_'));

        if valid && !result.contains_key(&key) {
            result.insert(key, value);
        }
    }

    result
}

/// Parse a .uasset file and extract device info
pub fn parse_uasset(buf: &[u8], file_path: &str) -> Result<Option<DeviceInfo>, ScanError> {
    if buf.len() < 64 {
        return Ok(None);
    }

    // Check magic number
    let magic = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
    if magic != UE_MAGIC {
        return Ok(None);
    }

    // Read Name Map
    let name_map = match NameMap::read(buf) {
        Some(nm) => nm,
        None => return Ok(None),
    };

    if name_map.names.len() < 5 {
        return Ok(None);
    }

    // Classify using _GEN_VARIABLE signal
    let gen_vars: std::collections::HashSet<&str> = name_map
        .names
        .iter()
        .filter(|n| n.ends_with("_GEN_VARIABLE"))
        .map(|n| &n[..n.len() - 13])
        .collect();

    let mut device_type = None;
    let mut triggers = Vec::new();
    let mut receivers = Vec::new();

    for name in &name_map.names {
        if name.ends_with("_GEN_VARIABLE") {
            continue;
        }

        // Find device type
        if device_type.is_none() && name.starts_with("Device_") && name.ends_with("_C") {
            device_type = Some(name.clone());
            continue;
        }

        // Classify triggers/receivers
        if gen_vars.contains(name.as_str()) {
            if name.starts_with("Receiver") || name.ends_with("WhenReceiving") {
                receivers.push(name.clone());
            } else {
                triggers.push(name.clone());
            }
        }
    }

    // Extract settings BEFORE determining final device type
    let scan_start = std::cmp::max(name_map.end_offset, buf.len() / 2);
    let settings = extract_settings(buf, scan_start);

    // Determine device type: try fingerprinting if no Device_*_C found
    let device_type = match device_type {
        Some(dt) => dt,
        None => {
            // Try fingerprint classification using settings
            crate::fingerprint::fingerprint_device(&settings)
                .unwrap_or_else(|| "Unknown".to_string())
        }
    };

    // Skip if no useful data
    if triggers.is_empty() && receivers.is_empty() && settings.is_empty() {
        return Ok(None);
    }

    // Extract label
    let label = extract_label(buf);

    Ok(Some(DeviceInfo {
        file: file_path.to_string(),
        device_type,
        label,
        triggers,
        receivers,
        settings: if settings.is_empty() { None } else { Some(settings) },
    }))
}

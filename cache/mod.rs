//! Scan cache implementation with mtime-based invalidation

use std::time::SystemTime;

use crate::lib_internal::ScanOutput;

/// Cache entry for scan results
#[derive(Debug, Clone)]
pub struct ScanCache {
    /// Cached scan output
    pub output: ScanOutput,
    /// Maximum mtime of scanned files when cache was created
    pub max_mtime: SystemTime,
    /// When was cache created (for future time-based expiration)
    #[allow(dead_code)]
    pub cached_at: SystemTime,
}

/// Get maximum modification time of scanned directories
///
/// Scans both __ExternalActors__ and __ExternalObjects__ for cache invalidation.
pub fn get_max_mtime(project_path: &std::path::Path) -> SystemTime {
    let content_root = project_path.join("Content");
    let scan_dirs = vec![
        content_root.join("__ExternalActors__"),
        content_root.join("__ExternalObjects__"),
    ];
    let mut max_mtime = SystemTime::UNIX_EPOCH;

    for scan_dir in scan_dirs {
        if let Ok(entries) = std::fs::read_dir(&scan_dir) {
            for entry in entries.flatten() {
                if entry.path().extension().is_some_and(|ext| ext == "uasset") {
                    if let Ok(metadata) = entry.metadata() {
                        if let Ok(mtime) = metadata.modified() {
                            if mtime > max_mtime {
                                max_mtime = mtime;
                            }
                        }
                    }
                }
            }
        }
    }

    max_mtime
}

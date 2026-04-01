use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::SystemTime;

#[derive(Debug, Clone)]
struct ScanCacheEntry {
    output: uasset_scan::ScanOutput,
    max_mtime: SystemTime,
    cached_at: SystemTime,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanProjectRequest {
    pub project_path: PathBuf,
    pub force_refresh: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScanCacheState {
    Miss,
    Hit,
    ForcedRefresh,
    ModifiedRefresh,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanExecutionMeta {
    pub cache_state: ScanCacheState,
    pub reason: String,
    pub cached_at_epoch_ms: u128,
    pub max_mtime_epoch_ms: Option<u128>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanProjectResponse {
    #[serde(flatten)]
    pub output: uasset_scan::ScanOutput,
    pub execution: ScanExecutionMeta,
}

#[derive(Debug, Default)]
struct EngineState {
    scan_cache: Mutex<HashMap<PathBuf, ScanCacheEntry>>,
}

#[derive(Debug, Clone, Default)]
pub struct GroundingEngine {
    state: Arc<EngineState>,
}

impl GroundingEngine {
    pub fn query_docs(
        &self,
        query: &str,
        options: crate::DocsQueryOptions,
    ) -> Result<crate::DocsQueryResponse> {
        crate::docs::query_docs(query, options)
    }

    pub fn fetch_doc_source(&self, url: &str) -> Result<crate::FetchedSource> {
        crate::docs::fetch_doc_source(url)
    }

    pub fn scan_project(&self, request: &ScanProjectRequest) -> Result<ScanProjectResponse> {
        let cache_key = normalize_cache_key(&request.project_path);
        let current_mtime = max_project_mtime(&request.project_path);
        let cached_max_mtime = {
            let cache = self.lock_cache();
            cache.get(&cache_key).map(|entry| entry.max_mtime)
        };

        if let Some(response) =
            self.try_cached_response(&cache_key, request.force_refresh, current_mtime)
        {
            return Ok(response);
        }

        let output = uasset_scan::scan_project(&request.project_path)?;
        let max_mtime = max_project_mtime(&request.project_path);
        let cached_at = SystemTime::now();
        let decision = classify_scan_request(request.force_refresh, cached_max_mtime, max_mtime);

        {
            let mut cache = self.lock_cache();
            cache.insert(
                cache_key,
                ScanCacheEntry {
                    output: output.clone(),
                    max_mtime,
                    cached_at,
                },
            );
        }

        Ok(ScanProjectResponse {
            output,
            execution: execution_meta(&decision, cached_at, max_mtime),
        })
    }

    fn try_cached_response(
        &self,
        cache_key: &Path,
        force_refresh: bool,
        current_mtime: SystemTime,
    ) -> Option<ScanProjectResponse> {
        let cache = self.lock_cache();
        let entry = cache.get(cache_key)?;
        let decision = classify_scan_request(force_refresh, Some(entry.max_mtime), current_mtime);
        if decision.cache_state != ScanCacheState::Hit {
            return None;
        }

        Some(ScanProjectResponse {
            output: entry.output.clone(),
            execution: execution_meta(&decision, entry.cached_at, entry.max_mtime),
        })
    }

    fn lock_cache(&self) -> MutexGuard<'_, HashMap<PathBuf, ScanCacheEntry>> {
        self.state
            .scan_cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

pub fn format_scan_response(response: &ScanProjectResponse) -> String {
    format!(
        "Scanned {} .uasset file(s), found {} device(s), skipped {}. Cache: {} ({})",
        response.output.total_files,
        response.output.total_devices,
        response.output.skipped,
        cache_state_label(&response.execution.cache_state),
        response.execution.reason
    )
}

fn cache_state_label(state: &ScanCacheState) -> &'static str {
    match state {
        ScanCacheState::Miss => "miss",
        ScanCacheState::Hit => "hit",
        ScanCacheState::ForcedRefresh => "forced_refresh",
        ScanCacheState::ModifiedRefresh => "modified_refresh",
    }
}

fn normalize_cache_key(project_path: &Path) -> PathBuf {
    std::fs::canonicalize(project_path).unwrap_or_else(|_| project_path.to_path_buf())
}

fn max_project_mtime(project_path: &Path) -> SystemTime {
    let content_root = project_path.join("Content");
    let scan_dirs = [
        content_root.join("__ExternalActors__"),
        content_root.join("__ExternalObjects__"),
    ];
    let mut max_mtime = SystemTime::UNIX_EPOCH;

    for scan_dir in scan_dirs {
        if !scan_dir.exists() {
            continue;
        }

        for entry in walkdir::WalkDir::new(&scan_dir)
            .into_iter()
            .filter_map(std::result::Result::ok)
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "uasset"))
        {
            if let Ok(metadata) = entry.metadata() {
                if let Ok(mtime) = metadata.modified() {
                    max_mtime = max_mtime.max(mtime);
                }
            }
        }
    }

    max_mtime
}

fn classify_scan_request(
    force_refresh: bool,
    cached_max_mtime: Option<SystemTime>,
    current_max_mtime: SystemTime,
) -> ScanDecision {
    if force_refresh {
        return ScanDecision {
            cache_state: ScanCacheState::ForcedRefresh,
            reason: "force_refresh requested".to_string(),
        };
    }

    match cached_max_mtime {
        None => ScanDecision {
            cache_state: ScanCacheState::Miss,
            reason: "no cached scan output for project".to_string(),
        },
        Some(cached_max_mtime) if current_max_mtime > cached_max_mtime => ScanDecision {
            cache_state: ScanCacheState::ModifiedRefresh,
            reason: "project assets changed since the cached scan".to_string(),
        },
        Some(_) => ScanDecision {
            cache_state: ScanCacheState::Hit,
            reason: "reused cached scan output".to_string(),
        },
    }
}

fn execution_meta(
    decision: &ScanDecision,
    cached_at: SystemTime,
    max_mtime: SystemTime,
) -> ScanExecutionMeta {
    ScanExecutionMeta {
        cache_state: decision.cache_state.clone(),
        reason: decision.reason.clone(),
        cached_at_epoch_ms: system_time_to_epoch_ms(cached_at),
        max_mtime_epoch_ms: system_time_to_epoch_ms_opt(max_mtime),
    }
}

fn system_time_to_epoch_ms(time: SystemTime) -> u128 {
    time.duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

fn system_time_to_epoch_ms_opt(time: SystemTime) -> Option<u128> {
    (time > SystemTime::UNIX_EPOCH).then(|| system_time_to_epoch_ms(time))
}

#[derive(Debug, Clone)]
struct ScanDecision {
    cache_state: ScanCacheState,
    reason: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn classify_scan_request_reports_expected_cache_states() {
        let now = SystemTime::now();
        let earlier = now.checked_sub(Duration::from_secs(10)).unwrap();

        assert_eq!(
            classify_scan_request(false, None, now).cache_state,
            ScanCacheState::Miss
        );
        assert_eq!(
            classify_scan_request(false, Some(now), now).cache_state,
            ScanCacheState::Hit
        );
        assert_eq!(
            classify_scan_request(true, Some(now), now).cache_state,
            ScanCacheState::ForcedRefresh
        );
        assert_eq!(
            classify_scan_request(false, Some(earlier), now).cache_state,
            ScanCacheState::ModifiedRefresh
        );
    }

    #[test]
    fn scan_project_reuses_cached_output_for_same_project() {
        let temp_root = temp_project_root("cache-hit");
        let project_path = write_invalid_uasset_project(&temp_root, "first.uasset");
        let engine = GroundingEngine::default();
        let request = ScanProjectRequest {
            project_path: project_path.clone(),
            force_refresh: false,
        };

        let first = engine.scan_project(&request).unwrap();
        let second = engine.scan_project(&request).unwrap();

        assert_eq!(first.execution.cache_state, ScanCacheState::Miss);
        assert_eq!(second.execution.cache_state, ScanCacheState::Hit);
        assert_eq!(second.output.total_files, 1);
        assert_eq!(second.output.skipped, 1);

        let _ = fs::remove_dir_all(temp_root);
    }

    #[test]
    fn scan_project_separates_cache_by_project_root() {
        let temp_root = temp_project_root("per-project");
        let project_a = write_invalid_uasset_project(&temp_root.join("a"), "a.uasset");
        let project_b = write_invalid_uasset_project(&temp_root.join("b"), "b.uasset");
        let engine = GroundingEngine::default();

        let response_a = engine
            .scan_project(&ScanProjectRequest {
                project_path: project_a,
                force_refresh: false,
            })
            .unwrap();
        let response_b = engine
            .scan_project(&ScanProjectRequest {
                project_path: project_b,
                force_refresh: false,
            })
            .unwrap();

        assert_eq!(response_a.execution.cache_state, ScanCacheState::Miss);
        assert_eq!(response_b.execution.cache_state, ScanCacheState::Miss);

        let _ = fs::remove_dir_all(temp_root);
    }

    #[test]
    fn scan_project_detects_modified_assets() {
        let temp_root = temp_project_root("modified-refresh");
        let project_path = write_invalid_uasset_project(&temp_root, "device.uasset");
        let engine = GroundingEngine::default();
        let request = ScanProjectRequest {
            project_path: project_path.clone(),
            force_refresh: false,
        };

        let first = engine.scan_project(&request).unwrap();
        assert_eq!(first.execution.cache_state, ScanCacheState::Miss);

        let asset_path = project_path
            .join("Content")
            .join("__ExternalActors__")
            .join("device.uasset");
        thread::sleep(Duration::from_millis(1100));
        fs::write(&asset_path, b"still-not-a-real-uasset-with-a-newer-mtime").unwrap();

        let second = engine.scan_project(&request).unwrap();
        assert_eq!(
            second.execution.cache_state,
            ScanCacheState::ModifiedRefresh
        );

        let _ = fs::remove_dir_all(temp_root);
    }

    fn temp_project_root(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "verse-mcp-grounding-engine-{label}-{}-{}",
            std::process::id(),
            system_time_to_epoch_ms(SystemTime::now())
        ))
    }

    fn write_invalid_uasset_project(root: &Path, asset_name: &str) -> PathBuf {
        let asset_dir = root.join("Content").join("__ExternalActors__");
        fs::create_dir_all(&asset_dir).unwrap();
        fs::write(asset_dir.join(asset_name), b"not-a-real-uasset").unwrap();
        root.to_path_buf()
    }
}

use anyhow::{bail, Result};
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

#[derive(Debug, Clone, Serialize)]
pub struct ReloadMetadataRequest {
    pub project_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScanCacheState {
    Miss,
    Hit,
    ForcedRefresh,
    ModifiedRefresh,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScanPolicyAction {
    FreshScan,
    UseCachedScan,
    ReloadMetadata,
    NoCachedMetadata,
    Deny,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanPolicyDecision {
    pub action: ScanPolicyAction,
    pub allowed: bool,
    pub reason: String,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanExecutionMeta {
    pub policy: ScanPolicyDecision,
    pub cache_state: ScanCacheState,
    pub cached_at_epoch_ms: u128,
    pub max_mtime_epoch_ms: Option<u128>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanProjectResponse {
    #[serde(flatten)]
    pub output: uasset_scan::ScanOutput,
    pub execution: ScanExecutionMeta,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReloadMetadataResponse {
    pub project_path: String,
    pub policy: ScanPolicyDecision,
    pub dropped_cached_scan: bool,
    pub cached_projects_remaining: usize,
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

    pub fn evaluate_scan_policy(&self, request: &ScanProjectRequest) -> ScanPolicyDecision {
        if let Some(denied) = deny_unless_scannable(&request.project_path) {
            return denied;
        }

        let cache_key = normalize_cache_key(&request.project_path);
        let current_mtime = max_project_mtime(&request.project_path);
        let cached_max_mtime = {
            let cache = self.lock_cache();
            cache.get(&cache_key).map(|entry| entry.max_mtime)
        };

        classify_scan_request(request.force_refresh, cached_max_mtime, current_mtime)
    }

    pub fn scan_project(&self, request: &ScanProjectRequest) -> Result<ScanProjectResponse> {
        let policy = self.evaluate_scan_policy(request);
        if !policy.allowed {
            bail!(policy.reason.clone());
        }

        let cache_key = normalize_cache_key(&request.project_path);
        let current_mtime = max_project_mtime(&request.project_path);
        if policy.action == ScanPolicyAction::UseCachedScan {
            if let Some(response) =
                self.try_cached_response(&cache_key, current_mtime, policy.clone())
            {
                return Ok(response);
            }
        }

        let output = uasset_scan::scan_project(&request.project_path)?;
        let max_mtime = max_project_mtime(&request.project_path);
        let cached_at = SystemTime::now();

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
            execution: execution_meta(policy, cached_at, max_mtime),
        })
    }

    pub fn reload_project_metadata(
        &self,
        request: &ReloadMetadataRequest,
    ) -> ReloadMetadataResponse {
        let cache_key = normalize_cache_key(&request.project_path);
        let project_path = request.project_path.display().to_string();
        let mut cache = self.lock_cache();
        let removed = cache.remove(&cache_key).is_some();
        let remaining = cache.len();
        let policy = if removed {
            ScanPolicyDecision {
                action: ScanPolicyAction::ReloadMetadata,
                allowed: true,
                reason: "cleared cached project scan metadata".to_string(),
                explanation: "The next scan will rebuild metadata for this project without restarting the MCP server.".to_string(),
            }
        } else {
            ScanPolicyDecision {
                action: ScanPolicyAction::NoCachedMetadata,
                allowed: true,
                reason: "project had no cached scan metadata".to_string(),
                explanation: "No in-memory scan cache existed for this project, so there was nothing to reload.".to_string(),
            }
        };

        ReloadMetadataResponse {
            project_path,
            policy,
            dropped_cached_scan: removed,
            cached_projects_remaining: remaining,
        }
    }

    pub fn list_agent_workflows(&self) -> Result<crate::WorkflowCatalog> {
        crate::workflows::list_workflows()
    }

    pub fn get_agent_workflow(&self, name: &str) -> Result<crate::AgentWorkflow> {
        crate::workflows::get_workflow(name)
    }

    fn try_cached_response(
        &self,
        cache_key: &Path,
        current_mtime: SystemTime,
        policy: ScanPolicyDecision,
    ) -> Option<ScanProjectResponse> {
        let cache = self.lock_cache();
        let entry = cache.get(cache_key)?;
        if current_mtime > entry.max_mtime {
            return None;
        }

        Some(ScanProjectResponse {
            output: entry.output.clone(),
            execution: execution_meta(policy, entry.cached_at, entry.max_mtime),
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
        response.execution.policy.reason
    )
}

pub fn format_reload_metadata_response(response: &ReloadMetadataResponse) -> String {
    format!(
        "Metadata reload for {}: {} ({})",
        response.project_path, response.policy.reason, response.policy.explanation
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

fn deny_unless_scannable(project_path: &Path) -> Option<ScanPolicyDecision> {
    if !project_path.exists() {
        return Some(ScanPolicyDecision {
            action: ScanPolicyAction::Deny,
            allowed: false,
            reason: "project path does not exist".to_string(),
            explanation: format!(
                "{} does not exist on disk. Pass a valid UEFN project root.",
                project_path.display()
            ),
        });
    }

    if !project_path.is_dir() {
        return Some(ScanPolicyDecision {
            action: ScanPolicyAction::Deny,
            allowed: false,
            reason: "project path is not a directory".to_string(),
            explanation: format!(
                "{} is not a directory. Pass the UEFN project root directory instead of a file.",
                project_path.display()
            ),
        });
    }

    let content_root = project_path.join("Content");
    let has_scan_roots = [
        content_root.join("__ExternalActors__"),
        content_root.join("__ExternalObjects__"),
    ]
    .iter()
    .any(|path| path.exists());

    if !has_scan_roots {
        return Some(ScanPolicyDecision {
            action: ScanPolicyAction::Deny,
            allowed: false,
            reason: "project is missing external actor/object scan roots".to_string(),
            explanation: "The project does not contain Content/__ExternalActors__ or Content/__ExternalObjects__, so scan_map_devices cannot read placed-device metadata.".to_string(),
        });
    }

    None
}

fn classify_scan_request(
    force_refresh: bool,
    cached_max_mtime: Option<SystemTime>,
    current_max_mtime: SystemTime,
) -> ScanPolicyDecision {
    if force_refresh {
        return ScanPolicyDecision {
            action: ScanPolicyAction::FreshScan,
            allowed: true,
            reason: "force_refresh requested".to_string(),
            explanation: "Bypassing the in-memory cache and rebuilding project metadata from disk."
                .to_string(),
        };
    }

    match cached_max_mtime {
        None => ScanPolicyDecision {
            action: ScanPolicyAction::FreshScan,
            allowed: true,
            reason: "no cached scan output for project".to_string(),
            explanation: "No cached metadata exists yet, so a fresh project scan is required."
                .to_string(),
        },
        Some(cached_max_mtime) if current_max_mtime > cached_max_mtime => ScanPolicyDecision {
            action: ScanPolicyAction::FreshScan,
            allowed: true,
            reason: "project assets changed since the cached scan".to_string(),
            explanation:
                "A newer .uasset modification time was detected, so the cached metadata must be rebuilt."
                    .to_string(),
        },
        Some(_) => ScanPolicyDecision {
            action: ScanPolicyAction::UseCachedScan,
            allowed: true,
            reason: "reused cached scan output".to_string(),
            explanation:
                "The in-memory scan cache is still fresh for this project, so no disk re-scan is needed."
                    .to_string(),
        },
    }
}

fn execution_meta(
    policy: ScanPolicyDecision,
    cached_at: SystemTime,
    max_mtime: SystemTime,
) -> ScanExecutionMeta {
    ScanExecutionMeta {
        cache_state: cache_state_from_policy(&policy),
        policy,
        cached_at_epoch_ms: system_time_to_epoch_ms(cached_at),
        max_mtime_epoch_ms: system_time_to_epoch_ms_opt(max_mtime),
    }
}

fn cache_state_from_policy(policy: &ScanPolicyDecision) -> ScanCacheState {
    match policy.action {
        ScanPolicyAction::UseCachedScan => ScanCacheState::Hit,
        ScanPolicyAction::FreshScan if policy.reason == "force_refresh requested" => {
            ScanCacheState::ForcedRefresh
        }
        ScanPolicyAction::FreshScan
            if policy.reason == "project assets changed since the cached scan" =>
        {
            ScanCacheState::ModifiedRefresh
        }
        ScanPolicyAction::FreshScan
        | ScanPolicyAction::ReloadMetadata
        | ScanPolicyAction::NoCachedMetadata
        | ScanPolicyAction::Deny => ScanCacheState::Miss,
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
            classify_scan_request(false, None, now).action,
            ScanPolicyAction::FreshScan
        );
        assert_eq!(
            classify_scan_request(false, Some(now), now).action,
            ScanPolicyAction::UseCachedScan
        );
        assert_eq!(
            classify_scan_request(true, Some(now), now).reason,
            "force_refresh requested"
        );
        assert_eq!(
            classify_scan_request(false, Some(earlier), now).reason,
            "project assets changed since the cached scan"
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

    #[test]
    fn evaluate_scan_policy_denies_missing_project_paths() {
        let engine = GroundingEngine::default();
        let policy = engine.evaluate_scan_policy(&ScanProjectRequest {
            project_path: std::env::temp_dir().join("verse-mcp-missing-project-does-not-exist"),
            force_refresh: false,
        });

        assert_eq!(policy.action, ScanPolicyAction::Deny);
        assert!(!policy.allowed);
        assert_eq!(policy.reason, "project path does not exist");
    }

    #[test]
    fn reload_project_metadata_clears_cached_scan_without_restart() {
        let temp_root = temp_project_root("reload-metadata");
        let project_path = write_invalid_uasset_project(&temp_root, "device.uasset");
        let engine = GroundingEngine::default();
        let request = ScanProjectRequest {
            project_path: project_path.clone(),
            force_refresh: false,
        };

        let first = engine.scan_project(&request).unwrap();
        assert_eq!(first.execution.cache_state, ScanCacheState::Miss);

        let reload = engine.reload_project_metadata(&ReloadMetadataRequest {
            project_path: project_path.clone(),
        });
        assert_eq!(reload.policy.action, ScanPolicyAction::ReloadMetadata);
        assert!(reload.dropped_cached_scan);

        let second = engine.scan_project(&request).unwrap();
        assert_eq!(second.execution.cache_state, ScanCacheState::Miss);

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

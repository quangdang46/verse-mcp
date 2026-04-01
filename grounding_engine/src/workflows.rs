use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const WORKFLOW_DIR_ENV: &str = "VERSE_MCP_WORKFLOW_DIR";

#[derive(Debug, Clone, Serialize)]
pub struct AgentWorkflowSummary {
    pub name: String,
    pub title: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub source_path: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AgentWorkflow {
    pub name: String,
    pub title: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub source_path: String,
    pub updated_at_epoch_ms: Option<u128>,
    pub body: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkflowCatalog {
    pub root: String,
    pub workflows: Vec<AgentWorkflowSummary>,
}

#[derive(Debug, Default, Deserialize)]
struct WorkflowFrontmatter {
    name: Option<String>,
    title: Option<String>,
    summary: Option<String>,
    tags: Option<Vec<String>>,
}

pub fn list_workflows() -> Result<WorkflowCatalog> {
    let root = workflow_root()?;
    list_workflows_from_dir(&root)
}

pub fn get_workflow(name: &str) -> Result<AgentWorkflow> {
    let root = workflow_root()?;
    get_workflow_from_dir(&root, name)
}

pub fn list_workflows_from_dir(root: &Path) -> Result<WorkflowCatalog> {
    let mut workflows = Vec::new();
    for path in markdown_files(root)? {
        let workflow = parse_workflow_file(root, &path)?;
        workflows.push(AgentWorkflowSummary {
            name: workflow.name,
            title: workflow.title,
            summary: workflow.summary,
            tags: workflow.tags,
            source_path: workflow.source_path,
        });
    }

    workflows.sort_by(|left, right| left.name.cmp(&right.name));

    Ok(WorkflowCatalog {
        root: root.display().to_string(),
        workflows,
    })
}

pub fn get_workflow_from_dir(root: &Path, name: &str) -> Result<AgentWorkflow> {
    let normalized_name = normalize_name(name);
    for path in markdown_files(root)? {
        let workflow = parse_workflow_file(root, &path)?;
        if workflow.name == normalized_name {
            return Ok(workflow);
        }
    }

    Err(anyhow!("workflow `{name}` was not found"))
}

fn workflow_root() -> Result<PathBuf> {
    if let Some(path) = std::env::var_os(WORKFLOW_DIR_ENV) {
        return Ok(PathBuf::from(path));
    }

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| anyhow!("grounding_engine must live under the repo root"))?;
    Ok(repo_root.join("workflows"))
}

fn markdown_files(root: &Path) -> Result<Vec<PathBuf>> {
    let entries = fs::read_dir(root)
        .with_context(|| format!("failed to read workflow directory {}", root.display()))?;
    let mut files = entries
        .filter_map(std::result::Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
        .collect::<Vec<_>>();
    files.sort();
    Ok(files)
}

fn parse_workflow_file(root: &Path, path: &Path) -> Result<AgentWorkflow> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read workflow file {}", path.display()))?;
    let (frontmatter, body) = split_frontmatter(&raw)?;
    let metadata = frontmatter
        .map(|yaml| serde_yaml::from_str::<WorkflowFrontmatter>(&yaml))
        .transpose()
        .with_context(|| format!("failed to parse workflow metadata in {}", path.display()))?
        .unwrap_or_default();
    let body = body.trim().to_string();
    let title = metadata
        .title
        .filter(|value| !value.trim().is_empty())
        .or_else(|| first_heading(&body))
        .unwrap_or_else(|| fallback_title(path));
    let summary = metadata
        .summary
        .filter(|value| !value.trim().is_empty())
        .or_else(|| first_paragraph(&body))
        .unwrap_or_else(|| "Verse troubleshooting workflow".to_string());
    let name = metadata
        .name
        .map(|value| normalize_name(&value))
        .unwrap_or_else(|| normalize_name(&fallback_name(path)));
    let updated_at_epoch_ms = path
        .metadata()
        .ok()
        .and_then(|metadata| metadata.modified().ok())
        .and_then(system_time_to_epoch_ms);

    Ok(AgentWorkflow {
        name,
        title,
        summary,
        tags: metadata.tags.unwrap_or_default(),
        source_path: path
            .strip_prefix(root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/"),
        updated_at_epoch_ms,
        body,
    })
}

fn split_frontmatter(raw: &str) -> Result<(Option<String>, String)> {
    let normalized = raw.replace("\r\n", "\n");
    if !normalized.starts_with("---\n") {
        return Ok((None, normalized));
    }

    let rest = &normalized[4..];
    let closing = rest
        .find("\n---\n")
        .ok_or_else(|| anyhow!("workflow frontmatter is missing a closing `---` fence"))?;
    let frontmatter = rest[..closing].to_string();
    let body = rest[closing + 5..].to_string();
    Ok((Some(frontmatter), body))
}

fn first_heading(body: &str) -> Option<String> {
    body.lines()
        .map(str::trim)
        .find_map(|line| line.strip_prefix("# ").map(str::trim))
        .map(ToOwned::to_owned)
}

fn first_paragraph(body: &str) -> Option<String> {
    body.split("\n\n")
        .map(str::trim)
        .find(|paragraph| !paragraph.is_empty() && !paragraph.starts_with('#'))
        .map(ToOwned::to_owned)
}

fn fallback_title(path: &Path) -> String {
    fallback_name(path).replace('-', " ")
}

fn fallback_name(path: &Path) -> String {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| "workflow".to_string())
}

fn normalize_name(name: &str) -> String {
    name.trim().to_ascii_lowercase().replace('_', "-")
}

fn system_time_to_epoch_ms(time: std::time::SystemTime) -> Option<u128> {
    time.duration_since(std::time::SystemTime::UNIX_EPOCH)
        .ok()
        .map(|duration| duration.as_millis())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_workflows_discovers_markdown_files() {
        let temp_root = temp_workflow_root("discover");
        fs::create_dir_all(&temp_root).unwrap();
        fs::write(
            temp_root.join("editable-audit.md"),
            "---\nname: editable-audit\ntitle: Editable Audit\nsummary: Audit editables.\ntags:\n  - editable\n---\n# Editable Audit\n\nChecklist body.\n",
        )
        .unwrap();

        let catalog = list_workflows_from_dir(&temp_root).unwrap();
        assert_eq!(catalog.workflows.len(), 1);
        assert_eq!(catalog.workflows[0].name, "editable-audit");
        assert_eq!(catalog.workflows[0].title, "Editable Audit");

        let _ = fs::remove_dir_all(temp_root);
    }

    #[test]
    fn get_workflow_reads_full_body() {
        let temp_root = temp_workflow_root("body");
        fs::create_dir_all(&temp_root).unwrap();
        fs::write(
            temp_root.join("ui-wiring.md"),
            "---\nname: ui-wiring\ntitle: UI Wiring\nsummary: Audit UI.\n---\n# UI Wiring\n\nStep one.\n",
        )
        .unwrap();

        let workflow = get_workflow_from_dir(&temp_root, "ui-wiring").unwrap();
        assert_eq!(workflow.name, "ui-wiring");
        assert!(workflow.body.contains("Step one."));

        let _ = fs::remove_dir_all(temp_root);
    }

    #[test]
    fn loader_observes_updated_markdown_on_reload() {
        let temp_root = temp_workflow_root("reload");
        fs::create_dir_all(&temp_root).unwrap();
        let workflow_path = temp_root.join("digest-lookup.md");
        fs::write(
            &workflow_path,
            "---\nname: digest-lookup\ntitle: Digest Lookup\nsummary: First summary.\n---\n# Digest Lookup\n\nFirst body.\n",
        )
        .unwrap();

        let first = get_workflow_from_dir(&temp_root, "digest-lookup").unwrap();
        assert_eq!(first.summary, "First summary.");

        fs::write(
            &workflow_path,
            "---\nname: digest-lookup\ntitle: Digest Lookup\nsummary: Updated summary.\n---\n# Digest Lookup\n\nUpdated body.\n",
        )
        .unwrap();

        let second = get_workflow_from_dir(&temp_root, "digest-lookup").unwrap();
        assert_eq!(second.summary, "Updated summary.");
        assert!(second.body.contains("Updated body."));

        let _ = fs::remove_dir_all(temp_root);
    }

    fn temp_workflow_root(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "verse-mcp-workflows-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        ))
    }
}

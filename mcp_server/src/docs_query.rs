use anyhow::{anyhow, Context, Result};
use rusqlite::{params, Connection, OpenFlags};
use serde::Serialize;
use std::collections::HashSet;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

static EMBEDDED_DOCS_DB: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/verse-docs.db"));
const DOCS_DIR_NAME: &str = ".vm";
const DOCS_DB_FILE_NAME: &str = "verse-docs.db";

const DEFAULT_LIMIT: usize = 5;
const MAX_LIMIT: usize = 10;
const STOP_WORDS: &[&str] = &[
    "a", "an", "and", "are", "find", "for", "from", "how", "in", "is", "latest", "me", "of", "on",
    "or", "show", "the", "to", "what", "with",
];

static MATERIALIZED_DOCS_DB: OnceLock<PathBuf> = OnceLock::new();

#[derive(Debug, Serialize)]
pub struct DocsQueryResponse {
    pub query: String,
    pub results: Vec<DocsQueryResult>,
}

#[derive(Debug, Serialize)]
pub struct DocsQueryResult {
    pub title: String,
    pub path: String,
    pub source_url: String,
    pub doc_type: String,
    pub snippet: String,
    pub score: f64,
}

struct SearchPlan {
    source_url_expr: String,
    doc_type_expr: String,
    snippet_column_index: usize,
}

impl SearchPlan {
    fn build(conn: &Connection) -> Result<Self> {
        let docs_columns = table_columns(conn, "docs")?;
        let fts_columns = table_columns(conn, "docs_fts")?;

        let source_url_expr = if docs_columns.iter().any(|column| column == "source_url") {
            "COALESCE(d.source_url, '')".to_string()
        } else {
            "''".to_string()
        };

        let path_based_doc_type = "CASE WHEN d.path LIKE 'verse-api-pages-canonical/%' THEN 'api' WHEN d.path LIKE 'fortnite-docs-pages-canonical/%' THEN 'guide' WHEN d.path LIKE 'assets/%.digest.verse' THEN 'digest' ELSE 'doc' END";
        let doc_type_expr = if docs_columns.iter().any(|column| column == "doc_type") {
            format!("COALESCE(NULLIF(d.doc_type, ''), {path_based_doc_type})")
        } else {
            path_based_doc_type.to_string()
        };

        let snippet_column_index = fts_columns
            .iter()
            .position(|column| column == "content")
            .unwrap_or(0);

        Ok(Self {
            source_url_expr,
            doc_type_expr,
            snippet_column_index,
        })
    }

    fn sql(&self) -> String {
        format!(
            r#"
            SELECT d.title,
                   d.path,
                   {source_url_expr} AS source_url,
                   {doc_type_expr} AS doc_type,
                   snippet(docs_fts, {snippet_column_index}, '[', ']', '...', 24) AS snippet,
                   bm25(docs_fts) AS score
            FROM docs_fts
            JOIN docs d ON d.id = docs_fts.rowid
            WHERE docs_fts MATCH ?1
            ORDER BY score ASC
            LIMIT ?2
            "#,
            source_url_expr = self.source_url_expr,
            doc_type_expr = self.doc_type_expr,
            snippet_column_index = self.snippet_column_index,
        )
    }
}

pub fn query_docs(query: &str, limit: Option<usize>) -> Result<DocsQueryResponse> {
    let normalized_query = normalize_query(query)?;
    let fts_query = build_fts_query(&normalized_query)?;
    let docs_db = docs_db_path()?;
    let conn = open_docs_db(&docs_db)?;
    let limit = clamp_limit(limit);
    let search_plan = SearchPlan::build(&conn)?;
    let sql = search_plan.sql();

    let mut stmt = conn.prepare(&sql)?;
    let results = stmt
        .query_map(params![fts_query.as_str(), limit as i64], |row| {
            Ok(DocsQueryResult {
                title: row.get(0)?,
                path: row.get(1)?,
                source_url: row.get(2)?,
                doc_type: row.get(3)?,
                snippet: normalize_snippet(row.get::<_, Option<String>>(4)?.unwrap_or_default()),
                score: row.get(5)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

    Ok(DocsQueryResponse {
        query: normalized_query,
        results,
    })
}

pub fn format_query_response(response: &DocsQueryResponse) -> String {
    if response.results.is_empty() {
        return format!("No documentation matches found for \"{}\".", response.query);
    }

    let mut output = String::new();

    for (index, result) in response.results.iter().enumerate() {
        if index > 0 {
            output.push_str("\n\n--------------------------------\n\n");
        }

        let _ = writeln!(output, "### {}", result.title);

        if !result.source_url.is_empty() {
            let _ = writeln!(output, "Source: {}", result.source_url);
        } else {
            let _ = writeln!(output, "Path: {}", result.path);
        }

        let _ = writeln!(output);
        output.push_str(&result.snippet);

        if !result.doc_type.is_empty() {
            let _ = write!(output, "\n\nType: {}", result.doc_type);
        }
    }

    output
}

fn normalize_query(query: &str) -> Result<String> {
    let normalized = query.split_whitespace().collect::<Vec<_>>().join(" ");
    if normalized.is_empty() {
        return Err(anyhow!("query must be a non-empty string"));
    }

    Ok(normalized)
}

fn build_fts_query(query: &str) -> Result<String> {
    if looks_like_fts_query(query) {
        return Ok(query.to_string());
    }

    let mut seen = HashSet::new();
    let mut terms = Vec::new();

    for raw_term in query.split(|ch: char| !(ch.is_alphanumeric() || ch == '_')) {
        if raw_term.is_empty() {
            continue;
        }

        let term = raw_term.to_lowercase();
        if term.len() < 2 || STOP_WORDS.contains(&term.as_str()) {
            continue;
        }

        if seen.insert(term.clone()) {
            terms.push(term);
        }
    }

    if terms.is_empty() {
        return Err(anyhow!("query did not contain searchable terms"));
    }

    let joiner = if terms.len() <= 3 { " " } else { " OR " };
    Ok(terms.join(joiner))
}

fn looks_like_fts_query(query: &str) -> bool {
    query.contains('"')
        || query.contains(" OR ")
        || query.contains(" AND ")
        || query.contains(" NOT ")
        || query.contains(" NEAR")
}

fn clamp_limit(limit: Option<usize>) -> usize {
    limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT)
}

fn docs_db_path() -> Result<PathBuf> {
    if let Some(path) = MATERIALIZED_DOCS_DB.get() {
        return Ok(path.clone());
    }

    let path = materialize_embedded_docs_db()?;
    let _ = MATERIALIZED_DOCS_DB.set(path.clone());
    Ok(path)
}

fn materialize_embedded_docs_db() -> Result<PathBuf> {
    let cache_dir = docs_cache_dir()?;
    materialize_docs_db(&cache_dir, DOCS_DB_FILE_NAME, EMBEDDED_DOCS_DB)
}

fn materialize_docs_db(cache_dir: &Path, file_name: &str, bytes: &[u8]) -> Result<PathBuf> {
    fs::create_dir_all(cache_dir).with_context(|| {
        format!(
            "failed to create docs cache directory at {}",
            cache_dir.display()
        )
    })?;

    let final_path = cache_dir.join(file_name);
    if fs::metadata(&final_path)
        .map(|metadata| metadata.len() == bytes.len() as u64)
        .unwrap_or(false)
    {
        return Ok(final_path);
    }

    let temp_path = cache_dir.join(format!(
        "{file_name}.{}.{}.tmp",
        std::process::id(),
        unique_suffix()
    ));
    fs::write(&temp_path, bytes)
        .with_context(|| format!("failed to write docs cache file to {}", temp_path.display()))?;

    if final_path.exists() {
        let _ = fs::remove_file(&final_path);
    }

    match fs::rename(&temp_path, &final_path) {
        Ok(()) => Ok(final_path),
        Err(error) => {
            let _ = fs::remove_file(&temp_path);
            Err(error).with_context(|| {
                format!(
                    "failed to move docs cache file into place at {}",
                    final_path.display()
                )
            })
        }
    }
}

fn docs_cache_dir() -> Result<PathBuf> {
    Ok(home_dir()?.join(DOCS_DIR_NAME))
}

fn home_dir() -> Result<PathBuf> {
    env::var_os("HOME")
        .map(PathBuf::from)
        .filter(|path| !path.as_os_str().is_empty())
        .or_else(|| {
            env::var_os("USERPROFILE")
                .map(PathBuf::from)
                .filter(|path| !path.as_os_str().is_empty())
        })
        .ok_or_else(|| anyhow!("could not determine home directory for docs cache"))
}

fn unique_suffix() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}

fn open_docs_db(path: &Path) -> Result<Connection> {
    if !path.exists() {
        return Err(anyhow!("docs database not found at {}", path.display()));
    }

    let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .with_context(|| format!("failed to open docs database at {}", path.display()))?;

    ensure_required_schema(&conn)?;
    Ok(conn)
}

fn ensure_required_schema(conn: &Connection) -> Result<()> {
    let docs_columns = table_columns(conn, "docs")?;
    if docs_columns.is_empty() {
        return Err(anyhow!("docs database is missing required table: docs"));
    }

    for required in ["id", "title", "path"] {
        if !docs_columns.iter().any(|column| column == required) {
            return Err(anyhow!(
                "docs database is missing required docs column: {required}"
            ));
        }
    }

    let fts_columns = table_columns(conn, "docs_fts")?;
    if fts_columns.is_empty() {
        return Err(anyhow!("docs database is missing required table: docs_fts"));
    }

    Ok(())
}

fn table_columns(conn: &Connection, table_name: &str) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({table_name})"))?;
    let columns = stmt
        .query_map([], |row| row.get::<_, String>(1))?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(columns)
}

fn normalize_snippet(snippet: String) -> String {
    snippet.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_query_rejects_blank_input() {
        let error = normalize_query("   \n\t  ").unwrap_err();
        assert_eq!(error.to_string(), "query must be a non-empty string");
    }

    #[test]
    fn normalize_query_collapses_whitespace() {
        assert_eq!(
            normalize_query(" editable   properties\n in verse ").unwrap(),
            "editable properties in verse"
        );
    }

    #[test]
    fn build_fts_query_keeps_explicit_fts_syntax() {
        assert_eq!(
            build_fts_query("\"editable properties\" OR @editable").unwrap(),
            "\"editable properties\" OR @editable"
        );
    }

    #[test]
    fn build_fts_query_sanitizes_natural_language() {
        assert_eq!(
            build_fts_query(
                "Find the latest Verse language documentation for editable properties and show examples of @editable usage."
            )
            .unwrap(),
            "verse OR language OR documentation OR editable OR properties OR examples OR usage"
        );
    }

    #[test]
    fn build_fts_query_rejects_empty_terms() {
        let error = build_fts_query("@@@").unwrap_err();
        assert_eq!(error.to_string(), "query did not contain searchable terms");
    }

    #[test]
    fn clamp_limit_uses_defaults_and_bounds() {
        assert_eq!(clamp_limit(None), DEFAULT_LIMIT);
        assert_eq!(clamp_limit(Some(0)), 1);
        assert_eq!(clamp_limit(Some(3)), 3);
        assert_eq!(clamp_limit(Some(99)), MAX_LIMIT);
    }

    #[test]
    fn materialize_docs_db_writes_and_reuses_existing_file() {
        let temp_dir = std::env::temp_dir().join(format!(
            "verse-mcp-docs-test-{}-{}",
            std::process::id(),
            unique_suffix()
        ));
        fs::create_dir_all(&temp_dir).unwrap();

        let path = materialize_docs_db(&temp_dir, DOCS_DB_FILE_NAME, b"abc").unwrap();
        assert_eq!(path, temp_dir.join(DOCS_DB_FILE_NAME));
        assert_eq!(fs::read(&path).unwrap(), b"abc");

        let reused = materialize_docs_db(&temp_dir, DOCS_DB_FILE_NAME, b"xyz").unwrap();
        assert_eq!(reused, path);
        assert_eq!(fs::read(&reused).unwrap(), b"abc");

        let _ = fs::remove_file(&reused);
        let _ = fs::remove_dir(&temp_dir);
    }

    #[test]
    fn normalize_snippet_collapses_whitespace() {
        assert_eq!(
            normalize_snippet("... foo\n   bar\t baz ...".to_string()),
            "... foo bar baz ..."
        );
    }

    #[test]
    fn format_query_response_renders_source_url() {
        let response = DocsQueryResponse {
            query: "button device".to_string(),
            results: vec![DocsQueryResult {
                title: "Button Widget".to_string(),
                path: "verse-api-pages-canonical/fortnitedotcom/ui/button_regular.md".to_string(),
                source_url: "https://example.com/button".to_string(),
                doc_type: "api".to_string(),
                snippet: "Defines a text button widget.".to_string(),
                score: -1.0,
            }],
        };

        assert_eq!(
            format_query_response(&response),
            "### Button Widget\nSource: https://example.com/button\n\nDefines a text button widget.\n\nType: api"
        );
    }

    #[test]
    fn format_query_response_falls_back_to_path() {
        let response = DocsQueryResponse {
            query: "GetHUDController".to_string(),
            results: vec![DocsQueryResult {
                title: "Fortnite digest".to_string(),
                path: "assets/Fortnite.digest.verse".to_string(),
                source_url: "".to_string(),
                doc_type: "digest".to_string(),
                snippet: "(Playspace:fort_playspace).GetHUDController<native><public>():fort_hud_controller".to_string(),
                score: -1.0,
            }],
        };

        assert_eq!(
            format_query_response(&response),
            "### Fortnite digest\nPath: assets/Fortnite.digest.verse\n\n(Playspace:fort_playspace).GetHUDController<native><public>():fort_hud_controller\n\nType: digest"
        );
    }

    #[test]
    fn format_query_response_handles_empty_results() {
        let response = DocsQueryResponse {
            query: "missing symbol".to_string(),
            results: vec![],
        };

        assert_eq!(
            format_query_response(&response),
            "No documentation matches found for \"missing symbol\"."
        );
    }
}

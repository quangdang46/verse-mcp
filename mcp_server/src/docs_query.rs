use anyhow::{anyhow, Context, Result};
use reqwest::blocking::Client;
use rusqlite::{params, Connection, OpenFlags};
use serde::Serialize;
use std::collections::HashSet;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

static EMBEDDED_DOCS_DB: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/verse-docs.db"));
const DOCS_DIR_NAME: &str = ".vm";
const DOCS_DB_FILE_NAME: &str = "verse-docs.db";

const DEFAULT_LIMIT: usize = 5;
const MAX_LIMIT: usize = 10;
const DEFAULT_OFFSET: usize = 0;
const DEFAULT_MAX_FETCHES: usize = 3;
const MAX_FETCHES: usize = 5;
const MAX_CONTENT_CHARS: usize = 16_000;
const MAX_FETCHED_TEXT_CHARS: usize = 12_000;
const STOP_WORDS: &[&str] = &[
    "a", "an", "and", "are", "find", "for", "from", "how", "in", "is", "latest", "me", "of", "on",
    "or", "show", "the", "to", "what", "with",
];

static MATERIALIZED_DOCS_DB: OnceLock<PathBuf> = OnceLock::new();

#[derive(Debug, Serialize)]
pub struct DocsQueryResponse {
    pub query: String,
    pub normalized_query: String,
    pub pagination: DocsQueryPagination,
    pub results: Vec<DocsQueryResult>,
    pub fetched_sources: Vec<FetchedSource>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct DocsQueryPagination {
    pub limit: usize,
    pub offset: usize,
    pub returned: usize,
    pub has_more: bool,
    pub next_offset: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct DocsQueryResult {
    pub title: String,
    pub path: String,
    pub source_url: String,
    pub doc_type: String,
    pub snippet: String,
    pub content: String,
    pub content_truncated: bool,
    pub score: f64,
}

#[derive(Debug, Serialize)]
pub struct FetchedSource {
    pub url: String,
    pub status: String,
    pub content_type: String,
    pub title: Option<String>,
    pub text: String,
    pub truncated: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DocsQueryOptions {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub fetch_source_urls: bool,
    pub max_fetches: Option<usize>,
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
                   d.content,
                   bm25(docs_fts) AS score
            FROM docs_fts
            JOIN docs d ON d.id = docs_fts.rowid
            WHERE docs_fts MATCH ?1
            ORDER BY score ASC
            LIMIT ?2 OFFSET ?3
            "#,
            source_url_expr = self.source_url_expr,
            doc_type_expr = self.doc_type_expr,
            snippet_column_index = self.snippet_column_index,
        )
    }

    fn count_sql(&self) -> String {
        format!(
            r#"
            SELECT COUNT(*)
            FROM docs_fts
            JOIN docs d ON d.id = docs_fts.rowid
            WHERE docs_fts MATCH ?1
            "#
        )
    }
}

pub fn query_docs(query: &str, options: DocsQueryOptions) -> Result<DocsQueryResponse> {
    let normalized_query = normalize_query(query)?;
    let fts_query = build_fts_query(&normalized_query)?;
    let docs_db = docs_db_path()?;
    let conn = open_docs_db(&docs_db)?;
    let limit = clamp_limit(options.limit);
    let offset = clamp_offset(options.offset);
    let max_fetches = clamp_fetches(options.max_fetches);
    let search_plan = SearchPlan::build(&conn)?;
    let sql = search_plan.sql();

    let total_matches: usize = conn
        .query_row(&search_plan.count_sql(), params![fts_query.as_str()], |row| {
            row.get::<_, i64>(0)
        })? as usize;

    let mut stmt = conn.prepare(&sql)?;
    let results = stmt
        .query_map(params![fts_query.as_str(), limit as i64, offset as i64], |row| {
            let content = truncate_text(normalize_content(row.get::<_, Option<String>>(5)?.unwrap_or_default()), MAX_CONTENT_CHARS);
            Ok(DocsQueryResult {
                title: row.get(0)?,
                path: row.get(1)?,
                source_url: row.get(2)?,
                doc_type: row.get(3)?,
                snippet: normalize_snippet(row.get::<_, Option<String>>(4)?.unwrap_or_default()),
                content_truncated: content.1,
                content: content.0,
                score: row.get(6)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

    let mut warnings = Vec::new();
    let fetched_sources = if options.fetch_source_urls {
        fetch_sources(&results, max_fetches, &mut warnings)
    } else {
        Vec::new()
    };

    let returned = results.len();
    let has_more = offset + returned < total_matches;
    let next_offset = has_more.then_some(offset + returned);

    if options.fetch_source_urls {
        warnings.push(format!(
            "Fetched up to {} source URLs for this query.",
            max_fetches
        ));
    }

    Ok(DocsQueryResponse {
        query: query.to_string(),
        normalized_query,
        pagination: DocsQueryPagination {
            limit,
            offset,
            returned,
            has_more,
            next_offset,
        },
        results,
        fetched_sources,
        warnings,
    })
}

pub fn format_query_response(response: &DocsQueryResponse) -> String {
    if response.results.is_empty() {
        return format!("No documentation matches found for \"{}\".", response.normalized_query);
    }

    let mut output = String::new();
    let _ = writeln!(
        output,
        "Found {} documentation match(es) for \"{}\" (offset {}, limit {}).",
        response.pagination.returned,
        response.normalized_query,
        response.pagination.offset,
        response.pagination.limit
    );

    if response.pagination.has_more {
        if let Some(next_offset) = response.pagination.next_offset {
            let _ = writeln!(output, "More results available. Use offset={} to continue.", next_offset);
        }
    }

    if !response.fetched_sources.is_empty() {
        let _ = writeln!(
            output,
            "Fetched {} linked source page(s) for enrichment.",
            response.fetched_sources.len()
        );
    }

    for warning in &response.warnings {
        let _ = writeln!(output, "Warning: {}", warning);
    }

    for (index, result) in response.results.iter().enumerate() {
        if index > 0 {
            output.push_str("\n\n--------------------------------\n\n");
        }

        let _ = writeln!(output, "### {}", result.title);
        let _ = writeln!(output, "Path: {}", result.path);
        if !result.source_url.is_empty() {
            let _ = writeln!(output, "Source: {}", result.source_url);
        }
        if !result.doc_type.is_empty() {
            let _ = writeln!(output, "Type: {}", result.doc_type);
        }
        let _ = writeln!(output, "Score: {:.4}", result.score);
        let _ = writeln!(output);
        let _ = writeln!(output, "Snippet: {}", result.snippet);
        let _ = writeln!(output);
        output.push_str(&result.content);
        if result.content_truncated {
            output.push_str("\n\n[content truncated]");
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

fn clamp_offset(offset: Option<usize>) -> usize {
    offset.unwrap_or(DEFAULT_OFFSET)
}

fn clamp_fetches(max_fetches: Option<usize>) -> usize {
    max_fetches
        .unwrap_or(DEFAULT_MAX_FETCHES)
        .clamp(0, MAX_FETCHES)
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

fn normalize_content(content: String) -> String {
    content
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .trim()
        .to_string()
}

fn truncate_text(text: String, max_chars: usize) -> (String, bool) {
    let mut chars = text.chars();
    let truncated: String = chars.by_ref().take(max_chars).collect();
    let was_truncated = chars.next().is_some();
    if was_truncated {
        (format!("{}…", truncated.trim_end()), true)
    } else {
        (text, false)
    }
}

pub fn fetch_doc_source(url: &str) -> Result<FetchedSource> {
    if url.trim().is_empty() {
        return Err(anyhow!("url must be a non-empty string"));
    }

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .context("failed to initialize HTTP client")?;

    Ok(fetch_source(&client, url))
}

fn fetch_sources(results: &[DocsQueryResult], max_fetches: usize, warnings: &mut Vec<String>) -> Vec<FetchedSource> {
    if max_fetches == 0 {
        warnings.push("Source URL fetching was requested with max_fetches=0, so no links were fetched.".to_string());
        return Vec::new();
    }

    let client = match Client::builder().timeout(Duration::from_secs(10)).build() {
        Ok(client) => client,
        Err(error) => {
            warnings.push(format!("Could not initialize HTTP client for source fetches: {error}"));
            return Vec::new();
        }
    };

    let mut fetched = Vec::new();
    for result in results.iter().filter(|result| !result.source_url.is_empty()).take(max_fetches) {
        fetched.push(fetch_source(&client, &result.source_url));
    }

    fetched
}

fn fetch_source(client: &Client, url: &str) -> FetchedSource {
    match client.get(url).send() {
        Ok(response) => {
            let status = response.status();
            let content_type = response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok())
                .unwrap_or("")
                .to_string();

            match response.text() {
                Ok(body) => {
                    let title = extract_html_title(&body);
                    let normalized = normalize_fetched_text(&body);
                    let (text, truncated) = truncate_text(normalized, MAX_FETCHED_TEXT_CHARS);
                    FetchedSource {
                        url: url.to_string(),
                        status: status.to_string(),
                        content_type,
                        title,
                        text,
                        truncated,
                        error: None,
                    }
                }
                Err(error) => FetchedSource {
                    url: url.to_string(),
                    status: status.to_string(),
                    content_type,
                    title: None,
                    text: String::new(),
                    truncated: false,
                    error: Some(format!("failed to read response body: {error}")),
                },
            }
        }
        Err(error) => FetchedSource {
            url: url.to_string(),
            status: "request_failed".to_string(),
            content_type: String::new(),
            title: None,
            text: String::new(),
            truncated: false,
            error: Some(error.to_string()),
        },
    }
}

fn normalize_fetched_text(body: &str) -> String {
    body.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn extract_html_title(body: &str) -> Option<String> {
    let lower = body.to_lowercase();
    let start = lower.find("<title>")?;
    let end = lower[start + 7..].find("</title>")? + start + 7;
    let title = body[start + 7..end].trim();
    (!title.is_empty()).then(|| title.to_string())
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
    fn fetch_doc_source_rejects_blank_url() {
        let error = fetch_doc_source("   ").unwrap_err();
        assert_eq!(error.to_string(), "url must be a non-empty string");
    }

    #[test]
    fn truncate_text_marks_truncation() {
        let (text, truncated) = truncate_text("abcdef".to_string(), 3);
        assert_eq!(text, "abc…");
        assert!(truncated);
    }

    #[test]
    fn extract_html_title_finds_title_tag() {
        assert_eq!(
            extract_html_title("<html><head><title>Verse Docs</title></head></html>"),
            Some("Verse Docs".to_string())
        );
    }

    #[test]
    fn format_query_response_renders_rich_summary() {
        let response = DocsQueryResponse {
            query: "button device".to_string(),
            normalized_query: "button device".to_string(),
            pagination: DocsQueryPagination {
                limit: 5,
                offset: 0,
                returned: 1,
                has_more: false,
                next_offset: None,
            },
            results: vec![DocsQueryResult {
                title: "Button Widget".to_string(),
                path: "verse-api-pages-canonical/fortnitedotcom/ui/button_regular.md".to_string(),
                source_url: "https://example.com/button".to_string(),
                doc_type: "api".to_string(),
                snippet: "Defines a text button widget.".to_string(),
                content: "# Button Widget\n\nFull content".to_string(),
                content_truncated: false,
                score: -1.0,
            }],
            fetched_sources: vec![],
            warnings: vec![],
        };

        let rendered = format_query_response(&response);
        assert!(rendered.contains("Found 1 documentation match(es) for \"button device\""));
        assert!(rendered.contains("Source: https://example.com/button"));
        assert!(rendered.contains("Snippet: Defines a text button widget."));
        assert!(rendered.contains("# Button Widget\n\nFull content"));
    }

    #[test]
    fn format_query_response_handles_empty_results() {
        let response = DocsQueryResponse {
            query: "missing symbol".to_string(),
            normalized_query: "missing symbol".to_string(),
            pagination: DocsQueryPagination {
                limit: 5,
                offset: 0,
                returned: 0,
                has_more: false,
                next_offset: None,
            },
            results: vec![],
            fetched_sources: vec![],
            warnings: vec![],
        };

        assert_eq!(
            format_query_response(&response),
            "No documentation matches found for \"missing symbol\"."
        );
    }
}

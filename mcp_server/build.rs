use regex::Regex;
use rusqlite::Connection;
use sha2::{Digest, Sha256};
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const DOCS_DIRS: &[&str] = &["verse-api-pages-canonical", "fortnite-docs-pages-canonical"];
const DIGEST_FILES: &[&str] = &[
    "assets/Fortnite.digest.verse",
    "assets/UnrealEngine.digest.verse",
    "assets/Verse.digest.verse",
];
const SKIP_TITLES: &[&str] = &["table of contents"];
const JUNK_MARKERS: &[&str] = &[
    "# 404",
    "Page not found",
    "**No document**",
    "The document you're looking for does not exist in this version.",
    "```\nNot Found\n```",
];

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let repo_root = manifest_dir
        .parent()
        .ok_or("mcp_server must live under repo root")?;
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let db_path = out_dir.join("verse-docs.db");

    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("build.rs").display()
    );
    for docs_dir in DOCS_DIRS {
        println!(
            "cargo:rerun-if-changed={}",
            repo_root.join(docs_dir).display()
        );
    }
    for digest_file in DIGEST_FILES {
        println!(
            "cargo:rerun-if-changed={}",
            repo_root.join(digest_file).display()
        );
    }

    build_database(repo_root, &db_path)?;

    let bytes = fs::read(&db_path)?;
    let hash = format!("{:x}", Sha256::digest(&bytes));
    println!("cargo:rustc-env=VM_DOCS_DB_SHA256={hash}");

    Ok(())
}

fn build_database(repo_root: &Path, output_path: &Path) -> Result<(), Box<dyn Error>> {
    if output_path.exists() {
        fs::remove_file(output_path)?;
    }

    let conn = Connection::open(output_path)?;
    create_schema(&conn)?;

    let mut rows = Vec::new();
    for path in iter_indexable_files(repo_root) {
        if let Some(row) = read_doc(repo_root, &path)? {
            rows.push(row);
        }
    }

    let tx = conn.unchecked_transaction()?;
    {
        let mut stmt = tx.prepare(
            "INSERT INTO docs (path, title, content, raw_md, tags, source_url, doc_type, mtime) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        )?;
        for row in rows {
            stmt.execute((
                &row.path,
                &row.title,
                &row.content,
                &row.raw_md,
                "[]",
                &row.source_url,
                &row.doc_type,
                row.mtime,
            ))?;
        }
    }
    tx.execute("INSERT INTO docs_fts(docs_fts) VALUES ('rebuild')", [])?;
    tx.commit()?;
    conn.execute_batch("VACUUM")?;

    Ok(())
}

fn create_schema(conn: &Connection) -> Result<(), Box<dyn Error>> {
    conn.execute_batch(
        r#"
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        DROP TABLE IF EXISTS docs;
        DROP TABLE IF EXISTS docs_fts;

        CREATE TABLE docs (
            id         INTEGER PRIMARY KEY,
            path       TEXT NOT NULL UNIQUE,
            title      TEXT NOT NULL,
            content    TEXT NOT NULL,
            raw_md     TEXT NOT NULL,
            tags       TEXT NOT NULL DEFAULT '[]',
            source_url TEXT NOT NULL DEFAULT '',
            doc_type   TEXT NOT NULL DEFAULT 'doc',
            mtime      INTEGER NOT NULL DEFAULT 0
        );

        CREATE VIRTUAL TABLE docs_fts USING fts5(
            title,
            content,
            tags,
            content='docs',
            content_rowid='id',
            tokenize='unicode61 remove_diacritics 1'
        );
        "#,
    )?;
    Ok(())
}

fn iter_indexable_files(repo_root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for docs_dir in DOCS_DIRS {
        files.extend(
            WalkDir::new(repo_root.join(docs_dir))
                .into_iter()
                .filter_map(Result::ok)
                .filter(|entry| entry.file_type().is_file())
                .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "md"))
                .map(|entry| entry.into_path()),
        );
    }

    files.extend(DIGEST_FILES.iter().map(|path| repo_root.join(path)));

    files.sort();
    files
}

fn read_doc(repo_root: &Path, path: &Path) -> Result<Option<DocRow>, Box<dyn Error>> {
    let raw_md = fs::read_to_string(path)?;
    let lines: Vec<&str> = raw_md.lines().collect();
    let relative_path = path
        .strip_prefix(repo_root)?
        .to_string_lossy()
        .replace('\\', "/");
    let doc_type = derive_doc_type(&relative_path);
    let is_digest = path.extension().is_some_and(|ext| ext == "verse");

    let (title, source_url, content) = if is_digest {
        (
            extract_digest_title(path, &lines),
            String::new(),
            normalize_digest_content(&raw_md),
        )
    } else {
        (
            extract_title(path, &lines),
            extract_source_url(&lines),
            normalize_content(&raw_md),
        )
    };

    if !is_digest && (content.is_empty() || is_junk_doc(&raw_md)) {
        return Ok(None);
    }

    Ok(Some(DocRow {
        path: relative_path,
        title,
        content,
        raw_md,
        source_url,
        doc_type,
        mtime: path
            .metadata()?
            .modified()?
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64,
    }))
}

fn extract_source_url(lines: &[&str]) -> String {
    lines
        .iter()
        .take(20)
        .map(|line| line.trim())
        .find_map(|line| {
            line.strip_prefix("## http://")
                .map(|rest| format!("http://{rest}"))
        })
        .or_else(|| {
            lines
                .iter()
                .take(20)
                .map(|line| line.trim())
                .find_map(|line| {
                    line.strip_prefix("## https://")
                        .map(|rest| format!("https://{rest}"))
                })
        })
        .unwrap_or_default()
}

fn extract_title(path: &Path, lines: &[&str]) -> String {
    lines
        .iter()
        .filter_map(|line| heading_text(line))
        .find(|heading| {
            let lower = heading.to_ascii_lowercase();
            !heading.starts_with("http://")
                && !heading.starts_with("https://")
                && !SKIP_TITLES.contains(&lower.as_str())
        })
        .unwrap_or_else(|| fallback_title(path))
}

fn heading_text(line: &str) -> Option<String> {
    let trimmed = line.trim();
    let level = trimmed.chars().take_while(|ch| *ch == '#').count();
    if !(1..=6).contains(&level) {
        return None;
    }

    let rest = trimmed.get(level..)?.trim_start();
    if rest.len() == trimmed.len().saturating_sub(level) {
        return None;
    }

    Some(rest.trim().to_string())
}

fn fallback_title(path: &Path) -> String {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .map(|stem| stem.replace(['-', '_'], " ").trim().to_string())
        .filter(|stem| !stem.is_empty())
        .unwrap_or_else(|| {
            path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned()
        })
}

fn extract_digest_title(path: &Path, lines: &[&str]) -> String {
    lines
        .iter()
        .map(|line| line.trim())
        .find_map(|line| {
            line.split_once("<public> := module:")
                .map(|(module, _)| module.trim())
        })
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| fallback_title(path))
}

fn normalize_content(raw_md: &str) -> String {
    let source_url_re = Regex::new(r"(?m)^##\s+https?://\S+\s*$").unwrap();
    let copy_snippet_re = Regex::new(r"(?m)^Copy full snippet\s*$").unwrap();
    let image_re = Regex::new(r"!\[[^\]]*\]\([^)]*\)").unwrap();
    let link_re = Regex::new(r"\[(.*?)\]\([^)]*\)").unwrap();
    let blank_lines_re = Regex::new(r"\n{3,}").unwrap();

    let content = raw_md.replace("\r\n", "\n");
    let content = source_url_re.replace_all(&content, "");
    let content = copy_snippet_re.replace_all(&content, "");
    let content = image_re.replace_all(&content, " ");
    let content = link_re.replace_all(&content, "$1");
    blank_lines_re
        .replace_all(content.trim(), "\n\n")
        .trim()
        .to_string()
}

fn is_junk_doc(raw_md: &str) -> bool {
    JUNK_MARKERS.iter().any(|marker| raw_md.contains(marker))
}

fn normalize_digest_content(raw_md: &str) -> String {
    let blank_lines_re = Regex::new(r"\n{3,}").unwrap();
    let content = raw_md.replace("\r\n", "\n");
    blank_lines_re
        .replace_all(content.trim(), "\n\n")
        .trim()
        .to_string()
}

fn derive_doc_type(relative_path: &str) -> String {
    if relative_path.starts_with("verse-api-pages-canonical/") {
        "api".to_string()
    } else if relative_path.starts_with("fortnite-docs-pages-canonical/") {
        "guide".to_string()
    } else if relative_path.starts_with("assets/") && relative_path.ends_with(".digest.verse") {
        "digest".to_string()
    } else {
        "doc".to_string()
    }
}

struct DocRow {
    path: String,
    title: String,
    content: String,
    raw_md: String,
    source_url: String,
    doc_type: String,
    mtime: i64,
}

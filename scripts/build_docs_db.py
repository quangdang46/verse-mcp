#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import sqlite3
from pathlib import Path

SOURCE_URL_RE = re.compile(r"^##\s+(https?://\S+)\s*$")
HEADING_RE = re.compile(r"^#{1,6}\s+(.+?)\s*$")
SKIP_TITLES = {"table of contents"}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Build the packaged Verse docs SQLite index")
    parser.add_argument("--output", required=True, help="Output SQLite database path")
    return parser.parse_args()


def repo_root() -> Path:
    return Path(__file__).resolve().parents[1]


def docs_roots(root: Path) -> list[Path]:
    return [
        root / "verse-api-pages-canonical",
        root / "fortnite-docs-pages-canonical",
    ]


def iter_markdown_files(root: Path) -> list[Path]:
    files: list[Path] = []
    for docs_root in docs_roots(root):
        files.extend(sorted(docs_root.rglob("*.md")))
    return files


def derive_doc_type(relative_path: str) -> str:
    if relative_path.startswith("verse-api-pages-canonical/"):
        return "api"
    if relative_path.startswith("fortnite-docs-pages-canonical/"):
        return "guide"
    return "doc"


def fallback_title(path: Path) -> str:
    return path.stem.replace("-", " ").replace("_", " ").strip() or path.name


def extract_source_url(lines: list[str]) -> str:
    for line in lines[:20]:
        match = SOURCE_URL_RE.match(line.strip())
        if match:
            return match.group(1)
    return ""


def extract_title(path: Path, lines: list[str]) -> str:
    for line in lines:
        match = HEADING_RE.match(line.strip())
        if not match:
            continue
        heading = match.group(1).strip()
        if heading.startswith("http://") or heading.startswith("https://"):
            continue
        if heading.lower() in SKIP_TITLES:
            continue
        return heading
    return fallback_title(path)


def normalize_content(raw_md: str) -> str:
    content = raw_md.replace("\r\n", "\n")
    content = re.sub(r"^##\s+https?://\S+\s*$", "", content, flags=re.MULTILINE)
    content = re.sub(r"^Copy full snippet\s*$", "", content, flags=re.MULTILINE)
    content = re.sub(r"!\[[^\]]*\]\([^)]*\)", " ", content)
    content = re.sub(r"\[(.*?)\]\([^)]*\)", r"\1", content)
    content = re.sub(r"\n{3,}", "\n\n", content)
    return content.strip()


def read_doc(root: Path, path: Path) -> tuple[str, str, str, str, str, str, int]:
    raw_md = path.read_text(encoding="utf-8")
    lines = raw_md.splitlines()
    relative_path = path.relative_to(root).as_posix()
    title = extract_title(path, lines)
    source_url = extract_source_url(lines)
    doc_type = derive_doc_type(relative_path)
    content = normalize_content(raw_md)
    mtime = int(path.stat().st_mtime)
    return relative_path, title, content, raw_md, source_url, doc_type, mtime


def create_schema(conn: sqlite3.Connection) -> None:
    conn.executescript(
        """
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
        """
    )


def build_database(root: Path, output_path: Path) -> None:
    output_path.parent.mkdir(parents=True, exist_ok=True)
    if output_path.exists():
        output_path.unlink()

    conn = sqlite3.connect(output_path)
    try:
        create_schema(conn)
        rows = [read_doc(root, path) for path in iter_markdown_files(root)]
        conn.executemany(
            """
            INSERT INTO docs (path, title, content, raw_md, tags, source_url, doc_type, mtime)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            """,
            [
                (path, title, content, raw_md, json.dumps([]), source_url, doc_type, mtime)
                for path, title, content, raw_md, source_url, doc_type, mtime in rows
            ],
        )
        conn.execute("INSERT INTO docs_fts(docs_fts) VALUES ('rebuild')")
        conn.commit()
        conn.execute("VACUUM")
    finally:
        conn.close()


if __name__ == "__main__":
    args = parse_args()
    root = repo_root()
    build_database(root, Path(args.output).resolve())

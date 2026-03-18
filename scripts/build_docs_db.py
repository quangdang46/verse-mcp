#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import sqlite3
from pathlib import Path
from urllib.parse import unquote, urlparse

SOURCE_URL_RE = re.compile(r"^##\s+(https?://\S+)\s*$")
HEADING_RE = re.compile(r"^#{1,6}\s+(.+?)\s*$")
SKIP_TITLES = {"table of contents"}
DOCS_DIRS = (
    "verse-api-pages-canonical",
    "fortnite-docs-pages-canonical",
)
SEARCH_TAG_FRAGMENT = "community/search?query="
FORTNITE_URL_PREFIXES = (
    ("/documentation/en-us/fortnite-creative/", "fortnite-creative/"),
    ("/documentation/en-us/fortnite/", ""),
)
STRIPPED_URL_SUFFIXES = (
    ".INT.udn",
    ".INT",
    ".udn",
    ".svg",
    ".png",
    ".jpg",
    ".jpeg",
    ".webp",
)
JUNK_MARKERS = (
    "# 404",
    "Page not found",
    "**No document**",
    "The document you're looking for does not exist in this version.",
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Build and normalize the packaged Verse docs corpus")
    parser.add_argument("--output", help="Output SQLite database path")
    parser.add_argument(
        "--rewrite-canonical-docs",
        action="store_true",
        help="Normalize canonical markdown docs in place before indexing",
    )
    args = parser.parse_args()
    if not args.output and not args.rewrite_canonical_docs:
        parser.error("provide --output and/or --rewrite-canonical-docs")
    return args


def repo_root() -> Path:
    return Path(__file__).resolve().parents[1]


def docs_roots(root: Path) -> list[Path]:
    return [root / docs_dir for docs_dir in DOCS_DIRS]


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


def normalize_line_endings(raw_md: str) -> str:
    return raw_md.replace("\r\n", "\n").replace("\r", "\n")


def trim_blank_lines(lines: list[str]) -> list[str]:
    start = 0
    end = len(lines)
    while start < end and not lines[start].strip():
        start += 1
    while end > start and not lines[end - 1].strip():
        end -= 1

    trimmed: list[str] = []
    previous_blank = False
    for line in lines[start:end]:
        blank = not line.strip()
        if blank and previous_blank:
            continue
        trimmed.append(line.rstrip())
        previous_blank = blank
    return trimmed


def strip_trailing_rule(lines: list[str]) -> list[str]:
    while lines and lines[-1].strip() == "---":
        lines.pop()
    return lines


def strip_trailing_search_tags(lines: list[str]) -> list[str]:
    end = len(lines)
    while end > 0 and not lines[end - 1].strip():
        end -= 1

    candidate = end
    while candidate > 0:
        stripped = lines[candidate - 1].strip()
        if not stripped or SEARCH_TAG_FRAGMENT in lines[candidate - 1]:
            candidate -= 1
            continue
        break

    if candidate < end and any(SEARCH_TAG_FRAGMENT in line for line in lines[candidate:end]):
        return lines[:candidate]
    return lines[:end]


def footer_start_index(lines: list[str]) -> int | None:
    for index, line in enumerate(lines):
        if line.strip() != "* * *":
            continue
        trailer = "\n".join(lines[index : index + 4])
        if "Developer Forums" in trailer or "Learning Library" in trailer:
            return index
    return None


def first_primary_heading_index(lines: list[str]) -> int | None:
    for index, line in enumerate(lines):
        stripped = line.strip()
        if stripped.startswith("# ") and stripped != "# 404":
            return index
    return None


def extract_primary_body_lines(raw_md: str) -> list[str]:
    lines = normalize_line_endings(raw_md).splitlines()
    start = first_primary_heading_index(lines)
    selected = lines[start:] if start is not None else []
    footer_index = footer_start_index(selected)
    if footer_index is not None:
        selected = selected[:footer_index]

    selected = [line.rstrip() for line in selected if line.strip() != "On this page"]
    selected = strip_trailing_search_tags(selected)
    selected = strip_trailing_rule(selected)
    return trim_blank_lines(selected)


def extract_junk_summary_lines(raw_md: str) -> list[str]:
    lines = normalize_line_endings(raw_md).splitlines()
    summary: list[str] = []
    for line in lines:
        stripped = line.strip()
        if not stripped:
            continue
        if SOURCE_URL_RE.match(stripped):
            continue
        if stripped in {"Table of Contents", "---", "* * *", "On this page"}:
            continue
        if stripped.startswith("!["):
            continue
        if SEARCH_TAG_FRAGMENT in line:
            continue
        if "Developer Forums" in line or "Learning Library" in line:
            continue
        if stripped.startswith("* ") and "http" in stripped:
            continue
        if (
            stripped == "# 404"
            or stripped.startswith("### Page not found")
            or "**No document**" in stripped
            or "The document you're looking for does not exist in this version." in stripped
            or stripped == "Not Found"
            or "The page you were looking for was not found." in stripped
        ):
            summary.append(stripped)
    return trim_blank_lines(summary)


def is_junk_doc(raw_md: str) -> bool:
    normalized = normalize_line_endings(raw_md)
    lower = normalized.lower()
    if any(marker.lower() in lower for marker in JUNK_MARKERS):
        return True
    if "```" in normalized and "Not Found" in normalized:
        return True
    if extract_primary_body_lines(normalized):
        return False

    for line in normalized.splitlines():
        stripped = line.strip()
        if not stripped:
            continue
        if SOURCE_URL_RE.match(stripped):
            continue
        if stripped in {"Table of Contents", "---", "* * *", "On this page"}:
            continue
        if SEARCH_TAG_FRAGMENT in line:
            continue
        if "Developer Forums" in line or "Learning Library" in line:
            continue
        return False
    return True


def rewrite_markdown(raw_md: str) -> str:
    normalized = normalize_line_endings(raw_md)
    lines = normalized.splitlines()
    source_url = extract_source_url(lines)
    body_lines = extract_junk_summary_lines(normalized) if is_junk_doc(normalized) else extract_primary_body_lines(normalized)

    output: list[str] = []
    if source_url:
        output.append(f"## {source_url}")
        if body_lines:
            output.append("")
    output.extend(body_lines)

    text = "\n".join(trim_blank_lines(output)).strip()
    return f"{text}\n" if text else ""


def normalize_content(raw_md: str) -> str:
    content = rewrite_markdown(raw_md)
    content = re.sub(r"^##\s+https?://\S+\s*$", "", content, flags=re.MULTILINE)
    content = re.sub(r"^Copy full snippet\s*$", "", content, flags=re.MULTILINE)
    content = re.sub(r"!\[[^\]]*\]\([^)]*\)", " ", content)
    content = re.sub(r"\[(.*?)\]\([^)]*\)", r"\1", content)
    content = re.sub(r"\n{3,}", "\n\n", content)
    return content.strip()


def read_doc(root: Path, path: Path) -> tuple[str, str, str, str, str, str, int] | None:
    raw_md = path.read_text(encoding="utf-8")
    lines = raw_md.splitlines()
    relative_path = path.relative_to(root).as_posix()
    if path.suffix == ".md" and is_junk_doc(raw_md):
        return None
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
        rows = [row for path in iter_markdown_files(root) if (row := read_doc(root, path)) is not None]
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


def strip_known_url_suffix(path_fragment: str) -> str:
    cleaned = path_fragment.rstrip("/")
    for suffix in STRIPPED_URL_SUFFIXES:
        if cleaned.endswith(suffix):
            return cleaned[: -len(suffix)]
    return cleaned


def strip_unmatched_trailing_parens(path_fragment: str) -> str:
    segments: list[str] = []
    for segment in path_fragment.split("/"):
        while segment.endswith(")") and segment.count(")") > segment.count("("):
            segment = segment[:-1]
        segments.append(segment)
    return "/".join(segments)


def normalized_url_key(source_url: str) -> str:
    if not source_url:
        return ""
    parsed_path = unquote(urlparse(source_url).path).replace("\\", "")
    return strip_unmatched_trailing_parens(parsed_path.rstrip("/"))


def canonicalize_fortnite_path(source_url: str, current_rel: str) -> str:
    if current_rel.startswith("-creative/"):
        fallback = f"fortnite-creative/{current_rel.removeprefix('-creative/')}"
    else:
        fallback = current_rel

    if not source_url:
        return fallback

    parsed_path = normalized_url_key(source_url)
    if parsed_path in {"/documentation/en-us/fortnite", "/documentation/en-us/fortnite/"}:
        return "index.md"

    for prefix, folder_prefix in FORTNITE_URL_PREFIXES:
        if not parsed_path.startswith(prefix):
            continue
        remainder = strip_known_url_suffix(parsed_path[len(prefix) :].lstrip("/"))
        if not remainder:
            remainder = "index"
        return f"{folder_prefix}{remainder}.md"

    return fallback


def canonicalize_verse_api_path(source_url: str, current_rel: str) -> str:
    if not source_url:
        return current_rel

    parsed_path = normalized_url_key(source_url)
    prefix = "/documentation/en-us/fortnite/verse-api/"
    if not parsed_path.startswith(prefix):
        return current_rel

    remainder = parsed_path[len(prefix) :].lstrip("/")
    if not remainder:
        remainder = "index"
    return f"{remainder}.md"


def rewrite_manifest(corpus_root: Path, entries: list[tuple[str, str]]) -> None:
    manifest_path = corpus_root / "_manifest.txt"
    manifest_text = "\n".join(f"{url}\t{rel}" for url, rel in entries)
    manifest_path.write_text(f"{manifest_text}\n", encoding="utf-8")


def cleanup_empty_directories(corpus_root: Path) -> None:
    for path in sorted(corpus_root.rglob("*"), key=lambda item: len(item.parts), reverse=True):
        if path.is_dir():
            try:
                path.rmdir()
            except OSError:
                pass


def rename_corpus_files(
    corpus_root: Path,
    rename_map: dict[str, str],
) -> None:
    staged_moves: list[tuple[Path, Path]] = []
    for old_rel, new_rel in rename_map.items():
        if old_rel == new_rel:
            continue
        old_path = corpus_root / old_rel
        temp_path = old_path.with_name(f"{old_path.name}.tmp-standardize")
        if temp_path.exists():
            raise RuntimeError(f"temporary path already exists: {temp_path}")
        old_path.rename(temp_path)
        staged_moves.append((temp_path, corpus_root / new_rel))

    for temp_path, new_path in staged_moves:
        new_path.parent.mkdir(parents=True, exist_ok=True)
        temp_path.rename(new_path)


def rewrite_corpus(root: Path, docs_dir: str) -> None:
    corpus_root = root / docs_dir
    manifest_path = corpus_root / "_manifest.txt"
    manifest_entries: list[tuple[str, str]] = []
    manifest_by_url: dict[str, str] = {}
    if manifest_path.exists():
        for line in manifest_path.read_text(encoding="utf-8").splitlines():
            if not line:
                continue
            url, rel = line.split("\t", 1)
            manifest_entries.append((url, rel))
            manifest_by_url[normalized_url_key(url)] = rel

    markdown_files = [
        path
        for path in sorted(corpus_root.rglob("*.md"))
        if path.name != "_manifest.txt"
    ]
    canonicalizer = (
        canonicalize_fortnite_path if docs_dir == "fortnite-docs-pages-canonical" else canonicalize_verse_api_path
    )

    rename_map: dict[str, str] = {}
    seen_targets: dict[str, str] = {}
    for path in markdown_files:
        rel = path.relative_to(corpus_root).as_posix()
        raw_md = path.read_text(encoding="utf-8")
        source_url = extract_source_url(raw_md.splitlines())
        new_rel = manifest_by_url.get(normalized_url_key(source_url)) or canonicalizer(source_url, rel)
        previous = seen_targets.get(new_rel)
        if previous is not None and previous != rel:
            raise RuntimeError(f"path collision in {docs_dir}: {previous} and {rel} -> {new_rel}")
        seen_targets[new_rel] = rel
        rename_map[rel] = new_rel

    rename_corpus_files(corpus_root, rename_map)

    for path in [
        path for path in sorted(corpus_root.rglob("*.md")) if path.name != "_manifest.txt"
    ]:
        original = path.read_text(encoding="utf-8")
        rewritten = rewrite_markdown(original)
        if rewritten and rewritten != original:
            path.write_text(rewritten, encoding="utf-8")

    if manifest_entries:
        rewrite_manifest(
            corpus_root,
            [
                (
                    url,
                    manifest_by_url.get(normalized_url_key(url))
                    or canonicalizer(url, rename_map.get(rel, rel)),
                )
                for url, rel in manifest_entries
            ],
        )

    cleanup_empty_directories(corpus_root)


def rewrite_canonical_docs(root: Path) -> None:
    rewrite_corpus(root, "verse-api-pages-canonical")
    rewrite_corpus(root, "fortnite-docs-pages-canonical")


if __name__ == "__main__":
    args = parse_args()
    root = repo_root()
    if args.rewrite_canonical_docs:
        rewrite_canonical_docs(root)
    if args.output:
        build_database(root, Path(args.output).resolve())

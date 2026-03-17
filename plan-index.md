## Adjusted plan for `query-docs` with the existing SQL index

### Project-aligned goal

The project should treat `query-docs` as the runtime query layer for the Verse/Fortnite documentation corpus that has already been canonicalized from `verse-api-pages-canonical/` and indexed into SQLite.

The important point is: **indexing `.md` -> SQLite should not become a primary runtime feature of the MCP server**. Indexing should remain an offline / maintenance pipeline, while MCP should only query the existing database and return concise results.

---

### Why the old plan needs adjustment

The old plan was correct about using **SQLite FTS5 + BM25 + snippets**, but it drifted out of scope in these ways:

1. it assumed the indexer had to be rebuilt from scratch in the runtime flow
2. it treated CLI commands like `index/search/get` as the primary deliverables
3. it made the MCP server responsible for data ingestion
4. it did not position `query-docs` as the main user-facing retrieval interface

Given the current project goals, the better direction is:

- **reuse the existing SQL index**
- **keep the MCP server small and focused**
- only keep ingest/reindex as internal tooling if data refresh is actually needed

---

### Proposed scope

#### Runtime scope

`query-docs` should do the following:

- accept a `query` in natural language or FTS-friendly form
- search the existing SQLite index
- rank results with BM25
- return:
  - relevant documentation excerpts
  - relevant Verse code examples
  - short path / title / snippet fields that are enough for agents to continue with
- limit result volume to avoid oversized dumps

#### Offline / maintenance scope

Only when data from `verse-api-pages-canonical/` needs to be refreshed should the system run:

- a canonical markdown parser
- upserts into the docs table
- FTS index sync
- incremental reindexing by `mtime` or hash

This **should not be on the MCP tool request path**.

---

### Better-fit architecture

```text
verse-api-pages-canonical/
        │
        ▼
offline importer / reindex job
        │
        ▼
SQLite docs index
        │
        ▼
`query-docs` MCP tool
        │
        ▼
ranked excerpts + code examples
```

This separation fits the repo goal better because it gives you:

- fast runtime behavior
- no reindex on server start
- easy crawler/canonicalizer changes without affecting the MCP layer
- a clean path to later combine docs retrieval with scanned asset retrieval

---

### What should be kept from the old plan

These parts of the old plan are still good:

- use **SQLite FTS5**
- use **BM25** for ranking
- use `snippet(...)` to generate short excerpts
- support **incremental reindexing** to avoid full rebuilds
- keep `raw_md` or an equivalent field if the original markdown needs to be rendered later

But these should be treated as **infrastructure for `query-docs`**, not as the main product by themselves.

---

### DB shape should follow `query-docs` needs

If the current DB is already sufficient for `query-docs`, **prefer keeping the existing schema**.
Only add fields when the tool actually needs more metadata to return better results.

A reasonable minimal schema for docs retrieval:

```sql
CREATE TABLE docs (
    id         INTEGER PRIMARY KEY,
    path       TEXT NOT NULL UNIQUE,
    title      TEXT NOT NULL,
    content    TEXT NOT NULL,
    raw_md     TEXT NOT NULL,
    tags       TEXT NOT NULL DEFAULT '[]',
    source_url TEXT NOT NULL DEFAULT '',
    doc_type   TEXT NOT NULL DEFAULT 'doc',   -- api | guide | example | doc
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
```

Notes:

- `doc_type` is useful if you want to prioritize `example` or `api`
- `source_url` is useful when the tool should return canonical links
- if the current DB is chunked by section instead of by file, that is still fine — there is no need to force it into this exact schema

---

### Search path for `query-docs`

The runtime logic should look roughly like this:

1. normalize the query
2. run FTS against the existing DB
3. rank by BM25
4. return short snippets, not full raw markdown by default
5. if needed, collect 1-2 code examples from the strongest matching docs

A suitable SQL skeleton:

```sql
SELECT d.path,
       d.title,
       d.doc_type,
       snippet(docs_fts, 1, '[', ']', '...', 24) AS snippet,
       bm25(docs_fts) AS score
FROM docs_fts
JOIN docs d ON d.id = docs_fts.rowid
WHERE docs_fts MATCH ?1
ORDER BY score
LIMIT ?2;
```

If you want to bias results toward Verse examples, you can add secondary sorting such as:

- prioritize `doc_type = 'example'`
- prioritize docs that contain code blocks
- prioritize paths under `verse-api-pages-canonical/versedotorg` or a specific namespace

---

### What should not be a priority right now

The following should not be prioritized in this plan:

- a public CLI like `cargo run -- index ./docs ./docs.db`
- a brand-new standalone indexer if the SQL index already exists
- any MCP request path that rebuilds the index automatically
- embedding/vector search as a first step if FTS5 is already sufficient for `query-docs`
- generic markdown search across unrelated non-Verse/Fortnite corpora

---

### Updated deliverables

#### Phase 1 — align with the current goal

- audit the current SQL schema used by `query-docs`
- confirm that the primary corpus is `verse-api-pages-canonical/`
- make `query-docs` read from the existing DB and return ranked excerpts
- keep output concise and useful for agents

#### Phase 2 — improve quality

- add needed metadata such as `doc_type`, `source_url`, and `tags`
- improve ranking across API pages vs examples vs guides
- improve snippet/citation selection

#### Phase 3 — optional maintenance

- add an internal command to reindex when canonical docs change
- support incremental updates by `mtime` or content hash
- use this only for data refresh, not as an end-user feature

---

### Acceptance criteria

The revised plan is on the right track if:

- `query-docs` is the main entry point for docs retrieval
- runtime behavior does not depend on reindexing when the server runs
- the implementation reuses the existing SQL index instead of building a parallel search system
- output is focused on excerpts + code examples + useful metadata
- indexing remains an internal maintenance pipeline

---

### Conclusion

This revision shifts the focus from **"building a new markdown indexing system"** to **"using the existing SQL index to make `query-docs` better"**.

That is a better match for the current project goal because it keeps:

- the MCP server small and clearly scoped
- retrieval practical for Verse docs
- the path open for later combining docs retrieval with map/device scan data instead of creating a separate index/search subsystem

---

## Current repo audit

### What already exists

- `README.md` already publicly declares `query-docs` as an MCP tool: `README.md:31`
- the project goal already emphasizes retrieval on top of docs/scan data: `plan.md:53`
- the local corpora already exist:
  - `verse-api-pages-canonical/` for Verse API pages
  - `fortnite-docs-pages-canonical/` for Fortnite guide/docs content, with `_manifest.txt`
- `fortnite-docs-pages-canonical/` contains many pages with real code snippets, for example `editable-properties-in-verse.md:102` and `debugging-and-troubleshooting-in-verse.md:96`

### What is still missing

- there is no Rust implementation of `query-docs` yet in `mcp_server/`
- `mcp_server/src/main.rs:181` currently only exposes `scan_map_devices`
- there is no visible SQLite dependency yet in the workspace or `mcp_server/Cargo.toml`
- there is no tracked docs index DB file in the repo; the only DB currently visible is `.beads/beads.db`, which is unrelated to MCP docs retrieval

### Audit conclusion

The real current state is:

1. **the product contract already exists** (`README.md` says `query-docs` exists)
2. **the local data corpora already exist** (`verse-api-pages-canonical/`, `fortnite-docs-pages-canonical/`)
3. **the runtime implementation does not exist yet**
4. **the docs SQL index may currently be assumed or external**, because it is not clearly present in the current codebase

So the correct engineering task right now is not to “tune an existing running implementation”, but to:

- confirm the intended DB path/configuration
- implement `query-docs` runtime behavior in the Rust server
- only add internal ingest/reindex support if there is truly no external pipeline already doing that

---

## Concrete technical tasks for `mcp_server`

### Phase 1 — wire `query-docs` runtime into the server

#### Task 1. Add the tool contract
- add the `query-docs` tool to `mcp_server/src/main.rs`
- minimum input schema:
  - required `query: string`
  - optional `limit: integer`, clamped to a small value
- update the `get_info()` instruction string if needed
- make sure `list_tools()` returns both `scan_map_devices` and `query-docs`

#### Task 2. Add a docs query service layer
- create a dedicated runtime module for docs retrieval, either inside `mcp_server/src/tools.rs` or as a new `mcp_server/src/docs_query.rs`
- module responsibilities:
  - open the SQLite DB in read-only mode
  - normalize the input query
  - run the FTS query
  - map rows into a stable result struct

#### Task 3. Define the result shape
- output should be concise and machine-friendly, and should not dump full markdown by default
- proposed shape:

```json
{
  "query": "editable properties",
  "results": [
    {
      "title": "Editable Properties",
      "path": "fortnite-docs-pages-canonical/editable-properties-in-verse.md",
      "source_url": "https://dev.epicgames.com/...",
      "doc_type": "guide",
      "snippet": "... @editable ... BasicInt:int = 0 ...",
      "score": 0.123
    }
  ]
}
```

#### Task 4. Add configuration for DB location
- do not hardcode the DB path into the business logic
- choose one simple mechanism:
  - an env var like `VERSE_DOCS_DB`
  - a server CLI arg like `--docs-db <path>`
- if the config is missing, the tool should return a clear error instead of a vague fallback

---

### Phase 2 — ranking and corpus shaping

#### Task 5. Distinguish corpus/source kind
- support at least the following metadata if the DB provides it:
  - `doc_type`
  - `source_url`
  - `path`
- if the DB does not yet have `doc_type`, derive it temporarily from the path prefix:
  - `verse-api-pages-canonical/` -> `api`
  - `fortnite-docs-pages-canonical/` -> `guide`

#### Task 6. Improve snippet quality
- prefer short, readable snippets
- do not return the full page body
- if a match comes from a long page, the snippet must include enough local context around the matched term

#### Task 7. Prefer pages with real code examples
- if the DB/index does not yet have explicit metadata, you can later add an offline flag like `has_code_block`
- but this is a quality improvement, not required for the first phase

---

### Phase 3 — internal maintenance only

#### Task 8. Decide whether the importer belongs in the repo
- if the SQL index is truly built elsewhere, only document the expected schema and DB path
- if no importer exists anywhere, add a private internal command to build the DB from the canonical folders
- this command does not need to be exposed as an MCP tool

#### Task 9. Internal reindex command
- if needed, add an internal CLI/subcommand to:
  - ingest `verse-api-pages-canonical/`
  - ingest `fortnite-docs-pages-canonical/`
  - upsert docs rows
  - rebuild/incrementally sync FTS
- this belongs on the maintenance path, not the request path

---

## File-level implementation map

### `mcp_server/src/main.rs`
Update this file to:
- add the tool schema for `query-docs`
- expose the tool in `list_tools()`
- handle the new branch in `call_tool()`

### `mcp_server/src/tools.rs`
Keep it here if you want the smallest diff:
- add docs query helpers
- add search result structs

Or split into a new module if you want clearer code organization:
- `mcp_server/src/docs_query.rs`

### `mcp_server/Cargo.toml`
This may need:
- `rusqlite`
- optionally the `bundled` feature for more stable builds

### `README.md`
Only update this if the real implementation differs from the current public contract.
If implementation follows this plan, `README.md` is already mostly correct.

---

## Technical acceptance criteria

Phase 1 is complete when all of the following are true:

- `list_tools()` shows `query-docs`
- `call_tool()` successfully handles `query-docs`
- the tool reads from a configured SQLite DB
- the query returns top-N results with `title`, `path`, `snippet`, and `score`
- missing DB config or invalid queries return clear errors
- there is no reindexing on the request path

---

## Practical decisions to lock in

If the goal is to move quickly while staying in scope, I recommend locking in these decisions:

1. runtime only does **read/search on SQLite**
2. the phase-1 corpus includes both:
   - `verse-api-pages-canonical/`
   - `fortnite-docs-pages-canonical/`
3. implement `query-docs` first
4. defer importer/reindex work unless the repo truly has no external pipeline

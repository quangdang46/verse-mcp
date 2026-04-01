# Verse UEFN MCP Server

> MCP server for Fortnite UEFN that scans placed devices from `.uasset` files.

[UEFN](https://dev.epicgames.com/community/fortnite/getting-started) | [Verse](https://dev.epicgames.com/documentation/en-us/fortnite/verse-language-quick-reference) | [MCP](https://modelcontextprotocol.io/) | [Rust](https://www.rust-lang.org/)

---

## Why This Exists

I'm transitioning from **web developer → game developer** and hit the same wall most Verse beginners hit: the syntax is fine, but integrating Verse with UEFN is where things get painful.

The specific friction points:

- **Properties and settings** live in the UEFN Details panel — not in code, not in docs
- **Device options** aren't always controllable from Verse, and it's not obvious which ones are
- **Game UI** — configuring it, wiring it to players, updating it correctly in multiplayer — is easy to get wrong in subtle ways
- **AI assistants hallucinate** Verse API names constantly, because training data for Verse is sparse

I looked for existing solutions — something like a "Verse MCP for UEFN" — and found nothing that was:

- **Project-aware** (able to read your actual code and digest files)
- **Digest-grounded** (using the managed Verse digest set as a source of truth)
- **Focused on the hard parts**: UI wiring, `@editable` properties, multiplayer patterns

So I built this.

---

## What It Solves

### 1. "Does this API actually exist?"

The MCP lets the AI or user guess naturally, then grounds that guess against your **managed digest files** and:

- Confirms whether a symbol is real
- Returns its actual signature
- Suggests likely matches for partial, approximate, or natural-language queries
- Prevents hallucinated method names before they reach your code

### 2. "What `@editable` fields do I have, and where do I set them in UEFN?"

The MCP parses your `.verse` source files and:

- Lists all `@editable` fields in your project
- Generates a wiring checklist (Details panel → reference/value assignment)
- Flags common mistakes: unused editables, unchecked optionals, missing null guards

### 3. "How do I write UI correctly — especially for multiplayer?"

The MCP provides UI scaffolding based on proven Verse patterns:

- Canvas and widget boilerplate (pure Verse UI, no UMG)
- Per-player widget storage patterns (prevents UI overlap and state bleed between players)
- Clean update/remove templates with correct lifecycle handling

### 4. "What is this device called in Verse?"

If I want to use the TRACK DUMMY device, I need to know its device name in Verse, but in Verse, the device name isn't the same as "TRACK DUMMY."
So, how can I find the name that's used for this device in Verse?

---

## What it does

This project reads UEFN map assets directly from:

- `Content/__ExternalActors__/**/*.uasset`
- `Content/__ExternalObjects__/**/*.uasset`

It extracts placed-device information such as:

- device type
- label
- triggers
- receivers
- configured settings

---

## MCP Tools

| Tool | Input | Output |
|---|---|---|
| `scan_map_devices` | `project_path`, `force_refresh` (optional) | Full placed-device scan output grouped by type |
| `query-docs` | `query`, `limit` (optional), `offset` (optional), `fetch_source_urls` (optional), `max_fetches` (optional) | Ranked Verse docs with full indexed content by default, plus optional fetched source page content |
| `fetch-doc-source` | `url` | Fetch and normalize one returned documentation URL into agent-friendly text and JSON metadata |
| `reload-project-metadata` | `project_path` | Drops cached project scan metadata so the next scan rebuilds without restarting the MCP server |

### `query-docs` implementation notes

Retrieves ranked Verse documentation matches from the packaged SQLite index.

Output:
- full indexed content for each returned match by default
- snippet preview and ranking metadata
- pagination metadata (`limit`, `offset`, `has_more`, `next_offset`)
- optional fetched `source_url` page content when requested

Use `fetch-doc-source` when you want a second-step fetch of one returned `source_url` without increasing the default `query-docs` payload.

This tool is Verse-only for this project, so the `query` should focus on Verse or Fortnite UEFN topics.

Important constraints:
- Do not call this tool more than 3 times per question. If you cannot find what you need after 3 calls, use the best information you have.
- `query` should be specific and include relevant details.
- Do not include sensitive or confidential information in the query, including API keys, passwords, credentials, personal data, or proprietary code.

Examples of good queries for this project:
- `Find the latest Verse language documentation for editable properties and show examples of @editable usage.`
- `Find Fortnite UEFN documentation about device events and listenable patterns with current code examples.`

Examples of bad queries:
- `verse`
- `uefn docs`

### `scan_map_devices` policy notes

`scan_map_devices` now reports the scan-policy decision in structured output. The
policy layer explicitly explains whether the server reused cached scan metadata,
forced a fresh scan, refreshed because `.uasset` mtimes changed, or denied the
request because the provided path is not a scanable UEFN project root.

If you need to invalidate a project's cached scan state without restarting `vm`,
call `reload-project-metadata` first and then run `scan_map_devices` again.

---

## Tech Stack

- **Rust** — MCP server + `.uasset` binary parser
- **rmcp** — MCP protocol implementation with stdio and HTTP (SSE) transport
- **rayon** — parallel scanning
- **tokio / tokio-util** — async runtime and cancellation handling
- **unreal_asset** — UE asset parsing support

---

## Installation

```bash
curl -fsSL "https://raw.githubusercontent.com/quangdang46/verse-mcp/main/install.sh?$(date +%s)" | bash
```

- Default install dir: `$HOME/.local/bin`
- If `vm` is not on PATH:

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc && source ~/.bashrc
```

### Verify installation

```bash
vm --version
```

---

## Usage

### CLI options

```bash
# Show help
vm --help

# Run with stdio (default)
vm

# Run with HTTP transport
vm --transport http --host 127.0.0.1 --port 2003

# Use custom port
vm --transport http --port 8080
```

`query-docs` works out of the box after installing `vm`. The docs index is built into the binary, and `vm` creates a private cached copy automatically on first docs query.

### MCP config

```json
{
  "mcpServers": {
    "verse-mcp": {
      "command": "vm"
    }
  }
}
```

---

## AI usage examples

### Scan a map

```
User: What devices are in my map?
AI: [Uses scan_map_devices]
I found 47 devices across 15 types.
```

---

## Project status

### Current shipped features
- [x] `.uasset` parser for placed devices
- [x] scans both `__ExternalActors__` and `__ExternalObjects__`
- [x] parallel scanning with rayon
- [x] MCP server with stdio and HTTP (SSE) transport
- [x] `scan_map_devices` with cache invalidation
- [x] `query-docs` runtime retrieval from a built-in SQLite docs index

### Current focus
- improve scan summaries for large maps
- improve semantic retrieval on top of scanned assets and documentation
- add higher-level map analysis without bringing back exact-match digest tooling

---

## Background: the `.uasset` discovery

UEFN stores placed actor state in binary `.uasset` files under `Content/__ExternalActors__` and `Content/__ExternalObjects__`. Those files contain the actual map configuration, which makes them more useful than static API docs for map-aware tooling.

The parser reads the UE5 asset binary format directly, extracts names and property data, and classifies them into device types, triggers, receivers, and settings.

---

## Contributing

If you're building UEFN tooling and want to improve scan quality or map-aware analysis, issues and PRs are welcome.

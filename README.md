# Verse UEFN MCP Server

> MCP server for Fortnite UEFN that scans placed devices from `.uasset` files.

[UEFN](https://dev.epicgames.com/community/fortnite/getting-started) | [Verse](https://dev.epicgames.com/documentation/en-us/fortnite/verse-language-quick-reference) | [MCP](https://modelcontextprotocol.io/) | [Rust](https://www.rust-lang.org/)

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
| `query-docs` | `query` | Relevant Verse documentation excerpts and Verse code examples matching the query |

### `query-docs` implementation notes

Retrieves relevant Verse documentation excerpts and Verse code examples matching the query.

Output:
- matching documentation excerpts
- relevant Verse code examples
- concise results focused on the requested topic

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

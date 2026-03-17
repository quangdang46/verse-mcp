# Verse UEFN MCP Server

> MCP server for Fortnite UEFN that scans placed devices from `.uasset` files and generates connection graphs.

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

It also turns scanned output into a Mermaid or DOT connection graph.

---

## MCP Tools

| Tool | Input | Output |
|---|---|---|
| `scan_map_devices` | `project_path`, `force_refresh` (optional) | Full placed-device scan output grouped by type |
| `generate_device_graph` | `project_path`, `format` (optional) | Mermaid or DOT device connection graph |

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

### Render the connection graph

```
User: Show me the device graph.
AI: [Uses generate_device_graph]
I generated a Mermaid graph for the scanned devices.
```

---

## Project status

### Current shipped features
- [x] `.uasset` parser for placed devices
- [x] scans both `__ExternalActors__` and `__ExternalObjects__`
- [x] parallel scanning with rayon
- [x] MCP server with stdio and HTTP (SSE) transport
- [x] `scan_map_devices` with cache invalidation
- [x] `generate_device_graph` with Mermaid and DOT output

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

If you're building UEFN tooling and want to improve scan quality, graphing, or map-aware analysis, issues and PRs are welcome.

# verse-mcp — Current Direction

> Goal: keep Verse MCP focused on scanning placed UEFN devices and visualizing map connections.

---

## Current product scope

The project now centers on two runtime capabilities:

1. **Rust CLI scanner** — scan `.uasset` files and emit structured device data
2. **Rust MCP server** — expose scan and graph tools over stdio or HTTP transport

---

## Active MCP tools

| Tool | Description |
|------|-------------|
| `scan_map_devices` | Scan a UEFN project for placed devices |
| `generate_device_graph` | Generate Mermaid or DOT graph output from scanned devices |

---

## Architecture

```text
verse-mcp/
├── Cargo.toml
├── uasset_scan/        ← parse .uasset → DeviceInfo / ScanOutput
├── uefn_scan_cli/      ← CLI binary
└── mcp_server/         ← MCP server
```

---

## Core logic

### Scanning
- walk `Content/__ExternalActors__` and `Content/__ExternalObjects__`
- parse `.uasset` files in parallel
- extract device type, label, triggers, receivers, and settings
- group by type into `ScanOutput`

### Graph generation
- derive channel-based connections from scanned settings
- render Mermaid or Graphviz DOT output

---

## Current priorities

### 1. Improve scan output ergonomics
- add smaller summaries for large maps
- avoid forcing agents to read giant scan blobs for simple counts
- keep full scan data available for debugging and deep analysis

### 2. Add semantic retrieval on top of scan/docs data
- move away from exact-match-first workflows
- support natural-language map understanding
- ground semantic retrieval against real scanned assets

### 3. Keep the server small and focused
- preserve scan correctness
- preserve graph generation
- avoid bringing back removed digest/template/validation features unless a real use case appears

---

## Deliverables

### Shipped
- [x] scanner CLI with JSON output
- [x] `.uasset` parsing and classification
- [x] MCP server with stdio and HTTP transport
- [x] scan caching
- [x] device graph generation

### Next
- [ ] scan summary tool or top-level summary fields
- [ ] semantic asset search / embedding-based retrieval
- [ ] better handling for large volumes of `Unknown` assets

---

## References

- [MCP Spec](https://modelcontextprotocol.io/specification/2025-06-18/basic/transports)
- [rmcp SDK](https://github.com/modelcontextprotocol/rust-sdk)
- [unreal_asset](https://docs.rs/unreal_asset)

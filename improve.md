# Verse MCP Improvements

## Completed

### 1. Binary Rename
- Renamed `verse-mcp` → `vm`
- Updated `mcp_server/Cargo.toml` binary name

### 2. HTTP Transport Support
- Added `transport-sse-server` feature to rmcp
- Added `tokio` signal feature, `tokio-util` dependency
- Added `axum` dependency for HTTP server
- Added CLI args:
  - `--transport {stdio|http}` (default: stdio)
  - `--host HOST` (default: 127.0.0.1)
  - `--port PORT` (default: 2003)
- Updated main.rs to support SSE server transport via `rmcp::transport::SseServer`

### 3. Scanner Updates
- Updated `scan_project()` to scan both:
  - `Content/__ExternalActors__/**`
  - `Content/__ExternalObjects__/**`
- Updated `get_max_mtime()` to check both directories for cache invalidation

### 4. Client Config (HTTP)

```json
{
  "mcpServers": {
    "verse-mcp": {
      "type": "http",
      "url": "http://localhost:2003"
    }
  }
}
```

### 5. Usage

```bash
# Run with stdio (default)
vm

# Run with HTTP transport
vm --transport http --host 127.0.0.1 --port 2003

# Show help
vm --help
```


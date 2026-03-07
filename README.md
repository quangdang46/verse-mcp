# Verse UEFN MCP Server

> **MCP server for Fortnite UEFN & Verse development - scan devices, validate wiring, query Fortnite API digest, and scaffold UI**

[UEFN](https://dev.epicgames.com/community/unreal-engine-for-fortnite) | [Verse](https://dev.epicgames.com/community/documentation/verse) | [MCP](https://modelcontextprotocol.io/) | [Rust](https://www.rust-lang.org/)

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
- **Digest-grounded** (using `Fortnite.digest.verse` as a source of truth)
- **Focused on the hard parts**: UI wiring, `@editable` properties, multiplayer patterns

So I built this.

---

## What It Solves

### 1. "Does this API actually exist?"

Instead of letting the AI guess, the MCP reads your **digest files** and:

- Confirms whether a symbol is real
- Returns its actual signature
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

## How It Works

The MCP server indexes two sources of truth from your local machine:

| Source | What it provides |
|---|---|
| `Fortnite.digest.verse` | All device types, method signatures, event names |
| `__ExternalActors__/**/*.uasset` | Every placed device in your map with real property values |
| `__ExternalObjects__/**/*.uasset` | Every placed device in your map with real property values |
| `*.verse` in your project | Your own `@editable` fields, modules, and code context |

The `.uasset` parser reads binary files directly — no external tools required. It extracts:

- Device type (e.g. `Device_Campfire_C`)
- Triggers (e.g. `TriggerOnEnterRadius`, `OnDisabled`)
- Receivers (e.g. `ReceiverAddFuel`, `DisableWhenReceiving`)
- Configured settings with real values (e.g. `HealthPerPulse: 2.0`, `StartLit: False`)

---

## MCP Tools

| Tool | Input | Output |
|---|---|---|
| `scan_map_devices` | project_path, force_refresh (optional) | All placed devices with triggers, receivers, settings |
| `get_device_props` | Device type name | Full property and event list from digest |
| `query_digest` | Symbol name or keyword | Matching entries from digest with signatures |
| `validate_wiring` | project_path | Wiring issues (orphaned channels, conflicts) |
| `validate_verse` | Verse source code | Validation issues and hallucinated APIs |
| `generate_device_graph` | project_path, format (optional) | Device connection graph (Mermaid/DOT) |
| `diff_digests` | Old/new digest content | API changes between digest versions |
| `list_templates` | - | Saved composition templates |
| `load_template` | Template name | Load a template by name |
| `save_template` | Template JSON/from scan | Save a composition template |
| `delete_template` | Template name | Delete a template |

---

## Tech Stack

- **Rust** — MCP server + `.uasset` binary parser (fast, single binary)
- **rmcp** — MCP protocol implementation with stdio and HTTP (SSE) transport
- **rayon** — parallel scanning
- **axum** — HTTP server for SSE transport
- **tokio-util** — Async runtime utilities
- **Fortnite.digest.verse** — source of truth for all device/API definitions

---

## Installation

### Cargo Install (Recommended)

```bash
cargo install --git https://github.com/quangdang46/verse-mcp.git
```

This will:
- Automatically download and compile the project
- Install the `vm` binary to `~/.cargo/bin` (Linux/macOS) or `%USERPROFILE%\.cargo\bin` (Windows)

**Note:** First-time install requires [Rust](https://rustup.rs/).

### Add Cargo bin to PATH

If `vm` command is not found, add Cargo bin to PATH:

**Windows (PowerShell):**
```powershell
$env:Path += ";$env:USERPROFILE\.cargo\bin"
```

**Linux / macOS:**
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Verify Installation

```bash
vm --version
```

Should output: `vm 0.1.0`

### Prebuilt binaries

Windows (PowerShell):

```powershell
# Cache-busted to avoid stale script in some environments
irm "https://raw.githubusercontent.com/quangdang46/verse-mcp/main/install.ps1?ts=$(Get-Random)" | iex
# Specific version
$env:VERSION = "v0.1.0"; irm "https://raw.githubusercontent.com/quangdang46/verse-mcp/main/install.ps1?ts=$(Get-Random)" | iex
```

- Default install dir: `%USERPROFILE%\.local\bin`
- Add to PATH for current session:

```powershell
$env:Path += ";$env:USERPROFILE\.local\bin"
```

macOS / Linux:

```bash
curl -fsSL "https://raw.githubusercontent.com/quangdang46/verse-mcp/main/install.sh" | bash
# Specific version and custom dir
INSTALL_DIR=/usr/local/bin VERSION=v0.1.0 curl -fsSL "https://raw.githubusercontent.com/quangdang46/verse-mcp/main/install.sh" | bash
```

- Default install dir: `$HOME/.local/bin` (no sudo required)
- If not on PATH, add to your shell profile:

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc && source ~/.bashrc
```

---

## Usage

### CLI Options

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

---

## Configuration

### HTTP Transport (Claude Desktop / Cursor)

Add to your MCP client config file:

**Cursor IDE , Claude Code** (`mcp.json`):

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

## Project Status

### Phase 1 — Complete ✅
- [x] `.uasset` binary parser with full property extraction
- [x] Device classification and fingerprinting system
- [x] Parallel scanning with rayon
- [x] Tested against real UEFN project files
- [x] Wiring validator for connection analysis
- [x] Scans both `__ExternalActors__` and `__ExternalObjects__` directories

### Phase 2 — Complete ✅
- [x] MCP server with stdio and HTTP (SSE) transport
- [x] `scan_map_devices` tool with mtime-based caching
- [x] `validate_wiring` tool for connection validation
- [x] Claude Desktop and Cursor config examples

### Phase 3 — Complete ✅
- [x] Digest parser for Fortnite.digest.verse
- [x] `get_device_props` tool (digest lookup)
- [x] `query_digest` tool (symbol search)
- [ ] `list_editables` tool (planned)
- [ ] `scaffold_ui` tool (planned)

### Phase 4 — Complete ✅
- [x] Installation scripts (install.sh, install.ps1)
- [x] Release workflow with multi-platform builds
- [x] Pre-built binary releases on GitHub
- [ ] Integration tests with real UEFN projects
- [ ] Documentation website

---

## Background: The `.uasset` Discovery

One of the key findings during research: UEFN stores placed device state in `Content/__ExternalActors__/**/*.uasset` and `Content/__ExternalObjects__/**/*.uasset` — binary files, one per placed actor. These files contain the **actual runtime configuration** of every device in your map, including all receiver/trigger channel assignments and editable property values.

This is more useful than the digest alone, because:

- The digest has type signatures — the `.uasset` files have **real values**
- The digest is static — the `.uasset` files **update every time you edit in UEFN**
- The digest has no placement context — the `.uasset` files know exactly **what's in your map**

The parser reads the UE5 asset binary format directly: verifies the magic header (`0x9E2A83C1`), locates the Name Map, reads all string entries, and classifies them by pattern into triggers, receivers, and settings — then extracts key/value pairs from the `PropertyOverrideData` block using a fixed 25-byte metadata stride.

---

## AI Usage Examples

### Finding Device Types

```
User: What devices are in my map?
AI: [Uses scan_map_devices tool]
I found 47 devices in your map across 15 types:
- Button_device: 5
- player_spawner_device: 3
- campfire_device: 2
- ...
```

### Validating Wiring

```
User: Are all my buttons connected properly?
AI: [Uses validate_wiring tool]
I found 2 wiring issues:
1. Button1 has OnTriggered but no channel configured
2. Channel "game_start" has 3 senders (potential conflict)
```

### Querying Device Properties

```
User: What events does the campfire device have?
AI: [Uses get_device_props tool]
The device_campfire_device has:
- Triggers: TriggerOnEnterRadius, OnDisabled, OnEnabled
- Receivers: ReceiverAddFuel, EnableWhenReceiving
- Methods: AddFuel, Extinguish, Light
```

### Searching the Digest

```
User: How do I use the score manager?
AI: [Uses query_digest tool with query="score"]
Found device_score_manager_device with:
- ScoreChanged event
- AddScore, GetScore, ResetScore methods
```

### Debugging Multiplayer UI

```
User: My timer display isn't syncing between players
AI: Let me check your canvas setup... [Uses scan_map_devices]
The issue is your timer canvas uses a single shared widget instead of per-player storage.
Try using `canvas.SetMyWidget[player] = value` pattern instead.
```

---

## Roadmap

**Phase 1 — Core** ✅ COMPLETE
- [x] `.uasset` binary parser with full property extraction
- [x] Device classification and fingerprinting
- [x] Parallel scanning with rayon
- [x] Scans both `__ExternalActors__` and `__ExternalObjects__` directories

**Phase 2 — MCP Server** ✅ COMPLETE
- [x] MCP server with stdio and HTTP (SSE) transport
- [x] `scan_map_devices` tool with caching
- [x] `validate_wiring` tool
- [x] Claude Desktop / Cursor config examples

**Phase 3 — Digest Integration** ✅ COMPLETE
- [x] Digest parser for Fortnite.digest.verse
- [x] `get_device_props` tool
- [x] `query_digest` tool
- [ ] `list_editables` tool
- [ ] `scaffold_ui` tool

**Phase 4 — Polish**
- [ ] Pre-built binary releases
- [ ] Integration tests with real UEFN projects
- [ ] Documentation website

---

## Contributing

This is early-stage. If you're also building in UEFN and hitting the same friction, issues and PRs are welcome.

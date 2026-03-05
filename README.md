# Verse UEFN MCP

> **A project-aware MCP server for Verse/UEFN development**

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
| `__ExternalActors__/*.uasset` | Every placed device in your map with real property values |
| `*.verse` in your project | Your own `@editable` fields, modules, and code context |

The `.uasset` parser reads binary files directly — no external tools required. It extracts:

- Device type (e.g. `Device_Campfire_C`)
- Triggers (e.g. `TriggerOnEnterRadius`, `OnDisabled`)
- Receivers (e.g. `ReceiverAddFuel`, `DisableWhenReceiving`)
- Configured settings with real values (e.g. `HealthPerPulse: 2.0`, `StartLit: False`)

---

## MCP Tools (Phase 1)

| Tool | Input | Output |
|---|---|---|
| `scan_map_devices` | Project path | All placed devices with triggers, receivers, settings |
| `get_device_props` | Device type name | Full property and event list |
| `query_digest` | Symbol name or keyword | Matching entries from digest with signatures |
| `list_editables` | Project path | All `@editable` fields with wiring checklist |
| `scaffold_ui` | UI intent (e.g. "round timer HUD") | Valid Verse UI code using canvas/text_block patterns |

---

## Tech Stack

- **Rust** — MCP server + `.uasset` binary parser (fast, single binary)
- **rmcp** — MCP protocol implementation (stdio transport)
- **rayon** — parallel scanning
- **Fortnite.digest.verse** — source of truth for all device/API definitions

---

## Project Status

### Phase 1 — Complete ✅
- [x] `.uasset` binary parser with full property extraction
- [x] Device classification and fingerprinting system
- [x] Parallel scanning with rayon
- [x] Tested against real UEFN project files
- [x] Wiring validator for connection analysis

### Phase 2 — In Progress 🚧
- [x] MCP server with stdio transport (rmcp)
- [x] `scan_map_devices` tool with mtime-based caching
- [x] `validate_wiring` tool for connection validation
- [ ] `get_device_props` tool (Phase 3)
- [ ] `query_digest` tool (Phase 3)
- [ ] `list_editables` tool
- [ ] `scaffold_ui` tool
- [ ] Claude Desktop and Cursor config examples

## Installation

### From Release (Recommended)

Download the latest release for your platform:
- Windows: `verse-mcp-x86_64-pc-windows-msvc.zip`
- macOS: `verse-mcp-aarch64-apple-darwin.tar.gz`
- Linux: `verse-mcp-x86_64-unknown-linux-gnu.tar.gz`

Extract and add to PATH.

### From Source

```bash
cargo install --path .
cargo build --release
```

The binary will be at `target/release/verse-mcp`.

## Configuration

### Claude Desktop

Add to your Claude Desktop config file:
- **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "verse": {
      "command": "/path/to/verse-mcp",
      "env": {
        "VERSE_PROJECT_PATH": "/path/to/your/uefn/project"
      }
    }
  }
}
```

### Cursor IDE

Add to `~/.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "verse": {
      "command": "/path/to/verse-mcp"
    }
  }
}
```

Set the `VERSE_PROJECT_PATH` environment variable to specify your UEFN project path.

## MCP Tools

| Tool | Description | Status |
|------|-------------|-------|
| `scan_map_devices` | Scan UEFN project for all placed devices | ✅ Ready |
| `validate_wiring` | Validate device wiring for issues | ✅ Ready |
| `get_device_props` | Get device properties from digest | 🚧 Phase 3 |
| `query_digest` | Search digest for symbols | 🚧 Phase 3 |
| `list_editables` | Find @editable fields in .verse files | 📝 Planned |
| `scaffold_ui` | Generate Verse UI scaffolding code | 📝 Planned |

---

## Background: The `.uasset` Discovery

One of the key findings during research: UEFN stores placed device state in `Content/__ExternalActors__/**/*.uasset` — binary files, one per placed actor. These files contain the **actual runtime configuration** of every device in your map, including all receiver/trigger channel assignments and editable property values.

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

**Phase 2 — MCP Server** ✅ COMPLETE
- [x] MCP server with stdio transport (rmcp)
- [x] `scan_map_devices` tool with caching
- [x] `validate_wiring` tool
- [x] Claude Desktop / Cursor config examples

**Phase 3 — Generation** 🚧 IN PROGRESS
- [ ] `get_device_props` tool (digest lookup)
- [ ] `query_digest` tool (symbol search)
- [ ] `list_editables` tool
- [ ] `scaffold_ui` tool

**Phase 4 — Polish**
- [ ] Pre-built binary releases
- [ ] Integration tests with real UEFN projects
- [ ] Documentation website

---

## Contributing

This is early-stage. If you're also building in UEFN and hitting the same friction, issues and PRs are welcome.

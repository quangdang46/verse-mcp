I’m going to assume “plan rust” = **plan to port your current Node `.uasset` scanner + (future) MCP server to Rust**, keeping the same outputs and then improving accuracy (fingerprinting, digest-grounding). This is based on the files you shared (POC + current scanner + sample output).    

## Target end-state

A Rust workspace that gives you:

1. **A Rust CLI scanner** (drop-in replacement for `scan-verse.js`) that:

   * Walks `Content/__ExternalActors__/**.uasset`
   * Parses UE5 `.uasset` binaries (same heuristics you proved work)
   * Emits the same JSON structure you already have (`scanned_at`, `by_type`, etc.)

2. **A Rust MCP server** using the **official Rust MCP SDK (`rmcp`)**:

   * stdio transport (newline-delimited JSON-RPC)
   * tools like `scan_map_devices`, `query_digest`, etc.
   * logs to `stderr` only (spec requirement for stdio servers) ([modelcontextprotocol.io][1])

---

## Why Rust is a good fit for your stack

* Your parser is **byte-level**, needs **tight control over offsets**, and benefits from **safe slicing** + explicit error handling.
* MCP in Rust is now well-supported via the **official Rust SDK (`rmcp`)** ([GitHub][2])
* You can ship **one static-ish binary** per platform (CLI + server), which is nice for onboarding.

---

## Suggested repo layout (Rust workspace)

```
verse-uefn-mcp/
  Cargo.toml                (workspace)
  crates/
    uasset_scan/            (library: parse .uasset -> DeviceInfo)
    uefn_scan_cli/          (binary: CLI wrapper, outputs JSON)
    verse_digest/           (library: parse Fortnite.digest.verse into index)
    verse_editables/        (library: parse *.verse for @editable)
    mcp_server/             (binary: rmcp stdio server exposing tools)
```

This lets you reuse parsing logic in both CLI + MCP server cleanly.

---

## Phase plan (practical, parity-first)

### Phase 1 — Port the scanner to Rust (parity with `scan-verse.js`)

Goal: replicate the current behavior exactly, then iterate.

**Core components to port from your Node script** :

* `readFString(buf, off)`

  * `i32 len` LE
  * `len == 0` => empty
  * `len < 0` => UTF-16LE, `(-len) * 2` byt - `len > 0` => UTF-8, length includes null terminator; validate last byte = `0`

* `readNameMap(buf)`

  * probe offsets ~`100..2000`
  * look for first FString that starts with `/Script/` or `/Game/`
  * confirm “run” of 5+ plausible consecutive FStrings
  * then read strings until invalid, skipping the trailing 4-byte hash per entry

* `extractLabel(buf)`

  * search needle bytes for `FString("ActorLabel")`
  * read the next FString as label

* `extractConfigValues(buf, scanStart)`

  * scan start = `max(name_map_end, file_len/2)`
  * interpret: `FString(key) + 25-byte metadata + FString(value)`
  * apply strict filters

**Rust implementation sketch**

* Use a small `Cursor` wrapper (or just index math) with:

  * `read_i32_le(off) -> Option<i32>`
  * `read_u32_le(off) -> Option<u32>`
  * `read_fstring(off) -> Option<(String, next_off)>`
* Keep everything on `&[u8]` slices; no unsafe needed.

**Dependencies**

* `walkdir` (directory walk)
* `clap` (CLI)
* `serde`, `serde_json` (output)
* `anyhow` or `thiserror` (errors)
* optional: `rayon` for parallel file parsing

**Definition of done**

* Running Rust scanner on the same project produces JSON matching your current schema and close-enough content to validate quickly against expectations.

---

### Phase 2 — Fix the two issues visible in your `result.json`

Your current `scan-verse.js v4` output shows two real-world pain points:

1. **“Unknown” deviceType for devices that you already know how to fingerprint**
   Your POC doc explicitly calls out fingerprinting for `button_device`, `player_spawner_device`, and Island Settings.
   But `result.json` still has many `Unknown` devices.

**Rust plan**

* Implement the fingeribed:

  * `InteractionRadius + InteractTime + TriggerS:contentReference[oaicite:11]{index=11}e`
  * `UseAsIslandStart + PlayerTeam` → `player_spawner_device`
  * `NPCCharacterDefinition + DespawnAIsWhenDisabled` → `character_spawner_device`
  * `MaxHealth + SpawnLocation + Teams` → `island_settings_device`
* Scoring:

  * count matched keys
  * require ≥ 2 hits
  * pick best score; tie-break with “more unique keys” or a priority order

2. **Trigger/receiver noise (components showing up as triggers)**
   Your v4 method (“if it has `_GEN_VARIABLE` twin then it’s a Verse event”) appears to be too permissive in practice (Blueprint variables/components can also have generated twins), leading to triggers like `ButtonMesh`, `ToyOptionsComponent`, etc. in `result.json`.

**Rust plan (two-stage filter)**

* Stage A (cheap heuristic): only keep names matching likely event patterns, e*`, `Trigger*`, `When*`

  * receivers: `Receiver*`, `*WhenReceiving`, plus any other patterns you’ve observed in real assets
* Stage B (correct filter): once digest integration exists (Phase 3), keep only events confirmed in `Fortnite.digest.verse`.

---

### Phase 3 — Add digest-grounding (matches your README intent)

Your README’s core promise is “digest-grounded” APIs to prevent hallucinations.

**Rust approach**

* Create a `verse_digest` crate that builds an index:

  * device type → events, methods, fielsignature + location
* You don’t need a full Verse parser to start:

  * a line-based / token-ish parser can get you very far for digest files
  * later you can swap in tree-sitter if Verse grammar support becomes worth it

**What digest-grounding unlocks**

* Correct trigger/receiver lists (no more component noise)
* Better classification of ambiguous names
* Real signatures for “does this method exist?”

---

### Phase 4 — Build the MCP server in Rust (stdio)

MCP uses JSON-RPC, and stdio is a standard transport; messages are newline-delimited and stdout must contain *only* protocol messages. ([modelcontextprotocol.io][1])

**Use the official Rust SDK**

* Official SDK repo: `modelcontextprotocol/rust-sdk` ([GitHub][2])
* Crate: `rmcp` (official Rust implementation) ([docs.rs][3])
* There are guides showing stdio MCP servers using `rmcp` with `transport-io`. ([Shuttle][4])

**MCP tools to implement (matching your roadmap)**
From your README (Phase 1 tools):

* `scan_map_devices(project_path)`

  * calls Rust scanner, returns structured JSON
  * add caching keyed by file mtimes or a content hash

* `get_device_props(device_type)`

  * returns digest-grounded events/properties (and optionally observed overrides)

* `query_digest(symbol_or_keyword)`

  * returns matching digest entries/signatures

* `list_editables(project_path)`

  * parses `*.verse` for `@urns wiring checklist

* `scaffold_ui(intent)`

  * returns Verse UI templates (digest-grounded types)

**Critical MCP implementation details**

* stdout: MCP JSON-RPC only
* stderr: logging only (safe)
* implement graceful shutdown (Ctrl-C), and consider a file watcher for cache invalidation.

---

## Testing strategy (so you don’t regress on UEFN updates)

### 1) Golden tests (schema + stability)

* Assert Rust output conforms to your JSON schema:

  * `scanned_at`, `total_files`, `total_devices`, `by_type`, etc.
* Maintain “golden” snapshots for a small fixture project.

### 2) Differential tests (Rust vs Node)

* Run both scanners on the same project path:

  * compare counts by type
  * compare label extraction success rate
  * compare top-N settings keys for known devices (campfire/button/spawner)

### 3) Digest cross-validation tests

* For each parsed device type:

  * `events_from_uasset ⊆ events_from_digest`
    fenders (helps you detect naming pattern shifts after UEFN updates)

---

## Concrete deliverables checklist

### MVP (fastest path)

* [ ] `uefn_scan_cli` that outputs JSON identical in structure to `scan-verse.js`
* [ ] `uasset_scan` library with the 4 core extraction steps (magic, name map, label, config)

### Quality pass

* [ ] Fingerprinting to eliminate most `Unknown`
* [ ] Event noise reduction (heuristics now, digest later)

### MCP

* [ ] `mcp_server` binary using `rmcp` stdio
* [ ] Tools: `scan_map_devices`, `query_digest` first (highest leverage)

---

## Links to the exact baseline you’re porting

* Current POC + binary format notes:
* Project goal + tool roadmap:
* Current scan output to measure against:
* Current implementation to port:

If you meant something different by “plan rust” (e.g., a Rust learning plan, or Rust-the-game), tell me the context and I’ll pivot—but this plan should map directly onto the code and artifacts you uploaded.



[1]: https://modelcontextprotocol.io/specification/2025-06-18/basic/transports?utm_source=chatgpt.com "Transports - Model Context Protocol"
[2]: https://github.com/modelcontextprotocol/rust-sdk?utm_source=chatgpt.com "GitHub - modelcontextprotocol/rust-sdk: The official Rust SDK for the ..."
[3]: https://docs.rs/rmcp?utm_source=chatgpt.com "rmcp - Rust - Docs.rs"
[4]: https://www.shuttle.dev/blog/2025/07/18/how-to-build-a-stdio-mcp-server-in-rust?utm_source=chatgpt.com "How to Build a stdio MCP Server in Rust - shuttle.dev"

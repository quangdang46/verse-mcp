# verse-mcp — Rust Port Plan

> **Goal:** Port Node.js `.uasset` scanner + MCP server to Rust with proper UE5 parsing.

---

## Target End-State

1. **Rust CLI scanner** — drop-in replacement for `scan-verse.js`
2. **Rust MCP server** — stdio transport with digest-grounded tools

---

## Critical Technical Findings

### Finding 1: Name Map — Use Header, Not Probing

| JS (Wrong) | Rust (Correct) |
|------------|----------------|
| Probe bytes 100-2000 for `/Script/` | Read `NameOffset`/`NameCount` from header |

### Finding 2: Properties — Tagged Format, Not Fixed Stride

| JS (Wrong) | Rust (Correct) |
|------------|----------------|
| Assumes 25-byte stride | UE uses tagged serialization |

### Finding 3: Existing Crate

`unreal_asset` crate supports UE5 with proper tagged parsing.

---

## Architecture

```
verse-mcp/
├── Cargo.toml
├── crates/
│   ├── uasset_scan/        ← parse .uasset → DeviceInfo
│   ├── uefn_scan_cli/      ← CLI binary
│   ├── verse_digest/       ← parse digest → index
│   └── mcp_server/         ← MCP stdio server
└── tests/
    └── fixtures/
```

---

## Dependencies

```toml
unreal_asset = "0.1"
walkdir = "2"
rayon = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
indexmap = "2"
tokio = { version = "1", features = ["io-std", "rt-multi-thread"] }
rmcp = "0.1"
clap = { version = "4", features = ["derive"] }
thiserror = "1"
anyhow = "1"
regex = "1"
```

---

## Core Logic

### Asset Open

```rust
pub fn open_asset(path: &Path) -> Result<Asset<File>> {
    for ver in [VER_UE5_3, VER_UE5_2, VER_UE5_1, VER_UE5_0] {
        if let Ok(asset) = Asset::new(File::open(path)?, None, ver) {
            return Ok(asset);
        }
    }
    bail!("Could not parse {}", path.display())
}
```

### Classification

```rust
pub fn classify(names: &[String]) -> Classification {
    let gen_set: HashSet<&str> = names.iter()
        .filter(|n| n.ends_with("_GEN_VARIABLE"))
        .map(|n| &n[..n.len() - 13])
        .collect();

    // Match device types, triggers, receivers
    // ...
}
```

### Fingerprinting

```rust
const FINGERPRINTS: &[(&str, &[&str])] = &[
    ("button_device", &["InteractionRadius", "InteractTime"]),
    ("player_spawner_device", &["UseAsIslandStart", "PlayerTeam"]),
    // ...
];
```

---

## MCP Tools

| Tool | Description |
|------|-------------|
| `scan_map_devices` | Scan project for devices |
| `get_device_props` | Get digest-grounded properties |
| `query_digest` | Search digest symbols |
| `list_editables` | Parse *.verse for @editable |

---

## Phase Plan

### Phase 1 — Scanner (1 week)

- [ ] Test `unreal_asset` compatibility with UEFN
- [ ] Implement classify/extract functions
- [ ] Add fingerprinting
- [ ] CLI binary with JSON output
- [ ] Golden tests

**Done:** `verse-mcp scan` matches `scan-verse.js` output.

---

### Phase 2 — MCP Server (1 week)

- [ ] Set up `rmcp` stdio transport
- [ ] Implement core tools
- [ ] Add mtime-based caching
- [ ] Graceful shutdown

**Done:** MCP server responds to tool calls.

---

### Phase 3 — Digest (1-2 weeks)

- [ ] Parse `Fortnite.digest.verse`
- [ ] Build device → events/methods index
- [ ] Wire `query_digest` tool
- [ ] Cross-validation tests

**Done:** Trigger/receiver validation against digest.

---

### Phase 4 — Release (1 week)

- [ ] GitHub Actions: Win/Mac/Linux builds
- [ ] Binary releases
- [ ] README with examples

**Done:** Single-command install.

---

### Phase 5 — Advanced Features (2 weeks)

> **Scope:** Analysis features only. No UEFN UI manipulation.

#### 5.1: Hallucination Detection

```rust
pub fn validate(code: &str, digest: &DigestIndex) -> Vec<Issue> {
    // Parse Verse, cross-ref against digest
}
```

#### 5.2: Breaking Change Detector

```rust
pub fn diff_digests(old: &Digest, new: &Digest) -> Vec<Change> {
    // Find removed/changed APIs
}
```

#### 5.3: Wiring Validator

```rust
pub fn validate_wiring(devices: &[DeviceInfo]) -> Vec<WiringIssue> {
    // Detect orphaned channels, conflicts
}
```

#### 5.4: Device Graph

```rust
pub fn to_mermaid(devices: &[DeviceInfo]) -> String {
    // Generate connection diagram
}
```

#### 5.5: Game Flow Analyzer

```rust
pub fn analyze_flow(devices: &[DeviceInfo]) -> FlowReport {
    // Detect unreachable states, missing win conditions
}
```

#### 5.6: Composition Templates

```yaml
# templates/scoring.yaml
devices: [score_manager_device, hud_device]
wiring:
  - ScoreManager.OnScoreChange → HUD.Update
```

#### 5.7: Event Storm Detector

```rust
pub fn detect_storms(devices: &[DeviceInfo]) -> Vec<StormWarning> {
    // Find cascading event chains
}
```

**Checklist:**
- [ ] 5.1: Verse code validation
- [ ] 5.2: Digest version diff
- [ ] 5.3: Channel wiring validation
- [ ] 5.4: Mermaid graph generation
- [ ] 5.5: State machine analysis
- [ ] 5.6: Template save/load
- [ ] 5.7: Event cascade detection

---

## Open Questions

| Question | Resolution |
|----------|------------|
| Does `unreal_asset` work with UEFN? | Test in Phase 1 |
| Which UE5 version? | Read from header |

---

## Deliverables

### MVP
- [ ] Scanner CLI with JSON output
- [ ] Fingerprinting for unknown devices
- [ ] Golden tests

### MCP
- [ ] stdio server with core tools
- [ ] Digest integration
- [ ] Cross-platform binaries

### Advanced
- [ ] Code validation
- [ ] Wiring checks
- [ ] Device graphs
- [ ] Templates

---

## References

- [MCP Spec](https://modelcontextprotocol.io/specification/2025-06-18/basic/transports)
- [rmcp SDK](https://github.com/modelcontextprotocol/rust-sdk)
- [unreal_asset](https://docs.rs/unreal_asset)

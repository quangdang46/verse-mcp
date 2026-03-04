# verse-mcp — Rust Port Plan (v2)

> **Updated:** 2026-03-04 — after cross-referencing UAssetAPI source, the UE5 binary
> format spec, and existing Rust crates (`uasset`, `unreal_asset`).
>
> **Key finding:** The JS parser's Name Map location strategy is a heuristic workaround
> for not reading the `FPackageFileSummary` header. The header contains exact offsets
> for every section. The Rust port should read the header properly — making the parser
> both correct and immune to the probe-based fragility.

---

## What Changed from v1

| Area | v1 assumption | Corrected understanding |
|---|---|---|
| Name Map location | Probe bytes 100–2000, find `/Script/` prefix | Header at offset 0 contains `NameOffset` and `NameCount` — use them directly |
| Name Map entry format | FString + 4-byte hash | Correct — but hash presence is version-conditional |
| PropertyOverrideData "25-byte stride" | Constant across all UEFN builds | **Wrong.** UE uses tagged serialization — FName key + type tag + size + value |
| Property serialization | Fixed 25-byte gap between key and value | Tagged format: `FName(key) + FName(type) + i32(size) + ... + value bytes` |
| Existing Rust crates | Build from scratch | `unreal_asset` crate exists and parses UE5 assets with full property support |

---

## 1. The Actual UE5 Binary Format

### 1.1 FPackageFileSummary — the real table of contents

The Package File Summary is the table of contents for every uasset file, located at offset 0.
The correct parse sequence starts here, not at a probed offset:

```
Offset 0:  u32  Tag (0x9E2A83C1)               ← magic
Offset 4:  i32  LegacyFileVersion               ← negative; -8 or below = UE5
Offset 8:  i32  LegacyUE3Version (conditional)
           i32  FileVersionUE4
           i32  FileVersionUE5 (if LegacyFileVersion <= -8)
           i32  FileVersionLicenseeUE
           ...  CustomVersions (i32 count + {16-byte GUID + i32 version} × count)
           i32  TotalHeaderSize
           FString  PackageName
           i32  PackageFlags
           i32  NameCount      ← exact count of Name Map entries
           i32  NameOffset     ← exact byte offset to start of Name Map
           i32  ExportCount
           i32  ExportOffset
           i32  ImportCount
           i32  ImportOffset
           ...  (more offsets follow)
```

The JS parser **never reads `NameCount` or `NameOffset`**. It probes bytes 100–2000
looking for a `/Script/` prefix. This is an unreliable heuristic: on different UEFN
versions or larger asset files, the Name Map may start outside that probe window,
causing silent misparse or complete miss.

**Rust fix:** Parse the summary header to extract `NameOffset` and `NameCount`, then
seek directly to `NameOffset` and read exactly `NameCount` entries. Zero probing.

### 1.2 Name Map entry format

Each entry in the Name Map is:
```
FString   (i32 length + UTF-8 bytes + null terminator)
u32       CityHash of the name, lower-cased
```

The JS `+4` skip after each FString is correct for this hash. What the JS does not do
is use `NameCount` to know when to stop — it stops at the first parse failure. Using
the exact count eliminates that fragility.

### 1.3 Tagged property format — the "25-byte stride" is incorrect

This is the most important correction. UE assets use **tagged serialization** for
properties. The real structure for each property entry in export data is:

```
FName    PropertyName        (index + number = 8 bytes)
FName    PropertyType        (index + number = 8 bytes, e.g. "FloatProperty")
i32      SerializedSize      (byte length of the value payload)
i32      DuplicationIndex
[type-specific tag data — varies by type, e.g. 8 bytes for EnumProperty]
[value bytes — exactly SerializedSize bytes]
```

The list ends when `PropertyName` resolves to "None".

The 25-byte gap in the JS parser coincidentally works on the test files because all
the test devices happen to use only simple scalar properties (floats, bools, short
strings) that produce a consistent total byte count in the raw export blob. It is
not a format specification and will silently break on:

- Properties with non-trivial type tags (EnumProperty, StructProperty, ArrayProperty)
- Asset reference properties
- Any future UEFN version that changes serialization for any device type

**Rust fix:** Read the Export Map from the header to seek to the correct export offset,
then iterate the tagged property list properly until the "None" sentinel.

### 1.4 ActorLabel extraction

The JS raw byte search for `\x0b\x00\x00\x00ActorLabel\x00` works because ActorLabel
is stored as a string property in the CDO export. With proper tagged property reading,
this becomes a direct named property lookup — cleaner and no byte-pattern scanning.

---

## 2. Existing Rust Crates — Evaluate Before Writing Anything

Two relevant Rust crates exist:

| Crate | UE Support | Status | Notes |
|---|---|---|---|
| `uasset` (jorgenpt/uasset-rs) | UE4.10–4.26 | Active | Header + name map + imports; no export data |
| `unreal_asset` | UE4–UE5 | Active | Full read/write, 100+ property types, UE5 |

**`unreal_asset`** is the strongest candidate. It supports UE5, correctly parses
tagged properties, and returns typed `PropertyData` variants for each property.
Its basic API:

```rust
use unreal_asset::{engine_version::EngineVersion, Asset};

let file = File::open(path)?;
let asset = Asset::new(file, None, EngineVersion::VER_UE5_1)?;

// Name Map is accessible directly — no probing
let names = asset.get_name_map();

// Exports contain typed properties
for export in &asset.asset_data.exports {
    if let Some(normal) = export.get_normal_export() {
        for prop in &normal.properties {
            println!("{}: {:?}", prop.get_name().get_owned_content(), prop);
        }
    }
}
```

**Decision:** Use `unreal_asset` as the parsing backend for Phase 1. The entire custom
binary parser from v1 (FString combinator, stride-scan, label byte search) is replaced
by a thin extraction layer on top of this crate.

If `unreal_asset` cannot handle UEFN's specific asset variant — UEFN is a custom UE5
fork and may have non-standard custom version GUIDs — implement a custom parser using
the correct header-aware approach described in §1.1, not the probe-based approach.
The compatibility test in §9 determines this immediately.

---

## 3. Revised Architecture

```
verse-mcp (single Rust binary)
├── bin/verse-mcp.rs              ← CLI + MCP stdio entrypoint
├── lib/
│   ├── extractor/                ← extraction layer over unreal_asset
│   │   ├── asset.rs              ← open file, handle engine version detection
│   │   ├── classify.rs           ← device type from Name Map + fingerprint table
│   │   ├── label.rs              ← ActorLabel property lookup
│   │   └── settings.rs           ← export property list → IndexMap<String, String>
│   ├── fingerprint.rs            ← static fingerprint table
│   ├── scanner.rs                ← walkdir + rayon parallel extraction
│   ├── digest.rs                 ← Fortnite.digest.verse reader (Phase 2)
│   └── mcp/
│       ├── server.rs             ← JSON-RPC stdio loop
│       └── tools/
│           ├── scan_map.rs
│           ├── get_device_props.rs
│           └── query_digest.rs
└── Cargo.toml
```

---

## 4. Revised Crate Dependencies

```toml
[dependencies]
# Asset parsing — existing UE5-capable Rust library
unreal_asset = "0.1"

# File system walk
walkdir      = "2"

# Parallelism
rayon        = "1"

# JSON
serde        = { version = "1", features = ["derive"] }
serde_json   = "1"
indexmap     = "2"             # preserve property insertion order

# MCP stdio transport
tokio        = { version = "1", features = ["io-std", "rt-multi-thread"] }

# CLI
clap         = { version = "4", features = ["derive"] }

# Errors
thiserror    = "1"
anyhow       = "1"
```

`nom` and `byteorder` are removed — no custom FString parser needed.

---

## 5. Core Extraction Logic

### 5.1 Asset open with engine version detection

UEFN uses a UE5 fork. Try versions in order and use the first that parses without error:

```rust
fn open_asset(path: &Path) -> anyhow::Result<Asset<File>> {
    for ver in [EngineVersion::VER_UE5_2, EngineVersion::VER_UE5_1, EngineVersion::VER_UE5_0] {
        if let Ok(asset) = Asset::new(File::open(path)?, None, ver) {
            return Ok(asset);
        }
    }
    bail!("Could not parse {} with any UE5 version", path.display())
}
```

### 5.2 Device classification from Name Map

The Name Map is returned as a clean `Vec<String>` — no parsing needed at call site:

```rust
pub fn classify(names: &[String]) -> Classification {
    let gen_set: HashSet<&str> = names.iter()
        .filter(|n| n.ends_with("_GEN_VARIABLE"))
        .map(|n| &n[..n.len() - "_GEN_VARIABLE".len()])
        .collect();

    let mut device_type = None;
    let mut triggers = vec![];
    let mut receivers = vec![];

    for name in names {
        if name.ends_with("_GEN_VARIABLE") { continue; }
        if device_type.is_none() && DEVICE_TYPE_RE.is_match(name) {
            device_type = Some(name.clone());
        } else if gen_set.contains(name.as_str()) {
            if name.starts_with("Receiver") || name.ends_with("WhenReceiving") {
                receivers.push(name.clone());
            } else {
                triggers.push(name.clone());
            }
        }
    }

    // Run fingerprinting BEFORE assigning "Unknown" fallback — this fixes the JS bug
    // where device_type was already "Unknown" string when fingerprinting ran
    if device_type.is_none() {
        // fingerprinting runs in settings.rs after properties are extracted
    }

    Classification { device_type, triggers, receivers }
}
```

### 5.3 Settings and label extraction via typed properties

```rust
pub fn extract(asset: &Asset<File>) -> (Option<String>, IndexMap<String, String>) {
    let mut label = None;
    let mut settings = IndexMap::new();

    for export in &asset.asset_data.exports {
        let Some(normal) = export.get_normal_export() else { continue };

        for prop in &normal.properties {
            let key = prop.get_name().get_owned_content();

            let value_str = match prop {
                PropertyData::StrPropertyData(s) => {
                    s.value.as_ref().map(|v| v.to_string())
                }
                PropertyData::FloatPropertyData(f) => {
                    Some(format!("{:.6}", f.value))
                }
                PropertyData::DoublePropertyData(d) => {
                    Some(format!("{:.6}", d.value))
                }
                PropertyData::BoolPropertyData(b) => {
                    Some(b.value.to_string())
                }
                PropertyData::EnumPropertyData(e) => {
                    Some(e.value.get_owned_content())
                }
                PropertyData::StructPropertyData(s) => {
                    // Team specs like (TeamType=Any,TeamIndex=1) live here
                    Some(format!("{:?}", s))
                }
                _ => None,
            };

            if let Some(val) = value_str {
                if key == "ActorLabel" || key == "LabelOverride" {
                    label = Some(val.clone());
                }
                if settings.get(&key).is_none() {
                    settings.insert(key, val);
                }
            }
        }
    }
    (label, settings)
}
```

This approach means the key/value inversion bug (`"False": "bIsPlayerBuildable"`)
is architecturally impossible — the property name always comes from the FName key,
never from the value bytes.

---

## 6. Bugs Resolved by Correct Parsing

| Bug in JS output | Root cause | Status in Rust |
|---|---|---|
| `"False": "bIsPlayerBuildable"` entries | Stride-scan misaligns, reads value as key | Impossible with typed property API |
| Button/spawn pad classified as `Unknown` | Fingerprinting ran after `"Unknown"` string was assigned | Fixed: fingerprint before fallback assignment |
| Component names in triggers list | No `_GEN_VARIABLE` twins for non-Verse devices | Fixed: naming convention fallback for fingerprinted devices |
| Empty receivers for fingerprinted devices | Same as above | Fixed: `Receiver*` / `*WhenReceiving` fallback |
| Potential Name Map miss for some files | Probe window 100–2000 may not cover all files | Fixed: header-based `NameOffset` used directly |

---

## 7. Fallback: Custom Header-Aware Parser

If `unreal_asset` fails on UEFN assets, implement a custom parser using the proper
header layout. The critical change vs the JS approach:

```rust
pub struct Summary {
    pub name_count:   u32,
    pub name_offset:  u32,
    pub export_count: u32,
    pub export_offset: u32,
}

impl Summary {
    pub fn parse(buf: &[u8]) -> Result<Self> {
        ensure!(buf.len() >= 64, "too short");
        ensure!(u32::from_le_bytes(buf[0..4].try_into()?) == 0x9E2A83C1, "bad magic");

        let legacy_ver = i32::from_le_bytes(buf[4..8].try_into()?);
        // Walk through variable-length header fields using the layout from §1.1
        // Extract NameCount and NameOffset from fixed positions relative to parse cursor
        // Then: seek to NameOffset, read NameCount × (FString + u32 hash)
        // Then: seek to ExportOffset, parse export table entries for CDO
        todo!()
    }
}
```

The Name Map read becomes:
```rust
let names = (0..summary.name_count)
    .map(|_| read_fstring_and_skip_hash(&mut cursor))
    .collect::<Result<Vec<String>>>()?;
```

No probing. No heuristics. Deterministic.

---

## 8. MCP Transport

No change from v1. Raw JSON-RPC over stdin/stdout for Phase 1 (~150 lines).
Migrate to `rmcp` crate when it matures.

The single binary makes the Cursor/Claude Desktop config trivially simple:

```json
{
  "mcpServers": {
    "verse": {
      "command": "verse-mcp",
      "args": ["serve", "--dir", "E:\\Projects\\Testproject"]
    }
  }
}
```

---

## 9. Phase Plan (revised)

### Phase 1 — Extraction layer (1 week)

- [ ] **Compatibility test first:** can `unreal_asset` open an actual UEFN `__ExternalActors__` file?
- [ ] If yes: implement `classify()`, `extract()` on top of it; skip custom parser work
- [ ] If no: implement custom header-aware parser (§7) — proper header read, not probe
- [ ] Fix fingerprinting order (run before `"Unknown"` assignment)
- [ ] Fix trigger naming convention fallback for fingerprinted devices
- [ ] Add `island_settings_device` fingerprint
- [ ] `verse-mcp scan` output is correct for all 9 known devices

### Phase 2 — MCP server (1 week)

- [ ] Raw JSON-RPC stdio transport
- [ ] `scan_map_devices`, `get_device_props`, `query_digest` tools
- [ ] Cursor + Claude Desktop config examples in README

### Phase 3 — Digest integration (1–2 weeks)

- [ ] `Fortnite.digest.verse` line parser
- [ ] `query_digest` tool
- [ ] Cross-reference Name Map names against digest (version drift detector)

### Phase 4 — Release (1 week)

- [ ] GitHub Actions build matrix: Windows x64, macOS arm64, Linux x64
- [ ] Pre-built binary releases (no Rust toolchain required)
- [ ] `cargo install verse-mcp` via crates.io

---

## 10. Open Questions (updated)

| Question | Impact | Resolution |
|---|---|---|
| Does `unreal_asset` open UEFN ExternalActors files? | **Critical** — determines entire parser approach | Compatibility test is the very first thing in Phase 1 |
| Which UE5 version does UEFN use for ExternalActors? | High | Read `FileVersionUE5` from header bytes 16–19 and try matching `VER_UE5_x` |
| Are UEFN ExternalActors assets cooked or editor/uncooked? | High — cooked assets use unversioned properties, require `.usmap` mappings | Check `PKG_FilterEditorOnly` flag in PackageFlags field; editor-saved assets are typically uncooked |
| Does the `PropertyOverrideData` label in the JS actually point to the CDO export? | Medium | Verify by matching the byte offset in the JS scan against `ExportOffset` + CDO export size |
| Is the 25-byte stride coincidentally consistent because all test devices use only scalar properties? | Confirmed likely yes | Will be validated or refuted when proper tagged parsing is implemented |

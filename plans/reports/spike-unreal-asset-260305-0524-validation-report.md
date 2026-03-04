# SPIKE Report: unreal_asset Crate Validation

**Task**: verse-mcp-3tm
**Date**: 2026-03-05
**Agent**: SageMoose (claude-opus-4-6)

## Summary

Validated the `unreal_asset` Rust crate (v0.1.16) for parsing UEFN ExternalActors `.uasset` files.

## Findings

### 1. Crate Availability ✅
- **Crate**: `unreal_asset` v0.1.16
- **Source**: crates.io
- **License**: MIT
- **Dependencies**: Compiles successfully on Linux

### 2. UE5 Version Support ⚠️
| Version | Supported |
|---------|-----------|
| UE5.0 | ✅ VER_UE5_0 |
| UE5.1 | ✅ VER_UE5_1 |
| UE5.2 | ✅ VER_UE5_2 |
| UE5.3 | ❌ NOT SUPPORTED |
| UE4.27 | ✅ VER_UE4_27 |

**Critical**: UEFN is based on UE5.3+, which is NOT supported by the crate.

### 3. Oodle Compression ⚠️
- **Feature**: `oodle` - required for some compressed assets
- **Platform**: Windows-only (requires `oodle-sys`)
- **Status**: Disabled in spike (Linux environment)

### 4. API Structure
```rust
// Parsing API
Asset::new(reader, None, EngineVersion::VER_UE5_2)

// Accessing data
asset.asset_data.exports       // Vec<Export>
asset.get_name_map()           // SharedResource<NameMap>
name_map.get_name_map_index_list()  // &[String]
```

## Validation Status

| Check | Status | Notes |
|-------|--------|-------|
| Crate compiles | ✅ PASS | Built successfully on Linux |
| API usable | ✅ PASS | Can access name map and exports |
| UE5.3 support | ❌ FAIL | Crate doesn't support VER_UE5_3 |
| UEFN file test | ⏸️ BLOCKED | No UEFN files available on this machine |

## Recommendations

### Primary Path: Try unreal_asset First
Despite the UE5.3 limitation, the crate may still work because:
1. UEFN uses custom version GUIDs (may not require UE5.3-specific parsing)
2. The JS scanner successfully parsed UEFN files without UE5.3 features
3. Worth testing with actual UEFN files

### Fallback Path: Custom Parser (verse-mcp-1k1)
If unreal_asset fails with UEFN files:
1. Implement custom UE5 binary parser
2. Read `PackageFileSummary` header directly
3. Use `NameOffset`/`NameCount` for name map (not probing)
4. Implement tagged property serialization

## Next Steps

1. **Transfer UEFN files** to this machine for actual testing
2. **Run spike** with real `.uasset` files:
   ```bash
   cargo run -- /path/to/ExternalActors/*.uasset
   ```
3. **If PASS**: Proceed with verse-mcp-37p (Rust workspace setup)
4. **If FAIL**: Close verse-mcp-3tm, start verse-mcp-1k1 (custom parser)

## Spike Code Location

```
/home/quangdang/projects/verse-mcp/spike-unreal-asset/
├── Cargo.toml
└── src/main.rs
```

## Related Tasks

- verse-mcp-2ow: [DECISION] Parser approach validated (blocked by this spike)
- verse-mcp-1k1: [SPIKE FALLBACK] Custom UE5 binary parser
- verse-mcp-37p: Set up Rust workspace and core data structures

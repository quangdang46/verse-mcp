# SPIKE Report: unreal_asset Crate Validation

**Task:** verse-mcp-3tm
**Date:** 2026-03-05
**Agent:** EmeraldBear (Claude Opus 4.6)

## Summary

Validated the `unreal_asset` Rust crate (v0.1.16) for parsing UEFN `.uasset` files.

## Findings

### 1. Compilation Status

| Platform | Status | Notes |
|----------|--------|-------|
| Linux | ✓ Compiles | Without `oodle` feature |
| Windows | ✓ Compiles | With `oodle` feature |

### 2. Critical Limitation: Oodle Compression

```
error: unable to find library -loo2core_9_win64
```

The `oodle` feature requires Epic's proprietary Oodle compression library:
- **Windows-only**: `oo2core_9_win64.dll`
- **Not available on Linux**
- **UEFN files may use Oodle compression**

### 3. Engine Version Support

| Version | Supported |
|---------|-----------|
| UE5.0 | ✓ |
| UE5.1 | ✓ |
| UE5.2 | ✓ |
| UE5.3 | ✗ Not supported |
| UE5.4+ | ✗ Not supported |

**UEFN is based on UE5.3+** - This is a potential compatibility issue.

### 4. API Observations

- `SharedResource<NameMap>` requires special access patterns
- `Asset::new()` consumes the file handle
- Asset data accessible via `asset.asset_data.exports`

## Test Limitations

Could not test with actual UEFN files:
- Test files located on Windows: `E:\Projects\Testproject\Content\__ExternalActors__`
- No sample files available on Linux development machine

## Recommendations

### Option A: Proceed with unreal_asset (Conditional)

**If:**
- UEFN files don't use Oodle compression
- UEFN files are compatible with UE5.2 format

**Then:**
- Use unreal_asset on Windows only
- Or find Linux Oodle library

### Option B: Implement Custom Parser (Recommended)

**Reasons:**
1. **Oodle limitation**: Cannot decompress Oodle-compressed assets on Linux
2. **Version mismatch**: UEFN uses UE5.3+, unreal_asset only supports up to UE5.2
3. **Custom version GUIDs**: UEFN may use non-standard version identifiers
4. **Proven approach**: The JS scanner (scan-verse.js) already works

**Implementation:**
- Port scan-verse.js logic to Rust
- Use direct binary parsing (FPackageFileSummary header)
- Support both Linux and Windows

## Decision

**Proceed with Option B: Custom Parser**

The existing JS scanner proves the binary format is parseable. Porting to Rust gives us:
- Cross-platform support (Linux + Windows)
- No external dependencies
- Full control over parsing logic
- Alignment with proven POC approach

## Next Steps

1. Close verse-mcp-3tm (this SPIKE)
2. Close verse-mcp-2ow (decision: use custom parser)
3. Start verse-mcp-37p (Rust workspace setup)
4. Implement verse-mcp-1k1 (custom UE5 binary parser)

## Files Created

- `/home/quangdang/projects/verse-mcp/spike-unreal-asset/` - SPIKE test project

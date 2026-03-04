#!/usr/bin/env node
/**
 * scan-verse.js — verse-mcp v4
 * Parse UE5 .uasset binary directly. Zero dependencies.
 *
 * Binary format confirmed from UAssetAPI + real file analysis:
 *   - Name Map: FString[] + uint32 hash for each entry (start with /Script/ path)
 *   - Label: FString("ActorLabel\0") next to FString(label_value) from export data
 *   - Config: FString(key) + 25 bytes metadata + FString(value) from PropertyOverrideData
 *
 * Usage:
 *   node scan-verse.js --dir "E:\Projects\Testproject"
 *   node scan-verse.js --dir "E:\Projects\Testproject" --out result.json
 */

const fs   = require("fs");
const path = require("path");

const arg = n => { const i = process.argv.indexOf(n); return i >= 0 ? process.argv[i+1] : null; };
const rootDir = arg("--dir");
const outFile = arg("--out");

if (!rootDir) {
  console.error('Usage: node scan-verse.js --dir "E:\\Projects\\Testproject"');
  console.error('       node scan-verse.js --dir "E:\\Projects\\Testproject" --out result.json');
  process.exit(1);
}

const actorsRoot = path.join(rootDir, "Content", "__ExternalActors__");
if (!fs.existsSync(actorsRoot)) {
  console.error(`[ERROR] Not found: ${actorsRoot}`); process.exit(1);
}

const UE_MAGIC = 0x9E2A83C1;
const GEN      = "_GEN_VARIABLE";

// ── readFString ────────────────────────────────────────────────
function readFString(buf, off) {
  if (off + 4 > buf.length) return null;
  const len = buf.readInt32LE(off);
  if (len === 0) return { str: "", nextOff: off + 4 };
  if (len < 0) {
    const end = off + 4 + (-len) * 2;
    if (end > buf.length) return null;
    return { str: buf.slice(off+4, end-2).toString("utf16le"), nextOff: end };
  }
  const end = off + 4 + len;
  if (end > buf.length || buf[end-1] !== 0) return null;
  for (let c = off+4; c < end-1; c++) {
    if (buf[c] < 0x20 || buf[c] > 0x7e) return null;
  }
  return { str: buf.slice(off+4, end-1).toString("utf8"), nextOff: end };
}

// ── readNameMap ────────────────────────────────────────────────
function readNameMap(buf) {
  for (let probe = 100; probe < Math.min(buf.length, 2000); probe++) {
    const r = readFString(buf, probe);
    if (!r || (!r.str.startsWith("/Script/") && !r.str.startsWith("/Game/"))) continue;
    let off = probe, run = 0;
    while (off < buf.length-8 && run < 10) {
      const r2 = readFString(buf, off);
      if (!r2 || !r2.str) break;
      run++; off = r2.nextOff + 4;
    }
    if (run < 5) continue;
    const names = [];
    off = probe;
    while (off < buf.length-8) {
      const r2 = readFString(buf, off);
      if (!r2 || !r2.str) break;
      names.push(r2.str);
      off = r2.nextOff + 4;
    }
    return { names, endOff: off };
  }
  return null;
}

// ── extractLabel ──────────────────────────────────────────────
// Scan binary for: FString("ActorLabel") immediately followed by FString(label)
function extractLabel(buf) {
  const needle = Buffer.from('\x0b\x00\x00\x00ActorLabel\x00'); // len=11 + "ActorLabel\0"
  for (let i = 0; i < buf.length - needle.length - 8; i++) {
    if (!buf.slice(i, i + needle.length).equals(needle)) continue;
    // Immediately after = FString(label_value)
    const labelOff = i + needle.length;
    const r = readFString(buf, labelOff);
    if (!r || !r.str) continue;
    if (/^[A-Za-z][A-Za-z0-9 _-]{0,40}$/.test(r.str)) return r.str;
  }
  return null;
}

// ── extractConfigValues ───────────────────────────────────────
// PropertyOverrideData: FString(key) + 25 bytes metadata + FString(value)
// Key must be PascalCase word (no spaces, no special chars)
// Value must be numeric, bool, enum, or team spec
function extractConfigValues(buf, scanStart) {
  const result = {};
  for (let i = scanStart; i < buf.length - 60; i++) {
    const kResult = readFString(buf, i);
    if (!kResult) continue;
    const key = kResult.str;
    // Strict PascalCase key: starts uppercase, only letters, 4-50 chars
    if (!/^[A-Z][A-Za-z]{3,49}$/.test(key)) continue;

    const vOff = kResult.nextOff + 25;
    if (vOff + 4 >= buf.length) continue;
    const vResult = readFString(buf, vOff);
    if (!vResult || !vResult.str) continue;
    const val = vResult.str;

    const valid =
      /^-?[\d]+\.[\d]+$/.test(val)        ||  // numeric: "2.000000"
      /^(True|False)$/.test(val)           ||  // bool
      /^(Always|Never|No Effect|Yes|No)$/.test(val) ||  // enum
      val.startsWith("(")                  ||  // team/class spec
      /^[A-Za-z][A-Za-z0-9_ ]{1,30}$/.test(val);      // plain string like "Campfire"

    if (!valid) continue;
    if (result[key] === undefined) result[key] = val;
    i = vResult.nextOff - 1;
  }
  return result;
}

// ── parseFile ──────────────────────────────────────────────────
function parseFile(filePath) {
  let buf;
  try { buf = fs.readFileSync(filePath); } catch { return null; }
  if (buf.length < 64) return null;
  if (buf.readUInt32LE(0) !== UE_MAGIC) return null;

  const nm = readNameMap(buf);
  if (!nm || nm.names.length < 5) return null;
  const { names, endOff } = nm;

  // Classify from name map using _GEN_VARIABLE as the signal.
  // Every Verse-accessible event (trigger/receiver) has a _GEN_VARIABLE twin in the Name Map.
  // Internal UE names (ArrayProperty, StaticMeshComponent, etc.) never do.
  // So: if "X_GEN_VARIABLE" exists → X is a real Verse event. No regex patterns needed.
  const genSet = new Set(names.filter(n => n.endsWith(GEN)).map(n => n.slice(0, -GEN.length)));

  let deviceType = null;
  const triggers  = [];
  const receivers = [];

  for (const n of names) {
    if (n.endsWith(GEN)) continue;
    if (!deviceType && /^Device_[A-Za-z0-9]+_C$/.test(n)) {
      deviceType = n;
    } else if (genSet.has(n)) {
      // Classify by naming convention from the binary itself
      if (n.startsWith("Receiver") || n.endsWith("WhenReceiving")) receivers.push(n);
      else triggers.push(n);
    }
  }

  // Config values (second half of file)
  const scanStart = Math.max(endOff, Math.floor(buf.length * 0.5));
  const settings  = extractConfigValues(buf, scanStart);

  if (!deviceType && triggers.length === 0 && Object.keys(settings).length === 0) return null;

  // Device type not found in Name Map — mark as Unknown
  if (!deviceType) deviceType = "Unknown";

  // Extract real label from binary
  const label = extractLabel(buf);

  return {
    file:       path.relative(actorsRoot, filePath).replace(/\\/g, "/"),
    deviceType,
    label:      label ?? null,
    triggers:   [...new Set(triggers)].sort(),
    receivers:  [...new Set(receivers)].sort(),
    settings:   Object.keys(settings).length > 0 ? settings : undefined,
  };
}

// ── walkDir ────────────────────────────────────────────────────
function walkDir(dir, out = []) {
  let entries;
  try { entries = fs.readdirSync(dir, { withFileTypes: true }); } catch { return out; }
  for (const e of entries) {
    const full = path.join(dir, e.name);
    if (e.isDirectory())                 walkDir(full, out);
    else if (e.name.endsWith(".uasset")) out.push(full);
  }
  return out;
}

// ── main ───────────────────────────────────────────────────────
process.stderr.write(`\n[verse-mcp] Project : ${rootDir}\n`);
process.stderr.write(`[verse-mcp] Scanning: ${actorsRoot}\n`);

const allFiles = walkDir(actorsRoot);
process.stderr.write(`[verse-mcp] Files   : ${allFiles.length} .uasset\n`);
process.stderr.write(`[verse-mcp] Parsing...\n\n`);

const devices = [], skipped = [];

for (let i = 0; i < allFiles.length; i++) {
  process.stderr.write(`\r[verse-mcp] ${i+1}/${allFiles.length} processed...`);
  const info = parseFile(allFiles[i]);
  if (info) devices.push(info);
  else      skipped.push(path.relative(actorsRoot, allFiles[i]).replace(/\\/g, "/"));
}

process.stderr.write(`\n\n`);
if (skipped.length > 0) {
  process.stderr.write(`[verse-mcp] Skipped (${skipped.length} files — map tiles or non-device assets):\n`);
  skipped.forEach(f => process.stderr.write(`  - ${f}\n`));
  process.stderr.write(`\n`);
}

const byType = {};
for (const d of devices) {
  (byType[d.deviceType] = byType[d.deviceType] || []).push(d);
}

const output = {
  scanned_at:    new Date().toISOString(),
  project_root:  rootDir,
  total_files:   allFiles.length,
  total_devices: devices.length,
  skipped:       skipped.length,
  device_types:  Object.keys(byType).sort(),
  by_type:       byType,
};

const json = JSON.stringify(output, null, 2);
if (outFile) {
  fs.writeFileSync(outFile, json, "utf8");
  process.stderr.write(`[verse-mcp] Saved → ${outFile}\n\n`);
} else {
  console.log(json);
}
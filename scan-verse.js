#!/usr/bin/env node
/**
 * scan-verse.js — verse-mcp v4
 * Parse UE5 .uasset binary trực tiếp. Zero dependencies.
 *
 * Binary format confirmed từ UAssetAPI + real file analysis:
 *   - Name Map: FString[] + uint32 hash mỗi entry (bắt đầu bằng /Script/ path)
 *   - Label: FString("ActorLabel\0") ngay tiếp theo FString(label_value) trong export data
 *   - Config: FString(key) + 25 bytes metadata + FString(value) trong PropertyOverrideData
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

// ── NOISE: invalid label strings ──────────────────────────────
const NOISE = new Set([
  "ArrayProperty","BoolProperty","ByteProperty","DoubleProperty","EnumProperty",
  "FloatProperty","IntProperty","NameProperty","ObjectProperty","StrProperty",
  "StructProperty","SoftObjectProperty","MulticastInlineDelegateProperty",
  "MulticastSparseDelegateProperty","ActorTemplateID","Class","Package","Level",
  "World","None","PersistentLevel","RootComponent","Owner","AttachParent",
  "True","False","Always","Never",
]);

// ── DEVICE FINGERPRINTS from settings keys ────────────────────
const FINGERPRINTS = [
  { type: "player_spawner_device",    keys: ["UseAsIslandStart","EnemyRangeCheck","PlayerTeam"] },
  { type: "button_device",            keys: ["InteractionRadius","InteractTime","ActivatingTeam","TriggerSound"] },
  { type: "trigger_device",           keys: ["TriggerRadius","ActivatingTeam","TriggerOnce"] },
  { type: "item_spawner_device",      keys: ["SpawnRadius","SpawnTimer","AllowInfiniteSpawn"] },
  { type: "character_spawner_device", keys: ["NPCCharacterDefinition","DespawnAIsWhenDisabled"] },
  { type: "timer_device",             keys: ["TimerDuration","AutoStart","CountdownTime"] },
  { type: "tracker_device",           keys: ["TrackingType","CountToWin"] },
  { type: "score_manager_device",     keys: ["ScoreIncrement","MaxScore","WinCondition"] },
];

// ── VERSE API per device type ─────────────────────────────────
const VERSE_API = {
  player_spawner_device:    { triggers: ["OnSpawnedPlayerEliminated","OnSpawnComplete"],        receivers: ["ReceiverSpawn","ReceiverEnable","ReceiverDisable"] },
  button_device:            { triggers: ["OnTriggered","TriggerOnInteracted","OnEnabled","OnDisabled"], receivers: ["ReceiverEnable","ReceiverDisable","ReceiverReset"] },
  trigger_device:           { triggers: ["OnTriggered","TriggerOnEnterZone","TriggerOnExitZone"], receivers: ["ReceiverEnable","ReceiverDisable","ReceiverReset"] },
  character_spawner_device: { triggers: ["OnSpawnedCharacterEliminated","OnAllCharactersEliminated"], receivers: ["ReceiverEnable","ReceiverDisable","ReceiverSpawnCharacter"] },
  timer_device:             { triggers: ["OnTimerExpired","OnReset"],                           receivers: ["ReceiverStart","ReceiverStop","ReceiverReset"] },
  tracker_device:           { triggers: ["OnTargetCountReached","OnCountUpdated"],              receivers: ["ReceiverIncrement","ReceiverDecrement","ReceiverReset"] },
};

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
    if (!r || !r.str || NOISE.has(r.str)) continue;
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
    // Skip noise
    if (NOISE.has(key)) continue;

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

  // Classify from name map
  let deviceType = null;
  const triggers  = [];
  const receivers = [];

  for (const n of names) {
    if (n.endsWith(GEN)) continue;
    if (!deviceType && /^Device_[A-Za-z0-9]+_C$/.test(n))
      deviceType = n;
    else if (/^(OnDisabled|OnEnabled|OnAddedToMinigame|OnRemovedFromMinigame|Trigger[A-Z][A-Za-z0-9]+)$/.test(n))
      triggers.push(n);
    else if (/^(Receiver[A-Z][A-Za-z0-9]+|EnableWhenReceiving|DisableWhenReceiving)$/.test(n))
      receivers.push(n);
  }

  // Config values (second half of file)
  const scanStart = Math.max(endOff, Math.floor(buf.length * 0.5));
  const settings  = extractConfigValues(buf, scanStart);

  if (!deviceType && triggers.length === 0 && Object.keys(settings).length === 0) return null;

  // Fingerprint unknown device type
  if (!deviceType) {
    const keys = Object.keys(settings);
    let best = null, bestScore = 0;
    for (const fp of FINGERPRINTS) {
      const score = fp.keys.filter(k => keys.includes(k)).length;
      if (score >= 2 && score > bestScore) { best = fp.type; bestScore = score; }
    }
    deviceType = best ?? "Unknown";
  }

  // Merge Verse API for fingerprinted types
  const api = VERSE_API[deviceType];
  if (api) {
    api.triggers.forEach(t  => { if (!triggers.includes(t))  triggers.push(t); });
    api.receivers.forEach(r => { if (!receivers.includes(r)) receivers.push(r); });
  }

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

# POC: `.uasset` Binary Parser (`scan-verse.js`)

> **Status:** Complete and validated against a real UEFN project.
> **Dependency:** Zero — pure Node.js, no npm packages required.

---

## What This POC Proves

UEFN stores every placed device in your map as a binary `.uasset` file inside `Content/__ExternalActors__/`. These files are not human-readable, and Epic provides no official tool to extract their data programmatically.

This POC proves that a pure Node.js script can:

1. Walk the entire `__ExternalActors__` directory tree (unlimited depth)
2. Parse the UE5 binary asset format with zero third-party libraries
3. Extract — for each device — its type, display label, all trigger/receiver event names, and all configured property values
4. Produce structured JSON ready to be consumed by an MCP server

---

## How to Run

```bash
# Scan entire project — print JSON to stdout
node scan-verse.js --dir "E:\Projects\Testproject"

# Scan and save to file
node scan-verse.js --dir "E:\Projects\Testproject" --out result.json
```

---

## Real Scan Results

Validated against a real UEFN project on **2026-03-04**:

```
total_files   : 14 .uasset
total_devices :  9 parsed
skipped       :  5 (map tiles / non-device objects — expected)
```

**Device types found:**

| Device Type | Instances | Label Examples |
|---|---|---|
| `Device_Campfire_C` | 3 | `Campfire`, `Campfire2`, `game manager` |
| `button_device` | 2 | `Button`, `Button2` |
| `player_spawner_device` | 2 | `Player 1 Spawn Pad`, `Player 2 Spawn Pad` |
| `Device_CharacterSpawner_C` | 1 | `NPC Spawner` |
| `Unknown` | 1 | *(Island Settings — see below)* |

---

## Output Sample: `Device_Campfire_C`

```json
{
  "file": "Testproject/E/HE/BPKN1GUL74YY49UFX81ONY.uasset",
  "deviceType": "Device_Campfire_C",
  "label": "Campfire",
  "triggers": [
    "OnDisabled", "OnEnabled",
    "TriggerOnEnterRadius", "TriggerOnExtinguish",
    "TriggerOnLeavingRadius", "TriggerOnLight",
    "TriggerOnTick", "TriggerOnTickPerPlayer"
  ],
  "receivers": [
    "DisableWhenReceiving", "EnableWhenReceiving",
    "ReceiverAddFuel", "ReceiverExtinguish", "ReceiverLight"
  ],
  "settings": {
    "AIDamagePerPulse": "2.000000",
    "CampfireZoneSize": "7.680000",
    "CanBeExtinguished": "True",
    "CanBeLit": "True",
    "EnabledDuringPhase": "Always",
    "HealthPerPulse": "2.000000",
    "PulseInterval": "1.000000",
    "StartLit": "False",
    "TimeToExtinguish": "1.000000",
    "WoodToAddOnTrigger": "10.000000"
  }
}
```

---

## Output Sample: `button_device`

Classified via fingerprinting (no `Device_*_C` name in Name Map):

```json
{
  "file": "Testproject/4/H4/VQ4QL3ZNB2VCB6CKM0914L.uasset",
  "deviceType": "button_device",
  "label": "Button",
  "triggers": ["OnDisabled", "OnEnabled", "OnTriggered", "TriggerOnInteracted"],
  "receivers": ["ReceiverDisable", "ReceiverEnable", "ReceiverReset"],
  "settings": {
    "ActivatingTeam": "(TeamType=Any,TeamIndex=1)",
    "Delay": "0.000000",
    "EnabledAtGameStart": "True",
    "InteractionRadius": "0.000000",
    "InteractTime": "0.000000",
    "ResetDelay": "0.000000",
    "TriggerSound": "True",
    "VisibleDuringGame": "True"
  }
}
```

---

## The `Unknown` Case: Island Settings Device

One device resolved to `Unknown` — its class name doesn't follow the `Device_*_C` pattern and its settings keys don't match any current fingerprint. However, 70+ settings were extracted correctly and are immediately recognizable as the **Island Settings** device:

```json
"settings": {
  "MaxHealth": "100.000000",
  "MaxShields": "100.000000",
  "SpawnLocation": "SpawnPads",
  "Teams": "(TeamType=FreeForAll,TeamIndex=1)",
  "VoiceChat": "All",
  "RespawnWaveType": "None",
  "TimerDirection": "Count Down",
  "MatchmakingType": "Off",
  "SpawnImmunityTime": "15.000000"
}
```

The property data is fully correct — only the device type label is missing. This will be fixed in Phase 1 by adding an Island Settings fingerprint entry matching keys like `MaxHealth` + `SpawnLocation` + `Teams`.

---

## Binary Parsing — How It Works

The UE5 `.uasset` format is undocumented, but the structure was reverse-engineered from real files using UAssetGUI as a reference. Four steps:

### Step 1: Validate the magic number

Every valid UE asset starts with `0x9E2A83C1` at offset 0. Files that don't match are skipped immediately — this correctly eliminates map tiles and non-device objects.

### Step 2: Read the Name Map

The Name Map is a sequential list of every string identifier used in the asset: device type names, property names, event names, script package paths. Each entry is an FString (4-byte little-endian length + string bytes + null terminator) plus a 4-byte hash.

The Name Map always starts within the first 2000 bytes. It's located by scanning for the first FString beginning with `/Script/` or `/Game/` — the standard UE package prefix. Five or more consecutive valid FStrings confirm the start position.

All device type names (`Device_*_C`), trigger names (`Trigger*`, `On*`), and receiver names (`Receiver*`, `*WhenReceiving`) are classified directly from the Name Map. The `_GEN_VARIABLE` suffixed duplicates (Verse compiler internals) are filtered out.

### Step 3: Extract the label

The device's display label — the name visible in the UEFN Details panel — is stored in the export section as a literal FString pair: `FString("ActorLabel")` immediately followed by `FString(label_value)`. The parser searches for the fixed byte pattern `\x0b\x00\x00\x00ActorLabel\x00` and reads the next FString as the label.

Confirmed in the real scan: `"Player 1 Spawn Pad"`, `"Campfire"`, `"NPC Spawner"`, `"game manager"` — all extracted correctly.

### Step 4: Extract property values

Configured values live in the `PropertyOverrideData` block in the second half of the file. Each entry has a fixed layout:

```
FString(key)  →  25 bytes metadata  →  FString(value)
```

The 25-byte gap is constant — confirmed across `Device_Campfire_C`, `button_device`, `player_spawner_device`, `Device_CharacterSpawner_C`, and the Island Settings device. Strict filters apply to both key and value:

- **Key**: PascalCase, letters only, 4–50 characters, not in the noise blocklist
- **Value**: must match — numeric float, boolean, enum word, team spec `(...)`, or plain alphanumeric string

---

## Device Fingerprinting

Some devices (like `button_device`) don't include a `Device_*_C` class name in their Name Map. For these, the parser identifies the device type by matching extracted settings keys against a fingerprint table:

| Fingerprint keys matched | Classified as | Validated |
|---|---|---|
| `InteractionRadius` + `InteractTime` + `TriggerSound` | `button_device` | ✅ real scan |
| `UseAsIslandStart` + `PlayerTeam` | `player_spawner_device` | ✅ real scan |
| `NPCCharacterDefinition` + `DespawnAIsWhenDisabled` | `character_spawner_device` | — |
| `TrackingType` + `CountToWin` | `tracker_device` | — |
| `MaxHealth` + `SpawnLocation` + `Teams` | `island_settings_device` | 🔜 to be added |

Minimum 2 matching keys required to avoid false positives.

---

## What `_GEN_VARIABLE` Means

Every trigger and receiver in the Name Map appears twice: the real name (e.g., `TriggerOnEnterRadius`) and an internal version with `_GEN_VARIABLE` appended — a backing variable auto-generated by the Verse compiler. These are filtered from all output so only the public API names appear.

---

## Why Skipped Files Are Expected

`__ExternalActors__` contains one `.uasset` per placed object — including terrain tiles, static meshes, and lighting actors, not just devices. These lack the device Name Map structure and are correctly skipped. In this project: 14 files total, 9 devices, 5 skipped (~35% skip rate). This is normal.

---

## Known Gaps

| Gap | Observed | Fix |
|---|---|---|
| Island Settings resolves to `Unknown` | Yes — type label missing, 70+ settings correct | Add fingerprint for `MaxHealth` + `SpawnLocation` + `Teams` |
| `Device_Campfire_C` instance labeled `"game manager"` has empty triggers/receivers | Yes — likely a Verse device using campfire as a class slot | Detect via label heuristic or digest cross-reference |
| 25-byte property gap not validated across all UEFN versions | Not yet tested beyond current build | Validate after each UEFN update |
| No cross-validation against `Fortnite.digest.verse` | Out of scope for this POC | Phase 1: cross-reference to confirm trigger/receiver names |

---

## Next Step: MCP Integration

The JSON output of this script is the core data source for the `scan_map_devices` MCP tool. The Go server will invoke this script at startup, cache the result, and expose it to AI clients:

```
scan-verse.js (Node.js)
       ↓  JSON
Go MCP Server  ←→  Fortnite.digest.verse
       ↓  MCP protocol
AI Client (Cursor / Claude Desktop)
```

With this foundation, an AI assistant querying the MCP can answer questions like:

- *"What triggers does the Campfire device expose?"* → real names from binary, not hallucinated
- *"What are the current settings on the Button device?"* → actual values from the project, not defaults
- *"Wire TriggerOnEnterRadius to disable the NPC spawner"* → correct Verse code using verified API names

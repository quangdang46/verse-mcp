---
name: device-setup-diagnosis
title: Device Setup Diagnosis
summary: Combine placed-device scan data, labels, and Verse docs to explain why a device setup is miswired or behaving differently than expected.
tags:
  - devices
  - diagnosis
  - troubleshooting
---
# Device Setup Diagnosis

Use this workflow when a placed device exists in the map but its runtime behavior, label wiring, or event hookup does not match the Verse code.

1. Run `scan_map_devices` for the project and group the returned devices by type and label.
2. Identify the device or label the user actually means, not just the nearest Verse class name.
3. Query docs for the device family, its listenable events, and any setup notes that mention Details panel options or direct bindings.
4. Compare three views:
   - the placed-device scan output
   - the Verse code that references the device
   - the docs or digest-backed symbol names
5. Look for mismatches such as:
   - device label names that do not match the intended Verse reference
   - events or receivers assumed in code but missing from the placed-device metadata
   - details-panel settings that change runtime behavior but are invisible in code
6. Finish with the smallest next action that would disambiguate the problem: rename labels, assign editables, correct a Verse symbol, or re-scan after metadata reload.

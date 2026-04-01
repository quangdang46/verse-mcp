---
name: editable-audit
title: Editable Property Audit
summary: Review Verse @editable fields, the matching UEFN Details panel assignments, and common null-guard mistakes before debugging device behavior.
tags:
  - editable
  - audit
  - device-setup
---
# Editable Property Audit

Use this workflow when a Verse device compiles but behaves as if references, settings, or linked devices were never assigned in UEFN.

1. Query the docs index for `editable properties` and any field types used by the project, especially `creative_device`, optional references, and arrays.
2. Enumerate the project's `@editable` fields and group them by source file or device class.
3. For each editable, answer:
   - what must be assigned in the Details panel
   - whether the field is optional
   - what runtime guard exists if the value is missing
4. Cross-check placed devices from `scan_map_devices` with the Verse code to find labels or devices that should be linked but are not obviously configured.
5. Flag risky patterns:
   - editables never read
   - optionals read without a guard
   - array editables assumed non-empty
   - mismatch between Verse field intent and actual placed-device labels
6. End with a wiring checklist the user can verify inside UEFN.

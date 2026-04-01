---
name: ui-wiring
title: Verse UI Wiring Troubleshooting
summary: Audit a Verse UI setup for per-player widget storage, widget lifecycle, and editor wiring before changing gameplay code.
tags:
  - ui
  - multiplayer
  - troubleshooting
---
# Verse UI Wiring Troubleshooting

Use this workflow when a Verse UI feature renders for the wrong player, overlaps across sessions, fails to update, or does not appear in game.

1. Query the docs index for `creating ui with verse`, `widget types`, and any widget API names already present in the project.
2. Inspect the Verse code paths that create widgets and verify the code stores UI state per player instead of in one shared global widget reference.
3. Check device or component editables in the project for missing player references, missing root widget references, or nullable widget targets without guards.
4. Confirm lifecycle behavior:
   - where widgets are added
   - where they are updated
   - where they are removed
5. If the bug only appears after edits, call `reload-project-metadata` for the project and repeat the scan before giving advice.
6. Produce a short fix plan that names the Verse files to touch, the widget ownership model, and any UEFN Details panel assignments the user must verify.

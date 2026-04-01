---
name: digest-lookup
title: Digest-Grounded Symbol Lookup
summary: Validate Verse API guesses against built-in docs and digest-backed content before suggesting names, events, or method signatures.
tags:
  - digest
  - symbols
  - grounding
---
# Digest-Grounded Symbol Lookup

Use this workflow when an agent or user is unsure whether a Verse symbol, method, event, or device name actually exists.

1. Start with `query-docs` using the guessed symbol name plus nearby context such as device category, UI module, or gameplay system.
2. Prefer exact symbol matches from digest-backed or API pages over prose guides.
3. If the result is ambiguous, broaden once with the owning module or device family rather than guessing from memory.
4. When a candidate symbol is found, confirm:
   - exact spelling
   - owning module or type
   - callable or field signature
   - any device-specific naming differences between UEFN labels and Verse names
5. If no result is found after a few focused queries, say that directly and suggest the closest grounded alternatives returned by the index.
6. Do not invent missing overloads, events, or properties. Cite only grounded names from the returned docs content.

---
id: 20
title: 'Spike: canonical form and hash of an oath'
type: spike
status: planned
milestone: v0.1
depends_on:
- 12
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: s
area: oath
---

## Question

What exactly is hashed? The identity of an oath must survive renaming of bound variables, whitespace, comments and example reordering, but must change when the type or any example changes. Does the oath's *name* participate in the hash? (Proposal: no; the name is a label and renames are free, as in Unison.)

## Timebox

One day.

## Options considered

- hash a canonical pretty-print of the de Bruijn–indexed AST with examples sorted
- hash a binary encoding of the same
- BLAKE3 vs SHA-256; proposal BLAKE3, first 16 bytes shown in base32 for display

## Answer

_(fill in before closing; a spike without an answer was wasted)_

## Spawns

-

## 2026-09-16

Question to answer explicitly: do examples participate in the hash? If yes (current plan), adding one example creates a new identity and hash importers stay on the old promise until they opt in; that is Unison's semantics and it is principled, since the importer tested against exactly that promise. If no, identity is type plus laws and examples are evidence that only invalidates sworn records. Lean: yes, examples are part of the promise; path imports exist for those who want to follow changes.

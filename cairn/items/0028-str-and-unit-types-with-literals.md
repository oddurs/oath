---
id: 01a26824-a3d2-43b7-b8b1-575b0c15a7c9
title: Str and Unit types with literals
type: feature
status: backlog
milestone: v0.2
depends_on:
- 45551b47-c978-4bd3-bc2c-a262d9451c24
- 37875bb2-b37c-4093-9ed7-3d0e723277e8
created: 2026-09-16
updated: 2026-09-16
priority: p1
effort: s
area: syntax
---

## Problem

Effects need `Unit`; provenance and holes will need `Str`.

## Proposal

`Str` with `"..."` literals and escapes, `++`, `len`, `show : a -> Str`. `Unit` with literal `()`.

## Acceptance criteria

- [ ] `print (show [1,2])` prints `[1, 2]`
- [ ] `Str` participates in canonicalisation and hashing with a test

---
id: 28
title: Str and Unit types with literals
type: feature
status: backlog
milestone: v0.2
depends_on:
- 11
- 18
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

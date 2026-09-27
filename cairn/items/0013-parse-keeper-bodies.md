---
id: d9310c65-7ebb-40db-8ed0-41a5c7ecb153
title: Parse keeper bodies
type: feature
status: planned
milestone: v0.1
depends_on:
- d69d21e3-49ea-40c2-bdb6-98d792a09e6b
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: s
area: syntax
---

## Problem

A keeper is a body attached to an oath by name and label.

## Proposal

```
keep sort by "reference"
  [] -> []
  (x:xs) -> insert x (sort xs)
```
`keep <name> by "<label>"` followed by either a single expression or a match-arm list over the oath's arguments. A file may contain several keepers for one oath.

## Acceptance criteria

- [ ] two keepers with the same oath and label is a parse error
- [ ] a keeper naming an oath not declared in the file is a resolve error with both spans
- [ ] arm-list form desugars to `fn args -> match args with ...`

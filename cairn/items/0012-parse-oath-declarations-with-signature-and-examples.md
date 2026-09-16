---
id: 12
title: Parse oath declarations with signature and examples
type: feature
status: planned
milestone: v0.1
depends_on:
- 11
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: syntax
---

## Problem

The oath is the unit of the language; it needs a syntax before it can be hashed.

## Proposal

```
oath sort : List Int -> List Int
  eg   sort [3,1,2] == [1,2,3]
  eg   sort [] == []
```
An oath has a name, a type signature, and zero or more `eg` lines. Each `eg` is an expression of the form `<call> == <expr>`.

## Acceptance criteria

- [ ] an oath with zero examples parses (it will be an unkeepable oath until v0.3)
- [ ] an `eg` whose left side does not call the oath is a parse error with a message saying so
- [ ] the AST keeps examples in source order (canonicalisation is a separate item)

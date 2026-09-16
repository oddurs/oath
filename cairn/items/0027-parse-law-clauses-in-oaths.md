---
id: 27
title: Parse law clauses in oaths
type: feature
status: backlog
milestone: v0.2
depends_on:
- 12
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: s
area: syntax
---

## Problem

Examples are evidence; laws are the promise.

## Proposal

```
oath sort : List Int -> List Int
  law \xs -> sorted (sort xs)
  law \xs -> length (sort xs) == length xs
```
A `law` is a lambda over the oath's argument types returning `Bool`.

## Acceptance criteria

- [ ] laws parse in any order relative to `eg` lines
- [ ] a law that is not a lambda is a parse error with a hint

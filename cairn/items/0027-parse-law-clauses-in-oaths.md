---
id: 437b3e57-8a55-42f9-a262-58c58155f48d
title: Parse law clauses in oaths
type: feature
status: backlog
milestone: v0.2
depends_on:
- d69d21e3-49ea-40c2-bdb6-98d792a09e6b
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

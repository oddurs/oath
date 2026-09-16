---
id: 101
title: Parse use declarations
type: feature
status: backlog
milestone: v0.4
depends_on:
- 100
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: syntax
---

## Proposal

Per the spike. Provisionally:
```
use ./lib.oath            -- every oath in the file, by name
use sort = oath:7f3a9k2m  -- one oath, by hash, under a local name
```

## Acceptance criteria

- [ ] a `use` after any non-`use` declaration is a parse error
- [ ] a malformed hash literal is a parse error naming the expected length

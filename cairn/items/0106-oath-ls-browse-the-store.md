---
id: 106
title: 'oath ls: browse the store'
type: feature
status: backlog
milestone: v0.4
depends_on:
- 41
created: 2026-09-16
updated: 2026-09-16
priority: p1
effort: m
area: cli
---

## Proposal

`oath ls` lists every oath in the store: names, hash, keeper count, best evidence depth. `oath ls <hash-prefix>` shows one. `--json` for tooling.

## Acceptance criteria

- [ ] unique hash prefixes are accepted everywhere a hash is
- [ ] golden test with a fixture store

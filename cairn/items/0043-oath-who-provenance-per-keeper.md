---
id: 43
title: 'oath who: provenance per keeper'
type: feature
status: backlog
milestone: v0.2
depends_on:
- 41
created: 2026-09-16
updated: 2026-09-16
priority: p1
effort: m
area: cli
---

## Problem

When a model or a colleague wrote a keeper, you want to know before you trust it.

## Proposal

`producer` defaults to `$USER@host`; `--producer` overrides; later keeper sources set their own. `oath who sort` lists every keeper of the current oath hash and of previous hashes of that name, with verdict, producer, depth, and time.

## Acceptance criteria

- [ ] output is stable and covered by a golden test with a fixed clock
- [ ] `--json` prints the records verbatim

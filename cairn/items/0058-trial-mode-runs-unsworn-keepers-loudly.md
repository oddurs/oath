---
id: e5c55be7-ce8a-4b9b-8acd-b227788e6188
title: Trial mode runs unsworn keepers loudly
type: feature
status: backlog
milestone: v0.3
depends_on:
- 00b844e9-f272-448c-aa30-2d4595d7b3df
- 35a4f75b-4dea-4681-b3c1-927cdd162ff4
created: 2026-09-16
updated: 2026-09-16
priority: p1
effort: m
area: oath
---

## Problem

Sometimes you want to run the thing before it is proven.

## Proposal

`oath run --trial`: unsworn keepers are eligible; every call to one prints `trial: sort by "fast" (unsworn)` to stderr once; exit code is 3 if any trial keeper ran.

## Acceptance criteria

- [ ] golden test asserts the stderr line and exit 3
- [ ] `--trial` cannot be combined with `--keep-with` (refused)

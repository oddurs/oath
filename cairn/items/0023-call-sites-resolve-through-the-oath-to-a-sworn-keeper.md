---
id: 23
title: Call sites resolve through the oath to a sworn keeper
type: feature
status: planned
milestone: v0.1
depends_on:
- 22
- 41
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: eval
---

## Problem

Callers must never name a body. They name the oath, and the runtime supplies a sworn keeper.

## Proposal

Before `oath run`, swear everything in the file. Bind each oath name to a dispatcher that picks the first sworn keeper in source order. Calling an oath with no sworn keeper is a runtime error naming the oath, its hash, and how many unsworn keepers exist.

## Acceptance criteria

- [ ] a program whose only keeper of `sort` is broken fails at the first call to `sort`, not at parse time
- [ ] with two keepers where the first is unsworn, the second is used (golden test with `print` proving which ran)
- [ ] `oath run --no-swear` is refused with a message; there is no way to run unsworn code in v0.1

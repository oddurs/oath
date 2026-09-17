---
id: 39
title: Laws checked during swearing with smallest counterexample
type: feature
status: backlog
milestone: v0.2
depends_on:
- 22
- 38
- 97
- 121
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: oath
---

## Problem

Examples show the keeper works where you looked. Laws show it works where you did not.

## Proposal

For each keeper: every law over every generated input, smallest first, stop at the first failure. Report `law 2 fails at xs = [1,0]` with expected `true`.

## Acceptance criteria

- [ ] golden test: a `sort` keeper that drops duplicates is unsworn by the length law with input `[0,0]`
- [ ] the sworn record stores the depth reached

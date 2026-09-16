---
id: 47
title: Prelude written in Oath
type: feature
status: backlog
milestone: v0.2
depends_on:
- 23
- 39
- 119
created: 2026-09-16
updated: 2026-09-16
priority: p1
effort: m
area: stdlib
---

## Problem

Every program rewrites `length`, `map`, `sorted`.

## Proposal

`stdlib/prelude.oath` embedded in the binary: `length map filter foldr foldl reverse sorted insert elem zip`. Every prelude function is an oath with laws and a sworn keeper. Loaded implicitly; `--no-prelude` disables.

## Acceptance criteria

- [ ] prelude swears clean at depth 5 in CI
- [ ] the README's sort example uses prelude `insert` and `sorted`

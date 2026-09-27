---
id: f74a2d05-8c6e-416e-9e09-4ae2a432bba8
title: Prelude written in Oath
type: feature
status: backlog
milestone: v0.2
depends_on:
- 00b844e9-f272-448c-aa30-2d4595d7b3df
- e29e8d64-cba1-44f8-bd19-168b0cc88bb9
- 470bab2e-32de-400e-804c-68b1d16ce8d6
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

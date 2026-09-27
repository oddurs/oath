---
id: e29e8d64-cba1-44f8-bd19-168b0cc88bb9
title: Laws checked during swearing with smallest counterexample
type: feature
status: backlog
milestone: v0.2
depends_on:
- 077ae825-1ca6-45c7-af09-fb4c147bad02
- 908186e3-49b5-49ee-bb45-d7f5efe3187e
- 2d1cc97f-2183-4421-abbb-7a1bfce2ab0b
- 4e904923-2eb3-4753-b3e6-fa3d9c2ad42d
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

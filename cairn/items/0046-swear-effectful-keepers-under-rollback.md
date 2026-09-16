---
id: 46
title: Swear effectful keepers under rollback
type: feature
status: backlog
milestone: v0.2
depends_on:
- 34
- 36
- 39
- 118
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: oath
---

## Problem

Most useful code performs effects. Swearing it must not change the world.

## Proposal

Each example and law run is wrapped in an implicit `try`; the journal is rewound after each. The sworn record lists the effects performed.

## Acceptance criteria

- [ ] golden test: swearing a keeper that increments a counter leaves the counter at 0 afterwards
- [ ] `oath who` shows `effects: [Counter.incr]` for that keeper
